---
name: single-consumer-code-stays-put
description: OWNER RULE carried over from the Glade project (2026-10-09): code that serves one consumer stays with that consumer, and moves to shared code only when a second, exactly matching use case appears
metadata:
  type: feedback
---

Code written for one consumer stays in that consumer's crate or module, even when it sits at the same layer as shared code or reads the same data structures. Only when a second, exactly matching use case exists is it split into a general utility plus an interface left with the consumer.

**Why:** shared code promises generality, and moving single-consumer code there before a second consumer exists fixes an interface around one caller's needs.

**How to apply:** when restructuring, leave such code in place, and do not propose moving it on the grounds of layer or shared data structures alone. Propose the split only when a second caller needs exactly the same operation.
