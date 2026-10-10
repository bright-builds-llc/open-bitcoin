---
phase: 159-authenticated-basic-filter-and-index-rpcs
plan: "04"
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 159-2026-10-09T16-11-49
generated_at: "2026-10-09T17:44:10Z"
---

# BASIC Query Work Measurements

Observed on the pinned Rust 1.94.1 debug lib-test build, 2026-10-09. The final `phase159-query-all` command discovered and passed 14 tests in 41.26 seconds. All Cargo invocations used pinned Bun and the cooperative timing wrapper. Times below are observations, not latency guarantees or production capacity claims.

## Measured Queries

| Case | Record reads | Lifecycle reads | Envelope bytes validated | Target bytes retained | SHA input copies | SHA padded bytes | Logical copies | Hex filter + header | Elapsed / authority hold µs |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Genuine continuous genesis | 1 | 1 | 174 | 4 | 228 | 512 | 232 | 72 | 42 / 41 |
| Genuine continuous height 20 | 2 | 1 | 348 | 4 | 360 | 832 | 364 | 72 | 57 / 57 |
| Genuine continuous height 400 | 2 | 1 | 348 | 4 | 360 | 832 | 364 | 72 | 55 / 55 |
| Genuine legal large-block singleton, height 16 | 2 | 1 | 351 | 7 | 363 | 832 | 370 | 78 | 59 / 58 |
| Retained stale height 2 after real reopen | 2 | 1 | 356 | 12 | 368 | 832 | 380 | 88 | 57 / 56 |
| Exact 32 MiB codec-capacity target + small parent | 2 | Not part of direct storage measurement | 33,554,776 | 33,554,432 | 33,554,788 | 33,555,264 | 67,109,220 | 67,108,928 | 4,452,297 / not measured |

The genuine fixtures reuse `TurnHistory`/`ValidatedHistory` and actual consensus staging/commit. Their seeded durable history is honestly legacy for the new provenance ledger; no fixture comment or body presence grants scripts-valid provenance. The stale case first genuinely accepts and processes height 2 ahead of durable coins height 1, drops every runtime/store handle, reopens at height 1, and retrieves the retained height-2 record.

The legal large-block fixture contains 99 additional legal 9,985-byte outputs. Those repeated scripts deduplicate, so its generated filter is seven bytes: it calibrates a legitimate large input block, not a maximum-size filter claim. The separate full-capacity fixture uses the production GCS codec to encode 13,421,770 mapped values, adjusting the final unary quotient to fill exactly 32 MiB. It is a codec-capacity storage fixture, not a claim that a consensus-valid Bitcoin block generated those elements.

These successful known-header queries perform zero provenance database reads, body reads, undo reads or filter generations. The record counters are observed through the existing store counter and asserted against the returned work. Summary performs one lifecycle point read and zero record/history reads. A legacy missing owner requires one additional bounded state point read. Known resolution can require one retained identity read when neither pending sealed identity nor a direct header exists; missing-row classification can perform one additional provenance point read. Live accepted batches are already bounded to 128 identities.

## Chosen Ceilings and Allocation Accounting

| Ceiling | Value | Derivation |
| --- | ---: | --- |
| `BASIC_FILTER_QUERY_MAX_RECORD_BYTES` | 33,554,602 | Codec `MAX_SIZE` 33,554,432 + immutable envelope 170 |
| `BASIC_FILTER_QUERY_MAX_READ_BYTES` | 67,109,204 | Two independently bounded immutable envelopes |
| `BASIC_FILTER_QUERY_MAX_HEX_BYTES` | 67,108,928 | Twice maximum filter bytes + 64 header hex characters |
| Private `MAX_LOGICAL_COPY_BYTES` | 100,663,648 | Three maximum filter lengths + 352 fixed hash-input bytes |
| Private `MAX_HASH_PADDED_BYTES` | 67,109,696 | Two maximum padded filter hash inputs + fixed SHA digest/header buffers |

Target and parent Fjall values are borrowed; owner/state and validation-history decoding also borrow exact envelopes instead of first copying arbitrary backend values. The BASIC stream validator allocates no mapped-value vector. SHA256 currently uses a padded input `Vec`, so both target and parent hash-input copies are accounted before invoking `parse_record`. Target accounting also includes `checkpoint_identity` repeating its header commitment hash. Only the target filter gets an owned response copy; the parent value drops before that copy.

The copy and padding fields are checked logical byte lengths derived from the current SHA implementation, not allocator instrumentation. `Vec` capacity growth, reallocations, allocator overhead, backend caching, string/key allocation, serialization peaks and RSS are not claimed by these figures. The byte ceilings bound logical input and response work; they do not promise low lock latency for a hostile maximum-size stored filter.

Exact envelope and hex limits, one-over limits, `usize::MAX` expansion overflow, exact two-record logical-copy/hash-padding admission and one additional charge are exercised. The full 32 MiB target executes full recovery and a successful point query at the codec boundary.

## Integrity and Error Controls

- Genesis uses one record; other valid requests fully parse only target and immediate parent. No serving path calls recovery walkers or reads the active projection.
- Full recovery covers every retained immutable row, including hidden records, using the existing linear scan with bounded work per row. Trusted immutable publication maintains its shared read capability.
- A raw clone writing/removing any BASIC key clears the capability before effects while holding the shared publication mutex. Queries cannot use an old epoch; failed recovery and poison fail closed.
- Tests reject corrupt target/header, corrupt parent hash, missing parent and an older ancestor changed by a raw clone. Genuine absence stays distinct from typed storage corruption.
- Mutation sensitivity: replacing the parent's `parse_record` with `parse_record_fields` made the corrupt-parent assertion fail; restoring full parsing returned GREEN.
- Disabled lifecycle is resolved before unknown block; known identity is resolved before target inspection. Pending sealed acceptance identity is recognized before its durable ledger row exists.

Full native verification, authenticated HTTP/RPC projection, actual paired-prune evidence, waiter lifecycle proof and final source/security/lifecycle review remain parent/dependent-plan gates.
