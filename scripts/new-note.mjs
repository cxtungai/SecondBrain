// Tạo ghi chú mới từ template.
// Dùng: npm run new -- <type> "Tiêu đề" [--tags a,b] [--inbox] [--slug ten-file]
//   type: topic | concept | fact | framework | case-study | source
//   --inbox: tạo trong knowledge/_inbox (bản nháp chờ duyệt, chưa lên web)
import fs from 'node:fs';
import path from 'node:path';
import { NOTE_TYPES } from '../src/lib/schema.mjs';
import { slugify } from '../src/lib/text.mjs';
import { c, parseArgs, today } from './args.mjs';
import { INBOX_ABS, KNOWLEDGE_DIR, loadKnowledge, toRepoPath } from './load.mjs';

const { positional, flags } = parseArgs();
const [type, ...titleParts] = positional;
const title = titleParts.join(' ').trim();

if (!type || !NOTE_TYPES[type] || !title) {
  console.log(`Dùng: npm run new -- <type> "Tiêu đề" [--tags a,b] [--inbox]\n\nCác type:`);
  for (const [k, v] of Object.entries(NOTE_TYPES)) console.log(`  ${k.padEnd(11)} ${v.label} — ${v.description}`);
  process.exit(1);
}

const slug = slugify(flags.slug || title);
const kb = loadKnowledge({ includeInbox: true });
const existing = kb.all.find((n) => n.slug === slug || n.updates === slug);
if (existing && !flags.inbox) {
  console.error(c.red(`Đã có ghi chú "${slug}": ${existing.path}`));
  process.exit(1);
}

const dir = flags.inbox ? INBOX_ABS : path.join(KNOWLEDGE_DIR, NOTE_TYPES[type].folder);
const file = path.join(dir, `${slug}.md`);
if (fs.existsSync(file)) {
  console.error(c.red(`File đã tồn tại: ${toRepoPath(file)}`));
  process.exit(1);
}

let content = fs.readFileSync(path.join(KNOWLEDGE_DIR, '_templates', `${type}.md`), 'utf8')
  .replaceAll('{{title}}', title.replace(/"/g, '\\"'))
  .replaceAll('{{date}}', today());
if (flags.tags) {
  const tags = String(flags.tags).split(',').map((t) => t.trim()).filter(Boolean);
  content = content.replace(/^tags: \[\]$/m, `tags: [${tags.join(', ')}]`);
}

fs.mkdirSync(dir, { recursive: true });
fs.writeFileSync(file, content);
console.log(c.green(`✓ Đã tạo ${toRepoPath(file)}`));
console.log(c.dim('  Nhớ điền summary (1–2 câu) và nguồn (sources) trước khi đăng.'));
