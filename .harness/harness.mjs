import { readFileSync, writeFileSync, mkdirSync, readdirSync, existsSync, realpathSync, renameSync, statSync } from 'node:fs';
import { resolve, relative, dirname, sep, extname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { createHash, randomUUID } from 'node:crypto';
import { spawnSync } from 'node:child_process';

export const workspace = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const json = path => JSON.parse(readFileSync(path, 'utf8').replace(/^\uFEFF/, ''));
const save = (path, value) => {
  mkdirSync(dirname(path), { recursive: true });
  const temporary = `${path}.${randomUUID()}.tmp`;
  writeFileSync(temporary, JSON.stringify(value, null, 2) + '\n');
  renameSync(temporary, path);
};
const need = (value, message) => { if (!value) throw new Error(message); return value; };
const now = () => new Date().toISOString();
const excluded = new Set(['.git', 'node_modules', 'dist', '.venv', '.venv-seam311', '.runtime', '.engine_snapshots', '.installation-backups', '__pycache__', '.npm-cache', 'test-results', 'test-artifacts', 'runs', 'reports']);
const extensions = new Set(['.rs', '.lock', '.jinja', '.svg', '.sh', '.bat', '.command', '.mjs', '.js', '.cjs', '.py', '.toml', '.json', '.md', '.txt', '.html', '.css', '.ps1', '.cmd', '.yaml', '.yml']);

export function contained(root, path) {
  const target = resolve(root, path);
  const inside = value => { const r = relative(root, value); return r !== '..' && !r.startsWith(`..${sep}`) && !r.includes(':'); };
  need(inside(target), `Path outside workspace: ${path}`);
  let ancestor = target;
  while (!existsSync(ancestor)) ancestor = dirname(ancestor);
  need(inside(realpathSync(ancestor)), `Linked path outside workspace: ${path}`);
  return target;
}

export function loadProjects(root = workspace) {
  const config = json(resolve(root, '.harness/projects.json'));
  need(config.version === 1 && Array.isArray(config.projects), 'Invalid project registry');
  const ids = new Set();
  for (const p of config.projects) {
    need(/^[a-z0-9][a-z0-9-]*$/.test(p.id) && !ids.has(p.id), `Invalid/duplicate project: ${p.id}`);
    ids.add(p.id);
    contained(root, p.path);
    need(Array.isArray(p.checks) && p.checks.length > 0, `${p.id}: configure at least one automated check`);
    for (const check of p.checks) {
      need(typeof check.name === 'string' && Array.isArray(check.argv) && check.argv.length > 0 && check.argv.every(v => typeof v === 'string'), 'Invalid check argv');
      need(Number.isInteger(check.timeout_ms) && check.timeout_ms > 0, 'Check timeout must be positive');
    }
  }
  return config.projects;
}

export function fingerprint(root, project) {
  const hash = createHash('sha256');
  const seen = new Set();
  const visit = path => {
    path = contained(root, path);
    if (seen.has(path)) return;
    seen.add(path);
    if (!existsSync(path)) { hash.update(`missing:${relative(root, path)}\0`); return; }
    if (statSync(path).isDirectory()) {
      for (const entry of readdirSync(path, { withFileTypes: true }).sort((a,b) => a.name.localeCompare(b.name))) {
        if (entry.isSymbolicLink() || excluded.has(entry.name)) continue;
        if (entry.isDirectory() || extensions.has(extname(entry.name))) visit(resolve(path, entry.name));
      }
    } else {
      hash.update(relative(root, path).replaceAll('\\', '/') + '\0');
      hash.update(readFileSync(path));
    }
  };
  for (const input of ['.harness/harness.mjs', '.harness/projects.json', '.harness/WORKFLOW.md', 'AGENTS.md', ...project.inputs]) visit(input);
  for (const input of project.shallow_inputs || []) {
    const directory = contained(root, input);
    for (const entry of readdirSync(directory, { withFileTypes: true }).sort((a,b) => a.name.localeCompare(b.name))) {
      if (entry.isFile() && extensions.has(extname(entry.name))) visit(resolve(directory, entry.name));
    }
  }
  return hash.digest('hex');
}

function commandSpec(root, project, check) {
  const argv = check.argv.map(v => v.replaceAll('{workspace}', root).replaceAll('{node}', process.execPath));
  // npm's .cmd shim cannot be spawned without cmd.exe; execute its JS entrypoint instead.
  if (argv[0] === 'npm') {
    const npmCli = resolve(dirname(process.execPath), 'node_modules/npm/bin/npm-cli.js');
    need(existsSync(npmCli), `npm CLI missing: ${npmCli}`);
    return { executable: process.execPath, args: [npmCli, ...argv.slice(1)] };
  }
  return { executable: argv[0], args: argv.slice(1) };
}

export function runChecks(root, project, directory) {
  const before = fingerprint(root, project);
  mkdirSync(resolve(directory, 'logs'), { recursive: true });
  const results = project.checks.map((check, i) => {
    let result;
    const started = Date.now();
    try {
      const command = commandSpec(root, project, check);
      result = spawnSync(command.executable, command.args, {
        cwd: contained(root, project.path), encoding: 'utf8', shell: false,
        timeout: check.timeout_ms, maxBuffer: 16 * 1024 * 1024, windowsHide: true,
      });
    } catch (error) { result = { status: null, error }; }
    const log = `logs/${i + 1}.txt`;
    writeFileSync(resolve(directory, log), `${result.stdout || ''}${result.stderr || ''}${result.error ? `\n${result.error.message}` : ''}`);
    return { name: check.name, argv: check.argv, exit_code: result.status, passed: result.status === 0 && !result.error, elapsed_ms: Date.now() - started, log };
  });
  const after = fingerprint(root, project);
  const report = { at: now(), project: project.id, fingerprint: before, source_unchanged: before === after, passed: before === after && results.every(r => r.passed), results, manual_gates: project.manual_gates };
  save(resolve(directory, 'checks.json'), report);
  return report;
}

function options(args) {
  const result = {};
  for (let i = 0; i < args.length; i += 2) {
    need(args[i].startsWith('--') && args[i + 1] && !args[i + 1].startsWith('--'), `Expected --key value: ${args[i]}`);
    const key = args[i].slice(2);
    need(!Object.hasOwn(result, key), `Duplicate option: ${key}`);
    result[key] = args[i + 1];
  }
  return result;
}
function taskDirectory(root, id) {
  need(/^[a-z0-9-]+$/.test(id || ''), 'Invalid or missing --task');
  return contained(root, `.harness/runs/${id}`);
}
function getTask(root, id) {
  const directory = taskDirectory(root, id);
  const task = json(resolve(directory, 'task.json'));
  need(task.id === id, 'Task ID mismatch');
  return { directory, task };
}

export function main(args, root = workspace) {
  const [command = 'help', ...rest] = args;
  if (command === 'help') return 'harness: list | doctor | baseline [--project ID] | start --project ID --title TEXT --accept TEXT [--mode clear|explore] | status [--task ID] | checkpoint --task ID --note TEXT --next TEXT | verify --task ID | review --task ID --summary TEXT --evidence PATH --gates TEXT | finish --task ID';
  const opt = options(rest);
  const projects = loadProjects(root);
  const projectById = id => need(projects.find(p => p.id === id), `Unknown project: ${id}`);
  if (command === 'list') return projects.map(p => ({ id: p.id, path: p.path, checks: p.checks.map(c => c.name), manual_gates: p.manual_gates }));
  if (command === 'doctor') {
    const issues = [];
    for (const p of projects) {
      for (const path of [p.path, ...p.instructions, ...p.required_files]) if (!existsSync(contained(root, path))) issues.push(`${p.id}: missing ${path}`);
      for (const check of p.checks) {
        const spec = commandSpec(root, p, check);
        if ((spec.executable.includes('/') || spec.executable.includes('\\')) && !existsSync(spec.executable)) issues.push(`${p.id}: missing executable ${spec.executable}`);
      }
    }
    need(issues.length === 0, issues.join('\n'));
    return { ready: true, projects: projects.length, node: process.version };
  }
  if (command === 'baseline') {
    const selected = opt.project ? [projectById(opt.project)] : projects;
    const reports = selected.map(project => {
      const directory = contained(root, `.harness/reports/baseline/${project.id}`);
      const result = runChecks(root, project, directory);
      return { project: project.id, passed: result.passed, checks: result.results, report: relative(root, resolve(directory, 'checks.json')) };
    });
    const summary = { at: now(), passed: reports.every(r => r.passed), projects: reports };
    save(resolve(root, '.harness/reports/baseline/summary.json'), summary);
    return summary;
  }
  if (command === 'start') {
    const project = projectById(opt.project);
    need(opt.title?.trim() && opt.accept?.trim(), 'Supply --title and observable --accept criteria');
    const mode = opt.mode || 'clear';
    need(['clear', 'explore'].includes(mode), 'Mode must be clear or explore');
    const id = `${project.id}-${new Date().toISOString().replace(/\D/g, '').slice(0, 14)}-${randomUUID().slice(0, 8)}`;
    const directory = taskDirectory(root, id);
    const task = { id, project: project.id, title: opt.title, acceptance: opt.accept, mode, status: 'active', created_at: now(), initial_fingerprint: fingerprint(root, project) };
    save(resolve(directory, 'task.json'), task);
    writeFileSync(resolve(directory, 'brief.md'), `# ${task.title}\n\nProject: ${project.id}\nMode: ${mode}\n\nAcceptance: ${task.acceptance}\n\nScope: record the files or geometry regions to change before editing.\n\n${mode === 'explore' ? 'Scout evidence -> propose 2-3 alternatives -> probe the uncertain premise on copies -> compare -> compose the chosen approach. Record results and rejected alternatives below.\n' : 'Read the project profile -> implement the bounded change -> verify -> review.\n'}\nDo not treat a generated brief as an approved design.\n`);
    writeFileSync(resolve(directory, 'checkpoint.md'), `# Resume ${id}\n\nGoal: ${task.title}\nAcceptance: ${task.acceptance}\nNext: inspect project instructions and define the bounded scope.\n`);
    return { ...task, directory };
  }
  if (command === 'status' && !opt.task) {
    const runs = resolve(root, '.harness/runs');
    return existsSync(runs) ? readdirSync(runs, { withFileTypes: true }).filter(d => d.isDirectory() && existsSync(resolve(runs, d.name, 'task.json'))).map(d => json(resolve(runs, d.name, 'task.json'))) : [];
  }
  const { directory, task } = getTask(root, opt.task);
  const project = projectById(task.project);
  if (command === 'status') return { ...task, directory, checkpoint: readFileSync(resolve(directory, 'checkpoint.md'), 'utf8'), checks: existsSync(resolve(directory, 'checks.json')) ? json(resolve(directory, 'checks.json')) : null };
  need(task.status === 'active', 'Task already complete; start a new task for further changes');
  if (command === 'checkpoint') {
    need(opt.note?.trim() && opt.next?.trim(), 'Supply --note and --next');
    writeFileSync(resolve(directory, 'checkpoint.md'), `# Resume ${task.id}\n\nUpdated: ${now()}\nGoal: ${task.title}\nAcceptance: ${task.acceptance}\n\nVerified facts / changed files / remaining issues:\n${opt.note}\n\nNext action:\n${opt.next}\n`);
    return { checkpoint: resolve(directory, 'checkpoint.md') };
  }
  if (command === 'verify') return runChecks(root, project, directory);
  if (command === 'review') {
    need(opt.summary?.trim() && opt.evidence && opt.gates?.trim(), 'Supply --summary, --evidence file, and --gates (manual gate results/limitations)');
    const evidence = contained(root, opt.evidence);
    need(existsSync(evidence) && statSync(evidence).isFile(), 'Review evidence must be an existing file in the workspace');
    save(resolve(directory, 'review.json'), { at: now(), summary: opt.summary, gates: opt.gates, evidence: relative(root, evidence), evidence_sha256: createHash('sha256').update(readFileSync(evidence)).digest('hex'), fingerprint: fingerprint(root, project), kind: 'agent-reported-review' });
    return { recorded: true, note: 'Review is agent-reported evidence, not independent certification.' };
  }
  if (command === 'finish') {
    const checks = json(resolve(directory, 'checks.json'));
    const review = json(resolve(directory, 'review.json'));
    const current = fingerprint(root, project);
    need(checks.passed && checks.results?.length === project.checks.length && checks.results.every(r => r.passed), 'Automated checks failed or missing');
    need(current === checks.fingerprint && current === review.fingerprint, 'Source/configuration changed after checks or review; run both again');
    const evidence = contained(root, review.evidence);
    need(createHash('sha256').update(readFileSync(evidence)).digest('hex') === review.evidence_sha256, 'Review evidence changed; review again');
    task.status = 'local-complete'; task.completed_at = now();
    save(resolve(directory, 'task.json'), task);
    return { ...task, note: 'Local task complete. External delivery and physical validation retain their own scope and authorization.' };
  }
  throw new Error(`Unknown command: ${command}`);
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try {
    const result = main(process.argv.slice(2));
    console.log(typeof result === 'string' ? result : JSON.stringify(result, null, 2));
    if (result?.passed === false) process.exitCode = 1;
  } catch (error) { console.error(error.message); process.exitCode = 1; }
}
