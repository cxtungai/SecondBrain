// Đăng một bản nháp đã duyệt: chuyển từ knowledge/_inbox sang thư mục đúng loại, đánh dấu đã duyệt,
// rồi gợi ý các liên kết cần thêm để tri thức kết nối chặt chẽ.
// Dùng: npm run publish                       (liệt kê bản nháp đang chờ)
//       npm run publish -- <file|slug>         (đăng, status mặc định: verified)
//       npm run publish -- <slug> --status draft   (đăng nhưng giữ trạng thái nháp)
//       npm run publish -- <slug> --dry-run    (xem trước, không ghi file)
//       npm run publish -- articles/<file>.md  (đăng bài viết SEO/GEO — cần tri thức gốc đã verified, hoặc --force)
import fs from 'node:fs';
import path from 'node:path';
import { parseDocument } from 'yaml';
import { buildArticles } from '../src/lib/articles.mjs';
import { buildKnowledge, parseNote, splitFrontmatter } from '../src/lib/kb.mjs';
import { articleSeoHints } from '../src/lib/seo.mjs';
import { NOTE_TYPES, STATUSES } from '../src/lib/schema.mjs';
import { c, parseArgs, today } from './args.mjs';
import { ARTICLES_ABS, INBOX_ABS, KNOWLEDGE_DIR, ROOT, readArticleFiles, readKnowledgeFiles, toRepoPath } from './load.mjs';

const { positional, flags } = parseArgs();
const arg = positional[0];

const inboxFiles = fs.existsSync(INBOX_ABS) ? fs.readdirSync(INBOX_ABS).filter((f) => f.endsWith('.md')) : [];
if (!arg) {
  const draftArticles = buildArticles(readArticleFiles(), buildKnowledge(readKnowledgeFiles()), { includeDrafts: true }).drafts;
  if (!inboxFiles.length) console.log('Không có bản nháp nào trong knowledge/_inbox.');
  else {
    console.log(c.bold('Bản nháp chờ duyệt:'));
    for (const f of inboxFiles) console.log(`  ${f}`);
    console.log(c.dim('\nĐăng: npm run publish -- <tên-file>'));
  }
  if (draftArticles.length) {
    console.log(c.bold('\nBài viết nháp:'));
    for (const a of draftArticles) console.log(`  ${a.path}`);
    console.log(c.dim('\nĐăng: npm run publish -- articles/<tên-file>.md'));
  }
  process.exit(0);
}

// ---------- Bài viết SEO/GEO ----------
const articleCandidates = [path.resolve(ROOT, arg), path.join(ARTICLES_ABS, arg), path.join(ARTICLES_ABS, `${arg}.md`)];
const articleSource = articleCandidates.find((p) => p.startsWith(ARTICLES_ABS + path.sep) && fs.existsSync(p) && fs.statSync(p).isFile());
if (articleSource) {
  publishArticle(articleSource);
  process.exit(0);
}

function publishArticle(abs) {
  const repoPath = toRepoPath(abs);
  const raw = fs.readFileSync(abs, 'utf8');
  const { yaml, body } = splitFrontmatter(raw);
  if (yaml === undefined) {
    console.error(c.red(`${repoPath}: không có frontmatter`));
    process.exit(1);
  }
  const doc = parseDocument(yaml);
  doc.set('status', 'published');
  doc.set('updated', today());
  if (!doc.get('published_on')) doc.set('published_on', today());
  const output = `---\n${doc.toString().trimEnd()}\n---\n\n${body.replace(/^\s*\n/, '')}`;

  // Kiểm tra như thể đã đăng (dựa trên tri thức đã đăng, không tính _inbox).
  const kb = buildKnowledge(readKnowledgeFiles());
  const posts = buildArticles([...readArticleFiles().filter((f) => f.path !== repoPath), { path: repoPath, raw: output }], kb);
  const own = posts.issues.filter((i) => i.path === repoPath);
  const article = posts.list.find((a) => a.path === repoPath);
  const blocking = own.filter((i) => i.level === 'error');
  if (blocking.length || !article) {
    console.error(c.red('Chưa thể đăng bài — còn lỗi:'));
    for (const i of blocking) console.error(`  - ${i.message}`);
    process.exit(1);
  }
  const unverified = article.notes.filter((n) => n.status !== 'verified');
  if (unverified.length && !flags.force) {
    console.error(c.red('Chưa thể đăng bài — tri thức gốc chưa được kiểm chứng:'));
    for (const n of unverified) console.error(`  - ${n.title} (${n.path}, status: ${n.status})`);
    console.error(c.dim('  Duyệt ghi chú gốc trước (npm run publish -- <ghi-chú>), hoặc thêm --force nếu chấp nhận đăng khi tri thức còn là nháp.'));
    process.exit(1);
  }

  console.log(`Đăng bài viết: ${c.bold(article.title)}  ${c.dim(`(${repoPath}, status: published, updated: ${today()})`)}`);
  for (const i of own) console.log(c.yellow(`  cảnh báo: ${i.message}`));
  const hints = articleSeoHints(article);
  const todo = hints.filter((h) => !h.ok);
  console.log(`  SEO/GEO: ${hints.length - todo.length}/${hints.length} tiêu chí đạt`);
  for (const h of todo) console.log(c.yellow(`  ○ ${h.label}`) + c.dim(` — ${h.tip}`));

  if (flags['dry-run']) console.log(c.dim('(dry-run: chưa ghi file)'));
  else {
    fs.writeFileSync(abs, output);
    console.log(c.green('✓ Đã đăng'));
  }
  console.log(c.dim(`Tri thức gốc: ${article.notes.map((n) => n.slug).join(', ')} — trang ghi chú sẽ tự hiện "Bài viết về chủ đề này".`));
  console.log(c.dim('Xong thì commit & push lên nhánh chính để site tự cập nhật.'));
}

const candidates = [path.resolve(ROOT, arg), path.join(INBOX_ABS, arg), path.join(INBOX_ABS, `${arg}.md`)];
const source = candidates.find((p) => fs.existsSync(p) && fs.statSync(p).isFile());
if (!source) {
  console.error(c.red(`Không tìm thấy bản nháp "${arg}" trong knowledge/_inbox`));
  process.exit(1);
}
const sourcePath = toRepoPath(source);
const raw = fs.readFileSync(source, 'utf8');
const parsed = parseNote({ path: sourcePath, raw });
if (!parsed.ok) {
  console.error(c.red(`Bản nháp chưa hợp lệ (${sourcePath}):`));
  for (const e of parsed.errors) console.error(`  - ${e}`);
  process.exit(1);
}
const draft = parsed.note;

// Ghi chú đã đăng trùng tên → đây là bản cập nhật: ghi đè đúng vị trí cũ, giữ ngày tạo.
const files = readKnowledgeFiles();
const publishedKb = buildKnowledge(files);
const existing = publishedKb.get(draft.slug);
const targetAbs = existing ? path.join(ROOT, existing.path) : path.join(KNOWLEDGE_DIR, NOTE_TYPES[draft.type].folder, `${draft.slug}.md`);
const targetPath = toRepoPath(targetAbs);

const status = flags.status ?? 'verified';
if (!STATUSES[status]) {
  console.error(c.red(`--status phải là: ${Object.keys(STATUSES).join(', ')}`));
  process.exit(1);
}

const { yaml, body } = splitFrontmatter(raw);
const doc = parseDocument(yaml);
doc.set('status', status);
doc.set('updated', today());
doc.set('reviewed', today());
if (existing) doc.set('created', existing.created);
const output = `---\n${doc.toString().trimEnd()}\n---\n\n${body.replace(/^\s*\n/, '')}`;

// Kiểm tra lại toàn kho như thể đã đăng.
const simulated = buildKnowledge([
  ...files.filter((f) => f.path !== sourcePath && f.path !== targetPath),
  { path: targetPath, raw: output },
]);
const own = simulated.issues.filter((i) => i.path === targetPath);
const blocking = own.filter((i) => i.level === 'error');
if (blocking.length) {
  console.error(c.red('Chưa thể đăng — còn lỗi:'));
  for (const i of blocking) console.error(`  - ${i.message}`);
  process.exit(1);
}

console.log(`${existing ? 'Cập nhật' : 'Đăng mới'}: ${c.bold(draft.title)}`);
console.log(`  ${sourcePath} → ${targetPath}  (status: ${status}, reviewed: ${today()})`);
for (const i of own) console.log(c.yellow(`  cảnh báo: ${i.message}`));

if (!flags['dry-run']) {
  fs.mkdirSync(path.dirname(targetAbs), { recursive: true });
  fs.writeFileSync(targetAbs, output);
  if (path.resolve(source) !== path.resolve(targetAbs)) fs.unlinkSync(source);
  console.log(c.green('✓ Đã đăng'));
} else {
  console.log(c.dim('(dry-run: chưa ghi file)'));
}

// Gợi ý kết nối
const note = simulated.get(draft.slug);
const mentions = simulated.mentions();
const outgoing = mentions.filter((m) => m.from === note.slug);
const incoming = mentions.filter((m) => m.to === note.slug);
const topicsToJoin = simulated
  .byType('topic')
  .filter((t) => t.slug !== note.slug && !t.links.includes(note.slug) && t.tags.some((tag) => note.tags.includes(tag)));

console.log(`\n${c.bold('Kết nối tri thức')}`);
console.log(`  Liên kết tới ${note.links.length} ghi chú · được ${note.backlinks.length} ghi chú liên kết tới`);
if (note.backlinks.length === 0) console.log(c.yellow('  ⚠ Chưa ghi chú nào liên kết tới đây — thêm vào một ghi chú chủ đề hoặc mục "Liên quan" của ghi chú gần nghĩa.'));
if (note.brokenLinks.length) console.log(`  Còn thiếu (nên nghiên cứu tiếp): ${note.brokenLinks.map((b) => `[[${b}]]`).join(', ')}`);
for (const m of outgoing) console.log(`  Nên thêm [[${m.to}]] trong ghi chú này: "…${m.context}…"`);
for (const m of incoming) console.log(`  Nên thêm [[${note.slug}]] trong ${m.from}: "…${m.context}…"`);
for (const t of topicsToJoin) console.log(`  Chủ đề cùng tag có thể bổ sung ghi chú này: ${t.slug} (${t.path})`);
console.log(c.dim('\nXong thì commit & push lên nhánh chính để site tự cập nhật.'));
