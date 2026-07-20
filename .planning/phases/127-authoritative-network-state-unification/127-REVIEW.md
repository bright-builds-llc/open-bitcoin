---
phase: 127-authoritative-network-state-unification
reviewed: 2026-07-20T00:24:00Z
depth: standard
files_reviewed: 24
files_reviewed_list:
  - docs/metrics/lines-of-code.md
  - docs/parity/catalog/p2p.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
  - packages/open-bitcoin-cli/src/operator/support/tests.rs
  - packages/open-bitcoin-node/src/network/block_serving.rs
  - packages/open-bitcoin-node/src/network/inventory.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/types.rs
  - packages/open-bitcoin-node/src/sync.rs
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/src/context.rs
  - packages/open-bitcoin-rpc/src/context/inbound_status.rs
  - packages/open-bitcoin-rpc/src/context/network.rs
  - packages/open-bitcoin-rpc/src/dispatch/node.rs
  - packages/open-bitcoin-rpc/src/inbound_listener.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs
  - packages/open-bitcoin-rpc/tests/black_box_parity.rs
  - scripts/check-phase124-milestone-gap-closure.test.ts
  - scripts/check-phase124-post-audit-gap-planning.ts
  - scripts/check-phase127-authoritative-network-state-unification.test.ts
  - scripts/check-phase127-authoritative-network-state-unification.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 2
  info: 0
  total: 2
status: issues_found
---

# Phase 127: Code Review Report

**Reviewed:** 2026-07-20T00:24:00Z
**Depth:** standard
**Files Reviewed:** 24
**Status:** issues_found

## Summary

This final auto re-review inspected the persisted original 24-file Phase 127 scope, the six review-fix commits through `f609d446`, and `scripts/rust-source-invariants.ts` as direct dependency context. WR-01 remains resolved: a pre-existing RPC context reloads current durable metadata per `getblockchaininfo` request and maps storage-read failure to unavailable. WR-04 remains resolved: the production support command consumes the authoritative raw RPC status and the regression inspects both generated JSON and Markdown for every forbidden material class.

The ordered response enum in `21b3c906` fixes the tested transaction/block permutations, including two-block and multi-cycle batches. One Knots queue case remains incorrect: an unknown inventory item produces `notfound` instead of being silently consumed. The `f609d446` parser extraction also remains bypassable at each protected data-flow boundary. Separate deterministic temporary-corpus mutations replaced production authority, durable lookup output, and operator projection while retaining dead or unused anchors; all three incorrectly returned an empty checker failure list.

The review applied repo-local authority, parity, generated-artifact, and verification guidance from `AGENTS.md`, the Bright Builds sidecar, and `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/operability.md`, `standards/core/testing.md`, `standards/core/verification.md`, `standards/languages/rust.md`, and `standards/languages/typescript-javascript.md`. No active standards override applies.

## Warnings

### WR-02: Unknown `getdata` inventory now receives a response that Knots never sends

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/network/inventory.rs:163-167`

**Issue:** The ordered queue loop correctly processes leading transactions, at most one following block-like item, and accumulated transaction misses. However, its fallback branch emits a one-item `WireNetworkMessage::NotFound` for `InventoryType::Unknown(_)`. The pinned Knots `ProcessGetData` implementation consumes a non-transaction queue head and explicitly erases an unknown type without responding (`packages/bitcoin-knots/src/net_processing.cpp:2465-2471`). Consequently, `[Unknown(U), available Transaction(T)]` emits `notfound(U), tx(T)` here but only `tx(T)` in Knots; `[missing Transaction(M), Unknown(U), available Transaction(T)]` also gains an extra response and changes the observable response sequence.

**Fix:** Handle `InventoryType::Unknown(_)` by consuming it without adding a response-plan item, while keeping the ordered cycle boundary. Add exact wire-response regressions for `[Unknown, available tx]` and `[missing tx, Unknown, available tx]` against the pinned `ProcessGetData` behavior.

### WR-03: The Phase 127 checker still validates anchors rather than the executed data flow

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase127-authoritative-network-state-unification.ts:92-125`

**Issue:** The extracted scanner improves literal/comment handling, but the checker still treats matching syntax anywhere in an extracted function as proof of the executed path. Production authority accepts one exact constructor call in an unreachable branch while the live context is built by a helper using an aliased duplicate-authority constructor; the global rejection is a string search that does not resolve aliases. Durable serving validates the initializer of `maybe_block` but not that the subsequent `match` consumes that binding, so an unused durable read plus a replacement lookup result passes. Operator projection uses `rustFieldInitializers` from `scripts/rust-source-invariants.ts:26-36` across the entire function, so an exact field in an unreachable decoy struct plus shorthand on the real returned field also passes. Each of those three separate temporary-corpus mutations returned `[]`, demonstrating that the checker can accept the split authority, durable-source bypass, and stale/default projection it is meant to prevent.

**Fix:** Parse and validate the concrete expressions that feed the live result: require the `context` initializer to be the authoritative constructor and resolve helper/alias origins; require the block-resolution `match` discriminant to be the exact durable-source binding; and inspect the fields of the actual returned `OpenBitcoinNetworkStatusResponse`, excluding nested or unreachable decoys. Prefer a compiler-backed Rust syntax tree if the lexical scanner cannot represent binding/use relationships. Add the three demonstrated mutations as regressions and require each to produce the corresponding Phase 127 failure.

## Verification

- `bun test scripts/check-phase127-authoritative-network-state-unification.test.ts` — passed, 12 tests.
- `bun run scripts/check-phase127-authoritative-network-state-unification.ts` — passed against the live repository.
- Deterministic pinned-source comparison — confirmed Open Bitcoin's unknown branch emits `NotFound` while Knots explicitly erases unknown queue heads silently.
- Three independent temporary-corpus authority/durable/projection bypass mutations — checker incorrectly returned `[]` for each.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --test black_box_parity phase127` through the timing wrapper — passed, 1 test.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc durable_block_serving` through the timing wrapper — passed, 4 tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli authoritative_rpc_status_support_bundle_redacts_every_forbidden_material_class_in_json_and_markdown` through the timing wrapper — passed, 1 test.
- `git diff --check` — passed before writing this report.
- Source files were not modified.

***

_Reviewed: 2026-07-20T00:24:00Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
