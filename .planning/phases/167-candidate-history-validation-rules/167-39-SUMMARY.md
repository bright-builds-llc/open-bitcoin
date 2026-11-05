---
phase: 167-candidate-history-validation-rules
plan: "39"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: node-storage
tags: [rust, logical-freshness, submission-inventory, canonical-reopen]
requires:
  - phase: 167-38
    provides: Opaque same-open stamps and complete tracked storage mutation coverage
  - phase: 167-40
    provides: Accepted earlier journal pressure with protected Plan 41/42 physical observation
provides:
  - Logical freshness in the actual detached submission-inventory consumer
  - Real physical positives and restored-write, catalog, capture and in-flight negatives
  - Both-mode command/reopen evidence and a complete original 305/306 retained-bound pass
affects: [167-30, 167-35, 167-13, 167-17, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Existing opaque mutation stamp, complete bounded reader, fresh achieved capture after own writes]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/canonical_history/records/projection/tests/freshness.rs
  modified:
    - packages/open-bitcoin-node/src/storage/canonical_history/records/projection.rs
    - packages/open-bitcoin-node/src/storage/canonical_history/records/projection/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tracked.rs
key-decisions:
  - Use the original identity-bound logical stamp around complete capture and current-source rereads.
  - Take a fresh achieved capture after legitimate own writes while preserving original store identity and all projected rows.
  - Retain physical sequence observation only under cfg(test) after removing its last normal consumer.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-23T10:13:44.900Z"
commits: false
duration: "49m 54.733s for the fresh nine-command matrix; earlier implementation, failures and review are separate"
completed: 2026-10-23
---

# Phase 167 Plan 39: Logical Submission Inventory Freshness Summary

**Actual submission inventory now tolerates purely physical maintenance while rejecting every tracked intervening logical write, including identical rewrites and ABA restoration.**

## Final Source Files

Two tasks have four source owners: 64,300 bytes and 1,807 normally formatted lines. The new test child meets its 620-line target; all owners remain within 628 lines. The other 58 current source inputs, including accepted Plans 40/41/42 and the original Node retained-bound fixture, remain exact.

| Source                                                                                           | Bytes | Lines | SHA256                                                             |
| ------------------------------------------------------------------------------------------------ | ----: | ----: | ------------------------------------------------------------------ |
| `packages/open-bitcoin-node/src/storage/canonical_history/records/projection.rs`                 | 11373 |   295 | `a9d8c5609a4e50832e0be13e63f4213e815f3c563ad6dbf02dcb62896e0842df` |
| `packages/open-bitcoin-node/src/storage/canonical_history/records/projection/tests.rs`           | 20374 |   529 | `c6428e14b5c30b271561133b5e4b9e66a5c49948b475cfbbff2fb6959b07d5c6` |
| `packages/open-bitcoin-node/src/storage/canonical_history/records/projection/tests/freshness.rs` | 22538 |   620 | `e174afde4befb6ccb4ed86e7cc0ea4e3fbc3d571985228fa95d5725dd70ca7ba` |
| `packages/open-bitcoin-node/src/storage/fjall_store/tracked.rs`                                  | 10015 |   363 | `1374416c5b67a9e3b0ee6e418ec5f7576bfb74b6caa40e7c215acce31f4887d3` |

## Bounded Implementation

`SubmissionInventory` replaces its physical sequence with Plan 38's opaque same-open `MutationStamp`. Capture acquires that stamp before catalog/native reading, retains the existing complete reader, namespace/resource policy and source digest, then checks the same stamp after hooks and catalog validation. `check_current_source` brackets a real complete reread with the original stamp and catalog checks. Equal final bytes cannot forgive a tracked write.

Projected headers, validation records, Coins, undo, chain metadata, bounded replacements and BASIC bytes stay unchanged. `compare_achieved` deliberately takes a **new stable capture after expected own writes**, retains the original store/publication identity and compares every projected namespace/key/value row. Its boxed `FnOnce` remains lifetime/store-bound data, not acceptance authority. The command still rebaselines after early KnownOnly/header writes and preserves its prepared, Always/BASIC and final physical gates.

The parent test file adds only child registration and capture routing. `tracked.rs` adds `cfg(test)` to the truthful native `visible_seqno` observation after its last normal inventory use is removed. No second clock, reader, engine, raw escape, public getter, dependency or writer redesign was introduced. Existing catalog serialization and short generation guards remain; no generation mutex spans I/O. Arbitrary independent raw-engine/filesystem writers remain outside the cooperating logical boundary.

## Actual Controls and Complete Retained-Bound Pass

The seven logical-freshness controls use real stores and mutations: physical rotation; identical rewrite/ABA/empty add-delete/nonexistent delete plus maintenance/clear-restore/catalog recreation; foreign identity; capture and reread writes; and an actual in-flight native conversion. In-flight results are saved before the writer is resumed and joined, then assertions verify stale old observations, equal restored rows and a valid fresh capture. Existing policy tests and inherited native Err/unwind/overflow/poison/foreign-batch controls also execute.

Both BASIC modes pass genuine command acceptance, full effective/durable state and all-nine-map comparisons, full StoredFilterRecord/epoch checks where enabled, all-owner-drop safe configured reopen and valid next acceptance. Early logical conflicts remain operational with accepted/native state preserved. Late conflicts remain operational with genuinely advanced state and successful reopen/next; no rollback or false Accepted receipt is claimed.

The unchanged six-case retained-bound selection now passes in **2,551.36 test seconds**. Its original 72-line, 2,559-byte unit remains SHA256 `4b89e729cfadd393be0aadf9e17414017bf58dfec856727d7e3c277a819027d6`. It accepts all 305 descendants, retaining 306 identities including genesis, refuses the next novel block before effects, checks complete accepted state/headers/manifest, drops all owners and safely reopens that 306-retained source. Enabled count/membership controls, physical maintenance and honest post-BASIC cases pass unchanged.

One gate during the prefix observes three safe Changed results followed by Ok on attempt four under the existing work allowance. The log names numeric `keyspaces/3`; no namespace identity or more precise cause is assigned to later metadata-unavailable/changed results. The actual large reopen reports 1,270 units, 2,498,710 bytes, a 65,536-byte authority bound, 9.251149917 seconds in the child and 27.307924333 seconds total, with source gates true. These are observed results, not latency, scheduling or quiescence guarantees.

## Fresh Nine-Command Verification

All nine original commands executed fresh on the same 62-input freeze. Each has its own 62 before and 62 after actual source copies: **1,116 current command copies**. The test selections pass **45 overlapping executions**, with zero failed or ignored cases; these are not 45 unique tests.

| Timing key                           | Named passes | Test seconds | Command seconds | Exit |
| ------------------------------------ | -----------: | -----------: | --------------: | ---: |
| `phase167-39-logical-inventory`      |            7 |       103.84 |         104.428 |    0 |
| `phase167-39-physical-regression`    |            1 |         0.81 |           1.273 |    0 |
| `phase167-39-command-smoke`          |            1 |        31.99 |          32.159 |    0 |
| `phase167-39-reopen-bound`           |            6 |      2551.36 |        2551.891 |    0 |
| `phase167-39-inventory-regressions`  |           13 |       120.16 |         120.932 |    0 |
| `phase167-39-foundation-regressions` |           12 |        13.92 |          14.487 |    0 |
| `phase167-39-seal-regressions`       |            5 |       166.31 |         166.523 |    0 |
| `phase167-39-final-format`           | format check |          n/a |           2.054 |    0 |
| `phase167-39-final-clippy`           |  strict lint |          n/a |           0.329 |    0 |

The current matrix spans 2026-10-23T09:13:18.454Z through 10:03:13.187Z: 2,994.733 seconds, excluding older implementation, failures and review. Pinned Bun 1.3.9, Rust 1.94.1, original physical TMPDIR/default target/cache and cooperative timing wrapper remain. Format is workspace check-only; Node Clippy retains all targets, all features and warnings denied. Source copies match throughout and command intervals do not overlap. The 201 native/lock inputs authenticate separately, without per-job native-copy claims. Independent stdout parsing confirms all intended names despite incomplete interleaved raw-stream name metadata. No test was shortened, ignored, retried or replaced by partial passes.

## Preserved Historical Evidence

The old 53-input full selection remains **5 passed / 1 failed / 0 ignored in 4,812.76 seconds**, result `c217bcd7669cf793ff2cf35bc28141f87d74a8903deac8560e6df4cbac89db61`, raw `b9c3ae9cf7e770d591e1860c171b03d6abe435ff3cfca22f958aff7df3ae0f69`. It failed at accepted physical inventory with an inspection-inventory-limit error. Its exact failed prefix height is unknown. The earlier three passing selections and five passes inside that failed selection do not supply new credit; the remaining five old final commands never started.

Earlier physical-control `94aa911c`, complete-prefix `33ac77c9`, initial behavioral RED, compiler/API setup, formatting setup and old source-only HOLD remain separately attributable in the retained history. Historical scopes are not relabeled as 62-input observations, and no combined historical job/copy total or full Plan 39 duration is inferred. The old failed datadir was neither opened nor removed by this work.

## Evidence and Original-Authority Inverse

Current receipts are under `.local/open-bitcoin-dev/phase167/after42-preparation/actual-runs/accepted42-20261023/`:

| Artifact                                         | SHA256                                                             |
| ------------------------------------------------ | ------------------------------------------------------------------ |
| Common `freeze/result.json`                      | `38374f2caee303006c4f75e960cad7381edc5d6bdd2fdf3cd32db9e80c7512ff` |
| `plan39/execution/final-index.json`              | `17549d9ee4557a5e7e041a8cd5254abf1fab10d3ed36bbc4e6062fb929c72297` |
| `plan39/inverse-audit/result.json`               | `a88a81deaa4aa305ccf7965020ce24f7028b2ec418a89452934f66ce2ee7228c` |
| `plan39/root-matrix-inverse-authentication.json` | `e5da2fbdf80a820f34f9eda35936af67a8f6c91c97e5ab8ef0b137100096e09e` |

The full six-case result is `e73a6d6a`, with raw log `1b95a922`. Detailed current and historical descriptors remain in the independent evidence packet.

Original inverse authority remains `plan39/root-before/capture.json`, 112,361 bytes, SHA256 `85e7957510add9fbbf5234a53e8a3806bee3975c54291517ce4471220c004e65`. Strict BSD patch at `-F0` restores **three original bodies plus the true freshness-child absence**, preserving 58 current non-owner states including accepted predecessors. The endpoint is 61 present files and one absent child; forward application reproduces all 62 final bodies without offset or fuzz. A distinct reviewer sandbox independently proves the same result, receipt `60744dfe`. Live source is never patched.

## Independent Review and Document QA

The whole immutable CORE was read and authenticated: `.local/open-bitcoin-dev/phase167/plan39/independent-review/after42/CORE.md`, 17,658 bytes, SHA256 `9ad5f4a35c8f47c50c4390f8379b131cc7945c1fbc63d7f1a7ec578c32d6317d`. Verdict is CLEAN with zero critical, warning or information findings. Its final index is `core-index-v2.json`, SHA256 `92fb67685c82672d2b5eb4788fb54257489939742b178285b8829b8ca061f737`; it has no Summary dependency. The unpublished prior index is retained as metadata history; v2 corrects its readiness-only 305-pass marker after the actual pass without changing source, CORE, evidence or verdict.

The independent final audit (`de32782f`) authenticates 4,021 reference occurrences across 1,731 unique resolved paths, all current command copies, separate native inputs and full named outcomes. These reference counts are not copy counts. Source/API/security and simplification review retain one existing stamp, complete reader/Budget, unchanged projections and fresh achieved capture. No unresolved source stub or new unmodeled production surface was found.

Scoped Summary QA uses installed GFM/frontmatter formatting, check-before-write on this file only, actual creation timestamp, exact baseline/lifecycle/verbatim requirements, four source hashes and GSD `verify-summary --check-count 4`. Initial/final copies and actual receipts are retained under `plan39/summary-after42/document-QA/`. Separate independent Summary binding must join this actual Summary hash with the unchanged CORE before root acceptance.

## Deviations and Remaining Gates

No further source change was needed for the fresh matrix after the separately accepted physical prerequisites. Preserved failures led to explicit predecessor plans; no original fixture, cap, observation budget, worker deadline or assertion was weakened. Review-only script/index metadata corrections did not rerun application commands or alter evidence outcomes.

The five requirement IDs are copied verbatim for this bounded contribution. CVLD-01, CVLD-02, CVLD-03, CVLD-04 and CVLD-06 remain **globally OPEN**. Root alone accepts Plan 39 and then releases Plan 30's full 17-command matrix: all 16 unique planned commands plus the inherited Plan 36 smoke. These 45 executions do not substitute for that work or complete the phase.

Later Plan 35 first, product/corpus/formal C-D-E work, canonical source review before guard renewal, security, parity/README/manifest, complete native verification, fresh pure-core coverage/Bazel, lifecycle and normal-hook Git gates remain. No differential, hard-size, global-availability, latency or quiescence theorem is claimed. This Summary author ran no application commands and changed no source, shared metadata, guards, manifest or Git state. Root retains source/build ownership; Phase 168 remains unstarted.

## Self-Check: PASSED

All four source files exist and match the current freeze. Nine exact fresh commands exited zero, including the complete original retained-bound case. Original three-body/true-absence inverse and 58 protected-state reconstruction pass. The independent CORE has zero findings. Commits remain disabled; separate Summary binding and root acceptance are pending.
