---
title: Biểu đồ nến Nhật
type: concept
summary: Biểu đồ nến Nhật thể hiện giá từng phiên bằng một "cây nến" — thân nến nối giá mở và đóng cửa, bóng nến chỉ giá cao nhất và thấp nhất — và cho thấy bên mua hay bên bán đã thắng phiên đó.
tags: [trading, phan-tich-ky-thuat]
status: draft
aliases: [Nến Nhật, Biểu đồ nến, Candlestick chart, Japanese candlestick]
sources:
  - title: "Introduction to Candlesticks"
    url: https://chartschool.stockcharts.com/table-of-contents/chart-analysis/candlestick-charts/introduction-to-candlesticks
    publisher: StockCharts.com — ChartSchool
    kind: education
    credibility: 3
    credibility_note: Nguồn thứ cấp (trang giáo dục của nhà cung cấp phần mềm biểu đồ) tóm theo sách gốc của Nison và Morris; dùng vì chưa đọc được phần mô hình trong sách gốc.
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
  - title: "Candlestick chart"
    url: https://en.wikipedia.org/wiki/Candlestick_chart
    publisher: Wikipedia
    kind: other
    credibility: 3
    credibility_note: Bách khoa mở; phần lịch sử trích sách của Steve Nison (có ISBN) nhưng chưa đọc được sách gốc để đối chiếu.
    accessed: 2026-11-25
  - title: "Honma Munehisa"
    url: https://en.wikipedia.org/wiki/Honma_Munehisa
    publisher: Wikipedia
    kind: other
    credibility: 2
    credibility_note: Bách khoa mở; một số câu dẫn từ trang hướng dẫn của sàn môi giới và có chi tiết khó kiểm chứng.
    accessed: 2026-11-25
  - title: "Technische Analyse mit Candlesticks — Kapitel 1: Einleitung (bản đọc thử của nhà xuất bản)"
    url: https://www.m-vg.de/mediafiles/Leseprobe/9783898798433.pdf
    author: Steve Nison
    publisher: FinanzBuch Verlag (bản tiếng Đức của Japanese Candlestick Charting Techniques)
    kind: book
    credibility: 3
    primary: true
    credibility_note: Tài liệu gốc của người đưa nến Nhật sang phương Tây, nhưng chỉ đọc được chương mở đầu bản dịch tiếng Đức; phần tiêu chí từng mô hình chưa đọc được.
    accessed: 2026-11-25
created: 2026-11-25
updated: 2026-11-25
research:
  keyword: Price action
  date: 2026-11-25
  agent: claude
---

## Định nghĩa

**Biểu đồ nến Nhật** (candlestick chart) là cách vẽ giá trong đó mỗi phiên (một ngày, một giờ, năm phút…) là một "cây nến" thể hiện bốn mức giá. Giá mở cửa và đóng cửa tạo thành **thân nến**, giá cao nhất và thấp nhất là hai đầu **bóng nến** (còn gọi là râu nến) (Nguồn [1]). Ưu điểm của nến so với biểu đồ đường là cho thấy ngay **bên mua hay bên bán đã thắng phiên đó**, và thắng áp đảo hay sít sao (Nguồn [1][2]).

## Mục tiêu bài học

Sau bài này bạn sẽ:

- đọc được bốn mức giá (mở cửa, đóng cửa, cao nhất, thấp nhất) trên một cây nến;
- giải thích được thân nến và bóng nến cho biết gì về bên mua và bên bán;
- giải thích được vì sao một cây nến phải đọc trong bối cảnh, và nến không cho biết điều gì.

## Giải thích

### Cấu tạo một cây nến

- **Thân nến** (real body): khoảng giữa giá mở cửa và giá đóng cửa. Đóng cửa cao hơn mở cửa thì thân rỗng hoặc xanh (lực mua); đóng cửa thấp hơn thì thân đặc hoặc đỏ (lực bán). Phần mềm cho đổi màu tuỳ ý, nên cần xem quy ước của từng nền tảng (Nguồn [1][3]).
- **Bóng trên, bóng dưới** (shadow, wick): phần giá vượt lên trên hoặc xuống dưới thân. Đỉnh bóng trên là giá cao nhất, đáy bóng dưới là giá thấp nhất phiên (Nguồn [1]).
- Nến **không cho biết thứ tự** giao dịch giữa lúc mở và lúc đóng cửa (Nguồn [1]). Muốn biết giá đi lên trước hay xuống trước thì phải xem khung thời gian nhỏ hơn (Nguồn [2]).

### Đọc thân nến: ai thắng, thắng mạnh hay yếu

- **Thân dài** = lực mua (hoặc bán) mạnh; **thân ngắn** = giá đi ngang, hai bên giằng co (Nguồn [1]).
- **Nến tăng dài**: bên mua kiểm soát. **Nến giảm dài**: bên bán kiểm soát (Nguồn [1]).
- Vị trí đổi ý nghĩa: một nến giảm dài sau đợt tăng dài có thể đánh dấu bước ngoặt hoặc kháng cự. Sau đợt giảm dài, nó có thể là dấu hiệu bán tháo hoảng loạn (Nguồn [1]).
- **Marubozu** (nến không bóng): tăng thì mở ở giá thấp nhất và đóng ở giá cao nhất; một bên thắng tuyệt đối suốt phiên (Nguồn [1]).
- Brooks chia mọi thanh thành hai loại: **thanh [[xu-huong-gia|xu hướng]]** (có thân rõ, một bên kiểm soát) và **thanh cân bằng** (thân nhỏ, như một vùng đi ngang thu nhỏ) (Nguồn [2]). Trong xu hướng mạnh, thanh xu hướng không có bóng ở phía đóng cửa cho thấy sức mạnh kéo dài tới tận cuối phiên (Nguồn [2]).

### Đọc bóng nến: giá đã bị từ chối ở đâu

- **Bóng trên dài, bóng dưới ngắn:** bên mua đã đẩy giá lên cao nhưng bên bán ép giá xuống lại trước khi đóng cửa (Nguồn [1]).
- **Bóng dưới dài, bóng trên ngắn:** bên bán đã đẩy giá xuống sâu nhưng bên mua kéo lên lại (Nguồn [1]). Brooks đọc bóng dưới dài là dấu hiệu lực bán đã bị từ chối và bên mua kiểm soát thanh (Nguồn [2]).
- **Thân nhỏ, hai bóng dài** (con quay, spinning top): lưỡng lự. Sau đợt tăng dài, nó cho thấy bên mua đang yếu đi (Nguồn [1]).
- **Doji** (giá mở ≈ giá đóng): đứng một mình thì trung tính; ý nghĩa đến từ bối cảnh trước và xác nhận sau. Doji chuồn chuồn (bóng dưới dài) và doji bia mộ (bóng trên dài) mang ý nghĩa đảo chiều tuỳ vị trí (Nguồn [1]).

### Đọc nến trong bối cảnh

Một cây nến chỉ là một phần thông tin. Brooks nhắc: một thanh đảo chiều đơn lẻ không đủ lý do vào lệnh; phải xem nó trong bối cảnh các thanh trước (Nguồn [2]). Ví dụ, một nến bóng dưới dài nhưng chồng lấn nhiều lên các nến trước có thể chỉ là đi ngang ở khung nhỏ hơn (Nguồn [2]). Cách dùng cụ thể:

- nhóm 1–3 nến có hình dạng đặc trưng (búa, nhấn chìm, sao mai…) → xem [[mau-hinh-nen]];
- nến tại [[ho-tro-va-khang-cu|hỗ trợ – kháng cự]] hay đường xu hướng → có ý nghĩa hơn nến giữa khoảng trống;
- nến làm thanh tín hiệu để vào lệnh → xem [[quy-trinh-giao-dich-price-action]].

### Lịch sử (tóm tắt)

- Câu chuyện thường được kể: thương nhân gạo Munehisa Homma (1724–1803) ở Sakata, Nhật Bản là "cha đẻ" của biểu đồ nến. Ngay Wikipedia cũng chỉ viết là ông "đôi khi được coi là" như vậy (Nguồn [3][4]).
- Theo chính Nison, kỹ thuật nến được người Viễn Đông hoàn thiện qua nhiều thế hệ bằng kinh nghiệm, và đã bị che giấu với phương Tây hơn một trăm năm. Cuốn *Japanese Candlestick Charting Techniques* (1991) của ông lần đầu giới thiệu chi tiết nến Nhật cho phương Tây. Ông kể đã biết tới nến năm 1987 qua một nữ môi giới người Nhật, người dùng từ "cửa sổ" cho thứ mà phương Tây gọi là khoảng trống giá (Nguồn [5]). Trong cuốn *Beyond Candlesticks* (1994), chính ông viết rằng ít có khả năng Homma đã dùng biểu đồ nến, và nhiều khả năng nó ra đời đầu thời Minh Trị, cuối thế kỷ 19 (Nguồn [3]).

## Ví dụ

Một phiên mở cửa ở 100, lên cao nhất 108, xuống thấp nhất 97 và đóng cửa ở 105. Cây nến khi đó có:

- thân xanh (tăng) từ 100 lên 105: bên mua thắng phiên;
- bóng trên từ 105 lên 108: bên bán đã ép giá xuống từ đỉnh 108;
- bóng dưới từ 97 lên 100: bên mua đã kéo giá lên từ đáy 97;
- thân (5) chiếm gần một nửa biên độ (11): bên mua thắng nhưng không áp đảo.

## Hiểu sai thường gặp

- **"Nến xanh luôn là phiên tăng."** Màu nến tuỳ phần mềm và có thể đổi (Nguồn [3]).
- **"Một cây nến đẹp là đủ để vào lệnh."** Cần bối cảnh và xác nhận (Nguồn [1][2]).
- **"Doji là tín hiệu đảo chiều."** Đứng một mình thì doji trung tính (Nguồn [1]).
- **"Homma phát minh ra biểu đồ nến."** Điều này còn tranh cãi; chính Nison nghi ngờ nó (Nguồn [3]).

## Bài tập

Tự làm trước, rồi mới mở đáp án. Mọi con số giá là **giả định** để luyện tập.

### Câu 1: Đọc một cây nến

Một phiên mở cửa ở 100, lên cao nhất 102, xuống thấp nhất 88 và đóng cửa ở 101. Thân nến và hai bóng nến nằm ở đâu? Phiên này cho biết gì về bên mua và bên bán?

**Đáp án:** Thân nến tăng, nhỏ, từ 100 lên 101. Bóng trên ngắn, từ 101 lên 102. Bóng dưới rất dài, từ 88 lên 100. Bóng dưới dài, bóng trên ngắn nghĩa là bên bán đã đẩy giá xuống sâu nhưng bên mua kéo lên lại (Nguồn [1]). Brooks đọc kiểu bóng dưới dài này là lực bán đã bị từ chối và bên mua kiểm soát thanh (Nguồn [2]). Hình dạng này gần với nến búa; tiêu chí của từng mô hình nằm ở bài [[mau-hinh-nen]].

### Câu 2: Điều cây nến không cho biết

Hai phiên cùng mở 100, cao nhất 108, thấp nhất 97, đóng 105. Phiên A lên 108 trước rồi mới xuống 97; phiên B xuống 97 trước rồi mới lên 108. Trên biểu đồ nến, hai cây nến có khác nhau không? Muốn biết giá đi lên trước hay xuống trước thì làm gì?

**Đáp án:** Hai cây nến **giống hệt nhau**, vì nến không cho biết thứ tự giao dịch giữa lúc mở và lúc đóng cửa (Nguồn [1]). Muốn biết thì xem khung thời gian nhỏ hơn (Nguồn [2]).

### Câu 3: Doji giữa vùng đi ngang

Giữa một vùng giá đi ngang, xuất hiện một doji (giá mở gần bằng giá đóng). Đó có phải tín hiệu đảo chiều không?

**Đáp án:** **Không, nếu chỉ dựa vào nó.** Doji đứng một mình thì trung tính; ý nghĩa đến từ bối cảnh trước nó và xác nhận sau nó (Nguồn [1]). Brooks cũng nhắc một thanh đảo chiều đơn lẻ không đủ lý do vào lệnh (Nguồn [2]).

### Câu 4: Cùng một cây nến, hai vị trí

Một nến giảm dài xuất hiện (a) ngay sau một đợt tăng dài, (b) ngay sau một đợt giảm dài. Ý nghĩa của nó khác nhau thế nào?

**Đáp án:** (a) Sau đợt tăng dài, nến giảm dài có thể đánh dấu bước ngoặt hoặc kháng cự. (b) Sau đợt giảm dài, nó có thể là dấu hiệu bán tháo hoảng loạn (Nguồn [1]). Cùng một hình dạng, vị trí làm đổi ý nghĩa.

### Câu 5: Ai phát minh ra biểu đồ nến?

Nhiều trang viết "Homma phát minh ra biểu đồ nến ở thế kỷ 18". Câu này có chắc chắn đúng không?

**Đáp án:** **Còn tranh cãi.** Ngay Wikipedia chỉ viết ông "đôi khi được coi là" cha đẻ của biểu đồ nến (Nguồn [3][4]). Trong *Beyond Candlesticks* (1994), Nison cho rằng ít có khả năng Homma đã dùng biểu đồ nến, và nhiều khả năng nó ra đời đầu thời Minh Trị, cuối thế kỷ 19 (Nguồn [3]).

## Câu hỏi thường gặp

### Biểu đồ nến Nhật là gì?

Đây là cách vẽ giá trong đó mỗi phiên là một cây nến. Thân nến nối giá mở cửa với giá đóng cửa, hai đầu bóng nến là giá cao nhất và thấp nhất của phiên (Nguồn [1]).

### Thân nến và bóng nến cho biết điều gì?

Thân nến cho biết bên nào thắng phiên và thắng mạnh hay yếu: thân dài là áp đảo, thân ngắn là giằng co. Bóng nến cho biết giá đã bị từ chối ở đâu. Ví dụ, bóng dưới dài nghĩa là giá bị bán xuống rồi được mua lên lại (Nguồn [1]).

### Doji nghĩa là gì?

Doji là nến có giá mở và giá đóng gần bằng nhau, thể hiện hai bên cân bằng. Đứng một mình nó trung tính; ý nghĩa phụ thuộc bối cảnh trước và xác nhận sau (Nguồn [1]).

### Ai phát minh ra biểu đồ nến?

Câu trả lời phổ biến là thương nhân gạo Munehisa Homma ở thế kỷ 18 (Nguồn [4]). Tuy vậy, Steve Nison, người đưa biểu đồ nến sang phương Tây, cho rằng nhiều khả năng nó ra đời cuối thế kỷ 19 (Nguồn [3]).

## Liên quan

- [[price-action]] — chủ đề trụ cột
- [[mau-hinh-nen]]
- [[ho-tro-va-khang-cu]]
- [[quy-trinh-giao-dich-price-action]]
