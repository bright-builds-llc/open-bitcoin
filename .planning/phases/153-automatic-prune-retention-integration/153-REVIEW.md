---
phase: 153-automatic-prune-retention-integration
reviewed: 2026-10-03T06:40:18Z
depth: standard
files_reviewed: 26
files_reviewed_list:
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/initialize.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests/fixtures.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests/writers.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs
  - packages/open-bitcoin-node/src/storage/fjall_store.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/payload_usage.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/payload_usage/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/prune.rs
  - packages/open-bitcoin-node/src/sync/types.rs
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/fixtures.rs
  - packages/open-bitcoin-rpc/src/context.rs
  - packages/open-bitcoin-rpc/src/context/prune.rs
  - packages/open-bitcoin-rpc/src/dispatch/prune/tests.rs
  - scripts/check-phase127-authoritative-network-state-unification.ts
  - scripts/check-phase127-authoritative-network-state-unification.test.ts
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
generated_by: gsd-code-review
lifecycle_mode: yolo
phase_lifecycle_id: 153-2026-10-03T04-01-54
generated_at: 2026-10-03T06:17:48Z
---

# Phase 153: Code Review Report

**Reviewed:** 2026-10-03T06:40:18Z
**Depth:** standard
**Files Reviewed:** 26
**Status:** clean

## Summary

Reviewed the uncommitted working-tree implementation from Plans 01–03 against base `6dd6f18c9b16158b16391fcd288aadc94728cfcd`, including new intent-to-add source files. The original review covered 24 source/manifest files; the targeted follow-up below adds two checker files. The exact source scope is recorded above. Planning/task ledgers and Plan 04 documentation work were excluded. No actionable correctness, security, or test-reliability findings were identified in this scope.

All reviewed files meet quality standards. No issues found.

The review was informed by `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, the local standards index and architecture/code-shape/testing/verification/Rust pages, both complete active lesson files, and Phase 153 context, research, Plans 01–03 and their summaries. Overrides contain no substantive exception; neither project skill directory exists.

## Reviewed Safety and Behavior

- Accounting sums actual retained block and encoded undo value lengths from one snapshot, independently of active candidate eligibility. Half pairs and present zero-length values remain distinct from absent pairs. Checked totals and backend/poison errors propagate.
- Payload writers share the measurement mutex across invalidation, live mutation and persistence completion. Typed block/undo writes, migration seeding, generic raw payload operations and the paired-delete batch route through that boundary. Other direct batches affect nonpayload records. Reopened identities and overflow cannot authorize idle reuse.
- Ordinary flushes call the existing pure planner under the network authority. Reuse keys include the captured revision, tip height/hash, mode and current locks. Periodic coalescing skips candidate application; Always bypasses it. Nonempty automatic plans select the existing full-checkpoint policy.
- Production lock publication, RPC list/status reads and manual current-lock revalidation use the same authority as automatic deletion. Startup recovery reads durable locks before readiness. The configured network threshold matches the pinned Knots chain parameters, and offline prune activation leaves sync/listener activation gates intact.
- Full checkpoints stage the current cache best-block before writing, preserving a newer overlay tip and carrying the durable parent tip for repeated idle flushes. Read errors propagate without a fabricated tip or leftover-snapshot fallback.
- Committed-delete callbacks and error receipts remove cached bodies and in-memory undo even when later coins or metadata persistence fails. Earned prune markers/support counts and existing finish-or-refuse recovery remain the owners of durable evidence.
- The genuine fixture uses a legal 550 MiB target and actual encoded values above that target through the same ordinary-cycle helper as the daemon. It covers protected survivors, deletion/checkpoint, repeat/reopen, serving categories and both wallet adapters. The later failure adapter delegates real accounting/locks/deletion/counters and injects only metadata persistence failure; its later stage uses an explicitly constructed fixture owner and coins view. The initial successful checkpoint uses the production durable owner.
- The existing paired-delete outcome is publicly nameable for dependent implementations of the public flush-sink trait. Breadcrumb manifest changes add the eight new Rust paths, preserve existing groups and reference existing pinned source anchors.

## Simplification Pass

The touched seams retain one storage adapter, one pure planner, one deletion owner and the existing Periodic/Always worker. Cohesive accounting and automatic orchestration live in small modules; fixtures are separated from behavior assertions. The current-best carry is placed at the common write boundary, which also handles subsequent cache-clearing flushes. No additional abstraction or alternate persistence path is warranted by this review.

## Verification and Limits

The original review performed static source/diff inspection, import/forwarding and caller checks, manifest parsing/reference checks, pinned chain-parameter comparison and `git diff --check`; these checks passed. No Cargo, Bazel or test-suite command was run by this reviewer. Targeted RED/GREEN results are executor-reported evidence in the three summaries, not independently executed results here. Additional read-only checker/path checks are recorded below. The root-owned default native verifier, coverage, Bazel and phase/lifecycle verification remain required finalization gates.

The large fixture is sparse codec-valid retention evidence, not continuous consensus-chain or public-network sync evidence. Most retained bytes are nonactive, so eligible deletes intentionally need not reach the soft target. Logical live-value bytes do not establish a physical allocation limit or immediate space reclamation. These limitations are explicit and do not undermine the reviewed retention integration claim.

Source files were not modified, staged or committed. Only this review artifact was created.

## Follow-up Audit — Checker Corpus Correction

**Reviewed:** 2026-10-03T06:40:18Z
**Delta:** `scripts/check-phase127-authoritative-network-state-unification.ts` and its matching test file.
**Result:** clean; original 24-file findings remain unchanged.

The first root-owned full native run stopped after 661170 ms at the old Phase 127 corpus guard, which excluded only `tests.rs` and treated the new nested daemon fixture as production. Reviewed the correction against the root TypeScript/JavaScript standards: the scanner now excludes a source only when its basename is `tests.rs` or its relative parent-directory components contain the exact name `tests`. Existing production constructor analysis, alias detection and all other invariants are unchanged.

Using `path.relative(helperRoot, path.dirname(sourcePath))` prevents an ancestor directory named `tests` from excluding the entire repository. Splitting by the matching native `path.sep` respects platform paths, and exact component membership retains ordinary `fixtures.rs` and `tests_support/fixtures.rs` as production candidates. The regression checks both legitimate nested fixture allowance and rejection of duplicate constructors under `tests_support`; it does not introduce a broad filename or substring exemption. The small filter is the simplest correction for the documented test-directory convention.

Root-reported regression evidence was 16 passing/1 failing against the former filter, then 17/17 passing after the correction, including the production-fixture negative control. This reviewer independently ran the actual Phase 127 checker successfully and exercised the filter with both `path.posix` and `path.win32` for nested tests, `tests_support`, ordinary fixtures, basename exclusions and an ancestor named `tests`; all boundary checks passed. `git diff --check` also passed. No source edits or Cargo/Bazel commands were performed. The failed full native run is not a passing release gate; root must rerun the full contract after this correction.

***

_Reviewed: 2026-10-03T06:40:18Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
