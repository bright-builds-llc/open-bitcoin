---
phase: 163-archive-safe-parity-provenance-and-milestone-handoff
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 163-2026-10-12T21-09-41
generated_at: "2026-10-12T21:09:41Z"
status: approved-scope
requirements: [CFGR-01]
scope_path: .planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-GAP-SCOPE.md
---

# Phase 163: Milestone Handoff Language

This context distinguishes the original verified delivery from the audit closure now required before v2.5 archival. The user approved the phase proposal and its prerequisites on 2026-10-12; the approved scope and acceptance obligations are recorded in [163-GAP-SCOPE.md](163-GAP-SCOPE.md).

**Mode:** Yolo. Recommendations below derive from the approved scope; original phase-creation provenance remains in GAP-SCOPE and CREATION-CHECKS.

<domain>
## Phase Boundary

Close CFGR-01/AG01/AF01 through archive-safe milestone provenance, native loader regressions and truthful handoff records. Actual archival/tag/shipment is separate; runtime scope is unchanged.
</domain>

<decisions>
## Implementation Decisions

- **D-01:** Preserve original Phase 162 certificates and six proof-source hashes; distinguish historical delivery from current closure ownership.
- **D-02:** Select fixed milestone-bound active/standard archived v2.5 requirements. Reject conflicting candidates, missing evidence, wrong milestone/baseline/owner and forged/inconsistent completion. An unrelated later active milestone cannot shadow archived v2.5.
- **D-03:** Accept CFGR-01 / Phase 163 / Pending only through linked audit/scope evidence. This transition grants no archive readiness or shipment claim.
- **D-04:** Test actual filesystem/CLI loaders and hermetic supplied maps. Missing supplied-map keys never fall back to disk.
- **D-05:** Preserve source/function, unique ownership, lifecycle, baseline, frozen hashes and excluded-capability checks. Audit/archive language requires narrow evidenced transitions.
- **D-06:** Reconcile current PROJECT/REQUIREMENTS/ROADMAP/UAT and both debug handoffs using actual native/CI receipts, preserving dated failures and proof limits.
- **D-07:** Require focused positive/refusal tests, independent review/security, full default native verification and renewed milestone audit before recommending archival. This invocation authorizes commit/push after clean verification.

### Agent Discretion

Choose the smallest robust shared provenance module and fixture arrangement. Add no dependencies or runtime scope. No pending GSD todos matched.
</decisions>

<canonical-refs>
## Canonical References

- `.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-GAP-SCOPE.md` — approved scope and acceptance states.
- `.planning/v2.5-MILESTONE-AUDIT.md`, `.planning/v2.5-INTEGRATION-AUDIT.md`, `.planning/v2.5-GAP-CLOSURE-PROPOSAL.md` — blocker, frozen audit and approved proposal.
- `.planning/PROJECT.md`, `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md` — current scope/ownership.
- `scripts/check-phase158-validated-reorg.ts`, `scripts/check-phase162-integrated-filter-proof.ts` and their test suites — shared claims/native loaders.
- `docs/parity/v2-5-integrated-filter-proof.md`, `docs/parity/index.json`, `.planning/phases/162-real-prune-retention-and-integrated-parity-proof/162-VERIFICATION.md` — historical proof.
- `scripts/verify.sh`, `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/verification.md`, `standards/core/testing.md`, `standards/languages/typescript-javascript.md` — implementation/verification contracts.
  </canonical-refs>

<code-context>
## Existing Code Insights

- Shared `checkPhase162ClaimState` and `loadPhase162OptionalSources` assume active requirements and uniform Phase 162 completion.
- Phase 162 adds mandatory loading and frozen checkpoint contracts; provenance repair must preserve both.
- Existing mutation suites use source maps; isolated native layouts are required to expose filesystem handoff failures.
  </code-context>

<deferred>
## Deferred Ideas

Actual archive/tag/shipment, new runtime behavior and unrelated v2.4 advisory cleanup remain separate.
</deferred>

## Language

**Historical delivery evidence:** Phase 162's source-bound implementation, native, review, security and formal verification records for the originally scoped requirements. They remain evidence of that delivery when a later audit reopens a requirement for a newly identified lifecycle gap.
_Avoid_: Failed Phase 162, revoked runtime proof.

**Active requirement owner:** The one current roadmap phase accountable for satisfying an outstanding requirement. CFGR-01's owner is Phase 163 and its current status is Pending; the original Phase 162 delivery remains historical provenance.
_Avoid_: Shared owner, completed closure.

**Milestone-bound provenance:** Evidence identified with the milestone whose parity claim is being assessed, independently of whichever milestone currently owns the active requirements document.
_Avoid_: Latest requirements, any matching file.

**Archive handoff:** The lifecycle transition from active v2.5 requirements to their standard archived location, including ordinary verification after a later milestone acquires the active requirements path.
_Avoid_: Archive-scale filter serving, archive node capability.

**Archive readiness:** A conclusion supported by closure of the blocking audit gaps and a renewed passing milestone audit. It is distinct from phase completion, current runtime integration, or actual archival/shipment.
_Avoid_: All plans complete, runtime tests pass, shipped.

**Planning validity:** GSD's ability to discover Phase 163, its approved scope, dependency, lifecycle identity and unique requirement ownership for detailed planning. It does not assert that the unresolved native verifier transition already passes.
_Avoid_: Repository verification passed, implementation complete.
