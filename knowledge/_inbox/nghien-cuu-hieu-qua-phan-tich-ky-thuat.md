---
title: Nghiên cứu về hiệu quả của phân tích kỹ thuật
type: fact
summary: Trong 95 nghiên cứu về lợi nhuận của phân tích kỹ thuật (Park & Irwin, 2007), 56 tích cực, 20 tiêu cực, 19 trái chiều — nhưng phần lớn có lỗi phương pháp.
tags: [trading, phan-tich-ky-thuat]
status: draft
aliases: [Phân tích kỹ thuật có hiệu quả không, Profitability of technical analysis]
sources:
  - title: "What Do We Know About the Profitability of Technical Analysis?"
    url: https://ideas.repec.org/a/bla/jecsur/v21y2007i4p786-826.html
    author: Cheol-Ho Park, Scott H. Irwin
    publisher: Journal of Economic Surveys, tập 21 số 4, tr. 786–826
    date: 2007-09
    kind: paper
    credibility: 5
    credibility_note: Bài tổng quan trên tạp chí có bình duyệt, tổng hợp 95 nghiên cứu và chỉ rõ lỗi phương pháp; đọc phần tóm tắt.
    accessed: 2026-11-25
  - title: "Foundations of Technical Analysis: Computational Algorithms, Statistical Inference, and Empirical Implementation"
    url: https://www.nber.org/papers/w7613
    author: Andrew W. Lo, Harry Mamaysky, Jiang Wang
    publisher: National Bureau of Economic Research — Working Paper 7613
    date: 2000-03
    kind: paper
    credibility: 5
    credibility_note: Nghiên cứu định lượng của các nhà kinh tế MIT trên cổ phiếu Mỹ 1962–1996, phương pháp công khai; đọc toàn văn bản NBER.
    accessed: 2026-11-25
  - title: "Support for Resistance: Technical Analysis and Intraday Exchange Rates"
    url: https://www.newyorkfed.org/research/epr/00v06n2/0007osle
    author: Carol L. Osler
    publisher: Federal Reserve Bank of New York — Economic Policy Review, tập 6 số 2
    date: 2000-07
    kind: paper
    credibility: 5
    credibility_note: Nghiên cứu có phản biện của Fed New York, dữ liệu thật và kiểm định thống kê; đọc toàn văn.
    accessed: 2026-11-25
  - title: "The predictive power of price patterns"
    url: https://ideas.repec.org/a/taf/apmtfi/v5y1998i3-4p181-205.html
    author: G. Caginalp, H. Laurent
    publisher: Applied Mathematical Finance, tập 5 số 3–4, tr. 181–205
    date: 1998
    kind: paper
    credibility: 4
    credibility_note: Tạp chí có bình duyệt; hạ 1 điểm vì chỉ đọc được phần tóm tắt và là nghiên cứu đơn lẻ.
    accessed: 2026-11-25
created: 2026-11-25
updated: 2026-11-25
research:
  keyword: Price action
  date: 2026-11-25
  agent: claude
---

## Dữ kiện

Park và Irwin (2007) tổng hợp 95 nghiên cứu "hiện đại" về khả năng sinh lời của các chiến lược [[phan-tich-ky-thuat|phân tích kỹ thuật]]: **56 nghiên cứu cho kết quả tích cực, 20 tiêu cực và 19 trái chiều** (Nguồn [1]). Cũng theo hai tác giả, phần lớn các nghiên cứu này có lỗi trong cách kiểm định, nên **chưa thể kết luận** phân tích kỹ thuật có sinh lời thật hay không (Nguồn [1]).

## Bối cảnh

### Tổng quan của Park & Irwin (2007)

- Các nghiên cứu "đời đầu" cho thấy chiến lược kỹ thuật có lãi trên thị trường ngoại hối và hợp đồng tương lai, nhưng **không có lãi trên thị trường cổ phiếu** (Nguồn [1]).
- Các nghiên cứu "hiện đại" cho thấy chiến lược kỹ thuật tạo lợi nhuận kinh tế đều đặn trên nhiều thị trường đầu cơ, **ít nhất đến đầu những năm 1990** (Nguồn [1]).
- Các lỗi phương pháp phổ biến (Nguồn [1]):
  - **Đào dữ liệu** (data snooping): thử rất nhiều quy tắc rồi chỉ báo cáo quy tắc tình cờ thắng.
  - **Chọn quy tắc sau khi đã biết kết quả.**
  - **Khó ước tính đúng rủi ro và chi phí giao dịch.**

### Các nghiên cứu khác đã đọc

| Nghiên cứu | Thị trường, giai đoạn | Kết quả chính |
| --- | --- | --- |
| Lo, Mamaysky & Wang (2000) | Cổ phiếu Mỹ, 1962–1996 | Dùng thuật toán tự nhận diện mẫu hình (vai–đầu–vai, hai đáy…). Một số mẫu hình mang thêm thông tin, nhất là với cổ phiếu Nasdaq. Tác giả nói rõ điều này **không có nghĩa** là chúng tạo được lợi nhuận vượt trội (Nguồn [2]). |
| Osler (2000) | Tỷ giá USD trong ngày, 1996–1998 | Mức [[ho-tro-va-khang-cu|hỗ trợ – kháng cự]] do 6 công ty công bố báo trước điểm dừng của xu hướng tốt hơn mức ngẫu nhiên. Nghiên cứu không kiểm tra lợi nhuận (Nguồn [3]). |
| Caginalp & Laurent (1998) | Cổ phiếu S&P 500, 1992–1996 | Mô hình [[bieu-do-nen-nhat|nến]] ba phiên có ý nghĩa thống kê ngoài mẫu, lợi nhuận gần 1% khi nắm giữ hai ngày (Nguồn [4]). |

### Vì sao price action khó kiểm chứng trực tiếp

Lo, Mamaysky và Wang cho rằng rào cản lớn nhất khi kiểm tra phân tích kỹ thuật là tính chủ quan: hình dạng trên biểu đồ thường "nằm trong mắt người xem" (Nguồn [2]). Điều này càng đúng với [[price-action]], vốn dựa vào việc người giao dịch tự đọc biểu đồ. Muốn kiểm tra, nhà nghiên cứu phải biến mẫu hình thành quy tắc máy tính (Nguồn [2]). Kết quả kiểm tra quy tắc đó chưa chắc phản ánh cách một người giao dịch cụ thể đọc biểu đồ.

## Cách dùng trong content

- **Nên viết:** "Kết quả nghiên cứu trái chiều. Một tổng quan 95 nghiên cứu thấy đa số có kết quả tích cực, nhưng phần lớn mắc lỗi phương pháp nên chưa có kết luận chắc chắn."
- **Không viết:** "Khoa học đã chứng minh phân tích kỹ thuật (hay price action) hiệu quả". Cũng không viết điều ngược lại, "đã chứng minh là vô dụng".
- Luôn ghi rõ **thị trường và giai đoạn** của từng nghiên cứu. Kết quả trên ngoại hối hay cổ phiếu Mỹ những năm 1990 không tự động đúng với thị trường Việt Nam bây giờ.
- Phân biệt **"có thông tin dự báo"** với **"có lãi sau chi phí"**. Hai điều này khác nhau (Nguồn [2][3]).

## Câu hỏi thường gặp

### Phân tích kỹ thuật có hiệu quả không?

Chưa có kết luận chắc chắn. Trong 95 nghiên cứu được Park và Irwin tổng hợp, 56 cho kết quả tích cực, 20 tiêu cực và 19 trái chiều. Phần lớn các nghiên cứu có lỗi như đào dữ liệu hoặc chưa tính đủ chi phí giao dịch (Nguồn [1]).

### Price action đã được nghiên cứu khoa học kiểm chứng chưa?

Có nghiên cứu về từng thành phần như hỗ trợ – kháng cự (Nguồn [3]) và mô hình nến (Nguồn [4]). Còn price action theo kiểu người giao dịch tự đọc biểu đồ thì rất khó kiểm chứng trực tiếp vì quá chủ quan (Nguồn [2]).

## Liên quan

- [[price-action]] — chủ đề trụ cột
- [[ho-tro-va-khang-cu]]
- [[bieu-do-nen-nhat]]
- [[ty-le-nha-giao-dich-ca-nhan-thua-lo]]
