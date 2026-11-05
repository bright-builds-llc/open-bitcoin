---
phase: 167-candidate-history-validation-rules
verified: "2026-11-05T06:02:35Z"
status: passed
score: 152/152 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-11-05T06:02:35Z"
lifecycle_validated: true
overrides_applied: 0
requirements_verified: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
source_files_verified: 318
source_sha256: da888c8929643e35d5cd770c9dfa0beeabcd5d96d4cc843a4e6f120e9c39cbc6
source_review_sha256: 01f852c06db6d99fbebd39c69bb477bdd25d344c7fee7f69fca1cbefa8606537
combined_tree_product_sha256: 82a2e42c455820c7e834755aa0c956284bc8b29a007f272b6c8fb3b3a08c1380
security_review_sha256: 56217330494cf3a4053ab410b39af520dc7983cfd0731825ed6cd0201f9c2f88
default_verifier_v15_log_sha256: 74a5226365e916707b0e723f74b586d754cb25025775ed0b1dcee7201afdb339
---

# Phase 167: Candidate-History Validation Rules Verification

**Goal:** Contributors observe pinned network- and candidate-history-derived validation decisions through ordinary header admission and durable body connection.

**Verdict:** Passed for the bounded local default-Regtest canonical route and exact independently seeded historical cases. All five independently mandatory roadmap criteria and 147 additional PLAN truth entries are verified. This is the initial canonical verification; no prior `167-VERIFICATION.md` existed. No human-only test is required to decide this bounded offline goal. Actual pinned-Knots daemon differential qualification belongs to Phase 168.

## Verification basis

I checked the roadmap criteria independently of PLAN summaries, then traced selected rules from the immutable network specification through candidate ancestry, header admission, chainstate apply, durable provenance, process observation and retained reopen. The fifty-one PLAN frontmatters declare 147 truths, 168 artifact declarations and 132 key links. All fifty-one SUMMARY files exist. The requirement union is exactly CVLD-01/02/03/04/06, matching the roadmap and REQUIREMENTS; there is no orphaned Phase 167 requirement. No verification override was used.

The [canonical source review](167-REVIEW.md) is clean: zero critical, warning or information findings across 318 code/config files, including the late one-line `.bazelrc` host-tool correction. I independently rehashed its current 318-file code/config scope using sorted `path`, tab, file SHA-256, newline rows; the result is `da888c8929643e35d5cd770c9dfa0beeabcd5d96d4cc843a4e6f120e9c39cbc6`, exactly the review's digest. The [security review](167-SECURITY.md) closes all 187 distinct declared threat concepts: 181 mitigated and six explicitly accepted; zero open or unregistered. Both final reports were rehashed to the frontmatter digests above. The accepted C v2 1,833-row frame `b98af995b4ce42116a7d732f948bf89eabc31e22b1351b3cf2e15057cc284901` remains the earlier product basis; the reviewed `.bazelrc` correction and ROOT status-document/LOC changes are tracked separately rather than falsely included in that frozen frame.

The [native validation](167-VALIDATION.md) binds two complete default `bash scripts/verify.sh` passes. The earlier v14 receipt exited zero on its frozen configuration; it cannot alone qualify the later `.bazelrc` correction. The revised-config [v15 raw log](../../../.local/open-bitcoin-dev/phase167/plan20/revised-default-v15.log) is 2,502,984 bytes; I independently rehashed it to `74a5226365e916707b0e723f74b586d754cb25025775ed0b1dcee7201afdb339`. The recorded command exited zero and the log ends `verify.sh completed in 2h 34m 54.863s`. It includes the Phase 164 1,255/1,255 and Phase 165 2,170/2,170 guards, nine original 16-thread stress selectors, complete scoped native tests, benchmark, six Bazel smoke targets/provenance, and fresh original eight-package pure-core coverage; it has zero `FAILED` or `Uncovered Lines:` headings. The 318-file live code/config digest still matches independent review after v15. My additional read-only `bun run scripts/check-phase167-candidate-history.ts` run exited zero. Earlier v10–v13 and two failed saving hooks retain their failed diagnostic status; the fresh direct six-target Bazel/provenance receipts do not substitute for v15.

After v14, ROOT fast-forwarded `origin/main` to `34731eca`, changing only the Bright Builds managed sidecar, audit and updater script. The inspected diff does not touch Phase 167 behavioral owners or the reviewed 318-file scope. I recalculated the frozen 1,828-path product digest `715bd1ef82cce36afd29b177cdb604b8db796bcdccb0f35bd78ca5d55fe03fd5` and the same scoped frame with the upstream updater row `82a2e42c455820c7e834755aa0c956284bc8b29a007f272b6c8fb3b3a08c1380`. Those frame digests exclude the later `.bazelrc`, current status documents and staged-index LOC; the 318-file digest and successful v15 bind the revised configuration. The normal saving hook must still validate final staged bytes and index-generated LOC before commit.

## Mandatory roadmap truths and requirements

| Requirement | Result | Actual behavior and evidence |
| --- | --- | --- |
| CVLD-01 | VERIFIED | `chain_spec/rules.rs` supplies selected maturity 100 and halving interval 150. The serialized default-Regtest corpus is connected from proved genesis through 301 descendants in `sync/tests/phase167_ordinary.rs`; it checks real coinbase spend refusal at 100 and acceptance at 101, and reward/fee boundaries 149/150 and 299/300. The same owner reopens after all handles drop at 150 and 301, then connects 302. |
| CVLD-02 | VERIFIED | `CandidateHistory::check` requires the selected candidate branch and windows; `CandidateRules::derive` computes BIP34 height, BIP65/BIP66 version floor, BIP113 finality, BIP68 sequence context and Segwit commitment rules. Chainstate `selected_candidate_context` rederives and compares held rules before stage/apply. Ordinary corpus, creating-MTP and branch-MTP tests exercise real positive and refusal outcomes. Caller switches or active-tip substitutes do not authorize context. |
| CVLD-03 | VERIFIED within evidence classes | `CandidateRules::derive` applies baseline P2SH/WITNESS/TAPROOT, exact hash replacements, then deployment additions; `engine/apply.rs` feeds selected `script_flags()` to real transaction script validation. Ordinary default-Regtest blocks record exact-hash `ScriptsValid` provenance. Three exact Mainnet/Testnet3 exception bodies execute through the managed chainstate kernel in `sync/tests/phase167_historical/scripts.rs`, but use **independently seeded mechanical contexts**, have no earned work/canonical chain/BASIC and never gain `ScriptsValid` product provenance. They prove bounded exception execution, not an earned Mainnet/Testnet3 ordinary durable product route. |
| CVLD-04 | VERIFIED | `CandidateHistory::check_required_windows`, `candidate_history/header_rules.rs`, selected `block/difficulty.rs`, and network `header_store/canonical_history.rs` use branch MTP, retarget/minimum-difficulty anchors and recovery. Body connection uses the held selected rules. Missing or mismatched history refuses; ordinary and historical-context tests cover supported boundaries and unchanged accepted state. |
| CVLD-06 | VERIFIED | `CandidateRules::bip30_disposition` permits only the exact two Mainnet repeat pairs or selected BIP34 ancestor disposition, with the pinned recheck boundary. `engine/overwrite_undo.rs` captures the complete prior Coin with exact block/height/outpoint/occurrence integrity and checks the actual parent Coin before replacement. Real repeat tests disconnect, physically reopen and reconnect, while changed prior Coin and storage-fault controls refuse without publication. Complete earlier-Coin restoration is the documented intentional difference from Knots normal disconnect. The Mainnet repeats are seeded historical evidence, not ordinary Mainnet product authority. |

The five REQUIREMENTS rows are marked Complete for these exact bounds. The current parity catalog, support matrix, PROJECT, README, ROADMAP and STATE describe the same scope: local default-Regtest 301-descendant private route, complete retained-history reopen, seeded historical nonauthority, unavailable canonical-history wallet and default-off networking. Phase 168 is unchecked and unstarted.

## D-01–D-10 decision trace

| Decision | Result | Goal-backward evidence |
| --- | --- | --- |
| D-01 | VERIFIED | Four immutable selectors in `chain_spec/rules.rs`; `network/canonical_admission.rs` binds proved genesis, network magic and earned owner. Only default-Regtest ordinary descendants are admitted; Mainnet, testnet3 and default Signet ordinary routes refuse. |
| D-02 | VERIFIED | Selected subsidy/maturity tables and ordinary serialized 100/101,149/150,299/300 checks; proved genesis starts with zero spendable UTXOs. |
| D-03 | VERIFIED | `CandidateHistory::check`, `CandidateRules::derive`, selected-chainstate rederivation and ordinary branch-MTP/version/sequence/witness controls. |
| D-04 | VERIFIED | Exact replacement-before-addition script flags reach `engine/apply.rs` and interpreter, with ordinary script-sensitive spends, seeded exact exception bodies and 80,000/80,001 sigop controls. Policy-only flags remain separate. |
| D-05 | VERIFIED | Selected header/body difficulty and MTP checks share required linked ancestry; missing retarget/recovery anchors refuse. |
| D-06 | VERIFIED | Exact BIP30 repeat/ancestor/recheck classification, complete parent Coin capture and restoration across disconnect and physical reopen. |
| D-07 | VERIFIED | `sync/tests/phase167_support.rs` drives actual header message admission, stored body and `network.connect_local_block`; `phase167_ordinary.rs` and RPC/CLI process tests prove local 301, BASIC and complete retained reopen. Seeded contexts cannot mint earned authority. |
| D-08 | VERIFIED | Ordinary negative inputs check unchanged tip/coins/BASIC/provenance; `storage/canonical_history/proof.rs` replays exact body, work, status and undo instead of trusting a persisted `ScriptsValid` label. |
| D-09 | VERIFIED | Bounded serialized corpus, exact source hashes, current-vs-historical source guards, parity breadcrumbs and classified historical results are checked by the registered Phase 167 offline suite. No local fixture is labeled as a pinned daemon differential. |
| D-10 | VERIFIED for phase handoff | The revised-config complete default v15 passes native, fresh pure-core coverage and Bazel/provenance; clean 318-file source review, closed security review and this goal/lifecycle check exist. The parity page gives repo-local Cargo/Bazel status commands. ROOT owns staged-index LOC reconciliation, normal hook, commit and push after this report; this report does not preclaim those future actions. Stop before Phase 168. |

## Artifacts, links and data flow

The read-only PLAN helper reports 167/168 declared artifacts present/substantive after source review and before this report, and 129/132 declared key links matched after this report. The remaining artifact text pattern and three link text patterns are stale metadata names/locations, not absent behavior:

| PLAN metadata | Actual verified path |
| --- | --- |
| 13 `best_chain_entries` in RPC context/dispatch | `context/canonical_history.rs` clones the same earned command Arc and returns detached accepted facts; `dispatch/node.rs:62–74` then calls `context.best_header_height()`, which reads `network.best_chain_entries()` in `context/network.rs:221–225`. The real ahead-header RPC test checks separate blocks/headers. |
| 15 `regtest-history.json` in `phase167_ordinary.rs` | Registered `phase167_support.rs:53–57` includes the exact frozen JSON; `phase167_ordinary.rs:50` consumes that helper and passes each parsed body through actual admission/connect/reopen. |
| 34 `consume_tracked_request_messages` / `release_departed_peer_inflight` | Production `session.rs:184,290` calls `PeerRequests::expire/handoff` in `block_reconcile/peer_requests.rs:242–275`; `handoff_core` checks peer membership and global capacity before outgoing packets, and `block_reconcile.rs:47` releases only hashes with no actual requester. The old named handoff helper is test-only; actual runtime tests pass. |

All other 167 artifact declarations and 129 literal links pass the helper. The 147 PLAN truths were checked against the source/review/receipt and grouped by their executable owner: Plans 01–16 and 21–33 establish selected consensus, body, header, BIP30, runtime and negative controls; Plans 34–51 close compact request provenance, packet lifetime, bounded inspection and stored success; Plans 17–19/25 provide current guards, verifier registration, parity and status documentation; Plan 20 closes the native and review gates. The helpers' lexical exceptions above were resolved by following production calls and tests, without relaxing any observable truth.

There is no UI artifact. Data flow is a real accepted header/body/coin/work/BASIC tuple from the selected owner to durable rows and the local RPC/CLI observer. `storage/canonical_history/proof.rs` compares re-executed complete history to the saved tuple on reopening; it does not project hardcoded fixture state or accept a status string alone. The RPC 301-descendant process test stops/reaps the daemon, reopens the same physical datadir, compares BASIC and accepted facts, and accepts the next body. Public network connections remain zero and wallet calls refuse.

## Anti-patterns, accepted risks and deferred qualification

The independent source review found no actionable source issue, unsafe block, unfinished `todo!`/`unimplemented!` path or production `unwrap()` across 318 reviewed code/config files. The repo-native panic-site, breadcrumb, current-claim, source and Bright Builds checks passed in v15. No blocker, orphaned artifact or hollow dynamic data path remains within the bounded phase goal. The six already declared accepted risks are: Plan 40 asynchronous scheduling latency; Plan 41 individual filesystem-call latency; Plan 42 eventual native-quiet latency; Plan 44 bounded owned synthetic fixture data; Plan 45 owned offline model fixtures; and Plan 46 synthetic scratch/genesis fixtures. They do not grant external-network or product authority.

Phase 168 owns actual identity-verified pinned-Knots daemon differential qualification. Phases 169–171 own broader fork/recovery, canonical prune/BASIC and classified historical parity closure. No Phase 168 execution or arbitrary whole-chain/Mainnet product qualification is credited here. The remaining staged-index LOC and normal-hook Git save is a ROOT-owned publication gate, not a missing Phase 167 behavior.

## Lifecycle and disposition

`167-CONTEXT.md`, all 51 PLANs, all 51 SUMMARIES, the native/security records and this report use the originating yolo lifecycle `167-2026-10-21T16-38-23`; no upstream artifact is a direct-fallback substitute. The canonical source review names the same phase lifecycle ID. Lifecycle validation is true for this report and subject to the post-write GSD lifecycle command. No gaps, overrides or human-only acceptance items are recorded.

_Verified: 2026-11-05T06:02:35Z_
_Verifier: gsd-verifier_
