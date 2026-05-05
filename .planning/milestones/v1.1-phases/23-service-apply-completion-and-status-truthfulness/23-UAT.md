---
status: complete
phase: 23-service-apply-completion-and-status-truthfulness
source:
  - 23-01-SUMMARY.md
  - 23-02-SUMMARY.md
  - 23-03-SUMMARY.md
started: 2026-05-05T02:15:12Z
updated: 2026-05-05T10:06:06Z
---

## Current Test

[testing complete]

## Tests

### 1. Unmanaged service status points to preview guidance
expected: If Open Bitcoin is not installed as a user service on this machine, run either `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- service status` or `bazel run //packages/open-bitcoin-cli:open_bitcoin -- service status`. The output should point you to `open-bitcoin service install` for the preview path and should not mention the older `--dry-run` wording.
result: pass

### 2. Service install preview and apply stay aligned
expected: Run either `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- service install` or `bazel run //packages/open-bitcoin-cli:open_bitcoin -- service install` first. It should preview the generated service file path and the exact manager commands apply mode will run. Then run the matching `... -- service install --apply` command. Apply mode should complete the same registration sequence immediately after the file write instead of stopping at file creation. On macOS that means the launchd registration completes; on Linux that means the user systemd reload and enable steps complete.
result: pass

### 3. Service status prints truthful installed enabled running flags
expected: Run either `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- service status` or `bazel run //packages/open-bitcoin-cli:open_bitcoin -- service status`. The output should include explicit `installed:`, `enabled:`, and `running:` booleans alongside the lifecycle label. Those booleans should match the real manager state instead of being guessed only from the lifecycle string, including odd combinations such as failed-but-enabled or running-but-disabled if you can reproduce them.
result: pass

### 4. Dashboard service actions still require confirmation and match CLI behavior
expected: Start the dashboard with either `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- dashboard` or `bazel run //packages/open-bitcoin-cli:open_bitcoin -- dashboard`. Trigger a service-affecting action such as install, enable, disable, or uninstall. The dashboard should show a confirmation prompt before side effects. After you confirm, the result should match the equivalent `open-bitcoin service ... --apply` behavior instead of diverging through a dashboard-only path.
result: pass

## Summary

total: 4
passed: 4
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps
