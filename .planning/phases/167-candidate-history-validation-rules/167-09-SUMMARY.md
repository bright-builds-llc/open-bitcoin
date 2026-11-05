---
phase: 167-candidate-history-validation-rules
plan: "09"
subsystem: network
tags: [rust, canonical-ingress, candidate-history, coins-authority, original-clock]
requires:
  - phase: 167-01
    provides: Immutable selected rules and proved genesis binding
  - phase: 167-03
    provides: Checked structural candidate history and rules
  - phase: 167-04
    provides: Earned header ancestry and retained original admission clocks
  - phase: 167-06
    provides: Independent generation-bound canonical coins admission and selected staging
  - phase: 167-07
    provides: Complete overwrite undo and selected body rules
  - phase: 167-08
    provides: Lossless durable undo codecs and genuine managed persistence foundations
provides:
  - Proved regtest header ingress and explicit contextual import re-admission
  - Exact held-header/original-clock bridge into independently checked coins admission
  - Genuine stored and peer body absorption with accepted-state refusal controls
affects: [167-10, 167-12, 167-17, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Owner-held shell clocks, Independent coins derivation, Existing private body linkage]
key-files:
  created:
    - packages/open-bitcoin-node/src/network/canonical_admission/tests/phase167.rs
    - packages/open-bitcoin-node/src/network/canonical_admission/tests/phase167/fixtures.rs
  modified:
    - packages/open-bitcoin-node/src/network/canonical_admission.rs
    - packages/open-bitcoin-node/src/network/header_sync.rs
    - packages/open-bitcoin-node/src/network/canonical_admission/tests.rs
    - packages/open-bitcoin-node/src/network/validation_history.rs
    - packages/open-bitcoin-node/src/chainstate/validation_history.rs
    - packages/open-bitcoin-network/src/header_store/eligibility.rs
key-decisions:
  - "Qualify only actual same-regtest header and coins owners; selector labels and fixtures remain insufficient."
  - "Use the held original shell clock for eligibility and independent coins admission, preserving duplicate/import observations."
  - "Keep compact candidate refusal evidence distinct from the pre-existing Knots transport fallback parity gap."
patterns-established:
  - "A crate-private exact-header/time bridge delegates to the existing core owner without exposing mutable chainstate."
requirements-completed: [CVLD-02, CVLD-03, CVLD-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-21T23:29:49Z"
commits: false
duration: 27min56s
completed: 2026-10-21
---

# Phase 167 Plan 09: Contextual canonical ingress Summary

**Actual proved-regtest ingress now re-admits headers with shell time, checks full body linkage with the original clock, and reaches genuine coins absorption through the existing private authority owners.**

The existing main-repository checkout baseline is `1b472fb225a290df5cdb06eeea2dc67c624c648e`; this is not a task commit. No staging, commit, push, hook bypass, worktree, shared-state update or Phase168 action occurred. Root retains saving and phase acceptance.

## Files Created/Modified

- Modified: `packages/open-bitcoin-node/src/network/canonical_admission.rs` — qualification using both real owners, contextual imports, mutable private body guards and checked genesis body registration;580lines.
- Modified: `packages/open-bitcoin-node/src/network/header_sync.rs` — fallible independent shell clock capture;89lines.
- Modified: `packages/open-bitcoin-node/src/network/canonical_admission/tests.rs` — existing test-child registration only;614lines.
- Created: `packages/open-bitcoin-node/src/network/canonical_admission/tests/phase167.rs` — all twelve focused behavior tests at their original module paths;612lines.
- Modified: `packages/open-bitcoin-node/src/network/validation_history.rs` — contextual canonical headers and exact held-history/time adapter before coins admission;272lines.
- Modified: `packages/open-bitcoin-node/src/chainstate/validation_history.rs` — narrow crate-private delegation to the existing mutable core admission method;306lines.
- Modified: `packages/open-bitcoin-network/src/header_store/eligibility.rs` — canonical full-genesis and descendant checks before the existing private linkage seal;537lines.
- Created: `packages/open-bitcoin-node/src/network/canonical_admission/tests/phase167/fixtures.rs` — cohesive real-genesis lifecycle, bounded mining, typed snapshot and socket-free compact helpers;231lines.

The ninth declared owner, `packages/open-bitcoin-node/src/network/runtime_authority/validation_history.rs`, was reviewed and remains unchanged:13lines/409bytes. Its existing wrapper already delegates snapshot persistence through the held network. Thus there are nine reviewed Rust owners, eight actually created/modified Rust files, two completed tasks and no artificial edit. This summary and ignored plan09 evidence are additional execution records.

## Completed tasks and actual authority path

Task1 re-admits canonical header inputs. Qualification requires the network selection, actual core coins owner and actual HeaderStore all to identify proved default regtest. The generic ordinary-support predicate is unchanged; Mainnet/Testnet3/Signet keep their existing chain-specific unsupported diagnostics, including missing Signet challenge validation. Merely calling into_canonical_scope on an unearned fixture cannot qualify it.

try_seed_canonical_headers checks roots and actual claimed header identities, derives parent-first ordering from header links rather than supplied height/work, then really calls HeaderStore::readmit_canonical_entries. Its complete validation gates publication. Insertion into a clone of the existing held owner also uses insert_canonical_header, retaining that owner's prior successful clocks, body seals and arrival order. This deliberate second check preserves existing observations; it is not a second rule calculator or a parallel authority type. Header ingress obtains SystemTime in the shell and inserts through contextual admission. Caller timestamps never supply canonical clock evidence. Fixture readmission/seeding cannot overwrite a canonical owner. Foreign root recognition retains precedence over fixture-scope refusal.

Task2 carries the exact admitted candidate into actual coins validation. The network adapter queries its own canonical_header_history and canonical_header_validation_context, checks exact candidate/header/hash/parent/selection and selected context parameters, ends the immutable header borrow, then invokes ManagedChainstate::admit_canonical_body_header. That crate-private method passes only the exact header and held original time into the existing core admit_canonical_header. The core independently derives its own ancestry, immutable rules and generation-bound pending admission. No caller flags, params, arithmetic scores, stored labels or structural envelope mint this authority. Repeated same-candidate guards preserve the original observation; already accepted duplicate bodies need no new admission and still pass body checking.

Canonical descendant eligibility requires this actual store's already admitted exact header and its held history/time. The shared check_block_with_candidate_history runs before the existing record_with_order and private linkage seal. It never calls legacy arithmetic insertion for a canonical descendant. The separate genesis case proves the exact full selected genesis, checks height/work and actual block/header validity, then records root linkage. The node's existing proved-genesis constructor registers its actual proof.block() before publishing that header owner. The pure HeaderStore constructor still creates no body seal; its original test is unchanged. No genesis coin, new ScriptsValid receipt, new proof factory or default network activation is introduced by root linkage.

Existing callers remain unchanged: private guards lead through retain_checked_candidate, body eligibility/binding, authorize_connect, prepare_authorized_connect and genuine managed absorption. Positives exercise stored completion plus general Block, sync Block, durable-serving Block, CompactBlock, actual BlockTxn completion and ReceivedBlock actions. The BlockTxn case first verifies no disposition/progress while reconstruction is partial. The explicit compact transport policy is enabled only in the socket-free test peer; constructor defaults and public networking stay off.

Refusal controls cover wrong version/MTP/difficulty with genuine PoW, caller future-time spoof, held-clock replay with i64::MIN, wrong body for an admitted header, header-only status, fixture reseeding, foreign arithmetic owner, mismatched params, excessive reward despite NONE flags, compact Merkle mismatch, and the retained local/replacement/fixture controls. Typed snapshots compare the complete active chain/exact work, every effective coin, all undo and txid-count fields, coins best block, BASIC processed/safe state and accepted live/durable provenance. Wrong-body/header-only checks run with BASIC both enabled and disabled. The script control matures an actually accepted P2SH coin over100 genuine blocks and verifies mandatory-script-verify-flag-failed at101 despite caller NONE, with the complete accepted snapshot unchanged. No seeded coins or mechanical canonical claims are used.

## Execution interval and scope checkpoints

The original executor stopped at0/2 before source edits when six owners could not access mutable managed coins admission. Root independently checked the managed-bridge amendment adding the two existing providers before release. The later ninth-owner split also waited for independent checking. No planning wait before the fresh continuation is attributed to implementation.

The fresh observable source/check interval is2026-10-21T22:59:33Z through2026-10-21T23:27:29.322Z:27minutes56seconds. It includes the explicitly recorded split-review and compact-semantics review pauses; it is elapsed time, not a claim of continuous coding or active CPU time. Complete logs, old provider bytes, exact split snapshots/inverses and final source hashes are under `.local/open-bitcoin-dev/phase167/plan09/`.

## Actual RED/GREEN and retry history

Tests were added before production correction. Earlier refusals masked later missing providers, so the evidence is progressive rather than a retrospective assertion that every layer was tested simultaneously.

| Receipt                                                      | Actual outcome and attribution                                                                                                                                                                                                                                                                                                                            |
| ------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| red-01-header.log                                            | Exit101;2 discovered, both genuine positives refused unsupported-validation-scope before production header changes. Actual header RED. An unused test import was subsequently removed.                                                                                                                                                                    |
| red-02-eligibility.log                                       | Exit101;2 discovered. Body reached the unchanged eligibility provider and refused time-too-new with caller MIN. The import test separately had a carrier setup error: its forged child omitted its parent. That setup error is not behavior RED; the carrier was corrected by adding the actual genesis row.                                              |
| red-03-eligibility.log                                       | Exit101;2 discovered,1 header positive passed and1 body positive hit unchanged canonical-header-requires-contextual-admission. Caller MAX exposed this next barrier; the original MIN-clock control remains in the final separate replay test. Actual eligibility RED before that provider's correction.                                                  |
| red-04-coins-admission.log                                   | Exit101 at compilation: check_block_with_params does not exist. This is an implementation/setup error, not behavioral RED. Corrected to the existing check_block_header_with_params and check_block after exact genesis proof.                                                                                                                            |
| red-05-coins-admission.log                                   | Exit101;2 discovered,1 passed/1 refused missing-canonical-header-admission on the then-unchanged core bridge. Actual managed admission RED before its provider was added.                                                                                                                                                                                 |
| green-01-stored.log                                          | Exit0;2/2 genuine header/import and stored-body positives.                                                                                                                                                                                                                                                                                                |
| expanded-01.log                                              | Exit0;11/11 including all six peer-route positives.                                                                                                                                                                                                                                                                                                       |
| expanded-02-script.log                                       | Exit101;11 passed/1 assertion error. The script-stage rejection occurred, but formatting HashMaps as Debug text made the test's snapshot comparison order-sensitive. This was a test observation defect, not changed coins acceptance.                                                                                                                    |
| expanded-03-typed-state.log                                  | Exit0;12/12 after using complete typed ChainstateSnapshot equality. The later precise P2SH-reason and compact-GetData assertions had not yet been behavior-tested; this earlier pass is not attributed to them.                                                                                                                                           |
| existing-node-01.log; existing-network-01.log; clippy-01.log | Before-split module28/28, header-store84/84 and strict combined Clippy exit0. These are retained historical checks, not final-source acceptance.                                                                                                                                                                                                          |
| red-06-foreign-fixture-precedence.log                        | After the exact split, exit101;12 discovered,10 passed/2 assertions. Foreign fixture root reproduced unsupported-validation-scope versus established noncanonical-genesis-header. Separately the newly strengthened compact GetData expectation encountered existing bad-txnmrklroot refusal. The precise P2SH script error assertion passed in this run. |
| final-task1.log; final-task2.log                             | Each original prescribed focused command ran on final source and discovered12:12 passed,0 ignored. Foreign-root order is restored; compact control now precisely asserts existing bad-txnmrklroot and preserves all accepted-state/provenance controls.                                                                                                   |

Root reviewed the compact failure against both implementations before authorizing the bounded test expectation correction. The removed new successful-fallback/GetData assertions correspond to real Knots behavior, not an invented or source-incorrect expectation. This correction records an existing transport difference and proves candidate refusal only; see Remaining review below. No production compact code was changed and no earlier receipt or test chronology was rewritten.

## Exact helper split proof

The authoritative frozen parent was832lines/30554bytes, SHA256344417890bb9d1eb2560d301c1b4e297a3f9c7a4c0bc3354501bc5e9ffb0c374. Before moving, all eight source hashes checked against split-basis/owners.sha256 and the new child was absent. The contiguous224-line/7744-byte helper block and591-line/22269-byte twelve-test suffix matched their recorded hashes.

The ignored split.ts moved exactly that block with enumerated pub(super) adaptations on the helper alias/type/functions, a child breadcrumb/import preamble and the parent registrar. AcceptedState fields remain private. Immediate inverse-move.json and inverse-formatted.json both passed byte-exact restoration of the original parent/helper/test suffix, without fallback normalization. The postmove parent was610lines and child231; parent-after-move.rs, child-after-move.rs and after-move.sha256 retain that precise checkpoint. All twelve test module paths stayed unchanged.

Only afterward were the foreign-root assertion, refusal-order correction and source-reviewed compact expectation correction made. The final parent is612lines and child231, both below629. The final augmented test bytes are not represented as equal to the earlier832 basis. No extra owner, compressed style, allowlist, bound, deadline or harness change was used.

## Final commands and source checks

All Cargo commands used pinned Bun1.3.9 and Rust1.94.1, physical process-local TMPDIR from pwd -P, the existing target and the cooperative timing wrapper, serially. Initial IDE artifact-lock waits were observed without killing/reconfiguring the IDE or copying targets. Quiet work was allowed to complete. Every retained final log is complete; the zero-discovery integration harness sections accompanying focused filters are not counted as test evidence.

| Exact command after the required PATH/TMPDIR setup                                                                                                                                                           | Final outcome                                                                                                                                         |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| bun run scripts/command-timings.ts run --key phase167-09-task1 -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase167_canonical_admission                                           | Exit0;12/12; final-task1.log                                                                                                                          |
| bun run scripts/command-timings.ts run --key phase167-09-task2 -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase167_canonical_admission                                           | Exit0;12/12; final-task2.log                                                                                                                          |
| bun run scripts/command-timings.ts run --key phase167-09-existing-node -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node network::canonical_admission::                                 | Exit0;28/28; final-existing-node.log                                                                                                                  |
| bun run scripts/command-timings.ts run --key phase167-09-existing-network -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network header_store::                                           | Exit0;84/84; final-existing-network.log                                                                                                               |
| bun run scripts/command-timings.ts run --key phase167-09-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node -p open-bitcoin-network --all-targets --all-features -- -D warnings | Exit0; final-clippy.log;10.09seconds                                                                                                                  |
| rustfmt --edition 2024 --config skip_children=true --check on all nine owners; git diff --check on all nine owners                                                                                           | Both exit0; final-style-checks.json. Separate new-file no-index whitespace checks emitted no diagnostics; their exit1 denotes added-file differences. |

Owned diff review found only the stated integration, registrations and tests. Final source-closure.json records every declared owner's exact hash/bytes/lines and nine-declared/eight-changed accounting. Actual links include canonical_admission.rs:207 readmit_canonical_entries, network/validation_history.rs:61–62 held history/context and:88 managed bridge, chainstate/validation_history.rs:93 core admit_canonical_header, and header_store/eligibility.rs:171 shared candidate body check. All new Rust files carry pinned-source breadcrumb comments; Plan19 owns manifest registration. No authorizing stub or new unplanned endpoint/schema/auth surface was found.

These are focused/module checks. Plan08's root-accepted2077-pass unfiltered result with four inherited default ignores remains historical08 evidence;09 did not rerun or relabel it, add skips, or claim the whole node suite has zero ignores. Plan27's separate accepted accounting tests likewise remain its own evidence.

## Deviations and simplification

The checked six-to-eight provider amendment resolved the real0-task ownership blocker; the checked ninth test-only owner resolved natural cohesive size. No unapproved owner expansion occurred. [Rule1 - Bug] Restored foreign-root diagnostic precedence after actual RED by moving the existing fixture-refusal block behind the root guard. [Rule3 - Blocking] Corrected the missing-parent test carrier, nonexistent helper call and order-sensitive test observation as explicitly classified above. No production logic was changed to force the compact fallback expectation.

The simplification pass retained one shared pure candidate validator, one existing header lineage owner, the existing body registry/seal and the core coins authority. The managed bridge is a ten-line delegation without a mutable public accessor. The contiguous fixture split removes no test or assertion at its move checkpoint and avoids growing unrelated adapters. Later compact assertion changes are separately disclosed, source-reviewed corrections of the bounded test contract.

## Remaining review and handoff limits

**Pre-existing compact transport parity gap remains open.** Pinned packages/bitcoin-knots/src/blockencodings.cpp:207–212 classifies mutated/Merkle-mismatched reconstruction as READ_STATUS_FAILED; packages/bitcoin-knots/src/net_processing.cpp:3377–3393 requests the full block for the first in-flight download. Existing packages/open-bitcoin-network/src/compact_reconstruction.rs:360–383 fills the body without that mutation check, after which the existing generic eligibility path also rejects its bad Merkle root. This difference predates09 and is outside its production owners. The final09 test establishes exact bad-txnmrklroot refusal and unchanged accepted state, **not Knots compact-fallback or full P2P parity**. Root explicitly retains this finding for scope/closure adjudication before independent17/20 review; it is neither corrected nor silently waived by this summary.

Plan10 retains full direct managed/local/reorg integration, factory/setter policy, durable checkpoint handling and accepted descendant BASIC continuation;09 proves the bounded peer/stored path and BASIC refusal invariants. New descendant reopen proof and product adapters remain their later plan owners. This plan claims no accepted BASIC descendant advancement, canonical Mainnet/Testnet3/Signet history, actual pinned-daemon differential result, unattended sync or public network activation. Full native verification, fresh pure-core coverage, Bazel, independent source/security/goal reports and lifecycle acceptance remain Plan20/root;17 reviews before current guard renewal and19 registers breadcrumbs. Root alone updates STATE/ROADMAP/REQUIREMENTS/config/todo and saves with normal hooks. Stop for root acceptance before10.

## Task commits

None: commits:false. Both tasks' source and evidence are ready for root review; no task or metadata commit was created.

## Self-Check: PASSED

All eight created/modified Rust files exist; the ninth reviewed owner remains unchanged. Final nonzero checks, scoped style review, exact move receipts, source links and final source hashes support the stated bounded work. The originating yolo lifecycle is preserved and the existing checkout baseline is verified. GSD verify-summary with check-count8 reports passed:true and errors:[] with all eight files present.
