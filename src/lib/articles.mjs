// Lớp "Bài viết": bài SEO/GEO để kéo traffic, DẪN XUẤT từ ghi chú tri thức.
//
// Nguyên tắc: tri thức (knowledge/) là nguồn sự thật. Bài viết chỉ trình bày lại tri thức theo câu hỏi
// người đọc hay tìm — mọi dữ kiện phải lấy từ ghi chú gốc (based_on) và dẫn [[liên kết]] về ghi chú đó.
// Ghi chú không liên kết ngược sang bài viết; site tự hiện "Bài viết về chủ đề này" trên trang ghi chú.
//
// Không phụ thuộc Astro/Node fs — nhận danh sách { path, raw } như kb.mjs.
import { parse as parseYaml } from 'yaml';
import { z } from 'zod';
import { splitFrontmatter } from './kb.mjs';
import { blankToUndefined, dateString, optional, stringList } from './schema.mjs';
import { countWords, extractWikilinks, keywordCore, slugify } from './text.mjs';

export const ARTICLES_DIR = 'articles';

/** Ý định tìm kiếm (search intent) mà bài viết nhắm tới. */
export const ARTICLE_INTENTS = {
  guide: { label: 'Cẩm nang', description: 'Bài tổng quan cho người mới, dẫn qua nhiều ghi chú của một chủ đề.' },
  'how-to': { label: 'Hướng dẫn', description: 'Các bước làm một việc cụ thể.' },
  compare: { label: 'So sánh', description: 'X và Y khác nhau thế nào, khi nào dùng cái nào.' },
  list: { label: 'Tổng hợp', description: 'Danh sách: sai lầm thường gặp, mẹo, ví dụ, câu hỏi…' },
  explain: { label: 'Giải thích', description: 'Giải thích dễ hiểu một vấn đề — tránh trùng truy vấn định nghĩa mà ghi chú gốc đã trả lời.' },
};

export const ARTICLE_STATUSES = {
  draft: { label: 'Bản nháp', description: 'Chưa đăng — chỉ hiện khi chạy local để xem trước.' },
  published: { label: 'Đã đăng', description: 'Hiện trên web.' },
};

export const ARTICLE_FIELDS = ['title', 'description', 'keyword', 'keywords', 'intent', 'based_on', 'status', 'created', 'updated', 'published_on'];

const INTENT_KEYS = /** @type {[string, ...string[]]} */ (Object.keys(ARTICLE_INTENTS));
const STATUS_KEYS = /** @type {[string, ...string[]]} */ (Object.keys(ARTICLE_STATUSES));

export const articleSchema = z.looseObject({
  title: z.string().min(1, 'Thiếu title'),
  description: z.string().min(1, 'Thiếu description (mô tả hiện trên Google, 1–2 câu)'),
  keyword: z.string().min(1, 'Thiếu keyword (từ khoá chính của bài)'),
  keywords: stringList,
  intent: z.preprocess(blankToUndefined, z.enum(INTENT_KEYS, { message: `intent phải là một trong: ${INTENT_KEYS.join(', ')}` }).default('guide')),
  based_on: stringList.refine((list) => list.length > 0, 'based_on cần ít nhất 1 ghi chú tri thức gốc (slug)'),
  status: z.preprocess(blankToUndefined, z.enum(STATUS_KEYS, { message: `status phải là một trong: ${STATUS_KEYS.join(', ')}` }).default('draft')),
  created: dateString,
  updated: optional(dateString),
  published_on: optional(dateString),
});

/** File bài viết: articles/**.md, bỏ README và thư mục/tên bắt đầu bằng "_" hoặc ".". */
export function isArticlePath(path) {
  const segments = path.split('/');
  const name = segments.at(-1) ?? '';
  if (segments[0] !== ARTICLES_DIR || segments.length < 2) return false;
  if (!name.toLowerCase().endsWith('.md') || name.toLowerCase() === 'readme.md') return false;
  return !segments.slice(1).some((s) => s.startsWith('_') || s.startsWith('.'));
}

/** @returns {{ ok: true, article: object, warnings: string[] } | { ok: false, errors: string[] }} */
export function parseArticle({ path, raw }) {
  const slug = path.split('/').at(-1).replace(/\.md$/i, '');
  const { yaml, body } = splitFrontmatter(raw);
  if (yaml === undefined) return { ok: false, errors: ['Không có frontmatter (khối --- ... --- ở đầu file)'] };
  let data;
  try {
    data = parseYaml(yaml) ?? {};
  } catch (e) {
    return { ok: false, errors: [`YAML lỗi: ${e.message.split('\n')[0]}`] };
  }
  if (typeof data !== 'object' || Array.isArray(data)) return { ok: false, errors: ['Frontmatter phải là danh sách key: value'] };
  const result = articleSchema.safeParse(data);
  if (!result.success) {
    return { ok: false, errors: result.error.issues.map((i) => `${i.path.length ? i.path.join('.') : '(frontmatter)'}: ${i.message}`) };
  }
  const fm = result.data;
  const warnings = [];
  for (const key of Object.keys(fm)) {
    if (!ARTICLE_FIELDS.includes(key)) warnings.push(`Trường lạ "${key}" (gõ nhầm?). Trường hợp lệ: ${ARTICLE_FIELDS.join(', ')}`);
  }
  if (slug !== slugify(slug)) warnings.push(`Tên file nên là slug không dấu, chữ thường, nối bằng "-" (gợi ý: ${slugify(slug)}.md)`);

  const description = fm.description.trim();
  return {
    ok: true,
    warnings,
    article: {
      kind: 'article',
      slug,
      path,
      title: fm.title,
      description,
      /** Để dùng chung renderer/tooltip với ghi chú. */
      summary: description,
      keyword: fm.keyword.trim(),
      keywords: fm.keywords,
      intent: fm.intent,
      basedOn: fm.based_on,
      status: fm.status,
      created: fm.created,
      updated: fm.updated ?? fm.created,
      publishedOn: fm.published_on ?? null,
      body: body.replace(/^\s*\n/, ''),
      wordCount: countWords(body),
      /** Ghi chú gốc (object) theo based_on. */
      notes: [],
      /** Slug ghi chú được [[liên kết]] trong bài. */
      links: [],
      /** Slug bài viết khác được [[liên kết]] trong bài. */
      articleLinks: [],
      brokenLinks: [],
    },
  };
}

/**
 * Dựng danh sách bài viết và kiểm tra chúng dựa trên kho tri thức.
 * @param {{ path: string, raw: string }[]} files
 * @param {ReturnType<import('./kb.mjs').buildKnowledge>} kb
 * @param {{ includeDrafts?: boolean }} [options] includeDrafts: hiện cả bài nháp (khi chạy dev)
 */
export function buildArticles(files, kb, { includeDrafts = false } = {}) {
  const issues = []; // { path, level, message, inbox } — inbox = bài nháp (lỗi không chặn build)
  const all = [];
  const bySlug = new Map();
  const add = (path, level, message, draft) => issues.push({ path, level, message, inbox: draft });

  for (const file of files.filter((f) => isArticlePath(f.path)).sort((a, b) => a.path.localeCompare(b.path))) {
    const parsed = parseArticle(file);
    if (!parsed.ok) {
      const looksPublished = /^status:\s*['"]?published/m.test(file.raw);
      for (const message of parsed.errors) add(file.path, 'error', message, !looksPublished);
      continue;
    }
    const { article, warnings } = parsed;
    const draft = article.status !== 'published';
    for (const message of warnings) add(file.path, 'warning', message, draft);
    if (bySlug.has(article.slug)) {
      add(file.path, 'error', `Trùng tên file "${article.slug}.md" với ${bySlug.get(article.slug).path}`, draft);
      continue;
    }
    bySlug.set(article.slug, article);
    all.push(article);
  }

  const byTitle = new Map(all.map((a) => [slugify(a.title), a]));
  const resolveAny = (target) => {
    const t = String(target).trim();
    return bySlug.get(t) ?? bySlug.get(slugify(t)) ?? byTitle.get(slugify(t)) ?? null;
  };

  for (const a of all) {
    const draft = a.status !== 'published';
    // Tri thức gốc
    for (const ref of a.basedOn) {
      const note = kb.resolve(ref);
      if (!note) {
        add(a.path, 'error', `based_on: không có ghi chú "${ref}" trong kho — nghiên cứu (/research) và đăng ghi chú trước`, draft);
        continue;
      }
      if (note.inbox) add(a.path, draft ? 'warning' : 'error', `based_on: "${note.title}" còn chờ duyệt trong _inbox — đăng ghi chú trước khi đăng bài`, draft);
      else if (note.status !== 'verified') add(a.path, 'warning', `Ghi chú gốc "${note.title}" chưa kiểm chứng (status: ${note.status}) — duyệt tri thức trước khi đăng bài`, draft);
      if (!a.notes.includes(note)) a.notes.push(note);
    }
    // Liên kết trong bài: ưu tiên ghi chú, sau đó bài viết khác
    const links = new Set();
    const articleLinks = new Set();
    const broken = new Set();
    for (const link of extractWikilinks(a.body)) {
      const note = kb.resolve(link.target);
      if (note) links.add(note.slug);
      else {
        const other = resolveAny(link.target);
        if (other && other !== a) articleLinks.add(other.slug);
        else if (!other) broken.add(link.target);
      }
    }
    a.links = [...links];
    a.articleLinks = [...articleLinks];
    a.brokenLinks = [...broken];
    if (a.brokenLinks.length) {
      add(a.path, 'warning', `Liên kết tới tri thức chưa có: ${a.brokenLinks.map((b) => `[[${b}]]`).join(', ')} — bài viết chỉ nên dẫn tới ghi chú đã có`, draft);
    }
    if (a.notes.length && !a.notes.some((n) => links.has(n.slug))) {
      add(a.path, 'warning', 'Chưa đặt [[liên kết]] nào tới ghi chú gốc trong nội dung — dẫn người đọc (và Google) về tri thức gốc', draft);
    }
  }

  // Tránh "ăn thịt từ khoá" (keyword cannibalization): mỗi từ khoá chính chỉ một trang.
  const byKeyword = new Map();
  for (const a of all) {
    const key = keywordCore(a.keyword);
    if (!byKeyword.has(key)) byKeyword.set(key, []);
    byKeyword.get(key).push(a);
  }
  for (const group of byKeyword.values()) {
    if (group.length < 2) continue;
    for (const a of group) {
      const others = group.filter((x) => x !== a).map((x) => x.slug).join(', ');
      add(a.path, 'warning', `Trùng từ khoá chính "${a.keyword}" với: ${others} — gộp bài hoặc đổi góc (intent) để không tự cạnh tranh trên Google`, a.status !== 'published');
    }
  }
  const noteNames = new Map();
  for (const n of kb.notes) for (const name of [n.title, ...n.aliases]) if (!noteNames.has(keywordCore(name))) noteNames.set(keywordCore(name), n);
  for (const a of all) {
    const owner = noteNames.get(keywordCore(a.keyword));
    if (owner) {
      add(
        a.path,
        'warning',
        `Từ khoá "${a.keyword}" trùng tên ghi chú "${owner.title}" — để ghi chú giữ truy vấn định nghĩa; bài viết nên nhắm truy vấn khác (hướng dẫn, so sánh, sai lầm thường gặp…)`,
        a.status !== 'published',
      );
    }
  }

  const sortList = (list) => [...list].sort((x, y) => y.updated.localeCompare(x.updated) || x.title.localeCompare(y.title, 'vi'));
  const published = sortList(all.filter((a) => a.status === 'published'));
  const drafts = sortList(all.filter((a) => a.status !== 'published'));
  const visible = includeDrafts ? [...published, ...drafts] : published;
  const visibleSet = new Set(visible);

  return {
    /** Bài đã đăng (lên web). */
    list: published,
    /** Bài nháp (chỉ thấy khi includeDrafts). */
    drafts: includeDrafts ? drafts : [],
    /** Bài được dựng trang (đã đăng + nháp khi dev). */
    all: visible,
    issues,
    get: (slug) => (visibleSet.has(bySlug.get(slug)) ? bySlug.get(slug) : null),
    resolve: (target) => {
      const a = resolveAny(target);
      return a && visibleSet.has(a) ? a : null;
    },
    /** Bài viết dựa trên hoặc liên kết tới một ghi chú. */
    forNote: (noteSlug) => visible.filter((a) => a.notes.some((n) => n.slug === noteSlug) || a.links.includes(noteSlug)),
    /** Bài viết khác có chung tri thức gốc. */
    related: (article, limit = 4) =>
      visible
        .filter((x) => x !== article)
        .map((x) => ({ x, shared: x.notes.filter((n) => article.notes.includes(n)).length + (article.articleLinks.includes(x.slug) ? 2 : 0) }))
        .filter((r) => r.shared > 0)
        .sort((p, q) => q.shared - p.shared)
        .slice(0, limit)
        .map((r) => r.x),
  };
}
