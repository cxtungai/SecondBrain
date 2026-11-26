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
| [GEO](https://arxiv.org/abs/2311.09735) (Aggarwal và cộng sự, KDD 2024) | bài nghiên cứu (mã Apache-2.0) | Dẫn nguồn, câu trích, số liệu giúp AI nhắc tới nhiều hơn; nhồi từ khoá gần như vô ích. Dùng trong `references/geo.md`. |
| [cxcscmu/AutoGEO](https://github.com/cxcscmu/AutoGEO) (ICLR 2026) | MIT | Quy tắc "AI tìm kiếm thích gì" rút từ Gemini, GPT, Claude: kết luận trước, đoạn tự đứng được, cân bằng, định nghĩa thuật ngữ, có ngày. |
| [zubair-trabzada/geo-seo-claude — geo-citability](https://github.com/zubair-trabzada/geo-seo-claude) | MIT | Thang "dễ được trích": câu trả lời đầu mục, đoạn tự đứng được, cấu trúc, mật độ bằng chứng, nội dung độc đáo. Rút gọn thành danh sách tự kiểm và gợi ý trong `./sb seo`. |
| [coreyhaines31/marketingskills — ai-seo](https://github.com/coreyhaines31/marketingskills) | MIT | Cách từng AI chọn nguồn; việc không được làm (viết riêng cho AI, cắt vụn bài, trích dẫn giả). |
| [AgriciDaniel/claude-seo — seo-agentic](https://github.com/AgriciDaniel/claude-seo) | MIT | robots.txt theo RFC 9309 (nhóm bot riêng phải lặp đủ quy tắc), danh sách bot AI theo mục đích, llms.txt, bản Markdown. Không cài cả bộ (cần Python + Chromium, nhiều phần trả phí). |

Phần riêng của Second Brain: thang điểm uy tín 1–5 (`AGENTS.md`), cách đọc nguồn khó truy cập (`references/truy-cap-nguon.md`, rút từ đợt Price action) và quy tắc giữ kho nhẹ.
