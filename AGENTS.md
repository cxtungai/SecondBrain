# Hướng dẫn cho AI agent — Second Brain

Second Brain là kho tri thức chuyên môn **đã được người dùng đọc, hiểu và duyệt**, đăng thành website và xuất thành dữ liệu cho AI agent viết content. Mọi tri thức nằm trong `knowledge/` dưới dạng Markdown, liên kết với nhau bằng `[[wikilink]]`.

Nguyên tắc cốt lõi: **AI nghiên cứu và đề xuất, người dùng duyệt, chỉ tri thức đã duyệt mới lên web.** Không bao giờ tự đăng hay tự đổi `status` thành `verified` khi người dùng chưa duyệt. Bản nháp chỉ hiện ở **Khu duyệt bài** (`/duyet/` trên web): `noindex`, chặn trong `robots.txt`, không vào sitemap, `llms.txt`, `knowledge.json`, tìm kiếm hay đồ thị công khai.

**Người dùng không rành kỹ thuật.** Họ làm việc qua: nói chuyện với bạn (Claude) bằng tiếng Việt, và duyệt bài ở một trong hai nơi:

- **Khu duyệt bài trên web** — `<địa chỉ web>/duyet/` (vd. https://cxtungai.github.io/SecondBrain/duyet/), mở được trên mọi máy và điện thoại. Mỗi bản nháp có nút **✅ Duyệt & đăng**, **💬 Nhờ Claude sửa** (chép sẵn lời nhờ), **✏️ Tự sửa trên GitHub**, **🗑 Bỏ**. Duyệt/Bỏ mở trang tạo issue GitHub có tiêu đề điền sẵn (`Duyệt: <file>`, `Bỏ: <file>`, `Duyệt tất cả bản nháp`). Người dùng bấm **Create**, rồi workflow `.github/workflows/duyet.yml` (chỉ chạy khi chủ repo tạo issue) đăng hoặc bỏ bài, đẩy lên nhánh mặc định, chạy lại web và trả lời trong issue. Người dùng đang dùng web thì chỉ họ tới đây.
- **Bàn làm việc** — trang chạy trên máy họ (bấm đúp `Mo-Second-Brain.bat` trên Windows / `Mo-Second-Brain.command` trên macOS): đọc bản nháp, sửa bằng form, bấm **Duyệt & đăng** và **Đưa lên web**.

Đừng yêu cầu họ gõ lệnh hay sửa YAML; bạn chạy lệnh thay họ và giải thích kết quả bằng lời thường. Bản nháp làm xong phải được đẩy lên nhánh mặc định thì mới hiện ở Khu duyệt bài.

Site có hai lớp nội dung: **tri thức** (`knowledge/`, nguồn sự thật) và **bài viết SEO/GEO** (`articles/`, lớp trình bày để kéo traffic, dẫn xuất từ tri thức). Xem mục "Bài viết SEO/GEO".

## Cấu trúc thư mục

```
knowledge/              Kho tri thức (mở được bằng Obsidian)
  _inbox/               Bản nháp AI nghiên cứu, chờ duyệt — KHÔNG lên web
  _templates/           Template cho từng loại ghi chú
  topics/               Chủ đề trụ cột (bản đồ nội dung)
  concepts/             Khái niệm
  facts/                Số liệu & dữ kiện
  frameworks/           Mô hình, phương pháp, quy trình, setup
  case-studies/         Case study
  sources/              Tóm tắt nguồn tham khảo
articles/               Bài viết SEO/GEO dẫn xuất từ tri thức (URL /bai-viet/<slug>/)
minh-hoa/               Hình minh hoạ chỉ dùng trên web (hình nến vẽ từ chữ) — không thuộc kho tri thức
src/                    Chương trình Rust `secondbrain`: lõi tri thức, sinh web tĩnh, Bàn làm việc, CLI
templates/  assets/     Giao diện web (minijinja) + CSS/JS — được nhúng vào chương trình
cau-hinh.yml            Tên, mô tả, tác giả, miễn trừ, quy tắc cho AI agent
sb                      Lệnh chạy chương trình cho agent: ./sb <lệnh> (cargo nếu có, không thì tải bản build sẵn)
Mo-Second-Brain.*       File bấm đúp cho người dùng: tải chương trình lần đầu rồi mở Bàn làm việc
.github/workflows/      deploy.yml (test + build + GitHub Pages + kiểm tra web thật + báo IndexNow), release.yml (file chạy), duyet.yml (nút duyệt trên web)
.github/scripts/duyet.sh  Đăng/bỏ bản nháp theo tiêu đề issue (chạy thử được trên máy, xem đầu file)
.github/scripts/kiem-tra-web.mjs, indexnow.mjs  Sau mỗi lần đăng: kiểm tra SEO của web thật; báo trang mới/sửa cho IndexNow
.claude/skills/         Skill cho Claude Code: research, publish, connect, seo-article, write-content
```

## Ghi chú

Mỗi ghi chú là một file `.md`. **Tên file là slug** (không dấu, chữ thường, nối bằng `-`) và phải duy nhất trong toàn kho.

```yaml
---
title: Price Action               # bắt buộc
type: topic                       # bắt buộc: topic | concept | fact | framework | case-study | source
summary: Price action là …        # bắt buộc, 1–2 câu — AI dùng để chọn ghi chú, Google dùng làm mô tả
tags: [trading, phan-tich-ky-thuat]
status: draft                     # draft | verified | outdated
aliases: [PA, Hành động giá]      # tên khác — dùng để tra từ điển và dò liên kết
sources:
  - title: Tên tài liệu
    url: https://…
    author: Tác giả
    publisher: Nhà xuất bản / tổ chức
    date: 2019                    # năm hoặc ngày phát hành
    kind: book                    # paper | official | data | book | media | education | expert | blog | video | forum | other
    credibility: 4                # 1–5, theo thang bên dưới
    credibility_note: Sách chuyên môn của tác giả có kinh nghiệm thực tế, có dẫn nguồn
    accessed: 2026-11-25
created: 2026-11-25
updated: 2026-11-25
reviewed: 2026-11-25              # ngày người dùng duyệt — `./sb publish` tự điền
research:                         # tuỳ chọn — do agent nghiên cứu điền
  keyword: Price action
  date: 2026-11-25
  agent: claude
---
```

| Loại | Dùng cho |
| --- | --- |
| `topic` | Chủ đề trụ cột: giới thiệu ngắn + dẫn tới các ghi chú con, nhóm theo loại. |
| `concept` | Một khái niệm/thuật ngữ. Câu đầu trả lời thẳng "X là gì?". |
| `fact` | Số liệu, dữ kiện có thể trích dẫn. **Bắt buộc có nguồn.** |
| `framework` | Mô hình, phương pháp, quy trình, setup, checklist. |
| `case-study` | Ví dụ thực tế: bối cảnh, cách làm, kết quả, bài học. |
| `source` | Tóm tắt một tài liệu (sách, báo cáo, video…). |

| Trạng thái | Ý nghĩa |
| --- | --- |
| `draft` | Chưa kiểm chứng — chỉ dùng làm ý tưởng, không trích dẫn như sự thật. |
| `verified` | Người dùng đã duyệt — được dùng và trích dẫn. |
| `outdated` | Lỗi thời — không dùng cho content mới. |

Nội dung bài: trích nguồn bằng số thứ tự trong `sources`, ví dụ `(Nguồn [2])`. Kết bài bằng `## Câu hỏi thường gặp` (mỗi câu hỏi là `### …?`) — site tự sinh dữ liệu FAQ cho Google/AI — và `## Liên quan`.

## Thang điểm uy tín nguồn

Mỗi nguồn được chấm 1–5. Xuất phát từ mức của loại nguồn, rồi điều chỉnh ±1 theo 5 tiêu chí:

- **Thẩm quyền** — ai nói? tác giả/tổ chức có chuyên môn và trách nhiệm với thông tin không?
- **Bằng chứng** — có dữ liệu, phương pháp, dẫn nguồn không?
- **Độc lập** — có bán khoá học/sản phẩm/tín hiệu, hay xung đột lợi ích không?
- **Thời điểm** — còn mới so với tốc độ thay đổi của chủ đề không?
- **Đối chiếu** — các nguồn độc lập khác có xác nhận không?

| Điểm | Mức | Ví dụ |
| --- | --- | --- |
| 5 | Rất cao | Nghiên cứu có bình duyệt, cơ quan quản lý, sở giao dịch, dữ liệu sơ cấp, tài liệu gốc của người đặt ra khái niệm |
| 4 | Cao | Sách chuyên môn của tác giả có uy tín, báo chí lớn, tổ chức nghề nghiệp/đào tạo có tên tuổi — có biên tập và dẫn nguồn |
| 3 | Trung bình | Chuyên gia/người thực hành có danh tiếng và thành tích kiểm chứng được; trang kiến thức chuyên ngành có biên tập |
| 2 | Thấp | Blog cá nhân, video, khoá học chưa rõ kiểm chứng; ý kiến ít dẫn chứng |
| 1 | Rất thấp | Diễn đàn, mạng xã hội, nguồn ẩn danh, nội dung quảng cáo hoặc có xung đột lợi ích rõ ràng |

Quy tắc dùng điểm: khẳng định quan trọng cần ≥ 2 nguồn độc lập hoặc 1 nguồn điểm 5; nguồn điểm 1 không làm căn cứ cho khẳng định; ghi chú `fact` ở trạng thái `verified` cần ít nhất một nguồn ≥ 3 (`./sb check` sẽ cảnh báo).

## Liên kết tri thức

- `[[slug]]`, `[[slug|nhãn hiển thị]]`, `[[slug#tiêu-đề]]`. Cũng có thể viết `[[Tiêu đề]]` hoặc `[[alias]]` — hệ thống tự tìm ghi chú.
- Link lần xuất hiện đầu tiên của mỗi thuật ngữ đã có ghi chú. Không dùng link Markdown thường cho ghi chú nội bộ.
- `[[thuật-ngữ-chưa-có]]` là hợp lệ: nó hiện trong trang **Tri thức còn thiếu** (`/gaps/`) — hàng đợi nghiên cứu.
- Mỗi ghi chú mới phải được ít nhất một ghi chú `topic` liên kết tới, và nên có mục `## Liên quan`.
- Backlinks, đồ thị, breadcrumb, "Đọc tiếp" đều được sinh tự động từ liên kết.

## Bài viết SEO/GEO

Mục đích chính của site là tri thức. Bài viết là **bài con** kéo traffic từ Google / AI search về tri thức — không thay thế, không làm sai lệch tri thức.

| | Ghi chú tri thức (`knowledge/`) | Bài viết (`articles/`) |
| --- | --- | --- |
| Vai trò | Nguồn sự thật, nguyên tử, có nguồn + điểm uy tín | Trình bày lại theo câu hỏi người đọc hay tìm |
| Viết cho | Chính xác, dễ trích dẫn (người, AI agent, AI search) | Ý định tìm kiếm: hướng dẫn, so sánh, cẩm nang, tổng hợp |
| Dữ kiện | Từ nguồn bên ngoài, có `sources` | **Chỉ từ ghi chú gốc** (`based_on`), dẫn `[[liên kết]]` về ghi chú |
| Từ khoá | Truy vấn định nghĩa ("X là gì") — câu đầu trả lời thẳng | Truy vấn khác; mỗi từ khoá chính chỉ **một** trang |
| Đăng khi | Người dùng duyệt (`verified`) | Người dùng duyệt **và** mọi ghi chú gốc đã `verified` |

```yaml
---
title: Cách vẽ vùng hỗ trợ và kháng cự cho người mới   # ≤ 60 ký tự, có từ khoá
description: Hướng dẫn từng bước …                    # 70–160 ký tự — mô tả trên Google
keyword: cách vẽ hỗ trợ kháng cự                     # từ khoá chính
keywords: [vẽ vùng hỗ trợ, support resistance]        # từ khoá phụ
intent: how-to                                        # guide | how-to | compare | list | explain
based_on: [ho-tro-va-khang-cu, price-action]          # ghi chú tri thức gốc (slug), bắt buộc
status: draft                                         # draft | published
created: 2026-11-25
updated: 2026-11-25
published_on: 2026-11-26                              # `./sb publish` tự điền
---
```

Quy tắc:

- Mọi dữ kiện, số liệu, định nghĩa trong bài phải có trong ghi chú gốc. Thiếu → nghiên cứu ghi chú trước (`/research`), ghi `[[đề-xuất]]` trong bài.
- Đặt `[[slug]]` tới ghi chú gốc ngay chỗ dùng dữ kiện. Trang bài viết tự hiện mục "Tri thức gốc" (nguồn, điểm uy tín); trang ghi chú tự hiện "Bài viết về chủ đề này". **Ghi chú không liên kết sang bài viết.**
- Không nhồi từ khoá, không giật tít, không hứa hẹn; giữ nguyên sắc thái "còn tranh cãi / giới hạn" của tri thức.
- Kiểm tra trùng từ khoá trước khi viết: `./sb seo --keyword "<từ khoá>"`.

**GEO cho ghi chú tri thức** — chỉ những gì làm tri thức rõ và đáng tin hơn: câu đầu trả lời thẳng ("X là …"), `summary` 70–160 ký tự, nguồn có điểm uy tín, `## Câu hỏi thường gặp`, liên kết nội bộ, ngày cập nhật. Thêm vào đó là cách trình bày giúp AI trích đúng: đoạn mở đầu nêu ý chính, mỗi mục mở bằng câu tự đứng được (gọi đúng tên chủ thể, không "Nó / Điều này"), mỗi đoạn một ý, không nhảy cấp tiêu đề, dẫn `(Nguồn [n])` đúng chỗ, mục giới hạn / khi nào không áp dụng, trích ngắn nguyên văn tài liệu gốc. Chi tiết, bằng chứng và việc không được làm: `.claude/skills/research/references/geo.md`.

**Tri thức đứng trên SEO/GEO** (người dùng dặn 2026-11-26): không thêm khẳng định, số liệu, câu trích không có trong nguồn; không cắt vụn bài, không viết bản riêng cho AI, không nhồi từ khoá, không bỏ giới hạn hay chỗ "còn tranh cãi". Gợi ý nào mà sửa sẽ hại độ chính xác thì để nguyên.

`./sb seo` liệt kê gợi ý (chỉ là gợi ý, không chặn build); trên Bàn làm việc (`./sb mo`), trang nào cũng có hộp "SEO / GEO" (không lên web).

**Phần kỹ thuật chương trình tự lo** khi dựng web: canonical, Open Graph và thẻ X/Twitter kèm ảnh chia sẻ (`assets/og-image.png`), JSON-LD (WebSite + Organization ở trang chủ; Article có tác giả hoặc tên thương hiệu khi `author` trống, đơn vị xuất bản, nguồn trích; FAQ; Breadcrumb; DefinedTermSet), sitemap có ngày sửa, `feed.xml` (Atom), `robots.txt` nêu tên bot tìm kiếm và bot AI, `llms.txt`, bản Markdown. Mã xác minh Google Search Console / Bing Webmaster điền ở `cau-hinh.yml` (`google_site_verification`, `bing_site_verification`). Web nằm dưới `/ten-repo/` thì bot không đọc `robots.txt` của mình (chỉ đọc ở gốc tên miền), nên cần gửi sitemap qua Search Console.

**Tự động sau mỗi lần đăng** (job `sau-khi-dang` trong `deploy.yml`, chạy cả khi người dùng bấm duyệt trên web): (1) `kiem-tra-web.mjs` kiểm web thật — canonical, ảnh chia sẻ, JSON-LD, mã xác minh Google/Bing, sitemap, robots, feed, llms.txt, file khoá IndexNow, `/duyet/` có noindex; hỏng thì báo đỏ. (2) `indexnow.mjs` so sitemap cũ với bản mới rồi báo trang mới/sửa/bỏ cho IndexNow (Bing, Yandex, Naver, Seznam) bằng `indexnow_key` trong `cau-hinh.yml`; lỗi chỉ cảnh báo. Google không dùng IndexNow — nó tự đọc lại sitemap. Không cần đăng ký Bing Webmaster để Bing biết bài mới. Cả hai script chạy thử được trên máy (xem đầu file).

## Quy trình

1. **Nghiên cứu** (`/research <từ khoá>`): agent lập kế hoạch câu hỏi theo nhiều góc nhìn, tìm và đọc nguồn gốc, chấm điểm uy tín, kiểm chứng từng khẳng định, viết bài tổng hợp tiếng Việt có liên kết, lưu vào `knowledge/_inbox/<slug>.md` với `status: draft`, rồi báo cáo cho người dùng (gồm cả những gì chưa đọc được).
2. **Duyệt** (người dùng): ở Khu duyệt bài trên web (`/duyet/`) hoặc Bàn làm việc — đọc bản nháp, nhờ bạn sửa bằng lời (nút "Nhờ Claude sửa" chép sẵn câu nhờ có tên file), hoặc tự sửa.
3. **Đăng**: người dùng bấm **Duyệt & đăng** (hoặc bảo bạn `/publish <file>` → `./sb publish <file>`): chuyển vào thư mục đúng loại, `status: verified`, điền `reviewed`. Sau đó **kết nối**: thêm liên kết hai chiều với ghi chú liên quan và chủ đề trụ cột (`/connect`).
4. **Đưa lên web**: bấm duyệt ở Khu duyệt bài thì GitHub tự đăng và đưa lên. Trên Bàn làm việc thì bấm nút **Đưa lên web** (git add/commit/push), hoặc bạn commit & push khi người dùng nhờ. Sau khi bài được duyệt trên web, người dùng có thể nhờ bạn "kết nối các bài vừa đăng" (`git pull` trước, rồi `/connect`).
5. **Kết nối định kỳ** (`/connect`): thêm liên kết còn thiếu, nối ghi chú cô lập, liệt kê tri thức còn thiếu.
6. **Bài viết SEO/GEO** (`/seo-article <từ khoá>`): agent kiểm tra trùng từ khoá, viết bài nháp vào `articles/` chỉ từ tri thức đã có; người dùng duyệt rồi `/publish articles/<file>.md`.
7. **Viết content cho kênh khác** (`/write-content <đề bài>`): agent đọc kho và viết bài có trích nguồn (blog ngoài, mạng xã hội, email…).

## Giữ kho nhẹ

Git giữ mọi phiên bản mãi mãi, nên file lớn đã commit thì không thu nhỏ lại được.

- `knowledge/` và `articles/` **chỉ chứa Markdown**. Không lưu PDF, ảnh hay bản lưu trang web; ghi link trong `sources`.
- Mỗi ghi chú một ý, dưới ~25 KB (tối đa 40 KB). Dài hơn thì tách thành chủ đề + ghi chú con.
- Không chép lại nội dung đã có ở ghi chú khác: tóm một câu rồi `[[liên kết]]` sang. Một tài liệu được nhiều ghi chú dùng và cần mô tả dài thì tạo ghi chú `source` cho nó.
- Bước `verify` của harness chạy `.harness/checks/check_repo_size.mjs` để kiểm các quy tắc này. Thư mục `bin/`, `target/`, `dist/` đã bỏ khỏi git; web được dựng lại trên GitHub Actions mỗi lần đưa lên.

## Hình minh hoạ (chỉ trên web)

Kho tri thức chỉ có chữ. Hình minh hoạ nằm riêng ở `minh-hoa/<slug>.md` (cùng slug với ghi chú hoặc bài viết) và chỉ được chèn vào **trang web** khi dựng site. File .md của ghi chú, `llms.txt`, `knowledge.json`, `notes/<slug>.md` không có hình.

- Mỗi mục `## <tiêu đề một mục trong bài>` trong file hình được chèn vào **cuối mục đó** trên trang. Sửa tên mục trong bài thì sửa cả ở đây; `./sb check` cảnh báo mục không khớp.
- Hình nến viết bằng khối ```nen, mỗi dòng một lệnh (chi tiết đầu file `src/nen.rs`):
  - `ten: Tiêu đề hình`: chú thích dưới hình;
  - `mở cao thấp đóng`, vd. `100 108 97 105`; thêm `| Nhãn` để tô nổi một cây nến;
  - `ngang 98 Hỗ trợ`: đường ngang; `vung 50 60 Gom hàng @1-6`: vùng giá (tuỳ chọn chỉ từ nến 1 tới 6);
  - `giai-phau`: ghi tên thân, bóng, giá mở/đóng/cao/thấp; `-`: chỗ trống.
- Hình phải **đúng định nghĩa trong bài** (vd. búa: bóng dưới ít nhất gấp đôi thân, sau đà giảm). Số liệu ví dụ lấy đúng số trong bài. Không lưu file ảnh, không chép hình trong sách.
- Trong `articles/` có thể viết khối ```nen ngay trong bài, vì bài viết đã là lớp web.

## Lệnh

Chạy từ thư mục gốc repo. `./sb` dùng `cargo run --release` nếu máy có Rust (code bạn sửa có hiệu lực ngay), không thì dùng/tải bản build sẵn trong `bin/` (`./sb cap-nhat` để tải lại bản mới nhất).

```bash
./sb mo                             # Bàn làm việc + xem trước web tại http://localhost:4321 (có bản nháp, tự cập nhật)
./sb check                          # kiểm tra cấu trúc, liên kết, chất lượng nguồn, trùng từ khoá
./sb check --inbox --verbose        # coi lỗi trong bản nháp cũng là lỗi, liệt kê mọi gợi ý
./sb build                          # check + xuất web tĩnh vào dist/ (đọc SITE_URL, BASE_PATH)
./sb new concept "Tiêu đề" [--inbox] [--tags a,b]
./sb publish [<file> [--status draft] [--dry-run]]
./sb kb search "<từ khoá>"          # tra cứu kho (thêm --json cho máy đọc)
./sb kb show <slug>                 # metadata + liên kết + backlinks + bài viết dựa trên
./sb kb list --type fact --status verified
./sb kb gaps | orphans | mentions [slug] | stats
./sb new article "Tiêu đề" --keyword "…" --based-on a,b [--intent how-to]
./sb publish articles/<file>.md     # đăng bài viết (cần ghi chú gốc verified; --force nếu người dùng chấp nhận rõ ràng)
./sb seo                            # báo cáo SEO/GEO mọi trang
./sb seo <slug> [--drafts]          # chi tiết một trang
./sb seo --keyword "<từ khoá>"      # trang nào đã nhắm từ khoá này
```

Đưa lên web = commit & push lên nhánh mặc định của repo (người dùng có nút **Đưa lên web** trên Bàn làm việc làm đúng việc này: commit, gộp với bản mới trên GitHub nếu có, rồi push; nút **Lấy bản mới** chỉ lấy về). GitHub Actions tự build và đăng lên GitHub Pages. Kho có thể được sửa song song từ nhiều nơi (máy người dùng, phiên cloud): luôn `git pull` trước khi sửa.

## Dùng kho để viết content (agent bên ngoài repo)

Site xuất sẵn: `/llms.txt` (chỉ mục + quy tắc), `/llms-full.txt` (toàn bộ nội dung), `/knowledge.json` (dữ liệu có cấu trúc), `/notes/<slug>.md` (từng ghi chú). Quy tắc bắt buộc nằm trong `cau-hinh.yml` (`agent_rules`):

- Chỉ dùng số liệu, dữ kiện, trích dẫn từ ghi chú `verified` như sự thật; luôn ghi nguồn.
- `draft` chỉ làm ý tưởng; không dùng `outdated` cho content mới.
- Không bịa số liệu, nguồn, trích dẫn. Thiếu thông tin thì nói rõ và đề xuất ghi chú cần bổ sung.

## Khi sửa code

- Chương trình là một crate Rust (`Cargo.toml`): `text.rs` (bỏ dấu, slug, wikilink), `schema.rs` (loại, trạng thái, thang điểm, đọc frontmatter), `kb.rs` (dựng kho, liên kết, backlinks, gaps), `mentions.rs`, `articles.rs`, `seo.rs`, `render.rs` (Markdown → HTML, FAQ, mục lục), `site.rs` (mọi trang + llms.txt, knowledge.json, sitemap…), `server.rs` (Bàn làm việc + API), `publish.rs`, `cli.rs`.
- Giao diện: `templates/*.html` (minijinja) và `assets/` (CSS/JS thuần, không thư viện) — nhúng vào chương trình khi build, nên sửa xong phải build lại.
- Đổi cấu trúc ghi chú → sửa `src/schema.rs`, templates trong `knowledge/_templates/`, và tài liệu này.
- Trước khi kết thúc: `cargo test`, `./sb check` và `./sb build` phải chạy thành công — `node .harness/harness.mjs verify --task …` chạy đủ cả ba (GitHub Actions cũng chạy `cargo test` mỗi lần đưa lên).
