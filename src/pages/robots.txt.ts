import type { APIRoute } from 'astro';
import { absolute, url } from '../lib/urls';

// Cho phép mọi công cụ tìm kiếm và AI crawler đọc kho (để được trích dẫn trên Google AI Overview, ChatGPT…).
export const GET: APIRoute = ({ site }) =>
  new Response(
    [
      'User-agent: *',
      'Allow: /',
      `Disallow: ${url('gaps/')}`,
      '',
      `Sitemap: ${absolute(url('sitemap.xml'), site)}`,
      `# Chỉ mục cho AI agent: ${absolute(url('llms.txt'), site)}`,
      '',
    ].join('\n'),
    { headers: { 'Content-Type': 'text/plain; charset=utf-8' } },
  );
