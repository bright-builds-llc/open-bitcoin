---
phase: 69-tip-tracking-and-stay-current-operation
plan: 05
subsystem: node-sync-runtime
tags: [docs, verification, sync-runtime, stay-current]

requires:
  - phase: 69-tip-tracking-and-stay-current-operation
    plan: 04
    provides: Post-catch-up stay-current progress regression coverage.
provides:
  - Operator documentation for Phase 69 tip and stay-current fields
  - Architecture documentation for best-known-tip and stay-current semantics
  - Repo-native Phase 69 checker wired into verification
  - Default verification guard against broader public-network or service-manager scope
affects: [operator-docs, status-docs, verification, phase-69]

tech-stack:
  added: []
  patterns: [repo-native-checker, bounded-status-docs, scope-guard-verification]

key-files:
  created:
    - scripts/check-phase69-tip-stay-current.ts
  modified:
    - docs/operator/runtime-guide.md
    - docs/architecture/status-snapshot.md
    - packages/open-bitcoin-node/src/status.rs
    - scripts/verify.sh
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Document Phase 69 fields as status and architecture semantics, not as a broader public-network readiness claim."
  - "Wire the Phase 69 checker immediately after the Phase 68 active-chain persistence checker in scripts/verify.sh."
  - "Keep default verification public-network-free by rejecting live-mainnet, manual-peer, system service manager, and mainnet IBD activation commands in verify.sh."

patterns-established:
  - "Phase closeout checkers should prove plan artifacts, docs, runtime contracts, tests, and default verification scope together."
  - "Operator docs should pair stay-current labels with Phase 68 active-chain counters so headers-only progress is not mistaken for current-at-tip status."

requirements-completed: [TIP-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 69-2026-06-11T15-13-14
generated_at: 2026-06-11T19:26:23Z

duration: 34min
completed: 2026-06-11
---

# Phase 69-05: Documentation and Verification Closeout

**Documented Phase 69 tip/stay-current semantics and added a repo-native checker that keeps the phase evidence and default verification scope bounded.**

## Performance

- **Duration:** 34 min
- **Completed:** 2026-06-11T19:26:23Z
- **Tasks:** 4
- **Files modified:** 6

## Accomplishments

- Added operator documentation for every Phase 69 status field:
  `sync.best_known_tip.source`, `height`, `block_hash`, `work`,
  `block_time_unix_seconds`, `observed_at_unix_seconds`, `freshness`,
  `peer_agreement`, `sync.stay_current`, and
  `sync.stay_current_next_action`.
- Added architecture documentation for `BestKnownTipStatus`,
  `StayCurrentStatus`, peer agreement labels, freshness, and the rule that
  `current_at_best_known_tip` requires matching validated active-chain progress.
- Added `scripts/check-phase69-tip-stay-current.ts`.
  - Requires Phase 69 planning artifacts and resolved research questions.
  - Checks status contracts, runtime projection strings, Phase 69 test names,
    and docs coverage.
  - Rejects default verification commands that would add public-network,
    manual-peer, service-manager, or mainnet IBD scope.
  - Scans docs and phase artifacts for broad shipped-scope claims while allowing
    explicit non-goal or negated wording.
- Wired the checker into `scripts/verify.sh` immediately after
  `scripts/check-phase68-active-chain-persistence.ts`.
- Added stable serialized-label comments to `StayCurrentStatus` so the source
  contract itself contains the exact machine labels audited by the checker.
- Refreshed the tracked LOC report through the repo-managed generator and
  commit hook.

## Task Commits

1. **Tasks 1-4: Document and enforce Phase 69 closeout evidence** - `ce36361` (feat)

## Files Created/Modified

- `scripts/check-phase69-tip-stay-current.ts` - Adds the Phase 69 closeout checker for artifacts, runtime/status contracts, docs, tests, and verification scope.
- `scripts/verify.sh` - Runs the Phase 69 checker after the Phase 68 active-chain persistence checker.
- `docs/operator/runtime-guide.md` - Documents the operator-facing Phase 69 tip and stay-current fields and labels.
- `docs/architecture/status-snapshot.md` - Documents the status contract semantics and the validated active-chain requirement for current-at-tip.
- `packages/open-bitcoin-node/src/status.rs` - Adds exact serialized-label comments to `StayCurrentStatus`.
- `docs/metrics/lines-of-code.md` - Refreshed by the repo-managed LOC generator and commit hook.

## Decisions Made

Default verification remains local and deterministic. Phase 69 does not add public-network smoke, manual peer activation, service-manager commands, or unattended mainnet IBD activation to `scripts/verify.sh`; the checker now rejects those strings if they are introduced later.

## Deviations from Plan

None.

## Issues Encountered

The first standalone `bash scripts/verify.sh` run stopped immediately on a stale LOC report. Regenerating `docs/metrics/lines-of-code.md` with the repo-managed LOC generator resolved it, and the full verifier then passed.

## User Setup Required

None. The checker, docs validation, tests, benchmark smoke, and Bazel smoke all run locally without public-network access.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase69_ --all-features`
- `bun run scripts/check-phase69-tip-stay-current.ts`
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md`
- `bash scripts/verify.sh` completed successfully in `5m 55.857s`.
- `git diff --check`
- Commit hook reran `bash scripts/verify.sh` successfully before `ce36361` and completed in `5m 29.291s`.

## Next Phase Readiness

Phase 69 is complete. The runtime has best-known-tip tracking, current-at-tip/stale/recovery/no-progress labels, post-catch-up regression coverage, operator documentation, and a repo-native verification guard that keeps default evidence deterministic and scope-bounded.

## Self-Check: PASSED

---
*Phase: 69-tip-tracking-and-stay-current-operation*
*Completed: 2026-06-11*
