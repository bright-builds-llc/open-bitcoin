---
phase: 159-authenticated-basic-filter-and-index-rpcs
plan: "08"
status: native-verified-root-formal-pending
tasks-focused-complete: [1, 2, 3]
tasks-pending: []
requirements-addressed: [CFRP-01, CFRP-02]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 159-2026-10-09T16-11-49
generated_at: "2026-10-10T00:04:40Z"
---

# Phase 159 Plan 08 Checkpoint

## Completed Focused Tasks

Tasks 1/2 have scoped implementation and verification evidence. Task 3 is the
explicit root-owned full native/source/security/lifecycle gate. Plan 08 is not
complete, no requirement is activated, and no native pass or commit is claimed.

Material guidance: repository AGENTS.md, Bright Builds sidecar, placeholder-only
overrides, architecture/code-shape/testing/verification/Rust/TypeScript standards,
approved CONTEXT/RESEARCH/UI-SPEC/plan checks, all 01–07 SUMMARY key-files, actual
changed/new Git source inventory, 04 measurements, 07 daemon proof and the early
01–06 review. Both active lesson files were loaded: 7,188 bytes, 2,397 estimated
tokens; no audit was needed. Root supplied synchronized main and pinned tools.

### Task1: Source Parity and Scoped Documentation

- Registered all **34 actual new Rust paths**, including untracked child modules
  and the later coverage/fault files, with exact existing ordered Knots anchors.
- Added one unique `in_progress` CFRP-01/02 parity owner with scoped source,
  query/daemon evidence and intentional validation-history/legacy/resource
  differences. Kept every historical phase owner and later-phase exclusion.
- Documented pinned normalization/results/errors, authentication, same-authority
  captured-frontier completion, initial latch versus processed/safe progress,
  64 pending waiters, 1–2 filter record reads plus bounded metadata/provenance,
  33,554,602 per-record/67,109,204 combined envelope bytes and 67,108,928 filter+
  header hex characters. Logical-copy/SHA-padding accounting is separate from
  allocator/RSS/whole-HTTP costs. Debug timings are not guarantees; 32 MiB is
  synthetic codec-capacity evidence and the genuine large block yields 7 bytes.
- Preserved UnknownLegacy absent-row unavailable behavior with valid retained
  rows still serving. No inferred backfill or raw-seeded acceptance claim.
- Recorded 402 original genuine accepts plus 11 replacements, real height 20 paired
  loss 1,126 logical bytes (451,352→450,226), live second-open refusal and all-handle
  configured reopen with exact active/stale/pruned/missing responses. Synthetic
  genesis/maturity 1/manual owner-plan/trailing 288 and unchanged automatic 1000
  limits are explicit; no ordinary prune-RPC/default-threshold proof is claimed.
- Distinguished private cfg(test) node query/owner faults composed with actual
  RPC projection from actual configured HTTP. No public test API/dependency or
  production/default activation changed. CFPR-02/CFNET/CFOP/CFGR/v2.5 and
  production/funds claims remain excluded.

### Task2: Mutation Guard, Native Routing and Contributor Commands

- Added one focused checker with a fixed contracts/inventory child module. It
  checks actual function bodies with comments/literals/test-only code masked,
  ordered source connections, same-store accepted receipt/storage controls,
  auth-before-parse, release-before-await, same-handle original-hash completion,
  captured provenance/frontier, full parent validation, numeric bounds, raw
  coverage guards and direct dispatcher poison settlement.
- Fixed **117 named Rust selectors** are required to remain executable,
  nonignored and nonempty. Meaningful independent mutations cover detached/lost
  wiring, acceptance/fence/readiness/provenance corruption, real paired deletion,
  body absence, live-open refusal/all-handle reopen, private faults, scope claims,
  parser/type aggregation/hash+header order and byte policy. Every selector has
  ignored and no-op mutations; a trivial assert cannot replace behavior.
- Inserted 159 test/check immediately after 158 in both documented and executed
  verifier routes. Every historical command remains ordered; the verifier also
  runs the new breadcrumb inventory regression before its existing check.
- Updated the root/packages READMEs with scoped current facts and catalog links.
  Cargo/Bazel daemon/client targets and arguments are verified against actual
  BUILD files and client/config source. The catalog gives exact repo-local
  commands and a build-then-start recipe avoiding a long daemon holding the
  cooperative build lock while another timed build/client waits.

## Initial Focused Evidence Before WR-04–WR-06

All commands used `PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH`. This executor
ran no Cargo/Bazel command or full native verifier and took no shared target slot.

| Check                                                                                                       | Positive result                                                                                                                                                                                                                                                                                                           |
| ----------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bun test ./scripts/check-phase159-filter-rpcs.test.ts`                                                     | **330 passed**, 0 failed; 658 assertions; 55.12s.                                                                                                                                                                                                                                                                         |
| `bun scripts/check-phase159-filter-rpcs.ts`                                                                 | Pass;117 fixed named Rust selectors and actual source/evidence/claims/routes. Rechecked after final UAT wording.                                                                                                                                                                                                          |
| `bun test ./scripts/check-parity-breadcrumbs.test.ts`                                                       | **3 passed**, 0 failed; 12 assertions. Real temp Git repos prove untracked manifested checked, untracked unmapped refused and ignored source excluded.                                                                                                                                                                    |
| `bun scripts/check-parity-breadcrumbs.ts --check`                                                           | Pass;**1,055 Rust files**, including all 34 untracked new paths.                                                                                                                                                                                                                                                          |
| `bun test ./scripts/check-phase157-index-catch-up.test.ts ./scripts/check-phase158-validated-reorg.test.ts` | **1,057 passed**, 0 failed; 2,539 assertions; 92.15s. Includes current source-path mutations, maintenance first-failure/flush/stop ordering and existing real shutdown/join protections.                                                                                                                                  |
| Historical154–158 CLI checks                                                                                | All pass after narrow source-location reconciliation.                                                                                                                                                                                                                                                                     |
| `bun scripts/bright-builds-check.ts all`                                                                    | 0 findings; 1,351 tracked source files scanned. All new script files individually below 628 lines.                                                                                                                                                                                                                        |
| `bash -n scripts/verify.sh`, `shfmt -i 2 -l -d scripts/verify.sh`                                           | Pass, no diff.                                                                                                                                                                                                                                                                                                            |
| Compatible mdformat GFM/frontmatter check                                                                   | Catalog and packages README pass. Extracted changed sections of root README and deviations also pass. Whole-file pre-existing diagnostics are exactly `File /tmp/phase159-baseline-readme.md is not formatted` and `File /tmp/phase159-baseline-deviations.md is not formatted` against HEAD; no broad rewrite performed. |
| `git diff --check`, focused diff/simplification review                                                      | Pass. No broad formatting, suppressions or dependency changes.                                                                                                                                                                                                                                                            |

Initial TDD import failed because the checker did not exist. The subsequent
semantic RED rejected exactly the absent documented/executed 159 verifier routes;
adding those routes produced GREEN. Intermediate fixture/path-selector mistakes
are not runtime RED evidence. All mutation writes occur only in disposable temp
fixtures; no production mutation is left behind.

## Approved Narrow Deviations

1. **Rule3: Validate before staging.** The repo-owned breadcrumb checker previously
   used tracked-only `git ls-files`, so all 18 new mapping groups were falsely
   unused under the mandatory no-staging gate. Root approved cached+untracked
   `--exclude-standard` inventory and three regression cases. Rust scope and
   ignored behavior remain unchanged. No managed Bright Builds file was edited.
1. **Rule3: Exact mapping ownership.** Existing rpc-surface direct-child globs also
   matched the two new filter-index entry modules. Replaced only those two broad
   globs with their existing concrete file paths, preserving every old anchor and
   assigning the new modules their exact narrower anchors. Native checker passes.
1. **Rule3: Follow moved source methods.** Root approved 157's connect and 158's
   reorg contracts following `chainstate/validation_history.rs`; corresponding
   old parity evidence lists now include that exact implementation path. Original
   acceptance/preflight/mempool/fence assertions remain. 157's maintenance contract
   now follows first-error retention and both flush/stop results; generated
   negative mutations and existing all-worker/join controls pass. Negative scope
   claims remain enforced without a broad bypass.
1. **Rule3: Coherent checker module.** Fixed source/test/path inventory and ordered
   contracts live in a child module; orchestration and mutation tests stay focused.
   The new boundary selector handles array-type signature semicolons without
   masking the actual implementation body or changing the historical selector.

## Exact Owned Files

Created:

- `scripts/check-phase159-filter-rpcs.ts`
- `scripts/check-phase159-filter-rpcs.test.ts`
- `scripts/check-phase159-filter-rpcs/contracts.ts`
- `scripts/check-phase159-filter-rpcs/evidence.ts`
- `scripts/check-parity-breadcrumbs.test.ts`
- `.planning/phases/159-authenticated-basic-filter-and-index-rpcs/159-08-CHECKPOINT.md`

Modified:

- `docs/parity/source-breadcrumbs.json`
- `docs/parity/index.json`
- `docs/parity/catalog/basic-compact-filters.md`
- `docs/parity/deviations-and-unknowns.md`
- `docs/parity/catalog/rpc-cli-config.md`
- `README.md`
- `packages/README.md`
- `scripts/check-parity-breadcrumbs.ts`
- `scripts/check-phase157-index-catch-up/contracts.ts`
- `scripts/check-phase158-validated-reorg.ts`
- `scripts/check-current-documentation-reconciliation.ts`
- `scripts/check-current-documentation-reconciliation.test.ts`
- `scripts/verify.sh`

No first-party Rust source was modified by Plan 08. Existing64 changed/new Rust
paths belong to Plans 01–07; the guard snapshots their actual inventory. No
STATE/ROADMAP/REQUIREMENTS/todo/config or formal VERIFICATION/SECURITY mutation,
staging, commit, push or hook bypass was performed here.

## Remaining Root Gate: Task 3

1. Independently review the complete final 01–08 source/doc/guard scope, including
   the three approved historical/inventory checker adjustments above; early 01–06
   review is not final approval of 07/08.
1. Complete independent ASVS1/STRIDE mitigation verification through T-159-27;
   high findings block finalization. Preserve the genuine evidence placement and
   resource/legacy/retention limitations described above.
1. Serialize and run default `bash scripts/verify.sh` with pinned tools, including
   Cargo formatting/clippy/build/tests/coverage/architecture and Bazel; review and
   regenerate/freshness-check tracked `docs/metrics/lines-of-code.md` as required.
   Focused Bun results and `--fast` are not substitutes.
1. Validate lifecycle/plan metadata, then supply actual native duration/results,
   final source/security reports and any fixes for accurate 08 SUMMARY creation.
   Re-run affected evidence/native gate after relevant fixes. Root alone activates
   CFRP-01/02 and performs consolidated commit/push after all gates are clean.

## Self-Check: PASSED

All owned files and fixed new Rust sources exist. Official breadcrumbs, focused
330-test guard, 3 inventory tests and 1,057 historical mutations pass. Requirements
remain uncompleted and all commits remain pending. No stubs, new threat surface
or full native pass is claimed. Task 3 remains explicitly outstanding.

## Review Fixes WR-04–WR-06

The final source review identified three checker defects. The fixes below are
implemented and independently re-reviewed: 159-REVIEW.md records clean status
across 79 files and closes WR-01–WR-06. Full native verification and formal
security/lifecycle acceptance remain outstanding, and the subsequent root-owned
Phase 134 fixture delta still needs its independent review. No production Rust
or public default changed in these checker fixes. Requirements-completed remains
empty and no checkout Git staging/commit/push was performed by this executor.

### Actual RED and Initial GREEN

The targeted pre-fix run selected **27 tests and failed all 27**: eight direct or
owning cfg/cfg_attr exclusions, eighteen constant/self-equality substitutions
across RPC numeric-error, validation-codec and node-query selectors, and one real
Git-ignored generated Rust file. These failures directly reproduce WR-04/05/06;
they are not setup/import failures. After the fixes, the same **27 passed**.
Expanded review regressions then passed **225 tests**, zero failures, with the
original 330 filtered out; 660 assertions and 96.86 seconds. Final verification
used pinned Bun 1.3.9 after the last guard/test edit:

| Check                                    | Final review-fix result                                                                                                   |
| ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| Full explicit Phase159 checker test file | **555 passed**, zero failed; **1,648 assertions**, **246.53s**. Includes the original330 controls and225 review controls. |
| Historical157/158 explicit test files    | **1,057 passed**, zero failed; **2,539 assertions**, **92.38s**.                                                          |
| Canonical breadcrumb inventory tests     | **3 passed**, zero failed; **12 assertions**, pinned Bun1.3.9.                                                            |
| Canonical breadcrumb check               | Pass, **1,055 Rust files**.                                                                                               |
| Phase159 source/evidence/claim CLI       | Pass;117 fixed selectors with minimum action/assertion contracts and supported owning configuration.                      |
| Bright Builds/shell/diff checks          | Zero managed findings; bash-n, shfmt-i2 and git diff checks pass.                                                         |

The test files were selected explicitly with ./ paths. Git mutations required
for ignored-tracked/gitlink controls are confined to disposable fixture repos.
The approximately four-minute focused guard runtime is local software evidence,
not a production query latency measurement. No native verification pass follows
from these results; Task3 remains root-owned.

### WR-04: Supported Executable Ownership

- Added a focused evidence helper and **67 actual external module-owner edges**
  spanning **70 owner files**, including every required selector's ancestor
  paths. Physical-path exceptions follow the actual catch-up-owned query/readiness
  tests and codec-owned private storage tests; path attributes must resolve to
  the expected child file.
- Function declarations, enclosing inline modules, external registrations and
  file-level attributes are checked. Canonical cfg(test) is accepted for tests;
  production paths cannot become test-only. Unsupported cfg/cfg_attr and ignore
  controls are rejected conservatively. The existing crate-level conditional
  Clippy deny policy is specifically recognized as non-disabling.
- All owner edges have false-cfg mutations. Additional controls cover direct
  false cfg/not(test), conditional ignore/exclusion, inline ownership and file
  exclusion. An inner-body attribute control passes, demonstrating that arbitrary
  implementation-body attributes are not misclassified as declaration ownership.
- Ignore/false-cfg mutations preserve valid declaration visibility/async syntax.
  No Rust semantic interpreter or universal configuration evaluator is claimed.

### WR-05: Meaningful Assertions and Actual Actions

- Reject literal-only equality/inequality/arithmetic comparisons and simple
  self-equality/comparisons, including optional assertion message arguments.
  Separate helper tests retain a real action while replacing its assertion, so
  missing-action detection cannot mask a weak assertion classifier.
- Added **117 fixed minimum actual action connections**, one per required
  selector, plus the numeric-errors roundtrip's explicit enum/code/serialization/
  deserialization contract. Every fixed action has a removal mutation preserving
  the test name and remaining assertions.
- These are bounded lexical checks of real action/assertion connections and
  selected constant/tautology forms. They supplement executed Rust behavior;
  they do not prove arbitrary expression semantics or general dataflow.

### WR-06: One Canonical Git Inventory

- Exported the repo-owned breadcrumb checker's existing worktreePaths helper and
  gated only its CLI main on import.meta.main. The phase guard now reuses that
  exact cached+untracked+exclude-standard inventory rather than recursive readdir.
  Fixed NEW_RUST requirements remain explicit.
- Real temp Git controls cover ignored generated source exclusion, ignored
  tracked source remaining checked, and Git links not being recursively scanned.
  Existing untracked manifested/unmapped canonical checker regressions still pass.
  Only disposable fixture indexes are written by tracked/gitlink regression
  setup; this checkout's index is untouched.

### Final Review-Fix Scope

Changed in this continuation: scripts/check-phase159-filter-rpcs.ts, its mutation
test, contracts.ts, the new evidence.ts helper, and the narrow import/main glue in
scripts/check-parity-breadcrumbs.ts, plus this checkpoint. The earlier exact
owned-file list remains applicable. All new checker files remain below 628 lines.
No root tracker/config, formal VERIFICATION/SECURITY, production Rust, native
gate, Cargo/Bazel target or Git finalization was touched.

### Review-Fix Self-Check: PASSED

All owned guard/helper/test and checkpoint files exist; the full 555-test suite,
1,057 historical regressions and canonical inventory checks pass. Exact source
and evidence limitations remain unchanged. WR-04–WR-06 are independently closed
in the root-owned source review; this executor did not overwrite that report.
The later Phase 134 repair delta, full native result and formal security/lifecycle
gate remain pending. `requirements-completed` remains `[]`.

## Root Native Receipt Before Attempt 2 Completed

Reconciled at 2026-10-09T20:04:11Z from the root receipt and
[159-NATIVE-CHECKS.md](159-NATIVE-CHECKS.md). This is a checkpoint update only;
159-08-SUMMARY.md remains unwritten pending actual full native exit 0.

- **Native attempt 1:** default bash scripts/verify.sh exited 1 after 7m 30.555s,
  before Cargo. Its Phase 159 guard passed 555 tests with 1,648 assertions in 251.54s;
  canonical inventory passed 3 tests and verified 1,055 Rust breadcrumbs.
- **Failure boundary:** the historical Phase 134 suite reported 252 passes and
  five failures because mutation insertion preconditions still expected the old
  bare apply_lifecycle_command line. Phase 159 now stores its outcome before
  collecting wakes. No forbidden I/O mutation was inserted in those failing
  cases; this is neither a native pass nor a production Rust test failure.
- **Root-owned repair/key file:** only the two insertion marker/newline lines in
  scripts/check-phase134-authoritative-lifecycle.test/mutations.ts were changed.
  The same five original forbidden I/O statements still enter at the actual
  call before drop(network), preserving original checker diagnostics/assertions.
  Root reports 257 focused tests passed, 358 assertions, 14.27s. Independent review
  of this small delta remains pending; this executor ran no additional tests.
- **Native attempt 2:** root owns live session 98961 and ignored log
  packages/target/phase159-native-2.log. A complete default rerun is in progress;
  no final result, Cargo/Bazel/coverage success or requirement completion is
  inferred. No second build or native session was started by this executor.
- **Source review:** root-owned 159-REVIEW.md records clean 79-file whole-phase
  status with six closed findings. The later Phase 134 mutation-fixture repair
  is additional root-produced review scope; it is not an executor-owned edit.
- **Formal gate:** the root's preaudit reports 25 truths/four criteria,
  19 artifacts/21 links and no gap, awaiting the actual native/security/Plan08
  closeout evidence. This is not a formal verification artifact or completion
  claim written by this executor.

All previous genuine acceptance/retention/resource/legacy limitations stand.
Task 3 remains root-owned. No 08 SUMMARY, formal VERIFICATION/SECURITY, production,
state/tracker/config, test execution or checkout Git mutation was performed in
this reconciliation.

## Native Attempt 2 Failure and Current RPC Catalog Reconciliation

The root subsequently reports default native attempt 2 exited 1 after 7m 21.043s,
again before Cargo. The Phase 134 mutation repair passed all 257 tests in that
attempt. The next blocker was current-documentation reconciliation: 18 tests,
16 passes and two failures. The Rust registry now contains 29 serde names, while
the checker still required 27 and the exact baseline-method catalog omitted
getblockfilter/getindexinfo. This failed native attempt is not a native pass or
a production Rust test failure.

Root assigned a narrow catalog/checker repair while keeping serialized Rust
preflight and any later complete native rerun root-owned. Actual source
inspection confirms **29 methods = 21 BaselineParity + 8 OpenBitcoinExtension**.
The eight extension names remain unchanged; both Phase 159 methods are baseline
methods. A scoped search of current scripts/docs/READMEs found no other active
method-count 27 consumer.

### Narrow Repair and Focused Evidence

- `docs/parity/catalog/rpc-cli-config.md` now lists both methods in its exact
  supported baseline-backed list, states 29/21/eight grouping, and documents only
  implemented authenticated BASIC query/summary behavior with the detailed
  scoped catalog link and unchanged later-phase/default/production exclusions.
- `scripts/check-current-documentation-reconciliation.ts` changes only the
  expected enum count and corresponding diagnostic 27→29. Existing exact-set,
  baseline/extension grouping, wallet/relay/claim and verifier-order checks stay.
- `scripts/check-current-documentation-reconciliation.test.ts` preserves all
  original 18 tests and adds two independent controls: remove either new method
  from the canonical catalog list while its other prose mentions remain, then
  require the precise missing-method and wrong-baseline-grouping failures.
- Actual pre-fix RED reproduced the native failure: 16 passed/two failed across
  the original 18. Final focused GREEN on pinned Bun 1.3.9 is **20 passed**,
  zero failed, **45 assertions**, **94ms**. The live reconciliation checker passes;
  Bright Builds reports zero findings. Checker/test files remain 480/375 lines.
- Compatible GFM/frontmatter Markdown check reports the catalog is not formatted;
  its sole formatter diff joins the existing Phase 106 `status --format json`
  code span at current lines 134–135. That unchanged section is outside this
  repair. The new method list/count/behavior sections introduce no formatter
  differences; no unrelated rewrite was made.

The final Plan 08 owned-file list above now includes these three additional
catalog/checker/test paths. The root-produced Phase 134 mutation fixture and
159-NATIVE-CHECKS.md remain additional review/evidence scope, not executor edits.
This is a Rule 3 blocking documentation-guard reconciliation caused directly by
the two new baseline methods; no historical assertion or negative scope was
removed. A transient new-test removal needle expected a trailing space before a
newline and was corrected; it is not semantic RED evidence.

No Cargo/Bazel/native process, production Rust, root tracker/config, checkout
Git operation or formal verification/security artifact was started or changed
by this executor. Task 3, full native success, final source/security/lifecycle
acceptance, requirements and Git finalization stay root-owned. No 08 SUMMARY is
written until the root supplies an actual successful full native receipt.

## CLI Conversion and Explicit Native Binary-Test Gate

Root Rust preflight subsequently found E0004: the CLI client's exhaustive
method_call_to_json match lacked GetBlockFilter/GetIndexInfo. Plan 02 owns the
production conversion fix and three client plus three argument-parser controls
in existing client.rs, client/tests.rs and args/tests.rs. This executor did not
edit those files or run Cargo/Bazel. The current CLI binary has test=false in
Cargo.toml, so ordinary workspace cargo test alone does not execute its client
tests. Existing client tests use hermetic localhost ephemeral TCP servers.

### Frozen Guard and Verifier Extension

- Default scripts/verify.sh now runs the entire explicit CLI binary test suite
  immediately after the existing workspace test stage, with no selector,
  no-run or list mode:

  ```bash
  cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin-cli --all-features
  ```

- The phase guard requires that exact executed stage immediately after workspace
  tests. Independent mutations remove it, remove its workspace predecessor, or
  substitute no-run/list/nonexistent-selector execution; these are rejected.

- Fixed Rust selectors/actions now number **123**, adding all six stable CLI
  controls. Contracts cover actual typed-to-wire conversion, raw hash reversal
  and lowercase display, Basic/V0 names, named blockhash/filtertype/index_name
  projection, omitted selector, shared normalization, node-scoped authenticated
  HTTP routing and the real executable's client entrypoint.

- Wire-key/type/auth string checks are bound to the selected actual function and
  executable macro/call span; comments cannot supply a missing projection.
  An entrypoint mutation exposed a substring weakness (detached_client matched
  client); the check now requires the exact client module boundary.

- Owning edges now number **71**, covering **76 files**. The binary main.rs owns
  client.rs/client/tests.rs; library lib.rs owns args.rs/args/tests.rs. The
  resolver narrowly recognizes main.rs as a crate root, with positive and
  false-cfg controls on every added ownership edge. Canonical Git-aware breadcrumb
  discovery includes the CLI source root without scanning ignored files.

- SOURCE_FILES now has 70 paths: the earlier 64 phase Rust paths, three additional
  changed CLI paths and three unchanged CLI entry/library/parser context paths.
  This is not a claim of 70 modified Rust files or new public APIs.

### Canonical Node Test Rename

Root independently renamed sync/runtime_state/validation_history_tests.rs to
sync/runtime_state/tests.rs with mod tests, updating its exact breadcrumb mapping,
parity evidence, guard references and Plan 03 summary. The actual three selectors
are unchanged. This executor preserved those root edits. Root reports canonical
panic rescan session 73929 exited 0. The live guard and 1,055-file breadcrumb check
pass against the canonical path; no extra storage authority or test API follows.

### Focused Guard Evidence and Unexecuted Runtime Evidence

- Six initial CLI guard mutations all failed RED before extension: lost filter
  or index conversion, wrong display order, wrong HTTP route, removed shared
  normalization and absent native binary-stage protection. The live guard then
  separately rejected the missing explicit native invocation before wiring it.
- The first expanded focused run passed 40/41; its exact-entrypoint negative case
  revealed the substring weakness described above and was fixed.
- Final pinned Bun 1.3.9 focused run: **41 passed**, zero failed, **122 assertions**,
  **20.40s**, 553 filtered. It covers the new six selectors' no-op/ignore/action
  controls, all added owner edges, production conversion/field/auth routes,
  native execution mutations, canonical ownership positive controls and body-
  attribute noninterference.
- The current full guard suite discovers **594 tests**. Its earlier 555-test full
  receipt predates this narrow extension; the full 594 run remains a root native
  gate. Per root instruction, this executor ran the focused extension controls
  and live checker instead of redundantly rerunning the four-minute full suite.
- Live checker passes with 123 selectors. Canonical breadcrumbs verify 1,055 Rust
  files; Bright Builds reports zero findings; bash-n and shfmt-i2 pass. Current
  guard/main-test/contracts/evidence files are 279/380/426/171 lines, all below 628.
- No actual new CLI Rust test pass is credited: Plan 02's command is waiting at
  the macOS pre-Rust loader/build-script boundary. Root owns liveness/recovery
  decisions and reports no source failure or successful test execution for that
  queued command. No cancellation, cache cleanup or host security change was
  performed by this executor.

Root reports clean 86-file source review before this guard/native extension;
independent review of the frozen extension is pending. Complete native attempt 3
has not yet started and remains root-owned after source freeze. All prior
acceptance/retention/resource/legacy/default exclusions remain. No 08 SUMMARY,
requirement activation, formal source/security/lifecycle completion or checkout
Git finalization is authorized by these focused results.

## Native Attempt 3 Node Failures and Narrow Guard Follow-Up

The root-owned native report now records attempt 3 exited 101 after
**1h5m5.214s**, with **1,417 Node tests passed, ten failed and three ignored**
(570.11s harness execution). Earlier Rust suites had 1,931 passes. The actual
Phase 159 guard passed 594 tests/1,765 assertions in 298.84s; production panic,
workspace formatting, strict full-workspace/all-target/all-feature Clippy and
build passed. This closes the CLI conversion's original E0004 at compile time,
not all CLI runtime proof. The CLI library's 365 tests include its three new
argument controls. Later RPC/explicit CLI binary/benchmark/Bazel/coverage stages
were not reached, so no full native pass or requirement completion follows.

### Root-Owned Repairs and Current Review Boundary

- Group A restores the raw metadata boundary in
  storage/fjall_store/validation_history.rs: read/decode the authentic ledger
  first, then compare ScriptsValid status and exact hash/parent/height with raw
  DTO fields. Absence/mismatch/non-ScriptsValid loses complete coverage durably;
  actual ledger decode errors still propagate. Sparse legacy metadata is not
  rejected by the stricter accepted-identity constructor. That constructor and
  genuine accepted-admission checks remain strict elsewhere.
- Group B fixes test authority setup: raw-seeded fixture history earns genuine
  configured recovery; private corruption tests mutate backend state beneath an
  already earned proof where that is the intended boundary. The new raw-clone
  regression proves ordinary raw mutation revokes captured proof and genuine
  complete recovery refuses an incompatible predecessor.
- Root also repairs the source-structure assertion in chainstate/tests.rs to
  follow moved producer functions with exact function-name boundaries, retaining
  its NoAlways/Periodic checks. This executor changed no Rust file or global
  source-sensitive marker.
- The prior 86-file/six-closed source-review receipt predates these real failure
  repairs. Source review is reopened for the production helper, regression and
  fixture/assertion deltas; final source/security/lifecycle acceptance remains
  root-owned and pending.

### Frozen Guard Delta

- The metadata helper contract now requires authentic ledger read with error
  propagation, ScriptsValid status, exact hash/parent/height binding,
  authenticated=false on absent/mismatched rows, and invalidation only after
  dropping the history guard. A separate negative check rejects reintroducing
  BlockValidationIdentity::new at this raw boundary.
- Strict accepted identity construction is independently guarded for zero hash,
  self-parent and genesis-parent/height consistency. Existing sealed admission,
  accepted receipt, private-field, publication and global source-sensitive checks
  remain unchanged; no broad exception or suppression was added.
- Added the actual sparse-raw-metadata and raw-clone/recovery selector/action
  contracts. Their tests must retain live/reopen UnknownLegacy versus accepted
  positives, strict constructor refusal, raw proof revocation, genuine recovery
  refusal, unchanged corrupt bytes and no newly fabricated immutable row. The
  append fixture's recovered helper must earn configured recovery after body/undo
  seeding. Actual module ownership follows the existing append/admission owners.
- Current guard inventory: **125 selectors**, **73 owner edges/78 owner files**,
  71 source/context paths. New read context is
  storage/fjall_store/filters/tests/append/admission.rs and its existing parent.
  Existing canonical Rust breadcrumb coverage remains 1,055 files.

### Focused Evidence, Not a New Full Native Receipt

- The stale pre-fix guard initially rejected the changed helper's old constructor
  route marker. After exact contract reconciliation, two independent guard-only
  mutations failed RED: reintroduce raw DTO strict parsing, and loosen the
  accepted identity constructor. Both now reject with specific diagnostics.
- Final pinned Bun 1.3.9 helper/new-selector/source mutation selection:
  **18 passed**, zero failed, **53 assertions**, **8.96s**, 593 filtered. Additional
  append-parent owner false-cfg control: **one passed**, zero failed,
  **three assertions**, 871ms, 610 filtered. The selections are distinct.
- Live source/evidence guard passes with 125 selectors. Canonical breadcrumbs
  verify 1,055 Rust files; Bright Builds reports zero findings; bash-n/shfmt-i2
  pass. Guard/main-test/contracts/evidence files are 288/422/434/171 lines.
- The current full suite discovers **611 tests**. The actual native 3 **594-pass**
  receipt is historical and predates these repairs. This executor did not run
  full 611 or Cargo/Bazel; root combined Rust verification and default native 4
  remain required after the frozen source deltas and independent review.

Only the Phase 159 guard, its contracts/mutation tests and this checkpoint were
changed by this executor in this follow-up. Additional root-produced Rust review
scope includes validation_history.rs, coverage.rs, filters/tests/append.rs and
admission.rs, plus chainstate/tests.rs. No production Rust, native process, tracker,
config, formal verification/security, requirement or checkout Git mutation was
performed by this executor. No 08 SUMMARY is written before actual full native
success; Task 3 remains root-owned.

## Final Artifact Preparation: Native Attempt 4 Still Running

The root supplied the following current-source receipts. They supersede earlier
pending execution notes where explicitly stated; they do not turn any failed
native attempt into a pass.

- **Rust recovery 4 exited 0:** ordered formatter, strict workspace/all-target/
  all-feature Clippy, all-target/all-feature build, test precompilation, the exact
  remaining catch-up regression, full workspace tests/doctests and explicit whole
  CLI binary tests completed successfully. Workspace primary passes are **3,857**,
  with **seven** explicit CLI binary passes (**3,864 primary Rust passes**).
  The separate exact diagnostic probe is excluded from that primary total.
- Node now reports **1,429 passes, zero failures and three existing opt-in
  ignores**. All ten prior failed cases and the two new metadata/raw-clone
  regressions pass. The three new CLI argument controls pass in the 365-test CLI
  library suite; the three client conversion/authenticated-root controls pass
  in the explicit seven-test CLI binary suite. Earlier host-stalled commands are
  still diagnostic aborts, not these successful executions.
- Independent final source review is clean across **91 paths**, with **all seven
  findings closed**. Its CLI and recovery guard extensions are clean. No open
  source finding or waived authority/corruption check is inferred.
- The full current Phase159 guard has **611 passes and 1,816 assertions** again
  inside native attempt 4. It covers 125 fixed selectors and 73 owner edges/78
  owner files. This is actual current full-guard execution, not merely the earlier
  focused selections. Native duration and remaining-stage success are not yet
  supplied and are not invented here.
- **Default native attempt 4 remains running**, root session 19945 and ignored
  log packages/target/phase159-native-4.log. Root reports Node passed and RPC
  execution active. Later CLI/benchmark/Bazel/provenance/coverage and final exit
  still belong to this ongoing complete run; the successful Rust recovery is
  not a substitute for default-native completion.
- The root-owned SECURITY artifact currently remains draft/OPEN_THREATS with
  T-159-26 open; its execution-era header predates final recovery. This executor
  does not rewrite or close it. Root/auditor formal security/lifecycle closeout
  remains required after the actual native receipt and Plan08 artifact handoff.

The final Summary's file scope, substantive claims, deviations and failure/
recovery history are prepared. No completed 159-08-SUMMARY.md is written before
the root supplies actual native attempt 4 success. Source/build/state/Git files
remain untouched by this artifact-only continuation, and requirements-completed
remains empty. The final native receipt will determine the exact completion date,
duration and final-stage results; root still owns activation and consolidated
commit/push.

## Actual Native Attempt 4 Success and Final Summary Handoff

The root supplied actual session 19945 exit 0, confirmed by the saved native
report's status passed: default bash scripts/verify.sh completed in
**36m 0.155s / 2,160,155ms**. Primary workspace tests/doctests passed 3,857;
explicit whole CLI binary passed 7 (0.02s); combined primary Rust total is
**3,864 passed, zero failed, three existing opt-in ignores**. Node's included
subset is 1,429 passed/zero failed/three ignores in 615.09s. Coverage reruns and
the separate diagnostic probe are excluded from the primary total.

The final current guard passed **611 tests/1,816 assertions in 305.92s**, with 125
fixed selectors and 73 owner edges/78 files. Canonical inventory/breadcrumbs,
all historical/reconciliation guards, managed/dependency/file-length/panic
checks, workspace formatting, strict all-target/all-feature Clippy/build,
benchmark list/smoke/report, all six Bazel smoke targets/provenance and configured
pure-core llvm-cov gate (**zero uncovered lines**) passed. Root regenerated LOC
to 398,783 counted lines. Clean independent source review remains 91 paths/all
seven findings closed.

Final 159-08-SUMMARY.md is now written with the same lifecycle ID/yolo mode,
actual generated timestamp and requirements-completed[]. Its self-check confirms
all 22 recorded key paths exist; failed/aborted trails and exact residual limits
are preserved. Task implementation/native handoff is complete. Root security,
formal goal/release lifecycle verification, requirement activation and
consolidated commit/push remain subsequent gates; no such approval or Git
mutation was performed by this executor. No builds/source/state files changed
in this artifact-only finalization.
