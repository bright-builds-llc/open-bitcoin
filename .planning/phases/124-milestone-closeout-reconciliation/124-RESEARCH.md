---
phase: 124-milestone-closeout-reconciliation
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 124-2026-07-16T20-19-53
generated_at: "2026-07-16T20:42:00Z"
status: complete
---

# Phase 124: Milestone Closeout Reconciliation - Research

## Research Question

What must the planner know to reconcile v2.1 metadata, rerun the final audit, preserve deterministic no-claim enforcement, and leave a truthful archival handoff under `HARD-05`?

## Executive Summary

Phase 124 is a planning-metadata and deterministic-checker closeout, not a runtime implementation phase. The active corpus is stale behind passed Phase 122 and Phase 123 evidence:

- `HARD-01` is satisfied by Phase 122 verification.
- `HARD-02`, `HARD-03`, and `HARD-04` are satisfied by Phase 123 verification.
- `HARD-05` remains the only legitimate pending requirement and belongs only to Phase 124.
- The canonical v2.1 audit still stops at Phase 121, scores 34/34 requirements and 12/12 phases, lists six non-critical debt items, and cites an interrupted full verifier run.
- ROADMAP and REQUIREMENTS still report five pending hardening/closeout requirements even though four now have passed evidence.
- ROADMAP's primary next step still points back to Phase 122 planning.

The closest repository precedent is Phase 109. It preserved exactly-one requirement ownership, updated planning projections only from current evidence, refreshed the canonical audit in place, recorded old findings as resolved, ran focused checkers and planning validation, required the full verifier, and routed directly to milestone completion.

## Recommended Architecture

### 1. Evidence-First Two-Gate Closeout

Use two ordered gates rather than one optimistic final rewrite:

1. Reconcile Phase 122/123 evidence into active metadata while keeping `HARD-05` and Phase 124 pending. At this point the truthful requirement total is 38/39 complete.
2. Run focused closeout checks. Only after they pass may Phase 124 completion, `HARD-05`, 39/39 totals, the final audit `passed` status, and archive routing become final.

This prevents planning assertions from becoming the evidence for their own truth. REQUIREMENTS remains the exactly-one-ownership source; the audit is derived evidence and must not remap requirements.

### 2. One Canonical Audit, Refreshed In Place

Refresh `.planning/v2.1-MILESTONE-AUDIT.md` in place. A second active audit would create pointer ambiguity, while an addendum would leave stale scores and stale integration links in the canonical report.

The refreshed audit should:

- cover Phases 110 through 124;
- report 39/39 requirements and 15/15 passed phase verifications;
- rerun integration and end-to-end flow evidence after Phases 122 and 123;
- replace the stale ManagedRpcContext metrics link with the authoritative sync-owned snapshot path;
- include a resolved-debt ledger for all six pre-hardening findings;
- distinguish intentional bounded/deferred scope from active hardening debt;
- remain non-passed if any genuine new requirement, integration, flow, verification, or lifecycle gap is found;
- point to `/gsd-complete-milestone v2.1` only after all closeout evidence is clean.

### 3. Deterministic Phase 124 Checker

The full verifier alone is insufficient: it already passed while active planning rollups were stale. Add a Phase 124 TypeScript checker and mutation suite following Phase 98, 117, 122, and 123 patterns.

The checker should validate exact, stable facts rather than broad prose:

- `HARD-05` exists once and maps only to Phase 124;
- all 39 active requirement checklist items and traceability rows are complete;
- mapped, complete, pending, and unmapped coverage totals are `39`, `39`, `0`, and `0` in the final corpus;
- ROADMAP reports Phase 124 complete with its actual plan count;
- the canonical audit frontmatter is `status: passed`, contains 39/39 and 15/15 scores, has no active approved hardening debt, and contains a resolved-debt ledger;
- ROADMAP, STATE, and audit route to `/gsd-complete-milestone v2.1` without a stale planning/execution route;
- the Phase 124 verification artifact is passed and lifecycle-valid;
- no prohibited public/default/production claim is introduced.

Add the visible and executable Phase 124 mutation/live commands to `scripts/verify.sh` after Phase 123 and before the Phase 117 mutation/live pair. Keeping Phase 117 last preserves its role as the final no-claim boundary gate.

## Exact Evidence Sources

### Phase 122

`.planning/phases/122-compact-relay-peer-completion/122-VERIFICATION.md` passes `HARD-01` and proves:

- eligible inbound `getblocktxn` reaches bounded `blocktxn` serving;
- compact-announcement provenance is peer-local and bounded;
- ordered witness-bearing transactions are returned;
- stale Phase 112 peer-noop terminology was corrected;
- parity/no-claim boundaries and full verification passed.

### Phase 123

`.planning/phases/123-runtime-timing-and-evidence-integrity/123-VERIFICATION.md` passes `HARD-02` through `HARD-04` and proves:

- timeout expiration advances on caller-clocked idle wakes without receive activity;
- successful typed block writes, not eligibility proxies, drive served evidence;
- sync-owned network state is sampled once per tick for metrics and logs;
- Phase 121's checker was migrated to authoritative provenance;
- the full verifier passed after review-fix closure.

### Phase 109 Precedent

Phase 109 demonstrates the intended closeout shape:

- no implementation requirements are remapped or duplicated;
- focused extension checkers are named explicitly;
- planning metadata and canonical audit are refreshed from evidence;
- `state validate`, `git diff --check`, and the full verifier are required;
- deferred public/production claims remain unchanged;
- the refreshed audit becomes `passed` only after the checks succeed.

## Planning Surface

The likely changed files are:

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md` through `gsd-tools.cjs`, not direct mutation
- `.planning/PROJECT.md` where the current milestone/closeout narrative is stale
- `.planning/MILESTONES.md` where the active milestone rollup is stale
- `.planning/v2.1-MILESTONE-AUDIT.md`
- `scripts/check-phase124-milestone-closeout-reconciliation.ts`
- `scripts/check-phase124-milestone-closeout-reconciliation.test.ts`
- `scripts/verify.sh`
- `.planning/phases/124-milestone-closeout-reconciliation/124-VERIFICATION.md` at verification time

No Rust source, Cargo manifest, runtime behavior, parity claim expansion, operator surface, or public-network gate should change.

## Plan Decomposition Recommendation

Use two plans in two waves:

1. **Wave 1: evidence reconciliation and deterministic guard**
   - reconcile Phase 122/123 completion into active metadata while `HARD-05` stays pending;
   - add the Phase 124 checker/test pair and verifier ordering;
   - run focused checker, roadmap/state, lifecycle, and diff validation.
2. **Wave 2: final audit and archive handoff**
   - refresh the canonical audit from the full evidence graph;
   - close `HARD-05` and Phase 124 only after Wave 1 evidence passes;
   - finalize 39/39 and 15/15 rollups and exact archive routing;
   - run focused checks and the full repository verifier.

Separating the gates makes dependency and evidence ordering explicit. The second plan must depend on the first.

## Threat Model

This phase changes no runtime attack surface, but integrity threats remain relevant:

| Threat | Severity | Mitigation |
| --- | --- | --- |
| Premature completion claim | High | Two-gate ordering; final markers only after focused checks pass |
| Requirement ownership drift | High | Exact checker count and one-owner assertions for all active requirements |
| Audit history erasure | Medium | Resolved-debt ledger retains each prior finding and closure evidence |
| No-claim boundary weakening | High | Keep Phase 117 last; mutation tests reject positive deferred/public/production claims |
| Split-brain audit authority | Medium | Refresh one canonical audit path in place |
| Checker brittleness after archival | Medium | Assert semantic fields and stable markers; avoid incidental prose coupling; handle active/archive paths explicitly if needed |

No secret, credential, network, storage, authentication, authorization, or destructive data path is introduced.

## Validation Architecture

### Focused pre-final checks

- `bun test scripts/check-phase122-compact-relay-peer-completion.test.ts`
- `bun run scripts/check-phase122-compact-relay-peer-completion.ts`
- `bun test scripts/check-phase123-runtime-timing-evidence-integrity.test.ts`
- `bun run scripts/check-phase123-runtime-timing-evidence-integrity.ts`
- `bun test scripts/check-phase124-milestone-closeout-reconciliation.test.ts`
- `bun run scripts/check-phase124-milestone-closeout-reconciliation.ts`
- `bun test scripts/check-phase117-parity-uat-release-boundary.test.ts`
- `bun run scripts/check-phase117-parity-uat-release-boundary.ts`
- `node /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs roadmap analyze`
- `node /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs state validate --raw`
- `node /Users/peterryszkiewicz/.codex/get-shit-done/bin/gsd-tools.cjs verify lifecycle 124 --require-plans --require-verification --raw`
- `git diff --check`

### Final repository gate

- `bash scripts/verify.sh`

Run ad-hoc Cargo or Bazel commands only through the repository timing wrapper; this phase should not need separate Cargo/Bazel commands because the full verifier owns those surfaces.

## Pitfalls to Avoid

- Do not mark `HARD-05` complete before Phase 124 verification exists and passes.
- Do not use the stale audit as the authority over Phase 122/123 verification.
- Do not create a second canonical active audit.
- Do not delete old findings without a resolved-debt record.
- Do not accept a green full verifier as proof that planning counts are current.
- Do not move Phase 117 before Phase 124 in the final checker order.
- Do not edit STATE directly; use `gsd-tools.cjs` mutations.
- Do not add Rust/runtime changes or broaden any public/default/production claim.

## RESEARCH COMPLETE

Phase 124 should use a two-plan, evidence-first closeout: reconcile and guard first, then finalize the canonical audit and archival handoff only after deterministic checks pass.
