//! Hình minh hoạ cho trang web — thư mục `minh-hoa/`, tách khỏi kho tri thức.
//!
//! Mỗi file `minh-hoa/<slug>.md` gắn với ghi chú (hoặc bài viết) cùng slug. Trong file, mỗi mục
//! `## <tiêu đề một mục trong bài>` chứa nội dung (thường là khối ```nen, xem `nen.rs`) được chèn
//! vào **cuối mục đó** khi dựng trang web. Bản tri thức (file .md của ghi chú, llms.txt,
//! knowledge.json, notes/<slug>.md) không chứa hình.

use crate::articles::Articles;
use crate::kb::{FileEntry, Issue, Kb, Level};
use crate::render::all_headings;
use crate::text::slugify;

pub const DIR: &str = "minh-hoa";

pub struct Section {
    pub heading: String,
    pub id: String,
    pub markdown: String,
}

pub struct FigureFile {
    pub slug: String,
    pub sections: Vec<Section>,
}

#[derive(Default)]
pub struct Figures {
    pub files: Vec<FigureFile>,
    pub issues: Vec<Issue>,
}

/// Tách file thành các mục `## …` (bỏ qua phần trước mục đầu tiên và dòng `##` nằm trong khối code).
fn split_sections(raw: &str) -> Vec<Section> {
    let mut out: Vec<Section> = Vec::new();
    let mut in_code = false;
    for line in raw.lines() {
        if line.trim_start().starts_with("```") {
            in_code = !in_code;
        }
        if !in_code {
            if let Some(h) = line.strip_prefix("## ") {
                let heading = h.trim().to_string();
                out.push(Section { id: slugify(&heading), heading, markdown: String::new() });
                continue;
            }
        }
        if let Some(s) = out.last_mut() {
            s.markdown.push_str(line);
            s.markdown.push('\n');
        }
    }
    out
}

/// Các khối ```nen trong một đoạn Markdown.
fn nen_blocks(md: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur: Option<String> = None;
    for line in md.lines() {
        let t = line.trim();
        match &mut cur {
            None if t.starts_with("```") && t.trim_start_matches('`').trim() == "nen" => cur = Some(String::new()),
            Some(_) if t.starts_with("```") => out.push(cur.take().unwrap_or_default()),
            Some(buf) => {
                buf.push_str(line);
                buf.push('\n');
            }
            None => {}
        }
    }
    out
}

impl Figures {
    pub fn build(files: &[FileEntry], kb: &Kb, articles: &Articles) -> Figures {
        let mut figs = Figures::default();
        for f in files {
            let slug = f.path.rsplit('/').next().unwrap_or("").trim_end_matches(".md").to_string();
            let sections = split_sections(&f.raw);
            let target: Option<(&str, bool)> = kb
                .get(&slug)
                .map(|n| (n.body.as_str(), n.inbox))
                .or_else(|| articles.get(&slug).map(|a| (a.body.as_str(), !a.is_published())));
            let draft = target.is_none_or(|(_, d)| d);
            let mut issue = |level: Level, message: String| figs.issues.push(Issue { path: f.path.clone(), level, message, draft });
            match target {
                None => issue(Level::Warning, format!("không có ghi chú hay bài viết nào tên \"{slug}\" — hình trong file này sẽ không hiện")),
                Some((body, _)) => {
                    let ids: Vec<String> = all_headings(body).into_iter().map(|h| h.id).collect();
                    for s in &sections {
                        if !ids.contains(&s.id) {
                            issue(Level::Warning, format!("không tìm thấy mục \"{}\" trong bài \"{slug}\" — hình của mục này sẽ không hiện", s.heading));
                        }
                    }
                }
            }
            for s in &sections {
                for block in nen_blocks(&s.markdown) {
                    if let Err(e) = crate::nen::render(&block) {
                        issue(Level::Error, format!("hình nến ở mục \"{}\" viết sai: {e}", s.heading));
                    }
                }
            }
            figs.files.push(FigureFile { slug, sections });
        }
        figs
    }

    pub fn for_slug(&self, slug: &str) -> Option<&FigureFile> {
        self.files.iter().find(|f| f.slug == slug)
    }
}

/// Chèn HTML của từng mục hình vào cuối mục tương ứng trong HTML của bài
/// (trước heading kế tiếp cùng cấp hoặc cấp cao hơn). `render` dựng HTML cho Markdown của mục.
pub fn insert(html: &str, file: &FigureFile, render: &dyn Fn(&str) -> String) -> String {
    let mut out = html.to_string();
    for s in &file.sections {
        let Some((level, start)) = (1..=6).find_map(|lv| out.find(&format!("<h{lv} id=\"{}\"", s.id)).map(|p| (lv, p))) else { continue };
        let after = start + out[start..].find('>').map_or(0, |i| i + 1);
        let end = (1..=level)
            .filter_map(|lv| out[after..].find(&format!("<h{lv}")).map(|p| after + p))
            .min()
            .unwrap_or(out.len());
        let block = format!("<div class=\"hinh-nhom\">{}</div>\n", render(&s.markdown));
        out.insert_str(end, &block);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hinh_chi_chen_vao_trang_web() {
        let note = FileEntry {
            path: "knowledge/_inbox/nen.md".into(),
            raw: "---\ntitle: Nến\ntype: concept\nsummary: Mẫu.\nstatus: draft\ncreated: 2026-01-01\n---\nNến là gì.\n\n## Cấu tạo\n\nThân và bóng.\n\n### Chi tiết\n\nX.\n\n## Khác\n\nY.\n".into(),
        };
        let kb = Kb::build(&[note], true);
        let articles = Articles::build(&[], &kb, true);
        let fig = FileEntry {
            path: "minh-hoa/nen.md".into(),
            raw: "# Hình cho nen\n\n## Cấu tạo\n\n```nen\nten: Một nến\n100 108 97 105\n```\n\n## Không có\n\n```nen\n1 2\n```\n".into(),
        };
        let figs = Figures::build(&[fig], &kb, &articles);
        assert_eq!(figs.issues.len(), 2, "{:?}", figs.issues.iter().map(|i| &i.message).collect::<Vec<_>>());
        assert!(figs.issues.iter().any(|i| i.message.contains("không tìm thấy mục \"Không có\"")));
        assert!(figs.issues.iter().any(|i| i.level == Level::Error && i.message.contains("4 số")));
        let html = "<p>Nến là gì.</p>\n<h2 id=\"cau-tao\">Cấu tạo</h2>\n<p>Thân và bóng.</p>\n<h3 id=\"chi-tiet\">Chi tiết</h3>\n<p>X.</p>\n<h2 id=\"khac\">Khác</h2>\n<p>Y.</p>\n";
        let out = insert(html, figs.for_slug("nen").unwrap(), &|md| crate::render::render_html(md, &|_, _| None, &|t| t.to_string()));
        let (fig_at, sub_at, next_at) = (out.find("hinh-nen").unwrap(), out.find("id=\"chi-tiet\"").unwrap(), out.find("id=\"khac\"").unwrap());
        // Hình nằm cuối mục "Cấu tạo" (sau cả mục con), trước mục kế tiếp cùng cấp.
        assert!(sub_at < fig_at && fig_at < next_at);
    }
}
