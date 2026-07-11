---
name: log-papercuts
description: Record small, non-blocking workflow friction in this project's PAPERCUTS.md. Use when an agent encounters a dead-end command, misleading error, broken link, stale cache, or missing or confusing setup step while working.
---

# Log Papercuts

Record actionable friction while it is fresh. Keep the log local and
write-only; do not call a model or mine a session automatically.

## Workflow

1. Log only small, non-blocking friction that a maintainer could remove. Do
   not log ordinary task progress, secrets, or bugs that belong in the issue
   tracker.
2. Write one or two sentences: what you were doing, what got in the way, and,
   when known, what would have prevented it.
3. Run the project CLI from the repository root:

   ```sh
   cargo run -- --author <author> -m <model> "<what happened and what would have prevented it>"
   ```

   Use the actual agent and model identifiers when available. Omit either flag
   only when it is unknown; the CLI records `unknown` for the author and
   `unspecified-model` for the model.
4. Continue the task unless the friction blocks progress.

## Review

Read `PAPERCUTS.md` during maintenance work to identify small, accumulated
improvements. Keep any bulk review or session mining explicit and
user-triggered.
