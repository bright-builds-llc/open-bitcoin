---
phase: 92-address-advertisement-and-discovery-boundaries
reviewed: 2026-06-26T10:16:56Z
depth: standard
files_reviewed: 43
files_reviewed_list:
  - docs/architecture/operator-observability.md
  - docs/architecture/status-snapshot.md
  - docs/metrics/lines-of-code.md
  - docs/operator/runtime-guide.md
  - docs/parity/catalog/p2p.md
  - docs/parity/checklist.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-cli/src/operator/status/render/inbound.rs
  - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
  - packages/open-bitcoin-cli/src/operator/status/tests.rs
  - packages/open-bitcoin-cli/src/operator/support/redaction.rs
  - packages/open-bitcoin-cli/src/operator/support/render/inbound.rs
  - packages/open-bitcoin-cli/src/operator/support/tests.rs
  - packages/open-bitcoin-network/src/address.rs
  - packages/open-bitcoin-network/src/address/advertisement.rs
  - packages/open-bitcoin-network/src/address/book.rs
  - packages/open-bitcoin-network/src/address/response.rs
  - packages/open-bitcoin-network/src/address/tests.rs
  - packages/open-bitcoin-network/src/inbound.rs
  - packages/open-bitcoin-network/src/inbound/tests.rs
  - packages/open-bitcoin-network/src/lib.rs
  - packages/open-bitcoin-network/src/message.rs
  - packages/open-bitcoin-network/src/message/tests.rs
  - packages/open-bitcoin-network/src/peer.rs
  - packages/open-bitcoin-network/src/peer/address_boundary.rs
  - packages/open-bitcoin-network/src/peer/inventory_state.rs
  - packages/open-bitcoin-network/src/peer/tests.rs
  - packages/open-bitcoin-node/src/lib.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/inbound.rs
  - packages/open-bitcoin-node/src/network/tests.rs
  - packages/open-bitcoin-node/src/status/inbound.rs
  - packages/open-bitcoin-node/src/status/inbound/tests.rs
  - packages/open-bitcoin-node/src/status/tests.rs
  - packages/open-bitcoin-rpc/src/context.rs
  - packages/open-bitcoin-rpc/src/context/address_boundary.rs
  - packages/open-bitcoin-rpc/src/context/network.rs
  - packages/open-bitcoin-rpc/src/dispatch/tests.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs
  - scripts/check-phase92-address-boundaries.test.ts
  - scripts/check-phase92-address-boundaries.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 2
  info: 0
  total: 2
status: issues_found
---

# Phase 92: Code Review Report

**Reviewed:** 2026-06-26T10:16:56Z
**Depth:** standard
**Files Reviewed:** 43
**Status:** issues_found

## Summary

Reviewed the listed Phase 92 documentation, Rust networking/node/RPC/CLI surfaces, support redaction and rendering, and TypeScript boundary checker wiring at standard depth. Project guidance consulted: `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/verification.md`, `standards/core/testing.md`, `standards/languages/rust.md`, and `standards/languages/typescript-javascript.md`.

Two warning-level correctness gaps were found: empty-payload wire commands accept trailing bytes inconsistently, and over-cap `addr` batches under-report learned-address rejections in the status/support boundary.

## Warnings

### WR-01: Empty-Payload Wire Messages Accept Trailing Bytes

**File:** `packages/open-bitcoin-network/src/message.rs:252-254`
**Issue:** `decode_payload` validates empty payloads for `getaddr`, but not for `verack`, `wtxidrelay`, or `sendheaders`. Those commands are modeled as empty-payload messages, so accepting trailing bytes can make malformed peer input look valid and creates inconsistent wire handling across equivalent no-body commands. The existing tests cover non-empty rejection for `getaddr`, but only round-trip empty payloads for these three variants.
**Fix:**
```rust
"verack" => {
    decode_empty_payload(payload)?;
    Ok(Self::Verack)
}
"wtxidrelay" => {
    decode_empty_payload(payload)?;
    Ok(Self::WtxidRelay)
}
"sendheaders" => {
    decode_empty_payload(payload)?;
    Ok(Self::SendHeaders)
}
```
Add regression tests that non-empty payloads for `verack`, `wtxidrelay`, and `sendheaders` return `NetworkMessageError::InvalidEncoding`.

### WR-02: Over-Cap Addr Batches Drop Rejection Counts

**File:** `packages/open-bitcoin-network/src/peer/address_boundary.rs:85-94`
**Issue:** `record_learned_addresses` records the batch-level `over_cap_batch` decision and returns, but does not preserve the rejected-address count from `LearnedAddressBook::learn_batch`. Downstream status projection reports `learned_address_rejections` from `evidence.learned_address_rejections.len()` in `packages/open-bitcoin-node/src/network/inbound.rs:78`, so an over-cap batch with many rejected addresses can be reported as zero learned-address rejections. This weakens the operator/RPC support surface for the exact bounded-intake behavior Phase 92 is documenting.
**Fix:** Preserve aggregate rejection evidence from `AddressBatchLearnResult::rejected_count` in the peer boundary evidence and project that numeric count instead of relying only on per-address rejection vector length. For example, add a `learned_address_rejection_count` field to the evidence, increment it by `batch.rejected_count` for over-cap and per-address rejection paths, and have the node status projection use that field. Add a regression test that an over-cap `addr` batch reports `learned_address_rejections == addresses.len()` while still recording the latest decision reason as `over_cap_batch`.

---

_Reviewed: 2026-06-26T10:16:56Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
