---
phase: 159-authenticated-basic-filter-and-index-rpcs
plan: "04"
subsystem: node-query
tags: [basic-filters, fjall, bounded-reads, integrity, provenance]
requires:
  - phase: 159-03
    provides: Genuine accepted validation identities and live pending provenance
provides:
  - Shared recovered-integrity BASIC target/parent queries with checked copy and response accounting
  - Default-disabled generic query backend and truthful processed-progress summaries
  - Opaque captured accepted frontier for Plan05 operational readiness
affects: [159-05, 159-06, 159-07, 159-08]
tech-stack:
  added: []
  patterns: [borrowed publication-guarded reader, typed absence and storage errors, captured branch identity]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/query.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/query/tests.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/query.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/query/tests.rs
    - .planning/phases/159-authenticated-basic-filter-and-index-rpcs/159-04-QUERY-MEASUREMENTS.md
  modified:
    - packages/open-bitcoin-node/src/lib.rs
    - packages/open-bitcoin-node/src/chainstate.rs
    - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
    - packages/open-bitcoin-node/src/chainstate/validation_history.rs
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/payload_usage.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/validation_history.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs
key-decisions:
  - Reuse complete linear all-record recovery as a shared read capability; raw BASIC writes invalidate before mutation.
  - Derive query ceilings from existing 32 MiB codec capacity, admitting the exact maximum rather than imposing a smaller arbitrary cap.
  - Count SHA256 temporary input copies and padding before both parses; logical bounds are not allocator or RSS claims.
  - Capture accepted height/hash with authority incarnation, generation and branch; height is not a globally monotonic sequence.
requirements-addressed: [CFRP-01, CFRP-02]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 159-2026-10-09T16-11-49
generated_at: "2026-10-09T17:44:10Z"
duration: not precisely recorded; final checks completed at 17:44 UTC
completed: 2026-10-09
---

# Phase 159 Plan 04: Bounded BASIC Queries Summary

**BASIC queries now fully validate at most a target and immediate parent under shared recovered integrity, while summaries report the initial-sync latch and processed height.**

## Accomplishments

- Completed 3/3 implementation tasks, pending root consolidated commit and phase verification. Four new Rust modules and fourteen narrow existing-source edits implement one bounded read concern; two planning artifacts record the evidence.
- `ChainstateStore::maybe_basic_filter_query_store` is an object-safe borrowed accessor with a disabled default; Fjall returns its existing configured store. No independent DB owner, downcast, full-history cache, dependency or activation default was added.
- One network authority and publication snapshot resolves lifecycle, sealed pending/header/retained identity, typed provenance and immutable data. Known identity precedes row inspection; returned identity also matches the known height/parent. Found wins during initial catch-up and retained stale lookup; unknown, absence and errors remain distinct.
- The recovered capability covers all immutable rows. Raw BASIC put/remove clears it across clones before effects; failures/poison fail closed. Startup adds only another linear bounded-per-row scan, leaving recovery walkers unchanged and off request paths.
- Summaries use `initially_synchronized()` and processed endpoint height with zero fallback. Tests prove later accepted lag preserves the latch, processed progress can lead safe durability, reopen reconciles conservatively, and initial missing work does not wait.

## Verification Evidence

| Check | Result |
| --- | --- |
| Initial `phase159-query-storage-red` | Expected absent point-query API compile failure |
| Parent validation mutation RED | One discovered regression test fails when full parent parsing is replaced with fields-only parsing; restored afterward |
| Final `phase159-query-all` | 14 passed, 0 failed, 0 ignored; 41.26s test execution, positive discovery |
| `phase159-query-measure` | 3 passed, including exact 32 MiB codec-capacity admission and overflow/one-over controls |
| `phase159-query-clippy`, node `--lib --tests -- -D warnings` | Passed, no suppressions added |
| Scoped Rust 1.94.1 rustfmt check, diff check | Passed |
| Bright Builds managed checks | Zero findings; new files individually measured below limits |

The initial 401-row storage fixture exceeded the pre-existing 128-candidate publication ceiling; setup now publishes bounded chunks. Two test-module path/visibility mistakes were fixed before behavioral verification. Neither changed production policy. Cargo/Bazel target work remained serialized through the parent and timing wrapper. No full `verify.sh`, staging, commits, pushes, hook bypass or root STATE/ROADMAP/REQUIREMENTS/todo/config changes were performed here.

## Measured Limits

See [QUERY-MEASUREMENTS](./159-04-QUERY-MEASUREMENTS.md) for exact fixtures, counters, allocation derivation and observed timings. Record limits are 33,554,602 bytes individually and 67,109,204 combined. Hex filter plus header is limited to 67,108,928 bytes. Checked internal logical-copy/padded-hash ceilings are 100,663,648 and 67,109,696 bytes. Genesis/20/400/stale use 1/2/2/2 record reads; summaries use no record/history reads. Known-header successes add one lifecycle read and zero provenance/body/undo/generation work.

The existing genuine large-block fixture produces a seven-byte filter because repeated scripts deduplicate. The separate 32 MiB codec-capacity fixture proves full accepted storage capacity; it does not claim consensus-generated maximum filter size. SHA padding/copy counters describe logical byte lengths, not allocator capacities, RSS or serialization peaks. Observed debug timing is not a latency guarantee.

## API Handoff

- `ManagedNetworkHandle<S,V>::basic_filter_query(hash) -> Result<BasicFilterQuery, BasicFilterQueryError>` works under existing generic bounds. States are `Disabled`, `UnknownBlock`, `Missing { provenance: BasicBlockValidationProvenance, initially_synchronized }`, `Found(BasicFilterRecordView)`, and `Pending(BasicFilterReadFrontier)`.
- `BasicFilterQueryError::{Authority, Storage}` retains typed backend/corruption failure. Plan06 must project fixed redacted RPC errors instead of displaying internal strings.
- `BasicFilterRecordView` owns exact bytes with one checked `FilterRecordIdentity`; accessors expose identity, encoded bytes, raw filter header and structural work. Plan06 reverses header bytes for uint256 display order; filter bytes retain wire order.
- `maybe_basic_index_summary() -> Result<Option<BasicIndexSummary>, BasicFilterQueryError>` returns None for disabled backends/lifecycle; summary fields are `synced` and `best_block_height`.
- `BasicFilterReadFrontier` has private incarnation/generation/branch/accepted-height/hash fields and read-only identity accessors, with no public constructor, Future or readiness success. It captures genuine unflushed accepted work only after initial synchronization. Plan05 must preserve this exact target, validate its ancestry/branch and invalidate authority/reorg changes without chasing later accepted targets. Accepted height is a captured position, not a global sequence.

## Deviations from Plan

Parent-authorized narrow correctness/test glue beyond the planned paths:

1. `storage/fjall_store/payload_usage.rs` serializes raw BASIC put/remove and invalidates shared integrity before effects.
2. `storage/fjall_store/filters/ownership.rs` installs read authority when exclusive complete recovery installs the append proof.
3. `storage/fjall_store/filters/tests.rs` and `sync/tests/filter_index/catch_up.rs` include the planned test files under their existing fixture owners, avoiding duplicate discovery, copied generators or public fixture constructors.
4. `chainstate/validation_history.rs` exposes only a read-only sealed pending identity, recognizing accepted work before durable/header publication.
5. `network.rs` reexports the new query contracts; the planned `runtime_authority.rs` remains unchanged after removing redundant reexports. Existing root files stay at or below 628 lines.
6. `storage/fjall_store/validation_history.rs` borrows exact record bytes for decoding instead of copying potentially oversized persisted values before the 70-byte check. Query owner/state reads use the same bounded borrowed approach.

The parent explicitly approved captured height/hash in place of calling height a global accepted sequence. No architectural expansion, adjacent lifecycle/coins refactor or crypto rewrite was introduced. The simplification pass removed redundant export hops and an unnecessary flag setter, reused existing fixture owners, and shared the production query helper with timing observation.

## Task Commits

1. Recovered-integrity target/parent storage reader — pending root consolidated commit.
2. Generic same-authority query/summary/frontier — pending root consolidated commit.
3. Measurements and checked numeric ceilings — pending root consolidated commit.

No authentication gate occurred. No unintentional stubs or unplanned threat surface was found. The opaque Pending carrier is the explicitly staged Plan04 contract; operational readiness belongs to Plan05. Authentication/HTTP, actual paired-prune RPC proof, independent source/security review, full native verification, lifecycle validation and requirement completion remain dependent/root gates. No requirement is marked complete here.

## Self-Check: PASSED

All four new Rust source/test files and the measurement report exist. The 14 focused behaviors and strict scoped lint pass; the weakened-parent mutation demonstrably fails. No plan/task commit exists because root-owned consolidated finalization is explicitly deferred.
