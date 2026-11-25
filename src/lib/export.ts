// Dữ liệu xuất cho AI agent: dùng chung giữa llms.txt, llms-full.txt, knowledge.json và .md từng ghi chú.
import { stringify } from 'yaml';
import { kb, type Note } from './knowledge';
import { toPortableMarkdown } from './render.mjs';
import { NOTE_TYPES, STATUSES } from './schema.mjs';
import { absolute, noteMdUrl, noteUrl } from './urls';

export function exporter(site?: URL) {
  const mdUrl = (note: Note) => absolute(noteMdUrl(note.slug), site);
  const htmlUrl = (note: Note) => absolute(noteUrl(note.slug), site);
  const portable = (note: Note) => toPortableMarkdown(note.body, { resolve: kb.resolve, urlFor: mdUrl }).trim();

  const metadata = (note: Note) => ({
    slug: note.slug,
    title: note.title,
    type: note.type,
    type_label: NOTE_TYPES[note.type as keyof typeof NOTE_TYPES].label,
    status: note.status,
    summary: note.summary,
    tags: note.tags,
    aliases: note.aliases,
    sources: note.sources,
    created: note.created,
    updated: note.updated,
    url: htmlUrl(note),
    markdown_url: mdUrl(note),
    links: note.links,
    backlinks: note.backlinks,
    ...(Object.keys(note.extra).length ? { extra: note.extra } : {}),
  });

  /** Một ghi chú dạng Markdown thuần (frontmatter + nội dung), wikilink đã đổi thành link chuẩn. */
  const noteMarkdown = (note: Note) => {
    const { links, backlinks, ...fm } = metadata(note);
    const related = [...new Set([...links, ...backlinks])].map((s) => kb.get(s)!).filter(Boolean);
    const relatedBlock = related.length
      ? `\n\n## Ghi chú liên quan\n\n${related.map((n) => `- [${n.title}](${mdUrl(n)}): ${n.summary}`).join('\n')}`
      : '';
    return `---\n${stringify(fm, { lineWidth: 0 }).trim()}\n---\n\n# ${note.title}\n\n> ${note.summary}\n\n${portable(note)}${relatedBlock}\n`;
  };

  const statusText = (note: Note) => STATUSES[note.status as keyof typeof STATUSES].label.toLowerCase();

  return { mdUrl, htmlUrl, portable, metadata, noteMarkdown, statusText, notes: kb.notes };
}
