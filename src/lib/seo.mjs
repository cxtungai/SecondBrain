// Gợi ý SEO / GEO cho ghi chú và bài viết — chỉ là gợi ý, không chặn build.
//
// SEO: giúp Google hiểu trang và hiển thị đúng (tiêu đề, mô tả, cấu trúc, liên kết nội bộ).
// GEO (Generative Engine Optimization): giúp AI search (Google AI Overview, ChatGPT, Perplexity…) trích dẫn
// đúng — câu trả lời thẳng ở đầu, định nghĩa rõ, có nguồn, có ngày cập nhật, có FAQ.
// Các gợi ý cho GHI CHÚ chỉ xoay quanh độ rõ ràng và độ tin cậy — đúng với bản chất tri thức, không nhồi từ khoá.
import { Marked } from 'marked';
import { extractFaq, extractHeadings } from './render.mjs';
import { keywordCore, markdownToPlainText, normalizeForSearch } from './text.mjs';

const length = (s) => [...String(s ?? '')].length;
const daysBetween = (from, to) => Math.round((Date.parse(to) - Date.parse(from)) / 86_400_000);

/** Đoạn văn đầu tiên của nội dung (bỏ heading, chú thích). */
function firstParagraph(markdown) {
  const tokens = new Marked({ gfm: true }).lexer(String(markdown).replace(/<!--[\s\S]*?-->/g, ''));
  const p = tokens.find((t) => t.type === 'paragraph');
  return p ? markdownToPlainText(p.raw) : '';
}

function countOccurrences(haystack, needle) {
  if (!needle) return 0;
  let n = 0;
  let i = 0;
  while ((i = haystack.indexOf(needle, i)) !== -1) {
    n++;
    i += needle.length;
  }
  return n;
}

/**
 * @typedef {{ ok: boolean, label: string, tip: string }} Hint
 * ok = đạt; tip = cách cải thiện khi chưa đạt.
 */

/** Gợi ý cho một ghi chú tri thức. @returns {Hint[]} */
export function noteSeoHints(note, { today = new Date().toISOString().slice(0, 10) } = {}) {
  const hints = [];
  const h = (ok, label, tip) => hints.push({ ok: Boolean(ok), label, tip });

  const t = length(note.title);
  h(t <= 60, `Tiêu đề ${t} ký tự`, 'Nên ≤ 60 ký tự để không bị cắt trên Google.');
  const d = length(note.summary);
  h(d >= 70 && d <= 160, `summary ${d} ký tự`, 'summary là mô tả trên Google và là câu AI đọc đầu tiên — nên 70–160 ký tự, nói rõ ghi chú trả lời điều gì.');

  if (note.type === 'concept') {
    const intro = normalizeForSearch(firstParagraph(note.body));
    const names = [note.title, ...note.aliases].map((n) => keywordCore(n)).filter(Boolean);
    h(
      names.some((n) => intro.includes(n)),
      'Đoạn mở đầu trả lời thẳng "… là gì"',
      'Câu đầu nên nhắc tên thuật ngữ và định nghĩa ngay ("X là …") — AI search hay trích nguyên câu này.',
    );
  }
  if (['concept', 'framework', 'topic'].includes(note.type)) {
    const faq = extractFaq(note.body).length;
    h(faq >= 2, `${faq} câu hỏi thường gặp`, 'Thêm "## Câu hỏi thường gặp" với 3–5 câu "### …?" — site tự sinh dữ liệu FAQ cho Google/AI.');
  }
  if (note.type !== 'topic') {
    h(note.sources.length > 0, `${note.sources.length} nguồn`, 'Nguồn có tên, ngày và điểm uy tín giúp người đọc lẫn AI tin và trích dẫn.');
  }
  h(note.backlinks.length > 0, `${note.backlinks.length} ghi chú liên kết tới`, 'Trang không có liên kết nội bộ trỏ tới khó được Google tìm thấy — thêm vào ghi chú chủ đề hoặc mục Liên quan.');
  h(note.wordCount >= 150, `${note.wordCount} từ`, 'Nội dung mỏng (< 150 từ) — bổ sung giải thích, ví dụ, giới hạn.');
  const age = daysBetween(note.updated, today);
  h(age <= 365, `Cập nhật ${age} ngày trước`, 'Đã hơn 1 năm — rà soát số liệu và nguồn (độ mới là tín hiệu tin cậy).');
  return hints;
}

/** Gợi ý cho một bài viết SEO/GEO. @returns {Hint[]} */
export function articleSeoHints(article) {
  const hints = [];
  const h = (ok, label, tip) => hints.push({ ok: Boolean(ok), label, tip });
  const kw = keywordCore(article.keyword);
  const has = (text) => normalizeForSearch(text).includes(kw);

  const t = length(article.title);
  h(t <= 60, `Tiêu đề ${t} ký tự`, 'Nên ≤ 60 ký tự để không bị cắt trên Google.');
  h(has(article.title), 'Tiêu đề chứa từ khoá chính', `Đưa "${article.keyword}" vào tiêu đề, càng gần đầu càng tốt — viết tự nhiên, không giật tít.`);
  const d = length(article.description);
  h(d >= 70 && d <= 160, `Mô tả ${d} ký tự`, 'description nên 70–160 ký tự: nói người đọc nhận được gì từ bài.');
  h(has(article.description), 'Mô tả chứa từ khoá chính', `Nhắc "${article.keyword}" trong description.`);

  const plain = markdownToPlainText(article.body);
  const intro = plain.split(/\s+/).slice(0, 120).join(' ');
  h(has(intro), 'Đoạn mở đầu trả lời ngay câu hỏi', 'Trả lời câu hỏi của người tìm trong 2–3 câu đầu (có từ khoá) — AI search và đoạn trích nổi bật lấy từ đây.');

  const h2 = extractHeadings(article.body).filter((x) => x.depth === 2).length;
  h(h2 >= 3, `${h2} mục H2`, 'Chia bài theo các câu hỏi phụ người đọc hay hỏi, mỗi câu một mục "## …".');
  const faq = extractFaq(article.body).length;
  h(faq >= 3, `${faq} câu hỏi thường gặp`, 'Thêm "## Câu hỏi thường gặp" với ≥ 3 câu "### …?" (lấy từ "Mọi người cũng hỏi" trên Google).');

  const linked = article.notes.filter((n) => article.links.includes(n.slug)).length;
  h(
    article.notes.length > 0 && linked === article.notes.length,
    `Liên kết về ${linked}/${article.notes.length} ghi chú gốc`,
    'Đặt [[liên kết]] tới từng ghi chú gốc ngay chỗ dùng dữ kiện của nó.',
  );
  const unverified = article.notes.filter((n) => n.inbox || n.status !== 'verified');
  h(
    unverified.length === 0,
    unverified.length ? `${unverified.length} ghi chú gốc chưa kiểm chứng` : 'Ghi chú gốc đều đã kiểm chứng',
    'Duyệt tri thức gốc (verified) trước khi đăng bài — bài viết không được "đi trước" tri thức.',
  );

  const per100 = article.wordCount ? (countOccurrences(normalizeForSearch(plain), kw) / article.wordCount) * 100 : 0;
  h(per100 <= 3, `Từ khoá ~${per100.toFixed(1)} lần/100 từ`, 'Quá dày — viết tự nhiên, dùng từ đồng nghĩa; nhồi từ khoá làm giảm chất lượng.');
  h(article.wordCount >= 600, `${article.wordCount} từ`, 'Bài kéo traffic thường cần ≥ 600 từ để trả lời đủ ý — nhưng đừng độn chữ.');
  return hints;
}

/**
 * Trang nào trong kho đã nhắm (hoặc gần) từ khoá này — kiểm tra trước khi viết bài mới để không tự cạnh tranh.
 * @returns {{ kind: 'note' | 'article', match: 'exact' | 'near', item: any }[]}
 */
export function keywordTargets(keyword, { notes, articles }) {
  const core = keywordCore(keyword);
  if (!core) return [];
  const out = [];
  const check = (kind, item, names) => {
    const cores = names.map(keywordCore).filter(Boolean);
    if (cores.includes(core)) out.push({ kind, match: 'exact', item });
    else if (cores.some((c) => (c.length > 3 && core.includes(c)) || (core.length > 3 && c.includes(core)))) out.push({ kind, match: 'near', item });
  };
  for (const n of notes) check('note', n, [n.title, ...n.aliases]);
  for (const a of articles) check('article', a, [a.keyword, ...a.keywords, a.title]);
  return out.sort((x, y) => (x.match === y.match ? 0 : x.match === 'exact' ? -1 : 1));
}
