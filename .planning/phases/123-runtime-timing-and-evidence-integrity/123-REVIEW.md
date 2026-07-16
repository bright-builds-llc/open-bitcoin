---
phase: 123-runtime-timing-and-evidence-integrity
reviewed: 2026-07-16T06:23:58Z
depth: standard
diff_base: 731f70e8334bad9980f3a68a6f50e7019885fdd4
fix_review_base: a4eccfe3
reviewed_head: 0111d6c1
files_reviewed: 35
files_reviewed_list:
  - docs/architecture/operator-observability.md
  - docs/metrics/lines-of-code.md
  - docs/parity/catalog/p2p.md
  - docs/parity/checklist.md
  - docs/parity/index.json
  - packages/open-bitcoin-bench/src/runtime_fixtures.rs
  - packages/open-bitcoin-network/src/message.rs
  - packages/open-bitcoin-node/src/lib.rs
  - packages/open-bitcoin-node/src/logging.rs
  - packages/open-bitcoin-node/src/logging/tests.rs
  - packages/open-bitcoin-node/src/metrics/block_relay.rs
  - packages/open-bitcoin-node/src/metrics/tests.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/block_relay_evidence.rs
  - packages/open-bitcoin-node/src/network/tests.rs
  - packages/open-bitcoin-node/src/sync.rs
  - packages/open-bitcoin-node/src/sync/metrics.rs
  - packages/open-bitcoin-node/src/sync/runtime_state.rs
  - packages/open-bitcoin-node/src/sync/session.rs
  - packages/open-bitcoin-node/src/sync/tcp.rs
  - packages/open-bitcoin-node/src/sync/tests.rs
  - packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs
  - packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs
  - packages/open-bitcoin-node/src/sync/tests/runtime_write_evidence_cases.rs
  - packages/open-bitcoin-node/src/sync/types.rs
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/src/context.rs
  - packages/open-bitcoin-rpc/src/context/network.rs
  - packages/open-bitcoin-rpc/src/inbound_listener.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs
  - scripts/check-phase121-block-relay-metrics-log-runtime.test.ts
  - scripts/check-phase121-block-relay-metrics-log-runtime.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity.test.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity.ts
  - scripts/verify.sh
findings:
  critical: 1
  warning: 0
  info: 0
  total: 1
status: issues_found
---

# Phase 123: Code Review Report

**Reviewed:** 2026-07-16T06:23:58Z
**Depth:** standard
**Files Reviewed:** 35
**Original Diff Base:** `731f70e8334bad9980f3a68a6f50e7019885fdd4`
**Final Fix Review Base:** `a4eccfe3`
**Reviewed Head:** `0111d6c1`
**Status:** issues_found

## Summary

Re-reviewed the original 35-file Phase 123 scope at the final merged fix head, plus directly adjacent compact-download state, block-request tracking, response classification, and daemon cancellation files needed to validate the complete runtime call chains. Repo-local guidance materially used for this review: `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/operability.md`, `standards/core/verification.md`, `standards/core/testing.md`, `standards/languages/rust.md`, and `standards/languages/typescript-javascript.md`.

The iteration-2 findings are fixed as requested. Compact work now survives thirteen default-cadence 5-second idle wakes until the greater-than-60-second expiration point; sessions without compact work yield on their first idle; daemon cancellation remains checked before receive and immediately after each idle receive; every `Message` samples the injected clock before activity, dispatch, and reconciliation; and the checker no longer requires a fixed idle-count literal. Production activation remains correctly default-off and correctly wired when explicitly enabled, post-write evidence remains effect-based, and metric/log projection still shares one authoritative snapshot.

One critical end-to-end gap remains at the timeout transition itself. The runtime writes the full-block fallback, but clears the only state used to retain the session and immediately tears the session down instead of polling it again. The fallback is also absent from ordinary requested-block tracking, so its response would not be accepted even if the session were retained.

## Fix Disposition

| Iteration-2 finding | Disposition | Evidence |
| --- | --- | --- |
| CR-01: fixed two-idle cutoff preceded compact timeout | Resolved through expiration | The fixed cutoff is gone; compact in-flight state retains the session through thirteen 5-second wakes and emits fallback after 65 seconds. |
| WR-01: messages without an idle used stale timestamps | Resolved | The `Message` branch samples `controls.0` before incrementing progress or dispatching; a slow-message test covers a late compact receipt with no preceding idle. |

Verification performed during this final review:

- `bun test scripts/check-phase123-runtime-timing-evidence-integrity.test.ts` passed (29 tests)
- `bun run scripts/check-phase123-runtime-timing-evidence-integrity.ts` passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase123_ -- --nocapture` passed through the repo timing wrapper (26 tests)
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check` passed
- `bash -n scripts/verify.sh` passed
- `git diff --check a4eccfe3..HEAD` passed

The orchestrator reported that the full `bash scripts/verify.sh` contract and remaining focused suites had already passed at `0111d6c1`; the full contract was not duplicated during this review.

## Critical Findings

### CR-01: Timeout Fallback Is Followed By Immediate Disconnect And Cannot Be Consumed

**File:** `packages/open-bitcoin-node/src/sync/session.rs:90`
**Issue:** On the expiration wake, `expire_compact_download_timeouts` removes the expired entry from `CompactDownloadPeerState::in_flight` and returns `GetData(Block)`. The session writes that message at lines 103-107, then immediately evaluates `peer_has_compact_download_in_flight`. Because expiration just cleared the entry, the predicate is false, the branch returns, and the common epilogue calls `disconnect_peer` at line 218 and drops the `TcpPeerSession`. This violates the locked Phase 123 Plan 01 requirement to continue polling after same-peer sends: the peer receives a request on a connection that is closed before the runtime can read its response. There is a second break in the same call chain: compact-timeout expiration constructs `GetData` directly and never inserts the hash into the peer's ordinary `requested_blocks` set. `sync_connected_peer_with_cancel` recognizes Block responses only through `peer_requested_block`; therefore a matching response would enter `record_unrequested_block_response`, where `was_requested == false` prevents an otherwise connected best-chain block from being saved or credited. The new cadence test and checker stop after asserting that outbound `GetData` exists, so both pass without proving a usable fallback.
**Fix:** Treat timeout as a state transition from compact in-flight work to tracked full-block fallback work, not as completion. Before or atomically with the successful fallback send, register the hash in the peer's requested-block state. Continue polling the same session after the send, bounded by cancellation and a response/read timeout; clear the tracked request on matching `Block` or `NotFound`. Yield only when neither compact work nor a just-issued full-block fallback is awaiting a response. Add an end-to-end scripted test with thirteen 5-second idles followed by the matching `Block` response, and assert the response is classified as requested, accepted/persisted, and credited before the session yields. Extend the checker and mutation suite to require the tracked transition and response-consumption regression, not only fallback emission.

***

_Reviewed: 2026-07-16T06:23:58Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
