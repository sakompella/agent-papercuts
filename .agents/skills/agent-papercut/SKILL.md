---
name: agent-papercut
description: Record a small, non-blocking source of workflow friction with this repository's papercut CLI. Use when a task reveals a misleading error, dead-end command, broken link, stale cache, or missing setup step that a maintainer could remove.
compatibility: Requires Git and Cargo. Run from this repository's Git root; works with any agent runtime that supports the Agent Skills SKILL.md format.
---

# Agent Papercut

Record actionable friction while it is fresh. The log stays local and
write-only: do not call a model, send data, or mine a session automatically.

## When to record

Log a small, non-blocking obstacle that a maintainer could remove, such as a
misleading error, dead-end command, broken link, stale cache, or undocumented
setup requirement. Do not log ordinary task progress, secrets, or bugs that
belong in the issue tracker.

Write one or two sentences that say what you were doing, what got in the way,
and, when known, what would have prevented it.

## Record an entry

From this repository's Git root, run:

```sh
cargo run -- --file docs/PAPERCUTS.md --author <author> -m <model> "<what happened and what would have prevented it>"
```

Always include `--file docs/PAPERCUTS.md` at the Git root. Without it, the CLI
intentionally stops and recommends that location rather than writing the
current directory's `PAPERCUTS.md`.

Use the actual agent and model identifiers when available. Omit either flag
only when it is unknown; the CLI records `unknown` for the author and
`unspecified-model` for the model.

Continue the task unless the friction blocks progress.

## Review

Read `docs/PAPERCUTS.md` during maintenance work to find accumulated small
improvements. Any bulk review or session mining must be explicit and
user-triggered.
