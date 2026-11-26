---
title: Kiểm chứng phương pháp giao dịch
type: framework
summary: Kiểm chứng là kiểm tra một cách giao dịch có lợi thế thật hay chỉ trông tốt nhờ may mắn. Càng thử nhiều cấu hình trên cùng dữ liệu, kết quả đẹp càng dễ là giả.
tags: [trading, phan-tich-ky-thuat]
status: draft
aliases: [Backtest, Kiểm thử quá khứ, Backtest overfitting, Quá khớp, Data-snooping, Đào dữ liệu, Out-of-sample, Nhật ký giao dịch]
sources:
  - title: "Pseudo-Mathematics and Financial Charlatanism: The Effects of Backtest Overfitting on Out-of-Sample Performance"
    url: https://carmamaths.org/jon/backtest.pdf
    author: David H. Bailey, Jonathan M. Borwein, Marcos López de Prado, Qiji Jim Zhu
    publisher: Notices of the American Mathematical Society 61(5), 458–471
    date: 2014-05
    kind: paper
    credibility: 5
    primary: true
    credibility_note: Bài có bình duyệt, đặt ra khái niệm độ dài kiểm thử tối thiểu (MinBTL) cho bài toán quá khớp; đọc toàn văn bản của tác giả (bản ngày 1/4/2014). Phần chứng minh dùng giả định toán học (lợi nhuận độc lập, phân phối chuẩn).
    accessed: 2026-11-26
  - title: "Data-Snooping, Technical Trading Rule Performance, and the Bootstrap"
    url: https://bashtage.github.io/kevinsheppard.com/files/teaching/mfe/advanced-econometrics/Sullivan_Timmermann_White.pdf
    author: Ryan Sullivan, Allan Timmermann, Halbert White
    publisher: The Journal of Finance 54(5), 1647–1691
    date: 1999-10
    kind: paper
    credibility: 5
    primary: true
    credibility_note: Nghiên cứu có bình duyệt trên tạp chí tài chính hàng đầu, kiểm định gần 8.000 quy tắc kỹ thuật trên 100 năm dữ liệu Dow Jones; đọc bản bài đăng tạp chí (tệp do một giảng viên đăng để dạy).
    accessed: 2026-11-26
  - title: "Foundations of Technical Analysis: Computational Algorithms, Statistical Inference, and Empirical Implementation"
    url: https://www.nber.org/papers/w7613
    author: Andrew W. Lo, Harry Mamaysky, Jiang Wang
    publisher: National Bureau of Economic Research — Working Paper 7613
    date: 2000-03
    kind: paper
    credibility: 5
    credibility_note: Nghiên cứu định lượng của các nhà kinh tế MIT; đọc toàn văn bản NBER.
    accessed: 2026-11-25
  - title: "The New Sell and Sell Short (2nd ed.) — Chapter 3: Good Records Lead to Good Trading (phần xem trước)"
    url: https://www.oreilly.com/library/view/the-new-sell/9781118005576/h1_elde_9781118005576_oeb_c03_r1.html
    author: Alexander Elder
    publisher: Wiley (qua O'Reilly)
    date: 2011
    kind: book
    credibility: 3
    primary: true
    credibility_note: Sách của chính người thực hành về ghi chép giao dịch; chỉ đọc được phần xem trước. Tác giả bán sách và khoá học.
    accessed: 2026-11-26
  - title: "How I Trade and Invest in Stocks and Bonds"
    url: https://archive.org/details/cu31924031269552
    author: Richard D. Wyckoff
    publisher: The Magazine of Wall Street
    date: 1922
    kind: book
    credibility: 4
    primary: true
    credibility_note: Tài liệu gốc của Wyckoff, đọc trọn 14 chương (bản in lần thứ 5, 1925).
    accessed: 2026-11-25
created: 2026-11-26
updated: 2026-11-26
research:
  keyword: Price action
  date: 2026-11-26
  agent: claude
---

## Mục đích

**Kiểm chứng phương pháp giao dịch** là kiểm tra xem một cách giao dịch có lợi thế thật hay chỉ trông tốt nhờ may mắn, trước khi dùng tiền thật. Cách phổ biến nhất là **kiểm thử quá khứ** (backtest): mô phỏng xem cách đó đã cho kết quả ra sao trên dữ liệu cũ (Nguồn [1]). Bailey và cộng sự chứng minh rằng chỉ cần thử một số không nhiều cấu hình khác nhau là đã dễ tìm được một cấu hình có kết quả quá khứ rất đẹp, hiện tượng họ gọi là **quá khớp khi kiểm thử** (backtest overfitting). Thử càng nhiều cấu hình, xác suất kết quả bị quá khớp càng cao (Nguồn [1]).

## Mục tiêu bài học

Sau bài này bạn sẽ:

- giải thích được vì sao một kết quả kiểm thử quá khứ đẹp chưa chứng minh được gì;
- nêu được các bước tự kiểm chứng: viết quy tắc rõ, đếm số lần thử, giữ dữ liệu chưa dùng, tính chi phí, ghi nhật ký;
- đọc đúng một nghiên cứu thật về quy tắc kỹ thuật trước và sau khi tính tới việc đã thử nhiều quy tắc.

## Các bước / thành phần

### Bước 1 — Viết cách giao dịch thành quy tắc đủ rõ

Muốn kiểm chứng thì phải biết chính xác mình đang kiểm cái gì: bối cảnh, setup, điểm vào, dừng lỗ, mục tiêu (xem [[quy-trinh-giao-dich-price-action]]). Với giới nghiên cứu, tính chủ quan là rào cản lớn nhất khi muốn kiểm chứng phân tích kỹ thuật: cách đọc biểu đồ bằng mắt phải được biến thành quy tắc mà máy tính làm lại được (Nguồn [3]). Price action theo kiểu tự đọc từng thanh vì vậy rất khó kiểm bằng dữ liệu quá khứ.

### Bước 2 — Hiểu kiểm thử quá khứ nói được gì

- **Trong mẫu và ngoài mẫu.** Kết quả trên chính dữ liệu dùng để chọn ra cách giao dịch gọi là kết quả **trong mẫu** (in-sample). Kết quả trên dữ liệu chưa từng dùng gọi là **ngoài mẫu** (out-of-sample). Bailey và cộng sự nhận xét phần lớn nghiên cứu công bố chiến lược sinh lời chỉ báo cáo kết quả trong mẫu (Nguồn [1]).
- **Đào dữ liệu** (data-snooping): khi một bộ dữ liệu được dùng nhiều lần để chọn mô hình, luôn có khả năng kết quả tốt chỉ là ngẫu nhiên chứ không phải nhờ phương pháp (Nguồn [2], tr. 1647–1648).

### Bước 3 — Đếm và ghi lại mọi lần thử

- Bailey và cộng sự: khi một kết quả kiểm thử không báo đã thử bao nhiêu cấu hình để chọn ra nó, người khác **không thể** đánh giá rủi ro quá khớp (Nguồn [1]).
- **Ví dụ của họ:** nếu chỉ có 5 năm dữ liệu thì không nên thử quá 45 cấu hình độc lập. Thử nhiều hơn thì gần như chắc chắn tìm được một chiến lược có tỷ số Sharpe (lợi nhuận so với rủi ro) trong mẫu bằng 1, trong khi kỳ vọng ngoài mẫu bằng 0. Với 2 năm dữ liệu, chỉ cần thử 7 cấu hình là đã đạt mức đó (Nguồn [1], mục 3).
- Kết luận của họ: **càng thử nhiều, càng phải đòi kết quả trong mẫu cao hơn** (Nguồn [1], mục 9).

### Bước 4 — Giữ lại một phần dữ liệu chưa dùng

Cách "giữ lại" (hold-out) là để riêng một phần dữ liệu, không dùng nó khi chọn cách giao dịch, rồi mới dùng nó để kiểm. Bailey và cộng sự nhận xét rất hiếm nghiên cứu làm bước này (Nguồn [1]). Sullivan, Timmermann và White làm đúng như vậy với 10 năm dữ liệu Dow Jones (1987–1996) mà nghiên cứu trước chưa dùng, và kết quả đẹp trước đó không còn (Nguồn [2]); xem phần "Ví dụ áp dụng".

### Bước 5 — Tính chi phí và điều kiện thực tế

Phí giao dịch và việc có bán khống được hay không có thể làm một quy tắc "thắng trên giấy" thành không có lãi. Sullivan và cộng sự kiểm thêm trên hợp đồng tương lai chỉ số S&P 500, nơi chi phí dễ kiểm soát và bán khống không bị hạn chế (Nguồn [2]). Với cổ phiếu Việt Nam, cần tính cả phí, thuế, biên độ giá và lệnh có khớp được hay không (xem [[cac-loai-lenh-giao-dich]]).

### Bước 6 — Giao dịch thử và ghi nhật ký trước khi dùng tiền thật

- Wyckoff khuyên tập giao dịch trên giấy hai, ba năm. Bản thân ông học 8 năm mới bắt đầu đầu tư, rồi 6 năm sau nữa mới giao dịch (Nguồn [5], tr. 17–18, 159).
- Elder: ghi chép tốt biến trải nghiệm thoáng qua thành bài học có thể xem lại. Một lệnh hay vài lệnh thắng không làm bạn thành người thắng; cần một chuỗi lệnh có kết quả tốt trong thời gian dài (Nguồn [4]).
- Kết quả giao dịch thử và nhật ký lệnh thật cũng là dữ liệu: đếm số lệnh, tỷ lệ thắng, lời và lỗ trung bình, rồi so với phương trình lời/lỗ ở [[quy-trinh-giao-dich-price-action]].

## Ví dụ áp dụng

Sullivan, Timmermann và White (1999) kiểm lại nghiên cứu nổi tiếng của Brock, Lakonishok và LeBaron (1992), vốn thấy 26 quy tắc kỹ thuật đều thắng việc giữ tiền mặt trên dữ liệu Dow Jones (Nguồn [2]):

1. **Mở rộng phép thử:** họ dựng gần 8.000 biến thể quy tắc và áp lên 100 năm dữ liệu Dow Jones (1897–1996) (Nguồn [2]).
2. **Trong giai đoạn cũ (1897–1986):** một số quy tắc vẫn thắng mốc chuẩn kể cả sau khi đã tính tới việc thử nhiều quy tắc (Nguồn [2]).
3. **Ngoài mẫu (1987–1996):** xác suất để quy tắc tốt nhất thật ra không thắng mốc chuẩn là gần 12%, nên theo các mức ý nghĩa thông thường hầu như không có bằng chứng quy tắc kỹ thuật có giá trị kinh tế trong giai đoạn này (Nguồn [2]).
4. **Ảnh hưởng của việc đào dữ liệu:** với quy tắc tốt nhất theo lợi nhuận trung bình ở giai đoạn 1987–1996, nếu chỉ kiểm riêng quy tắc đó thì trông rất có ý nghĩa (p = 0,000); tính cả việc nó được chọn ra từ cả "vũ trụ" quy tắc thì p = 0,341, tức không có ý nghĩa (Nguồn [2], Bảng VI).
5. **Hợp đồng tương lai S&P 500 (1984–1996):** không có bằng chứng các quy tắc thắng mốc chuẩn (Nguồn [2]).

## Lưu ý & giới hạn

- Nghiên cứu của Sullivan và cộng sự là về các quy tắc kỹ thuật viết được thành công thức, trên chỉ số Mỹ; nó không kiểm trực tiếp cách đọc biểu đồ bằng mắt kiểu price action. Bằng chứng khác ở [[nghien-cuu-hieu-qua-phan-tich-ky-thuat]].
- Các con số của Bailey và cộng sự (45 cấu hình cho 5 năm…) dựa trên giả định toán học và tỷ số Sharpe; chúng minh hoạ mức độ, không phải ngưỡng cho mọi trường hợp (Nguồn [1]).
- Lời khuyên giao dịch thử, ghi nhật ký là kinh nghiệm của người thực hành (Wyckoff, Elder), không phải kết quả nghiên cứu.

## Bài tập

Tự làm trước, rồi mới mở đáp án. Các tình huống là **giả định**.

### Câu 1: Bộ tham số tốt nhất trong 200 lần thử

Bạn thử 200 bộ tham số khác nhau cho một cách giao dịch trên 3 năm dữ liệu, rồi chọn bộ có kết quả tốt nhất. Có nên tin kết quả của bộ đó không? Cần làm gì thêm?

**Đáp án:** **Chưa nên tin.** Thử càng nhiều cấu hình, kết quả tốt nhất càng dễ là quá khớp; theo ví dụ của Bailey và cộng sự, 5 năm dữ liệu chỉ nên thử tối đa khoảng 45 cấu hình độc lập, mà ở đây là 200 lần trên 3 năm (Nguồn [1]). Cần ghi rõ đã thử 200 lần, đòi kết quả trong mẫu cao hơn, và kiểm lại trên phần dữ liệu chưa dùng (Nguồn [1][2]).

### Câu 2: Năm lệnh thắng liên tiếp

Bạn vừa thắng 5 lệnh liên tiếp với một cách giao dịch mới. Đã đủ để kết luận cách này có hiệu quả chưa?

**Đáp án:** **Chưa.** Theo Elder, một lệnh hay vài lệnh thắng không làm bạn thành người thắng; cần một chuỗi lệnh có kết quả tốt trong thời gian dài, và chỉ ghi chép đầy đủ mới cho thấy điều đó (Nguồn [4]). Wyckoff thì khuyên tập trên giấy hai, ba năm trước khi giao dịch thật (Nguồn [5], tr. 17–18).

## Câu hỏi còn mở

- Chưa có nghiên cứu nào trong kho kiểm chứng trực tiếp price action kiểu Brooks (đọc từng thanh bằng mắt).
- Chưa đọc tài liệu gốc về cách kiểm thử "đi tới" (walk-forward) hay kiểm định thống kê cho người giao dịch cá nhân.

## Câu hỏi thường gặp

### Backtest là gì?

Backtest (kiểm thử quá khứ) là mô phỏng xem một cách giao dịch đã cho kết quả ra sao nếu áp dụng trên dữ liệu giá trong quá khứ (Nguồn [1]).

### Quá khớp khi kiểm thử (backtest overfitting) là gì?

Là khi một cách giao dịch có kết quả quá khứ đẹp chỉ vì đã thử rất nhiều cấu hình rồi chọn cấu hình may mắn nhất. Bailey và cộng sự chứng minh thử càng nhiều cấu hình thì xác suất quá khớp càng cao, và cách quá khớp thường cho kết quả kém khi dùng thật (Nguồn [1]).

### Làm sao biết một phương pháp giao dịch có hiệu quả thật?

Viết quy tắc rõ, ghi lại số lần đã thử, kiểm trên dữ liệu chưa dùng, tính đủ chi phí, rồi giao dịch thử và ghi nhật ký đủ lâu (Nguồn [1][2][4][5]). Một nghiên cứu lớn trên 100 năm dữ liệu Dow Jones cho thấy quy tắc kỹ thuật tốt nhất không còn bằng chứng hiệu quả ở giai đoạn ngoài mẫu (Nguồn [2]).

## Liên quan

- [[price-action]] — chủ đề trụ cột
- [[nghien-cuu-hieu-qua-phan-tich-ky-thuat]]
- [[ty-le-nha-giao-dich-ca-nhan-thua-lo]]
- [[quy-trinh-giao-dich-price-action]]
- [[quan-ly-rui-ro-giao-dich]]
