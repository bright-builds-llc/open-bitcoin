# Phase 106: Parity Traceability, UAT, and Release Boundary Guardrails - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md. This log preserves the alternatives considered.

**Date:** 2026-07-02T03:48:26.726Z
**Phase:** 106 - Parity Traceability, UAT, and Release Boundary Guardrails
**Mode:** Yolo
**Areas discussed:** Traceability ownership, deterministic no-claim guardrails, UAT guidance, docs and release boundary wording, default verification contract

## Traceability Ownership

| Option | Description | Selected |
| --- | --- | --- |
| Canonical closeout audit | Make Phase 106 own BOUND-01 through BOUND-05 and prove all 32 v2.0 requirements have one roadmap owner plus concrete evidence roots. | yes |
| Prose-only closeout | Update docs manually without a deterministic audit. | |
| Defer traceability to milestone archive | Leave evidence reconciliation for milestone completion. | |

**User's choice:** Auto-selected recommended default: Canonical closeout audit.
**Notes:** Prior phases already own implementation evidence. Phase 106 should reconcile and guard it rather than reimplement relay behavior.

## Deterministic No-Claim Guardrails

| Option | Description | Selected |
| --- | --- | --- |
| Fixed-corpus checker with fixtures | Add or extend targeted TypeScript checks and tests for compact-block, bloom/filter, package-relay, public-default, production-readiness, production-service, public-network CI, and production-funds overclaims. | yes |
| Broad grep-only ban list | Scan every file for words without context. | |
| Documentation review only | Rely on manual review of release wording. | |

**User's choice:** Auto-selected recommended default: Fixed-corpus checker with fixtures.
**Notes:** The checker should reject positive support claims while allowing explicit deferred or out-of-scope wording.

## UAT Guidance

| Option | Description | Selected |
| --- | --- | --- |
| Repo-local Cargo and Bazel commands | Show exact `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...` and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...` forms. | yes |
| Installed alias only | Use `open-bitcoin ...` as the primary local operator command. | |
| No UAT commands | Describe workflows without copy-pasteable commands. | |

**User's choice:** Auto-selected recommended default: Repo-local Cargo and Bazel commands.
**Notes:** This carries forward the repo lesson and `AGENTS.md` local guidance. Public-network relay review must remain opt-in.

## Docs And Release Boundary Wording

| Option | Description | Selected |
| --- | --- | --- |
| Bounded evidence-focused v2.0 claim | Update README, operator docs, parity docs, runtime docs, and release notes to describe bounded default-off relay/mempool evidence and list deferred surfaces. | yes |
| Positive production-style launch language | Present v2.0 as broad relay or production-node readiness. | |
| Parity registry only | Update only machine-readable parity metadata and skip user-facing docs. | |

**User's choice:** Auto-selected recommended default: Bounded evidence-focused v2.0 claim.
**Notes:** Docs should remain quiet, operator-focused, and explicit about deferred surfaces.

## Default Verification Contract

| Option | Description | Selected |
| --- | --- | --- |
| Keep `scripts/verify.sh` deterministic | Wire Phase 106 guardrails into the default verifier while keeping public-network, service-manager, wall-clock soak, production-deployment, and destructive repair gates out. | yes |
| Add public-network release gate | Make public relay review part of default verification. | |
| Separate manual-only closeout | Leave guardrails outside the repo-native verifier. | |

**User's choice:** Auto-selected recommended default: Keep `scripts/verify.sh` deterministic.
**Notes:** Final phase verification should include parity breadcrumbs, checker tests, docs/UAT command checks, traceability audit, and `bash scripts/verify.sh`.

## the agent's Discretion

- Exact checker names, fixture layout, and audit implementation details.
- Exact doc section placement as long as existing docs stay coherent and evidence-focused.
- Whether to extend an existing release/no-claim checker or add a Phase 106-specific checker, based on the smallest maintainable change.

## Deferred Ideas

- Compact block relay, bloom/filter serving, package relay, public relay defaults, public-network relay CI, production service operation, production full-node readiness, production-funds wallet use, packaging, GUI, hosted dashboards, migration apply mode, destructive repair, and automatic support-bundle upload.
