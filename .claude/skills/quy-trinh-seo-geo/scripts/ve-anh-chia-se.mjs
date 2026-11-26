#!/usr/bin/env node
// Vẽ ảnh chia sẻ (assets/og-image.png, 1200×630) và logo (assets/logo.png, 256×256) cho trang tri thức,
// bằng trình duyệt Edge / Chrome chạy ngầm — không cần cài thêm thư viện.
//
// Chạy từ thư mục gốc repo:
//   node .claude/skills/quy-trinh-seo-geo/scripts/ve-anh-chia-se.mjs \
//     --ten "Second Brain" --dong1 "Kho tri thức chuyên môn" --dong2 "được chắt lọc và kiểm chứng" \
//     --nhan "Có nguồn gốc rõ ràng|Chấm điểm uy tín|Người duyệt trước khi đăng"
// Tuỳ chọn: --bieu-tuong assets/favicon.svg  --ra assets  --trinh-duyet "<đường dẫn msedge/chrome>"
// Thiếu --ten thì lấy `name` trong cau-hinh.yml. Vẽ xong phải mở ảnh xem tận mắt.

import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { execFileSync } from "node:child_process";
import { pathToFileURL } from "node:url";

const args = process.argv.slice(2);
const opt = (n, d) => {
  const i = args.indexOf(`--${n}`);
  return i >= 0 ? args[i + 1] : d;
};
const esc = (s) => String(s).replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");

const cfg = fs.existsSync("cau-hinh.yml") ? fs.readFileSync("cau-hinh.yml", "utf8") : "";
const cfgName = cfg.match(/^name:\s*"?([^"\n]+)"?/m)?.[1]?.trim();
const ten = opt("ten", cfgName || "Second Brain");
const dong1 = opt("dong1", "Kho tri thức chuyên môn");
const dong2 = opt("dong2", "được chắt lọc và kiểm chứng");
const nhan = opt("nhan", "Có nguồn gốc rõ ràng|Chấm điểm uy tín|Người duyệt trước khi đăng").split("|").filter(Boolean);
const svg = fs.readFileSync(opt("bieu-tuong", "assets/favicon.svg"), "utf8");
const outDir = path.resolve(opt("ra", "assets"));

function findBrowser() {
  const given = opt("trinh-duyet");
  if (given) return given;
  const candidates = {
    win32: [
      "C:/Program Files (x86)/Microsoft/Edge/Application/msedge.exe",
      "C:/Program Files/Microsoft/Edge/Application/msedge.exe",
      "C:/Program Files/Google/Chrome/Application/chrome.exe",
      `${process.env.LOCALAPPDATA}/Google/Chrome/Application/chrome.exe`,
    ],
    darwin: [
      "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
      "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge",
      "/Applications/Chromium.app/Contents/MacOS/Chromium",
    ],
    linux: ["/usr/bin/google-chrome", "/usr/bin/chromium", "/usr/bin/chromium-browser", "/usr/bin/microsoft-edge"],
  }[process.platform] ?? [];
  const found = candidates.find((p) => p && fs.existsSync(p));
  if (!found) throw new Error("Không tìm thấy Edge/Chrome — truyền --trinh-duyet <đường dẫn>");
  return found;
}

// Màu theo assets/style.css: nền ấm, chữ đậm, ba nhãn tím / xanh / lục.
const ogHtml = `<!doctype html><html><head><meta charset="utf-8"><style>
html,body{margin:0;width:1200px;height:630px;overflow:hidden}
body{background:#fbfaf7;font-family:"Segoe UI","Be Vietnam Pro",Arial,sans-serif;color:#1d1c1a;position:relative}
.bar{position:absolute;left:0;top:0;bottom:0;width:18px;background:linear-gradient(180deg,#7b3fbf,#2f5fd0)}
.wrap{position:absolute;left:96px;right:96px;top:92px}
.logo{display:flex;align-items:center;gap:28px}.logo svg{width:112px;height:112px}
.name{font-size:84px;font-weight:700;letter-spacing:-1.5px}
.tag{margin-top:46px;font-size:44px;line-height:1.3;font-weight:600;max-width:1000px}
.pills{position:absolute;left:96px;right:24px;bottom:84px;display:flex;flex-wrap:wrap;gap:18px}
.pill{font-size:28px;padding:12px 26px;border-radius:999px;font-weight:600}
.p0{background:#f1e9fa;color:#7b3fbf}.p1{background:#e8eefc;color:#2f5fd0}.p2{background:#e3f4ea;color:#1f7a4d}
</style></head><body><div class="bar"></div>
<div class="wrap"><div class="logo">${svg}<div class="name">${esc(ten)}</div></div>
<div class="tag">${esc(dong1)}${dong2 ? `<br>${esc(dong2)}` : ""}</div></div>
<div class="pills">${nhan.map((t, i) => `<span class="pill p${i % 3}">${esc(t)}</span>`).join("")}</div>
</body></html>`;
const logoHtml = `<!doctype html><html><head><meta charset="utf-8"><style>html,body{margin:0;width:256px;height:256px;overflow:hidden;background:transparent}svg{width:256px;height:256px;display:block}</style></head><body>${svg}</body></html>`;

const browser = findBrowser();
const tmp = fs.mkdtempSync(path.join(os.tmpdir(), "ve-anh-"));
function shot(html, name, w, h, transparent) {
  const file = path.join(tmp, `${name}.html`);
  fs.writeFileSync(file, html);
  const out = path.join(outDir, name);
  const flags = ["--headless=new", "--disable-gpu", "--hide-scrollbars", `--window-size=${w},${h}`, `--screenshot=${out}`];
  if (transparent) flags.push("--default-background-color=00000000");
  execFileSync(browser, [...flags, pathToFileURL(file).href], { stdio: "ignore" });
  console.log(`✓ ${path.relative(process.cwd(), out)} (${w}×${h}, ${fs.statSync(out).size} byte)`);
}
fs.mkdirSync(outDir, { recursive: true });
shot(ogHtml, "og-image.png", 1200, 630, false);
shot(logoHtml, "logo.png", 256, 256, true);
fs.rmSync(tmp, { recursive: true, force: true });
console.log("Mở hai ảnh ra xem tận mắt trước khi đưa lên web.");
