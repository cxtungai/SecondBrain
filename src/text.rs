//! Tiện ích xử lý văn bản: bỏ dấu, slug, wikilink, markdown → chữ thô.

use regex::Regex;
use std::sync::LazyLock;
use unicode_normalization::UnicodeNormalization;

fn is_combining(c: char) -> bool {
    ('\u{300}'..='\u{36f}').contains(&c)
}

/// Bỏ dấu tiếng Việt: "Đà Nẵng" -> "Da Nang".
pub fn strip_diacritics(input: &str) -> String {
    input
        .nfd()
        .filter(|c| !is_combining(*c))
        .map(|c| match c {
            'đ' => 'd',
            'Đ' => 'D',
            _ => c,
        })
        .collect()
}

/// Bỏ dấu một ký tự (dùng khi cần giữ vị trí 1–1).
pub fn fold_char(c: char) -> char {
    match c {
        'đ' => 'd',
        'Đ' => 'D',
        _ => c.nfd().find(|x| !is_combining(*x)).unwrap_or(c),
    }
}

/// "Khái niệm Second Brain" -> "khai-niem-second-brain"
pub fn slugify(input: &str) -> String {
    let mut out = String::new();
    let mut dash = false;
    for c in strip_diacritics(input).to_lowercase().chars() {
        if c.is_ascii_alphanumeric() {
            if dash && !out.is_empty() {
                out.push('-');
            }
            dash = false;
            out.push(c);
        } else {
            dash = true;
        }
    }
    out
}

/// Chuẩn hoá để tìm kiếm: bỏ dấu, chữ thường, gộp khoảng trắng.
pub fn normalize_for_search(input: &str) -> String {
    strip_diacritics(input)
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

static KW_PUNCT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"[?!.,:;"'“”()]"#).unwrap());
static KW_QUESTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(nghia la gi|la gi|la sao|what is|what are|what s)\b").unwrap());

/// Lõi của từ khoá để so trùng: "Price action là gì?" -> "price action"
pub fn keyword_core(input: &str) -> String {
    let s = normalize_for_search(input);
    let s = KW_PUNCT.replace_all(&s, " ");
    let s = KW_QUESTION.replace_all(&s, " ");
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// [[đích]] [[đích|nhãn]] [[đích#tiêu-đề]] — nhóm 1 = đích, 2 = heading, 3 = nhãn.
pub static WIKILINK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[\[([^\[\]|#\n]+)(?:#([^\[\]|\n]+))?(?:\|([^\[\]\n]+))?\]\]").unwrap());

static CODE_OR_COMMENT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)(^```[\s\S]*?^```[^\n]*$|^~~~[\s\S]*?^~~~[^\n]*$|`[^`\n]+`|<!--[\s\S]*?-->)").unwrap()
});

/// Gọi `f` cho từng đoạn văn bản nằm NGOÀI code block, inline code và chú thích <!-- -->.
pub fn map_outside_code(markdown: &str, mut f: impl FnMut(&str) -> String) -> String {
    let mut out = String::with_capacity(markdown.len());
    let mut last = 0;
    for m in CODE_OR_COMMENT.find_iter(markdown) {
        out.push_str(&f(&markdown[last..m.start()]));
        out.push_str(m.as_str());
        last = m.end();
    }
    out.push_str(&f(&markdown[last..]));
    out
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Wikilink {
    pub target: String,
    pub heading: Option<String>,
    pub label: Option<String>,
}

pub fn extract_wikilinks(markdown: &str) -> Vec<Wikilink> {
    let mut found = Vec::new();
    map_outside_code(markdown, |text| {
        for c in WIKILINK.captures_iter(text) {
            found.push(Wikilink {
                target: c[1].trim().to_string(),
                heading: c.get(2).map(|m| m.as_str().trim().to_string()),
                label: c.get(3).map(|m| m.as_str().trim().to_string()),
            });
        }
        text.to_string()
    });
    found
}

static RE_FENCE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?ms)^```.*?^```").unwrap());
static RE_COMMENT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)<!--.*?-->").unwrap());
static RE_IMG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"!\[([^\]]*)\]\([^)]*\)").unwrap());
static RE_LINK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[([^\]]*)\]\([^)]*\)").unwrap());
static RE_TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<[^>]+>").unwrap());
static RE_MARKS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[`*_>#~|]+").unwrap());
static RE_BULLET: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^\s*[-+]\s+").unwrap());

/// Markdown → văn bản thô (tìm kiếm, đếm từ, FAQ).
pub fn markdown_to_plain(markdown: &str) -> String {
    let s = RE_FENCE.replace_all(markdown, " ");
    let s = RE_COMMENT.replace_all(&s, " ");
    let s = WIKILINK.replace_all(&s, |c: &regex::Captures| {
        c.get(3).map(|m| m.as_str()).unwrap_or(&c[1]).to_string()
    });
    let s = RE_IMG.replace_all(&s, "$1");
    let s = RE_LINK.replace_all(&s, "$1");
    let s = RE_TAG.replace_all(&s, " ");
    let s = RE_MARKS.replace_all(&s, " ");
    let s = RE_BULLET.replace_all(&s, " ");
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn count_words(markdown: &str) -> usize {
    markdown_to_plain(markdown).split_whitespace().count()
}

pub fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

/// "2026-11-25" -> "25/11/2026"
pub fn format_date(iso: &str) -> String {
    let parts: Vec<&str> = iso.split('-').collect();
    if parts.len() == 3 {
        format!("{}/{}/{}", parts[2], parts[1], parts[0])
    } else {
        iso.to_string()
    }
}

pub fn is_iso_date(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 10
        && b.iter().enumerate().all(|(i, c)| if i == 4 || i == 7 { *c == b'-' } else { c.is_ascii_digit() })
}

/// So sánh chuỗi tiếng Việt gần đúng theo bảng chữ cái (bỏ dấu, rồi tới bản gốc).
pub fn vi_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    normalize_for_search(a).cmp(&normalize_for_search(b)).then_with(|| a.cmp(b))
}

pub fn today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugs() {
        assert_eq!(slugify("Khái niệm Second Brain"), "khai-niem-second-brain");
        assert_eq!(slugify("Đà Nẵng — 2026!"), "da-nang-2026");
        assert_eq!(keyword_core("Price action là gì?"), "price action");
    }

    #[test]
    fn wikilinks_outside_code() {
        let md = "a [[x|nhãn]] `[[y]]`\n```\n[[z]]\n```\n<!-- [[w]] --> [[v#h]]";
        let links: Vec<_> = extract_wikilinks(md).into_iter().map(|l| l.target).collect();
        assert_eq!(links, vec!["x", "v"]);
    }
}
