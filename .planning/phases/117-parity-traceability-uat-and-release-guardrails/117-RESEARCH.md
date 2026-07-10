---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 117-2026-07-10T05-06-19
generated_at: 2026-07-10T05:19:46.732Z
---

# Phase 117 Research: Parity Traceability, UAT, and Release Guardrails

## Research Summary

Phase 117 should follow the repository's established evidence-closeout pattern: backfill missing parity surfaces, add one aggregate Bun/TypeScript checker with mutation fixtures, refresh existing claim-bearing documentation, commit a deterministic UAT package, and leave public-network review optional. No runtime or P2P behavior is needed.

The nearest precedent is Phase 106, with Phase 116 providing the current v2.1 block-relay vocabulary. The main semantic difference is that compact-block relay is no longer wholly deferred: Phase 117 must allow the bounded/default-off v2.1 claim while continuing to reject public-default, archive, filter, package-relay, production-service, production-readiness, and production-funds claims.

## Standard Stack And Patterns

- Use Bun and TypeScript for substantial checker logic. Do not add Python or dependencies.
- Keep checker decision logic as pure data-in/data-out helpers and the filesystem/CLI entrypoint as a thin shell.
- Pair the checker with focused `bun:test` mutation fixtures using Arrange, Act, Assert.
- Use a curated current-doc corpus. Do not scan historical `.planning/` prose or the whole repository.
- Keep `docs/parity/index.json` as the machine root and mirror its exactly-once requirement ownership in `docs/parity/checklist.md`.
- Wire the checker test and execution commands in both the visible `VERIFY_COMMAND_ORDER` block and executable `run_step` chain in `scripts/verify.sh`.
- Treat `bash scripts/verify.sh` as the final verification contract.

## Codebase Findings

### Current Parity Gap

- `docs/parity/index.json` and `docs/parity/checklist.md` contain v2.1 surfaces for Phases 110 and 111, but not for Phases 112 through 116 or the Phase 117 closeout.
- `docs/parity/catalog/p2p.md` contains implementation-era Phase 110/111 sections and extensive v2.0 closeout precedents; it needs a v2.1 BIP152/reconstruction/operator-evidence rollup.
- `docs/parity/source-breadcrumbs.json` already has groups for block serving, BIP152 codecs, compact peer state, reconstruction, compact download, node status, RPC, CLI, dashboard, metrics/logs, and support. Some node/operator groups are broad or use explicit `none`; BOUND-01 requires a semantic review of those anchors.

### Checker Precedents

- `scripts/check-phase106-parity-uat-release-boundary.ts` is the primary template for fixed corpus loading, index parsing, exactly-once requirement ownership, Knots anchors, exact Cargo/Bazel commands, verifier-order checks, and paragraph-aware overclaim detection.
- `scripts/check-phase106-parity-uat-release-boundary.test.ts` demonstrates mutation fixtures for missing requirements, anchors, commands, wiring, and positive claims.
- `scripts/check-phase116-operator-block-relay-evidence.ts` and its test provide the current block-relay field, label, redaction, command, and claim vocabulary.
- `scripts/check-phase95-network-participation-release-boundary.ts` and Phase 98/88 checkers provide supporting aggregate-closeout and traceability patterns.

### Documentation Gaps

- `README.md` still describes compact-block relay as deferred from the v2.0 point of view and needs a concise v2.1 current-state update.
- `docs/parity/production-claim-boundary.md` and `docs/parity/deviations-and-unknowns.md` need to distinguish bounded/default-off v2.1 capability from still-deferred public defaults, archive behavior, production service/readiness, and production-funds use.
- `docs/parity/release-readiness.md` is the established release-facing handoff; the repo has no first-party release-notes tree.
- `docs/operator/runtime-guide.md` already contains Phase 116 evidence commands and is the correct home for Phase 117 UAT guidance.
- `docs/architecture/status-snapshot.md` and `docs/architecture/operator-observability.md` already describe the shared operator evidence model and need only scoped v2.1 closeout wording.

### Metadata Drift

- `.planning/phases/116-operator-evidence-metrics-logs-and-support-boundary/116-VERIFICATION.md` reports `passed`, but `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, and `.planning/STATE.md` still contain pending/current Phase 116 wording.
- Reconcile this only after evidence is current, preserve exactly-once requirement ownership, and use `gsd-tools.cjs` mutation commands for STATE/ROADMAP-owned changes.
- Do not archive v2.1 during Phase 117; milestone completion is a separate workflow.

## Canonical Knots Anchors

### Block Serving And Storage Eligibility

- `packages/bitcoin-knots/src/net_processing.cpp`
- `packages/bitcoin-knots/src/node/blockstorage.cpp`
- `packages/bitcoin-knots/src/validation.cpp`
- `packages/bitcoin-knots/src/net_permissions.h`
- `packages/bitcoin-knots/src/net_permissions.cpp`
- `packages/bitcoin-knots/test/functional/p2p_getdata.py`
- `packages/bitcoin-knots/test/functional/p2p_permissions.py`

### BIP152 And Reconstruction

- `packages/bitcoin-knots/src/protocol.h`
- `packages/bitcoin-knots/src/blockencodings.h`
- `packages/bitcoin-knots/src/blockencodings.cpp`
- `packages/bitcoin-knots/src/net_processing.cpp`
- `packages/bitcoin-knots/src/net_processing.h`
- `packages/bitcoin-knots/src/net.h`
- `packages/bitcoin-knots/test/functional/p2p_compactblocks.py`
- `packages/bitcoin-knots/test/functional/test_framework/messages.py`

## Recommended Four-Plan Structure

### 117-01: Parity Roots, Breadcrumbs, And Knots Anchor Index

- Add separate Phase 112 through 116 parity surfaces plus the Phase 117 BOUND surface.
- Update checklist and P2P/validation/operator parity narratives.
- Review and tighten breadcrumb groups with concrete Knots anchors.
- Verify all 34 requirements have exactly one owner.

### 117-02: Deterministic No-Claim And Verifier-Boundary Checker

- Add `scripts/check-phase117-parity-uat-release-boundary.ts` and `.test.ts`.
- Validate parity surfaces, requirement ownership, anchors, breadcrumb groups, required docs/UAT commands, allowed scoped wording, forbidden overclaims, and verifier ordering.
- Wire the pair after Phase 116 in both verifier-order locations.

### 117-03: README, Operator Docs, Runtime Docs, And Release Handoff

- Refresh README, runtime guide, status/observability docs, release-readiness, production boundary, support matrix, and deviations language.
- Use one consistent bounded/default-off v2.1 claim and explicit deferred surfaces.
- Keep `docs/parity/release-readiness.md` as the release-note handoff.

### 117-04: UAT Package And Milestone Release-Boundary Closure

- Create `117-UAT.md` with deterministic tests, exact Cargo/Bazel operator commands, optional public-network review, expected/result/evidence fields, and no-gaps closure.
- Reconcile Phase 116/117 current-milestone status after evidence is current.
- Run focused checkers, metadata validation, `git diff --check`, and full `bash scripts/verify.sh`.

## Threat Model

| Threat | Severity | Mitigation |
| --- | --- | --- |
| Scoped compact-relay text is rejected as an overclaim | Medium | Paragraph-aware classification and positive fixtures for bounded/default-off/opt-in wording. |
| Overclaims pass because a paragraph contains a weak qualifier such as `bounded` | High | Match forbidden capability and claim-strength pairs, require explicit default-off/opt-in/deferred context, and add mutation fixtures. |
| Mechanical breadcrumb coverage hides irrelevant or missing Knots anchors | High | Validate required groups and concrete path anchors; review broad and explicit-`none` groups semantically. |
| Public-network commands accidentally enter default verification | High | Check executable `run_step` commands and reject public-network, soak, service-manager, and deployment gates. |
| Requirement ownership is duplicated while checker coverage is aggregated | Medium | Parse all v2.1 surfaces and require each of 34 IDs exactly once. |
| Support or operator docs imply raw peer/transaction evidence | Medium | Require Phase 116 aggregate/redacted evidence roots and scan claim-bearing docs for raw-detail promises. |

No new secrets, network listeners, storage mutation, or runtime trust boundary should be introduced by this phase.

## Pitfalls

- Do not copy Phase 106's old rule that forbids positive `compact block relay` language; v2.1 now supports a bounded form.
- Do not treat explicit `none` breadcrumbs as automatically invalid; replace them only when a defensible source anchor exists.
- Do not create a new release-notes tree or parity manifest.
- Do not scan archived planning history for current claims.
- Do not make public-network UAT a required or release-blocking test.
- Do not update generated LOC metadata manually before the verifier requests freshness.
- Do not archive the milestone inside Phase 117.

## Validation Architecture

### Fast Feedback Per Plan

- Plan 117-01: `bun run scripts/check-parity-breadcrumbs.ts` plus JSON parsing and targeted `rg` checks for v2.1 surface IDs and Knots paths.
- Plan 117-02: `bun test scripts/check-phase117-parity-uat-release-boundary.test.ts` then `bun run scripts/check-phase117-parity-uat-release-boundary.ts`.
- Plan 117-03: rerun the Phase 117 checker/test so claim and command drift fails immediately.
- Plan 117-04: run Phase 116 and Phase 117 checker pairs, GSD lifecycle/metadata validation, `git diff --check`, and `bash scripts/verify.sh`.

### Completion Evidence

- All five BOUND requirements appear in plan frontmatter and the Phase 117 parity surface.
- All 34 v2.1 requirements appear exactly once across parity surfaces.
- The Phase 117 checker test includes pass fixtures plus mutations for missing surface, duplicate owner, missing anchor, missing command, wrong verifier order, forbidden positive claim, and accidental default-gate inclusion.
- `117-UAT.md` distinguishes deterministic required evidence from optional public-network review.
- `bash scripts/verify.sh` exits zero on the final working tree.

***

## RESEARCH COMPLETE

Phase 117 can be planned as four evidence-focused plans without runtime changes or new dependencies.
