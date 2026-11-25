// Chuyển nội dung ghi chú sang HTML (cho người đọc) và Markdown chuẩn (cho AI agent).
import { Marked } from 'marked';
import { mapOutsideCode, markdownToPlainText, slugify, WIKILINK_PATTERN, wikilinkRegex } from './text.mjs';

const escapeHtml = (s) =>
  String(s).replace(/[&<>"']/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' })[c]);

/**
 * Render markdown của ghi chú sang HTML.
 * @param {string} markdown
 * @param {{ resolve: (target: string) => any, hrefFor: (note: any, heading?: string) => string, gapHref?: (target: string) => string }} opts
 */
export function renderNoteHtml(markdown, { resolve, hrefFor, gapHref }) {
  const marked = new Marked({ gfm: true });
  marked.use({
    extensions: [
      {
        name: 'wikilink',
        level: 'inline',
        start: (src) => src.indexOf('[['),
        tokenizer(src) {
          const match = new RegExp('^' + WIKILINK_PATTERN).exec(src);
          if (!match) return undefined;
          return { type: 'wikilink', raw: match[0], target: match[1].trim(), heading: match[2]?.trim(), label: match[3]?.trim() };
        },
        renderer(token) {
          const note = resolve(token.target);
          const text = escapeHtml(token.label || (note ? note.title : token.target));
          if (!note) {
            const tip = 'Chưa có ghi chú này — cần nghiên cứu thêm';
            return gapHref
              ? `<a class="wikilink wikilink--broken" href="${escapeHtml(gapHref(token.target))}" title="${tip}">${text}</a>`
              : `<span class="wikilink wikilink--broken" title="${tip}">${text}</span>`;
          }
          return `<a class="wikilink" href="${escapeHtml(hrefFor(note, token.heading ? slugify(token.heading) : undefined))}" title="${escapeHtml(note.summary)}">${text}</a>`;
        },
      },
    ],
    renderer: {
      // Gắn id cho heading để liên kết được tới [[ghi-chu#tieu-de]].
      heading({ tokens, depth, text }) {
        const id = slugify(text);
        return `<h${depth} id="${id}">${this.parser.parseInline(tokens)}</h${depth}>\n`;
      },
      // Link ngoài mở tab mới.
      link({ href, title, tokens }) {
        const inner = this.parser.parseInline(tokens);
        const external = /^https?:\/\//.test(href);
        const attrs = [`href="${escapeHtml(href)}"`];
        if (title) attrs.push(`title="${escapeHtml(title)}"`);
        if (external) attrs.push('target="_blank" rel="noopener noreferrer"');
        return `<a ${attrs.join(' ')}>${inner}</a>`;
      },
    },
  });
  return marked.parse(markdown);
}

/**
 * Đổi wikilink thành link Markdown chuẩn để AI agent (không hiểu cú pháp Obsidian) đi theo được.
 * @param {string} markdown
 * @param {{ resolve: (target: string) => any, urlFor: (note: any) => string }} opts
 */
export function toPortableMarkdown(markdown, { resolve, urlFor }) {
  const withoutComments = markdown.replace(/<!--[\s\S]*?-->\n?/g, '');
  return mapOutsideCode(withoutComments, (text) =>
    text.replace(wikilinkRegex(), (_m, target, heading, label) => {
      const note = resolve(target.trim());
      const display = label?.trim() || note?.title || target.trim();
      if (!note) return display;
      return `[${display}](${urlFor(note)}${heading ? '#' + slugify(heading) : ''})`;
    }),
  );
}

/** Danh sách heading cấp 2–3 để làm mục lục. */
export function extractHeadings(markdown) {
  return new Marked({ gfm: true })
    .lexer(markdown)
    .filter((t) => t.type === 'heading' && (t.depth === 2 || t.depth === 3))
    .map((t) => ({ depth: t.depth, text: markdownToPlainText(t.text), id: slugify(t.text) }));
}

const FAQ_HEADING = /^(cau-hoi-thuong-gap|hoi-dap|faq)/;

/**
 * Lấy mục "Câu hỏi thường gặp": heading cấp 2 tên "Câu hỏi thường gặp"/"FAQ", mỗi câu hỏi là heading cấp 3.
 * Dùng để sinh dữ liệu có cấu trúc FAQPage (giúp Google / AI trích dẫn).
 */
export function extractFaq(markdown) {
  const tokens = new Marked({ gfm: true }).lexer(markdown);
  const faq = [];
  let inFaq = false;
  let current = null;
  for (const t of tokens) {
    if (t.type === 'heading' && t.depth <= 2) {
      inFaq = t.depth === 2 && FAQ_HEADING.test(slugify(t.text));
      current = null;
      continue;
    }
    if (!inFaq) continue;
    if (t.type === 'heading' && t.depth === 3) {
      current = { question: markdownToPlainText(t.text), answer: '' };
      faq.push(current);
    } else if (current && t.raw.trim()) {
      current.answer = (current.answer + ' ' + markdownToPlainText(t.raw)).trim();
    }
  }
  return faq.filter((f) => f.question && f.answer);
}
