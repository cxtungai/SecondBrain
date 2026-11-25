// Tiện ích xử lý văn bản dùng chung cho site và scripts (Node thuần, không phụ thuộc Astro).

/** Bỏ dấu tiếng Việt: "Đà Nẵng" -> "Da Nang". */
export function stripDiacritics(input) {
  return String(input)
    .normalize('NFD')
    .replace(/[̀-ͯ]/g, '')
    .replace(/đ/g, 'd')
    .replace(/Đ/g, 'D');
}

/** Tạo slug an toàn cho URL/tên file: "Khái niệm Second Brain" -> "khai-niem-second-brain". */
export function slugify(input) {
  return stripDiacritics(input)
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');
}

/** Chuẩn hoá để tìm kiếm không phân biệt hoa thường và dấu. */
export function normalizeForSearch(input) {
  return stripDiacritics(input).toLowerCase().replace(/\s+/g, ' ').trim();
}

/**
 * Cú pháp liên kết kiểu Obsidian:
 *   [[slug]]  [[slug|nhãn hiển thị]]  [[slug#tiêu-đề]]  [[Tiêu đề ghi chú]]
 * Nhóm: 1 = đích, 2 = heading (tuỳ chọn), 3 = nhãn (tuỳ chọn).
 */
export const WIKILINK_PATTERN = String.raw`\[\[([^\[\]|#\n]+)(?:#([^\[\]|\n]+))?(?:\|([^\[\]\n]+))?\]\]`;
export const wikilinkRegex = () => new RegExp(WIKILINK_PATTERN, 'g');

/**
 * Gọi `fn` cho từng đoạn văn bản nằm NGOÀI code block (```), inline code (`...`) và chú thích <!-- -->,
 * giữ nguyên phần code. Dùng để thay/thu thập wikilink mà không đụng vào ví dụ code.
 */
export function mapOutsideCode(markdown, fn) {
  const parts = String(markdown).split(/(^```[\s\S]*?^```[^\n]*$|^~~~[\s\S]*?^~~~[^\n]*$|`[^`\n]+`|<!--[\s\S]*?-->)/m);
  // split với nhóm bắt: phần chẵn là văn bản thường, phần lẻ là code.
  return parts.map((part, i) => (i % 2 === 0 ? fn(part) : part)).join('');
}

/** Liệt kê các wikilink trong markdown (bỏ qua code). */
export function extractWikilinks(markdown) {
  const found = [];
  mapOutsideCode(markdown, (text) => {
    for (const m of text.matchAll(wikilinkRegex())) {
      found.push({ raw: m[0], target: m[1].trim(), heading: m[2]?.trim(), label: m[3]?.trim() });
    }
    return text;
  });
  return found;
}

/** Chuyển markdown thành văn bản thô (phục vụ tìm kiếm, đếm từ). */
export function markdownToPlainText(markdown) {
  return String(markdown)
    .replace(/^```[\s\S]*?^```/gm, ' ')
    .replace(/<!--[\s\S]*?-->/g, ' ')
    .replace(new RegExp(WIKILINK_PATTERN, 'g'), (_m, target, _h, label) => label || target)
    .replace(/!\[([^\]]*)\]\([^)]*\)/g, '$1')
    .replace(/\[([^\]]*)\]\([^)]*\)/g, '$1')
    .replace(/<[^>]+>/g, ' ')
    .replace(/[`*_>#~|]+/g, ' ')
    .replace(/^\s*[-+]\s+/gm, ' ')
    .replace(/\s+/g, ' ')
    .trim();
}

export function countWords(markdown) {
  const text = markdownToPlainText(markdown);
  return text ? text.split(/\s+/).length : 0;
}
