---
phase: 69-tip-tracking-and-stay-current-operation
verified: 2026-06-12T12:10:27Z
status: passed
score: 4/4 must-haves verified
requirements: [TIP-01, TIP-02, TIP-03]
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 69-2026-06-11T15-13-14
generated_at: 2026-06-12T12:10:27Z
lifecycle_validated: true
overrides_applied: 0
re_verification: false
---

# Phase 69: Tip Tracking and Stay-Current Operation Verification Report

**Phase Goal:** Operators can understand best-known tip evidence and keep `open-bitcoind` caught up after initial sync.
**Status:** passed
**Verified:** 2026-06-12T12:10:27Z
**Re-verification:** No - initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Operator can inspect best-known mainnet tip source, height, hash, work, timestamp, freshness, and peer agreement evidence. | VERIFIED | `SyncStatus.best_known_tip` carries `BestKnownTipStatus` with source, height, hash, work, block time, observed time, freshness, and bounded peer agreement rows in `packages/open-bitcoin-node/src/status.rs`. Runtime projection fills it from the validated header store in `packages/open-bitcoin-node/src/sync/runtime_state.rs`; tests assert source/hash/work/freshness/peer agreement. |
| 2 | Status surfaces distinguish initial catch-up, current-at-best-known-tip, stale-tip, recovering, and no-progress states without renderer-specific interpretation. | VERIFIED | `StayCurrentStatus` is a shared serialized enum with all five required labels. `sync::tip::classify_stay_current` computes it from lifecycle, best tip, connected active-chain evidence, freshness, and progress; docs and checker validate the labels. |
| 3 | After catch-up, the daemon detects, validates, connects, and reports new headers and blocks as stay-current progress. | VERIFIED | `phase69_post_catch_up_new_headers_connect_and_report_stay_current_progress` exercises a post-catch-up height-2 header/block, verifies requested block data, `validated_active_chain_height = 2`, `CurrentAtBestKnownTip`, stored chainstate active tip height 2, and persisted block body. |
| 4 | Tip freshness and peer agreement evidence remain coherent across restart and peer rotation. | VERIFIED | `phase69_tip_evidence_survives_runtime_reopen` proves same-store reopen preserves and re-derives best-known tip/stay-current evidence. `phase69_peer_tip_observation_uses_peer_terminal_header_not_global_best` proves peer rows use the peer's accepted terminal header, not a copied global best tip. |

**Score:** 4/4 roadmap success criteria verified.

## Requirement Evidence

| Requirement | Status | Evidence |
| --- | --- | --- |
| TIP-01 | SATISFIED | `BestKnownTipStatus` is available in shared status JSON and docs. Runtime derives it from `header_store().best_tip()` and peer outcomes; tests cover serialization, freshness, source, hash/work, and agreement rows. |
| TIP-02 | SATISFIED | `StayCurrentStatus` provides `initial_catch_up`, `current_at_best_known_tip`, `stale_tip`, `recovering`, and `no_progress`; classifier and tests cover missing/fresh/stale/recovering, headers-only non-current, and stale-tip distinct from no-progress. |
| TIP-03 | SATISFIED | `sync_until_idle` can report `CurrentAtBestKnownTip` for fresh idle-at-tip and can process new post-catch-up headers/blocks through validation, connection, persistence, and status reporting. |

## Required Artifacts

| Artifact | Status | Details |
| --- | --- | --- |
| `packages/open-bitcoin-node/src/status.rs` | VERIFIED | Defines typed best-known tip, peer agreement, stay-current, and next-action fields with serde defaults. |
| `packages/open-bitcoin-node/src/sync/tip.rs` | VERIFIED | Pure helper module for best-tip evidence, peer agreement, freshness, current-at-tip gating, and next-action selection; includes parity breadcrumbs. |
| `packages/open-bitcoin-node/src/sync/runtime_state.rs` | VERIFIED | Projects `sync.best_known_tip`, `sync.stay_current`, and `sync.stay_current_next_action` into durable status. |
| `packages/open-bitcoin-node/src/sync/tests.rs` | VERIFIED | Contains deterministic Phase 69 coverage for serialization, peer agreement, terminal-header observation, idle current, post-catch-up progress, headers-only non-current, stale-tip, and reopen coherence. |
| `scripts/check-phase69-tip-stay-current.ts` | VERIFIED | Deterministic checker requires phase artifacts, status/runtime/test/doc evidence, and default-verification boundary guards. |
| `scripts/verify.sh` | VERIFIED | Runs `bun run scripts/check-phase69-tip-stay-current.ts` after the Phase 68 checker and contains no forbidden public-network/default service-manager commands. |
| `docs/operator/runtime-guide.md` and `docs/architecture/status-snapshot.md` | VERIFIED | Document Phase 69 fields and preserve the Phase 68 counter separation. |

`gsd-tools verify artifacts` passed for all five plans: 14/14 artifacts. `gsd-tools verify key-links` passed for all five plans: 5/5 links.

## Key Link Verification

| From | To | Status | Details |
| --- | --- | --- | --- |
| `status.rs` | `runtime_state.rs` | WIRED | Durable projection assigns Phase 69 fields. |
| `sync/tip.rs` | `runtime_state.rs` | WIRED | `classify_stay_current`, `build_best_known_tip_status`, and `stay_current_next_action` are used in durable status projection. |
| `sync.rs` | `sync/progress.rs` and `sync/tip.rs` | WIRED | Accepted `headers` messages record peer terminal tip evidence via `record_peer_terminal_tip` and `record_tip_observation`. |
| `sync/tests.rs` | runtime sync path | WIRED | Tests exercise `sync_once`, `sync_until_idle`, durable metadata load, and chainstate snapshot load. |
| `scripts/check-phase69-tip-stay-current.ts` | `scripts/verify.sh` | WIRED | Checker is called by the repo-native verifier. |

## Data-Flow Trace

| Artifact | Data Variable | Source | Produces Real Data | Status |
| --- | --- | --- | --- | --- |
| `sync.rs` | peer terminal tip fields | Accepted peer `Headers` message, validated through `receive_sync_message`, exact terminal header lookup in `HeaderStore` | Yes | FLOWING |
| `sync/progress.rs` | `maybe_tip_height/hash/work` | `record_tip_observation` and `PeerProgress::into_outcome` | Yes | FLOWING |
| `runtime_state.rs` | `best_known_tip`, `stay_current` | Durable validated `HeaderStore::best_tip`, connected active-chain progress, peer outcomes, lifecycle, fixed projection timestamp | Yes | FLOWING |
| `SyncStatus` JSON/RPC/CLI data | Phase 69 fields | Shared `RuntimeMetadata` / `DurableSyncState` serialization, plus CLI/RPC unavailable fallbacks | Yes | FLOWING |

## Commands Run

| Command | Result |
| --- | --- |
| `node ... gsd-tools.cjs roadmap get-phase 69 --raw` | Passed; confirmed Phase 69 goal, TIP-01/TIP-02/TIP-03, and 4 success criteria. |
| `node ... gsd-tools.cjs verify artifacts .../69-01..05-PLAN.md` | Passed; 14/14 artifacts present/substantive. |
| `node ... gsd-tools.cjs verify key-links .../69-01..05-PLAN.md` | Passed; 5/5 key links verified. |
| `node ... gsd-tools.cjs verify commits ec67e25 d56206b 321af94 aeb7140 ce36361` | Passed; all 5 documented implementation commits exist. |
| `bun run scripts/check-phase69-tip-stay-current.ts` | Passed; output: `validated Phase 69 tip stay-current evidence`. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase69_ --all-features` | Passed; 12 passed, 0 failed. |
| `bash scripts/verify.sh` | Passed; completed in 17m 53.844s. Includes checkers, parity breadcrumbs, panic/file-length checks, fmt, clippy, build, workspace tests, doc-tests, benchmark smoke, and Bazel smoke. |
| `git diff --check` | Passed. |

## Boundary Checks

- No external tip oracle, checkpoint shortcut, assumevalid, assumeutxo, centralized peer, or public API dependency was found in the Phase 69 data path. Best-known tip evidence is derived from the validated header store and bounded peer observations.
- Headers-only progress is not credited as current-at-tip. The classifier requires connected active-chain height/hash/work to match the fresh best-known validated tip.
- Default verification remains public-network-free and service-manager-free. `scripts/check-phase69-tip-stay-current.ts` rejects forbidden strings in `scripts/verify.sh`, and the full verifier passed.
- New `packages/open-bitcoin-node/src/sync/tip.rs` has parity breadcrumbs in source and `docs/parity/source-breadcrumbs.json`; the breadcrumb checker passed for 233 Rust files.
- Anti-pattern scan found no blocking TODO/FIXME/placeholder/stub implementations in Phase 69 code paths. The checker has a normal `console.log` success message only.

## Residual Risks

- Public-mainnet long-running stay-current UAT remains opt-in and is not part of default verification. This is consistent with the Phase 69 boundary and the later Phase 73 roadmap goal for opt-in public-mainnet UAT commands.
- Broader cross-surface support evidence across CLI, dashboard, RPC, metrics, logs, live-smoke reports, and support bundles is Phase 72 scope. Phase 69 verifies the shared status contract and deterministic docs/checker boundary.
- `69-02-SUMMARY.md` has a summary frontmatter typo (`TIP-04`, `TIP-05`) that does not match REQUIREMENTS or PLAN frontmatter. Verification used the roadmap/requirements/plan contract (`TIP-01`, `TIP-02`, `TIP-03`) and found all three satisfied.

## Gaps Summary

No blocking gaps found. Phase 69 goal achieved.

---

_Verified: 2026-06-12T12:10:27Z_
_Verifier: the agent (gsd-verifier)_
