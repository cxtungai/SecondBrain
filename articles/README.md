# Bài viết (SEO / GEO)

Bài viết kéo traffic cho site, **dẫn xuất từ tri thức** trong `knowledge/`. Mỗi file `.md` là một bài, URL `/bai-viet/<tên-file>/`.

- Tri thức là nguồn sự thật; bài viết chỉ trình bày lại theo câu hỏi người đọc hay tìm.
- `based_on` liệt kê ghi chú gốc; mọi dữ kiện trong bài phải lấy từ đó và có `[[liên kết]]` về ghi chú.
- Mỗi từ khoá chính chỉ một trang. Truy vấn định nghĩa ("X là gì") thuộc về ghi chú; bài viết nhắm truy vấn khác.
- `status: draft` chỉ hiện khi chạy `npm run dev`; đăng bằng `npm run publish -- articles/<file>.md`.

Tạo bài: `/seo-article <từ khoá>` trong Claude Code, hoặc `npm run new -- article "Tiêu đề" --keyword "..." --based-on slug-a,slug-b`.
Xem hướng dẫn đầy đủ trong `AGENTS.md`, mục "Bài viết SEO/GEO".
