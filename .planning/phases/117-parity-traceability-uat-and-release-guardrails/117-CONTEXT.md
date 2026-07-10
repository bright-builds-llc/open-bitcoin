---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 117-2026-07-10T05-06-19
generated_at: 2026-07-10T05:11:57.640Z
---

# Phase 117: Parity Traceability, UAT, and Release Guardrails - Context

**Gathered:** 2026-07-10
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 117 closes v2.1 by making the bounded, explicit, default-off block-serving and compact-block relay claim auditable. It owns parity traceability, concrete Bitcoin Knots anchors, deterministic no-claim and verifier-boundary checks, contributor/operator/release documentation, and an opt-in UAT package over the implementation and evidence already delivered by Phases 110 through 116.

This is a closeout and guardrail phase. It must not add or change P2P behavior, activate public serving by default, broaden historical or archive-node behavior, add package relay or filter serving, introduce production-service or production-funds claims, or move public-network, wall-clock, service-manager, or deployment checks into the default verifier.

</domain>

<decisions>
## Implementation Decisions

### Parity Surface And Requirement Ownership

- **D-01:** Phase 117 canonically owns only `BOUND-01` through `BOUND-05`. Phases 110 through 116 remain the exactly-once owners of their implementation and operator-evidence requirements even when the Phase 117 checker aggregates their evidence.
- **D-02:** Backfill distinct machine-readable and human-review parity surfaces for Phases 112 through 116, then add one Phase 117 closeout surface for `BOUND-01` through `BOUND-05`. Preserve the existing Phase 110 and 111 surfaces instead of collapsing all v2.1 evidence into one entry.
- **D-03:** `docs/parity/index.json` remains the machine-readable root. `docs/parity/checklist.md`, `docs/parity/catalog/p2p.md`, and `docs/parity/release-readiness.md` remain the human review roots; do not create a competing closeout manifest.
- **D-04:** Parity evidence must cite concrete Knots anchors for full block serving, BIP152 wire messages, compact-block negotiation and reconstruction, missing-transaction round trips and fallback, validation handoff, peer state, and resource governance.
- **D-05:** Review `docs/parity/source-breadcrumbs.json` semantically, not only mechanically. Tighten or split broad or explicit-`none` groups when they hide a defensible v2.1 Knots anchor, while preserving `none` only where no honest source anchor exists.
- **D-06:** Intentional differences and deferred behavior remain explicit in parity docs. Planning artifacts and phase verification may point to canonical parity roots but must not become parallel sources of requirement ownership.

### Deterministic No-Claim Guardrails

- **D-07:** Add one focused Phase 117 Bun/TypeScript checker and paired mutation-style test file. Use Phase 106 as the primary closeout pattern and Phase 116 as the current block-relay evidence vocabulary.
- **D-08:** The checker uses a curated current claim-bearing corpus, exact evidence/anchor/requirement checks, and paragraph-aware claim classification. It must not scan historical `.planning/` prose or ban the phrase `compact block relay` globally.
- **D-09:** The checker must allow the truthful scoped claim: bounded, explicit, default-off, opt-in block serving and compact-block relay with deterministic local evidence.
- **D-10:** The checker must reject broader positive claims for package relay, BIP37 bloom-filter serving, compact-filter serving, public serving or relay defaults, archive-node or production-scale historical serving, public-network CI or release gates, production service/deployment, production full-node readiness, and production-funds wallet safety or use.
- **D-11:** Explicit deferred, unsupported, future-gated, no-claim, and opt-in-UAT wording remains valid. Negative fixtures must prove both rejection of overclaims and acceptance of scoped or deferred wording.
- **D-12:** The aggregate checker validates all 34 v2.1 requirements exactly once, required Phase 110 through 117 parity surfaces, concrete Knots anchors, relevant breadcrumb groups, exact repo-local UAT commands, and both visible and executable verifier ordering.

### Documentation And Release Handoff

- **D-13:** Refresh `README.md` with a concise current-state description and links to the canonical evidence roots. Contributor-facing copy stays quiet, factual, and evidence-focused.
- **D-14:** Use `docs/parity/release-readiness.md` as the v2.1 release-note and release-review handoff. The repository has no first-party release-notes tree, so do not introduce a disconnected changelog for this phase.
- **D-15:** Update the existing parity, production-claim, support, operator, runtime, status-snapshot, and observability docs so they distinguish the shipped bounded/default-off v2.1 capability from still-deferred public-default, archive, production-service, production-readiness, and production-funds claims.
- **D-16:** Preserve Phase 116's aggregate-only, low-cardinality, allowlisted, and redacted operator evidence. Phase 117 should assert those surfaces rather than rewrite runtime behavior unless deterministic closure exposes a real evidence leak.
- **D-17:** Do not require `packages/README.md` or create new documentation roots unless execution finds a material contributor-facing gap there.

### UAT And Default Verification

- **D-18:** Create `.planning/phases/117-parity-traceability-uat-and-release-guardrails/117-UAT.md` as the committed UAT package with explicit tests, expected results, evidence fields, and a clear distinction between deterministic closure and optional operator review.
- **D-19:** UAT guidance must provide copy-pasteable repo-local Cargo and Bazel forms for the relevant `open-bitcoin`, `open-bitcoind`, and `open-bitcoin-cli` workflows instead of relying on installed aliases.
- **D-20:** Public-network block-serving or compact-relay review remains optional evidence. It may be recorded as not run without failing phase verification and must never become a pre-commit, default CI, release-boundary, wall-clock soak, service-manager, or production-deployment gate.
- **D-21:** Wire the Phase 117 checker pair immediately after Phase 116 in both the visible `VERIFY_COMMAND_ORDER` block and executable `run_step` chain in `scripts/verify.sh`, before pure-core checks.
- **D-22:** `bash scripts/verify.sh` remains the final deterministic non-regression contract. Focused checker tests and commands are iteration aids, not alternate release criteria.

### Planning Metadata Reconciliation

- **D-23:** Reconcile stale current-milestone metadata only after evidence is current: Phase 116 and OBS-01 through OBS-05 are complete on disk, while `.planning/STATE.md`, `.planning/ROADMAP.md`, and `.planning/REQUIREMENTS.md` still contain pending/current wording.
- **D-24:** Preserve exactly-once requirement ownership while reconciling metadata. Use `gsd-tools.cjs` mutation commands for STATE/ROADMAP updates rather than direct edits where the CLI owns those changes.
- **D-25:** Phase 117 may close current-milestone traceability and release-boundary evidence, but it must not archive v2.1 or invent a milestone audit workflow; milestone completion remains a separate GSD step after this phase passes.

### the agent's Discretion

The planner may choose exact checker helper names, fixture organization, paragraph-classification implementation, the smallest honest breadcrumb-group splits, exact doc section placement, and whether optional UAT items are recorded as pending or not run. Prefer small pure TypeScript helpers with focused tests, targeted doc edits, and the existing Phase 106/116 patterns. Do not spend discretion on runtime behavior changes or broader claims.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Governing Scope And Standards

- `AGENTS.md` — Repo-local verification, UAT command, parity breadcrumb, and GSD workflow rules.
- `AGENTS.bright-builds.md` — Bright Builds workflow, architecture, testing, and verification defaults.
- `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/testing.md`, `standards/core/verification.md` — Functional boundaries, code shape, tests, and repo-native verification.
- `standards/languages/rust.md`, `standards/languages/typescript-javascript.md` — Rust and Bun/TypeScript rules for any touched code or checker files.
- `.planning/PROJECT.md`, `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, `.planning/STATE.md` — v2.1 scope, BOUND requirements, phase success criteria, and current metadata.

### Locked v2.1 Evidence

- `.planning/phases/110-block-serving-activation-and-eligibility-boundary/110-CONTEXT.md`
- `.planning/phases/111-full-block-serving-request-path/111-CONTEXT.md`
- `.planning/phases/112-bip152-wire-codec-and-message-semantics/112-CONTEXT.md`
- `.planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-CONTEXT.md`
- `.planning/phases/114-compact-block-reconstruction-from-mempool-state/114-CONTEXT.md`
- `.planning/phases/115-missing-transaction-round-trip-fallback-and-validation-handoff/115-CONTEXT.md`
- `.planning/phases/116-operator-evidence-metrics-logs-and-support-boundary/116-CONTEXT.md`
- `.planning/phases/116-operator-evidence-metrics-logs-and-support-boundary/116-SUMMARY.md`
- `.planning/phases/116-operator-evidence-metrics-logs-and-support-boundary/116-VERIFICATION.md`

### Closeout And Checker Precedents

- `.planning/phases/95-network-participation-evidence-and-release-boundary/95-CONTEXT.md`
- `.planning/phases/106-parity-traceability-uat-and-release-boundary-guardrails/106-CONTEXT.md`
- `.planning/phases/109-milestone-archive-readiness-metadata-closure/109-CONTEXT.md`
- `scripts/check-phase95-network-participation-release-boundary.ts`, `scripts/check-phase95-network-participation-release-boundary.test.ts`
- `scripts/check-phase106-parity-uat-release-boundary.ts`, `scripts/check-phase106-parity-uat-release-boundary.test.ts`
- `scripts/check-phase116-operator-block-relay-evidence.ts`, `scripts/check-phase116-operator-block-relay-evidence.test.ts`
- `scripts/check-parity-breadcrumbs.ts`, `scripts/verify.sh`

### Parity, Release, And Operator Roots

- `README.md`
- `docs/parity/README.md`, `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/source-breadcrumbs.json`
- `docs/parity/catalog/p2p.md`, `docs/parity/catalog/consensus-validation.md`, `docs/parity/catalog/rpc-cli-config.md`, `docs/parity/catalog/operator-runtime-release-hardening.md`, `docs/parity/catalog/verification-harnesses.md`
- `docs/parity/release-readiness.md`, `docs/parity/production-claim-boundary.md`, `docs/parity/deviations-and-unknowns.md`, `docs/parity/support-matrix.md`
- `docs/operator/runtime-guide.md`, `docs/architecture/status-snapshot.md`, `docs/architecture/operator-observability.md`

### Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/protocol.h`, `packages/bitcoin-knots/src/blockencodings.h`, `packages/bitcoin-knots/src/blockencodings.cpp` — BIP152 messages, short IDs, reconstruction, and missing-transaction payloads.
- `packages/bitcoin-knots/src/net_processing.cpp`, `packages/bitcoin-knots/src/net_processing.h` — serving policy, compact negotiation, reconstruction/fallback, request cleanup, and in-flight limits.
- `packages/bitcoin-knots/src/net.cpp`, `packages/bitcoin-knots/src/net.h`, `packages/bitcoin-knots/src/net_permissions.h`, `packages/bitcoin-knots/src/net_permissions.cpp` — peer state, permissions, connection/resource behavior, and high-bandwidth compact state.
- `packages/bitcoin-knots/src/validation.cpp`, `packages/bitcoin-knots/src/node/blockstorage.cpp` — validated block availability, validation handoff, and storage eligibility.
- `packages/bitcoin-knots/test/functional/p2p_getdata.py`, `packages/bitcoin-knots/test/functional/p2p_compactblocks.py`, `packages/bitcoin-knots/test/functional/p2p_permissions.py` — externally observable serving, compact relay, and permission behavior.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- Phase 106 checker/test: closest aggregate parity/UAT/release-boundary template with fixed-corpus, ownership, anchor, command, verifier-order, and overclaim checks.
- Phase 116 checker/test: current v2.1 block-relay vocabulary, cross-surface evidence, breadcrumb groups, redaction assertions, and command checks.
- `scripts/verify.sh`: two required integration points, the visible order block and executable `run_step` chain.
- Existing `docs/parity/index.json` and checklist surfaces for Phases 110 and 111 establish the v2.1 surface schema; Phases 112 through 116 remain to be backfilled.

### Established Patterns

- Closeout phases add one aggregate Bun checker/test pair rather than rewriting every completed phase checker.
- Claim checks use curated current docs and mutation fixtures, not history-wide scans.
- Machine-readable parity surfaces and the human checklist mirror exactly-once requirement ownership.
- Public-network review stays optional and untracked; default verification stays deterministic and local.

### Integration Points

- Extend v2.1 entries in `docs/parity/index.json`, `docs/parity/checklist.md`, and `docs/parity/catalog/p2p.md`.
- Tighten relevant groups in `docs/parity/source-breadcrumbs.json` where semantic anchors are missing.
- Add the Phase 117 checker pair and wire it after Phase 116 in `scripts/verify.sh`.
- Refresh README, release-readiness, production-boundary, runtime-guide, status-snapshot, and observability wording around the bounded v2.1 claim.
- Commit the Phase 117 UAT package and reconcile current-milestone metadata after verification evidence is current.

</code_context>

<specifics>
## Specific Ideas

- Preferred release wording: "bounded, explicit, default-off block serving and compact-block relay with deterministic local evidence and opt-in public-network review."
- Required operator forms include `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...` and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`, plus explicit daemon and RPC CLI forms where the UAT workflow needs them.
- A public-network UAT item may truthfully say "not run" without turning Phase 117 verification into a failure.

</specifics>

<deferred>
## Deferred Ideas

Package relay, BIP37 bloom-filter serving, compact-filter serving, public block-serving or relay defaults, archive-node behavior, production-scale historical serving, public-network CI, production service/deployment, production full-node readiness, production-funds wallet use, packaging, GUI/hosted dashboards, migration apply mode, destructive repair, and automatic support-bundle upload remain outside Phase 117.

</deferred>

***

*Phase: 117-parity-traceability-uat-and-release-guardrails*
*Context gathered: 2026-07-10*
