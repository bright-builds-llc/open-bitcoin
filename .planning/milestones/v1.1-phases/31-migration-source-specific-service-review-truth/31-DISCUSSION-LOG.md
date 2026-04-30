# Phase 31: Migration Source-Specific Service Review Truth - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution
> agents. Decisions are captured in `31-CONTEXT.md` - this log preserves the
> alternatives considered during the yolo discuss pass.

**Date:** 2026-04-29T16:42:33.523Z
**Phase:** 31-migration-source-specific-service-review-truth
**Mode:** Yolo
**Areas discussed:** Service association source, Fallback semantics, Verification
and docs

---

## Service association source

| Option | Description | Selected |
|--------|-------------|----------|
| Keep cloning the scan-wide service list into every selected source review | Preserve the current behavior and continue showing whatever service definitions the detector found anywhere in the scan roots. | |
| Filter migration service review by source-specific service evidence | Keep the current detector surface, but only render service review paths when a service definition points at the selected source datadir or config. | ✓ |
| Replace the detector with a migration-only service scanner | Invent a second migration-time service discovery path separate from the current read-only detector. | |

**User's choice:** Auto-selected the narrow source-specific filtering path.
**Notes:** This closes `INT-v1.1-03` without reopening the broader detection
surface or the existing custom-path selection work from Phase 25.

---

## Fallback semantics

| Option | Description | Selected |
|--------|-------------|----------|
| Show every detected service definition anyway | Continue rendering unrelated or ambiguous service review paths. | |
| Fall back to an explicit manual-review step when service ownership is ambiguous | Keep the selected source install, but avoid showing service paths unless the service definition can be tied to that install. | ✓ |
| Treat ambiguous service evidence as a hard planner failure | Abort the whole migration plan instead of keeping the rest of the dry-run output. | |

**User's choice:** Auto-selected the manual-review fallback.
**Notes:** The migration planner already uses explicit manual steps for uncertain
evidence, so this keeps the existing dry-run safety posture and avoids false
precision.

---

## Verification and docs

| Option | Description | Selected |
|--------|-------------|----------|
| Code-only fix | Repair the planner behavior without adding new regression coverage or checking operator docs. | |
| Focused planner plus operator-binary regressions, and doc alignment if wording changes | Prove both source-specific inclusion and ambiguous-service fallback while keeping the migration guide truthful. | ✓ |
| Broad migration harness expansion | Build a larger integration harness around multi-install service cutover review. | |

**User's choice:** Auto-selected focused regressions and minimal doc alignment.
**Notes:** This matches the narrow blocker scope and the repo's preference for
deterministic, hermetic verification over new heavy harness work.

---

## Claude's Discretion

- Exact helper names and whether the service-association logic lives in
  `planning.rs` or a nearby migration helper module.
- Whether the selected-source match uses datadir evidence alone or datadir plus
  config-path evidence, as long as the behavior stays conservative and truthful.

## Deferred Ideas

- Reworking `DetectedInstallation` so service ownership is modeled directly at
  detection time for all downstream consumers.
- Broad migration apply-mode or automated service cutover behavior.
