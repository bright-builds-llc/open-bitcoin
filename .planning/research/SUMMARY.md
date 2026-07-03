# v2.1 Block Serving and Compact Block Relay Research Summary

**Defined:** 2026-07-03
**Milestone:** v2.1 Block Serving and Compact Block Relay Boundary

## Scope Decision

v2.1 should add bounded, opt-in block serving and compact-block relay behavior on top of the v1.9 inbound and v2.0 transaction relay foundations. The milestone should serve validated blocks to eligible peers, implement BIP152 wire messages, track compact-block negotiation, reconstruct compact blocks from mempool state, request missing transactions, fall back safely, and expose truthful operator evidence.

The milestone is not a public serving launch. It should explicitly avoid production full-node readiness, public serving by default, package relay, bloom/filter serving, compact filters, public-network CI, and production-funds wallet claims.

## Stack Additions

- No new external production dependency is recommended.
- Add first-party BIP152 payload codecs in `open-bitcoin-codec` and expose them through `open-bitcoin-network::message`.
- Add a small owned SipHash-2-4 or BIP152 short-ID helper with fixture tests if no existing first-party helper is available.
- Add pure block-serving and compact-relay policy/state modules under `open-bitcoin-network`.
- Add node-shell integration in `open-bitcoin-node` for block reads, compact-block construction, mempool reconstruction inputs, partial-state lifecycle, validation handoff, status, metrics, logs, and support evidence.
- Extend existing RPC/CLI/dashboard/support surfaces through one shared status contract.

## Feature Table Stakes

- Explicit activation and peer eligibility for block serving and compact relay.
- Full block and witness block serving for validated, available blocks inside the documented boundary.
- `sendcmpct`, `cmpctblock`, `getblocktxn`, and `blocktxn` encode/decode support.
- Per-peer compact-block negotiation with high-bandwidth and low-bandwidth semantics.
- Compact-block announcement only when activation, negotiation, header state, and block availability permit it.
- Reconstruction from mempool plus bounded extra/recent transaction inputs.
- Missing transaction requests and `blocktxn` responses matched to expected in-flight partial blocks.
- Full-block fallback for reconstruction failure, missing/incomplete responses, old/far blocks, timeouts, collisions, or ineligible state.
- Resource caps and cleanup for serving requests, compact partial state, queues, in-flight blocks, and peer churn.
- Sanitized RPC, CLI, dashboard, metrics, logs, support bundles, parity docs, UAT, and no-claim guardrails.

## Architecture Direction

- Keep BIP152 payload correctness in codec modules.
- Keep peer negotiation and serving decisions pure in `open-bitcoin-network`.
- Keep block storage, mempool snapshots, partial compact state, validation handoff, and observability in `open-bitcoin-node`.
- Keep operator surfaces as projections of one status contract.
- Keep public-network review outside the default verifier.

## Watch-Outs

- Do not let `sendcmpct` or compact-block capability imply public compact relay by default.
- Do not serve blocks before classifying peer eligibility and block status.
- Do not leak peer endpoints, permission strings, raw transaction lists, or dynamic labels through observability.
- Do not persist partial compact-block reconstruction state.
- Do not process `blocktxn` without a matching in-flight partial block for that peer.
- Do not let reconstruction failure create repeated full-block request storms.
- Do not allow BIP152 work to pull package relay, bloom/filter serving, compact filters, or production readiness into scope.

## Recommended Phase Order

1. Phase 110: Block Serving Activation and Eligibility Boundary.
2. Phase 111: Full Block Serving Request Path.
3. Phase 112: BIP152 Wire Codec and Message Semantics.
4. Phase 113: Compact Relay Negotiation and Announcement Policy.
5. Phase 114: Compact Block Reconstruction from Mempool State.
6. Phase 115: Missing Transaction Round Trip, Fallback, and Validation Handoff.
7. Phase 116: Operator Evidence, Metrics, Logs, and Support Boundary.
8. Phase 117: Parity Traceability, UAT, and Release Guardrails.

This order activates the serving boundary first, then proves the wire format, then adds compact relay negotiation, reconstruction, fallback, operator evidence, and final release guardrails.

## Deferred Or Out Of Scope

- Package relay, cluster mempool policy, and package orphan handling.
- BIP37 bloom filters, compact filters, and filter serving.
- Public block or compact-block serving by default.
- Public-network relay UAT as a default CI or pre-commit gate.
- Production full-node readiness, production service operation, and production-funds wallet safety.
- GUI, hosted dashboards, packaging, installer, service-manager expansion, and migration apply mode.

## Verification Implications

- Keep `bash scripts/verify.sh` as the default deterministic verification contract.
- Add pure tests for codecs, short IDs, differential indexes, negotiation, serving eligibility, reconstruction decisions, missing transaction requests, and fallback.
- Add node-shell tests for block store reads, compact construction, mempool reconstruction inputs, in-flight cleanup, validation handoff, restart cleanup, and status projection.
- Add static checkers for no-claim wording, parity breadcrumbs, source anchors, UAT command forms, and verifier boundaries.
- Keep public-network relay review opt-in and documented as non-default evidence.

## Sources

- `.planning/PROJECT.md`
- `.planning/milestones/v1.9-REQUIREMENTS.md`
- `.planning/milestones/v2.0-REQUIREMENTS.md`
- `packages/bitcoin-knots/src/protocol.h`
- `packages/bitcoin-knots/src/blockencodings.h`
- `packages/bitcoin-knots/src/blockencodings.cpp`
- `packages/bitcoin-knots/src/net_processing.cpp`
- `packages/bitcoin-knots/test/functional/p2p_compactblocks.py`
- `packages/bitcoin-knots/test/functional/p2p_compactblocks_extratxs.py`
- `packages/bitcoin-knots/test/functional/p2p_compactblocks_blocksonly.py`
- `packages/bitcoin-knots/test/functional/p2p_compactblocks_hb.py`
- `packages/bitcoin-knots/test/functional/p2p_mutated_blocks.py`
- `packages/open-bitcoin-network/src/message.rs`
- `packages/open-bitcoin-network/src/peer.rs`
- `packages/open-bitcoin-network/src/peer/relay_download.rs`
- `packages/open-bitcoin-node/src/network.rs`
- `packages/open-bitcoin-node/src/network/relay_serving.rs`
- `packages/open-bitcoin-node/src/status/relay_evidence.rs`
