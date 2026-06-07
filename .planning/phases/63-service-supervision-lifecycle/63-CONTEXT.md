---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 63-2026-06-07T14-20-10
generated_at: 2026-06-07T14:20:10.262Z
---

# Phase 63: Service Supervision Lifecycle - Context

**Gathered:** 2026-06-07
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 63 makes launchd and systemd supervision manageable for the explicit
opt-in `open-bitcoind` unattended mainnet review workflow. It owns service
preview, install, start, stop, restart, and inspect/status behavior; stable
service lifecycle state rendering; launchd and systemd runbook updates; and
deterministic tests proving service manager adapters and operator surfaces do
not imply a broad production-node claim.

This phase does not prove service-supervised same-datadir resume correctness,
restart-after-progress safety, support-bundle expansion, compatibility harness
wrapping, signed packaging, Windows service integration, inbound serving,
transaction relay, production-funds wallet use, migration apply mode, public
network default verification, or broad production-node readiness. Phase 64 owns
service-supervised restart and same-datadir resume evidence.

</domain>

<decisions>

## Implementation Decisions

### Service Command Surface

- **D-01:** Keep `open-bitcoin service install` as a dry-run preview unless
  `--apply` is supplied, and add or document an explicit preview path so the
  operator can run a side-effect-free service preview without guessing the
  `install` dry-run convention. Preview output must show the exact service file
  path, generated content, and manager commands that would run.
- **D-02:** Add start, stop, and restart service actions to the existing
  `ServiceManager` abstraction, fake manager, CLI dispatcher, and dashboard
  service action path. These actions are effectful manager operations and must
  return typed `ServiceCommandOutcome` values with the exact launchd/systemd
  command strings surfaced in human output.
- **D-03:** Preserve existing install, uninstall, enable, disable, and status
  behavior while extending it. Existing dry-run safety for install/uninstall
  must stay intact, and no command may mutate a Bitcoin Core or Bitcoin Knots
  source service or source datadir.
- **D-04:** Service definitions must supervise `open-bitcoind`, not the
  `open-bitcoin` operator CLI wrapper. Resolve the daemon binary through a
  small testable helper that prefers a sibling `open-bitcoind` next to the
  operator binary and falls back to the literal `open-bitcoind` command name
  when a concrete sibling path cannot be proven.

### Lifecycle Status Contract

- **D-05:** Normalize service status into the Phase 63 contract:
  `unmanaged`, `installed-stopped`, `running`, `failed`, `disabled`, and
  `unavailable-manager`. Existing `Installed`, `Enabled`, `Stopped`, and
  manager-error evidence should map into those operator-facing labels rather
  than leaking inconsistent platform vocabulary.
- **D-06:** `open-bitcoin service status`, `open-bitcoin status`, dashboard
  service rows, and JSON status output should agree on service manager,
  installed, enabled, running, log path, service file path, diagnostics, and
  unavailable reasons. Missing manager evidence must stay explicit with
  `Unavailable` reasons instead of false success, empty strings, or zeros.
- **D-07:** Preserve shared sync truth fields from Phase 62 alongside service
  lifecycle state. Service status should complement the existing sync lifecycle,
  progress, stop reason, recovery category, configured targets, and
  downloaded/connected block evidence instead of creating a second independent
  sync interpretation.
- **D-08:** Failed or unavailable manager calls should become typed
  operator-visible states where status inspection can still succeed. Action
  commands may fail when the requested manager operation cannot run, but status
  inspection should distinguish unsupported platform, missing manager command,
  unmanaged service, disabled service, stopped service, running service, and
  failed service.

### Launchd And Systemd Behavior

- **D-09:** launchd support should stay user-level under `~/Library/LaunchAgents`
  and systemd support should stay user-level under `~/.config/systemd/user`.
  Do not introduce sudo, machine-wide unit installation, packaging hooks, or
  global daemon claims in Phase 63.
- **D-10:** Generated launchd plist and systemd unit files must include the
  selected datadir and optional Open Bitcoin JSONC config path, route stdout and
  stderr to the configured operator service log path when one exists, and keep
  explicit generated-by comments.
- **D-11:** Start/stop/restart implementation should use platform-native user
  manager commands: `systemctl --user start|stop|restart
  open-bitcoin-node.service` on Linux and launchd `bootstrap`, `bootout`, or
  `kickstart -k` operations against `gui/<uid>/org.open-bitcoin.node` on macOS
  where those operations are the least surprising fit for the existing user
  plist model.

### Operator Documentation And UAT

- **D-12:** Update the operator runbook to show launchd and systemd command
  flows for preview, install, start, stop, restart, status, disable, uninstall,
  log inspection, config path review, safe shutdown, and recovery next actions.
  Use copy-pasteable repo-local Cargo and Bazel command forms, not only the
  installed `open-bitcoin` alias.
- **D-13:** Keep service workflow language bounded to opt-in extended operator
  review. Generated files, CLI output, docs, and verification notes must not
  call Open Bitcoin a production service, production full node, packaged
  service guarantee, or unattended production-node replacement.
- **D-14:** Public-network service checks are optional UAT only. Default
  verification must remain deterministic through Rust tests, docs/checker
  assertions where useful, and `bash scripts/verify.sh`; it must not start a
  live public-mainnet service or require network access.

### the agent's Discretion

- The planner may split work by service contract, platform adapters, operator
  surfaces, and docs if that keeps each plan reviewable.
- The executor may add a small pure helper for service display-state mapping,
  daemon binary path resolution, or platform command rendering when it reduces
  duplication between CLI, dashboard, and status collectors.
- If new first-party Rust source or test files are added under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, the executor
  must update parity breadcrumbs before committing.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 63 goal, success criteria, dependency on Phase
  62, and deferred Phase 64 through Phase 67 boundaries.
- `.planning/REQUIREMENTS.md` - SVC-01, SVC-02, SVC-04, OBS-04, REL-03, and
  v1.5 out-of-scope boundaries.
- `.planning/PROJECT.md` - v1.5 milestone goal, current state, and production
  claim boundaries.
- `.planning/STATE.md` - Current milestone state and prior decisions affecting
  deterministic verification and service-readiness scope.

### Prior Phase Decisions And Evidence

- `.planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md` - Shared
  sync truth fields, explicit unavailable reasons, status/dashboard/RPC/log
  agreement, and default-verification public-network exclusion.
- `.planning/phases/62-long-run-sync-truth-surfaces/62-VERIFICATION.md` -
  Passed evidence for Phase 62 truth-surface agreement.
- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md` -
  Stable recovery category, resource pressure, and deterministic verification
  decisions that service status must preserve.
- `.planning/phases/60-unattended-sync-loop-control/60-CONTEXT.md` - Explicit
  opt-in daemon loop activation, pause/resume/shutdown behavior, and production
  claim limits.
- `.planning/phases/58-same-datadir-restart-and-resume-evidence/58-CONTEXT.md`
  - Prior restart/resume boundaries that Phase 64 expands under service
  supervision.

### Implementation Surfaces

- `packages/open-bitcoin-cli/src/operator.rs` - Clap service command contract
  and service subcommand parsing.
- `packages/open-bitcoin-cli/src/operator/runtime.rs` - Operator runtime
  dispatch, service manager construction, current service binary path
  resolution, and dashboard service runtime construction.
- `packages/open-bitcoin-cli/src/operator/service.rs` - ServiceManager trait,
  service lifecycle states, command outcome rendering, status rendering, and
  command dispatcher.
- `packages/open-bitcoin-cli/src/operator/service/fake.rs` - Fake service
  manager used by deterministic tests.
- `packages/open-bitcoin-cli/src/operator/service/launchd.rs` - macOS launchd
  plist generation, user service paths, enabled/status parsing, and manager
  command integration.
- `packages/open-bitcoin-cli/src/operator/service/systemd.rs` - Linux systemd
  unit generation, user service paths, enabled/status parsing, and manager
  command integration.
- `packages/open-bitcoin-cli/src/operator/service/tests.rs` - Current service
  dry-run, generated file, parser, status, and dispatcher tests.
- `packages/open-bitcoin-cli/src/operator/status.rs` - Shared operator status
  collection and service status projection into `FieldAvailability`.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - Human and JSON
  status rendering for service and sync truth fields.
- `packages/open-bitcoin-cli/src/operator/dashboard/action.rs` - Dashboard
  service action dispatch through the same service command path.
- `packages/open-bitcoin-cli/src/operator/dashboard/app.rs` - Dashboard action
  bar and service action presentation.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` - Daemon startup,
  explicit opt-in unattended sync preflight, and production-claim boundary text.
- `docs/operator/runtime-guide.md` - Operator service lifecycle, status,
  dashboard, sync, live-smoke, and repo-local UAT command documentation.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `ServiceManager`, `ServiceCommandOutcome`, and `ServiceStateSnapshot` already
  provide the right adapter boundary for adding start, stop, restart, and
  richer status without leaking subprocess calls into higher-level operator
  logic.
- `FakeServiceManager` and `operator/service/tests.rs` already isolate service
  command tests from real launchd/systemd side effects.
- `service_log_path_from_log_dir()` already derives the stable
  `<log_dir>/open-bitcoin.log` service log path used by generated plist/unit
  files and status output.
- `FieldAvailability` in the status collector already models explicit
  unavailable reasons and should be reused for unavailable-manager and missing
  service evidence.

### Established Patterns

- Generated launchd and systemd service files are pure string-generation
  functions tested without filesystem or subprocess access; effectful writes and
  manager commands stay in platform adapters.
- Operator docs and UAT guidance prefer repo-local Cargo and Bazel commands
  instead of relying only on an installed alias.
- Public-network long-run and live-smoke checks remain opt-in UAT and outside
  `bash scripts/verify.sh`.
- Service integration is user-scope only, not sudo or machine-wide service
  installation.

### Integration Points

- Extend `ServiceCommand` parsing in `packages/open-bitcoin-cli/src/operator.rs`
  and dispatch in `packages/open-bitcoin-cli/src/operator/service.rs`.
- Update runtime service binary resolution in
  `packages/open-bitcoin-cli/src/operator/runtime.rs` and dashboard runtime
  construction so generated service definitions target `open-bitcoind`.
- Update launchd/systemd adapters and fake manager in
  `packages/open-bitcoin-cli/src/operator/service/`.
- Update shared service status projection in
  `packages/open-bitcoin-cli/src/operator/status.rs`, rendering in
  `packages/open-bitcoin-cli/src/operator/status/render.rs`, and dashboard
  action/status handling where service actions are exposed.
- Refresh `docs/operator/runtime-guide.md` and any deterministic checker if the
  service lifecycle wording needs a guard against unsupported production-node
  claims.

</code_context>

<specifics>

## Specific Ideas

- Keep `open-bitcoin service install` dry-run behavior, but make preview
  discoverable and grep-visible in CLI help and docs.
- Service status should use the operator-facing labels
  `unmanaged`, `installed-stopped`, `running`, `failed`, `disabled`, and
  `unavailable-manager`.
- Start/stop/restart commands should share the same trait and fake-manager
  dispatch path as install/enable/status so dashboard actions do not fork a
  second implementation.
- UAT examples should include both:
  `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`
  and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`.

</specifics>

<deferred>

## Deferred Ideas

- Service-supervised same-datadir restart/resume proof belongs to Phase 64.
- Redacted v1.5 support bundle expansion belongs to Phase 65.
- Compatibility harness operator wrapper belongs to Phase 66.
- v1.5 threat-model and release-boundary closeout belongs to Phase 67.
- Windows service integration, signed packages, machine-wide install flows, and
  broad production-node support remain out of scope for this milestone.

</deferred>

---

*Phase: 63-service-supervision-lifecycle*
*Context gathered: 2026-06-07*
