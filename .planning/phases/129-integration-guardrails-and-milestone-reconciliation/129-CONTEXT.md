---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 129-2026-07-20T19-28-06
generated_at: 2026-07-20T19:31:54.942Z
---

# Phase 129: Integration Guardrails and Milestone Reconciliation - Context

**Gathered:** 2026-07-20
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 129 closes the remaining cross-cutting verification and reconciliation findings from `.planning/v2.1-MILESTONE-AUDIT.md`. It makes the Phase 127/128 production repairs fail closed under one aggregate deterministic guard, proves all four repaired end-to-end flows through repository verification, independently closes the 10 reassigned requirements (with OBS-01, BOUND-02, and HARD-05 as the three still-Pending owners), and reconciles ROADMAP, REQUIREMENTS, PROJECT, STATE, MILESTONES, and the rerun milestone audit so v2.1 can route to archival.

This phase is guards, verification, and reconciliation. It does not add new runtime relay features, broaden serving defaults, add public-network verification, or perform the actual `/gsd-complete-milestone v2.1` archival — it only produces the agreed archive-ready state that routes there.

</domain>

<decisions>
## Implementation Decisions

### Aggregate Integration Guard

- **D-01:** Add a new deterministic checker pair `scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.ts` plus `scripts/check-phase129-integration-guardrails-and-milestone-reconciliation.test.ts` with fixture-based mutation coverage, following the Phase 127/128 checker conventions (exported `checkPhase129...(maybeRepoRoot?)` returning `string[]` failures, no network or process spawning inside the checker).
- **D-02:** Wire the new pair into `scripts/verify.sh` immediately after the Phase 128 test+check steps and before the Phase 117 test+check steps, updating the ordering comment, the `VERIFY_COMMAND_ORDER` heredoc, and the live `run_step` block together. Phase 117 remains the final `check-phase*` no-claim gate; Phase 129 must not absorb it.
- **D-03:** The aggregate guard covers all six repaired seams under one fail-closed surface: shared authoritative state, local `sendcmpct` emission, production announcement invocation, live per-peer header facts, transport emission, and post-write-only evidence. Reuse the exported Phase 127/128 check functions where practical instead of duplicating anchor logic; add cross-phase assertions those checkers cannot express phase-locally.
- **D-04:** The guard names and asserts the four repaired flows explicitly (FLOW-01 durable validated block → inbound serving; FLOW-02 handshake → bilateral compact negotiation → live header-aware announcement; FLOW-03 high-bandwidth decision → successful wire emission → post-write evidence; FLOW-04 authoritative sync runtime → RPC → CLI/dashboard/support) by requiring the existing Rust production-path test anchors (`packages/open-bitcoin-rpc/tests/black_box_parity.rs`, `packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs`, `packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs`, CLI operator tests) to stay present and connected.

### Runtime Scope

- **D-05:** No new runtime features. Research confirms OBS-01 production wiring is complete after Phases 127/128: RPC `openbitcoinnetworkstatus` reads the single shared `ManagedNetworkHandle` via `authoritative_operator_snapshot()`, and all six OBS-01 facets (activation, eligibility, negotiation, reconstruction, fallback, in-flight) project from that authority. Phase 129 closes OBS-01 with deterministic guards and verification evidence, not re-plumbing.
- **D-06:** If flow verification uncovers an actual production truthfulness defect (for example the snapshot-time mixing of live `getblocktxn_in_flight` entries into `compact_timeout_count` in fallback counters proves misleading), fix it minimally and fail closed; otherwise document the semantics and leave runtime code untouched.
- **D-07:** New or touched first-party Rust source/test files require parity breadcrumbs mapped through `docs/parity/source-breadcrumbs.json`. TypeScript checkers need no breadcrumbs.

### Phase 124 Stage Machine Evolution

- **D-08:** Evolve `scripts/check-phase124-post-audit-gap-planning.ts` (and its dispatch in `check-phase124-milestone-closeout-reconciliation.ts`) with an explicit archive-ready post-129 stage. Today the post-audit stage hard-requires `status: gaps_found`, `29/39`, the exact GAP/FLOW inventory, and Phase 129 pending; the new stage must accept Phase 129 checked with complete plans, coverage 39/39 with 0 pending, empty gap inventories, audit `status: passed`, and archive routing — while continuing to reject any intermediate inconsistent mixture.
- **D-09:** HARD-05 ownership stays on Phase 129 in the reconciled end-state; do not revert to the legacy Phase 124 ownership encoded in the old final-audit path. Update the Phase 124 fixtures/tests to cover the new archive-ready projection.
- **D-10:** Model the reconciliation as fail-closed distinct states (consistent with the Phase 126/128 closeout precedent): gaps-open post-audit state, Phase 129 verified pre-promotion, and reconciled archive-ready. No checker may accept a state where the audit says `passed` but requirement checkboxes, coverage counts, or roadmap status disagree.

### Requirement Closure And Milestone Reconciliation

- **D-11:** Independent verification (gsd-verifier, lifecycle-valid `129-VERIFICATION.md`) explicitly closes all 10 reassigned requirements (BSRV-03, BSRV-04, CMP-04, CMP-05, OBS-01, OBS-02, OBS-03, OBS-04, BOUND-02, HARD-05) against production-path evidence, re-attesting the 7 already Complete via Phases 127/128 and newly closing OBS-01, BOUND-02, HARD-05.
- **D-12:** Rerun the milestone audit in place: update `.planning/v2.1-MILESTONE-AUDIT.md` frontmatter to `status: passed` with full scores (requirements 39/39, integration and flows at full), empty `gaps.*`, and a refreshed body/conclusion/next-action, following the v2.0 passing-audit frontmatter shape. Do not create a companion rerun file.
- **D-13:** Reconcile all planning artifacts to agree: flip the three Pending checkboxes and traceability rows in `.planning/REQUIREMENTS.md` (39/39 Complete), update the Phase 129 row/checkbox and Next Step in `.planning/ROADMAP.md`, refresh the Current Milestone status in `.planning/PROJECT.md`, refresh `.planning/STATE.md` frontmatter/position/todos, and repair the stale `.planning/MILESTONES.md` (currently still 33/39 with a `/gsd-plan-phase 128` next step).
- **D-14:** Route the reconciled end-state to `/gsd-complete-milestone v2.1`. The archival itself (moving ROADMAP/REQUIREMENTS/AUDIT under `.planning/milestones/`) stays outside Phase 129.

### Boundary Preservation

- **D-15:** BOUND-02 closes by demonstrating the deterministic checkers now bind to production-path evidence (production callers, shared authority, post-write evidence) rather than passing without them, while the Phase 117 final no-claim gate and the bounded v2.1 claim vocabulary remain unchanged. Package relay, bloom/filter serving, compact filter serving, public-serving defaults, production readiness, and production-funds claims remain rejected from v2.1 artifacts.
- **D-16:** Default verification remains `bash scripts/verify.sh`: deterministic, local, and public-network-free. No public-network, soak, or service-manager gates may enter the default contract.

### Folded Todos

No pending todos matched Phase 129. The STATE.md pending todo "Plan and execute Phase 129 before rerunning the v2.1 milestone audit" is this phase itself and resolves with it.

### Claude's Discretion

The planner may choose the exact stage names and fixture shapes for the Phase 124 evolution, whether the Phase 129 checker imports 127/128 check functions or re-asserts shared corpus anchors, the split between guard plans and reconciliation plans, and the minimal set of new Rust flow-test anchors (if any) needed beyond the existing corpus. Prefer the smallest guard surface that makes an inconsistent archive claim unrepresentable.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project Rules And Phase Contract

- `AGENTS.md` — repo-local GSD, parity breadcrumb, verification, generated-artifact, and command-timing rules.
- `AGENTS.bright-builds.md` — managed Bright Builds workflow and cross-cutting standards.
- `standards-overrides.md` — local exceptions; no substantive active override applies.
- `standards/core/verification.md` — sync-first and repo-native verification gates.
- `standards/core/testing.md` — focused Arrange/Act/Assert test requirements.
- `standards/languages/typescript-javascript.md` — Bun/TypeScript checker conventions.
- `standards/languages/rust.md` — Rust invariant, module, and adapter guidance.
- `.planning/ROADMAP.md` § Phase 129 — fixed goal, dependency, requirements, and success criteria.
- `.planning/REQUIREMENTS.md` — normative OBS-01, BOUND-02, HARD-05 definitions and the 39-requirement traceability table.
- `.planning/PROJECT.md` — bounded v2.1 claim and integration-gap boundary.
- `.planning/STATE.md` — current route and lifecycle state.
- `.planning/v2.1-MILESTONE-AUDIT.md` — canonical GAP/FLOW evidence, the 10 reassigned requirements, and the fail-closed reconciliation signal.
- `.planning/MILESTONES.md` — stale milestone rollup that must be reconciled.
- `.planning/milestones/v2.0-MILESTONE-AUDIT.md` — passing-audit frontmatter precedent.

### Prior Locked Decisions

- `.planning/phases/127-authoritative-network-state-unification/127-CONTEXT.md` — one production authority, durable block source, short critical sections.
- `.planning/phases/128-production-compact-announcement-transport/128-CONTEXT.md` — bilateral negotiation, post-durable trigger, transport writes, post-write evidence.
- `.planning/phases/127-authoritative-network-state-unification/127-VERIFICATION.md` — closed BSRV-03, BSRV-04, OBS-02, OBS-04; deferred aggregate reconciliation to 129.
- `.planning/phases/128-production-compact-announcement-transport/128-VERIFICATION.md` — closed CMP-04, CMP-05, OBS-03; deferred aggregate guardrails to 129.
- `.planning/phases/126-compact-relay-residual-hardening/126-CONTEXT.md` — fail-closed closeout-state modeling precedent.

### Deterministic Verification Surface

- `scripts/verify.sh` — checker ordering contract (comment + heredoc + run_step triple must stay aligned; Phase 117 stays last).
- `scripts/check-phase127-authoritative-network-state-unification.ts` — shared-authority seam guard and FLOW-01/FLOW-04 anchors.
- `scripts/check-phase128-production-compact-announcement-transport.ts` — sendcmpct/announcement/transport/post-write seam guard and FLOW-02/FLOW-03 anchors.
- `scripts/check-phase124-milestone-closeout-reconciliation.ts` — stage dispatch that must gain the archive-ready stage.
- `scripts/check-phase124-post-audit-gap-planning.ts` — current post-audit stage assertions that reject reconciliation today.
- `scripts/check-phase124-milestone-gap-closure.ts` — imported gap-closure assertions.
- `scripts/check-active-milestone-verification-traceability.ts` — lifecycle-valid requirement verification ownership.
- `scripts/check-parity-breadcrumbs.ts` and `docs/parity/source-breadcrumbs.json` — Rust-file breadcrumb scope (TypeScript exempt).

### Production Evidence Corpus (guard anchors)

- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` — single-authority production composition.
- `packages/open-bitcoin-node/src/network/runtime_authority.rs` — `ManagedNetworkHandle` shared authority.
- `packages/open-bitcoin-rpc/src/context/inbound_status.rs` — `authoritative_operator_snapshot()`.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` and `packages/open-bitcoin-rpc/src/method/node.rs` — RPC status projection and stable schema.
- `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` and `packages/open-bitcoin-node/src/status/block_relay_evidence.rs` — live counters and stable aggregate contract.
- `packages/open-bitcoin-rpc/tests/black_box_parity.rs` — FLOW-01/FLOW-04 production-composition tests.
- `packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs` — FLOW-02/FLOW-03 transport tests.
- `packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs` — announcement policy unit anchors.
- `packages/open-bitcoin-cli/tests/operator_flows.rs` plus dashboard/support test modules — FLOW-04 operator surfaces.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- Phase 127/128 checkers already export check functions with fixture-mutation test suites; the Phase 129 aggregate can compose them.
- The `orderedLines`/`requireOrdered` subsequence pattern in upstream checkers is compatible with inserting Phase 129 between 128 and 117; `requireFinalPhaseChecker` keeps 117 last.
- `ManagedNetworkHandle::operator_snapshot()` already carries all six OBS-01 facets; no schema work is needed.
- The v2.0 passing audit provides the exact frontmatter and archive shape to reproduce.

### Established Patterns

- Substantial repo-owned guards are Bun/TypeScript with deterministic fixture mutation coverage, wired through `scripts/verify.sh`.
- Closeout reconciliation is modeled as explicit fail-closed stages (Phase 126 and 128 precedent), never as loosened assertions.
- Requirement promotion happens only after independent lifecycle-valid verification (Phase 126 precedent).
- Milestone audits are superseded in place; archives land under `.planning/milestones/` at completion time.

### Integration Points

- `scripts/verify.sh` ordering comment, `VERIFY_COMMAND_ORDER` heredoc, and `run_step` block (all three must change together).
- `check-phase124-post-audit-gap-planning.ts` stage assertions and the reconciliation dispatch that imports it.
- `.planning/REQUIREMENTS.md` traceability table, `.planning/ROADMAP.md` progress rows, `.planning/PROJECT.md` milestone status, `.planning/STATE.md`, `.planning/MILESTONES.md`, and `.planning/v2.1-MILESTONE-AUDIT.md`.

</code_context>

<specifics>
## Specific Ideas

- Treat "audit passed" as a state that is only representable when every planning artifact agrees; the guard should make a half-reconciled milestone fail verification.
- Keep the Phase 129 checker aggregate and cross-phase: its value over 127/128 is naming the four flows and the archive-ready contract, not re-implementing seam anchors.
- The stale `.planning/MILESTONES.md` (33/39, "run /gsd-plan-phase 128") is a concrete example of the drift this phase exists to eliminate — reconcile it and guard against recurrence.

</specifics>

<deferred>
## Deferred Ideas

- The actual `/gsd-complete-milestone v2.1` archival run (move ROADMAP/REQUIREMENTS/AUDIT under `.planning/milestones/`) happens after Phase 129 routes there.
- Refactoring the 1,505-line `scripts/check-phase124-milestone-gap-closure.ts` remains non-blocking maintainability debt unless the stage evolution requires touching it anyway.
- Package relay, bloom/filter serving, compact filters, public relay defaults, public-network CI, archive-node claims, production full-node readiness, production-funds wallet use, migration apply mode, packaging, hosted services, and GUI work remain outside v2.1.

</deferred>

---

*Phase: 129-integration-guardrails-and-milestone-reconciliation*
*Context gathered: 2026-07-20*
