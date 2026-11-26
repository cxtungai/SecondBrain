//! Dựng mọi trang của website (HTML + đầu ra cho AI) từ kho tri thức.
//! Dùng chung cho `secondbrain build` (ghi ra dist/) và Bàn làm việc (phục vụ trực tiếp, có bản nháp).

use crate::articles::{intent_label, Article, INTENTS};
use crate::kb::{level_in, level_name, Kb, Level, Note};
use crate::project::Loaded;
use crate::render::{extract_faq, extract_headings, render_html, to_portable_markdown, LinkTarget};
use crate::schema::{self, credibility_info, source_kind_label, status_info, type_label, NOTE_TYPES, STATUSES};
use crate::seo;
use crate::text::{format_date, markdown_to_plain, slugify, strip_diacritics, today, vi_cmp};
use minijinja::Environment;
use serde_json::{json, Value as Json};
use std::collections::HashSet;

pub struct Page {
    pub status: u16,
    pub content_type: &'static str,
    pub body: Vec<u8>,
}

impl Page {
    fn html(body: String) -> Page {
        Page { status: 200, content_type: "text/html; charset=utf-8", body: compact_html(&body).into_bytes() }
    }
    fn text(content_type: &'static str, body: String) -> Page {
        Page { status: 200, content_type, body: body.into_bytes() }
    }
}

/// Số ký tự nội dung mỗi bài đưa vào search-index.json (giữ file tìm kiếm nhẹ khi kho lớn).
const SEARCH_TEXT_CHARS: usize = 1000;

/// File tĩnh nhúng sẵn trong chương trình (không cần thư mục ngoài).
pub const ASSETS: &[(&str, &str, &[u8])] = &[
    ("assets/style.css", "text/css; charset=utf-8", include_bytes!("../assets/style.css")),
    ("assets/app.js", "text/javascript; charset=utf-8", include_bytes!("../assets/app.js")),
    ("assets/search.js", "text/javascript; charset=utf-8", include_bytes!("../assets/search.js")),
    ("assets/graph.js", "text/javascript; charset=utf-8", include_bytes!("../assets/graph.js")),
    ("assets/desk.js", "text/javascript; charset=utf-8", include_bytes!("../assets/desk.js")),
    ("assets/favicon.svg", "image/svg+xml", include_bytes!("../assets/favicon.svg")),
    ("assets/og-image.png", "image/png", include_bytes!("../assets/og-image.png")),
    ("assets/logo.png", "image/png", include_bytes!("../assets/logo.png")),
];

const TEMPLATES: &[(&str, &str)] = &[
    ("base.html", include_str!("../templates/base.html")),
    ("macros.html", include_str!("../templates/macros.html")),
    ("home.html", include_str!("../templates/home.html")),
    ("note.html", include_str!("../templates/note.html")),
    ("article.html", include_str!("../templates/article.html")),
    ("articles.html", include_str!("../templates/articles.html")),
    ("hub.html", include_str!("../templates/hub.html")),
    ("glossary.html", include_str!("../templates/glossary.html")),
    ("graph.html", include_str!("../templates/graph.html")),
    ("search.html", include_str!("../templates/search.html")),
    ("ai.html", include_str!("../templates/ai.html")),
    ("about.html", include_str!("../templates/about.html")),
    ("gaps.html", include_str!("../templates/gaps.html")),
    ("tags.html", include_str!("../templates/tags.html")),
    ("list.html", include_str!("../templates/list.html")),
    ("404.html", include_str!("../templates/404.html")),
    ("desk.html", include_str!("../templates/desk.html")),
    ("editor.html", include_str!("../templates/editor.html")),
    ("review.html", include_str!("../templates/review.html")),
];

/// Khu duyệt bài trên web (/duyet/): dựng trang cho bản nháp, nút bấm mở issue GitHub để duyệt/bỏ.
pub struct Review {
    /// "chủ/tên-repo" trên GitHub — không có thì ẩn các nút cần GitHub.
    pub repo: Option<String>,
    /// Nhánh chứa bản chính (để mở trình sửa của GitHub).
    pub branch: Option<String>,
}

pub struct Site {
    pub data: Loaded,
    /// Tiền tố đường dẫn khi deploy dạng /ten-repo/ ("" nếu ở gốc tên miền).
    pub base: String,
    /// Gốc URL tuyệt đối, vd. https://ten.github.io (để llms.txt, sitemap có link đầy đủ).
    pub site_url: Option<String>,
    /// Chạy trên máy (Bàn làm việc): hiện bản nháp, gợi ý SEO, tự tải lại.
    pub dev: bool,
    /// Có giá trị khi đây là bản dựng cho Khu duyệt bài (dữ liệu gồm bản nháp).
    pub review: Option<Review>,
    env: Environment<'static>,
    today: String,
}

fn fmt_score(s: f64) -> String {
    if s.fract() == 0.0 {
        format!("{s:.0}")
    } else {
        format!("{s:.1}")
    }
}

/// Dữ liệu hiển thị điểm uy tín (chấm tròn + chữ).
pub fn cred_view(score: Option<f64>, with_label: bool, title: Option<String>) -> Json {
    let value = score.map(|s| s.round() as u8);
    let info = value.and_then(credibility_info);
    let tip = title.unwrap_or_else(|| match (score, info) {
        (Some(s), Some((label, desc))) => format!("Uy tín {}/5 — {label}: {desc}", fmt_score(s)),
        _ => "Chưa chấm điểm uy tín".into(),
    });
    json!({
        "value": value,
        "text": score.map(|s| format!("{}/5", fmt_score(s))).unwrap_or_else(|| "chưa chấm".into()),
        "label": if with_label { info.map(|i| i.0) } else { None },
        "tip": tip,
    })
}

fn json_ld(v: &Json) -> String {
    v.to_string().replace("</", "<\\/")
}

thread_local! {
    /// Đường dẫn đang dựng — để render_template tự điền canonical / og:url mà không phải sửa từng trang.
    static CURRENT_ROUTE: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
}

/// Ảnh hiện khi chia sẻ link (Facebook, Zalo, X…): 1200×630.
pub const OG_IMAGE: &str = "assets/og-image.png";
/// Logo vuông 256×256 cho dữ liệu cấu trúc Organization.
pub const LOGO: &str = "assets/logo.png";

/// Ngày "YYYY-MM-DD" → thời điểm đầy đủ cho Atom / Open Graph.
fn iso_time(d: &str) -> String {
    if d.len() == 10 {
        format!("{d}T00:00:00Z")
    } else {
        d.to_string()
    }
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

impl Site {
    pub fn new(data: Loaded, base: &str, site_url: Option<String>, dev: bool) -> Site {
        let base = base.trim_end_matches('/').to_string();
        let base = if base.is_empty() || base.starts_with('/') { base } else { format!("/{base}") };
        let mut env = Environment::new();
        for (name, src) in TEMPLATES {
            env.add_template(name, src).unwrap_or_else(|e| panic!("Template {name} lỗi: {e}"));
        }
        // Thoát ký tự HTML nhưng giữ nguyên "/" (mặc định minijinja đổi "/" thành &#x2f;).
        env.set_formatter(|out, state, value| {
            if !value.is_safe() && state.auto_escape() != minijinja::AutoEscape::None {
                if let Some(s) = value.as_str() {
                    return out.write_str(&crate::text::escape_html(s)).map_err(|_| minijinja::Error::new(minijinja::ErrorKind::WriteFailure, "ghi lỗi"));
                }
            }
            minijinja::escape_formatter(out, state, value)
        });
        let b = base.clone();
        env.add_function("url", move |p: Option<String>| -> String { format!("{}/{}", b, p.unwrap_or_default().trim_start_matches('/')) });
        env.add_global("site", minijinja::Value::from_serialize(&data.config));
        env.add_global("dev", dev);
        let pending = data.kb.inbox_idx.len() + data.articles.drafts().len();
        env.add_global("pending", pending);
        Site { data, base, site_url: site_url.map(|s| s.trim_end_matches('/').to_string()), dev, review: None, env, today: today() }
    }

    /// Biến site (đã nạp cả bản nháp) thành bản dựng cho Khu duyệt bài: bản nháp nằm dưới /duyet/,
    /// trang nào cũng noindex, nút duyệt mở issue GitHub (xem .github/workflows/duyet.yml).
    pub fn into_review(mut self, review: Review) -> Site {
        self.env.add_global("review", true);
        self.env.add_global("review_pending", self.pending_count());
        self.review = Some(review);
        self
    }

    /// Số bản nháp chờ duyệt (ghi chú trong _inbox + bài viết nháp) — hiện ở nút "Chờ duyệt" trên web.
    pub fn set_review_pending(&mut self, pending: usize) {
        self.env.add_global("review_pending", pending);
    }

    fn pending_count(&self) -> usize {
        self.data.kb.inbox_idx.len() + self.data.articles.drafts().len()
    }

    pub fn kb(&self) -> &Kb {
        &self.data.kb
    }

    pub fn url(&self, p: &str) -> String {
        format!("{}/{}", self.base, p.trim_start_matches('/'))
    }

    /// URL tuyệt đối khi biết SITE_URL, ngược lại là đường dẫn.
    pub fn abs(&self, p: &str) -> String {
        match &self.site_url {
            Some(s) => format!("{s}{}", self.url(p)),
            None => self.url(p),
        }
    }

    pub fn note_url(&self, slug: &str) -> String {
        if self.review.is_some() && self.kb().get(slug).is_some_and(|n| n.inbox) {
            return self.url(&format!("duyet/notes/{slug}/"));
        }
        self.url(&format!("notes/{slug}/"))
    }
    fn article_url(&self, slug: &str) -> String {
        if self.review.is_some() && self.data.articles.get(slug).is_some_and(|a| !a.is_published()) {
            return self.url(&format!("duyet/bai-viet/{slug}/"));
        }
        self.url(&format!("bai-viet/{slug}/"))
    }
    fn tag_url(&self, tag: &str) -> String {
        let s = slugify(tag);
        // Khu duyệt bài: tag chỉ có ở bản nháp thì chưa có trang trên web — dẫn về danh sách chờ duyệt.
        if self.review.is_some() {
            let kb = self.kb();
            let public = kb.tags.iter().find(|t| t.slug == s).is_some_and(|t| t.notes.iter().any(|n| kb.get(n).is_some_and(|x| !x.inbox)));
            if !public {
                return self.url("duyet/");
            }
        }
        self.url(&format!("tags/{}/", if s.is_empty() { "tag".into() } else { s }))
    }
    fn gap_href(&self, target: &str) -> String {
        self.url(&format!("gaps/#{}", slugify(target)))
    }

    /// URL tuyệt đối chính thức của trang đang dựng — chỉ cho trang công khai được phép lập chỉ mục
    /// (không phải bản dựng trên máy, Khu duyệt bài, trang noindex hay 404).
    fn canonical_for(&self, route: Option<&str>, noindex: bool) -> Option<String> {
        let route = route?;
        if self.site_url.is_none() || self.dev || self.review.is_some() || noindex || route == "404.html" {
            return None;
        }
        Some(self.abs(route))
    }

    /// Đơn vị đứng tên web trong dữ liệu cấu trúc: tên thương hiệu (tên trang), không cần lộ tên người.
    fn org_ld(&self) -> Json {
        json!({"@type": "Organization", "@id": format!("{}#org", self.abs("")), "name": self.data.config.name, "url": self.abs(""),
            "logo": {"@type": "ImageObject", "url": self.abs(LOGO), "width": 256, "height": 256}})
    }

    /// Tác giả: người nếu cau-hinh.yml có điền, không thì là thương hiệu (đúng sự thật: web biên soạn và duyệt nội dung).
    fn author_ld(&self) -> Json {
        if self.data.config.author.is_empty() {
            json!({"@type": "Organization", "name": self.data.config.name, "url": self.abs("")})
        } else {
            json!({"@type": "Person", "name": self.data.config.author})
        }
    }

    pub fn render_template(&self, name: &str, mut ctx: Json) -> String {
        if let Json::Object(m) = &mut ctx {
            let noindex = m.get("noindex").and_then(|v| v.as_bool()).unwrap_or(false);
            let route = CURRENT_ROUTE.with(|r| r.borrow().clone());
            if let Some(c) = self.canonical_for(route.as_deref(), noindex) {
                m.entry("canonical").or_insert(json!(c));
            }
            if self.site_url.is_some() && !self.dev {
                m.entry("og_image").or_insert(json!(self.abs(OG_IMAGE)));
            }
        }
        match self.env.get_template(name).and_then(|t| t.render(&ctx)) {
            Ok(s) => s,
            Err(e) => {
                let mut msg = format!("Lỗi template {name}: {e}");
                let mut src = std::error::Error::source(&e);
                while let Some(s) = src {
                    msg.push_str(&format!("\n  do: {s}"));
                    src = s.source();
                }
                eprintln!("{msg}");
                format!("<pre>{}</pre>", crate::text::escape_html(&msg))
            }
        }
    }

    // ---------- Dữ liệu hiển thị ----------

    pub fn note_card(&self, n: &Note) -> Json {
        json!({
            "slug": n.slug, "title": n.title, "type": n.kind, "type_label": type_label(&n.kind),
            "type_url": self.url(&format!("types/{}/", n.kind)),
            "status": n.status, "status_label": status_info(&n.status).label, "status_desc": status_info(&n.status).description,
            "summary": n.summary, "url": self.note_url(&n.slug), "updated": format_date(&n.updated), "inbox": n.inbox,
            "tags": n.tags.iter().map(|t| json!({"name": t, "url": self.tag_url(t)})).collect::<Vec<_>>(),
            "cred": cred_view(n.credibility.average, false, None), "rated": n.credibility.rated,
        })
    }

    pub fn article_card(&self, a: &Article) -> Json {
        json!({
            "slug": a.slug, "title": a.title, "description": a.description, "url": self.article_url(&a.slug),
            "intent": a.intent, "intent_label": intent_label(&a.intent), "updated": format_date(&a.updated),
            "draft": !a.is_published(), "path": a.path,
            "notes": a.notes.iter().filter_map(|s| self.kb().get(s)).map(|n| n.title.clone()).collect::<Vec<_>>(),
        })
    }

    fn link(&self, n: &Note) -> Json {
        json!({"title": n.title, "url": self.note_url(&n.slug), "summary": n.summary})
    }

    fn render_body(&self, md: &str, with_articles: bool) -> String {
        let resolve = |t: &str, heading: Option<&str>| -> Option<LinkTarget> {
            if let Some(n) = self.kb().resolve(t) {
                let anchor = heading.map(|h| format!("#{}", slugify(h))).unwrap_or_default();
                return Some(LinkTarget { href: format!("{}{anchor}", self.note_url(&n.slug)), title: n.title.clone(), tooltip: n.summary.clone() });
            }
            if with_articles {
                if let Some(a) = self.data.articles.resolve(t) {
                    return Some(LinkTarget { href: self.article_url(&a.slug), title: a.title.clone(), tooltip: a.description.clone() });
                }
            }
            None
        };
        render_html(md, &resolve, &|t| self.gap_href(t))
    }

    /// HTML của bài trên web: nội dung + hình minh hoạ trong `minh-hoa/<slug>.md` (nếu có).
    fn page_html(&self, slug: &str, md: &str, with_articles: bool) -> String {
        let html = self.render_body(md, with_articles);
        match self.data.figures.for_slug(slug) {
            Some(f) => crate::minh_hoa::insert(&html, f, &|m| self.render_body(m, with_articles)),
            None => html,
        }
    }

    fn toc(&self, md: &str) -> Vec<Json> {
        let mut toc: Vec<Json> = Vec::new();
        for h in extract_headings(md) {
            if h.depth == 2 || toc.is_empty() {
                toc.push(json!({"text": h.text, "id": h.id, "children": []}));
            } else if let Some(Json::Array(children)) = toc.last_mut().and_then(|t| t.get_mut("children")) {
                children.push(json!({"text": h.text, "id": h.id}));
            }
        }
        toc
    }

    fn faq_ld(&self, md: &str) -> Option<Json> {
        let faq = extract_faq(md);
        (!faq.is_empty()).then(|| {
            json!({"@context": "https://schema.org", "@type": "FAQPage",
                "mainEntity": faq.iter().map(|f| json!({"@type": "Question", "name": f.question, "acceptedAnswer": {"@type": "Answer", "text": f.answer}})).collect::<Vec<_>>()})
        })
    }

    fn breadcrumb_ld(&self, crumbs: &[Json]) -> Json {
        json!({"@context": "https://schema.org", "@type": "BreadcrumbList",
            "itemListElement": crumbs.iter().enumerate().map(|(i, c)| json!({"@type": "ListItem", "position": i + 1, "name": c["name"], "item": self.abs_href(c["href"].as_str().unwrap_or(""))})).collect::<Vec<_>>()})
    }

    /// Đường dẫn đã có base → URL tuyệt đối.
    fn abs_href(&self, href: &str) -> String {
        match &self.site_url {
            Some(s) => format!("{s}{href}"),
            None => href.to_string(),
        }
    }

    fn graph_data(&self, focus: Option<&str>) -> String {
        let notes: Vec<&Note> = if self.dev || self.review.is_some() { self.kb().all.iter().collect() } else { self.kb().notes() };
        let notes: Vec<&Note> = match focus.and_then(|f| self.kb().get(f)) {
            Some(center) => {
                let ids: HashSet<&str> = std::iter::once(center.slug.as_str())
                    .chain(center.links.iter().map(|s| s.as_str()))
                    .chain(center.backlinks.iter().map(|s| s.as_str()))
                    .collect();
                notes.into_iter().filter(|n| ids.contains(n.slug.as_str())).collect()
            }
            None => notes,
        };
        let ids: HashSet<&str> = notes.iter().map(|n| n.slug.as_str()).collect();
        let edges: Vec<[&str; 2]> = notes.iter().flat_map(|n| n.links.iter().filter(|t| ids.contains(t.as_str())).map(move |t| [n.slug.as_str(), t.as_str()])).collect();
        json!({
            "focus": focus,
            "nodes": notes.iter().map(|n| json!({"id": n.slug, "title": n.title, "type": n.kind, "url": self.note_url(&n.slug), "inbox": n.inbox})).collect::<Vec<_>>(),
            "edges": edges,
        })
        .to_string()
    }

    // ---------- Danh sách đường dẫn ----------

    pub fn routes(&self) -> Vec<String> {
        let mut r: Vec<String> = [
            "", "kien-thuc/", "bai-viet/", "tu-dien/", "graph/", "search/", "ai/", "gioi-thieu/", "gaps/", "tags/", "llms.txt",
            "llms-full.txt", "knowledge.json", "search-index.json", "sitemap.xml", "robots.txt", "feed.xml", "404.html",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        r.extend(self.indexnow_file());
        r.extend(NOTE_TYPES.iter().map(|t| format!("types/{}/", t.key)));
        r.extend(self.kb().tags.iter().map(|t| format!("tags/{}/", t.slug)));
        let notes: Vec<&Note> = if self.dev { self.kb().all.iter().collect() } else { self.kb().notes() };
        for n in notes {
            r.push(format!("notes/{}/", n.slug));
            if !n.inbox {
                r.push(format!("notes/{}.md", n.slug));
            }
        }
        r.extend(self.data.articles.all.iter().map(|a| format!("bai-viet/{}/", a.slug)));
        r
    }

    /// File khoá IndexNow `<khoá>.txt` (nội dung là chính khoá) — để Bing… xác nhận yêu cầu báo trang mới là của web này.
    fn indexnow_file(&self) -> Option<String> {
        let k = &self.data.config.indexnow_key;
        crate::config::valid_indexnow_key(k).then(|| format!("{k}.txt"))
    }

    /// Đường dẫn của Khu duyệt bài (chỉ dùng với site đã `into_review`).
    pub fn review_routes(&self) -> Vec<String> {
        let mut r = vec!["duyet/".to_string()];
        r.extend(self.kb().inbox().iter().map(|n| format!("duyet/notes/{}/", n.slug)));
        r.extend(self.data.articles.drafts().iter().map(|a| format!("duyet/bai-viet/{}/", a.slug)));
        r
    }

    pub fn asset(&self, route: &str) -> Option<Page> {
        ASSETS.iter().find(|(p, _, _)| *p == route).map(|(_, ct, body)| Page { status: 200, content_type: ct, body: body.to_vec() })
    }

    /// Dựng một trang theo đường dẫn (không có base, không có "/" đầu).
    pub fn render(&self, route: &str) -> Option<Page> {
        CURRENT_ROUTE.with(|r| *r.borrow_mut() = Some(route.to_string()));
        let page = self.render_route(route);
        CURRENT_ROUTE.with(|r| *r.borrow_mut() = None);
        page
    }

    fn render_route(&self, route: &str) -> Option<Page> {
        if let Some(p) = self.asset(route) {
            return Some(p);
        }
        if let Some(key) = self.indexnow_file() {
            if route == key {
                return Some(Page::text("text/plain; charset=utf-8", self.data.config.indexnow_key.clone()));
            }
        }
        let page = match route {
            "" => Page::html(self.home()),
            "kien-thuc/" => Page::html(self.hub()),
            "bai-viet/" => Page::html(self.articles_index()),
            "tu-dien/" => Page::html(self.glossary()),
            "graph/" => Page::html(self.graph_page()),
            "search/" => Page::html(self.search_page()),
            "ai/" => Page::html(self.ai_page()),
            "gioi-thieu/" => Page::html(self.about()),
            "gaps/" => Page::html(self.gaps()),
            "tags/" => Page::html(self.tags_index()),
            "404.html" => Page::html(self.render_template("404.html", json!({"noindex": true}))),
            "llms.txt" => Page::text("text/plain; charset=utf-8", self.llms_txt()),
            "llms-full.txt" => Page::text("text/plain; charset=utf-8", self.llms_full()),
            "knowledge.json" => Page::text("application/json; charset=utf-8", self.knowledge_json()),
            "search-index.json" => Page::text("application/json; charset=utf-8", self.search_index()),
            "sitemap.xml" => Page::text("application/xml; charset=utf-8", self.sitemap()),
            "robots.txt" => Page::text("text/plain; charset=utf-8", self.robots()),
            "feed.xml" => Page::text("application/atom+xml; charset=utf-8", self.feed()),
            r => {
                let parts: Vec<&str> = r.trim_end_matches('/').split('/').collect();
                match (parts.as_slice(), r.ends_with('/')) {
                    (["duyet"], true) if self.review.is_some() => Page::html(self.review_index()),
                    (["duyet", "notes", slug], true) if self.review.is_some() => {
                        Page::html(self.note_page(self.kb().get(slug).filter(|n| n.inbox)?))
                    }
                    (["duyet", "bai-viet", slug], true) if self.review.is_some() => {
                        Page::html(self.article_page(self.data.articles.get(slug).filter(|a| !a.is_published())?))
                    }
                    (["notes", file], false) if file.ends_with(".md") => {
                        let n = self.kb().get(file.trim_end_matches(".md")).filter(|n| !n.inbox)?;
                        Page::text("text/markdown; charset=utf-8", self.note_markdown(n))
                    }
                    (["notes", slug], true) => {
                        let n = self.kb().get(slug).filter(|n| self.dev || !n.inbox)?;
                        Page::html(self.note_page(n))
                    }
                    (["bai-viet", slug], true) => Page::html(self.article_page(self.data.articles.get(slug)?)),
                    (["types", t], true) => {
                        let info = schema::type_info(t)?;
                        let notes: Vec<Json> = self.kb().by_type(t).iter().map(|n| self.note_card(n)).collect();
                        Page::html(self.render_template("list.html", json!({
                            "title": info.label, "description": info.description, "section": "types", "data_type": t,
                            "badge": format!("{} ghi chú", notes.len()), "lead": info.description, "notes": notes, "show_type": false,
                            "empty": format!("Chưa có ghi chú loại này. Nhờ Claude: \"nghiên cứu …\" hoặc tạo trên Bàn làm việc."),
                        })))
                    }
                    (["tags", t], true) => {
                        let tag = self.kb().tags.iter().find(|x| x.slug == *t)?;
                        let notes: Vec<Json> = self.kb().by_tag(t).iter().map(|n| self.note_card(n)).collect();
                        Page::html(self.render_template("list.html", json!({
                            "title": format!("#{}", tag.name), "section": "tags", "lead": format!("{} ghi chú", notes.len()),
                            "notes": notes, "show_type": true, "empty": "Chưa có ghi chú nào.",
                        })))
                    }
                    _ => return None,
                }
            }
        };
        Some(page)
    }

    // ---------- Trang HTML ----------

    fn home(&self) -> String {
        let kb = self.kb();
        let types: Vec<Json> = NOTE_TYPES
            .iter()
            .map(|t| json!({"key": t.key, "label": t.label, "description": t.description, "count": kb.by_type(t.key).len(), "url": self.url(&format!("types/{}/", t.key))}))
            .collect();
        let mut topics = kb.by_type("topic");
        topics.sort_by(|a, b| vi_cmp(&a.title, &b.title));
        let pillars: Vec<Json> = topics
            .iter()
            .take(6)
            .enumerate()
            .map(|(i, n)| json!({"index": format!("{:02}", i + 1), "title": n.title, "summary": n.summary, "url": self.url(&format!("kien-thuc/#{}", n.slug)), "count": n.links.len()}))
            .collect();
        let mut foundations: Vec<&Note> = kb.notes().into_iter().filter(|n| n.kind != "topic" && !n.backlinks.is_empty()).collect();
        foundations.sort_by(|a, b| b.backlinks.len().cmp(&a.backlinks.len()));
        let issues: Vec<Json> = kb
            .issues
            .iter()
            .chain(self.data.articles.issues.iter())
            .map(|i| json!({"path": i.path, "message": i.message, "error": i.level == Level::Error}))
            .collect();
        let errors = issues.iter().filter(|i| i["error"] == true).count();
        let cfg = &self.data.config;
        let ld = json!({"@context": "https://schema.org", "@graph": [
            {"@type": "WebSite", "@id": format!("{}#website", self.abs("")), "name": cfg.name, "url": self.abs(""),
             "description": cfg.description, "inLanguage": cfg.language, "publisher": {"@id": format!("{}#org", self.abs(""))}},
            self.org_ld(),
        ]});
        self.render_template("home.html", json!({
            "section": "home",
            "json_ld": if self.dev { Json::Null } else { json!(json_ld(&ld)) },
            "stats": {"notes": kb.published.len(), "verified": kb.notes().iter().filter(|n| n.status == "verified").count(), "articles": self.data.articles.published().len(), "gaps": kb.gaps.len()},
            "issues": if self.dev { issues.iter().take(8).cloned().collect::<Vec<_>>() } else { vec![] },
            "issues_total": issues.len(), "errors": errors,
            "types": types,
            "inbox": kb.inbox().iter().map(|n| self.note_card(n)).collect::<Vec<_>>(),
            "draft_articles": self.data.articles.drafts().iter().map(|a| self.article_card(a)).collect::<Vec<_>>(),
            "pillars": pillars,
            "foundations": foundations.iter().take(3).map(|n| self.note_card(n)).collect::<Vec<_>>(),
            "recent": kb.notes().iter().take(6).map(|n| self.note_card(n)).collect::<Vec<_>>(),
            "articles": self.data.articles.published().iter().take(4).map(|a| self.article_card(a)).collect::<Vec<_>>(),
            "tags": kb.tags.iter().take(30).map(|t| json!({"name": t.name, "url": self.tag_url(&t.name), "count": t.notes.len()})).collect::<Vec<_>>(),
        }))
    }

    fn note_page(&self, note: &Note) -> String {
        let kb = self.kb();
        let backlinks: Vec<&Note> = note.backlinks.iter().filter_map(|s| kb.get(s)).collect();
        let outgoing: Vec<&Note> = note.links.iter().filter_map(|s| kb.get(s)).collect();
        let mut related: Vec<(usize, &Note)> = kb
            .notes()
            .into_iter()
            .filter(|n| n.slug != note.slug && !note.links.contains(&n.slug) && !note.backlinks.contains(&n.slug))
            .map(|n| (n.tags.iter().filter(|t| note.tags.contains(t)).count(), n))
            .filter(|(c, _)| *c > 0)
            .collect();
        related.sort_by(|a, b| b.0.cmp(&a.0));
        let original = note.updates.as_deref().and_then(|s| kb.get(s));
        let with_drafts = self.dev || self.review.is_some();
        let place = kb.place(note, with_drafts);
        let parent = place.as_ref().map(|p| p.topic);
        let crumbs = vec![
            json!({"name": "Trang chủ", "href": self.url("")}),
            match parent {
                Some(p) => json!({"name": p.title, "href": self.note_url(&p.slug)}),
                None => json!({"name": type_label(&note.kind), "href": self.url(&format!("types/{}/", note.kind))}),
            },
            json!({"name": note.title, "href": self.note_url(&note.slug)}),
        ];
        // Bài trước / bài tiếp theo thứ tự đọc của chủ đề; bài đầu thì "bài trước" là chính chủ đề (bài giới thiệu).
        let lesson = |n: &Note, topic: &Note| {
            let mut c = self.note_card(n);
            c["level_name"] = json!(level_in(topic, &n.slug).map(level_name));
            c
        };
        let lesson_nav = place.as_ref().map(|p| {
            json!({
                "topic": self.link(p.topic),
                "prev": p.prev.map(|n| lesson(n, p.topic)).unwrap_or_else(|| {
                    let mut c = self.note_card(p.topic);
                    c["level_name"] = json!(if p.topic.learning_path.is_empty() { "Chủ đề chính" } else { level_name(1) });
                    c
                }),
                "next": p.next.map(|n| lesson(n, p.topic)),
            })
        });
        // Thẻ "Lộ trình học" 6 cấp ở đầu trang chủ đề.
        let ladder: Vec<Json> = if note.learning_path.is_empty() {
            vec![]
        } else {
            (1..=6)
                .map(|lv| {
                    let found: Vec<_> = note.learning_path.iter().filter(|l| l.level == lv).collect();
                    let shown = |s: &&String| kb.get(s).is_some_and(|n| with_drafts || !n.inbox);
                    let count = found.iter().flat_map(|l| l.notes.iter()).filter(shown).count();
                    let all = found.iter().map(|l| l.notes.len() + l.gaps.len()).sum::<usize>();
                    let mut parts: Vec<String> = vec![];
                    if lv == 1 {
                        parts.push("Bài này".into());
                    }
                    if count > 0 {
                        parts.push(format!("{count} bài"));
                    }
                    if all > count {
                        parts.push(format!("{} bài còn thiếu", all - count));
                    }
                    if parts.is_empty() {
                        parts.push("Chưa có bài".into());
                    }
                    json!({"level": lv, "name": level_name(lv), "id": found.first().map(|l| l.id.clone()),
                        "status": parts.join(" · "), "empty": lv > 1 && count == 0})
                })
                .collect()
        };
        let sources: Vec<Json> = note
            .sources
            .iter()
            .map(|s| {
                let meta: Vec<String> = [s.kind.as_deref().and_then(source_kind_label).map(String::from), s.author.clone(), s.publisher.clone(), s.date.clone()]
                    .into_iter()
                    .flatten()
                    .collect();
                json!({"title": s.title, "url": s.url, "meta": meta.join(" · "), "accessed": s.accessed.as_deref().map(format_date),
                    "cred": cred_view(s.credibility.map(|c| c as f64), true, None), "credibility_note": s.credibility_note, "note": s.note,
                    "primary": s.primary})
            })
            .collect();
        let articles: Vec<Json> = if note.inbox { vec![] } else { self.data.articles.for_note(&note.slug).iter().map(|a| self.article_card(a)).collect() };
        let headings = extract_headings(&note.body);

        let mut lds = vec![
            json!({
                "@context": "https://schema.org",
                "@type": if note.kind == "concept" { json!(["Article", "DefinedTerm"]) } else { json!("Article") },
                "headline": note.title, "name": note.title, "description": note.summary, "inLanguage": self.data.config.language,
                "datePublished": note.created, "dateModified": note.updated,
                "keywords": if note.tags.is_empty() { Json::Null } else { json!(note.tags.join(", ")) },
                "url": self.abs(&format!("notes/{}/", note.slug)),
                "mainEntityOfPage": self.abs(&format!("notes/{}/", note.slug)),
                "image": self.abs(OG_IMAGE),
                "author": self.author_ld(),
                "publisher": self.org_ld(),
                "citation": note.sources.iter().map(|s| match &s.url { Some(u) => json!({"@type": "CreativeWork", "name": s.title, "url": u}), None => json!(s.title) }).collect::<Vec<_>>(),
            }),
            self.breadcrumb_ld(&crumbs),
        ];
        if let Some(f) = self.faq_ld(&note.body) {
            lds.push(f);
        }
        let file = note.path.rsplit('/').next().unwrap_or("");
        self.render_template("note.html", json!({
            "title": note.title, "description": note.summary, "section": "", "og_type": "article",
            "pub_time": iso_time(&note.created), "mod_time": iso_time(&note.updated),
            "json_ld": if note.inbox { Json::Null } else { json!(json_ld(&json!(lds))) },
            "noindex": note.inbox,
            "md_url": if note.inbox { Json::Null } else { json!(self.url(&format!("notes/{}.md", note.slug))) },
            "note": {
                "card": self.note_card(note), "path": note.path, "file": file, "created": format_date(&note.created),
                "reviewed": note.reviewed.as_deref().map(format_date), "word_count": note.word_count,
                "sources_count": note.sources.len(), "rated": note.credibility.rated, "aliases": note.aliases.join(", "),
                "research_keyword": note.research.as_ref().and_then(|r| r.get("keyword")).and_then(|k| k.as_str()),
                "cred_title": cred_view(note.credibility.average, false, Some(format!("Uy tín nguồn trung bình {}/5 ({} nguồn đã chấm)", note.credibility.average.map(fmt_score).unwrap_or_default(), note.credibility.rated))),
                "inbox": note.inbox, "status": note.status, "md_url": self.url(&format!("notes/{}.md", note.slug)),
            },
            "original": original.map(|o| self.link(o)),
            "crumbs": crumbs, "html": self.page_html(&note.slug, &note.body, false),
            "toc": if headings.len() >= 3 { self.toc(&note.body) } else { vec![] },
            "sources": sources, "parent": parent.map(|p| self.link(p)), "lesson_nav": lesson_nav,
            "place": place.as_ref().map(|p| json!({"topic": self.link(p.topic), "index": p.index, "total": p.total,
                "level": p.level, "level_name": p.level.map(level_name)})),
            "is_topic": note.kind == "topic", "ladder": ladder,
            "read_minutes": (note.word_count as f64 / 220.0).round().max(1.0) as usize,
            "backlinks": backlinks.iter().map(|n| self.link(n)).collect::<Vec<_>>(),
            "outgoing": outgoing.iter().map(|n| self.link(n)).collect::<Vec<_>>(),
            "broken": note.broken_links.iter().map(|b| json!({"name": b, "href": self.gap_href(b)})).collect::<Vec<_>>(),
            "related": related.iter().take(5).map(|(_, n)| self.link(n)).collect::<Vec<_>>(),
            "articles": articles,
            "graph": if note.links.is_empty() && note.backlinks.is_empty() { Json::Null } else { json!(self.graph_data(Some(&note.slug))) },
            "hints": if self.dev { json!(seo::note_hints(note, &self.today)) } else { Json::Null },
            "edit_url": if self.dev { json!(self.url(&format!("ban-lam-viec/sua/?f={}", note.path))) } else { Json::Null },
            "actions": if note.inbox { self.review_actions(&note.path, &note.title) } else { Json::Null },
        }))
    }

    fn article_page(&self, a: &Article) -> String {
        let kb = self.kb();
        let basis: Vec<&Note> = a.notes.iter().filter_map(|s| kb.get(s)).collect();
        let mentioned: Vec<&Note> = a.links.iter().filter_map(|s| kb.get(s)).filter(|n| !a.notes.contains(&n.slug)).collect();
        let rated: Vec<f64> = basis.iter().flat_map(|n| n.sources.iter().filter_map(|s| s.credibility.map(|c| c as f64))).collect();
        let avg = (!rated.is_empty()).then(|| ((rated.iter().sum::<f64>() / rated.len() as f64) * 10.0).round() / 10.0);
        let published = a.published_on.clone().unwrap_or_else(|| a.created.clone());
        let crumbs = vec![
            json!({"name": "Trang chủ", "href": self.url("")}),
            json!({"name": "Bài viết", "href": self.url("bai-viet/")}),
            json!({"name": a.title, "href": self.article_url(&a.slug)}),
        ];
        let mut lds = vec![
            json!({
                "@context": "https://schema.org", "@type": "Article", "headline": a.title, "description": a.description,
                "inLanguage": self.data.config.language, "datePublished": published, "dateModified": a.updated,
                "keywords": std::iter::once(&a.keyword).chain(a.keywords.iter()).cloned().collect::<Vec<_>>().join(", "),
                "url": self.abs(&format!("bai-viet/{}/", a.slug)),
                "mainEntityOfPage": self.abs(&format!("bai-viet/{}/", a.slug)),
                "image": self.abs(OG_IMAGE),
                "author": self.author_ld(),
                "publisher": self.org_ld(),
                "isBasedOn": basis.iter().map(|n| json!({"@type": "CreativeWork", "name": n.title, "url": self.abs(&format!("notes/{}/", n.slug))})).collect::<Vec<_>>(),
                "about": basis.iter().map(|n| json!({"@type": "Thing", "name": n.title, "url": self.abs(&format!("notes/{}/", n.slug))})).collect::<Vec<_>>(),
            }),
            self.breadcrumb_ld(&crumbs),
        ];
        if let Some(f) = self.faq_ld(&a.body) {
            lds.push(f);
        }
        let headings = extract_headings(&a.body);
        let draft = !a.is_published();
        let extra = format!(
            "Từ khoá chính: “{}”{} · ý định: {}",
            a.keyword,
            if a.keywords.is_empty() { String::new() } else { format!(" · phụ: {}", a.keywords.join(", ")) },
            intent_label(&a.intent)
        );
        self.render_template("article.html", json!({
            "title": a.title, "description": a.description, "section": "articles", "og_type": "article",
            "pub_time": iso_time(&published), "mod_time": iso_time(&a.updated),
            "json_ld": if draft { Json::Null } else { json!(json_ld(&json!(lds))) }, "noindex": draft,
            "a": self.article_card(a), "path": a.path, "draft": draft, "crumbs": crumbs,
            "read_minutes": (a.word_count as f64 / 220.0).round().max(1.0) as usize, "word_count": a.word_count,
            "published": format_date(&published),
            "toc": if headings.len() >= 3 { self.toc(&a.body) } else { vec![] },
            "html": self.page_html(&a.slug, &a.body, true),
            "basis": basis.iter().map(|n| { let mut c = self.note_card(n); c["sources_count"] = json!(n.sources.len()); c }).collect::<Vec<_>>(),
            "mentioned": mentioned.iter().map(|n| self.link(n)).collect::<Vec<_>>(),
            "related": self.data.articles.related(a, 4).iter().map(|x| self.article_card(x)).collect::<Vec<_>>(),
            "sources": basis.iter().map(|n| n.sources.len()).sum::<usize>(), "avg_cred": avg.map(|s| cred_view(Some(s), false, None)),
            "hints": if self.dev { json!(seo::article_hints(a, kb)) } else { Json::Null }, "hints_extra": extra,
            "edit_url": if self.dev { json!(self.url(&format!("ban-lam-viec/sua/?f={}", a.path))) } else { Json::Null },
            "actions": if draft { self.review_actions(&a.path, &a.title) } else { Json::Null },
        }))
    }

    // ---------- Khu duyệt bài trên web ----------

    /// Nút cho một bản nháp ở Khu duyệt bài. Duyệt/Bỏ mở trang tạo issue GitHub có tiêu đề định sẵn;
    /// workflow duyet.yml đọc tiêu đề đó (chỉ khi chủ repo tạo) để đăng hoặc bỏ bài.
    fn review_actions(&self, path: &str, title: &str) -> Json {
        let Some(r) = &self.review else { return Json::Null };
        let prompt = format!("Sửa bản nháp \"{title}\" ({path}): ");
        let Some(repo) = &r.repo else { return json!({"path": path, "prompt": prompt}) };
        let approve_body = format!(
            "Bấm nút **Create** (Tạo) màu xanh bên dưới để đăng bài **{title}** (`{path}`).\n\n\
             GitHub sẽ tự đăng bài, đưa lên web (khoảng 3–5 phút) rồi trả lời và đóng mục này. Không cần làm gì thêm."
        );
        let remove_body = format!(
            "Bấm nút **Create** (Tạo) màu xanh bên dưới để bỏ bản nháp **{title}** (`{path}`).\n\n\
             Bản nháp bị xoá khỏi danh sách chờ duyệt; vẫn khôi phục được từ lịch sử GitHub nếu cần."
        );
        json!({
            "path": path,
            "prompt": prompt,
            "approve": issue_url(repo, &format!("Duyệt: {path}"), &approve_body),
            "remove": issue_url(repo, &format!("Bỏ: {path}"), &remove_body),
            "edit": r.branch.as_ref().map(|b| format!("https://github.com/{repo}/edit/{}/{path}", url_encode_path(b))),
        })
    }

    fn review_index(&self) -> String {
        let kb = self.kb();
        let card = |mut c: Json, path: &str, title: &str, errors: Vec<String>| {
            c["actions"] = self.review_actions(path, title);
            c["errors"] = json!(errors);
            c
        };
        let errors_for = |path: &str| -> Vec<String> {
            kb.issues.iter().chain(self.data.articles.issues.iter()).filter(|i| i.path == path && i.level == Level::Error).map(|i| i.message.clone()).collect()
        };
        let groups: Vec<Json> = kb
            .inbox_by_topic()
            .iter()
            .map(|g| {
                let mut items: Vec<Json> = vec![];
                if let Some(t) = g.topic.filter(|t| t.inbox) {
                    let mut c = card(self.note_card(t), &t.path, &t.title, errors_for(&t.path));
                    c["role"] = json!("⭐ Chủ đề chính · đọc trước");
                    c["main"] = json!(true);
                    items.push(c);
                }
                for n in &g.notes {
                    let mut c = card(self.note_card(n), &n.path, &n.title, errors_for(&n.path));
                    c["role"] = json!(kb.role_label(n));
                    items.push(c);
                }
                json!({
                    "title": g.topic.map(|t| t.title.clone()),
                    "url": g.topic.map(|t| self.note_url(&t.slug)),
                    "topic_published": g.topic.is_some_and(|t| !t.inbox),
                    "items": items,
                })
            })
            .collect();
        let notes_count = kb.inbox().len();
        let articles: Vec<Json> = self.data.articles.drafts().iter().map(|a| card(self.article_card(a), &a.path, &a.title, errors_for(&a.path))).collect();
        // Bản nháp lỗi nặng tới mức không đọc được (không có trong danh sách trên).
        let listed: HashSet<&str> = kb.inbox().iter().map(|n| n.path.as_str()).chain(self.data.articles.drafts().iter().map(|a| a.path.as_str())).collect();
        let mut broken: Vec<Json> = vec![];
        for i in kb.issues.iter().chain(self.data.articles.issues.iter()) {
            if i.draft && i.level == Level::Error && !listed.contains(i.path.as_str()) {
                broken.push(json!({"path": i.path, "message": i.message, "prompt": format!("Sửa lỗi bản nháp {}: {}", i.path, i.message)}));
            }
        }
        let repo = self.review.as_ref().and_then(|r| r.repo.clone());
        let total = notes_count + articles.len();
        let approve_all = repo.as_ref().filter(|_| total > 1).map(|repo| {
            issue_url(repo, "Duyệt tất cả bản nháp", &format!(
                "Bấm nút **Create** (Tạo) màu xanh bên dưới để đăng **tất cả {total} bản nháp** đang chờ duyệt.\n\n\
                 GitHub sẽ tự đăng (ghi chú trước, bài viết sau), đưa lên web rồi trả lời và đóng mục này."
            ))
        });
        self.render_template("review.html", json!({
            "title": "Khu duyệt bài", "description": "Bản nháp chờ duyệt — chưa kiểm chứng.", "section": "review", "noindex": true,
            "groups": groups, "notes_count": notes_count, "articles": articles, "broken": broken, "total": total,
            "approve_all": approve_all,
            "repo_url": repo.as_ref().map(|r| format!("https://github.com/{r}")),
            "issues_url": repo.as_ref().map(|r| format!("https://github.com/{r}/issues?q=is%3Aissue")),
        }))
    }

    fn articles_index(&self) -> String {
        let groups: Vec<Json> = INTENTS
            .iter()
            .map(|(k, label, _)| (label, self.data.articles.all.iter().filter(|a| a.intent == *k).map(|a| self.article_card(a)).collect::<Vec<_>>()))
            .filter(|(_, items)| !items.is_empty())
            .map(|(label, items)| json!({"label": label, "items": items}))
            .collect();
        self.render_template("articles.html", json!({"title": "Bài viết", "section": "articles",
            "description": "Bài viết dễ đọc theo những câu hỏi người đọc hay tìm — mỗi bài dựa trên các ghi chú tri thức có nguồn.", "groups": groups}))
    }

    fn hub(&self) -> String {
        let kb = self.kb();
        let mut topics = kb.by_type("topic");
        topics.sort_by(|a, b| vi_cmp(&a.title, &b.title));
        let pillars: Vec<Json> = topics
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let children: Vec<&Note> = t.links.iter().filter_map(|s| kb.get(s)).filter(|n| !n.inbox).collect();
                let groups: Vec<Json> = NOTE_TYPES
                    .iter()
                    .map(|ty| (ty, children.iter().filter(|n| n.kind == ty.key).map(|n| self.link(n)).collect::<Vec<_>>()))
                    .filter(|(_, list)| !list.is_empty())
                    .map(|(ty, list)| json!({"type": ty.key, "label": ty.label, "notes": list}))
                    .collect();
                json!({"index": i + 1, "index_pad": format!("{:02}", i + 1), "slug": t.slug, "title": t.title, "summary": t.summary,
                    "url": self.note_url(&t.slug), "count": children.len(), "groups": groups,
                    "broken": t.broken_links.iter().map(|b| json!({"name": b, "href": self.gap_href(b)})).collect::<Vec<_>>()})
            })
            .collect();
        let covered: HashSet<&str> = topics.iter().flat_map(|t| std::iter::once(t.slug.as_str()).chain(t.links.iter().map(|s| s.as_str()))).collect();
        let unassigned: Vec<Json> = kb
            .notes()
            .into_iter()
            .filter(|n| !covered.contains(n.slug.as_str()))
            .map(|n| json!({"title": n.title, "url": self.note_url(&n.slug), "type_label": type_label(&n.kind)}))
            .collect();
        self.render_template("hub.html", json!({
            "title": "Trung tâm kiến thức", "section": "hub",
            "description": "Toàn bộ tri thức được tổ chức theo các chủ đề trụ cột, mỗi chủ đề dẫn tới cụm ghi chú liên quan.",
            "pillars": pillars, "unassigned": unassigned,
            "types": NOTE_TYPES.iter().map(|t| json!({"key": t.key, "label": t.label, "count": kb.by_type(t.key).len(), "url": self.url(&format!("types/{}/", t.key))})).collect::<Vec<_>>(),
            "tags": kb.tags.iter().map(|t| json!({"name": t.name, "url": self.tag_url(&t.name), "count": t.notes.len()})).collect::<Vec<_>>(),
            "gaps": kb.gaps.len(),
        }))
    }

    fn glossary(&self) -> String {
        struct Entry<'a> {
            term: String,
            note: &'a Note,
            alias_of: Option<String>,
        }
        let mut entries: Vec<Entry> = Vec::new();
        for n in self.kb().notes().into_iter().filter(|n| ["concept", "framework", "topic"].contains(&n.kind.as_str())) {
            entries.push(Entry { term: n.title.clone(), note: n, alias_of: None });
            for a in &n.aliases {
                if strip_diacritics(a).to_lowercase() != strip_diacritics(&n.title).to_lowercase() {
                    entries.push(Entry { term: a.clone(), note: n, alias_of: Some(n.title.clone()) });
                }
            }
        }
        entries.sort_by(|a, b| vi_cmp(&a.term, &b.term));
        let letter_of = |s: &str| {
            let c = strip_diacritics(s.trim()).chars().next().map(|c| c.to_ascii_uppercase()).unwrap_or('#');
            if c.is_ascii_uppercase() {
                c.to_string()
            } else {
                "#".into()
            }
        };
        let mut groups: Vec<(String, Vec<Json>)> = Vec::new();
        for e in &entries {
            let l = letter_of(&e.term);
            let item = json!({"term": e.term, "url": self.note_url(&e.note.slug), "type": e.note.kind, "type_label": type_label(&e.note.kind), "alias_of": e.alias_of, "summary": e.note.summary});
            match groups.iter_mut().find(|g| g.0 == l) {
                Some(g) => g.1.push(item),
                None => groups.push((l, vec![item])),
            }
        }
        let present: HashSet<String> = groups.iter().map(|g| g.0.clone()).collect();
        let letters: Vec<Json> = ('A'..='Z').map(|c| json!({"l": c.to_string(), "present": present.contains(&c.to_string())})).collect();
        let ld = json!({"@context": "https://schema.org", "@type": "DefinedTermSet", "name": format!("Từ điển {}", self.data.config.name),
            "hasDefinedTerm": entries.iter().filter(|e| e.alias_of.is_none()).map(|e| json!({"@type": "DefinedTerm", "name": e.term, "description": e.note.summary, "url": self.abs(&format!("notes/{}/", e.note.slug))})).collect::<Vec<_>>()});
        self.render_template("glossary.html", json!({
            "title": "Từ điển A–Z", "section": "glossary", "json_ld": json_ld(&ld),
            "description": "Mục lục A–Z toàn bộ khái niệm, mô hình và chủ đề trong kho — mỗi thuật ngữ một dòng định nghĩa.",
            "letters": letters, "has_other": present.contains("#"),
            "groups": groups.into_iter().map(|(l, items)| json!({"letter": l, "id": if l == "#" { "az-other".to_string() } else { format!("az-{l}") }, "entries": items})).collect::<Vec<_>>(),
        }))
    }

    fn graph_page(&self) -> String {
        let kb = self.kb();
        let edges: usize = kb.notes().iter().map(|n| n.links.len()).sum();
        let present: HashSet<&str> = kb.all.iter().map(|n| n.kind.as_str()).collect();
        self.render_template("graph.html", json!({
            "title": "Đồ thị tri thức", "section": "graph", "description": "Bản đồ liên kết giữa toàn bộ ghi chú trong kho.",
            "count": kb.published.len(), "edges": edges, "graph": self.graph_data(None),
            "legend": NOTE_TYPES.iter().filter(|t| present.contains(t.key)).map(|t| json!({"key": t.key, "label": t.label})).collect::<Vec<_>>(),
            "orphans": kb.orphans().iter().map(|n| self.link(n)).collect::<Vec<_>>(),
        }))
    }

    fn search_page(&self) -> String {
        let mut types: Vec<Json> = NOTE_TYPES.iter().map(|t| json!({"key": t.key, "label": t.label})).collect();
        types.push(json!({"key": "article", "label": "Bài viết"}));
        let labels: serde_json::Map<String, Json> = types.iter().map(|t| (t["key"].as_str().unwrap().to_string(), t["label"].clone())).collect();
        let statuses: serde_json::Map<String, Json> = STATUSES.iter().map(|s| (s.key.to_string(), json!(s.label))).collect();
        self.render_template("search.html", json!({"title": "Tìm kiếm", "section": "search", "types": types,
            "types_json": Json::Object(labels).to_string(), "statuses_json": Json::Object(statuses).to_string()}))
    }

    fn ai_page(&self) -> String {
        let example = self.kb().by_type("topic").first().map(|n| n.slug.clone()).or_else(|| self.kb().notes().first().map(|n| n.slug.clone())).unwrap_or_else(|| "ten-ghi-chu".into());
        let endpoints = [
            ("llms.txt", "llms.txt — chỉ mục", "Danh sách mọi ghi chú theo loại, kèm tóm tắt, trạng thái và quy tắc sử dụng. Agent nên đọc file này đầu tiên."),
            ("llms-full.txt", "llms-full.txt — toàn bộ nội dung", "Tất cả ghi chú trong một file, phù hợp khi kho còn nhỏ hoặc agent có context lớn."),
            ("knowledge.json", "knowledge.json — dữ liệu có cấu trúc", "Metadata, nguồn, liên kết, backlinks và link tới bản .md của từng ghi chú — dùng cho tool/script tự lọc theo type, status, tags rồi chỉ tải các ghi chú cần."),
            (&format!("notes/{example}.md"), "notes/<slug>.md — từng ghi chú", "Markdown thuần của một ghi chú (wikilink đã đổi thành link chuẩn). Mỗi trang ghi chú đều có bản này."),
        ];
        let prompt = format!(
            "Bạn là người viết content. Nguồn tri thức duy nhất được phép dùng cho dữ kiện là {} tại:\n{}\n\nQuy trình:\n1. Đọc llms.txt để xem kho có những ghi chú nào.\n2. Chọn các ghi chú liên quan tới đề bài (ưu tiên ghi chú loại \"Chủ đề\" trước, rồi đi theo liên kết), đọc bản .md của chúng.\n3. Lập dàn ý dựa trên kiến thức trong kho, sau đó viết bài.\n4. Cuối bài, liệt kê các ghi chú và nguồn đã dùng.\n\nQuy tắc:\n{}\n\nĐề bài: <mô tả bài cần viết, kênh đăng, độ dài, giọng văn>",
            self.data.config.name,
            self.abs("llms.txt"),
            self.data.config.agent_rules.iter().map(|r| format!("- {r}")).collect::<Vec<_>>().join("\n")
        );
        self.render_template("ai.html", json!({
            "title": "Cho AI agent", "section": "ai", "prompt": prompt, "absolute": self.site_url.is_some(),
            "endpoints": endpoints.iter().map(|(p, t, d)| json!({"url": self.url(p), "abs": self.abs(p), "title": t, "desc": d})).collect::<Vec<_>>(),
        }))
    }

    fn about(&self) -> String {
        let kb = self.kb();
        let ld = json!({"@context": "https://schema.org", "@type": "AboutPage", "name": format!("Giới thiệu {}", self.data.config.name),
            "description": "Cách tri thức trong kho được nghiên cứu, chấm điểm nguồn, kiểm chứng, đăng và cập nhật.",
            "inLanguage": self.data.config.language, "url": self.abs("gioi-thieu/")});
        self.render_template("about.html", json!({
            "title": "Giới thiệu & quy trình biên tập", "section": "about", "json_ld": json_ld(&ld),
            "description": "Cách tri thức trong kho được nghiên cứu, chấm điểm nguồn, kiểm chứng, đăng và cập nhật.",
            "stats": {"notes": kb.published.len(), "verified": kb.notes().iter().filter(|n| n.status == "verified").count(),
                "sources": kb.notes().iter().map(|n| n.sources.len()).sum::<usize>(), "articles": self.data.articles.published().len()},
            "statuses": STATUSES.iter().map(|s| json!({"key": s.key, "label": s.label, "description": s.description})).collect::<Vec<_>>(),
            "scale": schema::CREDIBILITY.iter().map(|(s, _, d)| json!({"cred": cred_view(Some(*s as f64), true, None), "description": d})).collect::<Vec<_>>(),
        }))
    }

    fn gaps(&self) -> String {
        let kb = self.kb();
        let gaps: Vec<Json> = kb
            .gaps
            .iter()
            .map(|g| json!({"slug": g.slug, "name": g.name, "from": g.from.iter().filter_map(|s| kb.get(s)).map(|n| self.link(n)).collect::<Vec<_>>()}))
            .collect();
        self.render_template("gaps.html", json!({"title": "Tri thức còn thiếu", "section": "hub", "noindex": true,
            "description": "Các khái niệm đã được nhắc tới bằng [[liên kết]] nhưng chưa có ghi chú — hàng đợi nghiên cứu tiếp theo.", "gaps": gaps}))
    }

    fn tags_index(&self) -> String {
        self.render_template("tags.html", json!({"title": "Chủ đề (tag)", "section": "tags",
            "tags": self.kb().tags.iter().map(|t| json!({"name": t.name, "url": self.tag_url(&t.name), "count": t.notes.len()})).collect::<Vec<_>>()}))
    }

    // ---------- Đầu ra cho AI ----------

    fn md_url(&self, n: &Note) -> String {
        self.abs(&format!("notes/{}.md", n.slug))
    }

    fn portable(&self, n: &Note) -> String {
        to_portable_markdown(&n.body, &|t| self.kb().resolve(t).filter(|x| !x.inbox).map(|x| (self.md_url(x), x.title.clone())))
    }

    fn status_text(n: &Note) -> String {
        status_info(&n.status).label.to_lowercase()
    }

    fn metadata(&self, n: &Note) -> Json {
        let mut m = json!({
            "slug": n.slug, "title": n.title, "type": n.kind, "type_label": type_label(&n.kind), "status": n.status,
            "summary": n.summary, "tags": n.tags, "aliases": n.aliases, "sources": n.sources, "created": n.created, "updated": n.updated,
            "url": self.abs(&format!("notes/{}/", n.slug)), "markdown_url": self.md_url(n), "links": n.links, "backlinks": n.backlinks,
        });
        if !n.extra.is_empty() {
            m["extra"] = Json::Object(n.extra.clone());
        }
        if !n.learning_path.is_empty() {
            // Lộ trình học 6 cấp: chỉ giữ bài đã đăng, bài còn thiếu ghi riêng.
            let kb = self.kb();
            m["learning_path"] = json!(n.learning_path.iter().map(|l| json!({
                "level": l.level, "name": level_name(l.level),
                "notes": l.notes.iter().filter(|s| kb.get(s).is_some_and(|x| !x.inbox)).collect::<Vec<_>>(),
                "missing": l.gaps.iter().cloned().chain(l.notes.iter().filter(|s| kb.get(s).is_some_and(|x| x.inbox)).cloned()).collect::<Vec<_>>(),
            })).collect::<Vec<_>>());
        }
        m
    }

    fn note_markdown(&self, n: &Note) -> String {
        let mut fm = self.metadata(n);
        if let Some(o) = fm.as_object_mut() {
            o.remove("links");
            o.remove("backlinks");
            o.remove("learning_path"); // đã có trong nội dung (mục "Lộ trình học")
        }
        let yaml = serde_yaml::to_string(&fm).unwrap_or_default();
        let mut related: Vec<&Note> = Vec::new();
        for s in n.links.iter().chain(n.backlinks.iter()) {
            if let Some(x) = self.kb().get(s).filter(|x| !x.inbox) {
                if !related.iter().any(|r| r.slug == x.slug) {
                    related.push(x);
                }
            }
        }
        let rel = if related.is_empty() {
            String::new()
        } else {
            format!("\n\n## Ghi chú liên quan\n\n{}", related.iter().map(|r| format!("- [{}]({}): {}", r.title, self.md_url(r), r.summary)).collect::<Vec<_>>().join("\n"))
        };
        format!("---\n{}---\n\n# {}\n\n> {}\n\n{}{}\n", yaml, n.title, n.summary, self.portable(n), rel)
    }

    fn rules_block(&self) -> Vec<String> {
        self.data.config.agent_rules.iter().map(|r| format!("- {r}")).collect()
    }

    fn llms_txt(&self) -> String {
        let kb = self.kb();
        let mut lines = vec![
            format!("# {}", self.data.config.name),
            String::new(),
            format!("> {}", self.data.config.description),
            String::new(),
            format!("Kho gồm {} ghi chú. Mỗi liên kết dưới đây trỏ tới bản Markdown của ghi chú (có frontmatter: type, status, tags, sources, updated).", kb.published.len()),
            String::new(),
            "Quy tắc khi dùng kho này để viết content:".into(),
            String::new(),
        ];
        lines.extend(self.rules_block());
        for t in NOTE_TYPES {
            let notes = kb.by_type(t.key);
            if notes.is_empty() {
                continue;
            }
            lines.extend([String::new(), format!("## {}", t.label), String::new(), t.description.to_string(), String::new()]);
            for n in notes {
                let mut meta = vec![Self::status_text(n), format!("cập nhật {}", n.updated)];
                if !n.tags.is_empty() {
                    meta.push(format!("tags: {}", n.tags.join(", ")));
                }
                lines.push(format!("- [{}]({}): {} ({})", n.title, self.md_url(n), n.summary, meta.join("; ")));
            }
        }
        let published = self.data.articles.published();
        if !published.is_empty() {
            lines.extend([
                String::new(),
                "## Bài viết".into(),
                String::new(),
                "Bài viết dễ đọc, dẫn xuất từ các ghi chú trên. Khi cần sự thật và nguồn, trích dẫn ghi chú gốc (based on) thay vì bài viết.".into(),
                String::new(),
            ]);
            for a in published {
                lines.push(format!("- [{}]({}): {} (based on: {})", a.title, self.abs(&format!("bai-viet/{}/", a.slug)), a.description, a.notes.join(", ")));
            }
        }
        lines.extend([
            String::new(),
            "## Optional".into(),
            String::new(),
            format!("- [Toàn bộ nội dung kho]({}): tất cả ghi chú trong một file văn bản", self.abs("llms-full.txt")),
            format!("- [Chỉ mục JSON]({}): dữ liệu có cấu trúc gồm metadata, liên kết và link tới bản .md của từng ghi chú", self.abs("knowledge.json")),
            String::new(),
        ]);
        lines.join("\n")
    }

    fn llms_full(&self) -> String {
        let order: Vec<&str> = NOTE_TYPES.iter().map(|t| t.key).collect();
        let mut notes = self.kb().notes();
        notes.sort_by(|a, b| {
            order.iter().position(|t| *t == a.kind).cmp(&order.iter().position(|t| *t == b.kind)).then_with(|| vi_cmp(&a.title, &b.title))
        });
        let mut out = vec![
            format!("# {} — toàn bộ nội dung", self.data.config.name),
            String::new(),
            format!("> {}", self.data.config.description),
            String::new(),
            "Quy tắc khi dùng kho này để viết content:".into(),
            String::new(),
        ];
        out.extend(self.rules_block());
        out.push(String::new());
        for n in notes {
            let sources = if n.sources.is_empty() {
                "\nNguồn: (chưa có)".to_string()
            } else {
                let list: Vec<String> = n
                    .sources
                    .iter()
                    .map(|s| {
                        let parts: Vec<&str> = [Some(s.title.as_str()), s.author.as_deref(), s.publisher.as_deref(), s.date.as_deref()].into_iter().flatten().collect();
                        format!("- {}{}", parts.join(" — "), s.url.as_ref().map(|u| format!(" <{u}>")).unwrap_or_default())
                    })
                    .collect();
                format!("\nNguồn:\n{}", list.join("\n"))
            };
            let tags = if n.tags.is_empty() { String::new() } else { format!(" | Tags: {}", n.tags.join(", ")) };
            out.extend([
                "---".into(),
                String::new(),
                format!("## {}", n.title),
                String::new(),
                format!("Loại: {} | Trạng thái: {} | Cập nhật: {}{}", type_label(&n.kind), Self::status_text(n), n.updated, tags),
                format!("URL: {}", self.md_url(n)),
                String::new(),
                format!("Tóm tắt: {}", n.summary),
                String::new(),
                self.portable(n),
                sources,
                String::new(),
            ]);
        }
        out.join("\n")
    }

    fn knowledge_json(&self) -> String {
        let kb = self.kb();
        let cfg = &self.data.config;
        let data = json!({
            "name": cfg.name, "description": cfg.description, "language": cfg.language,
            "generated_at": chrono::Local::now().to_rfc3339(), "note_count": kb.published.len(), "agent_rules": cfg.agent_rules,
            "types": NOTE_TYPES.iter().map(|t| (t.key.to_string(), json!({"label": t.label, "folder": t.folder, "description": t.description}))).collect::<serde_json::Map<_, _>>(),
            "statuses": STATUSES.iter().map(|s| (s.key.to_string(), json!({"label": s.label, "description": s.description}))).collect::<serde_json::Map<_, _>>(),
            "tags": kb.tags,
            // Không chép nội dung vào đây: đã có ở notes/<slug>.md (markdown_url) và llms-full.txt.
            "notes": kb.notes().iter().map(|n| self.metadata(n)).collect::<Vec<_>>(),
            "articles": self.data.articles.published().iter().map(|a| json!({
                "slug": a.slug, "title": a.title, "description": a.description, "keyword": a.keyword, "intent": a.intent,
                "based_on": a.notes, "created": a.published_on.clone().unwrap_or_else(|| a.created.clone()), "updated": a.updated,
                "url": self.abs(&format!("bai-viet/{}/", a.slug)),
            })).collect::<Vec<_>>(),
        });
        serde_json::to_string(&data).unwrap_or_default()
    }

    fn search_index(&self) -> String {
        // Người xem tải cả file này khi tìm kiếm: chỉ giữ đoạn đầu bài (tiêu đề, tên khác, tóm tắt, tag đã có riêng).
        let clip = |s: String| s.chars().take(SEARCH_TEXT_CHARS).collect::<String>();
        let mut list: Vec<Json> = self
            .kb()
            .notes()
            .iter()
            .map(|n| json!({"url": self.note_url(&n.slug), "slug": n.slug, "title": n.title, "aliases": n.aliases, "type": n.kind, "status": n.status,
                "updated": n.updated, "summary": n.summary, "tags": n.tags, "text": clip(markdown_to_plain(&n.body))}))
            .collect();
        list.extend(self.data.articles.published().iter().map(|a| {
            json!({"url": self.article_url(&a.slug), "slug": a.slug, "title": a.title,
                "aliases": std::iter::once(&a.keyword).chain(a.keywords.iter()).collect::<Vec<_>>(), "type": "article", "status": "",
                "updated": a.updated, "summary": a.description, "tags": [], "text": clip(markdown_to_plain(&a.body))})
        }));
        Json::Array(list).to_string()
    }

    fn sitemap(&self) -> String {
        let kb = self.kb();
        let mut entries: Vec<(String, Option<String>)> = vec![
            (self.url(""), kb.notes().first().map(|n| n.updated.clone()).or_else(|| Some(self.today.clone()))),
            (self.url("kien-thuc/"), None),
            (self.url("bai-viet/"), self.data.articles.published().first().map(|a| a.updated.clone())),
            (self.url("tu-dien/"), None),
            (self.url("graph/"), None),
            (self.url("ai/"), None),
            (self.url("gioi-thieu/"), None),
        ];
        entries.extend(NOTE_TYPES.iter().map(|t| (self.url(&format!("types/{}/", t.key)), None)));
        entries.extend(kb.tags.iter().map(|t| (self.tag_url(&t.name), None)));
        entries.extend(kb.notes().iter().map(|n| (self.note_url(&n.slug), Some(n.updated.clone()))));
        entries.extend(self.data.articles.published().iter().map(|a| (self.article_url(&a.slug), Some(a.updated.clone()))));
        let urls: Vec<String> = entries
            .into_iter()
            .map(|(loc, lastmod)| format!("  <url><loc>{}</loc>{}</url>", self.abs_href(&loc), lastmod.map(|d| format!("<lastmod>{d}</lastmod>")).unwrap_or_default()))
            .collect();
        format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n{}\n</urlset>\n", urls.join("\n"))
    }

    fn robots(&self) -> String {
        let rules = format!("Allow: /\nDisallow: {}\nDisallow: {}\n", self.url("gaps/"), self.url("duyet/"));
        // Bot của công cụ tìm kiếm và trợ lý AI (tìm nguồn để trả lời, đọc trang khi người dùng hỏi) — nêu tên rõ
        // để chào đón. Theo RFC 9309, bot có nhóm riêng sẽ bỏ qua nhóm "*", nên nhóm này lặp lại đủ quy tắc.
        let bots = ["Googlebot", "Bingbot", "OAI-SearchBot", "ChatGPT-User", "Claude-SearchBot", "Claude-User", "PerplexityBot", "Perplexity-User"];
        let named: String = bots.iter().map(|b| format!("User-agent: {b}\n")).collect();
        let note = if self.base.is_empty() {
            String::new()
        } else {
            "# Lưu ý: bot chỉ đọc robots.txt ở gốc tên miền. Web đang nằm trong thư mục con nên file này chỉ để tham khảo —\n\
             # hãy gửi sitemap qua Google Search Console và Bing Webmaster Tools.\n"
                .to_string()
        };
        format!(
            "# {} — chào đón công cụ tìm kiếm và trợ lý AI đọc, trích dẫn tri thức đã duyệt.\n{note}\n{named}{rules}\nUser-agent: *\n{rules}\nSitemap: {}\n# Chỉ mục cho AI agent: {}\n# Tin mới (Atom): {}\n",
            self.data.config.name,
            self.abs("sitemap.xml"),
            self.abs("llms.txt"),
            self.abs("feed.xml")
        )
    }

    /// Tin mới dạng Atom: ghi chú đã duyệt + bài viết đã đăng, mới nhất trước (tối đa 50).
    fn feed(&self) -> String {
        let kb = self.kb();
        let mut items: Vec<(String, String, String, String)> = kb
            .notes()
            .iter()
            .map(|n| (n.updated.clone(), n.title.clone(), self.abs(&format!("notes/{}/", n.slug)), n.summary.clone()))
            .chain(self.data.articles.published().iter().map(|a| (a.updated.clone(), a.title.clone(), self.abs(&format!("bai-viet/{}/", a.slug)), a.description.clone())))
            .collect();
        items.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| vi_cmp(&a.1, &b.1)));
        items.truncate(50);
        let cfg = &self.data.config;
        let updated = items.first().map(|i| iso_time(&i.0)).unwrap_or_else(|| iso_time(&self.today));
        let author = if cfg.author.is_empty() { &cfg.name } else { &cfg.author };
        let entries: Vec<String> = items
            .iter()
            .map(|(d, title, url, summary)| {
                format!(
                    "  <entry>\n    <title>{}</title>\n    <link href=\"{url}\"/>\n    <id>{url}</id>\n    <updated>{}</updated>\n    <summary>{}</summary>\n  </entry>",
                    xml_escape(title),
                    iso_time(d),
                    xml_escape(summary)
                )
            })
            .collect();
        format!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<feed xmlns=\"http://www.w3.org/2005/Atom\" xml:lang=\"{}\">\n  <title>{}</title>\n  <subtitle>{}</subtitle>\n  <link href=\"{}\" rel=\"self\"/>\n  <link href=\"{}\"/>\n  <id>{}</id>\n  <updated>{updated}</updated>\n  <author><name>{}</name></author>\n{}\n</feed>\n",
            cfg.language,
            xml_escape(&cfg.name),
            xml_escape(&cfg.description),
            self.abs("feed.xml"),
            self.abs(""),
            self.abs(""),
            xml_escape(author),
            entries.join("\n")
        )
    }
}

/// Mã hoá phần truy vấn URL (giữ chữ, số và - _ . ~).
fn url_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(b as char),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// Như `url_encode` nhưng giữ "/" (cho tên nhánh trong đường dẫn).
fn url_encode_path(s: &str) -> String {
    s.split('/').map(url_encode).collect::<Vec<_>>().join("/")
}

/// Thu gọn HTML cho nhẹ: bỏ thụt đầu dòng và dòng trống do template sinh ra.
/// Giữ nguyên từng ký tự bên trong `<pre>` và `<textarea>` (khối code, ô soạn thảo).
fn compact_html(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut keep = 0usize; // số thẻ <pre>/<textarea> đang mở
    for line in html.lines() {
        if keep > 0 {
            out.push_str(line);
            out.push('\n');
        } else {
            let t = line.trim_start();
            if !t.trim_end().is_empty() {
                out.push_str(t);
                out.push('\n');
            }
        }
        let lower = line.to_ascii_lowercase();
        let opens = lower.matches("<pre").count() + lower.matches("<textarea").count();
        let closes = lower.matches("</pre>").count() + lower.matches("</textarea>").count();
        keep = (keep + opens).saturating_sub(closes);
    }
    out
}

/// Trang tạo issue GitHub với tiêu đề và nội dung điền sẵn.
fn issue_url(repo: &str, title: &str, body: &str) -> String {
    format!("https://github.com/{repo}/issues/new?title={}&body={}", url_encode(title), url_encode(body))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::articles::Articles;
    use crate::config::Config;
    use crate::kb::FileEntry;

    fn load(include_drafts: bool) -> Loaded {
        let files = vec![
            FileEntry { path: "knowledge/topics/chu-de.md".into(), raw: "---\ntitle: Chủ đề\ntype: topic\nsummary: Chủ đề mẫu.\nstatus: verified\ncreated: 2026-01-01\n---\nXem [[nhap]].\n".into() },
            FileEntry { path: "knowledge/_inbox/nhap.md".into(), raw: "---\ntitle: Bản nháp\ntype: concept\nsummary: Bản nháp mẫu.\ntags: [chi-nhap]\nstatus: draft\ncreated: 2026-01-01\n---\nBản nháp là gì. Xem [[chu-de]].\n".into() },
        ];
        let kb = Kb::build(&files, include_drafts);
        let articles = Articles::build(&[], &kb, include_drafts);
        Loaded { config: Config::default(), config_error: None, kb, articles, figures: Default::default() }
    }

    fn review_site() -> Site {
        Site::new(load(true), "/Repo", None, false).into_review(Review { repo: Some("chu/kho".into()), branch: Some("main".into()) })
    }

    #[test]
    fn web_cong_khai_khong_co_ban_nhap() {
        let site = Site::new(load(false), "/Repo", None, false);
        assert!(site.routes().iter().all(|r| !r.contains("nhap") && !r.starts_with("duyet")));
        let robots = site.render("robots.txt").unwrap();
        assert!(String::from_utf8(robots.body).unwrap().contains("Disallow: /Repo/duyet/"));
        let llms = String::from_utf8(site.render("llms-full.txt").unwrap().body).unwrap();
        assert!(!llms.contains("Bản nháp mẫu"));
    }

    #[test]
    fn khu_duyet_bai_dan_toi_ban_nhap() {
        let site = review_site();
        assert_eq!(site.review_routes(), vec!["duyet/".to_string(), "duyet/notes/nhap/".to_string()]);
        assert_eq!(site.note_url("nhap"), "/Repo/duyet/notes/nhap/");
        assert_eq!(site.note_url("chu-de"), "/Repo/notes/chu-de/");
        // Tag chỉ có ở bản nháp chưa có trang công khai.
        assert_eq!(site.tag_url("chi-nhap"), "/Repo/duyet/");
        let page = String::from_utf8(site.render("duyet/notes/nhap/").unwrap().body).unwrap();
        assert!(page.contains("noindex"));
        assert!(page.contains("https://github.com/chu/kho/issues/new?title=Duy%E1%BB%87t%3A%20knowledge%2F_inbox%2Fnhap.md"));
        assert!(page.contains("https://github.com/chu/kho/edit/main/knowledge/_inbox/nhap.md"));
        let index = String::from_utf8(site.render("duyet/").unwrap().body).unwrap();
        assert!(index.contains("/Repo/duyet/notes/nhap/"));
        // Ghi chú đã đăng không có trang trong Khu duyệt bài.
        assert!(site.render("duyet/notes/chu-de/").is_none());
    }

    #[test]
    fn ban_nhap_xep_theo_chu_de_va_thu_tu_doc() {
        let note = |path: &str, title: &str, kind: &str, body: &str| FileEntry {
            path: path.into(),
            raw: format!("---\ntitle: {title}\ntype: {kind}\nsummary: Mẫu.\nstatus: draft\ncreated: 2026-01-01\n---\n{body}\n"),
        };
        let files = vec![
            note("knowledge/_inbox/pa.md", "PA", "topic", "Theo sách [[sach]], đọc [[b]] trước, rồi [[a]]."),
            note("knowledge/_inbox/sach.md", "Sách", "source", "Sách gốc. Xem [[pa]]."),
            note("knowledge/_inbox/a.md", "A", "concept", "A là gì. Xem [[pa]]."),
            note("knowledge/_inbox/b.md", "B", "framework", "B là gì. Xem [[pa]]."),
            note("knowledge/_inbox/le.md", "Lẻ", "concept", "Lẻ là gì."),
        ];
        let kb = Kb::build(&files, true);
        let groups = kb.inbox_by_topic();
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].topic.map(|t| t.slug.as_str()), Some("pa"));
        // Bản đồ tài liệu gốc xếp cuối dù chủ đề nhắc tới nó trước.
        assert_eq!(groups[0].notes.iter().map(|n| n.slug.as_str()).collect::<Vec<_>>(), ["b", "a", "sach"]);
        assert!(groups[1].topic.is_none());
        assert_eq!(groups[1].notes[0].slug, "le");
        let (p, i, total) = kb.parent_topic(kb.get("a").unwrap(), true).unwrap();
        assert_eq!((p.slug.as_str(), i, total), ("pa", 2, 3));
        assert_eq!(kb.role_label(kb.get("sach").unwrap()), "Tài liệu gốc · 3/3");
        assert!(kb.parent_topic(kb.get("a").unwrap(), false).is_none());

        let articles = Articles::build(&[], &kb, true);
        let site = Site::new(Loaded { config: Config::default(), config_error: None, kb, articles, figures: Default::default() }, "/Repo", None, false)
            .into_review(Review { repo: Some("chu/kho".into()), branch: Some("main".into()) });
        let index = String::from_utf8(site.render("duyet/").unwrap().body).unwrap();
        let (main, b, a) = (index.find("Chủ đề chính").unwrap(), index.find("Bài con 1/3").unwrap(), index.find("Bài con 2/3").unwrap());
        assert!(main < b && b < a);
        assert!(index.contains("Chưa thuộc chủ đề"));
        let page = String::from_utf8(site.render("duyet/notes/a/").unwrap().body).unwrap();
        assert!(page.contains("Bài con <strong>2/3</strong>"));
    }

    #[test]
    fn lo_trinh_hoc_sau_cap() {
        let note = |path: &str, title: &str, kind: &str, body: &str| FileEntry {
            path: path.into(),
            raw: format!("---\ntitle: {title}\ntype: {kind}\nsummary: Mẫu.\nstatus: draft\ncreated: 2026-01-01\n---\n{body}\n"),
        };
        let topic = "Giới thiệu, nhắc [[c]] trước.\n\n## Lộ trình học\n\n### 1. Biết — là gì\n\nPhần trên.\n\n\
            ### 2. Hiểu\n\n- [[b]]\n- [[a]]\n\n### 3. Vận dụng: làm thế nào\n\n- [[c]]\n- [[chua-co]]\n\n\
            ### Đọc thêm\n\n- [[a]]\n\n### 6. Sáng tạo\n\n```\n[[trong-code]]\n```\n\n## Tài liệu gốc\n\n- [[sach]]\n";
        let files = vec![
            note("knowledge/_inbox/pa.md", "PA", "topic", topic),
            note("knowledge/_inbox/sach.md", "Sách", "source", "Sách gốc."),
            note("knowledge/_inbox/a.md", "A", "concept", "A."),
            note("knowledge/_inbox/b.md", "B", "concept", "B."),
            note("knowledge/_inbox/c.md", "C", "framework", "C."),
        ];
        let kb = Kb::build(&files, true);
        let pa = kb.get("pa").unwrap();
        let levels: Vec<(usize, Vec<&str>, Vec<&str>)> = pa
            .learning_path
            .iter()
            .map(|l| (l.level, l.notes.iter().map(String::as_str).collect(), l.gaps.iter().map(String::as_str).collect()))
            .collect();
        assert_eq!(levels, [(1, vec![], vec![]), (2, vec!["b", "a"], vec![]), (3, vec!["c"], vec!["chua-co"]), (6, vec![], vec![])]);
        assert_eq!(pa.learning_path[2].id, "3-van-dung-lam-the-nao");
        // Thứ tự đọc theo lộ trình (không theo lần nhắc đầu), tài liệu gốc ngoài lộ trình xếp cuối.
        assert_eq!(kb.reading_order(pa, true).iter().map(|n| n.slug.as_str()).collect::<Vec<_>>(), ["b", "a", "c", "sach"]);
        assert_eq!(kb.role_label(kb.get("a").unwrap()), "Hiểu · Bài 2/4");
        assert_eq!(kb.role_label(kb.get("sach").unwrap()), "Tài liệu gốc · 4/4");
        let p = kb.place(kb.get("c").unwrap(), true).unwrap();
        assert_eq!((p.level, p.prev.map(|n| n.slug.as_str()), p.next.map(|n| n.slug.as_str())), (Some(3), Some("a"), Some("sach")));
        assert!(kb.issues.iter().any(|i| i.message.contains("Đọc thêm") && i.level == Level::Warning));

        let articles = Articles::build(&[], &kb, true);
        let site = Site::new(Loaded { config: Config::default(), config_error: None, kb, articles, figures: Default::default() }, "/Repo", None, false)
            .into_review(Review { repo: Some("chu/kho".into()), branch: Some("main".into()) });
        let topic_page = String::from_utf8(site.render("duyet/notes/pa/").unwrap().body).unwrap();
        assert!(topic_page.contains("Lộ trình học · 6 cấp"));
        assert!(topic_page.contains("href=\"#2-hieu\""));
        assert!(topic_page.contains("1 bài còn thiếu"));
        let b = String::from_utf8(site.render("duyet/notes/b/").unwrap().body).unwrap();
        assert!(b.contains("Cấp 2 · <strong>Hiểu</strong>"));
        assert!(b.contains("← Bài trước · Biết") && b.contains("Bài tiếp · Hiểu →"));
        let sach = String::from_utf8(site.render("duyet/notes/sach/").unwrap().body).unwrap();
        assert!(sach.contains("Hết lộ trình"));
    }

    #[test]
    fn dinh_nghia_phai_dan_tai_lieu_goc() {
        let srcs = "sources:\n  - title: Tóm tắt\n    credibility: 3\n  - title: Sách gốc\n    credibility: 4\n    primary: true\n";
        let note = |path: &str, body: &str| FileEntry {
            path: path.into(),
            raw: format!("---\ntitle: T\ntype: concept\nsummary: Mẫu.\nstatus: draft\n{srcs}created: 2026-01-01\n---\n{body}\n"),
        };
        let warn = |kb: &Kb, path: &str| kb.issues.iter().filter(|i| i.path == path).any(|i| i.message.contains("tài liệu gốc"));
        let files = vec![
            note("knowledge/_inbox/a.md", "A là gì (Nguồn [1]).\n\n## Giải thích\n\nTheo sách (Nguồn [2])."),
            note("knowledge/_inbox/b.md", "B là gì (Nguồn [1][2])."),
            note("knowledge/_inbox/c.md", "C là gì.\n\n## Định nghĩa\n\nC là … (Nguồn [2]).\n\n```\n[1]\n```"),
            note("knowledge/_inbox/d.md", "D là gì, chưa dẫn nguồn."),
            note("knowledge/concepts/e.md", "E đã đăng (Nguồn [1])."),
            // Chỉ câu định nghĩa (đoạn đầu của mục) mới tính: nhắc sách gốc ở đoạn sau không đủ.
            note("knowledge/_inbox/f.md", "## Mục đích\n\nF là … (Nguồn [1]).\n\nTheo sách gốc (Nguồn [2])."),
            note("knowledge/_inbox/g.md", "> Lưu ý chung (Nguồn [1]).\n\nG là … (Nguồn [2]).\n\n## Mục đích\n\nDùng để …"),
        ];
        let kb = Kb::build(&files, true);
        assert!(warn(&kb, "knowledge/_inbox/f.md"));
        assert!(!warn(&kb, "knowledge/_inbox/g.md"), "bỏ qua đoạn trích ở đầu bài");
        assert!(warn(&kb, "knowledge/_inbox/a.md"), "định nghĩa chỉ dẫn nguồn thứ cấp");
        assert!(!warn(&kb, "knowledge/_inbox/b.md"));
        assert!(!warn(&kb, "knowledge/_inbox/c.md"), "mục Định nghĩa dẫn tài liệu gốc");
        assert!(warn(&kb, "knowledge/_inbox/d.md"));
        assert!(!warn(&kb, "knowledge/concepts/e.md"), "chỉ kiểm bản nháp");
        assert!(kb.get("b").unwrap().sources[1].primary && !kb.get("b").unwrap().sources[0].primary);
    }

    #[test]
    fn thu_gon_html_giu_khoi_code_va_o_soan() {
        let html = "<main>\n    <p>Chào</p>\n\n    <pre><code>fn a() {\n    b();\n}</code></pre>\n  <textarea>dòng 1\n\n    dòng 3</textarea>\n</main>\n";
        assert_eq!(
            compact_html(html),
            "<main>\n<p>Chào</p>\n<pre><code>fn a() {\n    b();\n}</code></pre>\n<textarea>dòng 1\n\n    dòng 3</textarea>\n</main>\n"
        );
    }

    #[test]
    fn du_lieu_cho_ai_va_tim_kiem_gon_nhe() {
        let site = Site::new(load(false), "/Repo", None, false);
        let kj = String::from_utf8(site.render("knowledge.json").unwrap().body).unwrap();
        let data: Json = serde_json::from_str(&kj).unwrap();
        let note = &data["notes"][0];
        assert!(note.get("content").is_none(), "nội dung đã có ở bản .md, không chép vào knowledge.json");
        assert!(note["markdown_url"].as_str().unwrap().ends_with("notes/chu-de.md"));
        assert!(!kj.contains("\n  "), "knowledge.json ghi gọn, không thụt lề");
        let si: Json = serde_json::from_str(&String::from_utf8(site.render("search-index.json").unwrap().body).unwrap()).unwrap();
        assert!(si.as_array().unwrap().iter().all(|x| x["text"].as_str().unwrap().chars().count() <= SEARCH_TEXT_CHARS));
    }

    #[test]
    fn seo_ky_thuat_tren_web_cong_khai() {
        let site = Site::new(load(false), "/Repo", Some("https://vd.github.io".into()), false);
        let page = |r: &str| String::from_utf8(site.render(r).unwrap().body).unwrap();
        // Địa chỉ chính thức, ảnh chia sẻ, thẻ X/Twitter trên trang ghi chú; thời điểm đăng/sửa.
        let note = page("notes/chu-de/");
        assert!(note.contains(r#"<link rel="canonical" href="https://vd.github.io/Repo/notes/chu-de/" />"#));
        assert!(note.contains(r#"<meta property="og:url" content="https://vd.github.io/Repo/notes/chu-de/" />"#));
        assert!(note.contains(r#"<meta property="og:image" content="https://vd.github.io/Repo/assets/og-image.png" />"#));
        assert!(note.contains(r#"content="summary_large_image""#));
        assert!(note.contains(r#"<meta property="article:modified_time" content="2026-01-01T00:00:00Z" />"#));
        // Tác giả để trống -> đứng tên thương hiệu, có đơn vị xuất bản.
        assert!(note.contains(r#""author":{"@type":"Organization","name":"Second Brain""#));
        assert!(note.contains(r#""publisher":{"@id":"https://vd.github.io/Repo/#org""#) || note.contains(r#""publisher":{"@type":"Organization""#));
        // Trang chủ: WebSite + Organization.
        let home = page("");
        assert!(home.contains(r#""@type":"WebSite""#) && home.contains(r#""@type":"Organization""#));
        assert!(home.contains(r#"<link rel="canonical" href="https://vd.github.io/Repo/" />"#));
        // 404 và trang noindex không có canonical.
        assert!(!page("404.html").contains("rel=\"canonical\""));
        assert!(!page("gaps/").contains("rel=\"canonical\""));
        // Tin mới Atom chỉ có bài đã duyệt.
        let feed = page("feed.xml");
        assert!(feed.contains("<id>https://vd.github.io/Repo/notes/chu-de/</id>") && !feed.contains("nhap"));
        // robots.txt: nhóm bot AI lặp đủ quy tắc chặn Khu duyệt bài.
        let robots = page("robots.txt");
        assert!(robots.contains("User-agent: Claude-SearchBot") && robots.contains("User-agent: OAI-SearchBot"));
        assert_eq!(robots.matches("Disallow: /Repo/duyet/").count(), 2);
        assert!(robots.contains("chỉ đọc robots.txt ở gốc tên miền"));
        // Ảnh chia sẻ và logo được xuất kèm.
        assert_eq!(site.render("assets/og-image.png").unwrap().content_type, "image/png");
    }

    #[test]
    fn ban_dung_tren_may_va_khu_duyet_khong_co_canonical() {
        let dev = Site::new(load(true), "", None, true);
        assert!(!String::from_utf8(dev.render("notes/chu-de/").unwrap().body).unwrap().contains("canonical"));
        let review = Site::new(load(true), "/Repo", Some("https://vd.github.io".into()), false)
            .into_review(Review { repo: None, branch: None });
        let page = String::from_utf8(review.render("duyet/notes/nhap/").unwrap().body).unwrap();
        assert!(!page.contains("canonical") && page.contains("noindex"));
    }

    #[test]
    fn ma_xac_minh_chu_web() {
        let mut data = load(false);
        data.config.google_site_verification = "abcXYZ_12-3".into();
        let site = Site::new(data, "", Some("https://vd.github.io".into()), false);
        let home = String::from_utf8(site.render("").unwrap().body).unwrap();
        assert!(home.contains(r#"<meta name="google-site-verification" content="abcXYZ_12-3" />"#));
        assert!(!home.contains("msvalidate.01"));
        assert_eq!(crate::config::verification_code(r#"<meta name="msvalidate.01" content="ABC123DEF" />"#), "ABC123DEF");
        assert_eq!(crate::config::verification_code(" abc-123 "), "abc-123");
    }

    #[test]
    fn khoa_indexnow() {
        let mut data = load(false);
        data.config.indexnow_key = "abcd1234ef".into();
        let site = Site::new(data, "/Repo", Some("https://vd.github.io".into()), false);
        assert!(site.routes().contains(&"abcd1234ef.txt".to_string()));
        assert_eq!(String::from_utf8(site.render("abcd1234ef.txt").unwrap().body).unwrap(), "abcd1234ef");
        // Không có khoá thì không có file.
        let none = Site::new(load(false), "/Repo", None, false);
        assert!(none.routes().iter().all(|r| !r.ends_with(".txt") || r == "llms.txt" || r == "llms-full.txt" || r == "robots.txt"));
    }

    #[test]
    fn ma_hoa_url() {
        assert_eq!(url_encode("Bỏ: a/b.md"), "B%E1%BB%8F%3A%20a%2Fb.md");
        assert_eq!(url_encode_path("claude/nhanh-1"), "claude/nhanh-1");
    }
}
