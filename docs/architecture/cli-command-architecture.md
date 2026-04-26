# CLI Command Architecture

## open-bitcoin operator path

`open-bitcoin` is the clap-owned operator path for Open Bitcoin-specific workflows. The initial command contract includes:

- `status`
- `config`
- `service`
- `dashboard`
- `onboard`

Shared operator options are `--config`, `--datadir`, `--network`, `--format human|json`, and `--no-color`. Phase 13 defines parsing and routing only; command execution, rendering, service mutation, dashboard launch, and onboarding writes are later phase work.

## open-bitcoin-cli compatibility path

`open-bitcoin-cli` remains the baseline-compatible RPC client path and continues to use `parse_cli_args`. Its arguments are not parsed by the operator clap tree.

The compatibility path owns Bitcoin/Knots-style flags and shapes including `-named`, `-stdin`, `-stdinrpcpass`, `-getinfo`, RPC method names, and positional JSON parameters. Regression tests must prove routing does not reinterpret those tokens.

## Routing Rule

Shell invocations whose binary name ends in `open-bitcoin-cli` route to raw compatibility args. Shell invocations whose binary name ends in `open-bitcoin` route through the clap operator contract.
