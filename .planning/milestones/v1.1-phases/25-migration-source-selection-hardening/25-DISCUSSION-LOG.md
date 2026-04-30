---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 25-2026-04-28T20-22-00
generated_at: 2026-04-28T20:22:00.000Z
---

# Phase 25 Discussion Log

## Prompt

Close the migration source-selection audit gap without weakening the existing
dry-run-only and read-only migration safety posture.

## Decisions Made

1. **Augment migration-time detection roots instead of inventing a second
   scanner.**
   The Phase 21 decision was to reuse the detector. The smallest honest fix is
   to thread the explicit `--source-datadir` into the detector roots only for
   `migrate plan`.

2. **Keep the fix migration-scoped.**
   Other operator commands should keep their existing detection behavior so the
   phase remains a narrow gap closure, not a cross-command detection rewrite.

3. **Require real source evidence before auto-selecting the explicit path.**
   If the explicit path exists but does not expose `bitcoin.conf`, `.cookie`, or
   wallet evidence, the planner should still fall back to manual review instead
   of pretending certainty.

4. **Add a true out-of-roots regression test.**
   The previous binary test covered an explicit source path already located at
   `HOME/.bitcoin`, so it could not catch `INT-W03`. The new regression must use
   a custom datadir outside the default detector roots.

## Alternatives Considered

- **Bypass detection and inspect the explicit path directly inside the planner.**
  Rejected because it would create a second source scanner and drift away from
  the read-only detection contract Phase 21 established.

- **Accept any existing directory as a supported explicit source path.**
  Rejected because a bare directory does not justify a concrete migration plan
  and would weaken the manual-review safety boundary.

- **Broaden detection roots for every command whenever `--source-datadir` is
  present in CLI args.**
  Rejected because `--source-datadir` is migration-specific and should not leak
  into unrelated operator flows.

## Resulting Plan Shape

- Plan 01: thread explicit source datadirs into migration-time detection
- Plan 02: keep explicit selection conservative and add regression coverage
- Plan 03: verification and bookkeeping closeout
