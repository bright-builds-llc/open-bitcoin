# v2.0 Transaction Relay and Mempool Participation Research Summary

**Defined:** 2026-06-29
**Milestone:** v2.0 Transaction Relay and Mempool Participation Boundary

## Scope Decision

v2.0 should deliberately move Open Bitcoin from "no transaction relay or mempool propagation claim" to a bounded, opt-in transaction relay and mempool participation claim. The milestone should validate, store, announce, request, serve, and relay unconfirmed transactions through explicit activation gates while preserving the existing no-claim boundary for public relay defaults, compact block relay, bloom/filter relay, package relay, production full-node readiness, and production-funds wallet safety.

The milestone is not a public relay launch. It is a boundary milestone that proves the transaction relay path is internally coherent, resource bounded, observable, and parity-traceable before broader public-network claims are made.

## Architecture Direction

- Keep relay policy in a pure `open-bitcoin-network::tx_relay` decision layer. It should accept peer state, txid/wtxid announcements, `notfound`, received transactions, clock values, and permission effects, then emit typed actions and evidence.
- Keep mempool truth in `open-bitcoin-mempool`. Extend admission and removal outcomes so relay code reacts to accepted, rejected, duplicate, missing-input, replaced, evicted, confirmed, disconnected, and expired states without parsing display strings.
- Keep `open-bitcoin-node` as the imperative shell. It should call pure relay and mempool decisions, persist accepted transactions, write metrics/logs/status, and encode or send wire messages.
- Keep RPC, CLI, dashboard, metrics, logs, and support bundles on one shared relay/mempool status contract. Do not create a separate operator-only truth source.
- Treat parity docs and checkers as implementation surfaces because v2.0 changes a previously deferred behavior claim.

## Stack And Module Impacts

- No new third-party dependencies are recommended.
- Extend first-party crates already in use: `open-bitcoin-network`, `open-bitcoin-mempool`, `open-bitcoin-node`, `open-bitcoin-rpc`, and `open-bitcoin-cli`.
- Reuse v1.9 inbound permission, resource-governance, status, metrics, logging, support-bundle, and deterministic-verifier foundations.
- Add durable mempool storage only through versioned Open Bitcoin-owned DTOs under the node storage shell.
- Keep substantial automation in Bun/TypeScript only for verification and docs checkers; keep Bash as thin orchestration.

## v2.0 Table Stakes

- Explicit relay activation that keeps public relay off by default.
- Permission-aware behavior for `relay`, `forcerelay`, and `mempool` without activating bloom/filter or compact-block behavior.
- Typed txid/wtxid inventory handling for `inv`, `getdata`, `tx`, and `notfound`.
- Bounded transaction download scheduling with in-flight caps, expiry, peer fallback, disconnect cleanup, and recent-reject/already-have suppression.
- Bounded missing-parent and orphan handling with parent requests, reconsideration, cap eviction, and expiry evidence.
- Stable mempool admission and removal outcomes for local and peer-submitted transactions.
- Chainstate reconciliation when blocks connect, disconnect, or reorg within the documented v2.0 boundary.
- Relay serving, announcement, fanout, and rebroadcast policy that uses peer eligibility, negotiated identity, rate limits, and queue limits.
- RPC, CLI, dashboard, metrics, logs, and support evidence for relay outcomes with strong redaction.
- Deterministic parity fixtures and release-boundary checks that prevent compact-block, public-relay-default, production-readiness, and production-funds claims from drifting in.

## Watch-Outs

- Do not let permission parsing alone imply activation. Every active effect needs an activation matrix and tests.
- Do not mix txid and wtxid identity in request maps, already-have sets, `notfound` handling, or received-transaction cleanup.
- Do not route peer transactions directly into mempool admission from socket code. Relay/download state must remain the policy boundary.
- Do not create unbounded orphan, request, fanout, or rebroadcast state.
- Do not serve transactions merely because they exist in local storage. Serving must require relay eligibility.
- Do not make default verification depend on public-network relay, wall-clock soak, service-manager state, or production deployment.
- Do not leak raw transaction hex, raw peer endpoints, permission strings, credentials, or dynamic identifiers through support, logs, or metrics.

## Recommended Phase Order

1. Phase 100: Relay Activation Boundary and Permission Semantics.
2. Phase 101: Transaction Inventory Identity and Download Scheduling.
3. Phase 102: Orphan Handling and Admission Outcome Bridge.
4. Phase 103: Mempool Chainstate Lifecycle and Durable Recovery.
5. Phase 104: Relay Serving, Fanout, and Rebroadcast Policy.
6. Phase 105: Operator, RPC, Metrics, Logs, and Support Evidence.
7. Phase 106: Parity Traceability, UAT, and Release Boundary Guardrails.

This order starts with pure policy and identity first, then adds mempool integration, durable/runtime shell behavior, operator evidence, and final release guardrails. Daemon socket activation should follow the pure policy work, not lead it.

## Deferred Or Out Of Scope

- Compact block relay and related `cmpctblock` or `blocktxn` behavior.
- BIP37 bloom filters, compact filters, and full filter serving.
- Broad package relay, cluster mempool policy, and package orphan handling.
- Public transaction relay by default.
- Public-network relay UAT as a default CI or pre-commit gate.
- Production full-node readiness, production service operation, and production-funds wallet safety.
- GUI, hosted dashboards, packaging, installer, and migration apply mode.

## Verification Implications

- Keep `bash scripts/verify.sh` as the default deterministic verification contract.
- Add pure tests first for relay decisions, identity handling, request expiry, orphan bounds, and mempool outcomes.
- Add adapter tests for durable mempool persistence, restart/load/repair behavior, and managed runtime message flows.
- Add static checkers for no-claim wording, parity breadcrumbs, source anchors, UAT command forms, and verifier order.
- UAT guidance should provide repo-local Cargo and Bazel commands, default to loopback/regtest-safe workflows, and mark public-network relay review as opt-in evidence.

## Sources

- `.planning/research/STACK.md`
- `.planning/research/FEATURES.md`
- `.planning/research/ARCHITECTURE.md`
- `.planning/research/PITFALLS.md`
- `.planning/PROJECT.md`
- `.planning/milestones/v1.9-REQUIREMENTS.md`
- `packages/bitcoin-knots/src/net_processing.cpp`
- `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp`
- `packages/bitcoin-knots/src/node/txdownloadman.h`
- `packages/bitcoin-knots/src/txmempool.cpp`
- `packages/bitcoin-knots/src/validation.cpp`
- `packages/bitcoin-knots/src/policy/`
- `packages/bitcoin-knots/test/functional/p2p_tx_download.py`
- `packages/bitcoin-knots/test/functional/feature_rbf.py`
