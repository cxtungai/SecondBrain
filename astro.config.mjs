// @ts-check
import { defineConfig } from 'astro/config';

// SITE_URL và BASE_PATH được GitHub Actions điền khi deploy (xem .github/workflows/deploy.yml).
// Chạy local thì để trống: site chạy ở http://localhost:4321/
export default defineConfig({
  site: process.env.SITE_URL || undefined,
  base: process.env.BASE_PATH || '/',
  trailingSlash: 'ignore',
  build: { format: 'directory' },
  server: { host: true },
  devToolbar: { enabled: false },
});
