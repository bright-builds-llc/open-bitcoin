---
phase: 159-authenticated-basic-filter-and-index-rpcs
status: passed
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 159-2026-10-09T16-11-49
generated_at: "2026-10-10T00:04:10Z"
---

# Phase 159 Native Verification

## Contract

Run the default `bash scripts/verify.sh` with Bun 1.3.9 (PATH prefix
`/tmp/open-bitcoin-bun-1.3.9-fresh`) and pinned Rust 1.94.1. No fast mode,
check bypass, overlapping Cargo/Bazel job or commit-hook bypass is used.
Root owns the serialized build slot. Ignored raw logs live in `packages/target/`.

## Attempt 1 — Failed Before Cargo

- Command: `bash scripts/verify.sh`.
- Exit: 1 after **7m 30.555s** (450,555 ms).
- Log: `packages/target/phase159-native-1.log`.
- Phase 159 guard: **555 passed**, zero failed, 1,648 assertions in 251.54s.
- Git inventory: three passed; 1,055 Rust breadcrumbs verified.
- Stop: historical Phase 134 authoritative lifecycle mutations, 252 passed and
  five failed. The five insertion precondition assertions could not find the old
  bare `apply_lifecycle_command` line after Phase 159 changed it to a saved
  `let outcome` before wake collection. No forbidden I/O mutation was inserted;
  this failed run is neither a native pass nor a Rust test failure.
- Repair: update only the insertion marker/newline in
  `scripts/check-phase134-authoritative-lifecycle.test/mutations.ts`. Inject all
  five original forbidden I/O statements at the actual call, still before
  `drop(network)`. Keep exact original checker diagnostics and assertions.
- Focused rerun: **257 passed**, zero failed, 358 assertions in 14.27s, exit 0;
  `packages/target/phase159-p134-retry.log`. Independent delta review requested.
- Cargo, benchmark, Bazel and coverage stages were not reached in this attempt.

## Attempt 2 — Failed Before Cargo

- Command: default `bash scripts/verify.sh`; exit 1 after **7m 21.043s**
  (441,043 ms); `packages/target/phase159-native-2.log`.
- Phase 159 guard passed again; historical Phase 134 repair passed all 257 tests
  inside this complete run.
- Stop: current-documentation reconciliation, 16 passed and two failed. The
  production SupportedMethod enum now has 29 methods, while the historical
  checker expects 27 and the RPC catalog omits getblockfilter/getindexinfo.
- Repair assigned to Plan08: reconcile the actual method count/grouping and
  catalog, preserving exact set equality and adding method-specific removal
  controls. No production methods or checker gate may be removed.
- Cargo, benchmark, Bazel and coverage stages were not reached in this attempt.

## Replan and Required Retry

After two failed full attempts, inspect current method-count consumers, repair
and review the scoped documentation/checker delta, and run serialized complete
workspace Rust preflight before restarting the entire native contract. Focused
and preflight success do not replace default-native success.

## Rust Preflight — Host Loader Recovery

The first serialized workspace preflight completed formatting, then waited on
CLI build-script child PID90383. Two process samples at 20:14:08Z and 20:14:46Z
showed only `_dyld_start`, a 96 KiB footprint, zero accumulated CPU and no Rust
frames. After liveness inspection, root sent SIGTERM only to that child; Cargo
reported the deliberate signal and preflight exited101. This diagnostically
aborted preflight is not a compiler/test failure or a pass. Raw log:
`packages/target/phase159-rust-preflight.log`; two sample files use the matching
`phase159-preflight-build-script-sample` prefix. No source, security metadata or
host service was changed. The same complete workspace preflight was restarted
in `packages/target/phase159-rust-preflight-2.log` before full-native attempt3.

The preflight retry waited on the same cached executable, child PID90869.
A further sample at 20:16:29Z again showed only `_dyld_start`,96 KiB and zero
CPU/no Rust frames. Root terminated that child; preflight2 exited101 because of
the deliberate signal, not a source diagnostic. After the repeated pre-start
condition, the recovery plan changed to a scoped, timed
`cargo clean --manifest-path packages/Cargo.toml -p open-bitcoin-cli`, which
exited0 and removed only that package's local Cargo artifacts (12,465 files,
4.5 GiB). No tracked file, source or system policy was changed. Complete
preflight3 now rebuilds the CLI artifact and reruns all four Rust stages;
`packages/target/phase159-rust-preflight-3.log` is its receipt.

Preflight3's host admission wait cleared without intervention. Formatting
passed, then complete-workspace Clippy found real E0004 in
`packages/open-bitcoin-cli/src/client.rs:205`: the exhaustive MethodCall JSON
conversion omitted GetBlockFilter/GetIndexInfo. Preflight exited101; build and
tests were not reached. Plan02 executor owns the required typed CLI encoding
and positional/named/default/selector controls, followed by focused strict
verification and independent review. This is a real source integration failure;
the earlier loader delay is not its cause or a waived check.

The documentation repair for native2 is independently clean: 29 methods
(21 baseline, eight extensions), exact baseline/extension set equality retained,
18 original tests plus two new per-method removal controls all pass (20 tests,
45 assertions,94ms), current checker passes and managed standards report zero.
Final review now covers83 paths before the pending CLI fix.

## Final Source Freeze and Attempt 3

- All six source-review findings are closed; final independent review covers86
  current source/test/script/doc paths, including CLI conversions and native
  guard wiring.
- Actual cfg(test)-owned `runtime_state/validation_history_tests.rs` was renamed
  to canonical `runtime_state/tests.rs`, preserving all three controls and
  updating the owner/parity/evidence paths. Production panic scan now passes.
- CLI source repair adds six named controls; actual runtime execution remains
  uncredited. Its focused command waited on build-script PID93108 before Rust
  main for692,417ms. Final sample799/799 `_dyld_start`,96KiB and zero CPU
  justified stopping only that child; Cargo101 reflects SIGTERM. See
  `159-02-FOLLOWUP-CHECKPOINT.md`, which preserves the original E0004 source
  failure and exact remaining commands. No definitive current policy denial or
  signature defect was established; earlier admission-denial timestamps may
  reflect cancellation. No host policy, signature or security attribute changed.
- Final guard has123 fixed Rust selectors,71 owner edges/76 files; full current
  suite594 tests. New focused CLI/ownership/native-invocation controls:41 passed,
  zero failed,122 assertions20.40s (553 filtered). Live guard,1055 breadcrumbs,
  Bright Builds and shell checks pass. Full594 execution is still required.
- Default verifier now explicitly runs the hermetic whole CLI binary test suite
  with all features immediately after workspace tests, covering Cargo's
  otherwise test=false binary.
- LOC report regenerated:398,653 lines counted. Preverification lifecycle valid
  with eight plans/seven completed summaries;08/formalverification still pending.
- Root started the complete default `bash scripts/verify.sh` again against final
  frozen source; `packages/target/phase159-native-3.log`. No prior failed or
  diagnostically aborted attempt is counted as success. Native3 outcome pending.

Attempt3 intermediate receipt: final Phase159 guard **594 passed**, zero failed,
1,765 assertions in298.84s, covering123 named Rust selectors. This is actual
full current guard execution inside default native3; remaining native/Rust/CLI
and final lifecycle receipts are still pending.

Attempt3 reached Rust stages: production panic scan and formatting check pass;
strict full-workspace/all-target/all-feature Clippy passes (6.49s), followed by
all-target/all-feature build (17.35s). This proves the current CLI conversion
compiles and closes original E0004; it does not establish runtime behavior.
Workspace tests reached the first bench-library executable PID17294 but had not
entered the harness. A20:49:48Z sample contains only `_dyld_start` (1,751 samples),
112KiB, zero CPU and no Rust frames. Current native attempt remains running;
no test outcome, native pass, benchmark/Bazel/coverage receipt is inferred.

## Attempt 3 — Failed Node Suite

- Default native exited101 after **1h5m5.214s** (3,905,214ms).
- Source guard594/0fail/1,765assertions and all historical checker repairs passed;
  dependency/file-length/panic checks, workspace formatting, strictClippy and
  all-target/all-feature build passed.
- Earlier completed Rust suites:1,931 passes. Node:1,417passed,10failed,
  threeignored (570.11s actual harness time). These partial counts are not a
  full-native pass; later RPC/CLI-bin/benchmark/Bazel/coverage stages were not
  reached. CLI library365 includes all three new argument controls.
- Failures: four legacy/raw prune/flush cases reject invalid validation-history
  identities; one source-structure assertion still reads moved producer
  functions from chainstate.rs; five recovered-prefix/immutable append cases
  lose a proof or return early no-authority instead of the expected local-edge error. No corrupt append success was observed. Exact names/errors are retained
  in raw log lines7282–7351 and the task recovery plan.
- Plan03/04 executors own metadata/append root-cause repair respectively; root
  owns the structural assertion. Preserve authority, corruption refusal and
  complete-coverage invalidation; no check may be waived.
- Host waits cleared without intervention for multiple executables; samples
  distinguish pre-main stalls from the actual assertion failures. No host policy
  or signature change was made. The queued full Cargo formatter completed0
  after native release. Final source freeze/review/verification gates reopen for
  the narrow fixes and a complete default rerun.

Recovery diagnosis: GroupA is a real compatibility regression: raw sparse
metadata DTOs do not constitute accepted-history identities. The guard now reads
actual strict ledger evidence and compares its hash/parent/height; missing,
nonScriptsValid or mismatching evidence durably invalidates coverage instead of
rejecting previously supported raw persistence. Authentic ledger decode errors
still propagate; strict accepted-identity construction is unchanged. A new
live/reopen regression covers absent and mismatching positive evidence.

GroupB's five failures are stale fixture assumptions: public raw BASIC writes
now intentionally revoke recovered append/read authority. Valid reuse fixtures
earn genuine recovery after seeding; local-corruption fixtures mutate the
private backend under an existing live proof to reach the original refusal
boundary. They retain refusal/no-rewrite assertions. A new clone-corruption
control proves raw invalidation before recovery and actual recovery refusal;
there is no fabricated clean epoch or production proof bypass.

The source-only producer guard now reads two moved functions from their actual
module and uses exact function-name boundaries, retaining no-Always/no-Periodic
assertions (the old `fn persist` needle could match `persist_block`). Combined
workspace Rust recovery verification started against all frozen Rust fixes;
`packages/target/phase159-rust-recovery-1.log`, root session56022. Current-source
execution and independent delta review are pending.

Rust recovery preflight1 stopped at strictClippy: one needless borrow in the new
private-backend fixture (`append.rs:273`, `&codec::active_key(0)`). Root applied
Clippy's owned-array form, without semantic change. Exit101 before runtime tests
is retained in `phase159-rust-recovery-1.log`. The complete ordered formatter,
workspace Clippy/build/tests and explicit CLI-bin sequence restarted as
`phase159-rust-recovery-2.log`/session83291. No suppressions were added.

Scope clarification: GroupB repairs exercise the real production stored-record
recovery/append paths using the existing synthetic storage fixtures. They do
not independently prove consensus-chain acceptance. The distinct genuine
accepted-chain/configured-daemon/prune/reopen proof remains in Plan07's report.

Rust recovery2 exited101: node1,428passed,onefailed,threeignored (609.74s).
All four original sparse-metadata cases and the new sparse live/reopen test
passed; WR07 closes on actual regression evidence. All four append conflict/
reuse cases and the new raw-clone recovery refusal control passed, as did the
producer source assertion. Remaining catchup fixture failed with 'untracked
BASIC replacement append positions'; a direct exact-case run of the already
admitted linked binary reproduced the failure in12.59s.

The fixture had reconfigured only its store, changing recovered branch identity
to persisted tip2 while the live manager retained stable genesis identity. The
production binding correctly refused. The repair drops old staged/core/store
references and uses the existing actual ReorgFixture::reopen path to close and
reopen BOTH store/manager, initialize configuration and bind from recovered
chainstate. Original one-block-turn, missing-payload, reuse1/generation1/lag0
assertions remain. No production authorization changed. This last one-file
repair is independently source-clean and awaits execution.

The updated guard fully passed611tests/0fail/1,816assertions/312.02s before that
last fixture sequencing edit. Current125selectors/73owneredges/78files preserve
strict accepted identities and raw-metadata fallback. Root recovery3 now runs
ordered fmt/Clippy/build, test precompilation (not execution proof), the exact
remaining case, then full workspace and whole CLI binary tests. Its raw receipt
is `phase159-rust-recovery-3.log`/session3623. Full default native4 remains required.

Recovery3 precompiled cleanly but its exact fixture test failed0/1 in0.94s:
actual paired recovery had already restored genesis0, and the extra one-block
worker advanced to1. The test correctly rejected setup that no longer lagged
behind common1. The final repair removes that extra mutation and asserts the
real read-only processed endpoint equals Some(the stored genesis identity),
without using zero-summary fallback or changing original reuse/absence checks.

Recovery4 formatter/strictworkspaceClippy14.63s/build7.63s/precompilation passed.
Its exact regression then passed1/0fail in0.87s with1431filtered; full workspace
and whole CLI binary execution continues in `phase159-rust-recovery-4.log`,
session63478. Independent final source review is clean across91paths, all seven
findings closed; WR07 has actual original-four-plus-new-regression GREEN.
No whole-native result or requirement activation is inferred.

## Rust Recovery4 — Passed

Complete ordered fmt, strict workspace/all-target/all-featureClippy, all-target/
all-featurebuild, test precompilation, exact remaining catchup regression, full
workspace tests/doctests and explicit whole CLI binary tests exited0.

- Full workspace: **3,857passed**, zerofailed,threeexistingoptinignores.
- Explicit CLI binary: **7passed**, zerofailed, including all three new request/
  authenticated-root HTTP controls. All three new args controls also passed in
  CLI library365. Combined primaryRust count is3,864; the separate exact probe
  adds one diagnostic pass and is not included in that primarycount.
- Node:1,429passed/zerofailed/threeignored (660.55s); all ten originally failed
  cases and both new metadata/raw-clone regressions are GREEN.
- Independent finalsource review:91paths, all seven findings closed.
- Raw receipt:`packages/target/phase159-rust-recovery-4.log`; session63478exit0.

## Attempt4 — Default Native In Progress

Root started default `bash scripts/verify.sh` against frozenfinalsource;
`packages/target/phase159-native-4.log`, session19945. LOCreport regenerated:
398,783lines. Managed checks0, diffcheckclean. The successful Rust recovery and
611guard run do not replace actual complete default-native success; benchmark,
Bazel/provenance and coverage receipts still require this run. No requirement,
phase, commit or push is activated by this intermediate record.

## Attempt4 — Passed Full Default Contract

The unchanged default `bash scripts/verify.sh` exited **0** in **36m 0.155s**
(2,160,155ms), with Bun1.3.9/Rust1.94.1 and frozen final source. Root session19945
completed; actual receipt is `packages/target/phase159-native-4.log`.

- Phase159 guard: **611passed**, zero failed, **1,816assertions**,305.92s;
  125 fixed Rust selectors and73 owner edges/78files. Git inventory controls,
  1,055 Rust breadcrumbs, historical guards and current-documentation
  reconciliation passed.
- Managed standards, dependency/file-length/panic checks, workspace formatting,
  strict all-target/all-feature Clippy and all-target/all-feature build passed.
- Primary Rust workspace tests/doctests: **3,857passed**, zero failed,
  three existing opt-in ignores. Node:1,429passed/zero failed/threeignored,
  615.09s. Explicit whole CLI binary: **7passed**, zero failed,0.02s,
  including all three new request/authenticated-root controls. Combined primary
  total is **3,864passed**; coverage reruns and prior diagnostic probes are
  excluded from that total.
- Benchmark list/smoke and report validation passed. All six Bazel smoke targets
  built; build provenance passed. Configured pure-core llvm-cov test/coverage
  gate passed with zero uncovered lines.
- LOC regenerated to398,783lines. No fast mode, skipped check, hook bypass,
  production dependency, host-security mutation or overlapping build was used.

This closes actual native execution. Independent security closeout, final08
summary, formal goal verification and release lifecycle validation remain
subsequent gates; no commit/push has yet occurred. Failed/aborted attempts above
remain historical evidence and are not counted as passes.
