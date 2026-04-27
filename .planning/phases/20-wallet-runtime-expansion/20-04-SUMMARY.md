---
phase: 20-wallet-runtime-expansion
plan: 04
subsystem: operator-status
tags: [wallet, status, onboarding, detect, dashboard]
requires:
  - phase: 20-02
    provides: durable named-wallet registry, persisted rescan jobs, restart-safe rescan recovery
provides:
  - shared wallet freshness and scan-progress status surfaced through node, CLI status, and dashboard consumers
  - format-aware, read-only wallet candidate inspection rendered in onboarding output
  - parity/docs updates describing the Phase 20 read-only inspection boundary
affects: [wallet-runtime-expansion, operator-runtime, parity]
tech-stack:
  added: []
  patterns:
    - shared wallet freshness state projected through existing status contracts
    - read-only wallet candidate metadata rendered through operator onboarding rather than hidden in detection internals
key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/status.rs
    - packages/open-bitcoin-cli/src/operator/status.rs
    - packages/open-bitcoin-cli/src/operator/status/render.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - packages/open-bitcoin-cli/src/operator/detect.rs
    - packages/open-bitcoin-cli/src/operator/detect/tests.rs
    - packages/open-bitcoin-cli/src/operator/onboarding.rs
    - packages/open-bitcoin-cli/src/operator/onboarding/tests.rs
    - packages/open-bitcoin-cli/tests/operator_binary.rs
    - packages/open-bitcoin-cli/tests/operator_flows.rs
    - docs/architecture/status-snapshot.md
    - docs/parity/catalog/wallet.md
key-decisions:
  - "Wallet completeness is now explicit through fresh/stale/partial/scanning status instead of being inferred from trusted balance alone."
  - "Read-only wallet inspection remains an onboarding/status support surface only; mutation stays deferred to Phase 21."
patterns-established:
  - "Operator-facing status and dashboard consumers reuse the shared WalletStatus model directly."
  - "Richer detection metadata flows into onboarding output with explicit product/chain/format hints and read-only cautions."
requirements-completed: [WAL-07, WAL-08]
generated_by: codex
lifecycle_mode: yolo
phase_lifecycle_id: 20-2026-04-27T09-35-46
generated_at: 2026-04-27T13:35:00Z
completed: 2026-04-27
---

# Phase 20 Plan 04: Wallet Freshness and Read-Only Inspection Summary

**Shared wallet freshness/scanning status and operator-visible read-only wallet inspection landed across node, CLI status/dashboard, onboarding, and parity docs.**

## Accomplishments

- Expanded the shared `WalletStatus` model so operators can distinguish `fresh`, `stale`, `partial`, and `scanning` wallet states, plus scan progress when a rescan is active.
- Surfaced the new wallet freshness model through CLI status renderers and the dashboard model without inventing renderer-local DTOs.
- Deepened read-only wallet candidate detection with product confidence and chain-scope hints, then rendered that metadata in onboarding output with explicit read-only cautions.
- Updated parity and architecture docs so the operator-visible wallet inspection boundary is auditable and does not overclaim Phase 21 mutation support.

## Task Commits

1. **Task 1: Expand shared wallet freshness and scanning status through status and dashboard projections** — `732dc24` `feat(20-04): surface wallet freshness in shared status`
2. **Task 2: Upgrade read-only external wallet inspection to format-aware classification and backup-planning metadata** — `24a4280` `feat(20-04): expose read-only wallet inspection metadata`

## Verification

Passed:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node status:: -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::status:: -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::dashboard::model:: -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::detect:: -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::onboarding:: -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary open_bitcoin_onboard_non_interactive_is_idempotent -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli -p open-bitcoin-node --all-features`
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-cli -p open-bitcoin-node --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml -p open-bitcoin-cli -p open-bitcoin-node --all-features`
- `bun run scripts/check-parity-breadcrumbs.ts --check`

Blocked outside this plan’s direct scope:

- `bash scripts/verify.sh` now gets past LOC and breadcrumb checks but still fails the repo-wide production Rust file-length rule on several oversized Phase 20 files:
  - `packages/open-bitcoin-wallet/src/descriptor.rs`
  - `packages/open-bitcoin-rpc/src/dispatch.rs`
  - `packages/open-bitcoin-rpc/src/context.rs`
  - `packages/open-bitcoin-node/src/sync.rs`
  - `packages/open-bitcoin-rpc/src/method.rs`
  - `packages/open-bitcoin-cli/src/operator/status.rs`

## Deviations / Notes

- The original end-to-end onboarding binary assertion expected the older generic `uncertain` messaging. It was updated to the new Phase 20 contract that surfaces explicit detection `confidence=` metadata instead.
- The old deferred-surface operator flow test still treated `sendtoaddress` and `-rpcwallet` as Phase 8 deferrals. It was narrowed so only still-deferred surfaces remain in that test, while the active Phase 20 wallet routes are covered by the new transport/send flows.
- A stale breadcrumb block in `packages/open-bitcoin-node/src/wallet_registry.rs` and repeated LOC report drift were cleared with the repo-owned generators so 20-04 verification reflected the real behavioral status of this wave.

## Next Phase Readiness

- The operator-facing wallet truth model is now ready for the final Phase 20 operator workflow and backup-export work in Plan 20-05.
- Full repo-native phase closeout is still blocked by the cross-file length gate in `bash scripts/verify.sh`, so final Phase 20 verification/push cannot be called clean until those oversized Rust files are addressed or deliberately split.
