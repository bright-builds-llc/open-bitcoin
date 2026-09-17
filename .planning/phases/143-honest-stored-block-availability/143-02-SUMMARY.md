---
phase: 143-honest-stored-block-availability
plan: 02
subsystem: network
tags: [block-serving, has-block, payload-probe, durable-serving, unavailable, rust]

requires:
  - phase: 143-honest-stored-block-availability
    provides: Presence facts and reserved Pruned; durable_payload_present is already a fact
provides:
  - "FjallNodeStore::has_block contains_key probe"
  - "DurableBlockSource::has_block fail-closed into the durable gate"
  - "gate_inventory_for_durable_serving uses durable_payload_present(block_hash)"
affects:
  - 143-03
  - durable-payload-probe
  - lookup-unavailable

tech-stack:
  added: []
  patterns:
    - "Payload presence is cache OR store contains_key; classify stays I/O-free"
    - "Store probe errors map to false and never reach the wire"

key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/block_presence.rs
  modified:
    - packages/open-bitcoin-node/src/network/inventory.rs
    - packages/open-bitcoin-node/src/network/action_translation.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-rpc/src/context.rs
    - packages/open-bitcoin-rpc/src/context/inbound_wire.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Combined 143-02 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh"
  - "Pack defer-or-probe into InventoryServingMode so clippy too_many_arguments stays at 7"
  - "node-storage-contract breadcrumbs stay none; HaveBlockData lives in has_block rustdoc"
  - "Leave HAVL-01 Pending until later plans and lifecycle-valid phase verification"

patterns-established:
  - "Pattern 1: Durable inbound classify uses a fail-closed has_block callback, never literal true"
  - "Pattern 2: Immediate serving stays cache-only through InventoryServingMode::Immediate"

requirements-completed: [HAVL-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 143-2026-09-17T01-47-45
generated_at: 2026-09-17T04:37:30Z

duration: 28min
completed: 2026-09-17
---

# Phase 143 Plan 02: Payload-Byte Probe Replaces Override Summary

**Durable inbound serving now classifies Available only after a cache-or-store `contains_key` probe; the unconditional `true` override is gone.**

## Performance

- **Duration:** 28 min
- **Started:** 2026-09-17T04:09:41Z
- **Completed:** 2026-09-17T04:37:30Z
- **Tasks:** 2
- **Files modified:** 16

## Accomplishments

- `FjallNodeStore::has_block` is a `contains_key` probe on `block:<64-hex>` and does not decode the body.
- `DurableBlockSource::has_block` is threaded as `Fn(BlockHash) -> bool` into `receive_message_for_durable_serving` / `gate_inventory_for_durable_serving`.
- Missing durable body classifies `Unavailable` at the gate with no `DurableBlock` intent; resolve still returns `NotFound` and `served_count == 0`.
- Probe errors fail closed as absent and are not copied onto the wire. Classifier I/O boundary is unchanged.

## Task Commits

Each task was committed atomically:

1. **Task 1 + Task 2: has_block tests and durable probe wiring** - `bd8b7f77` (feat)

**Plan metadata:** pending docs commit after this summary.

_Note: Combined RED+GREEN in one hook-passing feat commit because pre-commit runs `verify.sh`, matching Phases 140–142 and 143-01._

## Files Created/Modified

- `packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs` - `has_block` contains_key probe
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` - `mod blocks;` only
- `packages/open-bitcoin-node/src/storage/fjall_store/tests/block_presence.rs` - save/missing key and no-decode scan
- `packages/open-bitcoin-node/src/network/inventory.rs` - gate uses `durable_payload_present(block_hash)`
- `packages/open-bitcoin-node/src/network/action_translation.rs` - probe callback plus `InventoryServingMode`
- `packages/open-bitcoin-node/src/network/runtime_authority.rs` - durable receive forwards the probe
- `packages/open-bitcoin-node/src/network.rs` - memory paths stay `Immediate`
- `packages/open-bitcoin-rpc/src/context.rs` - fail-closed `has_block` closure
- `packages/open-bitcoin-rpc/src/context/inbound_wire.rs` - trait `has_block` plus crate-visible plan items
- `packages/open-bitcoin-rpc/src/inbound_listener/tests.rs` - scripted source returns `Ok(true)`
- `packages/open-bitcoin-rpc/src/inbound_listener/tests/block_serving.rs` - gate Unavailable without DurableBlock
- `packages/open-bitcoin-node/src/network/tests/block_serving.rs` - no literal true and cache-or-store OR
- `packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs` - tx GetData passes ` |_| false`
- `docs/parity/source-breadcrumbs.json` - new store files under `node-storage-contract`
- `docs/metrics/lines-of-code.md` - Hook-regenerated LOC

## Decisions Made

- Combined RED+GREEN because hooks run the workspace verifier; a RED-only commit cannot pass pre-commit.
- Pack `defer_block_serving` and the probe into `InventoryServingMode` so internal helpers stay at 7 arguments.
- Keep `node-storage-contract` file comments as `none`; the group mapping is empty, so a Knots breadcrumb on only these two files would fail the checker. `has_block` rustdoc names Knots `HaveBlockData` / `CheckBlockDataAvailability`.
- Leave HAVL-01 Pending. Plan 03 still owns `LookupUnavailable` status honesty, and phase verification is later.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] InventoryServingMode avoids clippy too_many_arguments**
- **Found during:** Task 2 commit hook
- **Issue:** Adding the probe as an extra argument made `receive_message_with_block_serving_mode` and `process_actions` 8/7.
- **Fix:** Replace the bool-plus-callback pair with `InventoryServingMode::{Immediate, Durable(F)}`. Public durable receive/gate APIs still take `Fn(BlockHash) -> bool`.
- **Files modified:** `packages/open-bitcoin-node/src/network/action_translation.rs`, `packages/open-bitcoin-node/src/network.rs`
- **Verification:** Node clippy `-D warnings` and `network::tests::block_serving` passed; `verify.sh` passed
- **Committed in:** `bd8b7f77` (combined feat commit)

**2. [Rule 1 - Bug] Presence test shadowed `block_hash`**
- **Found during:** Task 1 compile
- **Issue:** `let block_hash = store.save_block(...)` hid the `block_hash()` helper.
- **Fix:** Rename the binding to `saved_hash`.
- **Files modified:** `packages/open-bitcoin-node/src/storage/fjall_store/tests/block_presence.rs`
- **Verification:** `storage::fjall_store::tests::block_presence` passed
- **Committed in:** `bd8b7f77` (combined feat commit)

***

**Total deviations:** 2 auto-fixed (1 blocking, 1 bug)
**Impact on plan:** Narrow compile/clippy guards. No scope creep. Combined RED+GREEN was an allowed plan exception, not a deviation.

## Issues Encountered

The first hook-passing commit failed on clippy `too_many_arguments` and `unnecessary_get_then_check`. Both were fixed before the successful feat commit.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 03 can rewrite `LookupUnavailable` `status_label` to Unavailable without changing this probe seam.
- Existing corruption/backend redaction tests still reach `load_block` because `ScriptedDurableBlockSource::has_block` returns `Ok(true)`.
- No new operator UI, `getblock`, or prune-mode product behavior was added.

***
*Phase: 143-honest-stored-block-availability*
*Completed: 2026-09-17*

## Self-Check: PASSED
