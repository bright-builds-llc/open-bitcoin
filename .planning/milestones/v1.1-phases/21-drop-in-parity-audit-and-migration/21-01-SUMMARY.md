---
phase: 21-drop-in-parity-audit-and-migration
plan: "01"
subsystem: migration-planner-contracts
requirements-completed: [WAL-08, MIG-03, MIG-04]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 21-2026-04-27T23-11-20
generated_at: 2026-04-28T00:35:35Z
tags:
  - migration
  - operator
  - dry-run
  - cli
  - planning
key_files:
  created:
    - packages/open-bitcoin-cli/src/operator/migration.rs
    - packages/open-bitcoin-cli/src/operator/migration/planning.rs
    - packages/open-bitcoin-cli/src/operator/migration/types.rs
    - packages/open-bitcoin-cli/src/operator/migration/tests.rs
  modified:
    - packages/open-bitcoin-cli/src/operator.rs
    - packages/open-bitcoin-cli/src/operator/runtime.rs
    - packages/open-bitcoin-cli/src/operator/tests.rs
    - docs/parity/source-breadcrumbs.json
metrics:
  completed_date: "2026-04-27"
  files_created: 4
  files_modified: 4
---

# Phase 21 Plan 01 Summary

## One-Liner

Open Bitcoin now ships an operator-owned `migrate plan` surface that stays dry-run only, explains migration tradeoffs before action lists, and keeps source-install ambiguity and secret redaction explicit.

## What Was Built

- Added `open-bitcoin migrate plan` to the operator clap tree with a dedicated
  `--source-datadir` selector instead of repurposing onboarding.
- Introduced typed migration planning models for:
  - source selection vs. manual-review-required outcomes
  - target Open Bitcoin environment summaries
  - explanation sections for benefits, tradeoffs, unsupported surfaces,
    rollback expectations, and backup requirements
  - grouped migration actions for config, files/datadir, service, wallet, and
    operator follow-up
- Added focused unit coverage for:
  - explanation and action-group rendering
  - cookie and wallet-data redaction
  - ambiguous detection behavior
  - new operator CLI routing for `open-bitcoin migrate plan`
- Split the planner into `migration.rs` plus `migration/` child modules during
  closeout so the shipped behavior stayed the same while satisfying the repo's
  production Rust file-length guard.

## Task Commits

1. **Task 1 and Task 2: add the dry-run migration planner contract and runtime wiring** — `4a2087f` `feat(21): add migration dry-run planner and parity audit`

## Verification

Passed:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_flows -- --nocapture`
- `bash scripts/verify.sh`

## Deviations from Plan

- The first implementation started as a single `migration.rs` module, but final
  repo-native verification required a behavior-preserving split into
  `migration.rs`, `migration/planning.rs`, and `migration/types.rs`.
- The same closeout pass extracted shared wallet RPC and address helpers into
  `operator/wallet_support.rs` because Phase 21 touched an already-oversized
  `operator.rs` entrypoint.

## Self-Check: PASSED

- The new operator command contract, planner shapes, and dry-run rendering all
  pass focused CLI crate tests.
- Repo-native verification passes with the planner in its final split-module
  layout.
