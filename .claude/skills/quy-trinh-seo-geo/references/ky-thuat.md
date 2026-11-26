# Nền kỹ thuật SEO/GEO — những gì phải đúng trên web

Chương trình Second Brain (`src/site.rs`, `templates/base.html`) tự làm hết các mục dưới đây khi dựng web. Nếu dựng trang tri thức bằng công cụ khác (Astro, Hugo, Next.js…), dùng danh sách này làm yêu cầu, rồi kiểm bằng `.github/scripts/kiem-tra-web.mjs`.

## Mỗi trang công khai

| Mục | Vì sao | Second Brain làm ở đâu |
| --- | --- | --- |
| `<title>` ≤ 60 ký tự, `meta description` 70–160 ký tự | Dòng hiện trên Google; AI đọc đầu tiên | `title` + `summary` của ghi chú |
| `<link rel="canonical">` URL tuyệt đối của chính trang | Báo Google đâu là địa chỉ chính, tránh trùng lặp | Tự điền theo đường dẫn đang dựng (`CURRENT_ROUTE` trong `render_template`) |
| `meta robots`: trang công khai `index, follow, max-snippet:-1, max-image-preview:large`; bản nháp, trang lỗi `noindex` | Cho phép trích đoạn dài; bản nháp không bao giờ lên Google | `base.html` |
| Open Graph: `og:title`, `og:description`, `og:url`, `og:image` (1200×630, có width/height/alt), `og:type`, `og:locale`; `twitter:card` = `summary_large_image` | Dán link lên mạng xã hội có ảnh, tiêu đề đẹp | `base.html`, ảnh `assets/og-image.png` |
| `article:published_time`, `article:modified_time` | Ngày đăng / sửa cho trang bài | Lấy từ `created` / `updated` |
| JSON-LD hợp lệ | Máy hiểu trang nói về gì, ai viết, nguồn nào | Xem mục dưới |
| Ngày cập nhật hiện trên trang | Tín hiệu độ mới cho người đọc và AI | "Cập nhật dd/mm/yyyy" |
| `<link rel="alternate">` tới bản Markdown, `llms.txt`, `feed.xml` | AI agent đọc bản gọn | `base.html` |

## Dữ liệu cấu trúc (JSON-LD)

- **Trang chủ:** `WebSite` + `Organization`. Organization có `@id`, tên thương hiệu, `url` và `logo`: PNG vuông, `assets/logo.png` 256×256.
- **Ghi chú và bài viết:** `Article` gồm:
  - `headline`, `description`, `datePublished`, `dateModified`, `inLanguage`, `url`, `mainEntityOfPage`, `image`;
  - `author`: là người nếu có tên tác giả, không thì là Organization mang tên thương hiệu;
  - `publisher`: Organization;
  - `citation` (ghi chú): danh sách nguồn;
  - `isBasedOn` (bài viết): ghi chú gốc.

  Ghi chú loại khái niệm có thêm kiểu `DefinedTerm`.
- `BreadcrumbList` cho trang ghi chú và bài viết.
- `FAQPage` sinh từ mục `## Câu hỏi thường gặp`. Google chỉ còn hiện kết quả FAQ đặc biệt cho một số web nhà nước hoặc y tế, nhưng AI vẫn đọc được phần này.
- `DefinedTermSet` cho trang từ điển; `AboutPage` cho trang giới thiệu.

## File toàn web

| File | Yêu cầu |
| --- | --- |
| `sitemap.xml` | Mọi trang công khai, có `<lastmod>` đúng ngày sửa. Không có bản nháp. |
| `robots.txt` | `Allow: /`, chặn Khu duyệt bài và trang "còn thiếu"; dòng `Sitemap:`; nêu tên bot tìm kiếm và bot AI. Theo RFC 9309, nhóm bot có tên riêng phải **lặp đủ quy tắc**, vì bot có nhóm riêng bỏ qua nhóm `*`. |
| `llms.txt`, `llms-full.txt` | Theo đề xuất llmstxt.org: H1 tên web, câu tóm tắt, các mục H2 chứa danh sách link. |
| `feed.xml` | Atom, chỉ bài đã duyệt, mới nhất trước. |
| `knowledge.json`, `notes/<slug>.md` | Dữ liệu và bản Markdown cho AI agent. |
| `<khoá>.txt` | File khoá IndexNow, nội dung là chính khoá. |

## Bẫy đã gặp

- **Web nằm trong thư mục con** (`ten.github.io/Repo/`): bot chỉ đọc `robots.txt` ở gốc tên miền, nên `robots.txt` của web không có tác dụng. Vẫn an toàn nhờ `noindex` trên từng trang nháp, nhưng phải gửi sitemap qua Search Console. Có tên miền riêng thì hết vấn đề này.
- **Template có chỗ cho canonical nhưng chương trình chưa từng điền**, nên web thật không có canonical. Luôn kiểm trên web thật, đừng chỉ đọc template.
- **Dựng thử trong Git Bash:** `BASE_PATH=/Repo` bị đổi thành đường dẫn Windows. Dùng `MSYS_NO_PATHCONV=1` và đưa `--out` dạng Windows (`cygpath -w`).
- **Script Node trên Windows** gọi `process.exit()` khi `fetch` còn kết nối thì báo lỗi giả. Dùng `process.exitCode`.

## Kiểm tra toàn bộ bản dựng trước khi đẩy lên

Dựng với địa chỉ thật, rồi quét mọi trang: JSON-LD phải parse được; trang công khai nào cũng có canonical và og:image; trang noindex không có canonical. Script quét mẫu ghi trong `NHAT-KY.md` (2026-11-26): 36 trang, 0 lỗi.
