// Tạo ghi chú mới từ template.
// Dùng: npm run new -- <type> "Tiêu đề" [--tags a,b] [--inbox] [--slug ten-file]
//   type: topic | concept | fact | framework | case-study | source
//   --inbox: tạo trong knowledge/_inbox (bản nháp chờ duyệt, chưa lên web)
// Bài viết SEO/GEO: npm run new -- article "Tiêu đề" [--keyword "từ khoá"] [--based-on slug-a,slug-b] [--intent how-to]
import fs from 'node:fs';
import path from 'node:path';
import { NOTE_TYPES } from '../src/lib/schema.mjs';
import { slugify } from '../src/lib/text.mjs';
import { c, parseArgs, today } from './args.mjs';
import { ARTICLES_ABS, INBOX_ABS, KNOWLEDGE_DIR, loadArticles, loadKnowledge, toRepoPath } from './load.mjs';
import { ARTICLE_INTENTS } from '../src/lib/articles.mjs';

const { positional, flags } = parseArgs();
const [type, ...titleParts] = positional;
const title = titleParts.join(' ').trim();

if (type === 'article' && title) {
  const slug = slugify(flags.slug || title);
  const kb = loadKnowledge({ includeInbox: true });
  const posts = loadArticles(kb, { includeDrafts: true });
  if (posts.get(slug)) {
    console.error(c.red(`Đã có bài viết "${slug}": ${posts.get(slug).path}`));
    process.exit(1);
  }
  const intent = flags.intent ? String(flags.intent) : 'guide';
  if (!ARTICLE_INTENTS[intent]) {
    console.error(c.red(`--intent phải là: ${Object.keys(ARTICLE_INTENTS).join(', ')}`));
    process.exit(1);
  }
  const basedOn = flags['based-on'] ? String(flags['based-on']).split(',').map((s) => s.trim()).filter(Boolean) : [];
  const missing = basedOn.filter((s) => !kb.resolve(s));
  if (missing.length) console.log(c.yellow(`  Chưa có ghi chú: ${missing.join(', ')} — nghiên cứu (/research) trước khi viết bài.`));
  const q = (s) => `"${String(s).replace(/"/g, '\\"')}"`;
  let content = fs.readFileSync(path.join(KNOWLEDGE_DIR, '_templates', 'article.md'), 'utf8')
    .replaceAll('{{title}}', title.replace(/"/g, '\\"'))
    .replaceAll('{{date}}', today())
    .replace(/^intent: guide$/m, `intent: ${intent}`)
    .replace(/^based_on: \[\]$/m, `based_on: [${basedOn.join(', ')}]`);
  if (flags.keyword) content = content.replace(/^keyword: ""$/m, `keyword: ${q(flags.keyword)}`);
  const file = path.join(ARTICLES_ABS, `${slug}.md`);
  if (fs.existsSync(file)) {
    console.error(c.red(`File đã tồn tại: ${toRepoPath(file)}`));
    process.exit(1);
  }
  fs.mkdirSync(ARTICLES_ABS, { recursive: true });
  fs.writeFileSync(file, content);
  console.log(c.green(`✓ Đã tạo ${toRepoPath(file)} (bài nháp)`));
  console.log(c.dim('  Điền description, keyword, based_on; kiểm tra trùng từ khoá: npm run seo -- --keyword "..."'));
  process.exit(0);
}

if (!type || !NOTE_TYPES[type] || !title) {
  console.log(`Dùng: npm run new -- <type> "Tiêu đề" [--tags a,b] [--inbox]\n\nCác type:`);
  for (const [k, v] of Object.entries(NOTE_TYPES)) console.log(`  ${k.padEnd(11)} ${v.label} — ${v.description}`);
  console.log(`  ${'article'.padEnd(11)} Bài viết SEO/GEO (thư mục articles/) — thêm --keyword "…" --based-on slug-a,slug-b`);
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
