---
phase: 21-drop-in-parity-audit-and-migration
generated_by: gsd-research-phase
lifecycle_mode: yolo
phase_lifecycle_id: 21-2026-04-27T23-11-20
generated_at: 2026-04-27T23:11:20.765Z
---

# Phase 21 Research: Drop-In Parity Audit and Migration

## Summary

The smallest honest Phase 21 step is a new dry-run-only operator command,
`open-bitcoin migrate plan`, backed by a pure planner module and wired through
the existing operator runtime. Current code already has the needed ingredients:
read-only install detection, target-config planning, dry-run service previews,
and read-only external-wallet classification. What is missing is a dedicated
migration planner that turns that evidence into operator-safe action items and
links those items to the parity ledger.

The parity side should stay lightweight. The repo already treats
`docs/parity/index.json` as the machine-readable source of truth for intentional
deviations and catalog membership. Phase 21 should add a new catalog entry for
the drop-in audit/migration slice, populate the top-level `deviations[]` array
with migration-relevant intentional differences, and let the CLI surface only
the deviations relevant to the chosen dry-run plan.

## Existing Surfaces To Reuse

### Operator detection
- `packages/open-bitcoin-cli/src/operator/detect.rs` already discovers datadirs,
  configs, cookies, service definitions, and wallet candidates on macOS and
  Linux.
- Detection is intentionally read-only and already preserves bytes, timestamps,
  and permissions in tests.
- `DetectedInstallation` plus `WalletCandidate` is the right planner input
  surface. Phase 21 should not add a second scanner.

### Onboarding and plan/apply split
- `packages/open-bitcoin-cli/src/operator/onboarding.rs` already models a pure
  planning step and a later apply step.
- This is useful precedent, but onboarding should remain the target Open Bitcoin
  setup flow. Migration needs a separate source-selection concern.

### Service dry-run model
- `packages/open-bitcoin-cli/src/operator/service.rs` already proves the dry-run
  preview pattern the milestone wants: list what would be written or run without
  making changes unless explicitly applied.
- Migration planning can mirror that tone and structure without introducing an
  apply path in this phase.

### External-wallet safety
- `packages/open-bitcoin-cli/src/operator/wallet.rs` already treats external
  wallet candidates as high-risk data and rejects overlapping backup
  destinations.
- Phase 21 should extend that safety posture into migration planning rather than
  broadening mutation scope.

## Recommended Command Shape

### Primary command

Add a new operator-owned command:

```text
open-bitcoin migrate plan [--source-datadir PATH]
```

Recommended properties:
- dry-run only in Phase 21
- uses existing global `--config`, `--datadir`, `--network`, `--format`, and
  `--no-color`
- uses `--source-datadir` only when the detected source install is ambiguous
- prints benefits, tradeoffs, unsupported surfaces, rollback expectations,
  backup requirements, then the concrete action list

### Why not extend onboarding

`onboard` is already clearly about Open Bitcoin's target config and first-run
experience. Folding migration into that flow would blur source-vs-target
semantics, make the wizard heavier, and mix install detection with target
settings in one command.

## Minimal Honest Planner Model

The planner should stay explicit and conservative:

- **Source install**: either the explicit `--source-datadir` match or one
  unambiguous detected installation
- **Target environment**: current Open Bitcoin config resolution and target
  datadir/config path
- **Explanation section**:
  - benefits
  - tradeoffs
  - unsupported surfaces
  - rollback expectations
  - backup requirements
- **Action groups**:
  - config actions
  - datadir/file actions
  - service actions
  - wallet actions
  - operator follow-up actions
- **Relevant deviations**:
  - selected from `docs/parity/index.json`
  - surfaced in both human and JSON output

If the source install is ambiguous or partial, the planner should emit explicit
manual-review actions instead of pretending certainty.

## Parity Audit Approach

### New catalog entry

Add `docs/parity/catalog/drop-in-audit-and-migration.md` as a phase-sized audit
slice. This is the best place for MIG-01 because the audit spans several
existing catalog surfaces:
- CLI and config
- RPC
- datadir layout
- service behavior
- wallet behavior
- sync/runtime
- logs
- operator docs

The page should act as an audit matrix:
- baseline expectation
- Open Bitcoin current behavior
- evidence
- migration impact
- linked deviation ids

### Ledger changes

Update `docs/parity/index.json` to:
- register the new catalog document
- add a checklist surface for the migration audit
- populate `deviations[]` with any intentional differences Phase 21 confirms

Update `docs/parity/deviations-and-unknowns.md` to summarize:
- new intentional differences
- remaining migration unknowns or manual-only surfaces

## Likely Intentional Differences To Record

These are the most probable Phase 21 deviation categories:
- migration is dry-run-only and manual-review-heavy rather than mutation-capable
- Open Bitcoin keeps `open-bitcoin.jsonc` for Open Bitcoin-only settings instead
  of writing those into `bitcoin.conf`
- managed-wallet backup/export is Open Bitcoin-owned JSON, not a Core-compatible
  `wallet.dat` copy
- service cutover remains explicit and operator-driven rather than automatic

Only record a deviation once the Phase 21 implementation and docs can point to
clear evidence.

## Test Strategy

### High-value unit tests
- planner output includes explanation text plus action groups
- planner is dry-run only by default
- planner stays conservative under ambiguous detection
- planner redacts cookies, auth secrets, and raw wallet data
- planner includes only relevant deviations

### High-value CLI tests
- operator clap parsing for `migrate plan`
- runtime routing dispatches the new command
- one binary test exercises a realistic detected source install and proves the
  dry-run plan leaves source files untouched

### Verification
- fast loop: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli`
- final gate: `bash scripts/verify.sh`
- if new Rust files are added under `packages/open-bitcoin-cli/src/operator/`,
  update `docs/parity/source-breadcrumbs.json`

## Risks

1. **Source/target confusion**: existing global flags describe the Open Bitcoin
   target environment, while migration needs a source install selector too.
2. **Deviation drift**: if CLI notices are hardcoded instead of ledger-backed,
   MIG-05 will drift from `docs/parity/index.json`.
3. **Overclaiming certainty**: detection is intentionally conservative, so the
   planner must preserve ambiguity and manual-review steps where evidence is not
   strong enough.
