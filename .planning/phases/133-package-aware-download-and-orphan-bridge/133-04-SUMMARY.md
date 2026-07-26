---
phase: 133-package-aware-download-and-orphan-bridge
plan: "04"
subsystem: testing
tags:
  - rust
  - bun
  - package-relay
  - parity
  - source-guard
requires:
  - phase: 133-01
    provides: bounded rolling reject evidence and request suppression
  - phase: 133-02
    provides: shared orphan bodies with bounded provenance and candidate identity
  - phase: 133-03
    provides: node-owned opportunistic 1P1C package admission and typed feedback
provides:
  - integrated adversarial coverage for PPKG-01 through PPKG-03
  - mutation-tested fail-closed Phase 133 source and claim guard
  - parity catalog, checklist, breadcrumb, and README closure
  - passing full Rust, Bazel, coverage, checker, breadcrumb, and LOC contract
affects:
  - 134-orphan-lifecycle-feedback
  - 136-package-relay-fanout-and-receipts
  - 137-package-rpc-and-operator-surfaces
tech-stack:
  added: []
  patterns:
    - deterministic filesystem-only structural guards with independent mutations
    - bounded adversarial matrices over production relay and admission paths
key-files:
  created:
    - scripts/check-phase133-package-aware-download-orphan-bridge.ts
    - scripts/check-phase133-package-aware-download-orphan-bridge.test.ts
  modified:
    - packages/open-bitcoin-network/src/peer/transaction_relay/tests/reject_evidence_cases.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs
    - packages/open-bitcoin-node/src/network/tests/package_bridge_cases.rs
    - docs/parity/catalog/mempool-policy.md
    - docs/parity/index.json
    - docs/parity/source-breadcrumbs.json
    - scripts/verify.sh
    - docs/metrics/lines-of-code.md
key-decisions:
  - "Claim only bounded opportunistic same-peer 1P1C assembly over ordinary transaction messages; general wire, arbitrary multi-parent, fanout, public/default, and production surfaces remain deferred."
  - "Guard the exact node-owned Phase 132 report/fingerprint/delta handoff and exhaustive feedback boundary with a filesystem-only checker and 22 independent mutations."
  - "Treat probabilistic reject evidence as suppression-only, with active-tip reset and no peer punishment."
patterns-established:
  - "Parity closure: bind behavior, tests, Knots anchors, machine index entries, human checklist claims, and forbidden claims in one fail-closed guard."
  - "Resource proof: test configured maxima for fixed allocation, shared bodies, bounded announcers, bounded traversal, and coherent cleanup."
requirements-completed: []
requirements-addressed:
  - PPKG-01
  - PPKG-02
  - PPKG-03
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 133-2026-07-26T16-12-51
generated_at: 2026-07-26T22:00:40Z
duration: 38min
completed: 2026-07-26
---

# Phase 133 Plan 04: Package-Aware Download and Orphan Bridge Summary

**Integrated bounded-resource parity tests and a 22-mutation fail-closed guard now lock PPKG-01 through PPKG-03 to the exact same-peer ordinary-message 1P1C boundary.**

## Performance

- **Duration:** 38 min
- **Started:** 2026-07-26T21:22:52Z
- **Completed:** 2026-07-26T22:00:40Z
- **Tasks:** 3
- **Files modified:** 14

## Accomplishments

- Consolidated deterministic adversarial cases for fixed reject-evidence allocation, false-positive suppression without punishment, active-tip reset, one shared orphan body, bounded announcers and traversal, coherent cleanup, and multi-parent suppression.
- Published the exact PPKG-01/02/03 behavior, lineage, intentional wtxid/fingerprint scope difference, and deferred boundaries across the parity catalog, machine index, checklist, breadcrumbs, and contributor READMEs.
- Added a 540-line filesystem-only Phase 133 checker with 22 mutation tests and wired both immediately after Phase 132 in the default verifier.
- Passed focused network/node Cargo suites, focused Bazel builds, and the complete repository verifier including Rust, Bazel, coverage, parity, breadcrumbs, source guards, and fresh LOC evidence.

## Task Commits

Each task was committed atomically:

1. **Task 1: Consolidate deterministic integrated parity and bounded-work proof** - `adcbb9f4` (test)
2. **Task 2: Publish parity truth and mutation-tested Phase 133 guardrails** - `7ab32c53` (test)
3. **Task 3: Run final repository verification and inspect scope** - `b9b93575` (chore)

## Files Created/Modified

- `scripts/check-phase133-package-aware-download-orphan-bridge.ts` - Enforces PPKG behavior, traceability, verifier ordering, architectural boundaries, and forbidden claims.
- `scripts/check-phase133-package-aware-download-orphan-bridge.test.ts` - Proves 22 major guard contracts fail independently under mutation.
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests/reject_evidence_cases.rs` - Covers fixed allocation, deterministic reset, and suppression-only false positives.
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs` - Covers shared bodies, bounded provenance/traversal, newest-first selection, and coherent cleanup.
- `packages/open-bitcoin-node/src/network/tests/package_bridge_cases.rs` - Covers arrival order, provenance rejection, exact authoritative handoff, feedback, and multi-parent suppression.
- `docs/parity/catalog/mempool-policy.md` - Records the complete Phase 133 behavior, Knots anchors, intentional difference, and deferred surface.
- `docs/parity/index.json` - Adds machine-readable PPKG evidence groups and Knots lineage.
- `docs/parity/checklist.md` - Adds human review closure for Phase 133.
- `docs/parity/source-breadcrumbs.json` - Connects Phase 133 Rust behavior and tests to `p2p_opportunistic_1p1c.py` and related anchors.
- `README.md` and `packages/README.md` - State the bounded same-peer 1P1C claim without broadening relay or production claims.
- `scripts/verify.sh` - Runs Phase 133 mutation tests and checker immediately after Phase 132 in both ordering surfaces.
- `docs/metrics/lines-of-code.md` - Refreshes deterministic tracked LOC evidence to 640 files and 277,903 lines.

## Decisions Made

- Kept the public claim narrower than the implementation mechanics: bounded opportunistic same-peer 1P1C assembly over ordinary `inv`/`getdata`/`tx` behavior, without package wire, arbitrary graphs, fanout receipts, public defaults, or guaranteed propagation.
- Required exact report, fingerprint, and delta preservation through one node-owned admission call; no premature cross-cache lifecycle projection is permitted.
- Used deterministic fixed seeds, fixed times, configured maxima, and production paths so security and resource proofs remain hermetic.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Preserved the historical Phase 132 README boundary**

- **Found during:** Task 3 (Run final repository verification and inspect scope)
- **Issue:** The new Phase 133 README wording removed the exact `peer package assembly` phrase guarded by the Phase 132 checker.
- **Fix:** Rephrased the deferred claim as peer package assembly beyond the bounded 1P1C bridge, preserving both the historical contract and the narrower Phase 133 claim.
- **Files modified:** `README.md`
- **Verification:** Phase 132's 36 checker mutations, Phase 133's 22 checker mutations, both live checkers, and the full repository verifier passed.
- **Committed in:** `b9b93575`

**2. [Rule 1 - Bug] Corrected stale human-readable GSD position fields**

- **Found during:** Plan metadata finalization
- **Issue:** The GSD state commands advanced structured frontmatter to verification but left the body at `EXECUTING`, 50%, and `133-03-PLAN.md`.
- **Fix:** Aligned the human-readable current position with the tool-reported 4/4, 100%, ready-for-verification state.
- **Files modified:** `.planning/STATE.md`
- **Verification:** Reviewed the final STATE frontmatter and body together after `state update-progress` and `roadmap update-plan-progress`.
- **Committed in:** Plan metadata commit.

**3. [Rule 2 - Missing Critical] Kept PPKG requirements pending until lifecycle verification**

- **Found during:** Plan metadata finalization
- **Issue:** The generic requirements command and summary template activated PPKG-01 through PPKG-03 before Phase 133 had a lifecycle-valid `VERIFICATION.md`, violating the repository's active-milestone traceability contract.
- **Fix:** Restored checklist and traceability statuses to Pending, recorded the IDs as `requirements-addressed`, and left `requirements-completed` empty until the Phase 133 verifier promotes them.
- **Files modified:** `.planning/REQUIREMENTS.md`, `.planning/phases/133-package-aware-download-and-orphan-bridge/133-04-SUMMARY.md`
- **Verification:** The active-milestone verification-traceability mutation suite and live checker passed.
- **Committed in:** Plan metadata commit.

**Total deviations:** 3 auto-fixed (1 bug, 1 missing critical, 1 blocking)

**Impact on plan:** Compatibility wording only; no behavioral or architectural scope changed.

## Issues Encountered

- The first full-verifier pass correctly rejected stale tracked LOC after adding the checker and mutation suite. Regenerating `docs/metrics/lines-of-code.md` with the repository generator restored freshness.
- Final phase-diff inspection found no new wire message, network-to-mempool dependency, arbitrary package graph, unbounded exact evidence, retained parent body cache, package fanout/receipt, cross-cache lifecycle projection, RPC package adapter, operator implementation, or broadened release claim. The existing RPC diff only maps the typed package-shape network error into the established internal failure path.

## Verification Evidence

- `cargo test -p open-bitcoin-network`: 490 passed.
- `cargo test -p open-bitcoin-node`: 502 passed; 1 explicitly opt-in public-network test ignored.
- `bazel build //packages/open-bitcoin-network:open_bitcoin_network_lib`: passed.
- `bazel build //packages/open-bitcoin-node:open_bitcoin_node_lib`: passed.
- Phase 133 checker mutation suite: 22 passed.
- Parity breadcrumbs: 444 registered sources validated.
- `bash scripts/verify.sh`: passed in 5m 44.257s.
- `git diff --check`: passed.

## Known Stubs

None. The created checker and test suite are fully wired into default verification, and no UI or production data path was introduced.

## Authentication Gates

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- PPKG-01 through PPKG-03 are ready to support Phase 134 lifecycle feedback without reopening identity, provenance, or admission authority.
- Phase 136 fanout/receipts and Phase 137 RPC/operator surfaces remain explicitly deferred and guarded against accidental early claims.
- No blockers remain.

## Self-Check: PASSED

- All declared created files exist.
- Task commits `adcbb9f4`, `7ab32c53`, and `b9b93575` exist.
- YAML frontmatter has exactly one opening and one closing delimiter.
- Summary diff passes `git diff --check`.

*Phase: 133-package-aware-download-and-orphan-bridge*
*Completed: 2026-07-26*
