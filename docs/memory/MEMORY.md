# Project memory index

Shared across all coding-agent sessions (auto-imported by `CLAUDE.md`, read at session start in other harnesses). One line per memory, each linking its detail file and ending with a hook that says what the memory is about; the finding itself is in the detail file.

## Memories

- [Engine direction](engine-direction.md) -- restart Equivarx on wgpu; the owner owns design and predictions while agents write the code (2026-10-30)
- [Single-consumer code stays put](single-consumer-code-stays-put.md) -- OWNER RULE: code serving one consumer stays with it until a second, exactly matching use case appears
- [Docs are reference, not changelog](docs-are-reference-not-changelog.md) -- documentation of code is revised in place, never appended to
- [Rendering first](rendering-first-roadmap.md) -- why the renderer comes before engine systems, and where each is planned (2026-11-01)
- [Architecture doc at design altitude](architecture-doc-design-altitude.md) -- docs/architecture.md holds only design-level facts, is its artifact's single source, and changes at design decisions (2026-11-06)
- [Content as code](content-as-code.md) -- game content is authored as code by agents, not in Blender; shader-only and generated-frame rendering rejected (2026-11-11)
- [Window event loop design](window-event-loop-design.md) -- ongoing Option A vs Option B decision for unblocking App::run from winit; owner leans toward Option B, update once settled (2026-11-26)
