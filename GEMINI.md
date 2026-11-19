# GEMINI.md

Antigravity / Gemini-specific guidance for this repository. Shared repository layout, verification commands, coding conventions, memory format, and universal working rules live in `AGENTS.md`, which Antigravity loads alongside this file.

## Antigravity / Gemini rules

- Read `docs/memory/MEMORY.md` at the start of every session (Antigravity does not auto-import it).
- Default to **Swarm** (`swarm`) for delegated work (implementers, reviewers, research, and long checks) so thinking effort can be tiered per role.
- Keep at most **3 active (`working`) workers** concurrently. Check `swarm{action:"status"}` before adding or waking a worker, and `kill` completed workers once their deliverable is landed so the roster stays lean.
- Stay within the coordinator's model family when assigning Swarm workers, varying only the thinking effort (for example, when the session runs Gemini 4 Neon, pick only from `gemini-4-neon-low`, `gemini-4-neon-medium`, `gemini-4-neon-high`, and `gemini-4-neon-xhigh`), unless the user explicitly asks for another model family.
- For cold or adversarial reviews in Swarm mode, add a fresh reviewer worker after the build finishes and instruct it in its brief not to read earlier board history (`read_board{since:0}`) and not to edit any files.
- Every Swarm worker brief must explicitly preserve the repo boundaries: no state-mutating git commands (never commit, merge, or cherry-pick even in `share`/`branch` workspaces), no spawning helper workers, and read-only for reviewers.
- For periodic heartbeat checks, use the `schedule` tool's one-shot `DurationSeconds` timer re-armed at each firing -- never a recurring cron expression or a shell `sleep`. Check `manage_task` (`list`) before arming or re-arming and after every compaction.
- Antigravity cannot publish Claude artifacts. After editing `docs/architecture.md`, tell the owner that the published architecture artifact needs republishing from a Claude Code session.
