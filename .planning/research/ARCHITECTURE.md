# Architecture Research

**Domain:** Service-managed Bitcoin node runtime with terminal operator UX
**Researched:** 2026-04-26
**Confidence:** MEDIUM

## Standard Architecture

```text
CLI / TUI / RPC / service commands
    |
    v
Operator application layer
    - status projection
    - onboarding workflow
    - service lifecycle
    - migration planner
    - dashboard model
    |
    v
Node runtime shell
    - peer sockets
    - sync loop
    - storage adapters
    - log and metrics sinks
    |
    v
Pure domain core
    - consensus
    - chainstate transitions
    - mempool policy
    - wallet decisions
    - wire/protocol parsing
```

## Component Responsibilities

| Component | Responsibility | Typical Implementation |
|-----------|----------------|------------------------|
| CLI command model | Parse commands, options, global config, and output mode | `clap` parser plus compatibility handoff for baseline-shaped RPC invocations. |
| Status projection | Produce a single queryable view of node/service/config/sync/wallet state | Pure structs built from adapters; human and JSON renderers sit outside the model. |
| Metrics recorder | Store bounded historical samples | Thin adapter writing to selected DB or append-only local files. |
| Log pipeline | Structured logs plus rotation and retention | `tracing` with rolling appenders and status-visible paths. |
| Durable storage | Persist headers, block index, chainstate, wallet, and runtime metadata | Storage trait plus selected embedded DB implementation. |
| Sync runtime | Maintain peers and drive headers/block download | Tokio shell around existing peer manager, codec, consensus, and chainstate. |
| Service manager | Generate, install, enable, disable, and inspect launchd/systemd units | Shared command surface with OS-specific adapters. |
| Migration planner | Detect Core/Knots installs and produce safe migration plans | Filesystem/config inspection plus dry-run reports before mutation. |
| TUI dashboard | Present status, metrics, logs, and actions | Ratatui UI consuming status snapshots and command dispatch abstractions. |

## Recommended Project Structure

```text
packages/open-bitcoin-cli/src/
  commands.rs          # clap command tree and compatibility routing
  status.rs            # status command rendering and JSON output
  onboarding.rs        # first-run wizard shell
  service.rs           # CLI-facing service commands
  dashboard.rs         # TUI entrypoint

packages/open-bitcoin-node/src/
  runtime.rs           # long-running node runtime shell
  status.rs            # pure status projection types
  metrics.rs           # metrics sample model
  service.rs           # service lifecycle domain contracts
  migration.rs         # detection and migration planning contracts
  storage.rs           # storage traits and error contracts
  storage/             # concrete storage adapters
  sync.rs              # real-network sync orchestration

packages/open-bitcoin-rpc/src/
  method/status.rs     # RPC status projections when exposed
  method/peer.rs       # getpeerinfo/netinfo support when implemented
```

## Architectural Patterns

### Pattern 1: Status Model Before Renderers

Build a stable status snapshot first, then render it through `status`, JSON, TUI panels, and service diagnostics. This prevents each surface from inventing its own partial truth.

### Pattern 2: Storage Trait Before Storage Choice

Define durable contracts and recovery semantics before committing to a concrete database. The first implementation should be replaceable if benchmarking disproves the choice.

### Pattern 3: Wizard Plans Before File Writes

Onboarding and migration should first produce a plan. Applying the plan is a separate explicit step, which makes dry-run and test coverage natural.

### Pattern 4: UI Pulls Snapshots, Runtime Owns Effects

The Ratatui loop should poll or subscribe to immutable snapshots and dispatch explicit commands. It should not own consensus, sync, service, or migration side effects directly.

## Data Flow

### Runtime Flow

```text
Peer sockets -> codec -> peer manager -> sync planner -> block/header fetch
    -> validation -> chainstate transition -> durable storage
    -> status/metrics snapshots -> CLI/TUI/RPC renderers
```

### Onboarding Flow

```text
CLI first run -> detect datadir/config/installations -> ask questions
    -> produce config/migration/service plan -> dry-run display
    -> explicit apply -> write JSONC config and optional service files
```

### Dashboard Flow

```text
Ratatui event loop -> status snapshot provider -> panels/charts/menu
    -> user action -> typed command -> CLI/runtime adapter -> refreshed snapshot
```

## Anti-Patterns

| Anti-Pattern | Why It Fails | Better Approach |
|--------------|--------------|-----------------|
| Dashboard owns runtime state | UI bugs can corrupt node behavior and testing becomes hard | UI consumes snapshots and sends typed commands. |
| Storage as serialized whole snapshots only | Real sync will write too often and recovery becomes coarse | Persist indexed records with explicit recovery and compaction behavior. |
| Migration writes during discovery | Users lose trust and data can be damaged | Detection, explanation, dry-run, backup, then explicit apply. |
| Service commands hard-code one OS | Linux support will drift or duplicate behavior | Shared service contract with launchd and systemd adapters. |

## Sources

- Existing `open-bitcoin-node` adapter traits for chainstate and wallet storage.
- Existing `open-bitcoin-cli` and `open-bitcoin-rpc` command/RPC surfaces.
- Ratatui, clap, service-manager, tracing-appender, and JSONC research in `.planning/research/STACK.md`.

---
*Architecture research for: Open Bitcoin v1.1*
*Researched: 2026-04-26*
