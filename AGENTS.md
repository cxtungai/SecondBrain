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
.claude/skills/         Skill cho Claude Code: research, publish, connect, seo-article, write-content, quy-trinh-seo-geo
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
    primary: true                 # tài liệu gốc — xem "Nguồn theo cấp tri thức"
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
| `topic` | Chủ đề trụ cột: bài giới thiệu (cấp Biết) + **Lộ trình học** gồm các cấp chủ đề cần (thường Biết, Hiểu; đủ 6 cấp cho phương pháp), dẫn tới các ghi chú con (xem "Lộ trình học"). |
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

### Nguồn theo cấp tri thức

Người dùng dặn (2026-11-26): mỗi cấp của "Lộ trình học" lấy tài liệu theo một quy tắc riêng. Cấp càng gần "X là gì" thì nguồn càng phải gốc.

**Tài liệu gốc** (`primary: true` trong `sources`), một trong ba loại:

- do chính người đặt ra hoặc hệ thống hoá khái niệm / phương pháp viết: sách, bài, chương mẫu của nhà xuất bản, bản dịch của sách đó (vd. Al Brooks cho price action, Wyckoff, Hamilton cho lý thuyết Dow, Nison cho nến Nhật ở phương Tây);
- văn bản chính thức của tổ chức làm ra thứ được nói tới (vd. Google nói về Google Search, IndexNow.org về IndexNow, RFC 9309 về robots.txt);
- bài nghiên cứu gốc đặt ra khái niệm (vd. bài GEO năm 2024).

Bài tóm tắt, trang giáo dục, bách khoa, blog thuật lại **không** phải tài liệu gốc, dù uy tín cao. "Gốc" và "uy tín" là hai trục khác nhau: sách của Brooks là tài liệu gốc nhưng chỉ được 3 điểm uy tín vì ông bán khoá học.

| Cấp | Được lấy từ đâu |
| --- | --- |
| **1. Biết**: định nghĩa, nguồn gốc, nguyên lý cốt lõi, quy tắc của chính phương pháp | **Bắt buộc tài liệu gốc.** Câu định nghĩa phải dẫn ít nhất một nguồn `primary: true`; nguồn thứ cấp chỉ được diễn giải thêm, đặt sau. Chưa đọc được tài liệu gốc thì ghi rõ "lấy qua nguồn thứ cấp, chưa đối chiếu sách gốc" và đưa vào "Câu hỏi còn mở". |
| **2. Hiểu**, **3. Vận dụng**: giải thích cơ chế, ví dụ, bài tập vận dụng, cách làm từng bước | **Linh hoạt** giữa tài liệu gốc và nguồn uy tín ≥ 3: sách chuyên môn, sở giao dịch, tổ chức đào tạo có tên tuổi, nghiên cứu, chuyên gia kiểm chứng được. Không được trái với định nghĩa gốc; cách giải thích khác tác giả gốc thì ghi "theo …". Ví dụ tự dựng ghi rõ là giả định. |
| **4. Phân tích, tổng hợp** | Mỗi trường phái được so sánh phải dẫn tài liệu gốc của chính nó; nhận định so sánh dẫn nguồn uy tín ≥ 3, hoặc ghi rõ là tổng hợp của kho. |
| **5. Đánh giá** | Nghiên cứu có bình duyệt, dữ liệu sơ cấp, cơ quan quản lý (thường ≥ 4). Lời của chính tác giả về hiệu quả phương pháp của mình **không** phải bằng chứng hiệu quả. |
| **6. Sáng tạo** | Cách kết hợp phải có người viết thật: tác giả gốc, hoặc người thực hành / tổ chức uy tín ≥ 3. Nói rõ đã có kiểm chứng hay chưa. Không tự nghĩ ra chiến lược. |

Cấp 1 là quy tắc người dùng đặt ra; cấp 2–3 là quy tắc "linh hoạt nguồn uy tín" của người dùng; cấp 4–6 là đề xuất của agent theo cùng tinh thần, người dùng có thể sửa. `./sb check` cảnh báo bản nháp (`topic`, `concept`, `framework`) có câu định nghĩa (đoạn đầu bài, đoạn đầu mục "Định nghĩa" hoặc "Mục đích") không dẫn tài liệu gốc. Trên web, nguồn `primary: true` có nhãn "📜 Tài liệu gốc"; trong form sửa bài trên Bàn làm việc là ô đánh dấu "Tài liệu gốc".

## Liên kết tri thức

- `[[slug]]`, `[[slug|nhãn hiển thị]]`, `[[slug#tiêu-đề]]`. Cũng có thể viết `[[Tiêu đề]]` hoặc `[[alias]]` — hệ thống tự tìm ghi chú.
- Link lần xuất hiện đầu tiên của mỗi thuật ngữ đã có ghi chú. Không dùng link Markdown thường cho ghi chú nội bộ.
- `[[thuật-ngữ-chưa-có]]` là hợp lệ: nó hiện trong trang **Tri thức còn thiếu** (`/gaps/`) — hàng đợi nghiên cứu.
- Mỗi ghi chú mới phải được ít nhất một ghi chú `topic` liên kết tới, và nên có mục `## Liên quan`.
- Backlinks, đồ thị, breadcrumb, bài trước / bài tiếp đều được sinh tự động từ liên kết.

## Lộ trình học (tối đa 6 cấp, chọn theo chủ đề)

Người dùng dặn (2026-11-26): tri thức của một chủ đề dẫn người đọc theo các cấp nhận thức của thang Bloom, từ **biết** tới **tự sáng tạo**. Trang chủ đề giới thiệu; các bài con giúp hiểu, làm được, rồi đánh giá và kết hợp. Người dùng dặn thêm cùng ngày: **áp dụng linh hoạt, không bắt đủ 6 cấp**. Nhiều tri thức chỉ cần Biết và Hiểu là đủ; ép đủ 6 cấp sẽ thành máy móc.

| Cấp | Bài phải làm rõ | Thường là |
| --- | --- | --- |
| 1. Biết | X là gì, từ đâu ra, gồm những phần nào | phần Tổng quan của ghi chú `topic` |
| 2. Hiểu | vì sao, cơ chế, thành phần, hiểu sai thường gặp | `concept` |
| 3. Vận dụng | làm từng bước, ví dụ có số, lỗi khi làm | `framework`, `case-study` |
| 4. Phân tích, tổng hợp | so sánh trường phái / biến thể, mổ xẻ ví dụ thật, ghép nhiều yếu tố | `concept`, `case-study` |
| 5. Đánh giá | bằng chứng, số liệu, giới hạn, khi nào không dùng, cách tự kiểm chứng | `fact` |
| 6. Sáng tạo | kết hợp với phương pháp khác, tự xây cách làm — vẫn dựa trên nguồn | `framework` |

**Chọn cấp theo loại tri thức** (gợi ý, không phải luật):

| Loại tri thức | Thường cần | Ví dụ |
| --- | --- | --- |
| Khái niệm, kiến thức nền | Biết, Hiểu | biểu đồ nến, robots.txt, lãi kép |
| Kỹ năng, quy trình, công cụ | Biết, Hiểu, Vận dụng | viết bài SEO, đặt lệnh, đọc báo cáo tài chính |
| Phương pháp có nhiều trường phái, hiệu quả còn tranh cãi, người đọc sẽ dùng tiền thật | đủ 6 cấp | price action, một phương pháp đầu tư |

Quy tắc:

- Chỉ thêm một cấp khi người đọc thật sự có câu hỏi ở cấp đó **và** có nguồn tốt để trả lời. Không tạo bài chỉ để lấp cho đủ cấp.
- Một cấp không cần bài riêng. Nội dung ngắn thì viết thẳng dưới tiêu đề cấp trong bài chủ đề (thẻ lộ trình ghi "Ngay trong bài này"); đủ dài, đủ nguồn thì mới tách bài con.
- Một khái niệm đơn lẻ (`concept`) không cần lộ trình; lộ trình chỉ dành cho ghi chú `topic`.
- Bài tập (xem mục dưới) chỉ bắt buộc với bài con đặt ở cấp Hiểu và Vận dụng; số câu tuỳ độ dài bài.

Ghi chú `topic` có mục `## Lộ trình học` với các tiêu đề `### 1. Biết — …`, `### 2. Hiểu — …`, … chỉ cho những cấp chủ đề dùng (mẫu: `knowledge/_templates/topic.md`; ví dụ đủ 6 cấp: ghi chú Price action). Dưới mỗi cấp: một câu nói cấp đó trả lời gì, rồi `- [[slug]]: mô tả một dòng`; bài đã định viết mà chưa có vẫn ghi liên kết để thành việc cần nghiên cứu. Chương trình tự đọc mục này:

- thứ tự đọc của chủ đề = thứ tự trong lộ trình (cấp thấp → cao); ghi chú `source` không nằm trong lộ trình xếp cuối;
- trang bài con hiện "Cấp 2 · Hiểu — bài 3/8", thời gian đọc, nút **bài trước / bài tiếp**; Khu duyệt bài ghi "Hiểu · Bài 3/8";
- trang chủ đề có thẻ **Lộ trình học · N cấp**, chỉ gồm các cấp chủ đề dùng (số bài, số bài còn thiếu, hoặc "Ngay trong bài này");
- `knowledge.json` có `learning_path` cho chủ đề; `./sb seo <slug>` ghi các cấp chủ đề đang dùng; tiêu đề cấp 3 không khớp cấp nào thì `./sb check` cảnh báo.

Thêm bài mới cho chủ đề đã có lộ trình → đặt vào đúng cấp (`/connect`). Sắp xếp lại lộ trình chỉ là đổi cách trình bày: không thêm, bớt hay đổi dữ kiện của ghi chú.

### Mục tiêu bài học và bài tập tự kiểm tra

Người dùng dặn (2026-11-26): bài cấp **Hiểu** và **Vận dụng** phải có bài tập để người đọc tự làm, không chỉ đọc. Theo cách sách giáo khoa mở OpenStax làm (mục tiêu đầu bài, câu hỏi ôn tập cuối bài), và theo tổng hợp nghiên cứu của Dunlosky và cộng sự (2013): tự kiểm tra là một trong hai cách học hiệu quả nhất, còn đọc lại hay tô đậm thì kém.

- `## Mục tiêu bài học`: đặt ngay sau đoạn mở đầu (câu định nghĩa vẫn đứng đầu bài). 2–4 gạch đầu dòng "Sau bài này bạn sẽ…", mở bằng động từ của cấp (Hiểu: giải thích được, phân biệt được; Vận dụng: nhận ra được, tính được, làm được). Trên web hiện thành khung 🎯.
- `## Bài tập`: đặt trước "Câu hỏi thường gặp". Số câu tuỳ độ dài bài (bài ngắn 1–2 câu, bài dài tới 5 câu), mỗi câu là `### Câu 1: …`, đề bài ngay dưới, rồi một đoạn mở bằng `**Đáp án:**`; có thể thêm `**Gợi ý:**` trước đáp án. Trên web, gợi ý và đáp án ẩn, bấm mới hiện (phần ẩn kéo tới câu kế tiếp). File `.md`, `llms.txt` và bản cho AI vẫn có đủ đáp án.
- Mỗi mục tiêu có ít nhất một câu kiểm tra. Ưu tiên câu bắt người đọc tự nhớ lại, tự giải thích, tự làm, hơn là chọn đáp án có sẵn.
- **Nguồn:** đề và đáp án chỉ dùng điều đã có trong chính bài, đúng định nghĩa gốc, dẫn `(Nguồn [n])` như phần thân bài. Số liệu tự đặt ra để luyện (giá, khối lượng lệnh) ghi rõ "giả định". Không thêm dữ kiện mới chỉ để có câu hỏi.
- Bài tập không phải FAQ: "Câu hỏi thường gặp" trả lời điều người đọc hay hỏi (Google/AI dùng), còn "Bài tập" để người học tự luyện.
- Hình cho một câu: `minh-hoa/<slug>.md`, mục `## Câu 1: …` (cùng tên tiêu đề câu). Hình nằm sau đề, trước đáp án ẩn.
- `./sb check` cảnh báo bản nháp cấp Hiểu / Vận dụng thiếu một trong hai mục, và câu bài tập thiếu đáp án. Trang bài hiện "📝 N bài tập tự kiểm tra"; `knowledge.json` có `exercises` (số câu).

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

**Quy trình SEO/GEO dùng lại được:** `.claude/skills/quy-trinh-seo-geo/` (skill `/quy-trinh-seo-geo`) gom mọi bước theo 8 giai đoạn: dựng trang mới, nền kỹ thuật, đăng ký Google/Bing/Brave, tự động, nội dung, uy tín bên ngoài, đo lường, đổi tên miền. `trang-thai.md` ghi trang này đã làm tới đâu. **Mỗi lần tối ưu SEO/GEO được điều gì mới** (sửa chương trình, thêm bước, bài học) thì cập nhật luôn `SKILL.md`, `references/` liên quan, `NHAT-KY.md` và `trang-thai.md`; kiến thức chung thì cập nhật cả các ghi chú tri thức SEO/GEO. Dựng trang tri thức khác thì chép thư mục skill này (hoặc cả repo) rồi chạy `/quy-trinh-seo-geo trang-moi`.

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
