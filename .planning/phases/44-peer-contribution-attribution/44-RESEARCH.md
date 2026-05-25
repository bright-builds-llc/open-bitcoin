# Phase 44: Peer Contribution Attribution - Research

**Researched:** 2026-05-25 [VERIFIED: environment_context]
**Requirement:** PEER-03 - daemon sync records per-peer header and block contribution so idle or failing peers are not reported as useful sync progress. [VERIFIED: .planning/REQUIREMENTS.md]
**Confidence:** HIGH for implementation seams; MEDIUM for exact helper names, which are planner discretion. [VERIFIED: .planning/phases/44-peer-contribution-attribution/44-CONTEXT.md]

## User Constraints (from CONTEXT.md)

- Keep attribution in `DurableSyncRuntime` and the existing sync/status projection path; do not add a second telemetry channel or peer reputation system. [VERIFIED: .planning/phases/44-peer-contribution-attribution/44-CONTEXT.md]
- Treat `messages_processed` and `maybe_last_activity_unix_seconds` as activity evidence, not useful sync contribution. [VERIFIED: .planning/phases/44-peer-contribution-attribution/44-CONTEXT.md]
- Count header contribution only after headers are accepted through the sync validation path. [VERIFIED: .planning/phases/44-peer-contribution-attribution/44-CONTEXT.md]
- Count block contribution only after the block is accepted through the block sync path and preserved for connection/reconciliation. [VERIFIED: .planning/phases/44-peer-contribution-attribution/44-CONTEXT.md]
- Idle, stalled, waiting, and failed peers must stay visible without being credited for useful header/block progress. [VERIFIED: .planning/phases/44-peer-contribution-attribution/44-CONTEXT.md]
- Preserve Phase 43 waiting/backoff behavior and keep public-network checks out of `bash scripts/verify.sh`. [VERIFIED: .planning/phases/43-outbound-peer-resilience/43-01-SUMMARY.md, .planning/phases/44-peer-contribution-attribution/44-CONTEXT.md]
- Deferred: peer scoring/reputation/eviction heuristics, Phase 45 resource bounds, restart/recovery proof, and final public-mainnet evidence. [VERIFIED: .planning/phases/44-peer-contribution-attribution/44-CONTEXT.md]

## Key Findings

- `PeerContribution` already exists with `messages_processed`, `headers_received`, and `blocks_received`, and `PeerSyncOutcome` already carries it into sync summaries. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs]
- `PeerTelemetry` already exposes per-peer `headers_received`, `blocks_received`, last activity, failure reason, and error through durable status projection. [VERIFIED: packages/open-bitcoin-node/src/status.rs, packages/open-bitcoin-node/src/sync/types/projection.rs]
- The current bug is timing: `PeerProgress::record_message` counts header/block messages before `receive_sync_message` validates or accepts them. [VERIFIED: packages/open-bitcoin-node/src/sync.rs, packages/open-bitcoin-node/src/sync/progress.rs]
- Failed connected peers currently lose partial activity because `PeerFailure` does not carry the in-progress `PeerProgress`, and `record_outcome` creates a fresh failed outcome with zero activity. [VERIFIED: packages/open-bitcoin-node/src/sync.rs, packages/open-bitcoin-node/src/sync/progress.rs]
- The live-smoke script already parses durable `recent_peers`, but normalized `RuntimePeerTelemetry` drops header/block contribution fields and Markdown does not render a peer contribution table. [VERIFIED: scripts/run-live-mainnet-smoke.ts]

## Implementation Seams

| Seam | What To Change | Why |
|---|---|---|
| `PeerProgress` accounting | Split `record_message` into activity and contribution helpers, for example `record_activity`, `record_validated_headers`, and `record_accepted_block`. | Current helper counts useful contribution before validation. [VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs] |
| `sync_connected_peer` message loop | Keep activity update immediately after receiving a message, but increment header contribution only after `receive_sync_message` succeeds. | Header validation happens through `ManagedPeerNetwork::receive_sync_message` and `validate_header_for_sync`. [VERIFIED: packages/open-bitcoin-node/src/sync.rs, packages/open-bitcoin-node/src/network.rs, packages/open-bitcoin-node/src/network/header_sync.rs] |
| `sync_connected_peer` block path | Increment block contribution only after `receive_sync_message` succeeds and `store.save_block` completes. | Phase 44 requires accepted/preserved blocks, not merely observed block messages. [VERIFIED: packages/open-bitcoin-node/src/sync.rs] |
| Connected failure path | Carry `maybe_progress: Option<PeerProgress>` or equivalent in `PeerFailure`, using it when converting failed connected peers into outcomes. | Invalid-data peers must retain last activity and failure reason while keeping unaccepted contribution at zero. [VERIFIED: packages/open-bitcoin-node/src/sync.rs, .planning/phases/44-peer-contribution-attribution/44-CONTEXT.md] |
| Status projection | Preserve existing JSON field names; update tests/docs to define `headers_received` and `blocks_received` as validation-gated contribution. | Existing consumers already depend on these fields. [VERIFIED: packages/open-bitcoin-node/src/status.rs, scripts/run-live-mainnet-smoke.ts] |
| Live-smoke report | Add `headersReceived` and `blocksReceived` to normalized runtime peer telemetry and render a `Runtime Peer Contributions` Markdown table. | Final reports need per-peer contribution rows from durable peer telemetry. [VERIFIED: scripts/run-live-mainnet-smoke.ts, .planning/phases/44-peer-contribution-attribution/44-CONTEXT.md] |

## Recommended Plan Shape

1. Update `PeerProgress` helpers so activity and useful contribution are separate. [VERIFIED: packages/open-bitcoin-node/src/sync/progress.rs]
2. Move header/block contribution increments in `sync_connected_peer` to after validation/preservation succeeds. [VERIFIED: packages/open-bitcoin-node/src/sync.rs]
3. Preserve partial progress on connected-peer failures without changing pre-connect failure behavior. [VERIFIED: packages/open-bitcoin-node/src/sync.rs]
4. Add deterministic sync tests for accepted contribution, invalid headers with zero contribution plus retained activity, stalled/idle zero contribution, and mixed failure retention. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]
5. Extend live-smoke JSON/Markdown evidence and its fixture test. [VERIFIED: scripts/run-live-mainnet-smoke.ts, scripts/test-run-live-mainnet-smoke.sh]
6. Update operator docs to explain activity vs contribution counters. [VERIFIED: docs/operator/runtime-guide.md]

## Files Likely To Change

| File | Expected Change |
|---|---|
| `packages/open-bitcoin-node/src/sync/progress.rs` | Rename/split accounting helpers and possibly extend `PeerFailure` with partial progress. |
| `packages/open-bitcoin-node/src/sync.rs` | Move contribution increments after acceptance and preserve partial progress in connected failures. |
| `packages/open-bitcoin-node/src/sync/tests.rs` | Add/adjust deterministic regressions for PEER-03. |
| `scripts/run-live-mainnet-smoke.ts` | Preserve/render per-peer contribution fields from durable `recent_peers`. |
| `scripts/test-run-live-mainnet-smoke.sh` | Extend mocked final status and Markdown/JSON assertions. |
| `docs/operator/runtime-guide.md` | Document contribution semantics for operators. |
| `docs/metrics/lines-of-code.md` | May update only if repo verification regenerates it. [VERIFIED: AGENTS.md] |

## Risks And Pitfalls

- Counting before `receive_sync_message` succeeds will still credit invalid headers or failed blocks. [VERIFIED: packages/open-bitcoin-node/src/sync.rs]
- Creating a fresh failed outcome for invalid-data peers will hide last activity and make them look idle. [VERIFIED: packages/open-bitcoin-node/src/sync.rs]
- Renaming status fields would break existing durable status and live-smoke consumers; keep field names and clarify semantics. [VERIFIED: packages/open-bitcoin-node/src/status.rs, scripts/run-live-mainnet-smoke.ts]
- Waiting/backoff peers must remain non-attributed; Phase 43 behavior should not regress. [VERIFIED: .planning/phases/43-outbound-peer-resilience/43-01-SUMMARY.md]
- Do not add public-network tests to default verification. [VERIFIED: .planning/REQUIREMENTS.md, scripts/verify.sh]

## Deterministic Verification Commands

Use targeted commands during implementation:

```bash
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features peer_contribution
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features contextual_invalid_headers_fail_with_typed_invalid_data
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features mixed_peer_failures_rotate_to_replacement_without_corrupting_state
bash scripts/test-run-live-mainnet-smoke.sh
```

Use repo-native completion verification before marking the implementation phase done:

```bash
bash scripts/verify.sh
```

## Resolved Question

Should already-known but valid duplicate headers/blocks count as contribution?
Resolved: Phase 44 counts data that the existing sync path accepts, including
already-known but valid headers or blocks when the underlying handler returns
success. This keeps the phase scoped to validation-gated attribution and avoids
adding a new duplicate-deduplication or peer scoring policy. A future phase may
separate novel contribution from accepted duplicate activity if operator
evidence shows that distinction is needed. [VERIFIED:
packages/open-bitcoin-network/src/header_store.rs,
.planning/phases/44-peer-contribution-attribution/44-CONTEXT.md]

## Sources

- `.planning/phases/44-peer-contribution-attribution/44-CONTEXT.md`
- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/phases/43-outbound-peer-resilience/43-01-SUMMARY.md`
- `packages/open-bitcoin-node/src/sync.rs`
- `packages/open-bitcoin-node/src/sync/progress.rs`
- `packages/open-bitcoin-node/src/sync/types.rs`
- `packages/open-bitcoin-node/src/sync/types/projection.rs`
- `packages/open-bitcoin-node/src/sync/tests.rs`
- `packages/open-bitcoin-node/src/network.rs`
- `packages/open-bitcoin-node/src/network/header_sync.rs`
- `packages/open-bitcoin-node/src/status.rs`
- `scripts/run-live-mainnet-smoke.ts`
- `scripts/test-run-live-mainnet-smoke.sh`
- `docs/operator/runtime-guide.md`
