---
phase: 07
slug: wallet-core-and-adapters
status: verified
threats_open: 0
asvs_level: 1
created: 2026-04-26
updated: 2026-04-26
---

# Phase 07 - Security

Per-phase security contract: threat register, accepted risks, and audit trail.

## Scope

Phase 07 delivers the first wallet slice: pure-core descriptor, key, address,
snapshot, balance, transaction-building, and signing logic, plus a node-side
managed wallet adapter for snapshot persistence and RPC-shaped projections.

No explicit `<threat_model>` block or `## Threat Flags` entries were present in
the Phase 07 plan and summary artifacts. This report therefore records an
empty declared threat register and verifies the security-relevant controls
implied by the phase artifacts and repo guardrails.

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| Descriptor and key input to wallet core | Wallet descriptor text, WIF private keys, public keys, and x-only keys enter the wallet parser before becoming typed wallet records. | Descriptor syntax, optional checksum suffixes, Base58Check WIF payloads, network prefixes, public key bytes, x-only public key bytes, and labels. |
| Chainstate snapshot to wallet state | The wallet rebuilds its UTXO view from chainstate snapshots without owning persistence or direct I/O. | Tip height, median time past, UTXO outpoints, output scripts, values, coinbase flags, and creation metadata. |
| Build request to unsigned transaction | Recipient requests and fee policy are converted into deterministic inputs, outputs, change, lock time, and RBF sequence values. | Recipient scripts and amounts, fee rate, change descriptor selection, coinbase maturity, lock time, and spendable UTXOs. |
| Signing request to consensus transaction | Built transactions are signed through the consensus sighash helpers before script verification can validate the spend. | Selected UTXOs, previous outputs, script codes, signatures, witness stacks, scriptSigs, sighash type, and verification flags. |
| Wallet core to node adapter | The node shell stores and reloads wallet snapshots while keeping persistence outside the pure wallet crate. | `WalletSnapshot`, descriptor records, UTXOs, managed wallet metadata, and adapter-owned store state. |
| Parity evidence to reviewers | Phase 7 parity docs, UAT, and tests document implemented scope and deferred wallet capabilities. | Human-readable catalog entries, machine-readable parity status, verification reports, and UAT results. |

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| _none declared_ | - | Phase 07 artifacts | - | No explicit Phase 07 threat model or summary threat flags were found. Security-relevant controls verified below. | closed |

Status: open, closed.
Disposition: mitigate (implementation required), accept (documented risk), transfer (third-party).

## Controls Verified

| Control | Evidence | Status |
|---------|----------|--------|
| Pure-core wallet isolation | `scripts/pure-core-crates.txt` includes `open-bitcoin-wallet`; `packages/open-bitcoin-wallet/Cargo.toml` depends on first-party pure-core crates plus `secp256k1`; `scripts/check-pure-core-deps.sh` rejects forbidden runtime, network, filesystem, process, thread, random, and TLS dependencies or imports in pure-core crates. | verified |
| Unsafe-code and production panic-site hardening | `packages/open-bitcoin-wallet/src/lib.rs` forbids unsafe code and denies unwrap, expect, panic, unreachable, todo, unimplemented, and panic-in-result functions outside tests. `scripts/check-panic-sites.sh` found no unclassified production panic-like sites. | verified |
| Typed descriptor and key validation | `SingleKeyDescriptor::parse` accepts only the Phase 7 single-key `pkh`, `sh(wpkh)`, `wpkh`, and `tr` forms, rejects ranged or unsupported descriptors, validates WIF network prefixes, and returns typed key, network, and syntax errors. | verified |
| WIF, address, and script boundary validation | `PrivateKey::from_wif`, address derivation, segwit encoding, taproot tweaking, and push-data helpers enforce checksum, key-length, witness-program, script-element, and taproot-tweak constraints before producing scripts or addresses. | verified |
| Snapshot rescan without hidden persistence | `Wallet::rescan_chainstate` imports only outputs matching wallet descriptor scripts, sorts the resulting UTXOs deterministically, records tip metadata, and keeps persistence outside the pure wallet crate. | verified |
| Spendability controls | Wallet balance and transaction construction exclude watch-only descriptors from spendable funds, keep immature coinbase outputs unspendable until the configured maturity window passes, and reject duplicate labels or unknown descriptors through typed errors. | verified |
| Deterministic transaction construction | `build_transaction` requires recipients, sorts spendable UTXOs by effective value with deterministic tie-breakers, requires explicit internal change for changeful spends, folds dust change into fees, and returns typed no-coin or insufficient-funds errors. | verified |
| Consensus-backed signing | `sign_transaction` signs legacy, nested segwit, native segwit, and taproot key-path spends through the canonical consensus sighash helpers; ECDSA signatures are normalized to low-S and verification uses the wallet's standard script flags. | verified |
| Adapter-owned wallet persistence | `packages/open-bitcoin-node/src/wallet.rs` defines `WalletStore`, `MemoryWalletStore`, and `ManagedWallet` so descriptor imports and rescans persist snapshots in the node shell rather than adding direct I/O to the wallet core. | verified |
| Honest known-gap tracking | `docs/parity/catalog/wallet.md` documents ranged descriptors, HD derivation, miniscript, multisig, PSBT, wallet encryption, backup and migration behavior, external signers, real-node rescans, and RPC-facing wallet semantics as deferred capabilities. | verified |

## Accepted Risks Log

No accepted risks.

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-04-26 | 0 | 0 | 0 | Codex |

## Verification Evidence

| Command | Result |
|---------|--------|
| Targeted `rg` scan for threat-model and threat-flag terms across Phase 07 plan and summary artifacts. | No explicit Phase 07 threat model or summary threat flags found. |
| Targeted `rg` scan for forbidden runtime imports and panic-like production code across Phase 07 wallet files and guard scripts. | Production controls and dependency surfaces reviewed; panic-like findings are test-only or governed by the production guard script. |
| `bash scripts/check-pure-core-deps.sh` | Passed: pure-core dependency and import checks passed. |
| `bash scripts/check-panic-sites.sh` | Passed: no unclassified production panic-like sites. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-wallet --all-targets` | Passed: 35 wallet unit tests passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node wallet` | Passed: 2 matching node wallet tests passed. |

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

Approval: verified 2026-04-26
