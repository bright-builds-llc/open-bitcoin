---
phase: 111-full-block-serving-request-path
plan: 02
subsystem: network-node-block-serving
tags: [block-serving, getdata, node-shell, witness-block, transaction-relay]
requires:
  - phase: 111-full-block-serving-request-path
    provides: Plan 111-01 peer-manager getdata pressure and cleanup regression coverage
  - phase: 110-block-serving-activation-and-eligibility-boundary
    provides: block-serving activation, eligibility, status, resource-gate, and cleanup contracts
provides:
  - cache-backed node-shell block-serving adapter with lazy payload lookup
  - explicit block-serving activation plumbing on ManagedPeerNetwork
  - serve_inventory routing for full block, witness block, and compact block inventory
  - managed-network regressions for disabled, enabled, witness, compact, and mixed block/transaction getdata
affects: [phase-111, phase-112, block-serving, compact-relay, node-network]
tech-stack:
  added: []
  patterns: [lazy-payload-lookup-after-policy-gate, explicit-block-serving-activation]
key-files:
  created:
    - packages/open-bitcoin-node/src/network/block_serving.rs
    - .planning/phases/111-full-block-serving-request-path/111-02-SUMMARY.md
  modified:
    - docs/parity/source-breadcrumbs.json
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/inventory.rs
    - packages/open-bitcoin-node/src/network/relay_serving.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs
key-decisions:
  - "ManagedBlockServeInput carries only inventory, peer, status, and resource facts; block payload lookup is a lazy closure invoked after policy gates pass."
  - "Block serving is default-off and separate from transaction relay activation."
  - "Compact block getdata is classified and suppressed in Phase 111 without producing compact or full-block fallback payloads."
patterns-established:
  - "Node-shell serving adapters wrap pure network policy decisions before effectful cache or future storage reads."
  - "Transaction relay serving remains on RelayServingCache while block serving uses a separate adapter."
requirements-completed: [BSRV-04, GOV-01, GOV-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 111-2026-07-04T14-58-18
generated_at: 2026-07-04T16:53:33Z
duration: 12m
completed: 2026-07-04
---

# Phase 111 Plan 02: Node-Shell Block Serving Summary

**Eligible full and witness block getdata requests now serve validated local block data through a node-shell adapter after Phase 110 policy gates.**

## Performance

- **Duration:** 12m
- **Started:** 2026-07-04T16:41:34Z
- **Completed:** 2026-07-04T16:53:33Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- Added `packages/open-bitcoin-node/src/network/block_serving.rs` with `ManagedBlockServeInput`, `ManagedBlockServeDecision`, and `serve_managed_block_request`.
- Added explicit `block_relay_activation` plumbing and `new_with_block_relay_activation` while keeping transaction relay activation unchanged.
- Routed block, witness block, and compact block inventory through the adapter from `ManagedPeerNetwork::serve_inventory`.
- Preserved transaction and witness transaction serving through `RelayServingCache`.
- Added managed-network regressions for default-off block suppression, enabled full block serving, witness payload encode/decode preservation, compact block suppression, and mixed block/transaction getdata.
- Registered the new node block-serving adapter in `docs/parity/source-breadcrumbs.json`.

## Task Commits

Task changes are intentionally held for the final phase commit after full Phase 111 verification:

1. **Task 1: Create the managed block-serving adapter boundary** - pending final phase commit.
2. **Task 2: Route serve_inventory through the adapter without regressing transaction serving** - pending final phase commit.

## Validation Evidence

- `cargo fmt --manifest-path packages/Cargo.toml --all` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib phase111_block_serving -- --nocapture` passed with 4 adapter tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib phase111_ -- --nocapture` passed with 9 Phase 111 node tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib relay_serving -- --nocapture` passed with 4 relay-serving tests.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings` passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed.
- `bash scripts/check-file-lengths.sh` passed.
- Plan acceptance `rg` probes passed for activation plumbing, adapter contracts, inventory coverage, lazy lookup, parity breadcrumbs, permission/resource/status facts, transaction relay preservation, managed-network regression names, and forbidden compact/package/filter markers.

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/block_serving.rs` - Node-shell adapter around Phase 110 block-serving policy with lazy cache lookup.
- `packages/open-bitcoin-node/src/network.rs` - Block-serving module and activation field.
- `packages/open-bitcoin-node/src/network/inventory.rs` - `serve_inventory` routing through the block-serving adapter while preserving transaction relay serving.
- `packages/open-bitcoin-node/src/network/relay_serving.rs` - Constructor plumbing for explicit block-serving activation.
- `packages/open-bitcoin-node/src/network/tests.rs` - Phase 111 managed-network regressions.
- `packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs` - Existing block-branch regression updated to opt into block serving explicitly.
- `docs/parity/source-breadcrumbs.json` - New adapter file registered under node network breadcrumbs.

## Decisions Made

- The adapter does not accept a `Block` or `Option<Block>` in its input; payload lookup is only a closure called after status, eligibility, and resource gates allow serving.
- Active-chain cache metadata can produce `Available`; cached non-active blocks are side-chain/unavailable and are not served.
- `RecentValid` and `Stale` are covered directly at the adapter boundary until managed chainstate has first-class metadata for those positions.
- Compact block inventory always returns suppressed/missing in Phase 111 and does not emit `WireNetworkMessage::Block`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Updated stale relay-serving regression for explicit activation**

- **Found during:** Task 1 focused adapter test run.
- **Issue:** An existing `managed_getdata_preserves_block_serving_branch` test expected default block serving from the old direct cache path.
- **Fix:** Updated the test to use `new_with_block_relay_activation` with block serving enabled, preserving the test's original intent that block getdata stays separate from transaction relay serving.
- **Files modified:** `packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib block_serving -- --nocapture` and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib relay_serving -- --nocapture` passed.
- **Committed in:** pending final phase commit.

**2. [Rule 1 - Bug] Isolated witness serialization from consensus validation**

- **Found during:** Task 2 focused Phase 111 test run.
- **Issue:** Adding coinbase witness data before `connect_local_block` failed consensus with `unexpected-witness`.
- **Fix:** Connected a normal validated block first, then replaced only the local cached payload with a witness-carrying copy under the same block hash so the test proves serving-path encode/decode preservation without weakening validation.
- **Files modified:** `packages/open-bitcoin-node/src/network/tests.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib phase111_ -- --nocapture` passed.
- **Committed in:** pending final phase commit.

**Total deviations:** 2 auto-fixed issues.
**Impact on plan:** Both fixes tightened Phase 111's explicit activation and validation boundaries without adding out-of-scope behavior.

## Issues Encountered

- The initial `phase111_block_serving` test command matched zero tests, so adapter unit tests were renamed with a `phase111_block_serving_` prefix to make the planned verifier command meaningful.
- A coinbase witness payload cannot be validated directly under current consensus rules; the final regression validates the chain position first and then checks cached witness serialization only.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Plan 111-03 can add historical/pruned/request-pressure matrix coverage and docs/checker guardrails on top of the working node-shell block-serving path.

## Self-Check: PASSED

- [x] Eligible full and witness block requests serve only after activation, eligibility, status, resource gates, and local data all pass.
- [x] Disabled, unavailable, side-chain, compact, and stale paths do not invoke payload serving.
- [x] Transaction relay serving remains on `RelayServingCache`.
- [x] Witness block serving is covered by encode/decode preservation.
- [x] No compact payload serving, `getblocktxn`, `blocktxn`, package/filter relay, public default, or archive-node claim was introduced.
