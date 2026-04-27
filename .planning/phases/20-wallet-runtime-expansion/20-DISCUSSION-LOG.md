# Phase 20: Wallet Runtime Expansion - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution.
> Decisions are captured in CONTEXT.md — this log preserves alternatives considered.

**Date:** 2026-04-27T09:39:13.841Z
**Phase:** 20-Wallet Runtime Expansion
**Mode:** Yolo
**Areas discussed:** Send workflow contract, Wallet selection surface, Descriptor/address management model, Rescan/recovery/backup/migration inspection

---

## Send workflow contract

| Option | Description | Selected |
|--------|-------------|----------|
| Baseline-compatible `sendtoaddress` commit RPC plus shared preview/confirm core | Keep the mutating path parity-shaped while exposing operator preview and confirmation through a shared pure send-intent flow. | ✓ |
| Baseline-first commit RPC only | Reuse current build/sign helpers and leave preview entirely to existing lower-level flows. | |
| Broader Core `send`-style surface | Expand now into the wider wallet-send RPC family with preview/commit options. | |

**User's choice:** Baseline-compatible `sendtoaddress` commit RPC plus shared preview/confirm core.
**Notes:** This keeps the actual mutation surface close to Knots expectations while preserving the repo’s explicit operator confirmation posture and avoiding the much broader `send`/PSBT surface in this phase.

---

## Wallet selection surface

| Option | Description | Selected |
|--------|-------------|----------|
| Lightweight named-wallet registry with explicit selection | Add durable wallet identity and support `-rpcwallet`/wallet-routed selection for the current wallet method subset. | ✓ |
| Default wallet only | Keep one anonymous managed wallet and treat selection as unsupported or a thin alias. | |
| Full multiwallet lifecycle parity | Implement `loadwallet`/`unloadwallet`/`listwallets`-style runtime behavior now. | |

**User's choice:** Lightweight named-wallet registry with explicit selection.
**Notes:** This is the smallest honest step toward `-rpcwallet` compatibility without pulling the full Core multiwallet lifecycle and loaded-wallet semantics into Phase 20.

---

## Descriptor/address management model

| Option | Description | Selected |
|--------|-------------|----------|
| Minimal active ranged single-key descriptors | Persist external/internal descriptor ranges plus `next_index` and add only the derivation support required for practical receive/change rotation. | ✓ |
| Broader xpub/xprv descriptor parity | Expand deeper into descriptor-wallet breadth now for future watch-only, multisig, and PSBT work. | |
| Wallet-local address pool above fixed descriptors | Add a repo-specific allocation layer without widening descriptor grammar yet. | |

**User's choice:** Minimal active ranged single-key descriptors.
**Notes:** Phase 7 already deferred ranges and HD behavior. Phase 20 closes that exact gap without inventing a parallel address model or absorbing future multisig/PSBT scope prematurely.

---

## Rescan, recovery, backup, and migration inspection

| Option | Description | Selected |
|--------|-------------|----------|
| Runtime plus schema-aware read-only inspection plus Open Bitcoin backup export | Add resumable rescans, explicit wallet freshness states, backup export, and richer read-only inspection of Core/Knots wallet candidates. | ✓ |
| Runtime-only plus path-level inspection | Keep rescans durable but leave wallet candidate inspection mostly at path/existence level. | |
| Restore/import/migrate primitives now | Start mutation-capable wallet migration in the same phase. | |

**User's choice:** Runtime plus schema-aware read-only inspection plus Open Bitcoin backup export.
**Notes:** This satisfies the runtime and inspection requirements without violating the dry-run-first, no-mutation migration boundary reserved for Phase 21.

## the agent's Discretion

- Exact CLI and RPC method naming around preview may follow the established
  operator/RPC split as long as parity claims for the mutating send path remain
  explicit.
- Exact storage layout for named-wallet metadata, rescan checkpoints, and
  backup manifests may follow existing Fjall namespace patterns.
- Exact formatter details for partial/scanning wallet status may be chosen
  during planning if the stale-vs-fresh distinction stays obvious to operators.

## Deferred Ideas

- Full Core multiwallet lifecycle management.
- Broad PSBT/fundtransaction/sendall/bumpfee wallet RPC coverage.
- Multisig, miniscript, and external-signer support.
- Mutation-capable wallet migration, restore, or import flows.
