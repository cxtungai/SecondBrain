import { slugify } from './text.mjs';

const BASE = import.meta.env.BASE_URL.replace(/\/+$/, '');

/** Đường dẫn nội bộ có tính base path (khi deploy lên GitHub Pages dạng /ten-repo/). */
export const url = (path = '') => `${BASE}/${path.replace(/^\/+/, '')}`;

export const noteUrl = (slug: string, heading?: string) => url(`notes/${slug}/`) + (heading ? `#${heading}` : '');
export const noteMdUrl = (slug: string) => url(`notes/${slug}.md`);
export const typeUrl = (type: string) => url(`types/${type}/`);
export const tagSlug = (tag: string) => slugify(tag) || 'tag';
export const tagUrl = (tag: string) => url(`tags/${tagSlug(tag)}/`);

/** URL tuyệt đối khi đã cấu hình `site`; nếu chưa thì trả về đường dẫn tương đối. */
export const absolute = (path: string, site?: URL) => (site ? new URL(path, site).href : path);

export const formatDate = (iso: string) => {
  const [y, m, d] = iso.split('-');
  return `${d}/${m}/${y}`;
};
