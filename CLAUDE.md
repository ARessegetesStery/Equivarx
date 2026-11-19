# CLAUDE.md

Claude Code-specific guidance for this repository. Shared repository layout, verification commands, coding conventions, memory format, and universal working rules live in `AGENTS.md`, imported below alongside the project memory index:

@AGENTS.md
@docs/memory/MEMORY.md

## Claude Code rules

- `docs/memory/` is the canonical, portable project memory. The harness system prompt designates a machine-local auto-memory directory (`~/.claude/projects/<project>/memory/`); that directive is overridden here: never write a memory there, and treat anything found there as stale.
- Name the model explicitly on any subagent dispatch, tiered to the task. Use only Sonnet or Opus subagents: do not dispatch Fable or Haiku subagents.
- After editing `docs/architecture.md`, build the page with `sed '/data-embed=/r docs/architecture.md' docs/architecture.html > Temp/architecture-page.html`, read the published artifact (the Artifact tool requires it), and publish `Temp/architecture-page.html` to the link at the top of `docs/architecture.md`.
- For periodic heartbeat checks, use a chained background `sleep` whose completion notification triggers the check (re-armed at every check) -- never `CronCreate`, which can stay listed yet stop firing.
