---
phase: 165-shared-exact-work-authority
plan: "20"
subsystem: peer-fixture-source-authority
tags: [basic-filters, genuine-fixtures, reorg-preference, source-guards]
requires:
  - phase: 165-19
    provides: Strict fixture-source and accepted-header contracts
  - phase: 165-26
    provides: Completed current and historical fixture compatibility
  - phase: 165-27
    provides: Actual current reorg contracts and complete freeze collector
provides:
  - Complete pure validation of the original large-filter prefix and 40,000-output block
  - Raw serving reopens without coins-work replay or live header-owner rebinding
  - Actual early-body equal-height status and stronger readiness premises
  - Reviewed current-source identities and complete stable matrix evidence
affects: [165-21, 165-22, 165-23, 165-24, 165-14]
tech-stack:
  added: []
  patterns: [Pure fixture preparation, Header-only raw observation, Same-network body order]
key-files:
  created:
    - packages/open-bitcoin-node/src/sync/tests/filter_index/peer_fixtures/phase165_source.rs
  modified:
    - packages/open-bitcoin-node/src/sync/tests/filter_index/peer_fixtures.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/peer_evidence.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/operator_status.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests/failures.rs
    - docs/parity/source-breadcrumbs.json
    - scripts/check-phase165-shared-exact-work.ts
    - scripts/check-phase165-shared-exact-work/contracts.ts
    - scripts/check-phase165-shared-exact-work.test.ts
key-decisions:
  - Validate every actual prefix body with the complete original maturity 1 parameters before large-filter derivation.
  - Keep serving reopens unearned and observe existing genuine headers after actual reorgs.
  - Forward the exact early-branch declaration to the existing checked same-network lifecycle.
  - Correct the weaker readiness premise without changing preference policy or original assertions.
  - Renew only six reviewed current identities and three actual executable test-owner rows.
requirements-completed: []
requirements-contributed: [CWOR-02, CWOR-03, CWOR-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: "2026-10-17T10:20:02Z"
client_date: "2026-10-17"
duration: 53min
completed: 2026-10-17
tasks_completed: 3
files_modified: 10
---

# Phase 165 Plan 20: Genuine Peer Sources and Actual Preferred Forks Summary

**Shared peer fixtures now validate the complete large-filter source, preserve raw serving work origin, and use actual accepted body order for the equal-height status case; the eight scoped Rust corpora and both complete current guard matrices pass.**

## Scope and Completion

All three Plan20 tasks completed against plan `fdd1c28a5bd32efb711911638ec28c426f61872b70daee504a83df774dfe9957`. Actual command evidence began at 2026-10-17T09:26:31.367Z; complete verification closed at 2026-10-17T10:19:24.659Z. The evidence window is 52 min 53 s. UTC and America/Chicago client dates are 2026-10-17.

Ten implementation owners changed within the thirteen declared paths. The three declared old 164 tooling files remain byte-for-byte unchanged because none of the six Rust owners was already in that old 164 source inventory. The new Rust helper received its parity block and `phase165-peer-genuine-source` JSON mapping immediately, anchored to Knots validation.cpp and node/blockstorage.cpp.

Root owns shared planning state, subsequent plans, final native verification and Git. This executor performed no staging, commit, push, branch, worktree, hook, shared-state or native-verifier changes. The completed predecessor26/27 compatibility work remains separate.

## Actual Source and Preference Changes

`phase165_source::genuine_history` starts with `Chainstate::new()` and consumes every available recipe block using the supplied complete ConsensusParams and ScriptVerifyFlags. The large fixture retains its original maturity 1 parameters and P2SH flags. Its legal 40,000 distinct output scripts, actual block bytes, mining, staged validation, commit, accepted position, undo, filter generation and final snapshot derivation remain ordered as before. No raw snapshot promotes the source and no counts or missing history are reconstructed.

The original shared file was 623 lines; an intermediate formatted implementation reached 634. The approved cohesive extraction moves only large source preparation and expected-record derivation into the already-declared helper. `PeerFilterFixture::large()` delegates to `Self::from_history(phase165_source::large_history())`. The saved pre-extraction body reconstructs the new body modulo whitespace, function signature and final return. All ten original expectation boundaries remain in the same order. Serving, network, reopen, pruning and measured test bodies stayed in their owners.

`open()` and its serving reopens use header-only admission, with unearned-work observations before and after. `sparse()` likewise preserves raw origin around its existing direct header admission. The normal Regtest maturity 100 serving runtime no longer attempts to replay the maturity 1 fixture spends. Actual durable ScriptsValid rows remain historical store facts. Peer capabilities, handshakes, corruption/deletion controls, resource charges and independent wire oracles are preserved.

Three redundant admissions after genuine reorgs were removed: the shared stale-negative setup, displaced-branch positive and retained-competing-branch outbound case. They observe existing earned work; the outbound case also checks existing headers for both actual branches' record hashes. No header owner is rebound, no sequence/order is synthesized, and all original socket, stale-row, filter-byte, provenance and cleanup assertions remain.

`compact_with_early_branch(prefix,suffix,indexed,ancestor,count,tag)` forwards directly to the existing unchanged `compact_early`. That dependency already starts with actual genesis on the same fresh network, admits new contiguous descendant batches with full parameters, retains the specified side bodies before the competing active suffix, performs real flush/physical checks and attaches BASIC without replacing accepted seals. Its unchanged 17,245-byte source is explicitly frozen; it is not an invented modified owner.

Only the status constructor changes to `compact_with_early_branch(16,8,24,15,8,0x76)`. The later `fork(15,8,0x76)` and equal-height/32-retained-row assertions remain exact. Only the readiness replacement count changes 13→15: after the first branch ends at 24, the next actual branch ends at 25 instead of 23. Original invalidation, wake-once, latch and retained-undo assertions remain. No preference predicate, supplied score or ordinary acceptance policy changed.

## Executed Evidence

All Cargo work used pinned Rust 1.94.1 through command-timings and command-local Bun 1.3.9. Heavy execution was serial, resumable sessions were polled within 60 seconds, and healthy quiet jobs continued normally.

The five fresh original selectors each produced exit 101 with 0 pass/1 RED before source repair. Their corresponding GREEN selections each produced1pass/0 RED, exit 0, with no ignored case. Durations below are libtest seconds.

| Original selector                                                                    | RED seconds | GREEN seconds | Original signal                                                             |
| ------------------------------------------------------------------------------------ | ----------- | ------------- | --------------------------------------------------------------------------- |
| phase160_complete_preparation_actual_encoding_work_exact_and_one_under               | 0.94        | 0.96          | PrematureSpend at depth 1 under the unnecessary Regtest maturity 100 replay |
| phase160_managed_completeness_handoff_keeps_local_buffers_charged_until_transfer     | 0.93        | 0.95          | Same raw serving replay boundary                                            |
| phase160_peer_evidence_genuine_raw_spends_match_independent_pinned_socket_bytes      | 0.92        | 1.15          | Same raw serving replay boundary                                            |
| phase161_operator_authority_actual_equal_height_reorg_reports_verified_common_prefix | 1.06        | 2.29          | BranchNotPreferred                                                          |
| phase159_basic_readiness_owner_reorg_invalidates_exact_branch_and_keeps_latch        | 1.32        | 1.41          | BranchNotPreferred from the genuinely weaker premise                        |

The five exact baseline receipts are:

- `.local/open-bitcoin-dev/phase165-plan20-baseline-phase160_complete_preparation_actual_encoding_work_exact_and_one_under.log`
- `.local/open-bitcoin-dev/phase165-plan20-baseline-phase160_managed_completeness_handoff_keeps_local_buffers_charged_until_transfer.log`
- `.local/open-bitcoin-dev/phase165-plan20-baseline-phase160_peer_evidence_genuine_raw_spends_match_independent_pinned_socket_bytes.log`
- `.local/open-bitcoin-dev/phase165-plan20-baseline-phase161_operator_authority_actual_equal_height_reorg_reports_verified_common_prefix.log`
- `.local/open-bitcoin-dev/phase165-plan20-baseline-phase159_basic_readiness_owner_reorg_invalidates_exact_branch_and_keeps_latch.log`

Their exact GREEN counterparts are:

- `.local/open-bitcoin-dev/phase165-plan20-task1-green-phase160_complete_preparation_actual_encoding_work_exact_and_one_under.log`
- `.local/open-bitcoin-dev/phase165-plan20-task1-green-phase160_managed_completeness_handoff_keeps_local_buffers_charged_until_transfer.log`
- `.local/open-bitcoin-dev/phase165-plan20-task1-green-phase160_peer_evidence_genuine_raw_spends_match_independent_pinned_socket_bytes.log`
- `.local/open-bitcoin-dev/phase165-plan20-task2-green-phase161_operator_authority_actual_equal_height_reorg_reports_verified_common_prefix.log`
- `.local/open-bitcoin-dev/phase165-plan20-task2-green-phase159_basic_readiness_owner_reorg_invalidates_exact_branch_and_keeps_latch.log`

The three passing Task1 selections use ordinary 16-block fixtures and alone do not establish the later 40,000-output coverage.

The first full peer corpus exposed the missed third admission:19 pass/1 RED, no ignored case,450.61 s, child exit 101. Its exact `header-owner-already-bound` signal is retained in `phase165-plan20-fullrust-sync::tests::filter_index::peer.log`. The complete sparse 9,302-block case was allowed to finish. Independent review approved replacing only the redundant call with earned/header observations. The full rerun passed20/0,453.68 s. Its actual large wire-admission and persisted-matrix cases, plus the subsequent complete transport corpus, execute the 40,000-output fixture.

| Complete Rust selector                     | Pass/RED | Existing ignored | Libtest seconds | Receipt suffix                                          |
| ------------------------------------------ | -------- | ---------------- | --------------- | ------------------------------------------------------- |
| sync::tests::filter_index::reorg           | 38/0     | 1                | 197.55          | fullrust-sync::tests::filter_index::reorg.log           |
| sync::tests::filter_index::operator_status | 12/0     | 0                | 12.68           | fullrust-sync::tests::filter_index::operator_status.log |
| phase159_basic_readiness                   | 21/0     | 0                | 14.51           | fullrust-phase159_basic_readiness.log                   |
| sync::tests::filter_index::catch_up        | 58/0     | 1                | 60.02           | fullrust-sync::tests::filter_index::catch_up.log        |
| sync::tests::filter_index::peer            | 20/0     | 0                | 453.68          | finalrust-sync::tests::filter_index::peer.log           |
| peer_filter_transport                      | 19/0     | 0                | 14.39           | finalrust-peer_filter_transport.log                     |
| runtime_write_evidence_cases               | 11/0     | 0                | 4.77            | finalrust-runtime_write_evidence_cases.log              |
| phase165_candidates                        | 15/0     | 0                | 7.21            | finalrust-phase165_candidates.log                       |

All table receipts have prefix `.local/open-bitcoin-dev/phase165-plan20-`. Selectors can overlap; no unique-test aggregate is claimed. Existing ignored subprocess children remain unchanged. Static source declarations are distinct from discovered selector counts: for example, operator_status retains its 10-row declaration inventory while its full selector executes 12 cases.

Required format check, Node all-target/all-feature Clippy with `-D warnings`, and Node all-target/all-feature build each exited 0. Actual wrapper durations were 1.590 s, 6.856 s and 0.299 s respectively; the compiler reported 6.74 s for Clippy and 0.09 s for build. Receipts are phase165-plan20-fmt.log, phase165-plan20-clippy.log and phase165-plan20-build.log, with matching command-timings JSON records.

## Independent Source Review and Exact Renewal

The authentic thirteen-owner before packet remains `.local/open-bitcoin-dev/phase165-p20-before.json`, SHA `0b4980a6bc0adeefa02276646aed09ae3d30adbb738d6dac1bfd05e7436b2c6e`. The frozen pre-pin source/diff packet is phase165-plan20-prepin-source-review.json, SHA `02c596e50b4af0cb7735b96aca1851cf871869b2d526fa7bbe59da45152b3959`.

Independent review confirmed the narrow missed-call inverse, extraction correspondence and seven-file Rust/breadcrumb digest `23f469b4434cfe8ab7e8804d541647d7c51f30bc102eac5f0d6588c9e7036fb1`. Final raw Rust identities and formatted caps are:

| Owner under packages/open-bitcoin-node/src/                        | Lines | Raw SHA-256                                                      |
| ------------------------------------------------------------------ | ----- | ---------------------------------------------------------------- |
| sync/tests/filter_index/peer_fixtures.rs                           | 560   | b3c194131c7133f9deadf001af1334ec660621e0f0c9a45a1f2f86ffabfbcd90 |
| sync/tests/filter_index/peer_evidence.rs                           | 619   | 28ccf9d2294ee76ede6f383180fd220689874164138b0b7843b373f4c25be2d3 |
| sync/tests/filter_index/reorg/fixtures.rs                          | 451   | f2adb79ef209ab8fc63897ba6816e5201fb1c437d6d278aa787cec3a05ebe8b0 |
| sync/tests/filter_index/operator_status.rs                         | 619   | 1e4c42611ee0c667c949012819d93046d07dba038c574d9a7928ea7ea72bc01d |
| network/runtime_authority/filter_index/readiness/tests/failures.rs | 338   | 07bc502f429541f32b8969b2d1ab34d9a424445daf98ed638d7e11d49793ec58 |
| sync/tests/filter_index/peer_fixtures/phase165_source.rs           | 111   | 5240131e624d9b04009b4970cc687f531e507874d0c6f24a299418e2cd5ba7ca |

Before renewal, current 165 exited 1 attributed only to the stale operator_status identity. The four actual redirect/extra/missing/reordered inventory controls passed4/0 with 55 expectations. The reviewed proposal a91580d6543e819f4552b2a551491efab15a663c037de9d7574423ad1653bc2d renews existing source row 103, appends rows 112–116 and appends test rows 51–53 with actual 4/5/9 declarations. Helpers with no executable tests receive no zero-count row. Source owners grow 112→117, test-owner files 51→54 and fixed inputs 241→246. The five new inputs occupy zero-based 228–232; every old input retains its relative order. The ordered inventory SHA is `f9fd5ace788022192b68bdecb45e2100f027f58f5303864b23cf5e7ef26a30d7`.

The nine new actual-source mutations cover raw large-source restoration, skipped prefix bodies, default parameters, removed large delegation, full raw replay, discarded early forwarding, discarded exact status declaration, weaker readiness count and renewed live admission. Each requires a passing loaded baseline, exactly one present replacement token and an attributed owner finding. Four registration controls bind the new helper. The focused run passed18/0 with 129 expectations in 9.09 s; fast current 158 and 162 checkers exited 0, with 162 reporting 208 fixed contracts and exact originalFAA attribution.

Only contracts data and the entrypoint inventory literal changed after review. The exact reverse removes five source rows/three test rows and reverses row 103 plus the one inventory literal; it reproduces pre-pin sources byte-for-byte. The other eleven declared owners and protected dependencies remain exact. Root independently authenticated the ordered imported arrays and source reversal in phase165-p20-root-pin-proof.json, SHA `c6c17b51f5602519c4184f218bdf0ac975b66a006cf477244a1d6fb48f4d044d`. The reviewed thirteen-owner post-pin digest is `c1004140cdb4affcbe8e169fcef988e3858bef7c815bec37dc97119445fdacbb`.

Final guard source hashes are entrypoint `5365d5be2beaf8f9f2cb66e0db4e1cd68af3477b230360470acc07ab438dde64` (266 lines), contracts `ea036d698047f5c64d3430ef499b98d3dba8e5bd9c7d6e5925091eb9b6cc28c1` (523 lines), and tests `810f649a91c18edcfe601f7de7e328bc5b576d01dc418941a21951ce84b6144f` (627 lines). Literal-bearing source identities exclude only leading masked commentary; the raw identities above include every byte.

## Full Matrices and Stable Execution Closure

The new ignored collector preserves every original fixed raw read set, actual 163 factory initializer, corpus entry, optional boundary, canonical-path check and import-closure assertion. It changes only Plan20 labels/current246 data and adds the unchanged live dependency and actual driver. The unexecuted first preparation is superseded explicitly; the final driver uses a matrix-specific receipt prefix to preserve earlier pre-pin logs. Final review packet26b0e5de321b23d999fb60a8ecfba4269e2fe7bc746185ba9c3c175327550797 contains the exact collector diff and driver source.

Root authenticated the pre-run freeze through phase165-p20-root-freeze-proof.json, SHA `87620cf82272e85fda76175629b8a460a30def0be387467ba6187b65cfffdbc1`. It covers 467 canonical paths:465 present and the same two permitted absences,36actual TypeScript modules,14CLI modules and all 168 raw corpus entries. This complete execution union is separate from the 246-input current 165 inventory.

| Complete current matrix | Pass/RED | Expectations | Wall seconds | Log SHA-256                                                      |
| ----------------------- | -------- | ------------ | ------------ | ---------------------------------------------------------------- |
| 164                     | 868/0    | 1565         | 340.754804   | 55f23f92397107515d2c1f30e94ea5da4c8ab4e153c2d25a888270d34d03293e |
| 165                     | 1557/0   | 5603         | 632.142236   | a573b55f014d9329e7562a0f0f3862468c711da2dab2e76079fa23b773e586d7 |

All seven serial driver commands exited 0. Exact 164 reports 135 fixed Rust inputs/477 reviewed edges; exact 165 reports 117 reviewed owners/54 executable test-owner files. Breadcrumbs pass. Managed checks scan 1521 files with zero cap exceptions and zero findings. Whitespace passes. The matrices introduce no ignored/skipped tests. Exact commands, all seven log hashes and durations are in `.local/open-bitcoin-dev/phase165-plan20-matrix-matrix-results.json`, SHA `1610622882bea838ad71aaf4be72fbb541d6be6986e7d75a99322713b3a0441b`.

The complete pre-run freeze SHA is `860860ce5353f16bdc5eaf498cd7d05128f743383235bdd23844bf50ea916ccf`. At 10:19:24.659Z, phase165-plan20-full-freeze-closed.json confirms all 467 entries unchanged, SHA `f82901f6d6f663034cd65187bdab57533be06b6613472b969d22de6ed0468c62`. Root independently rehashed all canonical paths, bytes and presence states and authenticated all seven logs. No additional matrix was run after success.

The dynamically derived current 168-entry factory corpus is 4,406,522 bytes, SHA `8f3cd0d8f20c290a4cd61e8bebf0ada352af5fa01c10799f242d89d3a46b3cfd`. It correctly reflects current ROADMAP, breadcrumbs and reviewed reorg fixture bytes. This is distinct from immutable original snapshots/manifests/recipes/receipts; those remain exact. An initial coordinator probe used the prior derived corpus digest as an equality expectation; the corrected read-only proof records that audit-only setup assertion and required no product change.

## Preserved Dependencies and Historical Inverses

- Unchanged live.rs: `4e5376a4ff1b654f9532810d59d6d903cbc1e6ec7093e06b15f3108952327dcc`.
- Unchanged support_runtime.rs: `5a25857c552bb7dcfdd040ad15e19c4616f52ef8b35df69b4f11a42dcd5b1f05`.
- Unchanged current 20,688-byte preservation source: `e6cb5740cb5c83655aed14c43b1206847d264431d944741a365771c3e8d76f5b`; original 20,556-byte attribution remains separate.
- Unchanged fixture-source.ts: `b92d2d91da1dd7715e7de37737769f30bd543cb2fd8a7a1eecb92aaf3335f3e3`.
- Unchanged historical-tooling.ts: `47bbcd914cf5a632f618310b69b987462551ca2f103bce823a8d0cc6326a5f65`.
- Unchanged working 158 checker/tests: `1b87c3bddf0d65db9eea63039780d5b6b03bd3a6db9a99d56fd61637d0391c6c` / `418479c36f497919cf2ca289d93dcefaa8b009920dae467181cca528365c7f81`.
- Unchanged working 162 checker/tests: `5ffd8e4ea3d54197cd88923cbae2375f977fefe755e2258bd24c3450ec4f7c44` / `2932b0ee2de0ac1d600d454590fcbe2e9281f748bcb13c50048c7480da63dfcf`.
- Unchanged mixed C fixture bundle: `1330515f81cb39108b7605814f50ed4c1f1572a0280b8415902085791f9eab5c`.

Their exact historical inverse composition, originalFAA source projection, original evidence iteration, fourteen-module CLI and six mandatory native registrations remain intact. No old 164 data, historical manifest, recipe, receipt, snapshot, ordinary opener contract or adapter inverse was edited by Plan20.

## Deviations and Retained Gates

The approved simplification reuses existing compact_early rather than duplicating its checked lifecycle. The cap-driven extraction remains within the declared helper and retains every large derivation boundary. The initially missed third genuine admission received its authentic full-peer RED, independent narrow source review and complete20/0 rerun. The receipt-prefix correction changed only unexecuted ignored driver labels and preserved earlier receipts. No scope expansion, source-policy relaxation or cap exception occurred.

Stub scanning found no incomplete implementation; the existing unavailable wording is deliberate claim-test data. No new network endpoint, authorization path, schema or unplanned trust surface was introduced. No authentication gate occurred.

Plans21–24 and final 14 remain parent-owned. Default repo-native verification, complete workspace tests/coverage/Bazel, final source/security/formal acceptance, shared-state changes and normal-hook commit/push are pending coordinator gates. Scoped Node and offline guard results do not close them. Requirements remain contributed, not completed. Ordinary canonical selectors still refuse, networking defaults remain off, and Phases166/167/168 retain bootstrap/exact reopen/genesis BASIC, contextual and pinned-daemon qualification respectively.

## Self-Check: PASSED

The Summary and all ten implementation owners exist. The exact source/data review proofs and all eight scoped Rust corpus receipts are present; the seven final driver commands have exit 0 with authenticated logs. All 467 frozen canonical paths and presence states match the pre-run snapshot. Caps, breadcrumbs, managed rules and whitespace pass. Git finalization remains coordinator-owned; this Summary claims no commit hash or phase-wide acceptance.
