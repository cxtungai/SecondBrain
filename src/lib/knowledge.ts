// Kho tri thức phía site. Dùng import.meta.glob để Vite theo dõi file trong knowledge/:
// sửa/thêm ghi chú khi `npm run dev` là trang tự cập nhật.
import { buildKnowledge } from './kb.mjs';

const files = import.meta.glob<string>(['/knowledge/**/*.md', '!/knowledge/_templates/**'], {
  query: '?raw',
  import: 'default',
  eager: true,
});

// Bản nháp chờ duyệt (knowledge/_inbox) chỉ hiện khi chạy `npm run dev` để xem trước — không bao giờ lên web.
export const kb = buildKnowledge(
  Object.entries(files).map(([path, raw]) => ({ path: path.replace(/^\//, ''), raw })),
  { includeInbox: import.meta.env.DEV },
);

if (kb.issues.length && import.meta.env.DEV) {
  for (const issue of kb.issues) console.warn(`[knowledge] ${issue.level}: ${issue.path} — ${issue.message}`);
}

export type Note = (typeof kb.all)[number];
