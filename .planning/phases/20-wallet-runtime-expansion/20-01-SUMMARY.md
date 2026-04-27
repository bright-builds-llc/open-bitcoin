---
phase: 20-wallet-runtime-expansion
plan: 01
subsystem: wallet
tags: [rust, wallet, descriptors, bip32, rescan]
requires:
  - phase: 07-wallet-core-and-adapters
    provides: single-key descriptor parsing, wallet build/sign flow, and snapshot persistence seams
provides:
  - ranged single-key descriptor parsing for wpkh, sh(wpkh), and tr xpub/xprv forms
  - wallet-local receive and change allocation cursors persisted through normalized descriptor text
  - shared send-intent fee-selection and change-policy contracts mapped onto the existing build request
  - pure fresh/partial/scanning rescan state projection for later durable shell orchestration
affects: [20-02, 20-03, 20-04, wallet-runtime-expansion]
tech-stack:
  added: []
  patterns:
    - descriptor-owned BIP32 range metadata persisted through normalized descriptor text
    - script-match index recovery for ranged rescan/signing without widening shared snapshot DTOs
key-files:
  created: []
  modified:
    - packages/open-bitcoin-wallet/src/descriptor.rs
    - packages/open-bitcoin-wallet/src/error.rs
    - packages/open-bitcoin-wallet/src/wallet.rs
    - packages/open-bitcoin-wallet/src/wallet/scan.rs
    - packages/open-bitcoin-wallet/src/wallet/sign.rs
    - packages/open-bitcoin-wallet/src/wallet/tests.rs
key-decisions:
  - "Persist ranged descriptor range_start/range_end/next_index inside SingleKeyDescriptor and mirror it into DescriptorRecord.original_text so open-bitcoin-node snapshot DTOs stayed source-compatible."
  - "Recover ranged child indexes for rescanned UTXOs and signing by matching derived scripts instead of widening WalletUtxo or WalletSnapshot outside the plan write set."
patterns-established:
  - "Pure wallet contracts carry fee estimation intent and freshness state; downstream shells resolve estimator availability and persistence."
  - "Ranged descriptor mutation updates the normalized descriptor text immediately so later durable adapters can persist cursor state without extra wallet-side DTOs."
requirements-completed: [WAL-04, WAL-06, WAL-07]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 20-2026-04-27T09-35-46
generated_at: 2026-04-27T10:51:38Z
duration: 9 min
completed: 2026-04-27
---

# Phase 20 Plan 01: Pure Wallet-Core Ranged Descriptor, Send-Intent, and Rescan-Progress Contracts Summary

**Ranged single-key descriptor derivation, wallet-local cursor allocation, shared send-intent mapping, and pure wallet rescan freshness contracts landed in `open-bitcoin-wallet`.**

## Performance

- **Duration:** 9 min
- **Started:** 2026-04-27T10:42:50Z
- **Completed:** 2026-04-27T10:51:38Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments
- Added a narrow xpub/xprv BIP32 descriptor slice for `wpkh(...)`, `sh(wpkh(...))`, and `tr(...)`, while continuing to reject multipath, miniscript, and multisig forms explicitly.
- Added wallet-local receive and change allocation APIs that advance descriptor cursors exactly once per successful allocation and persist the updated cursor through normalized descriptor text.
- Added shared send-intent and wallet rescan freshness contracts so later node, RPC, and operator shells can reuse one pure-core surface for fee intent and fresh/partial/scanning status.

## Task Commits

1. **Task 1: Define failing ranged-descriptor, send-intent, and rescan-progress contracts** - `2f041dc` (`test`)
2. **Task 2: Implement deterministic ranged descriptors, address allocation, and send/rescan domain logic** - `c5525df` (`feat`)

## Files Created/Modified
- `packages/open-bitcoin-wallet/src/descriptor.rs` - Adds minimal in-crate BIP32 parsing/derivation, range metadata persistence, and ranged descriptor matching helpers.
- `packages/open-bitcoin-wallet/src/error.rs` - Adds typed wallet errors for descriptor ranges, cursor exhaustion, estimator resolution, fee ceilings, and change-policy/address-role mismatches.
- `packages/open-bitcoin-wallet/src/wallet.rs` - Adds send-intent and rescan-state contracts plus wallet-local address allocation that updates normalized descriptor text.
- `packages/open-bitcoin-wallet/src/wallet/scan.rs` - Matches rescans against ranged descriptor scripts and exposes pure fresh/partial/scanning state projection.
- `packages/open-bitcoin-wallet/src/wallet/sign.rs` - Recovers ranged child indexes from matched scripts so signing uses the correct derived child key.
- `packages/open-bitcoin-wallet/src/wallet/tests.rs` - Covers ranged descriptor normalization, cursor advancement, send-intent validation, and rescan freshness semantics.

## Decisions Made

- Stored range and cursor state inside the descriptor model and persisted it through normalized descriptor text to avoid changing shared wallet snapshot/descriptor wrapper shapes owned by downstream crates.
- Kept runtime estimator resolution out of `open-bitcoin-wallet`; the pure core accepts explicit fee rates or typed estimate intent and returns deterministic unresolved-estimator errors when shells do not supply a concrete `FeeRate`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Adjusted Rust formatter invocation to the workspace manifest**
- **Found during:** Task 1 verification
- **Issue:** `cargo fmt --all` from the repo root failed because the Rust workspace manifest lives at `packages/Cargo.toml`.
- **Fix:** Switched formatting and Rust verification invocations to `--manifest-path packages/Cargo.toml`, which matches the repo layout used elsewhere in the phase plan.
- **Files modified:** None
- **Verification:** `cargo fmt --manifest-path packages/Cargo.toml --all` passed
- **Committed in:** `c5525df` (task implementation commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** No scope creep. The deviation only corrected the workspace command shape needed to execute the planned Rust verification.

## Issues Encountered

- `bash scripts/verify.sh` failed on `docs/metrics/lines-of-code.md` being stale and requested `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md`. I did not update that generated file because it is outside the user-assigned write set for this plan. Wallet-scoped Rust verification passed; repo-native verification remains blocked on that out-of-scope generated artifact.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The pure wallet core now exposes ranged descriptor state, cursor allocation, send-intent mapping, and rescan freshness contracts for Plan 20-02 and later wallet runtime shells.
- The only remaining immediate concern is the stale LOC report generated artifact outside this plan’s owned files; repo-wide verify will continue to stop there until that file is refreshed in the appropriate scope.

## Self-Check: PASSED

- Found summary file `.planning/phases/20-wallet-runtime-expansion/20-01-SUMMARY.md`
- Found commit `2f041dc`
- Found commit `c5525df`

---
*Phase: 20-wallet-runtime-expansion*
*Completed: 2026-04-27*
