# Đổi tên miền / đổi tên web

## Tự cập nhật (không phải sửa)

Khi dựng web, GitHub Actions lấy địa chỉ từ GitHub Pages (`actions/configure-pages`: `origin`, `base_path`, `base_url`). Gắn tên miền mới trong GitHub thì lần dựng sau tự đổi:

- canonical, `og:url`, `og:image`, JSON-LD, sitemap, `robots.txt`, `llms.txt`, `feed.xml`, `knowledge.json`;
- job `sau-khi-dang`: kiểm tra web thật và IndexNow (file khoá nằm ở gốc tên miền mới);
- `robots.txt` có hiệu lực thật, vì web không còn nằm trong thư mục con;
- GitHub tự chuyển hướng địa chỉ `ten.github.io/Repo/…` sang tên miền mới, nên link cũ vẫn chạy và Google hiểu đây là web chuyển nhà.

## Làm tay một lần

1. **Người dùng mua tên miền.** Mua sớm khi web còn ít bài thì đỡ phải chuyển thứ hạng.
2. **DNS** ở nhà bán tên miền: agent đưa đúng các dòng theo tài liệu GitHub Pages "Managing a custom domain".
   - Tên miền gốc: 4 bản ghi A trỏ tới IP của GitHub Pages, kèm AAAA nếu cần.
   - `www` hoặc tên miền con: bản ghi CNAME trỏ tới `ten.github.io`.
   - Agent đọc lại tài liệu GitHub mỗi lần làm, vì IP có thể đổi.
3. **GitHub → Settings → Pages → Custom domain:** điền tên miền, chờ kiểm DNS xong thì bật **Enforce HTTPS**. Web dựng bằng GitHub Actions không cần file `CNAME`.
4. Đẩy một commit bất kỳ (hoặc chạy lại "Build & deploy") để dựng với địa chỉ mới. Chạy `kiem-tra-web.mjs --site https://ten-mien-moi/`.
5. **Google Search Console:**
   - thêm tài sản mới, nên chọn kiểu **Miền** (xác minh bằng bản ghi TXT trong DNS, bền hơn thẻ HTML);
   - gửi `sitemap.xml` mới;
   - giữ tài sản cũ để theo dõi quá trình chuyển;
   - nếu Google cho phép, dùng công cụ **Thay đổi địa chỉ** trong tài sản cũ.
6. **Bing:** thêm web mới (Import lại từ Search Console).
7. **Brave:** gửi trang chủ mới một lần.
8. **IndexNow:** gửi tay toàn bộ web một lần (lệnh ở giai đoạn 3 trong SKILL.md).
9. Sửa địa chỉ cũ trong `README.md`, `CLAUDE.md`, `AGENTS.md`, `references/geo.md` (tìm `github.io`).
10. Ghi vào `NHAT-KY.md` và `trang-thai.md`.

## Đổi tên web (thương hiệu)

- Sửa `name` (và `description` nếu cần) trong `cau-hinh.yml`, hoặc trong Cài đặt của Bàn làm việc. Mọi chữ trên web và dữ liệu cấu trúc tự đổi theo.
- **Vẽ lại ảnh chia sẻ và logo**, vì tên được vẽ sẵn trong ảnh: `node .claude/skills/quy-trinh-seo-geo/scripts/ve-anh-chia-se.mjs --ten "<Tên mới>" …`. Mở ảnh xem tận mắt.
- Đổi tên thương hiệu không cần làm lại Search Console, vì địa chỉ không đổi.
