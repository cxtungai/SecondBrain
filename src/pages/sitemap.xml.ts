import type { APIRoute } from 'astro';
import { posts } from '../lib/posts';
import { kb } from '../lib/knowledge';
import { NOTE_TYPES } from '../lib/schema.mjs';
import { absolute, articleUrl, noteUrl, tagUrl, typeUrl, url } from '../lib/urls';

export const GET: APIRoute = ({ site }) => {
  const today = new Date().toISOString().slice(0, 10);
  const entries: { loc: string; lastmod?: string }[] = [
    { loc: url(), lastmod: kb.notes[0]?.updated ?? today },
    { loc: url('kien-thuc/') },
    { loc: url('bai-viet/'), lastmod: posts.list[0]?.updated },
    { loc: url('gioi-thieu/') },
    { loc: url('tu-dien/') },
    { loc: url('graph/') },
    { loc: url('ai/') },
    ...Object.keys(NOTE_TYPES).map((t) => ({ loc: typeUrl(t) })),
    ...kb.tags.map((t) => ({ loc: tagUrl(t.name) })),
    ...kb.notes.map((n) => ({ loc: noteUrl(n.slug), lastmod: n.updated })),
    ...posts.list.map((a) => ({ loc: articleUrl(a.slug), lastmod: a.updated })),
  ];
  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${entries.map((e) => `  <url><loc>${absolute(e.loc, site)}</loc>${e.lastmod ? `<lastmod>${e.lastmod}</lastmod>` : ''}</url>`).join('\n')}
</urlset>
`;
  return new Response(xml, { headers: { 'Content-Type': 'application/xml; charset=utf-8' } });
};
