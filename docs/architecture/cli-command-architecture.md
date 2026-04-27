# CLI Command Architecture

## open-bitcoin operator path

`open-bitcoin` is the clap-owned operator path for Open Bitcoin-specific workflows. The initial command contract includes:

- `status`
- `config`
- `service`
- `dashboard`
- `migrate`
- `onboard`
- `wallet`

Shared operator options are `--config`, `--datadir`, `--network`, `--format human|json`, and `--no-color`.

The Phase 21 runtime wires `status`, `config paths`, `onboard`, `migrate`, and
the operator-owned `wallet` workflows through the actual `open-bitcoin` binary.
Status renders stopped, unreachable, and live-RPC evidence through the shared
`OpenBitcoinStatusSnapshot`; onboarding writes Open Bitcoin-only answers to
`open-bitcoin.jsonc` after explicit approval and does not mutate
`bitcoin.conf`.

`migrate plan` is an Open Bitcoin-owned dry-run planner for operators with an
existing Core or Knots install. It reuses read-only detection evidence, explains
benefits and tradeoffs plus rollback and backup requirements, enumerates config,
file, service, and wallet actions, and surfaces the relevant intentional
differences from the parity ledger. It does not disable source services, rewrite
source datadirs, or import external wallets in Phase 21.

`wallet send` is intentionally not a baseline-compatible parser surface. It is an Open Bitcoin-owned wrapper that:

- resolves the managed wallet from the durable registry or `--wallet`
- builds a deterministic preview from the shared send-intent model
- refuses mutation without `--confirm`
- submits the final mutation through the existing wallet-scoped `sendtoaddress` RPC path

`wallet backup` is likewise Open Bitcoin-owned. It writes a one-way JSON export for a managed wallet snapshot and rejects destinations that overlap detected Core or Knots wallet candidates. It does not copy, rewrite, restore, or import external wallet formats.

`service` commands remain a Phase 18 boundary and `dashboard` remains a Phase 19
boundary. Both return explicit boundary messages until those phases implement
their effect shells.

## open-bitcoin-cli compatibility path

`open-bitcoin-cli` remains the baseline-compatible RPC client path and continues to use `parse_cli_args`. Its arguments are not parsed by the operator clap tree.

The compatibility path owns Bitcoin/Knots-style flags and shapes including `-named`, `-stdin`, `-stdinrpcpass`, `-getinfo`, `-rpcwallet`, RPC method names, and positional JSON parameters. Wallet-scoped methods route through `/wallet/<name>` when `-rpcwallet` selects a managed wallet. Regression tests must prove routing does not reinterpret those tokens.

## Routing Rule

Shell invocations whose binary name ends in `open-bitcoin-cli` route to raw compatibility args. Shell invocations whose binary name ends in `open-bitcoin` route through the clap operator contract.
