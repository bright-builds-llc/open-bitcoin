---
phase: 152-post-prune-wallet-rescan-eligibility
plan: "01"
subsystem: wallet
tags: [wallet, rescan, fjall, prune, durable-evidence]
requires:
  - phase: 146-wallet-leftover-snapshot-cutover
    provides: Durable coins authority and checkpointed wallet jobs
  - phase: 148-fjall-payload-unlink-and-have-pruned
    provides: Paired block/undo deletion and have-pruned evidence
provides:
  - Shared staged wallet replacement and creating-payload eligibility
  - Interrupted-coins refusal at direct wallet authority loading
  - Node job-first refusal with durable sanitized Failed evidence
  - Real paired-prune, resumed chunk, reopen and success regressions
affects: [152-02, 152-03, SNAP-01]
tech-stack:
  added: []
  patterns: [staged replacement before shell presence probes, safe typed refusal facts]
key-files:
  created:
    - packages/open-bitcoin-node/src/wallet_registry/rescan.rs
    - packages/open-bitcoin-node/src/wallet_registry/rescan/tests.rs
    - packages/open-bitcoin-node/src/sync/tests/wallet_rescan_runtime/eligibility.rs
  modified:
    - packages/open-bitcoin-node/src/wallet_registry.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
    - packages/open-bitcoin-node/src/sync/wallet_rescan.rs
    - packages/open-bitcoin-node/src/sync/tests/wallet_rescan_runtime.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "Reuse pure wallet rescan selection on a staged copy; check requested heights and only selected creating heights."
  - "Keep typed original errors for callers and store only boundary/category/known height/hash in job detail."
  - "Expose one ordinary-build callback preparation seam and an interruption guard for dependent RPC adapters."
  - "Keep requirements-completed empty until complete phase verification; root owns state and commits."
patterns-established:
  - "Probe each distinct required hash once, while resolving every required height explicitly."
  - "Identified resumable jobs record Failed before returning authority or eligibility errors."
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 152-2026-10-03T00-26-52
generated_at: 2026-10-03T01:08:27Z
duration: 11 min
completed: 2026-10-03
---

# Phase 152 Plan 01: Shared Wallet Eligibility and Node Refusal Summary

**Staged full wallet replacement now checks earlier matching creating payloads, refuses interrupted coins authority, and preserves the saved wallet/checkpoint with durable Failed evidence.**

## Performance

- **Measured implementation/test interval:** 11 min, from first RED command at 2026-10-03T00:57:25Z through 2026-10-03T01:08:27Z; preparatory reading preceded this interval.
- **Tasks:** 3/3 implemented and targeted verification passed.
- **Source/manifest files modified:** 8.
- **Full phase verification:** Pending root execution after later waves; this summary does not claim its success.

## Accomplishments

- `prepare_durable_wallet_rescan` stages the existing pure full rescan on a copied wallet and gates all requested and selected creating heights before returning the candidate. Missing height membership fails explicitly, duplicate hashes probe once, unrelated old coins are excluded, and entries above the replacement height are filtered out.
- `WalletRescanEligibilityFailure` retains the original registry/storage category and known boundary/height/hash. Its durable `safe_detail` excludes backend text and paths.
- `wallet_scan_chainstate_snapshot` and the shared durable preparation boundary inspect actual coins head markers and refuse `InterruptedTwoHeads` with the existing typed InterruptedWrite error. They neither repair markers nor load leftover snapshots.
- Node advance identifies the existing job before authority reads, returns inactive jobs without unrelated chain reads, and saves Failed evidence on authority/probe/preparation errors. Saving Failed errors propagates visibly; replacement and checkpoint writes occur only after successful preparation.
- New behavior tests perform real paired block/undo deletion, preserve durable coins and best-block, assert have-pruned, and exercise midrange, in-range, resumed/reopened and retained/unrelated controls. A conflicting leftover blob stays on disk and cannot supply wallet truth.

## RED and Verification Evidence

All Cargo commands ran sequentially through `bun run scripts/command-timings.ts run --key ... --` with isolated pinned Bun 1.3.9 first on PATH. No overlapping Cargo jobs were started.

| Command/filter | Evidence |
| --- | --- |
| Node `--lib post_prune_midrange_rescan_preserves_wallet_and_fails_job --all-features` before adapter changes | **RED:** exit 101, 0 passed/1 failed. After paired deletion at creating height 1, the unchanged adapter returned `Ok(Complete, Fresh, through=3, next=4)` instead of refusal. No undefined-helper compiler failure was used as the reproducer. |
| Node `--lib wallet_registry::rescan --all-features` | 8 passed/0 failed. |
| Node `--lib wallet_rescan_runtime --all-features` | 14 passed/0 failed before the final metadata case was appended. |
| Node `--lib wallet_rescan_runtime::eligibility --all-features`, final source | 10 passed/0 failed, including the RED reproducer, missing metadata, real H/B, probe error and failure-save error cases. |
| Node `clippy --all-targets --all-features -- -D warnings` | Passed, exit 0. |
| Node `--lib coins_migration --all-features` | 15 passed/0 failed. |
| Workspace Cargo fmt | Passed through the timing wrapper. |
| `bun run scripts/check-parity-breadcrumbs.ts` | Passed for 909 first-party Rust files. |
| `bun run scripts/bright-builds-check.ts all` | 0 findings; file lengths and active lessons passed. |
| `git diff --check` and scoped diff review | Passed; no unintended source changes. |

The new eligibility evidence consists of runtime state assertions. Existing historical source-string tests remain in their original parent module and do not substitute for the new evidence.

## Task Commits

1. **Task 1: Shared staged eligibility** — commit deferred.
2. **Task 2: Node job-first authority/refusal** — commit deferred.
3. **Task 3: Real prune/resume/reopen regressions** — commit deferred.

The strict invoked wrapper forbids executor staging/commits/push until root full-phase verification passes. No executor commit or staging was performed. Root added intent-to-add visibility for the three new Rust files so tracked-file-only checks could inspect them. State, roadmap, requirements, task ledger, final commits and push remain root-owned.

## Files Created/Modified

- `wallet_registry/rescan.rs` and its `tests.rs`: staged selection, deterministic membership/probes, safe failures and ordinary-build RPC seam.
- `wallet_registry.rs`: public child module export through the existing module.
- `storage/fjall_store/coins.rs`: crate-private marker check and guarded direct wallet loading.
- `sync/wallet_rescan.rs`: common handled preparation boundary and a test-only probe/failure-save wrapper over the same node control flow.
- `sync/tests/wallet_rescan_runtime.rs` and its `eligibility.rs`: existing error assertion updated for safe detail, new behavior module registered, real persistence/prune/reopen fixtures.
- `docs/parity/source-breadcrumbs.json`: new paths anchored to Knots wallet.cpp, wallet RPC transactions.cpp and coins.cpp.

## Decisions Made

Repo-local AGENTS guidance, Bright Builds sidecar and relevant architecture/code-shape/testing/verification/Rust pages governed shell placement, module shape, optional names, parity registration, timing wrappers and verification. The active global and repo lessons were read completely (5,230 + 1,958 bytes); no archives were loaded. No project skills were present. The simplification pass reused existing selection and filtering, removed the node's duplicate filter, retained the job schema, and added no dependency or locking architecture.

## Concurrency Boundary

The reachable HTTP dispatcher holds `state.context.lock().await` across synchronous dispatch (`open-bitcoin-rpc/src/http.rs`); manual planned prune enters `ManagedNetworkHandle::flush_applying_prune_plan` under its `mutate` mutex (`network/runtime_authority/prune_flush.rs`). Current `flush_coins` supplies an empty prune plan. Separately exported node runtime/store callers do not share a global rescan/prune transaction. These tests establish completed-prune eligibility and fresh probes on each resumed chunk/reopen; they do **not** establish atomic presence at save for arbitrary concurrent library callers. A new concurrent owner or stronger guarantee requires replanning, as resolved in research.

## Deviations from Plan

- Commit/state steps were delegated to root by the explicit strict-wrapper ownership override. Requirements completion remains empty pending complete phase evidence.
- TDD uses the real existing-adapter paired-prune RED before implementation; helper cases were written before their implementation, but no undefined-helper compilation was counted as bug evidence and no RED commit was created.

## Issues Encountered

- The first runtime test compile used two incorrect fixture import paths; corrected to `crate::storage::coins_codec` and `crate::storage::coins_view`, then both runtime filters passed. No out-of-scope repair was required.
- Breadcrumb enumeration initially could not see untracked new files. Root supplied intent-to-add visibility; the checker then passed without changing its implementation.
- Quiet compiler/test periods were polled; process liveness was inspected without terminating commands on estimated duration.

## Known Stubs

None. No TODO/FIXME/placeholder or unwired candidate path was introduced.

## Threat Review

Planned integrity/evidence threats T-152-01 through T-152-05 are addressed by staged membership, actual marker refusal, handled durable Failed evidence, safe detail, and deduplicated probes. T-152-06 remains the explicitly accepted probe/save concurrency limit above. No unplanned endpoint, authentication, schema, or raw storage mutation surface was introduced.

## User Setup Required

None.

## Next Plan Readiness

The ordinary-build shared APIs are ready for Plan 152-02 RPC integration. Full native verification, lifecycle-valid phase evidence and requirement activation remain with the root after all plans finish.

## Self-Check: PASSED

All eight source/manifest paths exist, all three new Rust files are represented in the breadcrumb manifest, targeted test results above were observed, and scoped diff review passed. Commit existence checks are intentionally deferred under the strict no-commit override; no commit hash or aggregate verifier success is claimed.
