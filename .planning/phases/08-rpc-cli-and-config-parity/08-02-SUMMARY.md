---
phase: 08-rpc-cli-and-config-parity
plan: 02
subsystem: api
tags: [rpc, config, serde, adapter, wallet]
requires:
  - phase: 08-01
    provides: "First-class `open-bitcoin-rpc` crate ownership and repo-native verification coverage"
provides:
  - "Managed RPC-facing projection and command helpers on top of `ManagedPeerNetwork` and `ManagedWallet`"
  - "Typed JSON-RPC method, envelope, config, auth, error, and runtime-context contracts for the supported Phase 8 surface"
  - "A sealed `ManagedRpcContext` composition root over the node and wallet adapter seam"
affects: [08-03, 08-04, rpc, cli, config]
tech-stack:
  added: []
  patterns:
    - "Adapter-owned RPC helper projections in `open-bitcoin-node`"
    - "Typed contract modules in `open-bitcoin-rpc` with `foo.rs` plus `foo/tests.rs` layout"
key-files:
  created:
    - packages/open-bitcoin-rpc/src/error.rs
    - packages/open-bitcoin-rpc/src/envelope.rs
    - packages/open-bitcoin-rpc/src/method.rs
    - packages/open-bitcoin-rpc/src/config.rs
    - packages/open-bitcoin-rpc/src/context.rs
  modified:
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - packages/open-bitcoin-node/src/wallet.rs
    - packages/open-bitcoin-rpc/src/lib.rs
key-decisions:
  - "Keep the initial auth contract limited to cookie-file or explicit `rpcuser` plus `rpcpassword` modes and exclude `rpcauth`, whitelist, and wallet-path routing from the type system."
  - "Expose later handler entrypoints through `ManagedRpcContext` so transport and dispatcher code target seam-backed helpers instead of raw `peer_manager()` or `wallet()` access."
  - "Model the Open Bitcoin build or sign extensions with integer satoshi fields (`amount_sats`, `fee_rate_sat_per_kvb`) instead of claiming `sendtoaddress` parity."
patterns-established:
  - "Phase 8 transport and CLI code should parse operator input into `open-bitcoin-rpc` typed contracts before invoking the managed adapter seam."
  - "Later Phase 8 handlers should use `SupportedMethod` plus `ManagedRpcContext` instead of open-coding method-name strings or reaching through node or wallet internals."
requirements-completed: [RPC-01, CLI-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 08-2026-04-24T02-23-19
generated_at: 2026-04-23T04:00:23Z
lifecycle_repair_note: "Adopted into refreshed Phase 08 gap-closure lifecycle on 2026-04-24; original generated_at retained."
duration: 7m 05s
completed: 2026-04-22
---

# Phase 08 Plan 02: RPC Contract Layer Summary

**Managed RPC adapter projections plus typed JSON-RPC method, config, error, envelope, and runtime-context contracts for the supported Phase 8 surface**

## Performance

- **Duration:** 7m 05s
- **Started:** 2026-04-22T22:53:34-05:00
- **Completed:** 2026-04-22T23:00:39-05:00
- **Tasks:** 2
- **Files modified:** 13

## Accomplishments

- Added RPC-facing projection and command helpers to `ManagedPeerNetwork` and `ManagedWallet` so later handlers no longer need raw `peer_manager()` or `.wallet()` access.
- Defined the supported Phase 8 JSON-RPC method registry, typed envelopes, centralized error mapping, runtime config contracts, and the initial auth model in `open-bitcoin-rpc`.
- Added `ManagedRpcContext` as the shell-owned composition root over `ManagedPeerNetwork<MemoryChainstateStore>` and `ManagedWallet<MemoryWalletStore>`.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add adapter-owned RPC projection and command helpers before dispatcher work** - `ce342fa` (feat)
2. **Task 2: Define the shared typed envelope, method, config, error, and runtime-context contracts** - `5c380de` (feat)

## Files Created/Modified

- `packages/open-bitcoin-node/src/network.rs` - Adds chainstate, mempool, and network projection helpers plus tracked peer metadata for later RPC handlers.
- `packages/open-bitcoin-node/src/network/tests.rs` - Proves the managed network seam exposes the supported Phase 8 info surface without raw peer-manager reach-through.
- `packages/open-bitcoin-node/src/wallet.rs` - Adds wallet info, balance, UTXO, build, and sign helpers over the managed wallet facade.
- `packages/open-bitcoin-rpc/src/{lib.rs,error.rs,envelope.rs,method.rs,config.rs,context.rs}` - Defines the typed RPC contract layer and the seam-backed runtime context.
- `packages/open-bitcoin-rpc/src/{envelope,method,config,context}/tests.rs` - Covers the supported method set, extension flags, local single-wallet defaults, and managed context composition.

## Decisions Made

- Kept auth failures as transport-level `401 Unauthorized` outcomes with no JSON-RPC code, following the Knots `httprpc.cpp` behavior.
- Used `WalletRuntimeScope::LocalOperatorSingleWallet` to make the initial no-multiwallet scope explicit instead of leaving wallet routing as an ad hoc config detail.
- Left the typed RPC contract independent from transport parsing details so later dispatcher and CLI plans can consume the same surface.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- Repo-managed verification hooks regenerated `MODULE.bazel.lock` during commit-time Bazel or coverage checks. The lockfile drift was kept out of scope and will be restored before final closeout.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 8 can now add dispatcher, transport, and CLI behavior against a fixed managed seam and one typed contract layer.
- No code blockers remain for the next Phase 8 plans. The only incidental cleanup is keeping generated `MODULE.bazel.lock` drift out of scoped commits.

## Self-Check: PASSED

- `FOUND: .planning/phases/08-rpc-cli-and-config-parity/08-02-SUMMARY.md`
- `FOUND: ce342fa`
- `FOUND: 5c380de`
