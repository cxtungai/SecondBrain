//! Gợi ý SEO / GEO cho ghi chú và bài viết — chỉ là gợi ý, không chặn build.
//! Gợi ý cho GHI CHÚ chỉ xoay quanh độ rõ ràng và độ tin cậy — đúng bản chất tri thức, không nhồi từ khoá.

use crate::articles::Article;
use crate::kb::{Kb, Note};
use crate::render::{extract_faq, extract_headings};
use crate::text::{keyword_core, markdown_to_plain, normalize_for_search};
use pulldown_cmark::{Event, Parser, Tag};
use serde::Serialize;

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
