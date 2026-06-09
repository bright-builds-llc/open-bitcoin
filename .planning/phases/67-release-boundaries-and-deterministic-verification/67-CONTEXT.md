---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 67-2026-06-09T00-30-52
generated_at: 2026-06-09T00:30:52.044Z
---

# Phase 67: Release Boundaries and Deterministic Verification - Context

**Gathered:** 2026-06-09
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 67 closes v1.5 by making the shipped unattended mainnet operator-review
claim auditable from docs, parity roots, release-readiness evidence, and
deterministic verification. It consumes Phases 60 through 66 and must prove that
reviewers can distinguish extended unattended operator review readiness from
deferred production-node, inbound-serving, relay, wallet, migration-apply,
packaging, hosted-dashboard, Windows-service, and GUI claims.

This phase owns refreshed v1.5 threat-model and release-readiness docs, parity
root/checklist updates, deferred-scope wording, default-verification boundary
guards, and deterministic checks for the unattended-operation claim boundary. It
does not add new sync runtime behavior, broaden public-network verification,
start real launchd/systemd managers during default checks, or convert opt-in UAT
evidence into a release gate.

</domain>

<decisions>

## Implementation Decisions

### v1.5 Release Claim Shape

- **D-01:** Treat the v1.5 claim as "source-built, explicit opt-in unattended
  mainnet operator review readiness." The claim covers the bounded daemon loop,
  recovery taxonomy, long-run truth surfaces, launchd/systemd operator
  lifecycle, same-datadir service restart/resume evidence, redacted support
  bundles, operator review docs, and the compatibility harness wrapper.
- **D-02:** Do not use production-node phrasing. Docs, parity roots, and
  checkers must keep inbound serving, address advertisement, transaction relay,
  compact block relay, production-funds wallet use, migration apply mode,
  packaging distribution, Windows service integration, hosted dashboard, GUI
  work, and broad production full-node support as explicit deferred scopes.
- **D-03:** Keep v1.3 and v1.4 threat/release docs as historical evidence. Add
  v1.5-specific surfaces and links rather than rewriting earlier milestone
  claims as if they were current.

### Threat Model And Release Readiness

- **D-04:** Add a v1.5 threat model document, most likely
  `docs/parity/threat-model-v1.5.md`, covering unattended sync loop behavior,
  service supervision, long-run evidence, resource bounds, recovery states,
  support redaction, compatibility wrapper reports, deterministic checks, and
  deferred production surfaces.
- **D-05:** Refresh `docs/parity/release-readiness.md` with a v1.5 boundary
  matrix that maps each Phase 60 through Phase 66 evidence surface to the REL-01
  through REL-04 closeout. The matrix should name accepted evidence and explicit
  non-claims for each surface.
- **D-06:** Update reviewer-facing entrypoints (`docs/parity/README.md`,
  `docs/parity/checklist.md`, and `docs/parity/index.json`) so the v1.5
  closeout is discoverable from both human and machine-readable roots.

### Deterministic Verification Boundary

- **D-07:** Add a Phase 67 deterministic checker, following the existing
  TypeScript/Bun checker style, to assert that v1.5 docs and parity roots include
  the unattended-operation claim boundary and REL-01 through REL-04 traceability.
- **D-08:** Wire the Phase 67 checker into `bash scripts/verify.sh` only if it
  remains deterministic and local. The checker must not run live smoke, manual
  peers, `--restart-after-progress`, `systemctl --user`, `launchctl`, or any
  public-network/service-manager operation.
- **D-09:** The checker should fail when required v1.5 evidence paths, deferred
  scope strings, release-boundary wording, or `scripts/verify.sh` exclusion rules
  are missing.

### Documentation And Parity Roots

- **D-10:** Update `docs/parity/catalog/p2p.md` with a v1.5 release-boundary
  section that ties unattended review, service restart/resume, support bundles,
  and compatibility wrapper evidence together without broadening the P2P claim.
- **D-11:** Update `docs/operator/runtime-guide.md` only where reviewer closeout
  wording or command interpretation needs to point at the v1.5 release boundary.
  Preserve repo-local Cargo and Bazel command examples for operator workflows.
- **D-12:** Keep generated live-smoke reports, compatibility reports, support
  bundles, daemon logs, metrics stores, local datadirs, and real service-manager
  outputs as local artifacts outside git.

### Claude's Discretion

- The planner may split Phase 67 into docs/parity-root work and checker/verify
  wiring if that keeps review focused. A single plan is acceptable if the change
  remains cohesive and mostly documentation plus deterministic TypeScript.
- The executor may reuse the v1.4 release-boundary checker structure when it is
  the smallest robust path, but should use v1.5-specific constants and error
  messages so failures point at the current milestone.
- No new first-party Rust code is expected. If execution unexpectedly adds Rust
  source or tests under `packages/open-bitcoin-*`, update parity breadcrumbs
  before committing.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 67 goal, success criteria, dependency on Phase
  66, and REL-01 through REL-04 scope.
- `.planning/REQUIREMENTS.md` - REL-01, REL-02, REL-03, REL-04 and explicit
  v1.5 out-of-scope boundaries.
- `.planning/PROJECT.md` - v1.5 milestone goal, current state, core value, and
  production-claim constraints.
- `.planning/STATE.md` - Recent decisions about deterministic verification,
  opt-in UAT, support evidence, and compatibility wrapper boundaries.

### Prior v1.5 Decisions And Evidence

- `.planning/phases/60-unattended-sync-loop-control/60-CONTEXT.md` - Explicit
  opt-in unattended loop activation, stop reasons, pause/resume/shutdown, and
  deterministic verification posture.
- `.planning/phases/60-unattended-sync-loop-control/60-VERIFICATION.md` - Passed
  Phase 60 evidence.
- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md` -
  Resource bounds, recovery taxonomy, redaction, and default-verification
  decisions.
- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-VERIFICATION.md`
  - Passed recovery/resource evidence and checker pattern.
- `.planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md` - Shared truth
  fields across status, dashboard, RPC, metrics, logs, and live smoke.
- `.planning/phases/62-long-run-sync-truth-surfaces/62-VERIFICATION.md` - Passed
  Phase 62 truth-surface verification.
- `.planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md` - Service
  lifecycle labels, launchd/systemd boundaries, and service-manager UAT
  exclusions.
- `.planning/phases/63-service-supervision-lifecycle/63-VERIFICATION.md` - Passed
  Phase 63 service lifecycle evidence.
- `.planning/phases/64-service-restart-and-same-datadir-resume-evidence/64-CONTEXT.md`
  - Service restart/resume contract, same-datadir safety, and opt-in UAT
  boundary.
- `.planning/phases/64-service-restart-and-same-datadir-resume-evidence/64-VERIFICATION.md`
  - Passed Phase 64 restart/resume evidence.
- `.planning/phases/65-support-bundle-and-operator-review-docs/65-CONTEXT.md` -
  Redacted support bundle, operator review docs, and public-network/service UAT
  exclusions.
- `.planning/phases/65-support-bundle-and-operator-review-docs/65-VERIFICATION.md`
  - Passed Phase 65 support review evidence.
- `.planning/phases/66-compatibility-harness-operator-wrapper/66-CONTEXT.md` -
  Compatibility wrapper report shape, pure harness delegation, and default
  verification boundary.
- `.planning/phases/66-compatibility-harness-operator-wrapper/66-VERIFICATION.md`
  - Passed Phase 66 compatibility wrapper evidence.

### Existing Docs And Checkers

- `docs/parity/release-readiness.md` - Current v1.3/v1.4 release-readiness
  matrix and reviewer checklist to extend for v1.5.
- `docs/parity/threat-model-v1.4.md` - Existing current milestone threat-model
  pattern to adapt without overwriting as v1.5.
- `docs/parity/threat-model-v1.3.md` - Historical threat-model pattern and
  evidence-preservation model.
- `docs/parity/index.json` - Machine-readable parity root requiring a v1.5
  surface and audit entries.
- `docs/parity/checklist.md` - Human-readable parity root requiring a v1.5
  surface row.
- `docs/parity/README.md` - Parity entrypoint requiring current v1.5 closeout
  links.
- `docs/parity/catalog/p2p.md` - P2P catalog requiring v1.5 release-boundary
  wording.
- `docs/parity/deviations-and-unknowns.md` - Deferred-surface and known-risk
  register that must continue to name deferred production surfaces.
- `docs/operator/runtime-guide.md` - Operator workflow and repo-local UAT command
  guidance.
- `scripts/check-v1.4-release-boundaries.ts` - Prior release-boundary checker
  structure.
- `scripts/check-phase65-support-review.ts` and
  `scripts/check-phase66-compatibility-wrapper.ts` - Recent deterministic
  checker patterns.
- `scripts/verify.sh` - Repo-native deterministic verification contract.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- Existing Bun checkers use small `readText`, `requireContains`, and
  `requireNotContains` helpers with explicit constant lists of required and
  forbidden strings.
- `scripts/verify.sh` already runs v1.3, v1.4, Phase 61, Phase 62, Phase 63,
  Phase 64, Phase 65, and Phase 66 deterministic checkers before Rust builds and
  tests.
- `docs/parity/index.json` already has `checklist.surfaces` entries and `audit`
  sections for v1.3 and v1.4 release boundaries.
- `docs/parity/release-readiness.md` already has a matrix style that can be
  extended with v1.5 rather than replaced.
- `docs/parity/catalog/p2p.md` already has Phase 64, Phase 65, and Phase 66
  v1.5 boundary subsections.

### Established Patterns

- Public-network live-smoke, manual-peer, restart-after-progress, long-run, and
  real service-manager operations are opt-in UAT and stay outside
  `bash scripts/verify.sh`.
- Release-boundary docs preserve historical milestone claims while adding
  current milestone sections.
- Operator-facing docs use evidence-first language, repo-local commands, and
  explicit non-claims.
- Machine-readable parity roots and human checklist rows must agree on surface
  ids, requirements, evidence paths, known gaps, and suspected unknowns.

### Integration Points

- Add `scripts/check-v1.5-release-boundaries.ts` and wire it into
  `scripts/verify.sh` next to the v1.4 checker.
- Add `docs/parity/threat-model-v1.5.md` and link it from README, release
  readiness, checklist, and index roots.
- Update release-readiness, P2P catalog, operator runtime guide, and parity
  roots with REL-01 through REL-04 traceability and v1.5 non-claims.

</code_context>

<specifics>

## Specific Ideas

- Prefer a new checklist surface id such as
  `v1-5-unattended-operation-release-boundaries`.
- Prefer new audit entries such as `v1_5_threat_model` and
  `v1_5_release_boundaries` in `docs/parity/index.json`.
- The Phase 67 checker should require all REL IDs, v1.5 evidence paths, and
  forbidden default-verification strings so omissions fail deterministically.
- The v1.5 threat model should include a compact STRIDE register and release
  boundary matrix rather than a broad security certification claim.

</specifics>

<deferred>

## Deferred Ideas

- Production full-node support, inbound serving/address advertisement,
  transaction relay, compact block relay, production-funds wallet use,
  migration apply mode, signed packaging/distribution, Windows service
  integration, hosted dashboards, GUI parity, public-network CI, and real
  service-manager CI remain future milestones.
- Making public-network long-run evidence part of `bash scripts/verify.sh` is
  explicitly out of scope for v1.5.
- Hosted support upload, support-bundle artifact validators, and signed release
  attestations remain future release-engineering work.

</deferred>

---

*Phase: 67-release-boundaries-and-deterministic-verification*
*Context gathered: 2026-06-09 via yolo discussion*
