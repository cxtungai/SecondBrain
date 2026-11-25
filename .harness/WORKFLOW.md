# Local project workflow

The coordinator owns the user outcome and selects the project profile. Implementation and review are separate passes; one agent can perform both. This workflow does not select models or authorize delegation.

1. **Intake:** read project instructions and existing current-state evidence; inspect uncommitted changes. State the outcome and observable acceptance. Identify files, interfaces, source geometry and edit regions that may change.
2. **Route:** use `clear` for bounded work. Use `explore` when evidence is insufficient: record 2–3 approaches, probe the important uncertainty on isolated copies, compare observed results/cost, compose the smallest supported solution. Usually one round is enough. After two failed fixes sharing one premise, test that premise before retrying.
3. **Execute:** work in verifiable increments. Preserve unrelated changes and immutable 3D source. Avoid heavyweight tests, repeated renders or broad scans without a task-specific reason.
4. **Checkpoint:** save verified facts, modified paths, artifact locations, limitations and the next concrete step. Task files are the handoff. `brief.md` records scope, options and decisions. Reference large evidence by path.
5. **Verify:** run profile checks with `verify`. It records argv, exit status, duration, logs and SHA-256 of configured inputs before/after the run. A failing check or input change fails the automated gate. Additional runtime/geometry checks depend on affected behavior; cite their output in review evidence. Missing dependencies are failures.
6. **Review:** inspect real outputs against acceptance. Write substantive evidence, then `review` with its path, findings and manual gate results. This is agent-reported review, not independent certification. If a limitation prevents the requested outcome, continue work instead of finishing.
7. **Deliver:** `finish` requires successful current checks, current review and unchanged review evidence. It marks `local-complete`; it never commits, pushes, merges, sends messages or changes external tools. Complete external actions already authorized by the user with appropriate checks.
8. **Improve:** record a narrow lesson with failure, root cause, correction and evidence in `LESSONS.md` when useful. Compare an improvement against the same baseline before adoption. Do not weaken a gate to pass it.

CLI: `node .harness/harness.mjs help` from the workspace, or use its absolute path from a project. `status` lists tasks; `status --task ID` loads one checkpoint. Do not run mutating commands concurrently against one task.

Coverage is explicit in `projects.json.inputs`. Generated builds, runtime environments, historical 3D projects and caches are excluded from the shared source fingerprint. For a 3D task, record a manifest of changed source/output files and their hashes in review evidence, run checks on those actual files, and re-review after changes. The CLI checks the review file hash; it does not monitor every model mentioned in prose. This avoids hashing gigabytes of historical models on every task.

New work within a registered project uses its profile. Before adopting a new independent project, add its path, instructions, source inputs, non-empty executable checks and manual gates to `projects.json`, then run `doctor`. Keep portable checkers under `.harness/checks`; do not invent commands absent from the project.
