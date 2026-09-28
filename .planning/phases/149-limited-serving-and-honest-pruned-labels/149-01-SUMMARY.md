---
phase: 149-limited-serving-and-honest-pruned-labels
plan: "01"
subsystem: networking
tags: [network-limited, service-flags, bip159, prune, parity-breadcrumbs]

requires:
  - phase: 147-pure-prune-policy-and-lock-windows
    provides: PruneMode and MIN_BLOCKS_TO_KEEP=288
provides:
  - ServiceFlags::NETWORK_LIMITED service bit 10
  - advertised_service_flags from PruneMode
  - block_request_exceeds_limited_serve_window for tip distance greater than 288 plus 2
  - network-limited-serve parity breadcrumb group
affects:
  - 149-02 version-message advertisement
  - 149-03 out-of-window block-body refusal

tech-stack:
  added: []
  patterns:
    - "I/O-free limited-serve facts in open-bitcoin-network, importing the chainstate keep window"

key-files:
  created:
    - packages/open-bitcoin-network/src/limited_serve.rs
  modified:
    - packages/open-bitcoin-network/src/message.rs
    - packages/open-bitcoin-network/src/lib.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Combined 149-01 Task 1 and Task 2 into one hook-passing feat commit because pre-commit runs verify.sh and new Rust paths require breadcrumbs"
  - "SERV-01 is activated in 149-02-SUMMARY.md because 149-VERIFICATION.md is passed"
  - "A block is outside the limited serve window only when tip distance is greater than MIN_BLOCKS_TO_KEEP + 2; deletion still keeps 288"

patterns-established:
  - "advertised_service_flags reads PruneMode only: Disabled is NETWORK | WITNESS; every other mode is NETWORK_LIMITED | WITNESS"
  - "network-limited-serve cites protocol.h, init.cpp, net_processing.cpp, and validation.h"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 149-2026-09-27T19-48-11
generated_at: 2026-09-27T22:12:45Z

duration: 27min
completed: 2026-09-27
---

# Phase 149 Plan 01: Pure Limited-Serve Facts Summary

**Service bit 10 plus I/O-free prune advertisement and a BIP 159 window that is outside only when tip distance is greater than 288 plus 2**

## Performance

- **Duration:** 27 min
- **Started:** 2026-09-27T21:44:07Z
- **Completed:** 2026-09-27T22:12:45Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Added `ServiceFlags::NETWORK_LIMITED` as `1 << 10` beside `NETWORK` and `WITNESS`.
- Added `advertised_service_flags`: disabled mode stays `NETWORK | WITNESS`; manual and automatic prune advertise `NETWORK_LIMITED | WITNESS` without `NETWORK`.
- Added `block_request_exceeds_limited_serve_window` so distance 290 is inside and distance 291 is outside, while `MIN_BLOCKS_TO_KEEP` stays 288.
- Registered `limited_serve.rs` under `network-limited-serve` without moving `message.rs` out of `network-message-codec`.

## Task Commits

Each task was committed atomically:

1. **Task 1 + Task 2: service bit, limited-serve predicates, and breadcrumbs** - `ff2a5d14` (feat)

**Plan metadata:** docs commit for this summary

_Note: Task 1 action 6 lands the tests and implementation in the same commit as the Task 2 breadcrumbs so pre-commit `verify.sh` can pass._

## Files Created/Modified

- `packages/open-bitcoin-network/src/limited_serve.rs` — `advertised_service_flags`, `block_request_exceeds_limited_serve_window`, and unit tests
- `packages/open-bitcoin-network/src/message.rs` — `ServiceFlags::NETWORK_LIMITED = 1 << 10`
- `packages/open-bitcoin-network/src/lib.rs` — module registration and crate-root exports
- `docs/parity/source-breadcrumbs.json` — `network-limited-serve` group
- `docs/metrics/lines-of-code.md` — verifier freshness update from the pre-commit hook

## Decisions Made

- Disabled advertisement stays `NETWORK | WITNESS`. `LocalPeerConfig::default` and `VersionMessage::default` were left unchanged.
- Manual and automatic prune share `NETWORK_LIMITED | WITNESS` and omit `NETWORK`. The function takes only `PruneMode`.
- The window comparison is greater-than against `MIN_BLOCKS_TO_KEEP + LIMITED_SERVE_RACE_BUFFER` (`288 + 2`). `height_inside_keep_window` and the keep constant were not edited.
- SERV-01 is activated in `149-02-SUMMARY.md` because `149-VERIFICATION.md` is passed.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for 149-02. The predicates are exported and tested. Plan 02 can wire `advertised_service_flags` into the version message. Plan 03 can use `block_request_exceeds_limited_serve_window` when refusing block bodies. This plan does not serve bytes, disconnect peers, or emit `Pruned`.

## Self-Check: PASSED

- FOUND: packages/open-bitcoin-network/src/limited_serve.rs
- FOUND: ff2a5d14

---
*Phase: 149-limited-serving-and-honest-pruned-labels*
*Completed: 2026-09-27*
