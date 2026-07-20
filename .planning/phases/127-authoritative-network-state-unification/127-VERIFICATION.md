---
phase: 127-authoritative-network-state-unification
verified: 2026-07-20T01:39:32Z
status: passed
score: 11/11 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 127-2026-07-19T15-09-40
generated_at: 2026-07-20T01:39:32Z
lifecycle_validated: true
overrides_applied: 0
requirements_verified:
  - BSRV-03
  - BSRV-04
  - OBS-02
  - OBS-04
---

# Phase 127: Authoritative Network State Unification Verification Report

**Phase Goal:** Unify the authoritative network, chainstate, durable block, and evidence sources used by durable sync, inbound serving, RPC, and operator surfaces.
**Verified:** 2026-07-20T01:39:32Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Durable sync, inbound serving, RPC, and operator projections use one authoritative `ManagedPeerNetwork`/chainstate in production composition. | ✓ VERIFIED | `open-bitcoind` creates the authority before the RPC context and passes the same `ManagedNetworkHandle` into the context and durable sync worker (`packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:66-81`, `:211-230`). The production-composition black-box test passed. |
| 2 | Inbound full, witness, and compact block serving reads validated durable blocks while retaining request bounds, backpressure, and peer cleanup. | ✓ VERIFIED | Policy gating produces an owned durable serve intent; `InboundWireResponsePlan::resolve` performs request-scoped `FjallNodeStore::load_block` and maps missing/corrupt/backend-failed data to unavailable (`packages/open-bitcoin-rpc/src/context.rs:92-151`). Four restart/error durable-serving tests passed. |
| 3 | RPC, CLI, dashboard, metrics/log, and support projections consume the authoritative runtime without leaking sensitive material. | ✓ VERIFIED | RPC takes one owned operator snapshot (`packages/open-bitcoin-rpc/src/context/inbound_status.rs:62-71`); CLI/dashboard/support collect the aggregate RPC status (`packages/open-bitcoin-cli/src/operator/status.rs:172-232`); support applies production redaction before writing both formats (`packages/open-bitcoin-cli/src/operator/support.rs:91`, `support/redaction.rs:67`). |
| 4 | Deterministic integration tests and source guards fail when authority, durable data flow, or operator projection diverges. | ✓ VERIFIED | The Phase 127 checker passed, and its semantic suite passed 15/15 tests: one intact corpus plus fourteen independent authority, durable-read, and projection mutations. The production-composition loopback test also passed. |
| 5 | The shared authority exposes typed operations and owned snapshots without leaking a mutex guard or mutable network reference. | ✓ VERIFIED | `ManagedNetworkHandle` keeps its `Arc<Mutex<...>>` private and funnels access through private typed `read`/`mutate` helpers (`packages/open-bitcoin-node/src/network/runtime_authority.rs:84-151`); `operator_snapshot` returns an owned aggregate at `:192`. |
| 6 | Authority poisoning and operation failures fail closed, and effectful storage, socket, logging, serialization, and rendering work does not occur while the authority lock is held. | ✓ VERIFIED | The authority converts poison/operation failures to `ManagedNetworkAuthorityError`; the inbound plan is created under the short context guard, resolved only after guard release, then completed after reacquisition (`packages/open-bitcoin-rpc/src/context.rs:235-254`). |
| 7 | Fjall reads and socket writes occur outside the authority guard, with served evidence recorded only after a successful write. | ✓ VERIFIED | The resolved response is written before `acknowledge_inbound_response_write`; only `WriteWireMessageOutcome::Written` applies completion (`packages/open-bitcoin-rpc/src/inbound_listener.rs:546-588`, `:606-620`). |
| 8 | Frozen RPC/operator schemas and dashboard/support labels remain stable, and unavailable authority data is shown as unavailable instead of stale or fabricated. | ✓ VERIFIED | Existing response types are reused by dispatch; checker semantic mutations reject defaulted/dead-anchor fields. The actual support test seeds every forbidden material class, executes production collection/writing, reads `support-evidence.json` and `.md`, and asserts raw values are absent and redaction sentinels are present (`packages/open-bitcoin-cli/src/operator/support/tests.rs:1096-1187`). |
| 9 | Block-serving evidence remains bounded to the audited Knots parity claim and does not overclaim Phase 128/129 work. | ✓ VERIFIED | The parity entry anchors `NodeContext`, RPC resolution, `ProcessGetData`, validation, and block storage (`docs/parity/index.json:2952-2989`); the catalog explicitly defers production compact announcement transport to Phase 128 and aggregate reconciliation/archive routing to Phase 129 (`docs/parity/catalog/p2p.md:1501-1508`). |
| 10 | The four review closures are real: live durable metadata, complete Knots queue semantics including unknown inventory, 15/15 semantic checks, and actual JSON/Markdown support redaction. | ✓ VERIFIED | WR-01: each `getblockchaininfo` request calls `current_durable_sync_state`, which calls `load_runtime_metadata` (`context/network.rs:270-280`, `dispatch/node.rs:39-47`). WR-02: the queue drains leading transactions, processes one following item, silently consumes `InventoryType::Unknown`, and then emits accumulated misses (`network/inventory.rs:100-171`), matching Knots `ProcessGetData` (`bitcoin-knots/src/net_processing.cpp:2426-2480`). WR-03: 15/15 checker tests passed. WR-04: the production bundle test reads and checks both output files. |
| 11 | The final verifier wiring, parity breadcrumbs, lifecycle provenance, and generated LOC evidence are current. | ✓ VERIFIED | `scripts/verify.sh` invokes the Phase 127 semantic tests and checker before Phase 117 (`scripts/verify.sh:554-557`); parity breadcrumbs verified for 384 Rust files; the LOC `--check` reported current; lifecycle validation without the not-yet-written verification returned `valid`. |

**Score:** 11/11 truths verified

### Review Closure Audit

| Review item | Closure evidence | Status |
| --- | --- | --- |
| WR-01 — live durable metadata | `ManagedRpcContext::current_durable_sync_state` calls the Fjall metadata source on every invocation; `getblockchaininfo` invokes it for every request. The production-composition test pre-seeds stale metadata, syncs, and observes the current value through the pre-existing RPC context. | ✓ CLOSED |
| WR-02 — full Knots `getdata` queue semantics | Open Bitcoin drains leading transaction inventory, handles one following inventory item, silently consumes unknown inventory, emits accumulated transaction misses, and repeats. The loopback test covers tx/block permutations, multiple blocks, unknown+tx, and missing+unknown+tx with a ping barrier. | ✓ CLOSED |
| WR-03 — semantic mutation strength | `bun test scripts/check-phase127-authoritative-network-state-unification.test.ts` passed 15/15: one intact case and fourteen independently adversarial mutations. No tests are disabled or ignored. | ✓ CLOSED |
| WR-04 — actual support JSON and Markdown redaction | The test drives `collect_status_snapshot` and `execute_support_command`, reads the generated JSON and Markdown from disk, and checks every raw endpoint, permission, credential, transaction, and dynamic-label marker is absent while each redaction sentinel appears in both files. | ✓ CLOSED |

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `packages/open-bitcoin-node/src/network/runtime_authority.rs` | Shared typed network authority | ✓ VERIFIED | Exists, substantive, shared, and returns owned snapshots. |
| `packages/open-bitcoin-node/src/sync.rs` | Durable runtime owns/reuses authority | ✓ VERIFIED | Opens one network and exposes cloned handles. |
| `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` | Production composition root | ✓ VERIFIED | One authority is created before all consumers. |
| `packages/open-bitcoin-node/src/network/block_serving.rs` | Validated serve-intent gate | ✓ VERIFIED | Eligibility, caps, and achieved-effect completion are substantive. |
| `packages/open-bitcoin-node/src/network/inventory.rs` | Knots-compatible queue policy | ✓ VERIFIED | Unknown inventory is silently consumed; ordering is covered by integration tests. |
| `packages/open-bitcoin-node/src/storage/fjall_store.rs` | Canonical durable block and metadata reads | ✓ VERIFIED | `load_block` decodes the canonical hash-keyed persisted body; metadata is loaded on demand. |
| `packages/open-bitcoin-rpc/src/context.rs` | Durable response planning/data-flow bridge | ✓ VERIFIED | The exact loaded block value feeds block/witness/compact serialization outside guards. |
| `packages/open-bitcoin-rpc/src/inbound_listener.rs` | Socket write and completion bridge | ✓ VERIFIED | Evidence completion occurs only after a successful write. |
| `packages/open-bitcoin-rpc/src/context/network.rs` | Shared RPC authority and live metadata source | ✓ VERIFIED | Uses the injected handle and reads current Fjall metadata. |
| `packages/open-bitcoin-rpc/src/context/inbound_status.rs` | Owned aggregate operator snapshot | ✓ VERIFIED | One snapshot supplies network, mempool, inbound, relay, and block-relay fields. |
| `packages/open-bitcoin-cli/src/operator/support/redaction.rs` | Production support redaction | ✓ VERIFIED | All sensitive classes are redacted before serialization. |
| `packages/open-bitcoin-rpc/tests/black_box_parity.rs` | Headless production-composition guard | ✓ VERIFIED | Passed against loopback RPC/P2P and a restart-shaped Fjall datadir. |
| `scripts/check-phase127-authoritative-network-state-unification.ts` | Static semantic guard | ✓ VERIFIED | Passed and independently mutation-tested. |
| `docs/parity/catalog/p2p.md` / `docs/parity/index.json` | Bounded Knots evidence | ✓ VERIFIED | Five source anchors, two test anchors, and explicit later-phase exclusions. |

All four PLAN artifact checks passed (13/13 declarations), and all PLAN key-link checks passed (8/8 declarations).

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| daemon composition | durable sync runtime | one pre-context authority handle | ✓ WIRED | Same handle is cloned, not reconstructed. |
| RPC context | network authority | injected `ManagedNetworkHandle` | ✓ WIRED | Production constructor takes the shared handle. |
| inbound serve policy | Fjall block store | owned serve intent and exact `load_block(intent.block_hash())` value | ✓ WIRED | No cache substitution or unused durable read. |
| Fjall resolution | wire response | exact loaded `Block` value | ✓ WIRED | Full, witness, and compact modes serialize the durable value. |
| socket write | authority completion | success-only acknowledgement | ✓ WIRED | Failed/rejected writes receive failure completion. |
| RPC dispatch | aggregate response schemas | one authoritative operator snapshot | ✓ WIRED | No direct fallback fields or schema additions. |
| status collector | dashboard/support | `get_open_bitcoin_network_status` aggregate | ✓ WIRED | Both surfaces share the same collected snapshot. |
| default verifier | Phase 127 guard | semantic test followed by source checker | ✓ WIRED | Phase 127 runs before the downstream Phase 117 release-boundary check. |

### Data-Flow Trace (Level 4)

| Artifact | Data variable | Source | Produces real data | Status |
| --- | --- | --- | --- | --- |
| `dispatch/node.rs` | durable sync state | fresh `FjallNodeStore::load_runtime_metadata` per request | Yes | ✓ FLOWING |
| `context.rs` | `maybe_block` | request-scoped `FjallNodeStore::load_block` | Yes; exact value is matched and serialized | ✓ FLOWING |
| `inbound_status.rs` | authoritative operator snapshot | shared live `ManagedPeerNetwork` | Yes; owned aggregate | ✓ FLOWING |
| CLI status/dashboard/support | aggregate status snapshot | live HTTP RPC status client | Yes; production collector mapping | ✓ FLOWING |
| support bundle JSON/Markdown | redacted status | production `support_status_for_bundle` | Yes; actual files are written and checked | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| --- | --- | --- | --- |
| Semantic guard rejects plausible decoys and bypasses | `bun test scripts/check-phase127-authoritative-network-state-unification.test.ts` | 15 passed, 0 failed | ✓ PASS |
| Current source satisfies Phase 127 invariants | `bun run scripts/check-phase127-authoritative-network-state-unification.ts` | “Phase 127 authoritative network state unification validated.” | ✓ PASS |
| Production composition shares authority across sync, serving, and RPC | focused `cargo test` for `phase127_production_composition_shares_sync_serving_and_operator_authority` | 1 passed, 0 failed, 0 ignored | ✓ PASS |
| Durable block serving survives restart and redacts unavailable reads | focused `cargo test` filter `durable_block_serving` | 4 passed, 0 failed, 0 ignored | ✓ PASS |
| Parity breadcrumbs and generated LOC evidence are current | breadcrumb checker plus LOC `--check` | 384 Rust files verified; LOC report current | ✓ PASS |

The focused durable-serving command printed all four passing library tests before a zero-test daemon harness remained idle in the local runner; it was terminated after liveness inspection. This did not affect the four recorded test outcomes or the independently passing production-composition test.

### Requirements Coverage

| Requirement | Source plans | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| BSRV-03 | 127-01, 127-02, 127-04 | Serve only validated, available active-chain blocks from authoritative durable data. | ✓ SATISFIED | Shared chainstate gates an owned intent; the exact Fjall block is loaded and served after restart without cache hydration. |
| BSRV-04 | 127-02, 127-04 | Preserve full/witness/compact `getdata` parity, limits, backpressure, and cleanup. | ✓ SATISFIED | Knots queue ordering including unknown inventory is implemented and tested; request caps and success-only evidence remain wired. |
| OBS-02 | 127-01, 127-03, 127-04 | Shared truthful CLI/dashboard operational status. | ✓ SATISFIED | Both surfaces collect the same authoritative aggregate RPC snapshot with frozen schemas/labels. |
| OBS-04 | 127-03, 127-04 | Redacted support evidence without sensitive leakage. | ✓ SATISFIED | Production JSON and Markdown bundle path redacts all forbidden material classes. |

No Phase 127 requirements are orphaned: all four roadmap-mapped IDs are claimed by plans and have implementation evidence. Their canonical pending markers are expected before lifecycle promotion and were not edited by this verifier.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| Targeted Phase 127 corpus | — | No TODO/FIXME/placeholder, ignored-test, hollow-prop, static-empty-response, duplicate-production-authority, cache-substitution, or stale-default blocker found. | ℹ️ Info | None |

The few empty match arms found by the mechanical scan are deliberate protocol/result cases (for example silent unknown inventory and successful write acknowledgements), not empty implementations.

### Human Verification Required

None. The phase changes headless authority, storage, protocol, RPC, and evidence flow. Deterministic loopback, restart-shaped, file-output, schema, mutation, and source checks cover the observable contract without public networks or external services.

### Deferred Scope Review

No failed Phase 127 truth was moved to deferred scope. Phase 128 explicitly owns production compact negotiation/announcement transport, and Phase 129 owns aggregate four-flow reconciliation and the next archive decision; neither is required to satisfy this phase's bounded goal.

### Gaps Summary

No actionable gaps remain. All eleven merged roadmap/plan truths, thirteen declared artifacts, eight declared key links, four reassigned requirements, and the four review closures are verified.

***

_Verified: 2026-07-20T01:39:32Z_
_Verifier: the agent (gsd-verifier)_
