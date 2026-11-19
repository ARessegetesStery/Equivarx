---
name: content-as-code
description: Decided 2026-11-11: game content is authored as code by agents, replacing the 2026-11-01 choice of Blender; shader-only and generated-frame rendering were weighed and rejected
metadata:
  type: project
---

Decided 2026-11-11: the world Equivarx renders is authored as code kept in the repository, with meshes, collision shapes and mist fields derived from it. This replaces the 2026-11-01 choice of Blender for content. The renderer stays physically grounded and model-based.

**Why:** the owner has little art experience, cannot edit a model by hand in any form, and steers content only through qualitative remarks. An agent can answer such a remark with a small diff to a program, but can only replace a model file whole. Two alternatives were weighed in the same discussion and rejected: rendering the world entirely in shaders, because such a world exists only for the camera (collision and the deterministic CPU simulation cannot query it) and is hard to revise; and models that generate frames directly, because they hold no exact state and are not deterministic. The physical model was kept because it is what lets agents check a result against a path-traced reference without the owner's eye.

**How to apply:** do not propose Blender, another modelling tool or generated mesh files for game content. Follow the "Content is code" rule in `docs/roadmap/rendering.md`; how content code describes shape, and whether it is Rust or data, are owner decisions in that roadmap's Stage 1. Hand-sculpted organic detail is outside this approach, so raise it with the owner if the game comes to need it. Related: [[engine-direction]], [[rendering-first-roadmap]].
