---
phase: 167-candidate-history-validation-rules
plan: "50"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-28T05:24:00Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 50 independent source-review coverage correction

The D1 and D4 genuine RED/GREEN controls, Task 2 64/64, Task 3 93/93 and focused peer/timing tests passed. A frozen 495-source/201-native/Byteview1/four-Knots frame and strict 16-owner inverse passed; the first three final commands (format, strict Clippy, build) exited zero. Independent source review then returned **FAIL_SOURCE_ACCEPTANCE** on three missing test obligations. The worker paused before final command 4. These first three final runs are diagnostic attempts and cannot stand for the later final matrix after source changes.

ROOT reopens **only three already declared test owners** under a separate exact correction release: `packages/open-bitcoin-node/src/network/canonical_admission/tests/phase167/compact_reconstruction.rs`, `packages/open-bitcoin-node/src/sync/tests/peer_resolution/peer_protocol_failures/lifecycle.rs`, and `packages/open-bitcoin-network/src/peer/tests/compact_retry_cases.rs`. All other 13 Plan 50 owners, 479 protected first-party paths, 201 native paths, Byteview1 and four pinned Knots anchors remain frozen. This adds no owner, production code, dependency, public API, policy, timer or cap. Capture complete current whole bodies and hashes before reopening; verify no command is queued/running and the prior frame's protected/native/Knots copies remain exact.

## Required direct controls

1. In the existing D4 compact reconstruction child, use actual managed CompactBlock receive paths to test: guard-passing UnknownPeer before any new header/request effect; guard-first malformed genesis/foreign owner error order; disabled and incapable peers with no new KnownOnly/snapshot effect; invalid parent, bits/difficulty, PoW, version, MTP and future-time headers refusing before enrollment. Preserve the real unknown-valid-header positive and moved mutation mismatch controls. Where a case is rejected after an earlier header effect, compare full accepted tip/work/Coins/undo/BASIC/provenance against the correct pre-case baseline and distinguish KnownOnly from ScriptsValid. Do not count direct HeaderStore-only tests, fixture switches, or unreachable malformed setups as managed compact evidence. If exact source-derived cases cannot be built within the existing test owners, HOLD for checked refinement instead of claiming coverage.
2. In the leased lifecycle matrix, add the missing `shared=false && finish=2` assertion that the send-triggered disconnect accepted no candidate tip and stored no body. Retain all 12 first/later cases, the real First full-Block completion, exact WitnessBlock fetch, peer 99 conservation and terminal cleanup.
3. In the existing D1 retry child, drive many distinct compact hashes through consumed fallback, existing request cap, terminal cleanup and genuine re-enrollment. Assert bounded request/attempt counts and no accumulated tombstones or repeated full fetch; preserve separate First/Later/Absent and age 60/61 controls. Use actual peer manager APIs and bounded fixtures, not direct private-state fabrication.

Run focused Task 1, Task 3 and Task 4 literal commands after test additions with nonzero named discovery, and the unchanged Task 2 selector as a regression control. A failing test must be diagnosed as behavior or setup before any production owner is reopened. Normal rustfmt and all owner files remain at most 628 lines; move cohesive test helpers into the already-owned fixtures rather than packing or creating an undeclared child. Capture complete after source/native/Knots copies and strict original-before BSD `patch -F0` inverse/forward across all 16 direct owners (14 whole bodies/two true absences), conserving 479 protected paths.

Only after these controls pass and independent source re-review clears the three findings may the executor freeze a new final frame and rerun **all 16** published final commands fresh and serially from format. Keep every earlier diagnostic failure and pass; no pass-only retry, zero-discovery, default-target substitution, Git action, global CVLD closure or Phase 168 work is authorized.
