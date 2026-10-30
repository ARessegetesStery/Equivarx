---
name: engine-direction
description: Decided 2026-10-30: restart Equivarx on wgpu, with the owner owning design and predictions while agents write the code
metadata:
  type: project
---

Decided 2026-10-30: restart Equivarx on wgpu. The owner owns the design and the predictions, and agents write the code.

**Why:** in rendering the owner's interest is the algorithms, not GPU bookkeeping, which wgpu handles and raw Vulkan does not; raw Vulkan through `ash` was considered and rejected on that ground. Knowing the algorithm is what the owner needs. Hand-writing costs time on typos and similar slips that teach nothing, and hand-writing speed is why the earlier attempt at this project was abandoned.

**How to apply:** do not propose a lower-level graphics API unless an algorithm needs something wgpu does not expose. Do not expect the owner to hand-write code; bring design decisions to them instead of settling those alone.
