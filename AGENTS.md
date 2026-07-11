# agent-papercut

`papercut` is a local Rust CLI that appends small, non-blocking workflow
friction notes to a Markdown log. Keep it write-only: do not add network calls,
model calls, or automatic session mining.

## Project layout

- `src/main.rs` contains the binary and its implementation. Keep it ordered
  top-down: entrypoint and command flow first, lower-level helpers last.
- `tests/papercut.rs` contains end-to-end CLI tests and the Hegel property test
  for text normalization. Keep behavior tests outside `src/`.
- `README.md` documents installation, CLI use, output, and the reusable agent
  instruction. Update it for user-visible CLI or output changes.
- `docs/SOURCE.md` records the source thread, screenshots, and transcript.
  Preserve its source links and captures when changing the attribution material.

## Behavior to preserve

- The default output is `PAPERCUTS.md` in the current directory; `--file` may
  select another path and its parent directories are created as needed.
- An empty or newly created log begins with the `# Papercuts` heading. Entries
  are UTC RFC 3339 level-two Markdown headings followed by a normalized,
  non-empty message.
- Whitespace in messages, models, and authors is collapsed to one space.
  Omitted or blank labels fall back to `unspecified-model` and `unknown`.
- `--author` is the explicit attribution mechanism. Do not restore `$USER` or
  add a dependency solely to discover a username.

## Development

Run these checks before committing Rust changes:

```sh
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test --all-targets --all-features
```

Use `clap`, `color-eyre`, and `jiff` for the existing CLI, diagnostics, and
time handling. Keep the Clippy configuration in `Cargo.toml` clean; the
workspace treats all enabled lint groups as warnings.

## Log papercuts

When work exposes a small, non-blocking friction—such as a dead-end command,
misleading error, broken link, stale cache, or missing setup step—record it in
the moment:

```sh
cargo run -- --author <author> -m <model> "<what happened and what would have prevented it>"
```

Keep the note to one or two sentences. Do not record ordinary task progress,
secrets, or tracked bugs; continue working unless the issue blocks progress.
