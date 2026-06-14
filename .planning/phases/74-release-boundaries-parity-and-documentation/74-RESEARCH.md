# Phase 74: Release Boundaries, Parity, and Documentation - Research

**Researched:** 2026-06-14
**Domain:** release-boundary documentation, parity roots, deterministic Bun checks, operator guidance
**Confidence:** HIGH

## RESEARCH COMPLETE

### Scope Read

Phase 74 closes v1.6 Mainnet Full-Sync Completion. The deliverable is an
auditable release boundary, not new sync runtime behavior.

- REL-01 needs v1.6 parity roots, threat model, release-readiness matrix,
  README, and operator docs that describe only the explicit opt-in full-sync
  completion claim.
- REL-02 needs deterministic checks that prevent docs and status surfaces from
  implying inbound serving, relay, production-wallet, migration apply,
  packaging, GUI, hosted dashboards, public-network CI, release-blocking live
  sync, or broad production-node readiness.
- REL-03 needs operator-facing docs that explain shipped sync-to-tip evidence,
  opt-in UAT commands, support evidence locations, failure interpretation, and
  deferred scope.
- Final milestone traceability must show all 26 v1.6 requirements mapped and
  verified, with REL-01 through REL-03 closing in this phase.

### Existing Patterns

- Phase 67 is the closest historical closeout. It added a milestone-specific
  threat model, release-readiness matrix, parity root entries, README/runtime
  pointers, a deterministic release-boundary checker, and verify wiring.
- `scripts/check-v1.5-release-boundaries.ts` is the best checker template. It
  validates machine-readable roots, human docs, deferred-scope wording, threat
  model strings, release-readiness strings, and forbidden `scripts/verify.sh`
  commands.
- `scripts/check-phase73-uat-verification.ts` already validates opt-in public
  mainnet UAT boundaries and deterministic evidence for the Phase 73 closeout.
  Phase 74 should reference that evidence instead of duplicating its matrix.
- `docs/operator/runtime-guide.md` is the authoritative UAT command surface.
  Phase 74 should add a concise v1.6 closeout section or pointers while keeping
  Phase 73's matrix as the command source of truth.
- `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/README.md`,
  `docs/parity/release-readiness.md`, `docs/parity/deviations-and-unknowns.md`,
  and the P2P/chainstate/operator-runtime catalog pages already carry prior
  milestone and Phase 73 evidence roots.

### Implementation Approach

Use one execution plan so all release-boundary strings stay consistent across
docs, machine roots, checker assertions, and verification:

1. Add `docs/parity/threat-model-v1.6.md` with compact STRIDE, ASVS L1 mapping,
   evidence acceptance, release boundary matrix, REL traceability, and explicit
   non-claims.
2. Extend `docs/parity/release-readiness.md` with a v1.6 claim-boundary matrix
   mapping Phase 68 through Phase 73 evidence to REL-01 through REL-03 and final
   all-26 requirement traceability.
3. Update parity roots and catalog pages with a discoverable v1.6 closeout
   surface, threat-model audit root, release-boundary audit root, and deferred
   scope wording.
4. Update `README.md` and `docs/operator/runtime-guide.md` with current v1.6
   release posture and links to the parity roots without creating a second
   authoritative UAT matrix.
5. Add `scripts/check-v1.6-release-boundaries.ts`, wire it into
   `scripts/verify.sh` after the Phase 73 checker, and assert required v1.6
   docs, REL ids, all 26 requirements, Phase 68 through Phase 73 evidence paths,
   deferred scope terms, and forbidden default-verification commands.
6. Write Phase 74 verification after focused checker commands and
   `bash scripts/verify.sh` pass.

### Validation Architecture

The Phase 74 checker should assert:

- `docs/parity/index.json` has a checklist surface such as
  `v1-6-full-sync-completion-release-boundaries` with status `done`,
  requirements `REL-01`, `REL-02`, and `REL-03`, and evidence paths for the v1.6
  threat model, release-readiness docs, README, runtime guide, parity catalog
  pages, Phase 73 verification, checker, and verify script.
- `docs/parity/index.json` audit roots include `v1_6_threat_model`,
  `v1_6_release_boundaries`, `threat-model-v1.6.md`, and every REL id.
- `docs/parity/release-readiness.md` includes a v1.6 claim-boundary matrix,
  Phase 68 through Phase 73 evidence chain, REL traceability, and all 26 v1.6
  requirement ids.
- `docs/parity/threat-model-v1.6.md` includes a STRIDE register, ASVS L1
  mapping, `OWASP ASVS v5.0.0`, v1.6 threat ids, evidence acceptance, release
  boundary matrix, and REL traceability.
- Parity README, checklist, catalog pages, deviations, README, and runtime guide
  all preserve explicit non-claims for inbound serving, address relay, block
  serving, transaction relay, compact block relay, production-funds wallet
  safety, migration apply mode, signed packaging, Windows service support, GUI,
  hosted dashboards, public-network CI, release-blocking live sync, and broad
  production-node readiness.
- `scripts/verify.sh` runs the v1.6 checker and does not include public-network
  live smoke, manual peers, `--restart-after-progress`, real `systemctl` or
  `launchctl`, mainnet IBD activation, current-tip timing gates, or
  release-blocking live-sync commands in default verification.

### Verification Commands

Use focused checks before the aggregate gate:

```bash
bun --check scripts/check-v1.6-release-boundaries.ts
bun run scripts/check-v1.6-release-boundaries.ts
bun run scripts/check-phase73-uat-verification.ts
bun run scripts/check-parity-breadcrumbs.ts --check
bash scripts/verify.sh
```

`bash scripts/verify.sh` remains the final repo-native verification contract.
