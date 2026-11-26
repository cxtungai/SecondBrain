---
title: Dữ liệu có cấu trúc (structured data)
type: concept
summary: Dữ liệu có cấu trúc là mã chuẩn schema.org (thường là JSON-LD) mô tả trang cho máy hiểu; giúp có kết quả nổi bật, không bắt buộc cho AI của Google.
tags: [seo, geo, website]
status: draft
aliases: [Structured data, JSON-LD, Schema markup, schema.org, Dữ liệu cấu trúc]
sources:
  - title: "Introduction to structured data markup in Google Search"
    url: https://developers.google.com/search/docs/appearance/structured-data/intro-structured-data
    publisher: Google Search Central
    kind: official
    credibility: 5
    primary: true
    credibility_note: Tài liệu chính thức của Google; đọc toàn văn (định nghĩa, định dạng, khuyên dùng JSON-LD, nguyên tắc nội dung hiển thị).
    accessed: 2026-11-26
  - title: "Article (Article, NewsArticle, BlogPosting) structured data"
    url: https://developers.google.com/search/docs/appearance/structured-data/article
    publisher: Google Search Central
    kind: official
    credibility: 5
    primary: true
    credibility_note: Tài liệu chính thức cho kiểu Article; đọc phần thuộc tính.
    accessed: 2026-11-26
  - title: "Changes to HowTo and FAQ rich results"
    url: https://developers.google.com/search/blog/2023/08/howto-faq-changes
    publisher: Google Search Central Blog
    date: 2023-08
    kind: official
    credibility: 5
    primary: true
    credibility_note: Thông báo chính thức giới hạn kết quả FAQ; đọc toàn văn.
    accessed: 2026-11-26
  - title: "Optimizing your website for generative AI features on Google Search"
    url: https://developers.google.com/search/docs/fundamentals/ai-optimization-guide
    publisher: Google Search Central
    date: 2026
    kind: official
    credibility: 5
    primary: true
    credibility_note: Hướng dẫn chính thức về AI trong Google Search; đọc toàn văn, có phần nói về dữ liệu có cấu trúc.
    accessed: 2026-11-26
created: 2026-11-26
updated: 2026-11-26
research:
  keyword: "SEO và GEO cho trang tri thức"
  date: 2026-11-26
  agent: claude
---

**Dữ liệu có cấu trúc** (structured data) là đoạn mã theo **định dạng chuẩn** gắn trong trang, mô tả trang và phân loại nội dung. Ví dụ: đây là bài viết, tác giả là ai, đăng ngày nào. Nhờ đó Google hiểu trang rõ hơn, và trang có thể hiện thành **kết quả nổi bật** (rich results) (Nguồn [1]). Google khuyên dùng định dạng **JSON-LD**, với bộ từ vựng chủ yếu của **schema.org** (Nguồn [1]). Dữ liệu có cấu trúc **không bắt buộc** để xuất hiện trong AI Overviews hay AI Mode, nhưng vẫn nên dùng như một phần của SEO (Nguồn [4]).

## Cách hoạt động

- **Ba định dạng** Google đọc được: JSON-LD, Microdata, RDFa. Cả ba đều được chấp nhận nếu đúng cú pháp, nhưng Google **khuyên dùng JSON-LD**: một khối `<script type="application/ld+json">` tách riêng khỏi phần chữ của trang (Nguồn [1]).
- **Từ vựng:** phần lớn theo schema.org. Riêng về cách Google xử lý thì tài liệu Google Search Central mới là căn cứ, không phải schema.org (Nguồn [1]).
- **Nguyên tắc quan trọng nhất:** không đánh dấu thông tin **mà người đọc không thấy trên trang**, kể cả khi thông tin đó đúng (Nguồn [1]). Hướng dẫn về AI của Google cũng nhắc dữ liệu có cấu trúc phải **khớp với chữ hiện trên trang** (Nguồn [4]).
- **Ít mà đủ hơn nhiều mà sai:** nên điền ít thuộc tính nhưng đầy đủ và chính xác, thay vì cố điền mọi thuộc tính khuyên dùng mà dữ liệu thiếu hoặc sai (Nguồn [1]).

## Các kiểu dùng cho trang tri thức

| Kiểu (schema.org) | Gắn ở đâu | Ghi chú |
| --- | --- | --- |
| `WebSite` + `Organization` | Trang chủ | Tên web, địa chỉ, đơn vị xuất bản, logo |
| `Article` | Từng bài / ghi chú | **Không có thuộc tính bắt buộc**; nên có `headline`, `image`, `datePublished`, `dateModified`, `author` (Nguồn [2]) |
| `BreadcrumbList` | Trang con | Đường dẫn "Trang chủ › Chủ đề › Bài" |
| `FAQPage` | Mục câu hỏi thường gặp | Từ 08/2023, Google chỉ hiện kết quả FAQ đặc biệt cho **web nhà nước và y tế có uy tín** (Nguồn [3]) |
| `DefinedTerm`, `DefinedTermSet` | Khái niệm, từ điển | Mô tả thuật ngữ và định nghĩa |

Với kiểu `FAQPage`, trang thường không còn được hiện nổi bật trên Google (Nguồn [3]), nhưng phần hỏi đáp vẫn là nội dung có cấu trúc rõ cho người đọc và máy.

## Ví dụ

Một ghi chú tri thức trên Second Brain có khối JSON-LD kiểu `Article`:

- `headline`, `description`;
- `datePublished`, `dateModified` lấy từ ngày tạo và ngày sửa;
- `author`: tên thương hiệu khi chủ web không công khai tên;
- `publisher`: đơn vị xuất bản, có logo;
- `citation`: danh sách nguồn của ghi chú.

Đi kèm là `BreadcrumbList` và `FAQPage`. Mọi thông tin này **đều hiện trên trang** (tiêu đề, ngày cập nhật, mục nguồn), đúng nguyên tắc ở trên.

## Hiểu sai thường gặp

- **"Phải có schema đặc biệt mới được AI của Google trích."** Sai: Google nói không có markup schema.org đặc biệt nào cần thêm cho AI (Nguồn [4]).
- **"Có dữ liệu có cấu trúc là chắc chắn hiện kết quả nổi bật."** Không: dữ liệu chỉ làm trang **đủ điều kiện**. Riêng FAQ thì hầu hết web không còn được hiện nổi bật (Nguồn [1][3]).
- **"Thêm càng nhiều thuộc tính càng tốt."** Google khuyên ít mà đúng và đủ (Nguồn [1]).
- **"Đánh dấu cả thông tin ẩn cho máy đọc."** Vi phạm nguyên tắc: chỉ đánh dấu điều người đọc thấy trên trang (Nguồn [1]).

## Câu hỏi còn mở

- Chưa có công bố chính thức về việc ChatGPT, Perplexity, Claude có đọc JSON-LD khi chọn nguồn hay không.

## Câu hỏi thường gặp

### Dữ liệu có cấu trúc là gì?

Đó là đoạn mã theo định dạng chuẩn (thường là JSON-LD với từ vựng schema.org) gắn trong trang, cho máy biết trang nói về gì, ai viết, khi nào. Nhờ đó trang có thể đủ điều kiện hiện thành kết quả nổi bật trên Google (Nguồn [1]).

### Nên dùng JSON-LD hay Microdata?

Google đọc được cả hai, nhưng khuyên dùng JSON-LD vì tách riêng khỏi phần chữ, dễ viết và dễ bảo trì (Nguồn [1]).

### Có cần dữ liệu có cấu trúc để xuất hiện trong AI Overviews không?

Không bắt buộc, và không có schema đặc biệt nào cho AI. Nhưng Google vẫn khuyên dùng như một phần của SEO, vì nó giúp đủ điều kiện có kết quả nổi bật (Nguồn [4]).

### Mục câu hỏi thường gặp có còn hiện nổi bật trên Google không?

Từ 08/2023, kết quả FAQ nổi bật chỉ hiện cho các web nhà nước và y tế có uy tín (Nguồn [3]).

## Liên quan

- [[seo-va-geo]]
- [[quy-trinh-seo-geo-cho-trang-tri-thuc]]
- [[generative-engine-optimization]]
