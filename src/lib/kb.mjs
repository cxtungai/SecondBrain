// Lõi kho tri thức: parse ghi chú, kiểm tra schema, dựng đồ thị liên kết, đánh giá chất lượng.
// Không phụ thuộc Astro/Node fs — nhận vào danh sách { path, raw } để dùng được cả
// trong site (import.meta.glob) lẫn scripts (đọc đĩa).
import { parse as parseYaml } from 'yaml';
import { findUnlinkedMentions } from './mentions.mjs';
import { frontmatterSchema, KNOWN_FIELDS, NOTE_TYPES } from './schema.mjs';
import { countWords, extractWikilinks, normalizeForSearch, slugify } from './text.mjs';

const FRONTMATTER_RE = /^﻿?---\r?\n([\s\S]*?)\r?\n---[ \t]*(?:\r?\n|$)/;

/** Thư mục chứa bản nháp do AI agent nghiên cứu, chờ người dùng duyệt (không đăng lên web). */
export const INBOX_DIR = 'knowledge/_inbox';

export const isInboxPath = (path) => path.startsWith(INBOX_DIR + '/');

/**
 * File nào được coi là ghi chú: .md, không nằm trong thư mục/tên bắt đầu bằng "_" hoặc ".", bỏ README.
 * Ngoại lệ: knowledge/_inbox/ khi includeInbox = true.
 */
export function isNotePath(path, { includeInbox = false } = {}) {
  const segments = path.split('/');
  const name = segments.at(-1) ?? '';
  if (!name.toLowerCase().endsWith('.md')) return false;
  if (name.toLowerCase() === 'readme.md') return false;
  return !segments.some((s, i) => {
    if (includeInbox && i === 1 && s === '_inbox') return false;
    return s.startsWith('_') || s.startsWith('.');
  });
}

/** Tách frontmatter YAML và nội dung. */
export function splitFrontmatter(raw) {
  const match = FRONTMATTER_RE.exec(raw);
  if (!match) return { yaml: undefined, body: raw };
  return { yaml: match[1], body: raw.slice(match[0].length) };
}

function formatZodIssues(error) {
  return error.issues.map((issue) => {
    const where = issue.path.length ? issue.path.join('.') : '(frontmatter)';
    return `${where}: ${issue.message}`;
  });
}

/** Tóm tắt độ uy tín nguồn của một ghi chú. */
export function credibilitySummary(sources) {
  const scores = sources.map((s) => s.credibility).filter((c) => typeof c === 'number');
  if (!scores.length) return { rated: 0, total: sources.length, average: null, max: null, min: null };
  const average = Math.round((scores.reduce((a, b) => a + b, 0) / scores.length) * 10) / 10;
  return { rated: scores.length, total: sources.length, average, max: Math.max(...scores), min: Math.min(...scores) };
}

/**
 * Parse một file ghi chú.
 * @returns {{ ok: true, note: object, warnings: string[] } | { ok: false, errors: string[] }}
 */
export function parseNote({ path, raw }) {
  const name = path.split('/').at(-1);
  const slug = name.replace(/\.md$/i, '');
  const inbox = isInboxPath(path);
  const { yaml, body } = splitFrontmatter(raw);
  if (yaml === undefined) return { ok: false, errors: ['Không có frontmatter (khối --- ... --- ở đầu file)'] };

  let data;
  try {
    data = parseYaml(yaml) ?? {};
  } catch (e) {
    return { ok: false, errors: [`YAML lỗi: ${e.message.split('\n')[0]}`] };
  }
  if (typeof data !== 'object' || Array.isArray(data)) return { ok: false, errors: ['Frontmatter phải là danh sách key: value'] };

  const result = frontmatterSchema.safeParse(data);
  if (!result.success) return { ok: false, errors: formatZodIssues(result.error) };

  const fm = result.data;
  const warnings = [];
  const extra = {};
  for (const key of Object.keys(fm)) {
    if (!KNOWN_FIELDS.includes(key)) {
      extra[key] = fm[key];
      warnings.push(`Trường lạ "${key}" (gõ nhầm?). Trường hợp lệ: ${KNOWN_FIELDS.join(', ')}`);
    }
  }
  if (slug !== slugify(slug)) warnings.push(`Tên file nên là slug không dấu, chữ thường, nối bằng "-" (gợi ý: ${slugify(slug)}.md)`);

  const folder = path.split('/').slice(1, -1).join('/');
  const expectedFolder = NOTE_TYPES[fm.type].folder;
  if (!inbox && folder && folder.split('/')[0] !== expectedFolder) {
    warnings.push(`Ghi chú type "${fm.type}" nên nằm trong knowledge/${expectedFolder}/`);
  }

  const note = {
    slug,
    path,
    inbox,
    title: fm.title,
    type: fm.type,
    summary: fm.summary.trim(),
    tags: fm.tags,
    status: fm.status,
    sources: fm.sources,
    credibility: credibilitySummary(fm.sources),
    aliases: fm.aliases,
    created: fm.created,
    updated: fm.updated ?? fm.created,
    reviewed: fm.reviewed ?? null,
    research: fm.research ?? null,
    extra,
    body: body.replace(/^\s*\n/, ''),
    wordCount: countWords(body),
    links: [],
    brokenLinks: [],
    backlinks: [],
  };
  return { ok: true, note, warnings };
}

/** Kiểm tra chất lượng nội dung — quan trọng để AI không trích dẫn thông tin yếu như sự thật. */
function qualityWarnings(note) {
  const out = [];
  const { sources, credibility, status, type } = note;
  if (type === 'fact' && sources.length === 0) out.push({ level: 'error', message: 'Ghi chú "fact" bắt buộc có ít nhất 1 nguồn (sources)' });
  if (status === 'verified' && type !== 'topic' && sources.length === 0) {
    out.push({ level: 'warning', message: 'Đã "verified" nhưng không có nguồn nào' });
  }
  if (status === 'verified' && credibility.rated < credibility.total) {
    out.push({ level: 'warning', message: `${credibility.total - credibility.rated}/${credibility.total} nguồn chưa chấm điểm uy tín (credibility 1–5)` });
  }
  if (status === 'verified' && type === 'fact' && credibility.max !== null && credibility.max <= 2) {
    out.push({ level: 'warning', message: 'Dữ kiện "verified" nhưng chỉ dựa trên nguồn uy tín thấp (≤ 2/5) — nên tìm nguồn tốt hơn' });
  }
  if (note.summary.length > 400) out.push({ level: 'warning', message: 'summary quá dài (> 400 ký tự) — nên gói trong 1–2 câu' });
  return out;
}

/**
 * Dựng toàn bộ kho tri thức từ danh sách file.
 * @param {{ path: string, raw: string }[]} files  path tính từ gốc repo, vd. "knowledge/concepts/x.md"
 * @param {{ includeInbox?: boolean }} [options]  includeInbox: đọc cả bản nháp chờ duyệt trong knowledge/_inbox
 */
export function buildKnowledge(files, { includeInbox = false } = {}) {
  const issues = []; // { path, level: 'error' | 'warning', message, inbox }
  const all = [];
  const bySlug = new Map();
  const addIssue = (path, level, message) => issues.push({ path, level, message, inbox: isInboxPath(path) });

  const candidates = files
    .filter((f) => isNotePath(f.path, { includeInbox }))
    // Ghi chú đã đăng được xử lý trước để bản nháp trùng tên (bản cập nhật) không chiếm chỗ.
    .sort((a, b) => Number(isInboxPath(a.path)) - Number(isInboxPath(b.path)) || a.path.localeCompare(b.path));

  for (const file of candidates) {
    const parsed = parseNote(file);
    if (!parsed.ok) {
      for (const message of parsed.errors) addIssue(file.path, 'error', message);
      continue;
    }
    const { note, warnings } = parsed;
    for (const message of warnings) addIssue(file.path, 'warning', message);
    for (const w of qualityWarnings(note)) addIssue(file.path, w.level, w.message);

    if (bySlug.has(note.slug)) {
      const existing = bySlug.get(note.slug);
      if (note.inbox && !existing.inbox) {
        // Bản nháp cập nhật cho ghi chú đã có — hợp lệ, xem trước dưới slug riêng.
        note.updates = existing.slug;
        note.slug = `${note.slug}--nhap`;
      } else {
        addIssue(file.path, 'error', `Trùng tên file "${note.slug}.md" với ${existing.path} — tên file phải duy nhất trong toàn kho`);
        continue;
      }
    }
    bySlug.set(note.slug, note);
    all.push(note);
  }

  // Bảng tra cứu cho wikilink: slug > tiêu đề > alias (không phân biệt dấu, hoa thường). Bản nháp không được ưu tiên.
  const byName = new Map();
  for (const note of [...all].sort((a, b) => Number(a.inbox) - Number(b.inbox))) {
    for (const name of [note.title, ...note.aliases]) {
      const key = slugify(name);
      if (key && !byName.has(key)) byName.set(key, note);
    }
  }
  const resolve = (target) => {
    const t = String(target).trim();
    return bySlug.get(t) ?? bySlug.get(slugify(t)) ?? byName.get(slugify(t)) ?? null;
  };

  // Ghi chú còn thiếu: đích của các [[liên kết]] chưa có ghi chú.
  const gaps = new Map();
  for (const note of all) {
    const links = new Set();
    for (const link of extractWikilinks(note.body)) {
      const target = resolve(link.target);
      if (!target) {
        note.brokenLinks.push(link.target);
        const key = slugify(link.target) || link.target;
        if (!gaps.has(key)) gaps.set(key, { slug: key, name: link.target, from: [] });
        if (!gaps.get(key).from.includes(note.slug)) gaps.get(key).from.push(note.slug);
      } else if (target !== note) {
        links.add(target.slug);
      }
    }
    note.links = [...links];
    note.brokenLinks = [...new Set(note.brokenLinks)];
  }
  for (const note of all) {
    for (const slug of note.links) bySlug.get(slug).backlinks.push(note.slug);
  }

  const sortNotes = (list) => [...list].sort((a, b) => b.updated.localeCompare(a.updated) || a.title.localeCompare(b.title, 'vi'));
  const published = sortNotes(all.filter((n) => !n.inbox));
  const inbox = sortNotes(all.filter((n) => n.inbox));

  // Tags: gom theo slug để "AI", "ai" là một. Chỉ tính ghi chú đã đăng.
  const tags = new Map();
  for (const note of published) {
    for (const tag of note.tags) {
      const key = slugify(tag) || 'tag';
      if (!tags.has(key)) tags.set(key, { slug: key, name: tag, notes: [] });
      tags.get(key).notes.push(note.slug);
    }
  }

  let mentionsCache;
  return {
    /** Ghi chú đã đăng (xuất hiện trên web và trong llms.txt). */
    notes: published,
    /** Bản nháp chờ duyệt (chỉ có khi includeInbox). */
    inbox,
    /** Tất cả ghi chú đọc được (đã đăng + chờ duyệt). */
    all,
    issues,
    tags: [...tags.values()].sort((a, b) => b.notes.length - a.notes.length || a.name.localeCompare(b.name, 'vi')),
    /** Liên kết chưa có ghi chú — hàng đợi nghiên cứu tiếp theo. */
    gaps: [...gaps.values()].sort((a, b) => b.from.length - a.from.length || a.name.localeCompare(b.name, 'vi')),
    /** Ghi chú đã đăng không liên kết tới/được liên kết từ ghi chú nào. */
    orphans: () => published.filter((n) => n.links.length === 0 && n.backlinks.length === 0),
    /** Chỗ nhắc tên ghi chú khác mà chưa đặt [[liên kết]]. */
    mentions: () => (mentionsCache ??= findUnlinkedMentions(all)),
    get: (slug) => bySlug.get(slug) ?? null,
    resolve,
    byType: (type) => published.filter((n) => n.type === type),
    byTag: (tagSlug) => published.filter((n) => n.tags.some((t) => (slugify(t) || 'tag') === tagSlug)),
    search: (query, { withInbox = false } = {}) => searchNotes(withInbox ? [...published, ...inbox] : published, query),
  };
}

/** Tìm kiếm đơn giản phía server (dùng cho scripts); site dùng bản client tương tự. */
export function searchNotes(notes, query) {
  const terms = normalizeForSearch(query).split(' ').filter(Boolean);
  if (!terms.length) return [];
  return notes
    .map((note) => {
      const title = normalizeForSearch([note.title, ...note.aliases].join(' '));
      const meta = normalizeForSearch([note.summary, ...note.tags].join(' '));
      const body = normalizeForSearch(note.body);
      let score = 0;
      for (const term of terms) {
        const s = (title.includes(term) ? 10 : 0) + (meta.includes(term) ? 4 : 0) + (body.includes(term) ? 1 : 0);
        if (!s) return null;
        score += s;
      }
      return { note, score };
    })
    .filter(Boolean)
    .sort((a, b) => b.score - a.score)
    .map((r) => r.note);
}
