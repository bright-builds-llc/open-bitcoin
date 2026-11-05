---
phase: 167-candidate-history-validation-rules
plan: "34"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: network-node-sync
tags: [rust, compact-blocks, request-provenance, canonical-context, runtime-accounting]
requires:
  - phase: 167-31
    provides: Accepted serialized-script semantics and source-stable execution prerequisite
provides:
  - Actual ordered block request registration with coherent capacity and scoped cleanup
  - Explicit canonical and fixture compact adapters retaining selected rule facts
  - Intact runtime fallback delivery with atomic global accounting and membership-aware receipt/teardown cleanup
affects: [167-17, 167-19, 167-20, 167-25, future-compact-B-E]
tech-stack:
  added: []
  patterns: [Private request registry, earned canonical context, atomic actual-packet handoff]
key-files:
  created:
    - packages/open-bitcoin-network/src/peer/block_requests.rs
    - packages/open-bitcoin-network/src/peer/tests/block_request_order_cases.rs
  modified:
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/peer/inventory_state.rs
    - packages/open-bitcoin-network/src/peer/message_dispatch.rs
    - packages/open-bitcoin-network/src/peer/compact_download_state.rs
    - packages/open-bitcoin-network/src/peer/tests.rs
    - packages/open-bitcoin-network/src/peer/tests/block_sync_lifecycle_cases.rs
    - packages/open-bitcoin-node/src/network/compact_receive_candidates.rs
    - packages/open-bitcoin-network/src/peer/tests/compact_download_lifecycle_cases.rs
    - packages/open-bitcoin-network/src/peer/tests/compact_download_fallback_cases.rs
    - packages/open-bitcoin-network/src/peer/tests/transaction_lifecycle_cases.rs
    - packages/open-bitcoin-node/src/network/canonical_admission/tests/fixtures.rs
    - packages/open-bitcoin-node/src/network/tests/compact_announcement.rs
    - packages/open-bitcoin-node/src/sync/session.rs
    - packages/open-bitcoin-node/src/sync/block_reconcile.rs
    - packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases/compact_timeout.rs
key-decisions:
  - Keep ordinary-origin provenance distinct from compact attachment; preserve real registration order on attachment and resend.
  - Derive canonical compact rules from the exact admitted header and held context; explicit fixture data cannot confer authority.
  - Consume authorized runtime packets directly before fresh scheduling and release global hashes only after actual all-peer membership checks.
requirements-completed: [CVLD-01, CVLD-03, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-22T14:26:38.476Z"
commits: false
duration: "71m 46.442s in two measured source/execution windows through final static closure"
completed: 2026-10-22
---

# Phase 167 Plan 34: Ordered Compact Provenance and Runtime Handoff Summary

**Actual request insertion order, earned canonical compact rules, and intact timeout packet delivery now share bounded, coherent request accounting.**

## Final Source Files

These are all seventeen source owners. The first fourteen remain byte-exact to their preserved Task 4 execution freeze; Task 5 changes only the last three. Every source is normally formatted and at most 628 physical lines. The largest is 626.

| Source                                                                              | Lines | SHA256                                                           |
| ----------------------------------------------------------------------------------- | ----: | ---------------------------------------------------------------- |
| `packages/open-bitcoin-network/src/peer.rs`                                         |   626 | d676c692eb6cbd48c471fbfead1f94f58a968a23915e9949f3a768748506a0d6 |
| `packages/open-bitcoin-network/src/peer/block_requests.rs`                          |   179 | 423fdb5097e7cff36cc5f9598e93a0acdc63f8f199d1646fc25e7aca653e71da |
| `packages/open-bitcoin-network/src/peer/inventory_state.rs`                         |   535 | a3f1a857052e82442b39b65f004e9cfb928de175d9e24a348dac9222eca52801 |
| `packages/open-bitcoin-network/src/peer/message_dispatch.rs`                        |   493 | 1213345a5009dcebc5533b1b2b0a1cfcac0aff7abe5ad96ef3bc9a6a4cf4bbc2 |
| `packages/open-bitcoin-network/src/peer/compact_download_state.rs`                  |   393 | 3ca6b781965e8221fd37f1cc463f59f4653ba430714ec740c67b1c688f1fcaf5 |
| `packages/open-bitcoin-network/src/peer/tests.rs`                                   |   193 | daab36e66dcdc8f77fcf657eabf996a4ed4ece82a471580e95b59236ceb181c3 |
| `packages/open-bitcoin-network/src/peer/tests/block_request_order_cases.rs`         |   596 | a8ea2e37619947eba5e0317d118b557bca400aa570a5cc49ed53831c9c42fede |
| `packages/open-bitcoin-network/src/peer/tests/block_sync_lifecycle_cases.rs`        |   571 | 210380de3bd26f6a1bf153bb7189445c5e3251f72fdc809069652d3e30a4553e |
| `packages/open-bitcoin-node/src/network/compact_receive_candidates.rs`              |   575 | 5f372ac15198e183083d0d0fbb6a176b02cea876f594712ff92f1a1bd0d96bee |
| `packages/open-bitcoin-network/src/peer/tests/compact_download_lifecycle_cases.rs`  |   480 | 45b5e48a3b735473f4115e67c4ef5707d02b7831418763f71bf8861919e8e172 |
| `packages/open-bitcoin-network/src/peer/tests/compact_download_fallback_cases.rs`   |   227 | aa65e1d9d05afd2c4baf67ceac62b894ab246371aeca5e4f23721672f584ec77 |
| `packages/open-bitcoin-network/src/peer/tests/transaction_lifecycle_cases.rs`       |   597 | 70d318f91c005daca97771527b82de6539c007dbac2346d52f6633ef9992024c |
| `packages/open-bitcoin-node/src/network/canonical_admission/tests/fixtures.rs`      |   316 | ab74921266e5d6c8e380c272eb4b44707aeb30fc6693d0bd273249d042336240 |
| `packages/open-bitcoin-node/src/network/tests/compact_announcement.rs`              |   437 | ac6bcc533c38ab925cc2d3239be55c38ade880a7e49c7c801923831f18ea1144 |
| `packages/open-bitcoin-node/src/sync/session.rs`                                    |   624 | 7615b9517f19ff06c3e862a949c77b79c6e9f52d1e0154f92cde54ae7e433e7f |
| `packages/open-bitcoin-node/src/sync/block_reconcile.rs`                            |   618 | d7f888dfc42b60beaf32444676bd35673fce5c4924ea753af6f3cd423d65ea27 |
| `packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases/compact_timeout.rs` |   577 | 3da8c81ceab08c5af164701e58f4ea7897e994742be551ce277c07996e437517 |

## Bounded Outcome

All five checked tasks are implemented. The requirements field records this plan's bounded contribution, following the existing phase summary convention; it does not close shared requirements or accept the phase. The baseline commit identifies existing history. No task commits, staging, push, source-guard renewal, shared metadata, metrics or manifest updates were performed by this executor.

Task 1 records committed explicit requests in real peer arrival order, independent of numeric IDs or clocks. It updates membership, registry and scoped partial release together; duplicate registration preserves position, and removal followed by enrollment appends after surviving requesters. The parent receives exactly its three projected lines,623→626, with no additional exports.

Task 2 registers actual compact requests, immediate full fallbacks and retained timeout resends under the existing configured/Phase94 limits. New requests suppress at capacity; attachment/resend of an existing peer/hash uses no extra slot. Compact-only cleanup preserves ordinary requests. Original structural completion, negotiated capability, strict age>60 and backward-clock behavior remain intact. All three original peer fixture files and the new test callers use explicit fixture activation data.

Task 3 makes production compact entry canonical-only, requiring exact already-admitted header bytes, CandidateHistory rules and the held validation context. Node checks its admission scope, actual core owner and HeaderStore binding; any canonical binding prevents fixture fallback. Unbound fixtures require already-selected header parameters. Both original Node fixture caller files migrate explicitly. BlockTxn requires actual requested membership, matching partial data and retained selected rules. Actual external Node calls and a discovered no_run doctest prove inherent API reachability without another export or source owner.

Task 4 closes header-driven registration and tests all six explicit/header/compact producer permutations with independent expected peer sequences. It preserves unrelated peers/hashes through compact cleanup, NotFound, disconnect and reenrollment. Existing request, compact, transaction, canonical and announcement regressions remain intact.

Task 5 delivers actual authorized packet batches from Idle and Message wakes before ordinary fresh scheduling. It preflights target identity, retained membership and unique new runtime hashes for the whole batch before accounting or delivery. Existing hashes resend at capacity; novel overflow returns typed ResourceLimit and real teardown cancels the retained obligation. Post-manager receipt cleanup runs on success and error, and teardown removes only hashes without any remaining actual requester. Same-hash other-peer requests survive NotFound, unrequested Block and disconnect. Primary receive and cleanup errors remain visible. The legacy raw-counter helper retains its exact logic/signature under cfg(test) solely for its existing phase 110 fixture.

## Final Checks and Evidence

All twenty exact planned commands below passed on the renewed seventeen-source freeze with nonzero selected test discovery, no failing or ignored cases, and strict all-target/all-feature combined Clippy. Compiler/lint runs do not claim test discovery. Counts from overlapping selections are not distinct coverage.

|   # | Exact planned command                                                                                                                                                                                                                               |  Passed cases | Seconds |
| --: | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------: | ------: |
|   1 | `bun run scripts/command-timings.ts run --key phase167-34-order -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase167_compact_provenance_order -- --test-threads=1`                                                    |             4 |   0.353 |
|   2 | `bun run scripts/command-timings.ts run --key phase167-34-request-regressions -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network block_sync_request -- --test-threads=1`                                                     |            23 |   0.192 |
|   3 | `bun run scripts/command-timings.ts run --key phase167-34-compact -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase167_compact_provenance_compact -- --test-threads=1`                                                |             4 |   0.220 |
|   4 | `bun run scripts/command-timings.ts run --key phase167-34-compact-regressions -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network compact_download -- --test-threads=1`                                                       |            50 |   0.158 |
|   5 | `bun run scripts/command-timings.ts run --key phase167-34-transaction-regressions -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network transaction_lifecycle -- --test-threads=1`                                              |            30 |   0.268 |
|   6 | `bun run scripts/command-timings.ts run --key phase167-34-context -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network -p open-bitcoin-node phase167_compact_provenance_context -- --test-threads=1`                           |             7 |  15.804 |
|   7 | `bun run scripts/command-timings.ts run --key phase167-34-api -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --doc fixture_compact`                                                                                      |             1 |  32.373 |
|   8 | `bun run scripts/command-timings.ts run --key phase167-34-canonical-regressions -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node canonical_admission -- --test-threads=1`                                                     |            30 |  46.489 |
|   9 | `bun run scripts/command-timings.ts run --key phase167-34-announcement-regressions -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node compact_announcement -- --test-threads=1`                                                 |            11 |   0.332 |
|  10 | `bun run scripts/command-timings.ts run --key phase167-34-headers -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase167_compact_provenance_headers -- --test-threads=1`                                                |             2 |   0.158 |
|  11 | `bun run scripts/command-timings.ts run --key phase167-34-lifecycle -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network block_sync_lifecycle -- --test-threads=1`                                                             |            14 |   0.152 |
|  12 | `bun run scripts/command-timings.ts run --key phase167-34-final-matrix -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network -p open-bitcoin-node phase167_compact_provenance -- --test-threads=1`                              |            17 |   6.979 |
|  13 | `bun run scripts/command-timings.ts run --key phase167-34-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network -p open-bitcoin-node --all-targets --all-features -- -D warnings`                                      | compiler/lint |   9.045 |
|  14 | `bun run scripts/command-timings.ts run --key phase167-34-runtime-handoff -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase167_compact_runtime_handoff -- --test-threads=1`                                              |             7 |   7.813 |
|  15 | `bun run scripts/command-timings.ts run --key phase167-34-runtime-old-cadence -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase123_compact_download_survives_five_second_idle_cadence_until_timeout -- --test-threads=1` |             1 |   0.993 |
|  16 | `bun run scripts/command-timings.ts run --key phase167-34-runtime-old-response -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase123_compact_timeout_fallback_consumes_matching_block_before_yield -- --test-threads=1`   |             1 |   0.920 |
|  17 | `bun run scripts/command-timings.ts run --key phase167-34-runtime-old-message -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase123_message_after_idle_uses_session_clock_for_compact_timeout -- --test-threads=1`        |             1 |   1.003 |
|  18 | `bun run scripts/command-timings.ts run --key phase167-34-runtime-node-compact -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node compact -- --test-threads=1`                                                                  |            89 |  19.691 |
|  19 | `bun run scripts/command-timings.ts run --key phase167-34-runtime-timing -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node runtime_timing_cases -- --test-threads=1`                                                           |            21 |  16.079 |
|  20 | `bun run scripts/command-timings.ts run --key phase167-34-runtime-network-all -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-features -- --test-threads=1`                                                         |           694 |  26.912 |

The complete network run contains 688 unit cases, 2 parity cases, 2 property cases and 2 doctests. The broad Node compact selection passes 89 cases, and runtime timing passes 21, including the three original regression bodies unchanged. The response regression again reaches 19 clock reads. Task 5's seven new cases use actual manager registrations, timing transport, runtime accounting and operational errors, including competing fresh scheduling and error-after-release.

The final freeze is `.local/open-bitcoin-dev/phase167/plan34/execution-v2/freeze-2026-10-22T14-14-02.169Z/sources.json`, SHA256 e837827b398ca109ca8f2a474304ac7b1c44e25ab686c4e3a390ec2730409403. Both actual forward and inverse BSD patch -E -F0 -p1 reproductions pass. The two original new files invert to real absence using /dev/null; no manufactured empty baseline exists. All source copies, physical regular-file paths, patches and proof logs are retained beside that manifest.

The final twenty-command manifest is `.local/open-bitcoin-dev/phase167/plan34/execution-v2/final-commands-2026-10-22T14-14-20.313Z.json`. The source/build evidence index is `.local/open-bitcoin-dev/phase167/plan34/execution-v2/evidence-index-2026-10-22T14-18-38.066Z.json`, SHA256 e220e8397a02ad9e25c2401799897ff19a53c65012699213f5bcf8edf13142e0. It authenticates 79 completed receipts, raw log hashes, started arguments, UTC timing and each job's actual owned/external before-and-after fences. Historical14-owner and later17-owner jobs retain their real fence counts; no new owner was attributed retroactively. Current jobs fence 17 owned sources and 18 external references, including the unowned idle test, raw legacy fixture and both pending Phase 123 guard paths. The final static receipt additionally proves original 14, original 163-line test prefix, protected guards, formatting and clean diff.

Execution used pinned Bun 1.3.9, Rust 1.94.1, the physical TMPDIR and the existing serialized target through command-timings. No target copy, cache maintenance, signal, deadline reset or cap increase was used. Measured command time through source/static closure is 1373.707s; the final twenty commands take 185.934s. Execution windows are 12:25:28.117–13:06:49.000 UTC and 13:48:12.436–14:18:37.995 UTC, totaling 71m46.442s. The separate root planning hold is 13:06:49.000–13:48:12.436,41m23.436s. Earlier read-only preparation and later document QA are outside those source/execution metrics; the first window includes the brief root scheduling pause for cache preflight, without any maintenance.

## Preserved RED, Setup and Lint History

| Observation                            | Actual result and correction                                                                                                                                                                             |
| -------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Initial ordinary cleanup RED           | 2 passed / 1 failed,93.874s: NotFound retained the sender's partial.                                                                                                                                     |
| Actual old-state order RED             | 2 passed / 2 failed,447.574s: observed [2,7,90] instead of independent arrival [90,2,7], plus the partial release failure. A rustc sample showed dependency ReadDir enumeration; it recovered naturally. |
| Compact registration/cap RED           | 0 passed / 3 failed,4.847s: untracked compact/timeout requests and a new request bypassing capacity.                                                                                                     |
| Context test setup                     | 17.910s compiler failure: missing message import and unsupported Clone assumptions for real durable owners. Corrected only owned tests to construct actual owners and retain physical-store cleanup.     |
| Canonical/fixture context RED          | Network0pass/3fail,36.964s; separate Node0pass/2fail,21.412s, exposing all eight old binding paths.                                                                                                      |
| Requested membership RED               | 4 passed / 1 failed,5.066s: BlockTxn accepted a partial after its actual requested membership was removed.                                                                                               |
| Header producer RED                    | 0 passed / 2 failed,5.696s: emitted header requests were absent from the order registry.                                                                                                                 |
| Original broad Node compact regression | 79 passed / 3 failed,14.332s. The three timeout/runtime failures are preserved, together with WR-01; focused passes did not waive this result.                                                           |
| Renewed Task 5 old-test RED            | Each unchanged original case failed with NEW17-source/18-external fences:1.468s,1.320s and1.430s.                                                                                                        |
| New Message-wake RED                   | 0 passed / 1 failed,15.263s: session yielded at 6 clock reads instead of 8. The later transport control also checks real manager state before a competing body response.                                 |
| New Task 5 test setup                  | 11.604s compiler failure from two unqualified test-only return types; corrected to existing qualified types.                                                                                             |
| New error fixture expectation          | 6 passed / 1 failed,22.250s: unknown-peer Ping is nonfatal. Corrected the test to obtain a genuine invalid-body receive error and genuine unknown-peer cleanup error.                                    |
| Historical0356 final batch             | First 12 commands passed; strict Clippy rejected err().expect in the new test,9.574s. Root authorized the exact expect_err correction; all 17 were re-frozen and all 20 commands reran successfully.     |

Two ignored freeze-runner adaptation errors are also retained: the first read the old 14-owner manifest and correctly failed the17-owner assertion; the next used the old output template. Its new timestamped proofs remain, and the mutable historical pointer was archived then restored byte-for-byte before the corrected v2 proof. No historical source freeze, runner, command receipt or raw log was overwritten. Static notes and the pointer repair script are retained in execution-v2.

## Checked Refinement and Decisions

The initial14-owner/four-task implementation exposed a real consumer defect: the Idle runtime discarded an already-authorized GetData packet and re-requested its hash, so correct deduplication prevented transport delivery and runtime occupancy. Root held execution, independently checked the runtime refinement, and released exactly three additional owners as Task 5. The original four task XML blocks/thirteen commands and the original 14-source freeze remain unchanged. No speculative resend policy or fake ordinary provenance was added.

Local guidance materially informing the work includes AGENTS.md's command-timing lock, actual parity breadcrumbs and physical-target policy; the Bright Builds sidecar and architecture/code-shape/testing/verification/Rust standards; and the narrow standards override retaining the unrelated existing fixture panic only. Active global/repository lessons were read completely at 8856 bytes/2953 estimated tokens. No new dependency or public authority carrier was introduced. Final stub scanning found no placeholder/TODO/FIXME output path; intentional empty fixture candidate slices are explicit test data.

Independent final seventeen-source review is clean and closes WR-01, recorded in root-owned source-review/runtime-closure-v2/REVIEW.md (SHA256 61f2c67f6d85b7c8405b2c589b97f1581884afec272d75096231119ac4718ca9). The review records actual error-join coverage without claiming a physical snapshot-failure injection.

## Remaining Mandatory Gates

This is compact unit A, with the checked actual consumer closure. Pure reconstruction remains structural. Full mutation validation, first/later retry adjudication, consumed lifetime, selected full completion, service-fetch flag parity, successful stored-and-eligible all-request cleanup, pinned batch timer/16-per-peer/3-per-hash/optimistic scheduling and broad differential qualification are not delivered by this plan. B–E still require actual32/30 reconciliation and separately checked work before final17 acceptance.

The Phase 123 checker/test currently pin the replaced lossy consumer shape. They were preserved as explicit external guards. Root must formally assign and reconcile them after independent actual-source adjudication and before Plan 17 renewal and default native/Plan 20 acceptance; this is no stale-guard waiver. Source breadcrumbs/manifest/docs reconciliation remains with 19. Default native verification, fresh pure-core coverage/Bazel, security/goal/lifecycle, compact/Git gates and all phase/global requirement acceptance remain mandatory with root. Phase 168 remains unstarted.

Root independently authenticates the current final batch and source fences in root-final-evidence-audit-v3.json (SHA256 22b41b38a815e6f6b10972ee902f0c568308992f62daeec62a69580fa44f4a64), and all 79 indexed source/build receipts in root-history-index-audit-v3.json (SHA256 81cbf60d752602528be84ff736f34a50fbf6cd0046be1820537ef05ab0cbc913). Later document QA is separately timestamped and does not change that immutable source/build cutoff. An ignored draft-generator backtick escaping error and consequent missing-draft formatter check are retained as document setup history; only the ignored generator was corrected, and its parser error was captured separately. Final Summary formatting uses the installed GFM/frontmatter extensions.

## Self-Check: PASSED

All 17 source files, frozen hashes, normal line bounds, both patch directions, protected originals and twenty final commands were checked. The existing baseline Git object is preserved. Relative summary verification found all 17 source paths and passed the self-check; scoped GFM/frontmatter formatting passed.
