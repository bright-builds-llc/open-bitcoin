---
phase: 92-address-advertisement-and-discovery-boundaries
plan: 02
subsystem: networking
tags: [rust, open-bitcoin-network, getaddr, addr, wire-codec, version-message]

# Dependency graph
requires:
  - phase: 92-01
    provides: "local listener advertisement decisions and maybe_version_sender_address policy output"
provides:
  - "bounded legacy getaddr and addr wire message support"
  - "timestamped AddressAnnouncement and AddressList contract for learned-address intake"
  - "conservative version-message sender helper that defaults to zero address"
affects: [phase-92-getaddr-policy, phase-92-learned-addresses, network-peer-policy]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "pure wire parsing uses existing compact-size and 26-byte NetworkAddress codecs"
    - "sender address selection is consumed as policy input instead of inferred in runtime code"

key-files:
  created: []
  modified:
    - docs/metrics/lines-of-code.md
    - packages/open-bitcoin-network/src/address.rs
    - packages/open-bitcoin-network/src/message.rs
    - packages/open-bitcoin-network/src/message/tests.rs
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/peer/tests.rs

key-decisions:
  - "Kept addrv2 and sendaddrv2 unknown so Phase 92 does not overclaim full address relay."
  - "Kept existing LocalPeerConfig::version_message behavior unchanged and added a separate sender-policy helper."
  - "Deferred peer-level getaddr and addr policy by parsing messages but returning no peer actions until later bounded policy wiring."

patterns-established:
  - "Bound untrusted addr batches before record decoding through PHASE92_ADDR_BATCH_LIMIT."
  - "Use Option<NetworkAddress> as the pure policy boundary for version sender disclosure."

requirements-completed: [ADDR-01, ADDR-02, ADDR-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 92-2026-06-26T03-52-33
generated_at: 2026-06-26T06:15:44Z

# Metrics
duration: 18m 27s
completed: 2026-06-26
---

# Phase 92 Plan 02: Bounded Address Wire Summary

**Bounded legacy getaddr and addr wire parsing with conservative version sender-address disclosure.**

## Performance

- **Duration:** 18m 27s
- **Started:** 2026-06-26T05:57:17Z
- **Completed:** 2026-06-26T06:15:44Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- Added `WireNetworkMessage::GetAddr` with empty-payload validation and command name `getaddr`.
- Added timestamped legacy `addr` payload encode/decode using compact-size count plus the existing 26-byte network address codec.
- Enforced `PHASE92_ADDR_BATCH_LIMIT = 64` for `addr` payloads and covered over-limit encode/decode failures.
- Added `LocalPeerConfig::version_message_with_sender_policy`, which defaults sender address to zero unless a policy-approved `NetworkAddress` is supplied.
- Preserved `addrv2` and `sendaddrv2` as unknown commands and avoided relay, rebroadcast, trickle, DNS, NAT, or external-IP behavior.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add bounded `getaddr` and legacy `addr` payload support** - `5dcf5d3` (feat)
2. **Task 2: Gate version-message sender address** - `7259ab8` (feat)

## Files Created/Modified

- `docs/metrics/lines-of-code.md` - Hook-managed generated LOC report refreshed during task commits.
- `packages/open-bitcoin-network/src/address.rs` - Added the Phase 92 address batch limit and timestamped address-list types.
- `packages/open-bitcoin-network/src/message.rs` - Added `getaddr`/`addr` payload handling and the sender-policy version helper.
- `packages/open-bitcoin-network/src/message/tests.rs` - Added wire round-trip, payload cap, unknown `addrv2`, and sender-policy tests.
- `packages/open-bitcoin-network/src/peer.rs` - Deferred parsed address messages with no peer actions until bounded policy wiring.
- `packages/open-bitcoin-network/src/peer/tests.rs` - Covered the deferred peer action behavior for parsed address messages.

## Decisions Made

- Kept `LocalPeerConfig::version_message` unchanged for existing callers; new conservative behavior lives behind `version_message_with_sender_policy`.
- Parsed `getaddr` and legacy `addr` without wiring response policy or learned-address state, preserving this plan's wire-only boundary.
- Left `addrv2` and `sendaddrv2` unimplemented and explicitly tested as unknown commands.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added deferred peer handling for new address wire variants**
- **Found during:** Task 1 (Add bounded `getaddr` and legacy `addr` payload support)
- **Issue:** Adding `WireNetworkMessage::GetAddr` and `WireNetworkMessage::Addr` made the peer message-action match non-exhaustive.
- **Fix:** Added a no-action peer branch for parsed address messages, preserving the plan boundary until bounded getaddr policy wiring.
- **Files modified:** `packages/open-bitcoin-network/src/peer.rs`, `packages/open-bitcoin-network/src/peer/tests.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network address_messages_are_deferred_until_bounded_policy_wiring --no-fail-fast`
- **Committed in:** `5dcf5d3`

**2. [Rule 3 - Blocking] Added coverage for deferred peer branch**
- **Found during:** Task 1 pre-commit hook
- **Issue:** Hook coverage flagged the new peer branch as uncovered.
- **Fix:** Added a focused test proving `getaddr` and empty `addr` messages currently produce no peer actions.
- **Files modified:** `packages/open-bitcoin-network/src/peer/tests.rs`
- **Verification:** Normal hook-backed commit completed `bash scripts/verify.sh`.
- **Committed in:** `5dcf5d3`

**Total deviations:** 2 auto-fixed Rule 3 issues.
**Impact on plan:** No scope creep; the fixes keep the new wire variants buildable, covered, and explicitly deferred at the peer-policy boundary.

### Execution Adjustments

- The two plan tasks were TDD tasks. RED failures were captured locally, but failing-test commits were not created because this sequential run required normal git commits with hooks and no `--no-verify`; hook-backed commits cannot contain failing tests.

## Issues Encountered

- Task 1 RED failed as expected with missing `AddressAnnouncement`, `AddressList`, `PHASE92_ADDR_BATCH_LIMIT`, and `GetAddr`/`Addr` variants.
- Task 2 RED failed as expected with missing `version_message_with_sender_policy`.
- No authentication gates or external setup blockers occurred.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network message --no-fail-fast`
- `rg -n "PHASE92_ADDR_BATCH_LIMIT|struct AddressAnnouncement|struct AddressList" packages/open-bitcoin-network/src/address.rs`
- `rg -n "GetAddr|Addr\\(AddressList\\)|\"getaddr\"|\"addr\"" packages/open-bitcoin-network/src/message.rs packages/open-bitcoin-network/src/message/tests.rs`
- `rg -n "addrv2|sendaddrv2" packages/open-bitcoin-network/src/message/tests.rs`
- `! rg -n "MaybeSendAddr|PushAddress|trickle|fanout|rebroadcast|addrv2|sendaddrv2" packages/open-bitcoin-network/src/message.rs`
- `rg -n "version_message_with_sender_policy|maybe_sender|zero_address" packages/open-bitcoin-network/src/message.rs packages/open-bitcoin-network/src/message/tests.rs`
- `rg -n "local_peer_config_builds_expected_version_message" packages/open-bitcoin-network/src/message/tests.rs`
- `! rg -n "externalip|discover|interface|dns|UPnP|NAT" packages/open-bitcoin-network/src/message.rs`
- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- Normal commit hooks completed `bash scripts/verify.sh` for both task commits.

## Known Stubs

None - stub scan found no `TODO`, `FIXME`, placeholder text, empty mock data, or UI-facing empty placeholders in the plan-touched files.

## Threat Flags

None - new security-relevant surface was limited to the plan threat model: untrusted peer wire bytes parsed into typed messages and policy-approved sender addresses crossing into `version` payloads.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Later Phase 92 plans can consume `AddressList`, `AddressAnnouncement`, `WireNetworkMessage::GetAddr`, and `version_message_with_sender_policy` without revisiting the wire codec. Peer-level response policy, learned-address admission, and persistence evidence remain intentionally deferred.

## Self-Check: PASSED

- Found summary file at `.planning/phases/92-address-advertisement-and-discovery-boundaries/92-02-SUMMARY.md`.
- Found task commit `5dcf5d3`.
- Found task commit `7259ab8`.
- Confirmed `.planning/STATE.md`, `.planning/ROADMAP.md`, and `.planning/REQUIREMENTS.md` have no local diffs from this executor.

---
*Phase: 92-address-advertisement-and-discovery-boundaries*
*Completed: 2026-06-26*
