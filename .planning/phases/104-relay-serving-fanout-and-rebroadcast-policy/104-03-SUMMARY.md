---
phase: 104-relay-serving-fanout-and-rebroadcast-policy
plan: 03
subsystem: local-submission-relay
tags: [rust, rpc, node, transaction-relay, local-submission, parity, testing]
requirements-completed: [REL-03, REL-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 104-2026-07-01T14-38-26
generated_at: 2026-07-01T15:32:46.913Z
completed: 2026-07-01
---

# Phase 104 Plan 03: Local Submission Relay Evidence Summary

Added internal relay evidence for local transaction submissions while keeping
the public `sendrawtransaction` response shape unchanged.

## Evidence Path

RPC parse -> outcome-aware node admission -> serving cache update -> fanout queue evidence -> unchanged SendRawTransactionResponse.

The RPC method now decodes and parses the raw transaction, submits it through
the node outcome path, updates managed serving state for accepted/replaced
outcomes, records internal fanout evidence, and maps successful outcomes back to
the existing response fields: `txid_hex`, `replaced_txids`, and `evicted_txids`.

## Accomplishments

- Added `LocalRelaySubmissionEvidence`, `LocalRelaySubmissionLabel`, and
  `RebroadcastEvidenceLabel` with fixed labels only.
- Added `submit_local_transaction_outcome_at` for deterministic local
  submission tests and kept `submit_local_transaction_outcome` as a wrapper.
- Recorded local accepted/replaced outcomes as internal relay evidence with
  `accepted`, `queued`, `suppressed`, `not_eligible`, `relay_disabled`, and
  `rebroadcast_deferred` labels where applicable.
- Recorded duplicate, rejected, orphaned, evicted, and expired local outcomes as
  no-queue evidence, so those outcomes do not enqueue fanout.
- Routed `sendrawtransaction` through the outcome-aware local submission path.
- Preserved `sendrawtransaction` success JSON and kept it free of propagation,
  broadcast, public relay, or guarantee fields.

## Task Commit

- `86c8c6f2` - `feat(104-03): add local submission relay evidence`

## Key Files

- `packages/open-bitcoin-node/src/network/relay_fanout.rs`
- `packages/open-bitcoin-node/src/network/admission_bridge.rs`
- `packages/open-bitcoin-node/src/network/tests/relay_local_submission_cases.rs`
- `packages/open-bitcoin-rpc/src/context/network.rs`
- `packages/open-bitcoin-rpc/src/dispatch/node.rs`
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs`
- `docs/parity/source-breadcrumbs.json`
- `docs/metrics/lines-of-code.md`

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node relay_local_submission_cases -- --nocapture` passed: 3 tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc sendrawtransaction -- --nocapture` passed: 4 tests.
- `bash -lc '! rg -n "sleep|tokio::time|Instant::now|SystemTime::now" packages/open-bitcoin-node/src/network/admission_bridge.rs packages/open-bitcoin-node/src/network/relay_fanout.rs packages/open-bitcoin-node/src/network/tests/relay_local_submission_cases.rs'` passed.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node -p open-bitcoin-rpc --all-targets --all-features -- -D warnings` passed.
- `cargo fmt --manifest-path packages/Cargo.toml --all --check` passed.
- `bash scripts/check-file-lengths.sh` passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed.
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check` passed.
- `git diff --check` passed.
- `bash scripts/verify.sh` passed through the task commit hook.

## Boundaries

RPC success does not guarantee public propagation. Periodic rebroadcast
scheduling remains deferred beyond Phase 104; this plan records
`rebroadcast_deferred` evidence only and does not add timers, sleeps, or a
background broadcast loop.
