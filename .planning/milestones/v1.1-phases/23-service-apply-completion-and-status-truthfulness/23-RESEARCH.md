---
phase: 23-service-apply-completion-and-status-truthfulness
generated_by: gsd-research-phase
lifecycle_mode: yolo
phase_lifecycle_id: 23-2026-04-28T17-18-36
generated_at: 2026-04-28T17:18:36.921Z
---

# Phase 23 Research: Service Apply Completion and Status Truthfulness

## Summary

Phase 23 does not need a new operator surface. The fastest honest path is to
repair the existing service adapters so `install --apply` completes the real
manager registration step, then preserve manager-reported enablement in the
shared service snapshot so CLI status and dashboard projections stop inferring
truth from `ServiceLifecycleState` alone.

The dashboard action path already reuses `execute_service_command()` and already
requires confirmation for service-affecting actions. That means the dashboard
gap should close automatically if the shared install path and status projection
become truthful.

## Existing Surfaces To Reuse

### Shared service runtime path

- `packages/open-bitcoin-cli/src/operator/service.rs` already owns the shared
  request, outcome, error, and render path for `install`, `uninstall`,
  `enable`, `disable`, and `status`.
- `packages/open-bitcoin-cli/src/operator/dashboard/action.rs` already routes
  service actions through `execute_service_command()`.
- `packages/open-bitcoin-cli/src/operator/status.rs` already consumes
  `ServiceManager::status()` and projects it into the shared `ServiceStatus`
  model used by both CLI status and dashboard rendering.

### Existing service adapters

- `packages/open-bitcoin-cli/src/operator/service/launchd.rs` already knows how
  to generate the user-scope plist, derive the launch agent path, and call
  `launchctl` for uninstall, enable, disable, and list inspection.
- `packages/open-bitcoin-cli/src/operator/service/systemd.rs` already knows how
  to generate the user-scope unit file and call `systemctl --user` for
  uninstall, enable, disable, and active-state inspection.
- Both adapters already surface preview command strings in dry-run mode, so the
  missing behavior is specifically apply-mode execution of those previews.

### Existing test seams

- `packages/open-bitcoin-cli/src/operator/service/tests.rs` already has tempdir
  isolation, fake-manager coverage, and generator tests.
- `packages/open-bitcoin-cli/src/operator/status/tests.rs` already verifies how
  `ServiceStateSnapshot` becomes `ServiceStatus` for operator status and
  dashboard surfaces.
- These existing seams are enough to add parser and projection coverage without
  touching real launchd or systemd state.

## Audit Findings That Drive The Fix

- `.planning/v1.1-MILESTONE-AUDIT.md` blocker `INT-01` says both service
  adapters stop at file write on `install --apply`.
- The same audit notes that service truth is degraded because enabled-state is
  inferred from lifecycle enum variants instead of manager evidence.
- The dashboard install action is blocked only because it reuses the defective
  service path; there is no separate dashboard bug to solve first.

## Code Findings

### `launchd.rs`

- `install()` writes the plist and returns a preview `launchctl bootstrap`
  string, but never actually runs `launchctl bootstrap` on apply.
- `status()` relies on `launchctl list` only, which does not answer whether the
  service is disabled via `launchctl print-disabled`.
- The adapter can be improved with a pure parser for `print-disabled` output and
  a small amount of additional command execution in apply mode.

### `systemd.rs`

- `install()` writes the unit file and previews `daemon-reload` plus `enable`,
  but apply mode never executes either command.
- `status()` only calls `systemctl --user is-active`, so enabled-state is
  guessed from the resulting enum rather than measured.
- The adapter can query `systemctl --user is-enabled` without adding a new
  service surface or a new dependency.

### `status.rs`

- `collect_service_status()` currently treats `Enabled`, `Running`, and
  `Stopped` as the only enabled states.
- This breaks truthfulness whenever the manager reports a state combination that
  the enum cannot encode, especially failed-plus-enabled and running-plus-
  disabled cases.
- Adding explicit `maybe_enabled` evidence to `ServiceStateSnapshot` is the
  smallest correction because it keeps existing consumers intact and avoids
  inventing more enum variants that still would not cover every combination.

## Recommended Implementation Shape

1. Add `maybe_enabled: Option<bool>` to `ServiceStateSnapshot`.
2. Update launchd and systemd adapters to populate `maybe_enabled` from real
   manager inspection when possible.
3. Change `collect_service_status()` to prefer `snapshot.maybe_enabled` over the
   old enum inference.
4. Update the CLI service-status renderer to show explicit installed, enabled,
   and running booleans alongside the lifecycle label.
5. Add pure parser tests plus fake-manager status-projection tests so the fix
   stays hermetic.

## Test Strategy

### High-value service tests

- launchd dry-run preview shows both `enable` and `bootstrap`.
- systemd dry-run preview shows both `daemon-reload` and `enable`.
- `parse_launchd_disabled_services()` classifies disabled and enabled entries.
- `parse_systemd_enabled_state()` classifies common `is-enabled` outputs.
- `execute_service_command(...status...)` prints explicit enabled and running
  flags.

### High-value status tests

- `collect_status_snapshot()` preserves `enabled=true` when the manager reports
  `Failed` plus explicit enablement.
- `collect_status_snapshot()` preserves `enabled=false` even when the manager
  reports `Running`.

### Verification

- Focused loop:
  `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service::tests -- --nocapture`
- Focused loop:
  `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli status::tests -- --nocapture`
- Final package gate:
  `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli -- --nocapture`
- Final repo gate when practical:
  `bash scripts/verify.sh`

## Risks

1. `launchctl print-disabled` formatting differs across systems, so the parser
   must be permissive and degrade safely when it cannot determine enablement.
2. `systemctl is-enabled` can return non-binary states like `masked` or
   `indirect`; Phase 23 should classify the common states conservatively.
3. Apply-mode manager failures after file write may still leave artifacts on
   disk; the phase should improve descriptions and diagnostics rather than
   pretending the operation is fully atomic.
