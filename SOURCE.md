# Source

`agent-papercut` is inspired by Steve Ruiz's papercuts thread on X.

## Source posts

1. [Introducing the papercuts CLI](https://x.com/steveruizok/status/2075303919664734295)
2. [Using a model to fix the log](https://x.com/steveruizok/status/2075304096328798401)
3. [The `AGENTS.md` instruction](https://x.com/steveruizok/status/2075329969169850651)

## Captures

- [Complete recovered thread capture](docs/source/steve-ruiz-papercuts-thread.png)
- [Post 1 capture](docs/source/steve-ruiz-papercuts-post-1.png)
- [Post 2 capture](docs/source/steve-ruiz-papercuts-post-2.png)
- [Post 3 capture](docs/source/steve-ruiz-papercuts-post-3.png)

The complete capture is a single rendering of the three linked posts and the
text in the third post's attached image. It is included because X's guest view
did not show the reply chain together. The individual captures are direct
browser captures of the linked posts.

## Full text transcript

### Post 1 — 2026-07-09 7:39 PM

> I added a tiny "papercuts" cli tool that agents can use to complain about
> bullshit they encountered during work, like dead-end tool calls, broken links,
> or other frustrations. The models would usually just push through without
> mentioning any problems

### Post 2 — 2026-07-09 7:40 PM

> Every so often I ask a model to just make all the fixes to the PAPERCUTS.md
> file, which are usually tiny and easily fixed

### Post 3 — 2026-07-09 9:23 PM

> In my AGENTS.md

The post attaches an image containing this instruction:

> # Log papercuts
>
> Important! When you hit a small friction while working—a tool call that missed
> and had to be retried, a confusing or undocumented setup step, a flaky
> command, a stale cache, a misleading error, a non-obvious gotcha—log it to
> `PAPERCUTS.md` via `yarn papercut -m <model> "message"`. One or two sentences:
> what you were doing → what got in the way (a guess at the cause/fix is a
> bonus). Do this proactively, in the moment, even though none of these are
> blocking—logged together they show where the repo needs sanding down. This is
> distinct from `LOG.md` (what you accomplished) and from Linear issues (real
> bugs / tracked work).
>
> To mine a whole session for papercuts at once, `yarn papercut:review` feeds the
> session transcript to a cheap model (Gemini Flash, via `GOOGLE_API_KEY` in
> `.env`) and appends what it finds. This is user-triggered via the `/papercut`
> slash command—don't run the review yourself unprompted.
