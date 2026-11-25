import type { APIRoute } from 'astro';
import { AGENT_RULES, SITE } from '../config.mjs';
import { exporter } from '../lib/export';
import { posts } from '../lib/posts';
import { kb } from '../lib/knowledge';
import { NOTE_TYPES, STATUSES } from '../lib/schema.mjs';
import { absolute, articleUrl } from '../lib/urls';

// Dữ liệu có cấu trúc cho agent/tool tự động (lọc theo type, status, tags…).
export const GET: APIRoute = ({ site }) => {
  const ex = exporter(site);
  const data = {
    name: SITE.name,
    description: SITE.description,
    language: SITE.language,
    generated_at: new Date().toISOString(),
    note_count: kb.notes.length,
    agent_rules: AGENT_RULES,
    types: NOTE_TYPES,
    statuses: STATUSES,
    tags: kb.tags.map((t) => ({ name: t.name, slug: t.slug, notes: t.notes })),
    notes: kb.notes.map((n) => ({ ...ex.metadata(n), content: ex.portable(n) })),
    // Bài viết dẫn xuất từ ghi chú — nguồn sự thật vẫn là ghi chú trong based_on.
    articles: posts.list.map((a) => ({
      slug: a.slug,
      title: a.title,
      description: a.description,
      keyword: a.keyword,
      intent: a.intent,
      based_on: a.notes.map((n) => n.slug),
      created: a.publishedOn ?? a.created,
      updated: a.updated,
      url: absolute(articleUrl(a.slug), site),
    })),
  };
  return new Response(JSON.stringify(data, null, 2), { headers: { 'Content-Type': 'application/json; charset=utf-8' } });
};
