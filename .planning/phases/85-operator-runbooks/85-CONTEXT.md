---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 85-2026-06-22T11-57-13
generated_at: 2026-06-22T11:58:54.130Z
---

# Phase 85: Operator Runbooks - Context

**Gathered:** 2026-06-22
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 85 provides operator runbooks for long-running source-built Open Bitcoin
node operation inside the v1.8 production-readiness boundary. Operators should
be able to complete a preflight before a long run, monitor and diagnose
no-progress or degraded operation, recover or stop safely using existing v1.3
through v1.7 evidence surfaces, collect a redacted support-bundle timeline, and
know when escalation evidence is sufficient for support triage.

This phase should not claim production full-node readiness, promote real
service-manager operation to supported, make public-network or multi-day checks
part of default verification, add automatic support-bundle upload, authorize
destructive repair, broaden inbound serving or relay scope, support
production-funds wallet workflows, implement migration apply mode, or replace
the Phase 86 service-expectation work.
</domain>

<decisions>
## Implementation Decisions

### Preflight Runbook

- **D-01:** Create one canonical operator runbook surface that starts with a
  production-boundary preflight. The preflight must send readers through the
  Phase 82 production claim boundary, Phase 83 support matrix, and Phase 84
  upgrade/rollback policy before any long-running source-built operation.
- **D-02:** The preflight should be checklist-oriented but evidence-based. It
  must require the selected datadir, explicit source revision, repo-local
  verification status, Cargo or Bazel command form, config paths, current
  status evidence, resource/disk review, service state or unavailable reason,
  wallet scope, and support-bundle availability.
- **D-03:** Every operator command example must include repo-local Cargo and
  Bazel forms. Do not rely only on an installed `open-bitcoin` alias.
- **D-04:** The preflight must distinguish review-only evidence from mutation.
  Status, support bundles, config summaries, service state, and local report
  paths are acceptable evidence; source datadir, wallet, service, and config
  mutation remains outside this phase.

### Long-Run Monitoring And No-Progress Diagnosis

- **D-05:** Long-run guidance should be organized around observable evidence:
  sync status, progress credit, no-progress reasons, recovery evidence,
  resource-bound pressure, structured logs, metrics, support-bundle summaries,
  and soak or live-smoke reports where the operator explicitly opted in.
- **D-06:** No-progress diagnosis must avoid treating elapsed time, daemon
  startup, peer reachability, raw log tails, or report existence as sufficient
  proof. The runbook should require field-level evidence and unavailable-field
  reasons.
- **D-07:** Reuse existing v1.3 through v1.7 vocabulary for progress and
  recovery outcomes: `safe_retry`, `read_only_inspection`,
  `backup_then_rebuild`, `stop_and_escalate`, resource pressure, no-progress
  diagnosis, stalled subsystem, latest stop reason, and checkpoint timeline.
- **D-08:** Keep public-network, stay-current, and multi-day soak commands
  explicitly opt-in. The runbook may tell operators how to collect those
  reports, but it must state that default `bash scripts/verify.sh` remains
  deterministic, public-network-free, service-manager-free, and multi-day-free.

### Recovery And Escalation Guidance

- **D-09:** Recovery guidance should be decision-table based. `safe_retry`
  permits a bounded retry after preserving evidence; `read_only_inspection`
  means inspect without mutation; `backup_then_rebuild` means preserve backup
  and evidence before any future operator-decided rebuild workflow; and
  `stop_and_escalate` means stop normal attempts and attach redacted evidence.
- **D-10:** Destructive repair, source datadir mutation, external wallet
  mutation, service-manager mutation, config rewrite, and automatic rebuild are
  not part of Phase 85. The runbook can point to future gates but must not
  imply those actions are currently supported.
- **D-11:** Escalation thresholds should be practical and evidence-driven:
  repeated no-progress with typed cause, unavailable critical fields, recovery
  class requiring stop/escalate, resource pressure crossing documented bounds,
  inconsistent status/support evidence, or failure to collect the minimum
  redacted support-bundle timeline.
- **D-12:** Escalation language should tell operators what to stop, what to
  preserve, what to redact, and what exact commands produced the evidence. It
  should not promise response timelines, hosted support upload, or production
  service ownership.

### Support-Bundle Timeline

- **D-13:** The support-bundle runbook should define a redacted timeline shape:
  preflight evidence, command start, status snapshots, progress or
  no-progress events, resource/recovery events, support-bundle collection,
  operator action taken, final status, and escalation decision.
- **D-14:** The minimum useful bundle should mirror the Phase 83 issue-evidence
  checklist: redacted support JSON/Markdown when available, exact command
  output, bounded log summary, config summary, service state or unavailable
  reason, resource evidence, recovery/progress evidence, sync evidence,
  version/toolchain context, platform details, and exact repo-local
  reproduction command.
- **D-15:** Privacy and safety boundaries must be explicit. Do not ask for
  wallet private material, raw wallet files, RPC cookies, rpcpassword,
  rpcauth, raw datadirs, unredacted logs, raw unbounded logs, full sensitive
  peer tables, or automatic upload.

### Documentation And Verification Shape

- **D-16:** Prefer a canonical docs/parity runbook document, linked from the
  runtime guide, production boundary, support matrix, upgrade policy,
  release-readiness page, deviations register, parity README/checklist/index,
  README, and operator runtime catalog. Avoid duplicating the full runbook in
  README.
- **D-17:** If automation is added, keep it narrow to Phase 85: required
  runbook sections, exact support terms, required repo-local command forms,
  support-bundle evidence fields, forbidden mutation/upload claims, canonical
  links, and verifier wiring.
- **D-18:** Any Phase 85 checker must be Bun-backed like the Phase 82 through
  Phase 84 checkers, deterministic, public-network-free, real-service-manager-
  free, multi-day-free, and wired into `bash scripts/verify.sh`. It must not
  become the broad Phase 88 production-claim scanner.
- **D-19:** Final closeout should run `bash scripts/verify.sh`. Focused Bun
  checker/test commands may be used during iteration, but the phase
  verification evidence should cite the repo-native verifier.

### Folded Todos

No pending todos matched Phase 85.

### the agent's Discretion

- The planner may split work into canonical runbook content, parity/root link
  updates, deterministic checker and fixture tests, verifier wiring, and
  closeout evidence.
- The executor may decide whether the canonical runbook file is named
  `operator-runbooks.md` or a narrower filename, as long as there is one
  durable source of truth and entrypoints link to it.
- No Rust source changes are expected. If planning discovers a narrow Rust gap,
  update parity breadcrumbs for any new first-party Rust source or test files
  under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`.
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 85 goal, dependencies, success criteria, and
  v1.8 phase sequencing.
- `.planning/REQUIREMENTS.md` - RUN-01 through RUN-03, SVC-01/SVC-02
  boundaries for Phase 86, future requirements, and v1.8 out-of-scope table.
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

### Locked v1.8 Boundary, Support, And Upgrade Decisions

- `.planning/phases/82-production-claim-boundary/82-CONTEXT.md` - locked
  production vocabulary, evidence-gate model, deferred-surface inventory, and
  documentation/verification posture.
- `.planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md` -
  locked support matrix, issue-evidence expectations, residual-risk posture,
  and contributor update boundaries.
- `.planning/phases/84-upgrade-and-rollback-policy/84-CONTEXT.md` - locked
  upgrade preflight, state/schema compatibility, rollback, failed-upgrade, and
  hidden-mutation boundaries.
- `docs/parity/production-claim-boundary.md` - exact support terms,
  claim-to-evidence matrix, and deferred production-adjacent surfaces.
- `docs/parity/support-matrix.md` - current support classification and
  issue-evidence policy that Phase 85 must not broaden.
- `docs/parity/upgrade-and-rollback-policy.md` - source-built pre-upgrade,
  rollback, failed-upgrade, compatibility, and no-hidden-mutation policy.
- `docs/parity/release-readiness.md` - v1.8 handoff and historical v1.3
  through v1.7 release-boundary evidence.
- `docs/parity/deviations-and-unknowns.md` - deferred-surface register,
  destructive repair boundary, migration apply boundary, wallet non-claims, and
  production readiness non-claim.

### Existing Operator Evidence Surfaces

- `docs/operator/runtime-guide.md` - source-built operator workflows,
  public-network opt-in commands, support bundles, recovery diagnosis, service
  state, UAT posture, and repo-local Cargo and Bazel command forms.
- `docs/architecture/status-snapshot.md` - status snapshot fields, progress
  evidence, recovery states, compatibility categories, and unavailable-field
  policy.
- `docs/architecture/operator-observability.md` - status, metrics, logs,
  dashboard, support evidence interpretation, and operator evidence semantics.
- `docs/architecture/storage-decision.md` - durable storage, schema versioning,
  restart behavior, and recovery action classes.
- `docs/architecture/cli-command-architecture.md` - operator CLI boundaries,
  migration and wallet command boundaries, and backup expectations.
- `docs/parity/catalog/operator-runtime-release-hardening.md` - resource,
  recovery, support-bundle, service, UAT, and release-hardening evidence
  catalog.
- `docs/parity/catalog/chainstate.md` - chainstate release boundaries,
  sync-to-tip context, and production-node non-claims.
- `docs/parity/catalog/p2p.md` - outbound sync evidence and deferred inbound or
  relay surfaces.
- `docs/parity/catalog/wallet.md` - wallet support boundaries and
  production-funds non-claims.
- `docs/parity/catalog/drop-in-audit-and-migration.md` - dry-run migration
  boundary, rollback expectations, and source datadir/wallet mutation limits.
- `docs/parity/index.json` - machine-readable parity root.
- `docs/parity/checklist.md` - human-readable parity checklist root.
- `docs/parity/README.md` - parity entrypoint.
- `README.md` - contributor/operator entrypoint requiring a Phase 85 pointer
  without duplicating the full runbook.
- `scripts/verify.sh` - repo-native verification contract.

### Existing Checker And Report Patterns

- `scripts/check-phase82-production-claim-boundary.ts` and
  `scripts/check-phase82-production-claim-boundary.test.ts` - v1.8 checker and
  fixture-test pattern.
- `scripts/check-phase83-support-matrix-issue-evidence.ts` and
  `scripts/check-phase83-support-matrix-issue-evidence.test.ts` - support
  matrix and issue-evidence checker pattern.
- `scripts/check-phase84-upgrade-rollback-policy.ts` and
  `scripts/check-phase84-upgrade-rollback-policy.test.ts` - upgrade-policy
  checker pattern.
- `scripts/check-phase79-diagnostics-support-bundle.ts` and
  `scripts/check-phase79-diagnostics-support-bundle.test.ts` - support-bundle
  forensics checker pattern.
- `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts` and
  `scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts` -
  opt-in UAT and release-boundary checker pattern.
</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `docs/parity/production-claim-boundary.md` already defines the support terms,
  allowed claim shape, deferred surfaces, and future gates that the runbooks
  must preserve.
- `docs/parity/support-matrix.md` already defines issue-evidence expectations,
  support-bundle redaction boundaries, and residual-risk handling.
- `docs/parity/upgrade-and-rollback-policy.md` already defines pre-upgrade,
  rollback, failed-upgrade, compatibility, and no-hidden-mutation guidance.
- `docs/operator/runtime-guide.md` already has source-built commands, sync
  activation, status, support-bundle, service, opt-in public-network, and UAT
  material that Phase 85 can reorganize into runbooks.
- `docs/architecture/status-snapshot.md` and
  `docs/architecture/operator-observability.md` define the evidence fields that
  runbooks should reference rather than inventing new proof terms.
- Phase 82 through Phase 84 Bun checkers show the local pattern for narrow,
  deterministic documentation guardrails wired into `scripts/verify.sh`.

### Established Patterns

- v1.8 docs preserve exact support terms and avoid alternate production
  maturity language.
- Public-network, real service-manager, and multi-day evidence stays opt-in
  UAT and outside default verification.
- Operator-facing examples use repo-local Cargo and Bazel command forms.
- Docs use one canonical source-of-truth file with entrypoint links, while
  README and runtime guide keep concise pointers.
- Bun scripts under `scripts/` own repo automation; no new Python automation
  should be added.

### Integration Points

- Link the runbook from `README.md`, `docs/operator/runtime-guide.md`,
  `docs/parity/production-claim-boundary.md`,
  `docs/parity/support-matrix.md`,
  `docs/parity/upgrade-and-rollback-policy.md`,
  `docs/parity/release-readiness.md`,
  `docs/parity/deviations-and-unknowns.md`, `docs/parity/README.md`,
  `docs/parity/checklist.md`, `docs/parity/index.json`, and
  `docs/parity/catalog/operator-runtime-release-hardening.md` when planning
  confirms the exact link scope.
- If a checker is added, wire it through `scripts/verify.sh` and include a
  focused Bun test fixture.
</code_context>

<specifics>
## Specific Ideas

- Keep the runbooks operator-facing and concise enough to follow during an
  incident: preflight, monitor, diagnose, recover/stop, bundle, escalate.
- Prefer tables for evidence requirements and decision thresholds when they
  reduce ambiguity.
- Use exact string evidence for automated checks: required support terms,
  required command prefixes, forbidden mutation/upload claims, and canonical
  link paths.
</specifics>

<deferred>
## Deferred Ideas

None - discussion stayed within phase scope. Phase 86 owns service operation
expectations, Phase 87 owns the release-readiness checklist, and Phase 88 owns
the broad production-claim guardrail suite.
</deferred>

---

*Phase: 85-operator-runbooks*
*Context gathered: 2026-06-22*
