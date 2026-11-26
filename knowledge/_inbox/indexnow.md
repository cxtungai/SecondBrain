---
title: IndexNow
type: concept
summary: IndexNow là giao thức để web báo ngay cho Bing, Yandex, Naver, Seznam, Yep khi trang được thêm, sửa hay xoá; Google không tham gia mà tự đọc sitemap.
tags: [seo, website]
status: draft
aliases: [Index Now, giao thức IndexNow]
sources:
  - title: "IndexNow — Documentation"
    url: https://www.indexnow.org/documentation
    publisher: IndexNow.org
    date: 2026
    kind: official
    credibility: 5
    primary: true
    credibility_note: Tài liệu gốc của giao thức; đọc toàn văn (gửi một URL, gửi danh sách, xác minh khoá, mã phản hồi).
    accessed: 2026-11-26
  - title: "IndexNow — FAQ và danh sách công cụ tham gia"
    url: https://www.indexnow.org/faq
    publisher: IndexNow.org
    date: 2026
    kind: official
    credibility: 5
    primary: true
    credibility_note: Trang chính thức; đọc phần danh sách công cụ tham gia và yêu cầu chia sẻ URL giữa các công cụ (kèm trang searchengines).
    accessed: 2026-11-26
  - title: "Indexing API Quickstart"
    url: https://developers.google.com/search/apis/indexing-api/v3/quickstart
    publisher: Google Search Central
    kind: official
    credibility: 5
    primary: true
    credibility_note: Tài liệu chính thức về giới hạn của Indexing API của Google; đọc toàn văn.
    accessed: 2026-11-26
  - title: "Sitemaps ping endpoint is going away"
    url: https://developers.google.com/search/blog/2023/06/sitemaps-lastmod-ping
    publisher: Google Search Central Blog
    date: 2023-06
    kind: official
    credibility: 5
    primary: true
    credibility_note: Thông báo chính thức Google bỏ cách "ping" sitemap và cách Google dùng lastmod; đọc toàn văn.
    accessed: 2026-11-26
  - title: "Build and submit a sitemap"
    url: https://developers.google.com/search/docs/crawling-indexing/sitemaps/build-sitemap
    publisher: Google Search Central
    kind: official
    credibility: 5
    primary: true
    credibility_note: Tài liệu chính thức về sitemap và trường lastmod; đọc phần lastmod.
    accessed: 2026-11-26
created: 2026-11-26
updated: 2026-11-26
research:
  keyword: "SEO và GEO cho trang tri thức"
  date: 2026-11-26
  agent: claude
---

**IndexNow** là giao thức mở cho phép web **báo ngay** cho công cụ tìm kiếm khi một trang được **thêm, sửa hoặc xoá**, thay vì chờ bot tự ghé lại (Nguồn [1]). Các công cụ đang hỗ trợ gồm **Microsoft Bing, Naver, Seznam.cz, Yandex, Yep**. Công cụ nào nhận được thông báo cũng phải chia sẻ cho các công cụ tham gia khác (Nguồn [2]). **Google không có trong danh sách**. Google tự đọc lại [[sitemap-xml|sitemap]], và chỉ cho đẩy trang trực tiếp qua Indexing API với tin tuyển dụng hoặc livestream (Nguồn [2][3]).

## Cách hoạt động

1. **Tạo khoá:** 8–128 ký tự, chỉ gồm chữ thường, chữ hoa, số và dấu "-" (Nguồn [1]).
2. **Đặt file khoá trên web** để chứng minh mình là chủ web. Công cụ tìm kiếm sẽ tải file này về kiểm tra. Có hai cách (Nguồn [1]):
   - đặt `{khoá}.txt` ở **gốc web**, nội dung là chính khoá. Đây là cách được khuyên dùng;
   - đặt ở **thư mục khác** và khai báo `keyLocation` trong mỗi lần gửi. Khi đó **chỉ báo được các trang nằm dưới thư mục đó**. Ví dụ file ở `/catalog/` thì không báo được trang ở `/help/`.
3. **Gửi thông báo:**
   - một trang: gọi địa chỉ `…/indexnow?url=<trang>&key=<khoá>`;
   - nhiều trang: gửi một yêu cầu POST dạng JSON gồm `host`, `key`, `keyLocation`, `urlList`, tối đa **10.000 URL mỗi lần** (Nguồn [1]).
4. **Đọc mã trả về** (Nguồn [1]):

| Mã | Nghĩa |
| --- | --- |
| 200 | Đã nhận |
| 202 | Đã nhận, đang xác minh khoá (thường gặp ở lần đầu) |
| 400 | Sai định dạng |
| 403 | Khoá không hợp lệ (không thấy file khoá, hoặc file không chứa khoá) |
| 422 | URL không thuộc web, hoặc khoá sai quy cách |
| 429 | Gửi quá nhiều, có thể bị coi là spam |

Tài liệu khuyên **tự động hoá**: gửi ngay khi nội dung được thêm, sửa hoặc xoá (Nguồn [1]).

## Vì sao Google không cần IndexNow

- Google **đã bỏ cách "ping" sitemap** từ năm 2023: gọi tới địa chỉ ping cũ nay trả lỗi 404 (Nguồn [4]).
- Google dựa vào trường `<lastmod>` trong sitemap, **với điều kiện** ngày đó đúng một cách nhất quán và kiểm chứng được (vd. khớp với lần sửa thật của trang) (Nguồn [5]).
- **Indexing API** của Google chỉ dùng được cho trang có `JobPosting` (tin tuyển dụng) hoặc `BroadcastEvent` trong `VideoObject` (livestream) (Nguồn [3]).

Với web thường, cách báo Google là **sitemap có ngày sửa đúng**.

## Ví dụ

Trang tri thức Second Brain dùng IndexNow trong quy trình đăng bài trên GitHub:

1. Trước khi đăng, so sitemap đang chạy với sitemap mới, lấy ra các trang mới, vừa sửa hoặc vừa bỏ.
2. Đăng web, kiểm tra web thật.
3. Gửi danh sách cho IndexNow.

Web nằm trong thư mục con (`/SecondBrain/`) nên file khoá đặt ngay trong thư mục đó, kèm `keyLocation`. Lần gửi đầu cả 22 trang nhận mã 202 (đang xác minh khoá).

## Hiểu sai thường gặp

- **"IndexNow giúp lên Google nhanh hơn."** Google không tham gia IndexNow (Nguồn [2]).
- **"Gửi càng nhiều càng tốt."** Chỉ gửi trang thật sự mới, sửa hoặc xoá. Gửi dồn dập có thể nhận mã 429 và bị coi là spam (Nguồn [1]).
- **"Mã 200 nghĩa là trang đã được lập chỉ mục."** Mã 200 chỉ nghĩa là công cụ đã nhận thông báo; việc có lập chỉ mục hay không là chuyện khác (Nguồn [1]).
- **"Phải đăng ký tài khoản Bing mới dùng được."** Không cần: chỉ cần file khoá trên web (Nguồn [1]).

## Câu hỏi còn mở

- Chưa có số liệu công khai về việc IndexNow rút ngắn thời gian lập chỉ mục bao nhiêu với web nhỏ.

## Câu hỏi thường gặp

### IndexNow là gì, dùng để làm gì?

IndexNow là giao thức để web báo ngay cho Bing, Yandex, Naver, Seznam, Yep khi có trang mới, sửa hoặc xoá, để các công cụ này cập nhật sớm thay vì chờ bot ghé lại (Nguồn [1][2]).

### Google có dùng IndexNow không?

Không. Google không có trong danh sách công cụ tham gia (Nguồn [2]). Với Google, giữ sitemap có `lastmod` đúng là đủ, vì Google đã bỏ cách ping sitemap từ 2023 (Nguồn [4][5]).

### Web nằm trong thư mục con có dùng được IndexNow không?

Được. Đặt file khoá trong thư mục đó và khai báo `keyLocation`, khi đó chỉ báo được các trang dưới thư mục ấy. Tài liệu vẫn khuyên đặt file ở gốc web khi có thể (Nguồn [1]).

### Nhận mã 202 có phải lỗi không?

Không. 202 nghĩa là đã nhận, đang chờ xác minh khoá, thường gặp ở lần gửi đầu (Nguồn [1]).

## Liên quan

- [[seo-va-geo]]
- [[quy-trinh-seo-geo-cho-trang-tri-thuc]]
- [[generative-engine-optimization]]
