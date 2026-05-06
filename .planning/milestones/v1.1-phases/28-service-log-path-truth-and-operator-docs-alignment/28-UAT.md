---
status: complete
phase: 28-service-log-path-truth-and-operator-docs-alignment
source:
  - 28-01-SUMMARY.md
  - 28-02-SUMMARY.md
started: 2026-05-06T11:57:47Z
updated: 2026-05-06T12:01:06Z
---

## Current Test

[testing complete]

## Tests

### 1. Service Install Preview Uses Concrete Log File
expected: |
  From the repo root, run:
  `tmp=/tmp/open-bitcoin-uat-28-preview; rm -rf "$tmp"; mkdir -p "$tmp"; cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --network regtest --datadir="$tmp" --no-color service install`

  The dry-run output should include generated service content with `StandardOutPath` and `StandardErrorPath` pointing at `/tmp/open-bitcoin-uat-28-preview/logs/open-bitcoin.log`. It should not show the raw `/tmp/open-bitcoin-uat-28-preview/logs` directory as the log file.
result: pass
evidence: "Dry-run launchd plist included StandardOutPath and StandardErrorPath set to /tmp/open-bitcoin-uat-28-preview/logs/open-bitcoin.log."

### 2. Service Status Reports Log Path Truth
expected: |
  From the repo root, run:
  `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --network regtest --datadir=/tmp/open-bitcoin-uat-28-preview --no-color service status`

  The status output should always include a `logs:` line. If a launchd service is installed, `logs:` should show the effective installed service log file path ending in `open-bitcoin.log`; if no file-backed service log path is available, it should show an explicit unavailable reason rather than omitting the line.
result: pass
evidence: "service status printed a logs: line with the installed launchd file-backed path /Users/peterryszkiewicz/.open-bitcoin/logs/open-bitcoin.log."

### 3. Operator Runtime Guide Matches Shipped Behavior
expected: |
  Open `docs/operator/runtime-guide.md` and review the Service Lifecycle section. It should say service previews derive a concrete service-managed log file at `<log_dir>/open-bitcoin.log`, and that `open-bitcoin service status` surfaces the effective path from the installed service definition.
result: pass
evidence: "docs/operator/runtime-guide.md Service Lifecycle section describes <log_dir>/open-bitcoin.log and service status surfacing the installed effective path."

## Summary

total: 3
passed: 3
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
