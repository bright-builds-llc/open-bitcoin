---
status: complete
phase: 08-rpc-cli-and-config-parity
source:
  - .planning/phases/08-rpc-cli-and-config-parity/08-01-SUMMARY.md
  - .planning/phases/08-rpc-cli-and-config-parity/08-02-SUMMARY.md
  - .planning/phases/08-rpc-cli-and-config-parity/08-03-SUMMARY.md
  - .planning/phases/08-rpc-cli-and-config-parity/08-04-SUMMARY.md
  - .planning/phases/08-rpc-cli-and-config-parity/08-05-SUMMARY.md
  - .planning/phases/08-rpc-cli-and-config-parity/08-06-SUMMARY.md
  - .planning/phases/08-rpc-cli-and-config-parity/08-07-SUMMARY.md
  - .planning/phases/08-rpc-cli-and-config-parity/08-08-SUMMARY.md
started: 2026-04-26T11:24:11Z
updated: 2026-04-26T11:35:41Z
---

## Current Test

[testing complete]

## Tests

### 1. RPC and CLI Build Ownership
expected: The repo exposes `open-bitcoin-rpc` and `open-bitcoin-cli` as first-party Cargo workspace members and Bazel targets. Running the repo verification path includes the `//:rpc` and `//:cli` smoke targets instead of leaving either operator surface as an unowned package.
result: pass

### 2. Runtime Config and Authentication
expected: `open-bitcoind` and the shared RPC runtime config load the supported `bitcoin.conf` slice, including `includeconf`, datadir and chain precedence, explicit user/password auth, and cookie auth with a `__cookie__:<64 lowercase hex chars>` secret stored with owner-only permissions.
result: pass

### 3. JSON-RPC HTTP Transport
expected: The RPC server accepts authenticated POST requests, rejects unauthenticated or unsupported transport shapes cleanly, preserves legacy versus JSON-RPC 2.0 status handling, supports batches, and treats JSON-RPC 2.0 notifications as no-content responses after execution.
result: skipped
reason: user skipped

### 4. Supported RPC Method Surface
expected: The supported Phase 8 RPC methods expose node, mempool, wallet, descriptor import, full-snapshot rescan, list/balance, raw-transaction submission, and Open Bitcoin build/sign behavior through typed request and response contracts with explicit errors for deferred or unsupported parameters.
result: skipped
reason: user skipped

### 5. CLI Startup and Method Parsing
expected: `open-bitcoin-cli` resolves config file, datadir, endpoint, and auth precedence on the client path; accepts supported flags such as `-named`, `-stdin`, `-stdinrpcpass`, `-rpcconnect`, `-rpcport`, `-rpcuser`, `-rpcpassword`, and `-rpccookiefile`; and fails deferred surfaces like `-netinfo` and `-rpcwallet` before transport with actionable errors.
result: skipped
reason: user skipped

### 6. GetInfo Helper Output
expected: `open-bitcoin-cli -getinfo` behaves as a thin helper over `getnetworkinfo`, `getblockchaininfo`, `getwalletinfo`, and `getbalances`, with deterministic `--json` output for automation and a stable human-readable dashboard for operators.
result: skipped
reason: user skipped

### 7. Headless CLI Operator Flow
expected: A headless operator can use the real CLI binary against the Phase 8 RPC transport to import descriptors, rescan the active snapshot, read balances and UTXOs, build and sign a transaction, and submit raw transaction hex without any GUI dependency.
result: skipped
reason: user skipped

### 8. Closed Gap Safety Semantics
expected: The closed Phase 8 gaps are observable: unsupported `rescanblockchain` height ranges are rejected before wallet mutation, `sendrawtransaction` rejects unenforced `maxfeerate` and `maxburnamount` values, duplicate named CLI parameters fail before HTTP transport, hostname `-rpcconnect` endpoints preserve precedence, and normal CLI commands do not wait on open stdin unless stdin flags are enabled.
result: skipped
reason: user skipped

## Summary

total: 8
passed: 2
issues: 0
pending: 0
skipped: 6
blocked: 0

## Gaps

[none yet]
