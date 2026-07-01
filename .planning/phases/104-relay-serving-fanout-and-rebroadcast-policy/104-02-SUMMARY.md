---
phase: 104-relay-serving-fanout-and-rebroadcast-policy
plan: 02
subsystem: managed-peer-network
tags: [rust, node, transaction-relay, serving, fanout, parity, testing]
requirements-completed: [REL-01, REL-02, REL-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 104-2026-07-01T14-38-26
generated_at: 2026-07-01T15:32:46.913Z
completed: 2026-07-01
---

# Phase 104 Plan 02: Managed Relay Serving and Fanout Summary

Wired the pure transaction serving and fanout policies from Plan 01 into the
managed peer network runtime.

## Accomplishments

- Added `RelayServingCache` and managed `getdata` transaction serving so
  accepted, relay-eligible, identity-matched transactions can be served as `tx`
  while all non-serveable transaction requests return `notfound`.
- Preserved the existing block and witness-block serving branch outside the
  transaction relay serving classifier.
- Added `ManagedRelayFanoutState` to map accepted and replaced peer transaction
  outcomes into bounded `inv` fanout for eligible peers.
- Honored peer relay mode when announcing by txid versus wtxid, including
  `wtxidrelay` negotiation for witness transaction inventory.
- Suppressed fanout to the origin peer, relay-ineligible peers, and peers with
  recent rejects, recording only fixed labels and counts for inspection.
- Centralized lifecycle cleanup through `remove_stored_transactions_with_status`
  so transaction storage, serving state, and fanout queues are revoked together
  for confirmed, replaced, evicted, expired, and disconnected-peer paths.
- Registered new managed source and test files in parity breadcrumbs and kept
  the tracked LOC metrics current.

## Task Commits

1. `ea760def` - `feat(104-02): add managed relay serving cache`
2. `7002ea8e` - `feat(104-02): add managed relay fanout`

## Key Files

- `packages/open-bitcoin-node/src/network/relay_serving.rs`
- `packages/open-bitcoin-node/src/network/relay_fanout.rs`
- `packages/open-bitcoin-node/src/network/inventory.rs`
- `packages/open-bitcoin-node/src/network/action_translation.rs`
- `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs`
- `packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs`
- `packages/open-bitcoin-node/src/network/tests/relay_fanout_cases.rs`
- `docs/parity/source-breadcrumbs.json`
- `docs/metrics/lines-of-code.md`

## Knots Anchors

- `packages/bitcoin-knots/src/net_processing.cpp`
- `packages/bitcoin-knots/src/node/txdownloadman.h`
- `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp`
- `packages/bitcoin-knots/src/protocol.h`
- `packages/bitcoin-knots/src/txorphanage.cpp`
- `packages/bitcoin-knots/src/validation.cpp`
- `packages/bitcoin-knots/test/functional/p2p_getdata.py`
- `packages/bitcoin-knots/test/functional/p2p_tx_download.py`
- `packages/bitcoin-knots/test/functional/p2p_orphan_handling.py`
- `packages/bitcoin-knots/test/functional/mempool_accept.py`

## Decisions

- Kept managed serving and fanout as node-layer adapters over the pure
  `open-bitcoin-network` policies instead of adding mempool dependencies to the
  network crate.
- Kept public inspection surfaces low-cardinality: info structs expose counts
  and fixed action/outcome labels only, with no raw transaction hex, txids,
  wtxids, peer ids, endpoints, permission strings, class names, or credentials.
- Centralized transaction cleanup in `remove_stored_transactions_with_status`.
  Mempool lifecycle, admission replacement/eviction handling, and inventory
  storage removal now share that path so fanout cleanup cannot be recorded twice.
- Translated only `TxFanoutAction::Announce` into wire `inv` messages. Suppress,
  queue-cap, rate-limit, cleanup, and rebroadcast-deferred actions remain local
  evidence labels.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node managed_getdata -- --nocapture` passed: 3 tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node managed_fanout -- --nocapture` passed: 2 tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node network::tests::relay_fanout_cases::managed_lifecycle_cleanup_removes_serving_and_fanout_state -- --exact` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node relay_fanout_cases -- --nocapture` passed: 3 tests.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings` passed.
- `cargo fmt --manifest-path packages/Cargo.toml --all --check` passed.
- `bash scripts/check-file-lengths.sh` passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed.
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check` passed.
- `git diff --check` passed.
- `bash scripts/verify.sh` passed through both task commit hooks.

## Boundaries

Phase 104 Plan 02 does not add periodic local rebroadcast scheduling, public
relay defaults, compact block relay, package relay, bloom/filter serving,
public-network CI, production service operation, production full-node readiness,
or production-funds wallet use.

Local submission relay evidence, RPC integration, docs, checker guardrails, and
final phase verification remain for later Phase 104 plans.
