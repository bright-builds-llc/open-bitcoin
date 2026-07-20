---
phase: 128-production-compact-announcement-transport
plan: "01"
subsystem: network
tags: [bip152, sendcmpct, handshake, compact-relay]
requires:
  - phase: 113-compact-relay-negotiation-and-announcement-policy
    provides: Remote-derived compact negotiation and announcement policy
provides:
  - Directional local compact-relay offer state separate from remote preference
  - Activation-gated post-Verack version-2 low-bandwidth sendcmpct emission
  - Deterministic handshake, idempotence, and production composition regressions
affects: [128-02, 128-03, compact-relay, peer-transport]
tech-stack:
  added: []
  patterns:
    - Pure one-shot negotiation transition followed by existing PeerAction transport
    - Local offer state remains separate from remote-derived compact preferences
key-files:
  created: []
  modified:
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/peer/compact_relay.rs
    - packages/open-bitcoin-network/src/peer/message_dispatch.rs
    - packages/open-bitcoin-network/src/peer/tests.rs
    - packages/open-bitcoin-rpc/tests/black_box_parity.rs
key-decisions:
  - Store local compact offers separately from remote sendcmpct capability and preference.
  - Append sendcmpct(false, 2) after existing post-Verack actions only when explicit compact activation and protocol support are present.
patterns-established:
  - Local compact offer scheduling is a typed idempotent transition.
  - Production consumers account for the post-Verack compact negotiation frame.
requirements-completed:
  - CMP-04
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 128-2026-07-20T01-54-33
generated_at: 2026-07-20T03:36:00Z
duration: 44 min
completed: 2026-07-20
---

# Phase 128 Plan 01: Production Compact Negotiation Summary

Activation-gated post-Verack `sendcmpct(false, 2)` negotiation with directional local/remote state and one-shot ordering guarantees.

## Performance

- **Duration:** 44 min
- **Started:** 2026-07-20T02:52:00Z
- **Completed:** 2026-07-20T03:36:00Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Added a pure, typed local compact-relay offer transition gated by activation, handshake completion, and BIP152 protocol support.
- Routed exactly one low-bandwidth version-2 `sendcmpct` frame through the existing peer action transport after `Verack`.
- Locked directional independence, ordering, idempotence, disabled/unsupported behavior, and production composition into deterministic regressions.

## Task Commits

Each task was committed atomically:

1. **Task 1: Model directional local compact-relay offer state** - `d363fbf0`
2. **Task 2: Emit and verify the post-Verack compact offer** - `74b48b6e`

## Files Created/Modified

- `packages/open-bitcoin-network/src/peer.rs` - Stores remote protocol version and local offer state, and exposes the one-shot scheduling transition.
- `packages/open-bitcoin-network/src/peer/compact_relay.rs` - Defines the BIP152 protocol floor, typed local offer state, and pure offer policy.
- `packages/open-bitcoin-network/src/peer/message_dispatch.rs` - Records the negotiated protocol version and appends the post-Verack offer action.
- `packages/open-bitcoin-network/src/peer/tests.rs` - Covers pure policy and handshake-level transport behavior.
- `packages/open-bitcoin-rpc/tests/black_box_parity.rs` - Verifies production composition consumes the required negotiation frame before block traffic.

## Decisions Made

- Kept local offer state directional and independent from remote `sendcmpct` capability and preference so outbound negotiation cannot overwrite inbound policy.
- Reused `PeerAction::Send` for transport and preserved the pre-existing post-Verack action order before appending `sendcmpct(false, 2)`.
- Required explicit compact-relay activation plus remote protocol version `70014` or newer; transaction-relay activation alone does not enable the offer.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Updated production composition receive ordering**

- **Found during:** Task 2
- **Issue:** The Phase 127 black-box composition test expected a block frame immediately after `Verack`, while the newly required `SendCompact` negotiation frame correctly became the first received frame.
- **Fix:** Consumed and asserted the exact low-bandwidth version-2 offer before retaining the original block request/response assertions.
- **Files modified:** `packages/open-bitcoin-rpc/tests/black_box_parity.rs`
- **Verification:** Targeted black-box regression, ordered workspace Rust checks, and `bash scripts/verify.sh`
- **Commit:** `74b48b6e`

***

**Total deviations:** 1 auto-fixed bug.

**Impact on plan:** The adjustment preserves the existing production composition assertion while making it accurately model the required post-handshake wire ordering. No scope expansion or architectural change was introduced.

## Issues Encountered

- The first aggregate verifier run found a stale tracked LOC report. `docs/metrics/lines-of-code.md` was regenerated as shared phase state and intentionally left unstaged for Plan 04 ownership; the retained aggregate rerun then passed.
- TDD RED failures were observed before implementation. Failing RED commits were not created because the repository instruction contract requires every Rust commit to pass formatting, lint, build, and tests.

## Verification

- `cargo fmt --all`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo build --all-targets --all-features`
- `cargo test --all-features`
- Focused Phase 128 compact-relay and peer tests
- Targeted Phase 127 production composition black-box regression
- `bash scripts/verify.sh` — passed in 3m 23.953s, including Bazel provenance and coverage

## Known Stubs

None. The modified runtime and test paths contain no placeholder, TODO, FIXME, or unwired empty-data behavior.

## User Setup Required

None.

## Next Phase Readiness

- The peer layer now produces the exact local negotiation frame expected by subsequent compact announcement transport work.
- Plan 128-02 can consume the established directional state without conflating the local offer with remote high-bandwidth preferences.

## Self-Check: PASSED

- All five modified implementation and regression files exist.
- Task commits `d363fbf0` and `74b48b6e` are present.
- Summary frontmatter and whitespace checks pass.
