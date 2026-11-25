// Giữ kho Second Brain nhẹ: Git giữ mọi phiên bản mãi mãi, nên file lớn đã commit thì không thu nhỏ lại được.
// Kiểm các file Git đang theo dõi hoặc sắp thêm (bỏ qua file trong .gitignore):
// - knowledge/ và articles/ chỉ chứa Markdown (.md) — trừ cấu hình Obsidian (knowledge/.obsidian/*.json).
//   Ảnh, PDF, bản lưu trang web… không bỏ vào repo; ghi link trong `sources`.
// - Mỗi ghi chú/bài viết tối đa 40 KB (cảnh báo từ 25 KB: nên tách thành ghi chú con).
// - Mọi file trong repo tối đa 300 KB (cảnh báo từ 150 KB).
// - Tổng dung lượng file đang theo dõi: cảnh báo khi vượt 20 MB.
import { spawnSync } from 'node:child_process';
import { statSync, existsSync } from 'node:fs';
import { resolve, dirname, extname } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const KB = 1024;
const LIMIT = { note: 40 * KB, noteWarn: 25 * KB, file: 300 * KB, fileWarn: 150 * KB, totalWarn: 20 * 1024 * KB };

const git = spawnSync('git', ['ls-files', '--cached', '--others', '--exclude-standard', '-z'], { cwd: root, encoding: 'utf8', shell: false });
if (git.status !== 0) { console.error(`Không đọc được danh sách file từ Git:\n${git.stderr || git.error?.message}`); process.exit(1); }
const files = git.stdout.split('\0').filter(Boolean).filter(f => existsSync(resolve(root, f)));

const errors = [], warnings = [];
let total = 0;
const kb = n => `${(n / KB).toFixed(1)} KB`;
for (const f of files) {
  const size = statSync(resolve(root, f)).size;
  total += size;
  const content = f.startsWith('knowledge/') || f.startsWith('articles/');
  const allowed = /(^|\/)\.gitkeep$/.test(f) || /^knowledge\/\.obsidian\/[^/]+\.json$/.test(f);
  if (content && extname(f) !== '.md' && !allowed) {
    errors.push(`${f}: chỉ để file .md trong knowledge/ và articles/ — ảnh/PDF/bản lưu trang thì ghi link trong sources, đừng bỏ vào repo`);
    continue;
  }
  if (content && extname(f) === '.md') {
    if (size > LIMIT.note) errors.push(`${f}: ${kb(size)} > ${kb(LIMIT.note)} — tách thành chủ đề + ghi chú con`);
    else if (size > LIMIT.noteWarn) warnings.push(`${f}: ${kb(size)} — khá dài, nên tách thành ghi chú con`);
  } else if (size > LIMIT.file) errors.push(`${f}: ${kb(size)} > ${kb(LIMIT.file)} — file quá lớn cho repo`);
  else if (size > LIMIT.fileWarn) warnings.push(`${f}: ${kb(size)} — file lớn, kiểm tra có cần đưa vào repo không`);
}
if (total > LIMIT.totalWarn) warnings.push(`Tổng ${kb(total)} file đang theo dõi — vượt mức ${kb(LIMIT.totalWarn)}, rà lại file lớn`);

for (const w of warnings) console.log(`⚠ ${w}`);
for (const e of errors) console.error(`✗ ${e}`);
console.log(`${files.length} file, tổng ${kb(total)}; ${errors.length} lỗi, ${warnings.length} cảnh báo`);
process.exit(errors.length ? 1 : 0);
