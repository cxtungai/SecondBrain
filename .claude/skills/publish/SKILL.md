---
name: publish
description: Đăng bản nháp đã được người dùng đọc và duyệt lên Second Brain — ghi chú tri thức từ knowledge/_inbox (rồi kết nối với tri thức sẵn có) hoặc bài viết SEO/GEO trong articles/. Dùng khi người dùng nói đã duyệt/sửa xong bài và muốn đăng, post hoặc publish một bản nháp.
argument-hint: [tên file trong knowledge/_inbox hoặc articles/<file>.md]
---

# Đăng bản nháp đã duyệt

Bản nháp: **$ARGUMENTS** (trống → chạy `./sb publish` để liệt kê; nếu có nhiều bản nháp, hỏi người dùng muốn đăng bản nào).

**Bài viết SEO/GEO** (`articles/…`): chỉ cần `./sb publish articles/<file>.md` — script đặt `status: published`, `published_on`, `updated` và **từ chối** nếu ghi chú gốc (`based_on`) chưa `verified` (khi đó đăng ghi chú gốc trước; chỉ dùng `--force` khi người dùng chấp nhận rõ ràng). Sau đó `./sb check && ./sb build`, commit `Đăng bài: <tiêu đề>`. Không cần bước kết nối: trang ghi chú tự hiện "Bài viết về chủ đề này". Phần dưới dành cho ghi chú tri thức.

## 1. Kiểm tra

- Đọc bản nháp. Đây là nội dung người dùng **đã đọc và sửa** — không viết lại, không đổi ý. Chỉ sửa lỗi định dạng/frontmatter khi `check` báo lỗi, và nói rõ đã sửa gì.
- `./sb check --inbox` phải sạch lỗi cho file này.
- Nếu còn nguồn chưa chấm điểm uy tín, hoặc dữ kiện chỉ dựa trên nguồn điểm thấp: nhắc người dùng trước khi đăng ở trạng thái `verified`.

## 2. Đăng

```bash
./sb publish <file>                 # status: verified (mặc định)
./sb publish <file> --status draft  # nếu người dùng muốn đăng nhưng chưa chắc chắn
```

Script chuyển file sang `knowledge/<loại>/`, điền `reviewed`/`updated`, giữ `created` nếu là bản cập nhật, rồi in **gợi ý kết nối**.

## 3. Kết nối tri thức (bắt buộc)

Làm theo gợi ý script in ra:

- Thêm `[[slug-moi]]` vào ghi chú **chủ đề** phù hợp, đúng mục theo loại (Khái niệm / Mô hình & phương pháp / …). Nếu chưa có chủ đề trụ cột nào phù hợp, đề xuất tạo mới và hỏi người dùng.
- Tại các ghi chú khác có nhắc tên ghi chú mới mà chưa link: bọc đúng cụm từ đó thành `[[slug-moi|cụm từ]]`. Chỉ thêm liên kết, không đổi câu chữ.
- Thêm ghi chú mới vào mục `## Liên quan` của 1–3 ghi chú gần nghĩa nhất (nếu chưa có liên kết tới).
- Bổ sung liên kết bị sót trong chính ghi chú mới (`./sb kb mentions <slug>`).
- Chỉ thêm liên kết thì không đổi `updated` của các ghi chú khác.

## 4. Kiểm tra lại và lưu

- `./sb check` và `./sb build` phải chạy thành công.
- Commit với thông điệp `Đăng: <tiêu đề>` (kèm danh sách ghi chú đã thêm liên kết).
- Push theo quy trình của repo. Site chỉ cập nhật khi thay đổi vào nhánh chính — nếu bạn đang làm trên nhánh khác, hỏi người dùng trước khi merge/push lên nhánh chính.

## 5. Báo cáo

Ghi chú đã đăng (đường dẫn trang), các ghi chú đã thêm liên kết, và các mục "tri thức còn thiếu" mới phát sinh (gợi ý nghiên cứu tiếp bằng `/research`).
