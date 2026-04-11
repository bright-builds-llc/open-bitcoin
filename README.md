# open-bitcoin

<!-- bright-builds-rules-readme-badges:begin -->
<!-- Managed upstream by bright-builds-rules. If this badge block needs a fix, open an upstream PR or issue instead of editing the downstream managed block. Keep repo-local README content outside this managed badge block. -->
[![GitHub Stars](https://img.shields.io/github/stars/bright-builds-llc/open-bitcoin)](https://github.com/bright-builds-llc/open-bitcoin)
[![License](https://img.shields.io/github/license/bright-builds-llc/open-bitcoin?style=flat-square)](./LICENSE)
[![Bright Builds: Rules](https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/main/public/badges/bright-builds-rules-flat.svg)](https://github.com/bright-builds-llc/bright-builds-rules)
<!-- bright-builds-rules-readme-badges:end -->

Open Bitcoin is a headless Bitcoin node and wallet implementation in Rust. It is being built to preserve externally observable behavior from Bitcoin Knots `29.3.knots20260210` across the in-scope node, wallet, RPC, CLI, and configuration surfaces while keeping first-party internals more strongly typed and modular.

## Repository Layout

- `packages/bitcoin-knots` is the pinned upstream baseline.
- `packages/open-bitcoin-core` is the pure-core Rust crate for domain logic.
- `packages/open-bitcoin-node` is the shell/runtime Rust crate for adapters and orchestration.
- `docs/parity/` tracks parity status and intentional deviations from the pinned baseline.
