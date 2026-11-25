import type { APIRoute } from 'astro';
import { kb } from '../lib/knowledge';
import { markdownToPlainText } from '../lib/text.mjs';

// Chỉ mục gọn nhẹ cho ô tìm kiếm phía trình duyệt.
export const GET: APIRoute = () => {
  const index = kb.notes.map((n) => ({
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
  return new Response(JSON.stringify(index), { headers: { 'Content-Type': 'application/json; charset=utf-8' } });
};
