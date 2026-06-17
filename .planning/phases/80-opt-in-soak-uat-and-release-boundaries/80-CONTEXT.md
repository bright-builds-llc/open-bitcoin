---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 80-2026-06-17T22-54-57
generated_at: 2026-06-17T22:54:57.119Z
---

# Phase 80: Opt-In Soak UAT and Release Boundaries - Context

**Gathered:** 2026-06-17
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 80 closes the v1.7 milestone by keeping default verification deterministic,
documenting explicit opt-in long-run operator UAT commands, and making the
scoped v1.7 claim auditable. The phase should not add production-node readiness,
inbound serving, relay, production-funds wallet safety, migration apply mode,
packaging, GUI, hosted dashboards, public-network default checks, or multi-day
default gates.

</domain>

<decisions>
## Implementation Decisions

### Default Verification Boundary

- **D-01:** Add a focused Phase 80 deterministic boundary checker in the
  existing Bun/TypeScript style, with fixture tests, and wire both into
  `bash scripts/verify.sh`.
- **D-02:** The checker must prove the default verification path remains local,
  short-running, public-network-free, real-service-manager-free,
  multi-day-sleep-free, current-tip-timing-free, and free of large-disk
  allocation requirements.
- **D-03:** Guard `scripts/verify.sh` against accidental default invocation of
  live-mainnet smoke, manual peers, `--restart-after-progress`, real
  `systemctl` or `launchctl`, `-openbitcoinsync=mainnet-ibd`, multi-day sleeps,
  current-tip/release-blocking timing gates, `/proc` or `lsof` process scans,
  and large-disk stress paths.
- **D-04:** Do not add a runtime sandbox, hermetic container, or strict offline
  dependency mode as the Phase 80 proof. Those are future CI/release-engineering
  choices, not required for this closeout.

### Opt-In UAT Command Matrix

- **D-05:** Add a focused Phase 80 v1.7 UAT matrix instead of a broad v1.6-style
  scenario sweep. The matrix should cover exactly these operator workflows:
  multi-day soak lifecycle, bounded recovery drill, support-bundle generation,
  and post-failure diagnosis.
- **D-06:** Each CLI-backed workflow must provide copy-pasteable repo-local Cargo
  and Bazel command forms. Prefer
  `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`
  and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`.
- **D-07:** The matrix may reference existing Phase 75 soak lifecycle commands,
  Phase 77 recovery/status guidance, Phase 79 support-forensics commands, and
  deterministic fixture checks, but it should be one reviewer-friendly Phase 80
  entrypoint.
- **D-08:** The UAT wording must state what each workflow can prove and what it
  does not prove. Artifact existence, daemon startup, elapsed time, peer
  reachability, raw logs, stale reports, and support-bundle presence are not
  enough to prove soak stability or production readiness.

### Parity And Audit Closure

- **D-09:** Use one Phase 80 closure checker rather than a new manifest-driven
  evidence registry. The checker should require v1.7 evidence roots across
  docs, parity files, checkers, support schema anchors, and `scripts/verify.sh`
  ordering.
- **D-10:** Keep `docs/parity/source-breadcrumbs.json` and
  `scripts/check-parity-breadcrumbs.ts --check` as the required mechanism for
  new first-party Rust source or test files under `packages/open-bitcoin-*/src`
  or `packages/open-bitcoin-*/tests`.
- **D-11:** If Phase 80 adds Rust source or tests, add the required parity
  breadcrumb mapping and keep the breadcrumb checker green. If the phase stays
  docs and Bun checker only, no breadcrumb mapping change should be needed.
- **D-12:** The closure checker should assert that Phase 75 through Phase 79
  deterministic checkers remain wired before the Phase 80 checker and that the
  v1.7 roots mention VER-05, VER-06, VER-07, and REL-04.

### Release Boundary Wording

- **D-13:** Use a parity-rooted v1.7 boundary closeout. Update and guard
  README, `docs/operator/runtime-guide.md`, `docs/parity/release-readiness.md`,
  `docs/parity/README.md`, `docs/parity/checklist.md`,
  `docs/parity/index.json`, `docs/parity/deviations-and-unknowns.md`, and
  `docs/parity/catalog/operator-runtime-release-hardening.md` as needed.
- **D-14:** The v1.7 claim shape is explicit opt-in full-sync soak and recovery
  hardening: durable multi-day soak evidence, resource bounds, recovery
  diagnosis, progress guarantees, stall diagnosis, support-bundle forensics,
  opt-in UAT commands, and deterministic release-boundary checks.
- **D-15:** Preserve the non-claim list wherever Phase 80 touches docs, parity
  roots, checker constants, or status wording: inbound serving, address relay,
  block serving, transaction relay, compact block relay, production-funds wallet
  use, migration apply mode, signed packaging, Windows service support, GUI,
  hosted dashboards, public-network default checks, public-network CI,
  release-blocking live sync, automatic support-bundle upload, destructive
  repair, and broad production-node readiness.
- **D-16:** Add targeted status/output guards only where exact operator text is
  claim-bearing. Avoid broad text scans that would make legitimate historical
  non-claim wording brittle.

### Folded Todos

No pending todos matched Phase 80.

### the agent's Discretion

- The planner may split Phase 80 into UAT command documentation, parity/release
  root refresh, deterministic checker/test wiring, and final verification
  evidence.
- The executor may keep Phase 80 primarily in docs and Bun checker code if no
  source behavior gap is found.
- The executor may reuse the Phase 79 checker/test structure with
  v1.7-specific requirement ids, evidence paths, forbidden default-verification
  strings, and non-claim terms.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 80 goal, dependency on Phase 79, requirements,
  and success criteria.
- `.planning/REQUIREMENTS.md` - VER-05, VER-06, VER-07, REL-04, v1.7 traceability,
  and out-of-scope production surfaces.
- `.planning/PROJECT.md` - v1.7 milestone goal, current state, pinned Knots
  baseline, functional-core boundary, and production-claim limits.
- `.planning/STATE.md` - current focus and accumulated v1.7 decisions.
- `AGENTS.md` - repo-local verification, UAT command, parity breadcrumb, GSD,
  Rust, and generated artifact rules.
- `AGENTS.bright-builds.md` - Bright Builds workflow and standards routing.
- `standards-overrides.md` - current local standards override registry.
- `standards/core/architecture.md` - functional-core, parse-at-boundaries, and
  illegal-state modeling rules.
- `standards/core/code-shape.md` - early-return, optional naming, script, and
  file-size guidance.
- `standards/core/verification.md` - sync-first and repo-native verification
  requirements.
- `standards/core/testing.md` - unit-test and Arrange/Act/Assert expectations.
- `standards/languages/rust.md` - Rust module, option naming, invariant, and
  verification guidance.
- `standards/languages/typescript-javascript.md` - Bun/TS automation and
  nullish naming guidance.

### Prior Phase Decisions

- `.planning/phases/73-opt-in-uat-and-deterministic-verification/73-CONTEXT.md`
  - opt-in UAT command matrix, deterministic checker pattern, parity
  breadcrumbs, and default-verification exclusions.
- `.planning/phases/74-release-boundaries-parity-and-documentation/74-CONTEXT.md`
  - v1.6 release-claim shape, non-claim list, release-boundary checker posture,
  and final traceability.
- `.planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md`
  - `open-bitcoin soak`, durable run ledger, checkpoint/report projection, soak
  outcome taxonomy, and support summary projection.
- `.planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md`
  - resource-bound status surfaces, support-bundle size pressure,
  `resource_stop` semantics, and deterministic fixture policy.
- `.planning/phases/77-corruption-and-lock-recovery-hardening/77-CONTEXT.md`
  - recovery evidence, recovery-stop semantics, probe-only status/support
  boundaries, and deterministic recovery fixtures.
- `.planning/phases/78-progress-guarantees-and-stall-diagnosis/78-CONTEXT.md`
  - progress credit, stall diagnosis, peer contribution, resource/recovery
  precedence, and default-verification exclusions.
- `.planning/phases/79-diagnostics-and-support-bundle-forensics/79-CONTEXT.md`
  - support forensics, forensic timeline, checkpoint chain, failure narrative,
  redaction, size bounds, and cross-surface consistency.
- `.planning/phases/79-diagnostics-and-support-bundle-forensics/79-VERIFICATION.md`
  - passed Phase 79 evidence and readiness for Phase 80.

### Implementation And Verification Surfaces

- `scripts/verify.sh` - repo-native deterministic verification contract and
  ordered checker wiring.
- `scripts/check-phase73-uat-verification.ts` and
  `scripts/check-phase73-uat-verification.test.ts` - UAT matrix and deterministic
  verification checker pattern.
- `scripts/check-v1.6-release-boundaries.ts` - v1.6 release-boundary checker
  pattern.
- `scripts/check-phase75-soak-runner.ts` and
  `scripts/check-phase75-soak-runner.test.ts` - soak command, report, parity,
  and default-verification guard pattern.
- `scripts/check-phase76-resource-bounds.ts` and
  `scripts/check-phase76-resource-bounds.test.ts` - resource-bound checker
  pattern.
- `scripts/check-phase77-corruption-lock-recovery.ts` and
  `scripts/check-phase77-corruption-lock-recovery.test.ts` - recovery checker
  pattern.
- `scripts/check-phase78-progress-guarantees.ts` and
  `scripts/check-phase78-progress-guarantees.test.ts` - progress/stall checker
  pattern.
- `scripts/check-phase79-diagnostics-support-bundle.ts` and
  `scripts/check-phase79-diagnostics-support-bundle.test.ts` - closest checker
  and fixture-test pattern for Phase 80.
- `scripts/check-parity-breadcrumbs.ts` and
  `docs/parity/source-breadcrumbs.json` - required breadcrumb mechanism for new
  first-party Rust source or test files.
- `scripts/run-live-mainnet-smoke.ts` and
  `scripts/test-run-live-mainnet-smoke.sh` - opt-in public-network wrapper and
  deterministic fixture validation that must stay outside default verification.

### Operator, Release, And Parity Docs

- `README.md` - contributor/operator entrypoint that must reflect the current
  v1.7 review posture without production readiness claims.
- `docs/operator/runtime-guide.md` - authoritative operator workflow docs and
  natural location for the Phase 80 v1.7 UAT matrix.
- `docs/architecture/status-snapshot.md` - shared status contract and field
  interpretation.
- `docs/architecture/operator-observability.md` - metrics/log/support evidence,
  retention, compact snapshots, and deterministic verification boundaries.
- `docs/architecture/storage-decision.md` - durable storage and recovery
  posture.
- `docs/parity/release-readiness.md` - release-readiness matrix that currently
  needs v1.7 closeout.
- `docs/parity/index.json` - machine-readable parity root requiring Phase 80 and
  v1.7 evidence discoverability.
- `docs/parity/checklist.md` - human-readable parity root requiring Phase 80 and
  v1.7 evidence discoverability.
- `docs/parity/README.md` - parity entrypoint requiring current v1.7 closeout
  links.
- `docs/parity/deviations-and-unknowns.md` - deferred-surface and known-risk
  register requiring v1.7 refresh.
- `docs/parity/catalog/p2p.md` - P2P and public-mainnet boundary catalog.
- `docs/parity/catalog/chainstate.md` - active-chain, UTXO/undo, reorg, and
  persistence parity scope.
- `docs/parity/catalog/operator-runtime-release-hardening.md` - operator runtime
  and evidence boundary catalog.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `scripts/check-phase79-diagnostics-support-bundle.ts` and
  `scripts/check-phase79-diagnostics-support-bundle.test.ts`: nearest checker
  and fixture-test shape for a Phase 80 release/audit closeout.
- `scripts/verify.sh`: already runs deterministic checkers through Phase 79 and
  is the required Phase 80 wiring target.
- `docs/operator/runtime-guide.md`: already contains Phase 75 soak, Phase 77
  recovery, Phase 79 support-forensics, support bundle, and known-limitation
  sections that Phase 80 can consolidate into one v1.7 UAT matrix.
- `docs/parity/catalog/operator-runtime-release-hardening.md`: already records
  Phase 75 through Phase 79 rows and is the natural place for a Phase 80/v1.7
  closeout row.
- `docs/parity/index.json`, `docs/parity/checklist.md`, and
  `docs/parity/README.md`: existing machine and human roots for phase evidence.

### Established Patterns

- Recent milestone closeouts use Bun/TypeScript checkers plus checker tests,
  then wire them into `scripts/verify.sh`.
- Operator docs use repo-local Cargo and Bazel command forms for CLI-backed
  UAT workflows.
- Public-network, service-manager, multi-day, large-disk, and current-tip
  checks remain opt-in UAT and outside default verification.
- Parity roots document exact scoped claims and explicit non-claims instead of
  broad production readiness.

### Integration Points

- Add the Phase 80 checker/test next to existing `scripts/check-phase*.ts`
  files.
- Wire the checker test and checker immediately after the Phase 79 checker in
  `scripts/verify.sh`.
- Refresh v1.7 release wording in README, runtime guide, release readiness, and
  parity roots.
- Update source breadcrumbs only if new first-party Rust source or tests are
  added.

</code_context>

<specifics>
## Specific Ideas

- Prefer one focused Phase 80 v1.7 UAT matrix over scattered command updates.
- Prefer checker constants and fixture tests over a new evidence manifest.
- Keep generated live-smoke reports, support bundles, daemon logs, metrics
  stores, compatibility reports, and local datadirs out of git.
- Treat support bundles and reports as evidence projections, not source-of-truth
  ledgers or production-readiness proof.

</specifics>

<deferred>
## Deferred Ideas

- Runtime sandboxing or containerized offline proof for `scripts/verify.sh`.
- Signed or externally comparable support/soak artifacts.
- A reusable v1.x evidence manifest system.
- Production-node expansion, inbound serving, relay, wallet production safety,
  migration apply mode, packaging, GUI, hosted dashboards, and public-network CI.

</deferred>

---

*Phase: 80-opt-in-soak-uat-and-release-boundaries*
*Context gathered: 2026-06-17*
