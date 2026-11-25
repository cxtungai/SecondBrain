# Harness cho Second Brain

Quy trình làm việc có kiểm chứng cho các việc lớn trong dự án: sửa chương trình (`src/`, `templates/`, `assets/`), nghiên cứu nhiều ghi chú, viết loạt bài SEO. Chạy bằng Node.js, không cần API key hay gói npm. Chạy được trên máy (Windows/macOS) lẫn phiên Claude trên cloud, vì mọi thứ nằm trong repo.

Chạy tại thư mục gốc dự án:

```
node .harness/harness.mjs list
node .harness/harness.mjs doctor
node .harness/harness.mjs baseline
node .harness/harness.mjs start --project secondbrain --title "Sửa X" --accept "Kết quả quan sát được; ./sb check đạt" --mode clear
node .harness/harness.mjs checkpoint --task TASK_ID --note "Đã làm gì, file nào" --next "Bước tiếp theo"
node .harness/harness.mjs verify --task TASK_ID
node .harness/harness.mjs review --task TASK_ID --summary "Đã đối chiếu" --evidence ".harness/runs/TASK_ID/review-evidence.md" --gates "Kết quả từng kiểm tra thủ công"
node .harness/harness.mjs finish --task TASK_ID
node .harness/harness.mjs status
```

- `verify` chạy 4 kiểm tra: `./sb check` và `./sb build` (qua `.harness/checks/check_secondbrain.mjs`, tự chọn cargo / `bin/secondbrain.exe` / `bash sb`), `cargo test` (qua `check_rust_tests.mjs`; máy không có Rust thì bỏ qua khi không đổi code, báo lỗi khi có đổi — vì bản tải sẵn không phản ánh code vừa sửa) và test của harness, ghi log và dấu vân tay (SHA-256) của mã nguồn + tri thức. Sửa file sau khi verify/review thì `finish` từ chối — phải chạy lại.
- `review` cần file bằng chứng có thật (`review-evidence.md`), ghi kết quả các kiểm tra thủ công trong `projects.json`.
- `finish` chỉ đánh dấu `local-complete`; không commit, không push. Đưa lên web vẫn theo CLAUDE.md (chỉ khi người dùng nhờ).
- `runs/` và `reports/` chỉ nằm trên từng máy (đã bỏ khỏi git). Muốn tiếp tục một việc ở nơi khác, ghi điểm dừng vào bản nháp / commit, không dựa vào `runs/`.
- Bài học dùng lại được (có bằng chứng) ghi vào `LESSONS.md` — file này có trong git nên máy và cloud cùng thấy.

Việc nhỏ (sửa một câu, đăng một bài đã duyệt) không cần harness; chỉ cần `./sb check`.

Bản thích nghi từ harness ở workspace Codex của người dùng; nguồn ý tưởng xem `SOURCES.md`.
