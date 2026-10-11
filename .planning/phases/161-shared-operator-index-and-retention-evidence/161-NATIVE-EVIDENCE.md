---
phase: 161-shared-operator-index-and-retention-evidence
status: passed
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 161-2026-10-11T04-48-29
generated_at: "2026-10-11T08:48:19Z"
---

# Phase 161 Native Verification Receipts

This record preserves actual attempts. A failed default run is not passing phase evidence. The root serializes Cargo/Bazel work through the cooperative timing runner; the default verifier wraps its own `verify-full` lock/timing record. Bun 1.3.9 is supplied by the re-probed prepared runtime; Rust is pinned to 1.94.1.

## Attempt 1 — Required Panic Policy Failure

Command: `bash scripts/verify.sh`, default/full mode. Exit **1** after **10m 51.964s (651,964 ms)**. Parity, source/claim mutation suites, architecture/import and managed checks passed before the panic-site policy stopped execution; Cargo stages had not run.

The policy reported seven `expect` sites in the new test-only `network/operator_snapshot/basic_tests.rs`. Its parent was correctly `#[cfg(test)]`, but the deliberately simple policy recognizes only canonical `tests.rs`/`tests/` paths for separate test files. Plan04 was reopened to rename the child/module to `operator_snapshot/tests.rs`/`mod tests` and update its exact parity breadcrumb. Assertions remain intact; no panic allowlist or checker exception is authorized. The pre-rename child SHA256 is `bb80882d322ef3567841b339e23ae50d53a92e13137506956021ea843b8ba85c` for confirming a content-preserving move.

Source/security/formal approval remains pending the correction, focused checks, independent scope reconciliation and a successful complete default rerun. No staging, commit, push or requirement activation occurred after this failed attempt.

## Derived Inventory and Final Hook

The LOC generator and managed file-length inventory use tracked Git paths. Before staging, root separately checked all 24 new Rust/TypeScript source files against the managed 628-line limit; all passed, with a maximum of 622 lines. Actual Rust builds/tests and parity checks include the new source. The mandatory pre-commit hook regenerates tracked LOC from the final index and reruns the full verifier, bringing the new paths into managed tracked inventory. Current prestage LOC is not used as phase-size proof or an eventual-commit digest.

## Canonical Test-Layout Correction and Attempt 2

Correction released 2026-10-11T08:53:31Z. The renamed `operator_snapshot/tests.rs` has exactly the pre-rename SHA256 above; only its parent module name and parity path changed. Eighteen actual authority tests, scoped formatting, 1,122 parity paths, standalone panic policy and strict node all-target/all-feature Clippy passed. The complete failed-policy outcome is retained above.

Root reran actual Cargo formatting and regenerated worktree LOC, then started `bash scripts/verify.sh` in default/full mode again. Attempt2 is in progress; its successful completion is not yet claimed. The independent current source scope retains 110 paths with the canonical child name.

## Attempt 2 — Startup Diagnostic Precedence Failure

Receipt recorded 2026-10-11T09:19:21Z. Default/full exited **101** after **24m 0.476s (1,440,476 ms)**. Panic policy, all source/mutation/architecture/managed checks, strict workspace Clippy and all-target build passed. Node tests reported **1,631 passed, one failed and three existing opt-in ignores**; later RPC/benchmark/Bazel/coverage stages were not reached.

The sole failing test was `sync::tests::filter_index::startup::filter_index_production_reopen_refuses_each_lone_partial_artifact`. Its projection-only/no-state case still refused with typed BlockIndex corruption and Repair, but unconditional forest measurement moved `missing BASIC suffix projection record` before the established `BASIC state is absent but index artifacts exist` diagnostic. The original test also protects unchanged namespace/body/undo state. Plan02 was reopened to restore the bounded absent-state/artifact diagnostic order without weakening assertions, adding a scan, or changing present-state validation order. Full native and phase approval remain pending a successful rerun.

## Startup Classification Correction and Attempt 3

Correction released 2026-10-11T09:27:15Z. The original production lone-artifact test is unchanged and passes all three artifact variants, exact typed diagnostic and no-effects assertions. All45 startup regressions and22 recovery/cumulative controls pass, including strengthened proven-empty zero-scan evidence; strict node lint, format and parity also pass. Root refreshed the five affected historical guard paths and added ten isolated controls. Final combined suites pass283 tests/564 assertions; all related live checkers and managed checks pass.

After actual Cargo formatting and worktree LOC regeneration, root started the third complete default `bash scripts/verify.sh` run. Full success is not yet claimed. Independent source/security/formal reinspection is being refreshed for this narrow correction.

## Attempt 3 — Legacy Additive Status-Key Expectation

Receipt recorded 2026-10-11T09:56:45Z. Default/full exited **101** after **27m14.308s (1,634,308ms)**. Node passed **1,632 tests, zero failures, three existing ignores**; RPC library passed348 and daemon passed87. The exact prior startup diagnostic regression passed in this full run. The black-box parity suite then reported2pass/1fail: `phase127_production_composition_shares_sync_serving_and_operator_authority` still expected seven network-status keys, omitting the new additive `basic_index` eighth key. No later benchmark/Bazel/coverage success is claimed.

Plan05 was reopened for the existing black-box assertion only, preserving every prior key/authority check and adding BASIC authority equality where available. Root replanned late-stage verification: after the exact fix, preflight the remaining workspace (excluding the already-passed unchanged node crate), explicit CLI client binary, benchmarks, Bazel and pure-core coverage before the next mandatory complete native run. This does not waive the full contract or final mandatory hook. Independent source/security/formal scope must include the added existing test path.

## Late-Stage Preflight Passed; Attempt 4 Intentionally Stopped

Receipt recorded 2026-10-11T10:13:51Z. The corrected black-box target passed3/3; remaining workspace excluding the already-passed unchanged node passed2,596 (2,577tests +19doctests), zero failures/ignores across38 result groups; explicit CLI client passed7/7. Strict RPC lint, scoped formatting, parity and the new operator guard passed.

Root tail preflight passed all timed parts with exit0: benchmark list362ms, smoke5786ms, report192ms, six Bazel targets9059ms, provenance877ms, coverage clean633ms and the exact native pure-core coverage helper24581ms. Its nonempty text report has no Uncovered Lines section. These are additional late-stage checks, not a substitute for one clean complete default run.

After starting full attempt4, root proactively ran the old Phase127 guard against the changed black-box test. It reported15pass/2fail out of17 and rejected its own hard-coded seven-key status array. The intended additive BASIC group requires the exact eight-key array. Root intentionally stopped attempt4 **before that known later guard step**, sending SIGTERM only to its verified live Bun test child; the normal verifier/runner exited **143 after4m43.445s (283,445ms)** and released its lock. This was not a timeout, stall termination, direct Phase127 result from that native run or a Rust behavior failure.

The narrow guard repair must require all eight exact sorted keys, preserve every other production-authority/restart/redaction control and add an omission-negative. Two existing Phase127 script paths expand independent scope to113. Full default attempt5 and final mandatory hook remain required; no success, staging, requirement activation, commit or push is claimed.

## Exact Legacy Guard Correction and Attempt 5

Correction released 2026-10-11T10:18:52Z. The Phase127 checker now requires the exact eight sorted keys, preserving all seven earlier keys and every other shared-mutation/restart/authority/redaction anchor. An isolated omission control first proves the live fixture passes, then removes only BASIC and requires the existing proof failure. The final suite passes18 tests/19 assertions, zero failures; live checker, managed checks and scoped diff review pass.

Root reran actual Cargo formatting and regenerated worktree LOC, then started the fifth complete default `bash scripts/verify.sh` run. Independent scope is113 paths. All earlier failed/aborted attempts and successful split preflights remain historical evidence; a successful complete default run and mandatory commit hook are still required.

## Attempt 5 — Complete Default Contract Passed

Final receipt recorded 2026-10-11T10:45:04Z. The complete default `bash scripts/verify.sh` exited **0** in **27m41.348s (1,661,348ms)**. All source/mutation/claim/architecture/parity/managed/panic-policy checks, strict workspace Clippy, all-target build, full workspace tests, explicit CLI client binary, benchmark list/smoke/report, six Bazel targets/provenance and the zero-uncovered pure-core coverage policy passed.

Before coverage reruns, the log records **4,247 primary Rust passes, zero failures, three existing opt-in ignores across41 result groups**. It records **4,181 Bun pass records**, not a deduplicated whole-project test count. Coverage reruns are excluded from the primary Rust total. The native log is `/tmp/phase161-native-full-5.log`; ignored checkout-local `verify-full` timing receipts independently record success/exit0. The first three failed attempts and intentionally aborted fourth run remain above.

This is the successful frozen-source precommit gate. Independent final security/goal/lifecycle closure follows; the mandatory hook will regenerate final-index LOC and rerun the complete contract. Actual hook/commit/push outcomes are derived from the saving commit, tool receipt and upstream refs rather than preclaimed here. Phase162 and v2.5 completion remain pending.
