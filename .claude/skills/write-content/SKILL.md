---
name: write-content
description: Viết bài content (bài blog, bài SEO, post mạng xã hội, kịch bản video, email…) dựa trên tri thức trong Second Brain, có trích nguồn và tuân thủ quy tắc dùng tri thức. Dùng khi người dùng nhờ viết bài hoặc nội dung về một chủ đề có trong kho.
argument-hint: <đề bài, kênh đăng, độ dài, giọng văn>
---

# Viết content từ Second Brain

Đề bài: **$ARGUMENTS** (thiếu kênh/độ dài/giọng văn thì dùng mặc định: bài blog ~1.200 từ, giọng chuyên gia thân thiện, tiếng Việt).

Bài để đăng lên **chính website Second Brain** (kéo traffic, SEO/GEO) → dùng skill `/seo-article` (lưu vào `articles/`). Skill này dành cho content đăng ở kênh khác (blog ngoài, mạng xã hội, email, video…).

## 1. Thu thập tri thức

- `./sb kb search "<chủ đề>"`, bắt đầu từ ghi chú loại `topic`, đi theo liên kết (`./sb kb show <slug>`) và đọc file `.md` của các ghi chú liên quan.
- Tuân thủ quy tắc trong `cau-hinh.yml` (`agent_rules`): chỉ dùng dữ kiện/số liệu từ ghi chú `verified`; `draft` chỉ làm ý tưởng; bỏ qua `outdated`; không bịa số liệu, nguồn, trích dẫn.
- Kho thiếu thông tin cần thiết → nói rõ phần thiếu, đề xuất `/research <chủ đề>`; không lấp bằng kiến thức không có nguồn.

## 2. Viết

- Lập dàn ý ngắn trước; với bài dài, cho người dùng xem dàn ý nếu đề bài còn mơ hồ.
- Mở bài trả lời thẳng câu hỏi chính; mỗi phần một ý; ví dụ cụ thể lấy từ case study trong kho nếu có.
- Trích nguồn gốc (theo `sources` của ghi chú) cho số liệu và khẳng định quan trọng.
- Chủ đề tài chính / đầu tư / sức khoẻ: không khuyến nghị, không hứa hẹn kết quả, nêu rủi ro.

## 3. Giao bài

- Mặc định trả bài trong chat. Nếu người dùng muốn lưu file: `content/<YYYY-MM-DD>-<slug>.md` (không lưu vào `knowledge/` — đó là kho tri thức, không phải bài đăng).
- Cuối bài, liệt kê: ghi chú đã dùng (slug + trạng thái) và nguồn gốc đã trích.
