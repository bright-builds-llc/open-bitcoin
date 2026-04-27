# Phase 17: CLI Status and First-Run Onboarding - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-04-26T23:56:00.952Z
**Phase:** 17-CLI Status and First-Run Onboarding
**Mode:** Yolo
**Areas discussed:** Status command, First-run onboarding, Config and precedence, Core/Knots detection, Command boundary

---

## Status Command

| Option | Description | Selected |
|--------|-------------|----------|
| Shared snapshot renderer | Render `OpenBitcoinStatusSnapshot` for human and JSON output. | yes |
| CLI-local status model | Create a renderer-specific DTO in the CLI crate. | |
| RPC-only status | Fail when the daemon is stopped or unreachable. | |

**User's choice:** Shared snapshot renderer.
**Notes:** Selected because Phase 13 and Phase 16 already established shared status, metrics, logs, and unavailable-field contracts.

---

## First-Run Onboarding

| Option | Description | Selected |
|--------|-------------|----------|
| Idempotent practical wizard | Ask network/datadir/config/log/metrics/detection questions and require explicit write approval. | yes |
| Broad migration wizard | Include backup and migration planning now. | |
| Silent defaults | Generate config with minimal explanation. | |

**User's choice:** Idempotent practical wizard.
**Notes:** Migration planning is deferred. This phase should make first setup clear and safe without mutating existing Core/Knots data.

---

## Config and Precedence

| Option | Description | Selected |
|--------|-------------|----------|
| Preserve documented precedence | CLI flags > environment > JSONC > `bitcoin.conf` > cookies > defaults. | yes |
| Merge JSONC into `bitcoin.conf` | Store Open Bitcoin-only settings in the baseline config file. | |
| CLI-only settings | Avoid persistent JSONC settings for now. | |

**User's choice:** Preserve documented precedence.
**Notes:** Open Bitcoin-only settings stay in `open-bitcoin.jsonc`; baseline-compatible keys stay in `bitcoin.conf`.

---

## Core/Knots Detection

| Option | Description | Selected |
|--------|-------------|----------|
| Read-only detection | Report existing datadirs/configs/cookies/services/wallet candidates without mutation. | yes |
| Migration preparation | Generate dry-run migration plans now. | |
| Ignore existing installs | Defer all detection until migration phase. | |

**User's choice:** Read-only detection.
**Notes:** Phase 17 satisfies CLI-07 and MIG-02 detection support, while actual migration planning remains later scope.

---

## Command Boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Implement behind `open-bitcoin` | Use the clap operator path and keep `open-bitcoin-cli` compatibility untouched. | yes |
| Replace compatibility parser | Route all commands through clap. | |
| Add separate one-off binaries | Avoid the existing command contract. | |

**User's choice:** Implement behind `open-bitcoin`.
**Notes:** This preserves the Phase 13 compatibility boundary and keeps Bitcoin/Knots-style RPC invocation stable.

---

## the agent's Discretion

- Exact human status section ordering and labels may be chosen during planning/implementation.
- Exact helper names and module splits may follow existing crate patterns.
- Running-node collection may start with typed extension points if local stopped-node behavior and stable JSON are fully covered.

## Deferred Ideas

- launchd/systemd service lifecycle implementation.
- Ratatui dashboard UI.
- Wallet runtime expansion and multiwallet workflows.
- Backup-aware dry-run migration plans.
