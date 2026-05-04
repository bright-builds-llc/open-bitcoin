---
status: complete
phase: 20-wallet-runtime-expansion
source: [20-01-SUMMARY.md, 20-02-SUMMARY.md, 20-03-SUMMARY.md, 20-04-SUMMARY.md, 20-05-SUMMARY.md]
started: 2026-05-03T18:25:00Z
updated: 2026-05-04T02:51:21Z
---

## Current Test

[testing complete]

## Tests

### 1. Wallet-Scoped RPC Routing
expected: Calling a wallet RPC such as `getwalletinfo`, `getnewaddress`, or `sendtoaddress` with `-rpcwallet <name>` or the HTTP path `/wallet/<name>` should succeed against that named wallet, while the same wallet-only call at `/` without wallet selection should fail with an explicit wallet-scope error instead of silently using the wrong wallet.
result: pass

### 2. Ranged Receive Address Allocation
expected: In a wallet loaded from a supported ranged single-key descriptor (`wpkh(...)`, `sh(wpkh(...))`, or `tr(...)` xpub/xprv form), repeated `getnewaddress` calls should succeed, return distinct addresses, and keep advancing the descriptor cursor instead of repeating the same child key.
result: pass

### 3. Change Address and Send Intent Wiring
expected: A wallet send flow should resolve the requested fee policy, produce a valid change output when needed, and avoid reusing the last receive address as change or failing because the change-policy wiring is missing.
result: pass

### 4. Descriptor and Wallet Metadata Surface
expected: `listdescriptors` and `getwalletinfo` for a named wallet should succeed and expose the loaded descriptors plus the Phase 20 wallet metadata, including the wallet name and freshness or scanning state.
result: pass

### 5. Durable Wallet Selection Across Restart
expected: After creating or loading named wallets and selecting one, stopping and restarting `open-bitcoind` should preserve the wallet registry, the selected wallet, and descriptor cursor state so the same wallet remains available without manual reconstruction.
result: pass

### 6. Restart-Safe Rescan Recovery
expected: Starting `rescanblockchain`, interrupting the node, and restarting it should resume from a bounded checkpoint instead of restarting from scratch or forgetting that the wallet is still scanning.
result: pass

### 7. Wallet Freshness in Status and Dashboard
expected: `open-bitcoin status` or `open-bitcoin dashboard` should distinguish wallet states such as `fresh`, `stale`, `partial`, or `scanning`, and show scan progress when a rescan is active instead of only a generic balance or trust view.
result: pass

### 8. Read-Only Wallet Inspection in Onboarding
expected: `open-bitcoin onboard --detect-existing` should surface detected external wallet candidates with product, chain, and format hints plus a read-only caution, without implying that Open Bitcoin can back up, restore, import, or mutate those external wallets in Phase 20.
result: pass

### 9. Operator Wallet Send Preview and Confirm
expected: `open-bitcoin wallet --wallet <name> send ...` should render a deterministic preview and refuse mutation unless `--confirm` is present; adding `--confirm` should submit the send through the wallet-scoped RPC path.
result: pass

### 10. Managed Wallet Backup Export Safety
expected: `open-bitcoin wallet --wallet <name> backup <path>` should export a one-way JSON backup to a safe destination, but reject destinations that overlap detected Core or Knots wallet candidates instead of overwriting or mixing with external wallet data.
result: pass

## Summary

total: 10
passed: 10
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
