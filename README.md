# agent-papercuts

A small CLI for recording "papercuts" the pieces of friction agents encounter while
working: misleading errors, missing tools, broken links, and undocumented setup
steps.

It writes Markdown to `PAPERCUTS.md`, so the log can be reviewed and fixed  like any other repository artifact.

## Install

```sh
cargo install --path .
```

For development, run it without installing:

```sh
cargo run -- -m gpt-5.6-terra "The command's error did not explain the required setup."
```

## Use

```sh
papercut --author aditya -m gpt-5.6-terra "The local database reset command requires Docker, but the setup guide does not mention it."
papercut --author codex -m claude-sonnet-5 --file docs/PAPERCUTS.md "The API reference link redirects to a removed page."
```

The default author is `unknown`; use `--author` to record a person or agent.
The default log is `PAPERCUTS.md` in the current directory. `--file` chooses a
different log; its parent directories are created when needed. When run from a
Git repository root without `--file`, `papercut` stops and recommends
`docs/PAPERCUTS.md` instead, so repository friction stays with its project. The
message may contain multiple words and whitespace is normalized to one space.
Model and author labels may not contain ` — `, which separates fields in each
heading.

Use `papercut --help` for the complete, generated interface.

## Output

The first write creates the log and subsequent writes append entries. Timestamps
are UTC RFC 3339 values.

```md
# Papercuts

Small, non-blocking workflow friction recorded by agents.

## 2026-07-10T06:54:19.910458Z — gpt-5.6-terra — aditya

The documentation omits the required environment variable.
```

Argument errors are handled by clap. Filesystem failures include the affected
path and a color-eyre error report.

## Agent instruction

Add something like this to your `AGENTS.md`:

> When you encounter a small, non-blocking source of friction—such as a dead-end
> tool call, misleading error, missing setup step, stale cache, or broken
> link—record it immediately with `papercut --author <author> -m <model> "<what
> happened and what would have prevented it>"`. Keep it to one or two sentences.
> Do not log normal task progress, bugs that belong in the issue tracker, or
> secrets. Continue the task unless the problem blocks it.

Start by reviewing `PAPERCUTS.md` during maintenance work. Keep automatic
session mining as a separate, explicitly triggered command; unsolicited model
reviews would create noisy and potentially expensive output.

## Development

```sh
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test --all-targets --all-features
```
