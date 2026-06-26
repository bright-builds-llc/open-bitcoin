---
phase: 94-dos-and-resource-governance
plan: 02
subsystem: network
tags: [rust, networking, resource-governance, dos-hardening, peer-lifecycle]

requires:
  - phase: 94-01
    provides: Pure inbound message-envelope resource gate and stable resource events
provides:
  - Pure queue and request resource governance policy for open-bitcoin-network
  - Deterministic timeout, churn, repeated-failure, and reconnect suppression policy
  - Stable low-cardinality resource pressure and lifecycle labels for later runtime wiring
affects: [open-bitcoin-network, inbound-runtime, peer-manager, dos-resource-governance]

tech-stack:
  added: []
  patterns:
    - Functional-core resource governance over typed snapshots
    - Injected timestamp and counter inputs instead of runtime clocks or sockets
    - Stable operator-safe labels and next_action strings

key-files:
  created: []
  modified:
    - packages/open-bitcoin-network/src/resource.rs
    - packages/open-bitcoin-network/src/resource/tests.rs
    - packages/open-bitcoin-network/src/lib.rs
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Kept Phase 94 queue, request, timeout, churn, failure, and reconnect decisions as pure data-in/data-out policy."
  - "Treated inactive relay-like permission effects as evidence only, never as capacity multipliers."
  - "Exported the resource governance API from open-bitcoin-network so later runtime plans can consume the policy without reaching into private modules."
  - "Preserved repo hook requirements by recording TDD RED locally and committing only verification-passing task states."

patterns-established:
  - "ResourceGovernancePolicy::decide_queue and decide_request return stable pressure labels from bounded queue/request snapshots."
  - "ResourceGovernancePolicy::decide_timeout, decide_churn, decide_repeated_failure, and decide_reconnect use injected timestamps and counters only."
  - "Resource policy events reuse the bounded InboundResourceEvent shape from Plan 94-01."

requirements-completed: [DOS-02, DOS-03, DOS-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 94-2026-06-26T15-47-23
generated_at: 2026-06-26T18:28:36Z

duration: 29min
completed: 2026-06-26
---

# Phase 94 Plan 02: Queue, Request, and Lifecycle Resource Policy Summary

**Pure queue, request, timeout, churn, failure, and reconnect resource governance with stable Phase 94 labels**

## Performance

- **Duration:** 29 min
- **Started:** 2026-06-26T18:00:00Z
- **Completed:** 2026-06-26T18:28:36Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- Added named Phase 94 queue/request caps and deterministic policy decisions for read queue pressure, write queue pressure, queued-message pressure, and request caps.
- Added lifecycle governance for slow handshakes, idle peers, connection churn, repeated failures, active bans, and discouraged reconnect attempts.
- Covered inactive relay-like permission effects so `Relay`, `ForceRelay`, `Mempool`, `BloomFilter`, and `BlockFilters` do not raise resource caps.
- Exported the typed resource governance API from `open-bitcoin-network` for later runtime and peer-manager integration.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add queue and request resource policy** - `bb34d894` (`feat`)
2. **Task 2: Add timeout, churn, failure, and reconnect policy** - `9c6e3c6c` (`feat`)

## Files Created/Modified

- `packages/open-bitcoin-network/src/resource.rs` - Queue/request caps, pressure labels, lifecycle constants, input structs, and pure policy decisions.
- `packages/open-bitcoin-network/src/resource/tests.rs` - Deterministic tests for pressure labels, lifecycle labels, cap boundaries, inactive relay-like effects, and injected timestamp behavior.
- `packages/open-bitcoin-network/src/lib.rs` - Public exports for the Phase 94 resource governance policy API.
- `docs/metrics/lines-of-code.md` - Hook-generated line count refresh from the repo verifier.

## Checks Run

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network resource --no-fail-fast` - passed, 31 resource tests.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings` - passed.
- `cargo fmt --all --manifest-path packages/Cargo.toml` - passed before both task commits.
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` - passed before both task commits.
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` - passed before both task commits.
- `cargo test --manifest-path packages/Cargo.toml --all-features` - passed before both task commits.
- Normal git hooks ran `bash scripts/verify.sh` for both task commits; Task 1 completed in 2m 59.104s and Task 2 completed in 2m 53.365s.

## Acceptance Criteria

- Verified named queue/request constants, pressure labels, policy structs, and stable `resource_pressure_active`, `read_queue_pressure`, `write_queue_pressure`, and `request_cap_reached` actions with the plan `rg` checks.
- Verified inactive relay-like effect coverage for `Relay`, `ForceRelay`, `Mempool`, `BloomFilter`, and `BlockFilters`.
- Verified named lifecycle constants, lifecycle structs, stable labels, and `timeout_disconnect`, `churn_rejected`, and `reconnect_suppressed` next actions with the plan `rg` checks.
- Verified the no-side-effect scan for `sleep`, `Instant::now`, `SystemTime::now`, `TcpStream`, and `tokio::spawn` returned no matches in `resource.rs` or `resource/tests.rs`.

## Decisions Made

- Kept resource governance in the pure network module; no sockets, sleeps, system clocks, or runtime tasks were introduced.
- Used explicit Phase 94 constants instead of deriving caps from runtime configuration, preserving deterministic policy behavior for later integration.
- Modeled banned reconnects as hard disconnect decisions and discouraged reconnects as backpressure decisions while sharing the stable `reconnect_suppressed` action.
- Added crate-root exports for both queue/request and lifecycle policy types because later plans need a public integration surface.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Exported resource governance API from the crate root**
- **Found during:** Task 1 and Task 2
- **Issue:** The plan scoped file edits to `resource.rs` and `resource/tests.rs`, but later runtime and peer-manager plans need a stable public API rather than private-module access.
- **Fix:** Exported queue/request and lifecycle constants, input structs, labels, and `ResourceGovernancePolicy` from `packages/open-bitcoin-network/src/lib.rs`.
- **Files modified:** `packages/open-bitcoin-network/src/lib.rs`
- **Verification:** `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings`; normal hook `bash scripts/verify.sh`
- **Committed in:** `bb34d894`, `9c6e3c6c`

**2. [Rule 3 - Blocking] Added coverage-focused queue/request tests during Task 1**
- **Found during:** Task 1 (Add queue and request resource policy)
- **Issue:** The normal pre-commit verifier blocked the first task commit because the new resource policy paths did not satisfy repo coverage gates with only the initial behavior tests.
- **Fix:** Added focused tests for aggregate write queue pressure, queued-message pressure, request-cap branches, and accepted-at-cap behavior.
- **Files modified:** `packages/open-bitcoin-network/src/resource/tests.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network resource --no-fail-fast`; normal hook `bash scripts/verify.sh`
- **Committed in:** `bb34d894`

**3. [Process - Repo Hook Constraint] TDD RED states were not committed separately**
- **Found during:** Task 1 and Task 2
- **Issue:** The plan marked both tasks `tdd="true"`, but repo instructions and normal hooks require every commit to pass formatting, linting, build, tests, and verification. Committing failing RED states would violate those higher-priority rules.
- **Fix:** Ran and recorded RED failures locally, then committed only passing task states after GREEN. Task 1 failed on unresolved queue/request policy imports. Task 2 failed on unresolved lifecycle constants, inputs, fields, and methods.
- **Files modified:** None beyond task files
- **Verification:** Both task commits were made through normal hooks without `--no-verify`.
- **Committed in:** `bb34d894`, `9c6e3c6c`

***

**Total deviations:** 3 auto-handled (1 missing critical, 1 blocking, 1 process constraint)
**Impact on plan:** No scope creep. The extra export and coverage tests were required for correct integration and repo verification while preserving the pure policy boundary.

## Issues Encountered

- The Task 1 hook coverage gate reported uncovered `resource.rs` lines, so targeted tests were added before the task commit could succeed.
- The normal hook refreshed `docs/metrics/lines-of-code.md` as an intentional generated artifact.
- The stub scan matched `let malformed_version_payload = []` in a unit test. This is intentional malformed-payload test data, not a UI or data-source stub.

## Known Stubs

None - no placeholder, empty UI data source, or unwired mock data was introduced.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Later Phase 94 plans can consume `ResourceGovernancePolicy` from the crate root to wire queue/request pressure, timeout disconnects, churn rejection, repeated-failure limits, and reconnect suppression into runtime and peer-manager flows without adding policy logic to effectful adapters.

## Self-Check: PASSED

- Confirmed summary, resource source/tests, exports, and LOC report exist.
- Confirmed task commits `bb34d894` and `9c6e3c6c` exist in git history.

***
*Phase: 94-dos-and-resource-governance*
*Completed: 2026-06-26*
