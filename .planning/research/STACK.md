# Stack Research

**Domain:** Open Bitcoin v2.0 bounded transaction relay and mempool participation
**Researched:** 2026-06-29
**Confidence:** HIGH for stack direction and dependency stance; MEDIUM for exact relay-policy phase cuts until v2.0 requirements are finalized

## Recommended Stack

v2.0 should not add a new third-party stack. The current repo already has the right primitives for bounded relay: first-party transaction, codec, consensus, chainstate, mempool, network, node, RPC, CLI, test-harness, and benchmark crates; Fjall-backed durable runtime state; Tokio/Axum-owned daemon and RPC adapters; and deterministic verification through `bash scripts/verify.sh`.

The required stack change is first-party surface area, not dependency selection. Keep relay and mempool decisions in pure crates, wire them through `open-bitcoin-node`, and let `open-bitcoin-rpc`/`open-bitcoin-cli` expose opt-in operator and RPC controls. Do not pull socket runtimes, persistence, clocks, logging, RPC types, or serde DTOs into `open-bitcoin-mempool` or `open-bitcoin-network`.

Materially reviewed guidance and source context: repo-local `AGENTS.md` content from the prompt, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, `standards/core/architecture.md`, `standards/core/verification.md`, `standards/languages/rust.md`, the requested planning files and crate entrypoints, current manifests, relay/mempool/network/RPC modules, toolchain files, and vendored Knots relay/mempool anchors.

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| Rust | `1.94.1`, edition `2024` | Production language and pure-core implementation | Already pinned by `rust-toolchain.toml` and Bazel. Strong types fit relay policy, inventory state, orphan handling, permission effects, and bounded resource decisions. |
| Cargo workspace | `0.1.0` workspace crates | First-party crate graph | Keep v2.0 inside existing crates instead of adding a relay crate. The current split already matches functional-core / imperative-shell boundaries. |
| Bazel / Bzlmod | Bazelisk target `8.6.0`, `rules_rust 0.69.0` | Top-level smoke build and dependency mirror | Preserve the repo-native build contract. Add relay/mempool crate targets and tests here as surfaces grow. |
| Bitcoin Knots baseline | `29.3.knots20260210` vendored under `packages/bitcoin-knots` | External behavior anchor | Relay behavior must cite `net_processing.cpp`, `txmempool.cpp`, `validation.cpp`, `policy/`, `node/txdownloadman_impl.cpp`, and relay-related functional tests or document intentional deviations. |
| `open-bitcoin-mempool` | Workspace crate | Pure mempool admission and policy | Already owns `Mempool`, `PolicyConfig`, standardness, relay fee, RBF, ancestor/descendant limits, conflict replacement, trimming, and admission errors without non-first-party deps. Extend this crate for policy facts and state transitions only. |
| `open-bitcoin-network` | Workspace crate | Pure P2P message and peer relay state | Already supports `inv`, `getdata`, `notfound`, `tx`, `wtxidrelay`, per-peer requested tx sets, resource caps, and inactive relay/mempool permission labels. Extend this crate for relay eligibility, request scheduling, fanout decisions, orphan-resolution decisions, and `mempool` message support if scoped. |
| `open-bitcoin-node` | Workspace crate | Imperative shell joining mempool, chainstate, storage, and peer state | `ManagedPeerNetwork` already wires received `tx` messages into mempool admission and stores accepted transactions for `getdata` serving. v2.0 should add durable/runtime relay state, accepted/rejected/orphan outcomes, block-connect mempool removal, and peer fanout here. |
| `open-bitcoin-rpc` | Workspace crate with Axum/Tokio | JSON-RPC and daemon runtime adapter | Keep RPC transport here. Extend `RuntimeConfig`, `sendrawtransaction`, `getmempoolinfo`, and Open Bitcoin status methods for opt-in relay activation and evidence. |
| `open-bitcoin-cli` | Workspace crate with clap/Ratatui/Crossterm | Operator status, UAT, support, dashboard | Reuse current operator surfaces for relay status, support bundles, and copy-pasteable UAT commands. |

### First-Party Module Additions

| Surface | Add / Change | Boundary Rationale |
|---------|--------------|--------------------|
| `packages/open-bitcoin-mempool/src/pool.rs` | Add explicit removal APIs for block connection/reorg, acceptance timestamps or sequence evidence if needed, and stable accepted/replaced/evicted/rejected classifications. | Pure data/state transition. It should not know about peers, sockets, Fjall, RPC, or logs. |
| `packages/open-bitcoin-mempool/src/policy.rs` | Extend standardness and fee policy only where v2.0 requirements demand Knots parity, especially orphan/package/RBF edge cases already documented in Knots policy docs. | Pure policy checks stay unit-testable and parity-breadcrumbed. |
| `packages/open-bitcoin-network/src/message.rs` | Add `WireNetworkMessage::Mempool` if v2.0 includes serving bounded mempool inventory requests. Keep `inv/getdata/notfound/tx/wtxidrelay` in the existing codec path. | Wire parsing/encoding belongs in the pure network crate; socket reading remains in adapters. |
| `packages/open-bitcoin-network/src/peer.rs` and `peer/inventory_state.rs` | Promote current tx request tracking into a bounded transaction-relay state machine: per-peer relay opt-in, txid/wtxid preference, known inventory, request caps, notfound cleanup, recent reject awareness, and fanout decisions. | Pure peer lifecycle and relay decisions can be tested without a network. |
| `packages/open-bitcoin-network/src/inbound/permissions.rs` | Activate `relay`, `forcerelay`, and `mempool` permission effects narrowly for v2.0. Keep `bloomfilter` and `blockfilters` inactive unless separately scoped. | Permission semantics are peer-policy domain logic, not RPC/runtime glue. |
| `packages/open-bitcoin-network/src/resource.rs` | Reuse existing request/queue caps and add relay-specific labels for transaction request pressure, orphan pressure, rebroadcast pressure, and mempool inventory pressure. | Resource decisions stay deterministic; runtime only records and enforces them. |
| `packages/open-bitcoin-node/src/network.rs` and `network/inventory.rs` | Add the shell bridge: received tx -> mempool/orphan/reject outcome -> peer attribution -> stored tx serving -> eligible-peer announcements. | This is where pure mempool and pure peer decisions meet mutable runtime state. |
| `packages/open-bitcoin-node/src/storage/fjall_store.rs` | Add a bounded mempool/relay snapshot only if requirements need restart evidence across daemon restarts. Use the existing Fjall store and serde JSON codec pattern. | Persistence is shell-owned. Do not make the pure mempool crate depend on Fjall or serde DTOs. |
| `packages/open-bitcoin-node/src/status.rs`, `metrics.rs`, `logging.rs` | Add low-cardinality relay/mempool metrics and redacted structured logs: accepted, rejected, orphaned, requested, served, notfound, rebroadcast, evicted, and permission-gated counts. | Observability remains shared status data, not dynamic labels or peer-identifying logs. |
| `packages/open-bitcoin-rpc/src/config/open_bitcoin.rs` and loader modules | Add opt-in relay config with disabled/default-bounded posture: e.g. relay enabled flag, mempool max, orphan cap, rebroadcast cap, and permission-gated behavior. | Config parsing is edge validation; it should produce typed config for core/shell consumers. |
| `packages/open-bitcoin-rpc/src/method/node.rs` and `dispatch/node.rs` | Extend `sendrawtransaction` so accepted local submissions can enter bounded relay fanout when enabled. Consider `getrawmempool` only if roadmap needs operator/UAT visibility. | RPC should decode/encode requests and call `ManagedRpcContext`; it should not own policy. |
| `packages/open-bitcoin-cli/src/operator/status*` and support bundle code | Surface relay status and redacted support evidence alongside existing peer/mempool fields. | Operator UX should consume shared status contracts, not duplicate node internals. |
| `packages/open-bitcoin-bench` and `open-bitcoin-test-harness` | Add deterministic relay/mempool fixtures: tx request fanout, orphan bounds, rejection cache, permission gating, rebroadcast, and block-connect mempool removal. | Keeps verification local and public-network-free by default. |

### Supporting Libraries

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| Fjall | `3.1.4` | Durable runtime/state store | Use only in `open-bitcoin-node` shell storage for bounded mempool/relay snapshots, metrics, and recovery evidence. |
| Tokio | `1.52.1` | Async daemon/RPC/inbound runtime | Reuse in `open-bitcoin-rpc` inbound listener and daemon orchestration. Add relay runtime tasks only in shell adapters, not pure crates. |
| Axum | `0.8.9` | Local JSON-RPC HTTP server | Keep for RPC method exposure; relay does not need a new HTTP framework. |
| Serde / serde_json | `1.0.228` / `1.0.149` | Stable status, config, support, and persistence shapes | Use at shell boundaries and snapshot codecs. Avoid serde-driven domain modeling in pure policy modules unless existing public DTOs require it. |
| jsonc-parser | `0.32.3` | Open Bitcoin JSONC config parsing | Extend existing `open-bitcoin.jsonc` schema for relay controls rather than adding a new config format. |
| clap | `4.6.1` | Operator command parsing | Reuse for relay UAT/status/support commands. |
| Ratatui / Crossterm | `0.30` / `0.29` | Terminal dashboard | Add compact relay/mempool status only if roadmap includes dashboard evidence. |
| secp256k1 | `0.31` | Signature cryptography already used by consensus/wallet | Keep existing use. Do not introduce Bitcoin domain libraries around it. |
| Bun | `1.3.9` | Repo-owned TypeScript automation | Use for deterministic checkers and docs/parity guardrails, not runtime relay logic. |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| `bash scripts/verify.sh` | Repo-native verification contract | Add deterministic relay/mempool tests and guardrails here. Keep public-network relay outside default verification. |
| `cargo fmt --all`, `cargo clippy`, `cargo test` | Rust checks run through repo verification | Use crate-level runs for iteration, then the repo verifier before done. |
| Bazel smoke build | Top-level build confidence | Update `BUILD.bazel` targets when new modules/tests are added. |
| Parity breadcrumb checker | Auditable Knots anchors | New first-party Rust source/test files under `packages/open-bitcoin-*` need source breadcrumbs. |
| Bun-backed TypeScript checkers | Release-boundary and docs guardrails | Add v2.0 no-claim checks for compact blocks, production relay defaults, public-network CI, and production full-node wording. |

## Installation

No dependency installation change is recommended for v2.0.

```bash
# Keep existing repo bootstrap/verification flows.
git submodule update --init --recursive
bash scripts/verify.sh
```

If implementation later proves a new dependency is unavoidable, treat that as a phase-specific design decision with maintenance/security review. The current research found no necessary dependency addition.

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| Extend `open-bitcoin-mempool` and `open-bitcoin-network` | Add a new `open-bitcoin-relay` crate | Only if relay logic becomes a clearly separate domain with many cross-cutting modules that would otherwise bloat the network crate. Current code already has relay-adjacent state in `PeerManager`, so a new crate is premature. |
| Use existing `PeerManager` and `ManagedPeerNetwork` | Introduce an actor framework | Only if measured runtime complexity demands it. Current bounded relay decisions fit pure state machines plus thin shell loops. |
| Use Fjall for optional durable mempool/relay evidence | Add SQLite/RocksDB/LevelDB | Only after measured Fjall blockers and a storage migration design. Existing durable state, metrics, headers, blocks, and metadata already use Fjall. |
| Reuse Tokio/Axum shell surfaces | Add a new networking runtime or HTTP server | Not needed. The repo already has Tokio for inbound serving/RPC and standard-library TCP for sync transport. |
| First-party policy and codec work | Add `rust-bitcoin`, `bitcoinconsensus`, BDK, or a P2P crate | Not acceptable for production path under project constraints; parity must be owned and auditable. |
| Deterministic local relay harnesses | Public-network relay CI | Only a later milestone should change the default verification contract. v2.0 should keep live/public relay review opt-in. |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| Existing Rust Bitcoin production libraries | Violates the repo's explicit production-path ownership constraint and weakens auditable Knots parity. | First-party primitives, codec, consensus, chainstate, mempool, and network crates. |
| `libp2p` or generic P2P frameworks | Bitcoin relay semantics are protocol-specific and already modeled in first-party wire/peer code. | Extend `open-bitcoin-network` and existing TCP/Tokio adapters. |
| `dashmap`, actor systems, or async channels in pure crates | They hide deterministic state transitions behind concurrency primitives and erode functional-core testability. | Pure structs/enums returning relay actions; shell adapters execute effects. |
| New database engines | Adds migration, corruption, backup, and operational complexity without a proven need. | Fjall plus first-party snapshot codecs in `open-bitcoin-node`. |
| Prometheus/OpenTelemetry/dynamic metric labels | The project currently uses bounded low-cardinality metric samples and redacted support evidence. Dynamic peer/tx labels risk cardinality and privacy issues. | Extend `MetricKind`, shared status, and redacted structured logs. |
| Bloom/filter relay | v2.0 scope is transaction relay and mempool participation, not BIP37/filter serving. | Keep `bloomfilter` and `blockfilters` permission effects inactive unless a later phase scopes them. |
| Compact block relay or Erlay/minisketch | Explicitly out of scope for v2.0. Pulling minisketch or compact-block logic would blur the release boundary. | Keep compact block and relay optimization docs/checkers deferred. |
| Public relay by default | Conflicts with the bounded, opt-in milestone boundary and production no-claim posture. | Disabled/default-bounded relay config with explicit operator activation and evidence. |
| Logging raw transaction hex, peer endpoints, or dynamic identifiers | Leaks sensitive transaction and peer material into support bundles/logs. | Redacted structured records with stable reason/label/source fields. |

## Stack Patterns by Variant

**If implementing inbound tx acceptance:**
- Use `open-bitcoin-network` to decode `tx`, track requested txid/wtxid, and emit `PeerAction::ReceivedTransaction`.
- Use `open-bitcoin-node` to call `ManagedMempool::submit_transaction`, classify accepted/rejected/orphan outcomes, update caches, and schedule peer announcements.
- Because peer protocol state and mempool admission are separate pure decisions joined by the imperative shell.

**If implementing local `sendrawtransaction` relay:**
- Keep decode and JSON-RPC response shaping in `open-bitcoin-rpc`.
- Keep admission and relay fanout through `ManagedRpcContext` -> `ManagedPeerNetwork`.
- Because RPC should not own mempool policy or peer-selection behavior.

**If implementing orphan handling:**
- Model bounded orphan/recent-reject decisions as pure state, likely in `open-bitcoin-network` alongside tx download management unless requirements show they are mempool-internal.
- Store only bounded runtime/durable evidence in `open-bitcoin-node`.
- Because Knots-style tx download/orphan behavior is peer-relay state, while final admission remains mempool policy.

**If implementing peer permission activation:**
- Convert `inactive_relay`, `inactive_forcerelay`, and `inactive_mempool` into scoped active effects in `open-bitcoin-network/src/inbound/permissions.rs`.
- Keep `bloomfilter` and `blockfilters` inactive.
- Because v1.9 intentionally parsed relay-like labels without granting behavior; v2.0 should activate only the scoped transaction-relay effects.

**If implementing durable mempool evidence:**
- Use `FjallNodeStore` and `snapshot_codec` patterns in `open-bitcoin-node`.
- Persist bounded snapshots/checkpoints, not unbounded peer-specific gossip history.
- Because durable evidence is a shell concern and must remain redaction/recovery aware.

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| Rust `1.94.1` | Bazel `rules_rust 0.69.0` | Both are pinned in `rust-toolchain.toml` and `MODULE.bazel`; keep relay targets aligned with Rust 2024. |
| Workspace crates `0.1.0` | Cargo resolver `3` | Existing crate graph supports first-party relay work without introducing circular dependencies. Keep pure crates dependency-light. |
| `open-bitcoin-mempool` | `open-bitcoin-chainstate`, `open-bitcoin-codec`, `open-bitcoin-consensus`, `open-bitcoin-primitives` | Do not add `open-bitcoin-network`, `open-bitcoin-node`, Fjall, Tokio, Axum, or serde DTO dependencies here. |
| `open-bitcoin-network` | `open-bitcoin-chainstate`, `open-bitcoin-codec`, `open-bitcoin-consensus`, `open-bitcoin-primitives` | Do not add runtime/socket/storage dependencies here. Add pure relay state and wire messages only. |
| `open-bitcoin-node` | `fjall 3.1.4`, `serde 1.0.228`, `serde_json 1.0.149` | Correct place for durable relay/mempool snapshots, metrics, logs, and state projection. |
| `open-bitcoin-rpc` | `axum 0.8.9`, `tokio 1.52.1`, `jsonc-parser 0.32.3` | Correct place for daemon runtime, config loading, JSON-RPC, and inbound listener loops. |
| `open-bitcoin-cli` | `clap 4.6.1`, `ratatui 0.30`, `crossterm 0.29`, `ureq 3.3.0` | Correct place for operator command/status/support consumers; avoid duplicating relay policy. |

## Sources

- HIGH: `.planning/PROJECT.md` and `.planning/MILESTONES.md` - v2.0 scope, deferred surfaces, current milestone boundary.
- HIGH: `.planning/milestones/v1.9-REQUIREMENTS.md` - shipped inbound serving boundary and inactive relay/mempool permission labels.
- HIGH: `packages/Cargo.toml`, per-crate `Cargo.toml`, `rust-toolchain.toml`, `.bazelversion`, `.bun-version`, `MODULE.bazel` - current toolchain and dependency versions.
- HIGH: `packages/open-bitcoin-mempool/src/lib.rs`, `types.rs`, `policy.rs`, `pool.rs`, `error.rs` - current pure mempool policy/admission stack.
- HIGH: `packages/open-bitcoin-network/src/lib.rs`, `message.rs`, `peer.rs`, `peer/inventory_state.rs`, `resource.rs`, `inbound/permissions.rs` - current pure network, inventory, resource, and permission stack.
- HIGH: `packages/open-bitcoin-node/src/network.rs`, `network/inventory.rs`, `mempool.rs`, `storage/fjall_store.rs`, `status.rs`, `metrics.rs`, `logging.rs` - current shell integration, persistence, status, metrics, and logs.
- HIGH: `packages/open-bitcoin-rpc/src/context.rs`, `context/network.rs`, `method/node.rs`, `dispatch/node.rs`, `config/open_bitcoin.rs`, `config/loader/open_bitcoin_runtime.rs`, `inbound_listener.rs`, `bin/open-bitcoind.rs` - current RPC/config/daemon adapter surfaces.
- HIGH: vendored `packages/bitcoin-knots/src/net_processing.cpp`, `src/node/txdownloadman_impl.cpp`, `src/txmempool.cpp`, `src/validation.cpp`, `src/policy/`, `doc/policy/`, and relay/mempool functional tests - pinned parity anchors for future phase research.
- HIGH: `AGENTS.bright-builds.md`, `standards/core/architecture.md`, `standards/core/verification.md`, `standards/languages/rust.md` - functional-core/imperative-shell and verification constraints.

*Stack research for: Open Bitcoin v2.0 Transaction Relay and Mempool Participation Boundary*
*Researched: 2026-06-29*
