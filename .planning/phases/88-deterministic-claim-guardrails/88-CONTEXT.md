---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 88-2026-06-23T20-39-38
generated_at: 2026-06-23T20:39:39.056Z
---

# Phase 88: Deterministic Claim Guardrails - Context

**Gathered:** 2026-06-23
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 88 adds deterministic default verification that prevents overbroad
production full-node readiness claims and deferred-surface promotions while
keeping public-network, real service-manager, and multi-day checks opt-in UAT.
This phase closes REL-02, REL-03, and REL-04 without widening the v1.8 support
or production-readiness claim.

</domain>

<decisions>
## Implementation Decisions

### Claim Scan Boundary

- **D-01:** Add a new Phase 88 Bun checker rather than extending the completed
  Phase 87 release-readiness checker. Phase 87 remains the checklist gate;
  Phase 88 owns broad deterministic claim guardrails.
- **D-02:** Use a curated release and public-operator documentation surface
  instead of a whole-docs-tree scan. Include current release roots, parity
  entrypoints, the runtime guide, and relevant parity catalog pages where
  release or operator claims are likely to be read.
- **D-03:** Do not scan `.planning/` histories, milestone archives, or every
  historical doc as a blocking default verifier surface. Historical scoped
  claims must stay discoverable without becoming false positives.
- **D-04:** Allow production-readiness and deferred-surface terms only when the
  surrounding sentence, paragraph, or table row is explicitly negative or
  scoped, such as `does not claim`, `not allowed yet`, `deferred`,
  `unsupported`, `historical`, `opt-in UAT`, `future gate`, or `outside default
  verification`.

### Evidence Gate Semantics

- **D-05:** Define production full-node readiness as disallowed for v1.8 unless
  a future milestone satisfies every required evidence gate. The only allowed
  v1.8 claim remains that Open Bitcoin defines the gates required before a
  future production full-node readiness claim.
- **D-06:** Treat existing docs as the source of truth for gates:
  `docs/parity/production-claim-boundary.md`,
  `docs/parity/support-matrix.md`, and
  `docs/parity/release-readiness.md`. Do not introduce a separate
  machine-readable v1.8 evidence manifest in this phase.
- **D-07:** A deferred-surface promotion is valid only after a future scoped
  phase names concrete evidence, a verifier or opt-in UAT command, residual
  risk, and next-gate status. Prose-only promotion must fail deterministic
  verification.
- **D-08:** Field-based evidence and named verifier roots matter; artifact
  existence, daemon startup, elapsed time, peer reachability, raw log tail,
  service file existence, support bundle path, or context-only records are not
  sufficient by themselves.

### Deferred-Surface Promotion Rules

- **D-09:** Fail positive promotion language for the Phase 82 deferred inventory,
  including inbound serving, address relay, block serving, transaction relay,
  compact block relay, production-funds wallet use or safety, migration apply
  mode, signed packaging or package-manager distribution, Windows service
  integration, hosted dashboards, GUI parity, public-network default checks,
  public-network CI, release-blocking live sync, automatic support-bundle
  upload, destructive repair, and broad production-node readiness.
- **D-10:** Cover promotion predicates such as `production-ready`,
  `production-grade`, `fully supported`, `default-verified`,
  `release-blocking`, `proven`, `GA`, `certified`, and close variants when
  attached to deferred production-adjacent surfaces.
- **D-11:** Keep exact bad-phrase denylist checks as supplemental smoke coverage,
  but do not rely only on exact strings. The checker should combine curated
  phrase matching with scoped allow rules to catch obvious paraphrases without
  blocking valid no-claim text.

### Verifier Integration And Regression Tests

- **D-12:** Add `scripts/check-phase88-deterministic-claim-guardrails.ts` and
  `scripts/check-phase88-deterministic-claim-guardrails.test.ts`, following the
  Phase 82 through Phase 87 Bun checker and fixture-test pattern.
- **D-13:** Use an `OPEN_BITCOIN_PHASE88_REPO_ROOT` override in fixture tests so
  bad release-doc and verifier wiring cases can be tested in temporary repos.
- **D-14:** Wire both `bun test scripts/check-phase88-deterministic-claim-guardrails.test.ts`
  and `bun run scripts/check-phase88-deterministic-claim-guardrails.ts` into
  `scripts/verify.sh` immediately after the Phase 87 checker, both in the
  visible command-order heredoc and the executed `run_step` sequence.
- **D-15:** Strip the verifier command-order heredoc before checking the
  executable verifier text. A command that exists only in the heredoc must not
  satisfy Phase 88 verifier wiring.
- **D-16:** The default verifier must remain deterministic, short-running,
  public-network-free, real-service-manager-free, and multi-day-free. Fail
  verifier drift that adds commands or text such as `run-live-mainnet-smoke`,
  `systemctl`, `launchctl`, long `sleep` gates, `--restart-after-progress`,
  package-manager service commands, public-network CI/default gates,
  release-blocking live sync, automatic support upload, destructive repair, or
  broad production-node readiness.

### Folded Todos

No pending todos matched Phase 88.

### the agent's Discretion

- The planner may split the phase into checker implementation, fixture tests,
  docs/parity root updates, verifier wiring, and closeout verification.
- The executor may factor small shared helpers inside the Phase 88 checker if
  that keeps false-positive handling clear, but should avoid a broad shared
  rules registry unless it materially reduces duplication without creating a
  second source of truth.
- No Rust source changes are expected. If planning discovers a narrow Rust gap,
  update `docs/parity/source-breadcrumbs.json` for any new first-party Rust
  source or test files under `packages/open-bitcoin-*/src` or
  `packages/open-bitcoin-*/tests`.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope And Requirements

- `.planning/ROADMAP.md` — Phase 88 goal, dependency on Phase 87, and success
  criteria.
- `.planning/REQUIREMENTS.md` — REL-02, REL-03, and REL-04 ownership for Phase
  88.
- `.planning/STATE.md` — Current v1.8 status and default-verifier boundary
  reminders.

### v1.8 Claim Boundary Sources

- `docs/parity/production-claim-boundary.md` — Support-term vocabulary,
  allowed production-related statement, deferred-surface inventory, and future
  production-readiness gates.
- `docs/parity/support-matrix.md` — Support matrix rows, contributor update
  rules, evidence basis, residual risk, and next-gate expectations.
- `docs/parity/release-readiness.md` — v1.8 release-readiness checklist,
  no-claim review, Phase 88 boundary note, and context-only evidence signals.
- `docs/parity/deviations-and-unknowns.md` — Durable deferred-surface register
  and current no-claim boundary.
- `docs/parity/index.json` — Parity surface and audit roots that must remain
  coherent with Phase 88 evidence.
- `docs/parity/checklist.md` — Human-readable parity checklist surface roots.
- `docs/parity/README.md` — Contributor parity entrypoint and release-readiness
  pointers.
- `README.md` — Public project entrypoint that must not overstate production
  full-node readiness.
- `docs/operator/runtime-guide.md` — Operator-facing docs that must preserve
  opt-in UAT and no-production-claim boundaries.
- `docs/parity/catalog/operator-runtime-release-hardening.md` — Cross-phase
  operator/runtime/release-hardening catalog row for Phase 88.

### Existing Checker Patterns

- `scripts/check-phase82-production-claim-boundary.ts` — Prior production
  claim-boundary checker pattern, deferred-surface vocabulary, exact overclaim
  checks, repo-root override style, and verifier boundary checks.
- `scripts/check-phase82-production-claim-boundary.test.ts` — Fixture-test
  pattern for claim-boundary regressions.
- `scripts/check-phase87-release-readiness.ts` — Release-readiness checker
  pattern, no-claim vocabulary, context-only evidence checks, executable
  verifier text check, and Phase 88 handoff.
- `scripts/check-phase87-release-readiness.test.ts` — Fixture-test pattern for
  release-readiness and verifier wiring failures.
- `scripts/verify.sh` — Repo-native default verification contract and checker
  execution order.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- Phase 82 through Phase 87 Bun checker/test files provide the local automation
  pattern: exported checker function, environment-variable repo root override,
  `readText` helpers, normalized string checks, parity JSON parsing, fixture
  temp roots, and `import.meta.main` CLI execution.
- `scripts/verify.sh` already carries both a visible `VERIFY_COMMAND_ORDER`
  heredoc and executed `run_step` sequence. Phase 88 should update both and
  validate the executed sequence.

### Established Patterns

- Repo-owned automation uses Bun/TypeScript, not Python.
- Release-boundary checkers are deterministic local scans over tracked docs,
  JSON, and scripts.
- Default verification must avoid live public-network checks, real service
  managers, package-manager service commands, and multi-day timing gates.
- Existing checkers use exact constants for required docs, support terms,
  deferred surfaces, forbidden verifier strings, and required command order.

### Integration Points

- `scripts/verify.sh` must run the Phase 88 test and checker after Phase 87.
- `docs/parity/index.json`, `docs/parity/checklist.md`, and
  `docs/parity/catalog/operator-runtime-release-hardening.md` should record a
  Phase 88 parity surface if implementation follows prior phase patterns.
- `docs/metrics/lines-of-code.md` may change when verification regenerates LOC
  metrics after adding checker/test files.

</code_context>

<specifics>
## Specific Ideas

- Prefer a hybrid checker: parse canonical rows where practical, then scan the
  curated release/operator corpus for positive claim sentences with scoped
  allow rules.
- Avoid a new evidence registry in this phase. Existing canonical docs remain
  authoritative.
- Include fixture regressions for heredoc-only verifier wiring, explicit
  production full-node readiness claims, deferred-surface promotion prose, valid
  no-claim/deferred wording, and forbidden default-verifier drift.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 88-deterministic-claim-guardrails*
*Context gathered: 2026-06-23*
