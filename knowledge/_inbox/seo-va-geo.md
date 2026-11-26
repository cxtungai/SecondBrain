---
title: SEO và GEO
type: topic
summary: Bản đồ về SEO và GEO — cách để Google, Bing và trợ lý AI tìm thấy, hiểu đúng và trích dẫn một trang tri thức, kèm quy trình và các khái niệm nền.
tags: [seo, geo, website]
status: draft
aliases: [SEO, Tối ưu công cụ tìm kiếm, Search Engine Optimization, Tối ưu tìm kiếm và AI]
sources:
  - title: "Optimizing your website for generative AI features on Google Search"
    url: https://developers.google.com/search/docs/fundamentals/ai-optimization-guide
    publisher: Google Search Central
    date: 2026
    kind: official
    credibility: 5
    primary: true
    credibility_note: Hướng dẫn chính thức của Google cho AI Overviews và AI Mode; đọc toàn văn.
    accessed: 2026-11-26
  - title: "AI features and your website"
    url: https://developers.google.com/search/docs/appearance/ai-features
    publisher: Google Search Central
    kind: official
    credibility: 5
    primary: true
    credibility_note: Tài liệu chính thức về cách AI Overviews và AI Mode chọn link; đọc toàn văn.
    accessed: 2026-11-26
  - title: "GEO: Generative Engine Optimization"
    url: https://arxiv.org/abs/2311.09735
    author: Pranjal Aggarwal và cộng sự
    publisher: KDD 2024
    date: 2024
    kind: paper
    credibility: 5
    primary: true
    credibility_note: Bài gốc đặt ra khái niệm GEO, đã bình duyệt; đọc toàn văn.
    accessed: 2026-11-26
created: 2026-11-26
updated: 2026-11-26
research:
  keyword: "SEO và GEO cho trang tri thức"
  date: 2026-11-26
  agent: claude
---

**SEO** (Search Engine Optimization) là làm cho công cụ tìm kiếm như Google, Bing **tìm thấy, hiểu đúng và xếp hạng** trang. **GEO** (Generative Engine Optimization) là làm cho **trợ lý AI** (ChatGPT, Perplexity, Claude, Gemini, Google AI Overviews, Copilot) chọn trang làm nguồn và trích đúng khi trả lời (Nguồn [3]). Hai việc này chồng lên nhau: AI của Google lấy nguồn từ chính chỉ mục tìm kiếm, và Google xem tối ưu cho AI của mình "vẫn là SEO" (Nguồn [1]).

## Tổng quan

- **Công cụ tìm kiếm cũ** trả về danh sách link.
- **Công cụ tạo sinh** đọc nhiều trang rồi viết câu trả lời kèm vài nguồn. Bài GEO gốc đo "được nhìn thấy" bằng việc được trích và chiếm bao nhiêu phần trong câu trả lời (Nguồn [3]).
- **Google AI Overviews và AI Mode** có thể tách câu hỏi thành nhiều truy vấn con ("query fan-out") để tìm thêm nguồn, nên hiện được nhiều link đa dạng hơn tìm kiếm thường (Nguồn [2]).
- **Điều kiện để AI của Google trích:** trang đã được lập chỉ mục và đủ điều kiện hiện đoạn trích. Không có yêu cầu kỹ thuật riêng nào khác (Nguồn [2]).

Với một trang tri thức, SEO/GEO có thể chia thành 5 lớp:

| Lớp | Câu hỏi | Ghi chú liên quan |
| --- | --- | --- |
| Nền kỹ thuật | Máy có đọc được, hiểu đúng không? | [[du-lieu-co-cau-truc]], [[llms-txt]] |
| Được biết đến | Công cụ đã biết web tồn tại chưa? | [[indexnow]] |
| Nội dung | Bài có đáng được chọn và trích không? | [[generative-engine-optimization]] |
| Uy tín | Nơi khác có nhắc tới, dẫn về không? | [[quy-trinh-seo-geo-cho-trang-tri-thuc]] |
| Đo lường | Điều gì đang hiệu quả? | [[quy-trinh-seo-geo-cho-trang-tri-thuc]] |

Google nói **nội dung độc đáo, hữu ích, không đại trà** ảnh hưởng lâu dài tới việc xuất hiện trong AI nhiều hơn mọi gợi ý kỹ thuật khác (Nguồn [1]).

## Khái niệm

- [[generative-engine-optimization|GEO (Generative Engine Optimization)]]: định nghĩa, bằng chứng từ nghiên cứu, giới hạn và phản biện.
- [[du-lieu-co-cau-truc|Dữ liệu có cấu trúc]]: JSON-LD theo schema.org, các kiểu dùng cho trang tri thức.
- [[llms-txt|llms.txt]]: file tóm tắt cho AI agent; Google Search không dùng.
- [[indexnow|IndexNow]]: báo ngay cho Bing và các công cụ khác khi có trang mới.

## Mô hình & phương pháp

- [[quy-trinh-seo-geo-cho-trang-tri-thuc]]: 7 bước từ nền kỹ thuật tới đo lường.

## Câu hỏi còn mở

- [[sitemap-xml]], [[robots-txt]]: chưa có ghi chú riêng.
- Chưa có nghiên cứu GEO cho nội dung tiếng Việt.

## Câu hỏi thường gặp

### SEO còn cần khi đã có AI tìm kiếm không?

Còn. AI của Google lấy nguồn từ chỉ mục tìm kiếm, nên trang chưa được lập chỉ mục thì không thể được trích (Nguồn [1][2]).

### GEO có phải là SEO kiểu mới không?

Phần lớn là vậy. Google coi tối ưu cho AI của mình vẫn là SEO (Nguồn [1]). GEO nhấn mạnh thêm việc **được trích đúng**: dẫn nguồn, câu trích, chi tiết cụ thể (Nguồn [3]).

### Trang tri thức nên bắt đầu từ đâu?

Từ nền kỹ thuật và đăng ký Search Console, rồi dồn sức vào nội dung đã kiểm chứng. Xem [[quy-trinh-seo-geo-cho-trang-tri-thuc]].

## Liên quan

- [[generative-engine-optimization]]
- [[quy-trinh-seo-geo-cho-trang-tri-thuc]]
- [[du-lieu-co-cau-truc]], [[llms-txt]], [[indexnow]]
