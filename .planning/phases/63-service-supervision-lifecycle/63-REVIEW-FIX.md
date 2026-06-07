---
phase: 63-service-supervision-lifecycle
fixed_at: 2026-06-07T19:09:07Z
review_path: .planning/phases/63-service-supervision-lifecycle/63-REVIEW.md
status: all_fixed
findings_in_scope: 1
fixed: 1
skipped: 0
iteration: 1
---

# Phase 63: Code Review Fix Report

## Scope

Fixed warning findings from `63-REVIEW.md`.

## Fixes Applied

### WR-01: Generated service files passed operator CLI flags to `open-bitcoind`

Updated launchd and systemd service generators to render daemon-compatible argv tokens:

- `-datadir=<path>`
- `-openbitcoinconf=<path>`

Updated the service generator tests to reject `--datadir` and `--config` in generated service content, and extended the Phase 63 lifecycle checker with the same guard.

## Verification

- `cargo fmt --all --manifest-path packages/Cargo.toml`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features service::tests`
- `bun run scripts/check-phase63-service-lifecycle.ts`
