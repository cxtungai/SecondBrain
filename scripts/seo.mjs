// Báo cáo SEO / GEO cho ghi chú và bài viết — chỉ là gợi ý, không chặn build.
// Dùng: npm run seo                         (tóm tắt mọi trang, liệt kê trang còn nhiều điểm cần cải thiện)
//       npm run seo -- <slug>               (chi tiết một ghi chú hoặc bài viết)
//       npm run seo -- --keyword "từ khoá"  (trang nào đã nhắm từ khoá này — kiểm tra trước khi viết bài mới)
//       thêm --drafts để tính cả bản nháp, --json để máy đọc
import { c, parseArgs, today } from './args.mjs';
import { loadArticles, loadKnowledge } from './load.mjs';
import { articleSeoHints, keywordTargets, noteSeoHints } from '../src/lib/seo.mjs';

const { positional, flags } = parseArgs();
const kb = loadKnowledge({ includeInbox: Boolean(flags.drafts) });
const posts = loadArticles(kb, { includeDrafts: Boolean(flags.drafts) });
const json = Boolean(flags.json);

const notes = flags.drafts ? kb.all : kb.notes;
const articles = posts.all;

if (flags.keyword) {
  const targets = keywordTargets(String(flags.keyword), { notes, articles });
  if (json) {
    console.log(JSON.stringify(targets.map((t) => ({ kind: t.kind, match: t.match, slug: t.item.slug, title: t.item.title, path: t.item.path })), null, 2));
    process.exit(0);
  }
  console.log(c.bold(`Từ khoá "${flags.keyword}"`));
  if (!targets.length) console.log(c.green('  Chưa trang nào nhắm từ khoá này — có thể viết bài mới.'));
  for (const t of targets) {
    const what = t.kind === 'note' ? 'ghi chú' : 'bài viết';
    const how = t.match === 'exact' ? c.red('trùng') : c.yellow('gần');
    console.log(`  ${how}  ${what} ${c.bold(t.item.title)} ${c.dim(t.item.path)}`);
  }
  if (targets.some((t) => t.match === 'exact')) {
    console.log(c.dim('\n  Đã có trang nhắm đúng từ khoá → cải thiện trang đó, hoặc chọn góc khác (hướng dẫn, so sánh, sai lầm thường gặp…).'));
  }
  process.exit(0);
}

const rows = [
  ...notes.map((n) => ({ kind: 'note', item: n, hints: noteSeoHints(n, { today: today() }) })),
  ...articles.map((a) => ({ kind: 'article', item: a, hints: articleSeoHints(a) })),
];

if (positional[0]) {
  const row = rows.find((r) => r.item.slug === positional[0] || r.item.path === positional[0]);
  if (!row) {
    console.error(c.red(`Không tìm thấy "${positional[0]}" (thêm --drafts nếu là bản nháp)`));
    process.exit(1);
  }
  if (json) {
    console.log(JSON.stringify({ slug: row.item.slug, kind: row.kind, hints: row.hints }, null, 2));
    process.exit(0);
  }
  printRow(row, true);
  process.exit(0);
}

if (json) {
  console.log(JSON.stringify(rows.map((r) => ({ slug: r.item.slug, kind: r.kind, path: r.item.path, passed: r.hints.filter((h) => h.ok).length, total: r.hints.length, todo: r.hints.filter((h) => !h.ok) })), null, 2));
  process.exit(0);
}

function printRow(row, detailed) {
  const passed = row.hints.filter((h) => h.ok).length;
  const ratio = passed / row.hints.length;
  const color = ratio >= 0.85 ? c.green : ratio >= 0.6 ? c.yellow : c.red;
  console.log(`\n${color(`${passed}/${row.hints.length}`)} ${c.bold(row.item.title)} ${c.dim(`${row.kind === 'note' ? 'ghi chú' : 'bài viết'} · ${row.item.path}`)}`);
  for (const h of row.hints) {
    if (h.ok && !detailed) continue;
    console.log(`  ${h.ok ? c.green('✓') : c.yellow('○')} ${h.label}${h.ok ? '' : c.dim(` — ${h.tip}`)}`);
  }
}

console.log(c.bold('Báo cáo SEO / GEO') + c.dim(' — gợi ý, không chặn build. Ghi chú: ưu tiên rõ ràng & đáng tin; bài viết: ưu tiên đúng ý định tìm kiếm.'));
const sorted = [...rows].sort((a, b) => a.hints.filter((h) => h.ok).length / a.hints.length - b.hints.filter((h) => h.ok).length / b.hints.length);
for (const row of sorted) printRow(row, false);
const all = rows.flatMap((r) => r.hints);
console.log(`\n${c.bold('Tổng')}: ${all.filter((h) => h.ok).length}/${all.length} tiêu chí đạt trên ${notes.length} ghi chú, ${articles.length} bài viết.`);
console.log(c.dim('Chi tiết một trang: npm run seo -- <slug>'));
