---
status: automated_verified
trigger: "Investigate and fix failing GitHub Actions for Open Bitcoin... Bright Builds Checks fails file-lengths"
created: 2026-09-21T02:47:41Z
updated: 2026-10-13T19:19:24Z
---

## Current Focus

<!-- OVERWRITE on each update - reflects NOW -->

hypothesis: Confirmed — ten tracked sources exceeded the Bright Builds 628-line hard gate; repo-native check-file-lengths.sh cannot see test/script offenders.
test: Local red-capable loop was `bun scripts/bright-builds-check.ts all`; after splits it reports findings=0.
expecting: Historical remote uncertainty is superseded by the 2026-10-12 read-only recheck of successful CI and Bright Builds at exact d47f6e09b4621808974cc083fcc27ee9d2c1b4c7.
next_action: Preserve the original dated failures and historical e0 acceptance below. The current 21-input source14fa passed the full default verifier, four complete layout suites, independent acceptance, functional activation, renewed audit, readiness execution and final5/5verification. Strict normal-hook saving is the wrapper gate; actual Git outcomes are derived from its receipts and repository state. The old d47f6e09 remote CI observation remains dated evidence; no new remote CI result is inferred.
known_pattern_candidate: none

## Symptoms

<!-- Written during gathering, then IMMUTABLE -->

expected: Bright Builds Checks and CI pass on push to main / tag v2.3 after `bun scripts/bright-builds-check.ts all` and `bash scripts/verify.sh`.
actual: Workflow `Bright Builds Checks` fails in ~8-15s on every recent push (including v2.3 archive commits). The main `CI` workflow has been succeeding. This is pre-existing since at least Phase 142, not caused by the v2.3 archive itself.
errors: GitHub Actions run 35555166687 (main push of `chore: remove REQUIREMENTS.md for v2.3 milestone`) failed job `bright-builds-checks` with 10 file-lengths findings at 629+ physical lines. Same failure on tag push run 35555166833.
reproduction: From repo root: `bun scripts/bright-builds-check.ts all` (this is the CI command in `.github/workflows/bright-builds-checks.yml`). Do NOT treat `bash scripts/verify.sh` passing as proof the CI job passes.
started: Bright Builds Checks has failed on main pushes for Phases 142, 143, 144, 145, and the v2.3 archive. Main `CI` workflow has been green.

## Eliminated

<!-- APPEND only - prevents re-investigating -->

- hypothesis: Bright Builds Checks fails because of a missing .bright-builds-rules-checks.tsv allowlist that CI requires.
  evidence: Local bun scripts/bright-builds-check.ts all reproduced the exact 10 findings with exceptions=0. The files themselves exceed 628 lines; an allowlist would only hide the overflow, not the mismatch with verify.sh.
  timestamp: 2026-09-21T02:48:30Z
- hypothesis: The v2.3 archive or REQUIREMENTS.md deletion caused the Bright Builds failure.
  evidence: The same 10 files fail locally on the current tree; CI has been red since Phase 142. verify.sh still excludes these test/script paths.
  timestamp: 2026-09-21T02:48:30Z
- hypothesis: Phase 129 FLOW-03 unit-anchor tests must live in announcement_transport_cases.rs root text, not child modules.
  evidence: The live Phase 129 checker already uses readSourceCorpus. Adding announcement_transport_cases.rs to SPLIT_CORPUS_FILES made the fixture concatenate children into the root path; all 12 Phase 129 tests then passed without moving the tests back.
  timestamp: 2026-09-21T04:00:00Z

## Evidence

<!-- APPEND only - facts discovered -->

- timestamp: 2026-09-21T02:47:41Z
  checked: knowledge-base.md
  found: No keyword overlap with file-lengths / Bright Builds Checks / 628-line gate.
  implication: Treat as novel; do not reuse prior debug hypotheses.
- timestamp: 2026-09-21T02:48:20Z
  checked: bun scripts/bright-builds-check.ts all
  found: Exact same 10 FAIL file-lengths findings as GitHub Actions run 35555166687; scanned=1146 exceptions=0 findings=10.
  implication: Local red loop matches CI. Splitting those 10 files is the fix.
- timestamp: 2026-09-21T02:48:20Z
  checked: scripts/check-file-lengths.sh and .github/workflows/bright-builds-checks.yml
  found: CI runs bun scripts/bright-builds-check.ts all. Repo-native checker greps packages .rs and excludes /tests.rs$|/tests/. verify.sh is 638 lines and is itself a Bright Builds offender.
  implication: Confirmed checker mismatch. Do not treat verify.sh passing as Bright Builds green.
- timestamp: 2026-09-21T03:00:00Z
  checked: Phase 73/103/111/128/129 checkers vs split candidates
  found: Phase 73 reads snapshot_codec/tests.rs as a root-only file for chainstate_snapshot_round_trips_through_storage_dto, and concatenates restart_chainstate.rs (not its grandchildren) for three reopen test names. Phase 129 uses readSourceCorpus for announcement_transport_cases.rs. Phase 128 lists that file but does not inspect its body. Phase 111 reads network/tests.rs via corpus, so block_serving grandchildren remain visible. include_str! paths in flush.rs and block_serving.rs are relative to those roots.
  implication: Keep Phase 73 needles and include_str tests in their current roots; other tests can move to foo/ children.
- timestamp: 2026-09-21T03:50:00Z
  checked: cargo test -p open-bitcoin-node after first split
  found: include_str! paths copied into flush_lifecycle/tests/initialize.rs, recovery_and_persist.rs, and restart_chainstate/persist_and_hydrate.rs were one directory too shallow (or pointed at sync/sync/). rustc suggested the extra ../ for the flush_lifecycle paths.
  implication: Relative include_str! must be rewritten when tests move into a child directory.
- timestamp: 2026-09-21T04:05:00Z
  checked: bun scripts/bright-builds-check.ts all after splits
  found: SUMMARY file-lengths scanned=1168 exceptions=0 findings=0; SUMMARY all findings=0.
  implication: Local Bright Builds gate matches the CI command and is green.
- timestamp: 2026-09-21T04:05:00Z
  checked: targeted cargo/bun tests and repo-native length check
  found: chainstate 94 tests ok; node 126 matching tests ok; Phase 73/111/128/129/134 bun tests 287 pass; parity breadcrumbs 866 files; check-file-lengths.sh 398 production files passed.
  implication: Splits preserved behavior. GitHub Actions remains unverified until push.

## Resolution

<!-- OVERWRITE as understanding evolves -->

root_cause: Tracked test and script files grew past the Bright Builds 628-line hard gate. bun scripts/bright-builds-check.ts all fails those paths in CI, while bash scripts/check-file-lengths.sh only checks production Rust and excludes /tests.rs$ and /tests/, so local verify/pre-commit stayed green.
fix: Split the 10 oversized files into foo.rs + foo/ behavior-grouped children below 628 lines, without allowlisting them. Sourced verify.sh helpers into scripts/verify/helpers.sh. Added announcement_transport_cases.rs to Phase 129 SPLIT_CORPUS_FILES so fixtures concatenate child modules. Corrected include_str! depths after the moves.
verification: Local `bun scripts/bright-builds-check.ts all` reports findings=0. Targeted cargo tests, bun phase checkers, parity breadcrumbs, and `bash scripts/check-file-lengths.sh` passed. GitHub Actions has not been re-run.
files_changed:

- docs/parity/source-breadcrumbs.json
- packages/open-bitcoin-chainstate/src/coins/tests.rs
- packages/open-bitcoin-chainstate/src/coins/tests/cache_lookup_and_view.rs
- packages/open-bitcoin-chainstate/src/coins/tests/flush.rs
- packages/open-bitcoin-chainstate/src/coins/tests/flush/matrix_modes.rs
- packages/open-bitcoin-chainstate/src/coins/tests/flush/none_mode.rs
- packages/open-bitcoin-chainstate/src/coins/tests/flush/policy_time_and_accessors.rs
- packages/open-bitcoin-chainstate/src/coins/tests/view_errors_size_and_admission.rs
- packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo.rs
- packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/apply_and_connect_errors.rs
- packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/connect_snapshot_and_cache.rs
- packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/staged_connect_and_reorg.rs
- packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests.rs
- packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush.rs
- packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/initialize.rs
- packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/recovery_and_persist.rs
- packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs
- packages/open-bitcoin-node/src/network/tests/announcement_transport_cases/classification_and_outbox.rs
- packages/open-bitcoin-node/src/network/tests/announcement_transport_cases/receipt_credit.rs
- packages/open-bitcoin-node/src/network/tests/announcement_transport_cases/tx_emission.rs
- packages/open-bitcoin-node/src/network/tests/block_serving.rs
- packages/open-bitcoin-node/src/network/tests/block_serving/getdata_serving.rs
- packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs
- packages/open-bitcoin-node/src/storage/snapshot_codec/tests/chainstate_wallet_and_headers.rs
- packages/open-bitcoin-node/src/storage/snapshot_codec/tests/mempool_codec.rs
- packages/open-bitcoin-node/src/sync/tests/restart_chainstate.rs
- packages/open-bitcoin-node/src/sync/tests/restart_chainstate/download_and_invalid_body.rs
- packages/open-bitcoin-node/src/sync/tests/restart_chainstate/persist_and_hydrate.rs
- scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.test.ts
- scripts/check-phase134-authoritative-lifecycle.test.ts
- scripts/check-phase134-authoritative-lifecycle.test/fixture.ts
- scripts/check-phase134-authoritative-lifecycle.test/mutations.ts
- scripts/verify.sh
- scripts/verify/helpers.sh

## Automated evidence disposition | 2026-10-12

The original ten length failures at runs 35555166687/35555166833, the 2026-09-21 local split fixes, initial include_str compilation failure and local zero-findings receipt remain unchanged above. The earlier Resolution statement that GitHub Actions had not been rerun records its original date, rather than current remote uncertainty.

Actual read-only `gh run view 38206596321 --json status,conclusion,headSha,url` and `gh run view 38206596267 --json status,conclusion,headSha,url` each returned `status: completed`, `conclusion: success`, and exact `headSha: d47f6e09b4621808974cc083fcc27ee9d2c1b4c7`. Links: [CI](https://github.com/bright-builds-llc/open-bitcoin/actions/runs/38206596321), [Bright Builds Checks](https://github.com/bright-builds-llc/open-bitcoin/actions/runs/38206596267). This resolves the historical automated CI uncertainty without treating ordinary native verification as a substitute for the separate managed job.

Both actual local full-default JSON receipts were also rechecked: `6c7c6001-7fbd-42ec-875d-840c0c380707` (2,137,553ms wrapper) and `b427fa6a-122a-4c28-9dcb-e58342ec3eac` (2,064,537ms wrapper), each success/exit0/full on 2026-10-12. They record `a77edab276a9daa086055aa61e6143ce86bc1218` plus dirty Phase 162 worktree changes, not an exact d47 execution. [Phase 162 native evidence](../phases/162-real-prune-retention-and-integrated-parity-proof/162-NATIVE-EVIDENCE.md) and [the current handoff](../phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-HANDOFF.md) give paths and scope.

Status `automated_verified` records these observed automated results only. No human confirmation was performed, no historical failure was rewritten, and no fresh Phase 163 CI/full-native/closure/audit result is implied. The unrelated `main-ci-stale-loc-after-bright-builds-update.md` incident is not this reconciliation target.

## Logical-control source revalidation | 2026-10-13T17:42:27Z

Earlier current-focus statement retained as dated e0 history: next_action: Retain original ten failures, split fixes and dated local receipts below. Corrected21-input Phase163 has current local full6 acceptance (38c0607f,all176stages/exit0), clean independent review/security and functional verification05:36:20Z; current managed source checks have0findings. Actualrenewedaudit/readiness/finalSC1–SC5nowpassed; normal-hook saving remains pending. The prior successful remote d47f6e09 CI remains dated evidence; no new remote CI success is claimed for this uncommitted source.

Actual source repair replaces one active physical requirements assertion with four explicit logical availability/refusal cases and refreshes its frozen hash. Production selectors, claims and runtime proof remain unchanged. Original e0 reports are preserved in `.planning/reports/phase163-e0c-before-logical-control-revalidation/`; no failed or interrupted run becomes a PASS.

## Current audit boundary | 2026-10-13T19:09:31Z

The actual corrected-source audit passed22/22 at19:07:36Z after functional activation and independent integration31/31seams/18/18flows. Readiness publication followed at19:08:16Z; its actual suite is in progress. This update changes current focus only and does not retime earlier receipts or perform human confirmation, Git saving or archival.

## Final corrected-source acceptance | 2026-10-13T19:19:24Z

Fresh final SC1–SC5 verification passed on the unchanged source after actual native, matrix, activation, audit and readiness execution. Canonical verification SHA256 2d2531cf866871d78bb42f7570970d4715e71c84a49f44e115f8c21464f0472b. Root reconciled all22 unique Complete owners,5/5phaseplans,90/90milestoneplans and10/10phases. Original historical observations and the separate human-confirmation condition remain unchanged. Normal-hook source saving and actual archival are separate transactions.
