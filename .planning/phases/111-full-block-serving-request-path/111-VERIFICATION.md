---
phase: 111-full-block-serving-request-path
verified: 2026-07-04T19:05:40Z
status: passed
score: "10/10 must-haves verified"
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 111-2026-07-04T14-58-18
generated_at: 2026-07-04T19:05:40Z
lifecycle_validated: true
overrides_applied: 0
re_verification:
  previous_status: gaps_found
  previous_score: "9/10"
  gaps_closed:
    - "Checker reads and enforces packages/open-bitcoin-network/src/peer/inventory_state.rs and packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs."
    - "Checker rejects forbidden positive claims such as 'Phase 111 only supports BIP152 compact block payload serving.'"
  gaps_remaining: []
  regressions: []
---

# Phase 111: Full Block Serving Request Path Verification Report

**Phase Goal:** Add the node-shell path that serves eligible full and witness block requests from validated local block data without broad historical or archive-node claims.
**Verified:** 2026-07-04T19:05:40Z
**Status:** passed
**Re-verification:** Yes - after checker gap closure

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Eligible peers can request block and witness block inventory and receive the correct validated block serialization. | VERIFIED | `serve_inventory` routes `InventoryType::Block` and `InventoryType::WitnessBlock` through `serve_managed_block_request`; node Phase 111 tests passed, including full block serving and witness encode/decode preservation. |
| 2 | Unknown, stale, side-chain, pruned, unavailable, compact block, and ineligible requests produce deterministic missing, suppressed, or unavailable outcomes. | VERIFIED | Adapter and managed-network tests passed for disabled, compact, side-chain, pruned non-tip, unavailable tip, stale, and old-cache NotFound outcomes. |
| 3 | Full block serving participates in existing queue, request, and in-flight limits. | VERIFIED | `handle_getdata` builds `request_pressure_input` before `PeerAction::ServeInventory`; peer and node over-cap tests passed without block payload serving. |
| 4 | Historical and pruned block behavior stays bounded by documented eligibility rules and does not imply archive-node availability. | VERIFIED | Pruned, unavailable, side-chain, stale, and old cached block tests return `WireNetworkMessage::NotFound`; docs/parity roots keep archive-node behavior as a no-claim. |
| 5 | Full block, witness block, and compact block getdata requests pass through peer-manager request-pressure checks before node-shell serving. | VERIFIED | `inventory_state.rs` calls `request_pressure_input(peer, 0, inventory.inventory.len(), ...)` before `PeerAction::ServeInventory`; Phase 111 peer-manager tests passed. |
| 6 | Over-cap getdata bursts return resource-limit disconnect evidence and do not emit `PeerAction::ServeInventory`. | VERIFIED | `phase111_over_cap_block_witness_compact_getdata_disconnects_before_serve_inventory` and managed over-cap tests passed. |
| 7 | Received block, notfound, and peer removal cleanup keep block in-flight state bounded without compact-block in-flight state. | VERIFIED | Peer tests passed for block/witness `NotFound`, received block cleanup, peer removal, compact `NotFound`, and Phase 110 cleanup labels. |
| 8 | Transaction inventory in mixed getdata requests continues to use `RelayServingCache`, while only full/witness block inventory may produce `WireNetworkMessage::Block`. | VERIFIED | `serve_inventory` keeps transaction and witness-transaction branches on `relay_serving.classify_request`; mixed block/transaction test passed. |
| 9 | Docs and parity evidence record bounded Phase 111 behavior with BSRV-04, GOV-01, and GOV-05. | VERIFIED | `docs/parity/index.json` contains `v2-1-full-block-serving-request-path`, the three requirements, and the expected evidence roots; `scripts/verify.sh` runs Phase 111 checks after Phase 110. |
| 10 | Phase 111 checker rejects verifier omissions and forbidden compact/archive/public/default/production claims. | VERIFIED | Gap closure added both omitted evidence roots to `TARGET_FILES`, added root-specific checks and mutation tests, removed the broad standalone `only` no-claim marker, and passes the exact `only supports BIP152 compact block payload serving` mutation test. |

**Score:** 10/10 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `packages/open-bitcoin-network/src/peer/inventory_state.rs` | Peer-manager getdata pressure gate and ServeInventory routing before node-shell lookup | VERIFIED | Exists, substantive, and wired; source trace confirms request-pressure governance precedes `ServeInventory`. |
| `packages/open-bitcoin-network/src/peer/tests.rs` | Phase 111 peer-manager getdata pressure and cleanup regression coverage | VERIFIED | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib phase111_ -- --nocapture` passed 9 tests. |
| `packages/open-bitcoin-node/src/network/block_serving.rs` | Cache-backed node-shell block-serving adapter around Phase 110 policy | VERIFIED | Defines `ManagedBlockServeInput`, `ManagedBlockServeDecision`, and `serve_managed_block_request`; status, eligibility, and resource gates run before lookup. |
| `packages/open-bitcoin-node/src/network/inventory.rs` | Inventory serving router for block adapter and transaction cache | VERIFIED | Routes block/witness/compact inventory through the block adapter and transaction inventory through relay serving. |
| `packages/open-bitcoin-node/src/network/tests.rs` | Managed-network regressions for serving, suppressing, mixed tx, historical, pruned, and request pressure | VERIFIED | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib phase111_ -- --nocapture` passed 17 tests. |
| `packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs` | Relay-serving branch regression root | VERIFIED | Included in the checker corpus and enforced by `fails_when_declared_evidence_roots_lose_phase111_terms`. |
| `scripts/check-phase111-full-block-serving-request-path.ts` | Deterministic Phase 111 no-claim and evidence checker | VERIFIED | Exists, substantive, reads all declared Phase 111 evidence roots, and passes the current corpus. |
| `scripts/check-phase111-full-block-serving-request-path.test.ts` | Mutation tests for checker evidence and forbidden claims | VERIFIED | Passed 8 tests, including omitted-root and `only supports ...` forbidden-claim mutations. |
| `docs/parity/index.json` | Machine-readable bounded Phase 111 parity surface | VERIFIED | Contains the Phase 111 surface, requirements, Knots anchors, and evidence roots. |

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| `packages/open-bitcoin-network/src/peer/inventory_state.rs` | `packages/open-bitcoin-node/src/network/inventory.rs` | `PeerAction::ServeInventory` after `request_pressure_input` | VERIFIED | Source trace confirms peer-manager pressure checks precede node-shell serving. |
| `packages/open-bitcoin-network/src/peer/inventory_state.rs` | `packages/open-bitcoin-network/src/resource.rs` | `ResourceGovernancePolicy::decide_request` | VERIFIED | `resource_limit_disconnect_actions` delegates to the default resource-governance policy. |
| `packages/open-bitcoin-node/src/network/inventory.rs` | `packages/open-bitcoin-node/src/network/block_serving.rs` | `serve_managed_block_request` for block and witness block inventory | VERIFIED | Inventory serving delegates block and witness block requests to the adapter. |
| `packages/open-bitcoin-node/src/network/block_serving.rs` | `packages/open-bitcoin-network/src/block_serving.rs` | Phase 110 status, eligibility, and resource-gate functions | VERIFIED | Adapter imports and calls `classify_block_serving_status`, `classify_block_serving_eligibility`, and `evaluate_block_serving_resource_gate`. |
| `scripts/verify.sh` | `scripts/check-phase111-full-block-serving-request-path.ts` | Bun checker tests and checker execution | VERIFIED | Phase 111 checker commands appear after Phase 110 and before pure-core checks. |
| `scripts/check-phase111-full-block-serving-request-path.ts` | `packages/open-bitcoin-network/src/peer/inventory_state.rs` | `TARGET_FILES` evidence corpus and `checkRequiredEvidenceRoots` | VERIFIED | `gsd-tools verify key-links` passed, and mutation tests fail if `PeerAction::ServeInventory` evidence disappears. |
| `scripts/check-phase111-full-block-serving-request-path.ts` | `packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs` | `TARGET_FILES` evidence corpus and `checkRequiredEvidenceRoots` | VERIFIED | `gsd-tools verify key-links` passed, and mutation tests fail if the relay-serving branch regression disappears. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| --- | --- | --- | --- | --- |
| `packages/open-bitcoin-network/src/peer/inventory_state.rs` | `inventory.inventory` getdata vectors | Peer `WireNetworkMessage::GetData` through `handle_getdata` | Yes | VERIFIED - request counts feed resource policy before typed vectors become `ServeInventory`. |
| `packages/open-bitcoin-node/src/network/inventory.rs` | `ManagedBlockServeInput` and `decision.maybe_block` | Chainstate snapshot, peer context, resource snapshots, and lazy `blocks_by_hash` lookup | Yes | VERIFIED - payload lookup occurs in the adapter callback after policy gates; missing decisions flow to NotFound. |
| `packages/open-bitcoin-node/src/network/block_serving.rs` | `ManagedBlockServeDecision` | Phase 110 status/eligibility/resource decisions plus `lookup_block` | Yes | VERIFIED - only allowed block/witness requests with local data can return `Some(block)`. |
| `scripts/check-phase111-full-block-serving-request-path.ts` | `texts` map | Fixed `TARGET_FILES` corpus | Yes | VERIFIED - the corpus now includes and checks both formerly omitted evidence roots. |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| --- | --- | --- | --- |
| Peer-manager Phase 111 getdata pressure and cleanup matrix | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib phase111_ -- --nocapture` | 9 passed, 0 failed | PASS |
| Node-shell Phase 111 block-serving request path and negative matrix | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib phase111_ -- --nocapture` | 17 passed, 0 failed | PASS |
| Phase 111 checker mutation suite | `bun test scripts/check-phase111-full-block-serving-request-path.test.ts` | 8 passed, 0 failed | PASS |
| Phase 111 checker current corpus | `bun run scripts/check-phase111-full-block-serving-request-path.ts` | `validated Phase 111 full block-serving request path` | PASS |
| Gap-closure artifact and key-link metadata | `gsd-tools verify artifacts/key-links 111-04-PLAN.md` | 2/2 artifacts and 2/2 key links passed | PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| BSRV-04 | 111-01, 111-02, 111-03, 111-04 | Node handles block, witness block, and compact block `getdata` requests with bounded request caps, queue backpressure, and peer cleanup. | SATISFIED | Peer and node tests pass for block/witness/compact getdata, caps, cleanup, compact suppression, and NotFound behavior. |
| GOV-01 | 111-01, 111-02, 111-03, 111-04 | Full block serving and compact-block request classification participate in existing request, queue, and in-flight resource limits. | SATISFIED | `request_pressure_input`, `evaluate_block_serving_resource_gate`, request snapshots, and over-cap tests are wired and passing. |
| GOV-05 | 111-02, 111-03, 111-04 | Historical, pruned, stale, side-chain, and unavailable block serving remains bounded and does not imply archive-node behavior. | SATISFIED | Negative matrix tests and checker guardrails pass; forbidden compact/archive/public/default/production positive claims are rejected while no-claim wording remains allowed. |

No orphaned Phase 111 requirements were found in `.planning/REQUIREMENTS.md`; BSRV-04, GOV-01, and GOV-05 are the mapped Phase 111 requirements.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| None | - | - | - | No blocker, stub, orphaned artifact, broad `only` marker, compact payload serving, `getblocktxn`, `blocktxn`, archive-node implementation, or placeholder pattern was found in the verified Phase 111 serving/checker paths. |

The only `console.log` match in the checker is the intentional CLI success message emitted when validation passes.

### Human Verification Required

None. The phase goal is local and deterministic, and the request path plus checker guardrails are covered by source inspection and focused automated checks.

### Gaps Summary

No gaps remain. The two prior checker guardrail gaps are closed: the checker now reads and enforces the peer getdata source root plus relay-serving branch regression root, and the forbidden-claim detector rejects the exact `only supports BIP152 compact block payload serving` bypass while still allowing explicit no-claim wording.

### Guidance Applied

Verification followed repo-local guidance in `AGENTS.md`, Bright Builds verification/testing/Rust/TypeScript standards, and the GSD verifier escalation-gate process. Lifecycle provenance is consistent across `111-CONTEXT.md`, all four plans, all four summaries, and this verification report.

---

_Verified: 2026-07-04T19:05:40Z_
_Verifier: Claude (gsd-verifier)_
