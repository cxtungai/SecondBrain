---
name: quy-trinh-seo-geo
description: Quy trình SEO/GEO đầy đủ cho một trang tri thức — dựng trang mới từ Second Brain, nền kỹ thuật web, đăng ký Google/Bing/Brave, tự động sau mỗi lần đăng, viết nội dung để Google và AI trích đúng, uy tín bên ngoài, đo lường, đổi tên miền. Dùng khi dựng một trang tri thức mới, kiểm tra hay tối ưu SEO/GEO, chuẩn bị đổi tên miền / tên web, hoặc người dùng hỏi "quy trình SEO/GEO", "đã làm tới đâu", "còn thiếu gì". Mỗi lần tối ưu được điều gì mới thì cập nhật quy trình này và NHAT-KY.md.
argument-hint: "[kiem-tra | trang-moi | ten-mien | do-luong]"
---

# Quy trình SEO/GEO cho trang tri thức

Đây là quy trình đã chạy thật trên trang Second Brain đầu tiên (2026-11-26). Mục đích: dựng thêm trang tri thức khác (chủ đề khác, repo khác) thì chỉ cần đưa quy trình này cho agent làm theo.

- **SEO** là làm cho Google, Bing **tìm thấy, hiểu đúng và xếp hạng** trang.
- **GEO** (Generative Engine Optimization) là làm cho trợ lý AI (ChatGPT, Claude, Perplexity, Gemini, Google AI Overviews, Copilot) **chọn và trích đúng** trang khi trả lời.

Tham số: **$ARGUMENTS**

- `kiem-tra`: rà từng giai đoạn bên dưới, báo giai đoạn nào xong, giai đoạn nào thiếu. Cập nhật `trang-thai.md`.
- `trang-moi`: làm từ giai đoạn 0.
- `ten-mien`: làm theo `references/ten-mien.md`.
- `do-luong`: làm theo `references/do-luong.md`.
- Trống: hỏi người dùng muốn làm gì, gợi ý dựa trên `trang-thai.md`.

## Nguyên tắc (không đổi giữa các trang)

1. **Tri thức đứng trên SEO/GEO.** Không thêm điều nguồn không nói, không cắt vụn bài, không viết bản riêng cho AI, không nhồi từ khoá. Gợi ý nào mà sửa sẽ hại độ chính xác thì bỏ (người dùng dặn 2026-11-26). Chi tiết: `../research/references/geo.md`.
2. **AI làm, người duyệt.** Nội dung chỉ lên web khi người dùng duyệt. Việc cần tài khoản của người dùng (tạo tài khoản, xác minh, bấm cấp quyền, mua tên miền) thì **người dùng tự làm**; agent hướng dẫn từng cú bấm và làm mọi phần còn lại.
3. **Làm một lần, rồi để máy tự chạy.** Việc lặp lại mỗi lần đăng bài phải tự động trong GitHub Actions, không dựa vào việc ai nhớ làm.
4. **Đo rồi mới chỉnh.** Không đoán: dựa vào Search Console, Bing (kể cả mục AI Performance) và kiểm tra AI định kỳ.

## Bản đồ giai đoạn

| # | Giai đoạn | Mục đích | Ai làm | Xong khi |
| --- | --- | --- | --- | --- |
| 0 | Dựng trang mới | Có web tri thức chạy được với tên, mô tả, khoá riêng | Agent + người dùng bật Pages | Web mở được, `./sb check` không lỗi |
| 1 | Nền kỹ thuật | Google và AI **đọc được, hiểu đúng** từng trang | Chương trình tự làm; agent kiểm | `kiem-tra-web.mjs` đạt |
| 2 | Được biết đến | Công cụ tìm kiếm **biết web tồn tại**; có chỗ để đo | Người dùng (một lần), agent hướng dẫn | Google đã xác minh và nhận sitemap; Bing đã nhập; Brave đã gửi |
| 3 | Tự động sau mỗi lần đăng | Kiểm tra web thật và báo trang mới, không cần ai làm tay | GitHub Actions | Job `sau-khi-dang` xanh; IndexNow trả 200/202 |
| 4 | Nội dung | Bài **đáng được chọn và trích**: đúng, rõ, có nguồn | Agent viết nháp, người dùng duyệt | Có bài đã duyệt; gợi ý `./sb seo` đã xử lý |
| 5 | Uy tín bên ngoài | Nơi khác nhắc tới, dẫn link về web | Người dùng (agent soạn nội dung) | Có nhắc tới hoặc link từ nơi khác |
| 6 | Đo lường và lặp lại | Biết điều gì hiệu quả để chỉnh | Agent, hằng tháng | Có báo cáo tháng trong `trang-thai.md` |
| 7 | Đổi tên miền / tên web | Chuyển nhà không mất thứ hạng | Người dùng mua; agent làm phần còn lại | Địa chỉ cũ chuyển hướng; Google/Bing nhận địa chỉ mới |

Giai đoạn 1 và 3 là phần nền, phải xong trước. Giai đoạn 4–6 mới quyết định thứ hạng và việc được AI trích dẫn.

## Giai đoạn 0 — Dựng trang tri thức mới từ Second Brain

1. **Tạo repo mới** từ repo Second Brain. Có hai cách: trên GitHub bấm **Use this template** (repo gốc phải bật "Template repository" trong Settings), hoặc nhờ agent sao chép. Repo phải public, vì GitHub Pages với repo private cần gói trả phí.
2. **Dọn nội dung mẫu:** xoá ghi chú có tag `vi-du`, xoá bản nháp và `minh-hoa/` của chủ đề cũ. Giữ `knowledge/_templates/`.
3. **`cau-hinh.yml`:**
   - `name`, `description` (70–160 ký tự, nói web chứa tri thức gì);
   - `author` (để trống thì web đứng tên thương hiệu);
   - `disclaimer` hợp chủ đề;
   - **xoá trắng** `google_site_verification`, `bing_site_verification`, vì mã gắn với địa chỉ web cũ;
   - **tạo `indexnow_key` mới**: `node -e "console.log(require('crypto').randomBytes(16).toString('hex'))"`. Mỗi web một khoá riêng.
4. **Ảnh chia sẻ và logo** theo tên mới: `node .claude/skills/quy-trinh-seo-geo/scripts/ve-anh-chia-se.mjs --ten "<Tên>" --dong1 "…" --dong2 "…"`. Mở ảnh xem tận mắt.
5. **Người dùng bật web:** repo → Settings → Pages → Source: **GitHub Actions**.
6. Sửa địa chỉ web trong `README.md`, `CLAUDE.md`, `AGENTS.md` (tìm `github.io`).
7. Đẩy lên, chờ GitHub dựng xong, chạy `kiem-tra-web.mjs`, rồi chuyển sang giai đoạn 2.

## Giai đoạn 1 — Nền kỹ thuật

Chương trình Second Brain tự làm hết khi dựng web. Danh sách đầy đủ những gì phải đúng trên mọi trang, và cách làm khi dùng công cụ dựng web khác: `references/ky-thuat.md`.

Kiểm tra: `node .github/scripts/kiem-tra-web.mjs --site <địa chỉ web>/`. Lệnh này kiểm địa chỉ chính thức (canonical), ảnh chia sẻ, JSON-LD, mã xác minh, sitemap, robots, feed, llms.txt, khoá IndexNow, và việc Khu duyệt bài có `noindex`.

## Giai đoạn 2 — Được biết đến (người dùng làm một lần)

Hướng dẫn từng cú bấm, kèm những chỗ dễ vướng: `references/dang-ky.md`.

1. **Google Search Console**:
   - chọn kiểu **Tiền tố URL** và xác minh bằng **Thẻ HTML**;
   - người dùng dán thẻ cho agent, agent gắn vào `cau-hinh.yml` rồi đẩy lên;
   - chờ trang thật có thẻ thì bấm **Xác minh**;
   - gửi `sitemap.xml`, bấm **Yêu cầu lập chỉ mục** cho trang chủ;
   - kiểm **Cài đặt → Search generative AI** đang ở **Include** (mặc định), để AI của Google được trích web.
2. **Bing Webmaster Tools:** chọn **Import** từ Google Search Console. Không cần mã xác minh. Nếu mục Sitemaps trống thì gửi **địa chỉ đầy đủ** của sitemap.
3. **Brave Search:** dán trang chủ vào search.brave.com/submit-url, gửi một lần. Brave không có trang quản lý.

## Giai đoạn 3 — Tự động sau mỗi lần đăng

Đã có trong `.github/workflows/deploy.yml`, job `sau-khi-dang`. Job này chạy cả khi người dùng bấm duyệt trên web.

- `kiem-tra-web.mjs` kiểm web thật; hỏng thì lần chạy báo đỏ.
- `indexnow.mjs` so sitemap cũ với sitemap mới rồi báo trang mới, vừa sửa hoặc vừa bỏ cho IndexNow (Bing, Yandex, Naver, Seznam). Lỗi ở bước này chỉ là cảnh báo.
- **Lần đầu có khoá:** sitemap chưa đổi nên chưa có gì để báo. Gửi tay toàn bộ web một lần:

  ```bash
  node .github/scripts/indexnow.mjs doi --tat-ca --moi <địa chỉ web>/sitemap.xml > urls.txt
  node .github/scripts/indexnow.mjs gui --site <địa chỉ web>/ --tep urls.txt
  ```

- **Google không cần báo:** Google tự đọc lại sitemap. Không có cách tự động hợp lệ để bấm "Yêu cầu lập chỉ mục" cho bài thường, vì Indexing API chỉ dành cho tin tuyển dụng và livestream.

## Giai đoạn 4 — Nội dung

- **Ghi chú tri thức:** skill `research`, kèm danh sách tự kiểm trong `../research/references/geo.md`. Sau đó `./sb seo <slug> --drafts` và sửa cách trình bày theo gợi ý, không đổi ý.
- **Bài SEO** nhắm từ khoá người Việt hay tìm: skill `seo-article`. Kiểm trùng từ khoá trước; bài chỉ dùng dữ kiện từ ghi chú đã duyệt.
- **Thứ tự ưu tiên:** người dùng duyệt các ghi chú trụ cột trước, rồi mới viết bài SEO cho từng chủ đề. Chưa có bài đã duyệt thì Google và AI không có gì để xếp hạng.
- **Chủ đề tiền bạc, sức khoẻ, pháp lý** bị Google xét khắt khe hơn: phải rõ ai biên soạn, quy trình duyệt, nguồn và giới hạn. Trang "Giới thiệu & quy trình biên tập" phải luôn đúng với thực tế.

## Giai đoạn 5 — Uy tín bên ngoài

AI và Google tin một trang hơn khi nơi khác nhắc tới nó. Agent không tự đăng gì ra bên ngoài.

- Agent soạn sẵn nội dung chia sẻ cho từng bài đã duyệt (nhóm, diễn đàn, mạng xã hội, video) theo skill `write-content`. Nội dung phải có ích ngay tại chỗ, kèm link về bài gốc.
- Người dùng tự đăng, hoặc cho phép từng lần.
- Không mua link, không tạo đánh giá hay nhắc tới giả. Google phạt các việc này, và chúng làm mất niềm tin.

## Giai đoạn 6 — Đo lường và lặp lại (hằng tháng)

`references/do-luong.md`: đọc Search Console, đọc Bing (Search Performance, AI Performance), hỏi lại các AI bộ câu hỏi chính, ghi kết quả vào `trang-thai.md`, rồi chọn 1–3 việc cho tháng sau.

## Giai đoạn 7 — Đổi tên miền / tên web

`references/ten-mien.md`. Phần lớn tự cập nhật, vì SITE_URL lấy từ GitHub Pages khi dựng. Phần làm tay: DNS, Settings → Pages, Google/Bing/Brave cho địa chỉ mới, ảnh chia sẻ nếu đổi tên.

## Cập nhật quy trình này (bắt buộc)

Mỗi lần tối ưu SEO/GEO được điều gì mới (sửa chương trình, thêm bước, rút ra bài học, công cụ đổi cách làm):

1. Sửa giai đoạn tương ứng ở trên và file `references/` liên quan.
2. Thêm một dòng vào `NHAT-KY.md`: ngày, đổi gì, vì sao, bằng chứng.
3. Cập nhật `trang-thai.md` của trang này.
4. Nếu là kiến thức chung về SEO/GEO (không riêng repo này), cập nhật cả ghi chú tri thức liên quan. Đó là bản nháp trong `knowledge/_inbox/`, chờ người dùng duyệt: `seo-va-geo`, `generative-engine-optimization`, `quy-trinh-seo-geo-cho-trang-tri-thuc`…

Khi đem quy trình sang trang khác: chép cả thư mục `.claude/skills/quy-trinh-seo-geo/` (hoặc cả repo), xoá nội dung `trang-thai.md` rồi chạy `kiem-tra`.
