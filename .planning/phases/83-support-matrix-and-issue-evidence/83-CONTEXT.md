---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 83-2026-06-21T16-02-39
generated_at: 2026-06-21T16:02:39.628Z
---

# Phase 83: Support Matrix and Issue Evidence - Context

**Gathered:** 2026-06-21
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 83 turns the Phase 82 production claim boundary into a practical v1.8
support matrix and issue-evidence policy. Operators, contributors, and release
reviewers should be able to classify source-built install, runtime, network,
storage, and service-supervision environments by support level; know which
redacted evidence is expected for issue reports; see residual risks carried
forward from v1.1 through v1.7; and update the matrix without broadening
production, wallet, relay, migration, packaging, hosted-dashboard, GUI, support
upload, destructive repair, public-network CI, or production-readiness claims.

This phase should not claim production full-node readiness, add support for a
deferred surface, make public-network or real service-manager checks part of
default verification, implement upgrade/rollback policy, write operator
runbooks, or build the Phase 88 broad claim-guardrail scanner.
</domain>

<decisions>
## Implementation Decisions

### Support Matrix Scope

- **D-01:** Use the exact Phase 82 support terms in every matrix row:
  `supported`, `preview`, `opt-in UAT`, `unsupported`, and `deferred`. Do not
  introduce alternate maturity labels such as beta, production-grade,
  production-ish, partial production, or community-supported.
- **D-02:** Classify support by environment family rather than by marketing
  claim: source-built install, local runtime, public-network operation,
  storage/datadir safety, service supervision, wallet, migration, packaging,
  dashboard, GUI, support upload, destructive repair, and verification/CI.
- **D-03:** Keep the supported slice narrow and evidence-backed. Source-built,
  repo-local workflows that are covered by default verification and documented
  deterministic evidence may be `supported`; public-network, long-run, or real
  service-manager evidence remains `opt-in UAT`; shipped but not
  support-committed operator conveniences may be `preview`; unsafe or
  experiment-only paths are `unsupported`; future production-adjacent scope
  remains `deferred`.
- **D-04:** Every row should name the evidence basis, default verification
  status, opt-in UAT status, residual risk, and the gate required before the
  support term can change. Artifact existence alone is not an evidence basis.

### Issue Evidence Expectations

- **D-05:** Issue-report guidance should request the smallest useful redacted
  evidence set: support bundle JSON/Markdown when available, relevant command
  output, redacted logs or log summaries, configuration summary, service state,
  resource-bound evidence, recovery/progress evidence, sync status evidence,
  version/commit/toolchain information, platform details, and the exact
  repo-local command that reproduced the issue.
- **D-06:** Evidence collection must stay privacy-preserving and local-first.
  Do not ask operators to attach wallets, private keys, RPC credentials,
  cookies, raw datadirs, unredacted logs, full peer tables with sensitive local
  data, or automatic uploads. Automatic support-bundle upload stays deferred.
- **D-07:** Include copy-pasteable repo-local Cargo and Bazel forms when the
  docs ask operators to reproduce or collect evidence. Do not rely only on an
  installed `open-bitcoin` alias.
- **D-08:** Make insufficiency explicit. A support bundle, daemon startup,
  elapsed time, peer reachability, raw log tail, or local report file is not
  enough by itself unless the required fields and evidence basis are present.

### Residual Risk And Manual Validation

- **D-09:** Carry forward residual risks and manual validation surfaces from
  v1.1 through v1.7 in a release-reviewable table. Include at least dashboard
  pseudoterminal/raw-input manual validation, historical v1.2 closeout without
  a dedicated milestone audit artifact, v1.3 diagnosed-blocker closeout,
  v1.4 planning traceability correction, and the recurring public-network,
  service-manager, multi-day, support-bundle, recovery, and production-scope
  non-claims.
- **D-10:** Tie each residual risk to the latest evidence source and current
  handling status: accepted manual validation surface, historical closeout
  context, opt-in UAT evidence, deferred future gate, or deterministic checker
  coverage.
- **D-11:** Keep historical milestone claims discoverable but scoped. v1.3
  through v1.7 evidence supports source-built, explicit opt-in review paths; it
  does not promote any Phase 82 deferred surface into current production
  support.

### Contributor Update Boundaries

- **D-12:** Add clear edit rules for the support matrix: new or promoted
  support rows require a concrete evidence source, verifier or opt-in UAT
  command, residual-risk statement, and next gate; deferred surfaces cannot be
  promoted by prose-only edits.
- **D-13:** If Phase 83 adds automation, keep it narrow: validate the support
  matrix, issue-evidence checklist, residual-risk table, canonical links, and
  exact support terms. Do not implement the future broad all-doc production
  claim scanner owned by Phase 88.
- **D-14:** Wire any Phase 83 checker into `bash scripts/verify.sh` only if it
  is deterministic, short-running, public-network-free, real-service-manager-
  free, and multi-day-free. Follow the Phase 82 Bun checker and fixture-test
  pattern if automation is added.
- **D-15:** Matrix updates should preserve Phase 82 links to
  `docs/parity/production-claim-boundary.md`, `docs/parity/release-readiness.md`,
  `docs/parity/deviations-and-unknowns.md`, parity roots, README, runtime guide,
  and `scripts/verify.sh`.

### Folded Todos

No pending todos matched Phase 83.

### the agent's Discretion

- The planner may decide whether the support matrix belongs in a new
  `docs/parity/support-matrix.md` document or as a tightly scoped extension of
  an existing parity/release-readiness page, as long as there is one canonical
  reader path and no duplicated source of truth.
- The planner may split work into matrix content, issue-evidence policy,
  residual-risk register, contributor update guidance, entrypoint links, and
  optional deterministic checker coverage.
- The executor may add a small machine-readable parity root or checklist row if
  it materially improves support-matrix discoverability without creating a
  second evidence registry.
- No Rust source changes are expected. If planning discovers a narrow Rust gap,
  update parity breadcrumbs for any new first-party Rust source or test files
  under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`.
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 83 goal, dependency on Phase 82, success
  criteria, and v1.8 phase sequencing.
- `.planning/REQUIREMENTS.md` - SUP-01 through SUP-04, future production
  capability requirements, and v1.8 out-of-scope table.
- `.planning/PROJECT.md` - active v1.8 boundary-setting goal, current project
  posture, core value, and production-claim constraints.
- `.planning/STATE.md` - current milestone state and accumulated v1.8
  decisions.
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

### Locked Phase 82 Boundary

- `.planning/phases/82-production-claim-boundary/82-CONTEXT.md` - locked
  production vocabulary, evidence-gate model, deferred-surface inventory, and
  documentation/verification posture.
- `.planning/phases/82-production-claim-boundary/82-01-SUMMARY.md` - canonical
  production boundary document and release-readiness handoff.
- `.planning/phases/82-production-claim-boundary/82-02-SUMMARY.md` - parity
  roots, checklist row, and deferred-surface register.
- `.planning/phases/82-production-claim-boundary/82-03-SUMMARY.md` - README,
  runtime-guide, and parity catalog boundary pointers.
- `.planning/phases/82-production-claim-boundary/82-04-SUMMARY.md` - narrow
  Phase 82 checker, verifier wiring, and full verification evidence.
- `docs/parity/production-claim-boundary.md` - exact support terms,
  claim-to-evidence matrix, and deferred production-adjacent surfaces.
- `docs/parity/release-readiness.md` - current v1.8 handoff plus historical
  v1.3 through v1.7 release-boundary evidence.
- `docs/parity/deviations-and-unknowns.md` - durable deferred-surface register
  and historical residual-risk context.
- `scripts/check-phase82-production-claim-boundary.ts` and
  `scripts/check-phase82-production-claim-boundary.test.ts` - nearest v1.8
  deterministic checker and fixture-test pattern.

### Support Evidence And Historical Risk Sources

- `docs/operator/runtime-guide.md` - source-built operator workflows,
  support-bundle commands, redaction boundaries, service-supervision notes, and
  opt-in UAT commands.
- `docs/architecture/status-snapshot.md` - `OpenBitcoinStatusSnapshot` and
  shared status evidence fields used in support reports.
- `docs/architecture/operator-observability.md` - operator status, metrics,
  logs, dashboard, and support evidence interpretation.
- `docs/parity/catalog/operator-runtime-release-hardening.md` - accumulated
  operator runtime, service, support, UAT, and release-hardening evidence.
- `docs/parity/catalog/p2p.md` - outbound support evidence and deferred inbound
  or relay surfaces.
- `docs/parity/catalog/chainstate.md` - chainstate release boundaries and
  production-node non-claims.
- `docs/parity/catalog/wallet.md` - wallet support boundaries and production
  funds non-claims.
- `docs/parity/catalog/drop-in-audit-and-migration.md` - dry-run migration
  boundaries and migration apply-mode non-claims.
- `.planning/milestones/v1.7-MILESTONE-AUDIT.md` - passed v1.7 audit and
  residual-risk posture.
- `.planning/MILESTONES.md` and `.planning/RETROSPECTIVE.md` - historical
  milestone residual-risk summaries and support-evidence lessons.
- `scripts/check-phase65-support-review.ts` - support review checker pattern.
- `scripts/check-phase72-observability-evidence.ts` - observability evidence
  checker pattern.
- `scripts/check-phase79-diagnostics-support-bundle.ts` - support-bundle
  forensics checker pattern.
- `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts` - opt-in UAT
  and release-boundary checker pattern.
- `scripts/verify.sh` - repo-native verification contract.
</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `docs/parity/production-claim-boundary.md` already provides the exact
  support vocabulary and the rule that evidence is field- and gate-based.
- `docs/parity/release-readiness.md` already carries historical v1.3 through
  v1.7 matrices and the current v1.8 boundary handoff.
- `docs/parity/deviations-and-unknowns.md` already lists deferred
  production-adjacent surfaces and is the natural residual-risk register.
- `docs/operator/runtime-guide.md` already documents local support-bundle
  collection, redaction boundaries, service flows, and opt-in UAT command
  forms.
- Phase 65, 72, 79, 80, and 82 checker scripts provide deterministic Bun
  patterns for docs and support-evidence assertions.

### Established Patterns

- Release-boundary and support phases extend current milestone sections while
  preserving historical evidence instead of rewriting old milestones into
  current support.
- Default verification remains deterministic, local, public-network-free,
  real-service-manager-free, timing-stable, and short-running.
- Operator-facing UAT commands use explicit repo-local Cargo and Bazel forms.
- Support evidence is redacted, local, and field-specific. Raw logs, raw
  datadirs, wallet material, RPC credentials, and automatic upload are not
  support defaults.
- Deferred surfaces stay deferred until a future milestone names gates and
  evidence; prose-only support promotion is not allowed.

### Integration Points

- Add or update one canonical support-matrix reader path under `docs/parity/`.
- Link the matrix from README, runtime guide, parity README/checklist/index,
  release readiness, deviations register, and relevant catalog pages.
- If a Phase 83 checker is added, wire its test and runtime command near the
  Phase 82 checker in `scripts/verify.sh`.
- Refresh `docs/metrics/lines-of-code.md` if verification or hooks regenerate
  it.
</code_context>

<specifics>
## Specific Ideas

- Prefer a compact matrix with columns such as `Environment`, `Support term`,
  `Evidence basis`, `Default verification`, `Opt-in UAT`, `Residual risk`, and
  `Next gate`.
- Keep issue-evidence guidance practical and copy-pasteable: "include these
  fields or explain why unavailable" is better than asking for broad raw
  artifacts.
- Use `Unavailable: <reason>` style wording where evidence is expected but not
  available, matching existing operator evidence patterns.
- Treat Phase 83 as a support-policy and evidence-discovery phase, not as a new
  production-readiness or release-guardrail milestone.
</specifics>

<deferred>
## Deferred Ideas

- Phase 84 upgrade, rollback, backup, and state/schema compatibility policy.
- Phase 85 long-run operator runbooks and escalation workflows.
- Phase 86 detailed service operation expectations and command matrices.
- Phase 87 release-readiness checklist tying every v1.8 requirement to final
  evidence.
- Phase 88 broad deterministic production-claim guardrails.
- Any promotion of inbound serving, relay, production-funds wallet use,
  migration apply mode, signed packaging, hosted dashboards, GUI parity,
  public-network CI, destructive repair, automatic support upload, or broad
  production-node readiness.
</deferred>

---

*Phase: 83-support-matrix-and-issue-evidence*
*Context gathered: 2026-06-21*
