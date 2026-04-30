---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 34-2026-04-30T07-38-33
generated_at: 2026-04-30T07:38:33.000Z
---

# Phase 34: Migration Detection Ownership Model Cleanup - Context

**Gathered:** 2026-04-30
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Tighten the shared read-only detection model so scan-wide managed-service
definitions stop riding along inside every `DetectedInstallation`. This phase
keeps the current migration planner behavior truthful, preserves explicit
`--source-datadir` selection and manual-review fallbacks, and updates nearby
status, onboarding, wallet, and test consumers to use the repaired ownership
shape. It does not add migration apply mode, change the dry-run safety posture,
rewrite service ownership heuristics, or expand operator-visible migration
features beyond what the ownership cleanup requires.

</domain>

<decisions>
## Implementation Decisions

### Detection ownership model
- **D-01:** Treat service definitions as scan-level evidence, not
  installation-owned evidence. The shared detector should expose a typed
  scan/report object that contains both detected installations and the global
  service-candidate list instead of cloning that list into every
  `DetectedInstallation`.
- **D-02:** Keep `DetectedInstallation` limited to installation-local evidence:
  product family, confidence, uncertainty, datadir/config/cookie paths, and
  wallet candidates. The tightened type should no longer imply that every
  detected service belongs to every install.
- **D-03:** Keep the detector read-only and conservative. This phase may reshape
  the returned evidence model, but it must not broaden filesystem reads,
  introduce mutation, or claim stronger ownership certainty than the current
  migration association helper can prove.

### Consumer adoption
- **D-04:** Preserve the current truthful `migrate plan --source-datadir`
  behavior by threading scan-level service candidates into the existing
  migration association and summary helpers instead of restoring per-install
  ownership shortcuts.
- **D-05:** Status may keep using scan-level service evidence for the existing
  fallback "installed manager present" signal when the platform manager is not
  inspected. Onboarding and wallet should stay on installation-local evidence
  unless they explicitly need scan-level service information after the refactor.
- **D-06:** Keep consumer changes narrow and support-oriented. If a consumer only
  needed service candidates because the old type made them unavoidable, remove
  that dependency instead of inventing a second ownership wrapper.

### Verification and scope discipline
- **D-07:** Add focused regression coverage at the detector/model boundary and at
  the affected consumers, especially migration planner truth and status fallback
  truth. The fix should be proven through unit tests and the existing
  operator-binary migration coverage rather than manual-only inspection.
- **D-08:** Treat Phase 34 as the final optional cleanup. Do not reopen
  `REQUIREMENTS.md`, broaden service-owner parsing, or redesign the migration
  planner beyond the ownership-model tightening needed for truthful behavior.
- **D-09:** Update docs only if the repaired ownership model changes truthful
  operator-facing wording. If runtime output and existing docs remain aligned,
  leave the docs unchanged.

### Claude's Discretion
- If a dedicated name like `DetectionScan` or `DetectionReport` reads better
  than extending the existing vector-only return shape, prefer the clearer
  domain type.
- It is acceptable to stop copying service-definition paths into per-install
  `source_paths` when that makes ownership boundaries clearer, provided status
  and onboarding remain truthful.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope and audit evidence
- `.planning/ROADMAP.md` - Phase 34 goal, dependencies, and success criteria.
- `.planning/STATE.md` - current milestone position with Phase 34 as the final
  pending cleanup.
- `.planning/v1.1-MILESTONE-AUDIT-PHASE-32.md` - tech-debt evidence for the
  scan-wide service-candidate ownership risk.
- `.planning/phases/21-drop-in-parity-audit-and-migration/21-CONTEXT.md` -
  dry-run migration boundary and original detection reuse decisions.
- `.planning/phases/25-migration-source-selection-hardening/25-CONTEXT.md` -
  explicit `--source-datadir` selection rules that this cleanup must preserve.
- `.planning/phases/31-migration-source-specific-service-review-truth/31-CONTEXT.md`
  - planner-side filtering repair that closed the blocker while leaving the
  shared data-shape debt behind.

### Current shared code surfaces
- `packages/open-bitcoin-cli/src/operator/detect.rs` - current read-only
  detection types and the scan-wide `service_candidates` cloning behavior.
- `packages/open-bitcoin-cli/src/operator/runtime.rs` - shared detection
  gathering and runtime dispatch for status, onboarding, migration, dashboard,
  and wallet commands.
- `packages/open-bitcoin-cli/src/operator/status.rs` - fallback service-status
  inference that currently derives from `DetectedInstallation`.
- `packages/open-bitcoin-cli/src/operator/status/detection.rs` - detection
  health-signal rendering from shared evidence.
- `packages/open-bitcoin-cli/src/operator/onboarding.rs` - onboarding messages
  that reuse detected-installation evidence.
- `packages/open-bitcoin-cli/src/operator/wallet.rs` - wallet safety checks that
  reuse detected wallet candidates.
- `packages/open-bitcoin-cli/src/operator/migration.rs` - migration command
  entrypoint and plan rendering.
- `packages/open-bitcoin-cli/src/operator/migration/planning.rs` - selected
  source summary, action groups, and deviation relevance.
- `packages/open-bitcoin-cli/src/operator/migration/service_evidence.rs` -
  service ownership association helper that currently filters the cloned list.
- `packages/open-bitcoin-cli/src/operator/detect/tests.rs` - detector coverage
  for shared read-only evidence.
- `packages/open-bitcoin-cli/src/operator/status/tests.rs` - status fallback
  truth and detection-evidence fixtures.
- `packages/open-bitcoin-cli/src/operator/migration/tests.rs` - focused planner
  regressions around selected-source migration truth.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` - end-to-end migration
  and operator-surface regressions.

### Repo workflow and standards
- `AGENTS.md` - repo-native verification contract, planning workflow rules, and
  doc freshness guidance.
- `AGENTS.bright-builds.md` - sync-first, repo-native verification, and
  thin-shell guidance.
- `standards-overrides.md` - local standards exception ledger (no substantive
  overrides currently recorded).
- [Bright Builds `standards/core/architecture.md`](https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md)
  - functional-core and domain-type guidance.
- [Bright Builds `standards/core/code-shape.md`](https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/code-shape.md)
  - early returns and readability guardrails.
- [Bright Builds `standards/core/verification.md`](https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/verification.md)
  - sync-first and repo-native verification expectations.
- [Bright Builds `standards/core/testing.md`](https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/testing.md)
  - focused AAA unit-test expectations.
- [Bright Builds `standards/languages/rust.md`](https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/languages/rust.md)
  - Rust-specific domain-type and `let...else` guidance.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `detect_existing_installations()` already gathers installation, wallet, and
  service evidence in one read-only pass, so the phase can repair ownership by
  changing the returned model rather than adding another scanner.
- `associate_service_candidates()` already knows how to conservatively match a
  service definition to a selected installation using datadir/config evidence.
- `StatusCollectorInput` and runtime detection gathering already centralize the
  cross-command handoff point for status, dashboard, onboarding, migration, and
  wallet consumers.
- The focused detector, status, migration, and operator-binary tests already
  cover the edges most likely to regress during a data-shape cleanup.

### Established Patterns
- Shared operator workflows gather filesystem evidence once, then pass typed
  summaries into focused modules rather than reopening the filesystem in every
  consumer.
- Migration planning remains dry-run only and falls back to explicit manual
  review whenever source ownership is ambiguous.
- Operator output should stay support-oriented and secret-safe: paths and
  uncertainty are visible, but cookie contents and raw service file contents are
  not.

### Integration Points
- `DetectedInstallation.service_candidates` is currently the misleading shared
  field that future consumers could treat as source-specific.
- `StatusDetectionEvidence` currently exposes only detected installations, so
  status fallback will need either the new scan-level evidence type or an
  explicit service-candidate field after the refactor.
- Migration summaries and action groups already consume a filtered
  source-specific service list, so they can adopt the new scan-level evidence
  without changing the operator-facing truth contract.

</code_context>

<specifics>
## Specific Ideas

- Introduce a dedicated scan-level detection type that contains
  `installations: Vec<DetectedInstallation>` and
  `service_candidates: Vec<ServiceCandidate>`.
- Remove `service_candidates` from `DetectedInstallation` and update fixtures so
  installation test data only includes installation-local evidence.
- Keep migration ownership association in `migration/service_evidence.rs`, but
  pass the scan-level service list in explicitly instead of reaching through the
  installation type.
- Keep status fallback truthful by reading the scan-level service list directly
  when no platform manager is inspected.

</specifics>

<deferred>
## Deferred Ideas

- Teaching the base detector to assign service ownership eagerly instead of
  leaving ownership association to migration consumers.
- Adding mutation-capable migration apply behavior or automatic source-service
  cutover.
- Redesigning service-manager uncertainty semantics beyond what this ownership
  cleanup needs.

</deferred>

---

*Phase: 34-migration-detection-ownership-model-cleanup*
*Context gathered: 2026-04-30*
