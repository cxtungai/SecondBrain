---
title: Các loại lệnh giao dịch
type: concept
summary: Lệnh thị trường chắc khớp, không chắc giá; lệnh giới hạn chắc giá, có thể không khớp; lệnh dừng chờ giá chạm mức đặt trước. Sàn HOSE không có lệnh dừng.
tags: [trading]
status: draft
aliases: [Lệnh giao dịch, Order types, Lệnh thị trường, Lệnh giới hạn, Lệnh dừng, Stop order, Stop-loss order, Lệnh LO, Lệnh MTL, Lệnh ATO, Lệnh ATC, Lệnh điều kiện]
sources:
  - title: "Types of Orders"
    url: https://www.investor.gov/introduction-investing/investing-basics/how-stock-markets-work/types-orders
    publisher: U.S. Securities and Exchange Commission (SEC) — Investor.gov
    kind: official
    credibility: 5
    primary: true
    credibility_note: Trang giáo dục nhà đầu tư của cơ quan quản lý chứng khoán Mỹ; định nghĩa chính thức các loại lệnh phổ biến.
    accessed: 2026-11-26
  - title: "Investor Bulletin: Stop, Stop-Limit, and Trailing Stop Orders"
    url: https://www.investor.gov/additional-resources/news-alerts/alerts-bulletins/investor-bulletin-stop-stop-limit-trailing-stop
    publisher: SEC — Office of Investor Education and Assistance
    date: 2017-07-13
    kind: official
    credibility: 5
    primary: true
    credibility_note: Bản tin chính thức của SEC (cập nhật 18/8/2026) về lệnh dừng, dừng giới hạn, dừng kéo theo và rủi ro khi dùng.
    accessed: 2026-11-26
  - title: "Listed Securities Trading Regulations (quy định giao dịch HOSE, HNX)"
    url: https://www.hsc.com.vn/en/1-listed-securities-trading-regulations
    publisher: Công ty Cổ phần Chứng khoán Thành phố Hồ Chí Minh (HSC)
    date: 2025-05-03
    kind: education
    credibility: 3
    credibility_note: Bản tóm tắt quy định giao dịch của Sở theo hệ thống KRX (từ 5/2025) do một công ty chứng khoán lớn đăng; chưa đọc được văn bản gốc của Sở (bản PDF không trích được chữ), nên cần đối chiếu khi Sở đổi quy định.
    accessed: 2026-11-26
  - title: "Feature: Conditional order/Stop order (trang trợ giúp)"
    url: https://help.tcbs.com.vn/en/conditional-order/
    publisher: Công ty Cổ phần Chứng khoán Kỹ Thương (TCBS)
    kind: official
    credibility: 3
    primary: true
    credibility_note: Trang trợ giúp của chính công ty chứng khoán về tính năng của họ; chỉ dùng để cho thấy lệnh điều kiện là tính năng của công ty, nội dung chi tiết nằm trong hình nên chưa đọc được.
    accessed: 2026-11-26
  - title: "How to Trade Price Action Manual (trang giới thiệu)"
    url: https://www.brookstradingcourse.com/how-to-trade-price-action-manual/
    author: Al Brooks
    publisher: Brooks Trading Course
    kind: blog
    credibility: 2
    primary: true
    credibility_note: Trang của chính tác giả, trên trang bán khoá học; dùng cho lời khuyên của Brooks về cách vào lệnh.
    accessed: 2026-11-25
  - title: "Reading Price Charts Bar by Bar — Chapter 1: Price Action (trích đoạn của nhà xuất bản)"
    url: https://media.wiley.com/product_data/excerpt/52/04704439/0470443952.pdf
    author: Al Brooks
    publisher: Wiley
    date: 2009
    kind: book
    credibility: 3
    primary: true
    credibility_note: Tài liệu gốc của tác giả price action hiện đại, đọc toàn văn chương 1; tác giả bán khoá học.
    accessed: 2026-11-25
created: 2026-11-26
updated: 2026-11-26
research:
  keyword: Price action
  date: 2026-11-26
  agent: claude
---

## Định nghĩa

**Lệnh giao dịch** là chỉ thị mua hoặc bán một chứng khoán gửi qua công ty môi giới. Theo SEC (Ủy ban Chứng khoán Mỹ), ba loại lệnh phổ biến nhất là **lệnh thị trường**, **lệnh giới hạn** và **lệnh dừng** (Nguồn [1]). Mỗi loại đổi một thứ lấy một thứ: lệnh thị trường chắc chắn được khớp nhưng không chắc giá, lệnh giới hạn chắc giá nhưng chỉ khớp khi giá chạm mức đã đặt (Nguồn [1]).

## Mục tiêu bài học

Sau bài này bạn sẽ:

- phân biệt được lệnh thị trường, lệnh giới hạn, lệnh dừng và lệnh dừng giới hạn: mỗi loại đảm bảo điều gì và không đảm bảo điều gì;
- chọn được loại lệnh hợp với từng việc: vào lệnh, chốt lời, cắt lỗ;
- biết sàn HOSE có những loại lệnh nào, và vì sao muốn đặt lệnh dừng thì phải dùng tính năng của công ty chứng khoán.

## Giải thích

### Bốn loại lệnh cơ bản (theo SEC)

| Loại lệnh | Cách hoạt động | Đảm bảo | Không đảm bảo |
| --- | --- | --- | --- |
| **Lệnh thị trường** (market order) | Mua hoặc bán ngay, thường khớp quanh giá chào bán (khi mua) hoặc giá chào mua (khi bán) | Được khớp | Giá khớp: giá giao dịch gần nhất chưa chắc là giá bạn được khớp (Nguồn [1]) |
| **Lệnh giới hạn** (limit order) | Mua ở giá đặt hoặc thấp hơn; bán ở giá đặt hoặc cao hơn | Giá khớp không tệ hơn giá đặt | Được khớp: lệnh chỉ khớp nếu giá chạm mức đặt (Nguồn [1]) |
| **Lệnh dừng** (stop order, stop-loss order) | Chờ giá chạm **giá dừng**, khi đó lệnh thành lệnh thị trường | Lệnh được kích hoạt khi giá chạm mức dừng | Giá khớp: có thể lệch xa giá dừng (Nguồn [1][2]) |
| **Lệnh dừng giới hạn** (stop-limit order) | Chờ giá chạm giá dừng, khi đó lệnh thành lệnh giới hạn | Giá khớp không tệ hơn giá giới hạn | Được khớp: giá có thể chạy qua mức giới hạn và lệnh không khớp (Nguồn [2]) |

- **Lệnh dừng mua** đặt **trên** giá thị trường hiện tại; lệnh dừng bán đặt **dưới** giá thị trường hiện tại. Người đang giữ cổ phiếu thường dùng lệnh dừng bán để giới hạn thua lỗ hoặc giữ lãi (Nguồn [1][2]).
- **Giá dừng chỉ là điểm kích hoạt**, không phải giá được khớp. Theo SEC, giá khớp có thể lệch đáng kể so với giá dừng, tuỳ lượng lệnh đối ứng lúc đó. Một biến động ngắn trong phiên cũng có thể kích hoạt lệnh dừng và khớp ở giá tệ hơn nhiều so với giá đóng cửa hôm đó (Nguồn [2]).
- **Lệnh dừng kéo theo** (trailing stop): giá dừng không cố định mà cách giá thị trường một khoảng (theo phần trăm hoặc số tiền). Giá đi theo hướng có lợi thì giá dừng dời theo; giá đi ngược thì giá dừng đứng yên (Nguồn [2]).
- **Không phải công ty nào cũng có đủ các loại lệnh**, và mỗi công ty có thể dùng giá khớp gần nhất hoặc giá chào để xét lệnh dừng đã được kích hoạt chưa. SEC khuyên hỏi rõ công ty môi giới (Nguồn [2]).

### Loại lệnh trên sàn HOSE

Theo bản tóm tắt quy định giao dịch HOSE và HNX của HSC (hệ thống KRX, từ 5/2025), các loại lệnh của Sở gồm (Nguồn [3]):

- **LO** (lệnh giới hạn): mua ở giá đặt hoặc thấp hơn, bán ở giá đặt hoặc cao hơn. Nhập được trong cả phiên khớp lệnh liên tục và định kỳ, có hiệu lực tới hết phiên đóng cửa hoặc tới khi huỷ.
- **Lệnh thị trường**: chỉ nhập được trong phiên khớp lệnh liên tục; không có lệnh đối ứng thì bị huỷ ngay. Gồm **MTL** (phần chưa khớp chuyển thành lệnh LO ở giá khớp gần nhất cộng hoặc trừ một bước giá), **MOK** (không khớp hết ngay thì huỷ cả lệnh) và **MAK** (khớp được bao nhiêu thì khớp, phần còn lại huỷ).
- **ATO / ATC**: lệnh tại phiên xác định giá mở cửa / đóng cửa; phần chưa khớp sau phiên tự huỷ.
- **PLO**: lệnh sau giờ đóng cửa, khớp ở giá khớp cuối cùng của phiên liên tục.
- **Sửa, huỷ lệnh:** trong phiên mở cửa không được sửa, huỷ lệnh LO và ATO; trong phiên đóng cửa không được sửa, huỷ lệnh LO và ATC (Nguồn [3]).

Danh sách lệnh của Sở **không có lệnh dừng**. Một số công ty chứng khoán có tính năng **lệnh điều kiện** hay "stop order" trên ứng dụng của họ, ví dụ TCBS (Nguồn [4]). Vì Sở không nhận lệnh dừng, lệnh điều kiện nằm ở hệ thống của công ty: khi điều kiện xảy ra, công ty mới gửi một lệnh thường lên Sở. Đây là suy ra từ danh sách lệnh của Sở; điều kiện kích hoạt, loại lệnh được gửi đi và việc lệnh có khớp không là do từng công ty quy định, cần đọc hướng dẫn của công ty mình dùng.

### Lệnh dừng trong price action

- **Vào lệnh bằng lệnh dừng.** Brooks khuyên người mới vào lệnh bằng lệnh dừng (Nguồn [5]): lệnh chỉ khớp khi giá đã đi theo hướng mình dự định. Ví dụ của ông: bán ở một bước giá dưới đáy thanh tín hiệu (Nguồn [6]). Mua thì đối xứng: lệnh dừng mua đặt trên đỉnh thanh tín hiệu.
- **Cắt lỗ bằng lệnh dừng.** Lệnh dừng bán dưới giá mua (hoặc lệnh dừng mua trên giá bán khống) giới hạn thua lỗ nếu giá đi ngược (Nguồn [1][2]). Cách chọn mức dừng lỗ có ở [[quy-trinh-giao-dich-price-action]]; khối lượng lệnh có ở [[quan-ly-rui-ro-giao-dich]].

## Ví dụ

Bạn mua 1.000 cổ phiếu ở 30.000 đồng và muốn cắt lỗ nếu giá xuống 28.000 đồng (số giả định).

- **Lệnh dừng bán, giá dừng 28.000:** khi giá chạm 28.000, lệnh thành lệnh thị trường. Nếu giá rơi nhanh, bạn có thể bị khớp ở 27.500 hay thấp hơn (Nguồn [2]).
- **Lệnh dừng giới hạn, giá dừng 28.000, giá giới hạn 27.800:** bạn không bán dưới 27.800. Nhưng nếu giá rơi thẳng xuống 27.000, lệnh không khớp và bạn vẫn giữ cổ phiếu đang giảm (Nguồn [2]).
- **Trên HOSE:** không đặt được lệnh dừng lên Sở. Bạn dùng lệnh điều kiện của công ty chứng khoán (nếu có), hoặc tự theo dõi rồi đặt lệnh LO hay MTL khi giá chạm 28.000 (Nguồn [3][4]).

## Hiểu sai thường gặp

- **"Đặt lệnh dừng ở 28.000 thì chắc chắn bán được ở 28.000."** Giá dừng chỉ kích hoạt lệnh; giá khớp có thể lệch xa (Nguồn [2]).
- **"Lệnh dừng giới hạn an toàn hơn lệnh dừng."** Nó tránh được giá khớp quá tệ nhưng đổi lại có thể không khớp, đúng lúc cần thoát nhất (Nguồn [2]).
- **"Giá giao dịch gần nhất là giá mình sẽ được khớp."** Với lệnh thị trường, điều đó không chắc (Nguồn [1]).
- **"Sàn nào cũng có lệnh dừng."** Danh sách lệnh của HOSE không có; lệnh điều kiện là tính năng riêng của từng công ty (Nguồn [3][4]).

## Bài tập

Tự làm trước, rồi mới mở đáp án. Mọi con số giá là **giả định** để luyện tập.

### Câu 1: Chọn loại lệnh

Chọn loại lệnh cho từng việc: (a) muốn mua ngay, chấp nhận giá thị trường; (b) chỉ muốn mua nếu giá từ 25.000 trở xuống; (c) đang giữ cổ phiếu mua ở 30.000, muốn tự động bán nếu giá xuống 28.000.

**Đáp án:** (a) **Lệnh thị trường**: chắc được khớp, không chắc giá (Nguồn [1]). (b) **Lệnh giới hạn mua ở 25.000**: chỉ khớp ở 25.000 hoặc thấp hơn, và có thể không khớp (Nguồn [1]). (c) **Lệnh dừng bán, giá dừng 28.000**, đặt dưới giá thị trường (Nguồn [1][2]). Trên HOSE, lệnh này không có trong danh sách lệnh của Sở, nên phải dùng lệnh điều kiện của công ty chứng khoán nếu công ty có (Nguồn [3][4]).

### Câu 2: Giá mở cửa nhảy qua mức dừng

Bạn đặt lệnh dừng bán ở 28.000. Sáng hôm sau, giá mở cửa ở 26.500. Lệnh của bạn khớp quanh giá nào? Nếu thay bằng lệnh dừng giới hạn với giá giới hạn 27.800 thì sao?

**Đáp án:** Với **lệnh dừng**, giá 28.000 chỉ là điểm kích hoạt. Lệnh thành lệnh thị trường và khớp quanh giá đang có, tức khoảng 26.500, không phải 28.000 (Nguồn [2]). Với **lệnh dừng giới hạn** 27.800, lệnh chỉ được khớp từ 27.800 trở lên. Giá đang ở 26.500 nên lệnh **không khớp**, và bạn vẫn giữ cổ phiếu nếu giá tiếp tục giảm (Nguồn [2]).

### Câu 3: Vào lệnh theo kiểu Brooks

Thanh tín hiệu tăng có đỉnh 50.300. Bạn muốn chỉ mua khi giá vượt đỉnh thanh này. Nên đặt loại lệnh gì, ở đâu? Vì sao Brooks thích cách này?

**Đáp án:** Đặt **lệnh dừng mua** trên đỉnh thanh tín hiệu một bước giá, đối xứng với ví dụ bán một bước giá dưới đáy thanh của Brooks (Nguồn [6]). Lệnh chỉ khớp khi giá đã đi theo hướng bạn dự định, nên Brooks khuyên người mới vào lệnh theo cách này (Nguồn [5]). Trên HOSE, cần lệnh điều kiện của công ty chứng khoán, hoặc tự theo dõi rồi đặt lệnh khi giá vượt 50.300 (Nguồn [3][4]).

## Câu hỏi còn mở

- Văn bản gốc quy định giao dịch của HOSE (bản PDF trên trang của Sở) chưa trích được chữ; phần HOSE ở trên theo bản tóm tắt của HSC. Biên độ giá, bước giá và lô giao dịch nằm trong bảng chưa đọc được.
- Cách hoạt động cụ thể của lệnh điều kiện ở từng công ty chứng khoán Việt Nam.

## Câu hỏi thường gặp

### Lệnh thị trường và lệnh giới hạn khác nhau thế nào?

Lệnh thị trường chắc chắn được khớp ngay nhưng không chắc giá. Lệnh giới hạn chắc giá khớp không tệ hơn giá đặt nhưng chỉ khớp nếu giá chạm mức đó (Nguồn [1]).

### Lệnh dừng (stop-loss) là gì?

Lệnh dừng là lệnh chờ giá chạm một mức định trước (giá dừng) rồi mới thành lệnh thị trường. Người giữ cổ phiếu dùng lệnh dừng bán đặt dưới giá hiện tại để giới hạn thua lỗ. Giá khớp có thể lệch xa giá dừng (Nguồn [1][2]).

### Sàn HOSE có lệnh dừng không?

Danh sách lệnh của Sở (LO, MTL, MOK, MAK, ATO, ATC, PLO) không có lệnh dừng (Nguồn [3]). Một số công ty chứng khoán có lệnh điều kiện riêng trên ứng dụng (Nguồn [4]).

## Liên quan

- [[price-action]] — chủ đề trụ cột
- [[quy-trinh-giao-dich-price-action]]
- [[quan-ly-rui-ro-giao-dich]]
