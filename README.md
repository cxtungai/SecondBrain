# Second Brain

Kho tri thức chuyên môn **đã được bạn đọc, hiểu và duyệt**. Nó vừa là website để tra cứu, vừa là nguồn dữ liệu để AI (Claude…) viết content chính xác, có nguồn.

Bạn không cần biết lập trình: **nói chuyện với Claude** để nghiên cứu, viết, sửa; **bấm nút trên Bàn làm việc** để duyệt và đưa lên web.

## Dùng hằng ngày

```
 1. Nhờ Claude      "Nghiên cứu từ khoá: Price action"
        │            Claude tìm nguồn uy tín, chấm điểm từng nguồn, viết bản nháp tiếng Việt
        ▼
 2. Mở Bàn làm việc  bấm đúp Mo-Second-Brain  →  trình duyệt tự mở
        │            mục "Chờ bạn duyệt": đọc, bấm ✏️ Sửa nếu cần, rồi ✅ Duyệt & đăng
        ▼
 3. Đưa lên web      bấm 🚀 Đưa lên web  →  1–2 phút sau website cập nhật
```

**Mở Bàn làm việc:** trong thư mục SecondBrain, bấm đúp

- Windows: `Mo-Second-Brain.bat`
- macOS: `Mo-Second-Brain.command` (lần đầu: chuột phải → Open)

Lần đầu chương trình tự tải về (khoảng 5 MB). Một cửa sổ đen hiện ra, **giữ nó mở** trong lúc dùng; đóng cửa sổ là tắt.

**Trên Bàn làm việc có:**

| | |
| --- | --- |
| 📥 Chờ bạn duyệt | Bản nháp Claude viết hoặc bạn tự viết: Sửa · Duyệt & đăng · Bỏ (vào thùng rác, lấy lại được) |
| 🚀 Đưa lên Internet | **Đưa lên web**: lưu mọi thay đổi lên GitHub, website tự cập nhật. **Lấy bản mới**: tải về bản nháp Claude viết trên cloud |
| ✍️ Tự viết ghi chú | Ghi lại điều bạn vừa đọc: nhập tiêu đề, chọn loại, rồi viết trong ô soạn thảo có xem trước |
| 🤖 Nhờ Claude làm | Câu mẫu để chép vào Claude: nghiên cứu, nạp tài liệu đã đọc, viết bài SEO, sửa bản nháp… |
| 🔧 Cần chú ý | Lỗi và gợi ý cải thiện, bấm vào để sửa |
| ⚙️ Cài đặt trang | Tên trang, mô tả, tác giả, dòng miễn trừ trách nhiệm |

Bạn cũng có thể nói thẳng với Claude: *"đăng bài Price action"*, *"sửa đoạn mở đầu bài X cho dễ hiểu hơn"*, *"đưa thay đổi lên web"*.

## Dùng song song máy tính và cloud

Kho trên GitHub là bản chính; máy tính và Claude trên cloud cùng làm trên đó.

- **Trên cloud** (điện thoại, trình duyệt): nhờ Claude nghiên cứu, viết nháp, rồi nói *"đưa lên web"* để bản nháp về GitHub.
- **Trên máy**: mở Bàn làm việc — chương trình tự lấy bản mới, bản nháp từ cloud hiện ở "Chờ bạn duyệt". Đang mở mà muốn lấy ngay: bấm **🔄 Lấy bản mới**.
- Đừng sửa cùng một bài ở hai nơi cùng lúc. Lỡ sửa thì Bàn làm việc không ghi đè gì, chỉ báo; nhờ Claude *"gộp thay đổi từ GitHub giúp tôi"*.
- Skill bạn cài riêng trên máy không có trên cloud. Muốn dùng cả hai nơi: nhờ Claude trên máy *"chép các skill của tôi vào dự án để dùng được trên cloud"*.

## Cài đặt lần đầu (một lần)

1. Cài **Claude** (ứng dụng máy tính) và **Git** (Windows: [git-scm.com](https://git-scm.com/download/win), chọn mặc định hết).
2. Trong Claude → **Code**, nhờ: *"Tải repo cxtungai/SecondBrain về máy và mở thư mục đó"*. Hoặc tự làm:

   ```bash
   git clone https://github.com/cxtungai/SecondBrain.git
   ```

3. Bấm đúp `Mo-Second-Brain` trong thư mục vừa tải.
4. Bật website (một lần): trên GitHub vào repo → **Settings → Pages → Source: GitHub Actions**. Địa chỉ web: `https://cxtungai.github.io/SecondBrain/`.

Lần đầu bấm "Đưa lên web", Git có thể hỏi đăng nhập GitHub — đăng nhập một lần là xong.

## Website có gì

- **Trung tâm kiến thức**: mỗi chủ đề trụ cột kèm cụm ghi chú con.
- **Bài viết**: bài dễ đọc kéo traffic từ Google/AI search; mỗi bài dẫn về "Tri thức gốc". Tri thức là chính, bài viết là bài con.
- **Từ điển A–Z**, **Đồ thị tri thức**, **Tìm kiếm** (gõ không dấu cũng được).
- **Trang ghi chú**: mục lục, nguồn kèm điểm uy tín 1–5, câu hỏi thường gặp, liên kết hai chiều, "Đọc tiếp".
- **Tri thức còn thiếu**: các khái niệm đã nhắc tới nhưng chưa có ghi chú — gợi ý nghiên cứu tiếp.
- **Giới thiệu & quy trình biên tập**: minh bạch cách nghiên cứu, chấm điểm, duyệt.
- **Cho AI**: `llms.txt`, `llms-full.txt`, `knowledge.json`, bản `.md` từng ghi chú — AI nào cũng đọc được.

## Dành cho người rành kỹ thuật

- Một chương trình **Rust** duy nhất (`secondbrain`, ~5 MB, không cần Node): sinh web tĩnh, chạy Bàn làm việc, và các lệnh cho AI agent. Mã nguồn trong `src/`, giao diện trong `templates/` + `assets/` (nhúng vào chương trình).
- Tri thức là Markdown trong `knowledge/` (mở được bằng Obsidian), bài viết trong `articles/`, cấu hình trong `cau-hinh.yml`.
- Lệnh: `./sb mo | check | build | new | publish | kb | seo` — xem `./sb help`. `./sb` dùng `cargo` nếu có, không thì tải bản build sẵn.
- GitHub Actions: `deploy.yml` build và đăng GitHub Pages từ nhánh mặc định; `release.yml` build chương trình cho Windows/macOS/Linux vào bản phát hành `ban-moi-nhat` (file `Mo-Second-Brain` tải từ đây).
- Hướng dẫn cho AI agent: [`AGENTS.md`](AGENTS.md) — cấu trúc ghi chú, thang điểm uy tín nguồn, quy tắc liên kết, bài viết SEO/GEO.

GitHub Pages với repo private cần gói trả phí. Cách khác: Cloudflare Pages / Netlify với lệnh build `./sb build`, thư mục `dist`, biến `SITE_URL`.

Các ghi chú có tag `vi-du` (Second Brain, CODE, PARA…) chỉ để minh hoạ — xoá khi bắt đầu dùng thật.
