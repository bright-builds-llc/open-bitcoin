---
phase: 91-peer-permissions-and-connection-classes
plan: 02
subsystem: rpc-config
tags: [rust, p2p, peer-permissions, jsonc, cli, config]

requires:
  - phase: 91-01
    provides: "PeerPermissionClassRegistry, ParsedPeerPermissionClass, and stable permission parse errors"
provides:
  - "Open Bitcoin JSONC inbound.permission_classes DTOs"
  - "Runtime resolver validation into PeerPermissionClassRegistry"
  - "Repeatable -openbitcoininboundpermissionclass CLI overrides"
  - "Stable field/token/value errors for malformed permission class config"
  - "Baseline whitelist/whitebind-style parameter rejection coverage"
affects:
  - 91-03-permission-evidence-in-admission-records-and-managed-counters
  - 91-04-runtime-listener-permission-aware-admission-wiring
  - 91-05-shared-status-rpc-and-metrics-permission-evidence

tech-stack:
  added: []
  patterns:
    - "Resolve raw JSONC and CLI permission classes through the shared network permission parser"
    - "Treat CLI permission class specs as a complete override list"
    - "Carry a default PeerPermissionClassRegistry on InboundListenerConfig"

key-files:
  created: []
  modified:
    - packages/open-bitcoin-network/src/inbound.rs
    - packages/open-bitcoin-network/src/inbound/permissions.rs
    - packages/open-bitcoin-network/src/inbound/tests.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs
    - packages/open-bitcoin-rpc/src/config.rs
    - packages/open-bitcoin-rpc/src/config/loader.rs
    - packages/open-bitcoin-rpc/src/config/loader/inbound.rs
    - packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs
    - packages/open-bitcoin-rpc/src/config/open_bitcoin.rs
    - packages/open-bitcoin-rpc/src/config/tests.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Use Open Bitcoin-owned JSONC and Open Bitcoin-prefixed CLI flags only; Knots whitelist and whitebind-style inputs remain rejected."
  - "CLI permission-class flags replace the JSONC class list as a complete override, preserving deterministic order."
  - "Carry the parsed PeerPermissionClassRegistry on InboundListenerConfig so later listener wiring can use the typed registry directly."

patterns-established:
  - "RPC config maps network PeerPermissionParseError values into indexed Open Bitcoin config paths."
  - "CLI specs are parsed into InboundPermissionClassConfig DTOs before shared registry resolution."

requirements-completed: [PERM-01, PERM-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 91-2026-06-25T13-36-41
generated_at: 2026-06-25T16:38:41Z

duration: 34min
completed: 2026-06-25
---

# Phase 91 Plan 02: Open Bitcoin JSONC and CLI Permission-Class Config Summary

**Open Bitcoin-owned JSONC and CLI permission-class configuration now resolves into the typed network permission registry with stable validation errors.**

## Performance

- **Duration:** 34 min
- **Started:** 2026-06-25T16:04:47Z
- **Completed:** 2026-06-25T16:38:41Z
- **Tasks:** 2
- **Files modified:** 13, including the hook-regenerated LOC report

## Accomplishments

- Added `inbound.permission_classes` JSONC support with `name`, `addresses`, and `permissions` fields.
- Resolved JSONC and CLI permission classes through `ParsedPeerPermissionClass` into `PeerPermissionClassRegistry`.
- Added repeatable `-openbitcoininboundpermissionclass=name@127.0.0.1=in,noban,forceinbound,download,addr` CLI overrides.
- Added deterministic validation coverage for empty names, empty addresses, CIDR, hostnames, socket endpoints, duplicate literal IPs, unsupported tokens, direction-only tokens, missing `in`, and `out` combinations.
- Preserved rejection coverage for `-whitelist`, `-whitebind`, `-whitelistrelay`, and `-whitelistforcerelay`.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add JSONC permission class config and resolver validation** - `3b60e4f` (`feat`)
2. **Task 2: Add Open Bitcoin-prefixed CLI permission class overrides** - `d216e9a` (`feat`)

**Plan metadata:** final docs commit created after this summary.

## Files Created/Modified

- `packages/open-bitcoin-network/src/inbound.rs` - Adds a default permission registry to `InboundListenerConfig`.
- `packages/open-bitcoin-network/src/inbound/permissions.rs` - Adds registry defaulting and a parsed-class address accessor for duplicate-IP validation.
- `packages/open-bitcoin-network/src/inbound/tests.rs` - Updates listener config fixture for the registry field.
- `packages/open-bitcoin-rpc/src/config/open_bitcoin.rs` - Adds `InboundPermissionClassConfig` and JSONC DTO wiring.
- `packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs` - Resolves JSONC/CLI DTOs into `PeerPermissionClassRegistry` and maps parser errors to indexed config errors.
- `packages/open-bitcoin-rpc/src/config/loader.rs` - Stores repeated CLI permission-class specs.
- `packages/open-bitcoin-rpc/src/config/loader/inbound.rs` - Parses the Open Bitcoin-prefixed permission-class CLI flag.
- `packages/open-bitcoin-rpc/src/config/tests.rs` - Adds JSONC, CLI, malformed input, override, and baseline-rejection coverage.
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs`, `packages/open-bitcoin-rpc/src/dispatch/tests.rs`, and `packages/open-bitcoin-rpc/src/inbound_listener/tests.rs` - Update inbound listener config literals for the registry field.
- `docs/metrics/lines-of-code.md` - Hook-regenerated LOC report for the changed Rust files.

## Decisions Made

- CLI permission-class flags are a full replacement for JSONC permission classes, not an append or merge layer.
- Duplicate literal IP detection happens after Plan 91-01 parsing, using typed `IpAddr` values rather than raw string comparison.
- CLI malformed shape errors name `openbitcoininboundpermissionclass`; class semantic errors still use indexed `inbound.permission_classes[N]` paths through the shared resolver.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Carried the registry through InboundListenerConfig**
- **Found during:** Task 1 (JSONC resolver validation)
- **Issue:** The existing `InboundListenerConfig` had no field capable of carrying the `PeerPermissionClassRegistry` built by the config resolver into later listener/runtime plans.
- **Fix:** Added `permission_classes: PeerPermissionClassRegistry` with a default empty registry, plus a narrow `ParsedPeerPermissionClass::addresses()` accessor needed for duplicate literal-IP validation.
- **Files modified:** `packages/open-bitcoin-network/src/inbound.rs`, `packages/open-bitcoin-network/src/inbound/permissions.rs`, and existing test literals.
- **Verification:** `cargo check`, `cargo build`, `cargo clippy`, and `cargo test --no-run` for `open-bitcoin-rpc` passed.
- **Committed in:** `3b60e4f`

**2. Process adjustment: TDD red commits skipped**
- **Found during:** Task 1 and Task 2
- **Issue:** The plan marked both tasks `tdd="true"`, but repo rules require commits only after verification. Committing intentionally failing red tests would violate the repo instruction file.
- **Fix:** Added tests and implementation together per task, then committed only verified green states.
- **Files modified:** No additional files beyond planned implementation/test files.
- **Verification:** See verification results below.
- **Committed in:** `3b60e4f`, `d216e9a`

***

**Total deviations:** 1 auto-fixed issue plus 1 process adjustment.
**Impact on plan:** The network API addition was the small registry-carrying change explicitly allowed by the execution prompt. No Knots whitelist compatibility or deferred relay behavior was added.

### Generated Metadata

- `docs/metrics/lines-of-code.md` was regenerated by the repo commit hook. Repo guidance treats this as tracked generated output, so it is included in the final metadata commit.

## Verification Results

- `cargo fmt --all --manifest-path packages/Cargo.toml` - passed
- `cargo check --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features` - passed
- `cargo build --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features` - passed
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features -- -D warnings` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --no-run` - passed
- `timeout 30s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc inbound_permission -- --nocapture` - lib tests passed 5/5, then timed out with code 124 after launching `src/bin/open-bitcoind.rs`.
- `timeout 30s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc inbound_config -- --nocapture` - lib test passed 1/1, then timed out with code 124 after launching `src/bin/open-bitcoind.rs`.
- `timeout 30s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc daemon_inbound_permission -- --nocapture` - lib tests passed 2/2, then timed out with code 124 after launching `src/bin/open-bitcoind.rs`.

## Known Stubs

None.

## Threat Flags

None. The JSONC/CLI trust boundary and baseline-looking Knots key rejection were in the plan threat model and were mitigated with closed type parsing, literal-IP-only validation, CLI full-list override semantics, and deterministic errors.

## Issues Encountered

- Local generated Rust test binaries still hang immediately after Cargo launches non-lib test binaries. In this plan, each focused test command passed its matching `src/lib.rs` tests, then timed out after printing `Running unittests src/bin/open-bitcoind.rs`. No hung processes were left running.

## Authentication Gates

None.

## User Setup Required

None.

## Next Phase Readiness

Plan 91-03 can consume `RuntimeConfig.inbound.permission_classes` and thread permission decisions into admission records and managed counters without reparsing raw JSONC or CLI strings.

## Self-Check: PASSED

- Found `.planning/phases/91-peer-permissions-and-connection-classes/91-02-SUMMARY.md`
- Found task commit `3b60e4f`
- Found task commit `d216e9a`

***
*Phase: 91-peer-permissions-and-connection-classes*
*Completed: 2026-06-25*
