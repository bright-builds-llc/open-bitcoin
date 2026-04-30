---
phase: 20-wallet-runtime-expansion
plan: "05"
subsystem: wallet-operator-and-closeout
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 20-2026-04-27T09-35-46
generated_at: 2026-04-27T21:27:05Z
tags:
  - wallet
  - operator
  - backup
  - parity
  - verification
key_files:
  created: []
  modified:
    - packages/open-bitcoin-cli/src/operator.rs
    - packages/open-bitcoin-cli/src/operator/runtime.rs
    - packages/open-bitcoin-cli/src/operator/wallet.rs
    - packages/open-bitcoin-cli/src/operator/wallet/tests.rs
    - packages/open-bitcoin-cli/tests/operator_binary.rs
    - docs/architecture/cli-command-architecture.md
    - docs/parity/catalog/wallet.md
    - docs/parity/index.json
    - docs/parity/source-breadcrumbs.json
    - README.md
metrics:
  completed_date: "2026-04-27"
  files_created: 0
  files_modified: 10
---

# Phase 20 Plan 05 Summary

## One-Liner

Open Bitcoin now ships an operator-owned wallet workflow for preview/confirm send and one-way managed-wallet backup export, with parity/docs updated to describe the exact shipped wallet slice and its explicit deferrals.

## What Was Built

- Added `open-bitcoin wallet send` as an Open Bitcoin-owned wrapper over the wallet-scoped `sendtoaddress` path.
  It renders a deterministic preview from the shared send-intent model and refuses mutation unless `--confirm` is present.
- Added `open-bitcoin wallet backup` as a one-way managed-wallet export flow that refuses destinations overlapping detected external Core/Knots wallet candidates.
- Extended operator binary coverage for:
  - preview-before-confirm send behavior
  - confirmed send submission through the wallet-scoped RPC path
  - backup export success
  - unsafe backup destination rejection
- Updated contributor and parity docs so the shipped Phase 20 wallet surface is explicit:
  - `-rpcwallet` and `/wallet/<name>` routing
  - ranged single-key descriptor limits
  - wallet freshness/scanning visibility
  - Open Bitcoin-owned backup/export boundary
  - deferred multiwallet/migration surfaces

## Task Commits

1. **Task 1: Add operator wallet workflows for preview/confirm send and Open Bitcoin backup export** — `0391fa4` `feat(20-05): add operator wallet workflows`
2. **Task 1 follow-up: split wallet operator support to satisfy repo verification rules** — `68363a9` `fix(20-05): split wallet operator support for verify rules`
3. **Closeout blocker removal: split oversized modules and restore repo-native verification** — `241ae15` `refactor(wallet,rpc,node): split oversized modules and restore verify`

## Verification

Passed:

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::wallet:: -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary -- --nocapture`
- `bun run scripts/check-parity-breadcrumbs.ts --check`
- `bash scripts/verify.sh`

## Deviations from Plan

- The initial 20-05 implementation was functionally correct, but final closeout was blocked by repo-native verifier gates outside the immediate operator-wallet surface:
  - stale generated LOC report
  - oversized production Rust files
  - missing-lines coverage in the wallet descriptor refactor
- The follow-up refactor commit resolved those repo-wide gates without changing the shipped Phase 20 wallet behavior.

## Self-Check: PASSED

- Operator wallet feature tests and binary tests pass.
- Repo-native `bash scripts/verify.sh` passes after the final refactor/coverage cleanup.
