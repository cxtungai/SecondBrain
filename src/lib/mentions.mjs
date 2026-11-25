// Phát hiện "nhắc tên chưa liên kết": ghi chú A nhắc tới tên/alias của ghi chú B nhưng chưa có [[B]].
// Đây là gợi ý để kết nối tri thức chặt hơn (giống "Unlinked mentions" của Obsidian).
import { stripDiacritics, wikilinkRegex } from './text.mjs';

/** Các tên dùng để dò: tiêu đề, alias, và tiêu đề bỏ phần trong ngoặc. */
function namesOf(note) {
  const names = new Set([note.title, ...note.aliases]);
  const m = /^(.*?)\s*\(([^)]+)\)\s*$/.exec(note.title);
  if (m) {
    names.add(m[1]);
    names.add(m[2]);
  }
  // Bỏ tiền tố loại phổ biến: "Sách: ...", "Phương pháp ..."
  for (const n of [...names]) {
    const stripped = n.replace(/^(sách|bài báo|báo cáo|video|khoá học|khóa học)\s*:\s*/i, '');
    if (stripped !== n) names.add(stripped);
  }
  return [...names].map((n) => n.trim()).filter((n) => n.length >= 3);
}

/** Văn bản để dò: bỏ code, link sẵn có, frontmatter-like comment. Giữ NFC để vị trí khớp 1–1 sau khi bỏ dấu. */
function searchableText(body) {
  return body
    .normalize('NFC')
    .replace(/^```[\s\S]*?^```/gm, (m) => ' '.repeat(m.length))
    .replace(/`[^`\n]*`/g, (m) => ' '.repeat(m.length))
    .replace(/<!--[\s\S]*?-->/g, (m) => ' '.repeat(m.length))
    .replace(wikilinkRegex(), (m) => ' '.repeat(m.length))
    .replace(/\[[^\]]*\]\([^)]*\)/g, (m) => ' '.repeat(m.length));
}

const isWordChar = (c) => !!c && /[\p{L}\p{N}]/u.test(c);

function findName(text, folded, name) {
  const acronym = name.length <= 5 || name === name.toUpperCase();
  const hay = acronym ? stripDiacritics(text) : folded;
  const needle = acronym ? stripDiacritics(name) : stripDiacritics(name).toLowerCase();
  let from = 0;
  while ((from = hay.indexOf(needle, from)) !== -1) {
    const before = hay[from - 1];
    const after = hay[from + needle.length];
    if (!isWordChar(before) && !isWordChar(after)) return from;
    from += needle.length;
  }
  return -1;
}

/**
 * @param {any[]} notes
 * @returns {{ from: string, to: string, match: string, context: string }[]}
 */
export function findUnlinkedMentions(notes) {
  const targets = notes.map((n) => ({ note: n, names: namesOf(n) }));
  const results = [];
  for (const note of notes) {
    const text = searchableText(note.body);
    const folded = stripDiacritics(text).toLowerCase();
    const linked = new Set(note.links);
    for (const { note: target, names } of targets) {
      if (target === note || linked.has(target.slug)) continue;
      if (target.updates === note.slug || note.updates === target.slug) continue;
      for (const name of names) {
        const at = findName(text, folded, name);
        if (at === -1) continue;
        const start = Math.max(0, at - 50);
        const context = text.slice(start, at + name.length + 50).replace(/\s+/g, ' ').trim();
        results.push({ from: note.slug, to: target.slug, match: text.slice(at, at + name.length), context });
        break;
      }
    }
  }
  return results;
}
