//! Lớp "Bài viết": bài SEO/GEO kéo traffic, DẪN XUẤT từ ghi chú tri thức.
//! Tri thức là nguồn sự thật — bài viết chỉ trình bày lại; mọi dữ kiện lấy từ ghi chú gốc (based_on).

use crate::kb::{FileEntry, Issue, Kb, Level};
use crate::schema;
use crate::text::{count_words, extract_wikilinks, keyword_core, slugify, vi_cmp};
use serde::Serialize;
use std::collections::HashMap;

pub const ARTICLES_DIR: &str = "articles";

/// Ý định tìm kiếm: (khoá, nhãn, mô tả).
pub const INTENTS: &[(&str, &str, &str)] = &[
    ("guide", "Cẩm nang", "Bài tổng quan cho người mới, dẫn qua nhiều ghi chú của một chủ đề."),
    ("how-to", "Hướng dẫn", "Các bước làm một việc cụ thể."),
    ("compare", "So sánh", "X và Y khác nhau thế nào, khi nào dùng cái nào."),
    ("list", "Tổng hợp", "Danh sách: sai lầm thường gặp, mẹo, ví dụ, câu hỏi…"),
    ("explain", "Giải thích", "Giải thích dễ hiểu một vấn đề — tránh trùng truy vấn định nghĩa mà ghi chú gốc đã trả lời."),
];

pub fn intent_label(key: &str) -> &'static str {
    INTENTS.iter().find(|i| i.0 == key).map(|i| i.1).unwrap_or("Bài viết")
}

pub const ARTICLE_FIELDS: &[&str] =
    &["title", "description", "keyword", "keywords", "intent", "based_on", "status", "created", "updated", "published_on"];

#[derive(Debug, Clone, Serialize)]
pub struct Article {
    pub slug: String,
    pub path: String,
    pub title: String,
    pub description: String,
    pub keyword: String,
    pub keywords: Vec<String>,
    pub intent: String,
    pub based_on: Vec<String>,
    pub status: String,
    pub created: String,
    pub updated: String,
    pub published_on: Option<String>,
    #[serde(skip)]
    pub body: String,
    pub word_count: usize,
    /// Slug các ghi chú gốc đã tìm thấy.
    pub notes: Vec<String>,
    /// Slug ghi chú được [[liên kết]] trong bài.
    pub links: Vec<String>,
    /// Slug bài viết khác được [[liên kết]].
    pub article_links: Vec<String>,
    pub broken_links: Vec<String>,
}

impl Article {
    pub fn is_published(&self) -> bool {
        self.status == "published"
    }
}

/// articles/**.md, bỏ README và thư mục/tên bắt đầu bằng "_" hoặc ".".
pub fn is_article_path(path: &str) -> bool {
    let segments: Vec<&str> = path.split('/').collect();
    let name = segments.last().copied().unwrap_or("").to_lowercase();
    segments.len() >= 2
        && segments[0] == ARTICLES_DIR
        && name.ends_with(".md")
        && name != "readme.md"
        && !segments[1..].iter().any(|s| s.starts_with('_') || s.starts_with('.'))
}

pub fn parse_article(path: &str, raw: &str) -> Result<(Article, Vec<String>), Vec<String>> {
    let slug = path.rsplit('/').next().unwrap_or(path).trim_end_matches(".md").to_string();
    let (yaml, body) = schema::split_frontmatter(raw);
    let Some(yaml) = yaml else {
        return Err(vec!["Không có frontmatter (khối --- ... --- ở đầu file)".into()]);
    };
    let map = schema::parse_yaml_map(yaml).map_err(|e| vec![e])?;
    let mut errors = Vec::new();
    let title = schema::get_str(&map, "title");
    if title.is_none() {
        errors.push("title: Thiếu title".to_string());
    }
    let description = schema::get_str(&map, "description");
    if description.is_none() {
        errors.push("description: Thiếu description (mô tả hiện trên Google, 1–2 câu)".into());
    }
    let keyword = schema::get_str(&map, "keyword");
    if keyword.is_none() {
        errors.push("keyword: Thiếu keyword (từ khoá chính của bài)".into());
    }
    let intent = schema::get_str(&map, "intent").unwrap_or_else(|| "guide".into());
    if !INTENTS.iter().any(|i| i.0 == intent) {
        let keys: Vec<&str> = INTENTS.iter().map(|i| i.0).collect();
        errors.push(format!("intent: intent phải là một trong: {}", keys.join(", ")));
    }
    let based_on = schema::string_list(schema::get(&map, "based_on"));
    if based_on.is_empty() {
        errors.push("based_on: based_on cần ít nhất 1 ghi chú tri thức gốc (slug)".into());
    }
    let status = schema::get_str(&map, "status").unwrap_or_else(|| "draft".into());
    if status != "draft" && status != "published" {
        errors.push("status: status phải là một trong: draft, published".into());
    }
    let created = schema::date_field(&map, "created", true, &mut errors);
    let updated = schema::date_field(&map, "updated", false, &mut errors);
    let published_on = schema::date_field(&map, "published_on", false, &mut errors);
    if !errors.is_empty() {
        return Err(errors);
    }
    let mut warnings = Vec::new();
    for k in map.keys().filter_map(|k| k.as_str()) {
        if !ARTICLE_FIELDS.contains(&k) {
            warnings.push(format!("Trường lạ \"{k}\" (gõ nhầm?). Trường hợp lệ: {}", ARTICLE_FIELDS.join(", ")));
        }
    }
    if slug != slugify(&slug) {
        warnings.push(format!("Tên file nên là slug không dấu, chữ thường, nối bằng \"-\" (gợi ý: {}.md)", slugify(&slug)));
    }
    let body = body.trim_start_matches(['\n', '\r', ' ', '\t']).to_string();
    let created = created.unwrap();
    Ok((
        Article {
            slug,
            path: path.to_string(),
            title: title.unwrap(),
            description: description.unwrap(),
            keyword: keyword.unwrap(),
            keywords: schema::string_list(schema::get(&map, "keywords")),
            intent,
            based_on,
            status,
            updated: updated.unwrap_or_else(|| created.clone()),
            created,
            published_on,
            word_count: count_words(&body),
            body,
            notes: vec![],
            links: vec![],
            article_links: vec![],
            broken_links: vec![],
        },
        warnings,
    ))
}

pub struct Articles {
    /// Bài được dựng trang: đã đăng (+ nháp khi include_drafts), đã đăng đứng trước.
    pub all: Vec<Article>,
    pub issues: Vec<Issue>,
    by_slug: HashMap<String, usize>,
}

impl Articles {
    pub fn build(files: &[FileEntry], kb: &Kb, include_drafts: bool) -> Articles {
        let mut issues: Vec<Issue> = Vec::new();
        let mut all: Vec<Article> = Vec::new();
        let mut by_slug: HashMap<String, usize> = HashMap::new();
        let add = |issues: &mut Vec<Issue>, path: &str, level, message: String, draft: bool| {
            issues.push(Issue { path: path.to_string(), level, message, draft })
        };

        let mut files: Vec<&FileEntry> = files.iter().filter(|f| is_article_path(&f.path)).collect();
        files.sort_by(|a, b| a.path.cmp(&b.path));
        for f in files {
            match parse_article(&f.path, &f.raw) {
                Err(errs) => {
                    let looks_published = f.raw.lines().any(|l| {
                        let l = l.trim_start();
                        l.starts_with("status:") && l.contains("published")
                    });
                    for e in errs {
                        add(&mut issues, &f.path, Level::Error, e, !looks_published);
                    }
                }
                Ok((a, warnings)) => {
                    let draft = !a.is_published();
                    for w in warnings {
                        add(&mut issues, &f.path, Level::Warning, w, draft);
                    }
                    if let Some(&i) = by_slug.get(&a.slug) {
                        let msg = format!("Trùng tên file \"{}.md\" với {}", a.slug, all[i].path);
                        add(&mut issues, &f.path, Level::Error, msg, draft);
                        continue;
                    }
                    by_slug.insert(a.slug.clone(), all.len());
                    all.push(a);
                }
            }
        }

        let by_title: HashMap<String, usize> = all.iter().enumerate().map(|(i, a)| (slugify(&a.title), i)).collect();
        let resolve_article = |t: &str| -> Option<usize> {
            let t = t.trim();
            by_slug.get(t).or_else(|| by_slug.get(&slugify(t))).or_else(|| by_title.get(&slugify(t))).copied()
        };

        for i in 0..all.len() {
            let draft = !all[i].is_published();
            let path = all[i].path.clone();
            let mut notes: Vec<String> = Vec::new();
            for r in all[i].based_on.clone() {
                match kb.resolve(&r) {
                    None => add(&mut issues, &path, Level::Error, format!("based_on: không có ghi chú \"{r}\" trong kho — nghiên cứu (/research) và đăng ghi chú trước"), draft),
                    Some(n) => {
                        if n.inbox {
                            let level = if draft { Level::Warning } else { Level::Error };
                            add(&mut issues, &path, level, format!("based_on: \"{}\" còn chờ duyệt trong _inbox — đăng ghi chú trước khi đăng bài", n.title), draft);
                        } else if n.status != "verified" {
                            add(&mut issues, &path, Level::Warning, format!("Ghi chú gốc \"{}\" chưa kiểm chứng (status: {}) — duyệt tri thức trước khi đăng bài", n.title, n.status), draft);
                        }
                        if !notes.contains(&n.slug) {
                            notes.push(n.slug.clone());
                        }
                    }
                }
            }
            let (mut links, mut alinks, mut broken) = (Vec::new(), Vec::new(), Vec::new());
            for l in extract_wikilinks(&all[i].body) {
                if let Some(n) = kb.resolve(&l.target) {
                    if !links.contains(&n.slug) {
                        links.push(n.slug.clone());
                    }
                } else if let Some(j) = resolve_article(&l.target) {
                    if j != i && !alinks.contains(&all[j].slug) {
                        alinks.push(all[j].slug.clone());
                    }
                } else if !broken.contains(&l.target) {
                    broken.push(l.target.clone());
                }
            }
            if !broken.is_empty() {
                let list: Vec<String> = broken.iter().map(|b| format!("[[{b}]]")).collect();
                add(&mut issues, &path, Level::Warning, format!("Liên kết tới tri thức chưa có: {} — bài viết chỉ nên dẫn tới ghi chú đã có", list.join(", ")), draft);
            }
            if !notes.is_empty() && !notes.iter().any(|n| links.contains(n)) {
                add(&mut issues, &path, Level::Warning, "Chưa đặt [[liên kết]] nào tới ghi chú gốc trong nội dung — dẫn người đọc (và Google) về tri thức gốc".into(), draft);
            }
            let a = &mut all[i];
            a.notes = notes;
            a.links = links;
            a.article_links = alinks;
            a.broken_links = broken;
        }

        // Tránh "ăn thịt từ khoá": mỗi từ khoá chính chỉ một trang.
        let mut by_kw: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, a) in all.iter().enumerate() {
            by_kw.entry(keyword_core(&a.keyword)).or_default().push(i);
        }
        for group in by_kw.values().filter(|g| g.len() > 1) {
            for &i in group {
                let others: Vec<&str> = group.iter().filter(|&&j| j != i).map(|&j| all[j].slug.as_str()).collect();
                let msg = format!("Trùng từ khoá chính \"{}\" với: {} — gộp bài hoặc đổi góc (intent) để không tự cạnh tranh trên Google", all[i].keyword, others.join(", "));
                add(&mut issues, &all[i].path, Level::Warning, msg, !all[i].is_published());
            }
        }
        let mut note_names: HashMap<String, (String, String)> = HashMap::new();
        for n in kb.notes() {
            for name in std::iter::once(&n.title).chain(n.aliases.iter()) {
                note_names.entry(keyword_core(name)).or_insert((n.slug.clone(), n.title.clone()));
            }
        }
        for a in &all {
            if let Some((_, title)) = note_names.get(&keyword_core(&a.keyword)) {
                let msg = format!("Từ khoá \"{}\" trùng tên ghi chú \"{}\" — để ghi chú giữ truy vấn định nghĩa; bài viết nên nhắm truy vấn khác (hướng dẫn, so sánh, sai lầm thường gặp…)", a.keyword, title);
                add(&mut issues, &a.path, Level::Warning, msg, !a.is_published());
            }
        }

        // Thứ tự: đã đăng (mới trước), rồi nháp; bỏ nháp nếu không include_drafts.
        all.retain(|a| include_drafts || a.is_published());
        all.sort_by(|a, b| {
            b.is_published().cmp(&a.is_published()).then_with(|| b.updated.cmp(&a.updated)).then_with(|| vi_cmp(&a.title, &b.title))
        });
        let by_slug = all.iter().enumerate().map(|(i, a)| (a.slug.clone(), i)).collect();
        Articles { all, issues, by_slug }
    }

    pub fn published(&self) -> Vec<&Article> {
        self.all.iter().filter(|a| a.is_published()).collect()
    }

    pub fn drafts(&self) -> Vec<&Article> {
        self.all.iter().filter(|a| !a.is_published()).collect()
    }

    pub fn get(&self, slug: &str) -> Option<&Article> {
        self.by_slug.get(slug).map(|&i| &self.all[i])
    }

    pub fn resolve(&self, target: &str) -> Option<&Article> {
        let t = target.trim();
        self.get(t)
            .or_else(|| self.get(&slugify(t)))
            .or_else(|| self.all.iter().find(|a| slugify(&a.title) == slugify(t)))
    }

    /// Bài viết dựa trên hoặc liên kết tới một ghi chú.
    pub fn for_note(&self, note_slug: &str) -> Vec<&Article> {
        self.all.iter().filter(|a| a.notes.iter().any(|n| n == note_slug) || a.links.iter().any(|n| n == note_slug)).collect()
    }

    /// Bài viết khác có chung tri thức gốc.
    pub fn related(&self, article: &Article, limit: usize) -> Vec<&Article> {
        let mut scored: Vec<(usize, &Article)> = self
            .all
            .iter()
            .filter(|x| x.slug != article.slug)
            .map(|x| {
                let shared = x.notes.iter().filter(|n| article.notes.contains(n)).count()
                    + if article.article_links.contains(&x.slug) { 2 } else { 0 };
                (shared, x)
            })
            .filter(|(s, _)| *s > 0)
            .collect();
        scored.sort_by(|a, b| b.0.cmp(&a.0));
        scored.into_iter().take(limit).map(|(_, a)| a).collect()
    }
}
