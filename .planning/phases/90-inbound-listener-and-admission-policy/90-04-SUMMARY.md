---
phase: 90-inbound-listener-and-admission-policy
plan: 04
subsystem: networking
tags: [rust, p2p, inbound, tokio, daemon]

requires:
  - phase: 90-02
    provides: Disabled-by-default Open Bitcoin inbound runtime config and CLI controls
  - phase: 90-03
    provides: Managed inbound peer admission, handshake state, and counters
provides:
  - Thin Tokio inbound listener activation adapter with typed preflight and bind diagnostics
  - Loopback accept-loop worker that admits peers through ManagedPeerNetwork and existing wire-message handling
  - open-bitcoind startup and bounded listener shutdown wiring with stable evidence labels
  - Hermetic loopback tests for listener bind, daemon startup, handshake counts, and cap rejection evidence
affects:
  - phase-90-rpc-status
  - phase-90-cli-status
  - phase-90-support-evidence
  - phase-90-final-verification

tech-stack:
  added: []
  patterns:
    - Pure preflight before Tokio bind activation
    - Listener-owned runtime shell over existing ManagedPeerNetwork admission and PeerAction handshake flow
    - Stable startup labels for listener state, preflight reason, bound endpoint, and admission rejection

key-files:
  created:
    - packages/open-bitcoin-rpc/src/inbound_listener.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs
  modified:
    - packages/open-bitcoin-rpc/src/lib.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Kept socket ownership isolated in the new RPC inbound_listener module."
  - "Kept http.rs untouched per the owned-file boundary; the listener worker owns its runtime ManagedRpcContext for admission and wire-message handoff."
  - "Used the existing ParsedNetworkMessage and ManagedPeerNetwork receive path instead of adding an inbound-only parser or handshake engine."
  - "Registered new RPC listener files in the parity breadcrumb registry because repo rules require every first-party Rust source/test file to be covered."

patterns-established:
  - "open-bitcoind activates inbound listener work only after typed preflight and before RPC serving starts."
  - "Inbound accept loops spawn tracked per-connection tasks so one active peer cannot block later cap or duplicate admission decisions."
  - "Loopback integration tests use 127.0.0.1:0 and do not depend on public interfaces, DNS seeds, service managers, or long sleeps."

requirements-completed: [INB-01, INB-02, INB-03, INB-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 90-2026-06-25T04-23-47
generated_at: 2026-06-25T07:50:30Z

duration: 29 min
completed: 2026-06-25
---

# Phase 90 Plan 04: Runtime Listener Adapter, Daemon Startup, and Loopback Integration Summary

**Opt-in Tokio inbound listener wiring with loopback bind diagnostics, managed peer admission, and daemon shutdown control**

## Performance

- **Duration:** 29 min
- **Started:** 2026-06-25T07:21:28Z
- **Completed:** 2026-06-25T07:50:30Z
- **Tasks:** 3
- **Files modified:** 8

## Accomplishments

- Added `inbound_listener.rs` as the only new socket-owning module for Phase 90 RPC runtime listener work.
- Bound enabled loopback endpoints only after Plan 01 typed preflight and activation diagnostics.
- Routed accepted peers through `ManagedRpcContext::record_inbound_admission` and existing `ParsedNetworkMessage` / `WireNetworkMessage` handling.
- Wired `open-bitcoind` startup to activate listener work before RPC serving and shut it down when serving exits.
- Added deterministic loopback tests proving version/verack handshake responses, inbound count increments, outbound count preservation, and cap rejection evidence.

## Task Commits

1. **Task 1 RED: listener adapter tests** - `f78e539` (test)
2. **Task 1 GREEN: listener adapter implementation** - `f3c1167` (feat)
3. **Task 2 RED: daemon startup tests** - `f0c72d8` (test)
4. **Task 2 GREEN: daemon listener startup wiring** - `b8c0e71` (feat)
5. **Task 3 RED: loopback admission coverage** - `4df45e4` (test)
6. **Task 3 GREEN: concurrent admission fix** - `c524510` (fix)

## Files Created/Modified

- `packages/open-bitcoin-rpc/src/inbound_listener.rs` - Tokio listener activation, bounded evidence, accept-loop worker, wire-message handoff, and shutdown.
- `packages/open-bitcoin-rpc/src/inbound_listener/tests.rs` - Preflight, bind, daemon-safe loopback, handshake, count, and cap-rejection tests.
- `packages/open-bitcoin-rpc/src/lib.rs` - Exports the inbound listener module.
- `packages/open-bitcoin-rpc/src/context/network.rs` - Adds inbound admission and encoded wire-response handoff methods and applies resolved inbound caps.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` - Starts listener work before RPC serving and shuts it down after serving exits.
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs` - Covers disabled defaults, loopback startup, shutdown, and stable startup labels.
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - Fixture-only repair for the existing `PeerStatus.inbound` field.
- `docs/parity/source-breadcrumbs.json` - Registers the new RPC listener source/test files.

## Decisions Made

- Listener activation is a two-stage flow: pure preflight classification first, then scoped Tokio bind activation.
- Listener runtime evidence stays bounded to stable strings and counters; no raw peer table or endpoint-bearing metric surface was added.
- `open-bitcoind` logs stable labels but does not use listener startup text to claim deferred network participation.
- The HTTP RPC state surface was not refactored because it is outside this plan's owned-file list.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Repaired RPC status test fixtures for 90-05 status field**
- **Found during:** Task 1 RED
- **Issue:** `open-bitcoin-rpc` test binaries no longer compiled after `PeerStatus.inbound` was added by 90-05.
- **Fix:** Added unavailable inbound status fixtures in `dispatch/tests.rs`.
- **Files modified:** `packages/open-bitcoin-rpc/src/dispatch/tests.rs`
- **Verification:** Focused listener tests compiled to the intended RED failure, then all targeted RPC checks passed.
- **Committed in:** `f78e539`

**2. [Rule 3 - Blocking] Registered new listener files in parity breadcrumbs**
- **Found during:** Task 1 GREEN
- **Issue:** New first-party Rust source/test files require source breadcrumb coverage under repo rules.
- **Fix:** Added `rpc-inbound-listener` entries for `inbound_listener.rs` and `inbound_listener/tests.rs`.
- **Files modified:** `docs/parity/source-breadcrumbs.json`
- **Verification:** `bun run scripts/check-parity-breadcrumbs.ts --check` passed for 274 Rust files.
- **Committed in:** `f3c1167`

**3. [Rule 1 - Bug] Fixed serialized accept loop blocking cap evidence**
- **Found during:** Task 3 RED
- **Issue:** One active loopback peer could monopolize the accept loop, preventing a second candidate from reaching admission and cap rejection promptly.
- **Fix:** Spawned tracked per-connection tasks and abort them during listener shutdown.
- **Files modified:** `packages/open-bitcoin-rpc/src/inbound_listener.rs`, `packages/open-bitcoin-rpc/src/inbound_listener/tests.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc loopback_inbound -- --nocapture` passed.
- **Committed in:** `c524510`

**Total deviations:** 3 auto-fixed (1 bug, 2 blocking)
**Impact on plan:** All fixes were required for correctness, repo-rule compliance, or targeted test execution. No relay, permission-class, eviction, ban, public-default, or production-readiness behavior was added.

## Issues Encountered

- Cargo commands briefly waited on package/build locks during targeted checks.
- Task 3's RED test initially exposed the accept-loop serialization bug; after the implementation fix, the test wait was changed to cooperative Tokio yielding rather than blocking the async scheduler.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc inbound_listener -- --nocapture` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc open_bitcoind_inbound -- --nocapture` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc loopback_inbound -- --nocapture` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc inbound -- --nocapture` passed with 19 matching tests across library and daemon binaries.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features -- -D warnings` passed.
- `cargo build --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features` passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed for 274 Rust files.
- Plan acceptance `rg` scans for listener APIs, daemon evidence labels, loopback coverage, and deferred-claim guardrails passed.

## Known Stubs

None - stub and placeholder scans found no matches in the touched files.

## Threat Flags

None. The new network socket surface is the planned Phase 90 listener boundary and is covered by the plan threat model.

## Authentication Gates

None.

## User Setup Required

None - no external service configuration required.

## State Updates

Skipped intentionally. The orchestrator explicitly owns `.planning/STATE.md` and `.planning/ROADMAP.md` for this parallel phase run.

## Next Phase Readiness

Ready for downstream Phase 90 RPC/status/support plans to project listener and admission evidence. The runtime listener can bind loopback endpoints, admit peers through managed policy, drive existing handshake responses, record cap rejections, and shut down bounded worker tasks.

## Self-Check: PASSED

- Found all created and modified source, test, metadata, and summary files.
- Found task commits `f78e539`, `f3c1167`, `f0c72d8`, `b8c0e71`, `4df45e4`, and `c524510`.

---

*Phase: 90-inbound-listener-and-admission-policy*
*Completed: 2026-06-25*
