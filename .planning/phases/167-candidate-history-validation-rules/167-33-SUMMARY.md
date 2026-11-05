---
phase: 167-candidate-history-validation-rules
plan: "33"
subsystem: storage
tags: [rust, serialized-scripts, fjall, basic-filters, history-proof, mempool-snapshot]
requires:
  - phase: 167-12
    provides: Safe copied-child and immediate-original full history proof and configured reopen
  - phase: 167-32
    provides: Strict serialized block and transaction decoders with valid Amount and exact script bytes
provides:
  - Exact wide-script body load, history-record decode and budgeted BASIC turn inputs
  - Genuine wide-output accepted checkpoint, copied/original proof and all-handle reopen evidence
  - Exact serialized mempool snapshot records with unchanged corruption, resource and recovery boundaries
affects: [167-30, 167-13, 167-17, 167-18, 167-19, 167-20, 167-25]
tech-stack:
  added: []
  patterns: [Explicit serialized-data decoding, output-only BASIC ledger preservation, data representation separate from admission]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/serialized_body_tests.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/serialized_tests.rs
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - packages/open-bitcoin-node/src/storage/canonical_history/records.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs
key-decisions:
  - Change only the three complete-body decoding seams and the bounded snapshot transaction seam.
  - Remove only the BASIC scanner script-semantic gate while preserving every native and work bound.
  - Keep canonical Coin, undo and genesis decoding bounded and actual recovery admission unchanged.
requirements-completed: [CVLD-01, CVLD-03, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-22T12:20:20Z"
commits: false
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
duration: "74m26.267s from root release through first successful document QA; final recheck recorded separately"
completed: 2026-10-22
---

# Phase 167 Plan 33: Serialized durable bodies and mempool snapshots Summary

**Strict serialized decoding preserves valid wide scripts through durable bodies, budgeted BASIC construction, complete history reproof and bounded mempool snapshots.**

The existing Git baseline is `1b472fb225a290df5cdb06eeea2dc67c624c648e`, not a task commit. Both tasks are complete within the six-owner release. Requirements here describe this bounded contribution; root retains global requirement activation, metadata and phase acceptance.

## Files Created/Modified

These are the six exact final sources, totaling 87,065 bytes. All are normally formatted; the maximum is 564 physical lines.

| Source                                                                              | Role                                                                                        | Lines | SHA256                                                             |
| ----------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- | ----: | ------------------------------------------------------------------ |
| `packages/open-bitcoin-node/src/storage/fjall_store.rs`                             | Strict complete-body load and conventional test registration                                |   564 | `eb28254315f8814eea8feccc5c703fad441b520a2fe42875c2ffa659f47fde0f` |
| `packages/open-bitcoin-node/src/storage/canonical_history/records.rs`               | Strict copied/original body decode, retaining key/hash/re-encoding and source digest checks |   320 | `2fbae9ce008398142c53c3318de0907d244d73e3f3b792223e80e9239328a45a` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs.rs`         | Budgeted serialized-body decode; removal of only the premature script-semantic gate         |   285 | `b6f648fdb5db243353caef8f89db2bed27a973da4716aebcdd7c557f25557a84` |
| `packages/open-bitcoin-node/src/storage/fjall_store/serialized_body_tests.rs`       | Real scanner, canonical acceptance, BASIC, complete proof/reopen and corruption controls    |   423 | `c7230bcdd0608ca7dc13c370c28022f2e86d14996ed5c5ebb2857dbed1dbcc4a` |
| `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs`                  | Explicit strict serialized transaction decode and test registration                         |   475 | `1a158b9a0b72df51bcce3adfa13aea7c682ed6bafed5d6d8ad789193d1a4986f` |
| `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/serialized_tests.rs` | Current/legacy wide-script identities, witness, bounds, corruption and policy distinctions  |   304 | `17f7d92b9f316461d031934a9815a0ad78190934d21fddefd82d920e44a1c6b2` |

The immutable six-source freeze is `.local/open-bitcoin-dev/phase167/plan33/final-source.json`, SHA256 `39940309660115c6b9d556767747fa56d8e0f6c9fd24c3b3aecb672035264394`. It contains all final copies and twelve forward/inverse patches. Both executor and root applied BSD patch with `--batch -E -F0`: all four parents restore byte-for-byte and both new children become actually absent. Root's independent patch/pin audit is `.local/open-bitcoin-dev/phase167/plan33/root-source-patch-audit.json`, SHA256 `da0d155eeeaea37411c18797a2d1c0107382ad135786bc85acfad7e5237543a8`; six sources, eleven pinned breadcrumb targets and twelve patches pass.

Accepted dependency APIs remain `open_bitcoin_core::codec::block::wire::parse_serialized_block` and `open_bitcoin_core::codec::transaction::wire::parse_serialized_transaction`. Their unchanged source hashes are `9ed634a725554b851ad2efbec9103560aa8b5cad7e546435787c5b8ff6813d59` for codec block/wire.rs and `5c8adb1887e1f9a8b7ea3f910c9978cbc0c2b52946920b6d01a26960b0571a3b` for transaction/wire.rs. These APIs confer data representation, not validation authority.

Independent final source review is CLEAN: `.local/open-bitcoin-dev/phase167/plan33/source-review/final-closure/REVIEW.md`, SHA25614f1b4f17aeb2070bded186bd37aeafcb42066ec479ebb377fdea3dd74310eba, with hashes.json SHA379a6dd4d16baf0a122873f7162f4655407c0d1890a6205e680ce2a01c18a273. Root's independent final evidence audit `.local/open-bitcoin-dev/phase167/plan33/root-final-evidence-audit.json`, SHAe91f142154a4ff6ddac673927fbcb116fbcef7f864400f1e1cd8cb7ded1791ad, passes all seventeen recorded command receipts, nine final Cargo/static checks, seven actual final Cargo jobs, six exact planned argv, fifteen real owned/guard fences, genuine RED outcomes and same-run proof/abort observations. Root read both review reports; no source finding remains. Root acceptance and whole-phase qualification are separate.

## Behavior and Evidence

Task 1 replaces exactly three bounded whole-body decoder seams with the accepted strict serialized decoder. The scanner still probes the native 4,000,000-byte limit and remaining-body/first-candidate limits before admission, then reserves conservative wire work before scanning or allocation. Canonical CompactSize/MAX_SIZE, count envelopes, actual-frame takes, witness flags, strict trailing checks and every work charge remain. Only Cursor::script's len>10000 semantic refusal was removed. Input scriptSig traversal retains its conservative reservation and does not enter the output ledger.

The independently serialized and mined Regtest height-one body contains output scripts of 10001, 10000, zero and two bytes; the last begins OP_RETURN. Its complete wire length is 20,177 bytes. Exact raw block hash is bf1be087d4828ed89feedb87e7e8198a134f95acc8b8d9aac2652b9cbdd78958, and coinbase txid/merkle leaf is 9e1fd39d9630e73d62bfb28e6d6ab4696183021c7893bb23c54ba23bed8f00b6. Bun's independent literal serialization/mining and the existing pinned Python SipHash/source-derived oracle produce BASIC bytes 02058f7d160040 with two elements: both nonempty non-OP_RETURN wide/bounded scripts. Empty and OP_RETURN outputs remain excluded from BASIC.

The ACTUAL maybe_basic_filter_turn_body path returns exact observed work: body_bytes=20177, cloned_bytes=20177\*32+size_of(Block), script_items=4, script_bytes=20003, record_operations=1, with all other counters zero. The test independently checks the conservative pre-scan reservation as well. Full output bytes/items include empty and OP_RETURN outputs before filter exclusions. Every reservation dimension, native-wire overflow, remaining-body yield, first-candidate refusal, pre-scan admission, truncated frames, canonical/envelope/MAX_SIZE/overflow counts, invalid Amount, witness flags and suffix refusals remains covered.

The genuine selected network owner admits and connects this body, persists trusted headers, performs the existing Always flush, initializes the actual BASIC owner and drives the real budgeted filter turn. The 10001-byte output is absent Coins; the 10000-byte and empty outputs remain Coins; OP_RETURN is absent. The coinbase-only undo is empty. Exact body, txid/merkle, scripts and empty witness survive load. Raw oversized Coin and undo records remain corruption, with positive 10000-byte Coin/undo controls using the same DTO shape.

All live network/store handles are dropped before the accepted Plan 12 public safe configured reopen. The SAME supplementary final run records:

- HISTORY_INSPECTION: 30 logical units, 45,802 logical bytes, authority maximum 20,177; actual copied child stage Some(2), child 2.634328625s, total inspection 10.377757167s, source_gates=true.
- WIDE_BODY_PROOF: body maximum 20,177, BASIC maximum 177, undo maximum 1,210; physical entries 60, physical bytes 67,128,198, copy bytes 67,128,198; measurement 10.316298166s; manifest_equal=true; complete_original_reproof=true.
- Actual runtime core snapshot, earned headers, complete body, BASIC record/state and validation record compare exactly after reopen. Child time comes from the supervisor observation; the 29.86/30.10-second test harness totals are not substituted for it.

Oracle input/output are retained under execution/oracle-input.json and execution/oracle-output.json, SHA256 b0e0df227beb06c086163d2314fecb5c36e93dae7ceaf7fb05472da2808fa1ee and d59d70e25e59f017ae1e5e527ac8bbf34f2edb490247c31845df6faa9c0b96fe. This is pinned-source protocol evidence, not a running Knots differential.

Task 2 changes only decode_canonical_transaction's parser/import and registers the declared child. Current and legacy snapshots preserve exact 10001-byte OP_RETURN and non-OP_RETURN scripts, canonical bytes, txid/wtxid and actual witness stacks. Format/generation, acceptance time, duplicate/foreign members, identities, encoded/member/per-transaction/aggregate byte limits, existing input-edge representability and real recovery admission remain unchanged. Malformed JSON, strict framing, Amount, witness flags, noncanonical/MAX_SIZE lengths and suffix controls refuse.

The configured 10001-byte OP_RETURN test exercises the existing 10000-byte push-only suffix boundary: default standardness refuses, the existing explicit datacarrier configuration permits that standardness form, and a 10001-byte suffix refuses even when the total-byte policy setting is larger. Non-OP_RETURN wide snapshot data remains representable while existing standardness rejects that fixture. No live mempool acceptance, default policy change or missing-prevout/signature qualification is claimed.

## Commands and Results

All seven final Cargo commands run against the same six-source freeze, with stable before/after fences and raw log SHA256 checks. Six planned commands remain verbatim. The seventh is the root-approved affected-storage run, skipping only the two selections already tested by their exact final commands and using nocapture for the SAME wide-proof observation. Those are filtered selections tested elsewhere, not ignored tests. No redundant proof Cargo command or direct-binary proof ran.

| Final label / key                                           | Harness passes | Exit | Wrapper seconds | Raw log SHA256                                                     |
| ----------------------------------------------------------- | -------------: | ---: | --------------: | ------------------------------------------------------------------ |
| final-bodies / phase167-33-bodies                           |              6 |    0 |          30.391 | `52f9108122734dbaa4647e4e01b77768fdfabaa17bee1474b187cabdad021e35` |
| final-history / phase167-33-history                         |             12 |    0 |         460.437 | `0e3062de5a7dd3a563c18232d8b40a234c21783096dae50386d8b7ccc163d6e9` |
| final-worker / phase167-33-worker                           |              7 |    0 |         385.529 | `0441cf83b69ddaefad0c3ca826ac32855b814e6bd353440ce858bc88a097f779` |
| final-mempool / phase167-33-mempool                         |              6 |    0 |           0.551 | `6836649dde5f9df4e73dac436350f47cb7d117c59ba9e89dea8792e378101361` |
| final-mempool-regressions / phase167-33-mempool-regressions |             47 |    0 |          12.107 | `0ab0d503e0090dc6b3604b2192819338e3f3f7bc5bfa8e2f1dc63a6d29b0e6b2` |
| final-clippy / phase167-33-clippy                           |            n/a |    0 |          26.453 | `248aa7d41e94793fd137f80cf6b8d0a675615a0220a019026ba36ac4110e11d2` |
| final-storage-regressions / phase167-33-storage-regressions |            640 |    0 |        2598.261 | `08856e10bbc94b64243538cc21b7f7fe053db4efc45b07e5696944329f3610ae` |

Every final test harness has zero failed and zero ignored cases. Auxiliary integration harnesses selecting zero tests do not count as discovery. Selections overlap; these counts are not unique coverage. Scoped rustfmt and diff checks also exit0, with empty raw-log SHA e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855.

```bash
bun run scripts/command-timings.ts run --key phase167-33-bodies -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase167_serialized_body_durable -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-33-history -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase167_history -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-33-worker -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase167_preeffect_history_worker -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-33-mempool -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase167_serialized_mempool_snapshot -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-33-mempool-regressions -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node mempool_snapshot -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-33-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings
bun run scripts/command-timings.ts run --key phase167-33-storage-regressions -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features storage:: -- --test-threads=1 --skip phase167_history --skip phase167_preeffect_history_worker --nocapture
```

Final closure `.local/open-bitcoin-dev/phase167/plan33/final-closure.json`, SHA256 27f5c6b0bedda9df1a0bd402e86ee1879809a5ec5ba9ce41284091dc6234b121, passes/errors:[]. It authenticates all seven final argv/results, complete raw-log hashes, source fences and live files. execution/proof-observation-reference.json, SHA256 0e0ad51dbd7a426a1c2a8827bfaf6da942df57ce70a25dbf3677f37200506371, explicitly records actualCommandSpawned=false and references the passing storage run/result, rather than manufacturing an eighth Cargo result. The storage result SHA is 3c9dab19604d03e2cf17fcc2bea4597fec7f6bc571309daadb4ada04addc476e.

The closure's complete chronological history has twelve actual Cargo jobs, including interim/failed attempts, plus static and liveness commands. Detailed per-command directories contain UTC time, full argv/environment basis, raw output, source copies before/after and hashes.

### RED and preserved failed attempts

| UTC start    | Attempt                         | Exit | Actual result                                   | Seconds |
| ------------ | ------------------------------- | ---: | ----------------------------------------------- | ------: |
| 11:10:11.340 | task1-red                       |  101 | 6 discovered; 3 pass/3 fail                     |  56.657 |
| 11:11:20.324 | task1-green                     |    0 | 6 pass                                          |  68.101 |
| 11:13:35.937 | task2-red                       |  101 | compiler setup failure; no behavioral discovery |  11.038 |
| 11:15:10.179 | task2-red-constructor-corrected |  101 | 6 discovered; 2 pass/4 fail                     |  37.918 |
| 11:16:36.603 | task2-green                     |    0 | 6 pass                                          |  12.435 |

Task 1 genuine RED fails the ACTUAL scanner and budgeted BASIC turn at BASIC body script bound and load_block at script too large:10001. Task 2's first attempt used a private ScriptWitness field; correction to its existing public constructor precedes genuine RED. Corrected RED fails current/legacy raw-script decode, exact decode-limit success and data/standardness distinction. Only then does the snapshot parser change. The initial independent oracle attempt also returned ENOENT before its ignored execution directory existed; that setup failure is preserved in execution/fixture-initial-setup-failure.json and is not counted as behavioral RED. The final body matrix strengthens canonical MAX_SIZE+1 and 10000-byte Coin/undo positives; all original selections were repeated after the final freeze.

### Long-run and diagnostic observations

The supplementary run crossed command-timings' advisory threshold. Its retained diagnostics are under `.local/open-bitcoin-dev/stall-diagnostics/2026-10-22T11-47-52.096Z-phase167-33-storage-regressions`. The original run was not interrupted or restarted. Existing raw-backend characterization intentionally produces real SIGABRT6 for malformed integer/overflow names and retains exact owned scratch until the parent observes completion. Both variants naturally completed with signal=Some(6), scratch_retained=true and source_preserved=true; the harness then passed. Their panic/abort output is expected negative-test evidence, not a Plan33 production panic or failed test.

During both waits, lsof observed the child reading the existing 15,264,448-byte Cargo debug-artifact directory; process state/liveness and both raw lsof outputs are retained. A one-second diagnostic sample of the first child completed sampling but waited in symbol processing without a final report. After that child naturally exited and its real stderr stack existed, root authorized termination ONLY of the verified obsolete diagnostic sample PID24300 targeting exited PID24074. The diagnostic wrapper exits143; this is distinct from tests and Cargo. No test/Cargo signal, manual SIGABRT, target copy/cache change, environment change, deadline reset, editor configuration change or source change occurred. execution/obsolete-sample-disposition.json preserves the verified command and disposition. No finished sample stack report is claimed.

## Scope, Protected Boundaries and Guidance

Root release is 6ade2074be87c4258ffe7342c76d21798d70f5918a3a839aa63476cd5aa3ad78; before-source manifest de1115d47dfe24c3ce206fcff4bb84cfd6c779dceea380807043b69796364d1a. Accepted32 root receipt is 8ebd2b3aaee92dfa438f58513e863911b4619b8baa9fbcd58db37f624a454e2a. Effective Plan33 SHA1a07b25d145cc2915a1dbc363dc2f2c90b9a7755935d671d611f4b91f4e88046 and wire amendment eaa92e2ac25e1a20df8e8a529a909c5c7dfaa7b92059a02add4ed01c926e5bfb remain unchanged. Knots pin is a9aee730466ac67d35a3c03ee24676be5e045878.

The bounded Coin codec remains SHA6cea4ce8678c207f239111e70eac42251c1817432d1cec62d4cbbe9914e9c496 and canonical-genesis records SHA0728fd2eea1ca348a1469217bd6706834b007355df94063d68d785f74278e7c7. The actual bounded snapshot DTO parent is snapshot_codec.rs, 624 lines, SHAabd49912992ee8e183633a1bdbb787191eed5f106ec7124fc529026fcb81a3bf. Root's root-real-dto-guard.json and root-real-dto-command-coverage.json authenticate its complete bytes against accepted08. Earlier receipts' nonexistent output.rs/block_undo.rs guards attest absence only and are not retroactively claimed to fence the real parent. The actual parent was added before all final commands; final before/after fences retain it. Historical receipts remain unchanged.

AGENTS.md, AGENTS.bright-builds.md, overrides and the architecture/code-shape/testing/verification/Rust standards informed the narrow data seams, AAA tests, maybe names, source caps and serial verification. Active lessons were fully measured/read: 6,898+1,958=8,856 bytes and 2,953 conservative estimated tokens; no lesson archive was loaded. GSD init and execute-plan workflow were loaded. Root's explicit commits:false and shared-metadata ownership govern saving/state updates.

All Cargo jobs used Bun1.3.9, pinned Rust1.94.1, the existing target/cooperative lock and physical TMPDIR. Natural Cursor analyzer artifact-lock contention was observed and preserved. No agent Cargo/Bazel overlap, new dependency/crate, worktree, budget/cap increase or source-guard renewal occurred. New files contain pinned breadcrumbs; Plan19 owns central manifest reconciliation. No README/operator interface changed. The simplification pass retained four parser seam changes and a three-line semantic-gate deletion; substantive coverage stays in the two declared children. The final source audit passes with no stubs or undeclared threat surface; no new endpoint, authentication path or schema is introduced.

## Performance

- Tasks:2; sources:6, including two new children and four modified parents.
- Root source/build release:2026-10-22T11:05:54.200628Z; final source/evidence closure:2026-10-22T12:16:45.652Z; measured wall interval70m51.451s, including natural lock and host waits. Read-only preparation before release is excluded.
- All build jobs naturally finished at12:16:10.070Z and the agent explicitly released Cargo/Bazel ownership. First successful document QA ended at12:20:20.468Z, giving74m26.267s from root release including natural waits; document-qa-final.json records the measured-interval correction check and document-qa-handoff.json records the final review-reference check, both using the same relative six-file verification.
- The first passed source audit is preserved as execution/source-audit-first.json with its observed SHA40a47482a8911401f8423be9ec629b12462cb0d17e5fa6479e25f298d80d6537; the final repeated passed audit is source-audit.json SHA c80deda6603584acd64f387d2bbe93593e1e125bae26825d7e8e0bba8ec324cd. No failed result was relabeled.

The first Markdown check used the installed explicit gfm/frontmatter extensions and exited1 for formatting. Its complete source remains execution/summary-before-format.md, SHA cf1d3b034e9c315d08656b54840037915ce04a36f9687774f7ca8c9e15eab335; document-qa.json SHA908e98ae754431e3686e3680767f17d82a956b945b45b2a9b56cc1c68a453fbe records that failure, scoped formatter exit0, repeat check0 and relative six-file verification0. The initial formatted summary SHA5f6e6c39d192b341625d01a3f665d98d3b94214d19b6b4a4d7893364e4354f8d remains attributable. This final document adds the measured QA interval and this failure receipt, then repeats static document checks; no source or Cargo result changed.

## Deviations from Plan

None in production ownership, behavior, APIs, budgets or architecture. Routine test-constructor/setup corrections and root-approved supplementary selection reuse are documented above. All six original commands remain exact. No authentication gate occurred.

## Residual Limits and Next Readiness

These are storage, actual canonical Rust acceptance/reopen and pinned-source protocol receipts. Actual provider command/Always/BASIC/reopen/next evidence remains Plan30; authenticated daemon/process wide-output evidence remains Plan13. No running Knots differential, general wide-script P2P ingress, full-sync/funds-readiness or whole-phase qualification is claimed. Global qualification refusals and deferred168–171 work remain unchanged. Final17/18/19/20/25 native/fresh-coverage/Bazel/source/security/goal/lifecycle/compact and normal-hook Git gates remain mandatory.

## Task Commits

None; commits:false is explicit. Root owns STATE, ROADMAP, requirements, metrics, todo, manifest/guards and later normal-hook saving. This executor performed no staging, commit or push.

## Self-Check: PASSED

All six sources exist and match the final freeze; all twelve exact patches round-trip with actual new-child absence; every required final command and the affected storage run exits0 with nonzero test discovery and zero ignored cases where applicable. Raw logs/source fences agree with the final closure, scoped source formatting/diff checks pass and the existing baseline commit exists. Both tasks and all six owners are accounted for; root acceptance and later phase gates remain separate.
