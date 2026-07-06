# Phase 115: Missing Transaction Round Trip, Fallback, and Validation Handoff - Research

**Researched:** 2026-07-06
**Domain:** BIP152 missing-transaction round trip, blocktxn handling, validation handoff, fallback, cleanup
**Confidence:** HIGH

## Summary

Phase 115 ports Knots compact-block download completion: schedule `getblocktxn` for missing indexes on in-flight `PartialCompactBlock` state, validate and apply `blocktxn` responses, `FillBlock` into a full `Block`, emit `PeerAction::ReceivedBlock`, and fall back to full-block fetch or suppression on failure. Volatile per-peer in-flight maps clear on disconnect, timeout, reorg, restart, and block connect without chainstate mutation.

**Primary recommendation:** Add `compact_download` pure policy beside `compact_reconstruction`, extend `PartialCompactBlock` with `apply_block_transactions` and `fill_block`, wire peer message handlers to typed actions.

## Knots Anchors

- `PartiallyDownloadedBlock::ProcessTxns` and `FillBlock` in `blockencodings.cpp`
- Compact block download scheduling and fallback in `net_processing.cpp`
- `p2p_compactblocks.py` functional coverage

## Implementation Split

| Plan | Focus |
|------|-------|
| 115-01 | In-flight state, differential index request builder, scheduler eligibility |
| 115-02 | blocktxn validation/application and misbehavior outcomes |
| 115-03 | fill_block, ReceivedBlock handoff, full-block fallback |
| 115-04 | Cleanup matrix tests and lifecycle integration |
