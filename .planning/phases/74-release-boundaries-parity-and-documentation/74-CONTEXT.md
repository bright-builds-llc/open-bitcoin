---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 74-2026-06-14T15-07-06
generated_at: 2026-06-14T15:07:06.000Z
---

# Phase 74: Release Boundaries, Parity, and Documentation - Context

**Gathered:** 2026-06-14
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 74 closes v1.6 Mainnet Full-Sync Completion by making the shipped claim
auditable from parity roots, release-readiness docs, threat-model docs,
operator guidance, deterministic release-boundary checks, README wording, and
final milestone traceability. Reviewers should be able to confirm that v1.6
claims explicit opt-in full-sync completion only: sync-to-tip and stay-current
evidence for `open-bitcoind` under source-built, local operator review.

This phase owns REL-01 through REL-03 and the final v1.6 traceability closeout.
It consumes Phase 68 through Phase 73 evidence. It must not add new sync
runtime behavior, move public-network checks into default verification, broaden
public-network UAT into release-blocking CI, or imply inbound serving, address
relay, block serving, transaction relay, compact block relay, production-funds
wallet safety, migration apply mode, signed packaging, Windows service support,
GUI parity, hosted dashboards, or broad production-node readiness.

</domain>

<decisions>

## Implementation Decisions

### v1.6 Release Claim Shape

- **D-01:** Treat the v1.6 claim as "source-built, explicit opt-in mainnet
  full-sync completion evidence." The claim covers validated active-chain
  progress to the best-known peer tip, durable restart/resume state, stay-current
  review, reorg/no-progress/recovery handling, resource bounds, shared status
  evidence, redacted support bundles, and opt-in UAT commands.
- **D-02:** Preserve the established non-claim list everywhere this phase
  touches docs, parity roots, checkers, or status wording: inbound serving,
  address relay, block serving, transaction relay, compact block relay,
  production-funds wallet safety, migration apply mode, signed packaging,
  Windows service support, GUI parity, hosted dashboards, public-network CI,
  release-blocking live sync, and broad production-node readiness.
- **D-03:** Keep v1.3, v1.4, and v1.5 threat models and release-boundary
  sections as historical evidence. Add v1.6-specific sections and roots rather
  than rewriting prior milestone claims as current.

### Parity Roots And Threat Model

- **D-04:** Add a v1.6 threat model document, most likely
  `docs/parity/threat-model-v1.6.md`, using the existing compact STRIDE, ASVS
  L1 mapping, evidence acceptance, release-boundary matrix, and requirements
  traceability pattern from v1.5.
- **D-05:** Refresh `docs/parity/release-readiness.md` with a v1.6 claim
  boundary matrix that maps Phase 68 through Phase 73 evidence to REL-01 through
  REL-03 and names explicit non-claims for each production-adjacent surface.
- **D-06:** Update reviewer-facing roots (`docs/parity/README.md`,
  `docs/parity/checklist.md`, `docs/parity/index.json`,
  `docs/parity/deviations-and-unknowns.md`, and relevant catalog pages) so the
  v1.6 closeout is discoverable from both human and machine-readable entrypoints.

### Deterministic Release-Boundary Check

- **D-07:** Add a Phase 74 or v1.6 deterministic checker in the existing Bun
  style, for example `scripts/check-v1.6-release-boundaries.ts`, and wire it
  into `bash scripts/verify.sh` after the Phase 73 checker if it remains local,
  short-running, deterministic, public-network-free, service-manager-free, and
  timing-stable.
- **D-08:** The checker should require v1.6 parity roots, threat model,
  release-readiness matrix, README/operator docs, requirement IDs REL-01 through
  REL-03, final traceability for all 26 v1.6 requirements, and all deferred
  scope terms.
- **D-09:** The checker must fail if default verification invokes public-network
  live smoke, manual peers, `--restart-after-progress`, real `systemctl` or
  `launchctl`, mainnet IBD activation, current-tip timing gates, or
  release-blocking live-sync commands.

### Operator Docs And README

- **D-10:** Update operator docs to explain shipped sync-to-tip evidence,
  stay-current review, opt-in UAT commands, support evidence paths, failure
  interpretation, and deferred scope without scattering a second authoritative
  UAT matrix. Phase 73's matrix remains the command source of truth unless
  Phase 74 needs a short v1.6 release closeout subsection.
- **D-11:** README should reflect the current v1.6 review posture without
  implying production-funds or broad production-node readiness. It should point
  reviewers to parity roots, release readiness, runtime guide, and deterministic
  verification rather than duplicating all release evidence.
- **D-12:** Operator-facing command examples must continue using repo-local
  Cargo and Bazel forms for CLI-backed workflows, matching the local lessons and
  `AGENTS.md` guidance.

### Final Traceability And Archive Readiness

- **D-13:** Requirements traceability must show all 26 v1.6 requirements mapped
  and verified before archive readiness is claimed. REL-01 through REL-03 should
  close in this phase; prior completed requirements should stay linked to their
  owning phases.
- **D-14:** Keep generated live-mainnet reports, support bundles, daemon logs,
  metrics stores, compatibility reports, and local datadirs out of git. Docs may
  name local artifact paths and field names, but committed release evidence
  remains deterministic and redacted.
- **D-15:** If this phase changes first-party Rust source or tests under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, update
  `docs/parity/source-breadcrumbs.json`. No Rust changes are expected for the
  release-boundary closeout unless planning discovers a narrow status wording or
  test gap.

### the agent's Discretion

- The planner may split Phase 74 into docs/parity-root work, deterministic
  checker wiring, and final traceability/README closeout if that keeps review
  focused.
- The executor may reuse the v1.5 release-boundary checker structure with
  v1.6-specific constants, IDs, evidence paths, and error messages.
- The executor may keep Phase 74 implementation primarily in docs and Bun
  checker code if no source behavior gap is found.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 74 goal, dependency on Phase 73, success
  criteria, and final v1.6 milestone closeout boundary.
- `.planning/REQUIREMENTS.md` - REL-01 through REL-03, all 26 v1.6 requirement
  mappings, and future/out-of-scope production surfaces.
- `.planning/PROJECT.md` - v1.6 milestone goal, current state, pinned Knots
  baseline, functional-core boundary, and production-claim limits.
- `.planning/STATE.md` - Phase 74 readiness, accumulated v1.6 decisions, and
  current milestone status.
- `AGENTS.md` - Repo-local GSD workflow, Rust, parity breadcrumb, UAT command,
  generated artifact, and verification requirements.
- `AGENTS.bright-builds.md` - Bright Builds workflow and standards routing.
- `standards-overrides.md` - Current local standards override registry.

### Prior v1.6 Decisions And Evidence

- `.planning/phases/68-full-active-chain-validation-and-durable-persistence/68-CONTEXT.md`
  - Validated active-chain progress, durable UTXO/undo persistence, duplicate
  connect prevention, and no-credit peer outcomes.
- `.planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md`
  - Best-known tip evidence, current/stale/recovering states, and stay-current
  reporting semantics.
- `.planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md`
  - Reorg evidence, peer response failure handling, typed no-progress causes,
  and next-action guidance.
- `.planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md`
  - Resource bounds, same-datadir restart/resume, storage-pressure recovery, and
  deterministic long-chain proof.
- `.planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md`
  - Shared full-sync truth contract, support verdicts, live-smoke summaries,
  cross-surface agreement, and docs/checker closeout.
- `.planning/phases/73-opt-in-uat-and-deterministic-verification/73-CONTEXT.md`
  - Opt-in public-mainnet UAT matrix, deterministic checker coverage, parity
  breadcrumbs, fixture evidence, and default-verification exclusions.
- `.planning/phases/73-opt-in-uat-and-deterministic-verification/73-VERIFICATION.md`
  - Passed Phase 73 evidence to carry into final v1.6 traceability.

### Historical Release-Boundary Patterns

- `.planning/phases/49-threat-model-and-release-boundaries/49-CONTEXT.md` -
  v1.3 threat-model and release-boundary approach.
- `.planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-CONTEXT.md`
  - v1.4 evidence, threat model, and release-boundary approach.
- `.planning/phases/67-release-boundaries-and-deterministic-verification/67-CONTEXT.md`
  - v1.5 threat-model, release-readiness, parity root, and deterministic
  checker posture.
- `docs/parity/threat-model-v1.3.md`,
  `docs/parity/threat-model-v1.4.md`, and
  `docs/parity/threat-model-v1.5.md` - Existing threat-model documents to
  preserve as historical patterns.
- `scripts/check-v1.3-release-boundaries.ts`,
  `scripts/check-v1.4-release-boundaries.ts`, and
  `scripts/check-v1.5-release-boundaries.ts` - Existing deterministic
  release-boundary checker patterns.

### Current Implementation And Verification Surfaces

- `scripts/verify.sh` - Repo-native deterministic verification contract and
  checker wiring order.
- `scripts/check-phase73-uat-verification.ts` and
  `scripts/check-phase73-uat-verification.test.ts` - Phase 73 UAT and
  deterministic closeout checker pattern.
- `scripts/run-live-mainnet-smoke.ts` - Opt-in public-mainnet live-smoke report
  generator that must remain outside default verification.
- `scripts/test-run-live-mainnet-smoke.sh` - Deterministic live-smoke fixture
  checker.
- `scripts/check-parity-breadcrumbs.ts` and
  `docs/parity/source-breadcrumbs.json` - Required parity breadcrumb mechanism
  for new first-party Rust source or test files.
- `docs/operator/runtime-guide.md` - Authoritative operator guide and Phase 73
  UAT matrix.
- `docs/architecture/status-snapshot.md` - Shared full-sync status contract and
  field interpretation.
- `docs/architecture/operator-observability.md` - Metrics/log/support evidence,
  retention, compact snapshots, and deterministic verification boundaries.
- `docs/architecture/storage-decision.md` - Durable storage and recovery
  posture.
- `README.md` - Contributor/operator entrypoint requiring v1.6 claim and
  release-readiness wording.

### Parity Roots And Catalog Pages

- `docs/parity/release-readiness.md` - Current release-readiness matrix to
  extend with v1.6.
- `docs/parity/index.json` - Machine-readable parity root requiring v1.6
  surface and audit entries.
- `docs/parity/checklist.md` - Human-readable parity root requiring v1.6
  surface row.
- `docs/parity/README.md` - Parity entrypoint requiring current v1.6 closeout
  links.
- `docs/parity/deviations-and-unknowns.md` - Deferred-surface and known-risk
  register requiring v1.6 refresh.
- `docs/parity/catalog/p2p.md` - P2P and public-mainnet sync boundary catalog.
- `docs/parity/catalog/chainstate.md` - Active-chain, UTXO/undo, reorg, and
  persistence parity scope.
- `docs/parity/catalog/operator-runtime-release-hardening.md` - Operator runtime
  and evidence boundary catalog.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `scripts/check-v1.5-release-boundaries.ts` already validates parity roots,
  threat-model docs, release-readiness docs, runtime-guide wording, deferred
  scope strings, and `scripts/verify.sh` exclusions with small local helpers.
- `scripts/check-phase73-uat-verification.ts` already verifies opt-in UAT
  command coverage, VER-02 deterministic anchors, parity roots, forbidden
  default-verification strings, and public-network non-claims.
- `docs/parity/release-readiness.md` already contains v1.3, v1.4, and v1.5
  readiness sections and boundary matrices that can be extended with a v1.6
  section.
- `docs/parity/index.json` already has checklist surfaces and audit entries for
  v1.5 release boundaries and Phase 73 opt-in UAT evidence.
- `docs/operator/runtime-guide.md` already contains the authoritative Phase 73
  opt-in public-mainnet UAT matrix with Cargo, Bazel, and Bun command forms.
- `README.md` already points reviewers to parity roots and says the v1.6
  operator runtime is source-built local review, not a production-node claim.

### Established Patterns

- Default verification stays deterministic, public-network-free,
  service-manager-free, timing-stable, and short-running.
- Release-boundary checkers are Bun scripts with explicit required paths,
  required strings, machine-root validation, and forbidden default-verification
  strings.
- Current milestone docs are additive. Historical v1.3, v1.4, and v1.5 evidence
  stays discoverable but is not rewritten as current v1.6 scope.
- Operator evidence language is field-based. Daemon startup, elapsed time, peer
  reachability, report existence, or bundle existence are never sufficient proof
  by themselves.
- Operator UAT commands should use repo-local Cargo and Bazel forms rather than
  relying on an installed alias.

### Integration Points

- Add a v1.6 release-boundary checker and wire it after the Phase 73 checker in
  `scripts/verify.sh`.
- Add `docs/parity/threat-model-v1.6.md` and link it from parity README,
  checklist, release-readiness, index roots, and relevant catalog pages.
- Extend `docs/parity/release-readiness.md` with a v1.6 claim boundary matrix
  and deterministic command list.
- Update `docs/parity/index.json`, `docs/parity/checklist.md`,
  `docs/parity/README.md`, `docs/parity/deviations-and-unknowns.md`,
  `docs/parity/catalog/p2p.md`, `docs/parity/catalog/chainstate.md`, and
  `docs/parity/catalog/operator-runtime-release-hardening.md` with v1.6
  traceability.
- Update `README.md` and `docs/operator/runtime-guide.md` only enough to make
  the v1.6 closeout and evidence interpretation current without duplicating the
  full parity matrix.

</code_context>

<specifics>

## Specific Ideas

- Prefer a new checklist surface id such as
  `v1-6-full-sync-completion-release-boundaries`.
- Prefer new audit entries such as `v1_6_threat_model` and
  `v1_6_release_boundaries` in `docs/parity/index.json`.
- The v1.6 threat model should name the Phase 68 through Phase 73 evidence
  chain: active-chain validation, tip/stay-current, reorg/peer/no-progress,
  resource/restart/recovery, observability/support evidence, and opt-in UAT.
- The v1.6 release checker should require all 26 v1.6 requirement IDs or a
  machine-readable traceability section that proves each requirement is mapped
  to a completed phase and evidence surface.
- Keep generated live-smoke reports and support bundles as local artifacts;
  docs can list paths such as
  `packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json`,
  `support-evidence.json`, and `support-evidence.md` for operator review.

</specifics>

<deferred>

## Deferred Ideas

- Inbound peer serving, address relay, block serving, transaction relay,
  compact block relay, production-funds wallet safety, migration apply mode,
  signed packaging, Windows service support, GUI parity, hosted dashboards,
  public-network CI, release-blocking live sync, broad production-node
  readiness, centralized trusted peers, hidden tip oracles, pruning, assumeutxo,
  assumevalid, and snapshot bootstrap remain future scope.

</deferred>

---

*Phase: 74-release-boundaries-parity-and-documentation*
*Context gathered: 2026-06-14*
