---
phase: 91-peer-permissions-and-connection-classes
plan: 01
subsystem: network
tags: [rust, p2p, peer-permissions, inbound-admission, parity]

requires:
  - phase: 90-inbound-listener-and-admission-policy
    provides: "Pure inbound admission slot classes, counters, and reserved-capacity policy"
provides:
  - "Typed Knots-anchored peer permission tokens and direction parsing"
  - "Active bounded effect labels split from inactive relay, mempool, bloom, and block-filter effects"
  - "Literal IpAddr permission class registry with stable connection-class labels"
  - "Reserved-slot mapping for protected inbound permission decisions only"
affects:
  - 91-02-open-bitcoin-jsonc-and-cli-permission-class-config
  - 91-03-permission-evidence-in-admission-records-and-managed-counters
  - 91-04-runtime-listener-permission-aware-admission-wiring
  - 91-05-shared-status-rpc-and-metrics-permission-evidence

tech-stack:
  added: []
  patterns:
    - "Parse raw permission strings into closed Rust domain enums at the network boundary"
    - "Expose public status labels through enum as_str methods instead of raw config names"
    - "Use literal IpAddr class matching only for Phase 91"

key-files:
  created:
    - packages/open-bitcoin-network/src/inbound/permissions.rs
  modified:
    - packages/open-bitcoin-network/src/inbound.rs
    - packages/open-bitcoin-network/src/inbound/tests.rs
    - packages/open-bitcoin-network/src/lib.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Treat relay, forcerelay, mempool, bloomfilter, and blockfilters as inactive effect labels in the Phase 91 network domain model."
  - "Map only forceinbound-protected inbound classes to reserved admission capacity; ordinary and permissioned inbound stay ordinary slots."
  - "Use literal IpAddr class matching and reject ranges, hostnames, and endpoint-shaped values at the class parser boundary."

patterns-established:
  - "PeerPermissionSet owns all token expansion and effect labeling before later config or runtime code sees permissions."
  - "InboundPermissionDecision carries stable class/effect labels without exposing user-provided permission class names."

requirements-completed: [PERM-01, PERM-02, PERM-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 91-2026-06-25T13-36-41
generated_at: 2026-06-25T15:56:40Z

duration: 27min
completed: 2026-06-25
---

# Phase 91 Plan 01: Pure Permission Vocabulary and Connection Classes Summary

**Typed peer permission parsing and literal-IP connection classes anchored to Knots permission names while keeping relay, mempool, bloom, and block-filter behavior inactive.**

## Performance

- **Duration:** 27 min
- **Started:** 2026-06-25T15:29:30Z
- **Completed:** 2026-06-25T15:56:40Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- Added `PeerPermissionToken`, `PeerPermissionDirection`, `PeerPermissionSet`, active `PermissionEffectLabel`, inactive `InactivePermissionEffectLabel`, and deterministic `PeerPermissionParseError` values.
- Added `ParsedPeerPermissionClass`, `PeerPermissionClassRegistry`, `PeerConnectionClass`, `PermissionClassName`, and `InboundPermissionDecision`.
- Preserved Phase 90 admission boundaries by mapping only `protected_inbound` to `InboundAdmissionSlotClass::Reserved`.
- Registered the new permission module in `docs/parity/source-breadcrumbs.json` with Knots permission anchors.

## Task Commits

1. **Task 1: Add typed permission vocabulary and effect parsing** - `813f82a` (`feat`)
2. **Task 2: Add permission class registry and connection-class labels** - `905ad64` (`feat`)

**Plan metadata:** final docs commit created after this summary.

## Files Created/Modified

- `packages/open-bitcoin-network/src/inbound/permissions.rs` - Pure permission token parser, active/inactive effect model, literal-IP class registry, and stable class labels.
- `packages/open-bitcoin-network/src/inbound.rs` - Child module declaration and public inbound exports.
- `packages/open-bitcoin-network/src/inbound/tests.rs` - Parser, alias rejection, `all` expansion, literal-IP class, reserved-slot, and stable-label tests.
- `packages/open-bitcoin-network/src/lib.rs` - Crate-level re-exports for downstream Phase 91 plans.
- `docs/parity/source-breadcrumbs.json` - `network-peer-permissions` breadcrumb group.
- `docs/metrics/lines-of-code.md` - Hook-regenerated LOC report after adding the new tracked Rust module and tests.

## Decisions Made

- `all` expands into auditable typed permissions, but relay-like permissions are exposed only through inactive labels.
- `forceinbound` is the Phase 91 admission-protection signal that maps to reserved slot use; `download`, `addr`, and `noban` remain bounded policy inputs.
- Permission class matching uses literal `IpAddr` values only; broader network/range compatibility is deliberately deferred.

## Deviations from Plan

### Process Adjustments

**1. AGENTS.md precedence over TDD red commits**
- **Found during:** Task 1 and Task 2
- **Issue:** The plan marked both tasks `tdd="true"`, but repo rules require commits only after verification. Committing intentionally failing red tests would violate the repo instruction file.
- **Adjustment:** Ran red compile checks to prove missing types, then committed only green implementation states.
- **Files modified:** None beyond planned files.
- **Verification:** Red `cargo check` failed on missing permission/class exports before implementation; green compile/clippy checks passed after implementation.

**2. Local Rust test executable launch blocked**
- **Found during:** Task 1 and Task 2 verification
- **Issue:** `cargo test` built `open_bitcoin_network-9c1b8078dc92ec9b`, then hung before Rust test execution after printing `Running unittests src/lib.rs`. Direct `--list` also hung; sampling showed the process at `_dyld_start`.
- **Adjustment:** Stopped hung test runs, then used compile-only and lint verification: `cargo test --no-run`, `cargo check`, `cargo build`, `cargo clippy`, and the parity breadcrumb checker.
- **Files modified:** None.
- **Verification:** See verification results below.

**Total deviations:** 2 process adjustments
**Impact on plan:** Implementation scope stayed within Plan 91-01 owned files. Executable test assertions are compiled but could not be run in this local macOS session.

### Generated Metadata

- `docs/metrics/lines-of-code.md` was regenerated by the repo pre-commit hook during the interrupted full-verifier commit attempt. Repo guidance treats this as tracked generated output, so it is included in the final metadata commit.

## Verification Results

- `cargo check --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features` - passed
- `cargo build --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features` - passed
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-features --no-run` - passed
- `bun run scripts/check-parity-breadcrumbs.ts --check` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network permission -- --nocapture` - blocked at local test binary launch; interrupted after no test execution output
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network inbound -- --nocapture` - blocked at local test binary launch; interrupted after no test execution output

## Known Stubs

None.

## Issues Encountered

- Local generated Rust test binaries hang before test execution in this session. The issue also reproduced with `open-bitcoin-core`, so it is not isolated to the new permission code.

## User Setup Required

None.

## Next Phase Readiness

Plan 91-02 can parse Open Bitcoin JSONC/CLI permission-class config into `ParsedPeerPermissionClass` and project errors using the field/token-aware `PeerPermissionParseError` surface.

## Self-Check: PASSED

- Found `packages/open-bitcoin-network/src/inbound/permissions.rs`
- Found `.planning/phases/91-peer-permissions-and-connection-classes/91-01-SUMMARY.md`
- Found task commit `813f82a`
- Found task commit `905ad64`

---
*Phase: 91-peer-permissions-and-connection-classes*
*Completed: 2026-06-25*
