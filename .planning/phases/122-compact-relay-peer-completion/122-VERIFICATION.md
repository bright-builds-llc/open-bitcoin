---
phase: 122-compact-relay-peer-completion
status: passed
verified_at: "2026-07-15T16:37:14Z"
score: "15/15 must-haves verified"
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 122-2026-07-15T15-22-57
generated_at: "2026-07-15T16:37:14Z"
lifecycle_validated: true
requirements_verified:
  - HARD-01
review_fixes_verified:
  - WR-01
human_verification_required: false
---

# Phase 122: Compact Relay Peer Completion Verification Report

**Phase Goal:** Complete live per-peer compact-relay missing-transaction responses so eligible inbound `getblocktxn` requests for locally announced compact blocks reach a bounded, parity-auditable serving path.

**Status:** passed

**Score:** 15/15 must-haves verified

This verification was materially informed by the repo-local `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, and the architecture, code-shape, verification, testing, operability, Rust, and TypeScript standards.

## HARD-01 Traceability

`HARD-01` was explicitly verified despite the verifier initialization returning no requirement ID. The requirement is owned by Phase 122 in `.planning/REQUIREMENTS.md` and `.planning/ROADMAP.md`, is declared in `122-01-PLAN.md`, and is listed as completed by `122-01-SUMMARY.md`.

| HARD-01 obligation | Verified implementation and evidence |
| --- | --- |
| Serve eligible inbound `getblocktxn` | `PeerManager::handle_message` routes `GetBlockTxn` to `handle_get_block_transactions`; authorized requests emit `PeerAction::ServeCompactBlockTransactions`, which `ManagedPeerNetwork::process_actions` translates through `serve_managed_compact_block_transactions` into `WireNetworkMessage::BlockTxn`. |
| Locally announced compact blocks only | `ManagedPeerNetwork::announce_block` records a hash only after `announce_block_with_action` returns an actual `CompactBlock`; dispatch checks that hash in the requesting peer's `compact_announcements` before emitting the serving action. |
| Bounded peer path | `CompactAnnouncementProvenance` stores only `BlockHash` values in an eleven-entry `VecDeque` plus `BTreeSet`, inserts idempotently, and evicts FIFO. Peer removal drops the entire session state. |
| Eligible, validated, available data | The node builds `ManagedBlockServeInput` and reuses activation, peer eligibility, active-chain/validation, data-availability, and resource gates before the block lookup can serve. |
| Correct response semantics | `serve_managed_compact_block_transactions` clones transactions in expanded request-index order. The live test proves ordered transactions retain witness stacks. |
| Stable failure behavior | In-cap unannounced, unavailable, and ineligible requests are silent; differential-index overflow and live block-index out-of-bounds paths disconnect through typed compact misbehavior. Oversized requests pass through request-pressure governance before benign provenance suppression. |
| Parity-auditable scope | `docs/parity/index.json`, `docs/parity/catalog/p2p.md`, and `docs/parity/checklist.md` cite pinned Knots `GETBLOCKTXN`, `SendBlockTransactions`, `BlockTransactionsRequest`, and `test_getblocktxn_handler` anchors and explicitly state that old-block full-witness-block fallback is intentionally omitted. |

**HARD-01 result:** SATISFIED.

## Observable Truths

| # | Plan truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | A peer receives `blocktxn` only for a block that peer session was actually sent as `cmpctblock`. | VERIFIED | Post-construction recording in `network.rs`, peer-local membership check in `message_dispatch.rs`, peer-isolation tests in network and node crates. |
| 2 | Provenance is bounded per peer, hash-only, deterministic, and session-volatile. | VERIFIED | Eleven-entry FIFO/set implementation; idempotence/eviction unit test; disconnect/reconnect cleanup test. |
| 3 | A valid request returns transactions in request order with witness data preserved. | VERIFIED | Ordered index expansion and cloned stored transactions; `phase122_compact_announcement_then_getblocktxn_serves_ordered_witness_transactions`. |
| 4 | Benign unservable requests are silent; malformed indexes cause typed disconnect. | VERIFIED | Other-peer/unavailable/ineligible silence tests, differential-overflow peer test, and live out-of-bounds disconnect test. |
| 5 | The omitted Knots old-block full-block fallback is a documented scoped deviation. | VERIFIED | Exact deviation appears in the parity index, P2P catalog, and checklist without archive, public-default, or production claims. |
| 6 | Default verification runs a mutation-tested Phase 122 checker and the full repo contract. | VERIFIED | `scripts/verify.sh` contains both visible and executable Phase 122 checker steps after Phase 121; mutation suite and live checker pass. |

## Required Artifacts

| Artifact | Status | Verification |
| --- | --- | --- |
| `packages/open-bitcoin-network/src/peer/compact_relay.rs` | VERIFIED | Provides bounded hash-only `CompactAnnouncementProvenance`. |
| `packages/open-bitcoin-network/src/peer/message_dispatch.rs` | VERIFIED | Provides typed `getblocktxn` dispatch, pre-provenance pressure enforcement, benign suppression, and overflow disconnect. |
| `packages/open-bitcoin-node/src/network/action_translation.rs` | VERIFIED | Translates the typed peer action through current serving gates and emits `BlockTxn` only for `Served`. |
| `scripts/check-phase122-compact-relay-peer-completion.ts` | VERIFIED | Exports `checkPhase122CompactRelayPeerCompletion` and checks the production path, WR-01 ordering, tests, parity evidence, and verifier wiring. |
| `scripts/verify.sh` | VERIFIED | Runs the Phase 122 mutation tests and live checker in the default verifier order. |

## Key Link Verification

| From | To | Via | Status |
| --- | --- | --- | --- |
| `packages/open-bitcoin-node/src/network.rs` | `peer/compact_relay.rs` | Hash recorded only after successful `CompactBlock` construction | WIRED |
| `peer/message_dispatch.rs` | `network/action_translation.rs` | `PeerAction::ServeCompactBlockTransactions` | WIRED |
| `network/action_translation.rs` | `network/block_serving.rs` | `serve_managed_compact_block_transactions` with shared block lookup and policy inputs | WIRED |
| `scripts/verify.sh` | Phase 122 checker | Visible command list and executable `run_step` entries | WIRED |

## Code Review Fix

`WR-01` is fixed. The raw `request.index_deltas.len()` pressure decision now executes before the unannounced-provenance early return and before differential-index expansion/allocation. `phase122_unannounced_getblocktxn_over_request_cap_disconnects_before_suppression` proves the resource-governance disconnect, and the checker mutation suite rejects reordering that gate behind provenance suppression.

## Automated Evidence

| Check | Result |
| --- | --- |
| `bun test scripts/check-phase122-compact-relay-peer-completion.test.ts` | PASS: 15 passed, 0 failed |
| `bun run scripts/check-phase122-compact-relay-peer-completion.ts` | PASS: live corpus validated |
| `git diff --check` | PASS |
| Orchestrator `bash scripts/verify.sh` final rerun | PASS: exited 0; `verify.sh completed in 2m 29.811s (149811ms)` on 2026-07-15 |

The full verifier included the Phase 122 checker at 15 passed/0 failed, 451 `open-bitcoin-network` tests, node compact-relay cases, coverage with no uncovered-line failure, and successful Bazel build/run smoke checks.

## Parity and Scope Review

The parity claim is supported and deliberately narrow. The implementation matches the cited Knots behavior for ordered witness-bearing transaction selection, silence when data cannot be served, and out-of-bounds misbehavior. It intentionally tightens authorization to a peer-session compact-announcement token and explicitly records the absence of Knots' old-block full-witness-block fallback. No unsupported archive-node, public compact-relay default, public-network CI, production service, production-readiness, package relay, bloom-filter, or compact-filter claim was introduced.

## Human Verification

None required. The phase behavior is deterministic, local, public-network-free, and covered by implementation inspection, focused regression tests, mutation tests, coverage, and the full repository verifier.

## Gaps Summary

No gaps found. All six truths, five required artifacts, four key links, `HARD-01`, and review finding `WR-01` are verified.

## Verification Complete

**Status:** passed — **Score:** 15/15 must-haves verified.
