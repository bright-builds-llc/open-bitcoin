# Quick Task 260425-kao: Parity Breadcrumb Source Anchors

Created: 2026-04-25 14:37 CDT

## Goal

Add auditable, repo-root-relative parity breadcrumb comments to all first-party Rust source and test files so contributors can jump from Open Bitcoin files to the relevant pinned Bitcoin Knots source anchors.

## Plan

- Add a machine-readable breadcrumb mapping under `docs/parity/`.
- Add a Bun script that applies and checks breadcrumb blocks for all in-scope Rust files.
- Apply the breadcrumb blocks to every `packages/open-bitcoin-*/src/**/*.rs` and `packages/open-bitcoin-*/tests/**/*.rs` file, excluding `packages/bitcoin-knots/`.
- Add VS Code-compatible link-provider support for VS Code/Cursor because VS Code only documents default editor links for `http(s)` and `file` URIs.
- Update parity docs with the breadcrumb convention and verification command.

## Verification

- Run the breadcrumb script in `--check` mode.
- Spot-check representative consensus, codec, chainstate, mempool, network, node, wallet, RPC, CLI, bench, and harness mappings.
- Run `cargo fmt --manifest-path packages/Cargo.toml --all --check`.
- Run `bash scripts/verify.sh`.
