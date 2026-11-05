---
phase: 167-candidate-history-validation-rules
plan: "32"
subsystem: codec-consensus
tags: [rust, wire-decoding, signed-outputs, intrinsic-validation, allocation, merkle]
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
requires:
  - phase: 167-31
    provides: Accepted serialized ScriptBuf storage and syntax-only counters
provides:
  - Complete signed-output wire blocks and strict serialized domain records through one reader
  - Shared borrowed serialization, exact identities and ordered intrinsic checks
  - Selected raw PoW before intrinsic body checks with direct witness-independence evidence
affects: [167-33, 167-30, 167-13, 167-17, 167-18, 167-19, 167-20, 167-25]
tech-stack:
  added: []
  patterns: [Private decode modes, borrowed serialization facts, incremental fallible container growth]
key-files:
  created:
    - packages/open-bitcoin-codec/src/transaction/wire.rs
    - packages/open-bitcoin-codec/src/transaction/wire/tests.rs
    - packages/open-bitcoin-codec/src/block/wire.rs
    - packages/open-bitcoin-consensus/src/transaction/intrinsic.rs
    - packages/open-bitcoin-consensus/src/block/intrinsic.rs
    - packages/open-bitcoin-consensus/src/block/intrinsic/tests.rs
  modified:
    - packages/open-bitcoin-codec/src/transaction.rs
    - packages/open-bitcoin-codec/src/block.rs
    - packages/open-bitcoin-consensus/src/crypto.rs
    - packages/open-bitcoin-consensus/src/transaction.rs
    - packages/open-bitcoin-consensus/src/block.rs
    - packages/open-bitcoin-consensus/src/block/sigops.rs
key-decisions:
  - Preserve immediate checked Amount/script construction in bounded mode; raw mode completes decoding first.
  - Share serialization/hash/intrinsic algorithms over borrowed facts without creating authority.
  - Keep selected raw PoW separate from the unchanged typed entry's default PoW semantics.
  - Treat actual absence as part of inverse evidence, with explicit BSD patch -E proof.
requirements-completed: [CVLD-01, CVLD-02, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-22T11:04:02.923Z"
commits: false
duration: "22m 51.124s through summary content generation; document QA separate"
completed: 2026-10-22
---

# Phase 167 Plan 32: Lossless wire and shared intrinsic validation Summary

**Complete signed-output wire decoding now feeds exact shared identities and ordered selected intrinsic checks while retaining checked Amount and every bounded codec entry.**

All three tasks cover exactly twelve source owners. The baseline commit above is existing Git history, not the Bitcoin Knots pin or a task commit. This plan records a bounded requirement contribution; root owns global requirement completion and all later phase gates.

## Files Created/Modified

These are the exact frozen source files. All are normally formatted and below 629 physical lines; maximum 559. Source SHA256 covers the complete file. API SHA256 covers its literal public/crate-visible declaration lines as inventoried in the source closure; an empty declaration set has the SHA256 of empty bytes. Complete signatures/bodies remain bound by source SHA256.

| Source                                                         | Lines | Source SHA256                                                    | API declaration-line SHA256                                      |
| -------------------------------------------------------------- | ----: | ---------------------------------------------------------------- | ---------------------------------------------------------------- |
| `packages/open-bitcoin-codec/src/transaction.rs`               |   414 | b2d3bcbc770e13957d4d35fd451361b9668ecb770af98e26dcba4884d487aaea | 3c432e0977634eb0a467b2372ba063592293194e409a5e2d21a700cfd0254b07 |
| `packages/open-bitcoin-codec/src/transaction/wire.rs`          |   125 | 5c8adb1887e1f9a8b7ea3f910c9978cbc0c2b52946920b6d01a26960b0571a3b | 6012b8b47f31bd5a2cf0c32b7615ac791b8f8052d765f7216790549f21c50ae2 |
| `packages/open-bitcoin-codec/src/transaction/wire/tests.rs`    |   312 | 0a143a2b5378651238a2b8d200c90acde1dd041cb507947b9140d9ee80e59d76 | 92c9898695fb2b602d6e33a5cbe67bd877c06499da51d18453426f661e7023c3 |
| `packages/open-bitcoin-codec/src/block.rs`                     |   164 | 0e8daad933702881d6e34ec2a4abd69bd34953f0854a808cb1dc1dff2a44c68b | 79a11fddd7c46291a0c19bac1adbaab27f3cb4aab82e58a1a28885c1a0da9474 |
| `packages/open-bitcoin-codec/src/block/wire.rs`                |   117 | 9ed634a725554b851ad2efbec9103560aa8b5cad7e546435787c5b8ff6813d59 | 5c90bee1475b6e333c1d480d8d71bbca86c26dd07f41dece977e74a3fb5009e9 |
| `packages/open-bitcoin-consensus/src/crypto.rs`                |   559 | 399e8a496e7d0c1b564027f01ab1067c8bceebff189b3866b1b0cc764aece963 | 297b5faf214afd19ab6c7d4cc3af2157c58ceb7ad8f88222fbeecb4f24309392 |
| `packages/open-bitcoin-consensus/src/transaction.rs`           |   550 | 57922894913bedb8c588be50df05defcacf876be8294c149de803eaa19f19113 | cbe0e9e48473deacc2ed3bf229f261e385e3a5eef3baab8a785f824a72aa85db |
| `packages/open-bitcoin-consensus/src/transaction/intrinsic.rs` |   170 | f6b13b314d490247270bbf16980e58c8da030beadfac50427214cafac3a396cf | e74218246c7b7aa955881def57eab129cfa5bfd855c3901b4af5d6fefba33fd4 |
| `packages/open-bitcoin-consensus/src/block.rs`                 |   479 | 454932803af857e1f83bc9e343a4414dd3c45436463e98abdf5a373c64b6f078 | b86bc223d9ed131d7b58ed6b28a5462c24b3568190cc63cb5f1921a16bb32882 |
| `packages/open-bitcoin-consensus/src/block/intrinsic.rs`       |    95 | de471b8a61a8bb01b5f06b13e4b3d1ce4d4e5b007c2824079ced4dfe3924d274 | 850a10e7ebf16aab2d9f6adea15aa85848ecdb881ab3efe01d334a7eb473500e |
| `packages/open-bitcoin-consensus/src/block/sigops.rs`          |   139 | 745659a3dda954d4339685be1e46796da204fe56fc248bc6cc94909fad641afa | f23006687deb8c17be0daf1e4241588fee6c523a9812b06beca9191636025c97 |
| `packages/open-bitcoin-consensus/src/block/intrinsic/tests.rs` |   295 | a6effdf95ea73cedf3a3e444726cc71f959f29777e1eb39a4e0cadb679b65eb1 | e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 |

## Tasks and Public Contracts

1. Codec: one structural transaction reader and one block-parts reader support private bounded-domain, serialized-domain and submission-wire modes. Bounded mode still constructs Amount before reading each output script and still checks scripts at their original point. Serialized mode checks Amount while storing scripts losslessly. Submission mode retains signed i64 and scripts until complete witness/flags/locktime decode. Old public and crate-private reader signatures remain unchanged. Container allocation now follows successfully decoded elements through fallible growth.
1. Transaction: typed wrappers and raw callers use the same view serializer, txid/wtxid/merkle algorithm and intrinsic transaction checker. Negative, individual maximum and aggregate range checks precede duplicate-input/coinbase/null-prevout checks. Checked addition cannot overflow from unvalidated signed values; Amount has no escape constructor.
1. Block: the typed entry retains its default header proof semantics. The raw entry first checks the supplied selected parameters, then both routes share merkle/mutation, stripped size, coinbase multiplicity, transaction and legacy-sigop checks. No contextual witness/script/header admission, store operation or canonical receipt is introduced.

New full API paths are open_bitcoin_codec::transaction::wire::{WireOutput,WireTransaction,OutputRef,TransactionView,parse_serialized_transaction}, open_bitcoin_codec::block::wire::{WireBlock,BlockView,parse_submission_block_prefix,parse_serialized_block}, open_bitcoin_codec::transaction::encode_transaction_view, open_bitcoin_codec::block::encode_block_view, open_bitcoin_consensus::crypto::{transaction_txid_view,transaction_wtxid_view,block_merkle_root_view}, open_bitcoin_consensus::transaction::intrinsic::check_transaction_view and open_bitcoin_consensus::block::intrinsic::check_submission_wire_block. WireTransaction::try_into_transaction and WireBlock::try_into_block move scripts/witness exactly through fallible monetary conversion. Views expose facts only; the actual held node owner must supply selected authority. Regtest-only submission and unsupported Signet challenge refusal remain runtime-owner obligations.

## Checks and Evidence

Evidence base: .local/open-bitcoin-dev/phase167/plan32/. Every owned command directory under execution contains started.json, exact argv/environment/times, complete before copies, result.json with before/after SHA256 fences, and raw.log with its SHA256. The initial receipt preserves six original sources and six actual absent children. Source capture deliberately excludes root metadata, planning reports and unrelated active evidence.

| Command label                 | Exit | Actual passing tests / result            | Seconds |
| ----------------------------- | ---: | ---------------------------------------- | ------: |
| task1-red                     |  101 | 1pass/4 behavioral failures              |  51.172 |
| task1-green                   |    0 | 10                                       |   7.129 |
| task1-regressions             |    0 | 77unit+3property                         |  12.879 |
| task2-red                     |  101 | 2pass/2 behavioral failures              |   9.006 |
| task2-green                   |  101 | test comparison compiler errors          |  65.621 |
| task2-green-type-repair       |    0 | 5                                        |  14.266 |
| task2-regressions             |    0 | 36, interim before block additions       |   0.421 |
| task3-red                     |  101 | existing test import compiler error      |   4.293 |
| task3-red-import-repair       |  101 | 7pass/1 behavioral failure               |  64.356 |
| task3-green                   |    0 | 9                                        |  13.799 |
| final-clippy                  |    0 | all-target/all-feature strict lint       |  17.740 |
| final-codec-wire              |    0 | 10                                       |   3.657 |
| final-codec-regressions       |    0 | 77unit+3property                         |   3.135 |
| final-transaction             |    0 | 5                                        |   0.327 |
| final-transaction-regressions |    0 | 38                                       |   0.165 |
| final-intrinsic               |    0 | 9                                        |   0.310 |
| final-block-regressions       |    0 | 105unit+2BASIC integration               |   1.376 |
| final-capacity                |    0 | 10, with actual capacity diagnostics     |   0.290 |
| final-consensus-all           |    0 | 378unit+8BASIC+3parity+2taproot+4doctest |  21.940 |
| final-format                  |    0 | scoped rustfmt check, skip_children=true |   0.059 |
| final-diff-check              |    0 | owned-path diff check                    |   0.019 |

All seven exact planned automated commands were run on the final twelve-source freeze. Additional full-consensus/capacity checks use that same freeze. Zero-match secondary harnesses are excluded from passing totals; no ignored tests occurred. Repeated selections overlap and are not additive unique coverage. Total measured command intervals are 291.960s; the eleven final command intervals sum 49.018s. Complete argv are retained; all Cargo uses command-timings.ts, Bun 1.3.9, pinned Rust 1.94.1, existing packages/target and the physical TMPDIR. No Cargo/Bazel job from this executor overlapped another owned job.

Task 1 RED demonstrates amount/script premature rejection, late EOF precedence and actual eager capacity 4096 on a 7-byte declaration. Task 2 RED demonstrates individual negative values reaching the aggregate error. Task 3 corrected RED demonstrates default PoW incorrectly reaching merkle before the selected limit. Compiler/setup errors are retained separately and are never behavioral RED. Test-only slice comparison and preserved cfg(test) parent import repairs changed no expected bytes or assertions.

Cursor rust-analyzer did run separate workspace Cargo checks, causing real artifact-lock waits. The second writer's exact argv/PPID is captured in artifact-lock-second-processes.txt; source stayed fixed during each owned command and the waits ended naturally. This is sole agent-owned scheduling with disclosed external analyzer contention, not a no-contention claim. No quiet build was terminated, target copied, deadline reset or cache configuration changed. Disk remained 18GiB free at the recorded samples; the read-only du attempt's completion output was not collected and supplies no cache-size claim. Additional setup lookup observations remain in setup-observations.md.

## Actual Allocation and Boundary Evidence

The values below are actual Vec capacity events captured by test-only observation in the same parser before its eventual EOF. Declared count, complete input byte length and allocator-chosen capacity are separate observations; capacity 4 is a measured host result, not a new wire cap or allocation promise.

| Container / decoded elements | Declaration4096 input bytes | DeclarationMAX_SIZE33554432 input bytes | Observed capacities | Eventual needed/remaining EOF |
| ---------------------------- | --------------------------: | --------------------------------------: | ------------------- | ----------------------------- |
| inputs /0                    |                           7 |                                       9 | 0                   | 32/0                          |
| inputs /1                    |                          48 |                                      50 | 0,4                 | 32/0                          |
| outputs /1                   |                          58 |                                      60 | 0,4                 | 8/0                           |
| witness items /1             |                          62 |                                      64 | 0,4                 | 1/0                           |
| block transactions /1        |                         143 |                                     145 | 0,4                 | 4/0                           |

The original RED observed capacity 4096 before the same 32/0 EOF on the 7-byte input. Final MAX_SIZE declarations therefore execute safely without eager declared-count reserves. CompactSize bounds/noncanonical checks are unchanged; growth failures reuse LengthOutOfRange. Payload vectors still require their encoded bytes before allocation through the original Reader.

Raw signed i64::MIN, -1, MAX_MONEY+1, i64::MAX and scripts of 10001 bytes roundtrip exactly with/without witness. Late script/witness/flags/locktime defects remain decode failures; strict records reject suffixes while a complete block prefix returns the consumed offset. Bounded combined defects retain amount-before-script/EOF and script-before-locktime refusal. Independent Node-compatible crypto hashes literal LE layouts without invoking product serialization/hash code; identity-fixtures.json SHA256 127b25d3926df889121663a184b384ac28f787bc748441a3a4fb2fdad7a16949 binds signed txids/wtxids, raw merkle leaves and the wide-script identity.

The witness-reserve test uses an actual commitment-bearing coinbase with the same missing/filled 32-byte reserve. It directly compares header bytes/hash, every txid, merkle root, stripped block and transaction bytes/sizes, coinbase structure, exact output/prevout facts and legacy sigops, plus intrinsic results for valid and invalid-money bodies. Only total witness serialization size changes. Intrinsic code never reads witness commitment, contextual scripts or witness weight. Selected-PoW/merkle, merkle/empty/size/amount, mutation, coinbase multiplicity/amount, duplicate-input and legacy-sigop precedence controls pass. Typed/raw valid wide-script behavior shares the same counters and checker.

## Evidence Versions and Source Protection

Final source/API closure final-source-v2.json has SHA256e834d4f8369a22b36b76023e1bb143ce966904b678e34e3bdeee158c37f5be06. Source and protected hashes are identical to the first freeze; v2 only fixes patch representation. The old final-source.json SHA2564e4b337e2fceff6ee288acfd21c7821fb95706bd54e0daf2f89c277c1c272838 and original patch bytes remain history.

Original v1 new-file patches incorrectly forced a/b labels despite /dev/null comparison input; their headers did not encode actual deletion. Root caught the evidence defect. All 24 v2 headers now reflect six existing parents and six new children, including /dev/null targets in inverse deletion patches. The first v2 sandbox proof omitted BSD patch -E and left empty files; its unsuccessful absence result is retained in patch-proof-v2.json. The separate successful patch-proof-v2-E.json SHA256 ca47714535060c2615b21074b60480f72d56298d849ea856cad8c9d0d6b95dc8 uses explicit patch --batch -E -F0 -p1. It verifies every forward hash, every original parent hash and actual absence of all six children after inversion. These operations occurred solely in ignored evidence sandboxes; actual source was never applied/reverted. Existing-parent v1/v2 patch pairs are byte-exact.

evidence-audit.json SHA256 76d83c9e71d9e7f6a5bbe8f2c69c5b53e4a95c6d5fa28de25ba2535210ec7d0d passes/errors:[] for all 21 historical results, eleven final-source commands, exact raw-log hashes, before/after fences and current source. Root's independent final-evidence audit R1 SHA256 30d094bcbdead4c47f1b7371c0f651912b383766eb7e0a806f604fd6a5a1d4d2 also passes all 21/11 results and 20 path fences; root reproduced the v2 inverse proof independently. Root's initial audit reader expected 12 rather than 20 source/protected fences, retained that failure, and corrected the reader without changing evidence.

Independent codec review codec-source-review/REVIEW.md SHA256 0ebd0297f94f9f8aba2a46327d210694b20b3c8a9f6c9f7035c94d08cd0f0e15 and consensus review consensus-source-review/REVIEW.md SHA256 559cd814b29daf6848403269040038817062e56ab6b1513b3007a429fe30d66b are CLEAN with zero findings. The latter binds the twelve-file union, including the explicitly reviewed codec test logging/format delta; hashes.json SHA256 f69b3f2d7a7b763a35c17bc09e64c61744f84fc40ec40821c728b05ce3aa056b agrees with the final source. These bounded source reviews do not replace root implementation acceptance or later phase gates.

Amount remains exact SHA256 b174241ff27077d71a71b0eecab728839b84feb756edaef0dcd5c1e163836f35. Accepted 31 ScriptBuf, Reader, CompactSize, CodecError, both lib.rs exports and compact-block ingress source remain unchanged. The immutable source pin is Bitcoin Knots a9aee730466ac67d35a3c03ee24676be5e045878. Every owned breadcrumb names an existing pinned source. Central manifest reconciliation remains Plan 19's owner.

## Decisions, Deviations and Limits

The explicit simplification pass retained one mode-based structural reader, one view serializer/hash path and one shared checker per intrinsic level. Mode hooks occur at the existing construction points; no raw-then-convert bounded parser, second structural parser, substituted identity, placeholder transaction, amount clamp, production instrumentation, dependency or extra source owner was added.

The implementation followed the checked plan. Compiler/test typing repairs and the evidence-only patch-label/BSD deletion correction remained within owned files/ignored evidence. No architectural, source-budget, allocation-cap, deadline or ownership expansion occurred. No authentication gate was encountered. No goal-blocking stub was found; empty script/witness/default fields are real data states or test fixtures. The new pure raw-data surface is within the plan's threat model; it introduces no extra network endpoint, authentication path or filesystem boundary.

Existing bounded P2P/network/message/compact-block ingress keeps its wide-script limitations. This work supplies pure submission/durable decoding seams; runtime holding/selection/sealing, node wiring and canonical acceptance remain later owners. It claims local pinned-source and actual Rust execution evidence only. Actual Knots daemon/differential qualification, broad network parity, whole-chain acceptance, production/funds readiness, normal native verification, fresh coverage, Bazel, independent source/security/goal/lifecycle/compact gates and Git saving remain root/later obligations.

Local AGENTS, sidecar, standards-overrides and architecture/code-shape/testing/verification/Rust guidance informed scope, pure boundaries, naming, tests and scoped checks. Global/repository active lessons were measured 6898+1958=8856 bytes and read completely. Shared STATE/ROADMAP/requirements/config/metrics/todo/manifest and guard/lifecycle ownership remained with root.

## Performance and Task Commits

Tasks: 3; files: 12 (six modified/six created). Task intervals: 10:41:11.799Z–10:45:39.432Z (267.633s), 10:45:39.432Z–10:49:22.686Z (223.254s), and 10:49:22.686Z–10:53:12.967Z (230.281s), all on 2026-10-22. Startup reads before the first explicit sample are unmeasured. Final source/check/evidence and document work follows those task intervals. The measured interval 2026-10-22T10:41:11.799Z–2026-10-22T11:04:02.923Z is 1371.124s; document QA is recorded separately. There was no requested source-review hold during these intervals.

No commits were created: commits:false is explicit. HEAD remains the baseline and the index was empty at the recorded check. Sole agent-owned build scheduling was released to root after all owned jobs completed; root controls subsequent scheduling and acceptance.

## Self-Check: PASSED

All twelve source paths exist and match the final closure. Protected inputs agree, baseline history exists, every final command fence/log hash agrees, all scoped source checks pass and each named selection discovers actual tests. Three tasks and twelve owners are accounted for. V2 forward/inverse proof restores exact original parents and actual absence of all six new children.
