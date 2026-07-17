# agent-papercuts

A small (somewhat vibecoded) CLI for recording "papercuts" the pieces of friction agents encounter while
working: misleading errors, missing tools, broken links, and undocumented setup
steps.

It writes Markdown to `PAPERCUTS.md`, so the log can be reviewed and fixed  like any other repository artifact.

Use as so:

```sh
papercut --author aditya -m gpt-5.6-terra "The local database reset command requires Docker, but the setup guide does not mention it."
papercut --author codex -m claude-sonnet-5 --file docs/PAPERCUTS.md "The API reference link redirects to a removed page."
```
