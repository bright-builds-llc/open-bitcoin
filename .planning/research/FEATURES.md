# Feature Research

**Domain:** Bitcoin node operator runtime and real-network sync
**Researched:** 2026-04-26
**Confidence:** HIGH

## Feature Landscape

### Table Stakes

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Rich status command | Operators need to know whether the node is running, what config it is using, and how far it has synced | MEDIUM | Must work when the node is running and when it is stopped. |
| Durable database-backed state | Real sync cannot be credible if chainstate, headers, blocks, wallet state, and sync metadata vanish on restart | HIGH | Requires schema, recovery, and benchmark decisions. |
| Real network sync loop | A node must connect to peers, discover headers, download blocks, validate, and persist progress | HIGH | Build after storage and status contracts exist. |
| Logs and metrics history | Service-managed software needs local evidence for health, failures, and performance | MEDIUM | Keep bounded retention and expose paths in status. |
| First-run onboarding | The first CLI run should produce a usable config without forcing users to learn every option first | MEDIUM | Must be idempotent and non-destructive. |
| Service lifecycle commands | Operators expect daemon restart on reboot and fault recovery | HIGH | macOS launchd first, Linux systemd second. |
| Migration detection | Users may already have Bitcoin Core or Knots datadirs and configs | HIGH | Detection and education come before mutation. |

### Differentiators

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Ratatui dashboard | Gives local operators a rich, keyboard-friendly view without committing to a GUI milestone | MEDIUM | Should show sync, peers, mempool, wallet, metrics, and logs. |
| Drop-in audit view | Makes replacement confidence explicit instead of marketing-driven | HIGH | Must link to Knots/Core sources and Open Bitcoin deviations. |
| Migration dry-run | Helps users understand consequences before moving from Core/Knots | HIGH | Should explain benefits, tradeoffs, backups, and rollback. |
| Real-sync benchmarks | Moves benchmarks from pure smoke to runtime evidence | MEDIUM | Keep deterministic smoke coverage and optional live-network runs separated. |

### Anti-Features

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| One-click destructive migration | Convenient for users with existing nodes | High risk of data loss or unexpected wallet/config changes | Dry-run, backup, explicit confirmation, and auditable migration plan. |
| Full desktop GUI now | A GUI feels more approachable than CLI/TUI | It expands product scope before runtime correctness and sync are ready | Terminal dashboard in v1.1; GUI later. |
| Hosted public dashboard | Good for marketing and project visibility | Does not help local node correctness yet | Local status, metrics history, and generated reports. |

## Feature Dependencies

```text
Durable storage
    -> real network sync
        -> sync status and benchmarks
            -> Ratatui dashboard

Runtime status model
    -> status subcommand
    -> service status
    -> dashboard panels
    -> migration diagnostics

Config and onboarding
    -> service install
    -> migration wizard
    -> daemon startup

Drop-in audit
    -> migration guidance
    -> parity docs
```

## MVP Definition

### Launch With (v1.1)

- [ ] Durable state for headers, chainstate snapshots or UTXO state, block metadata, wallet state, metrics, and logs.
- [ ] Live peer connections and bounded real-network sync smoke coverage for at least non-mainnet networks, with mainnet-safe configuration and documentation.
- [ ] `status` command with human and JSON output.
- [ ] Idempotent onboarding wizard and Open Bitcoin JSONC config layered over `bitcoin.conf`.
- [ ] macOS launchd lifecycle commands and Linux systemd support.
- [ ] Ratatui dashboard fed by real status and metrics contracts.
- [ ] Core/Knots install detection, migration dry-run, and drop-in parity audit ledger.
- [ ] Expanded wallet runtime behavior for useful operator workflows.

### Defer

- [ ] Full graphical desktop GUI.
- [ ] Windows service integration.
- [ ] Hosted/public dashboard.
- [ ] Automatic live datadir mutation without a backup/rollback plan.

## Sources

- `.planning/milestones/v1.0-REQUIREMENTS.md`
- `docs/parity/catalog/rpc-cli-config.md`
- `docs/parity/catalog/p2p.md`
- `docs/parity/catalog/chainstate.md`
- `docs/parity/catalog/wallet.md`
- `docs/parity/release-readiness.md`
- User milestone scope, 2026-04-26.

---
*Feature research for: Open Bitcoin v1.1*
*Researched: 2026-04-26*
