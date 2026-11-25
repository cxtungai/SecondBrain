// Công cụ tra cứu kho cho người và AI agent.
// Dùng: npm run kb -- <lệnh> [tham số] [--json]
//   search "<từ khoá>"   tìm ghi chú (gồm cả bản nháp chờ duyệt)
//   show <slug>          metadata, liên kết, backlinks của một ghi chú
//   list [--type t] [--tag t] [--status s]
//   gaps                 [[liên kết]] chưa có ghi chú — hàng đợi nghiên cứu
//   orphans              ghi chú không liên kết với ghi chú nào
//   mentions [slug]      chỗ nhắc tên ghi chú khác mà chưa đặt [[liên kết]]
//   stats                thống kê nhanh
import { NOTE_TYPES } from '../src/lib/schema.mjs';
import { slugify } from '../src/lib/text.mjs';
import { c, parseArgs } from './args.mjs';
import { loadKnowledge } from './load.mjs';

const { positional, flags } = parseArgs();
const [cmd, ...rest] = positional;
const kb = loadKnowledge({ includeInbox: true });

const brief = (n) => ({
  slug: n.slug,
  title: n.title,
  type: n.type,
  status: n.inbox ? `${n.status} (chờ duyệt)` : n.status,
  summary: n.summary,
  tags: n.tags,
  path: n.path,
  credibility: n.credibility.average,
});
const line = (n) =>
  `${c.bold(n.slug)} ${c.dim(`[${n.type} · ${n.inbox ? 'chờ duyệt' : n.status}]`)} ${n.title}\n    ${c.dim(n.summary)}`;
const out = (data, text) => console.log(flags.json ? JSON.stringify(data, null, 2) : text());

switch (cmd) {
  case 'search': {
    const results = kb.search(rest.join(' '), { withInbox: true });
    out(results.map(brief), () => (results.length ? results.map(line).join('\n') : 'Không tìm thấy ghi chú nào.'));
    break;
  }
  case 'show': {
    const n = kb.get(rest[0]) ?? kb.resolve(rest.join(' '));
    if (!n) {
      console.error(`Không có ghi chú "${rest.join(' ')}"`);
      process.exit(1);
    }
    const data = { ...brief(n), aliases: n.aliases, sources: n.sources, created: n.created, updated: n.updated, reviewed: n.reviewed, links: n.links, backlinks: n.backlinks, brokenLinks: n.brokenLinks };
    out(data, () =>
      [
        line(n),
        `    file: ${n.path}`,
        `    tags: ${n.tags.join(', ') || '—'} · aliases: ${n.aliases.join(', ') || '—'}`,
        `    nguồn: ${n.sources.length} (uy tín TB: ${n.credibility.average ?? 'chưa chấm'})`,
        `    liên kết tới: ${n.links.join(', ') || '—'}`,
        `    được liên kết từ: ${n.backlinks.join(', ') || '—'}`,
        `    còn thiếu: ${n.brokenLinks.join(', ') || '—'}`,
      ].join('\n'),
    );
    break;
  }
  case 'list': {
    let list = [...kb.notes, ...kb.inbox];
    if (flags.type) list = list.filter((n) => n.type === flags.type);
    if (flags.status) list = list.filter((n) => n.status === flags.status);
    if (flags.tag) list = list.filter((n) => n.tags.some((t) => slugify(t) === slugify(flags.tag)));
    out(list.map(brief), () => list.map(line).join('\n') || 'Trống.');
    break;
  }
  case 'gaps':
    out(kb.gaps, () => kb.gaps.map((g) => `[[${g.name}]] ${c.dim(`← ${g.from.join(', ')}`)}`).join('\n') || 'Không thiếu gì.');
    break;
  case 'orphans': {
    const o = kb.orphans();
    out(o.map(brief), () => o.map(line).join('\n') || 'Không có ghi chú cô lập.');
    break;
  }
  case 'mentions': {
    const m = kb.mentions().filter((x) => !rest[0] || x.from === rest[0] || x.to === rest[0]);
    out(m, () => m.map((x) => `${x.from} → [[${x.to}]] ${c.dim(`"…${x.context}…"`)}`).join('\n') || 'Không có gợi ý.');
    break;
  }
  case 'stats': {
    const data = {
      published: kb.notes.length,
      inbox: kb.inbox.length,
      byType: Object.fromEntries(Object.keys(NOTE_TYPES).map((t) => [t, kb.byType(t).length])),
      verified: kb.notes.filter((n) => n.status === 'verified').length,
      links: kb.notes.reduce((s, n) => s + n.links.length, 0),
      gaps: kb.gaps.length,
      orphans: kb.orphans().length,
      tags: kb.tags.length,
    };
    out(data, () => JSON.stringify(data, null, 2));
    break;
  }
  default:
    console.log(`Dùng: npm run kb -- <search|show|list|gaps|orphans|mentions|stats> [...] [--json]`);
    process.exit(cmd ? 1 : 0);
}
