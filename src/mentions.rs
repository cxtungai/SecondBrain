//! "Nhắc tên chưa liên kết": ghi chú A nhắc tên/alias của ghi chú B nhưng chưa có [[B]].

use crate::kb::Note;
use crate::text::{fold_char, WIKILINK};
use regex::Regex;
use serde::Serialize;
use std::sync::LazyLock;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone, Serialize)]
pub struct Mention {
    pub from: String,
    pub to: String,
    #[serde(rename = "match")]
    pub matched: String,
    pub context: String,
}

static PAREN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(.*?)\s*\(([^)]+)\)\s*$").unwrap());
static PREFIX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^(sách|bài báo|báo cáo|video|khoá học|khóa học)\s*:\s*").unwrap());
static FENCE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?ms)^```.*?^```").unwrap());
static INLINE_CODE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"`[^`\n]*`").unwrap());
static COMMENT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)<!--.*?-->").unwrap());
static MD_LINK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[[^\]]*\]\([^)]*\)").unwrap());

/// Tên để dò: tiêu đề, alias, tiêu đề bỏ phần trong ngoặc, bỏ tiền tố "Sách:"…
fn names_of(note: &Note) -> Vec<Vec<char>> {
    let mut names: Vec<String> = vec![note.title.clone()];
    names.extend(note.aliases.iter().cloned());
    if let Some(c) = PAREN.captures(&note.title) {
        names.push(c[1].to_string());
        names.push(c[2].to_string());
    }
    for n in names.clone() {
        let stripped = PREFIX.replace(&n, "").to_string();
        if stripped != n {
            names.push(stripped);
        }
    }
    let mut out: Vec<Vec<char>> = Vec::new();
    for n in names {
        let t: Vec<char> = n.trim().nfc().collect();
        if t.len() >= 3 && !out.contains(&t) {
            out.push(t);
        }
    }
    out
}

/// Thay các vùng không dò (code, link sẵn có…) bằng khoảng trắng cùng số ký tự.
fn blank(re: &Regex, s: &str) -> String {
    re.replace_all(s, |c: &regex::Captures| " ".repeat(c[0].chars().count())).to_string()
}

fn searchable_text(body: &str) -> Vec<char> {
    let s: String = body.nfc().collect();
    let s = blank(&FENCE, &s);
    let s = blank(&INLINE_CODE, &s);
    let s = blank(&COMMENT, &s);
    let s = blank(&WIKILINK, &s);
    let s = blank(&MD_LINK, &s);
    s.chars().collect()
}

fn is_word(c: Option<&char>) -> bool {
    c.is_some_and(|c| c.is_alphanumeric())
}

fn lower(c: char) -> char {
    c.to_lowercase().next().unwrap_or(c)
}

fn find_name(stripped: &[char], folded: &[char], name: &[char]) -> Option<usize> {
    let upper: String = name.iter().collect::<String>().to_uppercase();
    let acronym = name.len() <= 5 || name.iter().collect::<String>() == upper;
    let (hay, needle): (&[char], Vec<char>) = if acronym {
        (stripped, name.iter().map(|&c| fold_char(c)).collect())
    } else {
        (folded, name.iter().map(|&c| lower(fold_char(c))).collect())
    };
    if needle.is_empty() || needle.len() > hay.len() {
        return None;
    }
    let mut i = 0;
    while i + needle.len() <= hay.len() {
        if hay[i..i + needle.len()] == needle[..] {
            let before = if i == 0 { None } else { hay.get(i - 1) };
            let after = hay.get(i + needle.len());
            if !is_word(before) && !is_word(after) {
                return Some(i);
            }
            i += needle.len();
        } else {
            i += 1;
        }
    }
    None
}

pub fn find_unlinked_mentions(notes: &[Note]) -> Vec<Mention> {
    let targets: Vec<(&Note, Vec<Vec<char>>)> = notes.iter().map(|n| (n, names_of(n))).collect();
    let mut results = Vec::new();
    for note in notes {
        let text = searchable_text(&note.body);
        let stripped: Vec<char> = text.iter().map(|&c| fold_char(c)).collect();
        let folded: Vec<char> = stripped.iter().map(|&c| lower(c)).collect();
        for (target, names) in &targets {
            if std::ptr::eq(*target, note) || note.links.contains(&target.slug) {
                continue;
            }
            if target.updates.as_deref() == Some(&note.slug) || note.updates.as_deref() == Some(&target.slug) {
                continue;
            }
            for name in names {
                if let Some(at) = find_name(&stripped, &folded, name) {
                    let start = at.saturating_sub(50);
                    let end = (at + name.len() + 50).min(text.len());
                    let context: String = text[start..end].iter().collect::<String>().split_whitespace().collect::<Vec<_>>().join(" ");
                    results.push(Mention {
                        from: note.slug.clone(),
                        to: target.slug.clone(),
                        matched: text[at..at + name.len()].iter().collect(),
                        context,
                    });
                    break;
                }
            }
        }
    }
    results
}
