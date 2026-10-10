---
phase: 159-authenticated-basic-filter-and-index-rpcs
plan: "08"
subsystem: verification
tags: [rust, bun, knots, parity, authenticated-rpc, native-guards]
requires:
  - phase: 159-07
    provides: Genuine configured daemon, retained-prune/reopen and composed private-fault evidence
provides:
  - Exact source breadcrumb and scoped CFRP-01/02 parity registration
  - Mutation-sensitive source/action/assertion/owning-configuration guard and native routing
  - Successful full default-native receipt and independent source-review handoff
affects: [160, 161, 162, phase-verification]
tech-stack:
  added: []
  patterns: [canonical Git worktree inventory, fixed executable-evidence contracts, explicit CLI binary test execution]
key-files:
  created:
    - scripts/check-phase159-filter-rpcs.ts
    - scripts/check-phase159-filter-rpcs.test.ts
    - scripts/check-phase159-filter-rpcs/contracts.ts
    - scripts/check-phase159-filter-rpcs/evidence.ts
    - scripts/check-parity-breadcrumbs.test.ts
    - .planning/phases/159-authenticated-basic-filter-and-index-rpcs/159-08-CHECKPOINT.md
    - .planning/phases/159-authenticated-basic-filter-and-index-rpcs/159-08-SUMMARY.md
  modified:
    - docs/parity/source-breadcrumbs.json
    - docs/parity/index.json
    - docs/parity/catalog/basic-compact-filters.md
    - docs/parity/deviations-and-unknowns.md
    - docs/parity/catalog/rpc-cli-config.md
    - README.md
    - packages/README.md
    - scripts/check-parity-breadcrumbs.ts
    - scripts/check-phase157-index-catch-up/contracts.ts
    - scripts/check-phase158-validated-reorg.ts
    - scripts/check-current-documentation-reconciliation.ts
    - scripts/check-current-documentation-reconciliation.test.ts
    - scripts/verify.sh
    - scripts/check-phase134-authoritative-lifecycle.test/mutations.ts
    - docs/metrics/lines-of-code.md
key-decisions:
  - Preserve UnknownLegacy absence and numeric resource limits as explicit compatibility differences.
  - Require actual actions, meaningful assertions and supported declaration/module ownership for fixed selectors.
  - Share cached-plus-nonignored-untracked inventory and preserve ignored tracked files and Git links.
  - Execute the test=false CLI binary explicitly after workspace tests.
  - Authenticate retained ledger identity before comparing raw metadata; keep accepted identity construction strict.
  - Keep formal security/lifecycle/requirement activation and consolidated Git finalization root-owned.
requirements-addressed: [CFRP-01, CFRP-02]
requirements-completed: [CFRP-01, CFRP-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 159-2026-10-09T16-11-49
generated_at: "2026-10-10T00:13:13Z"
duration: "36m 0.155s final native gate; executor-active time not precisely recorded"
completed: 2026-10-10
---

# Phase 159 Plan 08: Auditable BASIC RPC Parity and Native Guards Summary

**Exact BASIC RPC parity registration and a 611-test mutation guard now connect genuine shared-authority evidence to a successful full default native gate.**

## Completion and Ownership

Tasks 1/2 and the Task 3 native/source-review handoff are implemented and verified.
The root's actual default native attempt 4 exited 0 in **36m 0.155s**. This SUMMARY
enables the root's eight-of-eight lifecycle preverification; it does not activate
requirements or claim the subsequent formal security/goal/lifecycle/Git gates.
Requirements-completed was empty at this initial handoff as required by the parent; the later root activation is recorded below.

The final native receipt is [159-NATIVE-CHECKS.md](159-NATIVE-CHECKS.md);
[159-REVIEW.md](159-REVIEW.md) is independently clean across **91 paths**, with
all **seven** findings closed. The [checkpoint](159-08-CHECKPOINT.md) preserves
each partial receipt, failure, repair and handoff rather than counting failed or
host-aborted attempts as passes. Precise executor-active time across these
coordinated sessions was not captured; the duration above is the measured final
native gate, not a fabricated total work duration.

Material guidance: repository AGENTS.md, Bright Builds sidecar, placeholder-only
overrides, architecture/code-shape/testing/verification/Rust/TypeScript standards,
approved CONTEXT/RESEARCH/nonvisual UI-SPEC/plan checks, 01–07 SUMMARY key-files,
actual source inventories, 04 measurements and 07 daemon proof. Both active lesson
files were fully loaded: 7,188 bytes/2,397 estimated tokens; no audit trigger.
Root supplied synchronized main, materialized pin and serialized target access.

## Accomplishments

- Registered all **34 new Rust paths** with exact ordered Knots breadcrumb
  anchors, including the canonical runtime_state/tests.rs rename. The official
  current-worktree check verifies **1,055 Rust files** without premature staging.
- Added one scoped CFRP-01/02 parity owner and documented actual request/result/
  error behavior, initial latch versus processed/safe progress, captured-frontier
  completion, genuine acceptance, retained active/stale/pruned/missing lookup,
  explicit legacy uncertainty and numeric resource limits. All historical
  owners and later-phase exclusions remain.
- Added one focused Bun checker with coherent contracts/evidence children.
  It requires **125 fixed Rust selectors**, meaningful action/assertion
  connections and **73 module-owner edges across 78 files**. False/excluding cfg,
  conditional ignore, literal/tautological no-ops, lost production wiring,
  unearned provenance/readiness, raw coverage laundering, weak parent parsing,
  fake prune/reopen evidence and unsupported claims are mutation-tested.
- Reused the canonical cached+nonignored-untracked Git inventory, retaining
  ignored tracked paths and avoiding recursive Git-link/ignored-source traversal.
  New real-Git inventory controls run before the existing breadcrumb check.
- Preserved historical verifier order, placing Phase 159 directly after 158.
  Added the whole hermetic CLI binary test stage immediately after workspace
  tests because Cargo's test=false binary is otherwise omitted. Removal,
  no-run/list/empty selectors and broken CLI conversion/auth/node routes reject.
- Reconciled current RPC catalog/checker counts to **29 = 21 baseline + 8 unchanged
  extensions**, retaining exact sets/grouping, all 18 existing tests and two
  independent new-method-removal controls. Root/packages READMEs link the scoped
  contract and exact repo-local Cargo/Bazel UAT, including a build/start recipe
  that avoids holding the cooperative build lock during interactive clients.
- Followed moved 157 connect/158 reorg implementations without weakening their
  original acceptance/preflight/mempool/fence contracts. Maintenance checks still
  require first-failure retention, both final flush/stop outcomes and real joins.
  Root's narrow Phase134 mutation-marker repair retains all original forbidden
  I/O injections and diagnostics.

## Final Native Evidence

The root ran unchanged default **bash scripts/verify.sh**, with pinned
Bun 1.3.9/Rust 1.94.1 and serialized Cargo/Bazel work. Session 19945 exited 0;
raw ignored receipt: packages/target/phase159-native-4.log.

| Gate                                     | Actual final result                                                                                                                                     |
| ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Full default contract                    | **Exit 0; 36m 0.155s; 2,160,155ms**                                                                                                                     |
| Phase 159 mutation suite                 | **611 passed**, 0 failed; **1,816 assertions**; 305.92s                                                                                                 |
| Fixed evidence/ownership                 | 125 Rust selectors; 73 owner edges/78 files                                                                                                             |
| Canonical inventory/breadcrumbs          | 3 inventory tests passed; 1,055 Rust files verified                                                                                                     |
| Historical/current documentation guards  | Passed, including 257 Phase134, 1,057 Phase157/158 and 20 reconciliation controls                                                                       |
| Standards/architecture/format/lint/build | Managed checks, dependency/file-length/panic checks, workspace formatting, strict all-target/all-feature Clippy and all-target/all-feature build passed |
| Primary workspace tests/doctests         | **3,857 passed**, 0 failed; 3 existing opt-in ignores                                                                                                   |
| Node within workspace                    | **1,429 passed**, 0 failed; 3 existing opt-in ignores; 615.09s                                                                                          |
| Explicit whole CLI binary                | **7 passed**, 0 failed; 0.02s, including all 3 new client controls                                                                                      |
| Combined primary Rust passes             | **3,864** = workspace 3,857 + explicit CLI 7                                                                                                            |
| Benchmarks/Bazel/provenance              | List/smoke/report passed; all 6 Bazel smoke targets and provenance passed                                                                               |
| Configured pure-core coverage            | llvm-cov test/coverage gate passed; **0 uncovered lines**                                                                                               |
| Generated LOC                            | Root regenerated tracked report: 398,783 lines counted                                                                                                  |
| Independent source review                | 91 paths, 0 open findings; all 7 findings closed                                                                                                        |

Counts overlap where indicated. Node and CLI library tests are already included
in workspace 3,857. Coverage reruns and the separate exact diagnostic regression
probe are excluded from 3,864 primary passes. The source guard checks named
evidence; it does not execute those Rust selectors itself or prove parity alone.
Earlier recovery 4 independently passed the same 3,857+7 primary Rust tests; those
are corroborating receipts, not additional unique passes.

No fast mode, skipped check, hook bypass, overlapping build, new production
dependency or host-security mutation supplied this final result.

## Focused Mutation and Review Evidence

- Initial source/claim TDD saw the absent checker import and then actual missing
  documented/executed Phase 159 routes. Wiring restored GREEN; fixture mistakes
  are not credited as semantic runtime RED.
- WR-04/05/06 reproduced 27 guard bypasses: excluding owning cfg/cfg_attr,
  constant/self-equality substitutions and ignored-source traversal. All 27
  failed RED, then passed GREEN. Expanded 225 review controls and the then-current
  full 555 suite passed; final 611 above supersedes their old scope.
- Every fixed selector has no-op/ignore/action-removal controls. Module/path/
  declaration checks preserve canonical cfg(test) and the existing non-disabling
  crate lint policy while rejecting unsupported exclusions. Arbitrary inner-body
  attributes are not treated as declaration ownership.
- CLI extension reproduced 6 source/native wiring gaps; the exact-main-route
  mutation exposed and closed a substring weakness. Final focused 41 controls
  passed before the actual native 3 full 594 and final native 4 full 611 runs.
- Metadata follow-up requires authentic ScriptsValid hash/parent/height evidence,
  propagating ledger decode errors and dropping the history guard before durable
  invalidation. Two guard-only RED controls reject reintroduced strict raw DTO
  parsing and weakened accepted identity construction. Focused 18 checks plus a
  distinct append-owner control passed; both new Rust regressions passed in
  recovery 4 and default native 4.
- All current guard files remain below 628 lines. The simplification pass shares
  one canonical Git inventory and separates fixed contracts from executable
  ownership/assertion inspection; no general Rust interpreter/dataflow proof is
  claimed. Real Rust/native behavior remains the final execution evidence.

## Scoped Compatibility and Resource Limits

Only CFRP-01/02 are addressed. UnknownLegacy absence returns fixed unavailable
-32603 because old storage cannot prove whether an absent row was historically
accepted; valid retained records still serve. No inferred legacy backfill,
request-side regeneration, repair/download or active-membership/body-based
acceptance authority is introduced.

Queries fully parse at most **1–2 filter records**, plus bounded lifecycle/
identity/provenance work: **33,554,602 bytes per envelope**, **67,109,204 combined**,
and **67,108,928 filter/header hex characters**. Logical-copy/SHA-padding ceilings
are separate (**100,663,648 /67,109,696 bytes**); these are not whole-HTTP,
allocator-capacity, RSS or latency guarantees. The genuine approximately 1 MB
block generates 7 filter bytes; the 32 MiB capacity fixture is synthetic codec
evidence. Pending waiters are bounded at 64, retaining original request/hash/
provenance and captured frontier through cancellation or terminal settlement.

The genuine configured daemon proof remains **402 original accepts + 11 genuine
replacements**, with one real height 20 paired body/undo loss of **1,126 logical
bytes**, 451,352→450,226. Exact active/stale/pruned/missing responses survive
all-handle configured reopen. Synthetic genesis, maturity 1 and manual owner-plan
deletion outside the trailing 288 window do not prove ordinary prune-RPC/default
automatic threshold 1000 eligibility. Private cfg(test) node faults compose with
actual production RPC projection; they were not all injected through daemon
HTTP. Group B storage fixtures prove real recovery/append refusal, not independent
consensus acceptance; Plan 07 owns the distinct genuine chain/daemon evidence.

Default-off activation and production dependencies are unchanged. CFPR-02,
CFNET, CFOP, CFGR, V0 generation, new operator/peer surfaces, v2.5 completion,
public defaults, production/funds and hardware power-loss claims remain excluded.

## Files and Task Commits

The frontmatter records 20 executor-owned source/doc/test/planning artifacts and
two root-produced closeout paths. No first-party Rust file was edited by this
executor. CLI, raw-metadata, fixture and source-structure repairs belong to their
Plan 02/03/04/root owners and are dependencies of the final native handoff.

1. **Task 1: Source parity and scoped documentation** — pending root consolidated commit.
1. **Task 2: Guard, mutation suite, native routing and repo-local UAT** — pending root consolidated commit.
1. **Task 3: Native/source-review evidence and final artifact handoff** — pending root consolidated commit.

No task/metadata commit hash is claimed. All staging, commit, push, STATE,
ROADMAP, REQUIREMENTS, todo and config changes remain root-owned; no hook was
bypassed.

## Deviations from Plan

- **Rule3, pre-staging inventory:** tracked-only breadcrumb discovery made new
  manifest groups unused before staging. Root approved cached+untracked/
  exclude-standard discovery and real manifested/unmapped/ignored controls.
  Two old RPC direct-child globs became exact existing paths to avoid overlap
  with narrower new entry-module anchors.
- \*\*Rule3, actual historical routes:\*\*157/158 contracts/parity links followed
  moved implementations; 157 maintenance assertions followed real first-error
  and flush/stop settlement. Root repaired only the old Phase134 insertion
  marker/newline, preserving all five forbidden I/O mutations.
- **Rule1/3, checker closeout:** WR-04–WR-06 closed compiled-out selector,
  constant no-op and ignored-source bugs with real RED/GREEN and negative
  controls; no broad scope bypass or managed checker edit occurred.
- **Rule3, catalog and binary integration:** native 2 exposed stale 27-count/
  missing-method docs; root Rust preflight exposed real CLI E0004. The owning
  executor fixed conversion; Plan08 added six actual selectors, exact binary
  ownership and explicit native binary execution.
- **Rule1/3, native 3 recovery follow-up:** raw sparse DTOs were incorrectly
  parsed as strict accepted identities; Group A restored authentic-ledger-first
  comparison and durable uncertainty. Group B earned genuine fixture recovery
  and preserved corruption/proof refusal; root fixed exact moved-producer
  source assertions. The guard now requires the repaired contract while keeping
  accepted identity construction strict.

These changes directly close the phase's verification/compatibility boundaries.
The existing source-sensitive checks, immutable authority and later-phase
exclusions remain intact. No architectural/dependency/default expansion occurred.

## Failed and Host-Aborted Attempts

- Native 1 exited 1 before Cargo after 7m 30.555s at five stale Phase134 insertion
  preconditions; focused repair 257 passed.
- Native 2 exited 1 before Cargo after 7m 21.043s at current-documentation
  reconciliation (16/18 passed); corrected exact count/catalog 20 passed.
- Real preflight E0004 identified the missing CLI match arms. Distinct pre-main
  loader stalls showed dyld-only samples/zero CPU and were recorded as diagnostic
  aborts where the root terminated its owned child. Scoped Cargo cleanup and
  retries were root-owned; no signature/host-security change or timeout-as-pass
  inference occurred.
- Native 3 exited 101 after 1h 5m 5.214s with Node 1,417 passes/10 failures/3 ignores.
  Recovery 1's needless-borrow Clippy finding, recovery 2's remaining branch-binding
  fixture failure and recovery 3's extra-turn fixture setup failure are retained
  in the native report. The final recovery 4 and full native 4 passed; failed and
  partial counts are never promoted into the final primary total.

See the native report/checkpoint for exact boundaries, logs and receipt details.
No authentication gate or external credential provisioning was needed.

## Known Stubs and Threat Review

No unresolved product stub, placeholder result, raw acceptance factory, fake
clean epoch or successful-empty substitute prevents the implemented goal.
This plan adds repo-owned verification/file inventory rather than new production
RPC/auth/schema/dependency surfaces. Declared T-159-26/27 cover claim/native
evidence and exact breadcrumb integrity; no additional unregistered surface was
identified. Independent security closeout remains root/auditor-owned, not waived.

## Root Gates at Initial Handoff

Root must now close/update SECURITY evidence, perform formal phase goal
verification and release lifecycle validation against all eight summaries,
activate only CFRP-01/02 when proven, reconcile root trackers/generated freshness,
and perform the authorized consolidated commit/push. This SUMMARY records actual
implementation/native success while leaving requirements-completed empty at the initial handoff until
that formal root proof. Phase 160–162 and the v2.5 milestone remain separate.

## Self-Check: PASSED

All 22 recorded key paths exist, including this SUMMARY and the retained
checkpoint. The saved native receipt explicitly reports status passed and
default exit 0; full 611 guard and 3,864 primary Rust results are recorded without
adding overlapping subsets. Lifecycle ID/mode match Plan 08, exactly two
frontmatter delimiters are present; requirements-completed was empty at this initial-handoff self-check. Root formal activation is recorded below.
No commit hash, formal requirement activation, SECURED result or final lifecycle
approval is fabricated; those final gates remain root-owned.

## Root Formal Activation

After the initial implementation/native handoff, independent security closed27/27, formal goal verification passed26/26 and the root full lifecycle check returned valid for8plans/8summaries/current yolo attempt. CFRP-01/02 are now formally activated; other requirement owners remain pending. Consolidated commit/hook/push evidence is derived from actual Git commands and upstream refs.
