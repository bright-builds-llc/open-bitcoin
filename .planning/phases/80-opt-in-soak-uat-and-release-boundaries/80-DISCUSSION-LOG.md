# Phase 80: Opt-In Soak UAT and Release Boundaries - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-17T22:54:57.119Z
**Phase:** 80-Opt-In Soak UAT and Release Boundaries
**Mode:** Yolo
**Areas discussed:** Default Verification Boundary, Opt-In UAT Command Matrix, Parity And Audit Closure, Release Boundary Wording

---

## Default Verification Boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Phase 80 deterministic boundary checker | Add a focused Bun checker and fixture test that guards `scripts/verify.sh`, docs, parity roots, UAT commands, and forbidden public-network/default-gate strings. | yes |
| Existing checker roll-up plus manual release checklist | Reuse existing checkers and document a manual release checklist without a v1.7-specific gate. | |
| Two-tier verification surface | Add a separate opt-in UAT validation command in addition to default verification. | |
| Platform sandbox proof around `verify.sh` | Prove the default verifier under a runtime sandbox or container. | |

**User's choice:** Yolo selected the recommended deterministic boundary checker.
**Notes:** This matches the Phase 73 and Phase 75 through Phase 79 checker style and avoids adding sandbox or offline-cache fragility to the closeout.

---

## Opt-In UAT Command Matrix

| Option | Description | Selected |
|--------|-------------|----------|
| Focused Phase 80 v1.7 matrix | Document multi-day soak lifecycle, bounded recovery drill, support-bundle generation, and post-failure diagnosis with repo-local Cargo and Bazel forms. | yes |
| Expanded operator scenario matrix | Include daemon activation, live smoke, service restart, soak lifecycle, recovery/status, support bundle, and diagnosis. | |
| Distributed runbooks plus Phase 80 index | Keep commands in existing sections and add only an index. | |

**User's choice:** Yolo selected the focused Phase 80 v1.7 matrix.
**Notes:** This directly satisfies VER-06 without mixing older v1.6 full-sync UAT or real service-manager workflows into the v1.7 release closeout.

---

## Parity And Audit Closure

| Option | Description | Selected |
|--------|-------------|----------|
| Single Phase 80 closure checker | Use one focused checker/test to tie v1.7 docs, parity roots, support schemas, checker wiring, commands, breadcrumbs, and non-claims together. | yes |
| Manifest-driven v1.7 evidence registry | Add a new evidence manifest that maps every v1.7 surface. | |
| Extend existing Phase 75-79 checkers only | Patch existing checkers instead of adding a Phase 80 closure gate. | |

**User's choice:** Yolo selected the single Phase 80 closure checker.
**Notes:** The existing parity index and source-breadcrumb registry are enough; adding a new manifest would duplicate current audit roots.

---

## Release Boundary Wording

| Option | Description | Selected |
|--------|-------------|----------|
| README/runtime-guide wording only | Update only the main operator-facing entrypoints. | |
| Parity-rooted v1.7 boundary plus checker | Align README, runtime guide, release-readiness, parity roots, deferred-scope docs, and `verify.sh` through a deterministic guard. | yes |
| Add status/output guards to the v1.7 checker | Scan additional runtime output surfaces for claim-bearing wording. | |
| Formal v1.7 release package | Add a larger release-candidate handoff with release-note style artifacts. | |

**User's choice:** Yolo selected the parity-rooted v1.7 boundary plus checker.
**Notes:** Targeted status/output guards are acceptable only where exact operator text is claim-bearing; broad scans should be avoided to reduce brittle false positives.

---

## the agent's Discretion

- Keep Phase 80 primarily in docs and Bun checker code if no behavior gap is found.
- Reuse Phase 79 checker/test structure with v1.7-specific ids, anchors, and non-claims.
- Add source breadcrumbs only if new Rust source or test files are created.

## Deferred Ideas

- Runtime sandboxing or containerized offline proof for default verification.
- Signed or externally comparable support/soak artifacts.
- A reusable v1.x evidence manifest system.
- Production-node expansion and public-network CI.
