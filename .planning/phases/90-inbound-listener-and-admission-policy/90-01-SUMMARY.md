---
phase: 90-inbound-listener-and-admission-policy
plan: 01
subsystem: networking
tags: [rust, p2p, inbound, preflight, admission-policy]

requires: []
provides:
  - Pure inbound listener preflight contracts and stable diagnostic labels
  - Typed inbound admission policy records, counters, decisions, and rejection labels
  - Focused unit coverage for listener safety, caps, reserved slots, duplicates, self-connections, shutdown, and outbound count separation
affects:
  - phase-90-runtime-listener
  - phase-90-peer-status
  - phase-91-peer-permissions

tech-stack:
  added: []
  patterns:
    - Pure data-in/data-out listener preflight before runtime bind activation
    - Stable label methods on policy enums without adding serialization dependencies
    - Phase 90 reserved-slot primitive separate from later peer permission classes

key-files:
  created:
    - packages/open-bitcoin-network/src/inbound.rs
    - packages/open-bitcoin-network/src/inbound/tests.rs
  modified:
    - packages/open-bitcoin-network/src/lib.rs

key-decisions:
  - "Represented bind_unavailable and already_bound as typed activation diagnostics, not pure preflight classifier outputs."
  - "Exposed stable diagnostic labels through as_str methods instead of adding a new serialization dependency."
  - "Kept Phase 90 reserved-slot admission as ordinary vs reserved slot classes only."

patterns-established:
  - "Inbound listener config is classified into typed diagnostics before any socket effect."
  - "Inbound admission is a pure decision over candidate evidence, counters, duplicate sets, nonce evidence, and shutdown state."

requirements-completed: [INB-02, INB-03, INB-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 90-2026-06-25T04-23-47
generated_at: 2026-06-25T06:14:50Z

duration: 14 min
completed: 2026-06-25
---

# Phase 90 Plan 01: Inbound Listener and Admission Policy Contracts Summary

**Pure inbound listener preflight and admission policy contracts for deterministic Phase 90 runtime wiring**

## Performance

- **Duration:** 14 min
- **Started:** 2026-06-25T06:00:28Z
- **Completed:** 2026-06-25T06:14:50Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- Added `open-bitcoin-network` inbound contracts for listener config, parsed endpoints, preflight plans, activation diagnostics, admission requests, peer records, counters, handshake state, decisions, and rejection reasons.
- Implemented pure preflight classification for disabled config, missing addresses, malformed endpoints, unsafe non-loopback endpoints, and ready loopback/public-approved endpoints.
- Implemented pure admission decisions for inbound caps, reserved-slot availability, duplicate endpoints, duplicate peer IDs, self-connection nonce evidence, shutdown, and outbound-count separation.
- Exported inbound contracts from `open-bitcoin-network` for downstream Phase 90 runtime, status, and support plans.

## Task Commits

1. **RED tests for Tasks 1 and 2** - `5f70752` (test)
2. **GREEN implementation for Tasks 1 and 2** - `96de1cf` (feat)

_Note: The RED suite covered both contract and behavior expectations because the Task 2 behavior tests depend on the Task 1 contract API._

## Files Created/Modified

- `packages/open-bitcoin-network/src/inbound.rs` - Pure inbound listener preflight and admission policy module with Knots parity breadcrumbs.
- `packages/open-bitcoin-network/src/inbound/tests.rs` - Focused Arrange/Act/Assert tests for stable labels, endpoint safety, admission caps, reserved slots, duplicates, self-connection, shutdown, and outbound count separation.
- `packages/open-bitcoin-network/src/lib.rs` - Exports inbound contracts from the crate root.

## Decisions Made

- OS-observed bind outcomes remain activation diagnostics, while the pure classifier only produces config-derived outcomes and ready endpoints.
- Stable labels are exposed by `as_str()` methods, avoiding a new dependency in the pure network crate.
- Reserved slots are modeled only as a Phase 90 slot class, leaving broader peer permission semantics for later phases.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network inbound -- --nocapture` failed in RED as expected with unresolved inbound contract imports.
- `cargo fmt --manifest-path packages/Cargo.toml --all` passed.
- `rg -n "pub struct InboundListenerConfig|pub enum InboundPreflightReason|pub struct InboundAdmissionPolicy|pub enum InboundAdmissionDecision|pub struct InboundPeerRecord|pub use inbound" packages/open-bitcoin-network/src/inbound.rs packages/open-bitcoin-network/src/lib.rs` passed.
- `rg -n "noban|forcerelay|mempool|whitebind|whitelist|NetPermission" packages/open-bitcoin-network/src/inbound.rs` returned no matches.
- `rg -n "disabled|no_listen_addresses|invalid_endpoint|unsafe_endpoint|bind_unavailable|already_bound|ready|Arrange|Act|Assert" packages/open-bitcoin-network/src/inbound/tests.rs` passed.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings` passed after deriving `Default` for `InboundListenerConfig`.
- `cargo build --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network inbound -- --nocapture` passed with 18 inbound tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-features` passed with 59 unit tests, 4 integration/property tests, and doc tests.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- Clippy flagged a manual `Default` implementation that could be derived; the implementation was simplified and clippy passed.
- The source-breadcrumb registry JSON was not updated because this executor was scoped to plan-owned network files only; the new Rust files include top-of-file parity breadcrumb comments for the later Phase 90 verifier/docs wiring.

## Known Stubs

None - stub and placeholder scans found no matches in the touched files.

## Authentication Gates

None.

## Next Phase Readiness

Ready for the next Phase 90 plan. Downstream runtime and status work can import `InboundListenerConfig`, `InboundPreflightPlan`, `InboundListenerActivationDiagnostic`, `InboundAdmissionPolicy`, `InboundAdmissionDecision`, and `InboundPeerRecord` from `open-bitcoin-network`.

## Self-Check: PASSED

- Found `packages/open-bitcoin-network/src/inbound.rs`.
- Found `packages/open-bitcoin-network/src/inbound/tests.rs`.
- Found `.planning/phases/90-inbound-listener-and-admission-policy/90-01-SUMMARY.md`.
- Found commit `5f70752`.
- Found commit `96de1cf`.

---

*Phase: 90-inbound-listener-and-admission-policy*
*Completed: 2026-06-25*
