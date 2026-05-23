---
phase: 35
phase_name: "Daemon Mainnet Sync Activation"
generated_by: gsd-verify-work
lifecycle_mode: yolo
phase_lifecycle_id: "35-2026-05-01T21-26-04"
generated_at: "2026-05-01T22:18:29.464Z"
status: passed
lifecycle_validated: true
---

# Phase 35 Verification

## Result

Passed. Phase 35 now provides an explicit disabled-by-default `open-bitcoind` mainnet sync activation/preflight path. It validates Open Bitcoin-only JSONC and daemon CLI activation, rejects non-mainnet or partial activation, opens `FjallNodeStore`, constructs `DurableSyncRuntime`, and reports durable best header/block heights before RPC bind without starting peer transport.

## Commands

| Command | Result | Notes |
| --- | --- | --- |
| `node "$HOME/.codex/get-shit-done/bin/gsd-tools.cjs" verify lifecycle 35 --require-plans --raw` | Passed | Returned `valid`. |
| `cargo fmt --manifest-path packages/Cargo.toml --all` | Passed | Rerun after the loader split. |
| `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` | Passed | No warnings. |
| `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` | Passed | All targets built. |
| `cargo test --manifest-path packages/Cargo.toml --all-features` | Passed | Full suite passed; live sync smoke remains explicitly ignored without `OPEN_BITCOIN_LIVE_SYNC_SMOKE=1`. |
| `bash scripts/verify.sh` | Passed | Initial runs caught stale LOC and loader line cap; final run passed after regenerating `docs/metrics/lines-of-code.md` and splitting loader helpers. |
| `node "$HOME/.codex/get-shit-done/bin/gsd-tools.cjs" validate health` | Degraded, known | No errors. Warnings are known archive-layout/future-phase tool limitations: archived v1.1 phases are not in root `.planning/phases`, and future Phases 36-40 are in the roadmap before their phase directories exist. |
| `git diff --check` | Passed | No whitespace errors. |

## Evidence

- Config defaults keep daemon sync disabled.
- Open Bitcoin JSONC activation requires both `sync.network_enabled = true` and `sync.mode = "mainnet-ibd"`.
- `-openbitcoinsync=mainnet-ibd` enables the daemon preflight and `-openbitcoinsync=disabled` can override enabled JSONC.
- `bitcoin.conf` remains strict and does not accept Open Bitcoin-only sync keys.
- Enabled daemon preflight opens the durable store and constructs `DurableSyncRuntime` before RPC bind.
- Operator, architecture, parity, README, AGENTS, and planning docs now describe Phase 35 as activation/preflight only.

## Residual Risks

- Phase 35 does not resolve DNS seeds, open outbound peers, run header-first IBD, download/connect blocks, or prove live mainnet progress.
- Long-lived daemon sync cancellation and graceful shutdown remain later-phase work because Phase 35 does not start a sync task.
