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

## 2026-11-26 — Thứ tự đọc và đọc sách gốc
- Ghi chú `source` được bài chủ đề nhắc tới sớm (trong bảng nguồn gốc) nhảy lên "Bài con 1/8". Thứ tự đọc lấy theo lần nhắc đầu tiên, nhưng `source` phải xếp cuối (`Kb::reading_order`). Bằng chứng: dist/duyet/index.html trước và sau khi sửa.
- Bản quét archive.org có thể là bản in khác năm xuất bản đầu (cu31924031269552: metadata ghi 1922, trang tên sách ghi bản in lần 5, 1925). Ghi rõ bản in và lấy số trang theo bản đã đọc.
- Bản chữ djvu.txt có tiêu đề chạy đầu trang kèm số trang: đổi thành mốc `⟦tr.N⟧` (scratchpad/tools/reflow.mjs) để trích dẫn có trang; trang mở đầu chương có số ở chân trang.

## 2026-11-26 — SEO kỹ thuật trên GitHub Pages dạng thư mục con
- Web ở `cxtungai.github.io/SecondBrain/` thì bot chỉ đọc `cxtungai.github.io/robots.txt` (đang 404), nên `Disallow` và dòng `Sitemap` trong `/SecondBrain/robots.txt` không có tác dụng. Bản nháp vẫn an toàn nhờ thẻ `noindex`; sitemap phải gửi qua Search Console / Bing Webmaster. Bằng chứng: curl trả 404 cho `/robots.txt`, 200 cho `/SecondBrain/robots.txt`.
- Thẻ canonical có sẵn trong `templates/base.html` nhưng chương trình chưa từng điền (trang thật không có canonical). Giờ `render_template` tự điền theo đường dẫn đang dựng (`CURRENT_ROUTE`), bỏ qua bản trên máy, Khu duyệt bài, trang noindex và 404. Kiểm bằng `scratchpad/check-seo.mjs`: 36 trang, mọi trang công khai có canonical, JSON-LD đều hợp lệ.
- Dựng thử trong Git Bash: `BASE_PATH=/SecondBrain` bị đổi thành `C:/Program Files/Git/SecondBrain`. Dùng `MSYS_NO_PATHCONV=1` và đưa `--out` dạng Windows (`cygpath -w`), nếu không bản dựng rơi vào `C:\c\Users\…`.
- Ảnh PNG (ảnh chia sẻ 1200×630, logo) vẽ bằng Edge không giao diện: `msedge --headless=new --screenshot=… --window-size=1200,630 file:///…html`. Không cần thêm thư viện vào chương trình.
- Script Node gọi `process.exit()` khi `fetch` còn giữ kết nối thì trên Windows báo `Assertion failed: !(handle->flags & UV_HANDLE_CLOSING)` và thoát mã 127, dù việc kiểm đã xong. Dùng `process.exitCode = …` rồi để script tự kết thúc. Bằng chứng: lần chạy đầu của `.github/scripts/kiem-tra-web.mjs` trên máy (2026-11-26).
- IndexNow chỉ nhận URL nằm dưới thư mục chứa file khoá (`keyLocation`). Web ở `/SecondBrain/` thì đặt `<khoá>.txt` ngay trong `/SecondBrain/`. Lần đăng đầu sau khi thêm khoá, sitemap không đổi nên không có URL nào để báo; phải gửi tay một lần để Bing biết toàn bộ web.
- Chụp trang bằng Edge không giao diện thì không ghi được vào thư mục scratchpad (`Failed to write file …: Access is denied`). Ghi vào `target/` của repo (đã bỏ khỏi git) bằng đường dẫn Windows đầy đủ: `--screenshot='C:\…\target\shot.png'`. Bằng chứng: chụp trang Price action ngày 2026-11-26.
- `cargo build --release` báo `Access is denied (os error 5)` khi Bàn làm việc (`target/release/secondbrain.exe mo`) đang chạy, vì Windows khoá file exe. Dừng máy chủ xem trước rồi mới build lại.
- Chụp trang Bàn làm việc bằng Edge không giao diện mà không có `--virtual-time-budget` (và có `#mục` trong URL) thì ra ảnh trắng (6,6 KB). Thêm `--virtual-time-budget=4000`, bỏ `#mục` thì chụp được; trang dài hơn `--window-size` thì phần dưới bị cắt, nên để xem một mục ở cuối trang dùng khung trình duyệt (resize_window + cuộn + zoom). Bằng chứng: chụp mục Bài tập ngày 2026-11-26.
