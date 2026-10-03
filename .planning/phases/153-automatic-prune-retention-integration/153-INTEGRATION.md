---
phase: 153-automatic-prune-retention-integration
milestone: v2.4
milestone_name: Prune-Mode Product Behavior
audited: 2026-10-03
status: integration_passed
verification_basis: static_production_trace_and_recorded_scoped_runtime_evidence
native_verification_status: passed
refreshed_at: 2026-10-03T07:52:56Z
generated_by: gsd-integration-checker
lifecycle_mode: yolo
phase_lifecycle_id: 153-2026-10-03T04-01-54
scope_phases: [146, 147, 148, 149, 150, 151, 152, 153]
scores:
  requirements_wired: 17/17
  seams_connected: 20/20
  flows_traced: 10/10
  scoped_rpc_methods_consumed: 6/6
blocking_integration_gaps: []
closed_prior_gaps: [INT-01, INT-02]
advisory_debt:
  closed: 4
  retained: 3
---

# Integration Check Complete

**Result: integration_passed for the static production trace.** Both prior integration breaks are closed in the inspected working tree. All 17 canonical requirements have connected consumers, all 20 selected production seams connect, and all 10 scoped flows can be traced through their effects and observations. The final refresh records the separately completed default native verifier and root phase/lifecycle activation; this integration trace does not substitute for those gates.

Only the eight active ROADMAP phases 146–153 are included. The deliberately retained 135 historical phase directories are not extra milestone phases. The original audit covered 146–151; 152 and 153 supply the two gap closures. All eight phases now have passed reports. Phase 153's formal VERIFICATION passed 22/22 after the full native gate, and root activated PRUN-01/PRUN-02. All 17 canonical requirements are Complete. The initial report's pending-native handoff is preserved as historical context below.

The audit loaded both canonical active lesson files completely (7,188 bytes; 2,397 conservative estimated tokens), local AGENTS guidance, the Bright Builds sidecar, overrides, standards entrypoint and architecture/verification/testing/code-shape/Rust guidance. PROJECT, ROADMAP, REQUIREMENTS, the original milestone audit, scoped CONTEXT decisions, SUMMARY exports/ownership, and relevant VERIFICATION sections informed the trace. Work runs within the existing Phase 153 GSD execution context. Only this report is owned by this checker; no source change, staging, commit, Cargo or Bazel command was performed.

## Wiring Summary

**Connected:** 20 selected seams, including the key exported contracts below. **Orphaned:** zero required production exports found. **Missing:** zero required production connections found. Counts refer to the enumerated integration seams, not every public symbol in the workspace. Fixture-only injection APIs and historical helpers are not required production consumers.

| Phase | Provides | Consumed by |
| --- | --- | --- |
| 146 | Durable coins/chain-meta wallet authority; no leftover-snapshot fallback | 152 shared eligibility and both durable wallet adapters |
| 147 | `PruneMode`, `parse_prune_arg`, `plan_automatic_prune`, `plan_manual_prune`, keep/lock predicates | 150 config/manual/status; 153 automatic shell; 148 unlink/recovery |
| 148 | Paired Fjall unlink, earned have-pruned, intent recovery, deletion receipts/undo forgetting | 149 availability; 150 manual/support; 153 ordinary automatic flush |
| 149 | Limited service flags/window and earned labels | Running shared authority, inbound response planning, RPC/status evidence |
| 150 | JSONC mode, registered RPC/CLI commands, durable locks/counters, read-only dashboard/support | 153 owner/planning; RPC/CLI collectors/renderers; 151 claim corpus |
| 151 | Parity ownership/anchors, UAT package and no-claim checker | Current contributor docs and default verifier |
| 152 | `prepare_durable_wallet_rescan`, staged candidate membership/probes and handled Failed evidence | Node chunk/resume and durable RPC range/direct helper; 153 downstream regression |
| 153 | Exact `RetainedPayloadUsage`/revision, ordinary automatic owner, offline durable selection, full checkpoint carry | Existing worker Periodic/Always cycles and same paired-delete owner; downstream wallet/status/serving |

## Production Seam Matrix

Paths are repo-relative; line numbers identify the inspected connection rather than merely a file's existence.

| # | Producer → consumer → effect | Status | Evidence |
| --- | --- | --- | --- |
| 1 | JSONC integer → typed resolved mode | CONNECTED | `packages/open-bitcoin-rpc/src/config/open_bitcoin.rs:37,55`; `config/loader.rs:177,201`; `config/prune.rs:12`; `packages/open-bitcoin-chainstate/src/prune/mode.rs:34` |
| 2 | Explicit offline prune + datadir → recovered durable authority | CONNECTED | `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:361` opens the store when prune is explicit; `:306` opens `DurableSyncRuntime`; `packages/open-bitcoin-node/src/sync/open_runtime.rs:38` initializes recovery before hydration/readiness |
| 3 | Resolved network/mode → shared owner | CONNECTED | `open-bitcoind.rs:320,325`; `packages/open-bitcoin-node/src/sync/types.rs:81` maps mainnet 100000 and other scoped networks 1000; `open-bitcoind.rs:131` passes the cloned authority to RPC |
| 4 | Existing worker → ordinary Periodic/Always → automatic consumer | CONNECTED | `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs:113,126,144,158,161`; `packages/open-bitcoin-node/src/network/runtime_authority.rs:536,543` invokes `automatic_prune::flush` |
| 5 | Real store → exact retained total and active candidate sizes | CONNECTED | `packages/open-bitcoin-node/src/storage/fjall_store/payload_usage.rs:127` holds the clone-shared guard, takes one snapshot, sums all block/undo values at 134–146 and resolves current active pairs at 148–162; adapter forwards at `chainstate/fjall_store.rs:88` |
| 6 | Complete payload mutation attempts → conservative revision → reuse/coalescing | CONNECTED | `payload_usage.rs:89,108,184` serialize raw writes/removals and invalidate before effects; `storage/fjall_store/prune.rs:103` guards paired deletes; `network/runtime_authority/automatic_prune.rs:132,141,147,155,158,95` checks revisions captured with facts and invalidates failed cycles |
| 7 | Current tip/network/mode/usage/locks → existing pure automatic planner | CONNECTED | `network/runtime_authority/automatic_prune.rs:120` handles no-op gates; `:165` calls `plan_automatic_prune`; `packages/open-bitcoin-chainstate/src/prune/plan.rs:40` applies threshold, target, ordered sizes, window and locks |
| 8 | Nonempty automatic plan → existing full checkpoint policy | CONNECTED | `automatic_prune.rs:84` promotes the existing mode to Always; `chainstate/flush_lifecycle.rs:359,385,389,391` decides, persists prefix, applies deletion and completes coins/meta writes |
| 9 | Lock RPC read/replace/clear → same authority → durable current map | CONNECTED | `packages/open-bitcoin-rpc/src/context/prune.rs:32,67,75`; `network/runtime_authority/prune_flush.rs:79,85,104`; `storage/fjall_store/prune/records.rs:37,47` |
| 10 | Current locks/active chain → manual and unlink revalidation | CONNECTED | `prune_flush.rs:71` reloads durable locks inside the owner; `chainstate/flush_lifecycle/prune_apply.rs:43,51,54,57,110` checks lock/window/hash before unlink |
| 11 | Same owner → production sink → paired durable removal/have-pruned | CONNECTED | `chainstate/fjall_store.rs:130` forwards to the real Fjall sink; `storage/fjall_store/prune.rs:117,119` syncs intent then tombstones both mates, inserts have-pruned and clears intent in one SyncAll batch; both-absent returns at 113 |
| 12 | Delete receipts → cache eviction and undo forgetting even after later error | CONNECTED | `chainstate/flush_lifecycle/prune_apply.rs:87`; `chainstate.rs:423,438,441`; `network/runtime_authority/prune_flush.rs:34,38` removes reported deleted hashes for success and error |
| 13 | Current coins tip → full idle/staged checkpoint → chain metadata | CONNECTED | `chainstate/flush_lifecycle.rs:537` reads cache authority and carries best-block before flush/sync; `:546` persists active chain metadata. A newer staged tip wins; read errors propagate |
| 14 | Reopen → coins recovery/current locks → finish-or-refuse → hydration | CONNECTED | `chainstate/flush_lifecycle.rs:220,235,236,237`; `storage/fjall_store/prune.rs:154,175,184,187,192`; `sync/open_runtime.rs:38,39,40` reloads durable metadata and actual surviving undo |
| 15 | Mode → version services → limited-window/refusal/announcement rules | CONNECTED | `network/limited_serve.rs:24` updates peer services; `network/inventory.rs:319`; `network/action_translation.rs:224`; `network/announcement_transport.rs` shares the existing announcement path and limited-serving predicates |
| 16 | Durable presence/have-pruned → earned labels → status evidence | CONNECTED | `packages/open-bitcoin-rpc/src/context.rs:384,392,395,402` refreshes durable facts for each inbound plan; `network/inventory.rs:341` only labels missing validated active bodies Pruned after have-pruned; lookup failures/unknown requests retain their separate result paths |
| 17 | Registered manual CLI/RPC → pure plan → shared durable owner → returned height | CONNECTED | `packages/open-bitcoin-cli/src/operator/prune.rs:95,108`; `packages/open-bitcoin-rpc/src/dispatch.rs:43`; `dispatch/prune.rs:138,147,152,154` |
| 18 | Configured mode/completeness/counters → RPC quartet/status → CLI/dashboard/support | CONNECTED | `dispatch/node.rs:63,80,289`; `dispatch/prune/status.rs:58,74,87`; `packages/open-bitcoin-cli/src/operator/dashboard/model.rs:151`; `operator/support/render.rs:102`; `operator/support/redaction.rs:84,88` clears lock rows before support serialization |
| 19 | Durable coins + real pruned creating payload → shared stage/probe → both wallet saves/refusals | CONNECTED | `storage/fjall_store/coins.rs:115,147`; `wallet_registry/rescan.rs:98,119,137,140,159,163`; `sync/wallet_rescan.rs:107,145,152,162`; `packages/open-bitcoin-rpc/src/context/rescan.rs:108,235,245`; durable direct helper routes at `context/wallet_state.rs:222` |
| 20 | Scoped parity roots/requirements/claim corpus → checker → native contract | CONNECTED | `docs/parity/index.json`, `docs/parity/catalog/chainstate.md` Phase 152/153 sections and `docs/parity/release-readiness.md`; `scripts/verify.sh:303,304` runs Phase 151 tests and checker. Plan 04 docs describe implemented behavior with full verification still pending |

The accounting total includes recent/protected and nonactive values, while candidates refer only to current active height/hash positions. Either present mate contributes its actual value bytes; both-absent pairs are omitted. Logical live-value lengths are not physical database allocation or an immediate reclamation guarantee. First eligible activity and Always measure immediately; changed Periodic activity coalesces for 60 seconds without applying cached candidates. Completed facts are reusable only with equal reusable revision, tip, mode and locks. Production lock publication and planning/unlink hold the same authority mutex; direct low-level store writes remain outside that operator contract.

## API Coverage and Auth Protection

**Consumed:** six scoped RPC methods. **Orphaned:** zero. The four mutation/lock methods `pruneblockchain`, `listprunelocks`, `setprunelock`, and `clearprunelock` have explicit CLI request mappings in `operator/prune.rs:108`. `getblockchaininfo` and `openbitcoinnetworkstatus` feed ordinary status collection/projection and read-only rendering; their prune projections are used at `dispatch/node.rs:63,289`. The durable wallet rescan RPC is an additional downstream consumer, not a newly added Phase 153 method.

**Protected:** all six methods share the HTTP authentication boundary; **unprotected scoped methods:** zero found. `packages/open-bitcoin-rpc/src/http.rs:136` rejects unauthenticated requests before parsing/dispatch. `:277` serializes context dispatch. No separate scoped HTTP endpoint bypass or unauthenticated prune handler was found. This is source tracing, not a new HTTP penetration test.

## E2E Flows

**Complete:** ten production flows traced. **Broken:** zero within the locked scope. Complete means wiring and effects traced, with recorded behavioral evidence where listed; it does not mean ten newly executed live tests.

| Flow | Result | Decisive connection/evidence |
| --- | --- | --- |
| Config → resolved mode/network → limited advertisement → status/dashboard | COMPLETE | Seams 1–3, 15 and 18; existing Phase 149/150 verification and Phase 153 offline controls |
| Legal manual CLI → registered RPC → paired delete → cache/status/reopen | COMPLETE | Seams 10–14 and 17; actual Fjall overrides avoid the generic default no-op |
| Disabled/keep-window request → typed refusal without deletion | COMPLETE | Manual dispatch refusal at `dispatch/prune.rs:106,147`; automatic cheap gates at `automatic_prune.rs:120`; focused mode/window regressions |
| Named lock CRUD → durable map/reopen → subsequent protected pruning | COMPLETE | Seams 9–10 and 14; current-lock manual/stale-slice and serialized-publication tests |
| Interrupted prune → finish or Repair refusal before readiness | COMPLETE | Seam 14; actual-store intent/current-lock recovery test, plus existing hash/window/mate/coins controls |
| Historical or removed-body request → no bytes → honest label/refusal | COMPLETE | Seams 12, 15–16; actual deleted-body lookup and in-memory NotFound/Pruned regression |
| Committed delete counts → read surfaces → sanitized support JSON/Markdown | COMPLETE | Seams 11, 18; counts earned by real deletion; separate-summary interruption caveat remains |
| Parity ownership/anchors → overclaim checker → default verifier | COMPLETE WIRING | Seam 20; native execution/final lifecycle separately passed in the final refresh |
| Automatic legal target → ordinary ongoing retention/full checkpoint → reopen | COMPLETE | Seams 2–14; real legal-target production scenario described below closes INT-01 |
| Actual prune → midrange/chunk/resume full wallet replacement → creating-payload eligibility | COMPLETE | Seam 19; both production durable adapters refuse older creating-height absence and preserve durable wallet/checkpoint; closes INT-02 |

## Prior Blocker Closure and Runtime Evidence Limits

**INT-01 — closed in the static trace, scoped runtime evidence recorded.** The former `PrunePlan::default()` ordinary-flush break now calls `automatic_prune::flush` at `runtime_authority.rs:543`; its production planner caller is `automatic_prune.rs:165`. Explicit offline prune configuration with a datadir selects the recovered durable runtime at `open-bitcoind.rs:361,306`, installs network/mode before workers, and retains disabled sync/listener controls. The actual worker uses the same `flush_cycle` helper exercised by the genuine scenario. It adds no second deletion worker or store.

`packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs:123` constructs a production durable owner and same-authority RPC, succeeds through both wallet adapters with retained creating payloads, then drives ordinary Periodic at 197 despite a future coins-write deadline. Real paired deletion, cache/undo absence, have-pruned, counts and protected survivors are asserted at 205–233. The first durable checkpoint and production reopen at 282 are actual; the poisoned leftover snapshot at 254 is ignored. This test is registered in the daemon tests and is recorded passed in 153-03-SUMMARY.

| Measured fact | Recorded result |
| --- | --- |
| Legal automatic target | 550 MiB = 576,716,800 bytes |
| Initial logical retained block + encoded undo bytes | 578,359,864 |
| Nonactive codec-valid pairs | 235; 578,358,970 total bytes |
| Reused encoded nonactive body / undo lengths | 2,461,034 / 68 bytes |
| Six small active pairs | 894 bytes |
| Initial ordinary deletes | Heights 1, 500, 713; 447 bytes |
| Protected active survivors | 510 under lock 520 plus ten-block buffer; 714 and 1001 in the recent window |
| Retained logical bytes after initial deletion | 578,359,417; still above target by design |
| Initial earned support | One batch, three heights, last height 713; have-pruned true |
| Replenished same-tip pair / later committed deletion | Height 1; final two batches and four height deletions |

`automatic_prune/fixtures.rs:156` writes and independently sums actual encoded values; no lowered legal target or synthetic production accounting override is used. Bulk pairs are nonactive and only small active pairs are eligible. The target is therefore intentionally unreachable: retained protected/nonactive bytes never permit protected deletion. Fixtures are sparse and codec-valid, not a consensus-validated continuous chain, public full-sync proof, physical allocation cap, or total Fjall memory measurement.

The later error stage is intentionally narrower than the first production checkpoint: `automatic_prune.rs:295` uses `MetadataFaultStore`; `automatic_prune/fixtures.rs:239` builds a fixture owner with default `MemoryCoinsView` from durable snapshot truth. Its accounting/revision/locks/payload writes/paired unlink/counters delegate to real Fjall (`:316`), while `:329` injects only chain-metadata persistence refusal. The actual replenished pair is removed before that injected effect; cache/undo receipts and retry count stability are asserted at `automatic_prune.rs:303,307,322`. Final reopen is again a production owner at 333. This is not evidence of an actual hardware fault or a second full durable-coins error checkpoint. Unavailable serving completion is injected separately at 423; real removed-body absence and Unknown request behavior are actual. No network socket is used.

The production current-best carry at `flush_lifecycle.rs:537` repairs the meaningful genuine-scenario RED (`batch_write requires a new best-block`) and preserves staged-tip priority/read-error refusal. The existing `PairedDeleteOutcome` is publicly re-exported from `storage/fjall_store.rs` so a dependent public `FlushPersistSink` implementation can name its return type; `automatic_prune/fixtures.rs:277,352` consumes that contract. Neither fix creates a second storage authority.

**INT-02 — closed by Phase 152 and preserved after automatic deletion.** Shared `wallet_registry/rescan.rs:119` stages pure selection then probes the union of requested heights and every selected coin's creating height before persistence. Node `sync/wallet_rescan.rs:152` and durable RPC `context/rescan.rs:235` both handle refusal, persist safe Failed evidence for identified jobs and save only the prepared wallet on success. Durable direct helper delegates to that RPC range path. `storage/fjall_store/coins.rs:118` guards InterruptedTwoHeads before wallet-authority reads and never consults leftover snapshot blobs. Phase 152's passed native report records real paired-prune/reopen/chunk controls; Phase 153's actual automatic-delete test checks both adapters at `automatic_prune.rs:482` and preserved wallet/Failed/checkpoint facts through 539.

Presence probes and wallet save are explicitly separate effects (`wallet_registry/rescan.rs:97`), without a prune/save transaction. This report verifies completed-prune and fresh resume/range admission. Concurrent pruning after a successful probe and before save remains a documented boundary, including when driven by the ordinary worker; the static trace does not establish atomic presence-at-save. Phase 152 VERIFICATION's statement at line 137 that automatic flush still passes an empty plan is historical and superseded by Phase 153; it is not current wiring evidence.

## Requirements Integration Map

Ownership is taken from current ROADMAP/REQUIREMENTS, not historical completing-summary arrays. WIRED is an integration disposition; it does not activate a Pending requirement.

| Requirement | Canonical owner | Integration path | Status | Issue/qualification |
| --- | --- | --- | --- | --- |
| SNAP-01 | 152 | 146 durable authority → 152 shared stage/probe → node/RPC save/refuse → 148/153 actual deletion/reopen | WIRED | INT-02 closed; nontransactional concurrency boundary above |
| PRUN-01 | 153 | 147 typed parser → 150 JSONC → daemon durable mode → 153 ordinary owner | WIRED | Canonical Complete after root native/lifecycle activation |
| PRUN-02 | 153 | Actual usage/network/tip/locks → 147 automatic planner → 148 paired owner → full checkpoint | WIRED | INT-01 closed; legal soft target may remain unreachable; native passed |
| PRUN-03 | 147 | Manual CLI/RPC → pure legal-height refusal → same owner | WIRED | — |
| LOCK-01 | 147 | 150 durable locks → serialized 153 planner/current manual recheck → 148 unlink/recovery predicates | WIRED | — |
| UNLK-01 | 148 | Pure plan → production Fjall sink → paired SyncAll tombstones → receipts | WIRED | Generic sink default debt retained; production overrides connected |
| UNLK-02 | 148 | Committed real mate deletion → same-batch have-pruned → 149 serving/150 evidence | WIRED | No marker from mode/empty/both-absent alone |
| UNLK-03 | 148 | Daemon store → initialize/recovery/current locks → finish or refuse before readiness | WIRED | Stale-meta Repair refusal retained within contract |
| SERV-01 | 149 | Resolved mode → authority peer services → version message | WIRED | Defaults remain explicit and scoped |
| SERV-02 | 149 | Active tip/body request → limited predicate → NotFound/disconnect/announcement suppression | WIRED | Existing download permission behavior retained |
| SERV-03 | 149 | Actual unlink → cache eviction/durable lookup → no body/refusal | WIRED | — |
| LABL-01 | 149 | Durable have-pruned + missing active body → Pruned evidence → status/RPC; Unknown/Unavailable separate | WIRED | Have-pruned read failure conservatively supplies false; no fabricated label |
| OPER-01 | 150 | Shared configured mode/actual completeness → RPC quartet/status → CLI/dashboard | WIRED | Target projection and earned availability remain distinct |
| OPER-02 | 150 | CLI request → registered authenticated RPC → legal plan → same durable owner/result | WIRED | — |
| OPER-03 | 150 | Real deletion receipts → durable summary → status → redacted JSON/Markdown | WIRED | Separate-summary interruption undercount retained |
| LOCK-02 | 150 | CLI list/set/clear → registered authenticated RPC → authority-owned durable map → status/reopen | WIRED | — |
| GRD-01 | 151 | Canonical parity roots/pinned anchors/current docs → no-claim checker → verify.sh | WIRED | Current default native completion passed |

**Requirements with no cross-phase wiring:** none. Phase 146 has historical foundational scope and no current exclusive requirement; this does not orphan SNAP-01. Phase 147's historical PRUN-01/02 completion arrays do not replace their current Phase 153 ownership.

## Seven Original Advisory Items

Four are closed on the affected durable paths; three remain advisory. None is silently removed from the original audit.

| Original item | Current disposition | Evidence and affected requirements |
| --- | --- | --- |
| 146 CR-01: direct wallet authority admits InterruptedTwoHeads | CLOSED for the named gap | `storage/fjall_store/coins.rs:118,147` guards direct loading; shared durable preparation calls the same guard; node physical marker regression recorded in 152-VERIFICATION truth 7. SNAP-01 |
| 146 WR-02: payload-probe errors escape without durable Failed evidence | CLOSED for identified/resumable jobs | Node `sync/wallet_rescan.rs:152`; RPC `context/rescan.rs:235`; Failed-save errors propagate. Phase 152 verifies actual checkpoint preservation and sanitized failure categories. Missing unresolvable target/job identity is refused without fabricated progress/job. SNAP-01 |
| 146 IN-02: duplicate node/RPC partial filtering may drift | CLOSED on the durable eligibility contract | Both durable adapters use `prepare_durable_wallet_rescan` and its single shared partial filter at `wallet_registry/rescan.rs:180`. RPC's older `partial_chainstate_snapshot` remains for the transient Local branch at `context/rescan.rs:253,350`; it is not a parallel durable prune gate. SNAP-01 |
| 148 WR-01: synced prune intent may see stale durable chain metadata after crash | RETAINED, permitted finish-or-refuse | `storage/fjall_store/prune.rs:154,175,192` validates durable hash/window/coins tip and refuses Repair on disagreement. New successful checkpoints and current-best carry do not prove all crash windows removed. UNLK-03 |
| 148 IN-01: default paired-unlink sink silently reports AlreadyAbsent | RETAINED generic extension debt | `chainstate/flush_lifecycle.rs:125` still returns AlreadyAbsent by default. Real Fjall overrides forward at `chainstate/fjall_store.rs:130` and `flush_lifecycle.rs` Fjall implementation. Public outcome re-export fixes nameability, not this default. UNLK-01 |
| 150: support counters may undercount an unlink interrupted before summary write | RETAINED, no fabricated successes | Unlink commits at `storage/fjall_store/prune.rs:119`; receipt loop records summary later at `chainstate/flush_lifecycle/prune_apply.rs:90`; summary SyncAll is separate at `prune/records.rs:102`. AlreadyAbsent retry adds no count. New ordinary/error-retry tests prove earned counts in their observed stages, not crash-atomic summary accounting. OPER-03 |
| 150: offline daemon authority is transient even with retained datadir | CLOSED for explicit prune + datadir | `open-bitcoind.rs:364` now gates transient selection on Disabled too; actual explicit-mode, disabled-default, disabled worker/listener and absent-datadir controls at `tests/automatic_prune.rs:20,52,66,106`. PRUN-01/02, OPER-02 |

## Initial Verification Handoff — Historical Checkpoint

This checker executed no native suite and claims no fresh public-network or CI success. It reviewed recorded scoped passes in 153-01 through 153-03 SUMMARYs: final accounting 16/16 and storage 96/96; final ordinary automatic 25, flush/recovery 42, daemon cycle/worker 4, RPC prune 33, node wallet 15, RPC wallet 19; genuine legal-target scenario; affected strict Clippy, formatting, breadcrumbs, file lengths and whitespace checks. Those focused suites are runtime evidence, not substitutes for the default native gate. Source REVIEW is clean; SECURITY's final write and the default verifier/root lifecycle closeout were pending at this audit checkpoint.

Root owns native completion, final VERIFICATION, requirement activation, Plan 04 SUMMARY, aggregate milestone audit, commits/push and archival decisions. No blocking integration gap was found. The three retained advisory items, probe/save concurrency boundary and honest fixture limits remain part of that handoff. Archive serving, assumeutxo/second chainstate, BIP37, temporary IBD target, public defaults, unattended public-mainnet sync and production readiness/funds claims remain excluded.

## Final Gate Refresh

The full/default native timing record `12e39330-617d-4761-936e-94d0adca2e44` confirms actual success/exit 0, 2026-10-03T06:42:51.983Z–07:28:40.830Z, outer 2,748,847ms and console inner 2,747,953ms (45m47.953s). Workspace/doctests report 3,114 passes; the genuine fixture is `ok` in a 42/42 daemon suite with zero ignored/filtered. The one existing explicitly opt-in public-network smoke is ignored. Benchmark smoke/report, all six Bazel smoke targets/provenance and pure-core coverage passed. This checker read the final log/timing/report; it ran no Cargo/Bazel/test suite.

[153-VERIFICATION.md](153-VERIFICATION.md) passed 22/22 and originating lifecycle validation passed with required plans/verification. [153-03-SUMMARY.md](153-03-SUMMARY.md) now owns PRUN-01/PRUN-02 completion; [153-04-SUMMARY.md](153-04-SUMMARY.md) records final current-documentation activation. Canonical tables/checkmarks are 17/17 Complete; STATE records 8/8 phases and 34/34 plans. REVIEW is clean across 26 files and SECURITY verifies 14 closed/zero open with native passed. No source change followed the passed source gate; subsequent work reconciled documents/metadata. The exact Phase 127 `tests` directory exclusion was reviewed with 17 passing positive/negative controls, retaining production `tests_support/fixtures.rs` rejection. Three advisory items and the nonatomic wallet concurrency boundary remain unchanged. No public-network run, fresh CI result, archival or readiness claim is made.
