---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 86-2026-06-22T19-33-52
generated_at: 2026-06-22T19:33:52.813Z
---

# Phase 86: Service Operation Expectations - Context

**Gathered:** 2026-06-22
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 86 defines source-built daemon and service-supervision expectations for
Open Bitcoin. It should help operators distinguish direct `open-bitcoind`
operation, user-level launchd/systemd supervision, service preview/install
boundaries, packaged-service limits, real service-manager opt-in UAT, and
unsupported production-service claims. It should also give operators exact
repo-local Cargo and Bazel command forms for service lifecycle, restart/resume,
logs, metrics, resource bounds, recovery evidence, status, sync status, and
support-bundle collection.

This phase does not add a new service manager, signed package, Windows service,
production service ownership, public-network default verification, destructive
repair, migration apply mode, automatic support upload, or broad production
full-node readiness claim.

</domain>

<decisions>

## Implementation Decisions

### Service Support Boundary

- **D-01:** Create one canonical Phase 86 service expectation document under
  `docs/parity/`, preferably `service-operation-expectations.md`, instead of
  spreading service policy across README, runtime guide, support matrix, and
  runbooks.
- **D-02:** Preserve the exact Phase 82 support terms:
  `supported`, `preview`, `opt-in UAT`, `unsupported`, and `deferred`.
  Service expectation docs must not introduce alternate terms such as
  production-ready service, production-grade service, managed service,
  package-supported, or community-supported.
- **D-03:** Classify the existing service surfaces narrowly:
  source-built daemon command forms and local status/support evidence are
  `supported`; launchd/systemd generated definition preview is `preview`; real
  user-level launchd/systemd install/start/stop/restart/status evidence is
  `opt-in UAT`; packaged service distribution, Windows service integration,
  automatic updates, production service ownership, uptime guarantees, and broad
  production full-node readiness remain `deferred`.
- **D-04:** Explicitly state that generated launchd/systemd definitions
  supervise `open-bitcoind`, not the `open-bitcoin` operator wrapper, and that
  user-level paths stay local to the operator machine. Do not imply system-wide
  service ownership or packaged distribution support.
- **D-05:** Keep service-manager mutation boundaries visible. `service preview`
  is always side-effect-free. `service install` and `service uninstall` are
  previews unless `--apply` is supplied. Starting, stopping, restarting,
  enabling, disabling, or uninstalling a real service is opt-in local UAT and
  must not be part of default verification.

### Operator Evidence And Command Forms

- **D-06:** Every operator-facing command in this phase must show repo-local
  Cargo and Bazel forms. Do not rely only on an installed `open-bitcoin` alias.
- **D-07:** Service expectation evidence should be field-based, not artifact
  existence based. Service file existence, daemon startup, elapsed time, raw log
  tail, public peer reachability, or a support bundle path is context only
  unless the expected fields and unavailable reasons are present.
- **D-08:** Required evidence areas are service lifecycle, service
  restart/resume, status JSON, sync status JSON, structured logs, bounded
  metrics, resource bounds or resource pressure, recovery category/action,
  support-bundle evidence, service log path, service manager command strings,
  generated service file path, and unavailable reasons.
- **D-09:** Carry forward the existing lifecycle labels exactly:
  `unmanaged`, `installed-stopped`, `running`, `failed`, `disabled`, and
  `unavailable-manager`. Docs and checkers should require these labels where
  service status is explained.
- **D-10:** Restart/resume evidence should be interpreted from
  `service.restart_resume` and related status fields: `same_datadir`,
  `prior_shutdown`, `durable_progress`, `stale_inflight`, `recovery_category`,
  and `next_action`. Do not treat restart command success or elapsed time as
  resume proof.
- **D-11:** Include both direct daemon and service-supervised review flows:
  direct `open-bitcoind` startup/status evidence remains source-built daemon
  operation; service commands review the local user supervisor around that
  daemon path.

### Documentation And Traceability Shape

- **D-12:** Link the canonical service expectations from the production claim
  boundary, support matrix, operator runbooks, upgrade/rollback policy,
  release-readiness page, deviations register, parity README, parity checklist,
  parity index, README, runtime guide, and operator-runtime catalog without
  duplicating the full service expectation table in each file.
- **D-13:** Update `docs/parity/production-claim-boundary.md` so its production
  service operation row points to the canonical Phase 86 expectation document
  while preserving the `deferred` production-service claim boundary.
- **D-14:** Update `docs/parity/support-matrix.md` so launchd/systemd preview
  and real lifecycle rows point to the canonical service expectation document
  without changing their existing support terms.
- **D-15:** Update `docs/parity/operator-runbooks.md` and
  `docs/operator/runtime-guide.md` only with compact pointers or clarifying
  handoffs. Keep procedural command duplication controlled by making the new
  Phase 86 document the service expectation source of truth.
- **D-16:** Register a new parity surface such as
  `v1-8-service-operation-expectations` in `docs/parity/index.json`,
  `docs/parity/checklist.md`, and `docs/parity/README.md`, with requirements
  `SVC-01` and `SVC-02` and evidence that includes the canonical document,
  runtime guide, support matrix, operator runbooks, operator-runtime catalog,
  checker, and `scripts/verify.sh`.

### Verification And Guardrails

- **D-17:** Add a narrow Phase 86 Bun checker and fixture tests only for the
  Phase 86 document set: service expectation sections, support terms, command
  forms, service lifecycle labels, restart/resume fields, required links,
  parity roots, and verifier wiring.
- **D-18:** Wire the Phase 86 checker and fixture tests into
  `bash scripts/verify.sh` immediately after Phase 85, both in the visible
  command-order block and in the executed `run_step` sequence.
- **D-19:** The Phase 86 checker must reject default verifier drift that adds
  public-network live smoke, real `systemctl`, real `launchctl`, long
  wall-clock sleeps, `--restart-after-progress`, package-manager service
  commands, Windows service claims, production service ownership, automatic
  support upload, or broad production-node readiness.
- **D-20:** Final closeout should run focused checker commands, refresh
  `docs/metrics/lines-of-code.md` if checker files are added, and then run the
  full repo-native `bash scripts/verify.sh` gate.

### Folded Todos

No pending todos matched Phase 86.

### the agent's Discretion

- The planner may split Phase 86 into canonical service expectation docs,
  parity/root and entrypoint links, deterministic checker plus fixture tests,
  and closeout/LOC/full-verification work.
- The executor may keep Phase 86 primarily documentation and Bun automation if
  no source behavior gap is found.
- No Rust source changes are expected. If planning discovers a narrow Rust gap,
  update `docs/parity/source-breadcrumbs.json` for any new first-party Rust
  source or test files under `packages/open-bitcoin-*/src` or
  `packages/open-bitcoin-*/tests`.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 86 goal, dependencies, success criteria, and
  v1.8 phase sequencing.
- `.planning/REQUIREMENTS.md` - `SVC-01` and `SVC-02`, future requirements, and
  v1.8 out-of-scope table.
- `.planning/PROJECT.md` - active v1.8 boundary-setting posture, core value,
  production-claim constraints, and current project state.
- `.planning/STATE.md` - current milestone state and accumulated decisions.
- `AGENTS.md` - repo-local verification, UAT command, parity breadcrumb, GSD,
  generated artifact, and workflow rules.
- `AGENTS.bright-builds.md` - Bright Builds workflow and standards routing.
- `standards-overrides.md` - local standards override registry.
- `standards/core/architecture.md` - functional-core and illegal-state rules.
- `standards/core/code-shape.md` - code shape, script, and naming rules.
- `standards/core/verification.md` - sync-first and repo-native verification
  requirements.
- `standards/core/testing.md` - unit-test and Arrange/Act/Assert expectations.
- `standards/languages/rust.md` - Rust module, option naming, invariant, and
  verification guidance.
- `standards/languages/typescript-javascript.md` - Bun/TS automation guidance.

### Locked v1.8 Boundary, Support, Upgrade, And Runbook Decisions

- `.planning/phases/82-production-claim-boundary/82-CONTEXT.md` - locked
  production vocabulary, evidence-gate model, deferred-surface inventory, and
  documentation/verification posture.
- `.planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md` -
  locked support matrix, issue-evidence expectations, residual-risk posture,
  and contributor update boundaries.
- `.planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md` - locked
  source-built upgrade, rollback, backup, state/schema, and hidden-mutation
  boundaries.
- `.planning/phases/85-operator-runbooks/85-CONTEXT.md` - locked preflight,
  long-run monitoring, no-progress, recovery, support-bundle, and escalation
  evidence guidance.
- `docs/parity/production-claim-boundary.md` - support terms, production
  service non-claim, and deferred production-adjacent surfaces.
- `docs/parity/support-matrix.md` - launchd/systemd preview and real
  service-manager lifecycle support terms and issue-evidence policy.
- `docs/parity/upgrade-and-rollback-policy.md` - source-built rollback and no
  hidden service/config/datadir mutation boundary.
- `docs/parity/operator-runbooks.md` - preflight, service state evidence,
  support-bundle timeline, and escalation evidence.
- `docs/parity/release-readiness.md` - v1.8 handoff and historical v1.3
  through v1.7 release-boundary evidence.
- `docs/parity/deviations-and-unknowns.md` - deferred production-service,
  Windows service, packaging, public-network CI, and broad production-node
  readiness boundaries.

### Existing Service, Status, And Verification Surfaces

- `docs/operator/runtime-guide.md` - current service lifecycle docs, repo-local
  service command forms, lifecycle labels, restart/resume evidence, logs,
  metrics, resource bounds, status, and sync status guidance.
- `docs/architecture/status-snapshot.md` - service status, restart/resume,
  recovery, resource, and unavailable-field status contracts.
- `docs/architecture/operator-observability.md` - structured logs, metrics,
  support evidence, status interpretation, and bounded evidence posture.
- `docs/parity/catalog/operator-runtime-release-hardening.md` - operator
  runtime, service, UAT, support, and release-hardening catalog rows.
- `docs/parity/catalog/p2p.md` - restart/resume and public-network non-claim
  boundaries.
- `docs/parity/catalog/chainstate.md` - chainstate release boundaries and
  production-node non-claims.
- `docs/parity/index.json` - machine-readable parity root.
- `docs/parity/checklist.md` - human-readable parity checklist root.
- `docs/parity/README.md` - parity entrypoint.
- `README.md` - contributor/operator entrypoint requiring compact Phase 86
  pointers without duplicating the full expectation table.
- `scripts/check-phase63-service-lifecycle.ts` - existing service lifecycle,
  command, platform manager, dashboard action, and runtime guide checker.
- `scripts/check-phase64-service-restart-resume.ts` - existing service
  restart/resume status field and default-verification boundary checker.
- `scripts/check-phase82-production-claim-boundary.ts` and
  `scripts/check-phase82-production-claim-boundary.test.ts` - v1.8 checker and
  fixture-test pattern.
- `scripts/check-phase83-support-matrix-issue-evidence.ts` and
  `scripts/check-phase83-support-matrix-issue-evidence.test.ts` - support
  matrix checker pattern.
- `scripts/check-phase84-upgrade-rollback-policy.ts` and
  `scripts/check-phase84-upgrade-rollback-policy.test.ts` - policy checker
  pattern.
- `scripts/check-phase85-operator-runbooks.ts` and
  `scripts/check-phase85-operator-runbooks.test.ts` - closest Phase 86 checker
  and fixture-test pattern.
- `scripts/verify.sh` - repo-native verification contract and checker wiring.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `packages/open-bitcoin-cli/src/operator/service/` implements service
  definition generation and service manager adapters for user-level launchd and
  systemd flows.
- `packages/open-bitcoin-cli/src/operator/service/tests.rs` covers service
  preview/apply safety, start/stop/restart parsing, fake service manager
  behavior, service status, lifecycle labels, and error guidance.
- `packages/open-bitcoin-cli/src/operator/dashboard/` routes service actions
  through the same service command path and renders service status rows.
- `scripts/check-phase63-service-lifecycle.ts` and
  `scripts/check-phase64-service-restart-resume.ts` already prove older
  service lifecycle and restart/resume anchors.

### Established Patterns

- v1.8 docs phases use one canonical parity document, compact links from
  entrypoints, a narrow fixed-target Bun checker, fixture tests, verifier
  wiring after the previous phase, generated LOC freshness, and full
  `bash scripts/verify.sh` closeout evidence.
- Phase checkers strip the legacy `VERIFY_COMMAND_ORDER` heredoc before
  validating executed `run_step` wiring in `scripts/verify.sh`.
- Operator-facing docs use repo-local Cargo and Bazel commands and avoid bare
  installed aliases.

### Integration Points

- `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/README.md`,
  and `docs/parity/catalog/operator-runtime-release-hardening.md` are the parity
  registration surfaces for a new Phase 86 service expectation root.
- `README.md`, `docs/operator/runtime-guide.md`,
  `docs/parity/production-claim-boundary.md`, `docs/parity/support-matrix.md`,
  `docs/parity/operator-runbooks.md`, `docs/parity/upgrade-and-rollback-policy.md`,
  `docs/parity/release-readiness.md`, and
  `docs/parity/deviations-and-unknowns.md` should link to the canonical
  service expectation doc.
- `scripts/verify.sh` should run Phase 86 test and checker commands after Phase
  85 checker commands.

</code_context>

<specifics>

## Specific Ideas

- Prefer surface id `v1-8-service-operation-expectations`.
- Prefer audit key `v1_8_service_operation_expectations` and parity path
  `service-operation-expectations.md`.
- Keep the canonical service doc table-driven: service surface, support term,
  what it proves, command evidence, default verification status, opt-in UAT
  status, residual risk, and next gate.
- Include command groups for direct daemon operation, service preview/install,
  service lifecycle, restart/resume review, status/sync status, support bundle,
  log/metrics/resource evidence, and safe shutdown.
- Ensure forbidden claims include production service ownership, packaged
  service guarantee, signed packaging support, Windows service support,
  automatic update behavior, real service-manager default verification,
  public-network default verification, and broad production-node readiness.

</specifics>

<deferred>

## Deferred Ideas

None - discussion stayed within Phase 86 scope.

</deferred>

---

*Phase: 86-service-operation-expectations*
*Context gathered: 2026-06-22*
