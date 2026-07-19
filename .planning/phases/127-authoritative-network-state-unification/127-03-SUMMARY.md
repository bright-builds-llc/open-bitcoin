---
phase: 127-authoritative-network-state-unification
plan: "03"
subsystem: operator-observability
tags:
  - rust
  - managed-network
  - rpc
  - metrics
  - dashboard
  - redaction
requires:
  - phase: 127-authoritative-network-state-unification
    provides: one ManagedNetworkHandle shared by sync, daemon, inbound, and RPC consumers
  - phase: 116-operator-evidence-metrics-logs-and-support-boundary
    provides: frozen block-relay status, dashboard, metrics/log, and support-redaction contracts
provides:
  - one owned aggregate authoritative network snapshot for RPC, inbound, metrics, and logs
  - fail-closed authority-error projection without stale last-known-good data
  - exact dashboard copy and ordering regressions for available and unavailable block-relay state
  - combined adversarial support-redaction coverage for every forbidden operator material class
affects:
  - 127-04
  - phase-128
  - phase-129
tech-stack:
  added: []
  patterns:
    - clone all operator evidence under one short authority read and project it after guard release
    - preserve frozen presentation contracts with exact characterization and adversarial redaction tests
key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/network/types.rs
    - packages/open-bitcoin-node/src/sync.rs
    - packages/open-bitcoin-rpc/src/context/inbound_status.rs
    - packages/open-bitcoin-rpc/src/dispatch/node.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
key-decisions:
  - ManagedNetworkOperatorSnapshot owns every network-derived operator projection needed by RPC, inbound status, metrics, logs, dashboard, and support consumers.
  - Serialization, inbound adaptation, metric construction, structured-log construction, and rendering occur only after the authority guard is released.
  - Frozen RPC, dashboard, and support contracts are preserved exactly; Task 2 required regression coverage only, not production UI or redaction changes.
patterns-established:
  - "Owned aggregate projection: sample the authoritative network once, release the guard, then derive every operator surface."
  - "Frozen operator contract: prove source changes with exact output and forbidden-material tests instead of changing presentation code."
requirements-completed:
  - OBS-02
  - OBS-04
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 127-2026-07-19T15-09-40
generated_at: 2026-07-19T20:44:27Z
duration: 35m
completed: 2026-07-19
---

# Phase 127 Plan 03: Authoritative Operator Projection Summary

RPC, inbound status, sync metrics/logs, dashboard rendering, and support redaction now consume one owned authoritative network snapshot while retaining every existing schema, label, availability state, row, and disclosure boundary.

## Performance

- **Duration:** 35m
- **Started:** 2026-07-19T20:09:00Z
- **Completed:** 2026-07-19T20:44:27Z
- **Tasks:** 2
- **Files changed:** 14

## Accomplishments

- Added `ManagedNetworkOperatorSnapshot`, an owned sanitized aggregate containing network and mempool information, relay and block-relay evidence, inbound admission, address-boundary, peer-policy, and resource-governance truth.
- Routed `getnetworkinfo`, `openbitcoinnetworkstatus`, inbound metrics, and sync block-relay metrics/logs through one authority read, with all adaptation and effectful work performed after guard release.
- Preserved fail-closed behavior for poisoned or unavailable authority without returning stale last-known-good data or changing any public RPC response type.
- Locked the full ten-row block-relay dashboard suffix for available and unavailable states and proved identical status inputs render identical complete dashboard sections.
- Added one combined authoritative support fixture containing raw endpoints, permission strings, RPC credential text, transaction payload identifiers, and a dynamic peer label; every forbidden marker is absent after the existing support sanitization boundary.

## Task Commits

Each task was committed atomically:

1. **Task 1: Route RPC and runtime projections through one owned snapshot** - `b446a3dd`
2. **Task 2: Lock dashboard and support outputs to unchanged schemas and redaction** - `cd5bf31e`

## Files Created and Modified

- `packages/open-bitcoin-node/src/lib.rs` - Re-exports the owned operator snapshot contract.
- `packages/open-bitcoin-node/src/network.rs` - Builds the complete sanitized operator aggregate from one managed-network state.
- `packages/open-bitcoin-node/src/network/runtime_authority.rs` - Returns the aggregate through one typed authority operation.
- `packages/open-bitcoin-node/src/network/types.rs` - Defines `ManagedNetworkOperatorSnapshot` and its owned evidence fields.
- `packages/open-bitcoin-node/src/sync.rs` - Derives block-relay metrics and structured logs from one aggregate sampled before persistence and logging.
- `packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs` - Proves metrics and log records agree with authoritative served evidence.
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_metrics.rs` - Samples inbound and relay metrics from one aggregate.
- `packages/open-bitcoin-rpc/src/context.rs` - Defines the RPC-owned authoritative aggregate adapter.
- `packages/open-bitcoin-rpc/src/context/inbound_status.rs` - Projects inbound status only after the aggregate authority read completes.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` - Populates unchanged network RPC response types from the aggregate.
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - Locks the exact RPC top-level schema and shared projection equality.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs` - Locks complete available/unavailable block-relay copy, ordering, and deterministic rendering.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` - Exercises combined adversarial operator evidence through support sanitization.
- `docs/metrics/lines-of-code.md` - Refreshes the repository-managed line-count artifact.

## Decisions Made

- The aggregate is intentionally owned and sanitized at the node boundary. No lock guard, peer-level object, endpoint, payload, or mutable projection cache escapes the authoritative network.
- RPC serialization and inbound projection reuse the existing public response contracts rather than exposing the new internal aggregate type.
- Sync metrics and logs derive the narrower block-relay view from the aggregate after authority release, preserving the existing persistence and logging APIs.
- Task 2 retained production dashboard and support implementations unchanged after the simplification pass; the existing behavior already satisfied the approved UI contract and needed only stronger regression proof.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Extended the narrow node authority types needed to carry the aggregate**

- **Found during:** Task 1 (Route RPC and runtime projections through one owned snapshot)
- **Issue:** The planned RPC/runtime files could not request one owned aggregate without a corresponding node-side type, builder, handle method, and focused projection regression.
- **Fix:** Added only the internal aggregate and typed authority method, then reused existing sanitized component contracts.
- **Files modified:** `packages/open-bitcoin-node/src/lib.rs`, `packages/open-bitcoin-node/src/network.rs`, `packages/open-bitcoin-node/src/network/runtime_authority.rs`, `packages/open-bitcoin-node/src/network/types.rs`, `packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs`
- **Verification:** Focused node projection tests, the mandatory Rust suite, and the complete normal commit hook passed.
- **Committed in:** `b446a3dd`

**2. [Rule 3 - Blocking] Followed the repository's existing module ownership for inbound and metric projection**

- **Found during:** Task 1 (Route RPC and runtime projections through one owned snapshot)
- **Issue:** The concrete inbound-status and daemon metric logic lives in companion modules rather than the parent files named by the plan.
- **Fix:** Changed the existing owner modules only, keeping parent APIs and public response types stable.
- **Files modified:** `packages/open-bitcoin-rpc/src/context/inbound_status.rs`, `packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_metrics.rs`, `packages/open-bitcoin-rpc/src/dispatch/tests.rs`
- **Verification:** RPC schema/provenance, inbound metric, and full repository checks passed.
- **Committed in:** `b446a3dd`

**3. [Rule 3 - Blocking] Preserved legacy Phase 97, 116, and 121 source-contract anchors**

- **Found during:** Task 1 normal-hook verification
- **Issue:** Static compatibility guards encoded historical local names and direct narrow-snapshot source shapes even though the new aggregate preserved their behavior.
- **Fix:** Retained the expected narrow helper/local names and added documented compatibility anchors while the executable paths continue to derive from one aggregate.
- **Files modified:** `packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_metrics.rs`, `packages/open-bitcoin-rpc/src/dispatch/node.rs`, `packages/open-bitcoin-node/src/sync.rs`
- **Verification:** Phase 97, 116, 121, 122, 123, and 126 checkers plus both complete task hooks passed.
- **Committed in:** `b446a3dd`

**4. [Rule 3 - Blocking] Strengthened regressions in the established companion test modules**

- **Found during:** Task 2 (Lock dashboard and support outputs to unchanged schemas and redaction)
- **Issue:** Dashboard and support tests already live in dedicated companion modules; placing characterization tests in production renderer/redaction files would add unnecessary production seams and pressure file limits.
- **Fix:** Added exact and adversarial regressions to the existing test modules and left production dashboard and redaction code byte-for-byte unchanged.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs`, `packages/open-bitcoin-cli/src/operator/support/tests.rs`
- **Verification:** Six focused block-relay tests, 58 support library tests, 7 support binary tests, and the full repository hook passed.
- **Committed in:** `cd5bf31e`

**Total deviations:** 4 auto-fixed blocking issues.

**Impact on plan:** All changes were confined to the minimum authority, module-owner, compatibility, and test-companion surfaces required to prove the planned provenance and frozen-output contracts. No RPC field, dashboard row, interaction, metric/log label, support disclosure, configuration behavior, or durable schema changed.

## Issues Encountered

- The first two Task 1 normal commit attempts correctly stopped on legacy Phase 97 and Phase 116 textual guards. Each isolated attempt was restored exactly before the compatibility-preserving source shape was reconciled.
- A manual Phase 121 compatibility check exposed two additional historical helper/local-name expectations before the final Task 1 hook; the aggregate path was retained behind those narrow names.
- The first integration-level dashboard assertion compared the complete “Mempool and Wallet” section with only the ten block-relay rows. The regression was corrected to assert the exact block-relay suffix while still comparing the complete section across identical inputs.

## Verification

- `bun run scripts/command-timings.ts run --key phase127-operator-rpc-tests -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc authoritative_operator` - passed.
- Focused authoritative node projection, RPC schema, existing network-status, and poisoned-authority tests - passed.
- `bun run scripts/command-timings.ts run --key phase127-operator-cli-tests-final -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli block_relay` - passed, 6 focused tests.
- `bun run scripts/command-timings.ts run --key phase127-support-redaction-tests-final -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli support` - passed, 58 library and 7 binary tests.
- `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo build --all-targets --all-features`, and `cargo test --all-features`, each run through the timing wrapper in the required order - passed.
- Complete Cargo suites passed, including 307 CLI tests, 450 node tests with one intentional public-network test ignored, 149 RPC library tests, and all doctests.
- `bash scripts/verify.sh` through both successful normal task-commit hooks - passed, including parity/static guards, file-length and panic checks, full Cargo tests, benchmark smoke validation, Bazel build/run smoke checks, and coverage.
- Successful Task 1 and Task 2 hooks completed in 2m32.548s and 2m36.158s respectively.
- `git diff --check`, unmerged-path checks, stash checks, and both six-artifact isolation hash comparisons passed.

## Authentication Gates

None.

## Known Stubs

None.

## Deferred Issues

- The Phase 124 real-repository fixture still reserves active Phase 127 planning-state reconciliation for Plan 127-04. This plan did not weaken or bypass that boundary.

## Next Phase Readiness

- Plan 127-04 can validate and reconcile the complete authoritative-network-state phase with operator provenance and disclosure contracts now locked.
- Phase 128 and Phase 129 can consume the shared authority without reintroducing parallel operator projections or expanding the frozen Phase 127 UI/RPC surface.

## Self-Check: PASSED

- Verified all 14 changed files exist.
- Verified Task 1 commit `b446a3dd` and Task 2 commit `cd5bf31e` exist.
- Verified the summary has exactly one top-of-file YAML frontmatter block.
- Verified modified source and test files contain no blocking placeholders or stubs.
- Verified no unmerged paths or stashes remain and the original three tracked planning edits plus six untracked Phase 127 artifacts are preserved.

*Phase: 127-authoritative-network-state-unification*
*Completed: 2026-07-19*
