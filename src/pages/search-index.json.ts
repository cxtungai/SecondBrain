import type { APIRoute } from 'astro';
import { posts } from '../lib/posts';
import { kb } from '../lib/knowledge';
import { markdownToPlainText } from '../lib/text.mjs';
import { articleUrl, noteUrl } from '../lib/urls';

// Chỉ mục gọn nhẹ cho ô tìm kiếm phía trình duyệt.
export const GET: APIRoute = () => {
  const notes = kb.notes.map((n) => ({
    url: noteUrl(n.slug),
    slug: n.slug,
    title: n.title,
    aliases: n.aliases,
    type: n.type,
    status: n.status,
    updated: n.updated,
    summary: n.summary,
    tags: n.tags,
    text: markdownToPlainText(n.body).slice(0, 6000),
  }));
  const articles = posts.list.map((a) => ({
    url: articleUrl(a.slug),
    slug: a.slug,
    title: a.title,
    aliases: [a.keyword, ...a.keywords],
    type: 'article',
    status: '',
    updated: a.updated,
    summary: a.description,
    tags: [],
    text: markdownToPlainText(a.body).slice(0, 6000),
  }));
  const index = [...notes, ...articles];
  return new Response(JSON.stringify(index), { headers: { 'Content-Type': 'application/json; charset=utf-8' } });
};
