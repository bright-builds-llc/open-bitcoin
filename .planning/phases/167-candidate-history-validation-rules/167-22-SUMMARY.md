---
phase: 167-candidate-history-validation-rules
plan: "22"
subsystem: testing
tags: [rust, block-undo, fixtures, provenance]
requires:
  - phase: 167-07
    provides: Real BlockUndo overwrites field with explicitly nonauthorizing empty provenance
provides:
  - Seven mechanically adapted fixture literals in six existing node test owners
  - Exact pre-edit snapshots and byte recovery evidence preserving existing controls
affects: [167-08, 167-20]
tech-stack:
  added: []
  patterns: [Exact byte recovery after removing additive field lines]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/chainstate/replay/tests.rs
    - packages/open-bitcoin-node/src/chainstate/tests.rs
    - packages/open-bitcoin-node/src/storage/chain_compatibility/tests/preservation.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
key-decisions:
  - "An explicit empty overwrites collection remains missing provenance and grants no restoration authority."
  - "Node compilation and the full existing node regression remain paired with Plan08 production DTO conversion."
requirements-completed: [CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-21T21:30:18.944Z"
commits: false
duration: 50.495s
completed: 2026-10-21
---

# Phase 167 Plan 22: Existing node fixture constructors Summary

**Seven existing BlockUndo literals now explicitly initialize missing overwrite provenance while every original fixture byte remains recoverable unchanged.**

## Files Created/Modified

| Modified Rust owner | Original literal lines | Added fields | Final lines |
| --- | --- | --- | --- |
| `packages/open-bitcoin-node/src/chainstate/replay/tests.rs` | 233 | 1 | 416 |
| `packages/open-bitcoin-node/src/chainstate/tests.rs` | 217,289 | 2 | 615 |
| `packages/open-bitcoin-node/src/storage/chain_compatibility/tests/preservation.rs` | 203 | 1 | 577 |
| `packages/open-bitcoin-node/src/storage/fjall_store/tests.rs` | 219 | 1 | 274 |
| `packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs` | 62 | 1 | 605 |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs` | 91 | 1 | 619 |

Exactly six existing Rust owners were modified; no Rust file was created. Each existing direct literal gained only `overwrites: Vec::new(),`. No formatting rewrite was necessary. This summary and ignored receipts are execution records. The final largest owner is619lines, below629.

Pre-edit copies are retained under `.local/open-bitcoin-dev/phase167/plan22/before/` using their full repo-relative paths. `snapshot.json` records original byte counts and SHA256 values; `verify.json` records final values and the original recovered SHA256. Removing only the seven added field lines recovers all six original files byte-for-byte. Consequently every original token, test name, registration, assertion, input, expected error and injected fault remains unchanged. `HistoricalBlockUndo` wrappers, return signatures, Default calls and nested structures are untouched.

## Completed Task and Original Obligation Mapping

Task1 is complete within its mechanical evidence boundary. The original08.1 constructor compatibility obligation was relocated by the checked execution-scope amendment into22.1(first six owners) and24.1(other five owners). Complete production DTO conversion, missing-legacy refusal and all original08.2 physical durability evidence remain08.

| Locked decision / threat | Preserved obligation | Actual evidence |
| --- | --- | --- |
| D-06;T-167-22-1 | Use accepted07's exact domain field without granting overwrite authority | Seven direct constructors use the real `overwrites: Vec<BlockOverwriteUndo>` field; actual provider is `packages/open-bitcoin-chainstate/src/types.rs`. |
| D-08;T-167-22-1 | Preserve ordinary/malformed controls; empty data cannot prove no-overwrite | Full-file byte recovery proves all preexisting inputs, refusals and faults unchanged. No production decoder or fallback changed. |
| D-09;T-167-22-2 | Attributable mechanical source evidence and honest paired handoff | Immutable pre-edit snapshots, owned diff, six actual source links and explicit pending compilation boundary below. |

An empty outer `overwrites` vector means unknown/missing provenance. A single valid complete batch whose inner `overwritten` vector is empty represents a captured no-overwrite scan. These fixture additions supply only the former. They do not claim legacy compatibility proof, canonical lineage, complete restoration records or permission to disconnect successfully.

## Commands and Actual Outcomes

Receipt root: `.local/open-bitcoin-dev/phase167/plan22/`. The helper ran with pinned Bun1.3.9 at `/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin/bun`.

| Command / check | Actual outcome | Receipt |
| --- | --- | --- |
| `node /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs init execute-phase 167` | Phase exists; state loaded; explicit plan/root `commits:false` overrides generic saving defaults. No shared state changed. | Tool outcome |
| Pinned Bun `mechanical-evidence.ts snapshot` | Exit0; original inventory matches233 /217,289 /203 /219 /62 /91; six source copies retained before edits. | `snapshot.json` |
| Pinned Bun `mechanical-evidence.ts verify` | Exit0; seven additions, six exact original byte recoveries, all owner lengths below629; diff contains no removals or other source changes. | `verify.json`,`mechanical-check.log`,`owned.diff` |
| Scoped `rustfmt --check --edition 2024 --config skip_children=true` on the six table paths in that order | Exit0 on first attempt; no recursive formatting or broad write. | `rustfmt-initial.log`(empty successful output); tool exit0 |
| `node /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs verify key-links .planning/phases/167-candidate-history-validation-rules/167-22-PLAN.md` | all_verified:true,verified6,total6; each real direct-literal consumer links to the actual domain provider. | `key-links.json` |
| Complete owned `diff -u` review against each pre-edit snapshot | Expected diff exit1 for each changed file; exactly seven additive field lines. | `owned.diff` |

The snapshot inventory statically finds58 existing `#[test]` definitions across five files; the sixth is a shared fixture module with zero local test definitions. This is source inventory, **not runtime discovery or a test pass**. No Cargo, Clippy, Bazel, node compilation, node tests or TDD cycle ran. No manufactured RED/GREEN or skipped-test success is reported.

## Guidance, Simplification and Issues

AGENTS.md local guidance, Bright Builds sidecar/overrides, standards index and architecture/code-shape/testing/verification/Rust/TypeScript guidance informed the bounded GSD execution. Accepted07's exact codec contract, the constructor inventory/amendment and preserved original08.1 action establish the field/authority and paired-compilation boundaries. Both canonically distinct active lessons were read:6898global+1958repo=8856bytes;2300+653=2953conservative tokens. Existing recent lesson-audit baseline applies; no new audit trigger. No lesson/archive/shared task file was edited.

Setup observations: optional `.claude/skills` and `.agents/skills` paths are absent; their inventory search printed missing-directory errors, not product failures. Some combined context reads exceeded output limits and were followed by targeted reads. Neither observation required source changes. No authentication gate, failed implementation attempt, failed formatting attempt or architectural deviation occurred.

The simplification pass retains seven explicit one-line fields in existing fixtures. No helper, new test, production export, dependency, breadcrumb-manifest update or source-guard renewal is needed. These intentional empty collections are nonauthorizing test construction, not unresolved feature stubs. No new network endpoint, auth path, file-access behavior or trust-boundary schema is introduced by the Rust diff. No undeclared threat flag remains.

## Pending Paired Verification and Scope

**Plan08 must complete production BlockUndoDto From/TryFrom conversion, then compile all node fixtures and run the unfiltered existing node `--all-features` suite with both22and24 fixture sets.** The current production DTO is deliberately incomplete after07, so a node compile is not an isolated22 gate. No node pass or whole-phase acceptance is claimed. Root acceptance is required before08release. Native/fresh coverage/Bazel and independent source/security/goal/lifecycle acceptance remain20.

commits:false. No staging, commit, push, hook bypass, worktree creation or Git command occurred. Preexisting checkout baseline `1b472fb225a290df5cdb06eeea2dc67c624c648e` is a source reference only, **not a task commit**. Root owns shared STATE/ROADMAP/requirements/config/todo/plans and normal-hook saving. Parallel24's disjoint source owners and all prior receipts/frozen originals/acquisition/corpus remain unchanged by this executor. Work stops before08.

## Performance Attribution

One task,six actual Rust owners. Pre-edit snapshot timestamp2026-10-21T21:29:28.449Z to final mechanical verification timestamp2026-10-21T21:30:18.944Z is50.495seconds measured source/check execution. Context loading and subsequent summary writing/validation are excluded; no predecessor or parallel24 duration is added. Both timestamps are retained in `snapshot.json`/`verify.json`.

## Self-Check: PASSED

- All six declared modified Rust owners and this summary exist. All six original SHA256 values recover exactly after removing only the seven added field lines; no rustfmt effects required qualification.
- Scoped formatting exits0 and owned diff review confirms only the seven intended additions. All six actual key links are verified; all source owners remain below629lines.
- `gsd-tools verify-summary --check-count 6` returns passed:true,checked6,found6,errors:[]; summary frontmatter validation returns valid:true,missing:[]. Receipts: `verify-summary.json`,`frontmatter.json`. The commit-existence result resolves only the explicitly labelled preexisting checkout baseline, not a task commit.
- Compilation and runtime test discovery/execution remain deliberately pending08; no current node test pass is claimed. Shared metadata and saving remain root-owned; root acceptance precedes08release.
