//! Gợi ý SEO / GEO cho ghi chú và bài viết — chỉ là gợi ý, không chặn build.
//! Gợi ý cho GHI CHÚ chỉ xoay quanh độ rõ ràng và độ tin cậy — đúng bản chất tri thức, không nhồi từ khoá.

use crate::articles::Article;
use crate::kb::{Kb, Note};
use crate::render::{all_headings, extract_faq, extract_headings};
use crate::text::{keyword_core, markdown_to_plain, normalize_for_search, slugify};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use serde::Serialize;

// ---------- Cấu trúc bài cho GEO ----------
//
// Các gợi ý GEO rút từ nghiên cứu và bộ quy tắc mở (xem .claude/skills/research/references/geo.md):
// GEO (Aggarwal và cộng sự, KDD 2024): dẫn nguồn, trích dẫn, số liệu giúp nội dung được AI nhắc tới nhiều hơn;
// nhồi từ khoá gần như vô ích. AutoGEO (ICLR 2026): AI ưu tiên bài nêu kết luận trước, mỗi đoạn một ý,
// tự đứng được, cân bằng nhiều góc nhìn. Mọi gợi ý chỉ xét CẤU TRÚC và cách trình bày — không bao giờ
// đòi thêm khẳng định, số liệu hay từ khoá; độ chính xác của tri thức luôn đứng trên.

/// Đoạn dài hơn chừng này (đếm theo chữ cách nhau bằng dấu cách) thì khó đọc và khó trích.
const LONG_PARAGRAPH: usize = 160;

/// Câu mở đầu bằng những chữ này thường dựa vào đoạn trước — trích riêng sẽ khó hiểu.
const DANGLING_STARTS: &[&str] = &[
    "nó ", "điều này", "việc này", "cái này", "những điều này", "tuy nhiên", "nhưng ", "và ", "ngoài ra", "do đó", "vì vậy",
    "vì thế", "thế nên", "họ ", "chúng ", "hơn nữa", "bên cạnh đó", "trong khi đó", "cũng ",
];

/// Mục không cần xét "tự đứng được" (hỏi đáp, liên kết, nguồn).
const SKIP_SECTIONS: &[&str] = &["cau-hoi-thuong-gap", "hoi-dap", "faq", "lien-quan", "nguon", "tai-lieu", "doc-them"];

/// Tiêu đề mục cho thấy bài có nói giới hạn / mặt trái (góc nhìn cân bằng).
const BALANCE_HEADINGS: &[&str] = &[
    "gioi-han", "han-che", "tranh-cai", "rui-ro", "nhuoc-diem", "luu-y", "khi-nao-khong", "phan-bien", "sai-lam", "canh-bao",
    "diem-yeu", "chua-ro", "chua-thong-nhat",
];

#[derive(Default)]
struct Structure {
    /// Có đoạn văn trước mục "## …" đầu tiên.
    lead: bool,
    /// (tiêu đề mục cấp 2, đoạn văn đầu tiên ngay dưới nó nếu mục mở đầu bằng đoạn văn).
    sections: Vec<(String, Option<String>)>,
    long_paragraphs: usize,
    blockquote: bool,
    ordered_list: bool,
    table: bool,
    /// Tiêu đề đầu tiên nhảy cấp (vd. từ ## xuống ####).
    heading_skip: Option<String>,
}

fn analyze(markdown: &str) -> Structure {
    let cleaned = regex::Regex::new(r"(?s)<!--.*?-->").unwrap().replace_all(markdown, "").to_string();
    let mut s = Structure::default();
    let (mut seen_h2, mut prev_level) = (false, 1u8);
    let mut pending: Option<usize> = None;
    let (mut in_heading, mut in_para) = (false, false);
    let (mut heading, mut para) = (String::new(), String::new());
    let (mut lists, mut quotes) = (0usize, 0usize);
    let mut level = 1u8;
    for ev in Parser::new_ext(&cleaned, Options::ENABLE_TABLES) {
        match ev {
            Event::Start(Tag::Heading { level: l, .. }) => {
                level = l as u8;
                in_heading = true;
                heading.clear();
            }
            Event::End(TagEnd::Heading(_)) => {
                in_heading = false;
                if level > prev_level + 1 && s.heading_skip.is_none() {
                    s.heading_skip = Some(heading.trim().to_string());
                }
                prev_level = level;
                if level == 2 {
                    seen_h2 = true;
                    s.sections.push((heading.trim().to_string(), None));
                    pending = Some(s.sections.len() - 1);
                } else {
                    pending = None;
                }
            }
            Event::Start(Tag::Paragraph) => {
                in_para = true;
                para.clear();
            }
            Event::End(TagEnd::Paragraph) => {
                in_para = false;
                let words = para.split_whitespace().count();
                if lists == 0 && quotes == 0 {
                    if words > LONG_PARAGRAPH {
                        s.long_paragraphs += 1;
                    }
                    if !seen_h2 && words > 0 {
                        s.lead = true;
                    }
                    if let Some(i) = pending.take() {
                        s.sections[i].1 = Some(para.trim().to_string());
                    }
                }
            }
            Event::Start(Tag::List(start)) => {
                lists += 1;
                s.ordered_list |= start.is_some();
                pending = None;
            }
            Event::End(TagEnd::List(_)) => lists = lists.saturating_sub(1),
            Event::Start(Tag::Table(_)) => {
                s.table = true;
                pending = None;
            }
            Event::Start(Tag::BlockQuote(_)) => {
                quotes += 1;
                s.blockquote = true;
                pending = None;
            }
            Event::End(TagEnd::BlockQuote(_)) => quotes = quotes.saturating_sub(1),
            Event::Start(Tag::CodeBlock(_)) => pending = None,
            Event::Text(t) | Event::Code(t) => {
                if in_heading {
                    heading.push_str(&t);
                } else if in_para {
                    para.push_str(&t);
                }
            }
            Event::SoftBreak | Event::HardBreak if in_para => para.push(' '),
            _ => {}
        }
    }
    s
}

/// Đoạn mở đầu bằng chữ nối / đại từ trỏ ngược ("Nó…", "Tuy nhiên…").
fn dangling(paragraph: &str) -> bool {
    let p = paragraph.trim_start_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
    DANGLING_STARTS.iter().any(|d| p.starts_with(d))
}

/// Các mục "## …" cần xét: (số mục mở đầu tự đứng được, tổng số mục có đoạn mở đầu, mục đầu tiên chưa đạt).
fn self_contained(s: &Structure) -> (usize, usize, Option<String>) {
    let checked: Vec<&(String, Option<String>)> =
        s.sections.iter().filter(|(h, p)| p.is_some() && !SKIP_SECTIONS.iter().any(|k| slugify(h).starts_with(k))).collect();
    let bad: Vec<&String> = checked.iter().filter(|(_, p)| p.as_deref().is_some_and(dangling)).map(|(h, _)| h).collect();
    (checked.len() - bad.len(), checked.len(), bad.first().map(|h| h.to_string()))
}

/// Gợi ý cấu trúc dùng chung cho ghi chú và bài viết.
fn structure_hints(out: &mut Vec<Hint>, s: &Structure) {
    let (ok, total, first_bad) = self_contained(s);
    if total > 0 {
        let label = match &first_bad {
            Some(hd) => format!("{ok}/{total} mục mở đầu tự đứng được (xem mục \"{hd}\")"),
            None => format!("{ok}/{total} mục mở đầu tự đứng được"),
        };
        h(out, ok == total, label, "Câu đầu mỗi mục nên gọi đúng tên chủ thể (\"Mô hình búa là…\", \"Wyckoff chia…\") thay vì \"Nó / Điều này / Tuy nhiên…\" — để đoạn đó được trích riêng vẫn hiểu đúng. Chỉ đổi cách diễn đạt, giữ nguyên ý.");
    }
    h(
        out,
        s.long_paragraphs == 0,
        if s.long_paragraphs == 0 { "Không có đoạn quá dài".into() } else { format!("{} đoạn dài hơn {LONG_PARAGRAPH} chữ", s.long_paragraphs) },
        "Đoạn rất dài khó đọc và khó trích. Tách theo ý — mỗi đoạn một ý trọn vẹn; không cắt vụn, không bỏ bớt nội dung.",
    );
    let label = match &s.heading_skip {
        Some(hd) => format!("Tiêu đề nhảy cấp ở \"{hd}\""),
        None => "Cấp tiêu đề liền mạch".into(),
    };
    h(out, s.heading_skip.is_none(), label, "Đi lần lượt ## → ### → ####, không nhảy cấp — công cụ tìm kiếm và AI dựa vào đó để hiểu bố cục.");
}

#[derive(Debug, Clone, Serialize)]
pub struct Hint {
    pub ok: bool,
    pub label: String,
    pub tip: String,
}

fn h(out: &mut Vec<Hint>, ok: bool, label: String, tip: &str) {
    out.push(Hint { ok, label, tip: tip.to_string() });
}

fn first_paragraph(markdown: &str) -> String {
    let cleaned = regex::Regex::new(r"(?s)<!--.*?-->").unwrap().replace_all(markdown, "").to_string();
    for (ev, range) in Parser::new(&cleaned).into_offset_iter() {
        if let Event::Start(Tag::Paragraph) = ev {
            return markdown_to_plain(&cleaned[range]);
        }
    }
    String::new()
}

fn days_between(from: &str, to: &str) -> i64 {
    let p = |s: &str| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok();
    match (p(from), p(to)) {
        (Some(a), Some(b)) => (b - a).num_days(),
        _ => 0,
    }
}

pub fn note_hints(note: &Note, today: &str) -> Vec<Hint> {
    let mut out = Vec::new();
    let t = note.title.chars().count();
    h(&mut out, t <= 60, format!("Tiêu đề {t} ký tự"), "Nên ≤ 60 ký tự để không bị cắt trên Google.");
    let d = note.summary.chars().count();
    h(&mut out, (70..=160).contains(&d), format!("Tóm tắt {d} ký tự"), "Tóm tắt là mô tả trên Google và là câu AI đọc đầu tiên — nên 70–160 ký tự, nói rõ ghi chú trả lời điều gì.");
    if note.kind == "concept" {
        let intro = normalize_for_search(&first_paragraph(&note.body));
        let ok = std::iter::once(&note.title).chain(note.aliases.iter()).map(|n| keyword_core(n)).any(|n| !n.is_empty() && intro.contains(&n));
        h(&mut out, ok, "Đoạn mở đầu trả lời thẳng \"… là gì\"".into(), "Câu đầu nên nhắc tên thuật ngữ và định nghĩa ngay (\"X là …\") — AI search hay trích nguyên câu này.");
    }
    if ["concept", "framework", "topic"].contains(&note.kind.as_str()) {
        let faq = extract_faq(&note.body).len();
        h(&mut out, faq >= 2, format!("{faq} câu hỏi thường gặp"), "Thêm mục \"Câu hỏi thường gặp\" với 3–5 câu hỏi — site tự sinh dữ liệu FAQ cho Google/AI.");
    }
    if note.kind != "topic" {
        h(&mut out, !note.sources.is_empty(), format!("{} nguồn", note.sources.len()), "Nguồn có tên, ngày và điểm uy tín giúp người đọc lẫn AI tin và trích dẫn.");
    }
    // ---- GEO: cấu trúc giúp AI trích đúng (không đụng tới nội dung tri thức) ----
    let s = analyze(&note.body);
    if note.kind != "concept" {
        h(&mut out, s.lead, if s.lead { "Có đoạn mở đầu nêu ý chính".into() } else { "Chưa có đoạn mở đầu trước mục đầu tiên".into() },
            "Mở bài bằng 1–3 câu nêu ý chính của ghi chú trước khi vào các mục — AI và người đọc lấy ý từ đây. Tóm từ nội dung đã có, không thêm khẳng định mới.");
    }
    if note.kind != "topic" && !note.sources.is_empty() {
        // "(Nguồn [2])" hoặc "[2][4]" — số thứ tự nguồn trong ngoặc vuông.
        let cites = regex::Regex::new(r"\[\d{1,2}\]").unwrap().find_iter(&note.body).count();
        let need = (note.word_count / 500).max(1);
        h(&mut out, cites >= need, format!("Dẫn nguồn {cites} lần trong bài"),
            "Ghi (Nguồn [n]) ngay sau khẳng định quan trọng — dẫn nguồn là cách tăng độ tin cậy hiệu quả nhất với AI search. Chỉ dẫn nguồn thật trong mục sources và đúng chỗ nguồn đó nói.");
    }
    structure_hints(&mut out, &s);
    if ["concept", "framework"].contains(&note.kind.as_str()) {
        let balanced = all_headings(&note.body).iter().any(|hd| BALANCE_HEADINGS.iter().any(|k| hd.id.contains(k)));
        h(&mut out, balanced, if balanced { "Có mục giới hạn / lưu ý".into() } else { "Chưa có mục giới hạn / lưu ý".into() },
            "AI search ưu tiên nguồn cân bằng: thêm mục nêu giới hạn, chỗ còn tranh cãi hoặc khi nào không áp dụng — chỉ lấy từ nguồn đã có, ghi rõ chỗ chưa kiểm chứng.");
    }
    if note.kind == "framework" {
        let steps = s.ordered_list || s.table;
        h(&mut out, steps, if steps { "Có các bước đánh số hoặc bảng".into() } else { "Chưa có bước đánh số hoặc bảng".into() },
            "Phương pháp / quy trình nên có danh sách bước đánh số hoặc bảng — người đọc làm theo được, AI trích chính xác hơn.");
    }
    if note.kind != "topic" && note.sources.iter().any(|x| x.credibility.is_some_and(|c| c >= 4)) {
        let quoted = s.blockquote || regex::Regex::new(r"“[^”]{40,}”").unwrap().is_match(&note.body);
        h(&mut out, quoted, if quoted { "Có trích nguyên văn tài liệu gốc".into() } else { "Chưa trích nguyên văn tài liệu gốc".into() },
            "Một câu trích ngắn, đúng nguyên văn (kèm số trang) từ tài liệu gốc giúp người đọc và AI tin hơn. Giữ ngắn, không dịch sai ý, không trích để trang trí.");
    }
    h(&mut out, !note.backlinks.is_empty(), format!("{} ghi chú liên kết tới", note.backlinks.len()), "Trang không có liên kết nội bộ trỏ tới khó được Google tìm thấy — thêm vào ghi chú chủ đề hoặc mục Liên quan.");
    h(&mut out, note.word_count >= 150, format!("{} từ", note.word_count), "Nội dung mỏng (< 150 từ) — bổ sung giải thích, ví dụ, giới hạn.");
    let age = days_between(&note.updated, today);
    h(&mut out, age <= 365, format!("Cập nhật {age} ngày trước"), "Đã hơn 1 năm — rà soát số liệu và nguồn (độ mới là tín hiệu tin cậy).");
    out
}

pub fn article_hints(a: &Article, kb: &Kb) -> Vec<Hint> {
    let mut out = Vec::new();
    let kw = keyword_core(&a.keyword);
    let has = |s: &str| !kw.is_empty() && normalize_for_search(s).contains(&kw);
    let t = a.title.chars().count();
    h(&mut out, t <= 60, format!("Tiêu đề {t} ký tự"), "Nên ≤ 60 ký tự để không bị cắt trên Google.");
    h(&mut out, has(&a.title), "Tiêu đề chứa từ khoá chính".into(), "Đưa từ khoá chính vào tiêu đề, càng gần đầu càng tốt — viết tự nhiên, không giật tít.");
    let d = a.description.chars().count();
    h(&mut out, (70..=160).contains(&d), format!("Mô tả {d} ký tự"), "Mô tả nên 70–160 ký tự: nói người đọc nhận được gì từ bài.");
    h(&mut out, has(&a.description), "Mô tả chứa từ khoá chính".into(), "Nhắc từ khoá chính trong mô tả.");
    let plain = markdown_to_plain(&a.body);
    let intro: String = plain.split_whitespace().take(120).collect::<Vec<_>>().join(" ");
    h(&mut out, has(&intro), "Đoạn mở đầu trả lời ngay câu hỏi".into(), "Trả lời câu hỏi của người tìm trong 2–3 câu đầu (có từ khoá) — AI search và đoạn trích nổi bật lấy từ đây.");
    let h2 = extract_headings(&a.body).iter().filter(|x| x.depth == 2).count();
    h(&mut out, h2 >= 3, format!("{h2} mục lớn (H2)"), "Chia bài theo các câu hỏi phụ người đọc hay hỏi, mỗi câu một mục \"## …\".");
    let faq = extract_faq(&a.body).len();
    h(&mut out, faq >= 3, format!("{faq} câu hỏi thường gặp"), "Thêm \"## Câu hỏi thường gặp\" với ≥ 3 câu \"### …?\" (lấy từ \"Mọi người cũng hỏi\" trên Google).");
    let linked = a.notes.iter().filter(|n| a.links.contains(n)).count();
    h(&mut out, !a.notes.is_empty() && linked == a.notes.len(), format!("Liên kết về {linked}/{} ghi chú gốc", a.notes.len()), "Đặt [[liên kết]] tới từng ghi chú gốc ngay chỗ dùng dữ kiện của nó.");
    let unverified = a.notes.iter().filter(|s| kb.get(s).is_none_or(|n| n.inbox || n.status != "verified")).count();
    let label = if unverified > 0 { format!("{unverified} ghi chú gốc chưa kiểm chứng") } else { "Ghi chú gốc đều đã kiểm chứng".into() };
    h(&mut out, unverified == 0, label, "Duyệt tri thức gốc (verified) trước khi đăng bài — bài viết không được \"đi trước\" tri thức.");
    let occurrences = if kw.is_empty() { 0 } else { normalize_for_search(&plain).matches(&kw).count() };
    let per100 = if a.word_count > 0 { occurrences as f64 / a.word_count as f64 * 100.0 } else { 0.0 };
    h(&mut out, per100 <= 3.0, format!("Từ khoá ~{per100:.1} lần/100 từ"), "Quá dày — viết tự nhiên, dùng từ đồng nghĩa; nhồi từ khoá làm giảm chất lượng.");
    h(&mut out, a.word_count >= 600, format!("{} từ", a.word_count), "Bài kéo traffic thường cần ≥ 600 từ để trả lời đủ ý — nhưng đừng độn chữ.");
    structure_hints(&mut out, &analyze(&a.body));
    out
}

#[derive(Debug, Clone, Serialize)]
pub struct KeywordTarget {
    pub kind: &'static str,
    #[serde(rename = "match")]
    pub matched: &'static str,
    pub slug: String,
    pub title: String,
    pub path: String,
}

/// Trang nào đã nhắm (hoặc gần) từ khoá này — kiểm tra trước khi viết bài mới.
pub fn keyword_targets(keyword: &str, notes: &[&Note], articles: &[&Article]) -> Vec<KeywordTarget> {
    let core = keyword_core(keyword);
    if core.is_empty() {
        return vec![];
    }
    let mut out = Vec::new();
    let mut check = |kind: &'static str, slug: &str, title: &str, path: &str, names: Vec<&String>| {
        let cores: Vec<String> = names.into_iter().map(|n| keyword_core(n)).filter(|c| !c.is_empty()).collect();
        let matched = if cores.contains(&core) {
            Some("exact")
        } else if cores.iter().any(|c| (c.chars().count() > 3 && core.contains(c.as_str())) || (core.chars().count() > 3 && c.contains(&core))) {
            Some("near")
        } else {
            None
        };
        if let Some(m) = matched {
            out.push(KeywordTarget { kind, matched: m, slug: slug.into(), title: title.into(), path: path.into() });
        }
    };
    for n in notes {
        check("note", &n.slug, &n.title, &n.path, std::iter::once(&n.title).chain(n.aliases.iter()).collect());
    }
    for a in articles {
        check("article", &a.slug, &a.title, &a.path, std::iter::once(&a.keyword).chain(a.keywords.iter()).chain(std::iter::once(&a.title)).collect());
    }
    out.sort_by_key(|t| if t.matched == "exact" { 0 } else { 1 });
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cau_truc_bai_cho_geo() {
        let md = "Mở bài nêu ý chính.\n\n## Định nghĩa\n\nMô hình búa là nến có bóng dưới dài.\n\n## Cách dùng\n\nTuy nhiên cần chờ xác nhận.\n\n## Các bước\n\n1. Một\n2. Hai\n\n#### Nhảy cấp\n\n> Trích nguyên văn.\n\n## Câu hỏi thường gặp\n\nNó không tính.\n";
        let s = analyze(md);
        assert!(s.lead && s.ordered_list && s.blockquote && !s.table);
        assert_eq!(s.heading_skip.as_deref(), Some("Nhảy cấp"));
        // "Các bước" mở đầu bằng danh sách, "Câu hỏi thường gặp" bỏ qua -> xét 2 mục, 1 mục chưa tự đứng được.
        assert_eq!(self_contained(&s), (1, 2, Some("Cách dùng".to_string())));
        let long = format!("## A\n\n{}\n", "chữ ".repeat(LONG_PARAGRAPH + 1));
        let s = analyze(&long);
        assert!(!s.lead);
        assert_eq!(s.long_paragraphs, 1);
    }

    #[test]
    fn chu_noi_dau_doan() {
        assert!(dangling("Điều này cho thấy…"));
        assert!(dangling("**Tuy nhiên**, giá có thể…"));
        assert!(!dangling("Nến búa là…"));
        assert!(!dangling("Những người mua…"));
    }
}
