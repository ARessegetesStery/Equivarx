---
name: docs-are-reference-not-changelog
description: Carried over from the Glade project (recorded there 2026-08-05): documentation of code is reference, revised in place and never appended to
metadata:
  type: feedback
---

Documentation that describes code is reference: one short entry per unit, stating what it is now. Revise the affected entry in place and never append a paragraph. An entry that narrates a superseded state, or names a milestone or task number, is a defect.

**Why:** appending is the cheap edit and reads as safe, but it turns a doc into a changelog whose newest and oldest sentences contradict each other, with no signal about which is current. In the Glade project, two docs grown this way reached 17.6k and 25.6k words and still described code deleted long before.

**How to apply:** reasoning belongs in the code's own comment or docstring, and history in `docs/memory/` and `docs/lodestar/`.
