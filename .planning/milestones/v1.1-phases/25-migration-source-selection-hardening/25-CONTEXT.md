---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 25-2026-04-28T20-22-00
generated_at: 2026-04-28T20:22:00.000Z
---

# Phase 25: Migration Source Selection Hardening - Context

## Phase Boundary

**Goal:** Let explicit migration source paths participate directly in source
selection so custom-location Core or Knots installs can produce concrete
dry-run plans.

**Success criteria:**
1. `open-bitcoin migrate plan --source-datadir <custom-path>` can select a
   valid source install outside the default detection roots when the path is
   explicit and supported.
2. Ambiguous or missing source installs still fall back to manual review with
   clear operator guidance.
3. Migration verification covers explicit custom-path selection while
   preserving dry-run-first safety and source-data non-mutation.

**Out of scope:**
- Migration apply mode, source-service cutover, or source-datadir mutation.
- A new migration scanner independent of the existing read-only detector.
- New wallet import, restore, or rewrite behavior.
- Milestone evidence reconciliation owned by Phase 26.

## Requirements In Scope

- `MIG-02`: Onboarding and migration can detect or select existing Core/Knots
  installs, datadirs, config files, cookie files, services, and wallet
  candidates.
- `MIG-04`: Migration supports dry-run plans that show every proposed file,
  config, service, and wallet action before any write occurs.

## Canonical References

- `.planning/ROADMAP.md` — Phase 25 goal, requirements, and success criteria.
- `.planning/REQUIREMENTS.md` — migration traceability ledger.
- `.planning/v1.1-MILESTONE-AUDIT.md` — warning `INT-W03` and the custom-path
  source-selection gap evidence.
- `.planning/phases/21-drop-in-parity-audit-and-migration/21-CONTEXT.md` —
  Phase 21 migration decisions, especially reuse of existing detection evidence
  and the dry-run-only safety posture.
- `AGENTS.md` — repo-native verification contract and README review guidance.
- `AGENTS.bright-builds.md` plus the pinned Bright Builds standards pages for
  architecture, code shape, verification, testing, and Rust.
- `packages/open-bitcoin-cli/src/operator/runtime.rs` — command dispatch and
  one-shot detection gathering.
- `packages/open-bitcoin-cli/src/operator/detect.rs` — reusable read-only
  installation detector.
- `packages/open-bitcoin-cli/src/operator/migration/planning.rs` — source
  selection and dry-run plan construction.
- `packages/open-bitcoin-cli/src/operator/migration/tests.rs` — focused planner
  tests.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` — end-to-end operator
  flow coverage for dry-run migration planning.

## Repo Guidance That Materially Informs This Phase

- Use `bash scripts/verify.sh` as the repo-native verification contract.
- Reuse the existing read-only detection surface instead of inventing a second
  scanner.
- Keep migration planning dry-run only and conservative when evidence is weak.
- Prefer focused unit and operator-binary tests over environment-coupled manual
  validation.

## Current State

- `execute_operator_cli_inner()` gathers detection evidence once from the target
  config resolution and never adds the explicit `--source-datadir` path before
  dispatching `migrate plan`.
- `select_source_installation()` can only match explicit source paths against
  already-detected installs, so custom paths outside the default roots degrade
  to manual review.
- The existing migration binary test exercises `--source-datadir`, but its
  source datadir lives at `HOME/.bitcoin`, which is already inside the default
  detection roots and therefore misses the audit gap.

## Decisions

1. **Keep the existing read-only detector as the only source scanner.**
   Phase 25 should augment migration-time detection roots with the explicit
   source path instead of creating a second scanner or bypassing the detector.
2. **Only migration gets the augmented explicit path.**
   Status, onboarding, dashboard, and other commands should keep their existing
   detection behavior so the gap closure stays narrow.
3. **Explicit source selection must still require source evidence.**
   A bare existing directory is not enough to count as a supported source
   install. The planner should require source config, cookie, or wallet evidence
   before auto-selecting the explicit path.
4. **Dry-run and redaction guarantees stay unchanged.**
   The phase widens source selection only; it must not broaden migration into
   apply-mode behavior or expose cookie contents, raw wallet data, or other
   secrets.

## Key Files and Likely Change Surfaces

- `packages/open-bitcoin-cli/src/operator/runtime.rs`
- `packages/open-bitcoin-cli/src/operator/migration/planning.rs`
- `packages/open-bitcoin-cli/src/operator/migration/tests.rs`
- `packages/open-bitcoin-cli/tests/operator_binary.rs`
- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`

## Risks

- If the explicit path is added too broadly, unrelated commands could start
  treating arbitrary user-supplied paths as detection roots.
- If the planner accepts a bare directory with no migration evidence, it could
  overclaim certainty instead of preserving the manual-review safety posture.
- Service evidence is global to the detector roots rather than datadir-specific,
  so explicit-path validation should key off config, cookie, or wallet evidence
  instead of service files alone.

## Implementation Notes

- Prefer a small runtime helper that augments detection roots only for the
  migration command.
- Keep the explicit-path support check in pure planner logic so it is easy to
  unit test.
- Add an operator-binary regression whose source datadir sits outside the
  default `HOME/.bitcoin` roots to prove the audit gap is actually closed.
