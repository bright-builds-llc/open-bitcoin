---
phase: 167-candidate-history-validation-rules
plan: "50"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: compact-request-lifetime-and-header-admission
tags: [rust, compact-block, networking, canonical-header, chainstate, evidence]
requires:
  - phase: 167-49
    provides: Accepted selected compact completion and explicit witness and inventory facts
provides:
  - Arrival-ordered bounded First/Later/Absent compact failure and consumed lifetime
  - Full-fetch inventory selected from the peer's actual remote WITNESS service
  - Actual canonical compact-header admission before enrollment and reconstruction
  - Managed negative and conservation controls for header, request and terminal paths
affects: [167-51, 167-17, 167-18, 167-19, 167-20, CVLD]
tech-stack:
  added: []
  patterns: [one ordered request registry, service-selected full fetch, existing detailed HeaderStore admission]
key-files:
  created:
    - packages/open-bitcoin-network/src/peer/tests/compact_retry_cases.rs
    - packages/open-bitcoin-node/src/network/canonical_admission/tests/phase167/compact_reconstruction.rs
  modified:
    - packages/open-bitcoin-network/src/peer/block_requests.rs
    - packages/open-bitcoin-network/src/peer/compact_download_state.rs
    - packages/open-bitcoin-node/src/network/validation_history.rs
    - packages/open-bitcoin-node/src/network/compact_receive_candidates.rs
    - packages/open-bitcoin-node/src/network/canonical_admission/tests/phase167.rs
    - packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs
    - packages/open-bitcoin-node/src/sync/tests/peer_resolution/peer_protocol_failures/lifecycle.rs
key-decisions:
  - Keep First/Later/Absent and Active/Consumed/original start inside the existing bounded request vector.
  - Preserve the existing pre-dispatch guard error before UnknownPeer for malformed or foreign-owner inputs; use UnknownPeer first within guard-passing compact receive.
  - Expand Task 3 by three exact test-only owners after real failures, without widening production ownership or accepting either inventory type indiscriminately.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
requirements_scope: bounded-plan50-contribution-only
global_cvld_acceptance: open
generated_by: gsd-execute-plan
generated_at: "2026-10-30T03:36:18Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
status: canonical-summary-pending-independent-qa-and-root-final50
---

# Phase 167 Plan 50: Ordered compact retry and canonical header admission

Plan 50 now derives compact failure order from the actual arrival-ordered request vector. A First requester retains its bounded request, rules and original start while sending one full-block fetch after semantic failure or timeout. A Later requester releases only its own request and sends none; a genuinely Absent requester must first pass the existing registration cap. Consumed compact attempts cannot silently restart on reannouncement or resend fallback after strict age `>60` expiry. Matching BlockTxn after consumption is duplicate misbehavior with sender cleanup; unrelated stray BlockTxn remains silent. Full-fetch `WitnessBlock` versus `Block` follows the peer's actual Version WITNESS service, independently of compact protocol version.

The managed CompactBlock path checks real peer, capability, owner and selected context before new header effects. For a genuinely new canonical header, it calls the existing detailed HeaderStore admission with the shell-observed clock and real linkage, PoW, difficulty, version, MTP and future-time checks before request enrollment or reconstruction. KnownOnly identity and snapshot publication remain separate fallible steps. An already admitted exact header keeps its earned context. The pre-dispatch `guard_message_admission` can reject malformed genesis or foreign owner first; within inputs that pass it, UnknownPeer precedes new header effects. This is a bounded Plan 50 contribution. Plan 51 still owns final matching-request completion after actual stored eligible body and final public success; all five global CVLD requirements remain open.

## Test-first evidence and checked correction

D1's compiling behavioral RED discovered two named tests: an unchanged First control passed and the Later collision failed because the old manager sent GetData. The corrected selected path passed both, then the final Task 1 selector passed **6/6**, including many-distinct-hash consumed churn, cap, cleanup and genuine re-enrollment. D4's compiling behavioral RED discovered one named unknown-valid-Regtest-header case and failed at `missing-canonical-compact-context`; its first GREEN attempt had a missing `BlockHeader` import and earned no behavioral credit. The corrected D4 path passed, and the final Task 4 selector passed **5/5** with actual managed negatives for guard-passing UnknownPeer, guard-first malformed/owner cases, disabled/incapable no-effect and invalid parent, difficulty, PoW, version, MTP and future-time headers. Tests compare accepted state, KnownOnly, requests and stored effects at the appropriate boundary.

Task 2's final peer lifecycle/fallback selector passed **64/64**. Task 3's final broad node compact selector passed **97/97**, with the new header tests also matching that filter. A focused peer lifecycle selector passed 1/1 and runtime timing controls passed 21/21 during correction. The original Task 3 run passed 83 and failed ten old assertions; subsequent 87/6 and 92/1 runs exposed three test-only owners outside the original 13-file plan. ROOT checked and released those exact additions, raising Plan 50 to 16 direct owners (14 original bodies and two true-absent children). The five timing failures came from a Block-only helper under a real default WITNESS handshake; it now asserts the exact WitnessBlock type and hash. The peer-resolution controls now retain the First request until terminal cleanup and distinguish First full-Block completion from Later no-fetch conservation across all 12 cases. None of those test failures is credited as a D1 or D4 behavioral RED.

Independent source review initially failed on three missing direct controls: managed compact-header refusal cases, no-acceptance after send-triggered disconnect, and bounded consumed-hash churn. A separate three-test-owner correction added those controls without changing production source or weakening old assertions. All four literal task commands then passed at 6/64/97/5, and independent source re-review closed all three findings. The first three final format/strict/build commands before that correction remain diagnostic, not final credit. The checked [guard order](167-50-PREDISPATCH-GUARD-PRECEDENCE-AMENDMENT.md), [Task 3 owner expansion](167-50-TASK3-TEST-OWNER-EXPANSION-AMENDMENT.md), [terminal test](167-50-TASK3-FULL-BLOCK-TERMINAL-TEST-AMENDMENT.md), [First/Later matrix](167-50-TASK3-FIRST-LATER-MATRIX-AMENDMENT.md), and [source-review correction](167-50-SOURCE-REVIEW-COVERAGE-CORRECTION-AMENDMENT.md) preserve the causal history.

## Final frozen verification

On one fresh frozen frame, all **16 published final commands** ran serially from format and exited zero. Format, strict Clippy and build passed. Every test selector discovered named behavior; the overlapping selectors recorded **3,491 passing tests** in total, with no failed test. The full node-wide command passed **2,408 tests** across its four reported groups (2,377 + 15 + 3 + 13), with four explicitly ignored cases for public-network opt-in, timing evidence or subprocess control. It took 13,004,058 ms (about 3h36m44s). The [CORE](../../../.local/open-bitcoin-dev/phase167/plan50/execution-v1/CORE.json) and [test-discovery record](../../../.local/open-bitcoin-dev/phase167/plan50/execution-v1/test-discovery-v2.json) retain exact names, ignored reasons, raw streams, arguments, durations, exits and per-command source checks.

The nine-key environment was proved in a real asynchronous Bun child before each application and stayed identical through the final matrix. The original default Cargo target was used throughout Plan 50. One D4 rustc directory-read pause naturally completed into its genuine RED before any termination. During the final node-wide rustdoc phase, two more rustc samples showed `SearchPath::new → ReadDir → __getdirentries64`; those children also completed naturally. No process was killed and no isolated target, cache, flags, state or toolchain override was used. The checked [target observation](167-50-DIAGNOSED-ISOLATED-TARGET-AMENDMENT.md) records that the conditional exception remained dormant and does not qualify or extend any target policy to Plan 51. The natural host-global process guard and cooperative command-timings lock were retained. A separate unrelated Bazel process appeared only after the node-wide command began and was left untouched.

The final and post frames match all **495 first-party source** inputs (16 direct owners and 479 protected), **201 native** inputs, one separate Byteview input and four pinned Knots anchors. The strict original-before BSD `patch -F0` inverse restored 14 whole direct-owner bodies and two genuine absences, then reproduced all 16 final bodies without fuzz or offsets. Normal formatting keeps every direct owner at or below 627 physical lines. The [independent source re-review](../../../.local/open-bitcoin-dev/phase167/plan50/execution-v1/independent-source-review-v2.json) and [independent whole-plan review](../../../.local/open-bitcoin-dev/phase167/plan50/independent-whole-plan50-review.json) passed; the [summary release](../../../.local/open-bitcoin-dev/phase167/plan50/root-summary-release-v2/summary-release.json) binds their final hashes. All owned jobs and the cooperative lock are closed.

## Remaining boundary

Plan 51 must prove stored eligible body completion and final public success without premature global request cleanup. Plans 17, 18, 19, 25 and 20 then renew source guards, register verification, reconcile parity and root documentation, run the full default verifier with fresh coverage/Bazel, and obtain independent phase reviews. No Git staging, commit or push has occurred, and Phase 168 has not started.
