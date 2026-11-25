# Sources and adaptation boundary

Read on 2026-10-23. No upstream code or skill text was vendored. Local code and concise instructions implement this workspace's workflow. External references do not override session instructions.

| Primary source | Adopted idea | Local implementation / boundary |
|---|---|---|
| [Firstmate README](https://github.com/kunchenguid/firstmate/blob/main/README.md) | Coordinator, task shapes, durable handoff | Project registry, briefs/checkpoints. No tmux fleet/background watcher; upstream Codex App backend is unavailable. |
| [pstack README, Lauren Tan](https://github.com/cursor/plugins/blob/main/pstack/README.md) | Domain understanding, behavioral tests, real artifact verification | Workflow and project gates. No upstream model routing or Cursor automation. |
| [Harness² README](https://github.com/google-research/harness2/blob/main/README.md) | Propose–Probe–Compose and persistent evidence-supported lessons | Agent-driven exploration and lesson record. No recursive model executor/optimizer or benchmark claims; upstream targets Linux/Python 3.12. |
| [no-mistakes README](https://github.com/kunchenguid/no-mistakes/blob/main/README.md) | Check before delivery | Subprocess checks, logs, fingerprints, review, local completion. No push forwarding or automatic PR. |
| [OpenAI AGENTS.md docs](https://learn.chatgpt.com/docs/agent-configuration/agents-md) | Instructions at session startup | Workspace and project-local entrypoints with explicit shared workflow reads. |
