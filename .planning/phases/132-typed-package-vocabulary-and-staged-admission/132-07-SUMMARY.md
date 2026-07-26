---
phase: 132-typed-package-vocabulary-and-staged-admission
plan: "07"
subsystem: mempool
tags: [rust, mempool, truc, pay-to-anchor, ephemeral-dust, package-policy, parity]
requires:
  - phase: 132-typed-package-vocabulary-and-staged-admission
    plan: "06"
    provides: Atomic post-replacement prospective package transitions and late script boundary
provides:
  - Exact pay-to-anchor classification and upgradable-witness consensus behavior
  - Typed dust-relay and ephemeral-permission defaults with independent output gates
  - Complete Reject, Accept, and Enforce TRUC package policy over the prospective graph
  - Post-replacement zero-fee and complete ephemeral-dust spend validation before scripts
affects: [mempool, package-admission, package-relay, transaction-standardness, parity]
tech-stack:
  added: []
  patterns:
    - Evaluate TRUC against pre-replacement graph facts and explicit eviction intent
    - Evaluate ephemeral dust against the post-replacement prospective view before late scripts
    - Preserve policy failures as typed package-report vocabulary with mutation-free rollback
key-files:
  created:
    - packages/open-bitcoin-mempool/src/policy/truc.rs
    - packages/open-bitcoin-mempool/src/policy/truc/tests.rs
    - packages/open-bitcoin-mempool/src/policy/ephemeral.rs
    - packages/open-bitcoin-mempool/src/policy/ephemeral/tests.rs
  modified:
    - packages/open-bitcoin-consensus/src/classify.rs
    - packages/open-bitcoin-consensus/src/script/witness.rs
    - packages/open-bitcoin-mempool/src/fee.rs
    - packages/open-bitcoin-mempool/src/types.rs
    - packages/open-bitcoin-mempool/src/policy/output.rs
    - packages/open-bitcoin-mempool/src/package/report.rs
    - packages/open-bitcoin-mempool/src/pool/package_admission.rs
    - packages/open-bitcoin-mempool/src/pool/package_admission/residual.rs
    - packages/open-bitcoin-mempool/src/pool/tests/package_policy_cases.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "Pay-to-anchor remains an explicitly classified version-1 witness program with upgradable-witness consensus semantics; empty-witness enforcement is policy-only."
  - "TRUC evaluates before replacement from direct conflicts and eligible sibling-eviction intent; limited replacement remains the only stage that materializes removals."
  - "Ephemeral policy evaluates the post-replacement prospective view and must pass before late script checks or any transition reaches the base mempool."
patterns-established:
  - "Policy order is static floor, TRUC, rolling floor, limits, replacement, ephemeral dust, then late scripts."
  - "Package-policy errors cross the report boundary as typed hard failures while the owned prospective transition is discarded."
requirements-addressed: [PACK-07]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 132-2026-07-25T18-13-00
generated_at: 2026-07-26T08:47:28Z
duration: 1h 15m
completed: 2026-07-26
---

# Phase 132 Plan 07: TRUC, Pay-to-Anchor, and Ephemeral Dust Policy Summary

**Pinned Knots parity now covers explicit pay-to-anchor standardness, all three TRUC modes, and zero-fee complete ephemeral-dust spending in the staged package engine.**

## Performance

- **Duration:** 1h 15m
- **Started:** 2026-07-26T07:32:00Z
- **Completed:** 2026-07-26T08:47:28Z
- **Tasks:** 3
- **Files modified:** 23

## Accomplishments

- Added exact `OP_1 0x02 0x4e 0x73` pay-to-anchor classification while preserving upgradable-witness consensus behavior and enforcing empty-witness policy for spends.
- Replaced fixed dust cutoffs with a typed 3,000 sat/kvB dust relay rate and installed exact `anchor=true`, `send=false`, `dust=false` permission defaults with the complete form/value/permission matrix.
- Implemented pure typed TRUC policy for Reject, Accept, and Enforce modes, including version inheritance, exact size and topology limits, direct-child replacement, and eligible sibling-eviction intent.
- Integrated TRUC before replacement and ephemeral validation after replacement but before late scripts, with typed hard failures and unchanged base state on every rejection path.
- Anchored the new policy and test modules directly to pinned Bitcoin Knots source and functional-test evidence.

## Task Commits

Each task was committed atomically:

1. **Task 1: Classify pay-to-anchor and install typed dust/ephemeral defaults** - `7bee697f`
2. **Task 2: Enforce complete TRUC inheritance, topology, size, and replacement-aware rules** - `48e34549`
3. **Task 3: Enforce zero-fee ephemeral dust and complete parent-dust spending** - `2d976607`

## Files Created/Modified

- `packages/open-bitcoin-consensus/src/classify.rs` - Exact pay-to-anchor script classification before generic unknown witness handling.
- `packages/open-bitcoin-consensus/src/script/witness.rs` - Upgradable-witness consensus routing for pay-to-anchor.
- `packages/open-bitcoin-consensus/src/script/tests.rs` - Classification and witness behavior regressions.
- `packages/open-bitcoin-mempool/src/fee.rs` - Semantic `DustRelayFeeRate` wrapper with the pinned default.
- `packages/open-bitcoin-mempool/src/types.rs` - `EphemeralPolicy` and exact `PolicyConfig` defaults.
- `packages/open-bitcoin-mempool/src/policy/output.rs` - Rate-derived dust thresholds and independent anchor/send/dust output predicates.
- `packages/open-bitcoin-mempool/src/policy/truc.rs` - Pure version, size, ancestry, descendant, inheritance, and replacement-aware TRUC validation.
- `packages/open-bitcoin-mempool/src/policy/truc/tests.rs` - Direct policy matrix and typed branch coverage.
- `packages/open-bitcoin-mempool/src/policy/ephemeral.rs` - Zero-fee and complete parent-dust spend validation over a mempool view.
- `packages/open-bitcoin-mempool/src/policy/ephemeral/tests.rs` - Multiple-output, multiple-parent, permission, fee, and in-mempool parent regressions.
- `packages/open-bitcoin-mempool/src/package/report.rs` - Typed TRUC and ephemeral hard-member failures.
- `packages/open-bitcoin-mempool/src/pool/package_admission.rs` - Singleton TRUC and ephemeral stage integration.
- `packages/open-bitcoin-mempool/src/pool/package_admission/residual.rs` - Residual TRUC, sibling replacement intent, and post-replacement ephemeral sequencing.
- `packages/open-bitcoin-mempool/src/pool/tests/package_policy_cases.rs` - Public-path P2A, TRUC, ephemeral, ordering, rollback, script-count, and dry-run/submit parity matrix.
- `docs/parity/source-breadcrumbs.json` - Pinned Knots source registration for all new first-party Rust policy files.
- `docs/metrics/lines-of-code.md` - Refreshed tracked source metrics through normal hooks.

## Decisions Made

- P2A is a distinct script classification so policy can recognize its role, but consensus deliberately treats it like the corresponding unknown witness version rather than introducing a new spend rule.
- `anchor`, `send`, and `dust` are independent predicates: anchor permission cannot be recreated by enabling the other flags, non-anchor dust requires send permission, and every nonzero dust output additionally requires dust permission.
- Enforced TRUC uses the original pre-replacement graph plus explicit direct-conflict and sibling-eviction facts so topology is evaluated hypothetically before limited RBF owns removal staging.
- Residual ephemeral validation observes both accepted package parents and surviving mempool parents after replacement decisions, and every child must completely sweep all permitted dust outputs of every direct parent.
- Late scripts remain a separate final policy stage; TRUC and ephemeral failures execute zero scripts and discard the prospective transition.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Registered every new first-party test module with parity evidence**

- **Found during:** Task 2 normal-hook verification
- **Issue:** The parity checker rejected the newly split TRUC test source until its pinned Knots anchors were represented in the breadcrumb manifest.
- **Fix:** Added the new policy and test paths to the mempool TRUC parity group with exact `truc_policy` and `mempool_truc.py` anchors.
- **Files modified:** `docs/parity/source-breadcrumbs.json`
- **Verification:** `bun run scripts/check-parity-breadcrumbs.ts` passed, including all Plan 07 sources.
- **Committed in:** `48e34549`

**2. [Rule 3 - Blocking] Closed full-workspace coverage gaps exposed by the normal hook**

- **Found during:** Task 2 normal-hook verification
- **Issue:** The initial coverage run exposed unexercised typed TRUC diagnostics, package-report mappings, sibling-replacement policy, and staged fee guard branches.
- **Fix:** Added focused pure and public-path tests for each branch, including typed error displays and fail-closed injected-stage behavior.
- **Files modified:** `packages/open-bitcoin-mempool/src/policy/truc/tests.rs`, `packages/open-bitcoin-mempool/src/policy/replacement/tests.rs`, `packages/open-bitcoin-mempool/src/package/tests.rs`, `packages/open-bitcoin-mempool/src/pool/tests/package_admission_cases.rs`
- **Verification:** The retried Task 2 normal hook passed the complete workspace coverage gate.
- **Committed in:** `48e34549`

**3. [Rule 2 - Missing Critical] Preserved the repository production file-length contract**

- **Found during:** Task 2 file-length verification
- **Issue:** TRUC stage integration pushed production package-admission orchestration beyond the enforced code-shape boundary.
- **Fix:** Kept production orchestration focused and moved test-only stage controls behind the existing `package_admission::test_support` boundary.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/package_admission.rs`, `packages/open-bitcoin-mempool/src/pool/package_admission/test_support.rs`, `packages/open-bitcoin-mempool/src/pool/tests/package_admission_cases.rs`
- **Verification:** `bash scripts/check-file-lengths.sh` passed for all 320 checked production Rust files with the 628-line cap.
- **Committed in:** `48e34549`

**Total deviations:** 3 auto-fixed (2 blocking verification fixes, 1 missing-critical code-shape fix)

**Impact on plan:** The fixes supplied required parity registration, coverage, and repository code-shape compliance without changing the planned policy surface or adding adapter/configuration scope.

## Issues Encountered

- The Task 2 hook surfaced the new-source breadcrumb, coverage, and production code-shape obligations together; each was resolved inside the same task boundary before the atomic commit.
- No authentication, external-service, or architectural gate was encountered.

## Verification

- Focused pay-to-anchor consensus classification and mempool standardness tests passed with nonzero coverage.
- Focused TRUC policy and package-path matrices passed for all three modes, exact limits, inheritance, direct-child replacement, sibling intent, and atomic rollback.
- Focused ephemeral policy and package-path matrices passed for independent base/modified fees, multiple dust outputs and parents, in-mempool parents, complete sweeps, script ordering, and dry-run/submit equality.
- `cargo fmt --manifest-path packages/Cargo.toml --all` passed.
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` passed.
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml --all-features` passed.
- `bun run scripts/check-parity-breadcrumbs.ts` verified 438 Rust files.
- `bash scripts/check-file-lengths.sh` verified 320 production Rust files under the 628-line cap.
- Focused LLVM coverage reported no uncovered production lines in `policy/ephemeral.rs`.
- All three normal task commits passed the complete `bash scripts/verify.sh` contract, including Bazel smoke builds and workspace coverage; Task 3 completed in 3m 52.520s.

## Authentication Gates

None.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- PACK-07 now has the complete staged policy vocabulary and ordering needed by later package relay and operator surfaces.
- The mempool exposes no new RPC, CLI, persistence, JSONC, environment, or peer-origin coupling, so later adapters can consume the typed policy boundary without reworking core admission.
- No Plan 07 blockers remain.

## Self-Check: PASSED

- Summary file exists with the exact Plan 07 generator, lifecycle, and requirements metadata.
- Task commits `7bee697f`, `48e34549`, and `2d976607` exist in repository history.
- All four created TRUC and ephemeral policy/test files exist and are parity-registered.
- Stub scan found no implementation placeholders; the only textual match was the crate-level Clippy lint name `clippy::todo`.
- No new network, authentication, file-access, or schema trust boundary was introduced.
- Summary diff is whitespace-clean, and orchestrator-owned `STATE.md` and `ROADMAP.md` remain unstaged.
