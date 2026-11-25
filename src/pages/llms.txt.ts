import type { APIRoute } from 'astro';
import { AGENT_RULES, SITE } from '../config.mjs';
import { exporter } from '../lib/export';
import { posts } from '../lib/posts';
import { kb } from '../lib/knowledge';
import { NOTE_TYPES } from '../lib/schema.mjs';
import { absolute, articleUrl, url } from '../lib/urls';

// Chỉ mục theo chuẩn https://llmstxt.org — điểm vào đầu tiên cho mọi AI agent.
export const GET: APIRoute = ({ site }) => {
  const ex = exporter(site);
  const lines = [
    `# ${SITE.name}`,
    '',
    `> ${SITE.description}`,
    '',
    `Kho gồm ${kb.notes.length} ghi chú. Mỗi liên kết dưới đây trỏ tới bản Markdown của ghi chú (có frontmatter: type, status, tags, sources, updated).`,
    '',
    'Quy tắc khi dùng kho này để viết content:',
    '',
    ...AGENT_RULES.map((r) => `- ${r}`),
  ];

  for (const [type, info] of Object.entries(NOTE_TYPES)) {
    const notes = kb.byType(type);
    if (!notes.length) continue;
    lines.push('', `## ${info.label}`, '', `${info.description}`, '');
    for (const n of notes) {
      const meta = [ex.statusText(n), `cập nhật ${n.updated}`, n.tags.length ? `tags: ${n.tags.join(', ')}` : ''].filter(Boolean).join('; ');
      lines.push(`- [${n.title}](${ex.mdUrl(n)}): ${n.summary} (${meta})`);
    }
  }

  if (posts.list.length) {
    lines.push(
      '',
      '## Bài viết',
      '',
      'Bài viết dễ đọc, dẫn xuất từ các ghi chú trên. Khi cần sự thật và nguồn, trích dẫn ghi chú gốc (based on) thay vì bài viết.',
      '',
    );
    for (const a of posts.list) {
      lines.push(`- [${a.title}](${absolute(articleUrl(a.slug), site)}): ${a.description} (based on: ${a.notes.map((n) => n.slug).join(', ')})`);
    }
  }

  lines.push(
    '',
    '## Optional',
    '',
    `- [Toàn bộ nội dung kho](${absolute(url('llms-full.txt'), site)}): tất cả ghi chú trong một file văn bản`,
    `- [Chỉ mục JSON](${absolute(url('knowledge.json'), site)}): dữ liệu có cấu trúc gồm metadata, liên kết và nội dung từng ghi chú`,
    '',
  );

  return new Response(lines.join('\n'), { headers: { 'Content-Type': 'text/plain; charset=utf-8' } });
};
