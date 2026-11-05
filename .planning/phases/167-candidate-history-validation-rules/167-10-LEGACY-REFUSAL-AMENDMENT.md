---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-22T00:10:35Z"
commits: false
---

# Phase167 Plan10 legacy refusal test-owner amendment

Add only packages/open-bitcoin-node/src/sync/canonical_genesis/tests.rs to existing Task1. The file is427physical lines/14601bytes at SHA2563d56611964cc0be077e230fc6842af33d8fd347199e6ebb274f7ecf6f4247bcc. This is a compatibility-test ownership correction, not a production change or new task. Root requires its regression closure before accepting10 and must independently check this amendment before test execution. Earlier167-10-CONNECT-CALLERS-AMENDMENT.md remains unchanged historical evidence; this amendment supersedes only its11-owner allocation with12.

## Actual failure and source basis

The current completed11-owner implementation is frozen against plan10/final-source-closure.json, completed2026-10-22T00:07:12.466Z. All11current source hashes match that closure. Its24min47s interval is a pre-regression-closure checkpoint, not the final10duration or acceptance. Root owns final metrics and actual changed/created counts.

Retained final-task1.log and final-task2.log show13/13 managed-connect and6/6 BASIC-continuation cases; validation-history-regression.log shows30/30 and clippy.log strict node success. Root also reports28/28 canonical-admission and scoped style success. These results remain attributable to their unchanged11source basis; they do not establish that this remaining compatibility regression passed.

canonical-genesis-regression.log records16discovered,15passed,1failed,0ignored. The failing test is sync::canonical_genesis::tests::phase166_runtime_descendant_routes_refuse_without_owner_or_storage_effects at tests.rs:211. It expects error.to_string().contains("unsupported-validation-scope") for all four proved selectors. Accepted09 intentionally qualified actual proved Regtest header ingress. The failure occurs on the Headers assertion before any10connect caller. The log prints only the failed assertion, not the actual returned error, so no precise error category has yet been observed or claimed here.

The fixture opens each actual proved genesis but never connects peer1. It changes a clone of the genesis block's previous hash to form its alleged child; that child was not contextually admitted. Consequently this test proves refusal/effect isolation for this setup, not genuine candidate-context acceptance/rejection. Existing09/10genuine positives and contextual controls remain mandatory and passing on their recorded source. Neither the old assertion nor this metadata correction authorizes a production change to force blanket refusal.

## Exact test-only contract

Only the existing named failing test may change. Preserve all other tests and helper behavior in tests.rs byte-for-byte, including the shared SELECTORS constant and its four Mainnet/Testnet3/Signet/Regtest entries. Do not delete, skip, early-return or silently omit Regtest; do not introduce an ignore or an arbitrary is_err substitute for its exact refusal controls.

Inside the failing test preserve:

- The loop over all four selectors, genuine root/runtime setup, unconnected peer1, unchanged child construction and absence of contextual admission.
- Every Headers/Block/CompactBlock/BlockTxn message and each receive_sync_message, receive_message and receive_message_for_durable_serving route, retaining the exact message bytes, parameters and loop coverage.
- All stored-body, local-body, fixture-header and reorg calls after the message loop, and their effect assertions.
- The complete chainstate snapshot equality, including accepted chain/exact work, coins, full undo and coins best block; exact headers, peer snapshot and full durable rows equality after every call. Retain every assert_unchanged invocation and the panic-on-durable-payload callback.
- Cleanup/all-handle lifecycle and every existing unsupported-selector condition/effect guard. Mainnet/Testnet3/Signet retain their original exact unsupported semantics; genuine Regtest support is not reverted.

After independent release, first observe actual errors from current immutable production for the genuinely qualified Regtest iteration. Bounded diagnostic observations may modify this approved test only; retain complete output and distinguish diagnostic observation from final assertions. Determine the precise existing missing-peer, proof, membership/context or fixture/preference refusal category for each actual route as applicable, using observed typed errors or specific stable categories. These are possible source boundaries to investigate, not observed outcomes or strings to guess. Do not print a guessed category into a test and call its failure evidence.

Replace stale Regtest blanket-scope assumptions with strong exact-category assertions grounded in those observations. Assert the precise error for each message/entrypoint and other refusal route as appropriate; do not mask distinct categories with broad substrings, catch-all patterns or arbitrary is_err. Keep the original unsupported assertion for the other three selectors and all effect guards. Restore final exact assertions after diagnostics; temporary diagnostic-only code is not completion. No production11owner, global selector array, source guard, harness, threshold, deadline, dependency or proof owner may change.

The initial recorded failure is evidence of stale blanket support semantics, not a claim that the unknown Regtest error itself is a new production bug. If actual diagnostics instead expose a genuine production issue or an effect guard fails, stop with the exact observation and request a separate checkpoint; do not expand this test-only authorization or weaken the guard.

## Verification after independent release

Keep both original Plan10commands and all already-recorded outcomes unchanged. Run the exact failing test first with nonzero discovery and retained diagnostics, then its final exact assertions. Run the complete sync::canonical_genesis:: regression after correction; all16existing discovered cases must pass with no new ignore/skip. Serial pinned-Bun command-timings forms are:

```bash
bun run scripts/command-timings.ts run --key phase167-10-legacy-refusal -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::canonical_genesis::tests::phase166_runtime_descendant_routes_refuse_without_owner_or_storage_effects -- --exact --test-threads=1 --nocapture
bun run scripts/command-timings.ts run --key phase167-10-canonical-genesis-regression -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::canonical_genesis:: -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-10-legacy-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings
```

Use the plan's existing pinned Bun1.3.9 PATH, targets/cooperative lock, physical temporary directory, complete outcomes and within60s liveness/polling contract. Run scoped Rust formatting/whitespace/source review and final source closure covering all12owners; compare all previous11hashes to their frozen current implementation. Review the new test diff to prove only the named function changed and SELECTORS/all other tests/helpers remain exact. No broad repeat of already-passing unchanged-source checks is required solely for this test correction; new failures or source changes require appropriate fresh verification. Root's full20native/freshcoverage/Bazel and independent review/lifecycle requirements remain.

## Bounded scope, decisions and schedule

The only added owner is the existing427-line compatibility test file. The plan union changes11→12, below hard15. Root explicitly accepts this bounded scope warning to close the stale support assertion while preserving all original feature/tasks and strong refusal checks. The earlier\<=9target exception and11-owner caller history are preserved, not overwritten; no comparable relaxation is granted to another plan or future owner. Task1 has eight files, Task2 its unchanged four; substantial implementation is already frozen. No new plan/task/wave/dependency is introduced:27plans/46tasks/22waves,10wave11after08/09/27.

| Decision | Plan/task coverage                      | Status          | Test amendment effect                                                                             |
| -------- | --------------------------------------- | --------------- | ------------------------------------------------------------------------------------------------- |
| D-01     | Existing phase owners;10.1 controls     | Full            | Distinguish qualified Regtest from genuinely unsupported selectors without deleting any selector. |
| D-02     | Existing01/06/23/15;10.1                | Full, unchanged | No monetary, maturity, root or parameter change.                                                  |
| D-03     | Existing candidate-history owners;10.1  | Full, unchanged | Unadmitted cloned child is not relabeled contextual evidence.                                     |
| D-04     | Existing interpreter/flags owners;10.1  | Full, unchanged | Actual selected script behavior and passing controls remain.                                      |
| D-05     | Existing time/difficulty owners;10.1    | Full, unchanged | No clock, ancestry, proof or difficulty authority changes.                                        |
| D-06     | Existing07/08/27/16;10.1                | Full, unchanged | Complete undo/effect equality remains mandatory.                                                  |
| D-07     | Existing integration owners;10.1–2      | Full            | Refusal setup stays distinct from actual qualified positive routes.                               |
| D-08     | Existing affected owners;10.1–2         | Full            | All accepted coins/work/header/peer/durable effect guards retained.                               |
| D-09     | Existing evidence owners;10local checks | Full            | Retain actual failure, observe diagnostics and preserve11-source provenance.                      |
| D-10     | Existing17–20/25;10local checks         | Full            | Final12-owner checks supplement unchanged full acceptance gates.                                  |

All five original Plan10requirements CVLD-01/CVLD-02/CVLD-03/CVLD-04/CVLD-06, must-haves and original Task1/Task2behavior/actions/acceptance/commands/done remain exact. Task2whole XML remains exact. No approved requirement is narrowed or closed by this planning amendment.17independent source review still precedes current guard renewal;19owns breadcrumb/parity/docs;20retains full native/freshcoverage/Bazel/canonical independent source/security/goal/lifecycle and root normal-hook save. Compact fallback remains separately mandatory17/20review and adds no compact owner here. No168work.

## Current immutable snapshots and reversible metadata

At2026-10-22T00:10:35.051Z, amend10legacy/before/ captured all27current plans,46task blocks/all commands, current11source bytes and the new test file, exact failing function and SELECTORS block, plus retained failed log and11-owner closure. before-manifest.json records full paths, hashes/bytes/lines and test-prefix/suffix hashes. The failing function is4802bytes at SHA2566e24355454ee0a9ebe4f5cf81df58546c9dfa54d493bac889fe0a8e89bf37a18. This basis is the completed10implementation, not the earlier pre10snapshot from amend10callers; no frozen receipt is rewritten.

Only this new amendment and four bounded10plan additions may change: one frontmatter owner, one Task1files entry, one Task1read-first insertion and one outside-task context paragraph. Removing exactly those additions restores the complete current original10file. Every other26plan and the previous caller amendment remain byte-exact, as do original task contracts, Task2XML, lifecycle/generated_at, waves/dependencies/requirements/must-haves. The lifecycle remains yolo167-2026-10-21T16-38-23 although this amendment is October22UTC/October21local.

All12source inputs are read-only in this planner pass and source-preservation claims are against this current coordinated snapshot only. No source, Cargo/Bazel/test/Clippy/rustfmt, Git saving, guard renewal, frozen evidence/threshold/deadline change, shared STATE/ROADMAP/REQUIREMENTS/config/todo or PLAN-CHECK edit occurs here. Root controls independent release and final acceptance.

Previously loaded AGENTS.md local guidance/Bright Builds sidecar/overrides, architecture/code-shape/testing/verification/Rust standards and current167context/research inform this narrow test correction and observed exact errors. Active lessons remain fully loaded at8856bytes/2953conservative tokens with the existing October19baseline; no archive/audit/new skill/library research is needed. Simplification preserves one existing regression and its full setup/effect guard rather than manufacturing a replacement fixture or changing production to satisfy a stale assertion.

Planner static validation and exact inverses are retained separately under .local/open-bitcoin-dev/phase167/amend10legacy/. They establish metadata preservation only; diagnostics and final regression evidence remain future executor work after independent checking.

## Actual planner checks completed

All27GSD frontmatter and27XML structure checks pass with46tasks/22waves and errors:[]. Static dependency/owner checks find no missing/cyclic/later prerequisite, same-wave overlap or unordered shared owner. Frozen02's existing empty-dependency/wave advisory remains; the explicit12-owner10scope warning is retained rather than hidden by a changed threshold.

Every original behavioral contract/automated command is exact, Task2whole XML is exact, and the complete10file restores byte-for-byte by removing only the four enumerated additions. All26other plans and every captured prior planning artifact, including the11-owner caller amendment, remain exact. All12current source inputs, the failed regression log and source closure match their snapshots; the original11match the executor's00:07:12.466Zclosure. The exact failing-test block, its whole-file prefix/suffix and SELECTORS all remain unchanged. No earlier pre10source hash is substituted for this implemented basis.

The new amendment initially failed the available mdformat check and is then formatted only within this new file using the already-available GFM/frontmatter-compatible formatter. The existing plan is not whole-file formatted, preserving its original task bytes. Final artifact checks cover top-only frontmatter delimiters, whitespace and exact output hashes in planner-validation.json. No source diagnostics were executed by this planner, no precise Regtest category is asserted as observed, and no execution release or10acceptance is inferred from these metadata checks.
