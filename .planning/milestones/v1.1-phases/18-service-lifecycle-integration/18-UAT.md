---
status: complete
phase: 18-service-lifecycle-integration
source:
  - 18-01-SUMMARY.md
  - 18-02-SUMMARY.md
  - 18-03-SUMMARY.md
started: 2026-05-03T17:05:00Z
updated: 2026-05-03T17:07:57Z
---

## Current Test

[testing complete]

## Tests

### 1. Service Command Help Surface
expected: Run `cargo run --manifest-path packages/Cargo.toml --package open-bitcoin-cli --bin open-bitcoin -- service --help`. It should list the `status`, `install`, `uninstall`, `enable`, and `disable` subcommands. It should also show a global `--apply` flag whose help text says changes are dry-run by default unless `--apply` is passed.
result: pass

### 2. Service Install Dry-Run Preview
expected: Run `cargo run --manifest-path packages/Cargo.toml --package open-bitcoin-cli --bin open-bitcoin -- service install`. It should stay in dry-run mode, say `pass --apply to make changes`, show that it would write `~/Library/LaunchAgents/org.open-bitcoin.node.plist`, list the `launchctl enable` and `launchctl bootstrap` commands, say the scope is user-level with no sudo required, and print generated plist content that includes the Open Bitcoin binary path plus `--datadir`, `--config`, and log paths.
result: pass

### 3. Standalone Service Status Before Install
expected: Run `cargo run --manifest-path packages/Cargo.toml --package open-bitcoin-cli --bin open-bitcoin -- service status`. On an unmanaged setup it should report `service: unmanaged`, `installed: false`, `enabled: false`, `running: false`, logs unavailable because the service is not installed, and a hint telling you to run `open-bitcoin service install` to preview what would be created.
result: pass

### 4. Integrated Operator Status Service Line
expected: Run `cargo run --manifest-path packages/Cargo.toml --package open-bitcoin-cli --bin open-bitcoin -- status`. The human-readable output should include a `Service:` line that reports `manager=launchd installed=false enabled=false running=false` instead of leaving the service fields unavailable.
result: pass

## Summary

total: 4
passed: 4
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
