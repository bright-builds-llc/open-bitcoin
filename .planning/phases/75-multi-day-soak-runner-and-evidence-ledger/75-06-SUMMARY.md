---
phase: 75-multi-day-soak-runner-and-evidence-ledger
plan: 06
subsystem: deterministic-verification
tags:
  - bun
  - verification
  - soak
  - parity
  - metrics
dependency_graph:
  requires:
    - 75-01
    - 75-02
    - 75-03
    - 75-04
    - 75-05
  provides:
    - phase75-soak-runner-checker
    - phase75-default-verifier-wiring
    - refreshed-lines-of-code-metrics
  affects:
    - scripts/verify.sh
    - scripts/check-phase75-soak-runner.ts
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md
tech_stack:
  added: []
  patterns:
    - fixture-rooted Bun verification tests
    - deterministic source/docs/parity anchor checks
    - nested test modules excluded from production panic-site scanning
key_files:
  created:
    - scripts/check-phase75-soak-runner.ts
    - packages/open-bitcoin-cli/src/operator/soak/runtime.rs
    - packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs
    - packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs
    - .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-06-SUMMARY.md
  modified:
    - scripts/check-phase75-soak-runner.test.ts
    - scripts/verify.sh
    - scripts/check-panic-sites.sh
    - packages/open-bitcoin-cli/src/operator/soak.rs
    - packages/open-bitcoin-cli/src/operator/runtime.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md
decisions:
  - Keep Phase 75 default verification deterministic and offline by checking source, docs, parity roots, and verifier wiring instead of running a live soak.
  - Split the soak runtime and runtime tests into dedicated modules so the repo file-length gate remains enforceable.
  - Treat nested src/**/tests modules as test code in the panic-site scanner, matching the scanner's stated production-only boundary.
metrics:
  started_at: 2026-06-15T02:53:49Z
  completed_at: 2026-06-15T03:44:06Z
  duration_seconds: 3017
  tasks: 1
  files_changed: 12
requirements:
  completed:
    - SOAK-01
    - SOAK-02
    - SOAK-03
    - SOAK-04
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 75-2026-06-14T22-59-23
generated_at: 2026-06-15T03:44:06Z
---

# Phase 75 Plan 06: Deterministic Soak Verification Summary

Fixture-rooted Phase 75 soak checker wired into default verification with LOC freshness and production-gate cleanup.

## Accomplishments

- Added `scripts/check-phase75-soak-runner.ts` to verify all SOAK requirement IDs, source anchors, docs/parity roots, support redaction anchors, D-11 resume coverage, forbidden live-network/service-manager/default soak strings, and verify ordering.
- Added fixture-rooted Bun tests proving the checker fails on missing requirements, docs, source anchors, support redaction assertions, forbidden verify strings, and wrong verify ordering.
- Wired `bun test scripts/check-phase75-soak-runner.test.ts` and `bun run scripts/check-phase75-soak-runner.ts` into `scripts/verify.sh` immediately after the v1.6 boundary checker.
- Refreshed `docs/metrics/lines-of-code.md` and parity breadcrumbs for the new split Rust files.
- Kept default verification public-network-free, service-manager-free, and free of multi-day wall-clock soak gates.

## Completed Tasks

| Task | Description | Commit |
| --- | --- | --- |
| RED | Add failing checker coverage for Phase 75 verification boundaries | `961a458` |
| GREEN | Implement checker, verify wiring, and verifier cleanup | `045c166` |

## Verification

| Command | Result |
| --- | --- |
| `bun test scripts/check-phase75-soak-runner.test.ts` before implementation | Failed as expected: 7 pass, 1 fail because the checker was missing |
| `cargo fmt --manifest-path packages/Cargo.toml --all` | Passed |
| `bun test scripts/check-phase75-soak-runner.test.ts` | Passed: 8 tests |
| `bun --check scripts/check-phase75-soak-runner.ts` | Passed |
| `bun run scripts/check-phase75-soak-runner.ts` | Passed |
| `bash scripts/check-panic-sites.sh` | Passed |
| `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check` | Passed |
| `bun run scripts/check-parity-breadcrumbs.ts --check` | Passed: 249 Rust files |
| `bash scripts/check-file-lengths.sh` | Passed: 195 production Rust files checked, limit 628 lines |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_ --all-features` | Passed: 29 tests |
| `bash scripts/verify.sh` | Passed in 24m 15.046s |
| Plan acceptance `rg` checks for checker constants/functions/anchors/forbidden strings/verify ordering | Passed |

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Split Phase 75 soak runtime code to satisfy the production file-length gate**

- **Found during:** Full `bash scripts/verify.sh`
- **Issue:** The verifier failed because `packages/open-bitcoin-cli/src/operator/soak.rs` and `packages/open-bitcoin-cli/src/operator/runtime.rs` exceeded the production Rust file-length limit after prior Phase 75 work.
- **Fix:** Moved soak runtime command execution and helpers into `soak/runtime.rs` and `soak/runtime/helpers.rs`, moved inline soak runtime tests into `soak/tests/runtime.rs`, trimmed blank lines in `operator/runtime.rs`, and updated parity breadcrumbs plus checker anchors.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/soak.rs`, `packages/open-bitcoin-cli/src/operator/soak/runtime.rs`, `packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs`, `packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs`, `packages/open-bitcoin-cli/src/operator/runtime.rs`, `docs/parity/source-breadcrumbs.json`, `scripts/check-phase75-soak-runner.ts`, `scripts/check-phase75-soak-runner.test.ts`
- **Commit:** `045c166`

**2. [Rule 3 - Blocking] Align panic-site scanner with nested test modules**

- **Found during:** Full `bash scripts/verify.sh`
- **Issue:** `scripts/check-panic-sites.sh` reported test-only `expect(...)` calls under nested `src/**/tests/*.rs` modules as unclassified production panic-like sites.
- **Fix:** Extended the existing test-file skip rule from `*/tests.rs` to also skip `*/tests/*`, matching the script's production-only intent and avoiding brittle allowlist entries for test setup code.
- **Files modified:** `scripts/check-panic-sites.sh`
- **Commit:** `045c166`

## Known Stubs

None. Stub scan findings were limited to local accumulator/default initializers in checker tests and shell scripts.

## Auth Gates

None.

## Deferred Issues

None.

## Self-Check: PASSED

- Verified expected created/modified files exist.
- Verified task commits `961a458` and `045c166` exist in git history.
- Verified summary whitespace with `git diff --check`.
