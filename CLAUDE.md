@AGENTS.md

## Riêng cho Claude Code

- Trả lời người dùng bằng tiếng Việt, ngắn gọn, bằng lời thường — người dùng **không rành kỹ thuật**. Đừng bắt họ gõ lệnh hay sửa YAML; bạn chạy `./sb …` thay họ. Khi cần họ đọc/duyệt, chỉ họ mở **Bàn làm việc** (bấm đúp `Mo-Second-Brain`), mục "Chờ bạn duyệt".
- Skill của dự án nằm trong `.claude/skills/`: `/research <từ khoá>`, `/publish <file>`, `/connect`, `/seo-article <từ khoá>`, `/write-content <đề bài>`. Người dùng thường nói bằng lời ("nghiên cứu Price action", "viết bài SEO về …", "đăng bài X") — hãy dùng skill tương ứng.
- Bản nháp nghiên cứu luôn vào `knowledge/_inbox/`; chỉ đăng khi người dùng nói đã duyệt (hoặc họ bấm Duyệt & đăng trên Bàn làm việc).
- Bài viết SEO/GEO (`articles/`) chỉ dùng dữ kiện từ ghi chú tri thức; không sửa tri thức để hợp từ khoá.
- "Đưa lên web" = commit & push lên nhánh mặc định (Bàn làm việc có nút làm việc này). Hỏi trước khi push nếu người dùng chưa nhờ.
- **Máy tính và cloud chạy song song**, chung một kho trên GitHub (nhánh mặc định là bản chính; xem nhánh: `git remote show origin | grep HEAD`). Bàn làm việc tự lấy bản mới khi mở và trước khi đưa lên web, gộp được thì tự gộp, không gộp được thì dừng và giữ nguyên bài trên máy.
  - Đầu mỗi việc trên máy: `git pull` trước khi sửa. Đừng sửa cùng một bài ở hai nơi cùng lúc.
  - Phiên cloud thường làm trên nhánh `claude/…` riêng. Bản nháp hay thay đổi làm trên cloud chỉ hiện ở máy (mục "Chờ bạn duyệt") và trên web khi đã vào nhánh mặc định: khi người dùng bảo "đưa lên web" / "gửi về máy" / "đồng bộ", `git fetch origin <nhánh-mặc-định> && git merge origin/<nhánh-mặc-định>`, rồi `git push origin HEAD:<nhánh-mặc-định>`.
  - Người dùng nhờ "gộp thay đổi từ GitHub": `git pull --no-rebase`, sửa xung đột (giữ nội dung cả hai bên nếu được, hỏi khi hai bên mâu thuẫn), `./sb check`, commit, push.
- Skill: skill trong `.claude/skills/` của repo chạy được cả trên máy lẫn cloud. Skill cài riêng trên máy (`~/.claude/skills/`, plugin) chỉ có trên máy đó; muốn dùng trên cloud thì chép thư mục skill vào `.claude/skills/` rồi commit — trừ skill chứa mật khẩu/khoá API hoặc cần phần mềm chỉ có trên máy.
- **Harness (skill `project-harness`)** — áp dụng cả trên máy lẫn cloud cho việc lớn: sửa chương trình (`src/`, `templates/`, `assets/`), nghiên cứu/viết nhiều bài một lượt, sửa hàng loạt tri thức. Đọc `.harness/WORKFLOW.md` và `.harness/README.md`; `node .harness/harness.mjs start --project secondbrain …` → làm → `checkpoint` → `verify` (chạy `./sb check` + test harness) → viết `review-evidence.md` → `review` → `finish`. Ghi bài học có bằng chứng vào `.harness/LESSONS.md`. `finish` không thay cho việc người dùng duyệt hay "đưa lên web". Việc nhỏ chỉ cần `./sb check`. Đầu phiên cloud mới: `node .harness/harness.mjs doctor`.
- Lần đầu trên máy người dùng: nếu `./sb` báo không tải được chương trình, kiểm tra mạng / bản phát hành `ban-moi-nhat` trên GitHub; máy có Rust thì `./sb` tự build từ mã nguồn. Người dùng nhờ "cập nhật chương trình" (vd. sau khi code đổi trên cloud): `./sb cap-nhat` — trên Windows nhờ họ đóng cửa sổ Second Brain trước.
