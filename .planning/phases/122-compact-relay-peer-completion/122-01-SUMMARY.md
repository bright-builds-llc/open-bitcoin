---
phase: 122-compact-relay-peer-completion
plan: 01
subsystem: networking
tags: [bip152, compact-block-relay, getblocktxn, blocktxn, peer-policy, HARD-01]

requires:
  - phase: 113-compact-relay-negotiation-and-announcement-policy
    provides: version-2 compact relay capability and announcement eligibility
  - phase: 111-full-block-serving-request-path
    provides: shared block availability, eligibility, and resource-governance gates
provides:
  - bounded per-peer compact-announcement provenance with deterministic eviction
  - typed getblocktxn dispatch and ordered witness-preserving blocktxn serving
  - mutation-tested HARD-01 checker and explicit Knots fallback deviation evidence
affects:
  - 123-runtime-timing-and-evidence-integrity
  - v2.1 closeout evidence

tech-stack:
  added: []
  patterns:
    - Record peer provenance only after successful outbound payload construction
    - Reuse full-block serving policy gates for compact missing-transaction responses
    - Fixed-corpus Bun checker with one failing mutation per critical behavior group

key-files:
  created:
    - scripts/check-phase122-compact-relay-peer-completion.ts
    - scripts/check-phase122-compact-relay-peer-completion.test.ts
  modified:
    - packages/open-bitcoin-network/src/peer/compact_relay.rs
    - packages/open-bitcoin-network/src/peer/message_dispatch.rs
    - packages/open-bitcoin-node/src/network/action_translation.rs
    - packages/open-bitcoin-node/src/network/block_serving.rs
    - packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs
    - docs/parity/index.json
    - docs/parity/catalog/p2p.md
    - docs/parity/checklist.md
    - scripts/verify.sh

key-decisions:
  - "Retain at most eleven announced block hashes per peer session, matching Knots' inclusive ten-depth window without retaining block payloads"
  - "Treat unannounced, evicted, unavailable, or currently ineligible requests as benign silence"
  - "Treat differential-index overflow and live out-of-bounds indexes as compact-block misbehavior"
  - "Intentionally omit Knots' old-block full-witness-block fallback and record the scoped deviation"

patterns-established:
  - "Pattern: pure peer state authorizes a typed shell action; the node shell owns lookup and response effects"
  - "Pattern: response builders return served, suppressed, or malformed decisions so illegal optional payload states are unrepresentable"

requirements-completed:
  - HARD-01
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 122-2026-07-15T15-22-57
generated_at: 2026-07-15T16:19:48Z

duration: 35min
completed: 2026-07-15
---

# Phase 122 Plan 01: Compact Relay Peer Completion Summary

**Open Bitcoin now serves eligible `getblocktxn` requests only for compact blocks actually announced to that peer session, preserving transaction order and witness data behind bounded policy-aware state.**

## Performance

- **Duration:** 35 min
- **Completed:** 2026-07-15T16:19:48Z
- **Tasks:** 3
- **Files changed:** 21

## Accomplishments

- Added an idempotent eleven-hash FIFO plus membership set to each peer session and removed provenance automatically with peer cleanup.
- Replaced the Phase 112 `getblocktxn` no-op with exact-once differential-index expansion and a typed serving action restricted to the requesting peer's announcement history.
- Connected successful compact announcements to shared block-serving activation, eligibility, availability, and request-pressure gates, emitting ordered witness-bearing `blocktxn` payloads when eligible.
- Added deterministic local tests for peer isolation, eviction, unavailable and ineligible silence, disconnect/reconnect cleanup, overflow, and live out-of-bounds misbehavior.
- Added a fixed-corpus Phase 122 guard with fourteen passing mutation cases, default verifier wiring, pinned Knots anchors, and an explicit old-block fallback deviation.

## Task Commits

Each task was committed atomically:

1. **Task 1: Bounded peer provenance and typed request dispatch** — `b893aa15`
2. **Task 2: Live ordered witness-preserving response path** — `78fdc77c`
3. **Task 3: Deterministic checker, verifier wiring, and parity evidence** — `9fcfce35`

## Verification Results

```text
open-bitcoin-network compact tests: 133 passed, 0 failed
open-bitcoin-node compact tests: 49 passed, 0 failed
focused network + node clippy --all-targets --all-features -D warnings: passed
workspace cargo check --all-targets --all-features: passed
Phase 122 checker mutation tests: 14 passed, 0 failed
Phase 122 live checker: passed
Phase 117 parity closeout checker: passed
parity breadcrumbs: 380 Rust files verified
production Rust file-length policy: 289 files verified
git diff --check: passed
```

The orchestrator owns the final repository-wide `bash scripts/verify.sh` gate, so this worktree did not duplicate that long-running check.

## Decisions Made

- Provenance is recorded only after `announce_block_with_action` returns an actual `CompactBlock`; eligibility intent and full-block fallback do not create a token.
- The response path reuses `ManagedBlockServeInput` and the existing resource-pressure accounting instead of introducing compact-relay-specific policy drift.
- Stored transaction objects are cloned in requested index order, preserving witness data without re-encoding through a stripped representation.
- Knots' old-block full-witness-block response fallback remains deliberately absent; Open Bitcoin silently suppresses untracked or unavailable requests.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Updated the parity test harness for the new exhaustive peer action**

- **Found during:** Task 1 focused compilation
- **Issue:** Adding `ServeCompactBlockTransactions` made the parity harness action match non-exhaustive.
- **Fix:** Added the new typed action arm without changing parity behavior.
- **Files modified:** `packages/open-bitcoin-network/tests/parity.rs`
- **Commit:** `b893aa15`

**2. [Rule 1 - Bug] Restored an accidentally omitted crate-root export**

- **Found during:** Task 2 workspace check
- **Issue:** The Task 1 export edit inadvertently dropped `PeerAddressBoundaryDecision` from `open-bitcoin-network`'s public re-exports.
- **Fix:** Restored the existing export and reran focused clippy plus workspace checking.
- **Files modified:** `packages/open-bitcoin-network/src/lib.rs`
- **Commit:** `78fdc77c`

## Issues Encountered

- Consensus validation rejects unexpected witness data when the synthetic block lacks a witness commitment. The serving test therefore validates the no-witness block, then replaces the local cached payload under the same transaction-ID/merkle-root-derived hash with witness-bearing transaction clones. This isolates and proves the response path's witness preservation without weakening validation or requiring public-network access.

## Residual Risks

- The final full repository verification, coverage, and Bazel smoke build remain the orchestrator's final gate.
- Archive-node behavior, Knots' old-block full-block fallback, public compact-relay defaults, public-network CI, and production readiness remain explicitly outside this phase.

## Self-Check: PASSED

- FOUND: `packages/open-bitcoin-network/src/peer/compact_relay.rs` (`MAX_COMPACT_ANNOUNCEMENT_PROVENANCE = 11`)
- FOUND: `packages/open-bitcoin-node/src/network/action_translation.rs` (`ServeCompactBlockTransactions` translation)
- FOUND: `scripts/check-phase122-compact-relay-peer-completion.ts`
- FOUND: `scripts/check-phase122-compact-relay-peer-completion.test.ts`
- FOUND: `.planning/phases/122-compact-relay-peer-completion/122-01-SUMMARY.md`
