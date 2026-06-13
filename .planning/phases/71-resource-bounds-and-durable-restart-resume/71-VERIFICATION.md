---
phase: 71-resource-bounds-and-durable-restart-resume
verified: 2026-06-13T13:47:35Z
status: passed
score: 15/15 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 71-2026-06-13T10-34-37
generated_at: 2026-06-13T13:47:35Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 71: Resource Bounds and Durable Restart/Resume Verification Report

**Phase Goal:** Operators can run long full-sync attempts within documented resource bounds and recover safely after interruptions or storage pressure.
**Verified:** 2026-06-13T13:47:35Z
**Status:** passed
**Re-verification:** No - initial verification

## Goal Achievement

Phase 71 achieves its roadmap goal. The implementation proves resource bounds, restart/resume behavior, storage-pressure guidance, and deterministic long-chain coverage through source-level tests, docs, a deterministic Phase 71 checker, and the repo-native verification gate.

Inputs loaded for this verification included `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, pinned Bright Builds standards pages, `.planning/ROADMAP.md`, `.planning/REQUIREMENTS.md`, Phase 71 context/research/plans/summaries/review, and all changed Phase 71 source/docs/checker files. No previous `*-VERIFICATION.md` existed.

## Observable Truths

| # | Truth | Status | Evidence |
|---|---|---|---|
| 1 | Bounds are documented and tested for peers, in-flight blocks, queues, caches, storage writes, logs, metrics, and support evidence. | VERIFIED | `docs/operator/runtime-guide.md:785` documents the full bound list; `packages/open-bitcoin-node/src/sync/tests.rs:5729` asserts a 48-block synthetic bound path; `bash scripts/verify.sh` passed. |
| 2 | Support evidence remains compact and redacted during long sync review. | VERIFIED | `packages/open-bitcoin-cli/src/operator/support.rs:514` tests omitted credential/wallet/log material and safeguard labels. |
| 3 | Runtime support output lists configured sync resource bounds operators can inspect. | VERIFIED | `packages/open-bitcoin-cli/src/operator/runtime/support.rs:378` consumes `SyncResourcePressure`; `:623` tests the exact resource-pressure line. |
| 4 | Live-smoke support summaries use allowlisted fields and exclude raw peer, log, stdout, stderr, and credential material. | VERIFIED | `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs:344` builds a raw-field fixture and asserts the summary keeps `resourcePressure` while excluding raw names/values. |
| 5 | Same-datadir resume is safe after clean shutdown, unclean shutdown, mid-download interruption, mid-connect interruption, and stale in-flight work. | VERIFIED | `packages/open-bitcoin-node/src/sync/tests.rs:5506` covers all five named cases with real `FjallNodeStore::open` and `DurableSyncRuntime::open`. |
| 6 | Already connected blocks are not requested or connected again after reopen. | VERIFIED | `packages/open-bitcoin-node/src/sync/tests.rs:5633` checks sent `getdata` hashes and asserts the connected block is absent while the next missing block is requested. |
| 7 | Deterministic synthetic long-chain tests exercise resource bounds without public-network access. | VERIFIED | `packages/open-bitcoin-node/src/sync/tests.rs:5729` uses 48 synthetic blocks, manual local peers, empty DNS seeds, scripted transport, and local Fjall stores. |
| 8 | Long-chain test evidence stays within configured peer, in-flight, message, round, metrics, and log bounds. | VERIFIED | `packages/open-bitcoin-node/src/sync/tests.rs:5830` to `:5850` asserts in-flight, outbound peer, message, round, metric retention, and log retention caps. |
| 9 | Storage-first recovery guidance distinguishes schema mismatch, corruption, lock contention, low disk, and storage pressure. | VERIFIED | `packages/open-bitcoin-node/src/storage.rs:199` keeps schema/corruption/lock precedence; `:245` detects low-disk/storage-pressure phrases. |
| 10 | Low disk and storage pressure map to typed resource exhaustion with precise free-disk guidance. | VERIFIED | `packages/open-bitcoin-node/src/storage.rs:89`, `:98`, and `:110` define `FreeDisk`, map it to `ResourceExhaustion`, and provide the exact operator message. |
| 11 | Recovery guidance does not imply automatic repair, reindex, source datadir mutation, or hidden storage mutation. | VERIFIED | Code only classifies and returns guidance; docs state no automatic repair/prune/move/mutation at `docs/operator/runtime-guide.md:789`. |
| 12 | Operator docs describe bounded long sync, safe same-datadir resume, diagnosed storage/resource blockers, and deferred production-node scope. | VERIFIED | `docs/operator/runtime-guide.md:785` contains the Phase 71 proof section and opt-in/public-network boundary language. |
| 13 | Architecture docs name exact resource, restart/resume, and storage-pressure evidence contracts. | VERIFIED | `docs/architecture/status-snapshot.md:232`, `docs/architecture/operator-observability.md:69`, and `docs/architecture/storage-decision.md:43` name `SyncResourcePressure`, retention policies, `ResourceExhaustion`, and `FreeDisk`. |
| 14 | Parity docs scope Phase 71 to outbound full-sync review without broad production-node claims. | VERIFIED | `docs/parity/catalog/p2p.md:203` lists deferred inbound/relay/wallet/migration/packaging/GUI/dashboard/broad readiness surfaces; `docs/parity/catalog/chainstate.md:60` keeps the storage-pressure claim scoped. |
| 15 | Default verification runs the deterministic Phase 71 checker after Phase 70 and remains public-network-free. | VERIFIED | `scripts/verify.sh` runs `check-phase70-reorg-recovery.ts` then `check-phase71-resource-restart.ts`; grep found no `run-live-mainnet-smoke`, `--manual-peer`, `--restart-after-progress`, `systemctl`, `launchctl`, or `openbitcoinsync=mainnet-ibd` in `scripts/verify.sh`. |

**Score:** 15/15 truths verified

## Required Artifacts

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `packages/open-bitcoin-cli/src/operator/runtime/support.rs` | Runtime support rendering tests for `SyncResourcePressure` fields | VERIFIED | Exists, substantive, imports `SyncResourcePressure`, and tests `phase71_runtime_support_resource_pressure_lists_all_configured_bounds`. |
| `packages/open-bitcoin-cli/src/operator/support.rs` | Support bundle redaction and compact evidence tests | VERIFIED | Exists, substantive, tests exact omitted materials and safeguards in `phase71_support_redaction_names_compact_evidence_bounds`. |
| `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs` | Allowlisted live-smoke support summary tests | VERIFIED | Exists, substantive, tests raw peer/log/stdout/stderr/credential exclusion. |
| `packages/open-bitcoin-node/src/sync/tests.rs` | Restart/resume matrix and synthetic long-chain tests | VERIFIED | Exists, substantive, contains both Phase 71 sync tests and real Fjall reopen/scripted transport evidence. |
| `packages/open-bitcoin-node/src/storage.rs` | Typed storage recovery action and category mapping | VERIFIED | Defines `StorageRecoveryAction::FreeDisk`, pressure-signal detection, and tests low-disk/storage-pressure mapping. |
| `packages/open-bitcoin-node/src/storage/fjall_store.rs` | Backend storage error action selection | VERIFIED | `backend_failure` routes Fjall errors through `StorageRecoveryAction::for_backend_message`. |
| `packages/open-bitcoin-node/src/sync/types/recovery.rs` | Error-detail recovery category mapping | VERIFIED | `SyncRuntimeError::Storage(error) => error.recovery_category()` and pressure-signal classification are present. |
| `packages/open-bitcoin-node/src/sync/progress.rs` | No-progress storage/resource next-action guidance | VERIFIED | `NoProgressDiagnosis::StorageOrResourceBlocked` returns storage/free-disk/bounded-resource guidance. |
| `docs/operator/runtime-guide.md` | Operator-facing Phase 71 guidance | VERIFIED | Contains Phase 71 proof section, exact test anchors, free-disk guidance, and default-verification exclusion. |
| `docs/architecture/status-snapshot.md` | Shared status contract guidance | VERIFIED | Documents resource pressure, restart/resume test anchors, and storage-pressure category/action. |
| `docs/architecture/operator-observability.md` | Metrics/log/support compactness guidance | VERIFIED | Names retention policies and compact resource vocabulary. |
| `docs/architecture/storage-decision.md` | Storage-pressure evidence contract | VERIFIED | Documents `FreeDisk` and no hidden mutation. |
| `docs/parity/catalog/p2p.md` | P2P/release-boundary scope | VERIFIED | Keeps Phase 71 outbound and opt-in, excluding production-node surfaces. |
| `docs/parity/catalog/chainstate.md` | Chainstate/storage-pressure parity scope | VERIFIED | Documents scoped `FreeDisk`/`ResourceExhaustion` claim and deferred production surfaces. |
| `scripts/check-phase71-resource-restart.ts` | Deterministic Phase 71 checker | VERIFIED | Exists, Bun/TypeScript syntax-valid, reads only repo files, and prints `validated Phase 71 resource/restart evidence`. |
| `scripts/verify.sh` | Repo-native checker wiring | VERIFIED | Runs Phase 71 checker after Phase 70 and excludes public-network/service-manager commands. |
| `docs/metrics/lines-of-code.md` | Generated LOC freshness | VERIFIED | Includes `scripts/check-phase71-resource-restart.ts`; `generate-loc-report --check` passed. |

## Key Link Verification

| From | To | Via | Status | Details |
|---|---|---|---|---|
| `support/live_smoke.rs` | `support.rs` | `collect_live_smoke_evidence` calls `live_smoke::summary` | VERIFIED | Mechanical key-link check passed. |
| `runtime/support.rs` | `status.rs` | `sync_pressure_text(value: &SyncResourcePressure)` | VERIFIED | Manual trace: `runtime/support.rs:10` imports `SyncResourcePressure`, `:331` passes `sync_pressure_text`, and `:378` consumes typed pressure. Mechanical check had an exact-pattern false negative. |
| `sync/tests.rs` | `sync.rs` | `DurableSyncRuntime::open` and scripted `sync_once` fixtures | VERIFIED | Mechanical key-link check passed; manual trace confirms repeated `DurableSyncRuntime::open` in Phase 71 tests. |
| `sync/tests.rs` | `storage/fjall_store.rs` | `FjallNodeStore` reopen on same temp datadir | VERIFIED | Manual trace: `sync/tests.rs:5512`, `:5609`, `:5622`, `:5635`, `:5650`, `:5777`, and `:5809` open/reopen the same paths. Mechanical check had an exact-pattern false negative. |
| `storage.rs` | `sync/types/recovery.rs` | `StorageError::recovery_category` feeds runtime recovery category | VERIFIED | Manual trace: `sync/types/recovery.rs:59` delegates `SyncRuntimeError::Storage(error)` to `error.recovery_category()`. Mechanical check had an escaped-pattern false negative. |
| `sync/progress.rs` | `status.rs` | `NoProgressDiagnosis::StorageOrResourceBlocked` next action | VERIFIED | Mechanical key-link check passed. |
| `check-phase71-resource-restart.ts` | `sync/tests.rs` | Checker requires exact Phase 71 test names | VERIFIED | Mechanical key-link check passed. |
| `verify.sh` | `check-phase71-resource-restart.ts` | Default deterministic verification | VERIFIED | Mechanical key-link check passed; manual order check reported Phase 70 before Phase 71. |

## Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|---|---|---|---|---|
| `runtime/support.rs` | `SyncResourcePressure` | `DurableSyncState.sync.resource_pressure` from `open-bitcoin-node/src/status.rs` | Yes | FLOWING - renderer consumes typed status, not renderer-local labels. |
| `support/live_smoke.rs` | Live-smoke summary JSON | Allowlisted fields from report JSON | Yes | FLOWING - test fixture proves allowed `resourcePressure` survives while raw fields are omitted. |
| `sync/tests.rs` | Durable summaries/resource pressure | Real `FjallNodeStore` reopen plus `DurableSyncRuntime` snapshots | Yes | FLOWING - tests persist/reopen stores and inspect runtime metadata/status. |
| `storage.rs` and `sync/types/recovery.rs` | Recovery category/action | `StorageError`/backend-message classification into `SyncRuntimeError` recovery | Yes | FLOWING - tests prove low disk/storage pressure maps to `FreeDisk`/`ResourceExhaustion`; schema/corruption/lock paths remain typed. |
| `scripts/check-phase71-resource-restart.ts` | Checker inputs | Reads plans, source, docs, and `scripts/verify.sh` | Yes | FLOWING - `bun run` exits 0 and validates required needles. |

## Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|---|---|---|---|
| Phase 71 checker syntax | `bun --check scripts/check-phase71-resource-restart.ts` | Exited 0 | PASS |
| Phase 71 checker behavior | `bun run scripts/check-phase71-resource-restart.ts` | Printed `validated Phase 71 resource/restart evidence` | PASS |
| Verify-script ordering | Node index check for Phase 70 before Phase 71 | `phase70=4064 phase71=4112` | PASS |
| Verify-script public-network exclusion | `rg` rejection for live-smoke/manual-peer/restart/service-manager/mainnet activation strings in `scripts/verify.sh` | No matches, exit 0 | PASS |
| CLI Phase 71 tests | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli phase71_ --all-features` | 3 passed | PASS |
| Node Phase 71 tests | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase71_ --all-features` | 2 passed | PASS |
| Low-disk/storage-pressure mapping | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage_recovery_category_maps_low_disk_and_storage_pressure --all-features` | 1 passed | PASS |
| Recovery category coverage | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync_recovery_category --all-features` | 6 passed | PASS |
| Phase 70 storage/resource regression | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase70_no_progress_status_projects_storage_or_resource_blocker --all-features` | 1 passed | PASS |
| LOC freshness | `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check` | LOC report is current | PASS |
| File-length gate | `bash scripts/check-file-lengths.sh` | 187 production Rust files checked, limit 628 | PASS |
| Formatting | `cargo fmt --manifest-path packages/Cargo.toml --all --check` | Exited 0 | PASS |
| Repo-native full verification | `bash scripts/verify.sh` | Completed in 36m 50.905s with exit 0 | PASS |

## Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|---|---|---|---|---|
| RES-01 | 71-01, 71-02, 71-04 | Operator can run long mainnet sync attempts with documented and tested bounds for peers, in-flight blocks, queues, caches, storage writes, logs, metrics, and support evidence. | SATISFIED | Runtime/support tests, synthetic long-chain bounds, docs, checker, and full verifier all pass. |
| RES-02 | 71-02, 71-04 | Operator can resume safely after clean shutdown, unclean shutdown, mid-download interruption, mid-connect interruption, and stale in-flight work. | SATISFIED | `phase71_same_datadir_resume_matrix_covers_clean_unclean_mid_download_mid_connect_and_stale_inflight` passes and uses real same-datadir reopen. |
| RES-03 | 71-03, 71-04 | Operator can receive typed recovery guidance for schema mismatch, corruption markers, lock contention, low disk, and storage pressure without hidden data mutation. | SATISFIED | `FreeDisk`, `ResourceExhaustion`, `for_backend_message`, shared recovery parsing, no-progress guidance, and docs are wired and tested. |
| RES-04 | 71-02, 71-04 | Operator can run deterministic synthetic long-chain tests that exercise resource bounds without requiring public-network access. | SATISFIED | `phase71_synthetic_long_chain_exercises_resource_bounds_without_public_network` passes; default `verify.sh` excludes public-network commands. |

No orphaned Phase 71 requirements were found. `.planning/REQUIREMENTS.md` maps only RES-01, RES-02, RES-03, and RES-04 to Phase 71.

## Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|---|---|---|---|---|
| None | - | - | - | No TODO/FIXME/placeholder markers found in changed files. Stub-pattern grep produced only benign format strings, local arrays, test empty-match arms, and checker success logging. |

## Human Verification Required

None. Phase 71's required behaviors are covered by deterministic tests, checker wiring, source/docs inspection, and the full repo-native verifier. Public-network long-run/UAT remains intentionally opt-in and is not a Phase 71 default-verification requirement.

## Deferred Items

No Phase 71 gaps were deferred. Later phases intentionally own broader cross-surface observability/support evidence (Phase 72), opt-in UAT command coverage and default-verification hardening (Phase 73), and final release-boundary closeout (Phase 74).

## Gaps Summary

No blocking gaps found. All roadmap success criteria and plan frontmatter must-haves are implemented, substantive, wired, and covered by deterministic verification.

Residual risk: Phase 71 proves resource and restart/resume behavior through deterministic synthetic/local fixtures. It does not prove live public-mainnet timing or real service-manager restart behavior; those remain explicit opt-in UAT surfaces by roadmap design.

---

_Verified: 2026-06-13T13:47:35Z_
_Verifier: the agent (gsd-verifier)_
