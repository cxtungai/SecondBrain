import type { APIRoute } from 'astro';
import { exporter } from '../../lib/export';
import { kb } from '../../lib/knowledge';

// Bản Markdown của từng ghi chú: /notes/<slug>.md
export function getStaticPaths() {
  return kb.notes.map((note) => ({ params: { slug: note.slug } }));
}

export const GET: APIRoute = ({ params, site }) => {
  const note = kb.get(params.slug!);
  if (!note) return new Response('Not found', { status: 404 });
  return new Response(exporter(site).noteMarkdown(note), { headers: { 'Content-Type': 'text/markdown; charset=utf-8' } });
};
