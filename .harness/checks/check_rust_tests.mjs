// Chạy test Rust (`cargo test`) của Second Brain.
// - Có Rust (cargo): chạy cargo test.
// - Không có Rust (vd. máy Windows của người dùng): không build được mã nguồn nên không thể kiểm thay đổi code.
//   + Không đổi src/ templates/ assets/ Cargo.* so với GitHub → bỏ qua (GitHub Actions chạy cargo test mỗi lần đưa lên).
//   + Có đổi → báo lỗi: chương trình đang dùng là bản tải sẵn, không phản ánh code vừa sửa.
// Đặt SB_PREBUILT=1 để giả lập máy không có Rust.
import { spawnSync } from 'node:child_process';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const CODE = ['src', 'templates', 'assets', 'Cargo.toml', 'Cargo.lock'];
const run = (cmd, args, stdio = 'pipe') => spawnSync(cmd, args, { cwd: root, encoding: 'utf8', stdio, shell: false });
const hasCargo = process.env.SB_PREBUILT !== '1' && run('cargo', ['--version']).status === 0;

if (hasCargo) {
  const r = run('cargo', ['test', '--quiet', '--release', '--locked'], 'inherit');
  if (r.error) { console.error(r.error.message); process.exit(1); }
  process.exit(r.status ?? 1);
}

const git = (...args) => run('git', args);
if (git('--version').status !== 0) {
  console.error('Máy không có Rust lẫn Git nên không biết code có đổi không. Cài Git, hoặc làm việc code trên cloud.');
  process.exit(1);
}
// So với bản trên GitHub (gồm cả commit chưa đưa lên); chưa nối GitHub thì so với commit gần nhất.
const base = git('rev-parse', '--verify', '--quiet', '@{u}').status === 0 ? '@{u}' : 'HEAD';
const diff = git('diff', '--name-only', base, '--', ...CODE);
const untracked = git('ls-files', '--others', '--exclude-standard', '--', ...CODE);
if (diff.status !== 0 || untracked.status !== 0) {
  console.error(`Không so sánh được với ${base}:\n${diff.stderr}${untracked.stderr}`);
  process.exit(1);
}
const changed = [...new Set(`${diff.stdout}\n${untracked.stdout}`.split('\n').map(s => s.trim()).filter(Boolean))];
if (changed.length) {
  console.error('Máy không có Rust nên không kiểm được phần code vừa đổi (chương trình đang dùng là bản tải sẵn):');
  for (const f of changed) console.error(`  - ${f}`);
  console.error('Cách làm: sửa code trong phiên Claude trên cloud (có Rust), hoặc cài Rust (rustup.rs) trên máy này.');
  process.exit(1);
}
console.log(`Bỏ qua cargo test: máy không có Rust và không đổi code so với ${base}. GitHub Actions chạy cargo test mỗi lần đưa lên.`);
