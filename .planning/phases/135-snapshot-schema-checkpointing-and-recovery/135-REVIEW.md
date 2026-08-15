---
phase: 135-snapshot-schema-checkpointing-and-recovery
reviewed: 2026-08-15T20:55:00Z
depth: standard
files_reviewed: 18
files_reviewed_list:
  - packages/open-bitcoin-node/src/storage/mempool_snapshot.rs
  - packages/open-bitcoin-node/src/storage/mempool_snapshot/tests.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/representability.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/tests/terminal_generation.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/tests/representability.rs
  - packages/open-bitcoin-node/src/network/lifecycle_projection/recovery.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/representability.rs
  - packages/open-bitcoin-rpc/src/context/mempool_recovery.rs
  - scripts/check-phase135-snapshot-recovery.ts
  - scripts/check-phase135-snapshot-recovery/source.ts
  - scripts/check-phase135-snapshot-recovery/persisted-input.ts
  - scripts/check-phase135-snapshot-recovery/persisted-input-mutations.ts
  - scripts/check-phase135-snapshot-recovery.test.ts
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
---

# Phase 135: Code Review Report

**Reviewed:** 2026-08-15T20:55:00Z
**Depth:** standard
**Files Reviewed:** 18
**Status:** clean

## Summary

This standard review covered the Phase 135 gap-closure source from plans 12–14 (terminal-generation rejection, writer/reader representability, and production-use checker bypasses). Review judgment followed repo-local `AGENTS.md` / `AGENTS.bright-builds.md`, the empty concrete override table in `standards-overrides.md`, and `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/languages/rust.md`, and `standards/languages/typescript-javascript.md` (parse-at-boundary, illegal states, no `unwrap()`, `maybe_` option names, early returns, Bun/TypeScript checker shape).

All reviewed files meet quality standards. No issues found.

The prior advisory findings from 2026-08-10 are closed in the live tree:

- **CR-01** — `assert_mempool_snapshot_representable` and `encode_mempool_snapshot` share `persisted_mempool_input_limits()`. Capture proves that contract before `reserve_next`. Encode compares final bytes to `limits.max_encoded_bytes` (`268_435_456`), not a snapshot-derived `encoded_size_upper_bound`. One-over encode exact-aborts and leaves the prior snapshot in place.
- **WR-01** — Untrusted `u64::MAX` cannot become a current snapshot generation. Decode, `try_new_current`, live capture, and recovery install all refuse the terminal value. Crafted current-v2 bytes fail closed and leave the handle mutable.
- **WR-02 / WR-03 / WR-04** — The live checker now requires `for_persisted_input()` inside `recover_mempool_snapshot_with_loader`, the topology vertex and per-record edge use sites, and attribute-aware direct statements. The five named mutations each yield one intended diagnostic.

No production `unwrap()`, `expect()`, or `panic!` was introduced on the gap-closure paths. No accidental public-relay, readiness, repair, or MPDUR-complete claims appear in the reviewed source.

## Focus Area Checks

### Terminal-generation rejection

`CapturedMempoolGeneration::try_new` and `MempoolSnapshot::try_new_current` both reject `u64::MAX` as `StructuralCorruption`. The v2 `TryFrom` path constructs the generation only through `try_new` before `try_new_current`. Live capture copies `network.lifecycle_generation` through `try_new`. `PreparedRecoveryProjection::prepare` maps a captured generation with `from_raw` and then returns `InvalidPreparedRecovery` when the result is `LifecycleGeneration::MAX`, before projection rebuild.

The only `PreparedMempoolRecovery` constructor copies `snapshot.captured_generation()`, and a current snapshot cannot hold the terminal value. Legacy v1 still maps to `LifecycleGeneration::INITIAL`. Crafted-v2 decode and the RPC startup loader both fail closed; `expire_mempool` on the same handle returns `Ok`.

`CapturedMempoolGeneration::new` remains an infallible fixture constructor, as Plan 12 required. Production decode, capture, and `try_new_current` do not trust it for untrusted `u64`s.

### Writer/reader representability

`assert_mempool_snapshot_representable` loads `persisted_mempool_input_limits()` and fails `ResourceBoundExceeded` on missing limits, record count, unbroadcast count, per-transaction canonical bytes, checked aggregate transaction bytes, per-record input edges, and checked aggregate input edges. It does not mutate records, ages, or unbroadcast membership. `encode_mempool_snapshot` calls that predicate, then rejects `bytes.len() > limits.max_encoded_bytes`. `encoded_size_upper_bound` remains only as the format-formula proof inside limit construction.

`PrepareSnapshot` still applies live `max_live_entries`, then representability, then `reserve_next`. Fjall `save_mempool_snapshot` and `execute_prepared_mempool_snapshot_write` both encode before `put_bytes`; encode failure calls `abort_failed_write` and never replaces `SNAPSHOT_KEY`.

### Checker bypass coverage

`checkPersistedInputContract` now inspects extracted production bodies: startup `for_persisted_input()` with no `usize::MAX` / `MempoolSnapshotDecodeLimits::new(`, topology `if records.len() > limits.max_vertices` and `validate_parent_edges(record.record.transaction.inputs.len())`, encode `assert_mempool_snapshot_representable` plus `bytes.len() > limits.max_encoded_bytes` and no `encoded_size_upper_bound(total_transaction_bytes`, and capture representability. `directStatementIndex` ignores a same-line or immediately preceding `#[...]` attribute, including the `lineEnd > 0` walk that prevents the prior spin on a leading newline.

The five exact mutations (RPC five-`usize::MAX` constructor, vertex `if false`, `validate_parent_edges(0)`, `#[cfg(any())]` preflight, snapshot-derived encode ceiling) are present and marked `exact: true`. The checker remains a structural proof, not a Rust frontend; that residual is documented and is not a production defect.

### Unwrap / panic, parse-at-boundary, scope claims

Production files in scope propagate `Result` and use `let...else` / early returns. The public v2 DTO still stores `captured_generation: u64` and parses it into the domain at the decoder boundary. `decode_mempool_snapshot_with_limits` is `pub` with a caller-supplied limits type so the RPC test can decode crafted bytes; encode stays crate-private; production startup still builds limits with `MempoolSnapshotDecodeLimits::for_persisted_input()`. Reviewed sources do not promote MPDUR, parity completion, public relay, or readiness.

---

_Reviewed: 2026-08-15T20:55:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
