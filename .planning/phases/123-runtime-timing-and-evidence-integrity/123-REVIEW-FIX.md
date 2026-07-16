---
status: all_fixed
findings_in_scope: 1
fixed: 1
skipped: 0
iteration: 3
---

# Phase 123 Review Fix

## Outcome

The iteration 3 `CR-01` finding from `123-REVIEW.md` is fixed and verified. Compact-download timeout fallback now enters the ordinary full-block request lifecycle before the `getdata` write. The same peer session remains active while that request is pending, consumes a matching `block` or `notfound`, clears peer and runtime tracking, and yields only after neither compact nor ordinary fallback response work remains.

## Iteration 3 Finding Resolution

| Finding | Status | Commit | Resolution |
| --- | --- | --- | --- |
| CR-01 | Fixed | `e3c456e6` | Converted compact-timeout fallback into a tracked ordinary block request, retained the same session through response work, and added an end-to-end regression with thirteen five-second idle wakes followed by the matching full block. The regression proves the block was requested, accepted, persisted, credited, and removed from in-flight tracking before the final idle wake yields. |

## Supporting Commit

- `5e80c08c` refreshes the tracked line-count report after the iteration 3 production, test, and checker changes.

## Cumulative Review Fixes

Iteration 1 resolved the original four findings:

- `f5468aea` routed block-relay activation into durable daemon sync construction.
- `849f003e` routed block-serving activation into inbound RPC serving and successful-write acknowledgement.
- `bc24e696` propagated idle-sampled live session timestamps through activity, receive, and reconciliation.
- `00232789` added cancellation-aware silent-session shutdown and the initial fixed two-idle yield.
- `30e85693` aligned the Phase 107 activation checker with the production constructor.
- `39aa9eaa` extracted peer-session control and preserved the 628-line production file limit.

Iteration 2 resolved the two follow-up findings:

- `24a40957` replaced the fixed idle cutoff with compact-work-aware session retention through the compact timeout.
- `1f05d388` sampled the injected clock before every received-message dispatch.
- `149327c9` refreshed the tracked line-count report.

Iteration 3 completes the lifecycle that iteration 2 established: timeout no longer proves only that fallback `getdata` was emitted. The fallback is now tracked as ordinary download work, and its matching response is consumed, persisted, credited, and cleared before the session is allowed to yield.

## Verification

- Phase 107, Phase 121, and Phase 123 checker mutation suites — 62 passed total; all three live checkers passed.
- Focused `open-bitcoin-node` Phase 123 tests — 27 passed.
- Focused `open-bitcoin-rpc` Phase 123 tests — 10 passed across library and daemon targets.
- Targeted node/RPC clippy with all targets and features — passed with warnings denied.
- `bash scripts/verify.sh` — passed in 4m 21.943s, including formatting, linting, all-target builds and tests, coverage, and Bazel verification.

## Deviations

The tracked LOC artifact changed as expected after the production, regression, and checker updates; it was regenerated and committed in `5e80c08c` before the clean full-verifier pass. No dependency, public API, or unrelated behavior change was introduced.

## Residual Risk

No known review finding remains. Cancellation remains cooperative at session receive boundaries, with the existing socket read timeout bounding a single blocking receive. Compact and ordinary fallback work intentionally retain the current peer session only while their bounded in-flight state remains present.
