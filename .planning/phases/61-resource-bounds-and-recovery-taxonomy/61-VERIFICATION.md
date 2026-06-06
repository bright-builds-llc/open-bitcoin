---
phase: 61-resource-bounds-and-recovery-taxonomy
verified: 2026-06-06T16:24:18Z
status: passed
score: 21/21 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 61-2026-06-06T03-43-41
generated_at: 2026-06-06T16:24:18Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 61: Resource Bounds and Recovery Taxonomy Verification Report

**Phase Goal:** Operators can trust unattended sync bounds, recovery states, and next-action guidance across long runs.
**Verified:** 2026-06-06T16:24:18Z
**Status:** passed
**Re-verification:** No - initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|---|---|---|
| 1 | Operator can inspect documented bounds for outbound peers, in-flight headers or blocks, retry queues, storage writes, metrics samples, structured logs, and support evidence size. | VERIFIED | `docs/operator/runtime-guide.md:271` lists runtime bounds and `docs/operator/runtime-guide.md:289` lists all `SyncResourcePressure` fields; `scripts/check-phase61-resource-recovery-boundaries.ts:19` requires the fields. |
| 2 | Recovery handling distinguishes clean shutdown, unclean shutdown, incompatible schema, store corruption, storage lock contention, resource exhaustion, invalid peer data, public-network unreachability, and operator cancellation. | VERIFIED | `packages/open-bitcoin-node/src/status/recovery.rs:9` defines the typed enum and `as_str()` labels; storage, peer, stop, runtime, and detail mappings are in `storage.rs:91` and `sync/types/recovery.rs:10`. |
| 3 | Operator-visible errors and recovery guidance use consistent typed states across status, logs, support bundles, and docs. | VERIFIED | Status/dashboard/RPC render `SyncRecoveryCategory::as_str()` via `status/render.rs:92`, `dashboard/model.rs:132`, and `dispatch/node.rs:127`; support and docs use the same labels. |
| 4 | Extended unattended runs preserve the documented bounds without unbounded growth or silent loss of recovery evidence. | VERIFIED | `bounded_unattended_cycles_preserve_resource_pressure_and_retention` asserts pressure caps, endpoint-keyed retry state, synchronous durable writes, metrics retention, and log retention in `sync/tests.rs:3309`. |
| 5 | Shared status JSON has machine-readable `sync.recovery_category` beside human `sync.recovery_action`. | VERIFIED | `SyncStatus` includes `recovery_category` immediately before `recovery_action` in `status.rs:148`, and runtime projection sets both in `runtime_state.rs:397`. |
| 6 | Every Phase 61 recovery label serializes as stable snake_case text. | VERIFIED | `SyncRecoveryCategory` is `serde(rename_all = "snake_case")` and the label test covers all ten values in `status/recovery.rs:43`. |
| 7 | Every current `SyncStatus {` constructor sets `recovery_category`. | VERIFIED | `rg -n "SyncStatus \\{" packages/open-bitcoin-* -g '*.rs'` found all constructors; `cargo test` and `bash scripts/verify.sh` compile the full workspace with the required field. |
| 8 | Storage, peer, stop-reason, and runtime errors map into `SyncRecoveryCategory` without renderer-local string parsing. | VERIFIED | Pure mappings live in `storage.rs:91`, `storage.rs:192`, and `sync/types/recovery.rs:10`; renderers consume typed status fields. |
| 9 | Storage incompatibility and corruption outrank peer and public-network retry guidance. | VERIFIED | Durable precedence checks `metadata.maybe_last_recovery_action` before error detail, stop reason, and peer categories in `runtime_state.rs:433`; tests assert storage metadata beats peer/network detail in `sync/tests.rs:2926`. |
| 10 | Durable sync status projects `sync.recovery_category` with storage-first precedence. | VERIFIED | `recovery_category_for_durable_state` implements the ordered projection in `runtime_state.rs:433`; focused tests pass. |
| 11 | Structured sync logs include the same recovery category label as status when a category exists. | VERIFIED | `structured_log_records()` appends `recovery_category={label}` from `SyncRunSummary::recovery_category()` in `summary.rs:225`. |
| 12 | Deterministic repeated sync fixtures preserve configured resource pressure and bounded retention without public-network checks. | VERIFIED | `sync/tests.rs:3309` uses scripted transport/resolver only and asserts `max_blocks_in_flight_total`, outbound peer caps, metrics, and log retention. |
| 13 | Peer retry state is endpoint-keyed and bounded; durable storage writes have no queued write backlog. | VERIFIED | The repeated-cycle test reads durable metadata immediately after each cycle and asserts endpoint-keyed backoff length limits in `sync/tests.rs:3391` and `sync/tests.rs:3424`. |
| 14 | Opt-in live-smoke reports use the same Phase 61 recovery category labels as Rust status. | VERIFIED | `scripts/run-live-mainnet-smoke.ts:176` defines the same ten labels; `recoveryCategoryFromValue` accepts only those labels at `run-live-mainnet-smoke.ts:1302`. |
| 15 | Support evidence keeps recovery category and resource-pressure summaries compact and allowlisted. | VERIFIED | `support/live_smoke.rs:82` and `support/live_smoke.rs:95` define final-status and resource-pressure allowlists; support test asserts retained compact fields at `operator_binary.rs:1051`. |
| 16 | Support evidence excludes raw live-smoke reports, daemon tails, endpoint tables, secrets, wallet material, and unbounded samples. | VERIFIED | Recursive redaction is in `support/live_smoke.rs:308`; `operator_binary.rs:1081` asserts forbidden raw values are absent from JSON and Markdown. |
| 17 | Operator status output displays stable machine recovery category separately from human recovery action. | VERIFIED | `status/render.rs:92` prints `Sync recovery category` before `Sync recovery`; focused renderer test passed. |
| 18 | Dashboard sync rows display the same stable category label. | VERIFIED | `dashboard/model.rs:132` adds the `Recovery category` row sourced from `snapshot.sync.recovery_category`; focused dashboard test passed. |
| 19 | RPC-facing blockchain warnings include the category label when durable status has one. | VERIFIED | `durable_warnings` pushes `recovery_category={label}` in `dispatch/node.rs:127`; RPC test asserted the warning. |
| 20 | Operator docs list active resource bounds and all recovery labels, with repo-local Cargo and Bazel status/support commands. | VERIFIED | Runtime guide lists labels at `docs/operator/runtime-guide.md:263`, bounds at `docs/operator/runtime-guide.md:271`, status commands at `docs/operator/runtime-guide.md:652`, and support commands at `docs/operator/runtime-guide.md:660`. |
| 21 | `scripts/verify.sh` remains deterministic and does not run live-smoke or public-network commands. | VERIFIED | `scripts/verify.sh:118` runs the deterministic Phase 61 checker; negative grep for `run-live-mainnet-smoke`, `--manual-peer`, and `--restart-after-progress` passed. |

**Score:** 21/21 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `packages/open-bitcoin-node/src/status/recovery.rs` | Shared recovery enum and labels | VERIFIED | Exists, substantive, exported, tested. |
| `packages/open-bitcoin-node/src/status.rs` | `SyncStatus.recovery_category` field | VERIFIED | Field exists and is exported through status contract. |
| `packages/open-bitcoin-node/src/storage.rs` | Storage recovery category mapping | VERIFIED | Storage actions/errors map to typed categories. |
| `packages/open-bitcoin-node/src/sync/types/recovery.rs` | Peer, stop, runtime, detail mapping helpers | VERIFIED | Pure helper module with tests and parity breadcrumbs. |
| `packages/open-bitcoin-node/src/sync/types/summary.rs` | Summary status/log recovery projection | VERIFIED | Status and structured logs carry category labels. |
| `packages/open-bitcoin-node/src/sync/runtime_state.rs` | Durable storage-first recovery projection | VERIFIED | Uses runtime metadata, error detail, stop reason, peer, and shutdown precedence. |
| `packages/open-bitcoin-node/src/sync/tests.rs` | Deterministic recovery/resource tests | VERIFIED | Covers precedence, clean/unclean shutdown, and repeated-cycle bounds. |
| `scripts/run-live-mainnet-smoke.ts` | Opt-in report category/resource summaries | VERIFIED | Same label union and compact resource summary parser. |
| `scripts/test-run-live-mainnet-smoke.sh` | Deterministic live-smoke fixtures | VERIFIED | Passed in fresh verification. |
| `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs` | Support allowlist summary extraction | VERIFIED | Allows only compact recovery/resource fields and redacts recursively. |
| `packages/open-bitcoin-cli/src/operator/support/render.rs` | Support Markdown labels | VERIFIED | Renders `Recovery category` and `Resource pressure`. |
| `packages/open-bitcoin-cli/tests/operator_binary.rs` | Support redaction regression | VERIFIED | Passed focused support bundle test. |
| `packages/open-bitcoin-cli/src/operator/status/render.rs` | Human status recovery category rendering | VERIFIED | Passed focused status renderer test. |
| `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` | Dashboard recovery category row | VERIFIED | Passed focused dashboard test. |
| `packages/open-bitcoin-rpc/src/dispatch/node.rs` | RPC warning integration | VERIFIED | Passed focused RPC test. |
| `docs/operator/runtime-guide.md` | Operator docs for labels, bounds, commands | VERIFIED | Guarded by Phase 61 checker. |
| `docs/architecture/status-snapshot.md` | Shared status contract docs | VERIFIED | Lists recovery category and resource pressure semantics. |
| `docs/architecture/operator-observability.md` | Observability bound docs | VERIFIED | Documents bounded samples and exact RR-01 mechanism strings. |
| `scripts/check-phase61-resource-recovery-boundaries.ts` | Deterministic boundary checker | VERIFIED | Passed and is wired into `scripts/verify.sh`. |
| `docs/parity/source-breadcrumbs.json` | New Rust source breadcrumb coverage | VERIFIED | Contains both new Rust modules; breadcrumb checker passed. |

### Key Link Verification

| From | To | Via | Status | Details |
|---|---|---|---|---|
| `status.rs` | `status/recovery.rs` | module export | VERIFIED | `pub use recovery::SyncRecoveryCategory` found. |
| `lib.rs` | `SyncRecoveryCategory` | crate re-export | VERIFIED | Public node crate re-export found. |
| `sync/types.rs` | `sync/types/recovery.rs` | child module | VERIFIED | `mod recovery` found and used by summary/runtime. |
| `storage.rs` | `SyncRecoveryCategory` | storage error/action mapping | VERIFIED | `recovery_category` mappings found. |
| `runtime_state.rs` | `RuntimeMetadata.maybe_last_recovery_action` | storage-first projection | VERIFIED | Precedence implementation found. |
| `summary.rs` | structured log message | `recovery_category=<label>` | VERIFIED | Structured log format found. |
| `sync/tests.rs` | RR-01 resource bounds | repeated-cycle assertions | VERIFIED | Exact endpoint-keyed and no-backlog assertions found. |
| `run-live-mainnet-smoke.ts` | `support/live_smoke.rs` | schema v2 `final_status` fields | VERIFIED | `recoveryCategory` and `resourcePressure` link found. |
| `support/render.rs` | `support-evidence.md` | Markdown labels | VERIFIED | `Recovery category` and `Resource pressure` labels found. |
| `status/render.rs` | `SyncStatus.recovery_category` | snapshot rendering | VERIFIED | Human output reads `snapshot.sync.recovery_category`. |
| `dispatch/node.rs` | `getblockchaininfo` warnings | durable warnings | VERIFIED | `recovery_category=` warning found. |
| `verify.sh` | Phase 61 checker | Bun deterministic checker | VERIFIED | `bun run scripts/check-phase61-resource-recovery-boundaries.ts` found. |
| `runtime-guide.md` | repo-local Cargo/Bazel commands | operator copy-paste commands | VERIFIED | Required Cargo and Bazel command forms found. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
|---|---|---|---|---|
| `SyncStatus` / durable runtime | `sync.recovery_category` | `RuntimeMetadata.maybe_last_recovery_action`, `maybe_last_error`, `SyncStopReason`, peer outcomes, clean-shutdown metadata | Yes - typed mappings and runtime metadata drive projection | VERIFIED |
| Structured logs | `recovery_category=<label>` | `SyncRunSummary::recovery_category()` | Yes - stop reason and latest peer failure determine label, or unavailable explicitly | VERIFIED |
| Status and dashboard renderers | `snapshot.sync.recovery_category` | Shared status snapshot from durable or RPC-derived status | Yes - field is read directly and rendered separately from action text | VERIFIED |
| RPC `getblockchaininfo` warnings | `durable_sync_state.sync.recovery_category` | Durable sync state from node store/runtime | Yes - available category generates `recovery_category=<label>` warning | VERIFIED |
| Live-smoke report | `final_status.recoveryCategory`, `resourcePressure` | Parsed `sync.recovery_category.value` and `sync.resource_pressure.value` from status JSON | Yes - parser accepts typed labels and converts snake_case pressure fields | VERIFIED |
| Support bundle summary | `live_smoke.summary.finalStatus.recoveryCategory/resourcePressure` | Allowlisted live-smoke schema v2 report fields | Yes - compact fields retained; raw material omitted and redacted | VERIFIED |
| Phase 61 checker | docs and verify-script boundary strings | Runtime guide, architecture docs, `scripts/verify.sh` | Yes - checker fails on missing labels, bounds, commands, or public-network exclusion drift | VERIFIED |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|---|---|---|---|
| Phase 61 boundary checker enforces labels, bounds, commands, and verify exclusions | `bun run scripts/check-phase61-resource-recovery-boundaries.ts` | Printed `validated Phase 61 resource/recovery boundaries` | PASS |
| Live-smoke fixture labels and resource summaries are deterministic | `bash scripts/test-run-live-mainnet-smoke.sh` | Exit 0 | PASS |
| Default verification excludes public-network smoke commands | `bash -c 'if rg -n "run-live-mainnet-smoke|--manual-peer|--restart-after-progress" scripts/verify.sh; then exit 1; fi'` | No matches, exit 0 | PASS |
| Recovery category mappings and label tests pass | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node recovery_category --all-features` | 9 passed | PASS |
| Repeated-cycle resource bounds pass | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node bounded_unattended_cycles_preserve_resource_pressure_and_retention --all-features` | 1 passed | PASS |
| Support bundle keeps compact recovery/resource evidence and excludes raw material | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli open_bitcoin_support_bundle_summarizes_phase61_resource_recovery_evidence --all-features` | 1 passed | PASS |
| Human status renderer exposes category separately | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli status_render_includes_sync_progress_and_peer_evidence --all-features` | 1 passed | PASS |
| Dashboard exposes category row | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli dashboard_sections_surface_sync_progress_and_peer_counts --all-features` | 1 passed | PASS |
| RPC warning includes recovery category | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc get_blockchain_info --all-features` | 1 targeted library test passed | PASS |
| Aggregate repo-native deterministic verification | `bash scripts/verify.sh` | Passed in 2m 14.952s; included Cargo fmt/clippy/build/test, benchmark smoke/report validation, Bazel smoke, coverage | PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|---|---|---|---|---|
| RR-01 | 61-03, 61-04, 61-06 | Unattended sync enforces documented bounds for peers, in-flight work, retry queues, storage writes, metrics, logs, support evidence | SATISFIED | Runtime docs, repeated-cycle test, support allowlist, metrics/log retention assertions, and Phase 61 checker all passed. |
| RR-02 | 61-01, 61-02, 61-03, 61-04, 61-05, 61-06 | Recovery handling distinguishes all Phase 61 categories | SATISFIED | Shared enum has all ten labels; storage/sync/runtime/live-smoke mappings and tests cover them. |
| RR-04 | 61-01, 61-02, 61-03, 61-04, 61-05, 61-06 | Operator-visible errors and recovery guidance stay typed, actionable, and consistent across status, logs, support bundles, docs | SATISFIED | Status, dashboard, RPC warnings, structured logs, support summaries, live-smoke reports, and docs all use shared labels and keep action text separate. |

No orphaned Phase 61 requirements were found in `.planning/REQUIREMENTS.md`; RR-03 is explicitly mapped to Phase 64 and not part of Phase 61.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|---|---:|---|---|---|
| None | - | - | - | No blocker or warning anti-patterns found. Scan matches were lint configuration, test cleanup no-op arms, normal CLI/script logging, or parser fallback `null`/empty returns, not stubs. |

### Human Verification Required

None for the Phase 61 pass decision. Public-network live-smoke and long-run checks remain opt-in UAT and are deliberately outside `bash scripts/verify.sh`.

### Gaps Summary

No gaps found. Phase 61 achieves the goal through a typed shared recovery taxonomy, storage-first durable projection, deterministic resource-bound tests, compact support/live-smoke summaries, consistent status/dashboard/RPC/log/doc rendering, and a deterministic checker wired into default verification.

---

_Verified: 2026-06-06T16:24:18Z_
_Verifier: the agent (gsd-verifier)_
