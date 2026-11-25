// Bài viết SEO/GEO phía site. Bài nháp (status: draft) chỉ hiện khi chạy `npm run dev`.
import { buildArticles } from './articles.mjs';
import { kb } from './knowledge';

const files = import.meta.glob<string>('/articles/**/*.md', { query: '?raw', import: 'default', eager: true });

export const posts = buildArticles(
  Object.entries(files).map(([path, raw]) => ({ path: path.replace(/^\//, ''), raw })),
  kb,
  { includeDrafts: import.meta.env.DEV },
);

if (posts.issues.length && import.meta.env.DEV) {
  for (const issue of posts.issues) console.warn(`[articles] ${issue.level}: ${issue.path} — ${issue.message}`);
}

export type Article = (typeof posts.all)[number];
