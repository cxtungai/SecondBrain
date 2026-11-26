#!/usr/bin/env node
// Kiểm tra web THẬT sau khi đăng: những thứ SEO/GEO quan trọng còn nguyên không.
// Có lỗi → in ::error:: và thoát mã 1 (GitHub báo đỏ, gửi email cho chủ repo).
//
// Dùng (chạy được trên máy, chỉ cần Node 18+):
//   node .github/scripts/kiem-tra-web.mjs --site https://ten.github.io/Repo/ [--lan 10] [--cho 30]
//   --lan: số lần thử (GitHub Pages có thể mất 1–2 phút mới đổi bản), --cho: giây giữa hai lần.
// Mã xác minh Google/Bing và khoá IndexNow đọc từ cau-hinh.yml (--cau-hinh để đổi file).

import fs from "node:fs";
import { configValue } from "./indexnow.mjs";

const args = process.argv.slice(2);
const opt = (name, d) => {
  const i = args.indexOf(`--${name}`);
  return i >= 0 ? args[i + 1] : d;
};
const site = (opt("site", "") || "").replace(/\/?$/, "/");
const tries = Number(opt("lan", 10));
const wait = Number(opt("cho", 30));
const cfg = fs.existsSync(opt("cau-hinh", "cau-hinh.yml")) ? fs.readFileSync(opt("cau-hinh", "cau-hinh.yml"), "utf8") : "";
const google = configValue(cfg, "google_site_verification").match(/content="([^"]+)"/)?.[1] ?? configValue(cfg, "google_site_verification");
const bing = configValue(cfg, "bing_site_verification").match(/content="([^"]+)"/)?.[1] ?? configValue(cfg, "bing_site_verification");
const key = configValue(cfg, "indexnow_key");

if (!/^https?:\/\//.test(site)) {
  console.error("Dùng: kiem-tra-web.mjs --site https://ten.github.io/Repo/");
  process.exit(2); // chưa mở kết nối nào nên thoát ngay được
}

async function get(url) {
  try {
    const r = await fetch(`${url}${url.includes("?") ? "&" : "?"}kiem-tra=${Date.now()}`, { headers: { "cache-control": "no-cache" } });
    return { status: r.status, type: r.headers.get("content-type") ?? "", text: r.status < 400 ? await r.text() : "" };
  } catch (e) {
    return { status: 0, type: "", text: "", error: String(e) };
  }
}

const attr = (html, re) => html.match(re)?.[1];
const canonicalOf = (html) => attr(html, /<link rel="canonical" href="([^"]+)"/);
const ogImageOf = (html) => attr(html, /<meta property="og:image" content="([^"]+)"/);
function jsonLdOk(html) {
  const blocks = [...html.matchAll(/<script type="application\/ld\+json">([\s\S]*?)<\/script>/g)].map((m) => m[1]);
  try {
    blocks.forEach((b) => JSON.parse(b));
    return blocks.join("\n");
  } catch {
    return null;
  }
}

async function checkOnce() {
  const fails = [];
  const ok = (cond, msg) => cond || fails.push(msg);

  const home = await get(site);
  ok(home.status === 200, `Trang chủ trả mã ${home.status || home.error}`);
  if (home.status === 200) {
    ok(canonicalOf(home.text) === site, `Trang chủ: canonical là "${canonicalOf(home.text) ?? "không có"}", cần "${site}"`);
    const ld = jsonLdOk(home.text);
    ok(ld !== null, "Trang chủ: dữ liệu cấu trúc JSON-LD hỏng");
    ok(ld?.includes('"WebSite"'), "Trang chủ: thiếu dữ liệu WebSite");
    ok(!/content="noindex"/.test(home.text), "Trang chủ đang bị noindex");
    if (google) ok(home.text.includes(`name="google-site-verification" content="${google}"`), "Trang chủ: mất thẻ xác minh Google Search Console");
    if (bing) ok(home.text.includes(`name="msvalidate.01" content="${bing}"`), "Trang chủ: mất thẻ xác minh Bing");
    const img = ogImageOf(home.text);
    ok(!!img, "Trang chủ: thiếu ảnh chia sẻ (og:image)");
    if (img) {
      const r = await get(img);
      ok(r.status === 200 && r.type.startsWith("image/"), `Ảnh chia sẻ ${img} trả mã ${r.status} (${r.type})`);
    }
  }

  const sm = await get(`${site}sitemap.xml`);
  ok(sm.status === 200 && sm.text.includes(`<loc>${site}</loc>`), `sitemap.xml trả mã ${sm.status} hoặc thiếu trang chủ`);
  const page = sm.text.match(/<loc>([^<]+\/notes\/[^<]+\/)<\/loc>/)?.[1];
  if (page) {
    const p = await get(page);
    ok(p.status === 200, `Trang ghi chú ${page} trả mã ${p.status}`);
    if (p.status === 200) {
      ok(canonicalOf(p.text) === page, `${page}: canonical là "${canonicalOf(p.text) ?? "không có"}"`);
      ok(!!ogImageOf(p.text), `${page}: thiếu og:image`);
      ok(jsonLdOk(p.text)?.includes('"Article"'), `${page}: JSON-LD hỏng hoặc thiếu Article`);
      ok(!/content="noindex"/.test(p.text), `${page}: đang bị noindex`);
    }
  }

  const robots = await get(`${site}robots.txt`);
  ok(robots.status === 200 && robots.text.includes("Sitemap:"), `robots.txt trả mã ${robots.status} hoặc thiếu dòng Sitemap`);
  const feed = await get(`${site}feed.xml`);
  ok(feed.status === 200 && feed.text.includes("<feed"), `feed.xml trả mã ${feed.status}`);
  const llms = await get(`${site}llms.txt`);
  ok(llms.status === 200 && llms.text.startsWith("# "), `llms.txt trả mã ${llms.status}`);
  if (key) {
    const k = await get(`${site}${key}.txt`);
    ok(k.status === 200 && k.text.trim() === key, `File khoá IndexNow ${key}.txt trả mã ${k.status} hoặc sai nội dung`);
  }
  // Bản nháp không bao giờ được để Google lập chỉ mục.
  const review = await get(`${site}duyet/`);
  if (review.status === 200) ok(/content="noindex"/.test(review.text), "Khu duyệt bài (/duyet/) thiếu noindex — bản nháp có thể lên Google!");
  return fails;
}

// Đặt mã thoát thay vì process.exit(): trên Windows, thoát ngang khi fetch còn giữ kết nối làm Node báo lỗi giả.
let fails = [];
for (let i = 1; i <= tries; i++) {
  fails = await checkOnce();
  if (!fails.length) {
    console.log(`✓ Web ${site} đạt mọi mục kiểm tra (lần ${i}).`);
    break;
  }
  console.log(`Lần ${i}/${tries}: ${fails.length} mục chưa đạt.`);
  if (i < tries) await new Promise((r) => setTimeout(r, wait * 1000));
}
for (const f of fails) console.log(`::error::${f}`);
process.exitCode = fails.length ? 1 : 0;
