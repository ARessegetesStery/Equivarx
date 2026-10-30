# CLAUDE.md

This file provides project-wide guidance to Claude Code (claude.ai/code).

## Repository layout

Equivarx is a Rust game engine: a Cargo workspace whose root package `equivarx` re-exports the public API of its member crates. Rendering goes through wgpu and windowing through winit.

- `crates/` -- the workspace's library crates. `eqx_app` holds `App` and the `Module` trait through which engine modules are loaded and started. `eqx_core` holds the macros that locate shaders and assets (`shader_src_from!`, `asset_file_path!`). `eqx_window` holds the window and wgpu setup (`WindowDisplay`) and the basic rendering facilities (camera, primitives, texture). `eqx_render` is reserved for deeper rendering algorithms and holds no engine code. `eqx_prelude` gathers the re-exports behind the root package.
- `src/lib.rs` -- the root package `equivarx`, which re-exports `eqx_prelude`.
- `examples/hello_cube/` -- the example binary, which runs `WindowDisplay` through `App`.
- `shaders/` -- WGSL shaders, read from disk by path at run time.
- `assets/` -- textures (`assets/texture/`) and images the repository itself uses (`assets/_repo/`).
- `docs/` -- `memory/` holds the project memory (below). `lodestar/` holds the point-in-time records the `lodestar` skills write at their default paths -- dated specs, plans, handoffs, reports and wrap-ups: cite them by dated filename and never rewrite them to match later work. Run ledgers are not tracked: they live under `Temp/lodestar/`, and a run's tracked record is its report or wrap-up under `docs/lodestar/` plus its memory entry.
- `Temp/` -- gitignored scratch.
- `.github/workflows/build.yml` -- CI: `cargo check` on Windows, Linux and macOS, plus the format, lint and doc checks below on Linux.

## Verification

Run everything from the repo root. This section names the entry points, not what any of them currently returns.

- Build: `cargo check --workspace`
- Format: `cargo fmt --all --check`
- Lints: `cargo clippy --workspace --all-targets -- -D warnings`
- Docs: `cargo doc --no-deps --workspace --document-private-items`, with `RUSTDOCFLAGS` set to `-D warnings`
- Tests: `cargo test --workspace`
- Example: `cargo run -p hello_cube`, which opens a window

## Coding conventions

- Formatting is rustfmt's and lints are clippy's, at their defaults; the repository configures neither.
- Acronyms follow Rust's convention and are word-cased inside type names (`IpcModel`, `BvhNode`).

## Project memory

Durable, cross-session project knowledge lives in `docs/memory/` (git-tracked, so it travels with the repo across machines and fresh checkouts). `docs/memory/MEMORY.md` is the index, one line per memory, and is imported on the next line so it loads every session:

@docs/memory/MEMORY.md

To find a memory: scan the index (always loaded), then open the specific `docs/memory/<slug>.md` for detail. Only the index auto-loads; detail files are read on demand. Once the engine is divided into subsystems (for example rendering, physics, arch), each gets its own subfolder under `docs/memory/`; until then the memory is flat.

To write a new memory: create `docs/memory/<short-kebab-slug>.md` holding a single fact with the frontmatter below, then add a matching one-line pointer under `## Memories` in the index (a list item that links the file by its bare filename and ends with a short hook).

```markdown
---
name: <short-kebab-slug>
description: <one-line summary, used to judge relevance during recall>
metadata:
  type: user | feedback | project | reference
---

<The fact. For feedback and project entries, add **Why:** and **How to apply:** lines. Link related memories with [[their-slug]].>
```

Memory types: `user` (who the user is: role, expertise, preferences), `feedback` (how I should work, with the reason why), `project` (ongoing work, goals, or constraints not derivable from code or git history), `reference` (pointers to external resources: URLs, dashboards, tickets). Convert relative dates to absolute. Before adding, check for an existing file that already covers the fact and update it instead of duplicating; delete memories that turn out to be wrong. Do not record what the repo already captures (code structure, past fixes, git history, this file) or what only matters to a single conversation.

`docs/memory/` is the canonical, portable project memory. The harness system prompt designates a machine-local auto-memory directory (`~/.claude/projects/<project>/memory/`); that directive is overridden here: never write a memory there, and treat anything found there as stale.

## Project-wide working rules

- In conversation, lead with the verdict or the load-bearing facts, with a brief explanation of it. Only make necessary explanations, and explain succinctly. Over-explaining, or providing too much supporting detail I did not request costs me more to read than it gains me.
- Do not touch git. I will do the committing myself, after a review pass.
- Never dispatch more than 3 subagents at the same time. Never spawn nested agents (if you are a subagent, you are NOT allowed to spawn other subagents).
- In-session builds ARE permitted. Where a toolchain is available, prefer to build and run to verify a change rather than leaving it unbuilt. This does not relax the git rule above.
- When writing comments or printing messages, avoid using unicode character as they will cause trouble in IDE display.
- When writing markdown files, do not use newline characters within paragraphs. There is no per-line character limit.
- Whenever you add, remove, rename, or change something a doc describes or indexes, update that doc in the same change. ADDING a new file counts, not only editing an existing one. The doc that indexes code is this file (structure and conventions). If a substantial task is completed, also update the project memory.
- Keep this file as rules and current structure only: state what the code does NOW, never how it got there. No rationale, no milestone narration, no "formerly / was removed / now always", and no dates in prose. Citing a dated filename under `docs/lodestar/` is fine and is the preferred pointer. Reasons belong in the code comment or docstring they govern; history belongs in the project memory and those records.
- When explaining anything in conversation, be conceptual and intuitive, assuming I have no document open: do not lean on a pointer I cannot resolve mid-discussion (a register ID, a report or spec section, a task number), though naming a code symbol or config key is fine with a word on what it is, and a citation appended AFTER the explanation is welcome. When reporting numbers from a run, summarize qualitatively first -- saying plainly when a result is null, partial or negative -- then give the concrete numbers with an intuitive explanation of what each represents. This governs how work is COMMUNICATED, not how it is grounded: read the record as rigorously as ever, just do not make me read it to follow you. Conversations only -- in reports, write rigorously and ground every conclusion with concrete references and numbers.
- Scratch material that project documentation cites -- a script, a data file or a text record (note, report, review) -- must not stay in gitignored `Temp/`: move it, with the scripts it imports, to the tracked `test_scratches/` folder at the path it had under `Temp/`, and cite it by that tracked path. Run outputs, builds, logs and ledgers stay machine-local, and an item over 20 MB is not moved but reported to the owner. A living document that cites an item left in `Temp/` says it is machine-local. Once the engine is divided into subsystems, each gets its own subfolder under `test_scratches/`, named as under `docs/memory/`; until then the folder is flat.
- For testing/verification scripts, write in toplevel `Temp/` folder instead of user-wide Local/Temp/, so that all modifications are confined to this repository.
- A periodic check of running jobs (such as the every-45-minute check) runs on a chained background timer -- a background `sleep` whose completion notification triggers the check, re-armed at every check -- never on `CronCreate` jobs, which can stay listed yet stop firing. After every compaction, confirm a timer is running and start one if not.
- Prefer INLINE implementation when the main agent already holds the context and the task is mechanical and bounded in output; reach for a subagent when the work is output-heavy (headless runs, sweeps, suites), context-light, independent or parallelizable -- otherwise delegation just re-establishes context for no gain. When dispatching "cold" or "adversarial" review, always dispatch an agent to avoid bias from existing context. Name the model explicitly on any dispatch, tiered to the task. Use only Sonnet or Opus subagents: do not dispatch Fable or Haiku subagents. When a user instruction conflicts with this, state the tradeoff in one line and ask for confirmation instead of silently complying or overriding.
