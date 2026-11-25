//! Kho tri thức: đọc ghi chú, kiểm tra, dựng đồ thị liên kết (links, backlinks, gaps, tags).

use crate::mentions::{find_unlinked_mentions, Mention};
use crate::schema::{self, Source, KNOWN_FIELDS};
use crate::text::{count_words, extract_wikilinks, normalize_for_search, slugify, vi_cmp};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Thư mục bản nháp do AI nghiên cứu, chờ người dùng duyệt (không lên web).
pub const INBOX_DIR: &str = "knowledge/_inbox";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Error,
    Warning,
}

#[derive(Debug, Clone, Serialize)]
pub struct Issue {
    pub path: String,
    pub level: Level,
    pub message: String,
    /// Thuộc bản nháp (lỗi không chặn build).
    pub draft: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Credibility {
    pub rated: usize,
    pub total: usize,
    pub average: Option<f64>,
    pub max: Option<u8>,
    pub min: Option<u8>,
}

pub fn credibility_summary(sources: &[Source]) -> Credibility {
    let scores: Vec<u8> = sources.iter().filter_map(|s| s.credibility).collect();
    if scores.is_empty() {
        return Credibility { total: sources.len(), ..Default::default() };
    }
    let avg = scores.iter().map(|&s| s as f64).sum::<f64>() / scores.len() as f64;
    Credibility {
        rated: scores.len(),
        total: sources.len(),
        average: Some((avg * 10.0).round() / 10.0),
        max: scores.iter().copied().max(),
        min: scores.iter().copied().min(),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Note {
    pub slug: String,
    pub path: String,
    pub inbox: bool,
    pub title: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub status: String,
    pub sources: Vec<Source>,
    pub credibility: Credibility,
    pub aliases: Vec<String>,
    pub created: String,
    pub updated: String,
    pub reviewed: Option<String>,
    pub research: Option<serde_json::Value>,
    pub extra: serde_json::Map<String, serde_json::Value>,
    #[serde(skip)]
    pub body: String,
    pub word_count: usize,
    pub links: Vec<String>,
    pub broken_links: Vec<String>,
    pub backlinks: Vec<String>,
    /// Bản nháp này cập nhật cho ghi chú đã đăng nào (slug gốc).
    pub updates: Option<String>,
}

pub fn is_inbox_path(path: &str) -> bool {
    path.starts_with(&format!("{INBOX_DIR}/"))
}

/// File nào là ghi chú: .md, không nằm trong thư mục/tên bắt đầu bằng "_" hoặc ".", bỏ README.
/// Ngoại lệ: knowledge/_inbox/ khi include_inbox.
pub fn is_note_path(path: &str, include_inbox: bool) -> bool {
    let segments: Vec<&str> = path.split('/').collect();
    let name = segments.last().copied().unwrap_or("").to_lowercase();
    if !name.ends_with(".md") || name == "readme.md" || segments.first() != Some(&"knowledge") {
        return false;
    }
    !segments.iter().enumerate().any(|(i, s)| {
        if include_inbox && i == 1 && *s == "_inbox" {
            return false;
        }
        s.starts_with('_') || s.starts_with('.')
    })
}

pub struct ParsedNote {
    pub note: Note,
    pub warnings: Vec<String>,
}

pub fn parse_note(path: &str, raw: &str) -> Result<ParsedNote, Vec<String>> {
    let name = path.rsplit('/').next().unwrap_or(path);
    let slug = name.trim_end_matches(".md").to_string();
    let inbox = is_inbox_path(path);
    let (yaml, body) = schema::split_frontmatter(raw);
    let Some(yaml) = yaml else {
        return Err(vec!["Không có frontmatter (khối --- ... --- ở đầu file)".into()]);
    };
    let map = schema::parse_yaml_map(yaml).map_err(|e| vec![e])?;
    let mut errors = Vec::new();

    let title = schema::get_str(&map, "title");
    if title.is_none() {
        errors.push("title: Thiếu title".into());
    }
    let kind = schema::get_str(&map, "type");
    match &kind {
        None => errors.push("type: Thiếu type".into()),
        Some(k) if schema::type_info(k).is_none() => {
            let keys: Vec<&str> = schema::NOTE_TYPES.iter().map(|t| t.key).collect();
            errors.push(format!("type: type phải là một trong: {}", keys.join(", ")));
        }
        _ => {}
    }
    let summary = schema::get_str(&map, "summary");
    if summary.is_none() {
        errors.push("summary: Thiếu summary (1–2 câu tóm tắt để AI chọn ghi chú)".into());
    }
    let status = schema::get_str(&map, "status").unwrap_or_else(|| "draft".into());
    if !schema::STATUSES.iter().any(|s| s.key == status) {
        errors.push("status: status phải là một trong: draft, verified, outdated".into());
    }
    let sources = schema::parse_sources(schema::get(&map, "sources"), &mut errors);
    let created = schema::date_field(&map, "created", true, &mut errors);
    let updated = schema::date_field(&map, "updated", false, &mut errors);
    let reviewed = schema::date_field(&map, "reviewed", false, &mut errors);
    let research = schema::get(&map, "research").filter(|v| !v.is_null()).map(schema::yaml_to_json);
    if !errors.is_empty() {
        return Err(errors);
    }

    let mut warnings = Vec::new();
    let mut extra = serde_json::Map::new();
    for (k, v) in &map {
        let Some(k) = k.as_str() else { continue };
        if !KNOWN_FIELDS.contains(&k) {
            extra.insert(k.to_string(), schema::yaml_to_json(v));
            warnings.push(format!("Trường lạ \"{k}\" (gõ nhầm?). Trường hợp lệ: {}", KNOWN_FIELDS.join(", ")));
        }
    }
    if slug != slugify(&slug) {
        warnings.push(format!("Tên file nên là slug không dấu, chữ thường, nối bằng \"-\" (gợi ý: {}.md)", slugify(&slug)));
    }
    let kind = kind.unwrap();
    let folder: Vec<&str> = path.split('/').collect();
    let expected = schema::type_info(&kind).unwrap().folder;
    if !inbox && folder.len() > 2 && folder[1] != expected {
        warnings.push(format!("Ghi chú type \"{kind}\" nên nằm trong knowledge/{expected}/"));
    }

    let body = body.trim_start_matches(['\n', '\r', ' ', '\t']).to_string();
    let created = created.unwrap();
    let note = Note {
        slug,
        path: path.to_string(),
        inbox,
        title: title.unwrap(),
        kind,
        summary: summary.unwrap().trim().to_string(),
        tags: schema::string_list(schema::get(&map, "tags")),
        status,
        credibility: credibility_summary(&sources),
        sources,
        aliases: schema::string_list(schema::get(&map, "aliases")),
        updated: updated.unwrap_or_else(|| created.clone()),
        created,
        reviewed,
        research,
        extra,
        word_count: count_words(&body),
        body,
        links: vec![],
        broken_links: vec![],
        backlinks: vec![],
        updates: None,
    };
    Ok(ParsedNote { note, warnings })
}

/// Kiểm tra chất lượng — để AI không trích dẫn thông tin yếu như sự thật.
fn quality_warnings(note: &Note) -> Vec<(Level, String)> {
    let mut out = Vec::new();
    let c = &note.credibility;
    if note.kind == "fact" && note.sources.is_empty() {
        out.push((Level::Error, "Ghi chú \"fact\" bắt buộc có ít nhất 1 nguồn (sources)".into()));
    }
    if note.status == "verified" && note.kind != "topic" && note.sources.is_empty() {
        out.push((Level::Warning, "Đã \"verified\" nhưng không có nguồn nào".into()));
    }
    if note.status == "verified" && c.rated < c.total {
        out.push((Level::Warning, format!("{}/{} nguồn chưa chấm điểm uy tín (credibility 1–5)", c.total - c.rated, c.total)));
    }
    if note.status == "verified" && note.kind == "fact" && matches!(c.max, Some(m) if m <= 2) {
        out.push((Level::Warning, "Dữ kiện \"verified\" nhưng chỉ dựa trên nguồn uy tín thấp (≤ 2/5) — nên tìm nguồn tốt hơn".into()));
    }
    if note.summary.chars().count() > 400 {
        out.push((Level::Warning, "summary quá dài (> 400 ký tự) — nên gói trong 1–2 câu".into()));
    }
    out
}

#[derive(Debug, Clone, Serialize)]
pub struct Tag {
    pub slug: String,
    pub name: String,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Gap {
    pub slug: String,
    pub name: String,
    pub from: Vec<String>,
}

pub struct Kb {
    pub all: Vec<Note>,
    /// Chỉ số ghi chú đã đăng, mới cập nhật trước.
    pub published: Vec<usize>,
    /// Chỉ số bản nháp chờ duyệt.
    pub inbox_idx: Vec<usize>,
    pub issues: Vec<Issue>,
    pub tags: Vec<Tag>,
    pub gaps: Vec<Gap>,
    by_slug: HashMap<String, usize>,
    by_name: HashMap<String, usize>,
    mentions: OnceLock<Vec<Mention>>,
}

pub struct FileEntry {
    pub path: String,
    pub raw: String,
}

impl Kb {
    pub fn build(files: &[FileEntry], include_inbox: bool) -> Kb {
        let mut issues = Vec::new();
        let mut all: Vec<Note> = Vec::new();
        let mut by_slug: HashMap<String, usize> = HashMap::new();

        let mut candidates: Vec<&FileEntry> = files.iter().filter(|f| is_note_path(&f.path, include_inbox)).collect();
        // Ghi chú đã đăng xử lý trước để bản nháp trùng tên (bản cập nhật) không chiếm chỗ.
        candidates.sort_by(|a, b| is_inbox_path(&a.path).cmp(&is_inbox_path(&b.path)).then_with(|| a.path.cmp(&b.path)));

        for f in candidates {
            let draft = is_inbox_path(&f.path);
            let add = |issues: &mut Vec<Issue>, level, message: String| {
                issues.push(Issue { path: f.path.clone(), level, message, draft })
            };
            let parsed = match parse_note(&f.path, &f.raw) {
                Ok(p) => p,
                Err(errs) => {
                    for e in errs {
                        add(&mut issues, Level::Error, e);
                    }
                    continue;
                }
            };
            let mut note = parsed.note;
            for w in parsed.warnings {
                add(&mut issues, Level::Warning, w);
            }
            for (level, m) in quality_warnings(&note) {
                add(&mut issues, level, m);
            }
            if let Some(&i) = by_slug.get(&note.slug) {
                let existing = &all[i];
                if note.inbox && !existing.inbox {
                    note.updates = Some(existing.slug.clone());
                    note.slug = format!("{}--nhap", note.slug);
                } else {
                    let msg = format!("Trùng tên file \"{}.md\" với {} — tên file phải duy nhất trong toàn kho", note.slug, existing.path);
                    add(&mut issues, Level::Error, msg);
                    continue;
                }
            }
            by_slug.insert(note.slug.clone(), all.len());
            all.push(note);
        }

        // Tra cứu wikilink: slug > tiêu đề > alias (không phân biệt dấu). Bản nháp không được ưu tiên.
        let mut by_name: HashMap<String, usize> = HashMap::new();
        let mut order: Vec<usize> = (0..all.len()).collect();
        order.sort_by_key(|&i| all[i].inbox);
        for i in order {
            for name in std::iter::once(&all[i].title).chain(all[i].aliases.iter()) {
                let key = slugify(name);
                if !key.is_empty() {
                    by_name.entry(key).or_insert(i);
                }
            }
        }

        let mut kb = Kb {
            all,
            published: vec![],
            inbox_idx: vec![],
            issues,
            tags: vec![],
            gaps: vec![],
            by_slug,
            by_name,
            mentions: OnceLock::new(),
        };

        // Liên kết, liên kết hỏng (= tri thức còn thiếu), backlinks.
        let mut gaps: Vec<Gap> = Vec::new();
        let mut computed: Vec<(Vec<String>, Vec<String>)> = Vec::new();
        for i in 0..kb.all.len() {
            let mut links: Vec<String> = Vec::new();
            let mut broken: Vec<String> = Vec::new();
            for link in extract_wikilinks(&kb.all[i].body) {
                match kb.resolve_idx(&link.target) {
                    None => {
                        if !broken.contains(&link.target) {
                            broken.push(link.target.clone());
                        }
                        let key = Some(slugify(&link.target)).filter(|s| !s.is_empty()).unwrap_or(link.target.clone());
                        let slug = kb.all[i].slug.clone();
                        match gaps.iter_mut().find(|g| g.slug == key) {
                            Some(g) => {
                                if !g.from.contains(&slug) {
                                    g.from.push(slug)
                                }
                            }
                            None => gaps.push(Gap { slug: key, name: link.target.clone(), from: vec![slug] }),
                        }
                    }
                    Some(t) if t != i => {
                        let s = kb.all[t].slug.clone();
                        if !links.contains(&s) {
                            links.push(s);
                        }
                    }
                    _ => {}
                }
            }
            computed.push((links, broken));
        }
        for (i, (links, broken)) in computed.into_iter().enumerate() {
            kb.all[i].links = links;
            kb.all[i].broken_links = broken;
        }
        for i in 0..kb.all.len() {
            for target in kb.all[i].links.clone() {
                let t = kb.by_slug[&target];
                let from = kb.all[i].slug.clone();
                kb.all[t].backlinks.push(from);
            }
        }

        let sort = |kb: &Kb, list: &mut Vec<usize>| {
            list.sort_by(|&a, &b| kb.all[b].updated.cmp(&kb.all[a].updated).then_with(|| vi_cmp(&kb.all[a].title, &kb.all[b].title)))
        };
        let mut published: Vec<usize> = (0..kb.all.len()).filter(|&i| !kb.all[i].inbox).collect();
        let mut inbox: Vec<usize> = (0..kb.all.len()).filter(|&i| kb.all[i].inbox).collect();
        sort(&kb, &mut published);
        sort(&kb, &mut inbox);

        // Tag: gom theo slug ("AI" và "ai" là một). Chỉ tính ghi chú đã đăng.
        let mut tags: Vec<Tag> = Vec::new();
        for &i in &published {
            for tag in &kb.all[i].tags {
                let key = Some(slugify(tag)).filter(|s| !s.is_empty()).unwrap_or_else(|| "tag".into());
                match tags.iter_mut().find(|t| t.slug == key) {
                    Some(t) => t.notes.push(kb.all[i].slug.clone()),
                    None => tags.push(Tag { slug: key, name: tag.clone(), notes: vec![kb.all[i].slug.clone()] }),
                }
            }
        }
        tags.sort_by(|a, b| b.notes.len().cmp(&a.notes.len()).then_with(|| vi_cmp(&a.name, &b.name)));
        gaps.sort_by(|a, b| b.from.len().cmp(&a.from.len()).then_with(|| vi_cmp(&a.name, &b.name)));

        kb.published = published;
        kb.inbox_idx = inbox;
        kb.tags = tags;
        kb.gaps = gaps;
        kb
    }

    fn resolve_idx(&self, target: &str) -> Option<usize> {
        let t = target.trim();
        self.by_slug
            .get(t)
            .or_else(|| self.by_slug.get(&slugify(t)))
            .or_else(|| self.by_name.get(&slugify(t)))
            .copied()
    }

    /// Tìm ghi chú theo slug, tiêu đề hoặc tên khác.
    pub fn resolve(&self, target: &str) -> Option<&Note> {
        self.resolve_idx(target).map(|i| &self.all[i])
    }

    pub fn get(&self, slug: &str) -> Option<&Note> {
        self.by_slug.get(slug).map(|&i| &self.all[i])
    }

    /// Ghi chú đã đăng (lên web, llms.txt).
    pub fn notes(&self) -> Vec<&Note> {
        self.published.iter().map(|&i| &self.all[i]).collect()
    }

    /// Bản nháp chờ duyệt.
    pub fn inbox(&self) -> Vec<&Note> {
        self.inbox_idx.iter().map(|&i| &self.all[i]).collect()
    }

    pub fn by_type(&self, kind: &str) -> Vec<&Note> {
        self.notes().into_iter().filter(|n| n.kind == kind).collect()
    }

    pub fn by_tag(&self, tag_slug: &str) -> Vec<&Note> {
        self.notes()
            .into_iter()
            .filter(|n| n.tags.iter().any(|t| Some(slugify(t)).filter(|s| !s.is_empty()).unwrap_or_else(|| "tag".into()) == tag_slug))
            .collect()
    }

    /// Ghi chú đã đăng không liên kết tới / không được liên kết từ ghi chú nào.
    pub fn orphans(&self) -> Vec<&Note> {
        self.notes().into_iter().filter(|n| n.links.is_empty() && n.backlinks.is_empty()).collect()
    }

    /// Chỗ nhắc tên ghi chú khác mà chưa đặt [[liên kết]].
    pub fn mentions(&self) -> &Vec<Mention> {
        self.mentions.get_or_init(|| find_unlinked_mentions(&self.all))
    }

    pub fn search(&self, query: &str, with_inbox: bool) -> Vec<&Note> {
        let mut list = self.notes();
        if with_inbox {
            list.extend(self.inbox());
        }
        search_notes(list, query)
    }
}

/// Tìm kiếm đơn giản: tiêu đề/tên khác > tóm tắt/tag > nội dung; mọi từ phải khớp.
pub fn search_notes<'a>(notes: Vec<&'a Note>, query: &str) -> Vec<&'a Note> {
    let q = normalize_for_search(query);
    let terms: Vec<&str> = q.split(' ').filter(|t| !t.is_empty()).collect();
    if terms.is_empty() {
        return vec![];
    }
    let mut scored: Vec<(usize, &Note)> = notes
        .into_iter()
        .filter_map(|n| {
            let title = normalize_for_search(&[vec![n.title.clone()], n.aliases.clone()].concat().join(" "));
            let meta = normalize_for_search(&[vec![n.summary.clone()], n.tags.clone()].concat().join(" "));
            let body = normalize_for_search(&n.body);
            let mut score = 0;
            for t in &terms {
                let s = if title.contains(t) { 10 } else { 0 } + if meta.contains(t) { 4 } else { 0 } + if body.contains(t) { 1 } else { 0 };
                if s == 0 {
                    return None;
                }
                score += s;
            }
            Some((score, n))
        })
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored.into_iter().map(|(_, n)| n).collect()
}
