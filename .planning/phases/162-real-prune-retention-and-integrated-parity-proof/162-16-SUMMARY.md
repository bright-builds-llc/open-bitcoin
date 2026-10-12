---
phase: 162-real-prune-retention-and-integrated-parity-proof
plan: "16"
subsystem: wallet
tags: [rust, descriptor-storage, public-roundtrip, legacy-codec, bounded-wallet]
requires:
  - phase: 162-04
    provides: Verified configured retained-filter client lifecycle
provides:
  - Formatter-only correction for directly parseable future descriptor metadata
  - Public range/cursor/script regressions and explicit serialized legacy decoder compatibility
  - Unchanged genuine bounded public wallet scan and cold-reload preflight GREEN
affects: [162-05, 162-10, 162-12]
tech-stack:
  added: []
  patterns: [Public parse-storage-parse invariants, explicit legacy serialized input]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-wallet/src/descriptor/format.rs
    - packages/open-bitcoin-wallet/src/descriptor/tests/key_source_and_storage_helpers_cover_extended_branches.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/tests/chainstate_wallet_and_headers.rs
key-decisions:
  - Remove only the redundant formatter separator; preserve prefix, parser, defaults and node normalization.
  - Explicit legacy serialized bytes preserve existing compatibility and raw original_text without claiming migration.
  - Resume Plan05 only with unchanged bounded public-wallet assertions passing.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-11T21:14:35Z
duration: 6 min recorded execution window
completed: 2026-10-11
---

# Phase 162 Plan 16: Descriptor Storage Roundtrip Prerequisite Summary

**A one-separator formatter correction makes future ranged descriptor storage directly parseable, preserves existing legacy node decoding, and passes the unchanged genuine bounded wallet scan/cold reload.**

## Performance and Ownership

- Tasks: 2/2. Exactly three existing implementation/test paths changed; no new first-party source module or breadcrumb registration is needed.
- First recorded execution clock: 2026-10-11T21:08:28Z. Final Cargo receipt ended at 21:11:58.317Z. Final summary clock: 21:14:35Z. Initial mandatory context loading preceded the first recorded clock.
- Material guidance: AGENTS, Bright Builds sidecar and overrides, architecture/code-shape/testing/verification/Rust standards, execute-plan workflow/template, phase CONTEXT/RESEARCH, Plan05, Summaries04/14/15 and tracked DESCRIPTOR-DIAGNOSIS.
- Both active lessons were read completely: 7,188 bytes, conservative 2,397 tokens. No archive loading or new audit trigger. No project skills exist in the two checked directories.
- Parent supplied the synchronized pinned baseline and materialized Knots. Bun 1.3.9 and Rust 1.94.1 were used. Cargo jobs were exclusively owned and serial through the cooperative timing wrapper; all sessions exited normally.
- Root retains shared STATE/ROADMAP/REQUIREMENTS/todo, parity/LOC, independent reviews, full native/formal gates and all Git finalization. No staging, commit, branch, push or hook bypass occurred.

## Task 1: Public Semantic RED and Formatter GREEN

Before production edits, seven new public behavior tests were discovered: **2 passed / 5 failed**, exit 101. All five ranged cases failed while reparsing their stored text through `SingleKeyDescriptor::parse`, reporting `DescriptorSyntax` for duplicate-colon metadata. Exact failing tuples were 0/0/0, 2/5/3, 0/1000/0 and exhausted 0/0/1. The nonranged unchanged-output and strict malformed-input controls passed. This is executable semantic RED, not a compile failure or zero-discovery run.

The only production change is `format.rs:163`: `"{body}#{}:{}:{}:{}"` becomes `"{body}#{}{}:{}:{}"`. Arguments, `RANGE_METADATA_PREFIX="ob:"`, parser grammar, wildcard defaults, key/body canonicalization, cursor rules, API and consumers remain unchanged. The focused rerun passed **7/7**, 0.08-second test runtime.

The existing registered wallet test child adds seven concerns: explicit zero range, nonzero range/cursor with inclusive endpoint matching, omitted defaults for extended public/private keys, public advancement into exhausted cursor with typed refusal on the next allocation, accepted checksum/suffix forms, nonranged canonical storage, and strict malformed controls. Shared assertions prove exact body/suffix, full descriptor identity, all public range/cursor accessors, successful exact current/selected-index scripts, address equality and repeated-storage idempotence. No private fields are seeded. A pinned real checksum-bearing hardened-private fixture from `wallet_descriptor.py:193` supplements ignored suffix controls; it passed in the final full wallet run. Existing checksum acceptance/omission behavior is preserved, with no new checksum-validation claim.

Malformed controls retain public rejection of duplicate colon, missing/extra components, nonnumeric fields, negative unsigned input, each overflowing u32 field and inverted range. Exhausted cursor 0/0/1 remains a supported stored state, rather than being clamped or newly rejected.

## Task 2: Explicit Legacy Compatibility and Frozen Caller

The node codec regression explicitly replaces only encoded JSON `payload.descriptors[0].original_text` with historical double-colon text, using three independently specified canonical/legacy suffix pairs: 0/0/0, 2/5/3 and 0/1000/2. The test asserts that actual serialized input contains those bytes. The exact same raw text fails strict public parsing but succeeds through the unchanged existing `decode_wallet_snapshot` normalizer.

Decoded `original_text` must byte-equal the serialized legacy text. Full descriptor equality preserves key/origin/path/kind/signing semantics; explicit range/cursor, current and selected-index script, address and whole-snapshot equality preserve all other fields. Decoded `descriptor.storage_text()` must equal corrected canonical text and parse directly. This test passes 1/1 with all three cases. It is a compatibility guard over existing behavior, with no invented legacy RED or node production correction.

The existing dynamic wallet codec roundtrip, managed cursor persistence and actual named-wallet registry reopen each pass 1/1. Dynamic formatter-driven coverage is kept distinct from the explicit legacy serialized fixture. No whole-blob byte-equality or automatic rewriting of retained legacy `original_text` is claimed: re-encoding that record may still retain the old raw text.

The **unchanged** `phase162_wallet_funded_short_prefix_preflight` passes **1/1**, 2.23-second test runtime, 10,640-ms wrapper including 8.31-second compilation. Its frozen source uses actual public `#ob:0:0:0` import and still asserts body identity, exact stored text, all `Some(0)` metadata, original receive script, genuine accepted 102-block history with normal maturity 100, actual h20 tx0/vout1 funding 25,000 sats, named durable wallet creation, public scan, exact saved wallet/job and all-handle cold registry reload. Success cleanup asserts the datadir is absent.

Historical receipts remain distinct: parent-reported prior default-wildcard genuine scan/cold reload **93.64 seconds GREEN**; bounded body/wallet run **3.88 seconds, body 1 pass/wallet 1 fail, exit 101**, expected single-colon versus actual double-colon metadata. Neither is rewritten as this run. The short unknown-body control remains separate and cannot establish deleted h714 acceptance.

## Verification Receipts

All Cargo rows use pinned PATH and `bun run scripts/command-timings.ts run --key KEY -- cargo ...`, with `--manifest-path packages/Cargo.toml`. Tests use all-features and serial libtest threads. Node selectors use `--lib`; the frozen caller uses `-p open-bitcoin-rpc --bin open-bitcoind`.

| Timing key                                | Command or selector                                       | Actual result                                          | Wrapper ms |
| ----------------------------------------- | --------------------------------------------------------- | ------------------------------------------------------ | ---------- |
| phase162-descriptor-storage-roundtrip     | wallet lib phase162_descriptor_storage                    | RED2passed/5failed, exit101                            | 2,324      |
| phase162-descriptor-storage-roundtrip     | same focused selector after literal correction            | GREEN7passed,0failed                                   | 1,993      |
| phase162-wallet-storage-full              | wallet package --all-features                             | 55unitpassed,0failed;0doctests; no integration targets | 3,807      |
| phase162-wallet-storage-clippy            | wallet + node --all-targets --all-features -- -D warnings | Passed;9.78s Cargo                                     | 10,022     |
| phase162-explicit-legacy-descriptor-codec | node phase162_descriptor_legacy_codec                     | 1passed, three legacy tuples;0.04s test                | 10,419     |
| phase162-existing-wallet-codec-roundtrip  | wallet_snapshot_round_trips_through_original_descriptors  | 1passed                                                | 351        |
| phase162-existing-wallet-cursor-persist   | managed_wallet_persists_ranged_descriptor_cursor_updates  | 1passed                                                | 141        |
| phase162-existing-named-wallet-reopen     | named_wallet_registry_and_selection_survive_reopen        | 1passed;0.66s test                                     | 809        |
| phase162-wallet-bounded-after-storage-fix | frozen phase162_wallet_funded_short_prefix_preflight      | 1passed;2.23s test                                     | 10,640     |

Exact semantic RED receipt: `.local/open-bitcoin-dev/command-timings/phase162-descriptor-storage-roundtrip/2026-10-11T21-09-25.683Z-9ee6c0d7-ab92-407e-a621-36f74b20995b.json`.

Exact focused GREEN receipt: `.local/open-bitcoin-dev/command-timings/phase162-descriptor-storage-roundtrip/2026-10-11T21-09-44.560Z-4cdbd042-121c-4ccc-9c5d-6c12425fc37d.json`.

Exact frozen caller GREEN receipt: `.local/open-bitcoin-dev/command-timings/phase162-wallet-bounded-after-storage-fix/2026-10-11T21-11-47.677Z-d048d79e-07ef-4cee-a056-16784d46cf77.json`.

Additional checks passed: scoped `rustfmt --edition 2024 --config skip_children=true --check` on all three owned paths; parity breadcrumbs for1,134Rustfiles; managed all checks with0findings; `git diff --check`. Owned final file lengths are585formatter,609wallet tests and258node tests, all below628. Final full wallet suite includes the final checksum fixture. No unrelated suite was rerun and no full584-row pressure corpus was run by this prerequisite.

## Simplification, Scope and Cleanup Review

The explicit simplification pass retained the one-character root-cause correction and one small shared public-roundtrip assertion helper. Separate behavior tests keep failures specific; the legacy table avoids duplicating three codec workflows. No extraction, visibility widening, generic serializer, parser workaround, new dependency or suppression is necessary.

Owned diff review confirms one formatter replacement,196test-only added wallet lines and89test-only added node lines. Existing tests and breadcrumb headers remain intact. No prefix/default/parser branch, production wallet/node consumer, decoder, recipe, oracle, Plan05 source/assertion, schema or dependency changed.

Final SHA256 observations:

| Source                                                                            | SHA256                                                           |
| --------------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| wallet descriptor/format.rs                                                       | 03260f29ce116c49c43b0d04869bb4205b4dc73783bd579a8edb7f89e04ad417 |
| wallet descriptor/tests/key_source_and_storage_helpers_cover_extended_branches.rs | 371f37b62660fa770747ae9ec33edddea43df66a6b745f2498e9456fd57c9207 |
| node storage/snapshot_codec/tests/chainstate_wallet_and_headers.rs                | 65a3fddef450930be7e0ecd96f8fc3efb940d2eb195d542a2ea633152633e0db |

The frozen Plan05 restrictions, factory, daemon test root, lifecycle test and shared recipe hashes match the pre-run inventory; descriptor.rs and node production codec hashes also match. Final temporary-directory inventory under actual TMPDIR and/tmp finds no `open-bitcoind-sync-preflight-phase162-daemon-*` datadir. The unchanged fixture RAII closes owners before removal, and the passing preflight explicitly proves handle-free reopen before cleanup. No owned failed fixture remains. Cargo is idle.

## Declared Mitigations and Limits

| Threat     | Scoped evidence                                                                                                          |
| ---------- | ------------------------------------------------------------------------------------------------------------------------ |
| T-162-16-1 | Formatter-only correction with nonzero public semantic RED/GREEN and exact tuple/body/script/idempotence tests.          |
| T-162-16-2 | Strict public malformed negatives and actual explicit legacy codec acceptance with raw text retained; decoder unchanged. |
| T-162-16-3 | Frozen bounded caller genuine102-block/h20funding/publicscan/coldreload GREEN; separate historical receipts.             |
| T-162-16-4 | Raw hermetic keys stay in tests; summary records only safe categories, tuples/counts/timings.                            |

No blocking stub was found in the three owned paths. Empty/optional values already present in test helpers are normal test cases, not unwired UI/data. No new production endpoint, authentication path, file-access pattern, schema or dependency introduces a new threat surface. Independent source/declared-mitigation review and unresolved-High disposition remain root gates; no ASVS or whole-wallet Knots parity certification is claimed.

## Deviations and Task Commits

None in implementation scope. All RED/GREEN/task/metadata commits are explicitly deferred by D-14 and parent ownership, overriding ordinary per-task commit/state procedures. No authentication gate occurred. No out-of-scope test failure or additional file ownership was needed.

## Next Plan Handoff

Plan05 can resume with its descriptor blocker removed and every frozen acceptance intact. Its complete first public scan before `protected_request`, post-reorg8..1002public refresh, real manual/automatic pressure pruning, retainedh715positive/deletedh714in-window NotFound, final715..1002rescan refusal due to missingh20creating body, unchanged wallet/job/checkpoint and all prior filter clients remain mandatory.

Root still owns independent reviews, full default `bash scripts/verify.sh`, formal lifecycle/current-document gates, requirement activation, shared registry/LOC updates and final Git. Requirements remain pending. This future-write correction adds no historical-data repair/import/migration/backfill or automatic raw-text rewrite. BIP37/V0, assumeutxo/dual-chainstate, archive scale, public defaults/public-network CI, production service/readiness/funds and milestone shipment/audit remain excluded; Phase161 presentation/redaction and previous crash-window advisories are unchanged.

## Self-Check: PASSED

All three owned source paths and this summary exist. Recorded hashes match final sources; the seven frozen consumer/recipe/prefix/decoder hashes match the initial inventory. Every required new/existing selector has nonzero passing discovery; RED and GREEN receipts have actual exit101/0 with the recorded timings. Final formatting/parity/managed/diff checks pass. Baseline a77edab276a9daa086055aa61e6143ce86bc1218 exists and is unchanged. Exactly two top frontmatter delimiters, same lifecycle identity, fresh clock and empty requirements-completed are preserved. No task commit is falsely claimed; root-owned shared files and Git were untouched. Scoped GFM/frontmatter-aware Markdown verification and final file/receipt checks complete this artifact. Temporary fixtures are absent and all Cargo sessions exited.
