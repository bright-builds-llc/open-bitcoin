---
phase: 154-basic-generation-and-commitment-parity
plan: "01"
subsystem: consensus
tags: [rust, bip158, basic-filter, siphash, golomb-rice, commitments]
requires:
  - phase: 154-context
    provides: pinned BASIC selection, byte-order, historical-input and ancestry contracts
provides:
  - distinct raw FilterHash and FilterHeader identities
  - independently tested arbitrary-byte SipHash-2-4
  - bounded hash-free BASIC Golomb-Rice encoding
  - pure script-fact BASIC generation and explicit ancestry commitments
affects: [154-02, 154-03, compact-filter-indexing]
tech-stack:
  added: []
  patterns: [borrowed raw-script deduplication, resource preflight, explicit predecessor enum]
key-files:
  created:
    - packages/open-bitcoin-codec/src/block_filter.rs
    - packages/open-bitcoin-codec/src/block_filter/tests.rs
    - packages/open-bitcoin-consensus/src/block_filter.rs
    - packages/open-bitcoin-consensus/src/block_filter/tests.rs
  modified:
    - packages/open-bitcoin-primitives/src/hash.rs
    - packages/open-bitcoin-primitives/src/lib.rs
    - packages/open-bitcoin-consensus/src/crypto/siphash.rs
    - packages/open-bitcoin-consensus/src/crypto.rs
    - packages/open-bitcoin-codec/src/lib.rs
    - packages/open-bitcoin-consensus/src/lib.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Retain the specialized siphash_uint256 implementation and test byte-API equality.
  - Require a predecessor enum for contextual headers; historical adapters own missing-evidence refusal.
  - Keep sparse-vector hash composition separate from contextual ancestry checks.
  - Defer all commits and requirement activation to the root phase verification gate.
patterns-established:
  - BASIC count and encoded output each respect the codec MAX_SIZE bound.
  - Raw script identity determines N; mapped collisions remain zero-delta elements.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 154-2026-10-04T04-36-23
generated_at: 2026-10-04T05:12:00Z
duration: approximately 21min
completed: 2026-10-04
commits: []
finalization: deferred-until-root-phase-verification
---

# Phase 154 Plan 01: BASIC Generation and Commitment Parity Summary

**Pure BASIC generation now combines arbitrary-byte SipHash, bounded Rice bytes, distinct commitment types and explicit branch predecessor checks.**

## Accomplishments

- Added `FilterHash` and `FilterHeader` through the existing wrapper macro. Constructor, borrowed bytes, owned bytes, parsing and generic-hash conversions preserve raw order.
- Added `crypto::siphash_bytes` without changing the specialized `siphash_uint256` API or implementation. All 64 pinned length vectors and independent 64/255/256/257-byte expectations pass.
- Added fixed BASIC P=19/M=784931 encoding. It checks count, range, ordering and output size before allocation or unary writing; duplicates preserve zero deltas, count prefixes are canonical and padding is zero.
- Added `BasicFilter::from_script_facts`, raw-script deduplication, high-half multiplication mapping and SHA256d commitments. Output-first OP_RETURN exclusion differs deliberately from spent-script selection, matching Knots.
- Added required `Genesis`/`Previous` ancestry, height and parent correspondence refusals, a low-level sparse-vector composition function and explicit display-only byte reversal.
- Registered all four new Rust files with pinned Knots source breadcrumbs. No crate, dependency, I/O, index, runtime, RPC, peer or UI surface was added.

## Public Contracts

- `BasicFilter::from_script_facts(BlockHash, &[&[u8]], &[&[u8]]) -> Result<BasicFilter, BasicFilterEncodingError>` consumes low-level facts without claiming historical completeness.
- `block_hash()`, `encoded_bytes()` and `filter_hash()` expose the filter identity, canonical bytes and typed raw hash.
- `header(parent_hash: BlockHash, height: u32, predecessor: FilterHeaderPredecessor) -> Result<FilterHeader, FilterHeaderError>` requires explicit ancestry. The historical adapter must supply validated chain position and reject absent predecessor evidence before calling it.
- `compute_filter_header(FilterHash, FilterHeader)` composes raw commitments without a contextual ancestry claim.
- `filter_hash_display_hex` and `filter_header_display_hex` reverse only at formatting boundaries.
- Codec exports `encode_basic_filter_values`, `basic_filter_range`, constants and `BasicFilterEncodingError`; crypto exports `siphash_bytes` beside its unchanged fixed-width API.

## Verification Evidence

All ad-hoc Cargo operations used `scripts/command-timings.ts` and its cooperative build lock. No full verifier or Bazel build was run by this executor.

- `phase154-siphash-red`: failed with unresolved `siphash_bytes` imports before the implementation existed.
- `phase154-siphash`: passed six selected tests, including fixed-width/compact-block regression behavior.
- `phase154-rice-codec`: passed all 11 codec filter tests.
- `phase154-basic-core-red`: passed all 12 initial BASIC tests after its lock wait; this result is GREEN, despite the timing key's name.
- `phase154-core-crates`: passed primitives 23, codec 55 and consensus 170 unit tests (248 total), including existing crypto and compact-block suites.
- `phase154-core-clippy`: passed all targets/features for the three affected crates with warnings denied.
- `phase154-format-final`: passed workspace formatting.
- After adding deterministic mapped-vector allocation refusal coverage, `phase154-core-coverage-clean-report` passed primitives 23, codec 55 and consensus 171 unit tests (249 total).
- The fresh report `/tmp/open-bitcoin-154-core-coverage-clean.txt` has no uncovered lines in codec BASIC, consensus BASIC, SipHash or primitives hashes. The initial report merged stale pre-edit objects and falsely named SipHash blank lines; package-scoped coverage cleaning removed those entries.
- The three-crate-only report still names existing difficulty, witness and script paths not exercised by this scope. They were left unchanged; root owns the full pure-core coverage gate.
- Pure-core dependency/import checks, managed file-length checks, production panic-site checks and `git diff --check` passed.
- Breadcrumb write/check passed for 923 Rust files through an isolated temporary Git index. The real index remains unstaged.

Expected SipHash digests for 0–63 bytes come directly from pinned `src/test/hash_tests.cpp`. Longer expectations were computed with pinned `test_framework.crypto.siphash.siphash`, keys `0x0706050403020100`/`0x0f0e0d0c0b0a0908` and ascending bytes modulo 256. Genesis filter/header expectations come directly from the first pinned `blockfilters.json` row. The empty hash expectation was obtained independently with Python `hashlib` SHA256d of byte `00`.

## Intended Atomic Finalization

No commits or pushes were made, as required by the plan and outer wrapper. Changes remain unstaged. After root's lifecycle-valid full phase verification, the intended task units are:

1. `feat(154-01): add typed filter commitments and byte-slice SipHash` — primitives hash/export and consensus crypto implementation/export.
1. `feat(154-01): encode bounded BASIC mapped values` — codec module/tests/export and codec breadcrumb registration.
1. `feat(154-01): construct BASIC filters with explicit ancestry` — consensus module/tests/export and consensus breadcrumb registration.
1. Root-owned summary/state/roadmap/requirements finalization after the phase gate.

## Simplification Review

Retained fixed BASIC parameters, borrowed slices in a local `BTreeSet`, one mapped-value vector and one exact-size bit writer. No generic GCS stream framework, script parser, configurable filter abstraction or shared mutable state was introduced. The two private reservation helpers let allocation refusals be tested with unrepresentable capacities without allocating giant fixtures. Sparse-vector hash composition and contextual ancestry remain separate because they make materially different guarantees.

## Deviations from Plan

- Task 2 tests and implementation were installed together before the first observed run; no separate RED result was captured for that task.
- Task 3 tests were authored first, but its RED command waited behind an IDE Cargo check while implementation proceeded. It ultimately ran against the implementation, so no observed RED result is claimed for Task 3.
- The breadcrumb checker only sees tracked files. A temporary index was used to apply/check new-file breadcrumbs while preserving the wrapper's unstaged real-index contract.
- Commits, STATE, ROADMAP and requirement activation were deliberately deferred to root, exactly as specified by the execution rules.

## Issues Encountered

macOS delayed normal Cargo test binaries before the test harness. Polling and process sampling showed `_dyld_start`, then the binaries proceeded and passed. No command was terminated or code changed in response to elapsed estimates. Scoped coverage binaries ran normally. An IDE Cargo check caused the earlier artifact lock wait.

## Known Stubs and Threat Surface

No goal-blocking stubs were found. Empty script sets and zero predecessor headers are intentional protocol states. All new behavior is pure; no new file access, endpoint, authentication path or storage trust boundary was introduced. Resource preflight and typed ancestry implement T-154-01 through T-154-03. Public block-derived SipHash keys retain the accepted T-154-04 disposition.

## Next Plan Readiness

Plan 02 can consume the contracts above without dependencies or further API exploration. Plan 03 owns the comprehensive independent corpus and distinct-script mapped-collision proof. Full phase verification, requirement activation and commit/push remain pending at root.

## Self-Check: PASSED

All 11 planned implementation/export/manifest paths and this summary exist. Four new source/test breadcrumb blocks match their manifest entries. Task commit existence checks are intentionally inapplicable: commits are deferred, and no commit hash is claimed. Scoped tests, Clippy, coverage, architecture checks and diff review support the implementation claims; phase completion remains dependent on the root gate.

## Plan 02 Blocking Parity Fix Addendum

While exercising the required real coinbase-plus-two-transactions history fixture, Plan 02 exposed a pre-existing `block_merkle_root` mutation bug. The implementation compared pairs after odd-level padding, so the protocol-required duplicate last node incorrectly marked every odd transaction level mutated. Pinned `consensus/merkle.cpp::ComputeMerkleRoot` compares real adjacent pairs before adding padding.

- **Authorized deviation:** Rule 3, fix the pre-existing algorithm blocking the required validated history fixture rather than changing the fixture shape.
- **File changed:** `packages/open-bitcoin-consensus/src/crypto.rs` only, with inline behavior regressions.
- **RED:** `phase154-merkle-padding-red` failed the three-distinct-transaction regression before the production edit; the failure reported `distinct transaction count 3`.
- **Fix:** Move real-pair mutation detection before padding. Hash concatenation, duplicate padding and returned Merkle-root bytes remain unchanged.
- **GREEN:** `phase154-merkle-padding-green` passed all 17 selected Merkle tests, including three/five distinct transactions, explicit-padding root equality, real duplicate pairs and repeated real subtrees.
- **Verification:** Scoped Rust formatting and `git diff --check` passed. Root owns final full verification and post-fix coverage; the earlier 249-test coverage evidence predates this addendum.
- **Commits:** Still deferred to root. No STATE, ROADMAP, requirements, manifest or other source path was changed by this follow-up.
