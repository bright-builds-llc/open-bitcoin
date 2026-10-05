---
phase: 156-index-owned-manual-and-automatic-prune-coordination
plan: "08"
subsystem: verification
tags: [basic-filter, prune-protection, parity, claims, bun, mutation-testing]
requires:
  - phase: 156-07
    provides: Recorded production runtime, authenticated ownership and legal-target ordinary daemon evidence
provides:
  - Truthful CFPR-01 parity roots, compatibility and scoped evidence limits
  - Mutation-tested ordinary production-body, behavioral-test, provenance and claim guard
  - Default native verifier integration after the Phase 155 stages
affects: [156-verification, 157, 158, 159, 160, 161, 162]
tech-stack:
  added: []
  patterns: [masked narrow Rust body contracts, independent parity string claims, scoped snapshot mutation fixtures]
key-files:
  created:
    - scripts/check-phase156-prune-coordination.ts
    - scripts/check-phase156-prune-coordination.test.ts
    - scripts/check-phase156-prune-coordination/contracts.ts
    - scripts/check-phase156-prune-coordination/rust-evidence.ts
  modified:
    - docs/parity/index.json
    - docs/parity/catalog/basic-compact-filters.md
    - docs/parity/checklist.md
    - README.md
    - scripts/verify.sh
    - .planning/phases/156-index-owned-manual-and-automatic-prune-coordination/156-08-PLAN.md
key-decisions:
  - Keep CFPR-01 in_progress and localize full native and formal proof claims as pending root gates.
  - Inspect narrow ordinary production bodies with comments, literals and test-only modules masked; registered behavioral tests remain supplementary source evidence.
  - Check each parity string field and subject clause independently so unrelated deferred prose cannot authorize a positive current product claim.
patterns-established:
  - Required fixed upstream test roots must exist in the pinned corpus, not merely appear in the ledger.
  - Qualified cfg(test) module visibility cannot substitute for a missing ordinary caller.
requirements-completed: [CFPR-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 156-2026-10-04T20-28-36
generated_at: 2026-10-05T02:16:00Z
commits: []
git_finalization: pending consolidated root commit after clean whole-phase verification and full native gate
duration: 17min from first owned file creation to final scoped checks
completed: 2026-10-05
---

# Phase 156 Plan 08: Scoped Parity and Native Claim Guard Summary

**Internal CFPR-01 ownership parity with 84 mutation controls for actual source boundaries, named behavioral evidence and truthful current claims.**

## Performance and Scope

- First owned test file creation: 2026-10-05T01:59:16.084Z; final scoped checks observed at 2026-10-05T02:16:00.608Z. This measures the implementation interval, not earlier read-only preparation.
- Tasks completed: 3/3; all Git finalization and whole-phase proof remain root-owned.
- Four TypeScript files created; five existing docs/verifier files changed; the approved Plan 08 file list was updated separately. This summary is separate.
- Both active lesson files read completely: 7,188 bytes and 2,397 conservative estimated tokens. No active lesson was omitted or audit trigger applied.
- No Cargo, Bazel, full native verifier, commit, push, hook bypass, shared STATE/ROADMAP/REQUIREMENTS/config update or requirement activation was performed.

## Accomplishments

- Added the unique `v2-5-index-owned-manual-and-automatic-prune-coordination` parity owner for CFPR-01 with `in_progress` status. It links actual implementation/tests, exact pinned source roots, checker modules, runtime summary and shutdown repair evidence. Existing relevant README/catalog/checklist now distinguish implemented internal ownership from later public products.
- Documented exact reserved CRUD/map preservation, direct heights 0/1, additive 11-byte owner compatibility, store-bound generation/frontier and record-only fencing, current recovered-coins/ancestry candidate binding, atomic proof-before-release, ordered disable/re-enable, conservative faults, immutable retention and same-second automatic invalidation.
- Preserved Fjall paired-key logical accounting versus Knots file pruning, all three v2.4 advisories, actual daemon accounting and fixture provenance limits. The small engine controls use explicit coinbase maturity one and synthetic supplied work; large codec-valid daemon/deletion histories are not presented as accepted full daemon chains.
- Added 25 threat-indexed production/test contracts, additional concrete boundary/fault/reopen checks, selected upstream corpus roots and in-source/manifest breadcrumb equality. Ordinary function bodies mask comments, normal/raw literals and private/public/qualified test-only modules. Named executable test functions, assertions or a resolved local assertion helper and module registrations are required.
- Added explicit-file mutation tests and live guard steps immediately after the existing Phase 155 default verifier stages, retaining all existing native gates. Help output lists both phases consistently.

## Task Finalization Records

The strict wrapper defers every task/TDD/metadata commit to consolidated root finalization.

1. Task 1: actual scoped parity/docs and pending final proof published; inherited Phase 155 live guard and 53/53 mutation controls pass.
1. Task 2: local RED/GREEN, initial positive-fixture refinements and reviewer-requested regression controls complete; final Phase 156 suite passes 84/84 with 151 assertions.
1. Task 3: default verifier wiring, live guard, source provenance, managed checks, shell syntax and owned diff review complete. Existing source mappings already cover all 965 Rust files; this plan adds no Rust source and makes no breadcrumb-manifest edit.

Only the four exact new TypeScript paths received authorized `git add -N` for managed tracked-source discovery. No actual staging or commits were performed. Root retains final LOC freshness, lifecycle/requirement activation, clean review/native gates and the commit hook.

## Verification

Bun 1.3.9 was selected from `/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64` and reprobed. No dependency installation or new dependency was used.

| Check                       | Exact command                                                                                            | Evidence                                                                                                                                                                                                                               |
| --------------------------- | -------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Initial TDD RED             | `bun test ./scripts/check-phase156-prune-coordination.test.ts`                                           | Expected missing checker import: 0 pass, 1 fail, 1 error.                                                                                                                                                                              |
| First complete fixture run  | `bun test ./scripts/check-phase156-prune-coordination.test.ts`                                           | 72 pass, 2 positive-fixture failures; corrected checker assumptions described below.                                                                                                                                                   |
| Mixed-claim regression RED  | `bun test ./scripts/check-phase156-prune-coordination.test.ts -t 'cannot borrow'`                        | 1 pass, 1 fail: mixed positive/deferred subjects reproduced the exemption bug. The exact rationale case already rejected incidentally because serialized punctuation split another field; this did not prove independent field safety. |
| Qualified test-module RED   | `bun test ./scripts/check-phase156-prune-coordination.test.ts -t 'qualified test-only module'`           | 0 pass, 3 fail for pub(crate), pub(super) and pub(in crate::storage) substitutes.                                                                                                                                                      |
| Qualified test-module GREEN | Same focused command                                                                                     | 3/3 pass after matching Rust visibility qualifiers.                                                                                                                                                                                    |
| Final mutation GREEN        | `bun test ./scripts/check-phase156-prune-coordination.test.ts`                                           | 84/84 pass, 151 assertions, 6.02s. Includes both review claim cases, actual baseline test-root removal, qualified module substitutes and earlier controls.                                                                             |
| Existing mutation controls  | `bun test ./scripts/check-phase155-filter-index.test.ts`                                                 | 53/53 pass, 93 assertions, 533ms.                                                                                                                                                                                                      |
| Live scoped guards          | `bun run scripts/check-phase156-prune-coordination.ts`; `bun run scripts/check-phase155-filter-index.ts` | Both pass. Source/claim links only; no runtime execution or full-native result inferred.                                                                                                                                               |
| Source provenance           | `bun run scripts/check-parity-breadcrumbs.ts --check`                                                    | Passed for 965 first-party Rust files.                                                                                                                                                                                                 |
| Managed checks              | `bun scripts/bright-builds-check.ts all`                                                                 | Passed after exact TypeScript intent-to-add: 1,288 source files, active repository lesson structure, zero findings.                                                                                                                    |
| Shell and diff              | `bash -n scripts/verify.sh`; `git diff --check`                                                          | Passed. Owned changes reviewed; no dependency, Rust behavior, target or generated-file change by this plan.                                                                                                                            |

README/checklist Markdown use the installed explicit GFM/frontmatter extensions with compact tables to preserve existing compact rows. Catalog/plan checks use the same explicit extensions with their existing padded-table layout. The new checklist was checked before its scoped formatter write; final checks passed. Summary formatting/self-check is recorded below.

## Recorded Runtime Evidence and Limits

This plan ran no new Rust tests, fault injection, daemon activity or close/reopen. It links prior evidence rather than inventing observations. Plan 07 records node prune 129/129, authenticated RPC prune 38/38, legal daemon 1/1 and inherited daemon 5/5; root's shutdown repair independently records the filter-index 102/102 pass, 31 completed exact fault/reopen invocations and successful Bazel dependency resolution. Resolution is not a Bazel build or full native pass.

The ordinary daemon retains 578,571,326 bytes initially and 578,420,026 after 714 genuine paired deletions. Its 235 nonactive bulk pairs retain 578,358,970 bytes, keeping the unchanged 550 MiB target unattainable. Software commit/reply faults distinguish old conservative proof from newly committed state after real drop/reopen. The official dependency pin fixes the observed shutdown boundary; format-marker observations are not exhaustive migration or hardware resilience proof.

Public activation/options, scheduled catch-up, runtime reorg, filter/index RPC, peers, operator projections, complete client-after-prune proof, V0/BIP37, GUI, production readiness and production-funds claims remain deferred. No activation CLI, public defaults, scheduler or worker join/stop proof was added. Source checks are narrow deterministic regression guards, not a Rust compiler, semantic safety proof or replacement for the root native gate.

## Decision, Criterion and Threat Coverage

D-01/D-02 map to reserved CRUD and exact guarded map preservation; D-03 to current snapshot/direct/resumed candidate gates; D-04 to fresh automatic identity and honest total usage; D-05/D-06 to complete fenced publication and conservative fault/reopen proof; D-07/D-08 to ordered durable lifecycle and stale-work refusal; D-09/D-10 to named runtime evidence, scoped documentation, provenance and native wiring. All four roadmap criteria link to Plan 07's named evidence; this plan does not independently re-execute those criteria.

| Threats                                | Guard and evidence routing                                                                                                                                                                                                     |
| -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| T-156-01, T-156-02, T-156-03           | Exact bounded codec, checked generations/frontier, genuine artifact absence/legacy Active contracts and named pure/codec controls.                                                                                             |
| T-156-04, T-156-05, T-156-06, T-156-07 | Record-only capability gate, complete atomic release proof, lifecycle-aware guarded recovery and poisoned failure/reopen evidence.                                                                                             |
| T-156-08, T-156-09, T-156-10, T-156-11 | Ordered disable/release, historical preflight, real transition fault controls and trusted managed-authority host calls.                                                                                                        |
| T-156-12, T-156-13, T-156-14           | Authoritative reserved refusal, exact guarded map preservation and authenticated-before-parsing HTTP evidence.                                                                                                                 |
| T-156-15, T-156-16, T-156-17, T-156-18 | Guarded concrete deletion, fresh application, current recovered-coins/saved-fence/candidate ancestry and earned-only cleanup.                                                                                                  |
| T-156-19, T-156-20, T-156-21           | Fresh identity before both measurement gates, cloned-store visibility and required candidate exclusion with unchanged actual total.                                                                                            |
| T-156-22, T-156-23, T-156-24, T-156-25 | Direct/resumed low inputs, actual legal-target ordinary flush evidence, authenticated/direct bypass controls and explicit fixture/software/hardware limits.                                                                    |
| T-156-26                               | Current docs and independent parity string/subject checks reject unsupported claims while preserving explicit future/historical exclusions; eleven positive-product claim mutations plus mixed-clause/rationale controls pass. |
| T-156-27                               | Executable ordered default steps, absent source/test/provenance rejection and bounded CLI diagnostics; removal of either default Phase 156 stage refuses.                                                                      |

Each T-156-01 through T-156-25 contract has separate production-removal and named-test-removal mutation cases. T-156-26/T-156-27 have the scoped claim/default-stage mutations above. These strengthen structural regression coverage; independent whole-phase security closure remains root-owned. No new production endpoint, auth path, schema or file-access trust boundary was introduced by this docs/checker plan.

## Simplification Review

One declarative contract inventory drives live checks and one-concern mutation cases; a small masking/body helper prevents comments, diagnostics and test modules from replacing ordinary code. The checker reuses snapshot fixture conventions and has no parser package or dependency. It adds no runtime ownership model, full-index scanner, synchronization authority, accounting override or paired-delete owner. Fixed selected upstream files avoid a broad corpus scan. Diagnostics report bounded paths/categories and at most 40 individual CLI failures without dumping filter/script payloads.

## Deviations and Issues Encountered

- Root removed the nonexistent node crate README from plan scope; only existing relevant docs were updated. Root approved two small checker children and instructed their explicit addition to Plan 08/frontmatter/task files.
- First positive-fixture validation found six checker assumptions: the real helper uses equality for reserved refusal, the low-height fixture uses usize, one test delegates to a local assertion helper, a central old registry has no required roots, and the software caveat starts uppercase. Corrected these assumptions without weakening actual selected production/test provenance; final positive and negative cases pass.
- Supplemental review found unrelated deferred prose could mask a mixed positive current claim. The regression was reproduced before independent string-field/subject checks; both requested controls pass afterward. The rationale test's incidental initial pass is recorded rather than presented as a reproduced failure.
- Supplemental review found the cited `feature_blockfilterindex.py` does not exist in the pinned corpus. Replaced it in the new and approved existing Phase 155 ledger rows with the inspected `feature_index_prune.py`; both selected functional roots now must exist and carry concrete pruning anchors. No upstream functional suite was run.
- Supplemental review found qualified test-only module visibility bypassed the initial masker. Three RED cases reproduced it; the narrow visibility matcher fixes all three. Earlier comment/normal/raw-string and unqualified-module controls remain passing.
- No known scoped issue is deferred. Current `in_progress`/pending-native prose remains deliberately localized for root reconciliation after actual formal/native evidence. State, requirement activation, generated LOC, final native validation and Git finalization remain root gates.

## Known Stubs

None introduced. The owned TODO/FIXME/placeholder/unimplemented scan found no stub. Empty maps/arrays and temporary mutation files are explicit fixture/source-check arrangements rather than product data or fake runtime proof.

## Self-Check: PASSED

All four created TypeScript files and this summary exist. Final Phase 156 84/84 and inherited Phase 155 53/53 mutation suites passed with nonzero counts; both live guards, 965-file provenance, tracked-source managed checks, shell syntax and whitespace review passed. Owned Markdown and this summary were checked with explicit installed GFM/frontmatter extensions, scoped formatter writes and final checks. The source remained unchanged after the final 84-test run. No commit-existence claim applies: commits are empty, requirement completion is empty and consolidated root Git/native/lifecycle finalization remains pending.

## Root Requirement Activation | 2026-10-05 UTC

Root activated CFPR-01 only after the full default native verifier exited zero,
27/27 security mitigations closed, formal verification passed 28/28 must-haves and
all four roadmap criteria, and report-inclusive lifecycle validation returned valid.
Current parity/docs now link to the earned formal/native evidence. Phase 157 stays
pending; consolidated Git finalization is derived from the saving commit/upstream.
