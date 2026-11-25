---
name: project-harness
description: "Run substantial coding, research and 3D tasks in this workspace with project profiles, resumable checkpoints and evidence-backed local completion."
---

# Claude Code compatibility

This is an adapted local export; original instructions are in CODEX-SKILL.md. Use Claude's available tools. Codex namespaces (functions.*, tools.*, image_gen, collaboration, mcp__codex_apps__*, skills.read) are not installed by this folder. Never invent tool calls, credentials, or a successful connection. Read AGENTS.md and CLAUDE.md at the applicable scope. The user's current authorization remains authoritative.

In Second Brain the workspace is the repository root and the only project is `secondbrain` (`node .harness/harness.mjs start --project secondbrain ...`); `.harness/README.md` explains the commands in Vietnamese. Reply to the user in Vietnamese per CLAUDE.md. Locate the workspace containing `.harness/projects.json`. Read `.harness/WORKFLOW.md` and the target project's `AGENTS.md`. Use its registered checks and preserve its data/model authority.

Use clear mode for bounded requests. Explore only a material uncertainty: record alternatives and probe the disputed premise on copies before composing a solution. Do not automatically multiply agents, model calls or test runs.

For substantive work, CLI `start` needs project, title and observable acceptance. Define scope in the generated brief. Maintain `checkpoint` at milestones/before resets: facts, paths, limitations, next concrete action.

Run `verify`, inspect actual behavior/artifacts and applicable manual gates, then `review` with a real evidence file and explicit results. Use `finish` only when the requested local outcome is achieved. Keep active when required work remains. The CLI cannot judge prose acceptance or certify physical results.

After repeated failures sharing a premise, probe that premise. Record only demonstrated reusable lessons with evidence in `.harness/LESSONS.md`; prefer focused executable checks over more instructions.

This is a local adaptation of upstream workflow ideas, not their runtime installation. It supplies neither background agents nor automatic publication. Session authorization remains authoritative.
