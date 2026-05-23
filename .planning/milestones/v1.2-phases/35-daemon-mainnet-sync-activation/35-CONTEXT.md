---
phase: 35
phase_name: "Daemon Mainnet Sync Activation"
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: "35-2026-05-01T21-26-04"
generated_at: "2026-05-01T21:29:26.254Z"
---

# Phase 35 Context: Daemon Mainnet Sync Activation

## Goal

Activate an explicit, operator-visible mainnet sync bootstrap path in `open-bitcoind` without claiming full unattended public-mainnet IBD yet. This phase should let the daemon accept an Open Bitcoin-only opt-in, validate it safely, open the durable sync runtime against the configured datadir, and expose clear startup status/error behavior that later phases can extend into live networking and block connection.

## Non-Goals

- Do not start outbound peer transport from `open-bitcoind`.
- Do not implement DNS resolution, peer lifecycle management, headers-first IBD, block download, recovery loops, or live smoke validation.
- Do not make public mainnet sync the default.
- Do not write Open Bitcoin-only settings into `bitcoin.conf`.
- Do not change historical v1.1 phase artifacts except where current-facing docs need a link or status correction.

## Yolo Decisions

| Decision | Selected Approach | Rationale |
| --- | --- | --- |
| Activation boundary | Add an Open Bitcoin-only sync activation layer consumed by `open-bitcoind`, default disabled. | Avoids stale operator claims and keeps public-network behavior explicitly opt-in. |
| Config source | Read sync activation from `open-bitcoin.jsonc` plus a daemon CLI override. | Existing architecture reserves Open Bitcoin-only sync settings for JSONC and CLI flags sit at highest precedence. |
| Safety gate | Require mainnet chain plus explicit `network_enabled = true` and `mode = "mainnet-ibd"`. | Prevents accidental public-network activation from defaults or partial config. |
| Daemon behavior | When enabled, open `FjallNodeStore`, construct `DurableSyncRuntime`, and produce a pre-sync summary before RPC startup. | Verifies durable runtime wiring without over-claiming live sync. |
| Error copy | Fail startup with typed, actionable errors for missing datadir, non-mainnet chain, invalid mode, or store/runtime open failure. | Operators need deterministic failure before the daemon listens. |
| Documentation | Update current operator/planning/parity docs to describe this as activation/preflight foundation only. | Keeps docs accurate until Phases 36-40 complete the full sync workflow. |

## Acceptance Criteria

- `RuntimeConfig::default()` leaves daemon sync disabled.
- `open-bitcoin.jsonc` can enable Phase 35 mainnet sync only with both `sync.network_enabled = true` and `sync.mode = "mainnet-ibd"`.
- A daemon CLI override can enable or disable the same runtime mode without using `bitcoin.conf`.
- Non-mainnet chain activation is rejected before daemon RPC bind.
- `open-bitcoind` enabled mode opens the configured durable store and constructs `DurableSyncRuntime`, but does not invoke transport or start network sync.
- Operator docs explain the exact opt-in command/config shape and still state that unattended public-mainnet IBD is not complete.
- Verification includes lifecycle validation, Rust format/lint/build/test, repo-native verification, and final diff review before commit/push.

## References Read

- `AGENTS.md`
- `AGENTS.bright-builds.md`
- `standards-overrides.md`
- Bright Builds `standards/index.md`
- Bright Builds `standards/core/architecture.md`
- Bright Builds `standards/core/code-shape.md`
- Bright Builds `standards/core/verification.md`
- Bright Builds `standards/core/testing.md`
- Bright Builds `standards/languages/rust.md`
- `.planning/PROJECT.md`
- `.planning/REQUIREMENTS.md`
- `.planning/ROADMAP.md`
- `.planning/STATE.md`
- `.planning/ARCHITECTURE.md`
- `.planning/STACK.md`
- `.planning/CONVENTIONS.md`
- `.planning/milestones/v1.1-phases/15-real-network-sync-loop/15-CONTEXT.md`
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs`
- `packages/open-bitcoin-rpc/src/config.rs`
- `packages/open-bitcoin-rpc/src/config/loader.rs`
- `packages/open-bitcoin-rpc/src/config/open_bitcoin.rs`
- `packages/open-bitcoin-rpc/src/config/tests.rs`
- `packages/open-bitcoin-node/src/sync.rs`
- `packages/open-bitcoin-node/src/sync/types.rs`
- `packages/open-bitcoin-node/src/storage/fjall_store.rs`
- `docs/operator/runtime-guide.md`
- `docs/parity/source-breadcrumbs.json`
