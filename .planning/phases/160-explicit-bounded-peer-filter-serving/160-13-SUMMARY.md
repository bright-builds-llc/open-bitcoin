---
phase: 160-explicit-bounded-peer-filter-serving
plan: "13"
subsystem: storage
tags: [basic-filters, publication, integrity-epoch, affine-authority]
requires:
  - phase: 160-06
    provides: Guarded budgeted immutable peer reads
provides:
  - Opaque shared-origin-bound monotonic read-integrity captures
  - Checked raw/recovery/disable invalidation and memory-only freshness validation
affects: [160-07, 160-08, 160-09, 160-10]
tech-stack:
  added: []
  patterns: [opaque-origin-bound-token, checked-monotonic-invalidation]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication/epoch.rs
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/query.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/query/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/lifecycle.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/payload_usage.rs
key-decisions:
  - Read freshness uses an opaque epoch plus originating shared publication identity; persisted generation alone cannot reject same-generation ABA.
  - Trusted append preserves the epoch; raw put/delete, both recovery edges and pre-write disable invalidate it.
  - Epoch overflow poisons the existing publication authority permanently rather than wrapping.
requirements-addressed: [CFNET-06]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: "2026-10-10T20:24:15Z"
duration: approximately 15min
completed: 2026-10-10
---

# Phase 160 Plan 13: Monotonic Publication Freshness Summary

**Opaque shared-origin-bound read epochs permanently reject same-generation integrity ABA, while trusted immutable append preserves valid captures.**

## Accomplishments and Acceptance

- [x] The old boolean/generation contract was reproduced as a behavioral RED: same-generation recovery returned token0 both before and after recovery.
- [x] `BasicFilterReadEpoch` has private fields and no raw constructor, Default or deserialization. Its shared publication Arc prevents foreign-store or deallocated-origin identity reuse. It represents read identity and grants no write authority.
- [x] Both recovery invalidation sites advance the epoch; a successful same-generation recovery is tested to advance it exactly twice. Raw filter put/delete and disable advance before potentially fallible mutation.
- [x] Every clone sees invalidation. Disabled and untrusted readers refuse captures. BeforeDisable failure preserves the existing typed poisoned-publication error and denies prior authority.
- [x] Overflow refuses before raw data mutation, leaves the row unchanged, poisons publication, and rejects further capture, validation and recovery.
- [x] Trusted append preserves the capture and the exact point-query bytes. Memory-only validation performs zero backend reads across 100 checks.
- [x] Peer capture charges one additional MetadataReads unit through the established budget; OWNER plus epoch capture consumes two units. A one-unit limit refuses the capture. Existing Plan06 persisted-read measurements remain unchanged because they do not capture transport epochs.

## Verification

All Cargo commands used pinned Bun1.3.9 and `scripts/command-timings.ts` with the checkout build lock.

| Check | Actual result |
| -- | -- |
| Behavioral RED, `phase160_query_epoch` | One executed test failed with identical token0 before and after same-generation recovery. |
| Final Task2 command, key `phase160-query-epoch`, filter `phase160_query_epoch` | Seven executed, seven passed, zero ignored. |
| Final Task1 command, key `phase160-query-epoch-contract`, same filter | Seven executed, seven passed, zero ignored. Already-started duplicate run completed after an LSP artifact-lock wait. |
| Existing point-query regression, `filters::tests::query -- --skip phase160_query_epoch --nocapture` | Six executed, six passed, including 32MiB codec capacity, clone invalidation, target/parent corruption, absence and exact bounds. |
| Scoped rustfmt check on seven owned Rust files | Passed. |
| Managed `all`, final whitespace/diff review | Zero findings; `git diff --check` passed. |
| Breadcrumb registration | Parent registered the new child module; parent's check passed 1,082 Rust paths. |

The BeforeDisable fault test initially expected `Ok(false)`; the actual pre-existing fault contract poisons publication and returns a typed BackendFailure. The final assertion verifies that stronger fail-closed result. A test helper name typo was corrected before the final passing runs.

Background LSP Cargo and unrelated host builds delayed compilation. Sessions were polled and liveness was recorded; no command was terminated because of its estimate. Two warnings belong to already present, paused Plan07 carrier drafts awaiting their planned preparation caller; no epoch warning or failure remains. Scoped Clippy will run after Plan07 completes those interfaces, and the parent owns final native verification.

## Plan07/08 Interface Handoff

- Guarded `reader.capture_basic_peer_epoch(&mut budget)` returns the opaque token and charges metadata before capture; use it in the existing complete peer-query guard.
- `store.basic_peer_epoch_is_current(&token)` checks the same originating publication control, trusted state and exact epoch through memory only. It performs no backend I/O and returns typed storage errors when publication is poisoned.
- Clone the passive read token when binding the request identity; its fields and origin remain private. It is independent of accepted-tip append revision, mempool generation and persisted schema.
- Receipt carriers and socket authority remain exclusively Plan07 downstream work.

## Deviations and Simplification

The parent formally split freshness from the original Plan07 into this prerequisite Plan13, validated the revised plans, and authorized seven owned paths. The managed file-length check detected publication.rs at 643 lines after initial inline methods. A 35-line child `publication/epoch.rs` now owns the narrow checked operations without widening private-field visibility; a combined import and module wiring keep publication.rs at its existing 628-line limit. No checker exception or unrelated comment removal was used.

AGENTS.md, Bright Builds sidecar, placeholder-only overrides and architecture/code-shape/testing/verification/operability/Rust standards informed execution. The simplification pass retained one checked advancement operation, one existing publication guard and the existing poison behavior. No schema, namespace, RPC surface, recovery algorithm or generic payload policy was added.

## Security and Stub Review

T-160-37 is covered by same-generation ABA, shared-clone raw put/delete, both recovery edges, failed and successful disable, foreign origin, metadata refusal, trusted append and overflow controls. The token crosses only the existing guarded in-memory boundary. No new endpoint, filesystem access pattern, auth path or trust-boundary schema was introduced. No TODO/FIXME/placeholder implementation remains in the new module.

The new child module uses ordered pinned anchors `packages/bitcoin-knots/src/index/blockfilterindex.cpp`, then `packages/bitcoin-knots/src/blockfilter.cpp`. Existing files retain their breadcrumbs.

## Task Commits and Shared State

Both tasks and this summary await the strict outer consolidated commit. No staging, commit, push, branch, worktree, hook bypass or shared STATE/ROADMAP/REQUIREMENTS/todo mutation was performed by this executor. Requirements-completed remains empty until the parent gate.

## Authentication Gates and Deferred Issues

None. Planned transport integration, scoped Clippy and consolidated native verification remain downstream responsibilities.

## Self-Check: PASSED

All seven owned Rust files and this summary exist. Final epoch filters passed seven tests; the existing six point-query regressions passed. Scoped formatting, managed rules, whitespace and parent breadcrumb checks passed. Commit-existence checks are inapplicable under the strict no-intermediate-commit lifecycle; no hash or requirement completion was fabricated.
