import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, mkdirSync, writeFileSync, readFileSync, rmSync, existsSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { resolve } from 'node:path';
import { main, fingerprint, contained, loadProjects } from '../harness.mjs';

function fixture(t, script = 'console.log("real check ran")') {
  const root = mkdtempSync(resolve(tmpdir(), 'project-harness-'));
  t.after(() => rmSync(root, { recursive: true, force: true }));
  mkdirSync(resolve(root, '.harness'), { recursive: true });
  mkdirSync(resolve(root, 'app'));
  writeFileSync(resolve(root, 'app/check.mjs'), script);
  writeFileSync(resolve(root, 'app/source.txt'), 'initial source');
  writeFileSync(resolve(root, 'app/AGENTS.md'), 'fixture instructions');
  const project = { id: 'app', path: 'app', instructions: ['app/AGENTS.md'], required_files: ['app/source.txt'], inputs: ['app'], checks: [{ name: 'actual check', argv: ['{node}', 'check.mjs'], timeout_ms: 5000 }], manual_gates: ['Inspect actual behavior'] };
  writeFileSync(resolve(root, '.harness/projects.json'), JSON.stringify({ version: 1, projects: [project] }));
  const run = (...args) => main(args, root);
  const start = () => run('start', '--project', 'app', '--title', 'Fix behavior', '--accept', 'Expected output is observed');
  return { root, run, start, project };
}
function reviewed(f, task) {
  writeFileSync(resolve(f.root, 'evidence.md'), 'Observed expected behavior and inspected output');
  return f.run('review', '--task', task.id, '--summary', 'Checked the real result', '--evidence', 'evidence.md', '--gates', 'Behavior inspected; no physical claims');
}

test('task survives reload, keeps checkpoint, executes checks and completes locally', t => {
  const f = fixture(t), task = f.start();
  f.run('checkpoint', '--task', task.id, '--note', 'Changed app/source.txt; behavior observed', '--next', 'Review actual result');
  assert.match(main(['status', '--task', task.id], f.root).checkpoint, /Review actual result/);
  assert.equal(f.run('verify', '--task', task.id).passed, true);
  assert.match(readFileSync(resolve(task.directory, 'logs/1.txt'), 'utf8'), /real check ran/);
  reviewed(f, task);
  assert.equal(f.run('finish', '--task', task.id).status, 'local-complete');
  assert.equal(f.run('status')[0].status, 'local-complete');
  assert.throws(() => f.run('checkpoint', '--task', task.id, '--note', 'x', '--next', 'y'), /already complete/);
});
test('failed subprocess is logged and cannot finish', t => {
  const f = fixture(t, 'console.error("specific failure"); process.exit(7)'), task = f.start();
  const report = f.run('verify', '--task', task.id);
  assert.equal(report.passed, false);
  assert.equal(report.results[0].exit_code, 7);
  assert.match(readFileSync(resolve(task.directory, 'logs/1.txt'), 'utf8'), /specific failure/);
  reviewed(f, task);
  assert.throws(() => f.run('finish', '--task', task.id), /checks failed/);
});
test('source edits invalidate verification and review', t => {
  const f = fixture(t), task = f.start();
  f.run('verify', '--task', task.id); reviewed(f, task);
  writeFileSync(resolve(f.root, 'app/source.txt'), 'changed source');
  assert.throws(() => f.run('finish', '--task', task.id), /changed after/);
  f.run('verify', '--task', task.id);
  assert.throws(() => f.run('finish', '--task', task.id), /changed after/);
  reviewed(f, task);
  assert.equal(f.run('finish', '--task', task.id).status, 'local-complete');
});
test('review evidence changes invalidate completion', t => {
  const f = fixture(t), task = f.start();
  f.run('verify', '--task', task.id); reviewed(f, task);
  writeFileSync(resolve(f.root, 'evidence.md'), 'different report');
  assert.throws(() => f.run('finish', '--task', task.id), /evidence changed/);
});
test('source mutation during a check fails verification', t => {
  const f = fixture(t, 'import {writeFileSync} from "node:fs";writeFileSync("source.txt","mutated during checks")'), task = f.start();
  const report = f.run('verify', '--task', task.id);
  assert.equal(report.results[0].passed, true);
  assert.equal(report.source_unchanged, false);
  assert.equal(report.passed, false);
});
test('missing executable is a failure with actionable log', t => {
  const f = fixture(t), path = resolve(f.root, '.harness/projects.json');
  f.project.checks[0].argv = [resolve(f.root, 'does-not-exist.exe')];
  writeFileSync(path, JSON.stringify({ version: 1, projects: [f.project] }));
  const task = f.start(), report = f.run('verify', '--task', task.id);
  assert.equal(report.passed, false);
  assert.match(readFileSync(resolve(task.directory, 'logs/1.txt'), 'utf8'), /ENOENT/);
});
test('timeout is a failed gate', t => {
  const f = fixture(t, 'setInterval(()=>{},1000)'), path = resolve(f.root, '.harness/projects.json');
  f.project.checks[0].timeout_ms = 100;
  writeFileSync(path, JSON.stringify({ version: 1, projects: [f.project] }));
  assert.equal(f.run('verify', '--task', f.start().id).passed, false);
});
test('rejects path traversal and unknown projects before creating tasks', t => {
  const f = fixture(t);
  assert.throws(() => contained(f.root, '../outside.txt'), /outside workspace/);
  assert.throws(() => f.run('status', '--task', '../../outside'), /Invalid/);
  assert.throws(() => f.run('start', '--project', 'unknown', '--title', 'x', '--accept', 'y'), /Unknown project/);
  assert.equal(existsSync(resolve(f.root, '.harness/runs')), false);
});
test('requires acceptance, real review evidence and nonempty check configuration', t => {
  const f = fixture(t);
  assert.throws(() => f.run('start', '--project', 'app', '--title', 'x'), /accept/);
  const task = f.start();
  assert.throws(() => f.run('review', '--task', task.id, '--summary', 'ok', '--evidence', 'missing.md', '--gates', 'done'), /existing file/);
  assert.throws(() => f.run('finish', '--task', task.id), /ENOENT/);
  f.project.checks = [];
  writeFileSync(resolve(f.root, '.harness/projects.json'), JSON.stringify({ version: 1, projects: [f.project] }));
  assert.throws(() => loadProjects(f.root), /at least one/);
});
test('distinct IDs keep separate tasks and generated output does not stale checks', t => {
  const f = fixture(t), first = f.start(), second = f.start();
  assert.notEqual(first.id, second.id);
  const before = fingerprint(f.root, f.project);
  mkdirSync(resolve(f.root, 'app/dist'));
  writeFileSync(resolve(f.root, 'app/dist/bundle.js'), 'generated');
  assert.equal(fingerprint(f.root, f.project), before);
  writeFileSync(resolve(f.root, 'app/new-feature.mjs'), 'export const value=1');
  assert.notEqual(fingerprint(f.root, f.project), before);
});
test('loose workspace source is tracked without traversing artifact directories', t => {
  const f = fixture(t); f.project.shallow_inputs = ['.'];
  const before = fingerprint(f.root, f.project);
  mkdirSync(resolve(f.root, 'archive'));
  writeFileSync(resolve(f.root, 'archive/old.json'), '{}');
  assert.equal(fingerprint(f.root, f.project), before);
  writeFileSync(resolve(f.root, 'new-script.py'), 'print(1)');
  assert.notEqual(fingerprint(f.root, f.project), before);
});
test('baseline records real project failures and can select one project', t => {
  const f = fixture(t, 'process.exit(3)');
  const report = f.run('baseline', '--project', 'app');
  assert.equal(report.passed, false);
  assert.equal(report.projects.length, 1);
  assert.equal(report.projects[0].checks[0].exit_code, 3);
  assert.equal(JSON.parse(readFileSync(resolve(f.root, '.harness/reports/baseline/summary.json'), 'utf8')).passed, false);
});
