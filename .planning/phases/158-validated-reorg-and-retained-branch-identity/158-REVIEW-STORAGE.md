---
phase: 158-validated-reorg-and-retained-branch-identity
reviewed: 2026-10-09T03:42:03Z
depth: standard
diff_base: 2f21ac2052208c7e7a084ed006d908c5ccb714aa
files_reviewed: 20
files_reviewed_list:
  - packages/open-bitcoin-chainstate/src/engine.rs
  - packages/open-bitcoin-chainstate/src/engine/stage.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/staged_connect_and_reorg.rs
  - packages/open-bitcoin-chainstate/src/filter_index/catch_up.rs
  - packages/open-bitcoin-chainstate/src/filter_index/catch_up/reorg.rs
  - packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests.rs
  - packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests/reorg.rs
  - packages/open-bitcoin-chainstate/src/lib.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership/proofs.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg/faults.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs/undo.rs
findings:
  critical: 0
  warning: 1
  info: 0
  total: 1
status: issues_found
---

# Phase 158: Core and Storage Code Review Report

**Reviewed:** 2026-10-09T03:42:03Z
**Depth:** standard, with necessary cross-file authority tracing
**Files Reviewed:** 20
**Status:** issues_found

## Summary

All 20 assigned source files were read, including the new untracked modules and actual new test assertions. One concrete consecutive-reorg liveness bug was found. No additional critical security, immutable-retention, projection-authority or failure-progress finding was identified within this scope.

Material guidance: root AGENTS.md and Repo-Local Guidance, AGENTS.bright-builds.md, placeholder-only standards-overrides.md, standards/index.md, architecture/code-shape/testing/verification/local-guidance/Rust pages, 158-CONTEXT, Plans 01–06 summaries and their threat entries. Both canonical active lesson inputs were read completely: 7,188 bytes / 2,397 conservative estimated tokens. No lesson archive was read. Source review was performed within the parent Phase 158 workflow; no source or Git state was modified.

## Warnings

### WR-158-01: Permit a second validated reorg after an unflushed shorter replacement

**Severity:** Warning (P2)
**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs:181-184`
**Issue:** The unconditional `previous.durable_height > old.0` rejection conflates the displaced durable coins endpoint with the current accepted endpoint. Phase 158 deliberately allows a shorter replacement while leaving coins at the taller displaced branch until an ordinary own flush. During that valid interval, every subsequent reorg is rejected by this predicate, even when its stage and all retained sources are genuine.

**Concrete failure condition:**

1. Start with the existing fully indexed durable A tip at height 23.
2. Accept the existing genuine shorter B replacement from common ancestor 10 through height 14, and optionally finish its ordinary index turns. Do not request an Always flush. The production shorter test already asserts that actual durable coins remain at A23.
3. Request a genuine return B14 → A23: disconnect B11–14 and reconnect the retained original A11–23. Preparation captures old accepted height 14 and the authentic prior append identity still carries durable height 23.
4. `prepare_basic_filter_reorg` returns `BASIC displaced endpoint behind progress` at the cited lines before preflight/preview/acceptance. A normal own coins flush is required to unblock a transition that the accepted-unflushed contract should permit.

**Evidence:** `chainstate.rs:396-414` always invokes `prepare_basic_index_reorg` for the genuine stage; `chainstate/filter_reorg.rs:109` invokes the storage preparation. `sync/tests/filter_index/reorg/branches.rs:8-87,113-119` proves the shorter/disconnect-only transition and preserved old coins, then explicitly flushes before cleanup. Its return-to-original test also flushes the first branch before returning, so it misses this interval. This is a static deterministic branch proof; no new runtime repro is claimed.

**Fix:** Keep the processed-frontier height check, but admit the higher durable height only when the current private replacement identity already binds `maybe_displaced_fence` to the exact `(previous.durable_height, previous.durable_hash)`. Preserve same-store proof, live accepted lineage, checked generation/revision and actual coins checks. Recompute the next displaced marker from the genuine next branch as the current code does. Add a concrete Fjall production test that returns from the shorter branch before any own coins flush, checks successful ordinary continuation and immutable A/B records, and subsequently earns the normal own fence. Include a negative control showing an unbound taller durable tuple still refuses. Do not force an index-specific flush to conceal the mismatch.

## Reviewed Authority and Failure Paths

- Stage fields and accepted receipt fields are private. Genuine staging is the only stage construction expression; absorbed installation mints the non-Clone receipt. Compatibility absorption delegates to the receipt-producing method. Pure replacement validates exact generation renewal, old identities, indexed/safe shared endpoints, lag and conservative protection.
- Preview suspension uses the store publication guard and binds the exact prepared transition. Ordinary proof/token issuance, already-owned append/legacy completion and flush confirmation refuse suspension. Raw append invalidation does not clear the suspension.
- Guarded rewind publishes checkpoint, lifecycle generation and complete reserved protection through one SyncAll batch, leaves physical suffix and immutable hash rows intact, and withholds achieved capability on ambiguous/error returns. Commit failure and injected AfterCommit poison the writer.
- RecoveredPrefix and ValidatedReorg remain distinct. Every requested existing forward recovered projection requires accepted positions, including same-hash replay and a hidden row following a missing row. Positions borrow the accepted manager range and compare height/hash/parent, store, generation, branch, revision and processed frontier. Immutable encoded equality and predecessor-header checks remain enforced.
- The coins metadata exception requires the exact private displaced tuple; own receipt confirmation still checks actual coins, source revisions and manager provenance. Raw metadata and foreign/generic proof controls do not earn promotion.
- Native body wire length and scanner counts are admitted before allocating decode. Native undo JSON length reserves a conservative element/container/copy envelope before serde, then compares complete historical undo equality. Shared immutable reuse verifies accepted height/hash/parent and the predecessor commitment.
- The actual new storage tests use genuine staging and recovered manager lineage; they exercise stale/foreign positions, immutable conflict, recovery replay, sparse hidden suffixes and publication faults. Existing codec-only/raw fixtures are explicitly supplementary. No fake accepted receipt or new authority factory was found.
- An explicit simplification pass found that the shared append implementation and existing publication/ownership primitives avoid duplicate authority paths. No separate refactor request is needed. Recorded finite source/resource refusals and existing history costs were treated as documented limits, not new defects.

## Verification and Limits

This review ran read-only diff/line/ignore checks and read the test bodies. `git diff --check` for the assigned source areas passed; none of the 20 files is Git-ignored, and no .claudeignore file or project skill directory was present. Cargo/Bazel/tests were deliberately not run because the parent verifier owns the shared target and required serialized verification. The 98-test and focused fault/protection results are predecessor summary evidence, not executions by this reviewer. Full native verification, lifecycle/source/security consolidation and final Git remain parent-owned.

Additional current Rust modules were cross-read only to trace capability producers, manager calls and the reachable failure condition; they are not included in the 20-file review coverage count. This report covers the exact content hashes below. A changed hash requires targeted re-review.

## Reviewed Source Fingerprints

Captured at 2026-10-09T03:42:03Z; SHA-256 of complete file bytes.

| File | SHA-256 |
| --- | --- |
| `packages/open-bitcoin-chainstate/src/engine.rs` | `ce642b9f3c4f266c09712550f66acc53d8038f11da1463cef65c79effa7b69a7` |
| `packages/open-bitcoin-chainstate/src/engine/stage.rs` | `cc04325a42f7f45ce8d68f5a39300d2af89398842232e3fec7586bcf529c18e8` |
| `packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/staged_connect_and_reorg.rs` | `b918d51b6c6a95857d4e1897ca6828a73529a0639bf7b163e2963fd968fc7518` |
| `packages/open-bitcoin-chainstate/src/filter_index/catch_up.rs` | `332b402ce454f35cc88eba2e0d9b11fa294a53b1d41a9a9f656ba9ea69cef503` |
| `packages/open-bitcoin-chainstate/src/filter_index/catch_up/reorg.rs` | `a37403ada7a4ffcdd0bceee5a4f3f388404211f924c97b780c52ad216da26abf` |
| `packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests.rs` | `8f9e7102c97e66474bcb48e647065c34b77431857167c7921e63195422d96492` |
| `packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests/reorg.rs` | `22d74c633756ba3d91cadaf2f878b7f2fa33a6bf44fde21c4c201de017e8fe73` |
| `packages/open-bitcoin-chainstate/src/lib.rs` | `5f8153f6b948ae2871ecd066ff091f8dd444b3a228eed7c5a00c6f3c0fc5738a` |
| `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` | `a3bad66bbf4d848fd9c9ac0d01a4c556586cb34ef66c3804506f8187cd906ee1` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters.rs` | `91a61729172f33581dc6b7ff652effc96faf80e4d367c24e4241f5ffcc5ed442` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs` | `83bbaa80175bd9cd05b7a4e9b638faa7e0be81e89723b7793b2041b17883c69e` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs` | `379442b8623ad8f517de892ceace6603c01267f9c8460fe0b22b25420b5329d9` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership/proofs.rs` | `1298113df10505036770b40ed560853ed7d94513801408e4390b0384fc7a8542` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs` | `1c98f0b8ad4c7738befecff4dbdb2b76b1ca0f732a606ead5bc70fc51da72f41` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs` | `67feeec274284f4a7bf6c412fe656afa39bccbd0e03cde92b793aefefad2e20a` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs` | `6ed895afa1d1ac0126019ef6dca79d97a2bc81b53031cd8f6010c2fb84813d07` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg.rs` | `625d2a2932a2a7fe470fcde30f63fe8010355ad661444111d2fe854a10d40b41` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg/faults.rs` | `f767df6bd156930b1c625c49916ae6e9f8e014ff99ab514a0f069fe1e2fa43b7` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs.rs` | `81c363d712cf6aff21b81f5cbc691acdfc520fd17368c02a2fd4d1e3be45db79` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs/undo.rs` | `923371e2e2207190afdd1fbab69e6ae9970d556ea44a4ca3d10b3d66da6e15e7` |

***

_Reviewer: gsd-code-reviewer (core/storage slice)_
