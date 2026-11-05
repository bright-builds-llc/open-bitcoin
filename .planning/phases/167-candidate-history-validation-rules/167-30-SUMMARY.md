---
phase: 167-candidate-history-validation-rules
plan: "30"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: local-submission
tags: [rust, regtest, canonical-history, typed-validation, durable-storage]
requires:
  - phase: 167-12
    provides: Full copied-child history proof, immediate original reproof and safe configured runtime reopen
  - phase: 167-33
    provides: Strict serialized wide-script body, BASIC and mempool consumers with wide-output oracle and full history reproof
  - phase: 167-36
    provides: Single storage reopen bound and actual native resource projection
  - phase: 167-39
    provides: Accepted logical inventory freshness on the final Plan 40/42 source frame
provides:
  - Safe factory-earned local Regtest submission with typed validation boundaries
  - Real acceptance, Always flush, BASIC and achieved-state checks before public success
  - Complete retained-bound and native body, Coins and undo resource controls
affects: [167-35, 167-13, 167-14, 167-15, 167-16, 167-17, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Private store-bound command, shared typed validation, prepared data projection, achieved durable checks]
key-files:
  created:
    - packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests/resources.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests/reopen_bound.rs
  modified:
    - packages/open-bitcoin-consensus/src/block/witness.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/validation_history.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests.rs
    - packages/open-bitcoin-network/src/header_store/eligibility.rs
    - packages/open-bitcoin-network/src/header_store/canonical_history.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests/typed_boundaries.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/validation_history/workflow.rs
key-decisions:
  - Mint the command only through safe configured or genuine-genesis-seal factories and check its actual owner before validation.
  - Preserve raw intrinsic precedence and typed errors through the existing header and live body algorithms.
  - Treat capacity and native resource limits as operational bounded-support refusals and require real durable achieved state before Accepted.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-23T14:04:39.488Z"
commits: false
duration: "2h 28m 49.459s for the current 17-command interval; earlier implementation, failures and review are separate"
completed: 2026-10-23
---

# Phase 167 Plan 30: Safe Local Canonical Submission Summary

## Source Scope

Four tasks declare the following ten source owners, totaling 202,162 bytes. Relative to the composed original 7359 + 0702 authority, nine owners have changed patches; `block.rs` is byte-identical and remains an explicitly checked owner. All files fit the 628-line hard budget. The three dedicated test children are 590, 619 and 610 lines, within their 620-line target; the existing parent test file is 626 lines. The other 52 source inputs remain exact.

| Declared source owner                                                                                   | Lines | SHA256                                                             |
| ------------------------------------------------------------------------------------------------------- | ----: | ------------------------------------------------------------------ |
| `packages/open-bitcoin-consensus/src/block/witness.rs`                                                  |   626 | `558ff3b7f13286cd5b8e9110430f4a71fbb0e4298313ce58730bddd6e3159f9d` |
| `packages/open-bitcoin-consensus/src/block.rs`                                                          |   485 | `3af3a523ae571690c33bc75871bf49b1b40d213404d8803ce488a5351c177205` |
| `packages/open-bitcoin-node/src/network/runtime_authority/validation_history.rs`                        |   374 | `c4e06c6494d1581cf0a7b04e3597de9a1d0a1fd4371056632e2cee560c85a66b` |
| `packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests.rs`                  |   626 | `e035b7413661ff3d854378d4ba2ef976f7419c4d565de5f61132b950f058e339` |
| `packages/open-bitcoin-network/src/header_store/eligibility.rs`                                         |   604 | `1fd6670e49fd4b1af06a4f0eadcdfe5071e0151602d2b25078ae470a804e4f7b` |
| `packages/open-bitcoin-network/src/header_store/canonical_history.rs`                                   |   625 | `d4c00a16776991db8f13de1875566d61da4be8934768e8e579e578f4c751ee55` |
| `packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests/typed_boundaries.rs` |   610 | `d44b1dcba1d35156d6a45abaad5bc2e24b649261be397481b3446f03419405a6` |
| `packages/open-bitcoin-node/src/network/runtime_authority/validation_history/workflow.rs`               |   336 | `f5156a2ac288f49ab4c28908f03d6a0b6796aea3de74f191d3b484a2fbc25788` |
| `packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests/resources.rs`        |   619 | `77420c0f43c925a8bb6d723ec5f796085544daf22475f08b66792c9c550dc160` |
| `packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests/reopen_bound.rs`     |   590 | `bbfc922819873698805fa81f7b79cdc814b70b5a88081b415b4153c130bcde3d` |

All current bodies match the 62-input freeze `38374f2caee303006c4f75e960cad7381edc5d6bdd2fdf3cd32db9e80c7512ff`. The 201 native engine/lock inputs and actual copies authenticate separately; they are not claimed as per-job captures. Existing breadcrumbs and conventional child registration remain intact.

## Bounded Implementation

**A command earned through a safe configured or genuine-genesis-seal factory now executes local Regtest submissions through real validation and durable publication.** Other network selectors keep strict genesis/refusal routing and return no command. Private captured network, store, parameters, chain and incarnation, together with an operation mutex, prevent arbitrary runtime/store/bool construction. Both externally inferred `submit`/`fold` API examples compile across the private module boundary; their `no_run` status supplies compile evidence, while genuine runtime cases prove behavior.

The actual owner check precedes selected complete raw intrinsic validation. Fallible domain conversion, genuine accepted duplicate evidence, capacity and contextual admission follow in that order. Detailed header admission and live false-mode body eligibility share the existing algorithms; legacy wrappers preserve their spelling and mutation behavior, and cached true-mode behavior remains distinct. Direct missing parent uses the actual lookup. Genuine contextual Consensus failures may earn permanent failed provenance; witness Mutated, resource, registry, clock and storage errors remain operational or nonpermanent as appropriate. Error categories are not reconstructed from display text.

Earned witness normalization reuses the existing commitment selector and only supplies an absent applicable coinbase reserve. It preserves existing witness, inactive/no-commitment cases, base identity, stripped size, sigops and intrinsic results. The inherited complete raw signed/script/witness model and bounded codec remain unchanged.

Early header/KnownOnly resource projection runs before durable publication; a fresh inventory follows those legitimate own writes. Real normalized body eligibility, preference authorization, scripts/reward/Coins/BIP30/count/undo preparation and sealed mempool lifecycle precede body persistence and absorption. Prepared projection is expected data and cannot mint ScriptsValid or acceptance. A later resource refusal can retain earned header/KnownOnly and volatile body/eligibility facts while preserving accepted state.

Public Accepted requires the actual absorption receipt, normalized body and trusted headers, ordered Always flush with the real disk probe, BASIC query/barrier/turn, and final owner, target, pending-history, durability, complete achieved-map and physical-support checks. Late publication, flush, BASIC or achieved-state errors honestly retain advanced state and actual receipts. The command mutex spans this sequence, but direct authority can change while the inner guard is released for BASIC; final checks detect the observed changes.

## Four Task Outcomes

| Task                                                              | Result                                                                                                                                                                       |
| ----------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1. Normalize typed witness reserve after shared intrinsic success | Six pure controls pass; existing selector and shared intrinsic logic are reused.                                                                                             |
| 2. Earn and execute the store-bound local submission command      | Genuine factory/owner, typed errors, actual duplicates, acceptance, fault, resource and reopen controls pass; both external API examples compile.                            |
| 3. Preserve the safe-reopen envelope before new identities        | The existing storage getter bounds actual retained identities at 306; novel identities refuse before effects, while real existing membership and duplicates remain distinct. |
| 4. Partition complete retained-bound controls                     | The declared reopen child retains every substantive control and the unchanged whole original 305-descendant unit.                                                            |

No task commits were created: this lifecycle explicitly uses `commits: false`. Root owns later shared metadata, acceptance and Git finalization.

## Actual Behavioral Evidence

Both the broad provider selection and the separately required six-case retained-bound selection passed the original complete 305-descendant case. The original unit remains 72 lines and 2,559 bytes, SHA256 `4b89e729cfadd393be0aadf9e17414017bf58dfec856727d7e3c277a819027d6`. Each execution genuinely accepts all 305 descendants, retains 306 identities including genesis, refuses the next novel block before effects, checks complete accepted state/headers/physical manifest, drops all owners and safely reopens that source. The legacy 307 case remains a negative support control.

In the dedicated six-case run, one physical gate reports Changed on attempts 1, 2 and 3, then Ok on attempt 4 under the unchanged work allowance. Its actual safe reopen reports 1,270 units, 2,498,710 bytes and a 65,536-byte maximum authority row; child time is 10.706630291 seconds and total time is 29.611282125 seconds. The provider's separate run reports one Changed followed by Ok on attempt 2. These observations do not establish future worker availability, latency or quiescence. Numeric directory observations do not establish a namespace or a more precise metadata failure cause.

The real physical control observes sequence 15→16 with all nine maps and the catalog equal. Complete helpers preserve all five Accepted concerns, seven non-anchor BASIC fields and full processed/safe identities; constructor-derived branch anchors are checked separately. Full StoredFilterRecord identities/bytes, fresh epochs, all nine maps and valid next acceptance remain substantive oracles. Post-BASIC external writes and advancement produce operational errors with genuinely advanced state.

Both mature-spend modes earn their funding and maturity through the real command. The accepted baseline passes native inspection, the actual body is 31,655 bytes, and its 216,306-byte undo causes pre-absorption refusal. Disabled mode has 1,198 baseline units and projected Coins 101; Enabled has 1,403 units, Coins 101 and BASIC row 174 bytes. Both old accepted sources safely reopen and accept a supported next block. Both 4,096-unit near-full controls safely reopen. Normalized bodies grow from 65,536 to 65,572 bytes and refuse before absorption in both modes, followed by safe reopen and valid next acceptance. The original 66,147-byte body and 4,200-output probes now refuse operationally without advancing accepted state and safely reopen.

## Fresh Seventeen-Command Verification

All four published Task XML bodies are preserved. Fifteen automated occurrences contain 14 unique original commands because Tasks 3 and 4 repeat the retained-bound selection. The two resource-amendment commands and inherited Plan 36 smoke produce **17 unique mandatory commands**, all executed fresh with the original argv. There are **259 overlapping passing executions**, with zero failed, ignored or measured cases. The three zero-test rows are successful strict Clippy gates; the two API cases compile `no_run` examples.

| Timing key                          | Passing executions | Command seconds | Exit |
| ----------------------------------- | -----------------: | --------------: | ---: |
| `phase167-30-pure`                  |                  6 |           0.605 |    0 |
| `phase167-30-codec`                 |                 80 |           4.093 |    0 |
| `phase167-30-provider`              |                 29 |        3969.398 |    0 |
| `phase167-30-api`                   |                  2 |          14.679 |    0 |
| `phase167-30-clippy`                |                  0 |           0.412 |    0 |
| `phase167-30-typed`                 |                 14 |          35.482 |    0 |
| `phase167-30-network-regressions`   |                 88 |           1.168 |    0 |
| `phase167-30-network-clippy`        |                  0 |           0.309 |    0 |
| `phase167-30-inherited-wire`        |                 10 |           0.185 |    0 |
| `phase167-30-inherited-transaction` |                  5 |           0.204 |    0 |
| `phase167-30-inherited-intrinsic`   |                  9 |           0.323 |    0 |
| `phase167-30-reopen-bound`          |                  6 |        2562.158 |    0 |
| `phase167-30-legacy-307`            |                  1 |          29.829 |    0 |
| `phase167-30-reopen-bound-clippy`   |                  0 |           0.745 |    0 |
| `phase167-30-resource-probes`       |                  2 |          23.122 |    0 |
| `phase167-30-resource-matrix`       |                  6 |        1284.303 |    0 |
| `phase167-36-guard-smoke`           |                  1 |          33.581 |    0 |

The current command interval is 2026-10-23T11:06:04.421Z through 13:34:53.880Z: **2h 28m 49.459s**, including real inter-command waits. Earlier implementation, failures, review and documentation are excluded. Every command has 62 actual before and 62 actual after source copies: **2,108 command source copies**. Pinned Bun 1.3.9/Rust 1.94.1, original physical TMPDIR, default target/cache, selected environment and cooperative timing wrapper remain. No source writes occurred during builds and command intervals did not overlap. Same-frame Plan 40 workspace format-check evidence supplements these obligations without becoming an eighteenth Plan 30 command.

Exact commands, argv, named stdout, raw summaries, stderr, child records and timing originals/copies are retained in the [root matrix authentication](/Users/peterryszkiewicz/Repos/open-bitcoin/.local/open-bitcoin-dev/phase167/after42-preparation/actual-runs/accepted42-20261023-plan30-global-wait-v1/plan30/root-matrix-inverse-authentication.json). Interleaved stderr interrupts some raw completion lines; separate authenticated stdout supplies every named completion and raw summaries confirm full counts. Raw-parser undercounts are preserved as metadata diagnostics.

## Persisted Waits and Preserved History

The unchanged original host-global predicate observed 1,365 busy metadata checks: 446 before release, then 167 before command 13, 39 before command 14, 36 before command 15 and 677 before command 16. All other waits have zero busy observations. Terminal snapshots exit zero with no offenders and precede the actual starts. These are metadata observations, not application failures or retries. The old setup stop `429691e03c85a5af10a0ed7cb91419810063ebd43700dc96a468f7fba7852155` still has `jobs: []`.

The hash-pinned runner establishes wait→capture→started record→spawn order. No independent source-capture clock timestamp was recorded. There is no host-arbitration theorem, removal of the observation-to-spawn race or proof that a cooperative lock was held. The known timing-wrapper lock fail-open constraint remains for formal source ownership and canonical review before Plan 17 guards.

Original resource RED result `467ed4700b43821067359a6b7df34b145386b780ea2ae4b41d00cc81998661b5`, raw `795852ab258687b03c37658eb6c5d9f799578faffad6b9d8d0a14e2fd7594a89`, preserves actual Accepted followed by refused safe reopen. Historical Plan 39 result `c217bcd7669cf793ff2cf35bc28141f87d74a8903deac8560e6df4cbac89db61`, raw `b9c3ae9cf7e770d591e1860c171b03d6abe435ff3cfca22f958aff7df3ae0f69`, remains five passed and one failed in 4,812.76 seconds on its old 53-input scope; the failing prefix height is unknown and its last five queued commands never started. Earlier 32/53/55-input implementation, setup, partial failure and inverse receipts remain historical. No old pass supplies current credit, no combined historical job/copy total is inferred, and failed historical datadirs were not accessed or removed by this Summary work. Reviewer-only schema/read-helper diagnostics remain distinct from application failures.

## Original Inverse and Evidence Index

The composed inverse authority is original resumed capture `7359b36c46a32fdc49d490b81224abfa8eceb45b3d25fb6b13635aea78cec1e3` plus additional true reopen-child absence `070213608e8eea8c5d4a32b5779133e4e7ca159a2e69dcd752826cb3fc4595e4`. Earlier `f077d0a3aee8b934957f5c4b551fb27b14637201d110f57bd791769852190fe6` remains separate partial ancestry and is not the sole baseline.

Root and the distinct independent reviewer generate identical 133,118-byte inverse/forward patches. Strict BSD `--batch -E -F0 -p1` replay exits zero without offset, fuzz, reject or reversal. Inverse restores eight exact original bodies plus true absence of `resources.rs` and `reopen_bound.rs`, preserving all 52 non-owner inputs; forward reconstructs all 62 current bodies. There are nine changed-owner patches. Original workflow `4ebd` already existed; original parent `tests.rs` is 215 lines / `c1ec`, without NativeRows/Readable. Plan 38 cleanup is vacuous against that original parent. The whole original 305 unit and live source frame remain exact.

| Authority                                                                                                                                                                                                                          | Bytes | SHA256                                                             |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----: | ------------------------------------------------------------------ |
| [Current final matrix](/Users/peterryszkiewicz/Repos/open-bitcoin/.local/open-bitcoin-dev/phase167/after42-preparation/actual-runs/accepted42-20261023-plan30-global-wait-v1/plan30/execution/final-index.json)                    |  6042 | `9ed9cf2b2f4aecba2392b8be4326260d9edecbfbcba6120a00e848a447b8af61` |
| [Root matrix authentication](/Users/peterryszkiewicz/Repos/open-bitcoin/.local/open-bitcoin-dev/phase167/after42-preparation/actual-runs/accepted42-20261023-plan30-global-wait-v1/plan30/root-matrix-inverse-authentication.json) | 16582 | `75e2ab243d1a9fa26b30c7085256a0cce2de598b612bea9ad8a865eb5f07e711` |
| [Root original inverse](/Users/peterryszkiewicz/Repos/open-bitcoin/.local/open-bitcoin-dev/phase167/after42-preparation/actual-runs/accepted42-20261023-plan30-global-wait-v1/plan30/inverse-audit/result.json)                    |  6324 | `e90fcb5e3b80e6e1a38e20f24f2d63526beb7d0f0102790fe385a838569fdafb` |
| [Independent original inverse](/Users/peterryszkiewicz/Repos/open-bitcoin/.local/open-bitcoin-dev/phase167/plan30/independent-review/after42/independent-inverse/result.json)                                                      | 33429 | `4cf55e1062b245febd50ec522c6dcd9a5a2bbea2547aeacfe7da26adc0e8be2c` |
| [Supplemental wait proof](/Users/peterryszkiewicz/Repos/open-bitcoin/.local/open-bitcoin-dev/phase167/plan30/root-wait-auth-preparation/root-actual17-wait-authentication.json)                                                    | 58719 | `391de4f89ed70d7b532ca2521afdf4d968900d3e007eb8c0eba1ffeef7a21df1` |
| [Immutable independent CORE](/Users/peterryszkiewicz/Repos/open-bitcoin/.local/open-bitcoin-dev/phase167/plan30/independent-review/after42/final/167-30-CORE-REVIEW.md)                                                            | 22415 | `f4d4e8453d44e7b9d8bb694ed2d2449e1463e36370d42bf37ddcd8e665986e51` |
| [CORE index](/Users/peterryszkiewicz/Repos/open-bitcoin/.local/open-bitcoin-dev/phase167/plan30/independent-review/after42/final/core-review-index.json)                                                                           |  4536 | `7ae2e992f52397be6f5ee0197bd4567793a8f0d04708b790cbf325d95d7add33` |
| [Summary authoring release](/Users/peterryszkiewicz/Repos/open-bitcoin/.local/open-bitcoin-dev/phase167/plan30/root-summary-authoring-release-after42.json)                                                                        |  1697 | `f05455eaafd96998325176d92624f991c16ec3b1150ccf4525437a6e9e0b2de9` |

The immutable CORE reports zero source, API, security or evidence findings. Its input index authenticates 3,263 unique files, including current command copies and separate native inputs. Root read the whole CORE and packaging records and authenticated the input/range evidence before releasing this Summary. The CORE consumes completed source/runtime/inverse/wait evidence and has no dependency on this Summary or future Summary QA. Separate independent Summary binding and root Plan 30 acceptance remain pending.

## Deviations from Plan

The published retained-bound, resource-projection and test-partition amendments make the contribution concrete: one existing 306-identity capacity owner, the actual native resource consumer, and the declared coherent reopen child. Their exact original commands and substantive assertions remain. Accepted Plans 40, 42 and 39 provide the current common source frame before this fresh Plan 30 matrix; their earlier passes do not replace any of its 17 commands.

The reviewed ignored global-wait adapter preserves the original host-global guard, exact application commands and failure stops while waiting for unrelated matching processes to finish naturally. It follows the zero-application setup stop and adds no application retry, cap change or target/cache intervention. These bounded amendments and scheduling correction defer none of the four Plan 30 task outcomes.

Simplification review retained existing witness/header/body algorithms, the single storage capacity getter, real staged projection and one achieved-data check. Cohesive workflow and test children keep ordinary layout without another authority model or rule calculator. No new dependency or unresolved implementation stub was identified in this contribution.

## Remaining Acceptance and Phase Gates

`requirements-completed` retains the plan's five exact IDs for GSD mapping. **CVLD-01, CVLD-02, CVLD-03, CVLD-04 and CVLD-06 remain globally OPEN.** This bounded contribution does not renew source guards or the central manifest, close the phase, qualify a daemon/transport, or establish pinned-daemon differential parity.

Root must still bind separate independent Summary QA and adjudicate Plan 30. Subsequent phase gates remain: Plan 35 FIRST product cases B–E; Plan 13's formal visibility seam HOLD (`e786`) and Plans 14–16; canonical final SOURCE review before Plan 17 guards, including source 123 and formal E→17 ownership; formal D with fewer than five tasks; Plan 18 security/native/mutants; Plan 19 parity, README and manifest; Plan 25 writing-for-agents; and Plan 20's entire default native verifier, fresh pure-core coverage, Bazel, final source/security/goal audits and lifecycle checks. Normal-hook main commit/push remains root-owned. Phase 168 stays held. No future work is accepted by this Summary.

## Self-Check: PASSED

The ten declared paths and exact source hashes match the current 62-input freeze. The actual 17-command matrix, original inverse/forward proofs, separate native input authority and immutable independent CORE authenticate; historical evidence remains separately identified. Lifecycle, baseline, `commits: false`, actual document creation time, verbatim requirement mapping and globally OPEN status are checked by scoped document QA. GFM/frontmatter formatting and the GSD ten-file Summary check pass. This is a document self-check; separate independent Summary QA and root acceptance remain required.
