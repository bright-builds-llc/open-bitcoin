---
status: complete
phase: 66-compatibility-harness-operator-wrapper
source:
  - 66-01-SUMMARY.md
started: 2026-06-09T14:05:19.000Z
updated: 2026-06-09T14:05:19.000Z
---

## Current Test

[testing complete]

## Tests

### 1. Compatibility harness command and report generation
expected: `open-bitcoin compatibility harness` should accept a peer endpoint, scenario, network, and output directory, then write stable `compatibility-harness-report.json` and `compatibility-harness-report.md` artifacts without contacting public peers.
result: pass

### 2. Report shape, diagnosis coverage, and redaction boundaries
expected: The reports should include peer endpoint, network, scenario, negotiated capabilities, failing step, diagnosis, transcript summary, redaction boundaries, next action, and report paths; built-in scenarios should cover every stable compatibility diagnosis while omitting raw wire payloads, credentials, cookies, wallet material, and unbounded peer logs.
result: pass

### 3. Operator docs and parity boundary
expected: `docs/operator/runtime-guide.md` and `docs/parity/catalog/p2p.md` should document repo-local Cargo/Bazel command forms, report interpretation, supported diagnosis values, and the boundary that compatibility reports are opt-in local evidence, not proof of public-peer contact, inbound serving, relay completeness, or production-node readiness.
result: pass

### 4. Deterministic verification boundary
expected: `scripts/check-phase66-compatibility-wrapper.ts` should validate source/docs/test strings and default-verification exclusions; `scripts/verify.sh` should run that checker and should not run compatibility harness peer-endpoint scenarios, live-smoke, or manual-peer public-network commands.
result: pass

## Summary

total: 4
passed: 4
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[]

## Evidence

- Test 1 passed by repo inspection of `packages/open-bitcoin-cli/src/operator.rs`, `packages/open-bitcoin-cli/src/operator/runtime.rs`, `packages/open-bitcoin-cli/src/operator/compatibility.rs`, and `packages/open-bitcoin-cli/tests/operator_binary.rs`.
- Test 2 passed with `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli open_bitcoin_compatibility --all-features`, which ran 2 matching compatibility wrapper tests successfully.
- Test 2 also passed with `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network compatibility --all-features`, which ran 11 matching network compatibility tests successfully.
- Test 3 passed by repo inspection of `docs/operator/runtime-guide.md` and `docs/parity/catalog/p2p.md`.
- Test 4 passed with `bun run scripts/check-phase66-compatibility-wrapper.ts`, which printed `Phase 66 compatibility wrapper checks passed.`
- `scripts/verify.sh` contains `bun run scripts/check-phase66-compatibility-wrapper.ts` and contains no `compatibility harness --peer-endpoint`, `--scenario=service-bit-mismatch`, `run-live-mainnet-smoke`, or `--manual-peer` default-verification commands.
