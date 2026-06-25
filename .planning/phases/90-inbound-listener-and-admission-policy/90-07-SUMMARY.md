---
phase: 90-inbound-listener-and-admission-policy
plan: 07
subsystem: cli-status
tags: [rust, cli, status, inbound, rpc]

requires:
  - phase: 90-06
    provides: Open Bitcoin RPC extension method `openbitcoinnetworkstatus`
  - phase: 90-05
    provides: Shared `InboundPeerServingStatus` contract under `PeerStatus.inbound`
provides:
  - Operator status collection for shared inbound listener/admission evidence
  - Human status rendering that separates inbound serving from outbound sync and peer counts
  - Non-fatal older-daemon fallback for missing `openbitcoinnetworkstatus`
  - Focused status collector and renderer coverage for inbound evidence
affects:
  - phase-90-operator-uat
  - phase-90-final-verification
  - phase-91-peer-permissions

tech-stack:
  added: []
  patterns:
    - Optional Open Bitcoin extension RPC collection preserves baseline `getnetworkinfo` peer counts
    - Inbound serving human rendering is a projection of the shared status snapshot
    - New first-party Rust renderer files are registered in parity breadcrumb metadata

key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/status/render/inbound.rs
    - .planning/phases/90-inbound-listener-and-admission-policy/90-07-SUMMARY.md
  modified:
    - packages/open-bitcoin-cli/src/operator/status.rs
    - packages/open-bitcoin-cli/src/operator/status/http.rs
    - packages/open-bitcoin-cli/src/operator/status/render.rs
    - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - docs/parity/source-breadcrumbs.json
    - .planning/phases/90-inbound-listener-and-admission-policy/deferred-items.md

key-decisions:
  - "Kept detailed inbound evidence on `PeerStatus.inbound` and left JSON rendering as the shared snapshot serde projection."
  - "Made `openbitcoinnetworkstatus` collection non-fatal so older daemons preserve `getnetworkinfo` peer counts with unavailable inbound evidence."
  - "Rendered inbound serving on its own human line after `Peers:` so inbound listener/admission evidence stays separate from outbound sync."

patterns-established:
  - "Status RPC adapters expose optional Open Bitcoin extension methods through the injected `StatusRpcClient` trait."
  - "Human inbound status formatting lives in `status/render/inbound.rs` to keep the main renderer below the file-length trigger."

requirements-completed: [INB-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 90-2026-06-25T04-23-47
generated_at: 2026-06-25T08:30:30Z

duration: 10 min
completed: 2026-06-25
---

# Phase 90 Plan 07: Operator Status Inbound Evidence Summary

**Open Bitcoin operator status now collects and renders shared inbound listener/admission evidence without merging it into outbound sync or baseline peer counts**

## Performance

- **Duration:** 10 min
- **Started:** 2026-06-25T08:20:18Z
- **Completed:** 2026-06-25T08:30:30Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments

- Extended `StatusRpcClient` and `HttpStatusRpcClient` with `get_open_bitcoin_network_status`, calling `openbitcoinnetworkstatus`.
- Projected live `OpenBitcoinNetworkStatusResponse.inbound` into `OpenBitcoinStatusSnapshot.peers.inbound` while keeping `PeerCounts` sourced from `getnetworkinfo.connections_in/out`.
- Converted method-missing or older-daemon inbound status failures into explicit unavailable inbound evidence instead of failing the whole live status snapshot.
- Added a focused `Inbound serving:` human status line for listener state, bounded endpoints, preflight reason, admission counts, handshake counts, rejection counters, and latest admission event.
- Registered the new status renderer source in parity breadcrumb metadata required by repo verification.

## Task Commits

1. **Task 1 RED: inbound status collection coverage** - `b31decc` (test)
2. **Task 1 GREEN: collect inbound status evidence** - `ea8e0ec` (feat)
3. **Task 2 RED: inbound status render coverage** - `ca34982` (test)
4. **Task 2 GREEN: render inbound serving status** - `c75dedb` (feat)

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator/status.rs` - Collects optional inbound RPC evidence and stores unavailable reasons on `PeerStatus.inbound`.
- `packages/open-bitcoin-cli/src/operator/status/http.rs` - Calls `openbitcoinnetworkstatus` through the HTTP JSON-RPC adapter.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - Wires a separate `Inbound serving:` line after the existing `Peers:` count line.
- `packages/open-bitcoin-cli/src/operator/status/render/inbound.rs` - Renders shared inbound listener/admission evidence for human status output.
- `packages/open-bitcoin-cli/src/operator/status/render/tests.rs` - Covers inbound human labels and unavailable reason rendering.
- `packages/open-bitcoin-cli/src/operator/status/tests.rs` - Covers live inbound collection, older-daemon fallback, and secret non-rendering.
- `docs/parity/source-breadcrumbs.json` - Registers the new first-party Rust renderer file.
- `.planning/phases/90-inbound-listener-and-admission-policy/deferred-items.md` - Records non-owned CLI compile blockers encountered by 90-07 verification.

## Decisions Made

- Detailed inbound status is optional extension evidence: if `openbitcoinnetworkstatus` is absent, status remains running and peer counts stay available from `getnetworkinfo`.
- Human status keeps `Peers: in=<n> out=<n>` unchanged and uses `Inbound serving:` for listener/admission diagnostics.
- JSON output remains the shared `OpenBitcoinStatusSnapshot` serialization; no CLI-specific inbound DTO was introduced.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Registered parity breadcrumb metadata for the new status renderer**
- **Found during:** Task 2 (Render inbound serving separately from outbound sync)
- **Issue:** The new first-party Rust file `status/render/inbound.rs` requires parity breadcrumb registry coverage under repo-local rules.
- **Fix:** Added the new renderer path to `docs/parity/source-breadcrumbs.json`.
- **Files modified:** `docs/parity/source-breadcrumbs.json`
- **Verification:** `bun run scripts/check-parity-breadcrumbs.ts --check` passed for 276 Rust files.
- **Committed in:** `c75dedb`

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Required for repo verification metadata. No runtime behavior scope was expanded beyond status rendering.

## Issues Encountered

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli inbound_status -- --nocapture` remains blocked by non-owned compile gaps: missing `PeerStatus.inbound` fixtures in dashboard/runtime/soak files and a non-exhaustive dashboard `MetricKind` match for inbound metrics.
- `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json --no-live-rpc` remains blocked by the same non-owned dashboard `MetricKind` compile gap.
- `bash scripts/check-file-lengths.sh` remains blocked by pre-existing over-limit files: `packages/open-bitcoin-network/src/peer.rs`, `packages/open-bitcoin-node/src/network.rs`, and `packages/open-bitcoin-rpc/src/config/loader.rs`.

## Verification

- `rustfmt --edition 2024 --check packages/open-bitcoin-cli/src/operator/status.rs packages/open-bitcoin-cli/src/operator/status/http.rs packages/open-bitcoin-cli/src/operator/status/render.rs packages/open-bitcoin-cli/src/operator/status/render/inbound.rs packages/open-bitcoin-cli/src/operator/status/render/tests.rs packages/open-bitcoin-cli/src/operator/status/tests.rs` passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed for 276 Rust files.
- `rg -n "openbitcoinnetworkstatus|peers\\.inbound|Inbound serving:|Unavailable" ...` passed across the 90-07 status files.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli inbound_status -- --nocapture` failed on non-owned compile blockers listed above.
- `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json --no-live-rpc` failed on the non-owned dashboard metric match listed above.
- `bash scripts/check-file-lengths.sh` failed on the pre-existing non-owned over-limit files listed above.

## Known Stubs

None - stub and placeholder scan found no matches in the files created or modified by this plan.

## Threat Flags

None. The plan used the existing Open Bitcoin RPC extension from 90-06 and rendered the shared status contract without adding new network endpoints, raw peer tables, credential output, or production-readiness claims.

## Authentication Gates

None.

## User Setup Required

None - no external service configuration required.

## State Updates

Skipped intentionally. The orchestrator explicitly owns `.planning/STATE.md` and `.planning/ROADMAP.md` for this parallel phase run.

## Next Phase Readiness

Ready for Phase 90 UAT/release-boundary work once the non-owned CLI dashboard/runtime/soak compile blockers are resolved by their owning plan. The 90-07 status-owned files now collect and render inbound evidence from the shared contract.

## Self-Check: PASSED

- Found `.planning/phases/90-inbound-listener-and-admission-policy/90-07-SUMMARY.md`.
- Found `packages/open-bitcoin-cli/src/operator/status/render/inbound.rs`.
- Found task commits `b31decc`, `ea8e0ec`, `ca34982`, and `c75dedb`.

---
*Phase: 90-inbound-listener-and-admission-policy*
*Completed: 2026-06-25*
