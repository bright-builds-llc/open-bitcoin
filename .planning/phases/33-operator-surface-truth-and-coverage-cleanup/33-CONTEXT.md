---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 33-2026-04-30T05-02-20
generated_at: 2026-04-30T05:02:20.722Z
---

# Phase 33: Operator Surface Truth and Coverage Cleanup - Context

**Gathered:** 2026-04-30
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Resolve the remaining optional post-audit operator-surface truth and verification
debt without reopening shipped runtime scope. This phase cleans up one dead CLI
surface (`open-bitcoin status --watch`), aligns unmanaged service preview hints
with the real preview contract, adds higher-level coverage around interactive
dashboard service actions, and hardens the nearby fake-RPC operator-binary
coverage so the normal repo-native verification path stops depending on lucky
timing. It does not add new operator commands, new dashboard features, new
service lifecycle actions, or any broader migration redesign.

</domain>

<decisions>
## Implementation Decisions

### Status CLI contract
- **D-01:** Treat `open-bitcoin status --watch` as accidental surface area rather
  than a half-implemented feature. The narrow cleanup should remove or disable
  the dead flag so the public CLI contract matches the one-shot runtime
  behavior, instead of adding a new watch loop during milestone closeout.
- **D-02:** Keep `open-bitcoin status` itself snapshot-based and support-oriented.
  This phase should not change the shared status collection contract, output
  schema, or availability semantics beyond removing the misleading flag.

### Service preview truth
- **D-03:** Preserve the existing preview-by-default service contract:
  `open-bitcoin service install` previews, and `--apply` performs mutation.
  Cleanup should align all unmanaged hints and typed errors to that real
  contract rather than reintroducing a fake `--dry-run` flag.
- **D-04:** The dashboard must keep reusing the shared service command path. Any
  hint or preview-text repair belongs in the shared service surfaces so CLI
  errors, manager-backed status output, and dashboard action messaging stay in
  sync automatically.

### Dashboard coverage and flake hardening
- **D-05:** Add higher-level hermetic coverage around the interactive dashboard
  action loop, not just the lower-level action state machine. Tests should
  exercise confirmation flow and service-command reuse without requiring a real
  terminal session or platform service manager.
- **D-06:** Stabilize the nearby `open_bitcoin_status_json_uses_fake_running_rpc`
  path by hardening the fake RPC fixture or request handling, not by adding
  retries or downgrading the assertion. The verification contract should stay
  deterministic on a clean rerun.

### Scope discipline
- **D-07:** Treat Phase 33 as optional cleanup. It should not reopen
  `REQUIREMENTS.md` rows, widen Phase 34 migration-model scope, or add new
  operator features that deserve their own roadmap phase.
- **D-08:** Update operator-facing docs only if the shipped truthful contract
  changes. If `docs/operator/runtime-guide.md` already matches the repaired
  service preview behavior after the code fix, leave it unchanged.

### Claude's Discretion
- If clap contract cleanup is simpler by removing the `watch` field entirely,
  prefer that over leaving dead parsing code in place.
- Dashboard interaction coverage may live in `dashboard/app.rs` tests or another
  nearby higher-level dashboard test surface, provided it genuinely exercises
  the confirmation loop and shared service execution path.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope and audit evidence
- `.planning/ROADMAP.md` - Phase 33 goal, dependency edge, and success criteria.
- `.planning/STATE.md` - current milestone position after planning the cleanup
  follow-up.
- `.planning/v1.1-MILESTONE-AUDIT-PHASE-32.md` - non-blocking tech-debt findings
  for `status --watch`, unmanaged service hints, dashboard coverage, and the
  flaky operator-binary test.
- `.planning/phases/17-cli-status-and-first-run-onboarding/17-CONTEXT.md` -
  locked status command expectations and one-shot support-oriented status
  surface.
- `.planning/phases/19-ratatui-node-dashboard/19-CONTEXT.md` - dashboard
  interaction, confirmation, and shared-snapshot decisions.
- `.planning/phases/23-service-apply-completion-and-status-truthfulness/23-CONTEXT.md`
  - shared service runtime and dashboard action path decisions.
- `.planning/phases/28-service-log-path-truth-and-operator-docs-alignment/28-CONTEXT.md`
  - recent service-surface truthfulness and docs alignment decisions.

### Current operator surfaces
- `packages/open-bitcoin-cli/src/operator.rs` - clap-owned `status` and service
  argument contract, including the current `watch` flag.
- `packages/open-bitcoin-cli/src/operator/runtime.rs` - one-shot status runtime
  path and dashboard runtime wiring.
- `packages/open-bitcoin-cli/src/operator/service.rs` - shared service errors,
  snapshot rendering, and command execution path.
- `packages/open-bitcoin-cli/src/operator/service/launchd.rs` - unmanaged launchd
  diagnostics wording.
- `packages/open-bitcoin-cli/src/operator/service/systemd.rs` - unmanaged systemd
  diagnostics wording.
- `packages/open-bitcoin-cli/src/operator/dashboard/action.rs` - shared
  confirmation gate and service command reuse.
- `packages/open-bitcoin-cli/src/operator/dashboard/app.rs` - interactive action
  loop that currently lacks higher-level tests.
- `packages/open-bitcoin-cli/src/operator/dashboard/mod.rs` - dashboard runtime
  context and interactive-vs-snapshot routing.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` - current operator-binary
  coverage and the fake-RPC status test that occasionally flakes.
- `packages/open-bitcoin-cli/src/operator/service/tests.rs` - hermetic service
  command and status rendering coverage.
- `packages/open-bitcoin-cli/src/operator/tests.rs` - operator clap/runtime tests
  that currently build `StatusArgs { watch: false }`.
- `docs/operator/runtime-guide.md` - current truthful service preview and status
  workflow documentation.

### Repo workflow and standards
- `AGENTS.md` - repo-native verification contract and planning workflow rules.
- `AGENTS.bright-builds.md` - thin imperative-shell and verification guidance.
- `standards-overrides.md` - local standards exception ledger (currently no
  substantive overrides recorded).
- `https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/main/standards/core/verification.md`
  - repo-native verification expectations.
- `https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/main/standards/core/testing.md`
  - focused hermetic test expectations.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `execute_service_command()` in `packages/open-bitcoin-cli/src/operator/service.rs`
  already centralizes preview-vs-apply semantics for CLI and dashboard consumers.
- `DashboardActionState`, `confirm_and_execute()`, and `handle_action()` already
  provide a reusable confirmation and execution path for interactive service
  actions.
- `FakeServiceManager` and the existing service tests already cover dry-run and
  unmanaged cases without touching a real service manager.
- `FakeRpcServer` in `packages/open-bitcoin-cli/tests/operator_binary.rs` is the
  shared fixture for nearby status and wallet binary tests, so stabilizing it
  improves more than one operator-binary surface.

### Established Patterns
- Optional cleanup phases should stay narrow, verify the shipped surface
  directly, and avoid reopening requirement bookkeeping unless the audit found
  true requirement gaps.
- Operator status and dashboard surfaces are one-shot snapshot collectors that
  report unavailable reasons explicitly rather than inventing background state.
- Service and dashboard behavior should remain hermetic in local tests: fake
  managers and fake RPC fixtures are preferred over real launchd/systemd or
  daemon mutation.

### Integration Points
- The dead `watch` flag lives in the clap contract and the nearby operator tests,
  but the runtime never branches on it.
- The stale `--dry-run` text appears in one typed `ServiceError` plus the
  platform-backed unmanaged diagnostics in the launchd and systemd adapters.
- Higher-level dashboard coverage can sit above `confirm_and_execute()` by
  driving `handle_action()` with a real `DashboardRuntimeContext`.
- The flaky binary status path is isolated to the fake RPC harness and the
  `open_bitcoin_status_json_uses_fake_running_rpc` test.

</code_context>

<specifics>
## Specific Ideas

- Remove the `--watch` flag from clap and update nearby tests instead of
  implementing an undocumented refresh loop late in the milestone.
- Normalize all preview hints to language like "run `open-bitcoin service install`
  to see what would be created" and let `--apply` remain the explicit mutate
  switch.
- Add app-level dashboard tests that cover pending, confirmed, and cancelled
  service actions through `handle_action()`.
- If the fake RPC flake is timing-related, prefer a fixture-side readiness or
  shutdown improvement over test-side sleeps or retries.

</specifics>

<deferred>
## Deferred Ideas

- Implementing a real `status --watch` streaming mode with live repainting or
  polling controls.
- Full interactive dashboard operator-binary TTY automation with a pseudoterminal
  harness.
- Broader migration detection model cleanup beyond the shared ownership-shape
  tightening already scoped to Phase 34.

</deferred>

---

*Phase: 33-operator-surface-truth-and-coverage-cleanup*
*Context gathered: 2026-04-30*
