# Phase 114: Compact Block Reconstruction from Mempool State - Research

**Researched:** 2026-07-05
**Domain:** BIP152 compact-block reconstruction from mempool and extra transaction inputs
**Confidence:** HIGH

## Summary

Phase 114 ports the Knots `PartiallyDownloadedBlock::InitData` reconstruction boundary into pure Rust modules. Short-ID selector keys derive from `SHA256(header || nonce)`; witness-hash short IDs use SipHash-2-4 masked to six bytes. `PartialCompactBlock` holds volatile slots, reports missing indexes on `Ready`, and exposes lifecycle cleanup hooks without wire scheduling.

**Primary recommendation:** Keep reconstruction in `open-bitcoin-network::compact_reconstruction` with iterator-based transaction inputs and SipHash in `open-bitcoin-consensus`.

## Knots Anchors

- `CBlockHeaderAndShortTxIDs::FillShortTxIDSelector` and `GetShortID` in `blockencodings.cpp`
- `PartiallyDownloadedBlock::InitData` prefilled placement, short-ID map collision/bucket checks, mempool scan, extra-txn scan
- `MAX_BLOCK_WEIGHT / MIN_SERIALIZABLE_TRANSACTION_WEIGHT` transaction bound
- Bucket overload threshold of 12 entries per bucket

## Implementation Split

| Plan | Focus |
|------|-------|
| 114-01 | SipHash, short-ID selector helpers, `PartialCompactBlock` model |
| 114-02 | Mempool and extra-transaction matching in `init_partial_compact_block` |
| 114-03 | Collision, duplicate, missing, lifecycle integration tests |
