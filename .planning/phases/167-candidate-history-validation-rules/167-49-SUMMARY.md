---
phase: 167-candidate-history-validation-rules
plan: "49"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: compact-block-selected-completion
tags: [rust, compact-block, consensus, networking, canonical-admission, evidence]
requires:
  - phase: 167-16
    provides: Accepted source-bound historical and ordinary canonical evidence
  - phase: 167-35
    provides: Borrowed typed block-mutation predicate and selected consensus rules
provides:
  - One selected compact completion chain shared by the public facade and peer manager
  - Typed incomplete, structural, mutation, and codec-error outcomes before ReceivedBlock
  - Explicit held witness and inventory facts without caller-selected canonical policy
  - Exact final compact, canonical, peer, network, consensus, and node test evidence
affects: [167-50, 167-51, 167-17, 167-18, 167-19, 167-20, CVLD]
tech-stack:
  added: []
  patterns: [borrowed mutation classification, permanent crate-private selected API, thin public delegates, exact source inverse]
key-files:
  created:
    - packages/open-bitcoin-network/src/compact_download/completion.rs
  modified:
    - packages/open-bitcoin-network/src/compact_download.rs
    - packages/open-bitcoin-network/src/peer/compact_download_state.rs
    - packages/open-bitcoin-node/src/network/canonical_admission/tests/phase167.rs
    - packages/open-bitcoin-network/src/compact_download/tests.rs
    - packages/open-bitcoin-network/src/compact_download/tests/completion_cases.rs
    - packages/open-bitcoin-network/src/compact_download/tests/blocktxn_cases.rs
    - packages/open-bitcoin-network/src/compact_download/tests/initialization_cases.rs
    - packages/open-bitcoin-network/src/compact_download/tests/scheduling_cases.rs
key-decisions:
  - Preserve the seven permanent selected crate-private paths while original public names become thin explicit-fact delegates.
  - Keep C3's first runnable pass as a TDD process deviation; its scratch mutant is post-GREEN sensitivity, not preimplementation RED.
  - Correct four strict-lint findings under separate exact ownership and restart every final command on a fresh frozen frame.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
requirements_scope: bounded-plan49-contribution-only
global_cvld_acceptance: open
generated_by: gsd-execute-plan
generated_at: "2026-10-28T04:25:08Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
status: canonical-summary-pending-independent-qa-and-root-final49
---

# Phase 167 Plan 49: Selected compact completion

Plan 49 now classifies a completed compact block through the accepted borrowed consensus mutation predicate before a peer manager can emit `ReceivedBlock`. The original public completion, initialization, BlockTxn, and action-translation names delegate to the same selected implementations with explicit witness and inventory facts. A mutated complete body yields full-fetch fallback; incomplete reconstruction, malformed structure, and typed codec errors keep separate outcomes. The peer manager derives canonical witness selection from its held rules and context, rather than a caller flag. This is a bounded contribution to all five CVLD requirements; Plans 50 and 51 still own ordered failure, header admission, and final stored-body success. Global CVLD acceptance remains open.

## Source and test evidence

The four task stages changed only their declared owners. C1 created the private completion child and seven permanent crate-private selected functions; its genuine compiled Merkle-mismatch RED returned `Completed` where fallback was required, while the matched control passed. The final selected Task 1 selector passed 8/8, and the retained original-wrapper selection passed 34/34. The bounded borrowed-view test reaches the real consensus predicate and propagates `CodecError`. A concrete `Block` from the current BlockTxn signature cannot carry that lazy oversized test view without a new test hook or huge allocation, so no direct BlockTxn codec-error reachability claim is made.

C2 bound the real manager to the selected initialization, BlockTxn, and explicit-inventory translator paths. Its genuine canonical Merkle-mismatch RED reached later `bad-txnmrklroot` rejection through the old structural manager; the final test instead proves compact fallback before body absorption, no `ReceivedBlock`, unchanged tip/work/Coins/undo, and unchanged live and durable BASIC and provenance state. The canonical positive case and 12 canonical plus 6 network controls passed. A separate BASIC-owner fixture setup failure earned no RED credit.

C3 migrated 14 direct legacy test calls to permanent selected APIs without changing C1/C2 producers. Its Task 3 selector passed 36/36, including direct mutated-fallback and matched-completion controls; the final C4 caller selector passed 38/38 with explicit `Result` and Block/WitnessBlock inventory assertions. C4 changed three of its five leased files; the lifecycle test owner and `lib.rs` remained byte-identical. The public parent shrank to 463 normally formatted lines. All selected producer/reexport paths remain available to C2 and C3. No legacy structural public wrapper or parent-only completion projection remains.

**C3 TDD process deviation:** the first runnable C3 controls passed because C1 had already implemented the selected behavior. Its preceding `E0382` compiler failure was setup, not a behavioral RED. A ROOT-authorized scratch-only post-GREEN guard mutant then compiled and made the one named fallback test fail at the intended assertion: actual `Completed` versus expected `Fallback`. All 7,851 inventoried scratch and live files matched their original copy after strict restoration; four pre-existing Bazel symlinks were excluded from that inventory. This establishes test sensitivity, **not** a C3 preimplementation RED. C1 and C2 retain their separate genuine behavioral REDs. Task 4 had `tdd=false`; its initial import-cleanup compiler failure was setup, and no Task 4 behavioral RED is claimed.

The first final strict Clippy run found `expect_used` and `cloned_ref_to_slice_refs` in accepted C2/C1 owners. After those narrow fixes, a second strict run exposed needless lifetime and `err_expect` in two historical test-only files. ROOT issued separate exact correction leases. Final strict Clippy, format, compact 38/38, historical source 2/2 and contexts 2/2 passed, with no lint allow or public API change. The earlier failed attempts remain diagnostic evidence. The independent bounded correction review passed.

## Final frozen verification

After the corrections, C4 recaptured 490 source files, 201 original native files, and one separately accepted Byteview native input. Every original and copy matched before and after the final run. All **13 published final commands** ran fresh and serially on that frame, each with an actual asynchronous nine-key Bun child environment proof, the natural host process guard, and the cooperative command-timings lock. Format, strict Clippy, and build exited zero. The behavior selectors discovered and passed 193 network compact, 92 node compact, 12 canonical, 19 peer-resolution, 2 network documentation, 8 selected completion, 18 consensus mutation, 51 witness, and 713 network-wide tests across their respective groups. The full node-wide command passed **2,403 tests** across four groups (2,372 + 15 + 3 + 13), with four documented ignored tests and zero failures. Its measured duration was 10,997 seconds; repeated read-only samples showed movement through distinct storage and sync tests. The ignored names/reasons and every raw stream, argv, duration, exit, probe and source check are indexed in the C4 `CORE.json`.

The default Cargo target stalled during C2 setup in a macOS directory read. Two spaced samples and a ROOT release authorized a diagnosed isolated target for Plan 49 only, changing **only** `CARGO_TARGET_DIR` in the nine-key projection. That setup stall earned no behavioral credit. The other eight keys, literal Cargo arguments, async probe and serial lock remained intact. The original default target is not qualified by this exception, and Plans 50/51 receive no target exception from it. One C2 host-global guard observation lacked an independent raw snapshot at the exact moment unrelated Bazel cleared; the guarded serial command completed with no observed override or kill.

The whole-plan original-before inverse covers 14 source paths: 12 unique direct owners plus the two historical lint-correction owners, 13 original whole bodies and one genuinely absent completion child. Strict BSD `patch -F0` reversed and reapplied the 11 changed paths without fuzz or offsets, recovering every original body/absence and reproducing every final owner. The 476 other source paths remained byte-identical through both directions. C4's separate five-owner inverse and the final source/native copies also passed. The [independent whole-plan review](../../../.local/open-bitcoin-dev/phase167/plan49/independent-whole-plan49-review.json) found no source or evidence blocker. The [whole inverse](../../../.local/open-bitcoin-dev/phase167/plan49/root-whole-plan49-inverse-v1/whole-inverse.json), [C4 CORE](../../../.local/open-bitcoin-dev/phase167/plan49/c4-execution-v1/CORE.json), [correction acceptance](../../../.local/open-bitcoin-dev/phase167/plan49/root-strict-corrections-closure-v1/root-acceptance.json), and [summary release](../../../.local/open-bitcoin-dev/phase167/plan49/root-summary-release-v1/summary-release.json) retain the exact hashes and evidence references.

## Remaining boundary

All Plan 49 jobs are closed and its source lease returned. Plan 50 must implement arrival-ordered First/Later/Absent failure, bounded consumed attempt lifetime, service-selected fetch, and canonical compact-header admission; Plan 51 must complete matching requests only after actual stored eligible body and final public success. Plans 17/18/19/25/20 then renew guards, register verification, reconcile parity/docs and run full default verification, fresh coverage, Bazel, independent phase review and lifecycle closure. No Git staging, commit or push has occurred. Phase 168 has not started.
