---
phase: 155-recoverable-index-and-pre-prune-startup-protection
plan: "02"
subsystem: storage
tags: [basic-filters, fjall, recovery, atomic-publication, rust]
requires:
  - phase: 155-01
    provides: Bounded BASIC validation, immutable identities, fenced recovery and protection
provides:
  - Versioned additive immutable records and explicit checkpoint/projection envelopes
  - Serialized SyncAll publication of records, projection, fence and full protection map
  - Real reopen fault matrix and linear complete-record integrity scan
affects: [155-03, 155-04, compact-filter-index]
tech-stack:
  added: []
  patterns: [borrowed envelope parsing, direct-edge complete integrity proof, same-database atomic publication]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/filter_index.rs
    - packages/open-bitcoin-node/src/storage/filter_index/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/faults.rs
    - packages/open-bitcoin-chainstate/src/filter_index/tests/commitments.rs
  modified:
    - packages/open-bitcoin-node/src/storage.rs
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/prune.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/prune/records.rs
    - packages/open-bitcoin-chainstate/src/filter_index.rs
    - packages/open-bitcoin-chainstate/src/filter_index/tests.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Keep schema 2 and use additive versioned keys inside the existing BlockIndex keyspace.
  - Return unit from standalone commitment/edge proofs so they cannot construct unchecked ancestry identities.
  - Validate every immutable row and direct edge once, then every projection reference, without an index cache.
  - Share per-store publication serialization with full prune-lock map replacement and require runtime serialization of coins writers.
requirements-completed: [CFIX-02, CFIX-04]
requirements-addressed: [CFIX-02, CFIX-04, CFPR-03]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 155-2026-10-04T16-01-08
generated_at: 2026-10-04T17:37:28Z
commits: []
git_finalization: pending consolidated parent commit after phase verification and full native gate
duration: 43min
completed: 2026-10-04
---

# Phase 155 Plan 02: Durable BASIC Publication Summary

**Immutable BASIC envelopes and coins-fenced SyncAll publication preserve cursor/protection consistency through real Fjall close/reopen failures.**

## Performance

- First recorded test command: 2026-10-04T16:54:20.790Z; summary written 2026-10-04T17:37:28Z.
- Tasks: 3 complete; 14 owned source/test/parity paths plus this summary.
- Final node suite: 29 tests, including 8 codec and 21 concrete storage tests, passed in 7.83s test execution time.
- Core helper suite: all 25 matching chainstate library tests passed, including 2 new unit-returning proof tests.
- The 256-record/256-projection reopen corpus proves exactly 1,023 actual filter-key and prefix reads (`4N - 1`). Compiler, diagnostic artifact-lock waits and pre-Rust executable launch latency are separate from test execution time.

## Accomplishments

- Added fixed v1 record/state/projection envelopes with little-endian integers, raw hash bytes, canonical key correspondence, BASIC/type/version checks, bounded lengths, exact byte hashing, contextual commitments and explicit Empty/exhausted protection states. Frozen envelope tests cover field order and every truncation.
- Immutable writes require generated historical inputs and explicit typed predecessor identities. Full equality makes rewrites idempotent; self-consistent differing fields at an existing hash refuse. Record-only writes do not create or advance state, projection or protection.
- Atomic checkpoint batches include validated candidate rows, changed projection rows, explicit fence/state and the preserved full operator lock map. Candidates and changed projection rows are bounded to 128; aggregate record bytes are bounded to MAX_SIZE plus checked fixed overhead. Publication rereads recovered coins B and contiguous metadata and refuses a stale caller fence. Rewind hides suffix through the endpoint without deleting immutable/projection history.
- Complete integrity scans prove each own byte/hash/header commitment and each direct decreasing-height predecessor edge. Parent and projection references receive cheap bounded field parsing; every row independently receives full validation. This removes repeated full ancestor traversal from startup scans while preserving the strong typed identity constructor.
- Per-store faults cover before records, before checkpoint, before protection insertion and after actual successful SyncAll commit. Returned persistence/fault errors poison publication control shared by clones, require reopen, and are never credited as proven rollback.
- Real reopen tests independently inspect immutable rows, cursor/state/fence, projection visibility, reserved protection and operator locks. They cover conflicts, corrupt envelopes/state/projection/protection, missing predecessors, ahead rows, rewind, initialization atomicity and schema-1 migration/schema-2 reopen. The legacy migration fixture retains its exact genesis body for confirmation migration.

## Exact Downstream Interfaces

Types live in crate-internal `crate::storage::filter_index`:

```rust
StoredFilterRecord::generate(
    inputs: &BasicFilterInputs<'_>, position: &ChainPosition,
    maybe_predecessor: Option<&FilterRecordIdentity>,
) -> Result<StoredFilterRecord, StorageError>;
record.identity() -> FilterRecordIdentity;
record.encoded_bytes() -> &[u8];

StoredFilterState {
    maybe_endpoint: Option<(u32, BlockHash)>,
    fence_height: u32,
    fence_hash: BlockHash,
    protection: IndexInputProtection,
}
```

All store methods below are crate-internal methods on `FjallNodeStore`:

```rust
initialize_basic_filter_state(&VerifiedChainstateFence<'_>) -> Result<(), StorageError>;
load_basic_filter_record(BlockHash) -> Result<Option<StoredFilterRecord>, StorageError>;
maybe_basic_filter_state() -> Result<Option<StoredFilterState>, StorageError>;
maybe_basic_filter_checkpoint() -> Result<Option<FilterCheckpoint>, StorageError>;
maybe_active_basic_filter_record(u32) -> Result<Option<StoredFilterRecord>, StorageError>;
basic_filter_artifacts() -> Result<(bool, bool), StorageError>; // records, projection
basic_filter_projection(u32) -> Result<BlockHash, StorageError>;
validate_basic_filter_records() -> Result<(), StorageError>;
scan_basic_filter_checkpoint(
    FilterCheckpoint, IndexInputProtection, &VerifiedChainstateFence<'_>,
) -> Result<FilterRecoveryPlan, StorageError>;
verify_basic_filter_fence(&VerifiedChainstateFence<'_>) -> Result<(), StorageError>;
persist_basic_filter_records(&[StoredFilterRecord]) -> Result<(), StorageError>;
publish_basic_filter_checkpoint(
    &VerifiedChainstateFence<'_>, FilterCheckpoint, IndexInputProtection,
    &[StoredFilterRecord],
) -> Result<(), StorageError>;
```

`maybe_basic_filter_checkpoint` proves the entire saved contiguous projection, endpoint height and saved state protection. It does not itself establish current recovered authority or validate the reserved durable lock map; Plan 03 must load/validate those before pruning. `validate_basic_filter_records` also checks hidden projection references and all immutable fork/suffix rows. `scan_basic_filter_checkpoint` returns the pure Keep/Reconcile/Refuse decision without writing. Rows cannot initialize a cursor implicitly; initialization refuses partial artifacts or an existing reserved identity. Reconciliation may pass an empty candidate slice because its immutable rows already exist.

Test-only faults are re-exported as `crate::storage::fjall_store::filters::FilterPublicationFault::{BeforeRecords, BeforeCheckpoint, BeforeProtection, AfterCommit}`. Use `store.set_basic_filter_fault(point)` before a production adapter call; drop every owning handle before reopening. Raw codec constants/functions are crate-internal and retain the key spellings documented in the plan.

New pure helpers are exported from `open_bitcoin_core::chainstate::filter_index` (not newly re-exported at its crate root):

```rust
verify_filter_record_commitment(
    u32, BlockHash, FilterHash, FilterHeader, FilterHeader,
) -> Result<(), FilterIndexError>;
verify_filter_record_predecessor(
    u32, BlockHash, FilterHeader, Option<(u32, BlockHash, FilterHeader)>,
) -> Result<(), FilterIndexError>;
```

They return unit and do not mint `FilterRecordIdentity`; its existing constructor still requires a fully typed predecessor. Individual hash-addressed reads retain full ancestry proof; complete startup scans and saved projection scans avoid repeated ancestor walks.

## Verification and Fault Evidence

- Codec RED (`phase155-storage-codecs-red`): exit 101 for the deliberately absent codec module after tests were authored. Initial GREEN passed 3 codec tests; expanded codec behavior passes 8 tests in the final suite.
- `phase155-filter-publication`: 17 matching node tests passed, including the first concrete four-point fault matrix.
- `phase155-filter-reopen-faults-final`: 24 tests passed after correcting the migration fixture's missing retained body.
- `phase155-linear-filter-core-lib`: 25 chainstate library tests passed after the narrow pure proof refactor.
- `phase155-linear-filter-node`: 27 node tests passed, including exact linear read-count proof and additional post-record/initialization boundaries.
- `phase155-filter-final-confirmed`: final 29 node library tests passed; this includes accepted legacy empty H bytes and unrecovered two-head refusal. An earlier fixture mistakenly encoded CompactSize zero, which the existing coins decoder rejects; only the test fixture was corrected.
- `phase155-filter-clippy`: both affected crates, all targets/all features, `-D warnings`, passed. The subsequent fixture correction changes only the test's empty value representation.
- Parity breadcrumbs passed for 940 tracked Rust files. All 7 new Rust paths are registered and staged solely for tracked inventory; root-owned staged/unstaged changes were preserved.
- Pure dependency/import checks, production panic-site checks, scoped rustfmt check and diff whitespace review passed. Every touched source/test file is at most 628 lines; the shared store entrypoint is 624.

Six distinct real-reopen fault tests cover pre-record, pre-checkpoint, pre-protection, post-checkpoint commit, post-record commit and Empty initialization pre-protection refusal. Post-commit errors expose complete new durable state or ahead immutable rows with unchanged old authority; they never claim rollback. These are software boundary tests, not hardware power-loss simulation.

## Deviations from Plan

**[Rule 2 - Missing critical performance protection] Linear full-index validation.** The first adapter reused full ancestor reads for each row, creating quadratic work. Parent review required a root-cause correction before handoff. Two small pure unit-returning proofs and borrowed field parsing now permit complete direct-edge validation without a cache. The strong identity constructor's contract and error ordering are preserved. A new core test module keeps existing files below the local length threshold. Core 25/node 29 tests, exact 1,023-read corpus evidence and affected-crate Clippy verify the correction. Commit: pending consolidated parent finalization.

Execution rules intentionally defer commits, STATE/ROADMAP/REQUIREMENTS updates and requirement activation to the parent. No auth gate, new dependency/keyspace, global schema change or coins H/B write change occurred.

## Issues Encountered and Residual Risks

The accidentally broad core test invocation launched a zero-match parity integration executable that remained before Rust startup. A captured sample at `.local/open-bitcoin-dev/phase155/parity-launch-sample.txt` showed only `_dyld_start`, 96 KiB footprint and no test frames. After liveness capture, only that owned irrelevant child was terminated; the required scoped library run passed. Host launch-assessment involvement is an inference, not established policy evidence. No execution security metadata was changed. Parent full native verification must still run the required integration suites and Bazel.

The publication mutex serializes filter writes and full lock-map replacements across store clones. The existing runtime owner must also serialize coins/metadata/prune operations; read/compare is not CAS against arbitrary competing raw writers. Hash-specific reads may retain ancestry hashes; the complete integrity scan retains only one row and its referenced row's bytes.

Plan 03 must remove the temporary narrow `dead_code` allowances on `validate_basic_filter_records` in `filters.rs` and `publish_basic_filter_checkpoint` in `publication.rs` when it wires their production consumer. Four narrowly documented future-scope allowances remain on generated-record construction, explicit initialization, record-only persistence and the active-record serving query; activation/serving are excluded from this phase. No blanket module or coverage exclusion was added.

## Simplification and Threat Review

The simplification pass retained one specialized codec and one same-database publication boundary, with borrowed scans and reused commitments/prune types. No full filter cache, fallback memory store, second deletion manager, silent error or placeholder was introduced.

T-155-05: cheap envelope/BASIC bounds precede byte hashing, with exhaustive truncation/type/key tests and a linear full scan. T-155-06: reread B/metadata authority, immutable equality, complete saved projection proof and atomic full state/lock map batches. T-155-07: explicit poisoned publication errors and genuine post-commit/reopen evidence. T-155-08: crate-internal writes and shared publication serialization; no public activation or operator bypass surface added. T-155-09: bounded category diagnostics without raw filter/script payloads. Self-consistent commitments remain corruption evidence rather than authentication of historical provenance; new generated records require Phase 154 historical inputs.

No new unmodeled endpoint, authentication path, schema trust boundary or filesystem pattern was introduced. Stub scan found no placeholder or unwired path required by Plan 02's goal. Public activation is deliberately outside scope.

## Next Plan Readiness

Plan 03 can now consume the concrete reads, complete scan, pure reconciliation and atomic checkpoint publication inside production initialize before prune resume. This summary does not claim startup protection complete; requirement IDs remain Pending until the lifecycle-valid phase and full native gate pass.

## Self-Check: PASSED

All 14 owned source/test/parity paths and this summary exist; final focused tests and lint passed. No commits were created, so commit verification is intentionally inapplicable to the authorized consolidated-parent workflow. STATE, ROADMAP and REQUIREMENTS were not edited by this executor.
