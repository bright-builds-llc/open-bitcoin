---
status: complete
phase: 42-live-smoke-entry-and-network-preflight
source:
  - 42-01-SUMMARY.md
started: 2026-05-24T14:28:06.543Z
updated: 2026-05-24T14:31:04.572Z
---

## Current Test

[testing complete]

## Tests

### 1. Manual Peer Generated Config
expected: Running the live smoke runner with --manual-peer and no --config should create an open-bitcoin-live-mainnet-smoke.jsonc config under the output directory, record that path in the JSON report, and include the exact manual peer in the generated config.
result: pass

### 2. Endpoint Outcome Evidence
expected: A successful smoke run should write JSON and Markdown reports with network_preflight.endpoint_outcomes showing observable endpoint states such as connected preflight endpoints, skipped DNS entries from fixtures, and runtime handshook telemetry when the daemon reports peer capabilities.
result: pass

### 3. Typed No-Progress Guidance
expected: When the smoke runner times out without header or block progress, the report should include a typed maybeNoProgressCause and nextAction that explains the likely network failure mode instead of only printing a generic timeout.
result: pass

### 4. Cancellation Evidence
expected: Sending SIGTERM or SIGINT to the smoke runner should write partial reports with status cancelled and a nextAction describing operator cancellation, then exit nonzero.
result: pass

### 5. Operator Documentation
expected: The runtime guide should document the manual-peer command form, generated config behavior, endpoint outcomes, typed no-progress causes, and cancellation semantics in operator-facing language.
result: pass

## Summary

total: 5
passed: 5
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
