import type { APIRoute } from 'astro';
import { AGENT_RULES, SITE } from '../config.mjs';
import { exporter } from '../lib/export';
import { kb } from '../lib/knowledge';
import { NOTE_TYPES, STATUSES } from '../lib/schema.mjs';

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
  };
  return new Response(JSON.stringify(data, null, 2), { headers: { 'Content-Type': 'application/json; charset=utf-8' } });
};
