---
phase: 36
phase_name: "Mainnet Peer Discovery and Outbound Lifecycle"
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: "36-2026-05-01T22-57-33"
generated_at: "2026-05-01T22:57:33.102Z"
---

# Phase 36 Discussion Log

## Mode

This phase was discussed in `--yolo --chain` mode from the wrapper `gsd-yolo-discuss-plan-execute-commit-and-push`. The wrapper auto-selected Phase 36 because Phase 35 is complete and Phase 36 is the first pending phase without discussion artifacts.

## Gray Areas Resolved

| Gray Area | Resolution |
| --- | --- |
| How should peer addresses be sourced? | Use injected resolver interfaces for DNS seeds and manual peers, preserving source labels and keeping tests deterministic. |
| How should the runtime manage outbound peers? | Introduce a bounded lifecycle-aware outbound peer pool instead of walking peers sequentially through `candidate_peers()`. |
| What should happen to unhealthy peers? | Track retry/backoff/stall state explicitly and rotate bad peers out when alternatives exist so one peer cannot block IBD indefinitely. |
| How should failures surface? | Use typed runtime failure reasons plus existing health-signal/structured-log patterns, not silent retries or panics. |
| What telemetry is required now? | Record peer source, resolved endpoint, negotiated network/capabilities, contribution, attempts, last activity, and terminal failure reason in the sync telemetry model. |

## Carry-Forward Constraints

- Phase 35 remains only the daemon activation/preflight owner.
- Phase 36 must keep pure-core crates free of direct DNS, socket, clock, and filesystem effects.
- Phase 37 will own header-first synchronization on top of the Phase 36 peer layer.
- Phase 39 will own rich operator-facing status, dashboard, and RPC presentation of the telemetry added here.
- Default verification must remain hermetic and must not require public mainnet access.

## Required Verification

- `node "$HOME/.codex/get-shit-done/bin/gsd-tools.cjs" verify lifecycle 36 --require-plans --raw`
- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- `bash scripts/verify.sh`
- `git diff --check`
- Final diff review before commit/push.
