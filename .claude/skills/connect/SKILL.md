---
name: connect
description: Rà soát và tăng cường liên kết giữa các ghi chú trong Second Brain — thêm [[liên kết]] cho chỗ nhắc tên chưa liên kết, nối ghi chú cô lập vào chủ đề trụ cột, cập nhật ghi chú chủ đề, liệt kê tri thức còn thiếu. Dùng khi người dùng muốn kết nối, liên kết, dọn dẹp, tối ưu hoặc bảo trì kho tri thức.
---

# Kết nối tri thức toàn kho

Mục tiêu: mọi ghi chú đều nằm trong ít nhất một chủ đề trụ cột, và các ghi chú liên quan đều trỏ tới nhau.

## 1. Đo hiện trạng

```bash
npm run kb -- stats
npm run kb -- mentions   # nhắc tên ghi chú khác mà chưa đặt [[liên kết]]
npm run kb -- orphans    # ghi chú không liên kết với ai
npm run kb -- gaps       # [[liên kết]] chưa có ghi chú
```

Xem thêm trang `/kien-thuc/` (mục "Chưa thuộc chủ đề nào") khi chạy `npm run dev`.

## 2. Sửa

- **Nhắc tên chưa liên kết:** bọc cụm từ thành `[[slug|cụm từ]]` ở lần xuất hiện đầu tiên trong ghi chú. Bỏ qua nếu nghĩa khác (vd. "code" thường ≠ phương pháp CODE).
- **Ghi chú cô lập / chưa thuộc chủ đề:** thêm vào ghi chú `topic` phù hợp (đúng mục theo loại) và vào `## Liên quan` của ghi chú gần nghĩa nhất.
- **Chủ đề quá lớn** (> ~15 liên kết): đề xuất tách thành chủ đề con, hỏi người dùng trước khi làm.
- **Tri thức còn thiếu:** không tự viết nội dung. Liệt kê theo số lần được nhắc và đề xuất người dùng chạy `/research <tên>` cho các mục quan trọng nhất.
- Chỉ thêm/sửa liên kết — không thay đổi ý nghĩa nội dung, không đổi `updated`.

## 3. Kiểm tra và báo cáo

- `npm run check` và `npm run build` chạy thành công.
- Commit: `Kết nối tri thức: <tóm tắt>`.
- Báo cáo: số liên kết đã thêm, ghi chú đã nối vào chủ đề, top mục còn thiếu nên nghiên cứu.
