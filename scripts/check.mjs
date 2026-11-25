// Kiểm tra toàn bộ kho tri thức: schema, liên kết, chất lượng nguồn, gợi ý kết nối.
// Dùng: npm run check            (lỗi ở ghi chú đã đăng → exit 1; bản nháp trong _inbox chỉ cảnh báo)
//       npm run check -- --inbox (coi lỗi trong _inbox cũng là lỗi — dùng trước khi publish)
//       npm run check -- --verbose (liệt kê mọi gợi ý liên kết)
import { c, parseArgs } from './args.mjs';
import { loadKnowledge } from './load.mjs';

const { flags } = parseArgs();
const kb = loadKnowledge({ includeInbox: true });

const byFile = new Map();
for (const issue of kb.issues) {
  if (!byFile.has(issue.path)) byFile.set(issue.path, []);
  byFile.get(issue.path).push(issue);
}

let errors = 0;
let warnings = 0;
for (const [file, issues] of [...byFile].sort()) {
  console.log(`\n${c.bold(file)}${issues[0].inbox ? c.cyan(' (chờ duyệt)') : ''}`);
  for (const i of issues) {
    const fatal = i.level === 'error' && (!i.inbox || flags.inbox);
    if (fatal) errors++;
    else warnings++;
    const tag = i.level === 'error' ? (fatal ? c.red('  lỗi     ') : c.yellow('  lỗi*    ')) : c.yellow('  cảnh báo');
    console.log(`${tag} ${i.message}`);
  }
}

const mentions = kb.mentions();
const orphans = kb.orphans();
console.log(`\n${c.bold('Tổng quan')}`);
console.log(`  ${kb.notes.length} ghi chú đã đăng, ${kb.inbox.length} bản nháp chờ duyệt, ${kb.tags.length} tag`);
console.log(`  ${kb.notes.filter((n) => n.status === 'verified').length} verified · ${kb.notes.filter((n) => n.status === 'draft').length} draft · ${kb.notes.filter((n) => n.status === 'outdated').length} outdated`);

if (kb.gaps.length) {
  console.log(`\n${c.bold(`Tri thức còn thiếu (${kb.gaps.length})`)} ${c.dim('— [[liên kết]] chưa có ghi chú, nên nghiên cứu tiếp')}`);
  for (const g of kb.gaps.slice(0, flags.verbose ? Infinity : 10)) console.log(`  [[${g.name}]] ${c.dim(`← ${g.from.join(', ')}`)}`);
  if (!flags.verbose && kb.gaps.length > 10) console.log(c.dim(`  … và ${kb.gaps.length - 10} mục khác (--verbose)`));
}
if (orphans.length) {
  console.log(`\n${c.bold(`Ghi chú cô lập (${orphans.length})`)} ${c.dim('— không liên kết tới/được liên kết từ ghi chú nào')}`);
  for (const n of orphans) console.log(`  ${n.slug} ${c.dim(n.path)}`);
}
if (mentions.length) {
  console.log(`\n${c.bold(`Gợi ý liên kết (${mentions.length})`)} ${c.dim('— nhắc tên ghi chú khác nhưng chưa đặt [[liên kết]]')}`);
  for (const m of mentions.slice(0, flags.verbose ? Infinity : 10)) console.log(`  ${m.from} → [[${m.to}]] ${c.dim(`"…${m.context}…"`)}`);
  if (!flags.verbose && mentions.length > 10) console.log(c.dim(`  … và ${mentions.length - 10} gợi ý khác (--verbose)`));
}

console.log('');
if (errors) {
  console.log(c.red(`✗ ${errors} lỗi, ${warnings} cảnh báo`));
  process.exit(1);
}
console.log(c.green(`✓ Không có lỗi${warnings ? ` (${warnings} cảnh báo)` : ''}`));
if (kb.issues.some((i) => i.level === 'error' && i.inbox)) console.log(c.dim('  lỗi* = lỗi trong bản nháp _inbox, không chặn build; sửa trước khi publish.'));
