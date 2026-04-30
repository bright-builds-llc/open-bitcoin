---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 24-2026-04-28T17-43-00
generated_at: 2026-04-28T17:47:00.000Z
---

# Phase 24 Research

## Audit Findings Being Closed

### `INT-02`

- **Problem:** The live status path always calls `getwalletinfo` and
  `getbalances` on the root RPC endpoint and maps any wallet-scope failure to
  `NodeRuntimeState::Unreachable`.
- **Evidence:** `.planning/v1.1-MILESTONE-AUDIT.md` cites
  `packages/open-bitcoin-cli/src/operator/status.rs`,
  `packages/open-bitcoin-cli/src/operator/status/http.rs`, and
  `packages/open-bitcoin-rpc/src/context/wallet_state.rs`.

### `INT-W02`

- **Problem:** Later runtime shells never populate the build-provenance contract
  from Phase 13; both live and stopped status snapshots always use
  `BuildProvenance::unavailable()`.
- **Evidence:** `.planning/v1.1-MILESTONE-AUDIT.md` cites
  `packages/open-bitcoin-cli/src/operator/status.rs`.

## Existing Code Facts

1. `collect_live_status_snapshot()` in `packages/open-bitcoin-cli/src/operator/status.rs`
   already separates node-scoped RPC collection from service and rendering
   projection, so it is the right seam for repairing wallet-only degradation.
2. `HttpStatusRpcClient::call()` in
   `packages/open-bitcoin-cli/src/operator/status/http.rs` currently ignores the
   JSON-RPC `error` object and assumes success bodies always contain `result`.
3. The RPC server's wallet-selection logic in
   `packages/open-bitcoin-rpc/src/context/wallet_state.rs` emits:
   - `WalletNotFound` (`-18`) when no wallet is loaded
   - `WalletNotSpecified` (`-19`) when multiple wallets are loaded without an
     explicit wallet route
4. The operator wallet flow in
   `packages/open-bitcoin-cli/src/operator/wallet.rs` already prefers a
   selected wallet or a sole wallet from the local durable registry before
   making wallet-scoped calls.
5. Status and dashboard already share the same `OpenBitcoinStatusSnapshot`, so a
   repair in the shared collector automatically reaches both surfaces.

## Constraints From Guidance

- `AGENTS.md` requires `bash scripts/verify.sh` as the repo-native verification
  contract.
- Bright Builds verification guidance requires a sync-first workflow and
  relevant repo-native verification before commit.
- Bright Builds architecture and testing guidance favor a pure helper boundary
  for wallet-route resolution and build-provenance assembly, plus focused unit
  tests over environment-heavy integration coverage.

## Chosen Implementation Strategy

### Wallet-aware live status

- Introduce a small typed status-wallet access enum with three states:
  - root RPC access
  - selected wallet RPC access
  - locally-known wallet ambiguity or unavailability
- Resolve that enum in the operator runtime using local Fjall wallet registry
  state only when the result is trustworthy.
- Update the HTTP status client so it can route wallet calls through
  `/wallet/<name>` when a selected or sole wallet is known.
- Parse JSON-RPC `error` payloads into typed `StatusRpcError` values so the
  collector can recognize wallet selection failures without guessing.
- Keep node reachability tied to node-scoped RPC methods only; wallet failures
  degrade wallet fields plus health signals instead.

### Build provenance

- Add `packages/open-bitcoin-cli/build.rs` to emit compile-time env vars for:
  - commit SHA
  - build time
  - target triple
  - profile
- Use `option_env!` in the status collector so Cargo builds populate the shared
  model while Bazel smoke builds still compile and gracefully fall back to
  unavailable reasons.
- Update status and dashboard build rendering so those populated fields become
  visible operator output instead of remaining hidden in JSON only.

## Verification Plan

- Focused status tests for wallet-only degradation and wallet-route resolution.
- Existing operator binary test to guard the fake-running-RPC path against false
  local-store assumptions.
- Package-wide `cargo test -p open-bitcoin-cli`.
- Repo-native `bash scripts/verify.sh` before finalization.
