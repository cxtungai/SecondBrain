//! Các lệnh dòng lệnh. Người dùng chỉ cần bấm đúp (mở Bàn làm việc); các lệnh còn lại dành cho Claude.

use crate::articles::{intent_label, INTENTS};
use crate::kb::Level;
use crate::project::Project;
use crate::publish;
use crate::schema::{self, NOTE_TYPES};
use crate::seo;
use crate::server;
use crate::site::{Site, ASSETS};
use crate::text::{slugify, today};
use serde_json::json;
use std::collections::HashMap;
use std::io::IsTerminal;

const HELP: &str = "Second Brain — kho tri thức + website + Bàn làm việc

Dùng (bấm đúp chương trình = lệnh `mo`):
  secondbrain [mo] [--cong 4321] [--khong-mo-trinh-duyet]
                                   Mở Bàn làm việc và xem trước web (có bản nháp, tự cập nhật)
  secondbrain build [--out dist]   Xuất web tĩnh (đọc SITE_URL, BASE_PATH từ biến môi trường)
  secondbrain check [--inbox] [--verbose]
                                   Kiểm tra cấu trúc, liên kết, nguồn, trùng từ khoá
  secondbrain new <loại> \"Tiêu đề\" [--tags a,b] [--inbox] [--slug x]
                                   Loại: topic concept fact framework case-study source
  secondbrain new article \"Tiêu đề\" [--keyword \"…\"] [--based-on a,b] [--intent how-to]
  secondbrain publish [file|slug] [--status draft] [--force] [--dry-run]
                                   Đăng bản nháp đã được người dùng duyệt
  secondbrain kb <search|show|list|gaps|orphans|mentions|stats> [...] [--json]
  secondbrain seo [slug] [--keyword \"…\"] [--drafts] [--json]
  secondbrain init                 Tạo file cau-hinh.yml mặc định
";

struct Args {
    pos: Vec<String>,
    flags: HashMap<String, String>,
}

impl Args {
    fn parse(argv: Vec<String>) -> Args {
        let mut pos = Vec::new();
        let mut flags = HashMap::new();
        let mut i = 0;
        while i < argv.len() {
            let a = &argv[i];
            if let Some(f) = a.strip_prefix("--") {
                if let Some((k, v)) = f.split_once('=') {
                    flags.insert(k.to_string(), v.to_string());
                } else if i + 1 < argv.len() && !argv[i + 1].starts_with("--") && takes_value(f) {
                    flags.insert(f.to_string(), argv[i + 1].clone());
                    i += 1;
                } else {
                    flags.insert(f.to_string(), "true".into());
                }
            } else {
                pos.push(a.clone());
            }
            i += 1;
        }
        Args { pos, flags }
    }
    fn flag(&self, k: &str) -> bool {
        self.flags.contains_key(k)
    }
    fn get(&self, k: &str) -> Option<&str> {
        self.flags.get(k).map(|s| s.as_str())
    }
}

fn takes_value(flag: &str) -> bool {
    matches!(flag, "cong" | "port" | "out" | "tags" | "slug" | "keyword" | "based-on" | "intent" | "status" | "type" | "tag")
}

struct C {
    on: bool,
}
impl C {
    fn wrap(&self, code: &str, s: &str) -> String {
        if self.on {
            format!("\x1b[{code}m{s}\x1b[0m")
        } else {
            s.to_string()
        }
    }
    fn red(&self, s: &str) -> String {
        self.wrap("31", s)
    }
    fn yellow(&self, s: &str) -> String {
        self.wrap("33", s)
    }
    fn green(&self, s: &str) -> String {
        self.wrap("32", s)
    }
    fn cyan(&self, s: &str) -> String {
        self.wrap("36", s)
    }
    fn dim(&self, s: &str) -> String {
        self.wrap("2", s)
    }
    fn bold(&self, s: &str) -> String {
        self.wrap("1", s)
    }
}

pub fn run(argv: Vec<String>) -> i32 {
    let double_clicked = argv.is_empty();
    let args = Args::parse(argv);
    let c = C { on: std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none() };
    let cmd = args.pos.first().map(|s| s.as_str()).unwrap_or("mo");
    if matches!(cmd, "help" | "-h" | "--help" | "tro-giup") || args.flag("help") {
        print!("{HELP}");
        return 0;
    }
    if matches!(cmd, "version" | "--version" | "-V") {
        println!("secondbrain {}", env!("CARGO_PKG_VERSION"));
        return 0;
    }
    let project = match Project::find() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", c.red(&e));
            if double_clicked {
                pause();
            }
            return 1;
        }
    };
    let code = match cmd {
        "mo" | "open" | "dev" | "serve" => {
            let port = args.get("cong").or(args.get("port")).and_then(|p| p.parse().ok()).unwrap_or(4321);
            server::serve(project, port, !args.flag("khong-mo-trinh-duyet") && !args.flag("no-open"))
        }
        "build" => build(&project, &args, &c),
        "check" => check(&project, &args, &c),
        "new" => new(&project, &args, &c),
        "publish" => publish_cmd(&project, &args, &c),
        "kb" => kb_cmd(&project, &args, &c),
        "seo" => seo_cmd(&project, &args, &c),
        "init" => {
            let path = project.abs(crate::config::CONFIG_FILE);
            if path.exists() {
                println!("Đã có {}", crate::config::CONFIG_FILE);
            } else {
                std::fs::write(&path, server::default_config_yaml()).expect("ghi cau-hinh.yml");
                println!("{}", c.green(&format!("✓ Đã tạo {}", crate::config::CONFIG_FILE)));
            }
            0
        }
        other => {
            eprintln!("Không có lệnh \"{other}\".\n\n{HELP}");
            1
        }
    };
    if double_clicked && code != 0 {
        pause();
    }
    code
}

fn pause() {
    eprintln!("\nNhấn Enter để đóng cửa sổ…");
    let _ = std::io::stdin().read_line(&mut String::new());
}

// ---------- check ----------

fn check(project: &Project, args: &Args, c: &C) -> i32 {
    let data = project.load(true);
    let (errors, warnings) = print_issues(&data, args.flag("inbox"), c);
    let kb = &data.kb;
    if let Some(e) = &data.config_error {
        println!("\n{}", c.red(&format!("lỗi cấu hình: {e}")));
    }
    let verbose = args.flag("verbose");
    println!("\n{}", c.bold("Tổng quan"));
    println!("  {} ghi chú đã đăng, {} bản nháp chờ duyệt, {} tag", kb.published.len(), kb.inbox_idx.len(), kb.tags.len());
    println!("  {} bài viết đã đăng, {} bài nháp", data.articles.published().len(), data.articles.drafts().len());
    let count = |s: &str| kb.notes().iter().filter(|n| n.status == s).count();
    println!("  {} verified · {} draft · {} outdated", count("verified"), count("draft"), count("outdated"));
    if !kb.gaps.is_empty() {
        println!("\n{} {}", c.bold(&format!("Tri thức còn thiếu ({})", kb.gaps.len())), c.dim("— [[liên kết]] chưa có ghi chú, nên nghiên cứu tiếp"));
        for g in kb.gaps.iter().take(if verbose { usize::MAX } else { 10 }) {
            println!("  [[{}]] {}", g.name, c.dim(&format!("← {}", g.from.join(", "))));
        }
        if !verbose && kb.gaps.len() > 10 {
            println!("{}", c.dim(&format!("  … và {} mục khác (--verbose)", kb.gaps.len() - 10)));
        }
    }
    let orphans = kb.orphans();
    if !orphans.is_empty() {
        println!("\n{} {}", c.bold(&format!("Ghi chú cô lập ({})", orphans.len())), c.dim("— không liên kết tới/được liên kết từ ghi chú nào"));
        for n in orphans {
            println!("  {} {}", n.slug, c.dim(&n.path));
        }
    }
    let mentions = kb.mentions();
    if !mentions.is_empty() {
        println!("\n{} {}", c.bold(&format!("Gợi ý liên kết ({})", mentions.len())), c.dim("— nhắc tên ghi chú khác nhưng chưa đặt [[liên kết]]"));
        for m in mentions.iter().take(if verbose { usize::MAX } else { 10 }) {
            println!("  {} → [[{}]] {}", m.from, m.to, c.dim(&format!("\"…{}…\"", m.context)));
        }
        if !verbose && mentions.len() > 10 {
            println!("{}", c.dim(&format!("  … và {} gợi ý khác (--verbose)", mentions.len() - 10)));
        }
    }
    println!();
    if errors > 0 {
        println!("{}", c.red(&format!("✗ {errors} lỗi, {warnings} cảnh báo")));
        return 1;
    }
    println!("{}", c.green(&format!("✓ Không có lỗi{}", if warnings > 0 { format!(" ({warnings} cảnh báo)") } else { String::new() })));
    if kb.issues.iter().chain(data.articles.issues.iter()).any(|i| i.level == Level::Error && i.draft) {
        println!("{}", c.dim("  lỗi* = lỗi trong bản nháp (_inbox hoặc bài viết nháp), không chặn build; sửa trước khi đăng."));
    }
    0
}

fn print_issues(data: &crate::project::Loaded, strict_drafts: bool, c: &C) -> (usize, usize) {
    let mut by_file: Vec<(&str, Vec<&crate::kb::Issue>)> = Vec::new();
    for i in data.kb.issues.iter().chain(data.articles.issues.iter()) {
        match by_file.iter_mut().find(|(p, _)| *p == i.path) {
            Some((_, v)) => v.push(i),
            None => by_file.push((&i.path, vec![i])),
        }
    }
    by_file.sort_by(|a, b| a.0.cmp(b.0));
    let (mut errors, mut warnings) = (0, 0);
    for (file, issues) in by_file {
        let tag = if issues[0].draft { c.cyan(if file.starts_with("articles/") { " (bài nháp)" } else { " (chờ duyệt)" }) } else { String::new() };
        println!("\n{}{}", c.bold(file), tag);
        for i in issues {
            let fatal = i.level == Level::Error && (!i.draft || strict_drafts);
            if fatal {
                errors += 1
            } else {
                warnings += 1
            }
            let label = match (i.level, fatal) {
                (Level::Error, true) => c.red("  lỗi     "),
                (Level::Error, false) => c.yellow("  lỗi*    "),
                _ => c.yellow("  cảnh báo"),
            };
            println!("{label} {}", i.message);
        }
    }
    (errors, warnings)
}

// ---------- build ----------

fn build(project: &Project, args: &Args, c: &C) -> i32 {
    // Kiểm tra trước: lỗi ở ghi chú/bài đã đăng thì dừng.
    let check_data = project.load(true);
    let (errors, _) = {
        let fatal = check_data.kb.issues.iter().chain(check_data.articles.issues.iter()).filter(|i| i.level == Level::Error && !i.draft).count();
        (fatal, 0)
    };
    if errors > 0 {
        print_issues(&check_data, false, c);
        eprintln!("\n{}", c.red(&format!("✗ {errors} lỗi — sửa trước khi build")));
        return 1;
    }
    let site_url = std::env::var("SITE_URL").ok().filter(|s| !s.is_empty());
    let base = std::env::var("BASE_PATH").unwrap_or_default();
    let out = project.abs(args.get("out").unwrap_or("dist"));
    let site = Site::new(project.load(false), &base, site_url, false);
    if out.exists() {
        let _ = std::fs::remove_dir_all(&out);
    }
    let mut pages = 0;
    for route in site.routes() {
        let Some(page) = site.render(&route) else { continue };
        let file = if route.is_empty() || route.ends_with('/') { out.join(&route).join("index.html") } else { out.join(&route) };
        if let Some(dir) = file.parent() {
            std::fs::create_dir_all(dir).expect("tạo thư mục");
        }
        std::fs::write(&file, &page.body).expect("ghi file");
        if page.content_type.starts_with("text/html") {
            pages += 1;
        }
    }
    for (path, _, body) in ASSETS {
        let file = out.join(path);
        std::fs::create_dir_all(file.parent().unwrap()).expect("tạo thư mục");
        std::fs::write(file, body).expect("ghi file");
    }
    println!("{}", c.green(&format!("✓ Đã xuất {pages} trang vào {}", project.rel(&out))));
    0
}

// ---------- new ----------

fn new(project: &Project, args: &Args, c: &C) -> i32 {
    let kind = args.pos.get(1).map(|s| s.as_str()).unwrap_or("");
    let title = args.pos.get(2..).map(|v| v.join(" ")).unwrap_or_default().trim().to_string();
    if title.is_empty() || (kind != "article" && schema::type_info(kind).is_none()) {
        println!("Dùng: secondbrain new <loại> \"Tiêu đề\" [--tags a,b] [--inbox]\n\nCác loại:");
        for t in NOTE_TYPES {
            println!("  {:<11} {} — {}", t.key, t.label, t.description);
        }
        println!("  {:<11} Bài viết SEO/GEO (thư mục articles/) — thêm --keyword \"…\" --based-on a,b", "article");
        return 1;
    }
    let slug = slugify(args.get("slug").unwrap_or(&title));
    let data = project.load(true);
    let q = |s: &str| serde_json::to_string(s).unwrap_or_default();
    if kind == "article" {
        if data.articles.get(&slug).is_some() || project.abs(&format!("articles/{slug}.md")).exists() {
            eprintln!("{}", c.red(&format!("Đã có bài viết \"{slug}\"")));
            return 1;
        }
        let intent = args.get("intent").unwrap_or("guide");
        if !INTENTS.iter().any(|i| i.0 == intent) {
            eprintln!("{}", c.red(&format!("--intent phải là: {}", INTENTS.iter().map(|i| i.0).collect::<Vec<_>>().join(", "))));
            return 1;
        }
        let based: Vec<String> = args.get("based-on").map(|s| s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect()).unwrap_or_default();
        let missing: Vec<&String> = based.iter().filter(|s| data.kb.resolve(s).is_none()).collect();
        if !missing.is_empty() {
            println!("{}", c.yellow(&format!("  Chưa có ghi chú: {} — nghiên cứu trước khi viết bài.", missing.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "))));
        }
        let tpl = std::fs::read_to_string(project.abs("knowledge/_templates/article.md")).unwrap_or_default();
        let mut content = tpl.replace("{{title}}", &title.replace('"', "\\\"")).replace("{{date}}", &today());
        content = content.replace("intent: guide", &format!("intent: {intent}")).replace("based_on: []", &format!("based_on: [{}]", based.join(", ")));
        if let Some(k) = args.get("keyword") {
            content = content.replacen("keyword: \"\"", &format!("keyword: {}", q(k)), 1);
        }
        let file = project.abs(&format!("articles/{slug}.md"));
        std::fs::create_dir_all(file.parent().unwrap()).ok();
        std::fs::write(&file, content).expect("ghi file");
        println!("{}", c.green(&format!("✓ Đã tạo {} (bài nháp)", project.rel(&file))));
        println!("{}", c.dim("  Điền description, keyword, based_on; kiểm tra trùng từ khoá: secondbrain seo --keyword \"…\""));
        return 0;
    }
    if let Some(existing) = data.kb.all.iter().find(|n| n.slug == slug || n.updates.as_deref() == Some(&slug)) {
        if !args.flag("inbox") {
            eprintln!("{}", c.red(&format!("Đã có ghi chú \"{slug}\": {}", existing.path)));
            return 1;
        }
    }
    let info = schema::type_info(kind).unwrap();
    let dir = if args.flag("inbox") { "knowledge/_inbox".to_string() } else { format!("knowledge/{}", info.folder) };
    let file = project.abs(&format!("{dir}/{slug}.md"));
    if file.exists() {
        eprintln!("{}", c.red(&format!("File đã tồn tại: {}", project.rel(&file))));
        return 1;
    }
    let tpl = std::fs::read_to_string(project.abs(&format!("knowledge/_templates/{kind}.md"))).unwrap_or_default();
    let mut content = tpl.replace("{{title}}", &title.replace('"', "\\\"")).replace("{{date}}", &today());
    if let Some(tags) = args.get("tags") {
        let list: Vec<&str> = tags.split(',').map(|t| t.trim()).filter(|t| !t.is_empty()).collect();
        content = content.replacen("tags: []", &format!("tags: [{}]", list.join(", ")), 1);
    }
    std::fs::create_dir_all(file.parent().unwrap()).ok();
    std::fs::write(&file, content).expect("ghi file");
    println!("{}", c.green(&format!("✓ Đã tạo {}", project.rel(&file))));
    println!("{}", c.dim("  Nhớ điền summary (1–2 câu) và nguồn (sources) trước khi đăng."));
    0
}

// ---------- publish ----------

fn publish_cmd(project: &Project, args: &Args, c: &C) -> i32 {
    let Some(arg) = args.pos.get(1) else {
        let data = project.load(true);
        if data.kb.inbox_idx.is_empty() {
            println!("Không có bản nháp nào trong knowledge/_inbox.");
        } else {
            println!("{}", c.bold("Bản nháp chờ duyệt:"));
            for n in data.kb.inbox() {
                println!("  {}", n.path);
            }
        }
        // Liệt kê theo file để cả bài nháp còn lỗi cũng hiện ra.
        let drafts: Vec<String> = project
            .read_md("articles")
            .into_iter()
            .filter(|f| crate::articles::is_article_path(&f.path) && !f.raw.lines().any(|l| l.starts_with("status:") && l.contains("published")))
            .map(|f| f.path)
            .collect();
        if !drafts.is_empty() {
            println!("{}", c.bold("\nBài viết nháp:"));
            for p in drafts {
                println!("  {p}");
            }
        }
        println!("{}", c.dim("\nĐăng: secondbrain publish <tên-file>"));
        return 0;
    };
    let status = args.get("status").unwrap_or("verified");
    let dry = args.flag("dry-run");
    match publish::publish(project, arg, status, args.flag("force"), dry) {
        Err(errors) => {
            for e in errors {
                eprintln!("{}", c.red(&e));
            }
            1
        }
        Ok(r) => {
            println!("{}: {}", if r.updated_existing { "Cập nhật" } else { "Đăng" }, c.bold(&r.title));
            println!("  {} → {}", r.from, r.to);
            for w in &r.warnings {
                println!("{}", c.yellow(&format!("  cảnh báo: {w}")));
            }
            println!("{}", if dry { c.dim("(dry-run: chưa ghi file)") } else { c.green("✓ Đã đăng") });
            if !r.suggestions.is_empty() {
                println!("\n{}", c.bold("Gợi ý"));
                for s in &r.suggestions {
                    println!("  {s}");
                }
            }
            println!("{}", c.dim("\nXong thì đưa lên web: Bàn làm việc → \"Đưa lên web\" (hoặc commit & push)."));
            0
        }
    }
}

// ---------- kb ----------

fn kb_cmd(project: &Project, args: &Args, c: &C) -> i32 {
    let data = project.load(true);
    let kb = &data.kb;
    let sub = args.pos.get(1).map(|s| s.as_str()).unwrap_or("");
    let rest = args.pos.get(2..).map(|v| v.join(" ")).unwrap_or_default();
    let as_json = args.flag("json");
    let brief = |n: &crate::kb::Note| {
        json!({"slug": n.slug, "title": n.title, "type": n.kind, "status": if n.inbox { format!("{} (chờ duyệt)", n.status) } else { n.status.clone() },
            "summary": n.summary, "tags": n.tags, "path": n.path, "credibility": n.credibility.average})
    };
    let line = |n: &crate::kb::Note| {
        format!("{} {} {}\n    {}", c.bold(&n.slug), c.dim(&format!("[{} · {}]", n.kind, if n.inbox { "chờ duyệt" } else { &n.status })), n.title, c.dim(&n.summary))
    };
    let print_notes = |list: Vec<&crate::kb::Note>, empty: &str| {
        if as_json {
            println!("{}", serde_json::to_string_pretty(&list.iter().map(|n| brief(n)).collect::<Vec<_>>()).unwrap());
        } else if list.is_empty() {
            println!("{empty}");
        } else {
            println!("{}", list.iter().map(|n| line(n)).collect::<Vec<_>>().join("\n"));
        }
    };
    match sub {
        "search" => print_notes(kb.search(&rest, true), "Không tìm thấy ghi chú nào."),
        "show" => {
            let Some(n) = kb.get(&rest).or_else(|| kb.resolve(&rest)) else {
                eprintln!("Không có ghi chú \"{rest}\"");
                return 1;
            };
            if as_json {
                let mut v = brief(n);
                v["aliases"] = json!(n.aliases);
                v["sources"] = json!(n.sources);
                v["created"] = json!(n.created);
                v["updated"] = json!(n.updated);
                v["reviewed"] = json!(n.reviewed);
                v["links"] = json!(n.links);
                v["backlinks"] = json!(n.backlinks);
                v["brokenLinks"] = json!(n.broken_links);
                v["articles"] = json!(data.articles.for_note(&n.slug).iter().map(|a| &a.slug).collect::<Vec<_>>());
                println!("{}", serde_json::to_string_pretty(&v).unwrap());
            } else {
                let or = |v: &[String]| if v.is_empty() { "—".to_string() } else { v.join(", ") };
                println!("{}", line(n));
                println!("    file: {}", n.path);
                println!("    tags: {} · aliases: {}", or(&n.tags), or(&n.aliases));
                println!("    nguồn: {} (uy tín TB: {})", n.sources.len(), n.credibility.average.map(|a| a.to_string()).unwrap_or("chưa chấm".into()));
                println!("    liên kết tới: {}", or(&n.links));
                println!("    được liên kết từ: {}", or(&n.backlinks));
                println!("    còn thiếu: {}", or(&n.broken_links));
                let arts: Vec<String> = data.articles.for_note(&n.slug).iter().map(|a| a.slug.clone()).collect();
                println!("    bài viết dựa trên: {}", or(&arts));
            }
        }
        "list" => {
            let mut list: Vec<&crate::kb::Note> = kb.notes();
            list.extend(kb.inbox());
            if let Some(t) = args.get("type") {
                list.retain(|n| n.kind == t);
            }
            if let Some(s) = args.get("status") {
                list.retain(|n| n.status == s);
            }
            if let Some(t) = args.get("tag") {
                list.retain(|n| n.tags.iter().any(|x| slugify(x) == slugify(t)));
            }
            print_notes(list, "Trống.");
        }
        "gaps" => {
            if as_json {
                println!("{}", serde_json::to_string_pretty(&kb.gaps).unwrap());
            } else if kb.gaps.is_empty() {
                println!("Không thiếu gì.");
            } else {
                for g in &kb.gaps {
                    println!("[[{}]] {}", g.name, c.dim(&format!("← {}", g.from.join(", "))));
                }
            }
        }
        "orphans" => print_notes(kb.orphans(), "Không có ghi chú cô lập."),
        "mentions" => {
            let m: Vec<_> = kb.mentions().iter().filter(|x| rest.is_empty() || x.from == rest || x.to == rest).collect();
            if as_json {
                println!("{}", serde_json::to_string_pretty(&m).unwrap());
            } else if m.is_empty() {
                println!("Không có gợi ý.");
            } else {
                for x in m {
                    println!("{} → [[{}]] {}", x.from, x.to, c.dim(&format!("\"…{}…\"", x.context)));
                }
            }
        }
        "stats" => {
            let by_type: serde_json::Map<String, serde_json::Value> = NOTE_TYPES.iter().map(|t| (t.key.to_string(), json!(kb.by_type(t.key).len()))).collect();
            let v = json!({
                "published": kb.published.len(), "inbox": kb.inbox_idx.len(), "byType": by_type,
                "verified": kb.notes().iter().filter(|n| n.status == "verified").count(),
                "links": kb.notes().iter().map(|n| n.links.len()).sum::<usize>(), "gaps": kb.gaps.len(),
                "orphans": kb.orphans().len(), "tags": kb.tags.len(),
                "articles": data.articles.published().len(), "articleDrafts": data.articles.drafts().len(),
            });
            println!("{}", serde_json::to_string_pretty(&v).unwrap());
        }
        _ => {
            println!("Dùng: secondbrain kb <search|show|list|gaps|orphans|mentions|stats> [...] [--json]");
            return if sub.is_empty() { 0 } else { 1 };
        }
    }
    0
}

// ---------- seo ----------

fn seo_cmd(project: &Project, args: &Args, c: &C) -> i32 {
    let drafts = args.flag("drafts");
    let data = project.load(drafts);
    let kb = &data.kb;
    let notes: Vec<&crate::kb::Note> = if drafts { kb.all.iter().collect() } else { kb.notes() };
    let articles: Vec<&crate::articles::Article> = data.articles.all.iter().collect();
    let as_json = args.flag("json");

    if let Some(k) = args.get("keyword") {
        let targets = seo::keyword_targets(k, &notes, &articles);
        if as_json {
            println!("{}", serde_json::to_string_pretty(&targets).unwrap());
            return 0;
        }
        println!("{}", c.bold(&format!("Từ khoá \"{k}\"")));
        if targets.is_empty() {
            println!("{}", c.green("  Chưa trang nào nhắm từ khoá này — có thể viết bài mới."));
        }
        for t in &targets {
            let how = if t.matched == "exact" { c.red("trùng") } else { c.yellow("gần ") };
            println!("  {how}  {} {} {}", if t.kind == "note" { "ghi chú" } else { "bài viết" }, c.bold(&t.title), c.dim(&t.path));
        }
        if targets.iter().any(|t| t.matched == "exact") {
            println!("{}", c.dim("\n  Đã có trang nhắm đúng từ khoá → cải thiện trang đó, hoặc chọn góc khác (hướng dẫn, so sánh, sai lầm thường gặp…)."));
        }
        return 0;
    }

    let day = today();
    let mut rows: Vec<(String, String, &str, Vec<seo::Hint>)> = notes.iter().map(|n| (n.slug.clone(), n.title.clone(), "ghi chú", seo::note_hints(n, &day))).collect();
    let paths: HashMap<String, String> = notes.iter().map(|n| (n.slug.clone(), n.path.clone())).chain(articles.iter().map(|a| (a.slug.clone(), a.path.clone()))).collect();
    rows.extend(articles.iter().map(|a| (a.slug.clone(), a.title.clone(), "bài viết", seo::article_hints(a, kb))));
    let print_row = |r: &(String, String, &str, Vec<seo::Hint>), detailed: bool| {
        let passed = r.3.iter().filter(|h| h.ok).count();
        let ratio = passed as f64 / r.3.len().max(1) as f64;
        let score = format!("{passed}/{}", r.3.len());
        let score = if ratio >= 0.85 { c.green(&score) } else if ratio >= 0.6 { c.yellow(&score) } else { c.red(&score) };
        println!("\n{score} {} {}", c.bold(&r.1), c.dim(&format!("{} · {}", r.2, paths.get(&r.0).cloned().unwrap_or_default())));
        for h in &r.3 {
            if h.ok && !detailed {
                continue;
            }
            println!("  {} {}{}", if h.ok { c.green("✓") } else { c.yellow("○") }, h.label, if h.ok { String::new() } else { c.dim(&format!(" — {}", h.tip)) });
        }
    };
    if let Some(slug) = args.pos.get(1) {
        let Some(r) = rows.iter().find(|r| &r.0 == slug || paths.get(&r.0) == Some(slug)) else {
            eprintln!("{}", c.red(&format!("Không tìm thấy \"{slug}\" (thêm --drafts nếu là bản nháp)")));
            return 1;
        };
        if as_json {
            println!("{}", serde_json::to_string_pretty(&json!({"slug": r.0, "kind": r.2, "hints": r.3})).unwrap());
        } else {
            print_row(r, true);
        }
        return 0;
    }
    if as_json {
        let v: Vec<_> = rows
            .iter()
            .map(|r| json!({"slug": r.0, "kind": r.2, "path": paths.get(&r.0), "passed": r.3.iter().filter(|h| h.ok).count(), "total": r.3.len(), "todo": r.3.iter().filter(|h| !h.ok).collect::<Vec<_>>()}))
            .collect();
        println!("{}", serde_json::to_string_pretty(&v).unwrap());
        return 0;
    }
    println!("{}{}", c.bold("Báo cáo SEO / GEO"), c.dim(" — gợi ý, không chặn build. Ghi chú: ưu tiên rõ ràng & đáng tin; bài viết: ưu tiên đúng ý định tìm kiếm."));
    let ratio = |r: &(String, String, &str, Vec<seo::Hint>)| r.3.iter().filter(|h| h.ok).count() as f64 / r.3.len().max(1) as f64;
    let mut sorted: Vec<&(String, String, &str, Vec<seo::Hint>)> = rows.iter().collect();
    sorted.sort_by(|a, b| ratio(a).partial_cmp(&ratio(b)).unwrap());
    for r in sorted {
        print_row(r, false);
    }
    let all: Vec<&seo::Hint> = rows.iter().flat_map(|r| r.3.iter()).collect();
    println!("\n{}: {}/{} tiêu chí đạt trên {} ghi chú, {} bài viết.", c.bold("Tổng"), all.iter().filter(|h| h.ok).count(), all.len(), notes.len(), articles.len());
    println!("{}", c.dim("Chi tiết một trang: secondbrain seo <slug>"));
    let _ = intent_label;
    0
}
