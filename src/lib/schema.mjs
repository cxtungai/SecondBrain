// Định nghĩa cấu trúc (schema) cho mỗi ghi chú tri thức.
// Đây là "hợp đồng" giữa người viết, site và AI agent: sửa ở đây là sửa cho cả hệ thống.
import { z } from 'zod';

/** Các loại ghi chú. `folder` là thư mục gợi ý trong knowledge/. */
export const NOTE_TYPES = {
  topic: {
    label: 'Chủ đề',
    folder: 'topics',
    description: 'Ghi chú tổng hợp (bản đồ nội dung) dẫn tới các ghi chú liên quan trong một mảng kiến thức.',
  },
  concept: {
    label: 'Khái niệm',
    folder: 'concepts',
    description: 'Định nghĩa, giải thích một khái niệm, thuật ngữ hoặc nguyên lý.',
  },
  fact: {
    label: 'Số liệu & dữ kiện',
    folder: 'facts',
    description: 'Số liệu, thống kê, mốc thời gian, dữ kiện có thể trích dẫn. Bắt buộc có nguồn.',
  },
  framework: {
    label: 'Mô hình & phương pháp',
    folder: 'frameworks',
    description: 'Mô hình, quy trình, phương pháp, checklist có thể áp dụng.',
  },
  'case-study': {
    label: 'Case study',
    folder: 'case-studies',
    description: 'Ví dụ thực tế: bối cảnh, cách làm, kết quả và bài học.',
  },
  source: {
    label: 'Nguồn tham khảo',
    folder: 'sources',
    description: 'Tóm tắt một cuốn sách, bài báo, báo cáo, video… kèm các ý chính.',
  },
};

export const STATUSES = {
  draft: { label: 'Bản nháp', description: 'Chưa kiểm chứng — AI chỉ dùng làm gợi ý, không trích dẫn như sự thật.' },
  verified: { label: 'Đã kiểm chứng', description: 'Đã đối chiếu nguồn — AI có thể dùng và trích dẫn.' },
  outdated: { label: 'Lỗi thời', description: 'Thông tin cũ — AI không được dùng, trừ khi nói về bối cảnh lịch sử.' },
};

/**
 * Thang điểm uy tín nguồn (1–5). AI agent chấm khi nghiên cứu, người dùng xác nhận khi duyệt.
 * Tiêu chí: Thẩm quyền (ai nói), Bằng chứng (có dữ liệu/dẫn chứng), Độc lập (không bán hàng/xung đột lợi ích),
 * Thời điểm (còn mới so với chủ đề), Đối chiếu (các nguồn khác có xác nhận).
 */
export const CREDIBILITY = {
  5: {
    label: 'Rất cao',
    description: 'Nguồn gốc hoặc chính thức: nghiên cứu có bình duyệt, cơ quan quản lý, sở giao dịch, dữ liệu sơ cấp, tài liệu gốc của người đặt ra khái niệm.',
  },
  4: {
    label: 'Cao',
    description: 'Sách chuyên môn của tác giả có uy tín, báo chí lớn, tổ chức nghề nghiệp/đào tạo có tên tuổi — có biên tập và dẫn nguồn rõ ràng.',
  },
  3: {
    label: 'Trung bình',
    description: 'Chuyên gia/người thực hành có danh tiếng và thành tích kiểm chứng được; trang kiến thức chuyên ngành có biên tập.',
  },
  2: {
    label: 'Thấp',
    description: 'Blog cá nhân, video, khoá học chưa rõ kiểm chứng; ý kiến cá nhân ít dẫn chứng.',
  },
  1: {
    label: 'Rất thấp',
    description: 'Diễn đàn, mạng xã hội, nguồn ẩn danh, nội dung quảng cáo hoặc có xung đột lợi ích rõ ràng.',
  },
};

/** Loại nguồn (tuỳ chọn) — giúp chấm điểm nhất quán. */
export const SOURCE_KINDS = {
  paper: 'Nghiên cứu khoa học',
  official: 'Cơ quan / tổ chức chính thức',
  data: 'Dữ liệu sơ cấp',
  book: 'Sách',
  media: 'Báo chí',
  education: 'Trang kiến thức / đào tạo',
  expert: 'Chuyên gia / người thực hành',
  blog: 'Blog',
  video: 'Video / podcast',
  forum: 'Diễn đàn / mạng xã hội',
  other: 'Khác',
};

export const NOTE_TYPE_KEYS = /** @type {[string, ...string[]]} */ (Object.keys(NOTE_TYPES));
export const STATUS_KEYS = /** @type {[string, ...string[]]} */ (Object.keys(STATUSES));

/** Các trường frontmatter được hỗ trợ (dùng để cảnh báo khi gõ nhầm tên trường). */
export const KNOWN_FIELDS = ['title', 'type', 'summary', 'tags', 'status', 'sources', 'aliases', 'created', 'updated', 'reviewed', 'research'];
const SOURCE_KIND_KEYS = /** @type {[string, ...string[]]} */ (Object.keys(SOURCE_KINDS));

// YAML để trống (vd. `updated:`) cho ra null — coi như không khai báo.
const blankToUndefined = (v) => (v === null || v === '' ? undefined : v);
const optional = (schema) => z.preprocess(blankToUndefined, schema.optional());

const dateString = z.preprocess(
  (v) => (v instanceof Date ? v.toISOString().slice(0, 10) : v),
  z.string().regex(/^\d{4}-\d{2}-\d{2}$/, 'Ngày phải có dạng YYYY-MM-DD'),
);

const stringList = z.preprocess((v) => {
  v = blankToUndefined(v);
  if (v === undefined) return [];
  if (typeof v === 'string') return v.split(',');
  return v;
}, z.array(z.coerce.string()).transform((list) => [...new Set(list.map((s) => s.replace(/^#/, '').trim()).filter(Boolean))]));

const sourceSchema = z.preprocess(
  (v) => {
    // Cho phép viết gọn: `- https://...` hoặc `- Tên sách`
    if (typeof v === 'string') return /^https?:\/\//.test(v) ? { title: v, url: v } : { title: v };
    return v;
  },
  z.object({
    title: z.string().min(1, 'Nguồn cần có title'),
    url: optional(z.url({ message: 'url không hợp lệ' })),
    author: optional(z.string()),
    publisher: optional(z.string()),
    // Năm hoặc ngày phát hành, viết tự do: 2022, 2022-06, 2022-06-14
    date: optional(z.coerce.string()),
    // Ngày truy cập/kiểm tra nguồn
    accessed: optional(dateString),
    kind: optional(z.enum(SOURCE_KIND_KEYS, { message: `kind phải là một trong: ${SOURCE_KIND_KEYS.join(', ')}` })),
    // Điểm uy tín 1–5 theo thang CREDIBILITY, kèm lý do ngắn
    credibility: optional(z.coerce.number().int().min(1, 'credibility từ 1 đến 5').max(5, 'credibility từ 1 đến 5')),
    credibility_note: optional(z.string()),
    note: optional(z.string()),
  }),
);

export const frontmatterSchema = z.looseObject({
  title: z.string().min(1, 'Thiếu title'),
  type: z.enum(NOTE_TYPE_KEYS, { message: `type phải là một trong: ${NOTE_TYPE_KEYS.join(', ')}` }),
  summary: z.string().min(1, 'Thiếu summary (1–2 câu tóm tắt để AI chọn ghi chú)'),
  tags: stringList,
  status: z.preprocess(blankToUndefined, z.enum(STATUS_KEYS, { message: `status phải là một trong: ${STATUS_KEYS.join(', ')}` }).default('draft')),
  // Bỏ qua khối nguồn để trống từ template (không có title lẫn url).
  sources: z.preprocess(
    (v) => (Array.isArray(v) ? v.filter((x) => !(x && typeof x === 'object' && !x.title && !x.url)) : (blankToUndefined(v) ?? [])),
    z.array(sourceSchema),
  ),
  aliases: stringList,
  created: dateString,
  updated: optional(dateString),
  // Ngày người dùng đọc, sửa và duyệt đăng (scripts/publish.mjs tự điền)
  reviewed: optional(dateString),
  // Thông tin phiên nghiên cứu của AI agent (từ khoá gốc, ngày, agent) — tuỳ chọn
  research: optional(
    z.looseObject({
      keyword: optional(z.string()),
      date: optional(dateString),
      agent: optional(z.string()),
    }),
  ),
});
