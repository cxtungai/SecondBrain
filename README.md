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
```

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
- **SEO / AI search**: dữ liệu có cấu trúc (Article, FAQPage, BreadcrumbList, DefinedTermSet), `sitemap.xml`, `robots.txt`.

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
```

## Ghi chú ví dụ

Các ghi chú có tag `vi-du` (Second Brain, CODE, PARA, …) chỉ để minh hoạ. Xoá chúng khi bắt đầu dùng thật.
