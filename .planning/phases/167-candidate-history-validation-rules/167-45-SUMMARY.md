---
phase: 167-candidate-history-validation-rules
plan: "45"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: node-sync
tags: [rust, peer-lifetime, basic-filter, ownership]
requires:
  - phase: 167-34
    provides: Existing typed node authority and bounded storage foundations
  - phase: 167-46
    provides: Actual staged lifetime API and separately issued dependency-ready receipt
provides:
  - Real fresh-policy and lifetime-owned Session request, cleanup and outbox integration
  - Actual compact-completion Block payload handoff through durable recording and BASIC handling
affects: [167-46, 167-35, 167-17, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Original-lifetime ownership, typed continuation, exact whole-source evidence]
key-files:
  created:
    - packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs
    - packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests/tests.rs
    - packages/open-bitcoin-node/src/sync/session/outbox.rs
    - packages/open-bitcoin-node/src/sync/tests/peer_resolution/peer_protocol_failures/lifecycle.rs
  modified:
    - packages/open-bitcoin-node/src/network/action_translation.rs
    - packages/open-bitcoin-node/src/sync/block_reconcile.rs
    - packages/open-bitcoin-node/src/sync/block_response.rs
    - packages/open-bitcoin-node/src/sync/session.rs
    - packages/open-bitcoin-node/src/sync/session/emission_terminal.rs
    - packages/open-bitcoin-node/src/sync/tests/peer_resolution/peer_protocol_failures.rs
    - packages/open-bitcoin-node/src/network/types.rs
    - packages/open-bitcoin-node/src/sync/runtime_state.rs
key-decisions:
  - Preserve the original lifetime and typed production path before physical effects.
  - Keep current recovery separate from original whole-file inverse authority.
  - Require fresh combined strict evidence and dependency-ordered Summary QA.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-25T00:28:20.938Z"
completed: "2026-10-25"
commits: false
status: source-core-accepted-independent-summary-qa-pending
---

# Phase 167 Plan 45: Owned Session Integration and Durable Compact Completion Summary

**Actual Session effects, requested-body storage and cleanup remain bound to the original lifetime; genuine compact completion reaches durable recording, and the two handshake fixtures use explicit remote nonce1.**

ROOT accepted the exact joint source/runtime CORE for the Plan45 Summary stage and separately released this canonical Summary/scoped document QA under `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root45-r8-2-core-acceptance-summary-release-v1/release.json` (5374 bytes; SHA256 `6665f9365e4f1a21f80107411e521f303bd398359aad78e0ec95f6bd0f7aa776`). Its sourceCoreAccepted=true and sourceAccepted=false are distinct: independent Summary45QA and final counted ROOT source acceptance remain PENDING. The actual canonical generated_at records this publication. The five requirements IDs copy the bounded plan contribution; all five global CVLD requirements remain OPEN. No nested commits were made under commits:false.

## Exact source, task and provenance scope

The canonical plan has four tasks, including the checked R8 cross-layer Task1b; all34 command rows preserve the original33 literal argv plus the exact added Task1b selector. Canonical tasks:

- Task 1: Earn original RED and implement real fresh-policy/test contracts
- Task 1b: Carry the actual completed body to durable Session recording and retire obsolete lookup
- Task 2: Integrate actual authority, fresh policy and owned Session accounting
- Task 3: Close one final joint source frame and release Summaries only in dependency order

Twelve structural owners (eight original whole bodies/four original TRUE absences) are distinct from the latest two current-present nonce corrections. Current whole bodies are:

| Whole source owner                                                                              | Bytes | Normal lines | SHA256                                                             |
| ----------------------------------------------------------------------------------------------- | ----: | -----------: | ------------------------------------------------------------------ |
| `packages/open-bitcoin-node/src/network/action_translation.rs`                                  | 19763 |          484 | `317c964dcd691deaabb76b292f364502d4968e652974c6439c8b954878a4ddcf` |
| `packages/open-bitcoin-node/src/sync/block_reconcile.rs`                                        | 17810 |          502 | `f512500cc02000ee5909432660310b7c02c7376a4553db562b8ff689e3c96393` |
| `packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs`                          | 19036 |          544 | `2d019648284f5c362f9153fa420da24d1edf495f186fa24ead4cc1d69eb08db0` |
| `packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests/tests.rs`                    | 13592 |          409 | `52782ff59682e22e414ff0f05f33926ec0798ffa253d8c8669c2212d9d273765` |
| `packages/open-bitcoin-node/src/sync/block_response.rs`                                         | 15435 |          415 | `ad33d32c3837a85bd67f2b1dadd728808642fe40f859b7bed9ad6844615e1b49` |
| `packages/open-bitcoin-node/src/sync/session.rs`                                                | 21757 |          570 | `3c6e5e26f015c69191ac86baa95dbe9ff6ed1aaff1fb7ec26570ba2374fb0c9b` |
| `packages/open-bitcoin-node/src/sync/session/emission_terminal.rs`                              | 22809 |          618 | `348c445270ada9f40c1746fc16ba154ec9aef569aa18d331a8b611548295f624` |
| `packages/open-bitcoin-node/src/sync/session/outbox.rs`                                         | 23043 |          622 | `cd768c0e7d041fbb81b1f15a9c9dee2f92a1fafdfa898b99d1c09d786dc153be` |
| `packages/open-bitcoin-node/src/sync/tests/peer_resolution/peer_protocol_failures.rs`           | 20672 |          613 | `ff32b515719a5226efa041f8189443a5daf90fb38db6a4bf8ddb88102a398f43` |
| `packages/open-bitcoin-node/src/sync/tests/peer_resolution/peer_protocol_failures/lifecycle.rs` | 22113 |          596 | `b9842b902788759b7e5bf2cc7f34a968b36f23d2034ac5e0ed05de27bcb82c52` |
| `packages/open-bitcoin-node/src/network/types.rs`                                               |  8637 |          269 | `8d3ad97374ea2d575255ca1fd94e8461a9f5d01e8d19ce01c0611e89857a732f` |
| `packages/open-bitcoin-node/src/sync/runtime_state.rs`                                          | 25998 |          597 | `42e375f926bb29641cad04b36b369509163c3c362c3b388551fbcff3eaaa8cef` |

Original exclusive45before `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-before45-v1/capture.json` (318616 bytes; SHA256 `73dc6f47c3e2c99fbf80c2782027ad14b79c12c9b122e0049833775ed800995b`) is282states with5existing/4absent direct owners. Later types/runtime before observations and the original read-only action-translation body form the explicit mixed `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root45-r8-original-composite-v1/composite.json` (241705 bytes; SHA256 `c664a25787082205e93ac1411eb28192224c941f2b303f5c24d11da36f3dfde7`):284states,12direct owners8whole/4TRUEabsent,272protected. This per-path assembly is not an atomic historical snapshot. The current-before284 frame `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-r8-2-current-recovery-v1/capture.json` (339724 bytes; SHA256 `5b0af184f1b8c0e4af0530c01f175f50a1db0d1a551488565db5d3716b0fa021`) is later all-present held834c after the reversible counterfactual, not original73dc/c664 and not old9600/e08c. Current joint final is d3c5; no current-present outbox or other child replaces an original absence.

## Original behavioral REDs before source

Under genuine original4730 lease `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-actual45-release-v1/release.json` (73502 bytes; SHA256 `47302c1719e87f474d518b43a17c31a04276855911d931baa400125c35841d61`), both unchanged full named behavior cases compiled and actually failed101, each0PASS/1FAILED/0ignored. Root classified assertion mismatches as genuine behavior, not compiler/setup/zero-discovery substitutes. They preserve the original Compatibility contract against actual Network classification and precede original9 application/formatter. Their genuine root history is `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-r8-original-history-v1/history.json` (4591 bytes; SHA256 `381ea2431ec87d6626182819009995ddc9f3d0cfa572f92c183f89012ca46edb`).

| Original behavioral RED              | Exact named FAILED                                                                                                            | Test s | Recorded job s | Native exit |
| ------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------- | -----: | -------------: | ----------: |
| `phase167-45-original-duplicate-red` | `sync::tests::peer_resolution::peer_protocol_failures::duplicate_version_peer_is_failed_and_replaced_without_progress_credit` |   0.82 |          1.547 |         101 |
| `phase167-45-original-backoff-red`   | `sync::tests::phase70_peer::backoff_rotation::phase70_incompatible_peer_rotates_with_typed_backoff`                           |   0.73 |          1.306 |         101 |

Only after BOTH closed did separate cb88 permission release the original9 writes; original application `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution45-v1/application/result.json` (701816 bytes; SHA256 `2edc3e8dbdf49321a0711187f536902951cac8bad30437255c6f691ea02c06f5`) and formatter `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution45-v1/formatter/result.json` (693456 bytes; SHA256 `14c64e9c0ba5270c6871bad2f895a7edada16267a420ce488e99bb49f1df82a5`) remain old-generation history. The original198-line/6709-byte test prefix remains intact. Later GREEN16PASS/3FAILED, six diagnostic-native101 leaves and the temporary restored probe remain honestly failed/diagnostic history with no new pass credit. Neither original RED was rerun or relabeled to manufacture the current RED stage.

## Actual production integration and checked refinements

Fresh block policy combines the actual original-token manager query after preceding cleanup with runtime ledger membership through one pure predicate; full retained cleanup candidates are a separate concern. `PeerRequests` consumes the named46 production API, including real `Some(*peer)` unboxing, and preserves typed continuation denials, earned observations and honest errors. Session paths guard initial/Idle/post-dispatch/receipt request effects under the original lifetime. Outbox selection/removal combines the original Sync lifetime and expected Arc registration; physical message-budget exit retains existing Connected/capability semantics. Complete/transport/encode/error/cancel paths preserve bounded earned accounting without replacement cleanup credit or free physical I/O.

Checked R8 promoted three explicit production owners into the12-owner task scope. The sole `process_actions` result constructor moves an actual successfully connected ReceivedBlock into crate-visible `maybe_completed_block`. Session freshly classifies Block/CompactBlock/BlockTxn against original manager AND runtime request state and carries that genuine completed body into existing `record_block_disposition`, requested-body save and BASIC error handling before accepted progress/durable-tip notification. No public disposition, storage error, compact cleanup or notification order is bypassed. Normal Headers ingress on the bound owner retains genuine accepted proof; `from_entries`/an admission bypass does not supply it. Only the proven obsolete runtime peer-ID capability lookup/import and dead request re-exports are retired after actual caller evidence.

R8.1 changed only Session style: two explicit-auto-deref stars were removed and the unchanged owned-queue test module was placed after the production helper to satisfy strict Clippy; production behavior was retained. R8.2 changes only two test Version expressions in emission/outbox to explicit remote nonce1; all production prefixes and assertions remain exact. Root causal proof `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-real-send-counterfactual-causal-review-v1/authentication.json` (2560 bytes; SHA256 `ec252597ae0872ffe6dddd34e66bd4b9078b5780736d3e92e1bea4501dcf65d1`) and full frame addendum `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-real-send-counterfactual-causal-review-v1/frame-addendum.json` (726 bytes; SHA256 `650bfbe0ffbacf86bcf823f865a76e9e1f950cea342bf7c0f912651866ed4c43`) bind two actual nonce1GREEN/native0 leaves followed by exact current834c restoration and nonce0RED/native101/SelfConnection reproduction. That reversible diagnostic used the current bodies, never c664 where outbox was originally absent. The outside284 `message.rs` default definition stayed separate prospective causal context; it is not silently merged into canonical or inverse scope. Normal mixed child fit618/622 stays below628 without packing or a new limit/owner.

Latest genuine lease `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-actual45-r8-2-release-v1/release.json` (58794 bytes; SHA256 `43a6a9cafc56184e158446e1ee149cb20bbb39322391a0bfb16ac21894db9367`) and separate correction permit `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-actual45-r8-2-release-v1/continuation-permission.json` (4552 bytes; SHA256 `73ecabd187295480539474cd1fbe4c6303399ec8a3bdbd8ff59024e8c77473c6`) applied ONLY those two approved current-present buffers, protecting282other source inputs. Application `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution45-r8-2-v1/application/result.json` (707515 bytes; SHA256 `2c57de06667d587d637c46e3e787c320f1ca8f3dba083b012d044564152d6968`) retained per-fd write/fsync/close/parent-sync primitive outcomes before hashing; formatter `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution45-r8-2-v1/formatter/result.json` (703356 bytes; SHA256 `1aba7eb6cf0ba072dfc879ea205d363646dc2091b4d64ffcade11ce3ef0cfdf6`) was isolated to exactly those two paths and closed0 with empty diagnostics/zero approved-payload delta. All12final bodies remain exact; closed marker `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution45-r8-2-v1/source-writes-closed.json` (350111 bytes; SHA256 `b23a746d37d5f25b7559bfa42ae04af4cabacf902677db7f57d3cc945315c174`) precedes new runtime jobs.

## Fresh Task1b, GREEN and final outcomes

Task1b actually passes the exact full compact handoff/accounting case once; published GREEN passes19including the original duplicate case. After both naturally closed, ONE new joint284 frame preceded all18fresh finals (9Plan45+9Plan46). The actual dispatch scheduled format/strictClippy/jointformat/build first, then original relative test order; data/argv order is unchanged. The table below includes the two predecessors plus the nine Plan45 finals; the other nine original46 rows are detailed in the accepted canonical46 Summary and shared facts.

| Exact command key                | Named PASS | Ignored | Reported test s | Recorded job s | Native exit |
| -------------------------------- | ---------: | ------: | --------------: | -------------: | ----------: |
| `phase167-45-r8-compact-handoff` |          1 |       0 |           20.75 |         33.915 |           0 |
| `phase167-45-peer-green`         |         19 |       0 |           60.51 |         61.041 |           0 |
| `phase167-45-final-format`       | check only |       0 |            0.00 |          2.094 |           0 |
| `phase167-45-final-clippy`       | check only |       0 |            0.00 |         10.211 |           0 |
| `phase167-45-final-lifetime`     |         36 |       0 |           93.37 |         93.599 |           0 |
| `phase167-45-final-resolution`   |         19 |       0 |           60.74 |         60.991 |           0 |
| `phase167-45-final-backoff`      |         12 |       0 |            9.15 |          9.372 |           0 |
| `phase167-45-final-timing`       |         21 |       0 |           13.09 |         13.341 |           0 |
| `phase167-45-final-compact`      |         92 |       0 |           43.47 |         43.692 |           0 |
| `phase167-45-final-authority`    |        132 |       0 |         3993.00 |       3993.375 |           0 |
| `phase167-45-final-sync`         |        582 |       4 |         3440.25 |       3441.013 |           0 |

Exact actual argv for these eleven commands:

```sh
bun run scripts/command-timings.ts run --key phase167-45-r8-compact-handoff -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase167_peer_session_compact_completion_and_every_finish_keep_real_fallback_accounting -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-45-peer-green -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::peer_resolution -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-45-final-format -- cargo fmt --manifest-path packages/Cargo.toml --all -- --check
bun run scripts/command-timings.ts run --key phase167-45-final-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings
bun run scripts/command-timings.ts run --key phase167-45-final-lifetime -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase167_peer_session -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-45-final-resolution -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::peer_resolution -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-45-final-backoff -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::phase70_peer -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-45-final-timing -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::runtime_timing_cases -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-45-final-compact -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node compact -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-45-final-authority -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node network::runtime_authority -- --test-threads=1
bun run scripts/command-timings.ts run --key phase167-45-final-sync -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync:: -- --test-threads=1
```

The nine Plan45 finals give894overlapping passes; the nine46joint rows give234; Task1b1+GREEN19 make1148current overlapping passing observations across20new canonical jobs. These are not unique-case counts. Four non-test fmt/Clippy/build rows require checks, not invented discovery. Every test selector has genuine nonzero passing discovery; no FAILED or UNPARSED result is hidden. Original duplicate and phase70 case full names pass in their mandated GREEN/final mappings. Both formerly failing real-handshake cases individually pass in default and all-feature lifetime rows.

Exactly four ignored outer outcomes occur only in final-sync, with actual reasons:

- `sync::tests::errors_and_live::live_network_smoke_is_explicitly_opt_in`: requires public Bitcoin network; set OPEN_BITCOIN_LIVE_SYNC_SMOKE=1 to run.
- `sync::tests::filter_index::catch_up::measurements::phase157_measure_turns`: explicit timing evidence; no wall-time assertion.
- `sync::tests::filter_index::phase162::process::tests::phase162_process_child`: invoked only as a bounded subprocess control.
- `sync::tests::filter_index::reorg::measurements::phase158_reorg_measurements`: explicit concrete accounting and timings; no wall-time pass/fail ceiling.

They are retained without pass credit or extra outer activation; ordinary parent-driven bounded process controls remain unchanged. All reported target totals reconcile with raw names. No prior R7/R8/R8.1/counterfactual pass supplies these new runs. Current strict-D Clippy, both whole-workspace fmt checks and all-target/all-feature Node build close0with zero actual compiler diagnostic headers. Expected caught poison traces remain raw.

## Original inverse and current body conservation

Current scratch-only inverse `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution45-r8-2-v1/inverse/result.json` (2093 bytes; SHA256 `992c71c6a6689389a9f5c2a0be27fed1b135b666e67c6b447c38f9d5291b0ca5`) and full handoff `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution45-r8-2-v1/inverse-handoff-v1/index.json` (146584 bytes; SHA256 `246caa11fbca372fa90c0ff6c6e465581a9d4437320cb31faf6039e7ce64d5d3`) bind the originalc66412-owner8whole4TRUEabsence geometry. Exact BSD `--batch -E -F0 -p1` reverse verified280file membership/original bodies and protected272others, then forward verified all284current bodies with no offset/fuzz/reject. Native201/separate1were preserved separately; no live tree was patched. Post-inverse/current-body read/hash checks cover all486physical current bodies without a new copied capture or atomic claim. Original46de637/expanded498 and original35exclusive5b0retain separate meanings.

## Evidence, environment and remaining gates

The shared facts are `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/summary45-46-r8-2-preparation-v1/facts.json`: exact source-copy refs, all source hashes, original/current provenance, actual argv, raw-result links, measured timings, named outcome mappings, ignored reasons and stage ordering. Full raw names are retained in the actual CORE outcome artifact, rather than duplicated here. Primary joint CORE: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution45-r8-2-v1/joint-CORE-v1/CORE.json` (7739630 bytes; SHA256 `c20eb61cfc0d7623e05dc890e9ad784c4defbae60a4c7a323734426e4079b722`). CORE handoff: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution45-r8-2-v1/CORE-HANDOFF.json` (1682 bytes; SHA256 `f623c1e86845a6658e386e4dee1fc6c992d54f5fd69887d5a19e39c3909e1a4d`). Full delivery: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution45-r8-2-v1/core-delivery-v1/index.json` (7656797 bytes; SHA256 `830ac689a2cef49f4913a8ac931ba7ba9103902f83feaa3e831b9fc786a55958`). These are actual evidence, not an independent/ROOT final acceptance.

ROOT all20 authentication: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-r8-2-all20-review-v1/authentication.json` (6646 bytes; SHA256 `dbb3061f42393f778fb9a9707983d33572ab1956a98ed5d702f9b7b1b5fdfdf3`). ROOT inverse authentication: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-r8-2-inverse-review-v1/authentication.json` (1295 bytes; SHA256 `cd4fee3534b97dd9547c720647ce4d3151b354e60cb14674d8137c158a697d6e`). Actual19-source/API prerequisites: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-r8-2-core-input-review-v1/api-prerequisites.json` (14967 bytes; SHA256 `de08f2740106ecb2484c11aab669227dec72d1d7cf7ca396f5072da2700bb065`). Exact zero-diagnostic decision: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-r8-2-core-sealing-v3/diagnostic-decision.json` (9774 bytes; SHA256 `cfc52815ec88eb29f23cfafad62fe04b5fb7604940a8cef0c236479af230c6fe`). The historical26 operative unused-API warning groups (286 operative occurrences plus22 compiler-summary occurrences,308raw total) are retained in their original records. Actual production consumers/obsolete lookup and re-export retirement plus current strict-D and all20 zero-warning runs close those residuals; no lint whitelist, suppression, widened public API or artificial caller supplies the closure. The existing `Some(*peer)` unbox is part of the real Plan45 consumer body.

Actual two CORE-only metadata guard stops remain separate: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution45-r8-2-v1/core-sealing-HOLD-v1/index.json` (3612 bytes; SHA256 `17b8d2e1b3e46c336149f43b6c99a3994825f3d65a3fa9d1ae3c9513dbe1355f`) records relative-versus-absolute sealer-path rejection; `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution45-r8-2-v1/core-sealing-HOLD-v2/index.json` (3605 bytes; SHA256 `1333fb81d7d19cd354382fda4f8bd5956ee7471209abb0db8b08d484d3e3d0dd`) records missing exact Root diagnostic-decision fields. Both closed outer1 before CORE output and supply no runtime/source pass credit. Successful v3 uses `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-r8-2-core-sealing-v3/permission.json` (5349 bytes; SHA256 `327e024f55a3f0ac24bf720a94c7720945d4f2f793b69ad220b62bb0315283a4`) and `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root-r8-2-core-sealing-v3/binding.json` (9222 bytes; SHA256 `0e198ab1f67af8e65fb175aa0771020cc1c86db3426a7a20cfbb57908bb13ec7`); it closed0 and is not counted as a failure. Every old receipt and HOLD remains unchanged.

Pinned Bun1.3.9/Rust1.94.1, default target/cache and original physical TMPDIR were retained. Actual asynchronous nine-key child probes and product commands used the same explicit snapshot; the six target/state/flag/toolchain overrides remained unset. Natural host-global/cooperative timing guards serialized work. No signal, latency deadline, held host-global-lock theorem, atomic snapshot or observation-to-spawn race elimination is claimed. Native ORIGINAL201 and separate Byteview1 remain separate lists,202derived total; no old201coverage is invented. No failed fixture was accessed or reused.

The current joint284 frame is `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/execution45-r8-2-v1/joint-final/capture.json` (187416 bytes; SHA256 `d3c5ea565300f4e7396a6f323801de546835ef6f9ca73282662f0e1354d79861`). All20 fresh jobs were closed before the original12 scratch inverse. Source writes remained closed while runtime jobs ran. All18 finals started after the maximum source/native/supplemental capture end. Ignored outcomes are never pass credit: exactly four outer final-sync name/reason pairs are retained, multiplicity1, with no added outer `--ignored`/`--include-ignored` or live opt-in. Existing ordinary bounded subprocess supervision remains unchanged.

Independent actual CORE review: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/independent-r8-2-actual-joint-core-review-v1/index.json` (6674 bytes; SHA256 `8cc74e5c20a8a583aa0533ce88baeb0407973783bda80db3308316123cd73848`), with whole report `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/independent-r8-2-actual-joint-core-review-v1/REPORT.md` (16542 bytes; SHA256 `01353860a64f7e0e66e25d8aeed5f60cc739a968c2291b23de71dab200ab53d2`). The sealed review has0findings/18094semantic checks, authenticates all28925CORE files, binds all19source/API owners and all26historical warning groups to actual production links, and reconciles1148overlapping passes/four exact ignored outcomes. It grants no Root adjudication, Summary release or counted source acceptance. ROOT final46 acceptance is now genuine: `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root46-r8-2-final-acceptance-v1/acceptance.json` (8213 bytes; SHA256 `8fc69656cb28d2aa097f7ae76de795c4efe228b64ff405f1322280fdd338fc0d`), status ROOT_FINAL_SOURCE_ACCEPTED/sourceAccepted=true on d3c5, after accepted46Summary `.planning/phases/167-candidate-history-validation-rules/167-46-SUMMARY.md` (25064 bytes; SHA256 `f19c1836672d86915d551009b00e629851aa9840828168afe8464a802b30c731`) and independent Summary46QA `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/independent-summary46-r8-2-qa-v1/index.json` (9036 bytes; SHA256 `bcf196c4177574b31fa0d0ebef130c400d5cf418f7de7432086acbef28f81c2a`). This is distinct from unchanged old051 dependency-only readiness. ROOT separately released ONLY this Summary45/scopedQA under `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/root45-r8-2-core-acceptance-summary-release-v1/release.json` (5374 bytes; SHA256 `6665f9365e4f1a21f80107411e521f303bd398359aad78e0ec95f6bd0f7aa776`); sourceCoreAccepted=true/sourceAccepted=false for45. Independent Summary45QA and final counted Root45 acceptance remain PENDING. Current document QA is under `.local/open-bitcoin-dev/phase167/full35-repair-tail-plan/summary45-r8-2-execution-v1/document-QA/`; the earlier inert QA proposal remains separate. No GSD verifier was run against an absent canonical Summary. Summary46 remains unchanged.

No global CVLD closure, full35 rerun, original35exclusive5b0 replacement, shared STATE/count/requirements/ROADMAP mutation, commit/push or phase168 start follows. Full35 remains pending its own ROOT-released repair/composite/focused and final checks. Whole167/security/parity/breadcrumbs/README/default native/coverage/Bazel/normal-hook main finalization remain ROOT-owned. STOP168.

## Self-Check: PASSED

The twelve current whole source hashes and immutable joint/preconsumer/original inverse evidence are bound by the recorded facts. All284source/original201/separate1 current input hashes were checked for publication. Sealed independent actual CORE PASS8cc74e5c and ROOT Summary-stage grant6665f936 are bound; sourceCoreAccepted=true and sourceAccepted=false remain distinct. Actual canonical creation time is recorded. No commits are expected under commits:false. Independent Summary45QA and final counted ROOT acceptance remain PENDING. All five global CVLD,35,whole167,Git and168 gates remain open.
