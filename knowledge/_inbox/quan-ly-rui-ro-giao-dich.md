---
title: Quản lý rủi ro giao dịch
type: framework
summary: Quản lý rủi ro là quyết định trước mỗi lệnh được phép mất bao nhiêu, rồi tính khối lượng lệnh từ đó. Elder giới hạn 2% tài khoản mỗi lệnh; Wyckoff chỉ giao dịch bằng 5–10% vốn.
tags: [trading]
status: draft
aliases: [Quản lý vốn, Money management, Risk management, Quy tắc 2%, Quy tắc 6%, Khối lượng lệnh, Position sizing]
sources:
  - title: "The New Sell and Sell Short (2nd ed.) — Chapter 2: Risk Control (phần xem trước)"
    url: https://www.oreilly.com/library/view/the-new-sell/9781118005576/h2_elde_9781118005576_oeb_c02_r1.html
    author: Alexander Elder
    publisher: Wiley (qua O'Reilly)
    date: 2011
    kind: book
    credibility: 3
    primary: true
    credibility_note: Tài liệu gốc của quy tắc 2% và 6% do chính Elder đặt ra; chỉ đọc được phần xem trước (đoạn đầu mục Risk Control). Tác giả bán sách và khoá học; các quy tắc là kinh nghiệm, không phải kết quả nghiên cứu.
    accessed: 2026-11-26
  - title: "The New Sell and Sell Short (2nd ed.) — Answer to Question 15: The 6% Rule (phần xem trước)"
    url: https://www.oreilly.com/library/view/the-new-sell/9781118005576/h15_elde_9781118005576_oeb_c05_r1.html
    author: Alexander Elder
    publisher: Wiley (qua O'Reilly)
    date: 2011
    kind: book
    credibility: 3
    primary: true
    credibility_note: Lời giải của chính tác giả về mục đích quy tắc 6%; cách tính chi tiết nằm ở phần sách chưa đọc được.
    accessed: 2026-11-26
  - title: "How I Trade and Invest in Stocks and Bonds"
    url: https://archive.org/details/cu31924031269552
    author: Richard D. Wyckoff
    publisher: The Magazine of Wall Street
    date: 1922
    kind: book
    credibility: 4
    primary: true
    credibility_note: Tài liệu gốc của Wyckoff, đọc trọn 14 chương (bản in lần thứ 5, 1925). Số trang theo bản đó; bản đồ sách ở ghi chú riêng.
    accessed: 2026-11-25
  - title: "How to Trade Price Action Manual (trang giới thiệu)"
    url: https://www.brookstradingcourse.com/how-to-trade-price-action-manual/
    author: Al Brooks
    publisher: Brooks Trading Course
    kind: blog
    credibility: 2
    primary: true
    credibility_note: Trang của chính tác giả, trên trang bán khoá học; tỷ lệ thắng nêu ra là ước tính cá nhân.
    accessed: 2026-11-25
  - title: "Trading Price Action Trends — Chapter 1 (trích đoạn của nhà xuất bản)"
    url: https://media.wiley.com/product_data/excerpt/10/11180665/1118066510-72.pdf
    author: Al Brooks
    publisher: Wiley
    date: 2011
    kind: book
    credibility: 3
    primary: true
    credibility_note: Tài liệu gốc, đọc toàn văn chương 1; tác giả bán khoá học.
    accessed: 2026-11-25
  - title: "Investor Bulletin: Stop, Stop-Limit, and Trailing Stop Orders"
    url: https://www.investor.gov/additional-resources/news-alerts/alerts-bulletins/investor-bulletin-stop-stop-limit-trailing-stop
    publisher: SEC — Office of Investor Education and Assistance
    date: 2017-07-13
    kind: official
    credibility: 5
    primary: true
    credibility_note: Bản tin chính thức của SEC (cập nhật 18/8/2026) về rủi ro giá khớp của lệnh dừng.
    accessed: 2026-11-26
  - title: "Reading Price Charts Bar by Bar — Chapter 1: Price Action (trích đoạn của nhà xuất bản)"
    url: https://media.wiley.com/product_data/excerpt/52/04704439/0470443952.pdf
    author: Al Brooks
    publisher: Wiley
    date: 2009
    kind: book
    credibility: 3
    primary: true
    credibility_note: Tài liệu gốc, đọc toàn văn chương 1; tác giả bán khoá học.
    accessed: 2026-11-25
created: 2026-11-26
updated: 2026-11-26
research:
  keyword: Price action
  date: 2026-11-26
  agent: claude
---

## Mục đích

**Quản lý rủi ro** trong giao dịch là quyết định trước mỗi lệnh được phép mất bao nhiêu tiền, rồi tính khối lượng lệnh từ con số đó, để không một lệnh hay một chuỗi lệnh xấu nào làm hỏng tài khoản. Alexander Elder gọi quy tắc 2% và quy tắc 6% là "hai trụ cột" của quản lý vốn: quy tắc 2% chống một khoản lỗ thảm hoạ (ông ví là "cá mập cắn"), quy tắc 6% chống một chuỗi lệnh xấu (ông ví là "cá piranha cắn") (Nguồn [1]). Ông ví quản lý rủi ro như dây an toàn của người đi trên dây: nó cứu mạng bạn khi trượt chân (Nguồn [1]).

## Mục tiêu bài học

Sau bài này bạn sẽ:

- tính được số tiền tối đa được mất cho một lệnh, và khối lượng lệnh tương ứng;
- giải thích được vì sao một khoản lỗ lớn rất khó gỡ lại;
- nêu được cách giới hạn một chuỗi lệnh xấu, và phần vốn Wyckoff dành cho giao dịch.

## Các bước / thành phần

### Bước 1 — Tách phần vốn dành cho giao dịch

Wyckoff chỉ dùng 5–10% vốn nhàn rỗi để giao dịch; phần lớn vốn để ở chỗ an toàn, có thu nhập (Nguồn [3], tr. 42, 146–147). Ông cho rằng 85–90% thất bại là do giao dịch quá sức hoặc thiếu vốn, và hai thứ đó thực chất là một (Nguồn [3], tr. 137, 163). Ai không thắng được với vốn nhỏ thì chắc chắn sẽ mất nhiều hơn khi dùng vốn lớn (Nguồn [3], tr. 147).

### Bước 2 — Giới hạn rủi ro mỗi lệnh: quy tắc 2%

Quy tắc 2% của Elder: **không mạo hiểm quá 2% vốn trong tài khoản cho bất kỳ lệnh nào**. Tài khoản 100.000 USD thì mỗi lệnh được mất tối đa 2.000 USD (Nguồn [1]). "Mạo hiểm" ở đây là số tiền mất nếu giá chạm mức dừng lỗ: khoảng cách từ giá vào tới giá dừng lỗ nhân với khối lượng.

Elder giải thích vì sao: **giao dịch quá sức** là đặt lệnh có khối lượng quá lớn so với tài khoản. Khi tiền cược quá lớn, người giao dịch căng cứng, mất sự linh hoạt và làm kém đi (Nguồn [1]). Brooks cũng khuyên dùng khối lượng nhỏ để không bị nỗi lo thua lỗ chi phối (Nguồn [4]).

### Bước 3 — Tính khối lượng lệnh

Theo thứ tự trong ví dụ của Elder: **chọn mức dừng lỗ trước, rồi mới tính khối lượng** (Nguồn [1]).

> Khối lượng tối đa = Rủi ro tối đa cho lệnh ÷ Rủi ro mỗi cổ phiếu (giá vào − giá dừng lỗ)

Ví dụ của Elder: tài khoản 100.000 USD, mua cổ phiếu giá 40 USD, dừng lỗ 38 USD, tức rủi ro 2 USD mỗi cổ phiếu. Lấy 2.000 chia cho 2 (Nguồn [1]) thì được tối đa 1.000 cổ phiếu. Mức dừng lỗ đặt ở chỗ mà nếu giá tới thì ý tưởng giao dịch đã sai (xem [[quy-trinh-giao-dich-price-action]]), không phải ở chỗ để mua được nhiều hơn.

### Bước 4 — Giới hạn một chuỗi lệnh xấu: quy tắc 6%

Quy tắc 2% chỉ giới hạn từng lệnh. Theo Elder, quy tắc 6% bảo vệ tài khoản khỏi thiệt hại của **một chuỗi lệnh xấu**: nó buộc bạn lùi lại khi thua lỗ bắt đầu dồn lên, ngược với điều phần lớn người giao dịch làm là giao dịch cuống cuồng hơn để gỡ (Nguồn [2]). Ông dặn áp dụng quy tắc này **trước khi** vào lệnh, để biết mình còn chịu thêm rủi ro của một lệnh mới hay không (Nguồn [2]). Theo tên quy tắc, mức giới hạn là 6% tài khoản; cách tính cụ thể nằm ở phần sách chưa đọc được (xem "Câu hỏi còn mở").

### Bước 5 — Hiểu toán học của thua lỗ

Elder nêu ví dụ: người mới mất một phần ba vốn thì phải lãi 50% trên phần còn lại mới hoà vốn. Người bị một cú lỗ lớn không chỉ mất tiền mà còn mất tự tin, trở nên sợ hãi và không dám vào lệnh (Nguồn [1]). Phép tính chung: lỗ x% thì cần lãi x ÷ (100 − x) để hoà vốn.

| Lỗ | Cần lãi để hoà vốn |
| --- | --- |
| 10% | 11,1% |
| 20% | 25% |
| 33,3% | 50% |
| 50% | 100% |

### Bước 6 — Ghép với tỷ lệ lời/lỗ của từng lệnh

Quản lý rủi ro đi cùng việc chọn lệnh có lợi nhuận dự kiến đủ lớn so với rủi ro: Wyckoff đòi ít nhất gấp ba, bốn lần (Nguồn [3], tr. 49); Brooks nhắm ít nhất gấp đôi với lệnh theo sóng, và cho rằng phần lớn lệnh chỉ có xác suất thắng khoảng 40–60% (Nguồn [4]). Nếu lời bằng lỗ thì phải thắng trên 60% số lệnh mới đáng giao dịch (Nguồn [5]). Chi tiết ở [[quy-trinh-giao-dich-price-action]].

## Ví dụ áp dụng

Tài khoản giao dịch 200 triệu đồng (số giả định).

1. **Rủi ro tối đa mỗi lệnh (2%):** 200 triệu × 2% = 4 triệu đồng.
2. **Lệnh dự định:** mua ở 25.400 đồng, dừng lỗ 24.400 đồng, tức rủi ro 1.000 đồng mỗi cổ phiếu.
3. **Khối lượng tối đa:** 4.000.000 ÷ 1.000 = 4.000 cổ phiếu. Giá trị lệnh khoảng 101,6 triệu đồng, tức gần một nửa tài khoản, dù số tiền có thể mất chỉ là 2%.
4. **Sau ba lệnh thua liên tiếp,** mỗi lệnh khoảng 2%, tài khoản đã mất khoảng 6%. Theo tinh thần quy tắc 6%, đây là lúc lùi lại, không vào thêm lệnh (Nguồn [2]).

Ví dụ chỉ minh hoạ cách tính, **không phải khuyến nghị**.

## Lưu ý & giới hạn

- **Dừng lỗ không đảm bảo giá.** Lệnh dừng thành lệnh thị trường khi được kích hoạt, và giá khớp có thể lệch xa giá dừng (Nguồn [6]). Giá mở cửa nhảy qua mức dừng lỗ thì thua lỗ thật có thể lớn hơn 2%. Xem [[cac-loai-lenh-giao-dich]].
- **Đừng kéo dừng lỗ sát lại vì sợ.** Brooks kể ví dụ người giao dịch hoảng, kéo dừng lỗ sát lại và bị quét ra ngay trước khi giá chạy đúng hướng (Nguồn [7]).
- Quy tắc 2% và 6% là **kinh nghiệm của Elder**, không phải kết quả nghiên cứu. Các con số của Wyckoff (5–10% vốn, dừng lỗ 2–3 điểm) là bối cảnh cổ phiếu Mỹ những năm 1920.
- Quản lý rủi ro chỉ giới hạn thiệt hại của từng lệnh và chuỗi lệnh. Một phương pháp có hiệu quả hay không phải kiểm chứng riêng: xem [[kiem-chung-phuong-phap-giao-dich]] và [[ty-le-nha-giao-dich-ca-nhan-thua-lo]].

## Bài tập

Tự làm trước, rồi mới mở đáp án. Mọi con số là **giả định** để luyện cách tính.

### Câu 1: Tính khối lượng lệnh

Tài khoản giao dịch 150 triệu đồng, bạn theo quy tắc 2%. Bạn định mua ở 40.000 đồng, dừng lỗ ở 38.500 đồng. Được mua tối đa bao nhiêu cổ phiếu?

**Gợi ý:** Tính số tiền tối đa được mất trước, rồi chia cho rủi ro mỗi cổ phiếu.

**Đáp án:** Rủi ro tối đa: 150 triệu × 2% = **3 triệu đồng** (Nguồn [1]). Rủi ro mỗi cổ phiếu: 40.000 − 38.500 = 1.500 đồng. Khối lượng tối đa: 3.000.000 ÷ 1.500 = **2.000 cổ phiếu**.

### Câu 2: Gỡ lại một khoản lỗ

Tài khoản mất 25%. Phải lãi bao nhiêu phần trăm trên phần còn lại mới hoà vốn? Theo Elder, mất một phần ba vốn thì sao?

**Đáp án:** Mất 25% thì cần lãi 25 ÷ 75 ≈ **33,3%**. Mất một phần ba thì cần lãi **50%** trên phần còn lại, đúng ví dụ của Elder (Nguồn [1]). Lỗ càng lớn, mức lãi cần để gỡ càng tăng nhanh.

### Câu 3: Sau một chuỗi lệnh thua

Bạn vừa thua ba lệnh liên tiếp, mỗi lệnh khoảng 2% tài khoản, và đang muốn vào ngay lệnh thứ tư để gỡ. Theo quy tắc 6% của Elder, nên làm gì? Vì sao?

**Đáp án:** **Lùi lại, không vào thêm lệnh.** Quy tắc 6% được đặt ra để chặn thiệt hại của một chuỗi lệnh xấu, buộc người giao dịch lùi lại khi thua lỗ dồn lên. Elder chỉ ra rằng phần lớn người giao dịch làm ngược lại: giao dịch cuống cuồng và tự đào hố sâu hơn (Nguồn [2]).

### Câu 4: Kéo dừng lỗ để mua được nhiều hơn

Ở câu 1, bạn muốn mua 6.000 cổ phiếu nên dời dừng lỗ từ 38.500 lên 39.500 (rủi ro chỉ còn 500 đồng mỗi cổ phiếu). Cách làm này có ổn không?

**Đáp án:** **Không.** Mức dừng lỗ phải đặt ở chỗ mà nếu giá tới thì ý tưởng giao dịch đã sai, rồi mới tính khối lượng từ đó; ví dụ của Elder cũng chọn mức dừng lỗ trước rồi mới chia ra khối lượng (Nguồn [1]). Dời dừng lỗ sát lại chỉ để mua nhiều hơn dễ bị quét ra trước khi giá chạy đúng hướng, như ví dụ Brooks kể (Nguồn [7]).

## Câu hỏi còn mở

- Cách tính chi tiết của quy tắc 6% (tính trong khoảng thời gian nào, gồm những khoản lỗ nào) nằm trong phần sách của Elder chưa đọc được.
- Chưa có nguồn chính thức của Việt Nam về quản lý rủi ro cho nhà đầu tư cá nhân (ví dụ tài liệu giáo dục của Ủy ban Chứng khoán Nhà nước).

## Câu hỏi thường gặp

### Quy tắc 2% là gì?

Quy tắc 2% của Alexander Elder: không mạo hiểm quá 2% vốn trong tài khoản cho bất kỳ lệnh nào. Tài khoản 100.000 USD thì mỗi lệnh được mất tối đa 2.000 USD nếu giá chạm dừng lỗ (Nguồn [1]).

### Tính khối lượng lệnh thế nào?

Chọn mức dừng lỗ trước. Lấy số tiền tối đa được mất cho lệnh chia cho khoảng cách từ giá vào tới giá dừng lỗ. Ví dụ của Elder: 2.000 USD chia cho rủi ro 2 USD mỗi cổ phiếu (Nguồn [1]).

### Nên dùng bao nhiêu vốn để giao dịch?

Wyckoff chỉ dùng 5–10% vốn nhàn rỗi để giao dịch, phần còn lại để ở chỗ an toàn có thu nhập (Nguồn [3], tr. 42, 146–147). Đây là kinh nghiệm của ông, không phải quy định.

## Liên quan

- [[price-action]] — chủ đề trụ cột
- [[quy-trinh-giao-dich-price-action]]
- [[cac-loai-lenh-giao-dich]]
- [[kiem-chung-phuong-phap-giao-dich]]
- [[ty-le-nha-giao-dich-ca-nhan-thua-lo]]
- [[wyckoff-how-i-trade-and-invest]]: bản đồ sách gốc của Wyckoff
