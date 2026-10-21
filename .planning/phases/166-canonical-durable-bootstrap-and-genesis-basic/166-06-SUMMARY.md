---
phase: 166-canonical-durable-bootstrap-and-genesis-basic
plan: "06"
subsystem: storage
tags: [atomic-genesis, leased-reopen, exact-work, generation-fencing]
requires:
  - phase: 166-05
    provides: Strict complete-record decoder and sealed pure genesis proof
  - phase: 166-15
    provides: Isolated scratch inspection, source recheck and checked cleanup
provides:
  - One prevalidated SyncAll batch for every genesis authority row
  - Separate initializing and existing-only canonical genesis openers
  - Immediate original logical revalidation after leased scratch admission
  - Private instance and write-generation bound consuming recovery capability
affects: [166-07, 166-08, 166-09, 166-10, 166-11, 166-12, 166-13, 166-14]
tech-stack:
  added: []
  patterns: [prevalidated cross-keyspace publication, same-proof original admission, sealed consuming owner]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis/tests.rs
  modified:
    - packages/open-bitcoin-node/src/storage/canonical_bootstrap.rs
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs
    - packages/open-bitcoin-node/src/storage.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Reuse one strict decoder and logical validator before publication, on scratch data and immediately after original open.
  - Bind the recovery capability to the actual open instance and monotonic Fjall sequence, with complete logical revalidation on consumption.
  - Initialize existing history-coverage invalidation only after canonical proof, then revalidate before sealing; this grants no ordinary-history qualification.
patterns-established:
  - Public canonical owners expose immutable genesis facts without a raw store, clone or generic-store constructor.
  - An uncertain commit yields no owner; the next open independently proves completeness or refuses.
requirements-completed: [CBOT-02, CWOR-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 166-2026-10-18T22-36-40Z
generated_at: "2026-10-20T16:52:43Z"
commits: false
duration: 20 min 30 sec recorded implementation and verification window
completed: 2026-10-20
---

# Phase 166 Plan 06: Atomic Canonical Genesis and Proved Reopen Summary

**Canonical genesis now publishes as one durable authority batch and reopens only through leased scratch inspection, matching original proof and a private live-instance capability.**

## Completion Boundary

Tasks: **2/2 complete**. The recorded window runs from the first retained RED at 2026-10-20T16:30:10Z through final checks at 2026-10-20T16:50:40Z; initial instruction/source loading is excluded. Seven implementation/evidence paths changed, plus this summary. The writer module has 249 lines, its tests 604, the bootstrap module 139 and the main store 592; all remain below 629 lines.

This accepts plan 166-06 only. Requirement frontmatter records its contribution, not whole-phase closure. No staging, commit, push, branch change, hook bypass, shared state/task edit or historical contract renewal occurred. Root owns STATE/ROADMAP/REQUIREMENTS, subsequent integration, full native verification and final normal-hook Git saving.

## Atomic Publication

`packages/open-bitcoin-node/src/storage/canonical_bootstrap.rs` prepares every encoded row, decodes the proposed batch through the existing strict current-format decoder and runs the complete pure validator before publication. The private writer in `packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis.rs` inserts canonical identity/version, schema 2, complete genesis body, both header/index snapshots, one exact-work active position with explicit empty confirmed-transaction counts, coins B and complete ScriptsValid genesis identity/coverage through **one** `db.batch().durability(Some(SyncAll)).commit()`.

No public raw save sinks are chained. Coins contain B only: no C, H, genesis outputs, undo, legacy chainstate snapshot or BASIC records are introduced. All four selectors have exact durable-byte comparisons and agreement among genesis, active position and work. Height-zero work is small; these tests do not manufacture an above-u128 genesis chain. Existing independent wide-number evidence remains separate in 166-05/10.

Test-only BeforeCommit refuses without any authority rows; the engine-created authority-empty directory explicitly refuses subsequent initializing and observation opens. AfterCommit returns CommitUncertain without an owner, releases every handle, and a later existing-only open independently proves the fully committed state. Both boundaries run for all four selectors. These are software fault controls, not hardware/power-loss or actual-daemon differential evidence.

The batch seam records actual database sequence immediately before and after the authority commit. It proves a single increment across that commit and no subsequent durable writes during postproof recovery. The Fjall revision pinned by `packages/Cargo.lock` obtains one sequence for the entire nonempty batch and applies/publishes all its items under that sequence. The sequence is also a conservative live mutation fence, never decoded authority.

## Existing-Store Admission and Public API

The public associated methods are:

- `FjallNodeStore::open_canonical_genesis(path, selected) -> Result<OpenedCanonicalGenesis, CanonicalBootstrapRefusal>`: initialize missing/empty storage or prove existing storage.
- `FjallNodeStore::reopen_canonical_genesis_existing(path, selected) -> Result<OpenedCanonicalGenesis, CanonicalBootstrapRefusal>`: refuse missing/empty storage without creating a source directory or engine.

`OpenedCanonicalGenesis` and the fixed, payload-free refusal enum are exported through `packages/open-bitcoin-node/src/storage.rs`. The owner exposes only immutable `genesis()` and `position()` publicly. It has no public constructor, raw-store accessor, Clone, Default or deserializer.

Existing admission retains the same owned application lease through source-shape inspection, the existing source-engine lock, bounded private copying, actual child-only recovery, source recheck, checked scratch cleanup and the detached-data proof callback. The accepted 166-15 helper runs that callback only after successful cleanup while retaining source locks. After it returns, the temporary engine lock is released but the application lease remains owned. Only then does the private unrecovered original constructor run. The same complete logical validator immediately rereads original records and must produce an equal proof, including relevant BASIC/prune facts, before any node recovery.

Only after that proof does existing `recover_validation_history` initialize the current in-memory coverage invalidation owner. Complete current coverage was already proved; the test seam confirms this adds no durable progress. A further complete proof precedes sealing. No ensure_schema, schema migration, hydration, pruning or index recovery runs before admission, and no ordinary-history qualification is minted. Generic `open` and `open_ordinary` retain their existing contracts.

All twelve mismatched selector combinations, missing required rows, malformed/legacy/high-bit work, wrong B/identity/schema, corrupted body and extra C/H/undo/recovery/BASIC artifacts refuse before the original constructor. Unix preservation assertions compare every original file's contents and every file/directory mode and mtime; atime is intentionally excluded. Refusal releases locks. Unsafe symlink paths and actual engine-lock contention also refuse with unchanged target/source facts. The generic store remains unproved, and ordinary opening still refuses every proved genesis selector.

## Private Handoff to 166-07

The crate-only consuming seam is `OpenedCanonicalGenesis::into_parts() -> Result<(FjallNodeStore, RecoveredCanonicalGenesis), CanonicalBootstrapRefusal>`. Its token lives at `crate::storage::fjall_store::canonical_genesis::RecoveredCanonicalGenesis`; fields and constructor remain private, with no clone or generic-store factory.

`RecoveredCanonicalGenesis::check(&store) -> Result<&CanonicalGenesisProof, CanonicalBootstrapRefusal>` compares the actual publication-control Arc identity, the original database write sequence, the entire current logical proof and the sequence again after reading. Identical-byte rewrites invalidate the generation; an equal-looking separate live store fails instance identity. The public wrapper retains exclusive storage ownership until this consuming seam. Plan 166-07 must check/consume it before runtime/index writes and then establish its own existing managed lineage fences. Its one deliberately staged dead-code expectation is on `into_parts`; 166-07 explicitly owns removal once a production caller exists.

Fault controls independently change the just-opened original to malformed authority and to a different but independently valid proof. Both block sealing. Current history ownership is also exercised through the real public raw header sink: its write changes coverage to UnknownLegacy and prevents consumption/reopen. The recovered capability is distinct from ordinary validation and numerical DTOs.

## Decisions, Deviations and Review

**[Rule 3 - Blocking integration] Remove the consumed inspection staging expectation.** Root explicitly authorized the narrow additional ownership of `packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs`. Only the already-existing with_inspection dead-code expectation was removed; its implementation, visibility, locks, limits and cleanup are unchanged. The consumed bootstrap-module staging expectation was also removed within the originally assigned file. No lint suppression was broadened.

The simplification pass retained one decoder/validator and one sealed owner instead of parallel persistence formats or generic admission factories. The capability retains an Arc identity without an extra hidden engine/store clone. No new dependency, workspace crate, endpoint or unmodeled threat surface was introduced; planned T-166-06-01/02 cover publication, handoff and redacted diagnostics. Stub scanning found no unfinished goal-blocking implementation. The implemented consuming seam's future production caller belongs to 166-07, not a placeholder path here.

Guidance applied: repository AGENTS native verifier/tool pins/parity rules, Bright Builds sidecar, standards overrides (no concrete exception), standards index and architecture/code-shape/testing/verification/Rust guidance, the formal execute-plan workflow and summary template. Both active lesson inputs were loaded: 8,856 bytes and 2,953 conservative estimated tokens. No project skills were found or lesson/archive files edited. Contributor/operator documentation is assigned to 166-09/13; source-contract renewal remains 166-11/12.

## Issues and Evidence Limits

The initial atomic scaffold correctly failed its fresh-publication assertion. The initial reopen scaffold failed its required fresh positive control, so that RED is not evidence of a successful existing-store recovery attempt. Initial one-case GREEN retained four staged unused-code warnings before expanded tests consumed the internal seam. The expanded atomic run passed three cases and failed one test's incorrect absolute sequence expectation (observed 15, expected 1); database/keyspace setup had already advanced the sequence. Inspection of the exact pinned batch contract and the immediate before/after test seam resolved the assumption without weakening all-row or fault controls. Its unused test import was removed.

Initial scoped Clippy failed only two test-helper `ok().expect()` style violations. Direct Result::expect preserves success behavior and improves fixed-category failure output. Final all-target/all-feature Clippy and one recompiled affected helper/atomic test pass. Production code is unchanged from the final 20 focused cases and 562-case storage pass, so the full storage suite was not redundantly repeated after that helper-only lint correction.

The broad suite's long-running cases continued to emit progress and consume CPU; retained process samples show active exact-route inspection children. No command was killed, no deadline/inspection bound was weakened and no cache was replaced. This run is macOS/Unix evidence. The previously documented APFS non-UTF8 fixture setup limitation and unexecuted Linux path remain distinct from runtime acceptance. Supported cooperative application openers and the actual engine lock define the concurrency guarantee; arbitrary direct-engine/filesystem writers, parent/system crash, forced termination, OOM and OS crash-dump behavior remain outside it.

No authentication gate or external setup was needed. Ordinary descendants, contextual/Signet challenge validation, runtime/BASIC integration, actual Cargo/Bazel operator reopen and pinned-daemon differential qualification remain with their assigned subsequent plans/phases. Full default native verification, independent final reviews and Git saving remain 166-14/root responsibilities.

## Task Commits

None. `commits: false` follows the strict wrapper; no task/TDD/summary commit or Git mutation was attempted.

## Automated Evidence

Every Cargo invocation ran serially through `scripts/command-timings.ts`, pinned Bun 1.3.9 PATH, the coordinator-approved isolated CARGO_TARGET_DIR and canonical process-local TMPDIR. Logs remain ignored and preserved. Zero-match integration executables are excluded from positive counts. The storage suite includes the 20 focused cases; counts are not added as distinct tests.

| Check | Actual outcome | Full repository-relative receipt |
| --- | --- | --- |
| Atomic scaffold RED | Exit 101; zero passed, one failed | `.local/open-bitcoin-dev/phase166/plan06/atomic-red.log` |
| Reopen scaffold RED | Exit 101; zero passed, one failed at fresh control | `.local/open-bitcoin-dev/phase166/plan06/reopen-red.log` |
| Initial atomic GREEN | Exit 0; one passed; staged unused-code warnings | `.local/open-bitcoin-dev/phase166/plan06/atomic-green-initial.log` |
| Expanded atomic attempt | Exit 101; three passed, one failed on absolute sequence assumption; unused import warning | `.local/open-bitcoin-dev/phase166/plan06/atomic-expanded.log` |
| Expanded reopen | Exit 0; 15/15, 135.46 seconds | `.local/open-bitcoin-dev/phase166/plan06/reopen-expanded.log` |
| Final atomic | Exit 0; 4/4, 49.02 seconds | `.local/open-bitcoin-dev/phase166/plan06/atomic-final.log` |
| Final reopen including direct engine-lock control | Exit 0; 16/16, 149.35 seconds | `.local/open-bitcoin-dev/phase166/plan06/reopen-final.log` |
| Storage regressions | Exit 0; 562/562, 293.64 seconds | `.local/open-bitcoin-dev/phase166/plan06/storage-regression.log` |
| Initial scoped Clippy | Exit 101; two test-helper ok_expect violations | `.local/open-bitcoin-dev/phase166/plan06/clippy-first.log` |
| Final node all-target/all-feature Clippy, warnings denied | Exit 0; 22.87 seconds | `.local/open-bitcoin-dev/phase166/plan06/clippy-final.log` |
| Recompiled affected helper/atomic case | Exit 0; 1/1, 0.81 seconds; compile 7.73 seconds | `.local/open-bitcoin-dev/phase166/plan06/helper-final.log` |
| Final scoped rustfmt check | Exit 0 | `.local/open-bitcoin-dev/phase166/plan06/format-final.log` |
| Final breadcrumbs | Exit 0; 1227 Rust files | `.local/open-bitcoin-dev/phase166/plan06/breadcrumbs-final.log` |
| Final managed checks | Exit 0; zero findings | `.local/open-bitcoin-dev/phase166/plan06/managed-final.log` |
| Final diff whitespace | Exit 0 | `.local/open-bitcoin-dev/phase166/plan06/diff-check.log` |

Liveness receipts are `.local/open-bitcoin-dev/phase166/plan06/reopen-liveness.log`, `.local/open-bitcoin-dev/phase166/plan06/storage-liveness.log`, `.local/open-bitcoin-dev/phase166/plan06/storage-liveness-later.log` and `.local/open-bitcoin-dev/phase166/plan06/storage-liveness-final.log`. The final sample has no live test process because the completed suite had exited; the actual final test result above is the acceptance evidence. No unrelated tree was inspected for cleanup or removed.

## Self-Check: PASSED

All seven changed implementation/evidence files and this summary exist. The final four atomic and sixteen reopen cases, 562 storage regressions, post-lint affected case, all-target/all-feature Clippy, scoped formatting, breadcrumbs, managed checks and diff review pass. Every new Rust file is registered and owned files stay below the size limit. No goal-blocking stub or unresolved plan-06 issue remains. Commit-existence verification is inapplicable to intentionally uncommitted work. Root retains subsequent plan acceptance, shared lifecycle updates and full native/normal-hook saving.
