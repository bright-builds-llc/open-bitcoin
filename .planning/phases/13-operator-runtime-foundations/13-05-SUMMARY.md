---
phase: 13-operator-runtime-foundations
plan: "05"
subsystem: config
tags: [jsonc, config, precedence, cli-05, cli-06]
provides:
  - Open Bitcoin JSONC config ownership contract
  - Config precedence model and tests
affects: [CLI-05, CLI-06, rpc]
requirements-completed: [CLI-05, CLI-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 13-2026-04-26T18-50-22
generated_at: 2026-04-26T18:58:37.416Z
completed: 2026-04-26
---

# Phase 13 Plan 05: JSONC Config Ownership and Precedence

## Accomplishments

- Added `docs/architecture/config-precedence.md` with the `open-bitcoin.jsonc` ownership boundary and exact precedence order.
- Added `packages/open-bitcoin-rpc/src/config/open_bitcoin.rs` with JSONC parsing, Open Bitcoin config structs, and `ConfigPrecedence`.
- Added tests for JSONC comments/trailing commas, wizard answers, unknown-field rejection, exact precedence order, and rejection of Open Bitcoin-only keys in `bitcoin.conf`.
- Added `jsonc-parser` to the RPC crate and updated parity breadcrumb coverage.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc config::` passed.

## Notes

The JSONC contract is exported but not yet merged into the existing runtime config loader. No wizard, service lifecycle, migration mutator, or `bitcoin.conf` replacement was added.
