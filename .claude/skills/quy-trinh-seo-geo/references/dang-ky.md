# Đăng ký với công cụ tìm kiếm (người dùng làm một lần, agent hướng dẫn)

Agent không tạo tài khoản hay bấm cấp quyền thay người dùng. Agent hướng dẫn từng cú bấm, nhận lại mã do người dùng dán vào chat, rồi làm phần còn lại. Mã xác minh không phải mật khẩu: nó vốn hiện công khai trên web.

## 1. Google Search Console (quan trọng nhất)

1. Vào search.google.com/search-console, đăng nhập Google.
2. **Thêm tài sản**, chọn ô bên phải **Tiền tố URL**, dán địa chỉ web có `/` cuối, vd. `https://ten.github.io/Repo/`.
   - Chỉ chọn kiểu **Miền** (Domain, xác minh bằng DNS) khi có tên miền riêng. Kiểu này gom mọi biến thể http/https/www vào một chỗ.
3. Cách xác minh: **Thẻ HTML**, bấm sao chép. Người dùng dán nguyên thẻ `<meta name="google-site-verification" …>` vào chat.
4. Agent dán mã vào `cau-hinh.yml` → `google_site_verification` (dán cả thẻ cũng được), chạy `./sb check`, commit, push, rồi chờ đến khi trang chủ thật có thẻ:

   ```bash
   curl -s <địa chỉ web>/ | grep google-site-verification
   ```

5. Người dùng bấm **Xác minh**. Vào được trang **Tổng quan** là thành công. Dòng "Đang xử lý dữ liệu… khoảng một ngày" là bình thường.
6. Cột trái → **Sơ đồ trang web** → gõ `sitemap.xml` (phần đầu địa chỉ đã điền sẵn) → **Gửi**. Mới gửi mà báo "Không thể tìm nạp" thì đợi vài phút rồi tải lại trang.
7. Thanh **Kiểm tra URL** trên cùng: dán trang chủ, Enter, bấm **YÊU CẦU LẬP CHỈ MỤC**.
   - Kết quả "URL không nằm trên Google" là bình thường với web mới.
   - Chỉ cần làm với trang chủ, vì mỗi ngày Google giới hạn số lần yêu cầu. Các trang khác để sitemap lo.

8. **Cài đặt → Search generative AI:** kiểm tra đang ở **Include** (cho phép web xuất hiện trong AI Overviews, AI Mode, Discover). Đây là mặc định. Tài sản con có thể đang để "Inherit", tức theo tài sản cha. Tắt thì web không được AI của Google trích. Google đã đưa nút điều khiển này tới mọi web từ 2026-08-31.

Không cần gửi lại sitemap sau mỗi bài: Google tự đọc lại sitemap định kỳ.

## 2. Bing Webmaster Tools (sau Google)

1. Vào bing.com/webmasters, đăng nhập. Có thể đăng nhập bằng chính tài khoản Google.
2. Nếu khung **Settings** đang mở thì đóng lại (dấu X).
3. Ở ô **Google Search Console**, bấm **Import**, tích web, bấm **Import**. Không cần mã xác minh. Bỏ qua ô "Add your site manually".
4. Kiểm tra cột trái → **Sitemaps**. Nếu trống thì **Submit sitemap** bằng **địa chỉ đầy đủ** `https://…/sitemap.xml`. Bing không tự thêm phần đầu như Google.
5. Các mục đáng xem về sau:
   - **IndexNow:** các trang GitHub đã tự báo;
   - **AI Performance (BETA):** khi nào web được Copilot và câu trả lời AI của Bing trích dẫn. Đây là thước đo GEO trực tiếp nhất hiện có.

Bing cung cấp kết quả cho Copilot và một phần ChatGPT tìm kiếm. Dù không đăng ký, Bing vẫn được báo trang mới qua IndexNow; đăng ký chỉ để có số liệu.

## 3. Brave Search

1. Vào search.brave.com/submit-url.
2. Dán **trang chủ**, bấm **Submit**. Nếu hiện ô "tôi không phải người máy" thì người dùng tự bấm (agent không bao giờ làm thay).
3. Gửi một lần. Brave không có trang quản lý hay sitemap; theo diễn đàn của Brave, đôi khi vài tuần mới thấy kết quả.

Claude dùng kết quả của Brave khi tìm nguồn (theo báo chí từ 2025, Anthropic chưa công bố chi tiết).

## Ghi lại

Xong bước nào thì ghi vào `trang-thai.md`: ngày, kiểu tài sản Google (Tiền tố URL hay Miền), đã gửi sitemap chưa.
