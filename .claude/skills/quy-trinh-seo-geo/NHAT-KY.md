# Nhật ký cập nhật quy trình SEO/GEO

Mỗi lần tối ưu được điều gì mới thì thêm một mục ở **đầu** danh sách: ngày, đổi gì, vì sao, bằng chứng. Quy trình (`SKILL.md`, `references/`) luôn là bản mới nhất; nhật ký cho biết vì sao nó thành ra như vậy.

## 2026-11-26 — Bản 1.1: kiểm chứng bằng tài liệu gốc khi viết ghi chú tri thức SEO/GEO

**Đọc hướng dẫn chính thức của Google** "Optimizing your website for generative AI features on Google Search" (2026), rút ra 4 điều:

- **Thêm bước:** kiểm **Cài đặt → Search generative AI = Include** trong Search Console.
- **Đo lường:** thêm báo cáo **Generative AI performance**.
- **Sửa `geo.md`:**
  - Google đúng là nói không cần chia nhỏ bài cho AI. Trước đây ý này mới chỉ được thuật lại, nay đã thấy trong văn bản gốc.
  - Google Search bỏ qua `llms.txt`, nên giữ file này chỉ để phục vụ AI agent khác.
- **Phản biện:** bài tổng quan GEO 2026 (Martinez, bản thảo) cho rằng mức tăng "tới 40%" chỉ đúng khi trang đã được lấy vào câu trả lời. Vì vậy ưu tiên **được lập chỉ mục và nội dung đúng chủ đề** trước các mẹo viết.

## 2026-11-26 — Bản 1 (trang Second Brain đầu tiên)

**Nghiên cứu repo:** đã đọc claude-seo, geo-seo-claude, marketingskills (ai-seo), AutoGEO, GEO (Princeton, KDD 2024), squirrelscan, geo-optimizer. Kết luận: không cài nguyên bộ nào, chỉ lấy quy tắc. Nguồn ghi ở `../research/NGUON.md`.

**Kiểm tra web thật, thấy 4 lỗi:**

1. Không trang nào có canonical: template có chỗ nhưng chương trình chưa điền.
2. `robots.txt` không có tác dụng vì web nằm trong thư mục con. Bằng chứng: `/robots.txt` ở gốc tên miền trả 404.
3. Trang chủ thiếu JSON-LD; tác giả để trống nên `author: null`.
4. Không có ảnh chia sẻ (og:image, thẻ X/Twitter).

**Đợt 1, kỹ thuật** (commit `ac0dee1`):

- thêm canonical, `og:url`, `og:image` 1200×630, `twitter:card`, `article:*_time`;
- JSON-LD: WebSite + Organization ở trang chủ; Article có author (tên thương hiệu khi trống), publisher, image;
- thêm `feed.xml`;
- `robots.txt` nêu tên bot AI, lặp đủ quy tắc theo RFC 9309;
- thêm chỗ điền mã xác minh Google/Bing trong `cau-hinh.yml`.

Bằng chứng: quét 36 trang dựng thử, JSON-LD hợp lệ, 0 trang công khai thiếu canonical. Web thật đã kiểm sau khi đăng.

**Đợt 2, nội dung:**

- `./sb seo` có thêm 8 gợi ý GEO về cấu trúc (đoạn mở đầu, mục tự đứng được, đoạn quá dài, cấp tiêu đề, dẫn nguồn, mục giới hạn, các bước, trích nguyên văn);
- thêm hướng dẫn `research/references/geo.md`;
- người dùng dặn: tri thức đứng trên SEO/GEO.

**Đăng ký** (người dùng làm, agent hướng dẫn; commit `42f142c` gắn mã Google):

- Google Search Console, kiểu Tiền tố URL: đã xác minh, đã yêu cầu lập chỉ mục trang chủ;
- Bing: đã Import từ Google Search Console;
- Brave: đã hướng dẫn gửi.

**Tự động** (commit `0c0fb3e`):

- job `sau-khi-dang`: chạy `kiem-tra-web.mjs`, rồi `indexnow.mjs`;
- lần chạy thật đầu tiên xanh;
- gửi tay 22 trang cho IndexNow, nhận mã 202.

**Bài học:**

- Luôn kiểm trên web thật, đừng chỉ đọc template.
- Web nằm trong thư mục con thì phải gửi sitemap qua Search Console.
- Lần đầu có khoá IndexNow phải gửi tay toàn bộ web.
- Bing cần địa chỉ sitemap đầy đủ.
- Google không có API hợp lệ để yêu cầu lập chỉ mục bài thường.
