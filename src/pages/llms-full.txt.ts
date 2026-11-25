import type { APIRoute } from 'astro';
import { AGENT_RULES, SITE } from '../config.mjs';
import { exporter } from '../lib/export';
import { NOTE_TYPES } from '../lib/schema.mjs';

// Toàn bộ kho trong một file — tiện để nạp thẳng vào context của AI agent.
export const GET: APIRoute = ({ site }) => {
  const ex = exporter(site);
  const order = Object.keys(NOTE_TYPES);
  const notes = [...ex.notes].sort((a, b) => order.indexOf(a.type) - order.indexOf(b.type) || a.title.localeCompare(b.title, 'vi'));

  const header = [
    `# ${SITE.name} — toàn bộ nội dung`,
    '',
    `> ${SITE.description}`,
    '',
    'Quy tắc khi dùng kho này để viết content:',
    '',
    ...AGENT_RULES.map((r) => `- ${r}`),
    '',
  ].join('\n');

  const body = notes
    .map((n) => {
      const sources = n.sources.length
        ? '\nNguồn:\n' + n.sources.map((s) => `- ${[s.title, s.author, s.publisher, s.date].filter(Boolean).join(' — ')}${s.url ? ` <${s.url}>` : ''}`).join('\n')
        : '\nNguồn: (chưa có)';
      return [
        '---',
        '',
        `## ${n.title}`,
        '',
        `Loại: ${NOTE_TYPES[n.type as keyof typeof NOTE_TYPES].label} | Trạng thái: ${ex.statusText(n)} | Cập nhật: ${n.updated}${n.tags.length ? ` | Tags: ${n.tags.join(', ')}` : ''}`,
        `URL: ${ex.mdUrl(n)}`,
        '',
        `Tóm tắt: ${n.summary}`,
        '',
        ex.portable(n),
        sources,
        '',
      ].join('\n');
    })
    .join('\n');

  return new Response(header + '\n' + body, { headers: { 'Content-Type': 'text/plain; charset=utf-8' } });
};
