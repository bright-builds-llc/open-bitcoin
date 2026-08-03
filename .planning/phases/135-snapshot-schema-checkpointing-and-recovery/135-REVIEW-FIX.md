---
phase: 135
fixed_at: 2026-08-03T09:18:59Z
review_path: .planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-REVIEW.md
iteration: 3
findings_in_scope: 5
fixed: 5
skipped: 0
status: all_fixed
---

# Phase 135: Code Review Fix Report

**Fixed at:** 2026-08-03T09:18:59Z  
**Source review:** `.planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-REVIEW.md`  
**Iteration:** 3

**Summary:**

- Findings in scope: 5
- Fixed: 5
- Skipped: 0

## Fixed Issues

### CR-01: Escaped unknown JSON keys bypass narrow decode budgets before rejection

**Status:** fixed: requires human verification  
**Files modified:** `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/decode.rs`, `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/decode/key_preflight.rs`, `packages/open-bitcoin-node/src/storage/snapshot_codec/tests/mempool_limits.rs`, `scripts/check-phase135-snapshot-recovery.ts`, `scripts/check-phase135-snapshot-recovery.test.ts`  
**Commit:** 42fa3ea2  
**Applied fix:** Added a bounded raw JSON object-key preflight before Serde unescaping, with a repeated Unicode-escape regression proving an oversized unknown key fails under narrow budgets. The Phase 135 checker now guards the exact bound and preflight call.

### WR-01: Authoritative daemon startup bypasses legacy confirmation migration

**Status:** fixed: requires human verification  
**Files modified:** `packages/open-bitcoin-node/src/sync.rs`, `packages/open-bitcoin-rpc/src/context/network.rs`, `packages/open-bitcoin-rpc/src/context/tests.rs`, `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/daemon_sync.rs`, `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/inbound_runtime.rs`  
**Commit:** 42fa3ea2  
**Applied fix:** Routed both `DurableSyncRuntime` and the alternate store-backed RPC constructor through confirmation migration, propagated missing or corrupt migration inputs before publishing a network handle, and removed the error-to-empty-chainstate fallback. Daemon-path tests cover successful persisted migration and missing-block startup rejection.

### WR-02: Confirmation migration does not verify loaded block identity

**Status:** fixed: requires human verification  
**Files modified:** `packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs`, `packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/confirmation_migration.rs`  
**Commit:** 42fa3ea2  
**Applied fix:** Recomputed each decoded active-chain block header hash and compared it with the chain position key before counting transactions. A mismatch now returns typed chainstate corruption, with a regression that stores a different block under the expected active-chain key.

### WR-03: The generic snapshot constructor fabricates known-empty confirmation evidence

**Status:** fixed: requires human verification  
**Files modified:** `packages/open-bitcoin-chainstate/src/types.rs`, `packages/open-bitcoin-node/src/sync/tests/block_requests.rs`, `packages/open-bitcoin-node/src/sync/tests/reorg_reconciliation.rs`, `packages/open-bitcoin-rpc/src/context/tests.rs`  
**Commit:** 42fa3ea2  
**Applied fix:** Changed the generic `ChainstateSnapshot::new` constructor to preserve confirmation evidence as unknown. Authoritative engine/default paths continue supplying explicit known counts, while fixtures with known evidence were migrated and missing active-chain bodies now fail during runtime construction.

### IN-01: Mandatory Bright Builds file-length check fails for four source files

**Status:** fixed  
**Files modified:** `packages/open-bitcoin-chainstate/src/engine/tests/derives_contexts_from_chainstate_metadata.rs`, `packages/open-bitcoin-chainstate/src/engine/tests/derives_contexts_from_chainstate_metadata/difficulty.rs`, `packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence.rs`, `packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/confirmation_migration.rs`, `packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/reopen.rs`, `packages/open-bitcoin-rpc/src/context/tests.rs`, `packages/open-bitcoin-rpc/src/context/tests/construction.rs`, `scripts/check-phase135-snapshot-recovery.ts`, `scripts/check-phase135-snapshot-recovery/source.ts`, `docs/parity/source-breadcrumbs.json`, `docs/metrics/lines-of-code.md`  
**Commit:** 42fa3ea2  
**Applied fix:** Split the four oversized files into focused child modules without exceptions, registered every new Rust file in the parity breadcrumb corpus, and kept the checker corpus spanning both TypeScript modules. Bright Builds reports zero findings across 1,035 files.

## Verification

- Focused snapshot-key, confirmation-migration, daemon-startup, alternate-constructor, and generic-snapshot regressions passed.
- Phase 135 semantic checker: 50 passed, 0 failed; live checker passed.
- Bright Builds: 1,035 files scanned, 0 exceptions, 0 findings.
- Parity breadcrumbs: 756 Rust files verified.
- Mandatory Rust sequence passed: `cargo fmt --all`, Clippy with warnings denied, all-target/all-feature build, and all-feature tests.
- Final `bash scripts/verify.sh` passed in 6m34s, including coverage, benchmark smoke validation, Bazel smoke build/run, production file-length checks, and all repository policy gates.
- Normal repository pre-commit hooks passed for commit `42fa3ea2`.

## Skipped Issues

None.

***

_Fixed: 2026-08-03T09:18:59Z_  
_Fixer: the agent (gsd-code-fixer)_  
_Iteration: 3_
