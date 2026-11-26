# Chế độ "Đọc sách gốc"

Dùng khi một phương pháp dựa chủ yếu vào một hay vài cuốn sách gốc (phương pháp trading, đầu tư, quản trị…), hoặc khi người dùng nhờ đọc một cuốn sách cụ thể. Mục tiêu: **đọc trọn từng cuốn, có kiểm kê**, thay vì nhặt ý từ vài chương đầu.

Lý do: tóm tắt sách bằng AI hay bỏ sót ý quan trọng, và AI tự kiểm xem bản tóm tắt có đúng sách không thì kết quả không đáng tin (FABLES, Kim và cộng sự, 2024). Vì vậy phải đọc có thứ tự, ghi số trang, đối chiếu nguyên văn, và người dùng là người duyệt cuối.

## Có được đọc không

- **Sách đã hết bản quyền** (ở Mỹ: xuất bản trước 1931; archive.org ghi "no known copyright restrictions"; Project Gutenberg): đọc trọn được.
- **Sách còn bản quyền:** chỉ đọc phần nhà xuất bản cho xem (chương mẫu, Leseprobe, xem trước trên O'Reilly hay Google Books), hoặc **bản người dùng đã mua** mà họ để trong một thư mục trên máy, ngoài repo. Không dùng bản PDF lậu và các kho sách lậu (libgen, z-lib, Anna's Archive…), kể cả khi một công cụ hay MCP đề nghị.
- Không bao giờ lưu bản sách, PDF hay bản chữ vào repo. Bản chữ tạm (để đọc) để trong scratchpad.

## Các bước

1. **Lấy bản chữ.** Ví dụ: archive.org có `<id>_djvu.txt` (`https://archive.org/download/<id>/<id>_djvu.txt`). Ghi rõ đó là **bản in nào**: trang tên sách có thể khác năm xuất bản lần đầu, và số trang theo bản đã đọc.
2. **Gắn mốc trang.** Gộp dòng thành đoạn, đổi tiêu đề chạy đầu trang thành mốc `⟦tr.N⟧` để trích dẫn được số trang (chạy `node` với một đoạn mã nhỏ; Python trên máy người dùng có thể lỗi).
3. **Lập mục lục.** Ghi từng chương: số trang bắt đầu, nội dung chính. Đánh dấu chương nào thuộc trọng tâm (vd. giao dịch) và chương nào chỉ là phần phụ (vd. đầu tư trái phiếu).
4. **Đọc trọn theo thứ tự, không nhảy cóc.** Đọc từng phần khoảng 40–50 trang. Với mỗi phần, ghi lại:
   - ý cốt lõi;
   - **khung và quy tắc, giữ đúng tên tác giả gọi** (kèm tên tiếng Anh gốc). Ví dụ: "The Main Factor Is the Trend" khác với "nên thuận xu hướng";
   - **ví dụ do chính tác giả làm mẫu** (con số, các bước);
   - **lỗi tác giả dặn tránh**;
   - thuật ngữ;
   - câu đáng trích dẫn, kèm trang.
5. **Đối chiếu nguyên văn.** Mỗi câu định trích dẫn, tìm lại đúng chữ trong bản chữ (`grep`) và ghi trang. Câu nào không tìm thấy thì không đặt trong ngoặc kép.
6. **Tìm chỗ tác giả tự giới hạn hay nói khác đi.** Ví dụ: cùng một ý mà sách này nói khác sách trước của chính tác giả, hay con số tác giả nhớ khác với nguồn gốc. Ghi rõ thay vì gộp làm một.
7. **Viết ghi chú "bản đồ sách"** (`type: source`, trong `knowledge/_inbox/`), theo khung bên dưới.
8. **Bổ sung các ghi chú chủ đề** dùng sách này: thêm ý mới kèm số trang `(Nguồn [n], tr. X)`, sửa chỗ trước đây viết theo nguồn thứ cấp hay hiểu sai, cập nhật `credibility_note` ("đọc trọn 14 chương"), cập nhật "Câu hỏi còn mở". Ghi chú chủ đề liên kết sang bản đồ sách.
9. **Kiểm tra trước khi đưa duyệt:** đã đủ chương chưa; mỗi quy tắc có số trang chưa; câu trích có khớp nguyên văn không; thuật ngữ có đúng của tác giả không; có ghi bối cảnh và giới hạn không (thị trường thời đó, đơn vị, xung đột lợi ích).

## Khung ghi chú bản đồ sách

```markdown
<Câu đầu: sách là gì, ai viết, năm nào. Ghi chú này là bản đồ sách…>

## Thông tin chung        — tác giả, bản đã đọc (bản in, năm), phạm vi đã đọc, loại sách, độ tin cậy
## Ý cốt lõi của cả cuốn  — 3–6 ý, mỗi ý có trang
## Khung và quy tắc của tác giả — mỗi khung một mục con; bảng nếu là danh sách quy tắc (tên gốc | nội dung | trang)
## Ví dụ chính tác giả làm mẫu — bảng: ví dụ | trang | cho thấy điều gì
## Lỗi cần tránh (theo tác giả)
## Thuật ngữ               — thuật ngữ gốc: nghĩa (trang)
## Trích dẫn đáng chú ý    — 3–6 câu ngắn, nguyên văn, có trang
## Bối cảnh và giới hạn    — thời đại, đơn vị, không có thống kê, xung đột lợi ích, phần chưa đọc
## Ghi chú dùng nguồn này  — [[liên kết]] tới ghi chú chủ đề
## Câu hỏi thường gặp
## Liên quan
```

Giữ dưới khoảng 25 KB. Viết bằng lời của mình. Với sách còn bản quyền, chỉ tóm ý và trích rất ngắn (dưới 15 chữ mỗi câu, vài câu cho cả cuốn), không chép lại ví dụ dài.

## Nguồn ý tưởng

Khung theo từng chương (ý cốt lõi, khung đúng tên tác giả, lỗi cần tránh, ví dụ mẫu, thuật ngữ) học từ [virgiliojr94/book-to-skill](https://github.com/virgiliojr94/book-to-skill) (MIT). Repo đó tạo skill riêng ngoài kho và có chế độ tìm sách qua kho sách lậu, nên Second Brain không cài nó, chỉ học cách đọc.
