# Phase 13: Operator Runtime Foundations - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-04-26T18:50:22.441Z
**Phase:** 13-operator-runtime-foundations
**Mode:** Yolo
**Areas discussed:** storage foundation, observability contracts, shared status model, CLI command boundary, config ownership and precedence

---

## Storage Foundation

| Option | Description | Selected |
|--------|-------------|----------|
| `fjall` default decision target | Rust-native LSM-shaped key-value storage direction with lower native build cost than RocksDB. | yes |
| `redb` default | Rust-native ACID/MVCC option with a different write profile. | |
| `rocksdb` default | Mature LSM option with native C++ dependency cost. | |

**Selected choice:** Use `fjall` as the milestone default decision target, keep `redb` as a fallback, and require explicit measured evidence before adopting `rocksdb`.
**Notes:** Phase 13 records the decision and adapter-facing contracts only; Phase 14 owns implementation and recovery tests.

---

## Observability Contracts

| Option | Description | Selected |
|--------|-------------|----------|
| Shared bounded contracts | Define metrics/log retention before status and dashboard rendering. | yes |
| Renderer-owned windows | Let status/dashboard pick their own retention views later. | |

**Selected choice:** Define metrics defaults of 30 seconds, 2880 samples, and 24 hours; define log defaults of daily rotation, 14 files, 14 days, and 268435456 bytes.
**Notes:** Retention pruning is intentionally separate from log rolling.

---

## Shared Status Model

| Option | Description | Selected |
|--------|-------------|----------|
| Single snapshot with unavailable fields | One model supports running and stopped nodes with explicit reasons for missing live data. | yes |
| Live RPC wrapper only | Status exists only when the daemon is reachable. | |

**Selected choice:** Create `OpenBitcoinStatusSnapshot` with explicit `Unavailable { reason }` fields for stopped-node/live-data gaps.
**Notes:** Build provenance remains visible as unavailable values rather than disappearing.

---

## CLI Command Boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Two routed surfaces | `open-bitcoin` is clap-owned; `open-bitcoin-cli` remains the compatibility parser path. | yes |
| Rewrite compatibility parser with clap | One clap tree handles both operator commands and baseline RPC syntax. | |

**Selected choice:** Add a clap operator contract and routing enum without changing `open-bitcoin-cli` behavior.
**Notes:** Compatibility tests must cover `-named`, `-stdin`, `-stdinrpcpass`, `-getinfo`, RPC method names, and positional JSON parameters.

---

## Config Ownership and Precedence

| Option | Description | Selected |
|--------|-------------|----------|
| Additive Open Bitcoin JSONC | Open Bitcoin-only settings live in `open-bitcoin.jsonc`; `bitcoin.conf` remains baseline-compatible. | yes |
| Replace `bitcoin.conf` semantics | Move all settings into Open Bitcoin-owned JSONC. | |

**Selected choice:** Use `open-bitcoin.jsonc` for wizard/dashboard/service/migration/metrics/logging/storage/sync state and test precedence as `CLI flags > environment > Open Bitcoin JSONC > bitcoin.conf > cookies > defaults`.
**Notes:** Cookie files remain an auth fallback, not an application-settings layer.

## the agent's Discretion

- Exact helper names and field grouping may vary as long as public contracts, tests, and docs preserve the selected behavior.

## Deferred Ideas

- Implementing storage adapters, live sync, service lifecycle, onboarding, status rendering, and the dashboard is deferred to later v1.1 phases.
