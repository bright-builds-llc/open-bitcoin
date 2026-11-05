---
phase: 167-candidate-history-validation-rules
plan: "24"
subsystem: testing
tags: [block-undo, mechanical-fixtures, missing-provenance]
requires:
  - phase: 167-07
    provides: Accepted BlockUndo overwrites collection and exact empty-versus-complete contract
provides:
  - Five existing node fixture literals explicitly initialize missing overwrite provenance
  - Exact pre-edit byte recovery and pending paired Plan08 regression handoff
affects: [167-08, 167-16, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Additive fixture construction, Exact byte recovery]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append_proof.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/tests/chainstate_wallet_and_headers.rs
    - packages/open-bitcoin-node/src/sync/tests/phase165_live_fixture.rs
    - packages/open-bitcoin-node/tests/phase165_live_fixture.rs
key-decisions:
  - "Explicit empty overwrites remains missing/unproved provenance and grants no no-overwrite or restoration authority."
  - "Plan08 must perform paired full node regression after production DTO conversion and both Plan22/24 fixture sets."
requirements-completed: [CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-21T21:30:14Z"
commits: false
duration: 31s
completed: 2026-10-21
---

# Phase 167 Plan 24: Filter, snapshot and live-fixture constructors Summary

**Five existing direct BlockUndo literals now provide explicit empty overwrite collections while every original byte outside the added lines remains intact.**

## Files Created/Modified

| Modified file | Added field line | Current lines |
| --- | --- | --- |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append.rs` | 41 | 617 |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append_proof.rs` | 32 | 590 |
| `packages/open-bitcoin-node/src/storage/snapshot_codec/tests/chainstate_wallet_and_headers.rs` | 12 | 331 |
| `packages/open-bitcoin-node/src/sync/tests/phase165_live_fixture.rs` | 310 | 612 |
| `packages/open-bitcoin-node/tests/phase165_live_fixture.rs` | 549 | 569 |

Exactly five Rust owners, one added `overwrites: Vec::new(),` line per file. This summary and ignored Plan24 receipts are execution records. No source file was created. Before snapshots and SHA256/byte/line inventory are under `.local/open-bitcoin-dev/phase167/plan24/before/` and `before-manifest.json`; current hashes and recovery results are in `mechanical-evidence.json`.

Preexisting checkout reference only, not a task commit: `1b472fb225a290df5cdb06eeea2dc67c624c648e`. It resolves to an existing commit. commits:false; no staging, commit, push, worktree or hook bypass occurred.

## Completed Task and Original Obligation Mapping

Task1 is complete at its mechanical evidence boundary. The constructor amendment inventoried the original direct literals at lines40,31,11,309,548. The execution-scope amendment relocated this cohesive five-file portion of original08 Task1 to24;08 retains real production DTO conversion and durable evidence. The original08 Task1 snapshot and the accepted07 exact codec contract informed the adaptation.

| Preserved obligation | Current evidence |
| --- | --- |
| Filter append and append-proof fixture construction | Only the explicit field was inserted into each existing save_undo argument; ordinary transaction vectors and all append/proof assertions and fault injection remain identical. |
| Per-block snapshot undo roundtrip fixture | Only the explicit field was inserted; all Coin inputs, transactions, encoded-shape checks and expected roundtrip remain unchanged. |
| Phase165 changed-undo controls | Both unit/public changed-undo inputs, test names, refusal assertions and physical-storage comparisons remain identical. |
| D-06/D-08: preserve authority and refusal boundaries | Actual07 `BlockUndo.overwrites` is `Vec<BlockOverwriteUndo>`; zero batches is missing/unproved, never a captured no-overwrite scan. No complete batch, prior Coin or integrity digest is manufactured. |
| D-09: attributable preserved evidence | Removing the single added field line from each current file recovers its entire pre-edit snapshot byte for byte, with no rustfmt differences. Frozen sources, acquisition/corpus, receipts and prior accepted evidence are unchanged. |

The actual links are each of the five listed direct literals to `packages/open-bitcoin-chainstate/src/types.rs:40`; source inspection and an anchored literal check verify the field occurs inside the real constructor. These are executable fixture initializers, not comments or arbitrary pattern-only additions. HistoricalBlockUndo wrappers, return signatures, Default calls, nested other structs, production logic, tests and assertions receive no other edits.

## Actual Commands and Outcomes

Receipts live under `.local/open-bitcoin-dev/phase167/plan24/`.

| Command/check | Actual outcome | Receipt |
| --- | --- | --- |
| gsd-tools init execute-phase167 | Phase found;26 plans;existing execution context loaded. No shared state changed. | Tool output |
| `rustfmt --check --edition 2024 --config skip_children=true` on the five exact owners | Exit0, first attempt; no formatting writes or recursive sibling formatting. | `rustfmt.log` |
| `git diff --check --` on the five exact owners | Exit0; full owned diff reviewed and contains exactly five additions, zero deletions. | `diff-check.log`, `owned.diff` |
| Pre-edit/current comparison | 5/5 exact byte recovery after removing only the added line; one direct literal and one added field each; all sources below629lines. | `before-manifest.json`, `mechanical-evidence.json` |
| `gsd-tools.cjs verify key-links .planning/phases/167-candidate-history-validation-rules/167-24-PLAN.md` | all_verified:true, verified5,total5; actual constructor placement separately checked. | `key-links.json`, `mechanical-evidence.json` |
| Stub-text scan on the five owners | No TODO/FIXME/placeholder/coming-soon/not-available matches; rg exit1 is the expected no-match outcome. | Tool output |
| Checkout reference resolution | `git cat-file -t` returns commit for the explicitly labelled preexisting baseline. | Tool output |

**Tests were not run. No Cargo, Bazel, node compilation, Clippy, build, TDD RED/GREEN or full-verifier pass is claimed.** These are mechanical literal changes under the explicit checked plan, with production DTO conversion intentionally incomplete until08. Test discovery is not applicable to the source/format checks; zero discovered tests is not reported as success.

08 must compile and run the entire existing unfiltered node all-features suite after its real DTO conversion and both22/24 fixture sets, serially through the cooperative lock, for example `bun run scripts/command-timings.ts run --key phase167-08-node-regression -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features`. These future paired checks remain mandatory before combined acceptance. Plan20 retains full native/fresh coverage/Bazel and independent source/security/goal/lifecycle acceptance.

## Guidance, Simplification and Threat Review

AGENTS.md Repo-Local Guidance, Bright Builds sidecar/overrides, architecture/code-shape/testing/verification/Rust standards, accepted07 summary and the two constructor/scope amendments materially informed execution. Both canonically distinct active lesson files were loaded completely:6898+1958=8856bytes and2300+653=2953conservative tokens. The existing2026-10-19 repo audit baseline applies; no audit trigger or project skill directory was found. The GSD execute-plan/summary contracts apply with explicit root ownership of shared metadata and saving.

The simplification pass retains the smallest robust construction change: one field in each existing literal, without helper, type, module, dependency, new test or production behavior. Exact byte recovery proves all original assertions, expected errors, faults and inputs are preserved. All five files stay below629lines without exceptions or broad formatting.

T-167-24-1 is mitigated by using only the explicit empty nonauthorizing collection; T-167-24-2 by exact byte recovery and the mandatory pending08 handoff. No network endpoint, auth path, new file-access behavior, schema change or other undeclared threat surface was introduced. Empty vectors are intentional missing-provenance fixture data, not unresolved stubs or authority/completeness claims.

## Deviations, Issues and Remaining Work

No implementation deviation, failed formatting check, behavioral failure, auth gate or retry occurred. Initial optional-path rg discovery returned exit2 for absent paths; some oversized displayed context reads were truncated and the critical plan/amendment/codec contract sections were re-read in smaller sections. Neither is a product failure or test evidence.

No additional owner was needed. All parallel22 changes and prior accepted work are preserved. No source guard, ownership hash, breadcrumb manifest, corpus, frozen receipt, plan, STATE, ROADMAP, requirements, config or todo edit was made by24. Requirement frontmatter describes only this bounded CVLD-06 contribution; whole-phase closure remains pending. Root owns shared-state updates and normal-hook Git saving after acceptance.

The mechanical handoff is ready for root review; this executor stops before08. Paired node compilation/full regression, durable codec/reopen qualification, and final phase acceptance remain pending. No Phase168 work started.

## Performance Attribution

One task; five actual Rust source owners. Snapshot timestamp2026-10-21T21:29:42.805893Z precedes the edits. Mechanical check observation2026-10-21T21:30:14.010709Z yields31.205seconds, recorded as31s in frontmatter. This measures current implementation/source checks, excluding earlier context loading and subsequent summary validation; no predecessor duration is added.

## Self-Check: PASSED

- All five declared Rust owners and this summary exist; no extra Rust owner was changed by24.
- Five added fields are inside the five inventoried direct constructors; removing them recovers all five snapshots byte for byte. Every current file is below629lines.
- Scoped rustfmt and whitespace checks passed; all five actual source links verify.
- gsd-tools verify-summary --check-count5 reports passed:true,5checked/5found and errors:[]; frontmatter validation reports valid:true,missing:[]. The commits_exist check resolves only the explicitly labelled preexisting checkout baseline, never a24task commit.
- No test/compilation pass is asserted;08 paired full node regression remains pending. commits:false; root acceptance and shared saving remain separate.
