---
phase: 144-operator-flush-and-availability-evidence
plan: 02
subsystem: observability
tags: [chainstate-durability, csobs, rpc, cli, dashboard, human-tokens, rust]

requires:
  - phase: 144-operator-flush-and-availability-evidence
    provides: Dedicated chainstate_durability FieldAvailability field on OpenBitcoinStatusSnapshot
provides:
  - "openbitcoinnetworkstatus.chainstate_durability cloned from the managed snapshot"
  - "Live CLI JSON copies the RPC field; human status prints six locked Label: value lines after block-relay"
  - "Dashboard six rows after block-relay inside Mempool and Wallet; MAX_DASHBOARD_CHARTS stays 8"
affects:
  - 144-03
  - 144-04
  - metrics-logs-support
  - docs-checker

tech-stack:
  added: []
  patterns:
    - "Live CLI copies OpenBitcoinNetworkStatusResponse.chainstate_durability, not only the in-process snapshot"
    - "Human cache_size tokens OK/LARGE/CRITICAL; machine JSON stays ok/large/critical"
    - "Unavailable wrapper copies the same Unavailable: {reason} onto all six lines or rows"

key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/status/render/chainstate_durability.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/chainstate_durability.rs
  modified:
    - packages/open-bitcoin-rpc/src/method/node.rs
    - packages/open-bitcoin-rpc/src/dispatch/node.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests/network_status_schema.rs
    - packages/open-bitcoin-rpc/src/context/inbound_status.rs
    - packages/open-bitcoin-cli/src/operator/status.rs
    - packages/open-bitcoin-cli/src/operator/status/render.rs
    - packages/open-bitcoin-cli/src/operator/status/tests/snapshot.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests/projection.rs
    - packages/open-bitcoin-node/tests/black_box_parity/phase127_composition.rs
    - scripts/check-phase127-authoritative-network-state-unification.ts

key-decisions:
  - "Combined each task RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh"
  - "Live CLI copies OpenBitcoinNetworkStatusResponse.chainstate_durability rather than the in-process snapshot"
  - "Duplicate small human formatters in CLI render and dashboard model; no shared web crate"
  - "Leave CSOBS-01 and CSOBS-02 Pending until Plan 03 surfaces and lifecycle-valid phase verification"

patterns-established:
  - "Pattern 1: RPC and live CLI clone the shared field; renderers format enums only"
  - "Pattern 2: Six locked labels after block-relay and before Wallet; no blank lines or Pruned copy"
  - "Pattern 3: Dashboard rows stay inside Mempool and Wallet; eight charts and five sections unchanged"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 144-2026-09-17T11-23-17
generated_at: 2026-09-17T15:00:51Z

duration: 67min
completed: 2026-09-17
---

# Phase 144 Plan 02: RPC, CLI, And Dashboard Projection Summary

**`openbitcoinnetworkstatus` now publishes the shared `chainstate_durability` field; live CLI JSON copies that RPC value and human status plus the dashboard print the six locked lines after block-relay with `OK` / `LARGE` / `CRITICAL` cache_size tokens.**

## Performance

- **Duration:** 67 min
- **Started:** 2026-09-17T13:53:44Z
- **Completed:** 2026-09-17T15:00:51Z
- **Tasks:** 3
- **Files modified:** 20

## Accomplishments

- `OpenBitcoinNetworkStatusResponse` carries `chainstate_durability` immediately after `block_relay`. Dispatch clones `snapshot.chainstate_durability()` with no relabeling, `decide_flush`, or Fjall. `getblockchaininfo` and `getnetworkinfo` stay unchanged.
- Live CLI collect assigns `network_status.chainstate_durability`. RPC-failure fallback uses inbound-style `openbitcoinnetworkstatus unavailable: {detail}` on the wrapper. Human status prints six single-spaced `Label: value` lines after the block-relay cluster and before `Wallet:`.
- Dashboard appends the same six title-case rows after `block_relay_rows` inside Mempool and Wallet. `MAX_DASHBOARD_CHARTS` remains 8; section titles and the 35/35/30 split stay five sections with no ninth chart and no new section.

## Task Commits

Each task was committed atomically:

1. **Task 1: Publish chainstate_durability on openbitcoinnetworkstatus** - `9845b63d` (feat)
2. **Task 2: CLI JSON collect and six human lines** - `fcfe97cb` (feat)
3. **Task 3: Dashboard rows after block-relay** - `82ace714` (feat)

**Plan metadata:** pending docs commit after this summary.

_Note: Combined RED+GREEN in one hook-passing feat commit per task because pre-commit runs `verify.sh`, matching Phases 140–143 and 144-01._

## Files Created/Modified

- `packages/open-bitcoin-rpc/src/method/node.rs` - `#[serde(default)] pub chainstate_durability` after `block_relay`
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` - `chainstate_durability: snapshot.chainstate_durability().clone()`
- `packages/open-bitcoin-rpc/src/dispatch/tests/network_status_schema.rs` - Projection, exact keys, omitted getblock/pruned/peer_id, unchanged baseline methods
- `packages/open-bitcoin-rpc/src/context/inbound_status.rs` - Thin `chainstate_durability()` getter
- `packages/open-bitcoin-cli/src/operator/status.rs` - Live RPC copy plus inbound-style unavailable fallback
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - Extends human lines after block-relay
- `packages/open-bitcoin-cli/src/operator/status/render/chainstate_durability.rs` - Six locked human lines and OK/LARGE/CRITICAL mapping
- `packages/open-bitcoin-cli/src/operator/status/tests/snapshot.rs` - Live collect, fallback, JSON, and no-blank-line coverage
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - Extends Mempool and Wallet after block-relay
- `packages/open-bitcoin-cli/src/operator/dashboard/model/chainstate_durability.rs` - Six locked dashboard rows
- `packages/open-bitcoin-cli/src/operator/dashboard/model/tests/projection.rs` - Shared-contract, unavailable, eight-chart, and five-section tests
- `packages/open-bitcoin-node/tests/black_box_parity/phase127_composition.rs` - Exact keys include `chainstate_durability`
- `scripts/check-phase127-authoritative-network-state-unification.ts` - Integration anchor updated
- `docs/metrics/lines-of-code.md` - Hook-regenerated LOC

## Decisions Made

- Combined RED+GREEN because hooks run the workspace verifier; a RED-only commit cannot pass pre-commit.
- Live CLI copies `OpenBitcoinNetworkStatusResponse.chainstate_durability` rather than only the in-process snapshot (Pitfall 1).
- Duplicate the small human formatters in CLI render and dashboard model. The plan forbids a shared web crate.
- Leave CSOBS-01 and CSOBS-02 Pending. Plan 03 still owns metrics, logs, and support; phase verification is later.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Thin snapshot getter for RPC dispatch**
- **Found during:** Task 1
- **Issue:** `AuthoritativeOperatorSnapshot` / inbound status context had no `chainstate_durability()` method, so `snapshot.chainstate_durability().clone()` would not compile.
- **Fix:** Added a thin getter that returns the shared `FieldAvailability` reference.
- **Files modified:** `packages/open-bitcoin-rpc/src/context/inbound_status.rs`
- **Verification:** `cargo test -p open-bitcoin-rpc --all-features open_bitcoin_network_status` and pre-commit `verify.sh`
- **Committed in:** `9845b63d` (Task 1)

**2. [Rule 3 - Blocking] Compile-fix snapshot constructors after the new required field**
- **Found during:** Task 1
- **Issue:** CLI, support, and bench snapshot literals omitted `chainstate_durability` after Plan 01 added the field to live RPC/status construction paths used here.
- **Fix:** Set `default_unavailable()` / default field values on those constructors only. No Plan 03 metrics/logs/support products.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/status.rs`, `packages/open-bitcoin-cli/src/operator/status/tests.rs`, `packages/open-bitcoin-cli/src/operator/status/tests/snapshot.rs`, `packages/open-bitcoin-cli/src/operator/status/tests/status_inputs.rs`, `packages/open-bitcoin-cli/src/operator/status/tests/mempool_policy.rs`, `packages/open-bitcoin-cli/src/operator/support/tests/recovery_progress_inbound.rs`, `packages/open-bitcoin-cli/benches/cases/operator_runtime.rs`
- **Verification:** pre-commit `verify.sh`
- **Committed in:** `9845b63d` (Task 1)

**3. [Rule 3 - Blocking] Phase 127 exact keys include chainstate_durability**
- **Found during:** Task 1
- **Issue:** Phase 127 composition and the unification checker still expected the five-key `openbitcoinnetworkstatus` object.
- **Fix:** Added `chainstate_durability` to the sorted exact-key list and checker anchor.
- **Files modified:** `packages/open-bitcoin-node/tests/black_box_parity/phase127_composition.rs`, `scripts/check-phase127-authoritative-network-state-unification.ts`
- **Verification:** pre-commit `verify.sh`
- **Committed in:** `9845b63d` (Task 1)

**4. [Rule 3 - Blocking] CLI tests live beside collect fixtures in snapshot.rs**
- **Found during:** Task 2
- **Issue:** The plan listed `status/tests/rendering_and_service.rs`, but live `openbitcoinnetworkstatus` collect and human-line fixtures already live in `snapshot.rs`.
- **Fix:** Added the four named tests next to those fixtures instead of splitting collect coverage into a render-only file.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/status/tests/snapshot.rs`
- **Verification:** `cargo test -p open-bitcoin-cli --lib operator_status_chainstate_durability` plus the JSON and omit-pruned tests
- **Committed in:** `fcfe97cb` (Task 2)

***

**Total deviations:** 4 auto-fixed (4 blocking)
**Impact on plan:** All auto-fixes necessary for compile, Phase 127 exact keys, and test placement. No metrics/logs/support, no getblock, no ninth chart. Combined RED+GREEN was an allowed plan exception, not a deviation.

## Issues Encountered

- First Task 2 commit failed rustfmt because uncommitted Task 3 dashboard files were already dirty in the same worktree. Ran `cargo fmt --all`, then committed Task 2 without staging dashboard files. No amend.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 03 can emit metrics and structured logs from the same sanitized labels and redact support bundles without a second derivation.
- Plan 04 can document the six locked lines and wire the cross-surface checker.
- CSOBS-01 and CSOBS-02 stay Pending until those surfaces and lifecycle-valid phase verification exist.

***
*Phase: 144-operator-flush-and-availability-evidence*
*Completed: 2026-09-17*

## Self-Check: PASSED
