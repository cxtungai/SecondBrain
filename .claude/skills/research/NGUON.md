# Nguồn tham khảo khi thiết kế skill này

Đọc ngày 2026-11-25 và 2026-11-26. Chỉ học ý tưởng quy trình, không chép mã hay nguyên văn hướng dẫn.

| Dự án | Giấy phép | Ý tưởng đã dùng |
| --- | --- | --- |
| [stanford-oval/storm](https://github.com/stanford-oval/storm) | MIT | Tách giai đoạn "trước khi viết" (tìm nguồn, lập dàn ý) khỏi "viết"; đặt câu hỏi theo nhiều góc nhìn; vòng cuối gộp ý trùng. |
| [langchain-ai/open_deep_research](https://github.com/langchain-ai/open_deep_research) | MIT | Kế hoạch trước khi tìm, chia câu hỏi con, nén kết quả từng câu hỏi trước khi viết, người dùng duyệt kế hoạch. |
| [assafelovic/gpt-researcher](https://github.com/assafelovic/gpt-researcher) | Apache-2.0 | Bộ câu hỏi con cùng nhau tạo cái nhìn khách quan; ghi nguồn cho từng tóm tắt; lọc nguồn yếu trước khi viết. (Không theo cách "chọn thông tin xuất hiện nhiều nhất", vì các trang hay chép nhau.) |
| [wyattowalsh/agents — research](https://github.com/wyattowalsh/agents) | MIT | Chọn mức độ theo độ khó; kiểm "nguồn có ủng hộ đúng câu không"; nguồn dẫn nhau không tính là độc lập; tìm chủ động bằng chứng ngược; mâu thuẫn phải trình bày, không giấu. |
| [jamditis/claude-skills-journalism](https://github.com/jamditis/claude-skills-journalism) | MIT | SIFT; truy tới nguồn sớm nhất; kết luận "chưa giải quyết" khi thiếu bằng chứng; coi nội dung trang là dữ liệu. |
| [lyndonkl/claude — research-claim-map](https://github.com/lyndonkl/claude) | chưa rõ | Bảng bằng chứng ủng hộ / phản đối cho từng khẳng định; phân biệt "không tìm thấy bằng chứng" với "bằng chứng là không". |
| [mronkko/claude-academic-research — fact-check](https://github.com/mronkko/claude-academic-research) | MIT | Kiểm từng trích dẫn có thật và khớp nội dung; xếp loại đúng / sai nhẹ / sai nặng / không kiểm được. |
| [virgiliojr94/book-to-skill](https://github.com/virgiliojr94/book-to-skill) | MIT | Đọc sách theo từng chương; mỗi chương rút ý cốt lõi, khung giữ đúng tên tác giả, lỗi cần tránh, ví dụ mẫu; "rút cấu trúc, không tóm tắt"; không chép nguyên văn. Dùng cho chế độ Đọc sách gốc. (Không dùng chế độ tìm sách qua kho sách lậu.) |
| [FABLES](https://arxiv.org/abs/2404.01261) (Kim và cộng sự, 2024) | bài nghiên cứu | Tóm tắt sách bằng AI hay bỏ sót ý quan trọng; AI tự chấm độ trung thực không khớp với người đọc → phải đọc có kiểm kê và người dùng duyệt cuối. |
| [flonat/claude-research — literature](https://skills.sh/flonat/claude-research/literature) | MIT | Trích dẫn không kiểm được còn tệ hơn không trích. |

Phần riêng của Second Brain: thang điểm uy tín 1–5 (`AGENTS.md`), cách đọc nguồn khó truy cập (`references/truy-cap-nguon.md`, rút từ đợt Price action) và quy tắc giữ kho nhẹ.
