---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 89-2026-06-24T20-03-26
generated_at: 2026-06-24T20:03:57.087Z
---

# Phase 89: Release Readiness Guardrail Closure - Context

**Gathered:** 2026-06-24
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 89 closes the v1.8 milestone audit gaps left after Phase 88. Release
reviewers must be able to audit REL-02, REL-03, and REL-04 from the canonical
v1.8 release-readiness checklist, and the deterministic Phase 88 claim
guardrail must scan every canonical v1.8 policy document that could otherwise
promote deferred production-adjacent surfaces.

This phase is gap closure only. It must not broaden the production full-node
readiness claim, add public-network or real service-manager checks to default
verification, promote deferred surfaces, or introduce new runtime capability.

</domain>

<decisions>
## Implementation Decisions

### Release-Readiness Checklist Closure

- **D-01:** Add REL-02, REL-03, and REL-04 rows to the canonical
  `docs/parity/release-readiness.md` v1.8 checklist instead of leaving Phase 88
  ownership in prose below the table.
- **D-02:** Each new row must include Phase 88 evidence, focused checker/test
  commands, default verification posture, UAT/manual posture, residual risk,
  and no-claim or next-gate status.
- **D-03:** Preserve the existing release-readiness table shape and keep the
  Phase 87 checklist as the release reviewer source of truth. Do not create a
  second checklist or separate release evidence registry.
- **D-04:** Update checker expectations so the missing REL-02, REL-03, and
  REL-04 checklist rows cannot recur.

### Deterministic Claim-Guardrail Corpus

- **D-05:** Expand the Phase 88 deterministic claim-guardrail corpus to include
  the missing canonical v1.8 policy docs:
  `docs/parity/upgrade-and-rollback-policy.md`,
  `docs/parity/operator-runbooks.md`, and
  `docs/parity/service-operation-expectations.md`.
- **D-06:** Treat those docs as first-class release-review evidence roots. A
  production-readiness or deferred-surface promotion in any of them must fail
  deterministically unless the surrounding wording is explicitly scoped,
  deferred, unsupported, opt-in UAT, historical, or a future gate.
- **D-07:** Keep the corpus curated rather than scanning all historical
  `.planning/` or milestone archive files. Phase 89 should close the audit gap
  without turning scoped historical evidence into default-verifier false
  positives.

### Fixture And Verification Coverage

- **D-08:** Add fixture coverage proving deferred-surface promotion in the newly
  covered canonical policy docs fails the Phase 88 checker.
- **D-09:** Keep valid no-claim, deferred, unsupported, opt-in UAT, and
  outside-default-verification wording passing in the expanded corpus.
- **D-10:** Run focused Phase 87 and Phase 88 checker/test commands during
  iteration, refresh generated LOC metrics if changed, and close with the
  repo-native `bash scripts/verify.sh` gate.

### Planning Metadata Hygiene

- **D-11:** Record whether stale planning metadata was refreshed during this gap
  closure. If it remains stale, route it explicitly to milestone closeout so the
  Phase 89 verification artifact does not leave the audit concern ambiguous.
- **D-12:** Do not over-expand Phase 89 into full milestone archival. The active
  closure target is GAP-01, GAP-02, release-readiness reviewer flow, and
  deterministic claim-guardrail flow. Broader archive wording belongs to the
  milestone closeout workflow unless required by the checker or verification
  evidence.

### Folded Todos

No pending todos matched Phase 89.

### Claude's Discretion

- The planner may split the work into release-readiness checklist/checker
  updates, Phase 88 corpus and fixture updates, focused verification, and
  closeout evidence.
- The executor may keep Phase 89 documentation and Bun automation only; no Rust
  source changes are expected.
- If no first-party Rust source or test files change under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, no parity
  source breadcrumb update is required.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope And Audit Gaps

- `.planning/ROADMAP.md` - Phase 89 goal, requirements, gap closure scope, and
  success criteria.
- `.planning/REQUIREMENTS.md` - REL-01 through REL-04 routing to Phase 89 and
  v1.8 out-of-scope production-readiness boundaries.
- `.planning/PROJECT.md` - active v1.8 production-readiness boundary and
  no-claim posture.
- `.planning/STATE.md` - stale Phase 88 complete state and closeout hygiene
  context.
- `.planning/v1.8-MILESTONE-AUDIT.md` - GAP-01, GAP-02, affected flows,
  required closure, and stale planning metadata note.

### Locked Phase 87 And Phase 88 Evidence

- `.planning/phases/87-release-readiness-checklist/87-CONTEXT.md` - checklist
  shape, release reviewer flow, Phase 88 handoff, and checker expectations.
- `.planning/phases/87-release-readiness-checklist/87-VERIFICATION.md` - passed
  Phase 87 evidence and residual Phase 88 ownership note.
- `.planning/phases/88-deterministic-claim-guardrails/88-CONTEXT.md` - claim
  scan boundary, deferred-surface promotion rules, evidence gate semantics, and
  verifier integration decisions.
- `.planning/phases/88-deterministic-claim-guardrails/88-VERIFICATION.md` -
  passed Phase 88 evidence, focused commands, and residual production-readiness
  risk.

### Canonical v1.8 Policy Docs

- `docs/parity/release-readiness.md` - canonical v1.8 checklist that must gain
  REL-02, REL-03, and REL-04 rows.
- `docs/parity/production-claim-boundary.md` - support-term vocabulary, allowed
  production-related statement, deferred-surface inventory, and future gates.
- `docs/parity/support-matrix.md` - support classifications and issue-evidence
  boundaries.
- `docs/parity/upgrade-and-rollback-policy.md` - missing Phase 88 corpus target
  for source-built upgrade, rollback, backup, schema, and mutation boundaries.
- `docs/parity/operator-runbooks.md` - missing Phase 88 corpus target for
  preflight, long-run, diagnosis, recovery, support-bundle, and escalation
  guidance.
- `docs/parity/service-operation-expectations.md` - missing Phase 88 corpus
  target for source-built daemon, service preview, opt-in lifecycle UAT, and
  service non-claims.
- `docs/parity/deviations-and-unknowns.md` - deferred production-adjacent
  surfaces and residual-risk register.
- `README.md`, `docs/operator/runtime-guide.md`, `docs/parity/README.md`,
  `docs/parity/checklist.md`, `docs/parity/index.json`, and
  `docs/parity/catalog/operator-runtime-release-hardening.md` - release and
  operator entrypoints already covered by Phase 88 and relevant to evidence
  consistency.

### Existing Checker Patterns

- `scripts/check-phase87-release-readiness.ts` and
  `scripts/check-phase87-release-readiness.test.ts` - release-readiness checker,
  required requirement rows, fixture style, and verifier-wiring pattern.
- `scripts/check-phase88-deterministic-claim-guardrails.ts` and
  `scripts/check-phase88-deterministic-claim-guardrails.test.ts` - deterministic
  claim-guardrail checker, target corpus, fixture style, scoped allow rules, and
  verifier drift checks.
- `scripts/verify.sh` - repo-native verification contract and checker execution
  order.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- The Phase 87 checker already validates release-readiness rows and should be
  extended to require REL-02, REL-03, and REL-04 in the canonical checklist.
- The Phase 88 checker already centralizes target corpus paths and sentence or
  paragraph scanning for production-readiness and deferred-surface promotions.
- The Phase 88 fixture harness uses temporary repositories and a repo-root
  override to prove failing docs and valid scoped docs.
- `scripts/verify.sh` already runs Phase 87 and Phase 88 focused tests and
  checkers in order.

### Established Patterns

- v1.8 closure work uses Bun/TypeScript automation and deterministic local
  checks.
- Release-boundary docs must stay quiet, evidence-first, and explicit about
  no-production-readiness claims.
- Public-network, real service-manager, package-manager service, destructive
  repair, support upload, and multi-day checks remain opt-in or deferred, not
  default verification.
- Generated LOC metrics may change when scripts or tests are added or edited.

### Integration Points

- `docs/parity/release-readiness.md` is the primary documentation edit surface.
- `scripts/check-phase87-release-readiness.ts` and its tests should make the new
  checklist rows required.
- `scripts/check-phase88-deterministic-claim-guardrails.ts` and its tests should
  cover the added canonical policy docs.
- `.planning/v1.8-MILESTONE-AUDIT.md`, `.planning/ROADMAP.md`, `.planning/STATE.md`,
  and `.planning/PROJECT.md` may need narrow status refreshes or explicit
  routing to milestone closeout.

</code_context>

<specifics>
## Specific Ideas

- Prefer adding three checklist rows near the existing REL-01, REL-05, and
  REL-06 release-readiness rows so release reviewers can scan all REL items
  together.
- Use Phase 88 verification evidence in the rows:
  `bun test scripts/check-phase88-deterministic-claim-guardrails.test.ts`,
  `bun run scripts/check-phase88-deterministic-claim-guardrails.ts`, and
  `bash scripts/verify.sh`.
- Add one focused Phase 87 test for the missing REL rows and one focused Phase
  88 test that proves the newly added canonical policy docs are scanned.
- Keep planning metadata refresh minimal unless the closeout checks require a
  larger milestone narrative update.

</specifics>

<deferred>
## Deferred Ideas

- Full v1.8 milestone archival and broad narrative refresh belong to milestone
  closeout after Phase 89 passes unless needed to close the audit gap.
- Future production full-node readiness, inbound serving, relay, production
  wallet safety, migration apply mode, signed packaging, hosted dashboards, GUI
  parity, public-network CI, destructive repair, and automatic support upload
  remain future-scoped.

</deferred>

---

*Phase: 89-release-readiness-guardrail-closure*
*Context gathered: 2026-06-24*
