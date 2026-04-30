---
phase: 28-service-log-path-truth-and-operator-docs-alignment
generated_by: gsd-research-phase
lifecycle_mode: yolo
phase_lifecycle_id: 28-2026-04-29T13-37-12
generated_at: 2026-04-29T13:37:12.095Z
---

# Phase 28 Research: Service Log-Path Truth and Operator Docs Alignment

## Summary

The blocker is narrower than a full service-lifecycle redesign. The shared
service runtime already carries a log-related input, but it currently passes the
resolved log directory straight through as if it were a file path, systemd
ignores it entirely by hardcoding journald, and both adapters discard log-path
truth during `status()` inspection.

The lowest-risk repair is:

1. derive one concrete service log file from the selected operator log
   directory,
2. embed that file path in launchd and systemd generated artifacts,
3. recover the effective path from the installed plist or unit during
   `status()`, and
4. update docs to describe the shipped behavior precisely.

## Existing Surfaces To Reuse

### Shared service runtime path

- `packages/open-bitcoin-cli/src/operator/runtime.rs` already resolves operator
  config, service arguments, and dashboard service runtime dependencies.
- `packages/open-bitcoin-cli/src/operator/service.rs` already owns the shared
  `ServiceInstallRequest`, `ServiceStateSnapshot`, service rendering, and
  `execute_service_command()` path.
- `packages/open-bitcoin-cli/src/operator/dashboard/action.rs` already routes
  service actions through the same `execute_service_command()` call used by the
  CLI surface.

### Existing service generators and status adapters

- `launchd.rs` already supports file-backed log keys in generated plist content.
- `systemd.rs` already has a pure unit generator and existing enable or status
  parsers, so adding file-backed log directives stays local to that adapter.
- Both adapters already know the installed service file location, which makes
  installed-definition parsing feasible without a new discovery surface.

### Existing test seams

- `packages/open-bitcoin-cli/src/operator/service/tests.rs` already covers pure
  generator output, fake-manager dispatch, and dry-run command previews.
- The benchmark fixture in
  `packages/open-bitcoin-bench/src/cases/operator_runtime.rs` already models a
  concrete service log file at `<log_dir>/open-bitcoin.log`, which aligns with a
  single-file contract.

## Audit Findings That Drive The Fix

- `.planning/v1.1-MILESTONE-AUDIT-PHASE-27.md` identifies one integration
  blocker and one broken flow, both rooted in service log-path truth loss across
  preview, apply, and status.
- The audit evidence is concrete:
  - `systemd.rs` hardcodes `StandardOutput=journal` and `StandardError=journal`
  - `launchd.rs` and `systemd.rs` both return `maybe_log_path: None` from
    `status()`
  - `service.rs` renders the log-path line only when a value is present
  - `docs/operator/runtime-guide.md` still claims service config is derived from
    the selected log path

## Prior-Phase Findings Worth Reusing

### Phase 18 already researched the intended systemd shape

Phase 18 research recorded a Linux unit example using:

- `StandardOutput=append:/path/to/open-bitcoin.log`
- `StandardError=append:/path/to/open-bitcoin-error.log`

The current implementation drifted from that research and instead hardcoded
journald. Phase 28 can close the gap without reopening the rest of Phase 18.

### Phase 23 already established the "manager truth over inference" pattern

Phase 23 introduced `maybe_enabled` because the lifecycle enum alone could not
preserve service truth. The same principle applies here: the service snapshot
should preserve explicit log-path absence instead of letting the renderer infer
or omit it.

## Code Findings

### `runtime.rs`

- The service and dashboard paths both pass `config_resolution.maybe_log_dir`
  directly into service runtime execution.
- `maybe_log_dir` is a directory path, not a service log file path.

### `launchd.rs`

- `generate_plist_content()` can embed `StandardOutPath` and `StandardErrorPath`
  when a path is provided.
- `status()` never reads the installed plist for those keys, so it always drops
  the path later.

### `systemd.rs`

- `generate_unit_content()` currently accepts only binary, datadir, and config,
  then emits `StandardOutput=journal` and `StandardError=journal`.
- `status()` never inspects the installed unit for file-backed log directives.

### `service.rs`

- `render_service_state_snapshot()` only prints the log line when
  `maybe_log_path` is present.
- There is no structured way to preserve an explicit unavailable reason for the
  service log path.

## Recommended Implementation Shape

1. Add one shared helper that maps `maybe_log_dir` to a concrete service log
   file path, using `<log_dir>/open-bitcoin.log`.
2. Update the runtime and dashboard service wiring to pass that derived path
   into the shared service execution path.
3. Extend launchd and systemd generators to embed the concrete path
   consistently.
4. Add pure parser helpers that recover the effective path from installed plist
   or unit contents.
5. Extend `ServiceStateSnapshot` so `status()` can preserve either the recovered
   path or an explicit unavailable reason.
6. Update service-status rendering so the log line is always present.
7. Refresh the runtime guide to describe the derived file path behavior.

## Test Strategy

### High-value service tests

- `generate_unit_content(..., Some(path))` renders file-backed systemd log
  directives instead of journald.
- launchd status parsing recovers `StandardOutPath` from an installed plist.
- systemd status parsing recovers `StandardOutput=append:...` from an installed
  unit.
- service-status rendering prints a `logs:` line both when the path is available
  and when an explicit unavailable reason is carried.
- runtime or dashboard helper coverage proves that the shared service path now
  receives a concrete log file path instead of a raw directory.

### Final verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service::tests -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli -- --nocapture`
- `bash scripts/verify.sh`

## Risks

1. Systemd file-backed directives vary by deployment environment; the parser
   should be permissive enough to read the shipped unit format back reliably.
2. Existing launchd installs created before the fix may not have file-backed log
   keys, so status must preserve an explicit reason instead of treating absence
   as impossible.
3. The operator log directory is also used for structured runtime logs, so the
   docs must explain the service-managed file path clearly instead of implying
   that the directory itself is the service sink.
