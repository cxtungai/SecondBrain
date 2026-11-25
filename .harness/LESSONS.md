# Bài học (có bằng chứng)

Mỗi mục: lỗi gặp phải → nguyên nhân gốc → cách sửa → bằng chứng (lệnh/kết quả/đường dẫn). Chỉ ghi bài học đã chứng minh và dùng lại được.

## Nguồn PDF bị chặn công cụ tự động: đọc trong trình duyệt trên máy

- **Lỗi:** phiên cloud và WebFetch chỉ đọc được phần tóm tắt nhiều nghiên cứu (403, kết nối bị ngắt, PDF nén không đọc được). Lệnh `curl` giả User-Agent trình duyệt thì bị hệ thống quyền từ chối vì là lách chặn.
- **Nguyên nhân gốc:** nhiều kho tài liệu chặn yêu cầu từ máy chủ; trình xem PDF của trình duyệt không cho lấy chữ.
- **Cách sửa:** dùng trình duyệt trong app desktop: mở trang cùng tên miền, `fetch` file PDF trong trang, giải nén `FlateDecode` bằng `DecompressionStream`, lấy chữ từ `Tj`/`TJ` (cách làm chi tiết trong `.claude/skills/research/references/truy-cap-nguon.md`). Trang có CAPTCHA/Cloudflare/Anubis thì bỏ, tìm bản khác.
- **Bằng chứng:** task `secondbrain-20261125073845-8af29140`, ngày 2026-11-25: đọc được toàn văn Park & Irwin 2004 (ageconsearch), Chague & Giovannetti 2025 (periodicos.fgv.br), Barber và cộng sự 2014 (trang của Odean), Ben-David và cộng sự (NBER); kho FGV (Anubis) và OUP (Cloudflare) đã được bỏ qua.

## Chạy thử trên bản sao: không dùng chung thư mục build Rust với repo thật

- **Lỗi:** chạy thử template hỏng trên bản sao (`git clone` vào scratchpad) với `CARGO_TARGET_DIR` trỏ về `target/` của repo thật. Sau đó `./sb build` trong repo thật vẫn lỗi `unknown statement endblok`, dù `templates/about.html` của repo không hề đổi.
- **Nguyên nhân gốc:** template được nhúng vào chương trình lúc build (`include_str!`). Cargo coi bản build từ bản sao là còn mới, nên repo thật chạy lại đúng file chạy có template hỏng.
- **Cách sửa:** bản sao dùng thư mục build riêng (để mặc định, hoặc `CARGO_TARGET_DIR` riêng). Lỡ dùng chung rồi thì `cargo clean --release -p secondbrain` rồi build lại.
- **Bằng chứng:** task `secondbrain-20261125064153-667cce78`, ngày 2026-11-25. Lần `verify` đầu báo `./sb build` lỗi (panic `src/site.rs:107`); sau `cargo clean -p secondbrain` thì cả 4 bước kiểm tra đều đạt.
