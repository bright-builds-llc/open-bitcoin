---
phase: 155-recoverable-index-and-pre-prune-startup-protection
plan: "01"
subsystem: chainstate
tags: [basic-filters, codec, recovery, prune-protection, rust]
requires:
  - phase: 154-basic-generation-and-commitment-parity
    provides: BASIC encoding, typed filter commitments and historical input generation
provides:
  - Streaming bounded canonical BASIC byte validation
  - Immutable record identities and coins-plus-metadata recovery authority
  - Streamed conservative checkpoint reconciliation and direct input protection
affects: [155-02, 155-03, 155-04, compact-filter-index]
tech-stack:
  added: []
  patterns: [borrowed durable ancestry fence, per-record pure recovery reducer]
key-files:
  created:
    - packages/open-bitcoin-codec/src/block_filter/validation.rs
    - packages/open-bitcoin-codec/src/block_filter/validation/tests.rs
    - packages/open-bitcoin-chainstate/src/filter_index.rs
    - packages/open-bitcoin-chainstate/src/filter_index/recovery.rs
    - packages/open-bitcoin-chainstate/src/filter_index/tests.rs
  modified:
    - packages/open-bitcoin-codec/src/block_filter.rs
    - packages/open-bitcoin-chainstate/src/lib.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Borrow verified durable active metadata rather than creating a second owned chainstate cache.
  - Validate saved protection before rewind and retain stronger saved protection.
  - Poison a refused recovery reducer step so finish cannot accidentally claim progress.
patterns-established:
  - Immutable record commitments are separate from saved cursor assertions and recovered authority.
  - Direct earliest-input intent checks cover genesis and height one independently of buffered locks.
requirements-completed: []
requirements-addressed: [CFIX-02, CFIX-04, CFPR-03]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 155-2026-10-04T16-01-08
generated_at: 2026-10-04T16:51:40Z
commits: []
git_finalization: pending consolidated parent commit after phase verification and full native gate
duration: 17min
completed: 2026-10-04
---

# Phase 155 Plan 01: Pure BASIC Integrity and Recovery Summary

**Bounded canonical BASIC validation and coins-fenced recovery decisions protect required inputs without trusting numeric progress or discarding immutable rows.**

## Performance

- First recorded test command: 2026-10-04T16:34:33.261Z.
- Tasks: 3 implementation tasks complete.
- Files created/modified: 8 source, test and parity files, plus this summary.
- Test execution included normal Cargo diagnostic lock waits and macOS test process launch latency; sessions were polled and allowed to complete.

## Accomplishments

- Validator accepts canonical empty filters, encoder round trips, mapped collisions and byte-boundary endings. It refuses invalid CompactSize, byte/count limits, insufficient capacity, truncated unary/remainder streams, checked delta overflow, out-of-range values, nonzero padding and trailing bytes. No mapped-value vector, hash operation, dependency or I/O was added.
- Immutable identities check genesis, parent/height/predecessor correspondence and composed filter headers. A borrowed fence checks recomputed block hashes, unique identities, genesis/contiguous ancestry and recovered coins B equality with the durable metadata tip.
- Recovery distinguishes complete absence from partial saved state, proves the entire saved projection, keeps a valid prefix, and reconciles ahead/wrong-fork checkpoints only to the recovered common indexed prefix. Refusals do not request immutable deletion or claim progress.
- Explicit terminal height exhaustion avoids wraparound. Reserved lock ranges end at `u32::MAX - PRUNE_LOCK_BUFFER`, while direct intent checks protect heights 0, 1 and upper heights.

## Exact Downstream Interfaces

Codec API lives under `open_bitcoin_codec::block_filter::validation`:

```rust
pub fn validate_basic_filter_encoding(bytes: &[u8])
    -> Result<(), BasicFilterValidationError>;
```

The following types and constant are exported from both `chainstate::filter_index` and the crate root:

```rust
pub const BASIC_INDEX_PRUNE_LOCK: &str = "__open_bitcoin_basic_index";

FilterRecordIdentity::new(
    height: u32,
    block_hash: BlockHash,
    parent_hash: BlockHash,
    filter_hash: FilterHash,
    filter_header: FilterHeader,
    previous_header: FilterHeader,
    maybe_predecessor: Option<&FilterRecordIdentity>,
) -> Result<FilterRecordIdentity, FilterIndexError>;

IndexPrefix::{Empty, Committed(FilterRecordIdentity)};
FilterCheckpoint::new(prefix: IndexPrefix) -> FilterCheckpoint;
VerifiedChainstateFence::new(
    maybe_coins_best_block: Option<BlockHash>,
    maybe_positions: Option<&[ChainPosition]>,
) -> Result<VerifiedChainstateFence<'_>, FilterIndexError>;

IndexInputProtection::{FromHeight(u32), HeightSpaceExhausted};
FilterRecoveryScan::new(
    saved: FilterCheckpoint,
    maybe_protection: Option<IndexInputProtection>,
    fence: &VerifiedChainstateFence<'_>,
) -> Result<FilterRecoveryScan<'_, '_>, FilterIndexError>;
scan.push(record: FilterRecordIdentity, projected_hash: BlockHash)
    -> Result<(), FilterIndexError>;
scan.finish() -> FilterRecoveryPlan;
```

Identity accessors return `height`, `block_hash`, `parent_hash`, `filter_hash`, `filter_header` and `previous_header`. `FilterCheckpoint::prefix()` returns the saved assertion; `input_protection()` derives required input. `VerifiedChainstateFence::tip()` and `maybe_position(height)` borrow already proven positions.

`IndexInputProtection` provides `from_saved_lock(&PruneLockInfo)`, `maybe_prune_lock()`, `covers(required)` and `check_prune_intent(height)`. At terminal exhaustion no ordinary lock is required; the adapter must represent terminal state explicitly. High required starts render as stronger bounded ordinary lock ranges.

`FilterRecoveryPlan` has `LegacyAbsent`, `Keep { checkpoint, protection }`, `Reconcile { checkpoint, protection }` and `Refuse(FilterIndexError)`. Its `for_absent_state(has_records, has_projection, maybe_protection)` only permits legacy startup when every index artifact is absent. A scan borrows one fixed fence for its lifetime, consumes projection records in height order from genesis through the saved endpoint, and retains refusal after a failed step.

The byte-owning node record parser must additionally prove encoded-byte hash equality and supply a verified immutable predecessor. Checkpoints are saved assertions; only completed recovery establishes their compatibility with recovered authority. Record-only rows cannot advance a scan beyond its saved endpoint.

## Verification and RED/GREEN Evidence

- Codec RED: timing key `phase155-basic-validation-red` exited 101 with missing validator/error/helper symbols after the tests were authored.
- Combined contract/recovery RED: timing key `phase155-filter-contracts-red` exited 101 with unresolved `FilterRecoveryPlan` and `FilterRecoveryScan` imports while recovery tests existed and its implementation was absent.
- Initial codec GREEN: timing key `phase155-basic-validation` passed 22 matching tests, including 11 new validator tests.
- Initial contract GREEN: `phase155-filter-contracts` passed 8 matching tests.
- Expanded recovery GREEN: `phase155-recovery-policy` passed 22 matching tests; the subsequently added endpoint-corruption test passed in the complete scoped coverage run.
- Final scoped coverage: `phase155-core-coverage` ran `cargo llvm-cov --manifest-path packages/Cargo.toml -p open-bitcoin-codec -p open-bitcoin-chainstate --lib --json --output-path .local/open-bitcoin-dev/phase155/core-coverage.json`. All 302 unit tests passed: 235 chainstate and 67 codec. This includes 23 new recovery tests and 12 new validator tests.
- New production line coverage: `filter_index.rs` 146/146, `filter_index/recovery.rs` 107/107, and codec `validation.rs` 71/71, all 100%. No coverage exclusions were added.
- `phase155-core-clippy`: affected crates, all targets/all features, `-D warnings` passed.
- Scoped `rustfmt --check --edition 2024` passed; `git diff --check` passed.
- Parity breadcrumbs passed for 933 tracked Rust files. Pure dependency/import and native panic-site checks passed.

All Cargo commands used `scripts/command-timings.ts`; active verification completed without interrupting diagnostic builds or altering execution security metadata. New Rust paths were staged solely for tracked-file breadcrumb inventory. No commits were created.

## Simplification and Threat Review

The explicit simplification pass retained one streaming bit reader, one small projection reducer, immutable Copy identities and borrowed active metadata. Existing commitment hashing and prune types are reused. Bounded CompactSize counts prove range/minimum-bit products cannot overflow; checked delta recombination handles hostile arithmetic. No second owned chainstate cache, generic storage framework or automatic repair path was added.

T-155-01 is mitigated by byte/count/minimum-bit bounds, input-bit-bounded unary work, checked deltas and malformed tests. T-155-02 is mitigated by coins/metadata ancestry proof and full saved projection validation. T-155-03 is mitigated by explicit exhaustion, bounded ordinary ranges and direct 0/1/max intent tests. T-155-04 remains the plan's accepted limitation: self-consistent hashes detect corruption but do not authenticate historical provenance; later adapters must consume Phase 154 validated inputs.

No new endpoint, authentication path, file access or external trust surface was introduced in these pure modules. Stub scan found no placeholders or unwired production data paths.

## Deviations from Plan

No scope deviations. The explicit plan execution rules defer per-task/TDD and metadata commits, state changes and requirement activation to the parent. The two contract/recovery tasks used a shared RED compilation boundary while public interfaces evolved; final behavior and production coverage evidence covers both tasks.

Material guidance: local AGENTS and Bright Builds sidecar, placeholder overrides, architecture/code-shape/testing/verification/Rust standards, fully loaded global/repository active lessons, phase context/research and pinned Knots filter/index/prune sources.

## Next Plan Readiness and Residual Risks

Plans 02/03 can consume the exported contracts. Real Fjall atomic publication, immutable envelope/hash parsing, persisted partial-state detection, projection suffix handling and the production initialize-before-prune consumer remain their required work. Pure tests do not claim durable reopen or hardware fault evidence. All three requirement IDs remain Pending until lifecycle-valid phase verification and the full native gate pass.

Git finalization is pending the consolidated parent commit. STATE, ROADMAP and REQUIREMENTS were not edited by this executor. No external setup or authentication gate was required.

## Self-Check: PASSED

All eight owned source/test/parity paths exist and focused verification passed. Commit verification is intentionally inapplicable: the authorized no-commit workflow recorded `commits: []` and preserves parent finalization.
