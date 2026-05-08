---
status: complete
phase: 33-operator-surface-truth-and-coverage-cleanup
source:
  - 33-01-SUMMARY.md
  - 33-02-SUMMARY.md
  - 33-03-SUMMARY.md
started: 2026-05-08T14:04:06.633Z
updated: 2026-05-08T14:29:48.086Z
---

## Current Test

[testing complete]

## Tests

### 1. Status Watch Flag Rejected
expected: `open-bitcoin status --watch` exits with an argument error instead of silently parsing or running a no-op watch mode.
result: pass

### 2. Service Install Preview Is Truthful
expected: `open-bitcoin service install` previews the service file and commands by default, and the output says `--apply` is required before it mutates service state.
result: pass

### 3. Unmanaged Service Hint Points To Preview Flow
expected: `open-bitcoin service status` on an unmanaged install points operators to `open-bitcoin service install` to preview what would be created.
result: pass

### 4. Dashboard Snapshot Uses Real Sections
expected: Non-interactive `open-bitcoin dashboard --format human --no-color` renders the dashboard sections without ANSI escapes or deferred-command placeholder text.
result: pass

### 5. Phase 33 Verification Contract Still Passes
expected: The focused Open Bitcoin CLI regression tests and `bash scripts/verify.sh` pass from the repo root after the Phase 33 cleanup.
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
