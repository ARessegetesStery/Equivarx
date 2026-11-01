# AGENTS.md

This file provides project-wide guidance to DeepSeek Harness sessions working in this repository. It is the governing instruction file for DeepSeek only. `CLAUDE.md` is Claude Code's separate instruction file; the harness loads it, but it is not a rule source here. Its Claude-only working rules -- including its concurrency limit, model guidance, and recurring-scheduler guidance -- do not apply to DeepSeek, and this file is the complete DeepSeek guidance. Where the two differ, this file wins.

## Repository layout

Equivarx is a Rust game engine: a Cargo workspace whose root package `equivarx` re-exports `eqx_prelude::prelude::*` -- `App`, `Module` and `WindowDisplay`. Rendering goes through wgpu and windowing through winit.

- `AGENTS.md` -- this file, the DeepSeek Harness project instructions.
- `CLAUDE.md` -- Claude Code's separate project instructions.
- `crates/` -- the workspace's library crates. `eqx_app` holds `App` and the `Module` trait through which engine modules are loaded and started. `eqx_utils` holds the keyboard state (`Input`) and the macros that locate shaders and assets (`shader_src_from!`, `asset_file_path!`). `eqx_window` holds the window, wgpu setup and render loop (`WindowDisplay`) and the basic rendering facilities (camera and its controller, model and instance primitives, textures, and model and texture loading). `eqx_render` is a placeholder crate (template `add` and its test), reserved for deeper rendering algorithms; nothing depends on it. `eqx_prelude` gathers the re-exports behind the root package.
- `src/lib.rs` -- the root package `equivarx`, which re-exports `eqx_prelude`.
- `examples/hello_cube/` -- the example binary, which runs `WindowDisplay` through `App`. Its build script copies `assets/` to `$OUT_DIR/assets`, which `eqx_window` resolves from the `OUT_DIR` environment variable at run time.
- `shaders/` -- WGSL shaders, read from disk by path at run time.
- `assets/` -- textures (`assets/texture/`), models (`assets/cube/`) and images the repository itself uses (`assets/_repo/`).
- `docs/` -- `memory/` holds the project memory (below). `lodestar/` is where the `lodestar` skills write dated records -- point-in-time specs, plans, handoffs, reports and wrap-ups (none yet): cite them by dated filename and never rewrite them to match later work. Run ledgers are not tracked: they go under `Temp/lodestar/` (none yet), and a run's tracked record is its report or wrap-up under `docs/lodestar/` plus its memory entry.
- `Temp/` -- gitignored scratch.
- `.github/workflows/build.yml` -- CI: `cargo check` on Windows, Linux and macOS (root package only), plus the format, lint and doc checks below on Linux.

## Verification

Run everything from the repo root. This section names the entry points, not what any of them currently returns.

- Build: `cargo check --workspace`
- Format: `cargo fmt --all --check`
- Lints: `cargo clippy --workspace --all-targets -- -D warnings`
- Docs: `cargo doc --no-deps --workspace --document-private-items`, with `RUSTDOCFLAGS` set to `-D warnings`
- Tests: `cargo test --workspace`
- Example: `cargo run -p hello_cube`, which opens a window when `OUT_DIR` is set

## Coding conventions

- Formatting is rustfmt's, at its defaults. Lints are the compiler's and clippy's defaults plus those in the root `Cargo.toml`'s `[workspace.lints]`, which every package inherits.
- Acronyms follow Rust's convention and are word-cased inside type names (`IpcModel`, `BvhNode`).
- Treat compiler and clippy diagnostics as design feedback: fix the cause rather than quieting the diagnostic. Do not reach for `#[allow]`, `.clone()`, `Arc<Mutex<_>>`, `Rc<RefCell<_>>`, `unsafe` or a lifetime parameter merely to make an error go away. Where one of them is the right design, say why: `#[expect(..., reason = "...")]` for a lint, `// SAFETY:` for `unsafe`, a comment otherwise. When the borrow checker resists a design, raise it as a design question instead of working around it.

## Project memory

Durable, cross-session project knowledge lives in `docs/memory/` (git-tracked, so it travels with the repo across machines and fresh checkouts). The index has one line per memory; each detail file holds the finding.

Read `docs/memory/MEMORY.md` at the start of every session; read individual `docs/memory/<slug>.md` files on demand. The index is not auto-loaded; the harness does not expand path imports, so make that read your first action. Its "Imported by `CLAUDE.md`" note describes Claude's setup and does not apply here.

To find a memory: scan the index (read at session start), then open the specific `docs/memory/<slug>.md` for detail. Once the engine is divided into subsystems (for example rendering, physics, arch), each gets its own subfolder under `docs/memory/`; until then the memory is flat.

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

## Project-wide working rules

- In conversation, lead with the verdict or the load-bearing facts, with a brief explanation of it. Only make necessary explanations, and explain succinctly. Over-explaining, or providing too much supporting detail I did not request costs me more to read than it gains me.
- Do not touch git. I will do the committing myself, after a review pass.
- Never spawn nested agents (if you are a subagent, you are NOT allowed to spawn other subagents).
- In-session builds ARE permitted. Where a toolchain is available, prefer to build and run to verify a change rather than leaving it unbuilt. This does not relax the git rule above.
- When writing comments or printing messages, avoid using unicode character as they will cause trouble in IDE display.
- When writing markdown files, do not use newline characters within paragraphs. There is no per-line character limit.
- Whenever you add, remove, rename, or change something a doc describes or indexes, update that doc in the same change. ADDING a new file counts, not only editing an existing one. The doc that indexes code is this file (`AGENTS.md`; structure and conventions). If a substantial code or behavior change is completed, also update the project memory.
- Keep this file as rules and current structure only: state what the code does NOW, never how it got there. No rationale, no milestone narration, no "formerly / was removed / now always", and no dates in prose. Citing a dated filename under `docs/lodestar/` is fine and is the preferred pointer. Reasons belong in the code comment or docstring they govern; history belongs in the project memory and those records.
- When explaining anything in conversation, be conceptual and intuitive, assuming I have no document open: do not lean on a pointer I cannot resolve mid-discussion (a register ID, a report or spec section, a task number), though naming a code symbol or config key is fine with a word on what it is, and a citation appended AFTER the explanation is welcome. When reporting numbers from a run, summarize qualitatively first -- saying plainly when a result is null, partial or negative -- then give the concrete numbers with an intuitive explanation of what each represents. This governs how work is COMMUNICATED, not how it is grounded: read the record as rigorously as ever, just do not make me read it to follow you. Conversations only -- in reports, write rigorously and ground every conclusion with concrete references and numbers.
- Scratch material that project documentation cites -- a script, a data file or a text record (note, report, review) -- must not stay in gitignored `Temp/`: move it, with the scripts it imports, to the tracked `test_scratches/` folder at the path it had under `Temp/`, and cite it by that tracked path. Run outputs, builds, logs and ledgers stay machine-local, and an item over 20 MB is not moved but reported to the owner. A living document that cites an item left in `Temp/` says it is machine-local. Once the engine is divided into subsystems, each gets its own subfolder under `test_scratches/`, named as under `docs/memory/`; until then the folder is flat.
- For testing/verification scripts, write in toplevel `Temp/` folder instead of user-wide Local/Temp/, so that all modifications are confined to this repository.
- A periodic check of running jobs (such as the every-45-minute check) runs on a chained background timer -- one background `Start-Sleep` job whose completion notification triggers the check and is re-armed then; do not use a recurring scheduler such as `schedule_create`. Before arming or re-arming, confirm with `job_list` that no timer is pending; after every compaction, do the same and start one only if none is running.
- Prefer INLINE implementation when the main agent already holds the context and the task is mechanical and bounded in output; reach for a subagent when the work is output-heavy (headless runs, sweeps, suites), context-light, independent or parallelizable -- otherwise delegation just re-establishes context for no gain. When dispatching "cold" or "adversarial" review, always dispatch an agent to avoid bias from existing context. When a user instruction conflicts with this, state the tradeoff in one line and ask for confirmation instead of silently complying or overriding.
- `CLAUDE.md`, `.claude/`, and other Claude-specific files belong to Claude Code; do not edit them from a DeepSeek session. When you are unsure about Claude's behavior or need a Claude-side check, ask the user to verify it in Claude; never invoke `claude` or any Claude Code process from a DeepSeek session.