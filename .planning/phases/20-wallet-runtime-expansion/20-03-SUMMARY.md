---
phase: 20-wallet-runtime-expansion
plan: 03
subsystem: rpc
tags: [wallet, rpc, cli, routing, multiwallet, descriptors]
requires:
  - phase: 20-02
    provides: durable named-wallet registry, selected-wallet metadata, persisted rescan jobs
provides:
  - CLI `-rpcwallet` transport metadata routed to wallet-scoped RPC endpoints
  - HTTP root-vs-wallet RPC scope enforcement at `/` and `/wallet/<name>`
  - wallet RPC subset for send, address allocation, descriptor listing, expanded wallet info, and range-aware rescans
affects: [wallet-runtime-expansion, operator-runtime, rpc-cli-config]
tech-stack:
  added: []
  patterns: [wallet scope via URI path, manual JSON enrichment for compatibility, durable registry-backed RPC wallet orchestration]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-cli/src/args.rs
    - packages/open-bitcoin-cli/src/client.rs
    - packages/open-bitcoin-rpc/src/context.rs
    - packages/open-bitcoin-rpc/src/http.rs
    - packages/open-bitcoin-rpc/src/method.rs
    - packages/open-bitcoin-rpc/src/dispatch.rs
key-decisions:
  - "Keep wallet selection in transport metadata and URI routing instead of request JSON payloads."
  - "Preserve the typed `GetWalletInfoResponse` shape for downstream callers and append Phase 20 freshness metadata at JSON serialization time."
  - "Resolve `conf_target` and `estimate_mode` in the RPC shell into deterministic fee rates before reusing the shared build-and-sign spend path."
patterns-established:
  - "Wallet-scoped CLI and HTTP routing chooses `/wallet/<name>` only for wallet methods; node methods stay on `/`."
  - "Durable wallet RPC tests use Fjall-backed temp stores and explicit request wallet selection."
requirements-completed: [WAL-04, WAL-05, WAL-06, WAL-07]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 20-2026-04-27T09-35-46
generated_at: 2026-04-27T11:48:21Z
duration: 24m
completed: 2026-04-27
---

# Phase 20 Plan 03: Wallet Runtime Routing and Practical RPC Summary

**Wallet-scoped RPC transport, durable wallet URI selection, and a practical send/address/descriptor/rescan wallet RPC slice on the Phase 20 registry model**

## Performance

- **Duration:** 24m
- **Started:** 2026-04-27T11:25:05Z
- **Completed:** 2026-04-27T11:48:21Z
- **Tasks:** 2
- **Files modified:** 12

## Accomplishments

- Un-deferred `-rpcwallet` so CLI transport keeps wallet selection out of RPC JSON and routes wallet methods to `/wallet/<name>`.
- Added root-vs-wallet HTTP scope enforcement with durable registry-backed wallet lookup and explicit multiwallet selection failures.
- Implemented the Phase 20 wallet RPC subset: `sendtoaddress`, `getnewaddress`, `getrawchangeaddress`, `listdescriptors`, expanded `getwalletinfo` metadata, and range-aware `rescanblockchain`.

## Task Commits

1. **Task 1: Un-defer wallet-scoped transport routing through `-rpcwallet` and `/wallet/<name>`** - `9b8bb40` (`feat`)
2. **Task 2: Implement the minimal wallet RPC method subset on the new registry and ranged-descriptor model** - `a4acd93` (`feat`)

## Files Created/Modified

- `packages/open-bitcoin-cli/src/args.rs` - accepts `-rpcwallet` as startup transport metadata
- `packages/open-bitcoin-cli/src/client.rs` - routes wallet methods to wallet-scoped endpoints while keeping node methods at root
- `packages/open-bitcoin-rpc/src/context.rs` - adds durable wallet selection, wallet freshness, and persisted range-rescan orchestration
- `packages/open-bitcoin-rpc/src/http.rs` - parses `/wallet/<name>` paths and enforces root-vs-wallet method scope
- `packages/open-bitcoin-rpc/src/method.rs` - defines the Phase 20 wallet RPC method/request surface
- `packages/open-bitcoin-rpc/src/dispatch.rs` - implements send, address allocation, descriptor listing, expanded wallet info, and range-aware rescans
- `packages/open-bitcoin-rpc/src/*/tests.rs` and `packages/open-bitcoin-cli/src/*/tests.rs` - hermetic transport and wallet-surface coverage

## Decisions Made

- Manual JSON enrichment on `getwalletinfo` avoided breaking downstream typed consumers outside this plan’s write set while still exposing wallet name and freshness metadata.
- `sendtoaddress` resolves estimator inputs through a deterministic RPC-shell fee policy and then calls the existing build-and-sign path instead of introducing a second spending pipeline.
- Range rescans persist durable wallet rescan-job state even when execution completes synchronously in the RPC shell, so later runtime consumers see the same registry-backed semantics.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Repaired half-integrated durable wallet RPC changes already present in the worktree**
- **Found during:** Task 1
- **Issue:** `context.rs`/`dispatch.rs` had compile-breaking partial durable-wallet changes before transport tests could run.
- **Fix:** Completed imports, ownership handling, error mapping, and registry test setup needed for the new wallet scope path to compile and execute.
- **Files modified:** `packages/open-bitcoin-rpc/src/context.rs`, `packages/open-bitcoin-rpc/src/dispatch.rs`, `packages/open-bitcoin-rpc/src/http/tests.rs`
- **Verification:** targeted CLI/RPC transport tests and later package clippy/build/test passes
- **Committed in:** `9b8bb40`, `a4acd93`

**2. [Rule 3 - Blocking] Preserved `GetWalletInfoResponse` compatibility for downstream callers**
- **Found during:** Task 2 / repo-wide test pass
- **Issue:** adding fields directly to the typed response broke `open-bitcoin-cli` tests outside the plan write set.
- **Fix:** restored the typed struct shape and appended Phase 20 wallet metadata in RPC JSON serialization instead.
- **Files modified:** `packages/open-bitcoin-rpc/src/method.rs`, `packages/open-bitcoin-rpc/src/dispatch.rs`
- **Verification:** targeted method/dispatch tests and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli args::`, `client::`
- **Committed in:** `a4acd93`

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** both fixes were required to land the planned wallet-routing and wallet-RPC behavior on the existing branch state without widening the write set.

## Issues Encountered

- `bun run scripts/check-parity-breadcrumbs.ts --check` is blocked by `packages/open-bitcoin-node/src/wallet_registry.rs` reporting a missing/stale breadcrumb block. That source file is outside this plan’s write set.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc -p open-bitcoin-cli --all-features` is blocked by failing `operator::detect` tests in dirty files outside this plan’s write set.
- `bash scripts/verify.sh` is blocked immediately by the pre-existing stale LOC report at `docs/metrics/lines-of-code.md`, which is also outside this plan’s write set.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Wallet-scoped routing and the practical wallet RPC slice are ready for later operator wrappers and status consumers.
- Before Phase 20 closeout can claim a fully green repo-native verifier run in this worktree, the unrelated `operator::detect` failures, stale LOC report, and stale `wallet_registry.rs` breadcrumb block need separate cleanup in their own write sets.

## Self-Check

PASSED

---
*Phase: 20-wallet-runtime-expansion*
*Completed: 2026-04-27*
