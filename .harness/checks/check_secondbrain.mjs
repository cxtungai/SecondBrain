// Chạy `./sb check` của Second Brain trên mọi máy (Windows, macOS, Linux/cloud).
// - Có Rust (cargo): build từ mã nguồn, như ./sb.
// - Windows không có cargo: dùng bin/secondbrain.exe (tải sẵn bằng ./sb hoặc Mo-Second-Brain.bat).
// - macOS/Linux: gọi bash ./sb (tự tải chương trình nếu chưa có).
import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const args = process.argv.slice(2).length ? process.argv.slice(2) : ['check'];
const has = command => spawnSync(command, ['--version'], { stdio: 'ignore', shell: false }).status === 0;

let executable, argv;
if (has('cargo') && process.env.SB_PREBUILT !== '1') {
  executable = 'cargo'; argv = ['run', '--quiet', '--release', '--', ...args];
} else if (process.platform === 'win32') {
  executable = resolve(root, 'bin/secondbrain.exe'); argv = args;
  if (!existsSync(executable)) {
    console.error('Chưa có bin/secondbrain.exe. Chạy ./sb check một lần (hoặc bấm đúp Mo-Second-Brain.bat) để tải chương trình.');
    process.exit(2);
  }
} else {
  executable = 'bash'; argv = [resolve(root, 'sb'), ...args];
}

const result = spawnSync(executable, argv, { cwd: root, stdio: 'inherit', shell: false });
if (result.error) { console.error(result.error.message); process.exit(1); }
process.exit(result.status ?? 1);
