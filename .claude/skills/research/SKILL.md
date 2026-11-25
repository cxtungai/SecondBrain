---
name: research
description: Nghiên cứu một từ khoá/chủ đề để bổ sung vào Second Brain — tìm tài liệu từ nhiều nguồn, chấm điểm uy tín từng nguồn (1–5), tổng hợp thành bài nháp tiếng Việt có liên kết với tri thức sẵn có, lưu vào knowledge/_inbox để người dùng đọc, sửa và duyệt. Dùng khi người dùng đưa một từ khoá (vd. "Price action"), nhờ tìm hiểu/tổng hợp một chủ đề, hoặc muốn lấp một mục trong "Tri thức còn thiếu".
argument-hint: <từ khoá hoặc chủ đề>
---

# Nghiên cứu từ khoá cho Second Brain

Từ khoá: **$ARGUMENTS**
Nếu trống: hỏi người dùng, hoặc đề xuất 3 mục từ `npm run kb -- gaps`.

Đọc `AGENTS.md` (thang điểm uy tín, cấu trúc ghi chú) trước khi bắt đầu. Bạn **không tự đăng**: kết quả là bản nháp trong `knowledge/_inbox/`, chỉ người dùng duyệt mới đăng.

## 1. Xem kho đã có gì

- `npm run kb -- search "<từ khoá>"` — thử cả tiếng Việt, tiếng Anh, không dấu, tên viết tắt.
- Nếu đã có ghi chú cùng chủ đề: đọc nó. Bài nghiên cứu là **bản cập nhật** — dùng *cùng tên file*, giữ phần đúng, bổ sung phần mới, và nói rõ thay đổi trong báo cáo.
- `npm run kb -- list --type topic` — chủ đề trụ cột nào nên chứa bài này.
- Ghi lại các slug liên quan để đặt `[[liên kết]]`.

## 2. Tìm và đọc nguồn

- WebSearch nhiều truy vấn: định nghĩa, nguồn gốc / người đặt ra khái niệm, nghiên cứu & dữ liệu, phản biện / hạn chế; cả tiếng Anh và tiếng Việt.
- Đọc trực tiếp từng trang (WebFetch). Không tổng hợp từ đoạn trích của kết quả tìm kiếm.
- Mục tiêu 5–10 nguồn, ưu tiên nguồn gốc (sách/bài của tác giả khái niệm, nghiên cứu, cơ quan chính thức, sở giao dịch).
- Khẳng định quan trọng cần ≥ 2 nguồn độc lập, hoặc 1 nguồn điểm 5.
- Nếu không truy cập được web (mạng bị chặn…): dừng và báo người dùng. **Không** viết từ trí nhớ rồi gắn nguồn.

## 3. Chấm điểm uy tín từng nguồn (1–5)

Theo thang trong `AGENTS.md` (mục "Thang điểm uy tín nguồn"). Xuất phát từ mức của loại nguồn, rồi điều chỉnh ±1 theo 5 tiêu chí: thẩm quyền, bằng chứng, độc lập (không bán hàng/xung đột lợi ích), thời điểm, đối chiếu được với nguồn khác. Ghi `kind`, `credibility`, `credibility_note` (1 câu lý do), `accessed`.

Nguồn điểm 1: không dùng làm căn cứ cho khẳng định. Nếu cần nhắc tới quan điểm phổ biến từ nguồn yếu, nói rõ đó là quan điểm, không phải sự thật.

## 4. Viết bài tổng hợp

- **Chọn loại ghi chú:** chủ đề rộng (vd. "Price action") → `topic` làm trụ cột, kèm 2–4 ghi chú con cho khái niệm/phương pháp cốt lõi; một khái niệm → `concept`; phương pháp, setup, quy trình → `framework`; số liệu → `fact`; ví dụ thực tế → `case-study`; tóm tắt một tài liệu → `source`.
- Theo template `knowledge/_templates/<type>.md`; xoá các chú thích `<!-- -->` khi đã viết xong phần đó.
- Viết tiếng Việt, rõ ràng, có cấu trúc. **Câu đầu tiên trả lời thẳng "X là gì?"** — Google/AI hay trích câu này.
- Gắn nguồn sau mỗi khẳng định quan trọng: `(Nguồn [2])` hoặc `[2][4]` — số là vị trí trong danh sách `sources`.
- Phân biệt rõ: điều được đồng thuận / điều còn tranh cãi / ý kiến của một tác giả. Nêu giới hạn và cách hiểu sai thường gặp.
- **Liên kết:** thuật ngữ đã có ghi chú → `[[slug]]` hoặc `[[slug|nhãn]]`; thuật ngữ quan trọng chưa có → `[[ten-de-xuat]]` (sẽ hiện trong trang "Tri thức còn thiếu"). Chỉ link lần xuất hiện đầu tiên của mỗi thuật ngữ.
- Cuối bài: `## Câu hỏi thường gặp` (3–5 câu, mỗi câu là `### Câu hỏi?`) và `## Liên quan`.
- Chủ đề tài chính / đầu tư / trading / sức khoẻ: không đưa khuyến nghị, không hứa hẹn lợi nhuận, nêu rủi ro; mọi số liệu hiệu suất phải có nguồn và bối cảnh (thị trường, giai đoạn, phương pháp đo).
- Frontmatter: `status: draft`, `summary` 1–2 câu, `tags` (dùng lại tag sẵn có — xem `npm run kb -- stats` / trang Kiến thức), `aliases` (tên tiếng Anh, viết tắt), và:

  ```yaml
  research:
    keyword: "<từ khoá gốc>"
    date: <YYYY-MM-DD hôm nay>
    agent: claude
  ```

## 5. Lưu và kiểm tra

- Lưu vào `knowledge/_inbox/<slug>.md` (slug không dấu, chữ thường, nối bằng `-`). Ghi chú con cũng để trong `_inbox`.
- `npm run check -- --inbox` → không còn lỗi cho các file vừa tạo.
- `npm run kb -- mentions <slug>` → thêm các liên kết bị sót.

## 6. Báo cáo cho người dùng

Ngắn gọn, tiếng Việt:

1. File bản nháp (và cách xem trước: `npm run dev` → mục "Chờ duyệt" ở trang chủ).
2. 3–5 ý chính của bài.
3. Bảng nguồn: `# | Nguồn | Loại | Điểm | Lý do`.
4. Những điểm còn tranh cãi / chưa chắc chắn mà người dùng nên kiểm tra kỹ.
5. Liên kết tới ghi chú sẵn có, và các ghi chú mới nên nghiên cứu tiếp.
6. Bước tiếp theo: đọc, sửa trực tiếp file, rồi gọi `/publish <file>`.
