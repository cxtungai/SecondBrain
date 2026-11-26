# Đo lường hằng tháng

Mục đích: biết điều gì hiệu quả để chỉnh, thay vì đoán. Làm mỗi tháng một lần, bắt đầu khoảng 2–4 tuần sau khi đăng ký. Trước thời điểm đó các công cụ chưa có số liệu.

## 1. Google Search Console (người dùng mở, chụp màn hình hoặc xuất số liệu cho agent)

- **Hiệu suất:** lượt hiển thị, lượt nhấp, các truy vấn đã đưa người tới web, trang nào được tìm thấy. Truy vấn có nhiều lượt hiển thị mà ít nhấp gợi ý nên viết rõ hơn tiêu đề hoặc tóm tắt, **không đổi nội dung tri thức**.
- **Trang:** bao nhiêu trang đã được lập chỉ mục, trang nào bị loại và vì sao.
- **Sơ đồ trang web:** trạng thái phải là "Thành công".
- **Generative AI performance** (báo cáo hiệu suất AI tạo sinh): web xuất hiện thế nào trong AI Overviews, AI Mode và Discover. Đây là số đo GEO của Google. Lượt từ AI Overviews và AI Mode cũng được tính chung vào báo cáo Hiệu suất, loại "Web".

## 2. Bing Webmaster Tools

- **Search Performance:** giống Google.
- **AI Performance (BETA):** web được Copilot và câu trả lời AI của Bing trích ở đâu, trang nào. Đây là số đo GEO trực tiếp.
- **IndexNow:** các trang đã được báo.

## 3. Hỏi lại các AI (agent làm được một phần)

Bộ 5–10 câu hỏi chính, mỗi chủ đề đã đăng một câu, vd. "price action là gì", "cách xác định vùng hỗ trợ kháng cự". Ghi bộ câu hỏi vào `trang-thai.md` để tháng nào cũng hỏi đúng bộ đó.

- Agent dùng công cụ tìm kiếm web của mình để xem web có hiện trong kết quả không.
- Người dùng tự hỏi ChatGPT, Perplexity, Gemini, Google (chế độ AI), Copilot, rồi ghi lại 3 điều: có trích web không, trích trang nào, trích đúng hay sai.
- Trích sai thì xem lại đoạn bị trích. Thường đó là đoạn không tự đứng được: mở bằng "Nó…", thiếu tên chủ thể, hoặc thiếu điều kiện.

## 4. Ghi kết quả và chọn việc

Thêm một mục vào `trang-thai.md` theo mẫu:

```
### Tháng MM/YYYY
- Google: X trang đã lập chỉ mục, Y lượt hiển thị, Z lượt nhấp; truy vấn nổi bật: …
- Bing: … ; AI Performance: …
- AI trích web: k/n câu hỏi (đúng/sai: …)
- Việc tháng sau (1–3 việc): …
```

Việc tháng sau ưu tiên theo thứ tự:

1. duyệt thêm ghi chú trụ cột;
2. viết bài SEO cho truy vấn đã có lượt hiển thị;
3. sửa trình bày cho đoạn bị AI trích sai;
4. tìm nơi chia sẻ để có uy tín bên ngoài.

Điều gì rút ra được cho mọi trang thì ghi vào `NHAT-KY.md` và sửa quy trình.
