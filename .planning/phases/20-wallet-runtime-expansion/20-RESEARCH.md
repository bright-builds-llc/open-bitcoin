# Phase 20: Wallet Runtime Expansion - Research

**Researched:** 2026-04-27 [VERIFIED: .planning/STATE.md]
**Domain:** Wallet runtime parity, wallet-scoped RPC routing, ranged descriptors, resumable rescans, and read-only external wallet inspection. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
**Confidence:** MEDIUM [VERIFIED: packages/open-bitcoin-wallet/src/wallet.rs] [VERIFIED: packages/open-bitcoin-rpc/src/http.rs] [VERIFIED: packages/open-bitcoin-cli/src/args.rs]

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Send workflow and safety contract
- **D-01:** Add a baseline-compatible mutating wallet send path shaped around
  `sendtoaddress`-style semantics instead of promoting the broader Core `send`
  RPC surface in this phase. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
- **D-02:** Back both operator preview and final send execution with one shared
  pure-core send-intent model so fee, change, and error decisions stay
  deterministic and auditable across CLI and RPC adapters. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
- **D-03:** Keep preview and confirmation behavior as an Open Bitcoin-owned
  operator wrapper. Baseline parity applies to the commit path and supported
  parameter semantics, not to the preview surface itself. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
- **D-04:** The send path must support wallet-relevant fee and safety controls
  that matter for practical parity here: explicit fee-rate or estimate inputs,
  replaceability, change handling, fee ceilings, deterministic insufficient-fund
  or invalid-parameter errors, and clear confirmation before a mutating operator
  send. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]

### Wallet selection and runtime identity
- **D-05:** Move from one anonymous managed wallet snapshot to a lightweight
  named-wallet registry with durable per-wallet identity and explicit wallet
  selection metadata. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
- **D-06:** Support the expected wallet-scoped operator surface through
  `-rpcwallet`-style selection and wallet-routed RPC handling for the current
  wallet method subset, while explicitly deferring full `loadwallet`,
  `unloadwallet`, and `listwallets` lifecycle parity. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
- **D-07:** Keep root-vs-wallet boundary behavior explicit in docs and tests so
  node-scoped RPCs remain at the root surface and wallet-scoped RPCs require an
  identified wallet when appropriate. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]

### Descriptor range and address management
- **D-08:** Replace the Phase 7 fixed single-key descriptor limitation with a
  narrow Core-shaped active ranged-descriptor model: one external and one
  internal descriptor role, each with persisted range metadata and `next_index`
  state. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
- **D-09:** Add only the minimum derivation support needed for practical
  receive/change rotation in this phase: xpub/xprv plus path forms required for
  single-key ranged descriptors. Multipath descriptors, miniscript, multisig,
  PSBT, and broader descriptor-wallet parity remain deferred. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
- **D-10:** New-address and change-address allocation must be restart-safe and
  wallet-local. Cursor advancement belongs to durable wallet state rather than
  transient CLI or RPC adapter logic. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]

### Rescan, recovery, backup, and migration inspection
- **D-11:** Wallet rescans become resumable per-wallet runtime jobs with a
  persisted cursor and captured target tip so restart can continue an in-flight
  scan without silently resetting progress. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
- **D-12:** Wallet balance and status output must distinguish fresh, partial,
  and scanning states. Until a wallet catches up to the durable sync tip and
  associated refresh work completes, operator surfaces should expose that the
  wallet view is incomplete instead of implying a final balance. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
- **D-13:** Phase 20 may create Open Bitcoin-owned wallet backup exports, but it
  must not restore, import, copy, or mutate external Core/Knots wallets. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
- **D-14:** Existing Core/Knots wallet candidate inspection becomes
  schema-aware and strictly read-only: enough metadata to support backup and
  later migration planning, without crossing the mutation boundary reserved for
  Phase 21. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]

### Architecture and verification boundary
- **D-15:** Keep pure business logic in `open-bitcoin-wallet` where practical:
  ranged descriptor state, send-intent construction, rescan progress state, and
  wallet-domain validation should stay free of filesystem, network, and prompt
  side effects. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
- **D-16:** `open-bitcoin-node`, `open-bitcoin-rpc`, and `open-bitcoin-cli`
  own persistence, wallet registry storage, RPC routing, operator confirmation,
  read-only external wallet inspection, and long-running rescan orchestration. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
- **D-17:** Phase 20 is not complete unless repo-owned tests cover the new send
  contract, wallet selection routing, ranged descriptor cursor persistence,
  restart-safe rescans, and read-only inspection or backup behavior without
  relying on public network access. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]

### the agent's Discretion
- None provided in `20-CONTEXT.md`. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]

### Deferred Ideas (OUT OF SCOPE)
- full `loadwallet`, `unloadwallet`, and `listwallets` lifecycle parity [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
- the broader Core `send`, `sendall`, PSBT, and bump-fee wallet surfaces [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
- miniscript, multisig, external signers, and descriptor-wallet breadth beyond
  the minimal single-key ranged slice [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
- restore, import, copy, or mutation of Core/Knots wallets [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
- automatic migration plans and execution, which belong to Phase 21 [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| WAL-04 | Wallet supports richer send workflows equivalent to a safe `sendtoaddress`-style operator path, including fee limits, change handling, confirmation prompts, and deterministic error output. [VERIFIED: .planning/REQUIREMENTS.md] | `sendtoaddress` should reuse the existing build/sign helpers through one shared send-intent model, with RPC commit in `open-bitcoin-rpc` and preview/confirmation in the operator shell. [VERIFIED: packages/open-bitcoin-wallet/src/wallet/build.rs] [VERIFIED: packages/open-bitcoin-wallet/src/wallet/sign.rs] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/spend.cpp] |
| WAL-05 | Wallet supports multiwallet or wallet-scoped RPC/CLI selection compatible with the expected `-rpcwallet` style operator surface. [VERIFIED: .planning/REQUIREMENTS.md] | The minimal honest path is wallet-scoped URI routing plus durable named-wallet registry selection, not full Core multiwallet lifecycle parity. [VERIFIED: packages/open-bitcoin-cli/src/args.rs] [VERIFIED: packages/open-bitcoin-cli/src/client.rs] [VERIFIED: packages/open-bitcoin-rpc/src/http.rs] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/util.cpp] |
| WAL-06 | Wallet supports HD or ranged descriptor behavior needed for practical receive/change address management. [VERIFIED: .planning/REQUIREMENTS.md] | One active external and one active internal ranged single-key descriptor with persisted `range` and `next_index` is sufficient for receive/change rotation in this phase. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/backup.cpp] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/addresses.cpp] |
| WAL-07 | Wallet rescan, recovery, and balance tracking integrate with durable sync state and survive node restarts. [VERIFIED: .planning/REQUIREMENTS.md] | Rescan work should become a persisted per-wallet job keyed to the durable sync tip, with status freshness exposed through shared wallet status fields and `getwalletinfo`-style scanning output. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] [VERIFIED: packages/open-bitcoin-node/src/sync.rs] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/transactions.cpp] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/wallet.cpp] |
| WAL-08 | Wallet backup and migration planning can inspect existing Core/Knots wallet candidates without mutating them. [VERIFIED: .planning/REQUIREMENTS.md] | Backup export should remain Open Bitcoin-owned and one-way, while external wallet inspection should deepen the existing read-only detector into format-aware classification without any write path. [VERIFIED: packages/open-bitcoin-cli/src/operator/detect.rs] [VERIFIED: packages/bitcoin-knots/doc/managing-wallets.md] |
</phase_requirements>

## Project Constraints (from AGENTS.md)

- Use `git submodule update --init --recursive` when the Knots baseline under `packages/bitcoin-knots` must be materialized. [VERIFIED: AGENTS.md]
- Use `rust-toolchain.toml` as the Rust source of truth; the pinned toolchain is `1.94.1`. [VERIFIED: AGENTS.md] [VERIFIED: rust-toolchain.toml]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code. [VERIFIED: AGENTS.md] [VERIFIED: scripts/verify.sh]
- Use Bun for repo-owned higher-level automation and keep Bash thin. [VERIFIED: AGENTS.md]
- Record intentional parity differences in `docs/parity/index.json` and companion docs under `docs/parity/`. [VERIFIED: AGENTS.md]
- Add parity breadcrumb comments plus `docs/parity/source-breadcrumbs.json` entries for any new first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`. [VERIFIED: AGENTS.md]
- After substantial wallet/operator changes, check whether contributor-facing README files need updates. [VERIFIED: AGENTS.md]
- Preserve functional-core versus imperative-shell boundaries for wallet business logic and runtime adapters. [VERIFIED: AGENTS.md] [CITED: https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md]
- Treat large functions and files as refactor triggers, prefer early returns, and keep tests focused with explicit Arrange/Act/Assert structure. [VERIFIED: AGENTS.md] [CITED: https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/code-shape.md] [CITED: https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/testing.md]
- The standards sidecar requires reading the pinned Bright Builds standards entrypoint plus relevant architecture, code-shape, verification, testing, and Rust pages before planning. [VERIFIED: AGENTS.bright-builds.md] [CITED: https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/index.md]

## Summary

Phase 20 should stay on the current architecture line instead of creating a second wallet subsystem: keep ranged-descriptor state, address allocation, send-intent shaping, and scan-progress math in `open-bitcoin-wallet`, and let `open-bitcoin-node`, `open-bitcoin-rpc`, and `open-bitcoin-cli` own registry persistence, wallet selection, transport routing, status projection, and operator confirmation. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] [VERIFIED: packages/open-bitcoin-wallet/src/wallet.rs] [VERIFIED: packages/open-bitcoin-node/src/wallet.rs] [VERIFIED: packages/open-bitcoin-rpc/src/context.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/runtime.rs] [CITED: https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md]

The existing code already provides most of the hard primitives needed for that plan: deterministic build/sign helpers exist in `open-bitcoin-wallet`, durable Fjall keyspaces already persist wallet snapshots and runtime metadata, the CLI and RPC tests already mark `sendtoaddress` and `-rpcwallet` as deliberate gaps, and the shared status model already expects explicit unavailable-state reporting that can be extended to wallet freshness. [VERIFIED: packages/open-bitcoin-wallet/src/wallet/build.rs] [VERIFIED: packages/open-bitcoin-wallet/src/wallet/sign.rs] [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] [VERIFIED: packages/open-bitcoin-cli/tests/operator_flows.rs] [VERIFIED: packages/open-bitcoin-node/src/status.rs] [VERIFIED: docs/architecture/status-snapshot.md]

The minimal honest parity slice is narrower than full Core descriptor-wallet behavior: implement one named-wallet registry, wallet-scoped URI selection compatible with `-rpcwallet`, one active external and one active internal single-key ranged descriptor per wallet, `getnewaddress`/`getrawchangeaddress`/`listdescriptors`/`sendtoaddress` on that slice, resumable height-based rescans with persisted cursors, and Open Bitcoin-owned backup exports plus read-only external wallet classification. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/util.cpp] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/addresses.cpp] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/backup.cpp] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/spend.cpp]

**Primary recommendation:** Build Phase 20 around a durable named-wallet registry, wallet-scoped RPC routing, two active ranged single-key descriptors per wallet, and a `sendtoaddress` commit path that wraps the existing build/sign helpers instead of expanding to full multiwallet or full `send` parity. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] [VERIFIED: packages/open-bitcoin-wallet/src/wallet/build.rs] [VERIFIED: packages/open-bitcoin-wallet/src/wallet/sign.rs]

## Standard Stack

> These are repo-pinned versions already present in the workspace, not registry-latest recommendations. [VERIFIED: packages/Cargo.toml] [VERIFIED: packages/open-bitcoin-node/Cargo.toml] [VERIFIED: packages/open-bitcoin-wallet/Cargo.toml] [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml] [VERIFIED: packages/open-bitcoin-cli/Cargo.toml]

### Core

| Library | Version | Purpose | Why Standard | Source |
|---------|---------|---------|--------------|--------|
| `open-bitcoin-wallet` | `0.1.0` | Pure wallet domain logic, transaction build/sign flow, snapshot state, and future ranged-descriptor/send-intent logic. | This is the existing wallet core crate and already owns deterministic build/sign behavior. | [VERIFIED: packages/Cargo.toml] [VERIFIED: packages/open-bitcoin-wallet/src/wallet.rs] |
| `open-bitcoin-node` | `0.1.0` | Durable wallet registry, snapshot persistence, rescan job durability, and shared status projection. | This crate already owns Fjall-backed runtime persistence and adapter boundaries. | [VERIFIED: packages/Cargo.toml] [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] [VERIFIED: packages/open-bitcoin-node/src/status.rs] |
| `open-bitcoin-rpc` | `0.1.0` | Wallet-scoped HTTP/RPC routing and baseline-compatible wallet methods. | This crate already owns JSON-RPC normalization, HTTP transport, and wallet-method dispatch. | [VERIFIED: packages/Cargo.toml] [VERIFIED: packages/open-bitcoin-rpc/src/http.rs] [VERIFIED: packages/open-bitcoin-rpc/src/method.rs] [VERIFIED: packages/open-bitcoin-rpc/src/dispatch.rs] |
| `open-bitcoin-cli` | `0.1.0` | `-rpcwallet` CLI compatibility, endpoint URI selection, and operator-side preview/confirmation shell. | This crate already owns `bitcoin-cli` parity parsing and operator runtime shells. | [VERIFIED: packages/Cargo.toml] [VERIFIED: packages/open-bitcoin-cli/src/args.rs] [VERIFIED: packages/open-bitcoin-cli/src/client.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/runtime.rs] |

### Supporting

| Library | Version | Purpose | When to Use | Source |
|---------|---------|---------|-------------|--------|
| `fjall` | `3.1.4` | Durable keyspace storage for wallet snapshots, registry records, and rescan job metadata. | Use for all node-owned durable wallet state instead of introducing a second database. | [VERIFIED: packages/open-bitcoin-node/Cargo.toml] [VERIFIED: docs/architecture/storage-decision.md] [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] |
| `secp256k1` | `0.31` | Existing key, address, and signing primitive dependency for single-key descriptors. | Reuse for xpub/xprv-derived single-key descriptors and signing; do not introduce a second crypto stack. | [VERIFIED: packages/open-bitcoin-wallet/Cargo.toml] [VERIFIED: packages/open-bitcoin-wallet/src/descriptor.rs] [VERIFIED: packages/open-bitcoin-wallet/src/wallet/sign.rs] |
| `serde` / `serde_json` | `1.0.228` / `1.0.149` | Schema-versioned DTOs for wallet snapshots, registry records, backup exports, and status responses. | Use for durable DTOs and RPC/status payloads already following the repo pattern. | [VERIFIED: packages/open-bitcoin-node/Cargo.toml] [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml] [VERIFIED: packages/open-bitcoin-cli/Cargo.toml] |
| `axum` / `ureq` | `0.8.9` / `3.3.0` | HTTP server/client surface for `/wallet/<name>` routing and CLI endpoint selection. | Reuse the current transport stack for wallet-scoped URI routing instead of adding another transport layer. | [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml] [VERIFIED: packages/open-bitcoin-cli/Cargo.toml] [VERIFIED: packages/open-bitcoin-rpc/src/http.rs] [VERIFIED: packages/open-bitcoin-cli/src/client.rs] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff | Source |
|------------|-----------|----------|--------|
| Fjall-backed wallet registry and job state | A separate SQLite or BDB mirror for wallet runtime metadata | This would duplicate persistence, add migration surface, and conflict with the existing node-shell storage decision. | [VERIFIED: docs/architecture/storage-decision.md] [VERIFIED: packages/open-bitcoin-node/src/storage.rs] |
| Wallet-scoped URI routing plus durable named registry | Full `loadwallet` / `unloadwallet` / `listwallets` parity | Full lifecycle parity is explicitly deferred and would expand scope beyond the current wallet method subset. | [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/wallet.cpp] |
| One active external plus one active internal single-key ranged descriptor | Core’s default four active address-type descriptors and broad keypool parity | The narrower pair is the locked phase scope and is enough for practical receive/change rotation without promising full descriptor-wallet breadth. | [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/addresses.cpp] |

**Installation:** No new dependency installation is the preferred path; reuse the workspace crates and pinned dependencies already present in the repository. [VERIFIED: packages/Cargo.toml] [VERIFIED: packages/open-bitcoin-node/Cargo.toml] [VERIFIED: packages/open-bitcoin-wallet/Cargo.toml] [VERIFIED: packages/open-bitcoin-rpc/Cargo.toml] [VERIFIED: packages/open-bitcoin-cli/Cargo.toml]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-wallet/src/
├── descriptor.rs          # Single-key ranged descriptor parsing and address derivation
├── error.rs               # Wallet-domain typed errors
├── wallet.rs              # Wallet snapshot and public API
├── wallet/build.rs        # Deterministic transaction construction
├── wallet/scan.rs         # Incremental/full rescan state and balance freshness math
├── wallet/sign.rs         # Signing over built transactions
└── wallet/send.rs         # Shared send-intent model and send-specific validation

packages/open-bitcoin-node/src/
├── wallet.rs              # Managed per-wallet shell and mutation persistence
├── wallet_registry.rs     # Durable named-wallet registry and selection metadata
├── storage/fjall_store.rs # Registry, wallet snapshot, and rescan-job persistence
└── status.rs              # Wallet freshness/scanning fields for status consumers

packages/open-bitcoin-rpc/src/
├── http.rs                # Root and /wallet/<name> route handling
├── context.rs             # Shared node/network state plus selected-wallet access
├── method.rs              # Wallet-scoped request/response DTOs
└── dispatch.rs            # Root-vs-wallet method dispatch and send commit path

packages/open-bitcoin-cli/src/
├── args.rs                # -rpcwallet parsing
├── client.rs              # Wallet-scoped endpoint URL selection
└── operator/              # Preview/confirmation wrapper and read-only inspection rendering
```

The structure above keeps new pure business logic in `open-bitcoin-wallet` and confines persistence, transport, and prompt side effects to the existing shell crates. [VERIFIED: packages/open-bitcoin-wallet/src/wallet.rs] [VERIFIED: packages/open-bitcoin-node/src/wallet.rs] [VERIFIED: packages/open-bitcoin-rpc/src/context.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/runtime.rs] [CITED: https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md]

### Pattern 1: Durable Named Wallet Registry

**What:** Replace the current single `ManagedWallet<MemoryWalletStore>` assumption with a node-owned registry that maps wallet names to durable snapshot keys and selection metadata. [VERIFIED: packages/open-bitcoin-rpc/src/context.rs] [VERIFIED: packages/open-bitcoin-node/src/wallet.rs]

**When to use:** Use this for every wallet-scoped RPC or CLI action, and for any status projection that needs to know whether one wallet or many wallets exist. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/util.cpp]

**Why this seam matters:** Knots chooses wallet scope from the request URI and errors when multiple wallets exist but no wallet is specified; Phase 20 needs the same selection discipline without claiming full load/unload parity. [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/util.cpp]

**Example:**

```rust
pub fn import_descriptor(
    &mut self,
    label: impl Into<String>,
    role: DescriptorRole,
    descriptor_text: &str,
) -> Result<u32, WalletError> {
    let descriptor_id = self.wallet.import_descriptor(label, role, descriptor_text)?;
    self.persist();
    Ok(descriptor_id)
}
```

Source: `packages/open-bitcoin-node/src/wallet.rs`. [VERIFIED: packages/open-bitcoin-node/src/wallet.rs]

The same shell-owned persist-after-mutation pattern should be reused for named wallet selection metadata and descriptor cursor advancement. [VERIFIED: packages/open-bitcoin-node/src/wallet.rs]

### Pattern 2: Wallet-Scoped URI Routing, Not In-Band Wallet Mutation

**What:** Route wallet-scoped requests through `/wallet/<walletname>` or an equivalent internal selection marker derived from the URI, while keeping node-scoped RPCs on the root path. [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/util.cpp]

**When to use:** Use this for `getwalletinfo`, `getbalances`, `listunspent`, `importdescriptors`, `rescanblockchain`, `sendtoaddress`, `getnewaddress`, `getrawchangeaddress`, `listdescriptors`, and `backupwallet`. [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/wallet.cpp] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/backup.cpp] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/spend.cpp]

**Current gap:** The Open Bitcoin HTTP router only serves `/`, and the CLI client always posts to `http://host:port/`, so wallet selection cannot work until both transport ends grow path support. [VERIFIED: packages/open-bitcoin-rpc/src/http.rs] [VERIFIED: packages/open-bitcoin-cli/src/client.rs]

**Anti-drift rule:** Keep root-versus-wallet method ownership explicit in tests so `sendrawtransaction`, `getnetworkinfo`, and `getblockchaininfo` remain node-scoped while wallet calls fail clearly when no wallet is selected. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] [VERIFIED: packages/open-bitcoin-rpc/src/dispatch.rs]

### Pattern 3: One Active External Descriptor and One Active Internal Descriptor

**What:** Promote descriptors from fixed single-address records to ranged single-key records with `range_start`, `range_end`, `next_index`, `active`, and `internal` semantics. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/backup.cpp] [VERIFIED: packages/bitcoin-knots/src/wallet/walletutil.h]

**When to use:** Use this for practical receive and change address rotation, for `getnewaddress` / `getrawchangeaddress`, and for `listdescriptors` visibility. [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/addresses.cpp] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/backup.cpp]

**Current gap:** `SingleKeyDescriptor::parse` currently rejects `*` and multipath syntax outright, and the wallet snapshot codec stores only fixed descriptors with no range or cursor fields. [VERIFIED: packages/open-bitcoin-wallet/src/descriptor.rs] [VERIFIED: packages/open-bitcoin-node/src/storage/snapshot_codec/wallet.rs]

**Minimal parity rule:** Support only the xpub/xprv plus derivation forms needed for single-key ranged descriptors and persist cursor advancement inside wallet state before returning an allocated address. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/backup.cpp]

### Pattern 4: Rescan as a Durable Per-Wallet Job

**What:** Model rescans as explicit per-wallet jobs with a target tip, a next block height or hash cursor, and persisted partial wallet state. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]

**When to use:** Use this whenever a descriptor import or explicit `rescanblockchain` call asks the wallet to catch up to durable node state. [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/transactions.cpp] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/backup.cpp]

**Current gap:** Open Bitcoin currently rescans only by rebuilding from the current `ChainstateSnapshot`, and it rejects all partial height ranges except the full-tip slice. [VERIFIED: packages/open-bitcoin-wallet/src/wallet/scan.rs] [VERIFIED: packages/open-bitcoin-rpc/src/dispatch.rs]

**Recommended seam:** Reuse the durable sync runtime pattern in `open-bitcoin-node/src/sync.rs`: read persisted header/block order from Fjall, apply blocks incrementally to pure wallet scan state, and checkpoint the job in the store after bounded work chunks. [VERIFIED: packages/open-bitcoin-node/src/sync.rs] [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs]

### Pattern 5: Freshness Is Status Data, Not an Implicit Guess

**What:** Extend `WalletStatus` and `getwalletinfo` so operator surfaces can distinguish `fresh`, `partial`, and `scanning` wallet views instead of showing a raw trusted balance alone. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] [VERIFIED: packages/open-bitcoin-node/src/status.rs]

**When to use:** Use this in status, dashboard, wallet RPCs, and backup/export messaging whenever the wallet is behind the durable sync tip or a rescan is in progress. [VERIFIED: docs/architecture/status-snapshot.md] [VERIFIED: packages/open-bitcoin-cli/src/operator/dashboard/model.rs]

**Current gap:** The shared status model exposes only `trusted_balance_sats`, and the dashboard projects that one field directly. [VERIFIED: packages/open-bitcoin-node/src/status.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/dashboard/model.rs]

### Likely File Touch Points

- `packages/open-bitcoin-wallet/src/descriptor.rs`, `wallet.rs`, `wallet/scan.rs`, and one new wallet submodule for send-intent or address allocation. [VERIFIED: packages/open-bitcoin-wallet/src/descriptor.rs] [VERIFIED: packages/open-bitcoin-wallet/src/wallet.rs]
- `packages/open-bitcoin-wallet/src/error.rs` for new typed errors such as invalid range, cursor exhaustion, unsupported address-type mismatch, and fee-policy validation. [VERIFIED: packages/open-bitcoin-wallet/src/error.rs]
- `packages/open-bitcoin-node/src/wallet.rs` plus one new registry-oriented module and Fjall snapshot DTO updates. [VERIFIED: packages/open-bitcoin-node/src/wallet.rs] [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] [VERIFIED: packages/open-bitcoin-node/src/storage/snapshot_codec/wallet.rs]
- `packages/open-bitcoin-node/src/status.rs` and downstream CLI dashboard/status renderers for wallet freshness and scanning fields. [VERIFIED: packages/open-bitcoin-node/src/status.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/dashboard/model.rs]
- `packages/open-bitcoin-rpc/src/http.rs`, `context.rs`, `method.rs`, and `dispatch.rs` for wallet-scoped routing and new RPC shapes. [VERIFIED: packages/open-bitcoin-rpc/src/http.rs] [VERIFIED: packages/open-bitcoin-rpc/src/context.rs] [VERIFIED: packages/open-bitcoin-rpc/src/method.rs] [VERIFIED: packages/open-bitcoin-rpc/src/dispatch.rs]
- `packages/open-bitcoin-cli/src/args.rs`, `client.rs`, and operator modules for `-rpcwallet` and preview/confirmation flows. [VERIFIED: packages/open-bitcoin-cli/src/args.rs] [VERIFIED: packages/open-bitcoin-cli/src/client.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/runtime.rs]
- `packages/open-bitcoin-cli/src/operator/detect.rs` and related status/onboarding rendering for schema-aware read-only external wallet inspection. [VERIFIED: packages/open-bitcoin-cli/src/operator/detect.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs]

### Anti-Patterns to Avoid

- **Do not keep wallet selection as a CLI-only string:** wallet scope must survive transport and server dispatch, not disappear before the request reaches `open-bitcoin-rpc`. [VERIFIED: packages/open-bitcoin-cli/src/client.rs] [VERIFIED: packages/open-bitcoin-rpc/src/http.rs]
- **Do not advance `next_index` in CLI or RPC adapters:** cursor advancement belongs in durable wallet state so restart cannot reuse or skip addresses. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] [VERIFIED: packages/bitcoin-knots/src/wallet/scriptpubkeyman.cpp]
- **Do not infer “fresh” from a balance number alone:** status must surface scan state explicitly. [VERIFIED: docs/architecture/status-snapshot.md] [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
- **Do not widen scope into full Core wallet lifecycle or `send` parity:** those surfaces are explicitly deferred. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
- **Do not mutate Core/Knots wallet candidates during inspection:** the phase boundary is read-only. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] [VERIFIED: packages/open-bitcoin-cli/src/operator/detect.rs]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why | Source |
|---------|-------------|-------------|-----|--------|
| Transaction funding and signing | A second send-specific tx builder or signer | `Wallet::build_transaction`, `Wallet::build_and_sign`, and their existing fee/change logic | The deterministic build/sign helpers already encode spendability, dust folding, change selection, and signature behavior. | [VERIFIED: packages/open-bitcoin-wallet/src/wallet/build.rs] [VERIFIED: packages/open-bitcoin-wallet/src/wallet/sign.rs] |
| Wallet status transport DTOs | A dashboard-only or CLI-only wallet status struct | `OpenBitcoinStatusSnapshot` and an expanded `WalletStatus` | The status contract is already the shared source of truth for status and dashboard consumers. | [VERIFIED: docs/architecture/status-snapshot.md] [VERIFIED: packages/open-bitcoin-node/src/status.rs] |
| Durable wallet storage | Ad hoc JSON files or a second database for wallet runtime state | The existing Fjall keyspaces and schema-versioned DTO pattern | The repository already decided to keep wallet runtime durability inside the Fjall-backed node shell. | [VERIFIED: docs/architecture/storage-decision.md] [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] |
| External wallet migration | Direct import, copy, or rewrite of Core/Knots wallet data | Read-only inspection plus Open Bitcoin-owned backup export | Mutation of external wallets is explicitly out of scope until Phase 21. | [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] |
| Multiwallet lifecycle | `loadwallet`, `unloadwallet`, `listwallets`, or dynamic wallet manager parity | A durable registry of always-managed Open Bitcoin wallet snapshots | WAL-05 only requires selection compatibility, not lifecycle parity. | [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] [VERIFIED: .planning/REQUIREMENTS.md] |

**Key insight:** The honest way to move fast here is to widen the existing wallet shell one step at a time, not to imitate every Core wallet subsystem in one phase. [VERIFIED: docs/parity/catalog/rpc-cli-config.md] [VERIFIED: docs/parity/catalog/wallet.md]

## Common Pitfalls

### Pitfall 1: Treating `-rpcwallet` as Just Another Method Parameter

**What goes wrong:** Root-scoped methods and wallet-scoped methods become indistinguishable, or wallet selection is lost before the RPC server sees it. [VERIFIED: packages/open-bitcoin-cli/src/client.rs] [VERIFIED: packages/open-bitcoin-rpc/src/http.rs]

**Why it happens:** The current CLI compatibility client always posts to `/`, and the current HTTP router only serves `/`, so there is no path-level scope today. [VERIFIED: packages/open-bitcoin-cli/src/client.rs] [VERIFIED: packages/open-bitcoin-rpc/src/http.rs]

**How to avoid:** Add wallet name support to startup args and HTTP endpoint construction, then keep a root-vs-wallet allowlist in dispatch tests. [VERIFIED: packages/open-bitcoin-cli/src/args.rs] [VERIFIED: packages/open-bitcoin-rpc/src/dispatch/tests.rs]

**Warning signs:** `getwalletinfo` or `sendtoaddress` still work on the root path when multiple managed wallets exist. [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/util.cpp]

### Pitfall 2: Persisting Only the Final Wallet Snapshot

**What goes wrong:** Address cursors and rescans become non-restart-safe, so the same receive/change address can be reused after a crash or a rescan silently restarts from zero. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]

**Why it happens:** The current wallet snapshot codec stores descriptors and UTXOs, but no range or cursor metadata. [VERIFIED: packages/open-bitcoin-node/src/storage/snapshot_codec/wallet.rs]

**How to avoid:** Make `range`, `next_index`, and rescan job cursor fields part of the persisted schema and persist them immediately after mutation or bounded scan progress. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] [VERIFIED: packages/open-bitcoin-node/src/wallet.rs]

**Warning signs:** `listdescriptors` cannot report `next_index`, or `getnewaddress` returns the same value after restart. [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/backup.cpp]

### Pitfall 3: Showing a Trusted Balance Without Freshness

**What goes wrong:** Operators see a concrete balance value and assume it is final even when a rescan is still running or the wallet is behind the durable sync tip. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]

**Why it happens:** The current status model exposes only `trusted_balance_sats`, and the current dashboard mirrors that one field. [VERIFIED: packages/open-bitcoin-node/src/status.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/dashboard/model.rs]

**How to avoid:** Add explicit freshness and scanning fields to wallet status and `getwalletinfo`, and keep `getbalances` backward-shaped while clearly documenting freshness elsewhere. [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/wallet.cpp] [VERIFIED: docs/architecture/status-snapshot.md]

**Warning signs:** Status output cannot distinguish “node stopped,” “wallet stale,” and “wallet actively rescanning.” [VERIFIED: docs/architecture/status-snapshot.md]

### Pitfall 4: Over-Claiming Descriptor or Fee Parity

**What goes wrong:** The phase accidentally implies full descriptor-wallet or fee-estimator parity even though only a narrow ranged single-key slice is implemented. [VERIFIED: docs/parity/catalog/wallet.md] [VERIFIED: docs/parity/catalog/rpc-cli-config.md]

**Why it happens:** Knots covers four default active descriptor types, deep keypool behavior, and automatic fee estimation, while the current Open Bitcoin codebase has none of that beyond deterministic build/sign helpers. [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/addresses.cpp] [VERIFIED: packages/open-bitcoin-wallet/src/wallet/build.rs] [VERIFIED: packages/open-bitcoin-rpc/src/dispatch.rs]

**How to avoid:** Document the supported parameter subset, reject unsupported estimate-mode flows deterministically, and keep the parity ledger explicit. [VERIFIED: docs/parity/index.json] [VERIFIED: docs/parity/catalog/rpc-cli-config.md]

**Warning signs:** A planner task says “implement send parity” without naming the supported parameters or the unsupported ones. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]

### Pitfall 5: Doing Deep External Wallet Parsing Too Early

**What goes wrong:** The phase acquires a dependency-heavy or write-prone migration subsystem before the planner even reaches Phase 21. [VERIFIED: .planning/ROADMAP.md] [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]

**Why it happens:** Existing detection is path-based today, and it is tempting to jump straight from path discovery to mutation or format conversion. [VERIFIED: packages/open-bitcoin-cli/src/operator/detect.rs]

**How to avoid:** Limit Phase 20 inspection to read-only classification, source-path reporting, and backup planning metadata, and defer destructive migration to Phase 21. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]

**Warning signs:** New code opens external wallet paths for writing, copies `wallet.dat`, or promises restore/import compatibility. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]

## Code Examples

Verified patterns from existing first-party code and the pinned Knots baseline:

### Persist Wallet Mutations in the Shell

```rust
fn persist(&mut self) {
    self.store.save_snapshot(self.wallet.snapshot());
}
```

Source: `packages/open-bitcoin-node/src/wallet.rs`. [VERIFIED: packages/open-bitcoin-node/src/wallet.rs]

This is the pattern to preserve for descriptor cursor updates and rescan progress checkpoints: pure wallet state mutates first, then the shell persists the updated snapshot. [VERIFIED: packages/open-bitcoin-node/src/wallet.rs] [CITED: https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md]

### Reuse the Existing Build-and-Sign Pipeline

```rust
let built = wallet.build_transaction(request, coinbase_maturity)?;
let signed = sign_transaction(wallet, &built)?;
```

Source: `packages/open-bitcoin-wallet/src/wallet/sign.rs`. [VERIFIED: packages/open-bitcoin-wallet/src/wallet/sign.rs]

`sendtoaddress` should wrap this existing pipeline and then commit through the node or RPC shell, instead of inventing a second funding path. [VERIFIED: packages/open-bitcoin-wallet/src/wallet/sign.rs] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/spend.cpp]

### Knots Wallet Scope Is Chosen by URI

```cpp
if (request.URI.starts_with(WALLET_ENDPOINT_BASE)) {
    wallet_name = UrlDecode(std::string_view{request.URI}.substr(WALLET_ENDPOINT_BASE.size()));
    return true;
}
```

Source: `packages/bitcoin-knots/src/wallet/rpc/util.cpp`. [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/util.cpp]

Open Bitcoin should mirror this selection model for its current wallet-method subset so `-rpcwallet` can remain transport metadata instead of becoming method-specific JSON. [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/util.cpp] [VERIFIED: packages/open-bitcoin-cli/src/client.rs] [VERIFIED: packages/open-bitcoin-rpc/src/http.rs]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact | Source |
|--------------|------------------|--------------|--------|--------|
| One anonymous managed wallet snapshot | Durable named-wallet registry plus explicit wallet selection metadata | Phase 20 recommendation | Enables `-rpcwallet` compatibility without full `loadwallet` parity. | [VERIFIED: packages/open-bitcoin-rpc/src/context.rs] [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] |
| Fixed single-key descriptors with no `range` or `next_index` | Active ranged single-key external/internal descriptors with persisted cursor state | Phase 20 requirement | Enables practical receive/change rotation and `listdescriptors`-style visibility. | [VERIFIED: packages/open-bitcoin-wallet/src/descriptor.rs] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/backup.cpp] |
| Full-snapshot chainstate rebuild for rescans | Resumable per-wallet height-based rescan jobs against durable node state | Phase 20 recommendation | Enables restart-safe scan progress and wallet freshness states. | [VERIFIED: packages/open-bitcoin-wallet/src/wallet/scan.rs] [VERIFIED: packages/open-bitcoin-node/src/sync.rs] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/transactions.cpp] |
| `getwalletinfo` subset with counts and tip metadata only | `getwalletinfo` plus wallet name, scanning progress, and freshness metadata | Phase 20 recommendation | Gives operators a clear view of whether balances are complete. | [VERIFIED: packages/open-bitcoin-rpc/src/method.rs] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/wallet.cpp] |
| Path-only external wallet detection | Format-aware, read-only classification for backup and later migration planning | Phase 20 recommendation | Makes inspection more useful without crossing into mutation. | [VERIFIED: packages/open-bitcoin-cli/src/operator/detect.rs] [VERIFIED: packages/bitcoin-knots/doc/managing-wallets.md] |

**Deprecated/outdated:**

- The Phase 8 assumption that `rescanblockchain` should reject bounded height ranges is now outdated for the planned runtime wallet slice because Phase 20 needs resumable range-aware rescans tied to durable sync state. [VERIFIED: packages/open-bitcoin-rpc/src/dispatch.rs] [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
- The Phase 8 deferral of `-rpcwallet` is now outdated for WAL-05 and should become an implemented wallet-selection surface rather than a parser error. [VERIFIED: packages/open-bitcoin-cli/src/args.rs] [VERIFIED: .planning/REQUIREMENTS.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Open Bitcoin backup export can be a repo-owned snapshot format rather than a Core-compatible `wallet.dat` copy. [ASSUMED] | Summary; Architecture Patterns; Open Questions | If wrong, Phase 20 scope expands into restore/import compatibility and external wallet-file semantics. |
| A2 | The minimal honest `sendtoaddress` slice can require explicit `fee_rate` for commit flows and return a deterministic unsupported-error for automatic fee-estimation inputs until a real estimator exists. [ASSUMED] | Summary; Common Pitfalls; Planner Recommendation | If wrong, WAL-04 may require a fee-estimation subsystem that is not present in the current stack. |
| A3 | “Schema-aware” external wallet inspection is satisfied by reliable format classification and metadata extraction without full SQLite/BDB record parsing. [ASSUMED] | Summary; State of the Art; Open Questions | If wrong, Phase 20 may need new database-format dependencies or far deeper parsing work. |

## Open Questions

1. **How much of `sendtoaddress` should be implemented beyond explicit fee-rate flows?**
   - What we know: Knots supports `conf_target`, `estimate_mode`, `fee_rate`, `replaceable`, and `subtractfeefromamount`, but the current Open Bitcoin stack has deterministic build/sign helpers and no fee estimator. [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/spend.cpp] [VERIFIED: packages/open-bitcoin-wallet/src/wallet/build.rs] [VERIFIED: packages/open-bitcoin-rpc/src/dispatch.rs]
   - What's unclear: Whether WAL-04 is satisfied by explicit fee-rate support plus deterministic unsupported-estimate errors, or whether the planner must add a first estimator. [ASSUMED]
   - Recommendation: Keep the initial plan centered on explicit `fee_rate` and flag estimate-mode parity as a user confirmation point unless a repo-local estimator already exists. [ASSUMED]

2. **Where should the preview/confirmation wrapper live?**
   - What we know: The compatibility CLI path is baseline-oriented, while the operator clap path currently has `status`, `config`, `service`, `dashboard`, and `onboard`, but no wallet send subcommand. [VERIFIED: docs/architecture/cli-command-architecture.md] [VERIFIED: packages/open-bitcoin-cli/src/operator.rs]
   - What's unclear: Whether Phase 20 should add a new operator subcommand now or keep preview/confirm inside a later dashboard or separate operator action surface. [ASSUMED]
   - Recommendation: Keep the commit path in `open-bitcoin-cli sendtoaddress` and plan the preview/confirm wrapper as a thin operator feature only if the phase has room after the baseline RPC path is stable. [ASSUMED]

3. **How deep should external wallet inspection go in Phase 20?**
   - What we know: Current detection is path-based and read-only, and Phase 20 explicitly defers external wallet mutation. [VERIFIED: packages/open-bitcoin-cli/src/operator/detect.rs] [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
   - What's unclear: Whether the planner should stop at reliable format classification or also surface descriptor counts or more detailed metadata from external stores. [ASSUMED]
   - Recommendation: Keep Phase 20 at reliable classification plus path/backup metadata, and defer deep content parsing to Phase 21 unless a later user decision expands scope. [ASSUMED]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `cargo` | Rust build, test, and targeted inner-loop verification | ✓ | `1.94.1` | — |
| `cargo-llvm-cov` | Repo-native `scripts/verify.sh` phase gate | ✓ | `0.8.5` | No full fallback for the repo-native verification contract. |
| `bun` | Repo-owned verification scripts such as LOC and parity breadcrumb checks | ✓ | `1.3.9` | No equal fallback because `scripts/verify.sh` requires Bun. |
| `bazel` | Repo-native smoke build in `scripts/verify.sh` | ✓ | `8.6.0` | No equal fallback for the repo-native verification contract. |
| `bash` | Repo-native verification wrappers and helper scripts | ✓ | `3.2.57` | — |

Source for all rows: [VERIFIED: scripts/verify.sh] [VERIFIED: rust-toolchain.toml] [VERIFIED: local command output 2026-04-27]

**Missing dependencies with no fallback:**

- None. [VERIFIED: local command output 2026-04-27]

**Missing dependencies with fallback:**

- None. [VERIFIED: local command output 2026-04-27]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control | Source |
|---------------|---------|------------------|--------|
| V2 Authentication | yes | Reuse existing RPC Basic-auth and cookie-auth configuration; Phase 20 should not invent a new auth model. | [VERIFIED: packages/open-bitcoin-rpc/src/http.rs] [VERIFIED: packages/open-bitcoin-rpc/src/config.rs] |
| V3 Session Management | no | RPC is stateless HTTP Basic-auth or cookie-auth in the current stack. | [VERIFIED: packages/open-bitcoin-rpc/src/http.rs] |
| V4 Access Control | yes | Root-vs-wallet method allowlists plus wallet selection via `/wallet/<name>` or equivalent internal scope. | [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/util.cpp] [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] |
| V5 Input Validation | yes | Keep request normalization in `method.rs` and typed wallet errors in `open-bitcoin-wallet`; reject unsupported descriptor/range/fee combinations deterministically. | [VERIFIED: packages/open-bitcoin-rpc/src/method.rs] [VERIFIED: packages/open-bitcoin-wallet/src/error.rs] |
| V6 Cryptography | yes | Reuse existing `secp256k1` plus the consensus sighash helpers; do not add a second signing or key-derivation stack casually. | [VERIFIED: packages/open-bitcoin-wallet/Cargo.toml] [VERIFIED: packages/open-bitcoin-wallet/src/wallet/sign.rs] |

### Known Threat Patterns for This Stack

| Pattern | STRIDE | Standard Mitigation | Source |
|---------|--------|---------------------|--------|
| Wrong-wallet mutation through missing or ignored wallet scope | Tampering | Enforce explicit wallet selection when multiple named wallets exist, and keep wallet-scoped methods off the root surface. | [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/util.cpp] [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] |
| Descriptor or send parameter confusion | Tampering | Keep typed normalization in `method.rs` and reject unsupported or conflicting parameters before wallet mutation. | [VERIFIED: packages/open-bitcoin-rpc/src/method.rs] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/spend.cpp] |
| Address reuse after restart | Information Disclosure | Persist `next_index` in the wallet snapshot immediately after allocation. | [VERIFIED: packages/bitcoin-knots/src/wallet/scriptpubkeyman.cpp] [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] |
| Unsafe backup export overwriting external wallet data | Tampering | Refuse to write Open Bitcoin backup exports into known Core/Knots wallet candidate paths and keep export format explicitly Open Bitcoin-owned. | [VERIFIED: packages/open-bitcoin-cli/src/operator/detect.rs] [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] |
| Read-only inspection crossing into mutation | Elevation of Privilege | Keep external wallet inspection in the operator shell with read-only file access only. | [VERIFIED: packages/open-bitcoin-cli/src/operator/detect.rs] [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] |

## Sources

### Primary (HIGH confidence)

- `packages/open-bitcoin-wallet/src/descriptor.rs` - current descriptor parser rejects ranged descriptors and multipath syntax. [VERIFIED: packages/open-bitcoin-wallet/src/descriptor.rs]
- `packages/open-bitcoin-wallet/src/wallet.rs`, `wallet/build.rs`, `wallet/scan.rs`, `wallet/sign.rs` - current wallet state, build/sign helpers, and full-snapshot rescan behavior. [VERIFIED: packages/open-bitcoin-wallet/src/wallet.rs] [VERIFIED: packages/open-bitcoin-wallet/src/wallet/build.rs] [VERIFIED: packages/open-bitcoin-wallet/src/wallet/scan.rs] [VERIFIED: packages/open-bitcoin-wallet/src/wallet/sign.rs]
- `packages/open-bitcoin-node/src/wallet.rs`, `storage/fjall_store.rs`, `storage/snapshot_codec/wallet.rs`, `status.rs`, `sync.rs` - current wallet persistence, durable storage contracts, status model, and restart-safe runtime patterns. [VERIFIED: packages/open-bitcoin-node/src/wallet.rs] [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] [VERIFIED: packages/open-bitcoin-node/src/storage/snapshot_codec/wallet.rs] [VERIFIED: packages/open-bitcoin-node/src/status.rs] [VERIFIED: packages/open-bitcoin-node/src/sync.rs]
- `packages/open-bitcoin-rpc/src/http.rs`, `context.rs`, `method.rs`, `dispatch.rs` - current transport, single-wallet context, request normalization, and `rescanblockchain` limitations. [VERIFIED: packages/open-bitcoin-rpc/src/http.rs] [VERIFIED: packages/open-bitcoin-rpc/src/context.rs] [VERIFIED: packages/open-bitcoin-rpc/src/method.rs] [VERIFIED: packages/open-bitcoin-rpc/src/dispatch.rs]
- `packages/open-bitcoin-cli/src/args.rs`, `client.rs`, `operator/runtime.rs`, `operator/detect.rs`, `operator/status.rs` - current `-rpcwallet` deferral, root-only endpoint URL, read-only detector, and wallet status rendering. [VERIFIED: packages/open-bitcoin-cli/src/args.rs] [VERIFIED: packages/open-bitcoin-cli/src/client.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/runtime.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/detect.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs]
- `packages/bitcoin-knots/src/wallet/rpc/spend.cpp`, `transactions.cpp`, `wallet.cpp`, `backup.cpp`, `addresses.cpp`, and `rpc/util.cpp` - pinned baseline behavior for `sendtoaddress`, rescans, wallet status, descriptor metadata, address allocation, and wallet-scoped URI selection. [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/spend.cpp] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/transactions.cpp] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/wallet.cpp] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/backup.cpp] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/addresses.cpp] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/util.cpp]
- `packages/bitcoin-knots/test/functional/wallet_descriptor.py`, `wallet_gethdkeys.py`, `wallet_backup.py` - pinned behavioral examples for ranged descriptors, `next_index`, key exhaustion, and backup boundaries. [VERIFIED: packages/bitcoin-knots/test/functional/wallet_descriptor.py] [VERIFIED: packages/bitcoin-knots/test/functional/wallet_gethdkeys.py] [VERIFIED: packages/bitcoin-knots/test/functional/wallet_backup.py]
- `docs/architecture/cli-command-architecture.md`, `config-precedence.md`, `status-snapshot.md`, `storage-decision.md`, and parity docs under `docs/parity/` - repo architecture and parity constraints that Phase 20 must preserve. [VERIFIED: docs/architecture/cli-command-architecture.md] [VERIFIED: docs/architecture/config-precedence.md] [VERIFIED: docs/architecture/status-snapshot.md] [VERIFIED: docs/architecture/storage-decision.md] [VERIFIED: docs/parity/catalog/wallet.md] [VERIFIED: docs/parity/catalog/rpc-cli-config.md] [VERIFIED: docs/parity/index.json]

### Secondary (MEDIUM confidence)

- Bright Builds standards index plus the pinned core architecture, code-shape, testing, verification, and Rust pages informed structure, verification, and test-shape recommendations. [CITED: https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/index.md] [CITED: https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md] [CITED: https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/code-shape.md] [CITED: https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/testing.md] [CITED: https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/verification.md] [CITED: https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/languages/rust.md]

### Tertiary (LOW confidence)

- None. [VERIFIED: this research session]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - the phase can stay on the already-pinned workspace crates and external deps. [VERIFIED: packages/Cargo.toml] [VERIFIED: packages/open-bitcoin-node/Cargo.toml] [VERIFIED: packages/open-bitcoin-wallet/Cargo.toml]
- Architecture: MEDIUM - the core seams are strongly supported by current code, but backup export shape and fee-estimation boundaries still need explicit planner decisions. [VERIFIED: packages/open-bitcoin-node/src/wallet.rs] [VERIFIED: packages/open-bitcoin-rpc/src/http.rs] [ASSUMED]
- Pitfalls: HIGH - the main failure modes are directly visible in current deferred tests, status gaps, and Knots baseline behavior. [VERIFIED: packages/open-bitcoin-cli/tests/operator_flows.rs] [VERIFIED: packages/open-bitcoin-node/src/status.rs] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/util.cpp]

**Research date:** 2026-04-27 [VERIFIED: .planning/STATE.md]
**Valid until:** 2026-05-27 for planning purposes unless Phase 20 scope or the pinned Knots baseline changes. [ASSUMED]

## Planner Recommendation

Plan this phase as five sequential slices, with parity documentation and tests in every slice. [VERIFIED: .planning/REQUIREMENTS.md] [VERIFIED: AGENTS.md]

1. **Wallet core domain expansion.** Add ranged single-key descriptor state, persisted `range` and `next_index`, minimal xpub/xprv path support, address allocation APIs, and a shared send-intent model in `open-bitcoin-wallet`. [VERIFIED: packages/open-bitcoin-wallet/src/descriptor.rs] [VERIFIED: packages/open-bitcoin-wallet/src/wallet.rs] [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
2. **Durable node wallet state.** Introduce a named-wallet registry, snapshot schema upgrades, and per-wallet rescan job persistence in `open-bitcoin-node`, reusing Fjall keyspaces and schema-versioned DTOs. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] [VERIFIED: packages/open-bitcoin-node/src/storage/snapshot_codec/wallet.rs] [VERIFIED: packages/open-bitcoin-node/src/sync.rs]
3. **Wallet-scoped RPC and CLI routing.** Un-defer `-rpcwallet`, add `/wallet/<name>` handling, expand `ManagedRpcContext` for selected-wallet access, and implement the minimal new wallet methods: `sendtoaddress`, `getnewaddress`, `getrawchangeaddress`, `listdescriptors`, and expanded `getwalletinfo`, plus real ranged `rescanblockchain`. [VERIFIED: packages/open-bitcoin-cli/src/args.rs] [VERIFIED: packages/open-bitcoin-cli/src/client.rs] [VERIFIED: packages/open-bitcoin-rpc/src/http.rs] [VERIFIED: packages/open-bitcoin-rpc/src/context.rs] [VERIFIED: packages/open-bitcoin-rpc/src/method.rs] [VERIFIED: packages/open-bitcoin-rpc/src/dispatch.rs]
4. **Freshness, backup export, and read-only inspection.** Extend shared wallet status for `fresh` / `partial` / `scanning`, add Open Bitcoin-owned backup export for managed wallets, and upgrade the existing read-only detector to format-aware external wallet classification. [VERIFIED: packages/open-bitcoin-node/src/status.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/status.rs] [VERIFIED: packages/open-bitcoin-cli/src/operator/detect.rs] [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md]
5. **Verification and parity closeout.** Add breadcrumb coverage for any new Rust files, hermetic wallet routing/send/rescan/backup tests, update parity docs, and gate the phase on `bash scripts/verify.sh`. [VERIFIED: AGENTS.md] [VERIFIED: docs/parity/index.json] [VERIFIED: scripts/verify.sh]

The planner should explicitly defer `loadwallet`, `unloadwallet`, `listwallets`, `gethdkeys`, full fee-estimator parity, and any external wallet mutation to later phases even if implementation seams make them tempting follow-ons. [VERIFIED: .planning/phases/20-wallet-runtime-expansion/20-CONTEXT.md] [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/wallet.cpp]
