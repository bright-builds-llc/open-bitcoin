---
phase: 127-authoritative-network-state-unification
plan: "01"
subsystem: runtime-network-authority
tags:
  - rust
  - managed-network
  - rpc
  - daemon
  - synchronization
requires:
  - phase: 123-runtime-timing-and-evidence-integrity
    provides: authoritative runtime sampling and post-write evidence contracts
provides:
  - cloneable node-owned authoritative ManagedPeerNetwork handle
  - durable sync, daemon, inbound, and RPC composition sharing one allocation
  - typed authority failure propagation with owned snapshots
affects:
  - 127-02
  - 127-03
  - 127-04
  - phase-128
  - phase-129
tech-stack:
  added: []
  patterns:
    - private synchronous Arc<Mutex<_>> authority with typed commands and owned snapshots
    - durable-runtime-first composition with handle injection
key-files:
  created:
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
  modified:
    - packages/open-bitcoin-node/src/sync.rs
    - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
    - packages/open-bitcoin-rpc/src/context.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/inbound_listener.rs
key-decisions:
  - DurableSyncRuntime establishes the one production network authority before RPC and inbound consumers are constructed.
  - Test fixtures may still build explicit in-memory networks, while production composition injects only ManagedNetworkHandle.
  - Authority failures map to existing unavailable and client-not-connected vocabulary without changing RPC schemas.
requirements-completed:
  - BSRV-03
  - BSRV-04
  - OBS-02
  - OBS-04
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 127-2026-07-19T15-09-40
generated_at: 2026-07-19T18:18:03Z
duration: 36m
completed: 2026-07-19
---

# Phase 127 Plan 01: Authoritative Network State Unification Summary

One node-owned `ManagedNetworkHandle` now supplies durable sync, daemon, inbound, and RPC consumers with typed mutations and owned snapshots from one production `ManagedPeerNetwork` allocation.

## Performance

- **Duration:** 36m continuation execution
- **Started:** 2026-07-19T17:42:21Z
- **Completed:** 2026-07-19T18:18:03Z
- **Tasks:** 2
- **Files changed:** 40

## Accomplishments

- Added a narrow cloneable authority that owns the managed peer network behind a private synchronous mutex, returns owned results, and exposes explicit poison failures without leaking guards.
- Converted `DurableSyncRuntime` to establish and consume the shared authority, keeping lock scopes away from socket, Fjall, logging, metrics, serialization, and async work.
- Rewired `open-bitcoind` so the durable runtime creates the production authority before RPC and inbound composition, eliminating the prior split network allocation.
- Propagated authority failures through existing unavailable and client-not-connected paths while preserving RPC schemas and Knots-facing behavior.
- Added regressions proving a sync mutation is visible through RPC and that daemon composition shares the same runtime handle.

## Task Commits

Each task was committed atomically:

1. **Task 1: Define the narrow shared authority and convert durable sync** - `ee744ab8`
2. **Task 2: Compose daemon, inbound, and RPC around the one authority** - `f6c39dff`

## Files Created and Modified

- `packages/open-bitcoin-node/src/network/runtime_authority.rs` - Defines the private managed-network authority, cloneable handle, typed error, commands, and owned snapshots.
- `packages/open-bitcoin-node/src/sync.rs` - Stores and exposes the shared handle from `DurableSyncRuntime`.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` - Establishes durable runtime before RPC and inbound construction and injects its handle.
- `packages/open-bitcoin-rpc/src/context.rs` - Stores `ManagedNetworkHandle` instead of an independently owned peer network.
- `packages/open-bitcoin-rpc/src/context/network.rs` - Projects network state through typed authority operations.
- `packages/open-bitcoin-rpc/src/inbound_listener.rs` - Routes inbound peer mutations through the shared handle.
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/sync_seed.rs` - Holds extracted sync preflight helpers required by the production file-length contract.
- `packages/open-bitcoin-rpc/src/inbound_listener/resource_runtime.rs` - Holds extracted resource-runtime helpers required by the production file-length contract.
- `docs/parity/source-breadcrumbs.json` - Records the new first-party Rust source breadcrumb.
- `scripts/check-phase97-inbound-metrics.ts`, `scripts/check-phase116-operator-block-relay-evidence.ts`, and `scripts/check-phase123-runtime-timing-evidence-integrity.ts` - Preserve legacy source-contract checks across the handle and `Result` compatibility change.

## Decisions Made

- `DurableSyncRuntime` is the production composition root for the authoritative network allocation; downstream consumers receive only its handle.
- The handle exposes operation-specific methods and owned snapshots rather than a generic closure API, a cloned `ManagedPeerNetwork`, or a public lock guard.
- A private synchronous mutex is sufficient because critical sections contain only in-memory domain work and never cross an await or effectful boundary.
- Existing RPC unavailable vocabulary remains the external contract for authority failures; no endpoint, response schema, durable schema, or configuration behavior changed.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Split production modules to satisfy the enforced 628-line limit**

- **Found during:** Task 2 normal-hook commit
- **Issue:** The first normal commit attempt was rejected because `context/network.rs`, `open-bitcoind.rs`, and `inbound_listener.rs` exceeded the repository's production Rust file-length contract.
- **Fix:** Moved tests into the existing context test module, sync preflight helpers into `open_bitcoind/sync_seed.rs`, and inbound resource helpers into `inbound_listener/resource_runtime.rs`. The extraction was mechanical and did not change runtime or schema behavior.
- **Files modified:** `packages/open-bitcoin-rpc/src/context/network.rs`, `packages/open-bitcoin-rpc/src/context/tests.rs`, `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs`, `packages/open-bitcoin-rpc/src/bin/open_bitcoind/sync_seed.rs`, `packages/open-bitcoin-rpc/src/inbound_listener.rs`, `packages/open-bitcoin-rpc/src/inbound_listener/resource_runtime.rs`
- **Commit:** `f6c39dff`

**2. [Rule 3 - Blocking] Kept legacy source-contract guards compatible with the typed authority API**

- **Found during:** Tasks 1 and 2 verification
- **Issue:** Phase 97, 116, and 123 source checkers encoded the former owned-network constructor and direct-return shapes.
- **Fix:** Mechanically updated those checkers and their fixtures to recognize the handle and `Result` forms without weakening their behavioral assertions.
- **Files modified:** `scripts/check-phase97-inbound-metrics.ts`, `scripts/check-phase97-inbound-metrics.test.ts`, `scripts/check-phase116-operator-block-relay-evidence.ts`, `scripts/check-phase123-runtime-timing-evidence-integrity.ts`, `scripts/check-phase123-runtime-timing-evidence-integrity.test.ts`
- **Commits:** `ee744ab8`, `f6c39dff`

**3. [Rule 3 - Blocking] Recovered a path-limited stash replay conflict after the successful Task 2 commit**

- **Found during:** Task 2 planning-artifact restoration
- **Issue:** The approved path-limited `git stash push --keep-index` also recorded staged source state; after the hook regenerated and committed LOC metrics, applying the named stash conflicted on `docs/metrics/lines-of-code.md`.
- **Fix:** Restored the LOC file exactly from `f6c39dff`, preserved and unstaged the three original planning/config edits, restored the six untracked Phase 127 artifacts, and dropped only the named isolation stash. No source or planning content was manually rewritten.
- **Files modified:** None beyond restoration to the committed or pre-existing workspace state.
- **Commit:** `f6c39dff`

## Verification

- `cargo fmt --all --manifest-path packages/Cargo.toml` - passed
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` - passed
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` - passed
- `cargo test --manifest-path packages/Cargo.toml --all-features` - passed
- Focused authoritative-network node and RPC regressions - passed
- All 163 RPC tests: 144 library, 17 daemon, and 2 black-box tests - passed
- `bash scripts/verify.sh` through the normal Task 2 commit hook - passed in 11m25s, including parity breadcrumbs, deterministic legacy guards, file-length and panic checks, complete Cargo tests and doctests, benchmark smoke validation, Bazel build/run smoke checks, and coverage
- `git diff --check`, `git diff --cached --check`, unmerged-path check, stash check, and restoration status check - passed

## Authentication Gates

None.

## Deferred Issues

None.

## Next Phase Readiness

- Plan 127-02 can consume the shared authority for canonical peer lifecycle and activity timestamps.
- The static Phase 124 final-verification guard remains intentionally reserved for Plan 127-04 evolution.

## Self-Check: PASSED

- Verified all key created and modified files exist.
- Verified Task 1 commit `ee744ab8` and Task 2 commit `f6c39dff` exist.
- Verified the summary has exactly one top-of-file YAML frontmatter block.
- Verified no unmerged paths or stashes remain and the pre-existing planning/config workspace shape is preserved.
