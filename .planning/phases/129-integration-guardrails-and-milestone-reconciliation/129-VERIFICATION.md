---
phase: 129-integration-guardrails-and-milestone-reconciliation
verified: 2026-07-21T03:42:06Z
status: passed
score: 12/12 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 129-2026-07-20T19-28-06
generated_at: 2026-07-21T03:42:06Z
lifecycle_validated: true
overrides_applied: 0
requirements_verified:
  - BSRV-03
  - BSRV-04
  - CMP-04
  - CMP-05
  - OBS-01
  - OBS-02
  - OBS-03
  - OBS-04
  - BOUND-02
  - HARD-05
deferred:
  - truth: "ROADMAP, REQUIREMENTS, PROJECT, STATE, MILESTONES, and the rerun milestone audit agree before routing v2.1 to archival"
    addressed_in: "Plan 129-04"
    evidence: "129-04-PLAN.md truths pin the promotion ordering (locked decision D-11): checkbox flips, the in-place audit rerun to passed, and the routing reconciliation land in one atomic commit gated on this verification, accepted by the Plan 03 archive-ready stage"
---

# Phase 129: Integration Guardrails and Milestone Reconciliation Verification Report

**Phase Goal:** Make the repaired production integrations fail closed, independently verify the reassigned requirements, and reconcile the active milestone for a fresh archival decision.
**Verified:** 2026-07-21T03:42:06Z
**Status:** passed
**Re-verification:** No — initial verification

All evidence in this report was gathered against the clean working tree at commit `eea4bce1`, whose pre-commit hook ran the full `bash scripts/verify.sh` contract (including `cargo test --workspace --all-features`) to completion.

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | One aggregate deterministic guard fails when any of the six repaired seams (shared authoritative state, local sendcmpct emission, production announcement invocation, live per-peer header facts, transport emission, post-write-only evidence) regresses. | ✓ VERIFIED | `scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts` composes `checkPhase127AuthoritativeNetworkStateUnification(repoRoot)` (line 82) and `checkPhase128ProductionCompactAnnouncementTransport(repoRoot)` (line 84) under one surface. The live run exits 0; the 12-test mutation suite includes composed-seam rows 9 and 10 proving `P127`- and `P128`-prefixed failures surface through the aggregate checker. |
| 2 | The guard names FLOW-01, FLOW-02, FLOW-03, and FLOW-04 explicitly and fails when any named production-path test anchor disappears. | ✓ VERIFIED | Distinct `P129 FLOW-01`..`P129 FLOW-04` failure strings exist in the checker (lines 37–110). All five Rust anchors are present: `phase127_production_composition_shares_sync_serving_and_operator_authority` (black_box_parity.rs:560), `production_announcement_transport_cases_fanout_uses_live_peer_facts` (:143) and `..._partial_failure_credits_only_prefix_and_redacts` (:217), `compact_success_receipt_records_achieved_effect_once` (:49) and `failed_or_unsent_emission_receives_no_achieved_effect_credit` (:143), plus non-empty `operator_flows.rs`/`operator_binary.rs`. Mutation rows 1–5 remove each anchor and assert the exact failure string. |
| 3 | verify.sh runs the Phase 129 pair after the Phase 128 pair and before the Phase 117 pair, and Phase 117 remains the final check-phase gate. | ✓ VERIFIED | Exactly 4 occurrences of the Phase 129 checker in `scripts/verify.sh` (heredoc lines 415–416 between the 128 and 117 lines; run_step lines 563–564 between the 128 and 117 steps); the ordering-comment sentence was appended at line 310. `P129 final gate` assertions plus the pre-existing `requireFinalPhaseChecker` keep 117 last; `git diff 956f60f2^..HEAD` on both Phase 117 checker files is empty — the final gate is byte-identical. |
| 4 | The fallback facet's `compact_timeout_count` reports only real timeout cleanups, with the semantics explicitly documented. | ✓ VERIFIED | D-06 resolved on the fix path: `fn fallback_counters` and the `counters.compact_timeout_count += timed_out` snapshot-time mixing are gone from `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` (rg: no match); `with_components` now passes `self.fallback` directly (line 88). Doc comment pinning timeouts-only semantics sits on the field (`status/block_relay_evidence.rs:52-53`); the parity record is in `docs/parity/catalog/p2p.md:1327`. Regression tests `snapshot_with_live_getblocktxn_in_flight_reports_zero_fallback_timeouts` (:269) and `timeout_cleanup_increments_fallback_timeout_and_fallback_counts` (:299) exist in `network/tests/compact_timeout_cases.rs` and ran in the full cargo suite at eea4bce1. |
| 5 | All six OBS-01 facets (activation, eligibility, negotiation, reconstruction, fallback, in-flight) continue to project from the single ManagedNetworkHandle authority with no schema change. | ✓ VERIFIED | `BlockRelayEvidenceStatus::with_components(...)` still assembles all facets from the one evidence source (block_relay_evidence.rs:82-91); `CompactRelayFallbackCounters` keeps exactly its two fields (status/block_relay_evidence.rs:49-55). The Phase 127 checker (composed into P129) pins `authoritative_operator_snapshot()`, the `block_relay` projection, and the CLI/dashboard/support anchors; the Phase 116 checker pins the fixed `compact_timeout_count` label. Both exit 0. |
| 6 | Every Phase 116/121/123/127/128 deterministic guard and the new Phase 129 guard stay green after the D-06 outcome. | ✓ VERIFIED | Fresh runs this session: Phase 129 checker exit 0, Phase 116 checker exit 0, Phase 124 dispatcher exit 0, traceability checker exit 0, parity breadcrumbs 389 files exit 0. The full verify.sh chain (which runs every phase guard) passed in the eea4bce1 pre-commit hook. |
| 7 | The Phase 124 post-audit stage machine accepts exactly three Phase 129 states — gaps-open, verified pre-promotion, archive-ready — and rejects every mixture. | ✓ VERIFIED | `scripts/check-phase124-archive-ready.ts` (524 lines) exports `detectPhase129ReconciliationStage`, `verifyVerifiedPrePromotion`, and `verifyArchiveReady`; detection is evidence-claimed and fail-closed (audit passed, any promoted checkbox, or checked roadmap row claims archive-ready and then enforces the full D-13 condition set). The 88-test reconciliation suite (172 assertions, 0 failures) covers all three legal stages, six named mixture rejections, and an 11-row single-field end-state mutation table. |
| 8 | Today's gaps-open assertions are preserved bit-for-bit, so the repo verifies green with no planning-artifact changes. | ✓ VERIFIED | The Phase 124 dispatcher exits 0 against the current gaps-open repo (OBS-01/BOUND-02/HARD-05 unchecked, roadmap `**Plans:** 0 plans` at ROADMAP.md:466, audit `status: gaps_found`, STATE route `/gsd-plan-phase 129` at STATE.md:38); `git status --porcelain` is empty at eea4bce1 — no reconciliation-guarded artifact moved. |
| 9 | HARD-05 ownership stays pinned to Phase 129 in every state, and the legacy Phase 124 final-audit path stays unreachable. | ✓ VERIFIED | The archive-ready assertions pin the literal traceability row `| HARD-05 | Phase 129 | Complete |` (check-phase124-archive-ready.ts:172) alongside OBS-01/BOUND-02 rows; REQUIREMENTS.md currently maps all three to Phase 129. The Phase 125/126 roadmap headings are asserted present in the archive-ready state, keeping `isPhase124GapClosureStage` true and the legacy final-audit branch unreachable (Pitfall 3); a dedicated test guards this. |
| 10 | MILESTONES.md and PROJECT.md are guarded in the archive-ready state so the stale-routing drift that motivated HARD-05 cannot recur. | ✓ VERIFIED | `ARCHIVE_READY_ROUTED_FILES` includes `.planning/STATE.md`, `.planning/MILESTONES.md`, and `.planning/PROJECT.md` (check-phase124-archive-ready.ts:31-35), each required to carry `/gsd-complete-milestone v2.1` with the stale `/gsd-plan-phase 129`/`/gsd-plan-phase 128`/`/gsd-execute-phase 129` routes absent. |
| 11 | Repository verification exercises all four repaired end-to-end flows and preserves the bounded v2.1 no-claim boundary. | ✓ VERIFIED | The four FLOW anchors are Rust tests in the default cargo suite (ran clean at eea4bce1) and their presence is fail-closed via truth 2. The Phase 117 no-claim gate is unchanged and final; the Phase 129 run_step labels contain no forbidden default-gate tokens; the checker self-scans for network/process tokens (rg for `fetch(`/`Bun.spawn`/`node:child_process` finds no literal match). Default verification remains deterministic and public-network-free (D-16). |
| 12 | Independent verification explicitly closes all 10 reassigned requirements against production-path evidence. | ✓ VERIFIED | This report (see Requirements Coverage): the 7 requirements closed via Phases 127/128 are re-attested against their still-present production anchors and passing composed guards, and OBS-01/BOUND-02/HARD-05 are newly closed by the Phase 129 guard, truthfulness fix, and stage machine. |

**Score:** 12/12 truths verified

### Deferred Items

Items not yet met but explicitly addressed by the pending Plan 129-04, which is gated on this verification by locked decision D-11 (independent verification precedes promotion). These are required to be in the gaps-open state right now by the fail-closed stage machine and are NOT verification gaps.

| # | Item | Addressed In | Evidence |
|---|------|-------------|----------|
| 1 | ROADMAP, REQUIREMENTS, PROJECT, STATE, MILESTONES, and the rerun milestone audit agree (39/39, audit passed, archive routing) before routing v2.1 to archival | Plan 129-04 | 129-04-PLAN.md truths: "OBS-01, BOUND-02, and HARD-05 are promoted to checked/Complete only after a lifecycle-valid gsd-verifier 129-VERIFICATION.md with status: passed exists — never before"; the Plan 03 archive-ready stage already enforces the exact end-state byte-for-byte |
| 2 | OBS-01/BOUND-02/HARD-05 checkbox and traceability promotion in REQUIREMENTS.md | Plan 129-04 | Same gate; `check-active-milestone-verification-traceability.ts` requires the activation summary and this verification in the same commit as the flips |

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts` | Aggregate cross-phase seam and flow guard composing the Phase 127/128 exports | ✓ VERIFIED | 232 lines; exports `checkPhase129IntegrationGuardrailsAndMilestoneReconciliation`; composes both upstream checkers; exits 0 live. |
| `scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.test.ts` | Fixture-based mutation coverage for every Phase 129 assertion | ✓ VERIFIED | 12 tests pass (1 complete-corpus + 11 mutation rows incl. composed P127/P128 seams); imports `PHASE127_TARGET_FILES`/`PHASE128_TARGET_FILES` and copies the daemon helper directory. |
| `scripts/verify.sh` | Ordering comment, heredoc, and run_step wiring for the Phase 129 pair | ✓ VERIFIED | 4 occurrences, positioned 128 → 129 → 117 in both surfaces; comment sentence at line 310. |
| `scripts/check-phase127-authoritative-network-state-unification.ts` | Exported target list for fixture composition | ✓ VERIFIED | `export const PHASE127_TARGET_FILES` (line 30) and `export const PHASE127_DAEMON_HELPER_DIR` (line 27); its own 15-test suite still passes inside the full verify chain. |
| `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` | Truthful fallback counter snapshot semantics | ✓ VERIFIED | Snapshot-time mixing removed; `self.fallback` passed directly; durable Timeout-cleanup increments untouched; contains `CompactRelayFallbackCounters` usage. |
| `packages/open-bitcoin-node/src/status/block_relay_evidence.rs` | Documented timeouts-only field semantics, no schema change | ✓ VERIFIED | Doc comment on `compact_timeout_count`; struct keeps exactly two fields. |
| `packages/open-bitcoin-node/src/network/tests/compact_timeout_cases.rs` | D-06 regression tests | ✓ VERIFIED | Test A (live in-flight → zero fallback timeouts) and Test B (real Timeout cleanup → increments) present at lines 269/299. |
| `docs/parity/catalog/p2p.md` | Recorded D-06 semantics decision | ✓ VERIFIED | Fix-path sentence at line 1327 in no-claim vocabulary; `docs/parity/index.json` still contains "Phase 128 retains" and "Phase 129 retains" (Phase 127 checker green). |
| `scripts/check-phase124-archive-ready.ts` | Archive-ready and verified-pre-promotion stage assertions | ✓ VERIFIED | Exports `detectPhase129ReconciliationStage`, `verifyVerifiedPrePromotion`, `verifyArchiveReady`; contains the D-13 literals `/gsd-complete-milestone v2.1`, `requirements: "39/39"`, `**Plans:** 4/4 plans complete`, `| HARD-05 | Phase 129 | Complete |`, and the pinned verification frontmatter (`status: passed`, `lifecycle_validated: true`, `generated_by: gsd-verifier`, `lifecycle_mode: yolo`, lifecycle id). |
| `scripts/check-phase124-post-audit-gap-planning.ts` | Stage detection dispatching to the archive-ready module | ✓ VERIFIED | Imports from `./check-phase124-archive-ready` (line 8); dispatcher exits 0 against the current gaps-open repo. |
| `scripts/check-phase124-milestone-closeout-reconciliation.fixtures.ts` | `maybePhase129Stage` fixture projections | ✓ VERIFIED | 7 references to `maybePhase129Stage`; test file uses it in 13 places across legal-stage and mixture cases. |

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| P129 checker | P127 checker | imported `checkPhase127AuthoritativeNetworkStateUnification(repoRoot)` | ✓ WIRED | Import at line 6, invoked at line 82; mutation row 9 proves a P127 seam failure surfaces through the aggregate. |
| P129 checker | P128 checker | imported `checkPhase128ProductionCompactAnnouncementTransport(repoRoot)` | ✓ WIRED | Import at line 7, invoked at line 84; mutation row 10 proves a P128 seam failure surfaces. |
| `scripts/verify.sh` | P129 checker | default deterministic invocation between Phase 128 and Phase 117 | ✓ WIRED | `run_step "check Phase 129 ..."` at line 564; removal of either surface is a mutation-tested `P129 verifier` failure. |
| `network/block_relay_evidence.rs` | `status/block_relay_evidence.rs` | `BlockRelayEvidenceStatus::with_components` fallback facet | ✓ WIRED | `self.fallback` passes directly into the shared status contract at line 88; contract fields unchanged. |
| `check-phase124-post-audit-gap-planning.ts` | `check-phase124-archive-ready.ts` | stage-dispatch import inside `verifyPostAuditGapPlanningStage` | ✓ WIRED | Import present; dispatcher runs the stage detection first and passes on the live gaps-open corpus. |
| reconciliation test suite | fixtures | `createFixture` with `maybePhase129Stage` | ✓ WIRED | 88 tests exercise all three stage projections and mixture mutations through the shared fixture builder. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| --- | --- | --- | --- | --- |
| P129 aggregate checker | composed failure list | live repo corpus reads via both upstream checkers with the shared `repoRoot` | Yes — mutation rows prove real upstream failures propagate | ✓ FLOWING |
| Fallback facet status | `fallback.compact_timeout_count` | durable `record_cleanup` Timeout increments only (snapshot-time mixing removed) | Yes — regression tests pin both directions | ✓ FLOWING |
| In-flight facet status | `getblocktxn_in_flight_count` | live `PeerManager` per-peer in-flight state at snapshot time | Yes — single projection of the live fact | ✓ FLOWING |
| Stage machine | `Phase129ReconciliationStage` | live REQUIREMENTS/ROADMAP/audit/verification artifact evidence | Yes — detection is evidence-claimed, not flag-driven | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| --- | --- | --- | --- |
| Live aggregate guard on current repo | `bun run scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts` | "Phase 129 integration guardrails and milestone reconciliation validated." exit 0 | ✓ PASS |
| Every P129 assertion family mutation-fails | `bun test scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.test.ts` | 12 pass, 0 fail | ✓ PASS |
| Stage machine green on gaps-open repo | `bun run scripts/check-phase124-milestone-closeout-reconciliation.ts` | passed, exit 0 | ✓ PASS |
| Three legal stages + mixture rejection | `bun test scripts/check-phase124-milestone-closeout-reconciliation.test.ts` | 88 pass, 172 assertions, 0 fail | ✓ PASS |
| OBS-01 label vocabulary intact | `bun run scripts/check-phase116-operator-block-relay-evidence.ts` | validated, exit 0 | ✓ PASS |
| Promotion ordering guard | `bun run scripts/check-active-milestone-verification-traceability.ts` | passed, exit 0 | ✓ PASS |
| Parity breadcrumbs for touched Rust | `bun run scripts/check-parity-breadcrumbs.ts` | 389 Rust files verified | ✓ PASS |
| Lifecycle provenance | `gsd-tools.cjs verify lifecycle 129 --require-plans --raw` | `valid` (plans 01–04 + context + summaries 01–03; plan 04 summary pending by design) | ✓ PASS |
| Full contract incl. cargo suite | pre-commit hook of `eea4bce1` (`bash scripts/verify.sh`) | clean run recorded at commit time; working tree unchanged since | ✓ PASS |

### Requirements Coverage

Phase 129 roadmap requirements (OBS-01, BOUND-02, HARD-05) are claimed by plans 01/02/03/04 — no orphaned requirements. Per D-11, this verification also re-attests the 7 reassigned requirements already closed by Phases 127/128 against production-path evidence.

| Requirement | Source | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| OBS-01 | Plans 129-01, 129-02 | RPC and shared network status report activation, eligibility, negotiation, reconstruction, fallback, and in-flight compact-block state truthfully | ✓ SATISFIED (promotion in Plan 04) | All six facets project from the single `ManagedNetworkHandle` via `authoritative_operator_snapshot()` (guarded field-by-field by the composed Phase 127 checker); the D-06 truthfulness defect (snapshot-time in-flight mixing into the fallback timeout counter) was fixed with regression tests and documented semantics; Phase 116 label guard green. |
| BOUND-02 | Plan 129-01 | Deterministic checkers prevent forbidden claims from entering v2.1 artifacts and bind to production-path evidence | ✓ SATISFIED (promotion in Plan 04) | The aggregate guard now binds default verification to production callers, shared authority, live peer facts, transport emission, and post-write evidence (six seams + four named flows, all mutation-tested); the Phase 117 final no-claim gate is byte-identical and asserted final by `P129 final gate` and `requireFinalPhaseChecker`; verify.sh remains deterministic and public-network-free. |
| HARD-05 | Plan 129-03 | Roadmap, requirement coverage, phase status, and the milestone audit agree and route v2.1 to archival | ✓ SATISFIED (promotion in Plan 04) | The fail-closed three-state stage machine makes an inconsistent archive claim unrepresentable: any archive-ready evidence claims the full D-13 end-state; MILESTONES.md/PROJECT.md routing is guarded; HARD-05 ownership is pinned to Phase 129; mixtures are mutation-tested. The reconciled agreement itself lands in Plan 04's atomic commit, gated on this verification. |
| BSRV-03 | Phase 127 (re-attested) | Serve only validated, available blocks from authoritative durable data | ✓ SATISFIED | 127-VERIFICATION.md passed (11/11) with BSRV-03 in `requirements_verified`; the shared-authority/durable-serving seams are now additionally fail-closed through the composed P127 guard inside the P129 aggregate; the FLOW-01 production-composition anchor is present and guard-pinned. |
| BSRV-04 | Phase 127 (re-attested) | Bounded getdata handling with caps, backpressure, and cleanup | ✓ SATISFIED | Same 127 verification closure; the production composition and inbound-serving seam anchors remain present under the composed guard. |
| OBS-02 | Phase 127 (re-attested) | CLI/dashboard render from the shared status contract without leakage | ✓ SATISFIED | 127-VERIFICATION.md closure; the P127 checker (composed) pins the dashboard/support shared-projection test anchors (`dashboard_model_block_relay_rows_surface_shared_status_contract`, `support_bundle_renders_block_relay_evidence_from_shared_projection`). |
| OBS-04 | Phase 127 (re-attested) | Support bundles sanitize block-serving and compact-relay evidence | ✓ SATISFIED | 127-VERIFICATION.md closure (production JSON/Markdown redaction verified); redaction anchors remain guard-pinned via composition. |
| CMP-04 | Phase 128 (re-attested) | Deterministic per-peer compact capability and preference tracking | ✓ SATISFIED | 128-VERIFICATION.md passed (10/10) covering CMP-04; the local sendcmpct offer seam is fail-closed through the composed P128 guard (mutation row 10 reuses the exact local-offer mutation). |
| CMP-05 | Phase 128 (re-attested) | Announce compact blocks only when activation, negotiation, header state, availability, and resources permit | ✓ SATISFIED | 128-VERIFICATION.md closure; FLOW-02 (live peer facts) and FLOW-03 (post-write-only credit) production transport anchors are present and named by the P129 guard with distinct failure strings. |
| OBS-03 | Phase 128 (re-attested) | Fixed low-cardinality metrics/logs for relay outcomes | ✓ SATISFIED | 128-VERIFICATION.md closure (receipt-only fixed counters); the post-write evidence seam is composed into the aggregate guard, and the D-06 fix removed the one remaining counter-truthfulness ambiguity. |

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| `scripts/check-phase129-...test.ts` | 129 | literal "placeholder" inside a mutation-fixture string (`check Phase 130 placeholder`) | ℹ️ Info | Intentional final-gate mutation-row content, not a stub. |
| Phase corpus (checkers, Rust fix, tests) | — | No TODO/FIXME/HACK/not-yet-implemented, empty implementation, or hollow wiring found in any file created or modified by plans 01–03. | ℹ️ Info | None. |

### Locked Decision Audit

D-01 through D-16 were each checked against the delivered code: D-01/D-02/D-03/D-04/D-15/D-16 (aggregate guard, triple wiring, composition, named flows, final gate, deterministic scope) — delivered by plan 01; D-05/D-06/D-07 (no new runtime features, evidence-driven fix path, breadcrumbs) — delivered by plan 02 with the fix-path decision recorded; D-08/D-09/D-10 (stage machine, HARD-05 ownership, fail-closed mixtures) — delivered by plan 03; D-11/D-12/D-13/D-14 (verification-gated promotion, in-place audit rerun, artifact reconciliation, archive routing) — enforced by the stage machine and pending execution in plan 04 as designed. No locked decision was violated.

### Human Verification Required

None. The phase consists of deterministic local checkers, a headless Rust counter-semantics fix with regression tests, and planning-artifact stage machinery — all observable through the deterministic verification contract without public networks, visual surfaces, or external services (matching the Phase 127/128 precedent).

### Gaps Summary

No actionable gaps. All twelve merged roadmap/plan must-haves are verified against the committed codebase; all artifacts are substantive and wired; all key links hold; all ten reassigned requirements are closed or re-attested against production-path evidence. The only outstanding items — the requirement checkbox promotion, the in-place audit rerun to `status: passed`, and the six-artifact routing reconciliation — belong to Plan 129-04, which is deliberately gated on this verification by locked decision D-11 and enforced fail-closed by the Plan 03 archive-ready stage machine. The current gaps-open planning-artifact state is the required legal state at this point in the lifecycle.

***

_Verified: 2026-07-21T03:42:06Z_
_Verifier: the agent (gsd-verifier)_
