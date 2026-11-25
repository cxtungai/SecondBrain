//! Bàn làm việc: máy chủ nhỏ chạy trên máy (127.0.0.1) để xem trước web có cả bản nháp,
//! sửa ghi chú bằng form, duyệt & đăng, bỏ bản nháp và đưa lên web — không cần gõ lệnh.

use crate::articles::{is_article_path, INTENTS};
use crate::config::{Config, CONFIG_FILE};
use crate::kb::Level;
use crate::project::Project;
use crate::publish;
use crate::render::render_html;
use crate::schema::{self, NOTE_TYPES, SOURCE_KINDS};
use crate::site::Site;
use crate::text::{slugify, today};
use serde_json::{json, Value as Json};
use serde_yaml::{Mapping, Value as Yaml};
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;
use tiny_http::{Header, Request, Response, Server};

struct State {
    site: Site,
    version: u64,
}

pub fn serve(project: Project, port: u16, open: bool) -> i32 {
    let project = Arc::new(project);
    let state = Arc::new(RwLock::new(State { site: build_site(&project), version: 1 }));
    let lock = Arc::new(Mutex::new(())); // tránh dựng lại đồng thời

    let (server, port) = match (port..port + 10).find_map(|p| Server::http(("127.0.0.1", p)).ok().map(|s| (s, p))) {
        Some(x) => x,
        None => {
            eprintln!("Không mở được cổng {port}–{}. Có thể chương trình đang chạy ở cửa sổ khác.", port + 9);
            return 1;
        }
    };
    let address = format!("http://localhost:{port}/");
    println!("\n  Second Brain đang chạy tại {address}");
    println!("  Bàn làm việc: {address}ban-lam-viec/");
    println!("  Giữ cửa sổ này mở trong lúc dùng. Đóng cửa sổ (hoặc Ctrl+C) để tắt.\n");
    print_issue_summary(&state.read().unwrap().site);
    if open {
        open_browser(&address);
    }

    // Theo dõi thay đổi file → dựng lại → trình duyệt tự tải lại.
    {
        let (project, state, lock) = (project.clone(), state.clone(), lock.clone());
        std::thread::spawn(move || {
            let mut last = project.fingerprint();
            loop {
                std::thread::sleep(Duration::from_millis(700));
                let fp = project.fingerprint();
                if fp != last {
                    last = fp;
                    rebuild(&project, &state, &lock);
                    println!("  ↻ Đã cập nhật ({})", chrono::Local::now().format("%H:%M:%S"));
                }
            }
        });
    }

    for req in server.incoming_requests() {
        let (project, state, lock) = (project.clone(), state.clone(), lock.clone());
        std::thread::spawn(move || handle(req, &project, &state, &lock));
    }
    0
}

fn build_site(project: &Project) -> Site {
    Site::new(project.load(true), "", None, true)
}

fn rebuild(project: &Project, state: &RwLock<State>, lock: &Mutex<()>) {
    let _guard = lock.lock().unwrap();
    let site = build_site(project);
    let mut s = state.write().unwrap();
    s.site = site;
    s.version += 1;
}

fn print_issue_summary(site: &Site) {
    let issues: Vec<_> = site.kb().issues.iter().chain(site.data.articles.issues.iter()).collect();
    let errors = issues.iter().filter(|i| i.level == Level::Error).count();
    if errors > 0 {
        println!("  ⚠ {errors} lỗi cần sửa — xem mục \"Cần chú ý\" trên Bàn làm việc.\n");
    }
}

fn open_browser(url: &str) {
    #[cfg(target_os = "windows")]
    let r = Command::new("cmd").args(["/C", "start", "", url]).spawn();
    #[cfg(target_os = "macos")]
    let r = Command::new("open").arg(url).spawn();
    #[cfg(all(unix, not(target_os = "macos")))]
    let r = Command::new("xdg-open").arg(url).spawn();
    if r.is_err() {
        println!("  Mở trình duyệt và vào: {url}");
    }
}

fn header(k: &str, v: &str) -> Header {
    Header::from_bytes(k.as_bytes(), v.as_bytes()).unwrap()
}

fn respond(req: Request, status: u16, content_type: &str, body: Vec<u8>) {
    let resp = Response::from_data(body)
        .with_status_code(status)
        .with_header(header("Content-Type", content_type))
        .with_header(header("Cache-Control", "no-store"));
    let _ = req.respond(resp);
}

fn respond_json(req: Request, v: Json) {
    respond(req, 200, "application/json; charset=utf-8", v.to_string().into_bytes());
}

fn redirect(req: Request, to: &str) {
    let _ = req.respond(Response::empty(302).with_header(header("Location", to)));
}

fn percent_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'%' if i + 2 < b.len() => {
                match u8::from_str_radix(std::str::from_utf8(&b[i + 1..i + 3]).unwrap_or("zz"), 16) {
                    Ok(v) => {
                        out.push(v);
                        i += 3;
                    }
                    Err(_) => {
                        out.push(b'%');
                        i += 1;
                    }
                }
            }
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).to_string()
}

fn query_param(query: &str, key: &str) -> Option<String> {
    query.split('&').find_map(|kv| {
        let (k, v) = kv.split_once('=').unwrap_or((kv, ""));
        (k == key).then(|| percent_decode(v))
    })
}

fn handle(mut req: Request, project: &Project, state: &RwLock<State>, lock: &Mutex<()>) {
    let raw_url = req.url().to_string();
    let (path, query) = raw_url.split_once('?').unwrap_or((raw_url.as_str(), ""));
    let path = percent_decode(path);
    let method = req.method().as_str().to_uppercase();

    if method == "POST" {
        // Chặn trang web lạ gửi lệnh tới máy bạn: chỉ nhận yêu cầu từ chính Bàn làm việc.
        let ok_header = req.headers().iter().any(|h| h.field.equiv("X-Second-Brain") && h.value.as_str() == "1");
        if !ok_header || !path.starts_with("/api/") {
            return respond(req, 403, "text/plain; charset=utf-8", b"Forbidden".to_vec());
        }
        let mut body = String::new();
        let _ = req.as_reader().read_to_string(&mut body);
        let data: Json = serde_json::from_str(&body).unwrap_or(Json::Null);
        let result = api(&path["/api/".len()..], &data, project, state, lock);
        return respond_json(req, result);
    }

    let route = path.trim_start_matches('/').to_string();
    let route = route.strip_suffix("index.html").map(String::from).unwrap_or(route);
    if route == "__version" {
        let v = state.read().unwrap().version.to_string();
        return respond(req, 200, "text/plain", v.into_bytes());
    }
    let s = state.read().unwrap();
    if route == "ban-lam-viec/" {
        let html = desk(&s.site, project);
        drop(s);
        return respond(req, 200, "text/html; charset=utf-8", html.into_bytes());
    }
    if route == "ban-lam-viec/sua/" {
        let file = query_param(query, "f").unwrap_or_default();
        let html = editor(&s.site, project, &file);
        drop(s);
        return respond(req, 200, "text/html; charset=utf-8", html.into_bytes());
    }
    match s.site.render(&route) {
        Some(page) => {
            let (status, ct, body) = (page.status, page.content_type, page.body);
            drop(s);
            respond(req, status, ct, body)
        }
        None => {
            if !route.ends_with('/') && !route.contains('.') && s.site.render(&format!("{route}/")).is_some() {
                drop(s);
                return redirect(req, &format!("/{route}/"));
            }
            let body = s.site.render("404.html").map(|p| p.body).unwrap_or_default();
            drop(s);
            respond(req, 404, "text/html; charset=utf-8", body)
        }
    }
}

// ---------- Trang Bàn làm việc ----------

fn run_git(root: &Path, args: &[&str]) -> Result<String, String> {
    match Command::new("git").args(args).current_dir(root).output() {
        Ok(o) if o.status.success() => Ok(String::from_utf8_lossy(&o.stdout).to_string()),
        Ok(o) => Err(format!("{}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr))),
        Err(e) => Err(e.to_string()),
    }
}

fn git_info(root: &Path) -> Json {
    let Ok(status) = run_git(root, &["status", "--porcelain"]) else {
        return json!({"available": false});
    };
    let branch = run_git(root, &["rev-parse", "--abbrev-ref", "HEAD"]).unwrap_or_default().trim().to_string();
    let ahead = run_git(root, &["rev-list", "--count", "@{u}..HEAD"]).ok().and_then(|s| s.trim().parse::<usize>().ok()).unwrap_or(0);
    let has_remote = run_git(root, &["remote"]).map(|s| !s.trim().is_empty()).unwrap_or(false);
    json!({"available": has_remote, "changes": status.lines().filter(|l| !l.trim().is_empty()).count() + ahead, "branch": branch})
}

fn desk(site: &Site, project: &Project) -> String {
    let kb = site.kb();
    let errors_for = |path: &str| -> Vec<String> {
        kb.issues
            .iter()
            .chain(site.data.articles.issues.iter())
            .filter(|i| i.path == path && i.level == Level::Error)
            .map(|i| i.message.clone())
            .collect()
    };
    let mut pending: Vec<Json> = kb
        .inbox()
        .iter()
        .map(|n| {
            json!({"type": n.kind, "label": schema::type_label(&n.kind), "title": n.title, "summary": n.summary, "path": n.path,
                "url": site.note_url(&n.slug), "edit_url": format!("/ban-lam-viec/sua/?f={}", n.path), "errors": errors_for(&n.path),
                "cred": n.credibility.average.map(|a| format!("Uy tín nguồn TB {a}/5 · {} nguồn", n.sources.len()))})
        })
        .collect();
    pending.extend(site.data.articles.drafts().iter().map(|a| {
        json!({"type": "article", "label": format!("Bài viết · {}", crate::articles::intent_label(&a.intent)), "title": a.title,
            "summary": a.description, "path": a.path, "url": format!("/bai-viet/{}/", a.slug),
            "edit_url": format!("/ban-lam-viec/sua/?f={}", a.path), "errors": errors_for(&a.path), "cred": Json::Null})
    }));
    // Bản nháp bị lỗi đọc (không vào được danh sách) vẫn cần hiện để sửa.
    let issues: Vec<Json> = kb
        .issues
        .iter()
        .chain(site.data.articles.issues.iter())
        .map(|i| json!({"path": i.path, "message": i.message, "error": i.level == Level::Error, "edit_url": format!("/ban-lam-viec/sua/?f={}", i.path)}))
        .collect();
    let prompts = [
        ("Nghiên cứu một chủ đề", "Nghiên cứu từ khoá: (Price action) — tìm nguồn uy tín, chấm điểm, viết bản nháp cho tôi duyệt."),
        ("Nạp tài liệu bạn đã đọc", "Tôi vừa đọc (tên sách / link bài viết). Tóm tắt thành ghi chú nguồn và các khái niệm chính để tôi duyệt."),
        ("Viết bài kéo traffic", "Viết bài SEO về: (cách vẽ vùng hỗ trợ kháng cự) — chỉ dùng tri thức đã có trong kho."),
        ("Sửa bản nháp", "Sửa bản nháp (tên bài): (điều bạn muốn đổi)."),
        ("Kết nối & dọn dẹp kho", "Kết nối các ghi chú vừa đăng với tri thức liên quan và sửa các mục trong “Cần chú ý”."),
        ("Viết content từ kho", "Viết (bài Facebook 300 chữ) về (chủ đề) dựa trên kho tri thức, có trích nguồn."),
    ];
    site.render_template("desk.html", json!({
        "title": "Bàn làm việc", "section": "", "noindex": true,
        "pending_items": pending, "issues": issues, "git": git_info(&project.root),
        "types": NOTE_TYPES.iter().map(|t| json!({"key": t.key, "label": t.label})).collect::<Vec<_>>(),
        "prompts": prompts.iter().map(|(t, p)| json!({"title": t, "text": p})).collect::<Vec<_>>(),
        "gaps": kb.gaps.iter().take(8).map(|g| json!({"name": g.name, "count": g.from.len()})).collect::<Vec<_>>(),
    }))
}

fn safe_path(rel: &str) -> bool {
    !rel.contains("..")
        && !rel.contains('\\')
        && rel.ends_with(".md")
        && (rel.starts_with("knowledge/") || rel.starts_with("articles/"))
        && !rel.starts_with("knowledge/_templates/")
}

fn list_str(v: Option<&Yaml>) -> String {
    schema::string_list(v).join(", ")
}

fn editor(site: &Site, project: &Project, rel: &str) -> String {
    if !safe_path(rel) || !project.abs(rel).is_file() {
        return site.render_template("404.html", json!({"title": "Không tìm thấy file"}));
    }
    let raw = std::fs::read_to_string(project.abs(rel)).unwrap_or_default();
    let (yaml, body) = schema::split_frontmatter(&raw);
    let (map, parse_error) = match yaml.map(schema::parse_yaml_map) {
        Some(Ok(m)) => (m, None),
        Some(Err(e)) => (Mapping::new(), Some(e)),
        None => (Mapping::new(), Some("thiếu khối --- ở đầu file".to_string())),
    };
    let kind = if is_article_path(rel) { "article" } else { "note" };
    let g = |k: &str| schema::get_str(&map, k).unwrap_or_default();
    let fields = json!({
        "title": g("title"), "type": g("type"), "summary": g("summary"), "tags": list_str(schema::get(&map, "tags")),
        "aliases": list_str(schema::get(&map, "aliases")), "description": g("description"), "keyword": g("keyword"),
        "keywords": list_str(schema::get(&map, "keywords")), "intent": if g("intent").is_empty() { "guide".into() } else { g("intent") },
        "based_on": list_str(schema::get(&map, "based_on")),
    });
    let sources: Vec<Json> = match schema::get(&map, "sources") {
        Some(Yaml::Sequence(seq)) => seq
            .iter()
            .filter_map(|s| match s {
                Yaml::Mapping(m) => {
                    let mut extra = serde_json::Map::new();
                    for (k, v) in m {
                        if let Some(k) = k.as_str() {
                            if !["title", "url", "kind", "credibility", "credibility_note"].contains(&k) {
                                extra.insert(k.to_string(), schema::yaml_to_json(v));
                            }
                        }
                    }
                    let gs = |k: &str| schema::get_str(m, k).unwrap_or_default();
                    if gs("title").is_empty() && gs("url").is_empty() {
                        return None;
                    }
                    Some(json!({"title": gs("title"), "url": gs("url"), "kind": gs("kind"), "credibility": gs("credibility").parse::<u8>().ok(),
                        "credibility_note": gs("credibility_note"), "extra": Json::Object(extra).to_string()}))
                }
                Yaml::String(s) => Some(json!({"title": s, "url": if s.starts_with("http") { s.as_str() } else { "" }, "kind": "", "credibility": Json::Null, "credibility_note": "", "extra": "{}"})),
                _ => None,
            })
            .collect(),
        _ => vec![],
    };
    let body = body.trim_start_matches(['\n', '\r']);
    let is_draft = rel.starts_with("knowledge/_inbox/") || (kind == "article" && g("status") != "published");
    let slug = rel.rsplit('/').next().unwrap_or("").trim_end_matches(".md");
    let page_url = if kind == "article" {
        site.data.articles.get(slug).map(|a| format!("/bai-viet/{}/", a.slug))
    } else {
        site.kb().all.iter().find(|n| n.path == rel).map(|n| site.note_url(&n.slug))
    };
    site.render_template("editor.html", json!({
        "title": format!("Sửa: {}", if g("title").is_empty() { rel.to_string() } else { g("title") }), "noindex": true,
        "path": rel, "kind": kind, "fields": fields, "sources": sources, "body": body, "is_draft": is_draft, "page_url": page_url,
        "parse_error": parse_error, "preview": preview_html(site, body, kind == "article"),
        "types": NOTE_TYPES.iter().map(|t| json!({"key": t.key, "label": t.label})).collect::<Vec<_>>(),
        "intents": INTENTS.iter().map(|(k, l, _)| json!({"key": k, "label": l})).collect::<Vec<_>>(),
        "kinds": SOURCE_KINDS.iter().map(|(k, l)| json!({"key": k, "label": l})).collect::<Vec<_>>(),
        "scale": schema::CREDIBILITY.iter().map(|(s, l, _)| json!({"score": s, "label": l})).collect::<Vec<_>>(),
    }))
}

fn preview_html(site: &Site, body: &str, with_articles: bool) -> String {
    let kb = site.kb();
    let resolve = |t: &str, h: Option<&str>| {
        if let Some(n) = kb.resolve(t) {
            let anchor = h.map(|h| format!("#{}", slugify(h))).unwrap_or_default();
            return Some(crate::render::LinkTarget { href: format!("{}{anchor}", site.note_url(&n.slug)), title: n.title.clone(), tooltip: n.summary.clone() });
        }
        if with_articles {
            if let Some(a) = site.data.articles.resolve(t) {
                return Some(crate::render::LinkTarget { href: format!("/bai-viet/{}/", a.slug), title: a.title.clone(), tooltip: a.description.clone() });
            }
        }
        None
    };
    render_html(body, &resolve, &|t| format!("/gaps/#{}", slugify(t)))
}

// ---------- API ----------

fn err(msgs: &[&str]) -> Json {
    json!({"ok": false, "errors": msgs})
}

fn api(name: &str, data: &Json, project: &Project, state: &RwLock<State>, lock: &Mutex<()>) -> Json {
    let s = |k: &str| data.get(k).and_then(|v| v.as_str()).unwrap_or("").to_string();
    match name {
        "xem-truoc" => {
            let st = state.read().unwrap();
            json!({"ok": true, "html": preview_html(&st.site, &s("body"), s("kind") == "article")})
        }
        "luu" => {
            let rel = s("path");
            if !safe_path(&rel) {
                return err(&["Đường dẫn file không hợp lệ"]);
            }
            match save_file(project, &rel, data) {
                Err(e) => json!({"ok": false, "errors": [e]}),
                Ok(()) => {
                    rebuild(project, state, lock);
                    let st = state.read().unwrap();
                    let issues: Vec<Json> = st
                        .site
                        .kb()
                        .issues
                        .iter()
                        .chain(st.site.data.articles.issues.iter())
                        .filter(|i| i.path == rel)
                        .map(|i| json!({"level": if i.level == Level::Error { "error" } else { "warning" }, "message": i.message}))
                        .collect();
                    let slug = rel.rsplit('/').next().unwrap_or("").trim_end_matches(".md");
                    let page = if is_article_path(&rel) {
                        st.site.data.articles.get(slug).map(|a| format!("/bai-viet/{}/", a.slug))
                    } else {
                        st.site.kb().all.iter().find(|n| n.path == rel).map(|n| st.site.note_url(&n.slug))
                    };
                    json!({"ok": true, "path": rel, "issues": issues, "page": page})
                }
            }
        }
        "dang" => {
            let rel = s("path");
            if !safe_path(&rel) {
                return err(&["Đường dẫn file không hợp lệ"]);
            }
            let status = if s("status").is_empty() { "verified".to_string() } else { s("status") };
            let result = {
                let _g = lock.lock().unwrap();
                publish::publish(project, &rel, &status, false, false)
            };
            match result {
                Ok(report) => {
                    rebuild(project, state, lock);
                    json!({"ok": true, "report": report})
                }
                Err(errors) => json!({"ok": false, "errors": errors}),
            }
        }
        "xoa" => {
            let rel = s("path");
            let is_draft_file = rel.starts_with("knowledge/_inbox/")
                || (is_article_path(&rel) && state.read().unwrap().site.data.articles.drafts().iter().any(|a| a.path == rel));
            if !safe_path(&rel) || !is_draft_file {
                return err(&["Chỉ bỏ được bản nháp chưa đăng."]);
            }
            let trash = project.abs("knowledge/_thung-rac");
            let name = rel.rsplit('/').next().unwrap_or("nhap.md");
            let target = trash.join(format!("{}-{}", chrono::Local::now().format("%Y%m%d-%H%M%S"), name));
            let r = std::fs::create_dir_all(&trash).and_then(|_| std::fs::rename(project.abs(&rel), &target));
            match r {
                Ok(()) => {
                    rebuild(project, state, lock);
                    json!({"ok": true})
                }
                Err(e) => json!({"ok": false, "errors": [e.to_string()]}),
            }
        }
        "tao" => {
            let title = s("title").trim().to_string();
            let kind = s("type");
            let Some(info) = schema::type_info(&kind) else { return err(&["Loại ghi chú không hợp lệ"]) };
            let slug = slugify(&title);
            if slug.is_empty() {
                return err(&["Hãy nhập tiêu đề"]);
            }
            let rel = format!("knowledge/_inbox/{slug}.md");
            if project.abs(&rel).exists() {
                return json!({"ok": true, "edit_url": format!("/ban-lam-viec/sua/?f={rel}")});
            }
            let template = std::fs::read_to_string(project.abs(&format!("knowledge/_templates/{}.md", info.key)))
                .unwrap_or_else(|_| format!("---\ntitle: \"{{{{title}}}}\"\ntype: {}\nsummary: \"\"\ntags: []\nstatus: draft\ncreated: {{{{date}}}}\n---\n\n", info.key));
            let content = template.replace("{{title}}", &title.replace('"', "\\\"")).replace("{{date}}", &today());
            let r = std::fs::create_dir_all(project.abs("knowledge/_inbox")).and_then(|_| std::fs::write(project.abs(&rel), content));
            match r {
                Ok(()) => {
                    rebuild(project, state, lock);
                    json!({"ok": true, "edit_url": format!("/ban-lam-viec/sua/?f={rel}")})
                }
                Err(e) => json!({"ok": false, "errors": [e.to_string()]}),
            }
        }
        "cai-dat" => {
            let current = state.read().unwrap().site.data.config.clone();
            let cfg = Config { name: s("name"), description: s("description"), author: s("author"), disclaimer: s("disclaimer"), ..current };
            match std::fs::write(project.abs(CONFIG_FILE), config_yaml(&cfg)) {
                Ok(()) => {
                    rebuild(project, state, lock);
                    json!({"ok": true})
                }
                Err(e) => json!({"ok": false, "errors": [e.to_string()]}),
            }
        }
        "day-len-web" => push(project),
        _ => err(&["Không có chức năng này"]),
    }
}

fn csv(v: &str) -> Yaml {
    Yaml::Sequence(v.split(',').map(|x| x.trim()).filter(|x| !x.is_empty()).map(|x| Yaml::String(x.to_string())).collect())
}

fn set(map: &mut Mapping, key: &str, value: Yaml) {
    map.insert(Yaml::String(key.into()), value);
}

/// Ghi lại file từ form: cập nhật các trường đã sửa, giữ nguyên các trường khác.
fn save_file(project: &Project, rel: &str, data: &Json) -> Result<(), String> {
    let path = project.abs(rel);
    let raw = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let (yaml, _) = schema::split_frontmatter(&raw);
    let mut map = match yaml {
        Some(y) => schema::parse_yaml_map(y).unwrap_or_default(),
        None => Mapping::new(),
    };
    let f = |k: &str| data.get("fields").and_then(|x| x.get(k)).and_then(|v| v.as_str()).map(|s| s.trim().to_string());
    let str_val = |s: String| Yaml::String(s);
    if let Some(v) = f("title") {
        set(&mut map, "title", str_val(v));
    }
    if is_article_path(rel) {
        for k in ["description", "keyword", "intent"] {
            if let Some(v) = f(k) {
                set(&mut map, k, str_val(v));
            }
        }
        for k in ["keywords", "based_on"] {
            if let Some(v) = f(k) {
                set(&mut map, k, csv(&v));
            }
        }
    } else {
        for k in ["type", "summary"] {
            if let Some(v) = f(k) {
                set(&mut map, k, str_val(v));
            }
        }
        for k in ["tags", "aliases"] {
            if let Some(v) = f(k) {
                set(&mut map, k, csv(&v));
            }
        }
        if let Some(Json::Array(sources)) = data.get("sources") {
            let list: Vec<Yaml> = sources
                .iter()
                .filter_map(|s| {
                    let obj = s.as_object()?;
                    let mut m = Mapping::new();
                    // Thứ tự dễ đọc: tên, link, rồi các trường khác.
                    for key in ["title", "url", "author", "publisher", "date", "kind", "credibility", "credibility_note", "accessed", "note"] {
                        if let Some(v) = obj.get(key) {
                            let text = match v {
                                Json::String(x) => x.trim().to_string(),
                                Json::Number(n) => n.to_string(),
                                _ => continue,
                            };
                            if text.is_empty() {
                                continue;
                            }
                            let val = if key == "credibility" { text.parse::<u64>().map(|n| Yaml::Number(n.into())).unwrap_or(Yaml::String(text)) } else { Yaml::String(text) };
                            m.insert(Yaml::String(key.into()), val);
                        }
                    }
                    for (k, v) in obj {
                        if !m.contains_key(Yaml::String(k.clone())) && !v.is_null() && v.as_str() != Some("") {
                            if let Ok(y) = serde_yaml::to_value(v) {
                                m.insert(Yaml::String(k.clone()), y);
                            }
                        }
                    }
                    (!m.is_empty()).then_some(Yaml::Mapping(m))
                })
                .collect();
            set(&mut map, "sources", Yaml::Sequence(list));
        }
    }
    // Sửa ghi chú/bài đã đăng → cập nhật ngày.
    let published = if is_article_path(rel) {
        schema::get_str(&map, "status").as_deref() == Some("published")
    } else {
        !rel.starts_with("knowledge/_inbox/")
    };
    if published {
        set(&mut map, "updated", Yaml::String(today()));
    }
    let yaml = serde_yaml::to_string(&map).map_err(|e| e.to_string())?;
    let body = data.get("body").and_then(|b| b.as_str()).unwrap_or("").replace("\r\n", "\n");
    std::fs::write(&path, format!("---\n{}---\n\n{}\n", yaml, body.trim_end())).map_err(|e| e.to_string())
}

fn config_yaml(cfg: &Config) -> String {
    let q = |s: &str| serde_json::to_string(s).unwrap_or_default();
    let rules: Vec<String> = cfg.agent_rules.iter().map(|r| format!("  - {}", q(r))).collect();
    format!(
        "# Cấu hình Second Brain — sửa trên Bàn làm việc (mục Cài đặt) hoặc sửa trực tiếp file này.\n\n\
         # Tên và mô tả trang (hiện trên web và Google)\nname: {}\ndescription: {}\n\n\
         # Tác giả / thương hiệu (để trống nếu chưa muốn công khai)\nauthor: {}\n\n\
         # Miễn trừ trách nhiệm ở chân trang và cuối mỗi bài viết\ndisclaimer: {}\n\n\
         language: {}\nlocale: {}\n\n\
         # Quy tắc AI agent phải tuân theo khi dùng kho để viết content (hiện trong llms.txt)\nagent_rules:\n{}\n",
        q(&cfg.name),
        q(&cfg.description),
        q(&cfg.author),
        q(&cfg.disclaimer),
        cfg.language,
        cfg.locale,
        rules.join("\n")
    )
}

pub fn default_config_yaml() -> String {
    config_yaml(&Config::default())
}

/// Lưu mọi thay đổi và đẩy lên GitHub.
fn push(project: &Project) -> Json {
    let root = &project.root;
    let mut log = String::new();
    fn step(root: &Path, log: &mut String, args: &[&str]) -> Result<String, String> {
        log.push_str(&format!("$ git {}\n", args.join(" ")));
        let r = run_git(root, args);
        match &r {
            Ok(out) | Err(out) => log.push_str(out),
        }
        r
    }
    if let Err(e) = step(root, &mut log, &["add", "-A"]) {
        return json!({"ok": false, "message": "Không chạy được Git.", "log": e});
    }
    let message = format!("Cập nhật tri thức ({})", today());
    let commit = step(root, &mut log, &["commit", "-m", &message]);
    if let Err(out) = &commit {
        if !out.contains("nothing to commit") && !out.contains("không có gì") {
            return json!({"ok": false, "message": "Không lưu được thay đổi (commit).", "log": log});
        }
    }
    let pushed = match step(root, &mut log, &["push"]) {
        Ok(o) => Ok(o),
        Err(_) => step(root, &mut log, &["push", "-u", "origin", "HEAD"]),
    };
    match pushed {
        Ok(_) => json!({"ok": true, "log": log}),
        Err(out) => {
            let hint = if out.contains("Authentication") || out.contains("could not read Username") || out.contains("403") {
                "Máy chưa đăng nhập GitHub."
            } else if out.contains("rejected") {
                "Trên GitHub có thay đổi mới hơn — cần lấy về trước."
            } else {
                "Gửi lên GitHub không thành công."
            };
            json!({"ok": false, "message": hint, "log": log})
        }
    }
}
