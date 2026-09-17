---
phase: 143-honest-stored-block-availability
reviewed: 2026-09-17T06:40:55Z
depth: standard
files_reviewed: 10
files_reviewed_list:
  - packages/open-bitcoin-node/src/network/inventory.rs
  - packages/open-bitcoin-node/src/network/block_serving.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs
  - packages/open-bitcoin-node/src/storage/fjall_store.rs
  - packages/open-bitcoin-node/src/network/action_translation.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-rpc/src/context.rs
  - packages/open-bitcoin-rpc/src/context/inbound_wire.rs
  - packages/open-bitcoin-network/src/block_serving.rs
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
---

# Phase 143: Code Review Report

**Reviewed:** 2026-09-17T06:40:55Z
**Depth:** standard
**Files Reviewed:** 10
**Status:** clean

## Summary

Reviewed the Phase 143 production path for honest stored-block availability: presence-fact assembly, the `has_block` payload-byte probe, inbound durable gating, LookupUnavailable rewrite, and reserved-Pruned classifier copy. The changes match the locked decisions: Available is authorized only by `payload_present` (cache or store `contains_key`), production assembly never injects `Pruned`, `classify_block_serving_status` stays I/O-free, post-gate missing bytes refuse with existing `NotFound` and rewrite status to Unavailable, and no `getblock` or Phase 144 operator UI was added.

All reviewed files meet quality standards. No issues found.

Locked-decision checks that passed:

- `managed_block_serve_input` sets `data_availability` from `payload_present` only; `index_known` and `validated_on_active_chain` are report-only.
- Durable inbound classify uses `durable_payload_present(block_hash)` via `InventoryServingMode::Durable`; memory paths stay `Immediate` / cache-only.
- `FjallNodeStore::has_block` probes `block:<64-hex>` on the same keyspace/key as `save_block` / `load_block` and does not decode the body.
- RPC `has_block` errors fail closed (`unwrap_or(false)`); store errors are not copied onto the wire.
- `LookupUnavailable` rewrites `status_label` to Unavailable and `payload_present` to false, so NotFound cannot increment `available_count`.
- Network-core classifier gained reserved-Pruned rustdoc only; no I/O or fact-shape change.

---

_Reviewed: 2026-09-17T06:40:55Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
