//! Dựng mọi trang của website (HTML + đầu ra cho AI) từ kho tri thức.
//! Dùng chung cho `secondbrain build` (ghi ra dist/) và Bàn làm việc (phục vụ trực tiếp, có bản nháp).

use crate::articles::{intent_label, Article, INTENTS};
use crate::kb::{Kb, Level, Note};
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
        Page { status: 200, content_type: "text/html; charset=utf-8", body: body.into_bytes() }
    }
    fn text(content_type: &'static str, body: String) -> Page {
        Page { status: 200, content_type, body: body.into_bytes() }
    }
}

/// File tĩnh nhúng sẵn trong chương trình (không cần thư mục ngoài).
pub const ASSETS: &[(&str, &str, &[u8])] = &[
    ("assets/style.css", "text/css; charset=utf-8", include_bytes!("../assets/style.css")),
    ("assets/app.js", "text/javascript; charset=utf-8", include_bytes!("../assets/app.js")),
    ("assets/search.js", "text/javascript; charset=utf-8", include_bytes!("../assets/search.js")),
    ("assets/graph.js", "text/javascript; charset=utf-8", include_bytes!("../assets/graph.js")),
    ("assets/desk.js", "text/javascript; charset=utf-8", include_bytes!("../assets/desk.js")),
    ("assets/favicon.svg", "image/svg+xml", include_bytes!("../assets/favicon.svg")),
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
];

pub struct Site {
    pub data: Loaded,
    /// Tiền tố đường dẫn khi deploy dạng /ten-repo/ ("" nếu ở gốc tên miền).
    pub base: String,
    /// Gốc URL tuyệt đối, vd. https://ten.github.io (để llms.txt, sitemap có link đầy đủ).
    pub site_url: Option<String>,
    /// Chạy trên máy (Bàn làm việc): hiện bản nháp, gợi ý SEO, tự tải lại.
    pub dev: bool,
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
        Site { data, base, site_url: site_url.map(|s| s.trim_end_matches('/').to_string()), dev, env, today: today() }
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
        self.url(&format!("notes/{slug}/"))
    }
    fn article_url(&self, slug: &str) -> String {
        self.url(&format!("bai-viet/{slug}/"))
    }
    fn tag_url(&self, tag: &str) -> String {
        let s = slugify(tag);
        self.url(&format!("tags/{}/", if s.is_empty() { "tag".into() } else { s }))
    }
    fn gap_href(&self, target: &str) -> String {
        self.url(&format!("gaps/#{}", slugify(target)))
    }

    pub fn render_template(&self, name: &str, ctx: Json) -> String {
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
        let notes: Vec<&Note> = if self.dev { self.kb().all.iter().collect() } else { self.kb().notes() };
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
            "llms-full.txt", "knowledge.json", "search-index.json", "sitemap.xml", "robots.txt", "404.html",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
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

    pub fn asset(&self, route: &str) -> Option<Page> {
        ASSETS.iter().find(|(p, _, _)| *p == route).map(|(_, ct, body)| Page { status: 200, content_type: ct, body: body.to_vec() })
    }

    /// Dựng một trang theo đường dẫn (không có base, không có "/" đầu).
    pub fn render(&self, route: &str) -> Option<Page> {
        if let Some(p) = self.asset(route) {
            return Some(p);
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
            "404.html" => Page::html(self.render_template("404.html", json!({}))),
            "llms.txt" => Page::text("text/plain; charset=utf-8", self.llms_txt()),
            "llms-full.txt" => Page::text("text/plain; charset=utf-8", self.llms_full()),
            "knowledge.json" => Page::text("application/json; charset=utf-8", self.knowledge_json()),
            "search-index.json" => Page::text("application/json; charset=utf-8", self.search_index()),
            "sitemap.xml" => Page::text("application/xml; charset=utf-8", self.sitemap()),
            "robots.txt" => Page::text("text/plain; charset=utf-8", self.robots()),
            r => {
                let parts: Vec<&str> = r.trim_end_matches('/').split('/').collect();
                match (parts.as_slice(), r.ends_with('/')) {
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
        self.render_template("home.html", json!({
            "section": "home",
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
        let parent = backlinks.iter().find(|n| n.kind == "topic" && !n.inbox).copied();
        let crumbs = vec![
            json!({"name": "Trang chủ", "href": self.url("")}),
            match parent {
                Some(p) => json!({"name": p.title, "href": self.note_url(&p.slug)}),
                None => json!({"name": type_label(&note.kind), "href": self.url(&format!("types/{}/", note.kind))}),
            },
            json!({"name": note.title, "href": self.note_url(&note.slug)}),
        ];
        let next_reads: Vec<Json> = parent
            .map(|p| {
                p.links.iter().filter_map(|s| kb.get(s)).filter(|n| n.slug != note.slug && !note.links.contains(&n.slug)).take(4).map(|n| self.note_card(n)).collect()
            })
            .unwrap_or_default();
        let sources: Vec<Json> = note
            .sources
            .iter()
            .map(|s| {
                let meta: Vec<String> = [s.kind.as_deref().and_then(source_kind_label).map(String::from), s.author.clone(), s.publisher.clone(), s.date.clone()]
                    .into_iter()
                    .flatten()
                    .collect();
                json!({"title": s.title, "url": s.url, "meta": meta.join(" · "), "accessed": s.accessed.as_deref().map(format_date),
                    "cred": cred_view(s.credibility.map(|c| c as f64), true, None), "credibility_note": s.credibility_note, "note": s.note})
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
                "author": if self.data.config.author.is_empty() { Json::Null } else { json!({"@type": "Person", "name": self.data.config.author}) },
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
            "crumbs": crumbs, "html": self.render_body(&note.body, false),
            "toc": if headings.len() >= 3 { self.toc(&note.body) } else { vec![] },
            "sources": sources, "parent": parent.map(|p| self.link(p)), "next_reads": next_reads,
            "backlinks": backlinks.iter().map(|n| self.link(n)).collect::<Vec<_>>(),
            "outgoing": outgoing.iter().map(|n| self.link(n)).collect::<Vec<_>>(),
            "broken": note.broken_links.iter().map(|b| json!({"name": b, "href": self.gap_href(b)})).collect::<Vec<_>>(),
            "related": related.iter().take(5).map(|(_, n)| self.link(n)).collect::<Vec<_>>(),
            "articles": articles,
            "graph": if note.links.is_empty() && note.backlinks.is_empty() { Json::Null } else { json!(self.graph_data(Some(&note.slug))) },
            "hints": if self.dev { json!(seo::note_hints(note, &self.today)) } else { Json::Null },
            "edit_url": if self.dev { json!(self.url(&format!("ban-lam-viec/sua/?f={}", note.path))) } else { Json::Null },
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
                "author": if self.data.config.author.is_empty() { Json::Null } else { json!({"@type": "Person", "name": self.data.config.author}) },
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
            "json_ld": if draft { Json::Null } else { json!(json_ld(&json!(lds))) }, "noindex": draft,
            "a": self.article_card(a), "path": a.path, "draft": draft, "crumbs": crumbs,
            "read_minutes": (a.word_count as f64 / 220.0).round().max(1.0) as usize, "word_count": a.word_count,
            "published": format_date(&published),
            "toc": if headings.len() >= 3 { self.toc(&a.body) } else { vec![] },
            "html": self.render_body(&a.body, true),
            "basis": basis.iter().map(|n| { let mut c = self.note_card(n); c["sources_count"] = json!(n.sources.len()); c }).collect::<Vec<_>>(),
            "mentioned": mentioned.iter().map(|n| self.link(n)).collect::<Vec<_>>(),
            "related": self.data.articles.related(a, 4).iter().map(|x| self.article_card(x)).collect::<Vec<_>>(),
            "sources": basis.iter().map(|n| n.sources.len()).sum::<usize>(), "avg_cred": avg.map(|s| cred_view(Some(s), false, None)),
            "hints": if self.dev { json!(seo::article_hints(a, kb)) } else { Json::Null }, "hints_extra": extra,
            "edit_url": if self.dev { json!(self.url(&format!("ban-lam-viec/sua/?f={}", a.path))) } else { Json::Null },
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
            ("knowledge.json", "knowledge.json — dữ liệu có cấu trúc", "Metadata, liên kết, backlinks và nội dung từng ghi chú — dùng cho tool/script tự lọc theo type, status, tags."),
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
        m
    }

    fn note_markdown(&self, n: &Note) -> String {
        let mut fm = self.metadata(n);
        if let Some(o) = fm.as_object_mut() {
            o.remove("links");
            o.remove("backlinks");
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
            format!("- [Chỉ mục JSON]({}): dữ liệu có cấu trúc gồm metadata, liên kết và nội dung từng ghi chú", self.abs("knowledge.json")),
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
            "notes": kb.notes().iter().map(|n| { let mut m = self.metadata(n); m["content"] = json!(self.portable(n)); m }).collect::<Vec<_>>(),
            "articles": self.data.articles.published().iter().map(|a| json!({
                "slug": a.slug, "title": a.title, "description": a.description, "keyword": a.keyword, "intent": a.intent,
                "based_on": a.notes, "created": a.published_on.clone().unwrap_or_else(|| a.created.clone()), "updated": a.updated,
                "url": self.abs(&format!("bai-viet/{}/", a.slug)),
            })).collect::<Vec<_>>(),
        });
        serde_json::to_string_pretty(&data).unwrap_or_default()
    }

    fn search_index(&self) -> String {
        let clip = |s: String| s.chars().take(6000).collect::<String>();
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
        format!(
            "User-agent: *\nAllow: /\nDisallow: {}\n\nSitemap: {}\n# Chỉ mục cho AI agent: {}\n",
            self.url("gaps/"),
            self.abs("sitemap.xml"),
            self.abs("llms.txt")
        )
    }
}
