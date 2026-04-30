---
phase: 24-wallet-aware-live-status-and-build-provenance
plan: "01"
subsystem: live-status-wallet
requirements-completed: [OBS-01, OBS-02, WAL-05, DASH-01]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 24-2026-04-28T17-43-00
generated_at: 2026-04-28T19:18:00.000Z
completed: 2026-04-28
---

# Phase 24 Plan 01 Summary

## One-Liner

Live operator status no longer treats wallet selection ambiguity like node
unreachability: it now uses a selected or sole wallet when local evidence is
trustworthy and otherwise degrades only the wallet fields with wallet-specific
diagnostics.

## What Was Built

- Added a typed `StatusWalletRpcAccess` model so the status runtime can
  distinguish root access, selected-wallet access, and locally-known ambiguity
  without guessing from unrelated booleans.
- Updated the operator runtime to resolve a selected or sole managed wallet from
  the local Fjall registry when that evidence is trustworthy and pass that route
  into the status RPC client.
- Extended `HttpStatusRpcClient` to parse JSON-RPC `error` payloads and route
  wallet calls through `/wallet/<name>` when a selected wallet is known.
- Reworked the shared status collector so wallet RPC failures only mark wallet
  fields unavailable and add a wallet health signal while node, sync, mempool,
  and peer truth remain live.
- Added focused tests for wallet-only failure handling plus selected-wallet and
  ambiguous-multiwallet route resolution.

## Task Commits

1. **Task 1: repair wallet-aware live status without widening the operator
   surface** — Pending the wrapper-owned Phase 24 finalization commit.

## Verification

Passed:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli status::tests -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli -- --nocapture`

## Deviations from Plan

- An empty local wallet registry is no longer treated as proof that the live
  node has zero wallets. The phase keeps that path truthful by classifying the
  live wallet RPC error instead, which avoids false negatives for remote or
  fake RPC targets.

## Self-Check: PASSED

- Healthy node RPC data stays live even when wallet scope is ambiguous.
- Selected-wallet and sole-wallet paths continue to use wallet-routed RPC when
  local registry evidence supports it.
- Dashboard inherits the repaired wallet truth through the shared status
  collector instead of a dashboard-specific branch.
