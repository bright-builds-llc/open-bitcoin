---
phase: 94-dos-and-resource-governance
plan: 03
subsystem: network-runtime
tags: [dos, resource-governance, inbound-listener, rpc, rust, bazel]

requires:
  - phase: 94-01
    provides: Pure inbound resource envelope and policy model
  - phase: 94-02
    provides: Pure queue, timeout, churn, repeated-failure, and reconnect decisions
provides:
  - Runtime inbound listener envelope enforcement before payload allocation
  - Runtime listener resource evidence for payload, timeout, churn, reconnect, and queue pressure decisions
  - Queue/backpressure checks before listener socket reads and response writes
affects: [phase-94, phase-95, inbound-listener, resource-evidence, network-status]

tech-stack:
  added: []
  patterns:
    - Pure resource policies consumed at the runtime adapter boundary
    - Bounded listener evidence without raw endpoint, peer id, payload, or credential capture
    - Hook-compatible TDD with RED evidence runs and passing atomic task commits

key-files:
  created:
    - packages/open-bitcoin-rpc/src/inbound_listener/resource_runtime.rs
  modified:
    - MODULE.bazel.lock
    - docs/metrics/lines-of-code.md
    - docs/parity/source-breadcrumbs.json
    - packages/Cargo.lock
    - packages/open-bitcoin-rpc/BUILD.bazel
    - packages/open-bitcoin-rpc/Cargo.toml
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - packages/open-bitcoin-rpc/src/inbound_listener.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs

key-decisions:
  - "Use existing Phase 93 managed peer-policy aggregate projection for reconnect suppression instead of adding listener-local ban maps."
  - "Place runtime resource accounting helpers in an inbound_listener child module to satisfy repo production file-length limits while keeping the root listener as the adapter orchestration surface."
  - "Preserve hook requirements by running TDD RED checks locally and committing only verification-passing task states."

patterns-established:
  - "Runtime listener checks pure resource policies before allocating payload buffers, before peer work, and before socket reads/writes."
  - "Resource pressure events are recorded through both InboundListenerEvidence and ManagedRpcContext with low-cardinality labels only."
  - "Listener helper extraction requires parity breadcrumbs for first-party Rust files."

requirements-completed: [DOS-01, DOS-02, DOS-03, DOS-04, DOS-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 94-2026-06-26T15-47-23
generated_at: 2026-06-26T19:47:49Z

duration: 71min
completed: 2026-06-26
---

# Phase 94 Plan 03: Inbound Listener Resource Runtime Summary

**Inbound listener resource governance now enforces Phase 94 envelope, timeout, churn, reconnect, and queue-pressure decisions in the opt-in runtime path.**

## Performance

- **Duration:** 71min
- **Started:** 2026-06-26T18:36:41Z
- **Completed:** 2026-06-26T19:47:49Z
- **Tasks:** 3
- **Files modified:** 11

## Accomplishments

- Routed listener wire reads through `InboundEnvelopePolicy` before payload allocation, producing resource evidence for oversized payloads, wrong network magic, and unsupported commands.
- Added listener/runtime evidence counters for timeout disconnects, churn rejections, repeated failures, and reconnect suppression, with managed-context recording for shared status projection.
- Enforced `ResourceGovernancePolicy::decide_queue` before socket reads and response writes, recording pressure through both listener and managed resource evidence.

## Task Commits

Each task was committed atomically:

1. **Task 1: Replace listener payload-length parsing with envelope policy** - `94ac4b2c` (feat)
2. **Task 2: Record timeout, churn, and reconnect resource evidence in listener state** - `1d1f5425` (feat)
3. **Task 3: Enforce queue/backpressure decisions in runtime read and write paths** - `ebf02ea2` (feat)

## Files Created/Modified

- `packages/open-bitcoin-rpc/src/inbound_listener.rs` - Runtime adapter orchestration for envelope decisions, lifecycle pressure checks, timeout-aware reads/writes, and queue enforcement.
- `packages/open-bitcoin-rpc/src/inbound_listener/resource_runtime.rs` - Listener resource helper module for runtime counters, timeout events, queue-pressure inputs, wire-read outcomes, and decision-to-event conversion.
- `packages/open-bitcoin-rpc/src/inbound_listener/tests.rs` - Deterministic loopback and synthetic tests covering envelope rejection, timeout events, churn/reconnect handling, and queue pressure.
- `packages/open-bitcoin-rpc/src/context/network.rs` - Managed resource-event recording and reconnect-suppression input projection.
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - Shared status test coverage for inbound resource evidence propagation.
- `packages/open-bitcoin-rpc/Cargo.toml` and `packages/open-bitcoin-rpc/BUILD.bazel` - RPC crate dependency wiring for codec/envelope runtime use.
- `packages/Cargo.lock`, `MODULE.bazel.lock`, `docs/parity/source-breadcrumbs.json`, and `docs/metrics/lines-of-code.md` - Generated or policy-tracking updates required by hooks and verifier checks.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc inbound_listener --no-fail-fast` - passed after Task 3, 27 matching tests passed.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features -- -D warnings` - passed after Task 3.
- `cargo fmt --all --manifest-path packages/Cargo.toml` - passed before each task commit.
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` - passed before Task 3 commit.
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` - passed before Task 3 commit.
- `cargo test --manifest-path packages/Cargo.toml --all-features` - passed before Task 3 commit.
- Commit hooks ran `bash scripts/verify.sh` for each task commit and passed, including LOC, parity breadcrumbs, benchmark smoke checks, and Bazel smoke build.
- Acceptance `rg` checks for envelope policy usage, lifecycle policy calls, timeout wrapping, queue pressure usage, shared evidence recording, and sensitive evidence exclusions passed.

## Decisions Made

- Reconnect suppression uses the existing managed peer-policy aggregate projection (`active_bans` and `discouraged_peers`) as the Phase 93 source-of-truth input, avoiding listener-local ban or discourage state.
- Runtime resource helper code lives in `inbound_listener/resource_runtime.rs` because the root listener file was at the repo's production file-length limit. The root listener still calls queue and lifecycle policy decisions on the runtime path.
- TDD RED commits were not created because repository hooks require passing commits. RED test runs were still executed and observed before each implementation.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added missing Bazel dependency for codec use**
- **Found during:** Task 1 (Replace listener payload-length parsing with envelope policy)
- **Issue:** The first hook-backed commit attempt failed because the RPC Bazel target did not include the codec library required by the envelope-policy runtime path.
- **Fix:** Added `//packages/open-bitcoin-codec:open_bitcoin_codec_lib` to `packages/open-bitcoin-rpc/BUILD.bazel` and refreshed Bazel lock/metrics outputs.
- **Files modified:** `packages/open-bitcoin-rpc/BUILD.bazel`, `MODULE.bazel.lock`, `docs/metrics/lines-of-code.md`
- **Verification:** Task 1 hook-backed commit passed after the dependency update.
- **Committed in:** `94ac4b2c`

**2. [Rule 3 - Blocking] Split listener resource helpers to satisfy production file-length policy**
- **Found during:** Task 2 (Record timeout, churn, and reconnect resource evidence in listener state)
- **Issue:** Adding timeout, churn, reconnect, and evidence helpers pushed `inbound_listener.rs` past the repo production file-length gate.
- **Fix:** Moved reusable runtime helper code into `packages/open-bitcoin-rpc/src/inbound_listener/resource_runtime.rs` and added the required parity breadcrumb entry.
- **Files modified:** `packages/open-bitcoin-rpc/src/inbound_listener.rs`, `packages/open-bitcoin-rpc/src/inbound_listener/resource_runtime.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** Production file-length and parity breadcrumb hook checks passed.
- **Committed in:** `1d1f5425`

**3. [Rule 3 - Blocking] Corrected parity breadcrumb header alignment**
- **Found during:** Task 2 (Record timeout, churn, and reconnect resource evidence in listener state)
- **Issue:** The new helper file's breadcrumb header did not initially match `docs/parity/source-breadcrumbs.json`.
- **Fix:** Aligned the helper header with the registered breadcrumb source list.
- **Files modified:** `packages/open-bitcoin-rpc/src/inbound_listener/resource_runtime.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** `scripts/check-parity-breadcrumbs.ts` passed through the hook.
- **Committed in:** `1d1f5425`

**4. [Rule 3 - Blocking] Kept root listener below the strict LOC threshold**
- **Found during:** Task 3 (Enforce queue/backpressure decisions in runtime read and write paths)
- **Issue:** The first Task 3 hook attempt failed because `inbound_listener.rs` was exactly at the maximum production line threshold; the checker requires the file to remain under that limit.
- **Fix:** Removed a nonessential root comment and kept queue helper implementation in the child helper module.
- **Files modified:** `packages/open-bitcoin-rpc/src/inbound_listener.rs`, `docs/metrics/lines-of-code.md`
- **Verification:** Production file-length hook check passed, then Task 3 hook-backed commit passed.
- **Committed in:** `ebf02ea2`

**Total deviations:** 4 auto-fixed (all Rule 3 blocking issues)
**Impact on plan:** All fixes were required to satisfy repo verification and instruction-file policy. No public-network defaults, relay behavior, or architecture boundaries were broadened.

## Issues Encountered

- TDD RED checks intentionally failed before implementation for each task, but RED commits were skipped because this repo's hooks reject failing commits.
- The initial broad stub scan matched generated documentation embedded in `MODULE.bazel.lock`; a narrowed scan of first-party touched source and docs found no placeholder stubs.

## Known Stubs

None. The focused stub scan over first-party touched source and tracking docs found no `TODO`, `FIXME`, placeholder text, or hardcoded empty UI/data stubs.

## Threat Flags

None. The plan's declared listener trust boundaries covered the touched network runtime surface, and the implementation did not add new endpoints, authentication paths, file-access patterns, schema changes, or unplanned trust-boundary crossings.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 95 can consume bounded runtime resource evidence from the opt-in inbound listener and shared managed context. The listener still remains explicitly opt-in, loopback-safe by default, and outside public-network production-readiness claims.

---
*Phase: 94-dos-and-resource-governance*
*Completed: 2026-06-26*

## Self-Check: PASSED

- Summary file exists: `.planning/phases/94-dos-and-resource-governance/94-03-SUMMARY.md`
- Task commit found: `94ac4b2c`
- Task commit found: `1d1f5425`
- Task commit found: `ebf02ea2`
