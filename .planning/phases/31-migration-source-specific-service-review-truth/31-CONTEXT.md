---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 31-2026-04-29T16-42-33
generated_at: 2026-04-29T16:42:33.523Z
---

# Phase 31: Migration Source-Specific Service Review Truth - Context

**Gathered:** 2026-04-29
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Repair the dry-run migration planner so service-cutover review actions stay tied
to the selected source installation instead of inheriting unrelated scan-wide
service candidates. The fix must preserve the existing read-only detection and
migration safety posture, keep custom-path `--source-datadir` selection working,
and fall back to explicit manual review when service ownership is ambiguous. It
does not add migration apply mode, mutate source services, rewrite source
datadirs, or broaden onboarding or status detection behavior outside what this
truthfulness repair needs.

</domain>

<decisions>
## Implementation Decisions

### Service Association Truth
- **D-01:** Keep migration planning dry-run only. Phase 31 repairs which source
  service definitions appear in the review output; it does not disable,
  replace, or mutate any source supervisor.
- **D-02:** Repair the truth gap at the migration service-review boundary, not
  by inventing a second migration scanner or widening unrelated operator
  surfaces.
- **D-03:** A managed service definition only counts as source-specific when its
  read-only contents point at the selected installation's datadir or config
  file. Service paths that cannot be tied to the selected source must not be
  rendered as though they were that source's cutover checklist.
- **D-04:** When service evidence exists but remains ambiguous or unreadable, the
  planner must degrade to an explicit manual-review action instead of showing an
  unrelated or guessed service path.
- **D-05:** Service definitions that clearly target a different installation
  should be excluded from the selected source summary, service action group, and
  service-surface deviation relevance.

### Scope And Safety
- **D-06:** Keep the existing config, datadir, cookie, wallet, and deviation
  surfaces unchanged unless the service-association repair requires a small,
  clearly named helper.
- **D-07:** The repair may parse only the service-definition arguments needed to
  identify source ownership, such as datadir or config path evidence. It must
  not leak raw service contents into operator output.

### Verification And Operator Truth
- **D-08:** Add focused regression coverage for both sides of the bug:
  source-specific service inclusion when the service definition points at the
  selected install, and manual-review fallback when scan-wide service evidence
  is ambiguous.
- **D-09:** Keep operator-facing migration docs aligned if the repair changes the
  truthful description of how `migrate plan --source-datadir` scopes service
  review.

### Claude's Discretion
- Helper placement is flexible as long as the association logic stays small,
  readable, and obviously migration-scoped.
- The repair may use datadir or config-path matching, or both, provided the
  selected-source behavior stays truthful and conservative.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope and audit evidence
- `.planning/ROADMAP.md` - Phase 31 goal, dependencies, requirements, and
  success criteria.
- `.planning/REQUIREMENTS.md` - `MIG-02` and `MIG-04` traceability for this
  blocker phase.
- `.planning/PROJECT.md` - v1.1 dry-run-first migration posture and
  operator-surface truthfulness.
- `.planning/v1.1-MILESTONE-AUDIT-PHASE-29.md` - blocker `INT-v1.1-03` and
  broken flow `FLOW-v1.1-03`, including the exact evidence for the scan-wide
  service-candidate drift.
- `.planning/phases/21-drop-in-parity-audit-and-migration/21-CONTEXT.md` -
  original migration safety and dry-run design decisions.
- `.planning/phases/25-migration-source-selection-hardening/25-CONTEXT.md` -
  explicit custom-path source selection boundary that Phase 31 must preserve.

### Migration and service implementation
- `docs/operator/runtime-guide.md` - current operator-facing migration guidance.
- `docs/parity/catalog/drop-in-audit-and-migration.md` - parity narrative for
  migration services, read-only source evidence, and manual cutover review.
- `packages/open-bitcoin-cli/src/operator/detect.rs` - current installation and
  service-candidate detection model.
- `packages/open-bitcoin-cli/src/operator/migration/planning.rs` - current
  source selection, service action group, and deviation relevance logic.
- `packages/open-bitcoin-cli/src/operator/migration/tests.rs` - focused planner
  tests.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` - end-to-end migration
  regressions.
- `packages/open-bitcoin-cli/src/operator/service/systemd.rs` - service unit
  shape and systemd argument conventions relevant to read-only association.
- `packages/open-bitcoin-cli/src/operator/service/launchd.rs` - launchd plist
  argument shape relevant to read-only association.
- `packages/bitcoin-knots/contrib/init/bitcoind.service` - upstream systemd
  service reference shape.
- `packages/bitcoin-knots/contrib/init/org.bitcoin.bitcoind.plist` - upstream
  launchd service reference shape.

### Repo workflow and standards
- `AGENTS.md` - repo-local verification contract, README/doc freshness guidance,
  and GSD workflow enforcement.
- `AGENTS.bright-builds.md` - sync-first, repo-native verification, and thin
  adapter guidance.
- `standards-overrides.md` - local standards exception ledger (currently no
  substantive overrides recorded).
- [Bright Builds `standards/core/architecture.md`](https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/architecture.md)
  - functional-core / imperative-shell and domain-type guidance.
- [Bright Builds `standards/core/code-shape.md`](https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/code-shape.md)
  - early returns, `maybe` naming, and file-size guardrails.
- [Bright Builds `standards/core/verification.md`](https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/verification.md)
  - sync-first and repo-native verification rules.
- [Bright Builds `standards/core/testing.md`](https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/core/testing.md)
  - focused AAA unit-test expectations.
- [Bright Builds `standards/languages/rust.md`](https://raw.githubusercontent.com/bright-builds-llc/bright-builds-rules/05f8d7a6c9c2e157ec4f922a05273e72dab97676/standards/languages/rust.md)
  - Rust-specific `let...else`, `maybe_`, and module guidance.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `select_source_installation()` already owns the selected-source versus
  manual-review decision boundary for migration planning.
- `service_action_group()` already has the right place to surface either
  concrete review paths or explicit manual-review instructions.
- The existing service adapters already establish the argument shapes that
  generated systemd and launchd service definitions use for datadir and config
  paths.
- The operator binary migration tests already prove the custom-path
  `--source-datadir` flow end to end and can be tightened to catch this service
  truth defect.

### Established Patterns
- Migration planning prefers conservative, support-oriented manual steps when
  source evidence is incomplete.
- Operator-readable output should mention paths and ownership review, not dump
  raw service or cookie contents.
- Rust helpers should stay small, use early returns, and isolate filesystem
  reads to obvious read-only boundaries.

### Integration Points
- The current regression originates where `planning.rs` treats
  `installation.service_candidates` as source-specific after the detector cloned
  the full scan-wide list into every installation.
- The narrowest honest repair is to filter or associate those candidates before
  the selected-source summary, service actions, and service-surface deviation
  logic consume them.
- Focused tests should cover both unit-level planner behavior and the
  end-to-end `open-bitcoin migrate plan --source-datadir` operator flow.

</code_context>

<specifics>
## Specific Ideas

- Read only the service-definition arguments needed to identify ownership:
  `--datadir`, `-datadir=...`, `--conf`, or `-conf=...` in systemd or launchd
  service files.
- Treat service definitions with no usable ownership evidence as ambiguous for
  migration review, not as proof that they belong to every detected install.
- Keep custom-path explicit source selection intact: the selected install should
  still be chosen when its config, cookie, or wallet evidence is valid even if
  the service-cutover checklist falls back to manual review.

</specifics>

<deferred>
## Deferred Ideas

- A broader redesign of the detection data model so service ownership is
  materialized directly in `DetectedInstallation`.
- Migration apply mode, automatic source-service cutover, or source datadir
  mutation.
- New service-manager support beyond the current launchd and systemd evidence
  paths.

</deferred>

---

*Phase: 31-migration-source-specific-service-review-truth*
*Context gathered: 2026-04-29*
