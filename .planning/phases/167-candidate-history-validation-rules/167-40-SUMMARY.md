---
phase: 167-candidate-history-validation-rules
plan: "40"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: node-storage
tags: [rust, fjall, journal-pressure, canonical-reopen]
requires:
  - phase: 167-38
    provides: Tracked writers and opaque mutation stamps
  - phase: 167-41
    provides: Complete typed physical observation
  - phase: 167-42
    provides: Accepted work-bounded whole-pair live observation
provides:
  - Earlier native sealed-journal pressure through the existing leased opening
  - Automatic retirement, stale-capability and full-drop safe-reopen evidence
affects: [167-39, 167-30, 167-17, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Existing native maintenance policy, genuine canonical owner, complete map restoration]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/opening/tests.rs
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store/opening.rs
key-decisions:
  - Select native 64 MiB sealed-journal pressure without changing physical limits or writers.
  - Require automatic retirement and complete physical proof before all-owner-drop safe reopen.
  - Preserve earlier failures and verify every command fresh after accepted Plan 42.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-23T09:05:23.012Z"
commits: false
duration: "16m 21.785s for the fresh seven-command matrix; earlier attempts and review are separate"
completed: 2026-10-23
---

# Phase 167 Plan 40: Earlier Native Journal Pressure Summary

**The leased production opening now requests native sealed-journal pressure at 64 MiB, with a genuine workload proving automatic retirement and fresh safe genesis proof after all original owners drop.**

## Final Source Files

Two tasks change two source owners: 11,505 bytes across 311 normally formatted lines. All 60 non-owner inputs, including accepted Plans 41/42 and the original 305-prefix fixture, remain exact.

| Source                                                                | Bytes | Lines | SHA256                                                             |
| --------------------------------------------------------------------- | ----: | ----: | ------------------------------------------------------------------ |
| `packages/open-bitcoin-node/src/storage/fjall_store/opening.rs`       |  2614 |    60 | `91578210715748dc8a8265cb2be2166ced3f9e7b7caa0e97c8a54d23330f1f79` |
| `packages/open-bitcoin-node/src/storage/fjall_store/opening/tests.rs` |  8891 |   251 | `7bcc59bdcb628e52323abdd6d5dbf226be23791a8b696c1e1bef47a72af0484a` |

## Bounded Implementation

The existing `fjall::Database::builder(lease.source())` chain adds `max_journaling_size(64 * 1024 * 1024)` before the same open and tracked wrapper. The operation lease, error mapping and nine namespace constructions stay intact. The only other parent change is the private test registrar. No dependency, public authority API, raw constructor or writer algorithm was added.

Pinned Fjall 3.1.10 (`aa30dca811399a201e0b9595da93a4582dcb2b57`) accepts 67,108,864 bytes as its minimum. This requests asynchronous pressure on **sealed journals**. Active journal preallocation, tables and other files still add bytes; this is not a hard datadir bound, synchronous checkpoint or quiescence guarantee. Complete physical inspection retains its 512 MiB, 4,096-entry, depth-16 and 4,096-byte path limits.

The test uses the genuine canonical-genesis factory, keeps the original schema rows cold, and releases observation snapshots before pressure. One held metrics clone performs 4,096 same-key inserts using one reused 65,536-byte value: 268,435,456 cumulative payload bytes. Real journal identities are observed without WAL decoding, forced rotation, flush, compaction or a policy getter. The existing 30-second retirement observation bound is unchanged.

## Actual Pressure and Reopen Outcome

After accepted Plan 42, the unchanged pressure test passes in 17.57 test seconds. After all writes, retirement is confirmed at the 1,575 ms observation, reporting 134,564,701 regular-file bytes, 83 entries and depth 4. Those are that observation's metadata, not the exact retirement instant or a bound on every later tree state.

The full physical gate records three actual safe Changed results, then Ok on attempt four. The receipt reaches all nine maps/catalog restored, cold schema equal, old stamp stale and old seal rejected. Every original owner then drops before public safe reopen. Fresh proof, fresh seal, exact selected genesis position/work and all-nine equality pass; reopened owners drop before explicit cleanup, which is reached. No whole command is retried to obtain this result.

## Fresh Verification

All seven original commands ran serially on one immutable freeze of 62 source inputs. Each has 62 actual before and 62 after copies: **868 current command copies**. The test selections pass **86 overlapping executions**, with zero failed or ignored cases; these are not 86 unique tests.

| Timing key                      | Named passes | Test seconds | Command seconds | Exit |
| ------------------------------- | -----------: | -----------: | --------------: | ---: |
| `phase167-40-pressure`          |            1 |        17.57 |          18.169 |    0 |
| `phase167-40-canonical-genesis` |           29 |       564.54 |         565.016 |    0 |
| `phase167-40-generation`        |           12 |        11.11 |          11.565 |    0 |
| `phase167-40-seals`             |            5 |       163.72 |         164.092 |    0 |
| `phase167-40-native-reader`     |           39 |       219.36 |         219.855 |    0 |
| `phase167-40-format`            | format check |          n/a |           2.134 |    0 |
| `phase167-40-clippy`            |  strict lint |          n/a |           0.482 |    0 |

The current matrix spans 2026-10-23T08:29:57.073Z through 08:46:18.858Z: 981.785 seconds, excluding earlier implementation, failures and review. Commands retain pinned Bun 1.3.9, Rust 1.94.1, original physical TMPDIR/default Cargo target and cooperative timing wrapper. Format is workspace check-only; node Clippy retains all targets, all features and warnings denied. Source copies remain exact and command intervals do not overlap. The 201 native/lock inputs authenticate separately, without a per-job engine-copy claim.

## Preserved Failure History

| Historical receipt                       | Actual result                                           | Reached boundary                                                                                                                                                                                                                  |
| ---------------------------------------- | ------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Original default-512 MiB RED, `20a188e7` | 0 passed / 1 failed; 58.91 s; historical 55-input frame | Journal retained at the 30-second observation bound. Restoration, stale checks, physical proof, drops and explicit cleanup precede the expected retirement assertion failure. Safe reopen is skipped because retirement is false. |
| First 64 MiB final, `4f8084d8`           | 0 passed / 1 failed; 2.38 s; historical 55-input frame  | Retirement succeeds, then the physical gate refuses a changed source. Safe reopen and explicit cleanup are not reached.                                                                                                           |
| Fresh run after Plan 41, `8ce505a8`      | 0 passed / 1 failed; 2.37 s; historical 62-input frame  | Two actual directory changes cause the prescribed fixed-two refusal. Safe reopen and explicit cleanup are not reached; six later commands never start.                                                                            |

These receipts and source scopes remain immutable. Panic unwinding is not credited as explicit cleanup or safe reopen. No combined historical job/copy total is inferred. Accepted Plan 42 supplies a separately reviewed protocol prerequisite; it neither rewrites those outcomes nor substitutes for this new matrix.

## Evidence, Inverse and Independent Review

Current execution receipts are under `.local/open-bitcoin-dev/phase167/after42-preparation/actual-runs/accepted42-20261023/`:

| Artifact                                         | SHA256                                                             |
| ------------------------------------------------ | ------------------------------------------------------------------ |
| Common `freeze/result.json`                      | `38374f2caee303006c4f75e960cad7381edc5d6bdd2fdf3cd32db9e80c7512ff` |
| `plan40/execution/final-index.json`              | `f27c50030441e74fbaed76fc940100e858537c45d2999070fa82b4efcc9ab876` |
| `plan40/inverse-audit/result.json`               | `be7c9ed7ae9120da2b45bc6610131dd43b51df4f8512aba6ae145ba1a93a8022` |
| `plan40/root-matrix-inverse-authentication.json` | `b03611b9b83862e98f457a95ecf7546187259456660c46f6e8f476506f7b2220` |

The pressure result is `492c1780`, with raw log `2b7f05e1`. Detailed current and historical receipts remain indexed by the independent CORE packet.

Original inverse authority is `plan40/root-before/capture.json`, SHA256 `30cfc9debfbdcbe43a47b3c39468b43661e35c0f44c1561fd52fc4dbdbd5b927`. Its exact 56-line opening body (`7c674c87`) and true test-child absence are composed with 60 current protected inputs. Strict BSD patch at `-F0` restores that body and absence, then reproduces all 62 final bodies without offset or fuzz. The inverse endpoint has 61 present files and one absent child; live source is not patched. The independent reviewer reproduces this proof, receipt `c9656605`.

The whole immutable CORE was read: `.local/open-bitcoin-dev/phase167/plan40/independent-review/after42/CORE.md`, 14,199 bytes, SHA256 `d78dbc00a869f68ed15bae92c9da8eea7bba62f729a2c212fc3553ab08b64cc2`. It reports zero source, API, security, parity, evidence or inverse findings. Its index (`a18055cf`) authenticates supporting receipts and has no Summary dependency. Separate independent Summary QA must bind this Summary's actual hash and that unchanged CORE before root acceptance.

Scoped document QA uses installed GFM/frontmatter formatting, check-before-write on this file only, exact lifecycle/baseline/verbatim requirements, both source hashes and GSD `verify-summary --check-count 2`. Initial/final copies and receipts are retained in `plan40/summary-after42/document-QA/`. No application test or build was run by this Summary author.

## Deviations and Remaining Gates

No further source change is required after the separately checked Plan 41/42 prerequisites. The preserved physical-race failures led to those explicit predecessor plans; no workload, assertion, resource limit or observation deadline was weakened. The simplification/security pass retains one existing native setting and a cohesive test child. No unresolved stub or new unmodeled production surface was found.

The five requirement IDs are copied verbatim for this bounded contribution. CVLD-01, CVLD-02, CVLD-03, CVLD-04 and CVLD-06 remain **globally OPEN**. Root alone accepts Plan 40 after document binding. Plan 39 still requires all nine commands, including the unchanged original 305/306 contract, and Plan 30 requires its complete later matrix. This result does not establish their acceptance or whole-phase, differential, hard-size, scheduling or quiescence guarantees.

Canonical source review, security, parity/README/manifest, complete native verification, fresh core coverage/Bazel, lifecycle and normal-hook Git gates remain. No per-task commit, shared metadata, guard, manifest or Git write was made by this Summary author. Root retains source/build ownership and Phase 168 remains unstarted. No external user setup is required.

## Self-Check: PASSED

Both final source files exist and match the common freeze. Seven exact fresh commands exited zero; original-body/true-absence inverse and protected-state forward reconstruction pass. The independent CORE has zero findings. Commits remain disabled; separate Summary binding and root acceptance are pending.
