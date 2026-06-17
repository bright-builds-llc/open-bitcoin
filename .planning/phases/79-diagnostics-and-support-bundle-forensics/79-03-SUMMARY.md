---
phase: 79-diagnostics-and-support-bundle-forensics
plan: 03
subsystem: documentation-parity
tags: [support-bundle, diagnostics, forensics, parity, docs]

requires:
  - phase: 79-diagnostics-and-support-bundle-forensics
    provides: Plan 79-01 typed support_forensics sidecar
  - phase: 79-diagnostics-and-support-bundle-forensics
    provides: Plan 79-02 support-forensics Markdown rendering
provides:
  - operator-facing support-forensics semantics and UAT commands
  - architecture boundary for shared diagnostic status versus support-forensics provenance
  - parity root for phase79-diagnostics-support-bundle-forensics
affects: [operator-docs, architecture-docs, parity-ledger]

tech-stack:
  added: []
  patterns:
    - additive parity roots for phase-scoped diagnostics
    - support-bundle non-claim wording shared across operator and parity docs

key-files:
  created: []
  modified:
    - docs/operator/runtime-guide.md
    - docs/architecture/status-snapshot.md
    - docs/architecture/operator-observability.md
    - docs/parity/index.json
    - docs/parity/checklist.md
    - docs/parity/README.md
    - docs/parity/catalog/p2p.md
    - docs/parity/catalog/chainstate.md
    - docs/parity/catalog/operator-runtime-release-hardening.md

key-decisions:
  - "Documented checkpoint-chain evidence as ordering/truncation evidence, not authenticity, signing, or an external trust root."
  - "Kept OpenBitcoinStatusSnapshot as runtime truth and support_forensics as bundle-specific provenance."
  - "Made non-claims explicit across parity roots, including public-network default checks, multi-day default gates, automatic support-bundle upload, and production-node readiness."

patterns-established:
  - "Parity surfaces should carry the exact phase root id, requirement ids, evidence anchors, and scoped non-claims in both index.json and checklist.md."
  - "Operator-facing UAT docs should include repo-local Cargo and Bazel command forms."

requirements-completed: [DIAG-01, DIAG-02, DIAG-03, DIAG-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 79-2026-06-17T13-53-04
generated_at: 2026-06-17T18:30:25Z

duration: 42m
completed: 2026-06-17
---

# Phase 79-03: Support Forensics Docs And Parity Summary

**Support-bundle forensics are now documented as local diagnostic evidence with explicit parity roots and non-claims**

## Performance

- **Duration:** 42m
- **Started:** 2026-06-17T17:48:00Z
- **Completed:** 2026-06-17T18:30:25Z
- **Tasks:** 3
- **Files modified:** 9

## Accomplishments

- Added `### Phase 79 support bundle forensics` to the operator runtime guide with typed sidecar fields, verdict labels, checkpoint-chain limits, non-claim wording, and repo-local Cargo/Bazel UAT commands.
- Added architecture sections for the shared diagnostic contract and bounded support-forensics projection.
- Added `phase79-diagnostics-support-bundle-forensics` to parity roots with DIAG-01 through DIAG-04 and explicit exclusions for production, public-network default, upload, packaging, GUI, and hosted-dashboard claims.

## Task Commits

1. **Task 1: Document operator support-bundle forensic semantics** - `26c2e8f` (docs)
2. **Task 2: Document shared diagnostic contract and bounded observability projection** - `26c2e8f` (docs)
3. **Task 3: Update parity roots for Phase 79 traceability and non-claims** - `26c2e8f` (docs)

## Files Created/Modified

- `docs/operator/runtime-guide.md` - Documents support_forensics fields, verdicts, checkpoint-chain boundaries, non-claims, and UAT commands.
- `docs/architecture/status-snapshot.md` - Documents OpenBitcoinStatusSnapshot as runtime truth and support_forensics as bundle provenance.
- `docs/architecture/operator-observability.md` - Documents bounded labels/counts versus high-cardinality forensic objects.
- `docs/parity/index.json` - Adds machine-readable Phase 79 surface and audit root.
- `docs/parity/checklist.md` - Adds human-readable Phase 79 parity row.
- `docs/parity/README.md` - Adds top-level Phase 79 parity-root explanation.
- `docs/parity/catalog/p2p.md` - Adds P2P-facing support-forensics boundary.
- `docs/parity/catalog/chainstate.md` - Adds chainstate-facing support-forensics boundary.
- `docs/parity/catalog/operator-runtime-release-hardening.md` - Adds operator-runtime audit-matrix and known-gap entries.

## Verification

- `jq empty docs/parity/index.json`
- `rg -n "### Phase 79 support bundle forensics|support_forensics|forensic timeline|checkpoint chain|failure narrative|likely_cause|evidence_basis|next_action|confidence|soak_stable|blocker_diagnosed|inconclusive|collection_failed|not authenticity|public-network-free|service-manager-free|cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin|bazel run //packages/open-bitcoin-cli:open_bitcoin" docs/operator/runtime-guide.md`
- `rg -n "## Phase 79 shared diagnostic contract and support-forensics sidecar|OpenBitcoinStatusSnapshot|support_forensics|resource_bound_evidence\\.maybe_projected_bundle_size_bytes|checkpoint-chain validation" docs/architecture/status-snapshot.md`
- `rg -n "## Phase 79 support forensics projection|CLI status|dashboard status|RPC status|metrics|structured logs|live-smoke summaries|soak reports|bounded labels and counts|high-cardinality forensic objects" docs/architecture/operator-observability.md`
- `rg -n "phase79-diagnostics-support-bundle-forensics|DIAG-01|DIAG-02|DIAG-03|DIAG-04|support_forensics|forensic timeline|checkpoint chain|failure narrative|cross-surface consistency" docs/parity/index.json docs/parity/checklist.md docs/parity/README.md docs/parity/catalog/p2p.md docs/parity/catalog/chainstate.md docs/parity/catalog/operator-runtime-release-hardening.md`
- Commit hook for `26c2e8f` ran `bash scripts/verify.sh` successfully.

## Deviations from Plan

None.

## Issues Encountered

- Two exact-phrase acceptance checks initially would have failed because Markdown wrapping split the phrases. The lines were tightened before verification.

## User Setup Required

None - documentation and parity roots only.

## Next Phase Readiness

Plan 79-04 can add deterministic checker and verification wiring against the documented Phase 79 roots and exact non-claim phrases.

---
*Phase: 79-diagnostics-and-support-bundle-forensics*
*Completed: 2026-06-17*
