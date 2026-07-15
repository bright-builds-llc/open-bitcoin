---
phase: 122-compact-relay-peer-completion
reviewed: 2026-07-15T17:00:00Z
depth: standard
files_reviewed: 11
files_reviewed_list:
  - packages/open-bitcoin-network/src/peer/compact_relay.rs
  - packages/open-bitcoin-network/src/peer/message_dispatch.rs
  - packages/open-bitcoin-node/src/network/action_translation.rs
  - packages/open-bitcoin-node/src/network/block_serving.rs
  - packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs
  - docs/parity/index.json
  - docs/parity/catalog/p2p.md
  - docs/parity/checklist.md
  - scripts/verify.sh
  - scripts/check-phase122-compact-relay-peer-completion.ts
  - scripts/check-phase122-compact-relay-peer-completion.test.ts
findings:
  critical: 0
  warning: 1
  info: 0
  total: 1
status: issues_found
---

# Phase 122: Code Review Report

**Reviewed:** 2026-07-15
**Depth:** standard
**Files Reviewed:** 11
**Status:** issues_found

## Summary

The bounded per-peer provenance, ordered witness-bearing response construction, current block availability checks, malformed-index disconnect path, parity evidence, and verifier wiring are coherent. The review was materially informed by the repo-local `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, and the Bright Builds architecture, code-shape, verification, testing, operability, Rust, and TypeScript standards.

Focused verification passed:

- Phase 122 checker mutation tests: 14 passed, 0 failed.
- Phase 122 live checker: passed.
- `open-bitcoin-network` Phase 122 compact tests: 3 passed, 0 failed.
- `open-bitcoin-node` Phase 122 compact serving tests: 3 passed, 0 failed.
- `git diff --check` for the 11 reviewed files: passed.

## Warnings

### WR-01: Unannounced `getblocktxn` requests bypass the request-pressure gate

**File:** `packages/open-bitcoin-network/src/peer/message_dispatch.rs:104`

**Issue:** `handle_get_block_transactions` returns benign silence as soon as the requested hash is absent from the peer's announcement provenance (lines 104-106). Both differential-index expansion and the request-pressure check occur later (lines 108-126). Consequently, an unannounced request with an over-cap `index_deltas` vector never reaches `ResourceGovernancePolicy::decide_request`; a peer can repeatedly submit the largest decoder-accepted unannounced requests without triggering the Phase 94 request-cap disconnect. This contradicts the Phase 122 claim that inbound requests pass through shared request-pressure governance and leaves the highest-volume benign-suppression path outside that boundary. It also means the new checker would not catch the regression: `verifyTypedLiveResponse` only checks action/response token presence (`scripts/check-phase122-compact-relay-peer-completion.ts:118-152`), and its mutation test only replaces the response variant (`scripts/check-phase122-compact-relay-peer-completion.test.ts:63-67`).

**Fix:** Apply the raw request-count pressure check using `request.index_deltas.len()` before the provenance early return and before allocating the expanded index vector. Preserve silent handling for an in-cap unannounced hash, then expand indexes exactly once only for an authorized request. Add a focused peer test proving an over-cap unannounced request produces `ResourceGovernanceDisconnect`, plus a checker mutation that moves or removes this pre-provenance gate.

***

_Reviewer: gsd-code-reviewer_
_Depth: standard_
