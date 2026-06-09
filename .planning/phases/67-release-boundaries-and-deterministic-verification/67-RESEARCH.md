# Phase 67: Release Boundaries and Deterministic Verification - Research

## RESEARCH COMPLETE

### Scope Read

Phase 67 covers REL-01 through REL-04. The implementation surface is primarily
documentation and deterministic verification:

- REL-01 needs refreshed v1.5 threat-model and release-readiness docs covering
  the unattended sync loop, service supervision, long-run evidence, resource
  bounds, recovery states, support redaction, and compatibility wrapper output.
- REL-02 needs parity docs that distinguish v1.5 extended operator-review
  readiness from deferred production, inbound, relay, wallet, migration,
  packaging, hosted-dashboard, GUI, and broad production-node claims.
- REL-03 needs default verification to remain deterministic, with public-network
  long-run and service checks kept as opt-in UAT evidence.
- REL-04 needs deterministic checks that fail when v1.5 docs or parity roots omit
  the unattended-operation claim boundaries.

### Existing Patterns

- `scripts/check-v1.4-release-boundaries.ts` is the closest prior release-boundary
  checker. It validates a checklist surface, audit roots, release docs, deferred
  surface wording, and `scripts/verify.sh` default-verification exclusions.
- `scripts/check-phase65-support-review.ts` and
  `scripts/check-phase66-compatibility-wrapper.ts` are the latest phase checker
  style: small constant arrays, `readText`, `requireContains`,
  `requireNotContains`, and a clear success line.
- `scripts/verify.sh` already runs deterministic release and phase checkers before
  Rust formatting, clippy, build, tests, benchmarks, Bazel, and coverage.
- `docs/parity/index.json` already has v1.3 and v1.4 checklist/audit structures
  that can be extended with v1.5 entries.
- `docs/parity/release-readiness.md` already preserves historical v1.3 and v1.4
  sections; Phase 67 should add v1.5 without rewriting history.
- `docs/parity/catalog/p2p.md` already has v1.5 subsections for service
  restart, support review, and compatibility wrapper evidence.

### Implementation Approach

Use one plan because the docs, parity roots, checker, and verification wiring must
agree on exact strings and evidence paths. The safest shape is:

1. Add `docs/parity/threat-model-v1.5.md` with a compact STRIDE register,
   ASVS-language reviewer mapping, evidence acceptance, release boundary matrix,
   requirements traceability, and residual risks.
2. Extend `docs/parity/release-readiness.md`, `README.md`, `checklist.md`,
   `index.json`, `catalog/p2p.md`, and `deviations-and-unknowns.md` with v1.5
   release-boundary entries and deferred-surface wording.
3. Add `scripts/check-v1.5-release-boundaries.ts` and wire it into
   `scripts/verify.sh`.
4. Write `67-VERIFICATION.md` only after the checker and repo verification pass.

### Validation Architecture

The Phase 67 checker should assert:

- `docs/parity/index.json` has exactly one
  `v1-5-unattended-operation-release-boundaries` checklist surface with status
  `done`, requirements `REL-01`, `REL-02`, `REL-03`, `REL-04`, and required
  evidence paths.
- `docs/parity/index.json` audit roots include `v1_5_threat_model`,
  `v1_5_release_boundaries`, `threat-model-v1.5.md`, and every REL id.
- `docs/parity/README.md` names `threat-model-v1.5.md` as the current v1.5
  closeout while preserving v1.4/v1.3 as historical evidence.
- `docs/parity/release-readiness.md`, `docs/parity/deviations-and-unknowns.md`,
  and `docs/parity/catalog/p2p.md` include required deferred-surface wording.
- `docs/parity/threat-model-v1.5.md` includes a STRIDE register, ASVS L1 mapping,
  `OWASP ASVS v5.0.0`, threat ids `V15-TM-01` through `V15-TM-08`, and REL
  traceability.
- `scripts/verify.sh` includes `bun run scripts/check-v1.5-release-boundaries.ts`
  and does not include public-network or real service-manager commands.

### Verification Commands

Use focused checks before the aggregate gate:

```bash
bun run scripts/check-v1.5-release-boundaries.ts
bash scripts/verify.sh
```

`bash scripts/verify.sh` is still the final repo-native verification contract.
