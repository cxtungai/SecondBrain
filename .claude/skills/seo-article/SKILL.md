---
name: seo-article
description: Viết bài viết SEO/GEO cho website Second Brain (thư mục articles/) để kéo traffic — nhắm một từ khoá / ý định tìm kiếm, chỉ dùng dữ kiện từ ghi chú tri thức đã có và dẫn liên kết về ghi chú gốc, kiểm tra trùng từ khoá. Dùng khi người dùng muốn viết bài SEO, bài kéo traffic, bài blog cho web, tối ưu SEO/GEO cho một từ khoá, hoặc "bài con" link về tri thức.
argument-hint: <từ khoá, vd. "cách vẽ hỗ trợ kháng cự">
---

# Viết bài SEO/GEO từ tri thức

Từ khoá: **$ARGUMENTS** (trống → hỏi người dùng, hoặc đề xuất 3 góc bài từ các ghi chú `verified` có nhiều backlinks).

Đọc mục "Bài viết SEO/GEO" trong `AGENTS.md` trước. Nguyên tắc: **tri thức là nguồn sự thật, bài viết là lớp trình bày.** Bài viết không đưa dữ kiện mới, không làm sai lệch tri thức để hợp từ khoá. Kết quả là **bài nháp** (`status: draft`) — chỉ người dùng duyệt mới đăng.

## 1. Kiểm tra tri thức trước

- `./sb kb search "<từ khoá>"` (thử có dấu, không dấu, tiếng Anh) và `./sb kb show <slug>` để đọc ghi chú liên quan.
- Kho **chưa có** tri thức đủ để trả lời từ khoá → dừng, đề xuất `/research <chủ đề>` trước. Không viết bài từ trí nhớ.
- Ghi chú liên quan còn `draft` → vẫn có thể viết bài nháp, nhưng nói rõ bài chỉ đăng được khi ghi chú gốc đã `verified`.

## 2. Ý định tìm kiếm và trùng từ khoá

- `./sb seo --keyword "<từ khoá>"` — trang nào trong site đã nhắm từ khoá này.
  - **Trùng ghi chú** (vd. "X là gì" khi đã có ghi chú X): truy vấn định nghĩa thuộc về ghi chú. Đề xuất cải thiện ghi chú (đoạn mở đầu, FAQ) hoặc chọn góc khác cho bài: hướng dẫn, so sánh, sai lầm thường gặp, cẩm nang cho người mới…
  - **Trùng bài viết**: cập nhật bài cũ thay vì viết bài mới.
- WebSearch từ khoá (tiếng Việt) để hiểu người tìm muốn gì: dạng kết quả đứng đầu (định nghĩa / hướng dẫn / danh sách / so sánh), các câu "Mọi người cũng hỏi", các ý mà bài top đều có. **Chỉ học cấu trúc và câu hỏi — không lấy dữ kiện từ đó.** Dữ kiện còn thiếu trong kho → ghi `[[ten-ghi-chu-de-xuat]]` và báo người dùng nên nghiên cứu.
- Chọn `intent`: `guide` | `how-to` | `compare` | `list` | `explain`, và 2–5 từ khoá phụ (`keywords`).

## 3. Viết bài

Tạo file: `./sb new article "<Tiêu đề>" --keyword "<từ khoá>" --based-on slug-a,slug-b --intent <intent>`, rồi viết vào `articles/<slug>.md`.

- **Tiêu đề** ≤ 60 ký tự, có từ khoá chính, tự nhiên, không giật tít, không hứa hẹn.
- **description** 70–160 ký tự: người đọc nhận được gì; có từ khoá.
- **2–3 câu đầu trả lời ngay câu hỏi** (GEO: AI search và đoạn trích nổi bật lấy từ đây).
- Mỗi `##` là một câu hỏi phụ người đọc hay hỏi; câu ngắn, ví dụ cụ thể, bảng/danh sách khi so sánh hoặc liệt kê bước.
- **Mọi dữ kiện, số liệu, định nghĩa phải có trong ghi chú gốc** — đặt `[[slug]]` ngay chỗ dùng (lần đầu nhắc tới). Không cần chép nguồn ngoài vào bài: trang bài viết tự hiện mục "Tri thức gốc" có nguồn và điểm uy tín.
- Giữ đúng sắc thái của tri thức: điều còn tranh cãi vẫn nói là tranh cãi, giới hạn vẫn nêu rõ.
- Cuối bài: `## Câu hỏi thường gặp` (≥ 3 câu `### …?`) và `## Tìm hiểu sâu hơn` (danh sách `[[ghi chú]]`).
- Từ khoá xuất hiện tự nhiên (≤ ~3 lần/100 từ); dùng từ đồng nghĩa. Độ dài theo độ khó của câu hỏi (thường 800–1.500 từ) — không độn chữ.
- Tài chính/đầu tư/trading/sức khoẻ: không khuyến nghị mua bán, không hứa lợi nhuận, nêu rủi ro.
- Có thể dẫn tới bài viết khác bằng `[[slug-bai-viet]]` nếu liên quan.

## 4. Kiểm tra

- `./sb check` — không lỗi cho bài này (`based_on` tồn tại, có liên kết về ghi chú gốc, không trùng từ khoá).
- `./sb seo <slug> --drafts` — sửa các tiêu chí chưa đạt khi hợp lý (không đánh đổi độ chính xác lấy điểm).
- Xem trước: Khu duyệt bài trên web (`/duyet/`, sau khi đẩy bản nháp lên) hoặc Bàn làm việc (`./sb mo`) → "Chờ bạn duyệt"; hộp "SEO / GEO" chỉ hiện trên Bàn làm việc.

## 5. Báo cáo cho người dùng

1. File bài nháp và link xem trước.
2. Từ khoá chính/phụ, intent, lý do chọn góc này (và trang nào trong site đã nhắm từ khoá gần).
3. Ghi chú gốc đã dùng; dữ kiện còn thiếu nên `/research` thêm.
4. Điểm SEO/GEO và các tiêu chí còn chưa đạt.
5. Bước tiếp: đọc, sửa trực tiếp file, rồi `/publish articles/<file>.md` (chỉ đăng được khi ghi chú gốc đã `verified`).
