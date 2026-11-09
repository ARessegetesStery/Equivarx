---
name: architecture-doc-design-altitude
description: Decided 2026-11-06: docs/architecture.md records only design-level facts, is the single source of its artifact, and changes at design decisions and stage closes
metadata:
  type: feedback
---

`docs/architecture.md` records only what a design decision changes, is the single source that its Claude artifact renders, and is revised at design decisions and at each roadmap stage's Close step rather than on routine changes.

**Why:** the owner wants an architecture page to follow the design without reading code, but worried that with frequent edits it would go stale and mislead development. The first version (2026-11-05) kept a hand-mirrored HTML copy beside the markdown and recorded code-level detail (bind group slots, dependency versions, scene contents, known math bugs). Routine changes would have falsified that detail, agents read it from the code anyway, and two hand-kept copies drift.

**How to apply:** before adding a sentence, ask whether a routine change could make it false; if so, put it in the code's own docs (a crate's `//!` comment) instead. Never keep a second hand-written copy of the content: the publish step embeds the markdown into the page template, which renders it. Do not switch back to the page fetching the markdown as a published supporting file: on 2026-11-06 that fetch failed in the artifact viewer although the file was stored correctly. If the doc grows large, consider a test that checks its crate graph against `cargo metadata`. Related: [[docs-are-reference-not-changelog]], [[engine-direction]], [[rendering-first-roadmap]].
