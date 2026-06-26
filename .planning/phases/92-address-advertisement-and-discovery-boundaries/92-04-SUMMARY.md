---
phase: 92-address-advertisement-and-discovery-boundaries
plan: 04
subsystem: network
tags: [rust, network, peer-manager, addr, getaddr, parity]

requires:
  - phase: 92-01
    provides: Local address advertisement policy and labels
  - phase: 92-02
    provides: Learned address book and inbound addr policy
  - phase: 92-03
    provides: Bounded getaddr response selection policy
provides:
  - PeerManager inbound addr intake through LearnedAddressBook policy
  - Permission-aware bounded getaddr responses with per-peer served state
  - Address-boundary evidence snapshot for learned entries, rejections, served responses, and suppressions
  - Version sender-address gating through local advertisement decisions
affects: [network-peer, address-boundary, inbound-permissions, parity-evidence]

tech-stack:
  added: []
  patterns:
    - Pure PeerManager message handling delegates address decisions to address policy types
    - Address-boundary peer evidence stays bounded and testable without relay state

key-files:
  created:
    - packages/open-bitcoin-network/src/peer/address_boundary.rs
    - packages/open-bitcoin-network/src/peer/inventory_state.rs
  modified:
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/peer/tests.rs
    - packages/open-bitcoin-network/src/lib.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Keep addr and getaddr integration in the pure peer manager and record evidence instead of adding persistence, relay fanout, bans, or eviction."
  - "Gate getaddr responses by inbound role plus typed Phase 91 address-response permission evidence."
  - "Split peer address-boundary helpers out of peer.rs to satisfy the repo production file-length verifier."

patterns-established:
  - "Inbound addr handling records learned-address evidence through policy labels without disconnect side effects."
  - "Getaddr handling returns at most one direct Addr response per peer state and records stable suppression reasons."

requirements-completed: [ADDR-01, ADDR-02, ADDR-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 92-2026-06-26T03-52-33
generated_at: 2026-06-26T07:32:36Z

duration: 38m 11s
completed: 2026-06-26
---

# Phase 92 Plan 04: Peer Address Boundary Summary

**Pure peer-message integration for inbound addr intake and permission-aware bounded getaddr responses without full address relay machinery**

## Performance

- **Duration:** 38m 11s
- **Started:** 2026-06-26T06:54:25Z
- **Completed:** 2026-06-26T07:32:36Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments

- Added `PeerManager` address-boundary state and `address_boundary_evidence()` for local advertisements, learned address entries, rejections, getaddr responses, getaddr suppressions, and latest decision labels.
- Routed inbound `WireNetworkMessage::Addr` through `LearnedAddressBook` policy so accepted and rejected entries are recorded without disconnect, persistence, ban, eviction, or relay side effects.
- Added per-peer `GetAddrRequestState` and permission-aware `WireNetworkMessage::GetAddr` handling that serves one bounded direct `Addr` response only to eligible inbound peers.
- Wired inbound version responses through `version_message_with_sender_policy` so suppressed local advertisements keep the sender address empty.

## Task Commits

1. **Task 1: Handle inbound `addr` learned-address intake** - `7ba5ce4` (feat)
2. **Task 2: Handle bounded permission-aware `getaddr` responses** - `95ad524` (feat)

## Files Created/Modified

- `packages/open-bitcoin-network/src/peer/address_boundary.rs` - Address-boundary evidence, addr intake, local decision injection, and getaddr response handling.
- `packages/open-bitcoin-network/src/peer/inventory_state.rs` - Existing inventory request helper split out of `peer.rs` to keep production file size within repo limits.
- `packages/open-bitcoin-network/src/peer.rs` - Peer state now tracks getaddr request state and dispatches addr/getaddr through address-boundary helpers.
- `packages/open-bitcoin-network/src/peer/tests.rs` - Peer tests for learned addr intake, rejection labels, over-cap batches, getaddr permission gating, repeat suppression, version sender policy, and no unsolicited addr flows.
- `packages/open-bitcoin-network/src/lib.rs` - Public peer address-boundary evidence exports.
- `docs/parity/source-breadcrumbs.json` - Breadcrumb registration for new first-party Rust source.
- `docs/metrics/lines-of-code.md` - Regenerated tracked LOC artifact from hooks.

## Decisions Made

- Address handling stays in the pure peer manager, with policy decisions imported from `open-bitcoin-network::address`, rather than adding a new runtime relay or discovery service.
- `getaddr` eligibility uses typed `PermissionEffectLabel::AddressResponsePolicyInput` evidence and inbound role checks, not raw permission class names.
- The response cache is built from local advertisement candidates plus learned address entries and capped by `PHASE92_GETADDR_RESPONSE_LIMIT`.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network peer --no-fail-fast`
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings`
- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- Normal commit hooks ran `bash scripts/verify.sh` successfully for both task commits.
- Acceptance greps confirmed the learned-address evidence contract, getaddr response labels, bounded response cap, and absence of forbidden relay/fanout machinery.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Split peer helpers to satisfy production file-length verification**
- **Found during:** Task 1
- **Issue:** Adding address-boundary behavior pushed `packages/open-bitcoin-network/src/peer.rs` over the production Rust file-length limit enforced by hooks.
- **Fix:** Moved address-boundary logic to `packages/open-bitcoin-network/src/peer/address_boundary.rs` and moved existing inventory request state logic to `packages/open-bitcoin-network/src/peer/inventory_state.rs`.
- **Files modified:** `packages/open-bitcoin-network/src/peer.rs`, `packages/open-bitcoin-network/src/peer/address_boundary.rs`, `packages/open-bitcoin-network/src/peer/inventory_state.rs`
- **Verification:** Production file-length hook passed in `bash scripts/verify.sh`.
- **Committed in:** `7ba5ce4`

**2. [Rule 3 - Blocking] Added parity breadcrumb for new Rust source**
- **Found during:** Task 1
- **Issue:** The new address-boundary source file needed a required parity breadcrumb entry before hooks would pass.
- **Fix:** Ran the repo breadcrumb updater and committed `docs/parity/source-breadcrumbs.json`.
- **Files modified:** `packages/open-bitcoin-network/src/peer/address_boundary.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** Parity breadcrumb verification passed in `bash scripts/verify.sh`.
- **Committed in:** `7ba5ce4`

**3. [Rule 3 - Blocking] Filled coverage branches exposed by hooks**
- **Found during:** Task 1
- **Issue:** Coverage verification identified address-boundary branches that needed direct tests.
- **Fix:** Added peer tests for unknown peer, empty addr, local duplicate, over-cap batch, rejection, and outbound addr paths.
- **Files modified:** `packages/open-bitcoin-network/src/peer/tests.rs`
- **Verification:** Coverage and `bash scripts/verify.sh` passed.
- **Committed in:** `7ba5ce4`

**Total deviations:** 3 auto-fixed Rule 3 blocking issues.
**Impact on plan:** Scope stayed within the planned peer/address boundary. The extra work was required by repo verification and did not add relay, persistence, ban, eviction, or discovery machinery.

## Issues Encountered

- TDD RED tests were executed before GREEN implementation, but failing RED tests were not committed separately because the user required normal hooks and no `--no-verify`; repo hooks cannot pass intentionally failing tests.
- The pre-existing `.planning/config.json` working-tree change was left untouched and uncommitted.

## Stub Scan

No known stubs were introduced. The scan found no `TODO`, `FIXME`, placeholder wording, or hardcoded empty UI/data values in the files created or modified by this plan.

## Threat Surface Scan

No threat flags beyond the plan threat model. The new peer-message handling stays within the documented `peer message -> PeerManager` and `permission evidence -> address response` trust boundaries, with bounded response caps and typed permission checks.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

The peer manager can now process bounded address intake and direct getaddr responses using Phase 92 address policies and Phase 91 permission evidence. Future discovery work can build on the evidence contracts while preserving the no-relay boundary unless a later plan explicitly adds full address-manager persistence or relay behavior.

---
*Phase: 92-address-advertisement-and-discovery-boundaries*
*Completed: 2026-06-26*

## Self-Check: PASSED

- `FOUND: .planning/phases/92-address-advertisement-and-discovery-boundaries/92-04-SUMMARY.md`
- `FOUND: 7ba5ce4`
- `FOUND: 95ad524`
