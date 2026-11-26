//! Markdown → HTML (cho người đọc) và Markdown chuẩn (cho AI agent); mục lục, FAQ.

use crate::text::{escape_html, map_outside_code, markdown_to_plain, slugify, WIKILINK};
use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use regex::Regex;
use std::sync::LazyLock;

fn options() -> Options {
    Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS | Options::ENABLE_FOOTNOTES
}

/// Đích của một wikilink sau khi tra: đường dẫn, tiêu đề hiển thị, mô tả (tooltip).
pub struct LinkTarget {
    pub href: String,
    pub title: String,
    pub tooltip: String,
}

static HEADING_PREFIX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s{0,3}#{1,6}\s*").unwrap());
static HEADING_SUFFIX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+#+\s*$").unwrap());

fn level_num(l: HeadingLevel) -> u8 {
    match l {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Heading {
    pub depth: u8,
    pub text: String,
    pub id: String,
}

/// Mọi heading theo thứ tự, kèm id (slug của chữ hiển thị).
pub fn all_headings(markdown: &str) -> Vec<Heading> {
    let mut out = Vec::new();
    for (ev, range) in Parser::new_ext(markdown, options()).into_offset_iter() {
        if let Event::Start(Tag::Heading { level, .. }) = ev {
            let src = &markdown[range];
            let first = src.lines().next().unwrap_or("");
            // Heading kiểu ATX: bỏ "## " và "##" cuối; kiểu Setext: dòng đầu là chữ.
            let inline = HEADING_SUFFIX.replace(&HEADING_PREFIX.replace(first, ""), "").to_string();
            let text = markdown_to_plain(&inline);
            out.push(Heading { depth: level_num(level), id: slugify(&text), text });
        }
    }
    out
}

/// Heading cấp 2–3 để làm mục lục.
pub fn extract_headings(markdown: &str) -> Vec<Heading> {
    all_headings(markdown).into_iter().filter(|h| h.depth == 2 || h.depth == 3).collect()
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Faq {
    pub question: String,
    pub answer: String,
}

static FAQ_HEADING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(cau-hoi-thuong-gap|hoi-dap|faq)").unwrap());

/// Mục "Câu hỏi thường gặp": heading cấp 2, mỗi câu hỏi là heading cấp 3 — dùng cho dữ liệu FAQPage.
pub fn extract_faq(markdown: &str) -> Vec<Faq> {
    let headings = all_headings(markdown);
    let mut hi = 0;
    let mut faq: Vec<Faq> = Vec::new();
    let mut in_faq = false;
    let mut depth = 0usize;
    let mut current: Option<usize> = None;
    for (ev, range) in Parser::new_ext(markdown, options()).into_offset_iter() {
        match ev {
            Event::Start(tag) => {
                if depth == 0 {
                    if let Tag::Heading { level, .. } = &tag {
                        let h = &headings[hi];
                        hi += 1;
                        let lv = level_num(*level);
                        if lv <= 2 {
                            in_faq = lv == 2 && FAQ_HEADING.is_match(&h.id);
                            current = None;
                        } else if in_faq && lv == 3 {
                            faq.push(Faq { question: h.text.clone(), answer: String::new() });
                            current = Some(faq.len() - 1);
                        }
                    } else if in_faq {
                        if let Some(c) = current {
                            let text = markdown_to_plain(&markdown[range.clone()]);
                            let a = &mut faq[c].answer;
                            if !a.is_empty() {
                                a.push(' ');
                            }
                            a.push_str(&text);
                        }
                    }
                } else if let Tag::Heading { .. } = tag {
                    hi += 1;
                }
                depth += 1;
            }
            Event::End(_) => depth = depth.saturating_sub(1),
            _ => {}
        }
    }
    faq.into_iter().filter(|f| !f.question.is_empty() && !f.answer.is_empty()).collect()
}

/// Tiêu đề kiểu "## Chữ" (ngoài khối code — người gọi tự bỏ qua code): (cấp, chữ thô).
pub fn atx_heading(line: &str) -> Option<(usize, &str)> {
    let t = line.trim_start();
    let hashes = t.chars().take_while(|&c| c == '#').count();
    ((1..=6).contains(&hashes) && t[hashes..].starts_with(' ')).then(|| (hashes, t[hashes..].trim().trim_end_matches('#').trim()))
}

static EXERCISE_HEADING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(bai-tap|tu-kiem-tra|cau-hoi-on-tap)").unwrap());
static OBJECTIVES_HEADING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(muc-tieu-bai-hoc|sau-bai-nay)").unwrap());

/// Phần ẩn trong một bài tập: đoạn mở bằng **Đáp án:** hoặc **Gợi ý:**.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Reveal {
    Answer,
    Hint,
}

impl Reveal {
    fn of(label: &str) -> Option<Self> {
        let s = slugify(label);
        if s.starts_with("dap-an") {
            Some(Self::Answer)
        } else if s.starts_with("goi-y") {
            Some(Self::Hint)
        } else {
            None
        }
    }

    /// Dòng Markdown mở bằng **Đáp án:** / **Gợi ý:**.
    pub fn of_line(line: &str) -> Option<Self> {
        Self::of(line.trim_start().strip_prefix("**")?.split("**").next()?)
    }
}

/// Một câu trong mục "Bài tập" (tiêu đề cấp 3).
#[derive(Debug, Clone, serde::Serialize)]
pub struct Exercise {
    pub question: String,
    pub has_answer: bool,
}

/// Mục "Bài tập" (hoặc "Tự kiểm tra"): mỗi câu là một tiêu đề cấp 3; câu có đáp án khi có đoạn mở bằng **Đáp án:**.
pub fn extract_exercises(markdown: &str) -> Vec<Exercise> {
    let (mut out, mut fence, mut inside) = (Vec::<Exercise>::new(), false, false);
    for line in markdown.lines() {
        let t = line.trim_start();
        if t.starts_with("```") || t.starts_with("~~~") {
            fence = !fence;
            continue;
        }
        if fence {
            continue;
        }
        if let Some((lv, text)) = atx_heading(t) {
            let text = markdown_to_plain(text);
            if lv <= 2 {
                inside = lv == 2 && EXERCISE_HEADING.is_match(&slugify(&text));
            } else if inside && lv == 3 {
                out.push(Exercise { question: text, has_answer: false });
            }
            continue;
        }
        if inside && Reveal::of_line(t) == Some(Reveal::Answer) {
            if let Some(e) = out.last_mut() {
                e.has_answer = true;
            }
        }
    }
    out
}

/// Id của mục "Bài tập" trên trang (để dẫn tới), nếu có.
pub fn exercises_id(markdown: &str) -> Option<String> {
    all_headings(markdown).into_iter().find(|h| h.depth == 2 && EXERCISE_HEADING.is_match(&h.id)).map(|h| h.id)
}

/// Bài có mục "Mục tiêu bài học" ("Sau bài này bạn sẽ…") không.
pub fn has_objectives(markdown: &str) -> bool {
    all_headings(markdown).iter().any(|h| h.depth == 2 && OBJECTIVES_HEADING.is_match(&h.id))
}

#[derive(Clone, Copy, PartialEq)]
enum Block {
    Other,
    Objectives,
    Exercises,
}

fn raw_html<'a>(s: &str) -> Event<'a> {
    Event::Html(CowStr::from(s.to_string()))
}

/// Nhãn đậm mở đầu đoạn ("**Đáp án:**"): loại và vị trí sự kiện ngay sau nhãn, đã bỏ ":" và khoảng trắng theo sau.
fn reveal_label(events: &mut [Event<'_>], start: usize) -> Option<(Reveal, usize)> {
    if !matches!(events.get(start), Some(Event::Start(Tag::Strong))) {
        return None;
    }
    let (mut label, mut j) = (String::new(), start + 1);
    loop {
        match events.get(j)? {
            Event::Text(t) => label.push_str(t),
            Event::End(TagEnd::Strong) => break,
            _ => return None,
        }
        j += 1;
    }
    let kind = Reveal::of(&label)?;
    j += 1;
    while let Some(ev) = events.get(j) {
        match ev {
            Event::SoftBreak | Event::HardBreak => j += 1,
            Event::Text(t) => {
                let rest = t.trim_start_matches(|c: char| c == ':' || c.is_whitespace()).to_string();
                if !rest.is_empty() {
                    events[j] = Event::Text(CowStr::from(rest));
                    break;
                }
                j += 1;
            }
            _ => break,
        }
    }
    Some((kind, j))
}

/// Mục "Mục tiêu bài học" hiện thành khung; trong mục "Bài tập", đoạn mở bằng **Đáp án:** / **Gợi ý:**
/// cùng phần sau nó (tới tiêu đề kế tiếp) thành phần ẩn, bấm mới hiện (thẻ <details>, không cần JavaScript).
/// Không bọc thẻ nào quanh tiêu đề, để hình minh hoạ vẫn chèn đúng cuối mục (`minh_hoa::insert`).
fn learning_blocks(mut events: Vec<Event<'_>>) -> Vec<Event<'_>> {
    let mut out = Vec::with_capacity(events.len() + 8);
    let (mut depth, mut block, mut box_after_title) = (0usize, Block::Other, false);
    // Thẻ đang mở (khung mục tiêu, phần ẩn) — đóng trước tiêu đề kế tiếp.
    let mut open: Option<&'static str> = None;
    let mut i = 0;
    while i < events.len() {
        if depth == 0 {
            if let Event::Start(Tag::Heading { level, id, classes, .. }) = &mut events[i] {
                if let Some(close) = open.take() {
                    out.push(raw_html(close));
                }
                let lv = level_num(*level);
                if lv <= 2 {
                    let id = id.as_deref().unwrap_or("");
                    block = if OBJECTIVES_HEADING.is_match(id) {
                        Block::Objectives
                    } else if EXERCISE_HEADING.is_match(id) {
                        Block::Exercises
                    } else {
                        Block::Other
                    };
                }
                let class = match (block, lv) {
                    (Block::Objectives, 2) => Some("objectives-title"),
                    (Block::Exercises, 2) => Some("exercises-title"),
                    (Block::Exercises, 3) => Some("exercise-title"),
                    _ => None,
                };
                classes.extend(class.map(CowStr::from));
                box_after_title = block == Block::Objectives && lv == 2;
            } else if block == Block::Exercises && matches!(events[i], Event::Start(Tag::Paragraph)) {
                if let Some((kind, j)) = reveal_label(&mut events, i + 1) {
                    if let Some(close) = open.take() {
                        out.push(raw_html(close));
                    }
                    let (class, summary) = match kind {
                        Reveal::Answer => ("answer", "Xem đáp án"),
                        Reveal::Hint => ("hint", "Xem gợi ý"),
                    };
                    out.push(raw_html(&format!("<details class=\"reveal reveal--{class}\"><summary>{summary}</summary>")));
                    open = Some("</details>");
                    if matches!(events.get(j), Some(Event::End(TagEnd::Paragraph))) {
                        i = j + 1; // đoạn chỉ có nhãn
                    } else {
                        out.push(Event::Start(Tag::Paragraph));
                        depth += 1;
                        i = j;
                    }
                    continue;
                }
            }
        }
        let title_ends = box_after_title && depth == 1 && matches!(events[i], Event::End(TagEnd::Heading(_)));
        match &events[i] {
            Event::Start(_) => depth += 1,
            Event::End(_) => depth = depth.saturating_sub(1),
            _ => {}
        }
        out.push(std::mem::replace(&mut events[i], Event::SoftBreak));
        i += 1;
        if title_ends {
            out.push(raw_html("<div class=\"objectives\">"));
            open = Some("</div>");
            box_after_title = false;
        }
    }
    if let Some(close) = open {
        out.push(raw_html(close));
    }
    out
}

/// Render markdown của ghi chú/bài viết sang HTML.
/// `resolve(đích, heading)` trả về đích liên kết; `gap_href(đích)` cho liên kết tới ghi chú chưa có.
pub fn render_html(
    markdown: &str,
    resolve: &dyn Fn(&str, Option<&str>) -> Option<LinkTarget>,
    gap_href: &dyn Fn(&str) -> String,
) -> String {
    let ids: Vec<String> = all_headings(markdown).into_iter().map(|h| h.id).collect();
    // Đổi wikilink (ngoài code) thành thẻ <a> trước khi parse.
    let pre = map_outside_code(markdown, |text| {
        WIKILINK
            .replace_all(text, |c: &regex::Captures| {
                let target = c[1].trim();
                let heading = c.get(2).map(|m| m.as_str().trim());
                let label = c.get(3).map(|m| m.as_str().trim());
                match resolve(target, heading) {
                    Some(t) => format!(
                        "<a class=\"wikilink\" href=\"{}\" title=\"{}\">{}</a>",
                        escape_html(&t.href),
                        escape_html(&t.tooltip),
                        escape_html(label.unwrap_or(&t.title))
                    ),
                    None => format!(
                        "<a class=\"wikilink wikilink--broken\" href=\"{}\" title=\"Chưa có ghi chú này — cần nghiên cứu thêm\">{}</a>",
                        escape_html(&gap_href(target)),
                        escape_html(label.unwrap_or(target))
                    ),
                }
            })
            .to_string()
    });

    let mut hidx = 0;
    let mut link_stack: Vec<bool> = Vec::new();
    // Khối ```nen được vẽ thành hình SVG (xem `nen.rs`).
    let mut nen_buf: Option<String> = None;
    let mut events: Vec<Event> = Vec::new();
    for ev in Parser::new_ext(&pre, options()) {
        if let Some(buf) = nen_buf.as_mut() {
            match ev {
                Event::Text(t) => buf.push_str(&t),
                Event::End(TagEnd::CodeBlock) => {
                    let src = nen_buf.take().unwrap_or_default();
                    let html = crate::nen::render(&src).unwrap_or_else(|e| crate::nen::error_box(&e));
                    events.push(Event::Html(CowStr::from(html)));
                }
                _ => {}
            }
            continue;
        }
        if let Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) = &ev {
            if lang.trim() == "nen" {
                nen_buf = Some(String::new());
                continue;
            }
        }
        let mapped = match ev {
            Event::Start(Tag::Heading { level, classes, attrs, .. }) => {
                let id = ids.get(hidx).cloned().unwrap_or_default();
                hidx += 1;
                Event::Start(Tag::Heading { level, id: Some(CowStr::from(id)), classes, attrs })
            }
            Event::Start(Tag::Link { link_type, dest_url, title, id }) => {
                let external = dest_url.starts_with("http://") || dest_url.starts_with("https://");
                link_stack.push(external);
                if external {
                    let t = if title.is_empty() { String::new() } else { format!(" title=\"{}\"", escape_html(&title)) };
                    Event::Html(CowStr::from(format!(
                        "<a href=\"{}\"{} target=\"_blank\" rel=\"noopener noreferrer\">",
                        escape_html(&dest_url),
                        t
                    )))
                } else {
                    Event::Start(Tag::Link { link_type, dest_url, title, id })
                }
            }
            Event::End(TagEnd::Link) => {
                if link_stack.pop().unwrap_or(false) {
                    Event::Html(CowStr::from("</a>"))
                } else {
                    Event::End(TagEnd::Link)
                }
            }
            other => other,
        };
        events.push(mapped);
    }
    let mut out = String::new();
    html::push_html(&mut out, learning_blocks(events).into_iter());
    out
}

static COMMENT_LINE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)<!--.*?-->\n?").unwrap());

/// Đổi wikilink thành link Markdown chuẩn để AI agent (không hiểu cú pháp Obsidian) đi theo được.
pub fn to_portable_markdown(markdown: &str, resolve: &dyn Fn(&str) -> Option<(String, String)>) -> String {
    let without = COMMENT_LINE.replace_all(markdown, "");
    map_outside_code(&without, |text| {
        WIKILINK
            .replace_all(text, |c: &regex::Captures| {
                let target = c[1].trim();
                let heading = c.get(2).map(|m| m.as_str().trim());
                let label = c.get(3).map(|m| m.as_str().trim());
                match resolve(target) {
                    Some((url, title)) => {
                        let anchor = heading.map(|h| format!("#{}", slugify(h))).unwrap_or_default();
                        format!("[{}]({url}{anchor})", label.unwrap_or(&title))
                    }
                    None => label.unwrap_or(target).to_string(),
                }
            })
            .to_string()
    })
    .trim()
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn faq_and_headings() {
        let md = "Mở đầu\n\n## Định nghĩa\n\nX là Y.\n\n## Câu hỏi thường gặp\n\n### A là gì?\n\nA là [[b|bê]].\n\n### C?\n\n- một\n- hai\n\n## Liên quan\n\n- x\n";
        let h = extract_headings(md);
        assert_eq!(h[0].id, "dinh-nghia");
        let f = extract_faq(md);
        assert_eq!(f.len(), 2);
        assert_eq!(f[0].question, "A là gì?");
        assert_eq!(f[0].answer, "A là bê.");
        assert_eq!(f[1].answer, "một hai");
    }

    #[test]
    fn render_links() {
        let html = render_html(
            "## Tiêu đề\n\nXem [[a]] và [[chua-co]], [ngoài](https://x.y).",
            &|t, _| (t == "a").then(|| LinkTarget { href: "/notes/a/".into(), title: "A".into(), tooltip: "tóm tắt".into() }),
            &|t| format!("/gaps/#{t}"),
        );
        assert!(html.contains("<h2 id=\"tieu-de\">"));
        assert!(html.contains("href=\"/notes/a/\""));
        assert!(html.contains("wikilink--broken"));
        assert!(html.contains("target=\"_blank\""));
    }

    #[test]
    fn bai_tap_dap_an_an_va_muc_tieu() {
        let md = "Mở đầu.\n\n## Mục tiêu bài học\n\n- Nêu được X\n\n## Giải thích\n\nY.\n\n## Bài tập\n\nLàm thử.\n\n\
            ### Câu 1: X là gì?\n\nĐề bài.\n\n**Gợi ý:** nhìn lại mục Giải thích.\n\n**Đáp án:** X là Y.\n\nThêm ý.\n\n\
            ### Câu 2: Chưa có đáp án\n\nĐề.\n\n### Câu 3\n\n**Đáp án**:\nDòng sau.\n\n```\n### không phải câu\n**Đáp án:** trong code\n```\n\n\
            ## Câu hỏi thường gặp\n\n### Hỏi?\n\n**Đáp án:** không ẩn ngoài mục Bài tập.\n";
        let ex = extract_exercises(md);
        assert_eq!(ex.iter().map(|e| (e.question.as_str(), e.has_answer)).collect::<Vec<_>>(),
            [("Câu 1: X là gì?", true), ("Câu 2: Chưa có đáp án", false), ("Câu 3", true)]);
        assert_eq!(exercises_id(md).as_deref(), Some("bai-tap"));
        assert!(has_objectives(md) && !has_objectives("## Mục đích\n\nX"));

        // So khớp không tính xuống dòng giữa các thẻ.
        let html = render_html(md, &|_, _| None, &|t| t.to_string()).replace('\n', "");
        assert!(html.contains("<h2 id=\"muc-tieu-bai-hoc\" class=\"objectives-title\">"), "{html}");
        assert!(html.contains("<div class=\"objectives\"><ul>"), "{html}");
        assert!(html.contains("</ul></div><h2 id=\"giai-thich\">"), "{html}");
        assert!(html.contains("<h3 id=\"cau-1-x-la-gi\" class=\"exercise-title\">"));
        // Gợi ý và đáp án là hai phần ẩn riêng; nhãn được bỏ, đáp án kéo tới câu kế tiếp.
        assert!(html.contains("<details class=\"reveal reveal--hint\"><summary>Xem gợi ý</summary><p>nhìn lại mục Giải thích.</p></details>"), "{html}");
        assert!(html.contains("<summary>Xem đáp án</summary><p>X là Y.</p><p>Thêm ý.</p></details><h3"), "{html}");
        assert!(html.contains("<summary>Xem đáp án</summary><p>Dòng sau.</p>"), "{html}");
        assert!(html.contains("<code>### không phải câu**Đáp án:** trong code"));
        assert!(html.contains("<p><strong>Đáp án:</strong> không ẩn ngoài mục Bài tập.</p>"), "{html}");
        assert_eq!(html.matches("<details").count(), html.matches("</details>").count());
        assert_eq!(html.matches("<details").count(), 3);
    }
}
