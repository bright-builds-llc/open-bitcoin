# Phase 15: Real Network Sync Loop - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md; this log preserves the alternatives considered.

**Date:** 2026-04-26T21:08:01.619Z
**Phase:** 15-real-network-sync-loop
**Mode:** Yolo
**Areas discussed:** Runtime ownership, peer sources, persistence, flow control, tests

---

## Runtime Ownership

| Option | Description | Selected |
|--------|-------------|----------|
| Node-shell runtime | Put sockets, DNS, durable storage, and clocks in `open-bitcoin-node` around existing pure protocol/domain crates. | yes |
| New protocol stack | Replace the current `PeerManager` flow with a new runtime-specific protocol engine. | |
| Pure-crate live networking | Add socket/DNS concerns to network or core crates. | |

**User's choice:** Auto-selected node-shell runtime.
**Notes:** Preserves the functional-core/imperative-shell boundary and uses existing phase work.

## Peer Sources

| Option | Description | Selected |
|--------|-------------|----------|
| Manual peers plus DNS seeds | Support configured peers and seed hostnames while defaulting tests to fake adapters. | yes |
| Manual peers only | Avoid DNS now and leave seed discovery to later phases. | |
| Always-live DNS discovery | Resolve public seeds by default in normal verification. | |

**User's choice:** Auto-selected manual peers plus DNS seeds.
**Notes:** Live networking remains opt-in for tests.

## Persistence

| Option | Description | Selected |
|--------|-------------|----------|
| Persist as progress happens | Save headers, blocks, chainstate snapshots, runtime metadata, and metrics after accepted sync work. | yes |
| Persist only on clean shutdown | Lower write frequency but poor crash/restart behavior. | |
| Keep sync progress in memory | Simpler but fails Phase 15 resume criteria. | |

**User's choice:** Auto-selected persist as progress happens.
**Notes:** Uses the Phase 14 Fjall store and typed recovery errors.

## Flow Control And Failures

| Option | Description | Selected |
|--------|-------------|----------|
| Bounded in-flight with typed outcomes | Cap block requests and surface stalls/timeouts/disconnects/invalid data as typed runtime outcomes. | yes |
| Unbounded protocol requests | Simpler but unsafe against large `headers` responses. | |
| Panic or disconnect on every failure | Easy to implement but not operator-friendly. | |

**User's choice:** Auto-selected bounded in-flight with typed outcomes.
**Notes:** Phase 16 can later convert these outcomes into richer metrics/log retention.

## Tests

| Option | Description | Selected |
|--------|-------------|----------|
| Hermetic simulated-network tests plus ignored live smoke | Default verification uses fake peers; optional real-network smoke is explicit opt-in. | yes |
| Public network in default tests | Stronger live evidence but flaky and inappropriate for CI. | |
| Unit tests only without live adapter coverage | Too weak for Phase 15 success criteria. | |

**User's choice:** Auto-selected hermetic simulated-network tests plus ignored live smoke.
**Notes:** Default repository verification must remain deterministic.

## the agent's Discretion

- Exact runtime type names, summary fields, and helper structure.
- Whether low-risk network constants for testnet/signet/regtest are included now.

## Deferred Ideas

- Metrics retention and log rotation.
- CLI/TUI presentation.
- Service-managed runtime lifecycle.
- Full benchmark reports.
