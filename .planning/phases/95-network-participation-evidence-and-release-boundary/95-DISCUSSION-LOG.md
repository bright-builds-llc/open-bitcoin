# Phase 95: Network Participation Evidence and Release Boundary - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-06-27T12:49:20.758Z
**Phase:** 95-network-participation-evidence-and-release-boundary
**Mode:** Yolo
**Areas discussed:** Deterministic release-boundary checker, Parity roots and traceability, Operator UAT and non-regression, Support bundle redaction

---

## Deterministic Release-Boundary Checker

| Option | Description | Selected |
|--------|-------------|----------|
| Add one Phase 95 aggregate checker/test after Phase 94 | Matches the closeout scope, keeps earlier phase checkers stable, and gives BOUND-01, BOUND-03, and BOUND-06 one deterministic gate. | yes |
| Extend Phase 90-94 and Phase 88 checkers | Reuses existing local checks but scatters release-boundary logic across completed surfaces. | |
| Rely on existing checkers | Avoids churn but leaves Phase 95-specific release-boundary proof implicit. | |

**User's choice:** Auto-selected recommended option.
**Notes:** The checker should be milestone-wide, public-network-free, and wired after Phase 94 in both executable verification and the legacy verifier-order block.

---

## Parity Roots And Traceability

| Option | Description | Selected |
|--------|-------------|----------|
| New v1.9 release-boundary surface in existing parity roots | Adds a compact closeout row in `docs/parity/index.json` and `docs/parity/checklist.md` while keeping detailed Knots anchors in `catalog/p2p.md`. | yes |
| Extend existing P2P parity surfaces only | Minimal structure but weaker as a release-review entrypoint. | |
| Milestone audit and planning artifact closeout | Good GSD hygiene but insufficient for public release/parity docs. | |
| New standalone v1.9 boundary manifest/doc | Clear but risks creating another source of truth. | |

**User's choice:** Auto-selected recommended option.
**Notes:** `docs/parity/index.json` remains the machine-readable root. Human release evidence should live in `checklist.md`, `catalog/p2p.md`, and `release-readiness.md`.

---

## Operator UAT And Non-Regression

| Option | Description | Selected |
|--------|-------------|----------|
| Deterministic closeout mix | Combine Phase 95 checker/test, full `bash scripts/verify.sh`, and repo-local Cargo/Bazel loopback or synthetic UAT commands. | yes |
| Full `bash scripts/verify.sh` only | Strong regression proof but weak for UAT guidance and traceability. | |
| Focused checker commands only | Fast for iteration but too narrow for BOUND-03. | |
| Promote public-network full-sync/soak/support UAT to required proof | Strong live evidence when available but conflicts with deterministic default verification and risks overclaiming. | |

**User's choice:** Auto-selected recommended option.
**Notes:** Public-network, real service-manager, soak, and live support collection remain optional operator evidence, not release-blocking checks.

---

## Support Bundle Redaction

| Option | Description | Selected |
|--------|-------------|----------|
| Aggregate Phase 95 checker over existing support evidence | Reuses Phase 90-94 Rust support tests and proves BOUND-05 without behavior churn. | yes |
| Add new support redaction behavior/tests | Stronger if a real leak is found but risks changing behavior in a boundary phase. | |
| Documentation-only closure | Lowest churn but too weak for deterministic proof. | |
| Hybrid aggregate checker plus one focused resource assertion | Good if a narrow Phase 94 resource-evidence gap is found. | yes |

**User's choice:** Auto-selected aggregate-checker approach, with a narrow resource assertion if planning finds it necessary.
**Notes:** Existing support evidence covers endpoint, permission, address, peer-policy, and resource-governance evidence. Phase 95 should preserve useful diagnosis while redacting raw peer/address/config/payload material.

---

## the agent's Discretion

- Exact checker helper structure and fixture layout.
- Exact failure-message wording.
- Whether a narrow Rust support assertion is needed after inspecting the existing evidence.
- Whether Phase 95 produces a milestone audit artifact during execution.

## Deferred Ideas

- Transaction relay, compact block relay, mempool propagation, full address relay, public inbound defaults, production-service operation, public-network CI, and production full-node readiness are future milestone scope.
