---
status: complete
phase: 07-wallet-core-and-adapters
source:
  - .planning/phases/07-wallet-core-and-adapters/07-01-SUMMARY.md
  - .planning/phases/07-wallet-core-and-adapters/07-02-SUMMARY.md
  - .planning/phases/07-wallet-core-and-adapters/07-03-SUMMARY.md
  - .planning/phases/07-wallet-core-and-adapters/07-04-SUMMARY.md
started: 2026-04-26T11:09:15Z
updated: 2026-04-26T11:15:49Z
---

## Current Test

[testing complete]

## Tests

### 1. Descriptor, WIF, and Address Derivation
expected: From the repo root, `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-wallet parser_accepts_single_key_descriptors_with_optional_checksums`, `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-wallet private_key_round_trips_compressed_wif`, `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-wallet key_hash_addresses_match_known_upstream_vectors`, and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-wallet taproot_addresses_match_bip86_style_tweak_logic` pass, showing the wallet crate parses the in-scope single-key descriptors and derives legacy, nested segwit, native segwit, and taproot addresses from the vendored Knots-aligned fixtures.
result: pass

### 2. Chainstate Rescan, UTXO Tracking, and Balance View
expected: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-wallet rescan_populates_wallet_balance_from_matching_chainstate_outputs`, `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-wallet coinbase_outputs_stay_immature_until_the_maturity_window_passes`, and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-wallet watch_only_outputs_do_not_count_as_spendable` pass, proving wallet balances and spendable UTXOs rebuild from `ChainstateSnapshot` data without embedding persistence or chainstate logic in the wallet core.
result: pass

### 3. Deterministic Transaction Building
expected: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-wallet build_transaction_requires_change_descriptor_for_changeful_spends`, `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-wallet build_transaction_reports_insufficient_funds_and_uses_snapshot_sorting`, and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-wallet final_sequence_and_dust_change_paths_are_covered` pass, showing transaction construction uses deterministic input ordering, explicit change handling, and dust folding.
result: pass

### 4. Wallet Signing Across Descriptor Types
expected: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-wallet legacy_descriptor_signing_populates_script_sig`, `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-wallet nested_segwit_and_taproot_signing_cover_remaining_descriptor_paths`, and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-wallet signing_reports_missing_private_keys_and_watch_only_paths` pass, proving legacy, nested segwit, native segwit, and taproot key-path spends route through the wallet signing and consensus sighash surfaces with typed error behavior.
result: pass

### 5. Managed Wallet Persistence and Recovery
expected: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node wallet` passes, proving `WalletStore`, `MemoryWalletStore`, and `ManagedWallet` keep descriptor import, snapshot recovery, balance, build, and sign flows adapter-owned in the node shell.
result: pass

### 6. Wallet Parity Evidence and Ledger
expected: `docs/parity/catalog/wallet.md` and `docs/parity/index.json` mark the wallet surface as done with explicit HD, miniscript, PSBT, signer, encryption, and RPC/CLI deferrals, and `bash scripts/verify.sh` succeeds with the wallet crate included in pure-core verification, coverage, Bazel smoke build, parity checks, and report checks.
result: pass

## Summary

total: 6
passed: 6
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
