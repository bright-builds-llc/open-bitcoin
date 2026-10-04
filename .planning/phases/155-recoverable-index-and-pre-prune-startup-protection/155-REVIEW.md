---
phase: 155-recoverable-index-and-pre-prune-startup-protection
reviewed: 2026-10-04T18:22:15Z
reviewed_at: 2026-10-04T18:22:15Z
depth: standard
generated_by: gsd-code-review
lifecycle_mode: yolo
phase_lifecycle_id: 155-2026-10-04T16-01-08
diff_base: 88b726d8653eeda83be4df84b1a9c6e9b6d6214f
files_reviewed: 36
files_reviewed_list:
  - README.md
  - docs/parity/catalog/README.md
  - docs/parity/catalog/basic-compact-filters.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-chainstate/src/filter_index.rs
  - packages/open-bitcoin-chainstate/src/filter_index/recovery.rs
  - packages/open-bitcoin-chainstate/src/filter_index/tests.rs
  - packages/open-bitcoin-chainstate/src/filter_index/tests/commitments.rs
  - packages/open-bitcoin-chainstate/src/lib.rs
  - packages/open-bitcoin-codec/src/block_filter.rs
  - packages/open-bitcoin-codec/src/block_filter/validation.rs
  - packages/open-bitcoin-codec/src/block_filter/validation/tests.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
  - packages/open-bitcoin-node/src/storage.rs
  - packages/open-bitcoin-node/src/storage/filter_index.rs
  - packages/open-bitcoin-node/src/storage/filter_index/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/faults.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/prune.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/prune/records.rs
  - packages/open-bitcoin-node/src/sync/tests.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/faults.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/recovery.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/startup.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/startup/faults.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/startup/recovery.rs
  - scripts/check-phase155-filter-index.test.ts
  - scripts/check-phase155-filter-index.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
---

# Phase 155: Code Review Report

**Reviewed:** 2026-10-04T18:22:15Z
**Depth:** standard
**Files Reviewed:** 36
**Status:** clean

## Summary

Reviewed the explicit union of staged and unstaged changed source/documentation paths against `88b726d8653eeda83be4df84b1a9c6e9b6d6214f`, together with nonignored untracked source paths. Planning and task artifacts were excluded from the source scope. All new source paths were included; HEAD remained unchanged. No root project skills or `.claudeignore` were present.

No correctness, security or actionable maintainability findings were identified in the reviewed changes. The implementation meets the Phase 155 internal recovery and startup-protection contract. This review does not activate requirements or replace formal phase verification and the full native gate.

Material guidance: `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, `standards/index.md`, architecture/code-shape/testing/verification and Rust/TypeScript standards; both active lesson files were loaded completely (7,188 bytes; 2,397 conservative estimated tokens). Phase 155 CONTEXT, RESEARCH, all four PLANs and the available 01/02/03 SUMMARYs informed the review. Plan 04's final summary was not yet available at review time.

## Reviewed Safety and Integration Evidence

| Boundary | Assessment and source evidence |
| --- | --- |
| Stored bytes and immutable identity | `codec/src/block_filter/validation.rs:46` streams canonical bounded encodings and checks truncation, range, padding and arithmetic. `node/src/storage/filter_index.rs:193` validates byte hash and header commitments. Candidate publication at `node/src/storage/fjall_store/filters/publication.rs:227` proves predecessors and rejects conflicting immutable rewrites before committing. |
| Current recovery authority | `chainstate/src/filter_index.rs:189` proves contiguous unique metadata identities, recomputed header hashes and parent linkage, with recovered coins B equal to the metadata tip. `filter_index/recovery.rs:96` selects only a contiguous common saved prefix, and `:135` requires the exact saved endpoint. Ahead record-only rows cannot mint progress. |
| Saved fence provenance | `node/src/storage/fjall_store/filters.rs:104` rejects endpoint height above the saved fence and same-height hash conflict. Higher saved fences remain historical provenance; current B and full compatible metadata independently determine safe progress. Legitimate ahead/fork recovery does not invent historical header proof. |
| Cursor and protection durability | `filters/publication.rs:132` publishes immutable candidates, changed projection, explicit state and the complete preserved lock map in one SyncAll batch after rereading B/metadata. `:331` propagates commit uncertainty and poisons the live publisher. Reconciliation never removes immutable or hidden projection suffix rows. |
| Earliest startup deletion boundary | `chainstate/flush_lifecycle.rs:235` calls the guard after coins recovery/outcome projection and before lock loading, prune resume, readiness or cache construction. The unchanged `sync/open_runtime.rs` production constructor consumes this initialize path before managed-chainstate construction. `filters/startup.rs:97` checks intent against recovered protection before reconciliation mutation. |
| Low and upper heights | `chainstate/src/filter_index.rs:149` uses checked next-height arithmetic and explicit exhaustion. `:236` rejects malformed reserved endpoints before the ordinary buffered helper. Direct intent checks preserve protection at heights 0/1 and maximum height; stronger saved protection is retained. |
| Serialization and ownership | `node/src/storage/fjall_store.rs:75` shares publication control across clones; full lock-map replacement takes the same guard. The publisher explicitly requires runtime serialization for coins/metadata/prune writers and does not claim read/compare is CAS. Exclusive startup performs recovery before exposing the runtime. |
| Production fault and refusal evidence | New tests actually close/reopen Fjall and construct `DurableSyncRuntime`. They separately inspect cursor/projection/rows/protection, exact bodies/non-genesis undo and live intent. The matrix distinguishes sparse deletion-order fixtures from staged validated historical spends/forks, covers H/B replay and stale metadata, single-mate prune states, and before/after publication faults. `BeforeChainMeta` and its concrete writer hook remain `cfg(test)` only. |
| Parity and native claim guard | Exactly one parity surface owns CFIX-02, CFIX-04 and CFPR-03. New Rust files have mapped breadcrumbs. The checker has mutation tests for missing links, ordering, authority/fault evidence, claims and verifier wiring. Source checks remain explicitly supplementary to durable Rust evidence. README/catalog claims preserve the Phase 156–162 deferrals and software-fault limitations. |

Paths abbreviated in this table are under `packages/open-bitcoin-*`; the exact scope is preserved in frontmatter.

## Simplicity and Code Shape

The explicit simplification assessment found a coherent pure reducer, a specialized bounded codec and one concrete atomic publisher reused by startup. Existing hashes, historical input types, coins recovery and prune deletion ownership are reused. No second index cache, deletion manager, storage engine, dependency, public activation path or repair framework was added. Complete integrity validation proves each row and direct decreasing-height edge; the separate hash lookup intentionally retains full ancestry proof. The deterministic 256-row test asserts the complete scan's `4N - 1` reads without treating it as archive-scale timing evidence.

New modules follow `foo.rs` plus `foo/`; production functions stay below the approximately 161-line refactor trigger. The existing store entrypoint is 624 lines and the largest new file is a 609-line test module, within the managed 628-line bound. No additional abstraction or refactor is needed to make the reviewed phase's safety transitions clearer.

## Verification and Limits

Reviewer-executed checks passed: `bun run scripts/check-phase155-filter-index.ts`, `bash -n scripts/verify.sh`, `git diff HEAD --check`, JSON parsing and unique requirement-owner inspection. Source/test assertions and production call chains were inspected directly.

The available summaries report affected core coverage/Clippy, 29 storage/codec tests, 56 matching storage/startup tests, existing initialize/restart suites and Phase 123/135/154 guards. These are executor-reported evidence, not fresh Cargo results from this reviewer. The final expanded matrix and default full native verifier remained the executor/parent gates at dispatch. No Cargo or Bazel job was launched by this reviewer because the executor owns the shared build lock and the parent owns full verification.

`status: clean` means this source review found no issues. It does not mean the pending full native build, integration, coverage and Bazel gates have passed. Hardware power-loss behavior, archive-scale performance, public activation/catch-up, ordinary filter prune CRUD/disable, runtime reorg orchestration and later serving/product surfaces are outside this review's Phase 155 contract.

Only this REVIEW.md was created. No source file was modified and no commit was made.

***

_Reviewed: 2026-10-04T18:22:15Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
