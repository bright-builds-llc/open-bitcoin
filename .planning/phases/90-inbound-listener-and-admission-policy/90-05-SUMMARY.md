---
phase: 90-inbound-listener-and-admission-policy
plan: 05
subsystem: networking
tags: [rust, status, metrics, inbound, observability]

requires:
  - phase: 90-03
    provides: Managed inbound admission counters and peer-manager admission evidence
provides:
  - Shared inbound listener and admission status evidence under `OpenBitcoinStatusSnapshot.peers`
  - Explicit unavailable inbound status reason for stopped and legacy snapshots
  - Low-cardinality inbound admission metric kinds
  - Parity breadcrumb registry entries for Phase 90 inbound source files
affects:
  - phase-90-rpc-status
  - phase-90-cli-status
  - phase-90-support-evidence
  - phase-90-runtime-listener
  - phase-91-peer-permissions

tech-stack:
  added: []
  patterns:
    - Child status module for inbound peer serving evidence
    - `FieldAvailability` default for serde-compatible status evolution
    - Numeric metric kinds only for inbound admission outcomes

key-files:
  created:
    - packages/open-bitcoin-node/src/status/inbound.rs
    - packages/open-bitcoin-node/src/status/inbound/tests.rs
    - .planning/phases/90-inbound-listener-and-admission-policy/deferred-items.md
  modified:
    - packages/open-bitcoin-node/src/status.rs
    - packages/open-bitcoin-node/src/metrics.rs
    - packages/open-bitcoin-node/src/lib.rs
    - packages/open-bitcoin-node/src/status/tests.rs
    - packages/open-bitcoin-node/src/sync/types/summary.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Kept inbound listener and admission evidence as a child contract under PeerStatus instead of renderer-local summaries."
  - "Used stable snake_case string labels for listener state, preflight reason, and latest admission event fields."
  - "Added inbound metrics only as bounded numeric MetricKind variants with no endpoint or peer identifier labels."
  - "Updated the parity breadcrumb registry because new tracked Rust files must satisfy repo-local verification."

patterns-established:
  - "PeerStatus grows through serde defaults plus explicit Rust constructor updates."
  - "Inbound status evidence remains unavailable with a reason until runtime projection wires real listener/admission data."
  - "MetricKind::ALL remains the authoritative bounded metric series list."

requirements-completed: [INB-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 90-2026-06-25T04-23-47
generated_at: 2026-06-25T07:17:29Z

duration: 1h 17m
completed: 2026-06-25
---

# Phase 90 Plan 05: Inbound Status and Metrics Contract Summary

**Shared inbound listener/admission status evidence and bounded numeric metric kinds for INB-05 observability**

## Performance

- **Duration:** 1h 17m
- **Started:** 2026-06-25T06:00:00Z
- **Completed:** 2026-06-25T07:17:29Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments

- Added `InboundPeerServingStatus` under `PeerStatus.inbound` with listener state, bounded endpoints, preflight reason, admission counters, handshake counts, rejection counters, and latest admission event evidence.
- Preserved legacy/stopped status behavior with `FieldAvailability::Unavailable` and the stable reason `inbound listener evidence unavailable`.
- Added inbound admission metric kinds for admitted, rejected, cap, reserved-slot, duplicate, and self-connection counts without endpoint or peer identifier labels.
- Verified the full `open-bitcoin-node` crate test suite after the status and metrics surface changes.

## Task Commits

1. **Task 1 RED: inbound status tests** - `da4397c` (test)
2. **Task 1 GREEN: inbound status contract** - `9d2e4ef` (feat)
3. **Task 2 RED: inbound metric tests** - `4c1713d` (test)
4. **Task 2 GREEN: inbound admission metrics** - `21a2666` (feat)

## Files Created/Modified

- `packages/open-bitcoin-node/src/status/inbound.rs` - Shared inbound status types and unavailable default.
- `packages/open-bitcoin-node/src/status/inbound/tests.rs` - Serialization and legacy-default coverage for inbound status evidence.
- `packages/open-bitcoin-node/src/status.rs` - Adds `PeerStatus.inbound` and re-exports the child contract.
- `packages/open-bitcoin-node/src/metrics.rs` - Adds six bounded inbound metric kinds and label tests.
- `packages/open-bitcoin-node/src/lib.rs` - Re-exports inbound status types for downstream consumers.
- `packages/open-bitcoin-node/src/status/tests.rs` - Updates node status fixtures for the new peer status field.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Updates sync summary peer status projection for the new field.
- `docs/parity/source-breadcrumbs.json` - Registers current and prior Phase 90 inbound Rust files with parity breadcrumbs.
- `.planning/phases/90-inbound-listener-and-admission-policy/deferred-items.md` - Records unrelated existing file-length gate failures.

## Decisions Made

- Kept status data as plain serializable fields with stable labels so RPC, CLI, support, and dashboard renderers can consume one shared source.
- Kept endpoint evidence bounded to `bound_endpoints` strings and did not add raw peer tables or endpoint-bearing metric labels.
- Treated status constructor updates and parity breadcrumb mappings as required repo-instruction work, even though they are outside the initial owned-file list.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Updated node-local `PeerStatus` constructors**
- **Found during:** Task 1 GREEN
- **Issue:** Adding `PeerStatus.inbound` made existing `open-bitcoin-node` struct literals fail to compile.
- **Fix:** Added conservative unavailable inbound evidence in `status/tests.rs` and `sync/types/summary.rs`.
- **Files modified:** `packages/open-bitcoin-node/src/status/tests.rs`, `packages/open-bitcoin-node/src/sync/types/summary.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::types::summary -- --nocapture` and full node tests passed.
- **Committed in:** `9d2e4ef`

**2. [Rule 3 - Blocking] Registered missing parity breadcrumb mappings**
- **Found during:** Post-task repo-rule verification
- **Issue:** `bun run scripts/check-parity-breadcrumbs.ts --check` reported missing mappings for the new status inbound files and earlier Phase 90 network inbound files.
- **Fix:** Added `network-inbound-admission` mappings and registered `status/inbound.rs` plus `status/inbound/tests.rs` under the node status contract group.
- **Files modified:** `docs/parity/source-breadcrumbs.json`
- **Verification:** `bun run scripts/check-parity-breadcrumbs.ts --check` passed for 272 Rust files.
- **Committed in:** pending metadata commit

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** Both changes are narrow verifier-enabling updates. No relay, permission-class, eviction, ban, public-network, or production-readiness behavior was added.

## Issues Encountered

- `bash scripts/check-file-lengths.sh` fails on unrelated existing files: `packages/open-bitcoin-network/src/peer.rs`, `packages/open-bitcoin-node/src/network.rs`, and `packages/open-bitcoin-rpc/src/config/loader.rs`. Plan 90-05 kept its new status child module below the limit and recorded the out-of-scope failures in `deferred-items.md`.
- Stub scan found only existing Rust format placeholders in `sync/types/summary.rs`; no UI/data stubs or placeholder implementation were introduced.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node inbound_status -- --nocapture` passed with 3 tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node metrics -- --nocapture` passed with 18 matching tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::types::summary -- --nocapture` passed with 5 tests.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features` passed with 235 tests and 1 ignored live-network smoke test.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed for 272 Rust files.
- `rg -n "endpoint_label|peer_id_label|remote_addr|remote_endpoint" packages/open-bitcoin-node/src/metrics.rs` found no high-cardinality metric labels.
- `bash scripts/check-file-lengths.sh` failed only on the unrelated existing over-limit files recorded above.

## Known Stubs

None.

## Threat Flags

None. The plan-covered threat surface was mitigated by preserving unavailable reasons, bounding endpoint status evidence, and keeping inbound metrics numeric and low-cardinality.

## Authentication Gates

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for downstream Phase 90 RPC, CLI, support, and runtime listener plans to populate and render `PeerStatus.inbound` and the new inbound metric kinds from managed listener/admission evidence.

## Self-Check: PASSED

- Found `packages/open-bitcoin-node/src/status/inbound.rs`.
- Found `packages/open-bitcoin-node/src/status/inbound/tests.rs`.
- Found `packages/open-bitcoin-node/src/status.rs`.
- Found `packages/open-bitcoin-node/src/metrics.rs`.
- Found `packages/open-bitcoin-node/src/lib.rs`.
- Found `.planning/phases/90-inbound-listener-and-admission-policy/90-05-SUMMARY.md`.
- Found `.planning/phases/90-inbound-listener-and-admission-policy/deferred-items.md`.
- Found `docs/parity/source-breadcrumbs.json`.
- Found commits `da4397c`, `9d2e4ef`, `4c1713d`, and `21a2666`.

---

*Phase: 90-inbound-listener-and-admission-policy*
*Completed: 2026-06-25*
