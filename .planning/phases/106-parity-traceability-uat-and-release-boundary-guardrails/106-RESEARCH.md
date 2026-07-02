---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 106-2026-07-02T03-46-34
generated_at: 2026-07-02T04:10:00Z
---

# Phase 106: Parity Traceability, UAT, and Release Boundary Guardrails - Research

**Researched:** 2026-07-02
**Domain:** v2.0 parity traceability closeout, deterministic no-claim guardrails, operator UAT guidance, release-boundary documentation
**Confidence:** HIGH

<user_constraints>
## Locked Decisions From CONTEXT.md

- Phase 106 is the canonical closeout for `BOUND-01` through `BOUND-05`.
- Every v2.0 requirement must have exactly one roadmap owner and evidence roots.
- Deterministic guardrails must reject positive claims for compact block relay, bloom/filter serving, package relay, public relay defaults, public-network relay CI, production service operation, production full-node readiness, and production-funds wallet use.
- UAT instructions must use repo-local Cargo and Bazel commands, not only installed aliases.
- Public-network relay review remains opt-in and must not become default CI or release validation.
- `bash scripts/verify.sh` remains the repo-native verification contract and must include the Phase 106 guardrails.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
| --- | --- | --- |
| BOUND-01 | Parity docs identify the v2.0 transaction relay and mempool evidence boundary. | Add a Phase 106 parity surface and closeout text that references the Phase 100-105 evidence roots. |
| BOUND-02 | No unsupported production or public relay claim can drift into release-facing docs unnoticed. | Add a deterministic checker that scans release/operator/parity docs for forbidden positive claims. |
| BOUND-03 | Operator UAT uses repo-local Cargo and Bazel commands and marks public-network review as opt-in. | Add a Phase 106 runtime-guide section with exact Cargo and Bazel commands and bounded wording. |
| BOUND-04 | Default verification stays deterministic and local. | Wire the Phase 106 checker and tests into `scripts/verify.sh` after Phase 105 and before pure-core checks. |
| BOUND-05 | v2.0 traceability is auditable across requirements, roadmap ownership, docs, and parity index evidence. | Add checker assertions for exactly-one v2.0 requirement ownership, parity surface evidence, and Knots anchors. |
</phase_requirements>

## Summary

Phase 106 should not add new node behavior. The milestone already has behavior evidence from Phases 100-105 for relay activation, inventory and download scheduling, orphan handling, mempool lifecycle, relay serving/fanout, and operator evidence. The remaining work is to make the v2.0 claim boundary hard to misstate.

The safest shape is one closeout parity surface for `BOUND-01` through `BOUND-05`, one deterministic checker with regression fixtures, and concise operator/release documentation that names what v2.0 does and does not prove. The checker should verify the existing Phase 100-105 surfaces as a complete 32-requirement set, then verify the new Phase 106 surface and no-claim boundaries.

## Existing Evidence Map

- `docs/parity/index.json` already records Phase 100-105 v2.0 surfaces for `ACT-*`, `INV-*`, `DL-*`, `MEM-*`, `REL-*`, and `OBS-*`.
- `docs/parity/checklist.md` mirrors the v2.0 parity surface list in human-readable form.
- `docs/parity/catalog/p2p.md`, `docs/parity/catalog/mempool-policy.md`, and `docs/parity/catalog/rpc-cli-config.md` contain the main relay, mempool, and operator evidence narrative.
- `docs/operator/runtime-guide.md` contains Phase 100 and Phase 105 operator review sections with the required repo-local command style.
- `README.md` already describes bounded v2.0 relay evidence and the deferred production/public-network surfaces.
- `scripts/check-phase105-operator-relay-evidence.ts` is the closest checker template for no-claim scanning, verifier-order assertions, parity index assertions, and source breadcrumb coverage.
- `scripts/check-phase95-network-participation-release-boundary.ts` and `scripts/check-phase98-traceability-reconciliation.ts` are the closest templates for requirement ownership and release-boundary traceability checks.

## Recommended Plan Shape

### Plan 106-01: Parity, Operator UAT, and Release Boundary Docs

Update the Phase 106 parity index/checklist surface and the release/operator docs. Keep the language bounded: v2.0 has deterministic local relay/mempool evidence and operator surfaces, but not public relay defaults, public-network CI, compact block relay, package relay, bloom/filter serving, production service proof, production full-node readiness, or production-funds wallet safety.

### Plan 106-02: Deterministic Guardrail Checker and Verification Wiring

Add `scripts/check-phase106-parity-uat-release-boundary.ts` plus fixtures. The checker should assert the seven v2.0 parity surfaces cover exactly 32 requirements, the Phase 106 surface covers `BOUND-01` through `BOUND-05`, UAT docs contain exact Cargo and Bazel commands, no-claim text rejects unsupported production/public relay claims, and `scripts/verify.sh` runs the Phase 106 test/check pair after Phase 105.

## Validation Architecture

Required focused verification:

```bash
bun test scripts/check-phase106-parity-uat-release-boundary.test.ts
bun run scripts/check-phase106-parity-uat-release-boundary.ts
node -e "JSON.parse(require('fs').readFileSync('docs/parity/index.json','utf8'))"
git diff --check
```

Repo-native verification target:

```bash
bash scripts/verify.sh
```

Known local caveat: the existing environment has previously hung in the Cargo test phase of `scripts/verify.sh`. If that repeats, record the failed/hung command explicitly and do not commit or push.
