# Second Brain

Kho tri thức chuyên môn **đã được đọc, hiểu và duyệt** — vừa là website để tra cứu, vừa là nguồn dữ liệu chuẩn hoá để AI agent (Claude, …) viết content chính xác, có nguồn.

## Quy trình

```
 Từ khoá (vd. "Price action")
        │  /research
        ▼
 AI agent tìm tài liệu ─► chấm điểm uy tín từng nguồn (1–5) ─► viết bài tổng hợp có [[liên kết]]
        │
        ▼
 knowledge/_inbox/price-action.md      ← bản nháp, CHƯA lên web
        │  bạn đọc, sửa trực tiếp file (Obsidian / editor / GitHub), xem trước bằng npm run dev
        ▼
 /publish  →  knowledge/topics/price-action.md   (status: verified)
        │  agent nối liên kết hai chiều với ghi chú liên quan + chủ đề trụ cột
        ▼
 Website (Trung tâm kiến thức · Từ điển A–Z · Đồ thị · Tìm kiếm)
 + llms.txt / knowledge.json  ─►  AI agent viết content (/write-content)
        │  /seo-article "cách vẽ hỗ trợ kháng cự"
        ▼
 articles/*.md — bài viết SEO/GEO kéo traffic, chỉ dùng dữ kiện từ tri thức,
                 link về ghi chú gốc → /bai-viet/…
```

**Tri thức là chính, bài viết là bài con.** Ghi chú tri thức là nguồn sự thật (có nguồn, điểm uy tín, viết để chính xác). Bài viết trình bày lại tri thức theo câu hỏi người đọc hay tìm trên Google / AI search, dẫn liên kết về ghi chú gốc — không thêm dữ kiện mới, không sửa tri thức để hợp từ khoá. Chi tiết: mục "Bài viết SEO/GEO" trong [`AGENTS.md`](AGENTS.md).

## Bắt đầu

Cần Node.js ≥ 22.23.

```bash
npm install
```

```bash
npm run dev
```

Mở http://localhost:4321. Sửa hoặc thêm file trong `knowledge/` là trang tự cập nhật.

## Dùng với Claude Code

Mở repo này trong Claude Code, các lệnh sau có sẵn (định nghĩa trong `.claude/skills/`):

| Lệnh | Việc |
| --- | --- |
| `/research Price action` | Nghiên cứu từ khoá, chấm điểm uy tín nguồn, viết bản nháp vào `knowledge/_inbox/` |
| `/publish price-action.md` | Đăng bản nháp bạn đã duyệt và kết nối nó với tri thức sẵn có |
| `/connect` | Rà soát toàn kho: thêm liên kết còn thiếu, nối ghi chú cô lập, liệt kê tri thức còn thiếu |
| `/seo-article cách vẽ hỗ trợ kháng cự` | Viết bài SEO/GEO vào `articles/` từ tri thức đã có, kiểm tra trùng từ khoá |
| `/write-content Bài blog 1.000 từ về price action cho người mới` | Viết content từ kho, có trích nguồn |

Agent khác (Cursor, Codex, …) đọc hướng dẫn trong [`AGENTS.md`](AGENTS.md).

## Tự viết ghi chú

```bash
npm run new -- concept "Vùng hỗ trợ và kháng cự" --tags trading
```

Các loại: `topic` (chủ đề trụ cột), `concept`, `fact`, `framework`, `case-study`, `source`. Thêm `--inbox` để tạo bản nháp chưa đăng.

Có thể mở thư mục `knowledge/` như một vault **Obsidian**: liên kết `[[...]]` tương thích, template nằm trong `knowledge/_templates/` (bật core plugin *Templates*).

Cấu trúc frontmatter, thang điểm uy tín nguồn và quy tắc liên kết: xem [`AGENTS.md`](AGENTS.md).

## Trang web có gì

- **Trung tâm kiến thức** (`/kien-thuc/`): mỗi ghi chú chủ đề là một trụ cột, kèm cụm ghi chú con theo loại.
- **Từ điển A–Z** (`/tu-dien/`): mỗi thuật ngữ một dòng định nghĩa.
- **Đồ thị tri thức** (`/graph/`) và bản đồ liên kết trên từng ghi chú.
- **Trang ghi chú**: mục lục, breadcrumb, nguồn kèm điểm uy tín, backlinks, "Đọc tiếp", câu hỏi thường gặp.
- **Tri thức còn thiếu** (`/gaps/`): các `[[liên kết]]` chưa có ghi chú — hàng đợi nghiên cứu.
- **Tìm kiếm** gõ có dấu hay không dấu đều được.
- **Bài viết** (`/bai-viet/`): bài SEO/GEO theo ý định tìm kiếm, mỗi bài có mục "Tri thức gốc"; trang ghi chú tự hiện "Bài viết về chủ đề này".
- **Giới thiệu & quy trình biên tập** (`/gioi-thieu/`): minh bạch cách nghiên cứu, chấm điểm nguồn, duyệt — tín hiệu tin cậy cho người đọc, Google và AI.
- **SEO / AI search**: dữ liệu có cấu trúc (Article + isBasedOn, FAQPage, BreadcrumbList, DefinedTermSet, AboutPage), `sitemap.xml`, `robots.txt`. Khi chạy `npm run dev`, mỗi trang có hộp gợi ý **SEO / GEO** (không lên web).

## Cho AI agent

| URL | Nội dung |
| --- | --- |
| `/llms.txt` | Chỉ mục toàn kho theo chuẩn llmstxt.org, kèm quy tắc sử dụng |
| `/llms-full.txt` | Toàn bộ nội dung trong một file |
| `/knowledge.json` | Dữ liệu có cấu trúc: metadata, nguồn, điểm uy tín, liên kết, nội dung |
| `/notes/<slug>.md` | Markdown của từng ghi chú |

Trang `/ai/` có prompt mẫu để dán vào bất kỳ AI agent nào. Quy tắc agent phải tuân theo nằm trong `src/config.mjs`.

## Deploy

Workflow `.github/workflows/deploy.yml` build và đăng lên **GitHub Pages** mỗi khi push lên nhánh `main`:

1. Repo → *Settings* → *Pages* → *Source*: chọn **GitHub Actions**.
2. Push lên `main`.

GitHub Pages cho repo private cần gói GitHub trả phí. Cách khác: Vercel / Netlify / Cloudflare Pages với lệnh build `npm run build`, thư mục `dist`, và biến môi trường `SITE_URL` (vd. `https://tenmien.com`) để link trong `llms.txt` là URL tuyệt đối.

Bản nháp trong `knowledge/_inbox/` không bao giờ được build lên web, nhưng vẫn nằm trong repo — nếu repo public thì ai cũng xem được trên GitHub.

## Lệnh

```bash
npm run dev          # chạy site local (hiện cả bản nháp chờ duyệt)
npm run check        # kiểm tra schema, liên kết, chất lượng nguồn, gợi ý kết nối
npm run build        # check + build site tĩnh vào dist/
npm run new -- <type> "Tiêu đề" [--inbox] [--tags a,b]
npm run publish -- <file> [--status draft] [--dry-run]
npm run kb -- search "từ khoá" | show <slug> | list | gaps | orphans | mentions | stats
npm run new -- article "Tiêu đề" --keyword "từ khoá" --based-on slug-a,slug-b
npm run publish -- articles/<file>.md   # đăng bài viết (ghi chú gốc phải verified)
npm run seo [-- <slug> | --keyword "từ khoá"]   # gợi ý SEO/GEO, kiểm tra trùng từ khoá
```

## Ghi chú ví dụ

Các ghi chú có tag `vi-du` (Second Brain, CODE, PARA, …) chỉ để minh hoạ. Xoá chúng khi bắt đầu dùng thật.
