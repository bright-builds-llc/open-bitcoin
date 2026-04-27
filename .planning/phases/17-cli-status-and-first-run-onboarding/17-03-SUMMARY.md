---
phase: 17-cli-status-and-first-run-onboarding
plan: "03"
subsystem: cli-operator-detection
tags: [cli, detection, core, knots, datadir, wallets, services]

requires:
  - phase: 17-cli-status-and-first-run-onboarding
    provides: "operator detection contracts from Plan 17-01"
provides:
  - "read-only Core/Knots-style datadir, config, cookie, service, and wallet detection"
  - "hermetic macOS and Linux candidate detection tests"
  - "explicit product uncertainty for ambiguous Core versus Knots evidence"
affects: [CLI-07, MIG-02, operator, onboarding, status, migration]

tech-stack:
  added: []
  patterns:
    - "filesystem detection inspects only injected roots"
    - "candidate service paths are represented as evidence, not lifecycle actions"
    - "wallet candidates are path metadata only; wallet databases are never opened for write"

key-files:
  created: []
  modified:
    - packages/open-bitcoin-cli/src/operator/detect.rs
    - packages/open-bitcoin-cli/src/operator/detect/tests.rs

key-decisions:
  - "Default product classification remains Unknown with ProductAmbiguous uncertainty unless path evidence carries a product-specific signal."
  - "Service evidence is attached from candidate paths only; service lifecycle remains Phase 18."
  - "Detection reports wallet and cookie paths without reading wallet database or cookie contents into output."

patterns-established:
  - "Read-only operator discovery accepts DetectionRoots so tests and callers choose exactly which roots are inspected."
  - "Detection tests preserve candidate file bytes and modified timestamps across detection runs."

requirements-completed: [CLI-07, MIG-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 17-2026-04-26T23-56-00
generated_at: 2026-04-27T01:14:59Z

duration: 8min
completed: 2026-04-27
---

# Phase 17 Plan 03 Summary

**Read-only Core/Knots installation detection for datadirs, configs, cookies, services, and wallets**

## Performance

- **Duration:** 8 min
- **Started:** 2026-04-27T01:06:00Z
- **Completed:** 2026-04-27T01:14:59Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments

- Implemented `detect_existing_installations` over injected roots for Linux and macOS candidate layouts.
- Added service, wallet, config, cookie, and datadir evidence collection with explicit uncertainty.
- Added hermetic tests proving candidate file contents and modification times are unchanged after detection.

## Task Commits

1. **Plan 17-03: Add read-only installation detection** - `3db9672` (feat)

## Decisions Made

- Kept ambiguous `.bitcoin` and `Library/Application Support/Bitcoin` evidence classified as `ProductFamily::Unknown` plus `ProductAmbiguous`.
- Returned service files as evidence only; no service-manager commands or privileged operations are present.
- Collected chain-scoped wallet candidates for `regtest`, `signet`, and `testnet3` under injected datadirs.

## Deviations from Plan

None - plan scope was executed as written.

## Issues Encountered

None.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::detect::` passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed.
- `rg -n "fs::write|remove_file|remove_dir|rename|copy|OpenOptions::new\\(\\).*write|launchctl|systemctl" packages/open-bitcoin-cli/src/operator/detect.rs` returned no matches.
- Rust pre-commit gate passed before commit: fmt, clippy, build, and `cargo test --manifest-path packages/Cargo.toml --all-features`.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Status and onboarding can now surface read-only Core/Knots evidence without mutating existing installations or making migration decisions.

## Self-Check: PASSED

- Linux and macOS candidate path tests pass under temp roots.
- Detection tests do not branch on host service-manager availability.
- Candidate cookie and wallet data are not rendered as secret values.

---
*Phase: 17-cli-status-and-first-run-onboarding*
*Completed: 2026-04-27*
