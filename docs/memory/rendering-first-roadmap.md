---
name: rendering-first-roadmap
description: Decided 2026-11-01: rendering is built before engine systems, planned in docs/roadmap/rendering.md; engine systems get a separate roadmap, not yet written
metadata:
  type: project
---

Decided 2026-11-01: Equivarx builds its renderer first, following `docs/roadmap/rendering.md`. Game engine systems (job system, world model, gameplay, moving Occluscence in) go to a separate engine roadmap, which takes its place after the living-mist stage and has not been written. Meanwhile the owner does no mist rendering in Godot and experiments there with time mechanisms.

**Why:** rendering holds the hardest planned features and most of what the owner wants to learn, while the game's CPU load is still tiny. The engine work is large and also needs the owner to learn more Rust, so it gets its own plan.

**How to apply:** start rendering work from the roadmap's first unfinished stage, and keep engine systems out of it beyond the minimum rendering needs. The owner may use another LLM for implementation and teaching, so record decisions in the roadmap rather than relying on a session's context. Related: [[engine-direction]], [[single-consumer-code-stays-put]].
