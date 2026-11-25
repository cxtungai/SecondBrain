// Đọc tham số dòng lệnh đơn giản: vị trí + --co=gia-tri / --co gia-tri / --co
export function parseArgs(argv = process.argv.slice(2)) {
  const positional = [];
  const flags = {};
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a.startsWith('--')) {
      const [key, inline] = a.slice(2).split(/=(.*)/s);
      if (inline !== undefined) flags[key] = inline;
      else if (argv[i + 1] !== undefined && !argv[i + 1].startsWith('--')) flags[key] = argv[++i];
      else flags[key] = true;
    } else positional.push(a);
  }
  return { positional, flags };
}

export const today = () => {
  const d = new Date();
  const pad = (n) => String(n).padStart(2, '0');
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;
};

const useColor = process.stdout.isTTY && !process.env.NO_COLOR;
const wrap = (code) => (s) => (useColor ? `\x1b[${code}m${s}\x1b[0m` : String(s));
export const c = { red: wrap(31), yellow: wrap(33), green: wrap(32), cyan: wrap(36), dim: wrap(2), bold: wrap(1) };
