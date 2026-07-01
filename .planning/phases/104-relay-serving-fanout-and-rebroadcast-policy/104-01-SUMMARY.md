---
phase: 104-relay-serving-fanout-and-rebroadcast-policy
plan: 01
subsystem: transaction-relay
tags: [rust, network, transaction-relay, fanout, parity, testing]
requirements-completed: [REL-01, REL-02, REL-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 104-2026-07-01T14-38-26
generated_at: 2026-07-01T14:38:26.627Z
completed: 2026-07-01
---

# Phase 104 Plan 01: Pure Relay Serving and Fanout Policy Summary

Added side-effect-free transaction serving and fanout policy contracts in
`open-bitcoin-network`.

## Accomplishments

- Added `TxServeOutcomeLabel`, `TxServingRecordStatus`, `TxServeDecision`, and
  `classify_tx_serve_request` for typed transaction `getdata` serving decisions.
- Added `TxFanoutAction`, `TxFanoutQueue`, `TxFanoutPolicy`,
  `TxFanoutPeerInput`, `TxFanoutAdmission`, and fixed fanout suppression and
  cleanup labels.
- Kept fanout pure and fake-clock driven: callers provide transaction identity,
  peer eligibility, peer facts, and `now_unix_seconds`; the policy performs no
  I/O, sleeps, timers, logging, storage access, or mempool mutation.
- Added explicit `rebroadcast_deferred` evidence through
  `defer_local_rebroadcast` for accepted local transactions when periodic
  rebroadcast is requested.
- Registered new source and test files under the transaction relay parity
  breadcrumb group.

## Task Commits

1. `3bbae208` - `feat(104-01): add pure relay serving classifier`
2. `e1150307` - `feat(104-01): add pure relay fanout policy`

## Key Files

- `packages/open-bitcoin-network/src/peer/transaction_relay/serving.rs`
- `packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs`
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests/serving_cases.rs`
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests/fanout_cases.rs`
- `packages/open-bitcoin-network/src/peer/transaction_relay.rs`
- `packages/open-bitcoin-network/src/peer.rs`
- `packages/open-bitcoin-network/src/lib.rs`
- `docs/parity/source-breadcrumbs.json`
- `docs/metrics/lines-of-code.md`

## Knots Anchors

- `packages/bitcoin-knots/src/protocol.h`
- `packages/bitcoin-knots/src/net_processing.cpp`
- `packages/bitcoin-knots/src/node/txdownloadman.h`
- `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp`
- `packages/bitcoin-knots/src/txrequest.h`
- `packages/bitcoin-knots/src/txrequest.cpp`
- `packages/bitcoin-knots/test/functional/p2p_getdata.py`
- `packages/bitcoin-knots/test/functional/p2p_tx_download.py`

## Decisions

- Kept `open-bitcoin-network` free of a new `open-bitcoin-mempool`
  dependency. The pure fanout API accepts `TxFanoutAdmission` with
  `TxFanoutAdmissionOutcome::{Accepted, Replaced}`; managed node adapters can
  map `MempoolOutcome` into that smaller contract in later plans.
- Used fixed low-cardinality labels for all serve, suppress, cleanup, rate, and
  rebroadcast decisions.
- Kept `peer.rs` under the production file-length gate with minimal re-export
  formatting changes only.
- Added coverage-focused fanout tests after the full verifier identified missing
  branch coverage in label, duplicate, cleanup, empty-state, and identity
  unavailable paths.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network peer::transaction_relay::tests::tx_serving_policy_reports_low_cardinality_outcomes -- --exact` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network peer::transaction_relay::tests::tx_serving_policy_rejects_identity_mismatch_and_non_transaction_inventory -- --exact` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network tx_fanout_policy -- --nocapture` passed: 5 tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network peer::transaction_relay::tests::tx_fanout_policy_honors_identity_and_limits -- --exact` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network peer::transaction_relay::tests::tx_fanout_policy_suppresses_origin_and_ineligible_peers -- --exact` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network peer::transaction_relay::tests::tx_fanout_policy_reports_rebroadcast_deferred_without_timer -- --exact` passed.
- `bash -lc '! rg -n "sleep|tokio::time|Instant::now|SystemTime::now" packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs packages/open-bitcoin-network/src/peer/transaction_relay/tests/fanout_cases.rs'` passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings` passed.
- `cargo llvm-cov --manifest-path packages/Cargo.toml --package open-bitcoin-network --show-missing-lines --text` passed after the expanded fanout coverage tests.
- `bash scripts/verify.sh` passed in both task commit hooks.

## Boundaries

Phase 104 Plan 01 provides pure serving and fanout policy only. It does not add periodic rebroadcast scheduling, compact block relay, package relay, bloom/filter serving, public relay defaults, public-network CI, production service operation, production full-node readiness, or production-funds wallet use.

Managed transaction storage, `tx`/`notfound`/`inv` translation, local RPC
submission evidence, mempool lifecycle cleanup wiring, docs, and verifier
guardrails remain for later Phase 104 plans.
