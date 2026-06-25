---
phase: 90-inbound-listener-and-admission-policy
plan: 02
subsystem: rpc-config
tags: [rust, config, jsonc, cli, inbound]

requires:
  - phase: 90-01
    provides: Pure inbound listener preflight and InboundListenerConfig contract
provides:
  - Disabled-by-default Open Bitcoin JSONC inbound config section
  - RuntimeConfig.inbound resolved as the shared network InboundListenerConfig
  - Open Bitcoin-prefixed daemon CLI overrides for inbound listener controls
  - Focused config precedence and validation coverage
affects:
  - phase-90-runtime-listener
  - phase-90-inbound-status
  - phase-91-peer-permissions

tech-stack:
  added: []
  patterns:
    - Open Bitcoin JSONC plus daemon CLI controls resolve through one validation path
    - CLI > JSONC > defaults precedence for inbound listener settings
    - Baseline bitcoin.conf listener and permission-looking keys remain invalid for Phase 90 controls

key-files:
  created:
    - .planning/phases/90-inbound-listener-and-admission-policy/90-02-SUMMARY.md
  modified:
    - packages/open-bitcoin-rpc/src/config.rs
    - packages/open-bitcoin-rpc/src/config/open_bitcoin.rs
    - packages/open-bitcoin-rpc/src/config/loader.rs
    - packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs
    - packages/open-bitcoin-rpc/src/config/tests.rs
    - packages/open-bitcoin-rpc/Cargo.toml
    - packages/open-bitcoin-rpc/BUILD.bazel
    - packages/Cargo.lock

key-decisions:
  - "Default inbound config stays disabled while carrying loopback 127.0.0.1:18444 and max_peers 8 for explicit enablement."
  - "Open Bitcoin-prefixed CLI overrides are applied in the runtime resolver so JSONC and CLI share validation."
  - "Baseline listener and permission-looking bitcoin.conf keys remain invalid and never activate Phase 90 inbound serving."

patterns-established:
  - "RuntimeConfig owns the resolved network InboundListenerConfig rather than a parallel RPC-local shape."
  - "Inbound numeric validation names exact config fields before runtime listener use."

requirements-completed: [INB-01, INB-02, INB-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 90-2026-06-25T04-23-47
generated_at: 2026-06-25T06:53:21Z

duration: 34 min
completed: 2026-06-25
---

# Phase 90 Plan 02: Open Bitcoin-Owned Inbound Config and CLI Controls Summary

**Disabled-by-default inbound listener configuration with Open Bitcoin JSONC and daemon CLI controls resolved into the shared network contract**

## Performance

- **Duration:** 34 min
- **Started:** 2026-06-25T06:18:53Z
- **Completed:** 2026-06-25T06:53:21Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments

- Added an `inbound` Open Bitcoin JSONC section with `enabled`, `listen_addresses`, `max_peers`, `reserved_slots`, and `allow_public` fields under `deny_unknown_fields`.
- Added `RuntimeConfig.inbound` as `open_bitcoin_network::InboundListenerConfig`, defaulting to disabled and resolving JSONC/defaults before runtime use.
- Added daemon CLI overrides: `-openbitcoininbound`, `-openbitcoinlisten`, `-openbitcoinmaxinbound`, `-openbitcoinreservedslots`, and `-openbitcoinallowpublic`.
- Added focused tests proving disabled defaults, JSONC mapping, invalid numeric values, unknown fields, CLI precedence, repeated listen ordering, baseline key rejection, and unsafe public endpoint preflight.

## Task Commits

1. **Task 1 RED: JSONC inbound config tests** - `6988c77` (test)
2. **Task 1 GREEN: JSONC runtime resolver** - `e9b8d5b` (feat)
3. **Task 2 RED: daemon inbound CLI tests** - `cfc8174` (test)
4. **Task 2 GREEN: daemon inbound CLI overrides** - `f67b316` (feat)

## Files Created/Modified

- `packages/open-bitcoin-rpc/src/config.rs` - Adds `RuntimeConfig.inbound` and re-exports the Open Bitcoin inbound config contract.
- `packages/open-bitcoin-rpc/src/config/open_bitcoin.rs` - Adds JSONC `InboundConfig` defaults and mapping to the network listener config.
- `packages/open-bitcoin-rpc/src/config/loader.rs` - Parses Open Bitcoin-prefixed inbound daemon CLI flags.
- `packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs` - Resolves CLI/JSONC/default inbound settings and validates limits.
- `packages/open-bitcoin-rpc/src/config/tests.rs` - Covers JSONC, CLI precedence, validation, baseline key rejection, and public endpoint preflight behavior.
- `packages/open-bitcoin-rpc/Cargo.toml`, `packages/open-bitcoin-rpc/BUILD.bazel`, `packages/Cargo.lock` - Add the required first-party dependency edge to `open-bitcoin-network`.

## Decisions Made

- Kept inbound serving disabled by default even though defaults include a loopback address and max peer count for explicit enablement.
- Used the Plan 01 `InboundListenerConfig` directly in `RuntimeConfig` instead of creating an RPC-local mirror type.
- Left public endpoint safety to the shared preflight classifier: public CLI endpoints load with `allow_public = false` and classify as `unsafe_endpoint` until `-openbitcoinallowpublic=1` is set.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc open_bitcoin_jsonc_accepts_inbound_listener_contract -- --nocapture`
  - RED failed as expected with unresolved `open_bitcoin_network` and missing `inbound` fields.
  - GREEN passed after implementation.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc daemon_inbound_cli_override_can_enable_or_disable_open_bitcoin_jsonc -- --nocapture`
  - RED failed as expected with `Invalid parameter -openbitcoininbound=1`.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc daemon_inbound -- --nocapture` passed with 4 config tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc inbound -- --nocapture` passed with 8 config tests.
- `rg -n "openbitcoininbound|openbitcoinlisten|openbitcoinmaxinbound|openbitcoinreservedslots|openbitcoinallowpublic" packages/open-bitcoin-rpc/src/config/loader.rs packages/open-bitcoin-rpc/src/config/tests.rs` passed.
- `rg -n "noban|forcerelay|mempool|NetPermission" packages/open-bitcoin-rpc/src/config.rs packages/open-bitcoin-rpc/src/config/open_bitcoin.rs packages/open-bitcoin-rpc/src/config/loader.rs packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs` returned no matches.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features -- -D warnings` was attempted but blocked by unrelated in-progress 90-03 work in `open-bitcoin-node/src/network/inventory.rs` referencing missing `DisconnectReason::SelfConnection` and `NetworkError::SelfConnection`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added required first-party network dependency edge**
- **Found during:** Task 1
- **Issue:** `RuntimeConfig.inbound` must use the Plan 01 `InboundListenerConfig` contract, but `open-bitcoin-rpc` did not directly depend on `open-bitcoin-network`.
- **Fix:** Added the direct Cargo and Bazel dependency plus the Cargo lock update.
- **Files modified:** `packages/open-bitcoin-rpc/Cargo.toml`, `packages/open-bitcoin-rpc/BUILD.bazel`, `packages/Cargo.lock`
- **Verification:** Focused RPC inbound config tests compile and pass.
- **Committed in:** `e9b8d5b`

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Required for correctness and buildability. No new external dependency or behavior scope was introduced.

## Issues Encountered

- Concurrent 90-03 work modified node/network files while this plan was executing. Focused 90-02 tests passed after those changes compiled, but the final clippy pass was blocked by unrelated 90-03 WIP.
- Full `bash scripts/verify.sh` was not run because the orchestrator owns final verification for this parallel phase run.

## Known Stubs

None - stub and placeholder scans found no matches in the touched config files.

## Authentication Gates

None.

## Next Phase Readiness

Ready for Phase 90 runtime listener wiring. Downstream plans can read `RuntimeConfig.inbound`, run `classify_inbound_preflight`, and honor Open Bitcoin-owned CLI/JSONC controls without treating baseline `bitcoin.conf` listener keys as supported Phase 90 controls.

## Self-Check: PASSED

- Found `.planning/phases/90-inbound-listener-and-admission-policy/90-02-SUMMARY.md`.
- Found all plan-owned config files.
- Found commits `6988c77`, `e9b8d5b`, `cfc8174`, and `f67b316`.

---
*Phase: 90-inbound-listener-and-admission-policy*
*Completed: 2026-06-25*
