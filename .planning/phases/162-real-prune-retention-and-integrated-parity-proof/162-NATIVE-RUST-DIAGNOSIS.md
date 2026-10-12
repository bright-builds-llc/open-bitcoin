---
generated_by: gsd-debugger
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-12T02:12:32Z
status: self_verified
---

# Native4 Rust Persist Structural Guard Diagnosis and Repair

## Diagnosis Plan

- [x] Reproduce the exact existing chainstate Rust failure with pinned Bun timing wrapper and exclusive Cargo ownership.
- [x] Trace actual persist/delegate/lifecycle policy and approved Plan14/15 guard requirements.
- [x] Inspect bounded adjacent include_str guards for the same old direct-call assumption.
- [x] Present concrete test-only repair and focused/full verification plan before changing Rust.

Initial ownership is only this durable record and the ignored debug session. Runtime, transport, recipes, current documents/registry/requirements and six source anchors stay unchanged. Parent owns independent acceptance, updated68-file review inventory, native/formal closure and all Git. No new Rust file or breadcrumb is needed for an accepted modification to the existing test.

## Initial Failure

Native4 `/tmp/phase162-native-full-4.log` records all Bun, format, strict Clippy and all-target build passed, then chainstate327pass/1fail,0ignored,0filtered in0.01s. The sole failing test is `coins::tests::flush::managed_chainstate_persist_calls_decide_flush_and_does_not_write_snapshot`, expecting a direct execute_flush IfNeeded call in the whole node chainstate source. The default verifier exited101after957941ms before node/RPC tests. Actual source must be inspected before treating this as a stale structural obligation.

## Exact RED Receipts

Both Cargo commands ran exclusively through `PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH bun run scripts/command-timings.ts run --key KEY --`, with pinned Rust1.94.1, default packages/target and existing warm Native4 artifacts.

1. Key `phase162-native-rust-persist-red`, command `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate --lib --all-features coins::tests::flush::managed_chainstate_persist_calls_decide_flush_and_does_not_write_snapshot -- --exact --nocapture`:0pass/1fail/327filtered,0.00s libtest,189ms wrapper,exit101. Log `/tmp/phase162-native-rust-persist-red.log`; timing receipt `.local/open-bitcoin-dev/command-timings/phase162-native-rust-persist-red/2026-10-12T02-03-01.304Z-3588b648-decc-4ff1-b24d-503e566f9d96.json`.
1. Key `phase162-native-rust-adjacent-red`, command `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib --all-features storage::fjall_store::tests::coins_migration::persist_progress_source_does_not_write_leftover_snapshot -- --exact --nocapture`:0pass/1fail/1698filtered,0.00s libtest,317ms wrapper,exit101. Log `/tmp/phase162-native-rust-adjacent-red.log`; receipt `.local/open-bitcoin-dev/command-timings/phase162-native-rust-adjacent-red/2026-10-12T02-03-48.517Z-1765c874-1942-43bc-bf6a-3516a3001400.json`.

Both commands completed immediately; no ongoing Cargo session remains. The adjacent node failure is the same raw source substring assertion at coins_migration.rs595, discovered before another full native attempt.

## Root Cause and Bounded Adjacent Scope

Actual helper spelling is `flush_with_mode`. The ordinary path is `persist → flush_with_mode(IfNeeded) → flush_applying_plan → flush_applying_plan_with_header_checkpoint → FlushLifecycle::execute_flush_applying_plan_with_header_checkpoint`. Rustfmt splits `self`, `.flush_lifecycle` and the long method across lines. Both old whole-file guards require the obsolete contiguous `self.flush_lifecycle.execute_flush` substring; it is absent despite the correct caller path.

Read-only `/tmp/phase162-native-rust-probes.log` uses the existing ordinary Rust parser and confirms each actual four-hop edge, IfNeeded forwarding, no leftover snapshot, unchanged decide_flush mode policy and selected prefix/checkpoint/prune ordering. No runtime workaround is indicated. Plan14 explicitly preserves selected policy/no-op/action ordering and typed failures; Plan15 requires a meaningful whitespace-stable scoped Rust guard, complementary comment/literal/ownership-aware TS mutations and no save_snapshot. Its recovery_and_persist.rs guard was already updated and must remain unchanged.

Repository source search found exactly two stale raw-call guards: the failing core file and the existing final node coins-migration source test. Adjacent node chainstate tests retain lifecycle presence/readiness and function-scoped Always/Periodic exclusion, while restart/durable_sync/wallet guards inspect runtime-state no-save/no-seed behavior rather than the stale call string. Replay/initialize tests inspect the pure coins policy source. Those paths need verification, not edits.

## Concrete Proposed Repair

Parent accepted this proposal, obtained actual independent plancheck PASS and explicitly authorized only these two existing test edits before implementation.

1. Existing `packages/open-bitcoin-chainstate/src/coins/tests/flush.rs`: replace only the stale test's whole-file positive check with whitespace-normalized, unique-function/body extraction following the already-approved Plan15 local pattern. Assert persist IfNeeded, mode/plan forwarding and private actual lifecycle invocation within their owning functions. Retain the no-leftover-snapshot assertion, AAA organization and test name. This strengthens the old unrelated whole-file IfNeeded search rather than weakening it.
1. Existing `packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs`: normalize whitespace only for chainstate include_str text in its final test, leaving every durable_sync/no-save/no-seed/execute assertion unchanged. It is597lines and gains roughly4, staying under the managed628-line limit; no helper/module/source expansion is needed.

No runtime/transport/recipe, original Plan15 Rust guard, TS checker/test, current docs/proof, registry/requirements or six-source edit is needed. No new Rust file means no new breadcrumb mapping. Parent adds both actual existing paths to the69-file independent review/evidence inventory.

After acceptance: exact2GREEN, fullchainstate328tests, bounded existing node coins_migration/flush_lifecycle/chainstate source/durable behavior selectors, scoped Rust format plus strict affected Clippy, live guards/managed/parity/diff and frozen-source hashes. Parent owns remaining-workspace/CLI/bench/Bazel/coverage tail preflight and full Native5; none substitutes for native closure. Keep all three requirements Pending, no Git or hook bypass.

## Pre-Repair Hashes

- Core existing test: `b78568cb2632219a061efa1491f26a19fa5d3104aed5f04218a2df1eb5ffe2b4`.
- Node existing test: `b32071f4df5edd8c8b3de584448649f7571d2abf3fe1e7867062bf3761c223c7`.
- Unchanged runtime chainstate: `61f9b1fb5d5f3e0643ffeaa12a93f2651cd2a82ee90a5e81694ed551d3ae4a7f`.
- Unchanged runtime lifecycle: `3b31f7b4b0489992e547079e549b808e56d1f945ffd8ae70e9a73bfd3fe89324`.
- Original Plan15 Rust guard: `bf7d12d4a795c90e2b1448d5446f5dc170e9945e3463a4c781c00dfc45a3337b`.

Those hashes describe the diagnosis stage before the explicitly accepted repair below.

## Authorized Repair and Measured Verification

- [x] Core guard uses the actual unique-function, normalized four-hop IfNeeded path; original full-source snapshot prohibition and AAA/test name remain.
- [x] Node change only normalizes the existing chainstate include_str source; every existing assertion remains byte-for-byte identical.
- [x] Actual full cargo fmt first, exact2GREEN, strict affected Clippy, fullchainstate and bounded related node checks ran serially with timing wrappers.
- [x] Final live/parity/managed/scoped-format/diff and source-preservation checks passed; Cargo idle handed to parent.

All commands used the pinned PATH and timing-wrapper shape above. Receipt files are under `.local/open-bitcoin-dev/command-timings/<key>/` with logs `/tmp/<key>.log`; exact GREEN logs use `/tmp/phase162-native-rust-persist-green.log` and `/tmp/phase162-native-rust-adjacent-green.log`.

| Stable timing key                           | Actual command after cargo and manifest                                                            | Result                    | Libtest/tool time         | Wrapper  |
| ------------------------------------------- | -------------------------------------------------------------------------------------------------- | ------------------------- | ------------------------- | -------- |
| phase162-native-rust-format                 | fmt --all                                                                                          | exit0                     | No separate Cargo summary | 1,391ms  |
| phase162-native-rust-persist-green          | exact original core test                                                                           | 1pass/0fail/327filtered   | 0.00s                     | 2,677ms  |
| phase162-native-rust-adjacent-green         | exact adjacent node test                                                                           | 1pass/0fail/1698filtered  | 0.00s                     | 7,650ms  |
| phase162-native-rust-clippy                 | clippy -p open-bitcoin-chainstate -p open-bitcoin-node --all-targets --all-features -- -D warnings | exit0                     | 7.65s                     | 7,867ms  |
| phase162-native-rust-chainstate-full        | test -p open-bitcoin-chainstate --lib --all-features -- --test-threads=1                           | 328pass/0fail/0filtered   | 0.04s                     | 290ms    |
| phase162-native-rust-node-coins_migration   | node lib/all-features selector coins_migration                                                     | 15pass/0fail/1684filtered | 7.90s                     | 8,088ms  |
| phase162-native-rust-node-flush_lifecycle   | node lib/all-features selector flush_lifecycle                                                     | 71pass/0fail/1628filtered | 32.47s                    | 32,620ms |
| phase162-native-rust-node-chainstate--tests | node lib/all-features selector chainstate::tests::                                                 | 14pass/0fail/1685filtered | 0.01s                     | 145ms    |
| phase162-native-rust-node-persist_progress  | node lib/all-features selector persist_progress                                                    | 5pass/0fail/1694filtered  | 3.00s                     | 3,137ms  |

The format command was exactly `cargo fmt --manifest-path packages/Cargo.toml --all`. Every Cargo test command used `--manifest-path packages/Cargo.toml`; node selectors used `-p open-bitcoin-node --lib --all-features SELECTOR -- --test-threads=1`. No command ran concurrently against the target directory. Format session97134, exact-GREEN session8435 and serial affected-verification session41112 all exited0. At02:10:36Z parent received an immediate Cargo-idle handoff and could begin its independent remaining-workspace/tail preflight. Child performed no subsequent Cargo operation.

Final `/tmp/phase162-native-rust-live-managed-parity.log`: existing live154–162guards pass, parity breadcrumbs verified for1,141Rustfiles, managed all0findings. Scoped rustfmt `--edition 2024 --config skip_children=true --check` on both owned test paths and `git diff --check` pass. The same exact two source hashes remain frozen after all checks.

## Byte Preservation and Simplification

Before editing, an actual roster/hash snapshot captured1,464current first-party .rs/.ts/.py files under packages/scripts, including untracked current source and excluding pinned Knots. Final `/tmp/phase162-native-rust-invariants.log` proves the roster is unchanged and exactly the two authorized test files differ. Every other1,462source file, including production runtime, original Plan15 guard, TS checkers/tests, transport and recipes, is byte-for-byte unchanged.

The node inverse replaces only its four-line normalized source declaration with the old one and reconstructs the entire original file exactly. Core before/after comparison confines changes to its one named existing test; raw source confirms AAA/name and the exact original full-source no-leftover-snapshot assertion. An initial scratch invariant incorrectly searched masked parser output for AAA comments; correcting that probe to inspect the raw owning function made the intended check pass. No repository source change was needed for this probe correction.

Explicit simplification keeps the already-approved local Plan15 extraction pattern in the existing core test and only whitespace normalization in the node guard. No new Rust file, module, public/test helper API, dependency, production policy or breadcrumb mapping is introduced. Final file lengths are220/600, both below managed limits. Diff is43insertions/5deletions in the core test and4insertions/1deletion in the node test; no broad normalization remains.

Final SHA256:

- Core existing test: `f36c460d84b7eefdac849d4afff474b38fbdccde1585cdccd5971a5496b73c4e`.
- Node existing test: `1144bdd64916361ba1f523c9c30e6842a194dddf07a0aa3100479d0fa4d5f843`.
- Six-source ordered digest remains `a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575`.

The prior Phase157row/gap repair is unchanged. Whole index equality is intentionally not asserted: parent explicitly added the two actual existing test evidence paths and updated its69-file independent review inventory. Parent owns those registry/scope/LOC changes; they are outside this child repair.

## Parent Closure Still Required

All three requirements remain Pending and current162owner in_progress. Parent owns the69-file independent source/security refresh, remaining-workspace/explicit CLI/bench/Bazel/coverage tail preflight, full default Native5, formal verification/activation and Git finalization. Tail preflight is not a replacement native pass. This record claims only the actual scoped checks above, with runtime/source6 unchanged; no full-native/formal completion, stage/commit/push/branch/hook bypass or debug archival occurred. All child execution sessions are idle and no further source/Cargo changes are planned.
