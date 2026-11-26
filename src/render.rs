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
    html::push_html(&mut out, events.into_iter());
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
}
