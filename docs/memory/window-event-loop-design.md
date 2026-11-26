---
name: window-event-loop-design
description: Ongoing design (2026-11-26): unblocking App::run from winit's event loop -- Option A (pump_app_events) vs Option B (pluggable App runner); owner leans toward Option B, not yet finalized
metadata:
  type: project
---

As of 2026-11-26, `WindowDisplay::init` still blocks inside `winit` 0.30's `EventLoop::run_app`, so `Module::update` never runs and modules loaded after `WindowDisplay` never initialize. Preliminary work is in place (`winit` 0.30 with `ApplicationHandler`, `Arc<Window>` + `wgpu::Surface<'static>`, and `App` owning an engine-native `IoRuntime` wrapping Tokio so clients use a plain `fn main()` without `#[tokio::main]` or a custom proc-macro). Two designs are under consideration to unblock `App::run`:

- **Option A (`App` owns the outer loop; `WindowDisplay::update` pumps `winit` once):** `App::run` owns `while running { tick(); }`, and `WindowDisplay::update` drains OS events via `EventLoopExtPumpEvents::pump_app_events(Some(Duration::ZERO), ...)`. Keeps `WindowDisplay` shaped like an ordinary `Module`, but relies on `winit`'s secondary desktop-only escape hatch (which stalls during Win32 modal drag/resize loops).
- **Option B (pluggable runner on `App`; `winit`'s `run_app` drives `App::tick`):** `App` holds a pluggable runner (defaulting to a plain headless loop for Stage 0's headless bench), and `eqx_window` provides a `run_app` runner whose `ApplicationHandler` calls `app.init_modules()` on resume and `app.tick()` each frame. Uses `winit`'s primary code path and keeps headless runs completely `winit`-free, while leaving two open design details to settle with the owner: how the window module installs the runner (passing `&mut App` into module setup vs explicit `App::with_runner`), and how the runner shares input/window state with modules (such as a lightweight typed `Resources` map on `App` without introducing an ECS).

The owner currently leans toward **Option B**, since the window host is structurally different from ordinary modules while keeping `App` modular so users load only the modules they need, but has **not finalized the decision** yet pending a careful review of both options.

**Why:** Unblocking `App::run` is required by Stage 0 of `docs/roadmap/rendering.md` (which needs both live windowed rendering and a headless offscreen bench without `winit`). Recording both options preserves the exact decision context across sessions without committing to an unfinalized choice.

**How to apply:** In the next session, review Option A vs Option B (and, if Option B is chosen, settle runner registration and inter-module state sharing) with the owner before implementing. **Once the restructure is settled and completed, update this memory entry in place** to record the final decision and rejected alternatives (or replace it with the settled architecture memory). Related: [[rendering-first-roadmap]], [[single-consumer-code-stays-put]].
