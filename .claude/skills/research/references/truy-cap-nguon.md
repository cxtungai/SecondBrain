# Đọc nguồn khó truy cập

Rút từ đợt nghiên cứu Price action (2026-11-25): phiên cloud chỉ đọc được phần tóm tắt nhiều nghiên cứu, còn trình duyệt trên máy đọc được toàn văn.

## Thứ tự thử

1. **WebFetch** (hoặc agent `web-fetch`): nhanh, dùng trước cho trang HTML.
2. **Bản khác của cùng tài liệu** (thường bị bỏ sót):
   - working paper trên trang của tác giả (vd. trang giảng viên), NBER, SSRN, arXiv, RePEc/IDEAS;
   - kho lưu trữ của trường (luận án thường là bản đầy đủ của bài tạp chí);
   - báo cáo của chính cơ quan (PDF trên trang `.gov`, `.europa.eu`…);
   - tạp chí truy cập mở (giấy phép CC BY).
   Ghi rõ đã đọc bản nào. Bản trước khi bình duyệt có thể khác số liệu.
3. **Trình duyệt trên máy** (`mcp__Claude_Browser__*`, chỉ có khi chạy trong app desktop):
   - Trang HTML: `navigate` rồi `get_page_text`.
   - PDF: trình xem PDF không cho lấy chữ. Mở một trang **cùng tên miền** với file PDF, rồi dùng `javascript_tool` để `fetch` file PDF trong trang, giải nén các luồng `FlateDecode` bằng `DecompressionStream`, và lấy chữ từ các lệnh `Tj`/`TJ`. Chỉ đưa ra **đoạn cần thiết** (tìm theo từ khoá), không đưa cả file.
   - File tải xuống (`Content-Disposition: attachment`) sẽ bật hộp lưu file trên máy người dùng: **đừng mở lại**, báo người dùng bấm Huỷ.
4. **Không đọc được:** dùng phần tóm tắt, ghi "chỉ đọc tóm tắt" trong `credibility_note` và đưa vào "Câu hỏi còn mở". Không trích chi tiết không có trong tóm tắt.

## Không được làm

- Không vượt qua lớp kiểm tra chống máy tự động (CAPTCHA, Cloudflare "Just a moment…", Anubis "Making sure you're not a bot"), tường trả phí hay trang đăng nhập. Gặp thì chuyển sang bản khác hoặc dùng tóm tắt.
- Không giả danh trình duyệt (đổi User-Agent) để tải bằng `curl`. Hệ thống sẽ chặn, và làm vậy là lách chặn.
- Không lưu PDF hay bản sao trang vào repo. File tạm đặt trong thư mục nháp.
- Không bấm "Chấp nhận" cookie; nếu cần thì chọn từ chối cookie không cần thiết.
- Không đăng nhập thay người dùng.

## Ghi chú về chất lượng chữ lấy từ PDF

- Chữ có thể dính liền, mất dấu gạch nối, ký tự lạ thay cho chữ ghép (fi, fl). Kiểm lại số liệu bằng hai đoạn khác nhau trong bài (tóm tắt và kết luận) trước khi dùng.
- Số trong bảng thường lộn thứ tự. Ưu tiên số được nhắc trong câu văn.
