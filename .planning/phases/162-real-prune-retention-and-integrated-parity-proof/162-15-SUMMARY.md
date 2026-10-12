---
phase: 162-real-prune-retention-and-integrated-parity-proof
plan: "15"
subsystem: testing
tags: [rust, source-guards, retained-filters, cold-reopen, peer-capability, paired-prune]
requires:
  - phase: 162-14
    provides: Lazy selected trusted-header checkpoint and genuine cold stale authorization
provides:
  - Function-scoped persistence and ordered lazy-checkpoint source guards with mutation controls
  - Full real paired-prune and cold-reopen RPC and exact granted inbound proof
  - Pinned all-family absent-capability physical disconnect with fresh-session cleanup
affects: [162-04, 162-05, 162-09, 162-10, 162-12]
tech-stack:
  added: []
  patterns: [Comment-aware source contracts, whitespace-stable delegation checks, fresh denied sockets]
key-files:
  created: []
  modified:
    - scripts/check-phase158-validated-reorg.ts
    - scripts/check-phase158-validated-reorg.test.ts
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/recovery_and_persist.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/inbound.rs
key-decisions:
  - Preserve historical lineage and claim checks while following the actual private callback executor.
  - Absent local BASIC capability must disconnect without response, matching pinned Knots.
  - Preserve advertised All-only capability with absent backend as a separate silent/Pong case.
  - Keep every granted request, independent expectation and sentinel unchanged.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-11T20:36:08Z
duration: 39 min
completed: 2026-10-11
---

# Phase 162 Plan 15: Guard Compatibility and Full Cold Client Proof Summary

**The complete real prune/reopen lifecycle passes exact active, bodyless and stale RPC/peer responses, with historical persistence guards restored and absent-capability disconnect verified on fresh sockets.**

## Performance and Scope

- Tasks: 2/2. Exactly four owned source paths; no production, recipe, fixture, oracle or RPC implementation change.
- First recorded execution clock: 2026-10-11T19:57:27Z. Final full-corpus run ended 20:27:43.971Z; strict lint after that run ended 20:28:17.552Z. Root's final cleanup review led to one focused-only correction and fresh focused/lint/managed checks before this final summary. Duration includes the independently checked no-grant diagnosis/amendment pause.
- Material guidance: repo AGENTS, Bright Builds sidecar and overrides; architecture/code-shape/testing/verification/Rust/TypeScript standards; execute-plan workflow; phase context/research/UI contract; Plans 15/04/10; Plan14 review/summary and both tracked diagnoses.
- Both active lesson inputs were fully read: 7,188 bytes / 2,397 conservative estimated tokens, with no archive loading or new audit trigger. Root had fetched origin and prepared the pinned baseline before delegation.
- All Cargo commands used Bun 1.3.9 via the pinned PATH, Rust 1.94.1, the timing wrapper and serial build ownership. No overlapping Cargo/Bazel jobs.
- D-14 defers task and metadata commits to root. No staging, commit, branch, hook bypass, push, shared state, roadmap, requirements, parity registry or generated LOC edit occurred here.

## Task 1: Historical Guards and Mandatory Full Flush Gate

The old Rust `persist_calls_execute_flush_ifneeded` checked contiguous raw source `self.flush_lifecycle.execute_flush`. Plan14's correct longer private method was wrapped by rustfmt. Its broad selector honestly reported **70 passed / 1 failed**; that receipt remains:

`.local/open-bitcoin-dev/command-timings/phase162-header-checkpoint-flush-regression/2026-10-11T19-50-22.639Z-d9f72194-ca91-4524-b1b1-df47191a1013.json`

The existing Rust test now scopes actual method bodies and removes whitespace before checking the real chain: persist → flush_with_mode(IfNeeded) → flush_applying_plan → private callback executor → lifecycle executor. It retains the complete source prohibition against leftover `save_snapshot(self.chainstate.snapshot())`. Production formatting was untouched.

Phase158's historical lineage contract follows the actual private wrapper and callback owner. Preparation, wrote-coins completion, accepted position, coins fence, pending completion, confirm_validated_flush and reorg-durable checks remain. Separate checkpoint contracts guard typed fallibility, readiness/selection/no-work before the prefix, body/undo prefix before exactly one checkpoint, then unlink and coins; the selected producer captures only inside its callback and returns snapshot failure.

Existing comment/literal/test-only ownership masking and all historical claim/parity predicates remain intact. New controls cover rustfmt whitespace as a positive; removed delegates, changed/removed IfNeeded, actual leftover snapshots, comment/literal fake delegates, removed/reordered/eager checkpoints, eager capture, swallowed failures and changed callback error types as negatives. Every new checkpoint anchor also has an independent removal mutation.

Semantic RED for the new changed-IfNeeded mutation was **0 passed / 1 failed**, proving the missing guard. Final complete checker suite: **508 passed / 0 failed**, 965 assertions, 36.75 seconds; 53 passing tests have the checkpoint-guard prefix. The earlier narrow intermediate checkpoint set passed 14 tests before full per-anchor/type controls were added.

The mandatory whole flush selector is now **71 passed / 0 failed / 0 ignored**, including the existing corrected test. No failure waiver or zero-test selector was used.

### Caller, Lock and Simplification Review

Periodic, pressure/critical and Always managed settlement, including daemon shutdown/error settlement, use flush_coins → automatic_prune::flush → flush_and_evict_pruned_blocks. Authenticated manual pruning uses flush_applying_prune_plan → the same helper. The guarded chainstate owner delegates to the single selected lifecycle executor.

The existing order remains authority → automatic state when applicable → individual publication guard → history/payload. No handle call recursively acquires authority; no publication/history guard survives into the lazy callback. Snapshot lookup/capture/admission/write remain after selection/body-undo prefix and before unlink/coins. Executed no-op/failure/legacy/header-only/missing-proof controls remain effective. Old missing stale-header stores remain fail closed; no backfill was added.

Simplification retained the existing comment-aware source machinery and one function-scoped Rust guard rather than changing production layout. The denial amendment below reuses the existing physical-session handshake, scope checks, timeout, peer cleanup and listener join. No new source path, dependency, public API or policy abstraction was needed.

## Task 2: Honest RED, Pinned Amendment and Focused GREEN

Two earlier parent-reported full-corpus failures stopped at the stale canonical h1000 request before Plan14. They remain distinct from the short diagnostic and from this executor's runs.

This executor's first complete run after Plan14 passed real prune/reopen, RPC and every original granted request in all three sessions: **20 frames / 1,890 bytes per session**, 60 frames / 5,670 bytes total, exact expectations, sentinel fences, zero charges and joined listeners. It then failed at no-grant exchange0: ConnectionReset before an expected Pong. Result: **0 passed / 1 failed**, 241.57-second test / 254,697-ms wrapper, exit101.

The exact semantic RED receipt is:

`.local/open-bitcoin-dev/command-timings/phase162-stale-after-normal-header-checkpoint/2026-10-11T20-07-12.418Z-0be048b2-5288-4f79-8a7a-171ce081cbc1.json`

All 13 then-frozen source hashes matched after that failure, and its PID78486 corpus was cleaned. No completion summary or rerun was issued before independent diagnosis and approved four-file Plan15/Plan04 amendment.

Pinned Knots `net_processing.cpp:3162–3169` requires BASIC plus own NODE_COMPACT_FILTERS and explicitly disconnects otherwise. `p2p_blockfilters.py:195–218` uses a fresh peer and waits for disconnect separately for all three families. Existing first-party capability validation and InvalidDisconnect match that ordering before backend reads or resource reservation. The frozen no-grant Pong expectation was the defect; production policy was correct.

Only inbound.rs was unfrozen for this correction. An absent-grant session now requires zero bytes followed by EOF, ConnectionReset or ConnectionAborted within five seconds. Any data, Pong, other I/O error or timeout fails. The existing pre-request handshake fence remains; no post-denial sentinel is sent. Every original one of the ten denied requests gets its own fresh connection/listener, peer disappearance, zero charges, joined cleanup and settled identity check.

The focused `phase162_inbound_no_grant_all_families` genuinely accepts 102 normal-maturity100 recipe blocks, persists bodies, collects actual receipts, settles through ordinary Always, closes the runtime and uses the existing parsed BASIC-enabled/general-off/listener-on/in-only factory. It proves each of getcfilters/getcfheaders/getcfcheckpt without pressure or positive-response oracle data. It closes all handles, proves a raw reopen and removes its datadir.

Initial focused GREEN: 1 passed, 2.03 seconds. After scoped import/size simplification, final focused GREEN: **1 passed, 1.90 seconds**. Separate unchanged `phase160_peer_evidence_configured_all_only_no_backend_all_families`: **1 passed, 0.93 seconds**, retaining advertised capability plus absent-backend silent/Pong behavior and invalid-type disconnect.

Root's final source review identified that the new focused test initially cleaned its datadir only on success. A local RAII Cleanup guard now invokes the existing remove_dir_if_exists helper, is instantiated before runtime/fixture owners, and is explicitly dropped after successful all-handle raw reopen. On unwind, later runtime/fixture owners drop before the cleanup guard. Named actual parameters/store/handle/provenance keep the file at628lines; the explicit handle/runtime pair drops before configured reopen. This correction changes only the focused test, leaving every full-client function and fragment unchanged. Per root instruction, the full238-second corpus was not repeated. Post-correction focused denial passed **1 test / 1.81 seconds**; fresh node+RPC strict lint passed **1.95 seconds**; managed/rustfmt/diff checks pass and focused datadirs are absent.

## Successful Complete-Corpus Measurements

Final `phase162_integrated_lifecycle`: **1 passed / 0 failed**, 238.05-second test / 238,216-ms wrapper. No shortened corpus, private persistence, seeded snapshot authority, provenance invention, stale-policy relaxation or proof-before-reopen was used.

| Observation                                                         | Final actual result                                                    |
| ------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| Initial accepted blocks / final active positions / accepted records | 1,001 / 1,003 / 1,006                                                  |
| Maturity / startup height / initial scheduled turns                 | 100 / h7 / 125                                                         |
| Seed / catch-up / whole lifecycle                                   | 13,979 / 5,732 / 238,050 ms                                            |
| Extension / genuine shallow replacement                             | h1001 / h999..1002, ancestor998, three stale rows                      |
| Manual paired absence / keep-window refusal                         | h0..20 / passed                                                        |
| Manual logical usage before / after                                 | 1,066,313 / 1,061,726 bytes                                            |
| Nonactive codec-only pressure rows / encoded value bytes            | 584 / 578,091,088                                                      |
| Actual automatic target                                             | 550 MiB = 576,716,800 bytes                                            |
| Automatic logical usage before / after                              | 579,152,814 / 578,426,929 bytes                                        |
| Newly deleted automatic pairs / retained active window              | 694, h21..714 / h715..1002                                             |
| Allocated temporary file observation                                | 1,052,647,424 bytes                                                    |
| Pressure construction, streams, apply and configured reopen         | 198,312 ms                                                             |
| Remaining soft-target excess                                        | 1,710,129 bytes; target not attained                                   |
| Authenticated RPC                                                   | 9 hashes × 2 selectors; 4 unauthorized controls; exact index selection |
| Each general/explicit-blockfilters/All granted session              | Original 10 exchanges; 20 frames / 1,890 bytes                         |
| Total granted client-observed replies                               | 60 frames / 5,670 bytes                                                |
| Denied original requests                                            | 10 fresh zero-byte terminal disconnects                                |
| Listener/peer/transport cleanup                                     | Joined, peers removed, zero live charges                               |
| Success and expected failure worker settlement                      | Joined; typed failure retained; reopened; final datadir absent         |

All three granted sessions preserve exact independent active/bodyless/stale cfilter and cfheaders frames, predecessors/order and positive h1000 checkpoints for both canonical displaced and replacement ancestry. Body/undo absence, coins B, metadata, checkpoint/protection and all 1,006 immutable records remain checked through cold reopen and client reads.

These are physical client-observed achieved totals, not cumulative public transport receipts. Plan09 still owns actual private complete/partial-write accounting. Pressure rows are codec-only nonactive storage volume, never validated history. Allocated file bytes are a point observation, not peak disk/RSS or a physical cap guarantee.

Final successful receipt:

`.local/open-bitcoin-dev/command-timings/phase162-stale-after-normal-header-checkpoint/2026-10-11T20-23-45.755Z-e9f8b108-2431-4ae8-8564-ec92fe418da4.json`

Full failed/final output is retained locally at `/tmp/phase162-plan15-client.log` and `/tmp/phase162-plan15-client-final.log`. Timing receipts remain under the repo-defined ignored `.local/open-bitcoin-dev/command-timings/`.

## Verification Receipts

All Cargo rows use `bun run scripts/command-timings.ts run --key KEY -- cargo ...` with pinned PATH and `--manifest-path packages/Cargo.toml`. Tests use all-features and serial libtest threads.

| Timing key                                    | Selector / command                                     | Result                                     | Wrapper ms |
| --------------------------------------------- | ------------------------------------------------------ | ------------------------------------------ | ---------- |
| phase162-flush-lifecycle-compatibility        | node lib flush_lifecycle                               | 71 passed                                  | 41,288     |
| phase162-plan15-checkpoint-regression         | node lib phase162_header_checkpoint                    | 18 passed                                  | 11,065     |
| phase162-plan15-history-regression            | node lib phase159_validation_history                   | 30 passed                                  | 25,822     |
| phase162-plan15-reorg-regression              | node lib phase158                                      | 102 passed, 1 existing ignored measurement | 311,593    |
| phase162-plan15-range-regression              | network lib phase160_range_policy                      | 9 passed                                   | 654        |
| phase162-plan15-stale-regression              | network lib phase160_coverage_stale_stop               | 1 passed                                   | 136        |
| phase162-no-grant-focused                     | daemon phase162_inbound_no_grant                       | Initial 1 passed                           | 6,152      |
| phase162-no-grant-focused-final               | daemon phase162_inbound_no_grant                       | Final 1 passed                             | 4,879      |
| phase162-no-grant-existing-all-only           | daemon all-only/no-backend control                     | 1 passed                                   | 1,190      |
| phase162-stale-after-normal-header-checkpoint | daemon phase162_integrated_lifecycle                   | RED 0 passed / 1 failed                    | 254,697    |
| phase162-stale-after-normal-header-checkpoint | daemon phase162_integrated_lifecycle                   | Final 1 passed                             | 238,216    |
| phase162-plan15-final-clippy                  | node + RPC all-targets/all-features, -D warnings       | Passed; 6.22s Cargo runtime                | 6,478      |
| phase162-no-grant-cleanup-focused             | daemon phase162_inbound_no_grant after RAII correction | 1 passed; 1.81s                            | 5,116      |
| phase162-no-grant-cleanup-clippy              | node + RPC strict lint after RAII correction           | Passed; 1.95s                              | 2,182      |

The initially queued regression shell ran serially to normal completion. After root advised against redundant unchanged reruns, no further unaffected runtime suites were scheduled. The long Phase158 run showed 93% CPU at 3m41s and completed in 311.45 seconds. Both corpus runs were polled with process/heartbeat evidence; quiet yields were never treated as timeouts.

Final additional checks passed: source checkers158/159/160; parity breadcrumbs for 1,133 Rust files; managed all checks with zero findings; scoped rustfmt for Plan14's ten paths plus the existing source guard, then amended inbound/guard; diff whitespace. No standalone TS formatter/typecheck configuration is present; Bun execution/mutations and managed checks cover the two scripts. Root retains the full native verifier.

The inbound path is still untracked as part of prior phase work, so review included an explicit before/after source diff rather than relying on git diff alone. Eight original helper/exchange functions and three granted service/scope/reply-sentinel/success-log fragments were compared byte for byte and are unchanged.

## Source Freeze and Registration Handoff

All twelve other frozen client/fixture/lifecycle/RPC/recipe/oracle files matched their initial SHA256 observations after the successful corpus. The three guard hashes also remain unchanged from the initial successful guard freeze.

| Owned source                                                                            | Final SHA256                                                     |
| --------------------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| scripts/check-phase158-validated-reorg.ts                                               | 75a86eef986661491ab1db463e855d18322f89d7ad6fdd1d641c6739ea525dd7 |
| scripts/check-phase158-validated-reorg.test.ts                                          | 6fac9217925121ced4f682b45b74beeae786b6a3fe3737b1a50de0ee9870f969 |
| packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/recovery_and_persist.rs | bf7d12d4a795c90e2b1448d5446f5dc170e9945e3463a4c781c00dfc45a3337b |
| packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/inbound.rs  | 1d991eeabfe62e1722828a9cdeb79fbf9ebc122b7a9e0fdb3b7d6ac0b85e7c60 |

The full-corpus source snapshot was c3cffcbd585f34a7d18c527f0f0747165a55ce9b2b72f392c915bce57f9b155c. Its sole later change is the new focused test's RAII cleanup and equivalent setup simplification; the full scenario, response and denial helpers remain unchanged. Fresh focused/lint checks cover the final hash above.

No new Rust path was created. Existing inbound breadcrumbs remain pinned net_processing.cpp, index/blockfilterindex.cpp and test/functional/p2p_blockfilters.py. Parent's Plan14 registration already passes. Parent should retain this summary and the corrected denial selector with the existing integrated-client parity evidence; no requirement owner or production scope changes.

## Declared Mitigation Dispositions

| Threat     | Disposition and executed evidence                                                                                                                                                                                          |
| ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T-162-15-1 | Mitigated in scope: actual lineage/IfNeeded/coins-fence/checkpoint source edges, positive whitespace and semantic negative mutations; full flush71 green.                                                                  |
| T-162-15-2 | Mitigated in scope: twelve frozen hashes and granted fragments preserved; actual stale cold responses; no fixture injection, provenance invention, backfill or policy change.                                              |
| T-162-15-3 | Mitigated in scope: complete584-row real manual/automatic paired-prune/cold-reopen run1/1; focused denial explicitly separate.                                                                                             |
| T-162-15-4 | Mitigated in scope: one pressure corpus at a time; finite socket/frame/child/task bounds; zero charges, peer removal, joined workers/listeners and absent owned datadirs.                                                  |
| T-162-15-6 | Mitigated in scope: pinned absent-capability disconnect, all3focused families and all10full original requests on fresh connections; separate All-only/no-backend control passes.                                           |
| T-162-15-5 | Handoff obligation: Plan10 may transition current claim predicates only; it must preserve CHECKPOINT_CONTRACTS, actual delegated lineage/IfNeeded/no-snapshot/order/error guards and every new positive/negative mutation. |

Independent source/security review and unresolved-High disposition remain root gates. No ASVS certification is claimed.

## Known Stubs and Threat Surface

The focused-denial handoff intentionally carries `oracle: Value::Null` at inbound.rs:591 because rejection occurs before backend/response acquisition. Its empty expected vectors are denial expectations, not missing successful data. Its genuine ordered raw-input prefix digest is computed from 102 blocks and is distinct from the long integrated corpus digest; no positive independent-response evidence is attributed to this focused field. No blocking stub remains.

No production endpoint, auth path, schema, dependency or trust authority was introduced. The new tests reuse existing temporary-datadir and loopback caller patterns. Support/operator output, redaction and public defaults are untouched.

## Deviations, Commits and Exact Next Handoff

1. The independently approved amendment expanded ownership from three to four files solely for the discovered no-grant expectation bug. The first full run remains semantic RED. Correcting production to keep the denied socket alive would contradict pinned policy and was not done.
1. Initial formatted inbound size reached639, then632lines. Moving test imports to the existing module imports and naming the raw-input/handoff/exchange values simplified it to exactly628lines. No suppression, new child or removed assertion was used.
1. No authentication gate occurred. All tasks and summary commits are explicitly deferred by D-14; root owns native/formal gates, requirements/state/registries/generated LOC and final Git.
1. Root's cleanup review added focused-test-only unwind cleanup, verified by fresh focused/lint/managed checks. No full-client change or redundant corpus rerun followed.

**Plan04 handoff:** Both Plan14's selected producer and this mandatory Plan15 are verified in their scoped acceptance. Root can now accept Plan04 against this actual final same-datadir proof: authenticated exact9hashes×2selectors/4auth, original general/explicit/All10exchange sessions, retained active/bodyless/stale ancestry and positive checkpoints, corrected pinned no-grant all-family disconnect, settled identities and bounded cleanup. A redundant pressure rerun is unnecessary unless missing acceptance or changed source justifies it. Plan04 still needs its parent-owned summary/readiness decision; this artifact does not silently complete it.

**Plan10 handoff:** Preserve all new guard arrays, wrapper/callback lineage checks, function-scoped persistence obligations and checkpoint mutations while adjusting later current-document claim predicates.

**Plan09/parent handoff:** Private actual receipt/partial-write controls, later wallet/body/outbound/fault work, independent reviews, full default bash scripts/verify.sh, formal lifecycle validation and requirement activation remain outstanding phase gates. The inherited subsidy interval210000 versus Knots Regtest150 remains explicit; no whole-chain Knots consensus differential, most-work selection, archive scale, physical cap, public networking, production/funds readiness, milestone audit or shipment is claimed.

Cargo is idle. Failed and final corpus datadirs and focused denial datadirs are absent. Root shared artifacts and Git remain untouched by this executor.

## Self-Check: PASSED

All four owned source files and this summary exist. Each final source SHA256 matches the table; the twelve other frozen files matched the pre-run inventory. Original helper/exchange and granted critical fragments are unchanged. The successful full-corpus receipt records exit0 and1/1 discovery; the prior semantic RED records exit101 and0/1. The final RAII-only correction has fresh focused1/1 and strict lint receipts, with no full-client fragment change. Existing breadcrumb registration and final managed/rustfmt/diff checks pass; inbound remains628lines. The baseline commit a77edab276a9daa086055aa61e6143ce86bc1218 exists. No task commits are claimed because D-14 defers them to root. Frontmatter identity, empty requirements-completed and exactly two standalone top delimiters were checked. Markdown uses the installed GFM/frontmatter-aware formatter. Owned datadirs are absent and all serial Cargo sessions exited; no shared state or Git mutation occurred.
