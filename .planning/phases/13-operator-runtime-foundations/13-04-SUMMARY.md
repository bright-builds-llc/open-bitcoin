---
phase: 13-operator-runtime-foundations
plan: "04"
subsystem: cli
tags: [clap, cli, compatibility, cli-03]
provides:
  - Clap operator command contract
  - `open-bitcoin-cli` compatibility routing boundary
affects: [CLI-03, cli]
requirements-completed: [CLI-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 13-2026-04-26T18-50-22
generated_at: 2026-04-26T18:58:37.416Z
completed: 2026-04-26
---

# Phase 13 Plan 04: Clap Operator CLI Boundary

## Accomplishments

- Added `docs/architecture/cli-command-architecture.md` documenting the `open-bitcoin` operator path and `open-bitcoin-cli` compatibility path.
- Added `packages/open-bitcoin-cli/src/operator.rs` and tests for clap parsing plus binary-name routing.
- Exported the operator contract from `open-bitcoin-cli` and added parity breadcrumb coverage.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli args::` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_flows` passed.

## Notes

The existing `open-bitcoin-cli` parser remains the RPC invocation path. No status command implementation, service action, dashboard launch, onboarding wizard, or RPC transport rewrite was added.
