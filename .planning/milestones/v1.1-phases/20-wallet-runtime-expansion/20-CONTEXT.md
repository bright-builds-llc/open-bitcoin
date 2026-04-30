---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 20-2026-04-27T09-35-46
generated_at: 2026-04-27T09:39:13.841Z
---

# Phase 20: Wallet Runtime Expansion - Context

**Gathered:** 2026-04-27
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Expand the existing headless wallet core into practical runtime workflows that
operators can actually use against durable sync state. This phase owns safe
send behavior, wallet-scoped selection, practical receive or change address
management through ranged descriptors, resumable wallet rescans and balance
freshness, and read-only backup or migration inspection. It does not claim full
Bitcoin Core or Knots multiwallet lifecycle parity, PSBT flows, miniscript,
multisig, restore or import mutation, or automatic migration.

</domain>

<decisions>
## Implementation Decisions

### Send workflow and safety contract
- **D-01:** Add a baseline-compatible mutating wallet send path shaped around
  `sendtoaddress`-style semantics instead of promoting the broader Core `send`
  RPC surface in this phase.
- **D-02:** Back both operator preview and final send execution with one shared
  pure-core send-intent model so fee, change, and error decisions stay
  deterministic and auditable across CLI and RPC adapters.
- **D-03:** Keep preview and confirmation behavior as an Open Bitcoin-owned
  operator wrapper. Baseline parity applies to the commit path and supported
  parameter semantics, not to the preview surface itself.
- **D-04:** The send path must support wallet-relevant fee and safety controls
  that matter for practical parity here: explicit fee-rate or estimate inputs,
  replaceability, change handling, fee ceilings, deterministic insufficient-fund
  or invalid-parameter errors, and clear confirmation before a mutating operator
  send.

### Wallet selection and runtime identity
- **D-05:** Move from one anonymous managed wallet snapshot to a lightweight
  named-wallet registry with durable per-wallet identity and explicit wallet
  selection metadata.
- **D-06:** Support the expected wallet-scoped operator surface through
  `-rpcwallet`-style selection and wallet-routed RPC handling for the current
  wallet method subset, while explicitly deferring full `loadwallet`,
  `unloadwallet`, and `listwallets` lifecycle parity.
- **D-07:** Keep root-vs-wallet boundary behavior explicit in docs and tests so
  node-scoped RPCs remain at the root surface and wallet-scoped RPCs require an
  identified wallet when appropriate.

### Descriptor range and address management
- **D-08:** Replace the Phase 7 fixed single-key descriptor limitation with a
  narrow Core-shaped active ranged-descriptor model: one external and one
  internal descriptor role, each with persisted range metadata and `next_index`
  state.
- **D-09:** Add only the minimum derivation support needed for practical
  receive/change rotation in this phase: xpub/xprv plus path forms required for
  single-key ranged descriptors. Multipath descriptors, miniscript, multisig,
  PSBT, and broader descriptor-wallet parity remain deferred.
- **D-10:** New-address and change-address allocation must be restart-safe and
  wallet-local. Cursor advancement belongs to durable wallet state rather than
  transient CLI or RPC adapter logic.

### Rescan, recovery, backup, and migration inspection
- **D-11:** Wallet rescans become resumable per-wallet runtime jobs with a
  persisted cursor and captured target tip so restart can continue an in-flight
  scan without silently resetting progress.
- **D-12:** Wallet balance and status output must distinguish fresh, partial,
  and scanning states. Until a wallet catches up to the durable sync tip and
  associated refresh work completes, operator surfaces should expose that the
  wallet view is incomplete instead of implying a final balance.
- **D-13:** Phase 20 may create Open Bitcoin-owned wallet backup exports, but it
  must not restore, import, copy, or mutate external Core/Knots wallets.
- **D-14:** Existing Core/Knots wallet candidate inspection becomes
  schema-aware and strictly read-only: enough metadata to support backup and
  later migration planning, without crossing the mutation boundary reserved for
  Phase 21.

### Architecture and verification boundary
- **D-15:** Keep pure business logic in `open-bitcoin-wallet` where practical:
  ranged descriptor state, send-intent construction, rescan progress state, and
  wallet-domain validation should stay free of filesystem, network, and prompt
  side effects.
- **D-16:** `open-bitcoin-node`, `open-bitcoin-rpc`, and `open-bitcoin-cli`
  own persistence, wallet registry storage, RPC routing, operator confirmation,
  read-only external wallet inspection, and long-running rescan orchestration.
- **D-17:** Phase 20 is not complete unless repo-owned tests cover the new send
  contract, wallet selection routing, ranged descriptor cursor persistence,
  restart-safe rescans, and read-only inspection or backup behavior without
  relying on public network access.

</decisions>

<canonical_refs>
## Canonical References

Downstream agents MUST read these before planning or implementing.

### Phase scope
- `.planning/ROADMAP.md` — Phase 20 goal, requirements, and success criteria.
- `.planning/REQUIREMENTS.md` — `WAL-04`, `WAL-05`, `WAL-06`, `WAL-07`,
  `WAL-08`, plus `MIG-02` carry-forward context.
- `.planning/PROJECT.md` — v1.1 operator-runtime constraints and parity goals.
- `.planning/STATE.md` — current milestone state and current-phase focus.
- `.planning/phases/07-wallet-core-and-adapters/07-CONTEXT.md` — locked wallet
  core scope and explicit Phase 7 deferrals.
- `.planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md` —
  operator UX, config precedence, and read-only detection decisions.
- `.planning/phases/19-ratatui-node-dashboard/19-CONTEXT.md` — wallet status
  and operator action expectations already established for later consumers.

### Architecture and parity docs
- `docs/architecture/cli-command-architecture.md` — operator vs
  `open-bitcoin-cli` routing boundary.
- `docs/architecture/config-precedence.md` — config ownership and precedence.
- `docs/architecture/status-snapshot.md` — shared status and unavailable-state
  contract for wallet freshness reporting.
- `docs/architecture/storage-decision.md` — durable wallet state expectations
  under the Fjall-backed adapter path.
- `docs/parity/catalog/wallet.md` — current wallet parity slice and known gaps.
- `docs/parity/index.json` — shipped-vs-deferred parity ledger contract.
- `docs/parity/source-breadcrumbs.json` — breadcrumb requirements for any new
  first-party Rust source or test files added under the scoped packages.

### Existing implementation to extend directly
- `packages/open-bitcoin-wallet/src/descriptor.rs`
- `packages/open-bitcoin-wallet/src/wallet.rs`
- `packages/open-bitcoin-wallet/src/wallet/build.rs`
- `packages/open-bitcoin-wallet/src/wallet/scan.rs`
- `packages/open-bitcoin-wallet/src/wallet/sign.rs`
- `packages/open-bitcoin-node/src/wallet.rs`
- `packages/open-bitcoin-rpc/src/context.rs`
- `packages/open-bitcoin-rpc/src/method.rs`
- `packages/open-bitcoin-rpc/src/dispatch.rs`
- `packages/open-bitcoin-rpc/src/config.rs`
- `packages/open-bitcoin-cli/src/operator.rs`
- `packages/open-bitcoin-cli/src/operator/runtime.rs`
- `packages/open-bitcoin-cli/src/operator/detect.rs`
- `packages/open-bitcoin-node/src/status.rs`

### Bitcoin Knots baseline
- `packages/bitcoin-knots/doc/descriptors.md`
- `packages/bitcoin-knots/doc/managing-wallets.md`
- `packages/bitcoin-knots/src/wallet/wallet.cpp`
- `packages/bitcoin-knots/src/wallet/rpc/spend.cpp`
- `packages/bitcoin-knots/test/functional/wallet_descriptor.py`
- `packages/bitcoin-knots/test/functional/wallet_gethdkeys.py`
- `packages/bitcoin-knots/test/functional/wallet_backup.py`

### Standards
- `AGENTS.md`
- `AGENTS.bright-builds.md`
- `standards-overrides.md`
- `https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md`
- `https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/code-shape.md`
- `https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/testing.md`
- `https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/verification.md`
- `https://github.com/bright-builds-llc/bright-builds-rules/blob/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/languages/rust.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `open-bitcoin-wallet` already has deterministic transaction building,
  signing, snapshot persistence shape, descriptor roles, and chainstate-based
  rescans for the fixed single-key slice.
- `ManagedWallet` in `open-bitcoin-node` already provides the adapter seam for
  durable wallet persistence that Phase 20 should extend into a named-wallet
  registry instead of bypassing.
- `open-bitcoin-rpc` already exposes wallet-centric methods such as
  `getwalletinfo`, `getbalances`, `listunspent`, `importdescriptors`,
  `rescanblockchain`, `buildtransaction`, and `buildandsigntransaction`.
- `open-bitcoin-cli` already has operator/runtime routing, read-only
  installation detection, and explicit confirmation-oriented operator patterns
  that the wallet send path can reuse.

### Established Patterns
- Pure-core crates own deterministic business rules; node, RPC, and CLI crates
  own persistence and effectful orchestration.
- Operator-facing surfaces favor explicit preview, dry-run, unavailable-state
  reporting, and read-only detection before any mutation.
- Durable state lives behind the Fjall-backed `open-bitcoin-node` storage
  adapter and should survive restart with typed recovery behavior.
- New first-party Rust files under the scoped packages require parity
  breadcrumb coverage and tests that stay hermetic under the repo-native verify
  contract.

### Integration Points
- `packages/open-bitcoin-wallet` should own ranged descriptor state, send-intent
  construction, and rescan-domain progress semantics.
- `packages/open-bitcoin-node` should own named-wallet persistence, rescan job
  durability, and registry-aware wallet runtime composition.
- `packages/open-bitcoin-rpc` should add wallet-scoped routing and the
  `sendtoaddress`-style commit path while preserving clear parity boundaries for
  deferred methods.
- `packages/open-bitcoin-cli` should expose wallet selection and preview or
  confirmation flows through the existing operator surface without collapsing
  back into undocumented ad hoc commands.

</code_context>

<specifics>
## Specific Ideas

- Prefer the smallest honest parity step over a wide but shallow wallet-surface
  grab: one good send path, one good named-wallet selection model, and one good
  ranged-descriptor slice beat partial implementations of every Core wallet RPC.
- Treat wallet freshness as an operator-visible state, not an internal detail:
  if a rescan is incomplete or a wallet is behind the durable sync tip, say so
  explicitly in status and wallet responses.
- Keep external-wallet interaction read-only in this phase even if parsing
  external metadata becomes more capable. Backup-aware mutation belongs to the
  next migration phase.

</specifics>

<deferred>
## Deferred Ideas

- full `loadwallet`, `unloadwallet`, and `listwallets` lifecycle parity
- the broader Core `send`, `sendall`, PSBT, and bump-fee wallet surfaces
- miniscript, multisig, external signers, and descriptor-wallet breadth beyond
  the minimal single-key ranged slice
- restore, import, copy, or mutation of Core/Knots wallets
- automatic migration plans and execution, which belong to Phase 21

</deferred>

---

*Phase: 20-wallet-runtime-expansion*
*Context gathered: 2026-04-27*
