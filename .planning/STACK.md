# Open Bitcoin Stack

Last updated: 2026-05-01

## Core Runtime

- Rust `1.94.1`, pinned by `rust-toolchain.toml`, with Rust 2024 edition for
  first-party crates.
- Cargo workspace under `packages/Cargo.toml`; first-party packages are split by
  primitives, codec, consensus, chainstate, mempool, network, wallet, node, RPC,
  CLI, test harness, and benchmarks.
- Bitcoin Knots `29.3.knots20260210` is vendored under
  `packages/bitcoin-knots/` as the external behavior baseline.

## Build And Verification

- Bazel/Bzlmod remains the top-level build smoke surface through `MODULE.bazel`
  and `rules_rust`.
- `bash scripts/verify.sh` is the repo-native verification contract. It runs
  Bun-backed TypeScript checks, pure-core guards, Rust format/lint/build/test,
  benchmark smoke/report validation, Bazel smoke builds, build provenance
  checks, and pure-core coverage.
- Repo-managed Git hooks live under `.githooks/` and can be installed with
  `bash scripts/install-git-hooks.sh`.

## Automation Runtime

- Bun is pinned by `.bun-version` and is used as the runtime for repo-owned
  TypeScript automation in `scripts/`.
- This repository does not have a `package.json`; local setup should verify Bun
  availability with `bun --version`, not run `bun install`.
- Substantial repo-owned automation should remain TypeScript run by Bun unless a
  concrete compatibility reason justifies another runtime.

## Runtime Dependencies

- `fjall` backs the v1.1 durable node store.
- `tokio` and `axum` back the current JSON-RPC server runtime.
- `clap` owns the Open Bitcoin operator command tree.
- `ratatui` and `crossterm` back the terminal dashboard.
- `jsonc-parser` parses Open Bitcoin-owned JSONC config.
- `serde` and `serde_json` are used for stable runtime, RPC, report, and config
  shapes.
- `secp256k1` is the cryptographic dependency used by consensus and wallet
  code.
- `ureq`, `base64`, and `getrandom` support CLI/RPC transport, auth, and local
  runtime utilities.

## Current Product Boundary

- The shipped milestones are headless and terminal-first.
- `open-bitcoind` exposes the local JSON-RPC server runtime and an opt-in
  mainnet sync activation preflight that opens the durable store and constructs
  `DurableSyncRuntime`.
- `open-bitcoind` does not yet drive unattended public-mainnet full sync through
  peer transport, headers-first IBD, and block connect loops.
- Public-network sync remains an opt-in review/runtime foundation and benchmark
  evidence surface, not part of the default local verification gate.
