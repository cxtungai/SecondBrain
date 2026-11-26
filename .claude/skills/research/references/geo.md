# Viết để Google và AI tìm kiếm trích đúng (SEO/GEO) — không đánh đổi chất lượng tri thức

Dùng cho skill `research` (ghi chú tri thức) và `seo-article` (bài viết). GEO là viết sao cho ChatGPT, Perplexity, Claude, Gemini, Google AI Overviews **hiểu đúng và trích dẫn** trang của mình.

## Nguyên tắc số 1: tri thức đứng trên GEO

Người dùng dặn ngày 2026-11-26: tối ưu SEO/GEO được, nhưng **không được làm ảnh hưởng chất lượng kiến thức**. Khi một gợi ý bên dưới buộc phải thêm, bớt hay đổi ý của nguồn thì bỏ gợi ý đó. Chính xác thắng điểm SEO.

| Được làm (chỉ đổi cách trình bày) | Không được làm |
| --- | --- |
| Đưa ý chính lên đầu bài và đầu mỗi mục | Thêm khẳng định, số liệu hay câu trích **không có trong nguồn đã đọc** |
| Gọi đúng tên chủ thể thay cho "Nó / Điều này" | Đổi hay "làm đẹp" kết luận cho hấp dẫn |
| Tách đoạn quá dài theo ý, mỗi đoạn một ý trọn vẹn | Cắt vụn bài thành mẩu nhỏ "cho AI dễ đọc" (Google khuyên không làm) |
| Dùng bảng khi so sánh, danh sách đánh số khi có các bước | Viết một bản riêng cho AI khác bản cho người đọc |
| Gắn `(Nguồn [n])` đúng chỗ nguồn đó nói | Nhồi từ khoá (nghiên cứu GEO: gần như vô ích) |
| Trích ngắn nguyên văn tài liệu gốc kèm số trang | Bỏ điều kiện, giới hạn, chỗ "còn tranh cãi" cho câu gọn hơn |
| Nêu mục giới hạn / khi nào không áp dụng (từ nguồn đã có) | Tạo trích dẫn, đánh giá hay "chuyên gia" không có thật |

## Vì sao (bằng chứng)

- **GEO** (Aggarwal và cộng sự, KDD 2024): thêm trích dẫn nguồn, câu trích và số liệu giúp nội dung được AI nhắc tới nhiều hơn, tới khoảng 40%; nhồi từ khoá kiểu SEO cũ gần như không có tác dụng.
- **AutoGEO** (Wu và cộng sự, ICLR 2026): rút quy tắc từ chính Gemini, GPT, Claude. Các quy tắc lặp lại ở cả ba: nêu kết luận ngay đầu; mỗi đoạn một ý, tự đứng được; chính xác, cập nhật, có ngày; giải thích "vì sao / như thế nào"; định nghĩa thuật ngữ ở lần đầu; phân biệt khái niệm dễ nhầm; cân bằng nhiều góc nhìn; bảng và danh sách; không quảng cáo, không lan man.
- **Claude** rất kén nguồn: ưu tiên nội dung chính xác, dẫn nguồn rõ, nhiều chi tiết cụ thể (theo skill `ai-seo` của repo marketingskills).
- Những điều này trùng với cách kho vốn viết (nguồn có điểm uy tín, phân biệt đồng thuận / tranh cãi). GEO cho kho này nghĩa là **làm tri thức rõ hơn**, không phải viết khác đi.

## Tự kiểm trước khi lưu bản nháp

Phỏng theo thang "dễ được trích" (citability) của repo geo-seo-claude, rút gọn cho tiếng Việt:

1. **Trả lời trước.** Câu đầu bài trả lời thẳng câu hỏi chính ("X là …"). Mỗi mục `##` mở bằng 1–2 câu nêu ý chính của mục.
2. **Tự đứng được.** Mỗi đoạn gọi đúng tên chủ thể, không mở bằng "Nó / Điều này / Tuy nhiên / Ngoài ra". Đoạn khoảng 40–150 chữ, một ý.
3. **Cấu trúc.** `##` → `###` → `####`, không nhảy cấp. Tiêu đề mục dạng câu hỏi khi tự nhiên. Bảng khi so sánh từ 3 thứ. Danh sách đánh số cho các bước. In đậm thuật ngữ ở lần đầu.
4. **Bằng chứng cụ thể.** Số liệu kèm thị trường, giai đoạn, cách đo và `(Nguồn [n])`. Tên người, sách, năm, số trang. Không viết "nhiều nghiên cứu cho thấy" khi không nêu được nghiên cứu nào.
5. **Trích nguyên văn ngắn** từ tài liệu gốc cho ý cốt lõi (kèm số trang, kèm câu dịch sát nghĩa). Ngắn, đúng chữ, không trích để trang trí.
6. **Cân bằng.** Có mục giới hạn, chỗ còn tranh cãi hoặc khi nào không áp dụng.
7. **Phần riêng của kho.** Bản đồ sách có số trang, so sánh giữa các tác giả gốc, ví dụ có số: đây là thứ AI không tìm được ở nơi khác.
8. **Ngày.** `updated` đúng ngày sửa; số liệu có mốc thời gian.

`./sb seo <slug> --drafts` tự kiểm phần lớn mục 1–6 (đoạn mở đầu, mục tự đứng được, đoạn quá dài, cấp tiêu đề, số lần dẫn nguồn, mục giới hạn, các bước, trích nguyên văn). Đó chỉ là **gợi ý**: mục nào chưa đạt mà sửa sẽ hại độ chính xác thì để nguyên và nói lý do trong báo cáo.

## Khi sửa bài cũ theo gợi ý

- Chỉ đổi cách trình bày. Giữ nguyên khẳng định, con số, nguồn và sắc thái.
- Muốn thêm câu trích hay số liệu: đọc lại nguồn, có `(Nguồn [n])` và số trang. Không tìm được trong nguồn thì thôi.
- Ghi chú đã duyệt (`verified`): báo rõ cho người dùng đã sửa trình bày ở đâu để họ xem lại.

## Phần kỹ thuật chương trình đã tự lo

Không cần làm tay: địa chỉ chính thức (canonical), ảnh và thẻ khi chia sẻ link (Open Graph, X/Twitter), dữ liệu cấu trúc (WebSite, Organization, Article có tác giả, đơn vị xuất bản, nguồn trích; FAQ; Breadcrumb; từ điển thuật ngữ), sitemap có ngày sửa, tin mới `feed.xml`, `robots.txt` chào đón bot tìm kiếm và bot AI, `llms.txt`, bản Markdown từng ghi chú, `knowledge.json`. Bản nháp luôn `noindex`.

Việc của agent chỉ là: đúng tên mục `## Câu hỏi thường gặp` (mỗi câu `### …?`) để sinh dữ liệu FAQ, `summary` 70–160 ký tự, `updated` đúng.

## Việc người dùng làm một lần (hướng dẫn họ khi được hỏi)

- **Google Search Console** và **Bing Webmaster Tools**: đăng ký, chọn cách xác minh bằng thẻ HTML, dán mã vào `cau-hinh.yml` (`google_site_verification`, `bing_site_verification`; dán cả thẻ meta cũng được), đưa lên web, bấm xác minh, rồi gửi `sitemap.xml`. Web đang nằm dưới `/SecondBrain/` nên Google không tự đọc được `robots.txt` của mình; gửi sitemap là cách chắc chắn.
- **Brave Search**: gửi địa chỉ web tại search.brave.com/submit-url (Claude dùng kết quả Brave khi tìm nguồn).
- Muốn bền lâu: tên miền riêng (tốn phí hằng năm) giúp `robots.txt` có hiệu lực và tăng nhận diện.

## Theo dõi

Mỗi tháng: hỏi ChatGPT, Perplexity, Claude, Google (chế độ AI) 5–10 câu chính của các chủ đề đã đăng, ghi lại web có được trích không, trích đúng hay sai. Trích sai → xem lại đoạn bị trích (thường là đoạn không tự đứng được). Search Console cho biết từ khoá nào đã có người tìm thấy web.

## Nguồn

- GEO: Generative Engine Optimization — https://arxiv.org/abs/2311.09735 (mã: https://github.com/GEO-optim/GEO)
- AutoGEO — https://github.com/cxcscmu/AutoGEO (MIT), bài báo https://arxiv.org/abs/2510.11438
- geo-seo-claude, skill `geo-citability` — https://github.com/zubair-trabzada/geo-seo-claude (MIT)
- marketingskills, skill `ai-seo` — https://github.com/coreyhaines31/marketingskills (MIT)
- claude-seo, skill `seo-agentic` (robots.txt cho bot AI, llms.txt, bản Markdown) — https://github.com/AgriciDaniel/claude-seo (MIT)
- Chuẩn llms.txt — https://github.com/AnswerDotAI/llms-txt
