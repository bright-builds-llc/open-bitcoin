# open-bitcoin

<!-- bright-builds-rules-readme-badges:begin -->

<!-- Managed upstream by bright-builds-rules. If this badge block needs a fix, open an upstream PR or issue instead of editing the downstream managed block. Keep repo-local README content outside this managed badge block. -->

[![GitHub Stars](https://img.shields.io/github/stars/bright-builds-llc/open-bitcoin)](https://github.com/bright-builds-llc/open-bitcoin)
[![CI](https://img.shields.io/github/actions/workflow/status/bright-builds-llc/open-bitcoin/ci.yml?style=flat-square&logo=github&label=CI)](https://github.com/bright-builds-llc/open-bitcoin/actions/workflows/ci.yml)
[![License](https://img.shields.io/github/license/bright-builds-llc/open-bitcoin?style=flat-square)](./LICENSE)
[![Rust 1.94.1](https://img.shields.io/badge/Rust-1.94.1-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Bright Builds: Rules](https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/main/public/badges/bright-builds-rules-flat.svg)](https://github.com/bright-builds-llc/bright-builds-rules)

<!-- bright-builds-rules-readme-badges:end -->

Open Bitcoin is a headless Bitcoin node and wallet implementation in Rust. It is being built to preserve externally observable behavior from Bitcoin Knots `29.3.knots20260210` across the in-scope consensus, validation, chainstate, mempool, networking, wallet, RPC, CLI, and configuration surfaces while keeping first-party internals more strongly typed, auditable, and modular.

> Status: Phase 1, "Workspace, Baseline, and Guardrails," is complete. The repository is still in its scaffold and foundation stage, and there is not yet a runnable Open Bitcoin node, wallet, RPC server, or CLI implementation. The current first-party crates establish workspace, architecture, and verification boundaries for later phases.

## Why This Exists

- Behavioral parity matters more than source-level mimicry. The goal is to match the pinned Knots baseline on the outside, not to port its C++ code line by line.
- The project uses functional core and imperative shell boundaries so pure domain logic stays separate from I/O, runtime side effects, and adapter code.
- Parity claims are meant to be auditable. Intentional differences from the baseline are supposed to be explicit and documented rather than tribal knowledge.
- The initial milestone is headless by design. GUI work is deferred until the node and wallet core are further along.

## What Exists Today

- `packages/bitcoin-knots/` vendors the pinned Bitcoin Knots baseline used as the external behavioral reference.
- `packages/open-bitcoin-core/` is the initial pure-core Rust crate, currently a scaffold for future domain logic.
- `packages/open-bitcoin-node/` is the initial shell/runtime crate, currently a scaffold for future adapters and orchestration.
- The repository has both a Rust workspace under `packages/` and top-level Bazelisk/Bazel+Bzlmod workspace scaffolding for first-party code.
- `rust-toolchain.toml` pins Rust `1.94.1` as the shared Cargo, CI, and Bazel toolchain target.
- `bash scripts/verify.sh` is the repo-native verification entrypoint for format, lint, build, tests, the Bazel smoke build, architecture-policy enforcement, and the current pure-core coverage gate.
- `docs/parity/` contains the seeded parity and deviation ledger, with all in-scope surfaces currently marked as `planned`.

## What Is Next

Phase 2 has not started yet. The next step is building the typed core domain and serialization foundations that later work depends on.

After that, the roadmap layers in consensus validation, chainstate and UTXO behavior, mempool policy, P2P networking and sync, wallet behavior, RPC and CLI/config parity, and finally the parity harnesses, fuzzing, benchmarks, and audit artifacts needed to make parity claims defensible.

## Repository Layout

- `packages/bitcoin-knots/` is the pinned upstream behavioral baseline. Treat it as the reference implementation, not the first-party production path.
- `packages/open-bitcoin-core/` is the pure-core Rust crate that will hold domain logic and stay free of direct I/O and runtime side effects.
- `packages/open-bitcoin-node/` is the shell/runtime Rust crate that will own adapters, orchestration, and other effectful boundaries.
- `docs/parity/` tracks parity status and intentional deviations from the pinned baseline.
- `.githooks/` contains the repo-managed Git hooks used to run the local verification contract before commit.
- `scripts/verify.sh` is the source-of-truth local verification command for first-party code.

## Contributor Quickstart

Materialize the pinned reference baseline:

```bash
git submodule update --init --recursive
```

Install the repo-managed Git hooks once per clone:

```bash
bash scripts/install-git-hooks.sh
```

Run the repo-native verification flow:

```bash
bash scripts/verify.sh
```

For contributor workflow details beyond those two entrypoints, see [CONTRIBUTING.md](./CONTRIBUTING.md).

## Parity And Deviations

- [`docs/parity/README.md`](./docs/parity/README.md) explains what the parity ledger is for and how intentional deviations from Bitcoin Knots should be recorded.
- [`docs/parity/index.json`](./docs/parity/index.json) is the machine-readable status index for the in-scope surfaces. It currently records the baseline version and marks each tracked surface as `planned`.
