---
generated_by: gsd-debugger
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-12T02:50:32Z
status: self_verified
---

# Tail Preflight Ordinary Prune Header Diagnosis and Repair

## Plan and Ownership

- [x] Reproduce exact daemon test through pinned timing wrapper with exclusive Cargo.
- [x] Trace actual fixture ancestry, live header seed, lazy snapshot and strict cold reader; discriminate fixture omission from publication/runtime invariant failure.
- [x] Inventory and verify ownership/liveness of any failed test store before cleanup.
- [x] Present minimal checked repair and bounded verification plan before source edits.

Initial ownership is only this record and an ignored debug session. No runtime/test/registry/current-claim/source6 change is authorized. Strict ancestor validation, real hash identities, genesis/coins authority, paired retention, reopen, wallet and error assertions must not be weakened. Parent owns further plan acceptance, evidence/review registration, remaining-workspace/tail/native/formal/Git closure; all requirements remain Pending.

## Initial Actual Failure

`/tmp/phase162-native-tail-preflight.log` records Node1,695pass/0fail/4ignored in694.38s and RPC lib375pass/0fail/1ignored in63.68s. Daemon102pass/1fail/1ignored in296.54s; complete phase162_integrated_lifecycle passed. Sole existing ordinary retention test fails at automatic_prune.rs285with missing null header ancestor on successful-checkpoint reopen. Its actual pressure evidence is target576716800/logical578359864/nonactive578358970/235blocks/body2461034/undo68. These are the existing codec-valid sparse-retention fixture, not the separate validated continuous Phase162 history.

## Exact Reproduction and Cleanup

With pinned PATH, `bun run scripts/command-timings.ts run --key phase162-native-prune-red -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind --all-features tests::automatic_prune::automatic_prune_genuine_ordinary_retention_and_reopen -- --exact --nocapture` reproduces0pass/1fail/103filtered in11.98s. Wrapper12577ms,exit101; session44401finished. Log `/tmp/phase162-native-prune-red.log`; receipt `.local/open-bitcoin-dev/command-timings/phase162-native-prune-red/2026-10-12T02-32-23.965Z-6329f0a0-158e-43b2-9264-928bf73445b4.json`. Pressure counts/bytes and missing null ancestor exactly match the original failed tail.

The fixture owns its uniquely reserved `open-bitcoind-sync-preflight-genuine-retention-PID-TIMESTAMP.SUFFIX` directory using TempDir Drop. Inventory of actual TMPDIR and/tmp observed only the focused active PID39652directory, born02:32:24Z; it was never manually deleted. After test panic/unwind the same inventory finds0genuine-retentiondirectories. The previous failed store is also absent. No stale large store needs manual cleanup, and no unrelated path was deleted.

## Confirmed Cause

Original seed creates only positions at1/500/510/713/714/1001. Each header uses the default null previous hash and every position is nonzero-height. `seed_local_chain → HeaderStore::seed_from_chain → record_position` records those legacy fixture facts without checking ancestor closure. Plan14's lazy checkpoint clones all live entries and serializes them without dropping or substituting an ancestor. The existing codec preserves entries and canonical header identities. Strict cold `HeaderStore::from_entries` treats every nonzero-height entry as having a required parent and correctly refuses the absent null hash.

Thus the failure is unsupported disconnected fixture ancestry exposed by newly exercised durable header publication. It is not evidence of a lost valid ancestor, a new consensus failure or a reason to tolerate corrupt saved headers. The old six sparse bodies/coins remain honest codec-volume/ordinary-retention evidence; they must not be promoted to continuous consensus-accepted chain evidence.

Existing strict-reader controls ran through timing key `phase162-native-prune-strict-reader`: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib --all-features header_store_rebuild -- --test-threads=1 --nocapture` passes both valid reconstruction and missing-parent refusal,2pass/0fail/603filtered in0.01s. Log `/tmp/phase162-native-prune-strict-reader.log`. These controls remain unchanged.

## Concrete Fixture-Only Proposal

Parent accepted this plan after actual independent planchecker PASS and explicitly authorized only existing `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/fixtures.rs`, originally385lines:

1. In existing seed, build a finite1002-header parent-linked codec ancestry through public HeaderStore insertion, using each actual computed parent/hash/height/work. Null parent belongs only to the actual serialized height-zero test root; never invent an all-zero root identity or claim pinned network-genesis/ScriptsValid acceptance.
1. Select the same six sparse heights from that coherent header ancestry for the existing payload/coins truth. Keep the original existing seed_coins_from_snapshot call exactly once, original UTXO/wallet construction, six bodies/undo and pressure recipe; introduce no extra coins/genesis/validation authority seed.
1. Save the complete coherent header entries using the existing public persistence path before production owner open; require strict roundtrip reconstruction. Raw fixture persistence retains the existing UnknownLegacy/unvalidated boundary and does not grant ScriptsValid.
1. Add fast regression coverage in this same existing fixture file for complete saved ancestry/canonical identities/root and independent missing-parent refusal. Run it before the12-second pressure test, then all existing automatic_prune cases. Preserve original automatic_prune.rs target, ordered deletion, accounting, wallet, body-serving, injected-error and all-handle reopen assertions byte-for-byte.

No production reader/writer/validation/API/transport/recipe/source6/TS/current-claim change is needed. Complete continuous Phase162 integrated evidence already passed the tail and need not be redundantly rerun for this isolated codec-fixture change. Parent owns registration, independent source/security review, continuation of remaining workspace/tail and actual Native5/formal/Git closure. All requirements remain Pending.

## Pre-Repair Hashes

- Main test, to remain unchanged: `87ee951bac32baaf010756ee603af702e24ae3b574358d95c02c551b20290ded`.
- Existing fixture: `7b005790d52caf70737887c447574435a497385b61fe8691d68aaf9d5bf0fe39`.
- Unchanged lazy caller: `8deb3e561d9948a4a00c35032e7b9f9e6078ac9eddcca2747c761d564782b814`.
- Unchanged trusted writer: `23b7cff93e0d0c1f38a73cdcb8211c8aa0e2c45b102c022f6d5fb44ae23ac471`.

Those hashes and the diagnosis handoff preceded the explicitly accepted repair below.

## Authorized Fixture Repair

- [x] Existing seed constructs1002continuous codec headers with public HeaderStore insertion and real computed parent/hash/height/work; null parent appears only on the encoded height-zero test root.
- [x] Original six sparse payload/coins positions remain1/500/510/713/714/1001; original one coins-seed call, UTXO/wallet/payload/pressure flow is preserved.
- [x] Complete header entries are saved through the existing public persistence path and strict construction succeeds before runtime open.
- [x] Fast saved-ancestry/canonical-identity/root/parent-closure/typedmissingparent regression passes; original pressure test and allautomatic cases pass.
- [x] Actual fullfmt, strictRPCClippy, shared/live/parity/managed/scopedformat/diff/source-preservation/cleanup checks pass; Cargoidle handed back immediately.

New regression `automatic_prune_fixture_saved_headers_have_complete_ancestry` observes the public strict saved-header loader and roundtrip reconstruction. It verifies1002headers, canonical nonzero actual root hash/null root parent, canonical hash for every header, all1001nonnull parent edges and sequential heights, exact six sparse positions matching saved entries, original unconfirmed snapshot marker, and typed MissingHeaderAncestor after removing the actual height500parent. It does not manufacture a pinned Regtest genesis, scripts-valid history or new authority.

The first fast compile failed before test execution because the RPC fixture attempted to use node-private provenance APIs and omitted a borrowed header lookup argument. The corrective edit remained in the owned fixture: preserve the node privacy boundary, remove private API access, assert the existing public unconfirmed metadata marker, and borrow the lookup hash. No visibility/runtime/dependency expansion occurred. UnknownLegacy preservation is supported by unchanged production source: the existing raw coins seed and public save_header_entries invalidate coverage and cannot mint accepted validation batches. It is not presented as an executable private-provenance assertion. The fast regression then passed on retry; no second same-step failure occurred.

The shared position helper stays unchanged for index-protection and nonactive pressure fixtures. Main automatic_prune.rs, pressure/calibration/body/undo/wallet helpers, metadata-fault adapter and all original target/deletion/accounting/body-serving/wallet/error/reopen assertions remain byte-for-byte identical. Existing injected later metadata error and retry are exercised fully after coherent checkpoint reopen.

## Actual Measured Results

Every Cargo command used pinned PATH, the repo manifest and exclusive timing wrapper. Final format command is `cargo fmt --manifest-path packages/Cargo.toml --all`; strict Clippy is `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features -- -D warnings`.

| Timing key                          | Result                                                                   | Libtest/tool time         | Wrapper                     |
| ----------------------------------- | ------------------------------------------------------------------------ | ------------------------- | --------------------------- |
| phase162-native-prune-red           | Original0pass/1fail/103filtered,exit101                                  | 11.98s                    | 12,577ms                    |
| phase162-native-prune-strict-reader | Existing2pass/0fail/603filtered                                          | 0.01s                     | 305ms                       |
| phase162-native-prune-format        | Three scoped-workflow fullfmt runs,all exit0; final used for frozen hash | No separate Cargo summary | 1,414ms / 1,580ms / 1,400ms |
| phase162-native-prune-fast          | First compile failure,exit101; no test executed                          | No libtest result         | 3,314ms                     |
| phase162-native-prune-fast          | Corrected1pass/0fail/104filtered,exit0                                   | 1.09s                     | 5,484ms                     |
| phase162-native-prune-green         | Original focused1pass/0fail/104filtered,exit0                            | 17.36s                    | 17,707ms                    |
| phase162-native-prune-all           | All7automatic cases pass/0fail/98filtered,exit0                          | 148.12s                   | 148,353ms                   |
| phase162-native-prune-clippy        | Strict RPC all-target/all-feature,exit0                                  | 2.43s                     | 2,572ms                     |

All-case command: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind --all-features automatic_prune -- --test-threads=1 --nocapture`. Fast command uses exact `tests::automatic_prune::fixtures::automatic_prune_fixture_saved_headers_have_complete_ancestry`; focused command retains the original exact selector above. Logs use `/tmp/<timing-key>.log`; timing receipt JSONs live under `.local/open-bitcoin-dev/command-timings/<key>/`, retaining both failed and passing fast attempts and all formatter runs.

Focused GREEN preserves original pressure target576716800/logical578359864/nonactive578358970/235blocks/body2461034/undo68 exactly. It now completes every unchanged assertion with initial logical-after578359417, four total deletions and two successful batches across later-error/retry/reopen. The all-case repeat likewise passes; its separate existing dense index-protection case earns714paired deletions,151300deleted bytes and logical578571326→578420026 with the same235nonactive pressure bodies. These are distinct codec-retention scenarios and are not the continuous validated Phase162 scenario.

Serial session64352finished0; at02:47:43Z parent received immediate Cargoidlehandoff to resume remaining workspace/tail. No subsequent child Cargo operation occurred. No quiet heartbeat was treated as timeout; advancing timing evidence was reported while index protection ran.

## Static and Preservation Evidence

Final `/tmp/phase162-native-prune-static.log`: shared authority127suite18pass/0fail,19assertions,11.90s; live127and154–162pass; parity breadcrumbs verify1141Rustfiles; managed all0findings. Scoped Rust formatting and `git diff --check` pass. No TS source/test was modified.

Final `/tmp/phase162-native-prune-invariants.log`: actual1464-file current first-party source roster is unchanged and only this fixture differs. Every other1463source file, including main automatic_prune.rs, runtime, original Plan15 guards, previously repaired tests, TS and source6, is byte-for-byte unchanged. Existing position/small-block/save/cache/pressure/wallet/fault helpers are individually identical; the entire metadata-fault adapter suffix is identical; there remains exactly one existing coins-seed call. Final source6digest remains `a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575`.

Actual TMPDIR and/tmp contain0owned genuine-retention, retention-header-ancestry or index-protection-legal-target fixture directories after all sessions. Destructor cleanup sufficed on failed and passing runs; no active or unrelated path was manually removed.

Final fixture SHA256: `209b83454e17dfdbc3b63134ed2bc8262b021f8c70b8278c2d3f5fcd754e88dd`. Main test remains `87ee951bac32baaf010756ee603af702e24ae3b574358d95c02c551b20290ded`. Final fixture466lines; diff86insertions/5deletions. Simplification keeps construction and regression in one existing fixture, using public HeaderStore/persistence rather than a production workaround, new module or private API. No new dependency or Rust file/breadcrumb is introduced.

## Parent Closure Remains Separate

Parent received the final hash for70-file independent source/security review, actual counts/timings, source/runtime preservation and idle handoff. Parent owns registry/scope/LOC review refresh, remaining-workspace/doc/bench/Bazel/coverage tail, actual full default Native5, formal closure/activation and normal-hook Git finalization. All three requirements remain Pending and current owner in_progress. No full-native/formal completion, requirement activation, source6/runtime/TS/current-claim change, stage/commit/push/branch/hook bypass or debug archival is claimed here. All child sessions are idle; no further source/Cargo edits are planned.
