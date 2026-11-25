//! Đăng bản nháp đã duyệt (dùng chung cho lệnh `publish` và nút bấm trên Bàn làm việc).

use crate::articles::{is_article_path, Articles};
use crate::kb::{is_inbox_path, parse_note, FileEntry, Kb, Level};
use crate::project::Project;
use crate::schema::{self, type_info};
use crate::seo;
use crate::text::today;
use std::path::PathBuf;

/// Kết quả đăng — để in ra màn hình hoặc hiện trên Bàn làm việc.
#[derive(Debug, Default, serde::Serialize)]
pub struct Report {
    pub title: String,
    pub from: String,
    pub to: String,
    /// Đường dẫn trang trên web (vd. notes/x/).
    pub page: String,
    pub updated_existing: bool,
    pub warnings: Vec<String>,
    /// Gợi ý kết nối / cải thiện (tiếng Việt dễ hiểu).
    pub suggestions: Vec<String>,
}

/// Đặt/thay các trường cấp cao nhất của frontmatter mà vẫn giữ nguyên phần còn lại (chú thích, thứ tự).
pub fn set_fields(raw: &str, fields: &[(&str, &str)]) -> Option<String> {
    let (yaml, body) = schema::split_frontmatter(raw);
    let yaml = yaml?;
    let mut lines: Vec<String> = yaml.lines().map(String::from).collect();
    for (key, value) in fields {
        let prefix = format!("{key}:");
        match lines.iter().position(|l| l.starts_with(&prefix)) {
            Some(i) => lines[i] = format!("{key}: {value}"),
            None => lines.push(format!("{key}: {value}")),
        }
    }
    Some(format!("---\n{}\n---\n\n{}", lines.join("\n"), body.trim_start_matches(['\n', '\r'])))
}

/// Tìm bản nháp theo tên file, slug hoặc đường dẫn.
pub fn find_draft(project: &Project, arg: &str) -> Option<PathBuf> {
    let candidates = [
        project.root.join(arg),
        project.abs("knowledge/_inbox").join(arg),
        project.abs("knowledge/_inbox").join(format!("{arg}.md")),
        project.abs("articles").join(arg),
        project.abs("articles").join(format!("{arg}.md")),
    ];
    candidates.into_iter().find(|p| p.is_file())
}

pub fn publish(project: &Project, arg: &str, status: &str, force: bool, dry_run: bool) -> Result<Report, Vec<String>> {
    let Some(source) = find_draft(project, arg) else {
        return Err(vec![format!("Không tìm thấy bản nháp \"{arg}\" (trong knowledge/_inbox hoặc articles/)")]);
    };
    let rel = project.rel(&source);
    if is_article_path(&rel) {
        publish_article(project, &rel, force, dry_run)
    } else {
        publish_note(project, &rel, status, dry_run)
    }
}

fn publish_note(project: &Project, rel: &str, status: &str, dry_run: bool) -> Result<Report, Vec<String>> {
    if !schema::STATUSES.iter().any(|s| s.key == status) {
        return Err(vec!["Trạng thái phải là: draft, verified, outdated".into()]);
    }
    let raw = std::fs::read_to_string(project.abs(rel)).map_err(|e| vec![e.to_string()])?;
    let draft = parse_note(rel, &raw).map_err(|errs| {
        let mut v = vec![format!("Bản nháp chưa hợp lệ ({rel}):")];
        v.extend(errs);
        v
    })?;
    let draft = draft.note;

    // Ghi chú đã đăng trùng tên → bản cập nhật: ghi đè đúng chỗ cũ, giữ ngày tạo.
    let files = project.read_md("knowledge");
    let published = Kb::build(&files, false);
    let existing = published.get(&draft.slug);
    let target = match existing {
        Some(n) => n.path.clone(),
        None => format!("knowledge/{}/{}.md", type_info(&draft.kind).unwrap().folder, draft.slug),
    };
    let day = today();
    let mut fields: Vec<(&str, &str)> = vec![("status", status), ("updated", &day), ("reviewed", &day)];
    if let Some(n) = existing {
        fields.push(("created", &n.created));
    }
    let output = set_fields(&raw, &fields).ok_or_else(|| vec!["Không đọc được frontmatter".to_string()])?;

    // Kiểm tra lại toàn kho như thể đã đăng.
    let mut sim: Vec<FileEntry> = files.into_iter().filter(|f| f.path != rel && f.path != target).collect();
    sim.push(FileEntry { path: target.clone(), raw: output.clone() });
    let kb = Kb::build(&sim, false);
    let own: Vec<_> = kb.issues.iter().filter(|i| i.path == target).collect();
    let blocking: Vec<String> = own.iter().filter(|i| i.level == Level::Error).map(|i| i.message.clone()).collect();
    if !blocking.is_empty() {
        let mut v = vec!["Chưa thể đăng — còn lỗi:".to_string()];
        v.extend(blocking);
        return Err(v);
    }

    if !dry_run {
        let abs = project.abs(&target);
        if let Some(dir) = abs.parent() {
            std::fs::create_dir_all(dir).map_err(|e| vec![e.to_string()])?;
        }
        std::fs::write(&abs, &output).map_err(|e| vec![e.to_string()])?;
        if project.abs(rel) != abs && is_inbox_path(rel) {
            let _ = std::fs::remove_file(project.abs(rel));
        }
    }

    // Gợi ý kết nối
    let note = kb.get(&draft.slug).expect("ghi chú vừa đăng");
    let mut suggestions = Vec::new();
    if note.backlinks.is_empty() {
        suggestions.push("Chưa ghi chú nào liên kết tới đây — nên thêm vào một ghi chú Chủ đề hoặc mục \"Liên quan\" của ghi chú gần nghĩa (nhờ Claude: \"kết nối ghi chú này\").".into());
    }
    if !note.broken_links.is_empty() {
        let list: Vec<String> = note.broken_links.iter().map(|b| format!("[[{b}]]")).collect();
        suggestions.push(format!("Còn thiếu (nên nghiên cứu tiếp): {}", list.join(", ")));
    }
    for m in kb.mentions().iter().filter(|m| m.from == note.slug) {
        suggestions.push(format!("Nên thêm [[{}]] trong ghi chú này: \"…{}…\"", m.to, m.context));
    }
    for m in kb.mentions().iter().filter(|m| m.to == note.slug) {
        suggestions.push(format!("Nên thêm [[{}]] trong {}: \"…{}…\"", note.slug, m.from, m.context));
    }
    for t in kb.by_type("topic") {
        if t.slug != note.slug && !t.links.contains(&note.slug) && t.tags.iter().any(|tag| note.tags.contains(tag)) {
            suggestions.push(format!("Chủ đề cùng tag có thể bổ sung ghi chú này: {} ({})", t.title, t.path));
        }
    }

    Ok(Report {
        title: note.title.clone(),
        from: rel.to_string(),
        to: target,
        page: format!("notes/{}/", note.slug),
        updated_existing: existing.is_some(),
        warnings: own.iter().map(|i| i.message.clone()).collect(),
        suggestions,
    })
}

fn publish_article(project: &Project, rel: &str, force: bool, dry_run: bool) -> Result<Report, Vec<String>> {
    let raw = std::fs::read_to_string(project.abs(rel)).map_err(|e| vec![e.to_string()])?;
    let day = today();
    let (yaml, _) = schema::split_frontmatter(&raw);
    let published_on = yaml
        .and_then(|y| schema::parse_yaml_map(y).ok())
        .and_then(|m| schema::get_str(&m, "published_on"))
        .unwrap_or_else(|| day.clone());
    let output = set_fields(&raw, &[("status", "published"), ("updated", &day), ("published_on", &published_on)])
        .ok_or_else(|| vec!["Không đọc được frontmatter".to_string()])?;

    let kb = Kb::build(&project.read_md("knowledge"), false);
    let mut files: Vec<FileEntry> = project.read_md("articles").into_iter().filter(|f| f.path != rel).collect();
    files.push(FileEntry { path: rel.to_string(), raw: output.clone() });
    let arts = Articles::build(&files, &kb, false);
    let own: Vec<_> = arts.issues.iter().filter(|i| i.path == rel).collect();
    let blocking: Vec<String> = own.iter().filter(|i| i.level == Level::Error).map(|i| i.message.clone()).collect();
    let Some(article) = arts.all.iter().find(|a| a.path == rel) else {
        let mut v = vec!["Chưa thể đăng bài — còn lỗi:".to_string()];
        v.extend(blocking);
        return Err(v);
    };
    if !blocking.is_empty() {
        let mut v = vec!["Chưa thể đăng bài — còn lỗi:".to_string()];
        v.extend(blocking);
        return Err(v);
    }
    let unverified: Vec<String> = article
        .notes
        .iter()
        .filter_map(|s| kb.get(s))
        .filter(|n| n.status != "verified")
        .map(|n| format!("{} ({}, trạng thái: {})", n.title, n.path, schema::status_info(&n.status).label))
        .collect();
    if !unverified.is_empty() && !force {
        let mut v = vec!["Chưa thể đăng bài — tri thức gốc chưa được kiểm chứng:".to_string()];
        v.extend(unverified);
        v.push("Duyệt các ghi chú gốc trước, hoặc dùng --force nếu chấp nhận đăng khi tri thức còn là nháp.".into());
        return Err(v);
    }
    if !dry_run {
        std::fs::write(project.abs(rel), &output).map_err(|e| vec![e.to_string()])?;
    }
    let suggestions = seo::article_hints(article, &kb).into_iter().filter(|h| !h.ok).map(|h| format!("{} — {}", h.label, h.tip)).collect();
    Ok(Report {
        title: article.title.clone(),
        from: rel.to_string(),
        to: rel.to_string(),
        page: format!("bai-viet/{}/", article.slug),
        updated_existing: false,
        warnings: own.iter().map(|i| i.message.clone()).collect(),
        suggestions,
    })
}
