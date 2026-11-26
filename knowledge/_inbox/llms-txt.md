---
title: llms.txt
type: concept
summary: llms.txt là file Markdown đặt ở web, tóm tắt và chỉ đường tới nội dung cho AI agent đọc; là đề xuất, không phải chuẩn, và Google Search không dùng.
tags: [seo, geo, website]
status: draft
aliases: [llms-txt, llms-full.txt, LLMs.txt]
sources:
  - title: "The /llms.txt file, v2"
    url: https://llmstxt.org/
    author: Jeremy Howard
    publisher: Answer.AI
    date: 2024-09-03 (bản 2 sửa 2026-08-10)
    kind: official
    credibility: 5
    primary: true
    credibility_note: Văn bản đề xuất gốc của người đặt ra llms.txt; đọc toàn văn. Là đề xuất, chưa phải chuẩn chính thức.
    accessed: 2026-11-26
  - title: "Optimizing your website for generative AI features on Google Search"
    url: https://developers.google.com/search/docs/fundamentals/ai-optimization-guide
    publisher: Google Search Central
    date: 2026
    kind: official
    credibility: 5
    primary: true
    credibility_note: Hướng dẫn chính thức của Google, mục "Mythbusting" nói rõ Google Search không dùng llms.txt; đọc toàn văn.
    accessed: 2026-11-26
created: 2026-11-26
updated: 2026-11-26
research:
  keyword: "SEO và GEO cho trang tri thức"
  date: 2026-11-26
  agent: claude
---

**llms.txt** là một file văn bản dạng Markdown đặt trên web, tóm tắt web có gì và chỉ đường tới các trang quan trọng, để **AI agent** (trợ lý AI, công cụ lập trình bằng AI) đọc nhanh mà không phải lọc qua menu, quảng cáo hay mã JavaScript. Jeremy Howard (Answer.AI) đưa ra đề xuất này ngày 3/9/2024; bản 2 sửa ngày 10/8/2026 (Nguồn [1]). llms.txt **là đề xuất, không phải chuẩn chính thức**. **Google Search không dùng** file này, kể cả cho AI Overviews và AI Mode (Nguồn [2]).

## Định nghĩa

Theo đề xuất, file `llms.txt` đặt ở gốc web (`/llms.txt`) hoặc ở một thư mục con (vd. `/docs/llms.txt`), và áp dụng cho các trang dưới thư mục đó (Nguồn [1]). Nội dung theo thứ tự:

1. Một tiêu đề cấp 1 (`# Tên web`): **phần bắt buộc duy nhất**.
2. Một đoạn trích dẫn (`> …`) tóm tắt web.
3. Các đoạn giải thích (không có tiêu đề).
4. Các mục tiêu đề cấp 2 (`## …`), mỗi mục là danh sách link `[tên](url): ghi chú`.
5. Mục `## Optional` theo quy ước: các link AI agent có thể bỏ qua khi cần ngắn gọn.

(Nguồn [1])

Đề xuất còn khuyên mỗi trang có **bản Markdown sạch** ở cùng địa chỉ thêm đuôi `.md`, và gắn trên trang thẻ `<link rel="alternate" type="text/markdown">` trỏ tới bản đó (Nguồn [1]).

## Vì sao có llms.txt

- **Trang web làm cho người đọc:** thông tin bị bọc trong menu, quảng cáo, JavaScript; chuyển lại thành chữ sạch vừa khó vừa thiếu chính xác.
- **Cửa sổ ngữ cảnh** của mô hình AI vẫn quá nhỏ để chứa cả web.
- **Sitemap** liệt kê mọi trang cho công cụ tìm kiếm, nhưng quá dài, không có bản Markdown, không có link ra ngoài.

llms.txt là **bản tóm tắt có chọn lọc** cho AI (Nguồn [1]).

Theo tác giả, sau hai năm đã có hàng nghìn web dùng llms.txt. Các nền tảng làm tài liệu tự sinh file này, và công cụ Lighthouse của Chrome có kiểm tra llms.txt trong nhóm kiểm tra "agentic browsing". File được dùng nhiều nhất cho **tài liệu phần mềm**, nơi công cụ lập trình bằng AI tra cứu API (Nguồn [1]).

## Google nói gì

Hướng dẫn chính thức của Google về AI trong tìm kiếm xếp llms.txt vào nhóm "không cần làm":

- Không cần tạo file máy đọc mới, file văn bản cho AI, markup hay bản Markdown để xuất hiện trên Google Search, kể cả trong các tính năng AI, **vì Google Search không dùng chúng**.
- Tạo và giữ llms.txt cho dịch vụ khác là **hoàn toàn được**: "không hại cũng không giúp" thứ hạng trên Google.

(Nguồn [2])

## Ví dụ

Trang tri thức Second Brain tự sinh `llms.txt`:

- tiêu đề là tên web;
- đoạn tóm tắt là mô tả web;
- quy tắc dùng tri thức cho AI viết content;
- các mục theo loại ghi chú (Chủ đề, Khái niệm, Mô hình & phương pháp, Nguồn tham khảo), mỗi mục trỏ tới bản `.md` của ghi chú;
- mục `## Optional` trỏ tới `llms-full.txt` (toàn bộ nội dung).

File nằm ở thư mục con của web (`/SecondBrain/llms.txt`), đúng với đề xuất cho phép (Nguồn [1]).

## Hiểu sai thường gặp

- **"Có llms.txt thì lên AI Overviews."** Sai với Google: Google Search bỏ qua file này (Nguồn [2]).
- **"llms.txt thay cho robots.txt hoặc sitemap."** Không: robots.txt cho bot biết được đọc gì, sitemap liệt kê mọi trang, còn llms.txt là bản tóm tắt chọn lọc cho AI. Ba file làm ba việc khác nhau (Nguồn [1]).
- **"llms.txt là chuẩn chính thức."** Hiện vẫn là đề xuất của một tác giả, được nhiều nơi dùng, chưa có tổ chức tiêu chuẩn nào phê duyệt (Nguồn [1]).

## Câu hỏi còn mở

- Ngoài Lighthouse, chưa có công bố chính thức nào của OpenAI, Anthropic hay Perplexity về việc trợ lý tìm kiếm của họ đọc llms.txt.

## Câu hỏi thường gặp

### Trang tri thức có nên có llms.txt không?

Nên, nếu muốn AI agent (công cụ viết content, trợ lý lập trình…) đọc kho tri thức dễ hơn. Việc này không hại thứ hạng trên Google (Nguồn [2]). Nhưng đừng kỳ vọng nó giúp lên Google hay AI Overviews.

### Đặt llms.txt ở thư mục con có được không?

Được. Đề xuất cho phép đặt ở gốc web hoặc ở thư mục con, và file áp dụng cho các trang dưới thư mục đó. Đây là cách cho web không làm chủ gốc tên miền, như trang GitHub Pages của một dự án (Nguồn [1]).

### llms.txt khác llms-full.txt thế nào?

llms.txt là mục lục ngắn có link. llms-full.txt (cách nhiều web làm thêm) gộp toàn bộ nội dung vào một file cho AI có ngữ cảnh lớn. Đề xuất gốc dùng mục `## Optional` cho các link phụ như vậy (Nguồn [1]).

## Liên quan

- [[seo-va-geo]]
- [[generative-engine-optimization]]
- [[quy-trinh-seo-geo-cho-trang-tri-thuc]]
