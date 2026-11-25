// Đọc kho tri thức từ đĩa cho các script Node (site dùng import.meta.glob trong src/lib/knowledge.ts).
import fs from 'node:fs';
import path from 'node:path';
import { buildArticles } from '../src/lib/articles.mjs';
import { buildKnowledge } from '../src/lib/kb.mjs';

export const ROOT = process.cwd();
export const KNOWLEDGE_DIR = path.join(ROOT, 'knowledge');
export const INBOX_ABS = path.join(KNOWLEDGE_DIR, '_inbox');
export const ARTICLES_ABS = path.join(ROOT, 'articles');

function walk(dir) {
  if (!fs.existsSync(dir)) return [];
  return fs.readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
    const full = path.join(dir, entry.name);
    return entry.isDirectory() ? walk(full) : [full];
  });
}

export const toRepoPath = (abs) => path.relative(ROOT, abs).split(path.sep).join('/');

export function readKnowledgeFiles(dir = KNOWLEDGE_DIR) {
  return walk(dir)
    .filter((f) => f.endsWith('.md'))
    .map((f) => ({ path: toRepoPath(f), raw: fs.readFileSync(f, 'utf8') }));
}

/** @param {{ includeInbox?: boolean }} [options] */
export function loadKnowledge(options = {}) {
  return buildKnowledge(readKnowledgeFiles(), options);
}

export const readArticleFiles = () => readKnowledgeFiles(ARTICLES_ABS);

/**
 * Bài viết SEO/GEO, kiểm tra dựa trên kho tri thức.
 * @param {ReturnType<typeof buildKnowledge>} kb
 * @param {{ includeDrafts?: boolean }} [options]
 */
export function loadArticles(kb, options = {}) {
  return buildArticles(readArticleFiles(), kb, options);
}
