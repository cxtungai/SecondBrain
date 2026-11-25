// Cấu hình chung của Second Brain — sửa tên, mô tả và quy tắc cho AI agent tại đây.

export const SITE = {
  name: 'Second Brain',
  description:
    'Kho tri thức chuyên môn được chắt lọc và kiểm chứng — để người đọc tra cứu nhanh và để AI agent viết content chính xác, có nguồn.',
  language: 'vi',
  locale: 'vi_VN',
  /** Tên tác giả/thương hiệu hiển thị trong dữ liệu có cấu trúc (JSON-LD). Để trống nếu chưa muốn công khai. */
  author: '',
};

/** Quy tắc bắt buộc với AI agent khi dùng kho này để viết content. Xuất hiện trong llms.txt, llms-full.txt, knowledge.json. */
export const AGENT_RULES = [
  'Chỉ dùng số liệu, dữ kiện, trích dẫn từ ghi chú có status "verified" như sự thật; luôn ghi nguồn theo trường sources của ghi chú đó.',
  'Ghi chú "draft" chỉ dùng làm ý tưởng hoặc định hướng — không trích dẫn con số hay khẳng định từ đó như sự thật.',
  'Không dùng ghi chú "outdated" cho content mới, trừ khi nói về bối cảnh lịch sử.',
  'Không bịa số liệu, nguồn hay trích dẫn. Nếu kho không có thông tin cần thiết, nói rõ là thiếu và đề xuất ghi chú cần bổ sung.',
  'Ưu tiên ghi chú cập nhật gần hơn khi có thông tin mâu thuẫn, và nêu rõ mốc thời gian của số liệu.',
  'Bắt đầu từ ghi chú loại "topic" (bản đồ chủ đề) để nắm tổng quan, rồi đi theo liên kết tới ghi chú chi tiết.',
];
