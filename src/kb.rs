//! Kho tri thức: đọc ghi chú, kiểm tra, dựng đồ thị liên kết (links, backlinks, gaps, tags).

use crate::mentions::{find_unlinked_mentions, Mention};
use crate::render::{atx_heading, content_lines, extract_exercises, has_objectives};
use crate::schema::{self, Source, KNOWN_FIELDS};
use crate::text::{count_words, extract_wikilinks, markdown_to_plain, normalize_for_search, slugify, vi_cmp};
use regex::Regex;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::sync::{LazyLock, OnceLock};

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
    /// Chủ đề: lộ trình học (các cấp chủ đề dùng, tối đa 6) đọc từ mục "## Lộ trình học" (rỗng nếu chưa có).
    #[serde(skip)]
    pub learning_path: Vec<PathLevel>,
}

/// Sáu cấp của lộ trình học, theo thang nhận thức Bloom: từ biết tới tự sáng tạo.
pub const LEVELS: [&str; 6] = ["Biết", "Hiểu", "Vận dụng", "Phân tích, tổng hợp", "Đánh giá", "Sáng tạo"];

/// Tên cấp 1–6 ("" nếu ngoài khoảng).
pub fn level_name(level: usize) -> &'static str {
    level.checked_sub(1).and_then(|i| LEVELS.get(i)).copied().unwrap_or("")
}

/// Cấp (1–6) của một tiêu đề cấp 3 trong mục "Lộ trình học": "2. Hiểu — vì sao…" → 2.
pub fn level_of_heading(text: &str) -> Option<usize> {
    const KEYS: [(&str, usize); 7] =
        [("biet", 1), ("hieu", 2), ("van-dung", 3), ("phan-tich", 4), ("tong-hop", 4), ("danh-gia", 5), ("sang-tao", 6)];
    let s = slugify(text);
    let s = s.trim_start_matches(|c: char| c.is_ascii_digit() || c == '-');
    KEYS.iter().find(|(k, _)| s == *k || s.starts_with(&format!("{k}-"))).map(|&(_, l)| l)
}

/// Một cấp trong lộ trình học của chủ đề.
#[derive(Debug, Clone, Serialize)]
pub struct PathLevel {
    pub level: usize,
    /// Chữ của tiêu đề cấp 3 và id của nó trên trang.
    pub heading: String,
    pub id: String,
    /// Bài đã có (slug), theo thứ tự trong mục.
    pub notes: Vec<String>,
    /// Bài còn thiếu: liên kết tới ghi chú chưa có.
    pub gaps: Vec<String>,
}

struct RawLevel {
    level: usize,
    heading: String,
    targets: Vec<String>,
}

/// Đọc mục "## Lộ trình học" của chủ đề: mỗi "### <cấp>" cùng các [[liên kết]] bên dưới.
/// Trả về các cấp và những tiêu đề cấp 3 không khớp cấp nào.
fn parse_learning_path(body: &str) -> (Vec<RawLevel>, Vec<String>) {
    let (mut levels, mut unknown): (Vec<RawLevel>, Vec<String>) = (vec![], vec![]);
    let (mut inside, mut current) = (false, None::<usize>);
    for line in content_lines(body) {
        let t = line.trim_start();
        if let Some((hashes, text)) = atx_heading(t) {
            let text = markdown_to_plain(text);
            if hashes <= 2 {
                inside = hashes == 2 && slugify(&text).starts_with("lo-trinh");
                current = None;
            } else if inside && hashes == 3 {
                current = level_of_heading(&text).map(|level| {
                    levels.push(RawLevel { level, heading: text.clone(), targets: vec![] });
                    levels.len() - 1
                });
                if current.is_none() {
                    unknown.push(text);
                }
            }
            continue;
        }
        if let (true, Some(c)) = (inside, current) {
            levels[c].targets.extend(extract_wikilinks(line).into_iter().map(|w| w.target));
        }
    }
    (levels, unknown)
}

/// Vị trí của một bài trong lộ trình chủ đề: thứ tự, cấp, bài trước và bài tiếp.
pub struct Place<'a> {
    pub topic: &'a Note,
    pub index: usize,
    pub total: usize,
    pub level: Option<usize>,
    pub prev: Option<&'a Note>,
    pub next: Option<&'a Note>,
}

/// Cấp của một bài trong lộ trình của chủ đề (cấp đầu tiên liệt kê bài đó).
pub fn level_in(topic: &Note, slug: &str) -> Option<usize> {
    topic.learning_path.iter().find(|l| l.notes.iter().any(|s| s == slug)).map(|l| l.level)
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
        learning_path: vec![],
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
    // Cấp Biết: "X là gì" phải chuẩn nên định nghĩa dẫn tài liệu gốc (người dùng dặn 2026-11-26).
    // Kiểm lúc còn là bản nháp — khi nghiên cứu và khi duyệt.
    if note.inbox && ["topic", "concept", "framework"].contains(&note.kind.as_str()) {
        if let Some(msg) = definition_source_problem(note) {
            out.push((Level::Warning, msg));
        }
    }
    for e in extract_exercises(&note.body).iter().filter(|e| !e.has_answer) {
        out.push((Level::Warning, format!("Bài tập \"{}\" chưa có đáp án — thêm đoạn mở bằng **Đáp án:** (người đọc bấm mới hiện)", e.question)));
    }
    out
}

static CITATION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[(\d{1,2})\]").unwrap());

/// Câu định nghĩa của ghi chú: đoạn văn đầu tiên trước tiêu đề đầu tiên, và đoạn văn đầu tiên
/// của mục "## Định nghĩa" hoặc "## Mục đích" (framework). Bỏ qua đoạn trích (">"), chú thích, code.
pub fn definition_text(body: &str) -> String {
    let mut out = String::new();
    // Vùng đang đọc: mở đầu bài hoặc mục định nghĩa; `taken` = vùng này đã lấy xong đoạn đầu.
    let (mut zone, mut taken, mut started) = (true, false, false);
    for line in content_lines(body) {
        let t = line.trim_start();
        if let Some((hashes, text)) = atx_heading(t) {
            if started {
                taken = true;
            }
            if hashes <= 2 {
                let s = slugify(&markdown_to_plain(text));
                zone = s.starts_with("dinh-nghia") || s.starts_with("muc-dich");
                (taken, started) = (false, false);
            }
            continue;
        }
        if !zone || taken {
            continue;
        }
        if t.is_empty() {
            taken |= started;
            continue;
        }
        if !started && (t.starts_with('>') || t.starts_with("<!--")) {
            continue;
        }
        started = true;
        out.push_str(line);
        out.push('\n');
    }
    out
}

/// Định nghĩa có dẫn tài liệu gốc (nguồn `primary: true`) không? Trả về lời nhắc nếu chưa.
fn definition_source_problem(note: &Note) -> Option<String> {
    let rule = "cấp Biết phải dẫn tài liệu gốc (nguồn có primary: true)";
    if note.sources.is_empty() {
        return Some(format!("Chưa có nguồn nào — {rule}"));
    }
    if !note.sources.iter().any(|s| s.primary) {
        return Some(format!("Chưa nguồn nào được đánh dấu tài liệu gốc — {rule}"));
    }
    let mut cited: Vec<usize> = CITATION.captures_iter(&definition_text(&note.body)).filter_map(|c| c[1].parse().ok()).collect();
    cited.sort_unstable();
    cited.dedup();
    if cited.is_empty() {
        return Some(format!("Định nghĩa (đoạn mở đầu, mục Định nghĩa) chưa dẫn nguồn — {rule}"));
    }
    if !cited.iter().any(|&n| n >= 1 && note.sources.get(n - 1).is_some_and(|s| s.primary)) {
        let list: Vec<String> = cited.iter().map(|n| format!("[{n}]")).collect();
        return Some(format!("Định nghĩa chỉ dẫn {} — không có tài liệu gốc; {rule}", list.join("")));
    }
    None
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

/// Một nhóm bản nháp: chủ đề chính và các bài con theo thứ tự đọc (xem `Kb::inbox_by_topic`).
pub struct TopicGroup<'a> {
    pub topic: Option<&'a Note>,
    pub notes: Vec<&'a Note>,
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

        // Lộ trình học của các chủ đề: cấp của từng bài, bài còn thiếu.
        for i in 0..kb.all.len() {
            if kb.all[i].kind != "topic" {
                continue;
            }
            let (raw, unknown) = parse_learning_path(&kb.all[i].body);
            let mut path = Vec::new();
            for r in raw {
                let (mut notes, mut gaps): (Vec<String>, Vec<String>) = (vec![], vec![]);
                for t in r.targets {
                    match kb.resolve_idx(&t) {
                        Some(j) if j != i => {
                            let s = kb.all[j].slug.clone();
                            if !notes.contains(&s) {
                                notes.push(s);
                            }
                        }
                        Some(_) => {}
                        None if !gaps.contains(&t) => gaps.push(t),
                        None => {}
                    }
                }
                path.push(PathLevel { level: r.level, id: slugify(&r.heading), heading: r.heading, notes, gaps });
            }
            for u in unknown {
                kb.issues.push(Issue {
                    path: kb.all[i].path.clone(),
                    level: Level::Warning,
                    message: format!("Lộ trình học: \"### {u}\" không khớp cấp nào — dùng Biết, Hiểu, Vận dụng, Phân tích/tổng hợp, Đánh giá, Sáng tạo"),
                    draft: kb.all[i].inbox,
                });
            }
            kb.all[i].learning_path = path;
        }
        // Bài cấp Hiểu / Vận dụng: có mục tiêu bài học và bài tập tự kiểm tra (người dùng dặn 2026-11-26).
        // Kiểm lúc còn là bản nháp, như câu định nghĩa.
        let mut checked: HashSet<&str> = HashSet::new();
        let mut lesson_issues = Vec::new();
        for topic in kb.all.iter().filter(|n| n.kind == "topic") {
            for l in topic.learning_path.iter().filter(|l| l.level == 2 || l.level == 3) {
                for s in &l.notes {
                    let n = &kb.all[kb.by_slug[s]];
                    if !n.inbox || n.kind == "source" || !checked.insert(n.slug.as_str()) {
                        continue;
                    }
                    let mut missing = vec![];
                    if !has_objectives(&n.body) {
                        missing.push("mục \"Mục tiêu bài học\"");
                    }
                    if extract_exercises(&n.body).is_empty() {
                        missing.push("mục \"Bài tập\" (mỗi câu một tiêu đề ###, kèm **Đáp án:**)");
                    }
                    if !missing.is_empty() {
                        lesson_issues.push(Issue {
                            path: n.path.clone(),
                            level: Level::Warning,
                            message: format!("Bài cấp {} · {} của lộ trình \"{}\" chưa có {}", l.level, level_name(l.level), topic.title, missing.join(" và ")),
                            draft: true,
                        });
                    }
                }
            }
        }
        kb.issues.extend(lesson_issues);
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

    /// Bản nháp xếp theo chủ đề để người duyệt biết đâu là bài chính, đâu là bài con:
    /// mỗi nhóm là một chủ đề (bản nháp hoặc đã đăng) với các bài con nháp theo thứ tự
    /// chủ đề nhắc tới (= thứ tự đọc). Chủ đề nháp đứng trước; bài không thuộc chủ đề nào
    /// nằm ở nhóm cuối (`topic: None`).
    pub fn inbox_by_topic(&self) -> Vec<TopicGroup<'_>> {
        let topics = self.topics_in_order(true);
        let mut taken: HashSet<&str> = topics.iter().map(|t| t.slug.as_str()).collect();
        let mut groups: Vec<TopicGroup> = vec![];
        for t in topics {
            let notes: Vec<&Note> = self.reading_order(t, true).into_iter().filter(|n| n.inbox && taken.insert(n.slug.as_str())).collect();
            if t.inbox || !notes.is_empty() {
                groups.push(TopicGroup { topic: Some(t), notes });
            }
        }
        let rest: Vec<&Note> = self.inbox().into_iter().filter(|n| !taken.contains(n.slug.as_str())).collect();
        if !rest.is_empty() {
            groups.push(TopicGroup { topic: None, notes: rest });
        }
        groups
    }

    /// Nhãn vai trò của một bản nháp trong Khu duyệt bài / Bàn làm việc:
    /// "Hiểu · Bài 2/8" (chủ đề có lộ trình học), "Bài con 2/8", "Tài liệu gốc · 8/8"…
    pub fn role_label(&self, note: &Note) -> String {
        match self.place(note, true) {
            Some(Place { level: Some(l), index, total, .. }) => format!("{} · Bài {index}/{total}", level_name(l)),
            Some(Place { index, total, .. }) if note.kind == "source" => format!("Tài liệu gốc · {index}/{total}"),
            Some(Place { index, total, .. }) => format!("Bài con {index}/{total}"),
            None => "Chưa thuộc chủ đề".into(),
        }
    }

    /// Các chủ đề theo thứ tự ưu tiên khi một bài thuộc nhiều chủ đề: chủ đề nháp trước, rồi chủ đề đã đăng, mỗi nhóm theo tên.
    fn topics_in_order(&self, with_drafts: bool) -> Vec<&Note> {
        let mut topics: Vec<&Note> = if with_drafts { self.inbox().into_iter().filter(|n| n.kind == "topic").collect() } else { vec![] };
        topics.sort_by(|a, b| vi_cmp(&a.title, &b.title));
        let mut published = self.by_type("topic");
        published.sort_by(|a, b| vi_cmp(&a.title, &b.title));
        topics.extend(published);
        topics
    }

    /// Thứ tự đọc các bài con của một chủ đề: trước hết theo "## Lộ trình học" (cấp 1 → 6), rồi
    /// các bài khác theo thứ tự chủ đề nhắc tới lần đầu; ghi chú `source` (bản đồ tài liệu gốc)
    /// không nằm trong lộ trình thì xếp cuối vì là tài liệu tra cứu, không phải bài học.
    pub fn reading_order(&self, topic: &Note, with_drafts: bool) -> Vec<&Note> {
        let visible = |n: &&Note| (with_drafts || !n.inbox) && n.kind != "topic";
        let mut seen: HashSet<&str> = HashSet::new();
        let mut ordered: Vec<&Note> = topic
            .learning_path
            .iter()
            .flat_map(|l| l.notes.iter())
            .filter_map(|s| self.get(s))
            .filter(visible)
            .filter(|n| seen.insert(n.slug.as_str()))
            .collect();
        let children: Vec<&Note> = topic.links.iter().filter_map(|s| self.get(s)).filter(visible).filter(|n| !seen.contains(n.slug.as_str())).collect();
        let (lessons, sources): (Vec<&Note>, Vec<&Note>) = children.into_iter().partition(|n| n.kind != "source");
        ordered.extend(lessons);
        ordered.extend(sources);
        ordered
    }

    /// Chủ đề mà ghi chú thuộc về, vị trí trong thứ tự đọc (từ 1), cấp trong lộ trình, bài trước và bài tiếp.
    /// `with_drafts`: tính cả chủ đề và bài con đang là bản nháp (Khu duyệt bài, Bàn làm việc).
    pub fn place(&self, note: &Note, with_drafts: bool) -> Option<Place<'_>> {
        if note.kind == "topic" {
            return None;
        }
        self.topics_in_order(with_drafts).into_iter().find_map(|t| {
            let order = self.reading_order(t, with_drafts);
            let pos = order.iter().position(|n| n.slug == note.slug)?;
            Some(Place {
                topic: t,
                index: pos + 1,
                total: order.len(),
                level: level_in(t, &note.slug),
                prev: pos.checked_sub(1).map(|p| order[p]),
                next: order.get(pos + 1).copied(),
            })
        })
    }

    /// Chủ đề mà ghi chú thuộc về và vị trí của nó trong thứ tự đọc của chủ đề (bắt đầu từ 1, trên tổng số).
    #[cfg(test)]
    pub fn parent_topic(&self, note: &Note, with_drafts: bool) -> Option<(&Note, usize, usize)> {
        self.place(note, with_drafts).map(|p| (p.topic, p.index, p.total))
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
