@AGENTS.md

## Riêng cho Claude Code

- Trả lời người dùng bằng tiếng Việt, ngắn gọn, bằng lời thường — người dùng **không rành kỹ thuật**. Đừng bắt họ gõ lệnh hay sửa YAML; bạn chạy `./sb …` thay họ. Khi cần họ đọc/duyệt, chỉ họ mở **Bàn làm việc** (bấm đúp `Mo-Second-Brain`), mục "Chờ bạn duyệt".
- Skill của dự án nằm trong `.claude/skills/`: `/research <từ khoá>`, `/publish <file>`, `/connect`, `/seo-article <từ khoá>`, `/write-content <đề bài>`. Người dùng thường nói bằng lời ("nghiên cứu Price action", "viết bài SEO về …", "đăng bài X") — hãy dùng skill tương ứng.
- Bản nháp nghiên cứu luôn vào `knowledge/_inbox/`; chỉ đăng khi người dùng nói đã duyệt (hoặc họ bấm Duyệt & đăng trên Bàn làm việc).
- Bài viết SEO/GEO (`articles/`) chỉ dùng dữ kiện từ ghi chú tri thức; không sửa tri thức để hợp từ khoá.
- "Đưa lên web" = commit & push lên nhánh mặc định (Bàn làm việc có nút làm việc này). Hỏi trước khi push nếu người dùng chưa nhờ.
- Lần đầu trên máy người dùng: nếu `./sb` báo không tải được chương trình, kiểm tra mạng / bản phát hành `ban-moi-nhat` trên GitHub; máy có Rust thì `./sb` tự build từ mã nguồn.
