# Phase 34: Migration Detection Ownership Model Cleanup - Discussion Log

> **Audit trail only.** Do not use as input to planning or execution agents.
> Decisions are captured in `34-CONTEXT.md`; this log preserves the alternatives
> considered during the yolo discuss pass.

**Date:** 2026-04-30
**Phase:** 34-migration-detection-ownership-model-cleanup
**Mode:** Yolo
**Areas discussed:** detection ownership model, migration consumer contract,
status fallback truth, verification scope

---

## Detection Ownership Model

| Option | Description | Selected |
|--------|-------------|----------|
| Introduce a scan-level detection aggregate | Represent installations and service definitions separately so service ownership is explicit instead of implied. | ✓ |
| Keep cloning service candidates into each installation | Preserve the current shape and rely on every consumer to remember it is shared evidence. | |
| Add a warning-only comment to the existing field | Leave the misleading data shape in place and document around it. | |

**User's choice:** Auto-selected the explicit scan-level model: service
definitions are shared scan evidence, not installation-owned evidence.
**Notes:** This is the narrowest fix that removes the structural footgun without
inventing new migration behavior.

---

## Migration Consumer Contract

| Option | Description | Selected |
|--------|-------------|----------|
| Thread scan-level service evidence into the existing association helper | Keep Phase 31's truthful selected-source behavior while removing the misleading shared field. | ✓ |
| Move service ownership parsing into the detector immediately | Turn this cleanup into a broader eager-association redesign. | |
| Drop service review actions from migration planning | Avoid the ownership issue by shrinking the migration surface. | |

**User's choice:** Auto-selected explicit threading into the existing
association helper.
**Notes:** Phase 31 already proved the planner contract; Phase 34 should clean up
the shared model underneath it instead of reopening the operator-facing flow.

---

## Non-Migration Consumers

| Option | Description | Selected |
|--------|-------------|----------|
| Keep status on scan-level service evidence; keep onboarding and wallet installation-local | Preserve truthful fallback status while removing unnecessary ownership coupling elsewhere. | ✓ |
| Push scan-level service evidence into every consumer | Replace one leaky shared field with a broader one. | |
| Remove service awareness from status fallback entirely | Tighten the model by regressing an existing truthful fallback signal. | |

**User's choice:** Auto-selected the narrow consumer split: status still sees
scan-level service presence, while onboarding and wallet remain on
installation-local evidence.
**Notes:** This keeps the Phase 34 write set focused on real consumers instead of
creating a second broad service-evidence API.

---

## Verification Scope

| Option | Description | Selected |
|--------|-------------|----------|
| Add focused detector, status, migration, and operator-binary regressions plus repo verify | Prove the data-shape repair at the shared type boundary and the affected operator flows. | ✓ |
| Verify only migration planner tests | Risk hidden regressions in status or detector consumers. | |
| Rely on manual review because the behavior is "internal only" | Skip proving the shared ownership model at the exact place it previously leaked. | |

**User's choice:** Auto-selected focused shared-surface tests plus the existing
repo-native verification contract.
**Notes:** The cleanup changes a shared type, so it needs more than a single
planner test to stay trustworthy.

---

## Claude's Discretion

- The exact aggregate type name is flexible as long as it clearly communicates
  scan-level ownership.
- If per-install `source_paths` become clearer without cloned service-definition
  entries, prefer the clearer installation-local shape.

## Deferred Ideas

- Eager service ownership association directly in the base detector.
- Broader uncertainty-model cleanup for service-manager ownership semantics.
- Migration apply mode or automatic service cutover.
