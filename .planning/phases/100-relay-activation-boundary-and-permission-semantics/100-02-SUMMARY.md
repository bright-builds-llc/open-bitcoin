---
phase: 100-relay-activation-boundary-and-permission-semantics
plan: 02
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 100-2026-06-29T16-18-03
generated_at: 2026-06-29T19:05:20Z
subsystem: rpc-config-relay-activation
tags: [relay-activation, config, cli, parser, v2.0]
key-files:
  created:
    - .planning/phases/100-relay-activation-boundary-and-permission-semantics/100-02-SUMMARY.md
  modified:
    - docs/metrics/lines-of-code.md
    - packages/open-bitcoin-rpc/src/config.rs
    - packages/open-bitcoin-rpc/src/config/loader.rs
    - packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs
    - packages/open-bitcoin-rpc/src/config/open_bitcoin.rs
    - packages/open-bitcoin-rpc/src/config/tests.rs
requirements-completed: [ACT-01, ACT-02]
duration: 25m
completed: 2026-06-29
---

# Phase 100 Plan 02: Relay Activation Config Summary

**Plan 100-02 wires default-off relay activation into Open Bitcoin-owned JSONC, CLI, and resolved runtime config without changing peer sockets, mempool behavior, service bits, or public status.**

## Performance

- **Duration:** 25m
- **Started:** 2026-06-29T18:39:56Z
- **Completed:** 2026-06-29T19:05:20Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- Added JSONC `relay.enabled` via `RelayConfig`, defaulting to false with `deny_unknown_fields`.
- Added `RuntimeConfig.relay: RelayActivationConfig` and loader resolution from Open Bitcoin JSONC/defaults.
- Added `-openbitcoinrelay`, `-openbitcoinrelay=1`, `-openbitcoinrelay=0`, and `-noopenbitcoinrelay` parsing through the existing boolean parser.
- Preserved deterministic CLI-over-JSONC precedence for relay activation.
- Kept Knots whitelist/whitebind-style relay shortcuts rejected; no aliases were added for `whitelist`, `whitebind`, `whitelistrelay`, or `whitelistforcerelay`.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add JSONC `relay.enabled` and typed runtime config** - `a3ee07c5`
2. **Task 2: Add `-openbitcoinrelay` CLI override without Knots shortcuts** - `9a52a555`

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all --check` passed.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features -- -D warnings` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --no-run` passed.
- `timeout 90s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib open_bitcoin_jsonc_defaults_relay_activation_to_disabled -- --nocapture` passed.
- `timeout 90s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib open_bitcoin_jsonc_accepts_relay_activation_enabled -- --nocapture` passed.
- `timeout 90s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib open_bitcoin_jsonc_rejects_unknown_relay_fields -- --nocapture` passed.
- `timeout 90s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib daemon_relay_cli -- --nocapture` passed.
- `timeout 90s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib daemon_inbound_rejects_baseline_listener_and_permission_keys -- --nocapture` passed.
- Acceptance `rg` checks passed for `RelayConfig`, `RuntimeConfig.relay`, focused tests, no production `PeerManager`/`LocalPeerConfig` relay wiring, baseline whitelist/whitebind rejection coverage, and no `openbitcoinrelay` match in `config/loader/inbound.rs`.
- Rust pre-commit sequence passed before the Task 1 and Task 2 commits: `cargo fmt --manifest-path packages/Cargo.toml --all`, `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`, and `cargo test --manifest-path packages/Cargo.toml --all-features`.
- Repo-native commit hook verification passed through `bash scripts/verify.sh` for both task commits; the hooks completed in 3m 22.800s and 3m 21.716s.

## Deviations

- The implementation did not need the broader mechanical `RuntimeConfig` literal updates anticipated by the plan because existing helpers and constructors absorbed the new runtime field.
- The commit hooks regenerated and staged `docs/metrics/lines-of-code.md`, which is an intentionally tracked generated artifact for this repo.

## Self-Check: PASSED

- [x] `relay.enabled` defaults to false in JSONC and resolved runtime config.
- [x] Unknown relay JSONC fields are rejected.
- [x] `-openbitcoinrelay` is the only new CLI relay activation override.
- [x] CLI values override JSONC true and false deterministically.
- [x] Knots whitelist/whitebind compatibility inputs remain rejected.
- [x] No peer socket, mempool, service-bit, or public status behavior was wired in this plan.
