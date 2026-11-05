---
phase: 167-candidate-history-validation-rules
plan: "02"
subsystem: consensus-fixtures
tags: [bun, historical-corpus, bip30, script-flags, provenance]
requires:
  - phase: 167-01
    provides: Selected immutable network facts and source inventory
provides:
  - Seven exact serialized historical bodies with independent hash, merkle and witness checks
  - Complete bounded seeded header, expiry, previous-transaction and UTXO envelopes
  - Independent pinned-source expectations and an offline integrity checker
affects: [167-07, 167-08, 167-16, 167-18, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Explicit acquisition, Offline checking, Lossless response references]
key-files:
  created:
    - scripts/phase167-historical-corpus.ts
    - scripts/phase167-historical-corpus.test.ts
    - packages/testdata/phase167/manifest.json
    - packages/testdata/phase167/blocks.json
    - packages/testdata/phase167/contexts.json
    - docs/parity/fixtures/phase167-rule-expectations.json
  modified: []
key-decisions:
  - "Independent seeded segments never establish ordinary canonical ancestry or actual-daemon qualification."
  - "Exact coinbase bytes bind repeat91842 to original91812 and repeat91880 to original91722."
  - "Raw transaction responses reference existing identical serialized transaction bytes, with exact response SHA256 and URL binding."
requirements-completed: [CVLD-02, CVLD-03, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-21T20:24:15Z"
commits: false
duration: 176min
completed: 2026-10-21
---

# Phase 167 Plan 02: Independent exact historical corpus Summary

**Seven real historical bodies now have complete bounded seeded prerequisites, exact response provenance and independent pinned-source expectations, validated entirely offline.**

Source checkout baseline, not a task commit: `1b472fb225a290df5cdb06eeea2dc67c624c648e`, observed with git rev-parse HEAD. No task commit exists or is claimed.

## Files Created/Modified

- Created: `scripts/phase167-historical-corpus.ts` — explicit acquisition and offline checking;569 lines.
- Created: `scripts/phase167-historical-corpus.test.ts` —55 focused positive, corruption, authority and resource cases;282 lines.
- Created: `packages/testdata/phase167/manifest.json` — fixed identities, twelve source digests, acquisition outcomes and resource accounting.
- Created: `packages/testdata/phase167/blocks.json` — unchanged original serialized bodies and their acquisition provenance.
- Created: `packages/testdata/phase167/contexts.json` — linked windows, detached anchor, previous transactions, proofs and complete candidate-input UTXOs.
- Created: `docs/parity/fixtures/phase167-rule-expectations.json` — source-derived activation, flags, locks, difficulty, expiry and BIP30 cases.
- Created: `.planning/phases/167-candidate-history-validation-rules/167-02-SUMMARY.md` — attributable execution results.

## Completed tasks

1. Acquire finite exact exception bytes and provenance: all seven fixed identities are frozen. The checker independently decodes serialized transactions, txids/wtxids, header double-SHA256, transaction merkle root and witness commitment. Changed bytes, selected network, supplied hash, pin, source digest, truncation and extra payload refuse. Explicit acquisition permits only fixed Blockstream/mempool HTTPS data routes, refuses redirects/credentials/query tricks, and bounds response size, cumulative payload and time.
2. Complete history and UTXO envelopes with source expectations:5263 previous transactions and6223 candidate-input UTXOs have exact raw serialization, inclusion proofs, creating headers and complete creating-parent MTP windows.13230 headers cover all required segments. Each candidate has a linked suffix including parent MTP and the eleven-header expiry MTP ending at parent.height-min(144,parent.height); high-height suffixes contain155 rows, and testnet394 retains its394-row recovery path. Every walk is bounded by4096 rows.

| Network | Height | Exact block hash | Purpose |
| --- | --- | --- | --- |
| mainnet |170060|00000000000002dc756eebf4f49723ed8d30cc28a5f108eb94b1ba88ac4f9c22|P2SH replacement NONE|
| mainnet |692261|0000000000000000000f14c35b2d841e986ab5441de8c585d5ffe55ea1e395ad|Taproot replacement P2SH/WITNESS|
| testnet3 |394|00000000dd30457c001f4095d208cc1296b0eed002427aa599874af7a432b105|P2SH replacement NONE|
| mainnet |91842|00000000000a4d0a398161ffc163c503763b1f4360639393e0e4c8e300e0caec|Repeat of91812|
| mainnet |91880|00000000000743f190a18c5577a3c2d2a1f610ae9601ac046a38084ccb7cd721|Repeat of91722|
| mainnet |91722|00000000000271a2dc26e7667f8419f2e15416dc6955e5a6c6cdf3f2574dd08e|Original repeated at91880|
| mainnet |91812|00000000000af0aed4792b1acee3d966af36cf5def14935db8de83d6f9306f2f|Original repeated at91842|

The mainnet BIP34 header at227931 is a separately hash-bound independent seed. It is never described as linked to692261 across the missing interval. Genuine absence means parent height is below activation. High-height missing-anchor formula rows explicitly say unavailable/conditional and ordinary-context refusal; at1983702 the anchor is unneeded for unconditional BIP30 scanning. These data are seeded historical evidence, not an ancestry-complete chain or actual Knots-daemon comparison.

## Handoff contract

- Blocks retain network, height, exact hash, serialized raw bytes, raw SHA256, URL, acquisition date and outcome.
- Header rows retain raw80-byte serialization, selected network, height/hash and the exact source response URL. Hash lists are stored in ascending height order; CandidateHistory callers reverse candidate_history to supply parent followed by descending ancestors.
- Envelopes retain parent_window, candidate_history, expiry_window/expiry_mtp, difficulty path, explicit BIP34 state, external_utxos, overwrite_original hashes and expected_flags.
- Previous-transaction rows retain raw bytes, txid, inclusion/status response references, creating block/height and independently computed creating MTP. Their distant creating windows are independent seeded segments, not a fabricated continuous branch.
- Responses retain exact original URL/date/SHA256 and either raw payload hex or a restricted network:txid transaction reference. References resolve only to transactions[].raw, never to another response. Exact bytes and SHA256 are checked before use; JSON responses retain original payload bytes, including whitespace and case.
- bip30_pairs explicitly binds91842→91812 with coinbase txid d5d27987d2a3dfc724e359870c6644b40e497bdc0589a033220fe15429d88599, and91880→91722 with e3bf3d07d4b0375638d5f1db5255fe07ba2c4cb067cd81b84ee974b6585fb468. Consumers must use these pairs rather than zip the separate original/repeat lists. The original body's coinbase/output and parent MTP window supply complete earlier-coin restoration facts.

## Executed commands and actual outcomes

All Bun commands used PATH=/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin:$PATH. Complete receipts remain under .local/open-bitcoin-dev/phase167/plan02/.

| Command / stage | Actual outcome | Retained receipt |
| --- | --- | --- |
| bun test ./scripts/phase167-historical-corpus.test.ts — Task1 | RED0/8; GREEN8/8. | task1-red.log; task1-green.log |
| bun run scripts/phase167-historical-corpus.ts --acquire — bodies | Exit0; all seven acquired and independently decoded. | task1-acquire.log |
| Initial body --check | Exit1 because a repeated-group lexical regex refused valid large hex; corrected linear character-class/even-length check then exit0. | task1-offline.log; task1-offline-final.log |
| Task2 behavior tests | Actual RED11pass/2fail; later13,20,23 and24-case suites green. | task2-red.log and successive unit logs |
| First Task2 setup | Exit1: Git show does not normalize src/../CMakeLists.txt; corrected canonical Git path. | task2-acquire.log |
| Unpaced Task2 acquisition | Exit1 with seven recorded HTTP429 outcomes; immutable successful cache retained. | task2-acquire-after-source-path-fix.log; acquisition-failure-1792604142055.json |
| Intermediate offline tests | Exit1 with two stale source-inventory assertions while the expanded manifest had not yet been written. | task2-intermediate-offline-refusal.log |
| Paced Blockstream acquisition | Eight further HTTP429s with absent Retry-After and logged30s cooldowns. Root authorized SIGINT after observed persistent rate limits; producer and observer actually exited130. This was not a timeout. | task2-acquire-paced.log; paced-blockstream-terminal.json |
| Alternate acquisition, frozen source312b84a2 | All5263 previous transactions acquired; actual final exit1 because my BIP30 pairing assumption was reversed. | task2-acquire-alternate.log; acquisition-failure-1792613153586.json; bip30-pair-identities.log |
| Pair/compact regressions | Actual RED24pass/2fail; GREEN26/26 after corrections. | task2-pair-and-rounding-red.log; task2-pair-and-rounding-green.log |
| Bounded expiry/suffix follow-up, frozen sourcebf56d15c | Exit0; full pre-dedup --check exit0. | task2-acquire-followup.log; task2-full-offline-first-green.log |
| Packaging/explicit-pair tests | Actual RED27pass/2fail; GREEN29/29 after response references and explicit pair table. Pre-dedup full offline/no-HTTP case also passed during RED. | task2-response-reference-red.log; task2-response-reference-green.log |
| Lossless dedup acquisition, final source5f903129 | Exit0; all existing raw cache reused. | task2-acquire-dedup.log |
| Final focused tests |55 discovered,55 passed,0 ignored,84 assertions;15.39s. | task2-full-negatives.log |
| Final bun run scripts/phase167-historical-corpus.ts --check | Exit0;7 bodies,5263 previous transactions,80,190,547-byte corpus. Full no-HTTP test replaces fetch with a throwing function. | final-offline-check.log |
| Final bun build ./scripts/phase167-historical-corpus.ts --target=bun | Exit0; one standard-library-only module. | final-build.log |
| Owned source review and git diff --check | Exit0;569/282 lines, no trailing whitespace or stub markers, both files below628. Source/data diff and integrity contracts reviewed. | final-owned-review.log; final-diff-check.log |

Tool-usage probes for unsupported --help and bare gsd-tools invocation returned usage errors; these are retained in verify-summary-help-unsupported.log and verify-summary-usage.log. They are separate from behavioral RED and source checks.

## Corrections and retained history

- The reversed BIP30 assumption was exposed by the actual complete-body check. Comparing exact serialized coinbases independently proves91842→91812 and91880→91722. One shared mapping helper now drives generation and validation. Both correct byte pairs and both reversed-pair refusals are covered.
- The independent compact encoder originally returned0x01000000 for target0 because hexadecimal text "0" has length1. For old bits0x01010000 and elapsed0, pinned pow.cpp clamps span to302400, then target1*302400/1209600 truncates to0. Pinned arith_uint256.cpp::GetCompact uses bit length0 and returns compact0. The retained RED distinguishes expected0 from actual16777216; an explicit zero case fixes the offline calculator. No raw block, cached response or serialized transaction byte changed.
- Concrete Plan03 API inspection established its unconditional expiry lookback requirement. Additional bounded headers and complete155-row suffixes were acquired after the previous producer genuinely terminated; no ancestry gap was relabeled linked.
- Blockstream rate limiting required an explicitly authorized fixed alternate origin. Every original source generation, actual error, cooldown and cache record remains attributable. Acquisition starts are paced at most2/s, with bounded attempts and Retry-After handling. No native verifier uses acquisition.

## Resource fit and exact-byte packaging

Final corpus:80,190,547 bytes against134,217,728. Downloaded payload across attempts:29,883,014 bytes against536,870,912. Maximum response4,194,304 bytes; maximum actual linked walk394 against4096. Final contexts.json is77,269,907 bytes; all six owned files are below100MiB.

The successful pre-dedup context was112,015,489 bytes, exceeding GitHub's100MiB per-file limit despite fitting the task's total corpus bound. [GitHub's file-size policy](https://docs.github.com/en/repositories/working-with-files/managing-large-files/about-large-files-on-github) motivated lossless packaging, not evidence removal. Expanding all5263 transaction references recreates that entire original file byte-for-byte, including every URL/date, payload and serialization whitespace: both SHA256 values are3c92e1ccd8273808f8c0377ff46e12ec58147ce9d5040f6c2e516f7d9c7d5261. The original successful output is retained under followup-bf56-output/; dedup-byte-reconstruction.json records exact equality.

The immutable blocks.json SHA256 remains6e81f7f06037d8c1e18b75ddef67d16aa6ab09a505ca616db612de1f29e0c335. Final context SHA256 is3067e98fb235c97226482a6c1b9fad2484fe58610f5ed1c25e8860a0a970fd5a. final-owned-stats.json records all six file sizes/digests, twelve pinned source digests and actual bounds. First retained test receipt is2026-10-21T17:28:58.491Z; duration176min is measured to this summary's finalization time.

## Decisions, simplification and scope

One shared linked-window validator checks MTP, difficulty and candidate suffixes. One byte-derived repeat mapping drives both paths. Response references eliminate35,166,624 repeated hex characters while retaining exact on-wire payload evidence and preventing recursive, missing, wrong-network or wrong-txid references. No new crate, production dependency, Python automation or file-length exception was added.

AGENTS.md, Bright Builds sidecar/overrides, architecture/code-shape/testing/verification/TypeScript standards and both active lesson inputs informed execution. Both active lesson files were loaded completely:8856 bytes/2953 conservative tokens. No project skill was present. Source pin remains a9aee730466ac67d35a3c03ee24676be5e045878; twelve Git/file digest comparisons include validation, chainparams, tx_verify, pow, arith_uint256, coins, chain, deployment, interpreter and expiry/build-year sources.

## Known stubs and threat scan

No unresolved stubs remain. Empty UTXO lists belong to genuine coinbase-only bodies. Empty flag replacements are the pinned NONE cases. Seeded creation windows and the detached BIP34 anchor are intentional evidence classes, not missing ordinary ancestry disguised as data.

T-167-02-01/02/03 are covered by independent byte/hash/merkle/witness checks, strict selected-network/schema/evidence gates, exact response reconstruction, bounds and retained command/source history. Public HTTPS data acquisition and local fixture writes are the plan's declared surfaces; no production endpoint, authentication path or durable node schema was introduced.

## Task commits and remaining phase work

commits:false. No staging, commit, push, hook bypass, worktree or shared STATE/ROADMAP/requirements/guard edits occurred. Root owns strict final saving and metadata. No Cargo/Bazel command was necessary for these Bun/data owners. Full native, fresh coverage/Bazel, independent phase reviews and actual durable historical execution remain the explicit later-plan/root obligations. Requirement IDs above describe this plan's bounded contribution, not phase-wide closure. No whole-chain, ordinary mainnet/testnet, actual-daemon, public-default, production or funds claim is made.

## Root follow-up: executable key-link metadata

The [root key-link amendment](167-02-KEY-LINK-AMENDMENT.md) records a metadata-only follow-up, independently reviewed at 2026-10-21T20:26:36Z with 0 blockers/warnings. The original scanner reported 0/1 because it expected a contiguous literal contexts path, while the source composes that path and executes `checkContexts(context, corpus.blocks)`. Root added only the plan's matching pattern; the current result is 1/1. The [later unchanged-snapshot reproduction](../../../.local/open-bitcoin-dev/phase167/plan02-key-link-amendment/reproduced-original-key-link.json) retains the reproducible 0/1 result separately from the original transcript, beside the [current result](../../../.local/open-bitcoin-dev/phase167/plan02-key-link-amendment/final-key-link.json). This was a parser/metadata result, not a source or behavioral-test failure. Source, data, acquisition and the 55 focused tests remain unchanged; the original 176min measurement and all prior receipts remain attributable. No acquisition or source tests were rerun for this follow-up.

## Self-Check: PASSED

All six owned source/data files and this summary exist. All55 focused tests, complete offline validation, standard-library build, exact response reconstruction, source digests, resource/hosting checks, scoped whitespace and owned diff review passed. The labelled checkout baseline exists. Saving and shared metadata remain root-owned.
