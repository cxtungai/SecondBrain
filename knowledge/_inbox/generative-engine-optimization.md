---
title: GEO (Generative Engine Optimization)
type: concept
summary: GEO là tối ưu nội dung web để trợ lý AI chọn và trích dẫn khi trả lời; bằng chứng cho thấy dẫn nguồn, trích dẫn, số liệu giúp ích, nhồi từ khoá thì không.
tags: [seo, geo, website]
status: draft
aliases: [Generative Engine Optimization, Tối ưu cho công cụ tạo sinh, AEO, Answer Engine Optimization, AI SEO]
sources:
  - title: "GEO: Generative Engine Optimization"
    url: https://arxiv.org/abs/2311.09735
    author: Pranjal Aggarwal, Vishvak Murahari, Tanmay Rajpurohit, Ashwin Kalyan, Karthik Narasimhan, Ameet Deshpande
    publisher: KDD 2024 (ACM SIGKDD)
    date: 2024
    kind: paper
    credibility: 5
    primary: true
    credibility_note: Bài gốc đặt ra khái niệm GEO, đã bình duyệt tại KDD 2024; đọc toàn văn bản HTML v3 (phương pháp, bảng 1–3, phần giới hạn).
    accessed: 2026-11-26
  - title: "What Generative Search Engines Like and How to Optimize Web Content Cooperatively (AutoGEO)"
    url: https://arxiv.org/abs/2510.11438
    author: Yujiang Wu, Shanshan Zhong, Yubin Kim, Chenyan Xiong
    publisher: ICLR 2026 (Carnegie Mellon University)
    date: 2025
    kind: paper
    credibility: 4
    credibility_note: Nghiên cứu được nhận đăng ở ICLR 2026; chỉ đọc tóm tắt và bộ quy tắc mặc định trong mã nguồn công khai (autogeo/rewriters/core.py), chưa đọc toàn văn.
    accessed: 2026-11-26
  - title: "Optimizing your website for generative AI features on Google Search"
    url: https://developers.google.com/search/docs/fundamentals/ai-optimization-guide
    publisher: Google Search Central
    date: 2026
    kind: official
    credibility: 5
    primary: true
    credibility_note: Hướng dẫn chính thức của Google cho AI Overviews và AI Mode; đọc toàn văn. Chỉ nói về Google, không về ChatGPT, Perplexity hay Claude.
    accessed: 2026-11-26
  - title: "Optimizing Visibility in Generative Engines: A Critical Survey of Generative Engine Optimization (2023–2026)"
    url: https://arxiv.org/abs/2607.14035
    author: Olivier Martinez
    publisher: arXiv (bản thảo)
    date: 2026
    kind: paper
    credibility: 3
    credibility_note: Tổng quan phản biện 45 nghiên cứu, một tác giả, chưa bình duyệt; chỉ đọc tóm tắt.
    accessed: 2026-11-26
  - title: "Counter-GEO-Bench: Evaluating Defenses Against Information-Distorting Generative Engine Optimization"
    url: https://arxiv.org/abs/2609.02316
    author: Bing Zheng, Zongyao Zhao, Wenming Yang
    publisher: arXiv (bản thảo)
    date: 2026
    kind: paper
    credibility: 3
    credibility_note: Bản thảo chưa bình duyệt về việc lạm dụng GEO để lan tin sai; chỉ đọc tóm tắt.
    accessed: 2026-11-26
  - title: "Introducing AI Performance in Bing Webmaster Tools Public Preview"
    url: https://blogs.bing.com/webmaster/February-2026/Introducing-AI-Performance-in-Bing-Webmaster-Tools-Public-Preview
    publisher: Microsoft Bing
    date: 2026-02
    kind: official
    credibility: 5
    primary: true
    credibility_note: Thông báo chính thức của Bing về báo cáo trích dẫn trong câu trả lời AI; đọc toàn văn.
    accessed: 2026-11-26
created: 2026-11-26
updated: 2026-11-26
research:
  keyword: "SEO và GEO cho trang tri thức"
  date: 2026-11-26
  agent: claude
---

**GEO (Generative Engine Optimization)** là việc tối ưu nội dung web để các **công cụ trả lời bằng AI** chọn trang của mình làm nguồn và trích đúng khi trả lời. Các công cụ này gồm ChatGPT, Perplexity, Claude, Gemini, Google AI Overviews, Copilot. Khái niệm được đặt ra trong bài nghiên cứu "GEO: Generative Engine Optimization" (Aggarwal và cộng sự, KDD 2024) (Nguồn [1]). Bằng chứng tới nay cho thấy: **dẫn nguồn, trích dẫn và số liệu cụ thể** giúp nội dung được AI nhắc tới nhiều hơn, còn **nhồi từ khoá kiểu SEO cũ gần như vô ích**. Mức tăng chỉ đo được khi trang **đã nằm trong số nguồn AI lấy về** (Nguồn [1][4]).

## Định nghĩa

GEO là phần mở rộng của [[seo-va-geo|SEO]] cho kiểu tìm kiếm mới.

- Công cụ tìm kiếm cũ trả về **danh sách link**.
- **Công cụ tạo sinh** (generative engine) đọc nhiều trang rồi **viết một câu trả lời**, kèm vài nguồn được trích.

Theo bài gốc, công cụ tạo sinh làm hai bước: lấy các nguồn liên quan tới câu hỏi, rồi để mô hình ngôn ngữ viết câu trả lời dựa trên các nguồn đó. Vì vậy, "được nhìn thấy" nghĩa là **được trích và chiếm bao nhiêu phần** trong câu trả lời, không còn là đứng thứ mấy (Nguồn [1]).

Hai tên khác hay gặp là **AEO** (Answer Engine Optimization) và **AI SEO**. Google xem việc tối ưu cho AI của Google "vẫn là SEO" (Nguồn [3]).

## Cách GEO hoạt động (theo nghiên cứu)

Bài GEO gốc thử 9 cách viết lại nội dung trên **GEO-bench**. Đây là bộ 10.000 câu hỏi thuộc 25 lĩnh vực, khoảng 80% là câu hỏi tìm thông tin. Các cách được thử: viết giọng uy tín, thêm số liệu, nhồi từ khoá, dẫn nguồn, thêm câu trích, viết dễ hiểu, viết trôi chảy, thêm từ hiếm, thêm thuật ngữ (Nguồn [1]).

| Cách viết lại | Kết quả trong bài gốc |
| --- | --- |
| **Dẫn nguồn** (Cite Sources), **thêm câu trích** (Quotation Addition), **thêm số liệu** (Statistics Addition) | Hiệu quả nhất, tăng hiển thị tới khoảng 40%, chỉ cần sửa ít |
| Viết trôi chảy, viết dễ hiểu | Tăng khoảng 15–30% |
| **Nhồi từ khoá** (Keyword Stuffing) | "Gần như không cải thiện" |
| Thử trên Perplexity.ai (công cụ thật) | Thêm câu trích tăng 22%, các cách khác tới 9% và 37% tuỳ thước đo |

(Nguồn [1])

**Lợi nhiều nhất cho trang xếp thấp.** Với trang đứng thứ 5 trong kết quả tìm kiếm, dẫn nguồn làm hiển thị tăng 115,1%, thêm câu trích tăng 99,7%. Với trang đứng thứ 1, các cách này lại làm hiển thị **giảm** khoảng 20–30% (Nguồn [1]).

**Mỗi lĩnh vực hợp một cách.** Dẫn nguồn hợp câu hỏi về sự thật. Câu trích hợp các mảng con người, giải thích, lịch sử. Số liệu hợp mảng luật và chính sách (Nguồn [1]).

**AutoGEO** (ICLR 2026) đi xa hơn: để chính các mô hình AI giải thích vì sao chúng chọn nguồn, rồi rút thành quy tắc (Nguồn [2]). Bộ quy tắc với Gemini, GPT, Claude có nhiều điểm chung:

1. Nêu kết luận ngay đầu bài.
2. Mỗi đoạn hay mỗi mục một ý, tự đứng được, không cần đọc chỗ khác mới hiểu.
3. Chính xác, nhất quán, cập nhật; ghi ngày với chủ đề hay thay đổi.
4. Giải thích "vì sao" và "như thế nào", không chỉ liệt kê sự kiện.
5. Định nghĩa thuật ngữ ngay lần đầu; phân biệt rõ các khái niệm dễ nhầm.
6. Cân bằng nhiều góc nhìn với chủ đề còn tranh cãi; giọng trung lập, không quảng cáo.
7. Dùng chi tiết cụ thể (tên, ngày, số liệu, ví dụ) thay cho nhận định chung chung.
8. Tiêu đề, danh sách, bảng rõ ràng; viết gọn, không lan man.

Các quy tắc này trùng với tiêu chuẩn của một ghi chú tri thức tốt. Với một trang tri thức, GEO trước hết là **làm tri thức rõ và đáng tin hơn**, không phải viết khác đi.

## Ví dụ

Đoạn khó được trích: "Nó là một cách rất hay mà nhiều người dùng, giúp bạn thắng nhiều hơn."

- Đoạn này mở bằng "Nó" nên trích riêng không biết đang nói gì.
- Không có con số hay nguồn nào.

Đoạn dễ được trích: "[[price-action|Price action]] là phương pháp đọc và giao dịch dựa trên chính diễn biến giá…", kèm số thứ tự nguồn ngay sau câu.

- Gọi đúng tên ngay câu đầu, có định nghĩa và dẫn nguồn.
- Ý không đổi, chỉ đổi cách trình bày.

Cách tự kiểm từng bước nằm ở [[quy-trinh-seo-geo-cho-trang-tri-thuc]].

## Giới hạn và phản biện

- **Mức tăng là trong điều kiện thí nghiệm.** Một bài tổng quan 45 nghiên cứu GEO (Martinez, 2026, bản thảo chưa bình duyệt) cho rằng con số "tới 40%" chỉ đúng khi trang **đã nằm trong số nguồn được đưa cho AI**. Nó không chứng minh trang sẽ được tìm thấy, cũng không chứng minh có thêm lượt truy cập lâu dài (Nguồn [4]).
- **Mẹo chung khó áp sang nơi khác.** Cũng theo bài tổng quan, mức liên quan tới chủ đề và vị trí trong ngữ cảnh là hai yếu tố lặp lại được rõ nhất. Mẹo chung ít khi đúng ở nơi khác, cạnh tranh có thể triệt tiêu lợi ích, và viết lại để được trích đôi khi làm trang **khó được lấy về hơn**. Kết quả AI còn thay đổi giữa các lần hỏi (Nguồn [4]).
- **Mỗi công cụ, mỗi lĩnh vực khác nhau.** Bài GEO gốc tự nhận phương pháp có thể phải đổi khi công cụ AI thay đổi, và không đánh giá ảnh hưởng tới thứ hạng tìm kiếm thường (Nguồn [1]). AutoGEO cũng lưu ý phải rút lại quy tắc khi đổi công cụ hay lĩnh vực (Nguồn [2]).
- **Có thể bị lạm dụng để lan tin sai.** Kỹ thuật GEO có thể khiến AI tổng hợp nội dung sai trông như bình thường. Các bộ lọc an toàn có sẵn chỉ giảm được tối đa 5,7% số lần tấn công thành công (Nguồn [5]).
- **Google không cần "mẹo GEO".** Google nói các kỹ thuật sau không cần cho AI của Google: chia nhỏ bài, viết riêng cho AI, file `llms.txt`, dữ liệu cấu trúc đặc biệt. Tạo "nhắc tới" giả cũng không có ích. Làm hàng loạt trang cho từng biến thể câu hỏi là vi phạm chính sách spam. Điều quan trọng nhất là nội dung độc đáo, không đại trà (Nguồn [3]).

## Hiểu sai thường gặp

- **"GEO thay thế SEO."** Sai. AI của Google lấy nguồn từ chính chỉ mục tìm kiếm: trang phải được lập chỉ mục và hiện được đoạn trích thì mới được trích trong AI Overviews hay AI Mode (Nguồn [3]). Chưa được tìm thấy thì không có gì để tối ưu.
- **"Cứ thêm số liệu và câu trích là được AI trích."** Chỉ đúng khi số liệu và câu trích **có thật, có nguồn**. Thêm số không nguồn làm hỏng tri thức. Bằng chứng cũng chỉ cho thấy lợi ích khi trang đã được lấy về (Nguồn [1][4]).
- **"Có công cụ đo chính xác việc AI trích mình."** Mới có một phần: Bing có báo cáo AI Performance, cho biết số lần web được trích trong Copilot và câu trả lời AI của Bing, nhưng chỉ là dữ liệu mẫu (Nguồn [6]). Google có báo cáo hiệu suất AI tạo sinh trong Search Console (Nguồn [3]).

## Câu hỏi còn mở

- Chưa đọc toàn văn AutoGEO và bài tổng quan 2026, nên chưa có số liệu chi tiết của hai bài này.
- Chưa có nghiên cứu nào cho nội dung **tiếng Việt**, hay cho trang tri thức chuyên ngành tài chính.
- Cách ChatGPT, Perplexity, Claude chọn nguồn chưa được các hãng công bố chi tiết như Google.

## Câu hỏi thường gặp

### GEO khác SEO thế nào?

SEO nhắm vào thứ hạng trong danh sách link của Google, Bing. GEO nhắm vào việc được AI chọn làm nguồn và trích đúng trong câu trả lời. Hai việc chồng lên nhau: AI của Google lấy nguồn từ chính chỉ mục tìm kiếm, nên nền SEO vẫn là điều kiện đầu tiên (Nguồn [3]).

### Cách GEO nào có bằng chứng tốt nhất?

Trong bài gốc, dẫn nguồn, thêm câu trích và thêm số liệu tăng hiển thị tới khoảng 40%; nhồi từ khoá gần như không có tác dụng (Nguồn [1]). Các kết quả này đo trong thí nghiệm với trang đã được đưa vào câu trả lời, chưa phải bằng chứng về lượng truy cập lâu dài (Nguồn [4]).

### Có cần file llms.txt hay dữ liệu cấu trúc đặc biệt cho AI không?

Với Google thì không. Google nói AI của mình không dùng llms.txt và không cần schema đặc biệt (Nguồn [3]). [[llms-txt|llms.txt]] vẫn có ích cho các AI agent khác đọc trang, và [[du-lieu-co-cau-truc|dữ liệu có cấu trúc]] vẫn có ích cho SEO thường.

### Làm sao biết AI có trích trang của mình không?

Xem báo cáo AI Performance trong Bing Webmaster Tools (Copilot và Bing) (Nguồn [6]), báo cáo hiệu suất AI tạo sinh trong Google Search Console (Nguồn [3]), và tự hỏi các AI một bộ câu hỏi cố định mỗi tháng. Kết quả AI thay đổi giữa các lần hỏi, nên phải hỏi nhiều lần trước khi kết luận (Nguồn [4]).

## Liên quan

- [[seo-va-geo]]: bản đồ chủ đề
- [[quy-trinh-seo-geo-cho-trang-tri-thuc]]: áp dụng từng bước cho một trang tri thức
- [[du-lieu-co-cau-truc]], [[llms-txt]], [[indexnow]]
