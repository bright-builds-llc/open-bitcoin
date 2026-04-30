---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 28-2026-04-29T13-37-12
generated_at: 2026-04-29T13:37:12.095Z
---

# Phase 28: Service Log-Path Truth and Operator Docs Alignment - Context

## Phase Boundary

**Goal:** Preserve configured service log-path truth across launchd or systemd
preview, apply, status, and operator docs without introducing a new service
surface.

**Success criteria:**
1. Launchd and systemd service definitions preserve or truthfully surface the
   operator-selected log-path behavior in generated artifacts and dry-run
   previews.
2. `open-bitcoin service status` returns the effective service log path or an
   explicit platform-backed unavailable reason through `ServiceStateSnapshot`.
3. Operator docs and dashboard-shared service actions stay aligned with the
   repaired service log-path behavior.

**Out of scope:**
- Broader build provenance cleanup owned by Phase 29.
- New service commands such as restart, start, or system-scope installs.
- Changing the shared `open-bitcoin status` `service.*` contract beyond what is
  needed to keep the service command truthful.

## Requirements In Scope

- `SVC-03`: Service commands surface scope, generated files, command previews,
  and log-path truth before mutation.
- `SVC-04`: Service status truthfully reports relevant service diagnostics,
  including the effective service log path.
- `VER-07`: Operator docs describe the shipped service lifecycle behavior
  accurately.

## Canonical References

- `.planning/ROADMAP.md` — Phase 28 goal, success criteria, and reopened
  requirement IDs.
- `.planning/REQUIREMENTS.md` — current ledger state for `SVC-03`, `SVC-04`,
  and `VER-07`.
- `.planning/v1.1-MILESTONE-AUDIT-PHASE-27.md` — blocker `INT-v1.1-01` and
  flow break `FLOW-v1.1-01`.
- `.planning/PROJECT.md` — operator-surface tone, functional-core constraints,
  and v1.1 service lifecycle scope.
- `.planning/phases/18-service-lifecycle-integration/18-CONTEXT.md` — original
  service lifecycle decisions and status-surface intent.
- `.planning/phases/18-service-lifecycle-integration/18-RESEARCH.md` — prior
  service-definition examples, including launchd `StandardOutPath` and systemd
  `StandardOutput=append:...` patterns.
- `.planning/phases/23-service-apply-completion-and-status-truthfulness/23-CONTEXT.md`
  — existing shared service-path and status-truth decisions.
- `packages/open-bitcoin-cli/src/operator/runtime.rs` — current service and
  dashboard runtime wiring from resolved operator config.
- `packages/open-bitcoin-cli/src/operator/service.rs` — shared service snapshot
  contract, renderers, and command execution path.
- `packages/open-bitcoin-cli/src/operator/service/launchd.rs` — launchd plist
  generation and status inspection.
- `packages/open-bitcoin-cli/src/operator/service/systemd.rs` — systemd unit
  generation and status inspection.
- `packages/open-bitcoin-cli/src/operator/service/tests.rs` — hermetic service
  generator and command-path coverage.
- `packages/open-bitcoin-cli/src/operator/dashboard/mod.rs` and
  `packages/open-bitcoin-cli/src/operator/dashboard/action.rs` — shared
  dashboard service runtime path.
- `docs/operator/runtime-guide.md` — shipped operator-facing service lifecycle
  documentation.
- `docs/architecture/status-snapshot.md` — unavailable-value semantics for
  operator-facing status reporting.

## Repo Guidance That Materially Informs This Phase

- Use `bash scripts/verify.sh` as the repo-native verification contract.
- Keep pure parsing and path-derivation helpers separate from effectful adapter
  code.
- Prefer focused hermetic service tests over real service-manager mutation.
- Keep operator docs truthful even when behavior is conservative or local-only.

## Current State

- `OperatorConfigResolution` resolves a structured log directory, but the
  service runtime currently passes that directory straight through as a
  `maybe_log_path` input instead of deriving a concrete service log file.
- `LaunchdAdapter::generate_plist_content()` can embed a log path, but
  `status()` never recovers it from the installed plist, so
  `open-bitcoin service status` drops the line entirely.
- `SystemdAdapter::generate_unit_content()` still hardcodes
  `StandardOutput=journal` and `StandardError=journal`, so dry-run and apply do
  not preserve the chosen log path on Linux.
- Dashboard service actions already reuse the shared `execute_service_command()`
  path, so keeping the runtime log-path input truthful there should align the
  dashboard automatically.

## Decisions

1. **Derive one deterministic service log file from the selected log
   directory.**
   Treat the operator-selected log directory as the parent and derive a single
   combined service log file at `open-bitcoin.log` inside it. Both stdout and
   stderr point at that file so the current shared snapshot can surface one
   stable path.
2. **Launchd and systemd previews must describe the same effective path the
   apply path writes.**
   The generated plist or unit content is the source of truth for preview,
   apply, and later status recovery.
3. **Systemd should preserve the chosen file path instead of defaulting to
   journald when a managed log path is available.**
   Use file-backed directives in generated units so Linux no longer advertises a
   chosen log path while writing elsewhere.
4. **Service status must carry either the recovered path or an explicit
   unavailable reason.**
   Silent omission is the bug. The shared service snapshot should preserve the
   absence explicitly when an installed definition does not expose a file-backed
   path.
5. **Recover log-path truth from the installed service definition, not from the
   current operator config alone.**
   Status should read the installed plist or unit so it remains honest even if
   operator config later drifts.
6. **Dashboard service actions should stay on the shared runtime path.**
   Fix the shared service runtime input rather than adding dashboard-only log
   handling.
7. **Docs should describe the shipped service log file behavior in terms of the
   selected log directory.**
   The runtime guide should explain that service previews and status surface the
   concrete service-managed log file derived from the configured operator log
   directory.

## Key Files and Likely Change Surfaces

- `packages/open-bitcoin-cli/src/operator/runtime.rs`
- `packages/open-bitcoin-cli/src/operator/service.rs`
- `packages/open-bitcoin-cli/src/operator/service/fake.rs`
- `packages/open-bitcoin-cli/src/operator/service/launchd.rs`
- `packages/open-bitcoin-cli/src/operator/service/systemd.rs`
- `packages/open-bitcoin-cli/src/operator/service/tests.rs`
- `packages/open-bitcoin-cli/src/operator/dashboard/mod.rs`
- `docs/operator/runtime-guide.md`

## Risks

- The selected operator log path is currently modeled as a directory, so the fix
  must avoid treating a directory path as a writeable file target.
- Systemd log directives vary by environment; the generated unit should use a
  conservative file-backed form that can also be parsed back from the installed
  unit.
- Existing service-status tests only cover present paths, so the phase must add
  explicit unavailable-reason coverage to avoid regressing back to silent
  omission.

## Implementation Notes

- Prefer tiny pure helpers for service-log path derivation and installed-file
  parsing so both preview and status paths share the same rules.
- Keep `open-bitcoin status` unchanged unless a shared helper refactor makes the
  service runtime input clearer.
- Verification should start with focused `open-bitcoin-cli` service tests before
  the repo-native gate.
