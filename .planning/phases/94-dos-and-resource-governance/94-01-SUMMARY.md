---
phase: 94-dos-and-resource-governance
plan: 01
subsystem: network
tags: [rust, networking, resource-governance, dos-hardening, parity]

requires: []
provides:
  - Pure inbound message-envelope resource gate for open-bitcoin-network
  - Stable low-cardinality resource rejection labels for invalid wire input
  - Deterministic resource-label test coverage without public-network dependencies
affects: [open-bitcoin-network, inbound-runtime, peer-manager, parity-breadcrumbs]

tech-stack:
  added: []
  patterns:
    - Functional-core inbound envelope policy before payload allocation
    - Low-cardinality resource events without peer, endpoint, payload, or credential material

key-files:
  created:
    - packages/open-bitcoin-network/src/resource.rs
    - packages/open-bitcoin-network/src/resource/tests.rs
  modified:
    - packages/open-bitcoin-network/src/lib.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Kept Phase 94 resource governance in a pure open-bitcoin-network module with no socket, runtime, or peer-manager side effects."
  - "Used existing codec and message decoding APIs rather than introducing a new wire parser."
  - "Preserved repo hook requirements by recording TDD RED locally and committing only verification-passing task states."

patterns-established:
  - "InboundEnvelopePolicy::evaluate_header classifies 24-byte headers before payload allocation."
  - "InboundEnvelopePolicy::decode_payload maps checksum, malformed payload, trailing payload, unsupported command, and oversized payload failures to stable labels."
  - "Resource events use fixed message and next_action fields for operator-safe evidence."

requirements-completed: [DOS-01, DOS-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 94-2026-06-26T15-47-23
generated_at: 2026-06-26T17:51:08Z

duration: 31min
completed: 2026-06-26
---

# Phase 94 Plan 01: Message-Envelope Resource Gate Summary

**Pure inbound message-envelope resource governance with stable rejection labels before payload allocation**

## Performance

- **Duration:** 31 min
- **Started:** 2026-06-26T17:20:08Z
- **Completed:** 2026-06-26T17:51:08Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Added `InboundEnvelopePolicy` and exported Phase 94 resource labels, event shape, payload cap, and envelope decisions from `open-bitcoin-network`.
- Classified wrong network magic, malformed headers, oversized payloads, unsupported commands, invalid checksums, malformed payloads, and trailing payloads into stable low-cardinality labels.
- Registered parity breadcrumbs for the new first-party resource module and tests.
- Added deterministic unit coverage for all D-04 parser/resource labels without sleeps, sockets, threads, or public-network dependencies.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add resource envelope contracts and exports** - `60606e28` (`feat`)
2. **Task 2: Cover envelope rejection labels with deterministic tests** - `b6d0eaa1` (`test`)

## Files Created/Modified

- `packages/open-bitcoin-network/src/resource.rs` - Pure inbound resource policy, stable labels, event contract, payload cap, and payload decode mapping.
- `packages/open-bitcoin-network/src/resource/tests.rs` - Deterministic label coverage for header, checksum, malformed payload, and trailing payload rejection paths.
- `packages/open-bitcoin-network/src/lib.rs` - Public exports for the resource governance API.
- `docs/parity/source-breadcrumbs.json` - `network-resource-governance` breadcrumb group for the new Rust files.
- `docs/metrics/lines-of-code.md` - Hook-generated line count refresh from the repo verifier.

## Checks Run

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network resource --no-fail-fast` - passed, 13 resource tests.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings` - passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` - passed, 295 Rust files verified.
- Normal git hooks ran `bash scripts/verify.sh` for both task commits; Task 2 hook completed successfully in 2m 56.128s.

## Decisions Made

- Kept the resource gate as a pure policy module and deferred runtime socket integration to later Phase 94 plans.
- Reused `open_bitcoin_codec::parse_message_header`, `WireNetworkMessage::decode_payload`, and existing checksum semantics for parity with the current network message stack.
- Kept event evidence bounded to fixed strings and reason labels, satisfying the plan threat model for information disclosure.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added coverage-focused resource tests during Task 1**
- **Found during:** Task 1 (Add resource envelope contracts and exports)
- **Issue:** The normal pre-commit verifier blocked the task commit because the new resource module did not satisfy repo coverage gates with only the three initial behavior tests.
- **Fix:** Added focused pure tests for label/source string coverage, valid header read decisions, malformed header parse failures, valid decode, pre-decoder guards, and decoder error mapping.
- **Files modified:** `packages/open-bitcoin-network/src/resource/tests.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network resource --no-fail-fast`; normal git hook `bash scripts/verify.sh`
- **Committed in:** `60606e28`

**2. [Rule 3 - Blocking] Added a narrow clippy allowance for the plan-mandated error return**
- **Found during:** Task 1 (Add resource envelope contracts and exports)
- **Issue:** `cargo clippy -D warnings` rejected `decode_payload` because the plan-required `Result<ParsedNetworkMessage, InboundResourceEvent>` returns a large error value.
- **Fix:** Added a targeted `#[allow(clippy::result_large_err)]` on `decode_payload` instead of changing the public contract away from the plan.
- **Files modified:** `packages/open-bitcoin-network/src/resource.rs`
- **Verification:** `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings`
- **Committed in:** `60606e28`

**3. [Process - Repo Hook Constraint] TDD RED states were not committed separately**
- **Found during:** Task 1 and Task 2
- **Issue:** The plan marked both tasks `tdd="true"`, but repo instructions and normal hooks require every commit to pass formatting, linting, build, tests, and verification. Committing failing RED states would violate those higher-priority rules.
- **Fix:** Ran/recorded the RED failure locally for Task 1, then committed only passing task states. Task 2 tests passed immediately because Task 1 coverage work already implemented the required branches.
- **Files modified:** None beyond task files
- **Verification:** Task commits were made through normal hooks without `--no-verify`
- **Committed in:** `60606e28`, `b6d0eaa1`

***

**Total deviations:** 3 auto-handled (2 blocking, 1 process constraint)
**Impact on plan:** No scope creep. The changes were required to satisfy the repo verifier and higher-priority commit rules while preserving the plan's API and behavior.

## Issues Encountered

- New files had to be staged before `scripts/check-parity-breadcrumbs.ts --check` could validate them, because the checker uses tracked file state.
- The normal hook refreshed `docs/metrics/lines-of-code.md` as an intentional generated artifact.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

The pure resource gate is ready for later Phase 94 runtime wiring. Future plans can call `InboundEnvelopePolicy::evaluate_header` before allocating payload buffers and `decode_payload` after bounded reads to produce stable operator-safe resource evidence.

## Self-Check: PASSED

- Confirmed summary, resource source/tests, exports, parity breadcrumbs, and LOC report exist.
- Confirmed task commits `60606e28` and `b6d0eaa1` exist in git history.

***
*Phase: 94-dos-and-resource-governance*
*Completed: 2026-06-26*
