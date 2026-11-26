#!/usr/bin/env node
// Báo trang mới / vừa sửa / vừa xoá cho IndexNow (Bing, Yandex, Naver, Seznam…) sau mỗi lần dựng web.
// Google không dùng IndexNow: Google tự đọc lại sitemap.xml (có ngày sửa từng trang).
//
// Dùng (chạy được trên máy, chỉ cần Node 18+):
//   node .github/scripts/indexnow.mjs doi --cu https://ten.github.io/Repo/sitemap.xml --moi dist/sitemap.xml > urls.txt
//       So sitemap đang chạy trên web (bản cũ) với bản vừa dựng: in ra URL mới, URL có ngày sửa khác, URL đã bỏ.
//       Không tải được bản cũ (lần đầu) → in mọi URL trong bản mới.
//   node .github/scripts/indexnow.mjs doi --tat-ca --moi https://ten.github.io/Repo/sitemap.xml > urls.txt
//       Mọi URL của web (dùng cho lần gửi đầu sau khi có khoá). --moi nhận cả file lẫn địa chỉ web.
//   node .github/scripts/indexnow.mjs gui --site https://ten.github.io/Repo/ --tep urls.txt [--thu]
//       Gửi danh sách cho api.indexnow.org. Khoá đọc từ cau-hinh.yml (indexnow_key).
//       --thu: chỉ in nội dung sẽ gửi, không gửi.
// Lỗi khi gửi chỉ là cảnh báo (::warning::) — không làm hỏng lần đăng web.

import fs from "node:fs";

const args = process.argv.slice(2);
const cmd = args[0];
const opt = (name) => {
  const i = args.indexOf(`--${name}`);
  return i >= 0 ? args[i + 1] : undefined;
};
const flag = (name) => args.includes(`--${name}`);

/** Đọc một trường chuỗi đơn giản trong cau-hinh.yml. */
export function configValue(text, key) {
  const m = text.match(new RegExp(`^${key}:\\s*(?:"([^"]*)"|'([^']*)'|([^#\\n]*))`, "m"));
  return m ? (m[1] ?? m[2] ?? m[3] ?? "").trim() : "";
}

/** <loc> → <lastmod> (hoặc "") của một sitemap. */
export function parseSitemap(xml) {
  const out = new Map();
  for (const m of xml.matchAll(/<url>([\s\S]*?)<\/url>/g)) {
    const loc = m[1].match(/<loc>([^<]+)<\/loc>/)?.[1]?.trim();
    const mod = m[1].match(/<lastmod>([^<]+)<\/lastmod>/)?.[1]?.trim() ?? "";
    if (loc) out.set(loc.replace(/&amp;/g, "&"), mod);
  }
  return out;
}

/** URL cần báo: mới, đổi ngày sửa, hoặc đã bỏ khỏi sitemap. */
export function changedUrls(oldMap, newMap) {
  if (!oldMap) return [...newMap.keys()];
  const out = [];
  for (const [loc, mod] of newMap) if (!oldMap.has(loc) || oldMap.get(loc) !== mod) out.push(loc);
  for (const loc of oldMap.keys()) if (!newMap.has(loc)) out.push(loc);
  return out;
}

async function fetchText(url) {
  try {
    const r = await fetch(url, { headers: { "cache-control": "no-cache" } });
    return r.ok ? await r.text() : null;
  } catch {
    return null;
  }
}

async function doi() {
  const cu = flag("tat-ca") ? undefined : opt("cu"), moi = opt("moi");
  if (!moi) throw new Error("thiếu --moi <sitemap.xml vừa dựng — đường dẫn file hoặc địa chỉ web>");
  const newXml = /^https?:\/\//.test(moi) ? await fetchText(moi) : fs.readFileSync(moi, "utf8");
  if (!newXml) throw new Error(`không tải được ${moi}`);
  const newMap = parseSitemap(newXml);
  const oldXml = cu ? await fetchText(cu) : null;
  const oldMap = oldXml ? parseSitemap(oldXml) : null;
  if (!oldMap) console.error(`Không tải được sitemap cũ (${cu ?? "không có --cu"}) — báo mọi trang.`);
  const urls = changedUrls(oldMap, newMap);
  console.error(`${urls.length} trang cần báo (trên ${newMap.size} trang trong sitemap).`);
  if (urls.length) console.log(urls.join("\n"));
}

async function gui() {
  const site = (opt("site") ?? "").replace(/\/?$/, "/");
  const tep = opt("tep");
  const key = configValue(fs.readFileSync(opt("cau-hinh") ?? "cau-hinh.yml", "utf8"), "indexnow_key");
  if (!/^[A-Za-z0-9-]{8,128}$/.test(key)) {
    console.log("Chưa có indexnow_key hợp lệ trong cau-hinh.yml — bỏ qua.");
    return;
  }
  if (!/^https?:\/\//.test(site)) throw new Error("thiếu --site <địa chỉ web, vd. https://ten.github.io/Repo/>");
  const urls = [...new Set((tep ? fs.readFileSync(tep, "utf8") : "").split(/\r?\n/).map((s) => s.trim()).filter(Boolean))]
    .filter((u) => u.startsWith(site))
    .slice(0, 10000);
  if (!urls.length) {
    console.log("Không có trang mới hay vừa sửa — không cần báo.");
    return;
  }
  const body = { host: new URL(site).host, key, keyLocation: `${site}${key}.txt`, urlList: urls };
  if (flag("thu")) {
    console.log(JSON.stringify(body, null, 2));
    return;
  }
  let status = 0, text = "";
  try {
    const r = await fetch("https://api.indexnow.org/indexnow", {
      method: "POST",
      headers: { "content-type": "application/json; charset=utf-8" },
      body: JSON.stringify(body),
    });
    status = r.status;
    text = (await r.text()).slice(0, 300);
  } catch (e) {
    text = String(e);
  }
  // 200: đã nhận; 202: đã nhận, đang xác minh khoá (lần đầu).
  if (status === 200 || status === 202) {
    console.log(`IndexNow đã nhận ${urls.length} trang (mã ${status}).`);
    for (const u of urls) console.log(`  ${u}`);
  } else {
    const why = { 400: "yêu cầu sai định dạng", 403: "khoá không khớp file khoá trên web", 422: "URL không thuộc web này", 429: "gửi quá nhiều" }[status] ?? "không gửi được";
    console.log(`::warning::IndexNow trả mã ${status || "—"} (${why}). ${text}`);
  }
}

// Chỉ chạy khi gọi trực tiếp (kiem-tra-web.mjs chỉ mượn hàm configValue).
const main = { doi, gui }[cmd];
if (process.argv[1]?.endsWith("indexnow.mjs")) {
  if (!main) {
    console.error("Dùng: indexnow.mjs doi --cu <url sitemap cũ> --moi <sitemap mới> | gui --site <url> --tep <file> [--thu]");
    process.exitCode = 2;
  } else {
    main().catch((e) => {
      console.error(`Lỗi: ${e.message}`);
      process.exitCode = 1;
    });
  }
}
