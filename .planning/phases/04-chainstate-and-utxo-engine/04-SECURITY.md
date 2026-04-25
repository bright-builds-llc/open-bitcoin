---
phase: 04
slug: chainstate-and-utxo-engine
status: verified
threats_open: 0
asvs_level: 1
created: 2026-04-25
updated: 2026-04-25
---

# Phase 04 - Security

Per-phase security contract: threat register, accepted risks, and audit trail.

## Scope

Phase 04 delivers the first chainstate and UTXO engine slice: typed coin,
undo, chain-position, and snapshot models; pure-core block connect,
disconnect, and reorg behavior; targeted parity fixtures; and a node-side
snapshot adapter boundary.

No explicit `<threat_model>` block or `## Threat Flags` entries were present in
the Phase 04 plan and summary artifacts. This report therefore records an empty
declared threat register and verifies the security-relevant controls implied by
the phase artifacts and repo guardrails.

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| Typed Bitcoin domain values to chainstate core | Chainstate accepts already-typed blocks, transactions, outpoints, and consensus flags instead of raw network or disk bytes. | `Block`, `Transaction`, `OutPoint`, `ScriptVerifyFlags`, and `ConsensusParams` values. |
| Chainstate UTXO state to consensus validation | Chainstate derives `TransactionValidationContext` and `BlockValidationContext` from explicit UTXO and active-chain metadata before applying state transitions. | Coin values, scripts, coinbase flags, creation heights, median-time-past values, current time, and active-tip metadata. |
| Chainstate core to runtime persistence | Pure chainstate emits and restores `ChainstateSnapshot` values; `open-bitcoin-node` owns the `ChainstateStore` adapter and in-memory persistence wrapper. | Active chain positions, UTXO map, and per-block undo payloads. |
| Chainstate parity evidence to reviewers | Parity docs and UAT artifacts communicate implemented scope and known deferred surfaces. | Human-readable catalog entries, machine-readable parity status, verification reports, and UAT results. |

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| _none declared_ | - | Phase 04 artifacts | - | No explicit Phase 04 threat model or summary threat flags were found. Security-relevant controls verified below. | closed |

Status: open, closed.
Disposition: mitigate (implementation required), accept (documented risk), transfer (third-party).

## Controls Verified

| Control | Evidence | Status |
|---------|----------|--------|
| Pure-core chainstate isolation | `scripts/pure-core-crates.txt` includes `open-bitcoin-chainstate`; `open-bitcoin-chainstate` depends only on `open-bitcoin-consensus` and `open-bitcoin-primitives`; `scripts/check-pure-core-deps.sh` rejects forbidden runtime, network, filesystem, process, thread, random, and TLS dependencies or imports in pure-core crates. | verified |
| Unsafe-code and production panic-site hardening | `packages/open-bitcoin-chainstate/src/lib.rs` forbids unsafe code and denies unwrap, expect, panic, unreachable, todo, unimplemented, and panic-in-result functions outside tests. `scripts/check-panic-sites.sh` found no unclassified production panic-like sites. | verified |
| Explicit UTXO and undo modeling | `Coin`, `TxUndo`, `BlockUndo`, `ChainPosition`, `ChainstateSnapshot`, and `ChainTransition` model spendable state, rollback data, active-chain identity, and reorg deltas as typed Rust state. | verified |
| Context derivation before mutation | `connect_block_with_current_time` derives block and transaction validation contexts from active-chain and UTXO metadata, validates through existing consensus APIs, and uses typed `ChainstateError` variants for missing coins and consensus failures. | verified |
| Connect mutation integrity | Block connect stages mutations in a cloned UTXO map, records undo for non-coinbase spends, rejects missing prevouts, rejects BIP30-style output overwrites, skips unspendable outputs, enforces accumulated-fee bounds, and commits the new map only after validation succeeds. | verified |
| Disconnect and reorg integrity | Disconnect validates tip identity, requires stored undo, checks undo shape, removes created outputs, restores spent inputs in reverse order, and returns typed errors for mismatched or corrupt state. Reorg applies explicit disconnect and replacement-branch connect paths. | verified |
| Deterministic best-tip selection | `prefer_candidate_tip` chooses by chain work, then height, then block hash, matching the Phase 04 deterministic fixture contract while documenting the Knots pointer-identity difference. | verified |
| Adapter-owned persistence boundary | `ManagedChainstate`, `ChainstateStore`, and `MemoryChainstateStore` live in `open-bitcoin-node`; the pure-core crate exposes snapshots but does not own storage or runtime I/O. | verified |
| Honest known-gap tracking | `docs/parity/catalog/chainstate.md` documents disk-backed coins databases, cache-flush policy, assumeutxo, mempool repair, disconnected-transaction pools, header-chain validation, and full chainstate-manager behavior as outside this Phase 04 slice. | verified |

## Accepted Risks Log

No accepted risks.

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-04-25 | 0 | 0 | 0 | Codex |

## Verification Evidence

| Command | Result |
|---------|--------|
| `rg -n "threat_model|Threat Flags|threat|Threat|security|Security|mitigation|trust boundary|boundary" .planning/phases/04-chainstate-and-utxo-engine/*-PLAN.md .planning/phases/04-chainstate-and-utxo-engine/*-SUMMARY.md` | No explicit Phase 04 threat model or summary threat flags found. |
| `rg -n "unsafe|unwrap\\(|expect\\(|panic!|unreachable!|todo!|unimplemented!|std::fs|std::net|std::env|std::process|std::thread|tokio|reqwest|rustls|\\brand\\b" packages/open-bitcoin-chainstate/src packages/open-bitcoin-chainstate/tests packages/open-bitcoin-node/src/chainstate.rs packages/open-bitcoin-chainstate/Cargo.toml packages/open-bitcoin-node/Cargo.toml packages/open-bitcoin-chainstate/BUILD.bazel packages/open-bitcoin-node/BUILD.bazel scripts/check-pure-core-deps.sh scripts/check-panic-sites.sh scripts/pure-core-crates.txt` | Production controls and dependency surfaces reviewed; panic-like findings are test-only or governed by the production guard script. |
| `bash scripts/check-pure-core-deps.sh` | Passed: pure-core dependency and import checks passed. |
| `bash scripts/check-panic-sites.sh` | Passed: no unclassified production panic-like sites. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate` | Passed: 35 unit tests, 3 parity tests, and doc-tests passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node chainstate` | Passed: 3 matching node tests passed. |
| `git diff --check` | Passed. |

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

Approval: verified 2026-04-25
