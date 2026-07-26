---
phase: 133-package-aware-download-and-orphan-bridge
plan: "02"
subsystem: network
tags: [rust, transaction-relay, orphanage, peer-provenance, bitcoin-knots-parity]

requires:
  - phase: 133-01
    provides: Fixed-memory hard and reconsiderable reject evidence with authoritative tip reset
provides:
  - Deterministic receipt provenance captured before scheduler cleanup
  - Shared orphan bodies with bounded multi-peer announcer ownership
  - Newest-first same-peer 1P1C candidate traversal over ordinary transaction messages
  - PeerManager-owned orphan lifecycle, late inventory, and disconnect cleanup
affects: [133-03, 133-04, transaction-download, package-admission, orphan-reconsideration]

tech-stack:
  added: []
  patterns:
    - Transient scheduler provenance converted into bounded retained orphan evidence
    - Opaque consume-only candidate types as eligibility proofs
    - One pure network owner for download, orphan, evidence, and disconnect state

key-files:
  created:
    - packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs
  modified:
    - packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs
    - packages/open-bitcoin-network/src/peer/inventory_state.rs
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-node/src/network/admission_bridge.rs
    - packages/open-bitcoin-node/src/network/action_translation.rs
    - packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Capture receipt provenance before request cleanup, deterministically unioning txid and wtxid announcers while always retaining the delivering peer."
  - "Retain one orphan body with a policy-bounded announcer set; late inventory may add ownership but cannot replace the body, alter dependencies, or refresh TTL."
  - "Treat the opaque same-peer 1P1C candidate as the eligibility proof and expose only consume-only ordered parts to node admission."
  - "Co-locate scheduler, orphanage, evidence, and disconnect mutation under PeerManager so node adapters cannot reconstruct provenance qualification."

patterns-established:
  - "Provenance boundary: scheduler snapshots bounded transient announcers before cleanup, then orphanage applies its independent retained cap."
  - "Candidate boundary: private construction proves reconsiderable parent evidence, same-peer announcement, newest-first ordering, and hard-reject exclusion."
  - "Late inventory boundary: only an already-retained orphan may gain an announcer, with no body clone or lifetime refresh."

requirements-completed: []
requirements-addressed: [PPKG-01, PPKG-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 133-2026-07-26T16-12-51
generated_at: 2026-07-26T20:01:32Z

duration: 1h 10m
completed: 2026-07-26
---

# Phase 133 Plan 02: Provenance-Aware Orphan Bridge Summary

**Bounded scheduler provenance now feeds shared multi-announcer orphan bodies and opaque newest-first same-peer 1P1C candidates through one PeerManager-owned lifecycle.**

## Performance

- **Duration:** 1h 10m
- **Started:** 2026-07-26T18:52:17Z
- **Completed:** 2026-07-26T20:01:32Z
- **Tasks:** 3
- **Files modified:** 28

## Accomplishments

- Captured deterministic txid/wtxid receipt announcers before scheduler cleanup while always preserving the delivering peer.
- Replaced single-owner orphan entries with one shared body, bounded announcers, per-peer ownership counts, and centralized lifecycle cleanup.
- Added a newest-first parent index and bounded cursor that produce only opaque `[parent, child]` candidates with aligned same-peer origins.
- Moved orphan ownership under `PeerManager`, routed real receipt provenance into admission staging, and attached late ordinary inventory without replacing bodies or refreshing expiry.
- Preserved existing node admission seams through narrow owner methods while preventing node code from reconstructing announcer eligibility.
- Anchored all new first-party Rust sources and tests to pinned Bitcoin Knots orphan and opportunistic 1P1C evidence.

## Task Commits

Each task was committed atomically:

1. **Task 1: Capture receipt announcers before scheduler cleanup** - `3a90c465`
2. **Task 2: Build shared-body newest-first candidate selection** - `d1e10825`
3. **Task 3: Co-locate orphan/download ownership and preserve node seams** - `9f33dd63`

## Files Created/Modified

- `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs` - Private cursor and opaque same-peer 1P1C candidate proof.
- `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs` - Bounded announcers, shared orphan bodies, parent index, lifecycle cleanup, and candidate selection.
- `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs` - Receipt provenance captured before pending relay cleanup.
- `packages/open-bitcoin-network/src/peer/inventory_state.rs` - Typed received-transaction actions and late-inventory announcer routing.
- `packages/open-bitcoin-network/src/peer.rs` - Unified transaction-download, orphanage, evidence, and disconnect owner APIs.
- `packages/open-bitcoin-network/src/peer/tests.rs` - Owner-boundary provenance, late inventory, disconnect, and candidate lifecycle coverage.
- `packages/open-bitcoin-node/src/network/admission_bridge.rs` - Provenance-aware orphan staging through `PeerManager`.
- `packages/open-bitcoin-node/src/network/action_translation.rs` - Typed receipt translation and unified disconnect cleanup.
- `packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs` - Node seam regressions for staging, reconsideration, caps, and disconnect.
- `scripts/check-phase102-orphan-admission-bridge.ts` - Updated architectural guardrail for PeerManager-owned cleanup order.
- `docs/parity/source-breadcrumbs.json` - Pinned source and functional-test lineage for the new candidate module.
- `docs/metrics/lines-of-code.md` - Refreshed tracked source metrics through normal hooks.

## Decisions Made

- Receipt provenance is a transient deterministic snapshot of already-bounded scheduler state; the orphanage independently applies its retained announcer cap.
- `delivered_by` remains mandatory even when the scheduler has no prior announcement for that peer, preventing delivery evidence from being lost during deduplication.
- Orphan lifetime and dependency identity belong to the retained body. Adding an announcer changes only ownership evidence and per-peer accounting.
- Newest-first candidate traversal is bounded per parent and never aggregates siblings, grandchildren, multiple parents, or arbitrary graphs.
- Node code receives typed provenance and consume-only candidate parts, while `PeerManager` remains the only mutation authority for scheduler, orphan, evidence, and peer cleanup state.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Preserved the enforced production file-length boundary**

- **Found during:** Tasks 1 and 2
- **Issue:** Provenance and candidate integration pushed existing production modules toward or beyond the repository's 628-line limit.
- **Fix:** Compacted scheduler helpers and split the private candidate implementation into `orphanage/candidate.rs`.
- **Files modified:** `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs`, `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs`, `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage/candidate.rs`
- **Verification:** The full repository verifier's file-length gate passed.
- **Committed in:** `3a90c465`, `d1e10825`

**2. [Rule 3 - Blocking] Registered generated parity and source-metric artifacts**

- **Found during:** Tasks 1 and 2 normal-hook verification
- **Issue:** New and reorganized first-party Rust files required refreshed parity breadcrumbs and tracked LOC metrics.
- **Fix:** Registered the candidate source with exact Knots anchors and retained the hook-generated LOC refreshes.
- **Files modified:** `docs/parity/source-breadcrumbs.json`, `docs/metrics/lines-of-code.md`
- **Verification:** The parity checker verified all 442 current Rust files and normal hooks accepted the generated metrics.
- **Committed in:** `3a90c465`, `d1e10825`

**3. [Rule 3 - Blocking] Closed strict coverage gaps at narrow owner boundaries**

- **Found during:** Tasks 2 and 3 normal-hook verification
- **Issue:** Full-workspace coverage exposed unexercised bounded candidate branches and thin `PeerManager` delegation seams.
- **Fix:** Added direct behavior tests for candidate limits, cleanup/index invariants, provenance ownership, lifecycle delegation, late inventory, and disconnect.
- **Files modified:** `packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs`, `packages/open-bitcoin-network/src/peer/tests.rs`, `packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs`
- **Verification:** The complete workspace coverage gate passed in the Task 3 commit hook.
- **Committed in:** `d1e10825`, `9f33dd63`

**4. [Rule 3 - Blocking] Updated the Phase 102 architectural guardrail for unified ownership**

- **Found during:** Task 3 full verification
- **Issue:** The historical checker still required node-owned orphan cleanup, which the planned `PeerManager` ownership migration intentionally removed.
- **Fix:** Changed the guardrail to require transaction-download and orphan cleanup in the unified peer owner before node-local admission cleanup, with fixture regressions for missing and reversed calls.
- **Files modified:** `scripts/check-phase102-orphan-admission-bridge.ts`, `scripts/check-phase102-orphan-admission-bridge.test.ts`
- **Verification:** The guardrail tests and complete repository verifier passed.
- **Committed in:** `9f33dd63`

**Total deviations:** 4 auto-fixed (1 missing-critical code-shape fix, 3 blocking verification fixes)

**Impact on plan:** Every deviation enforced existing repository contracts or updated a historical guardrail for the planned ownership migration. No package wire protocol, mempool dependency, adapter expansion, or architectural scope was added.

## Issues Encountered

- The plan's broad non-forgeability regex also matches the legitimate public `TxOrphanage::new` constructor. The candidate-specific check passes: `SamePeerOneParentOneChildCandidate` exposes no public fields or public constructor.
- No authentication, external-service, or architectural gate was encountered.

## Verification

- Receipt provenance suite passed: 2 tests.
- Orphan candidate suite passed: 20 tests.
- Node admission bridge suite passed: 24 tests.
- `bun run scripts/check-parity-breadcrumbs.ts` verified 442 Rust files.
- `git diff --check` passed.
- Candidate-specific non-forgeability, pure-network boundary, typed receipt, retained announcer, unified ownership, and no-node-owned-orphan acceptance checks passed.
- Task 3's normal commit hook completed the full `bash scripts/verify.sh` contract in 3m 53.593s, including formatting, Clippy with warnings denied, all-target build, all-feature tests, coverage, architecture checks, and Bazel smoke build.

## Authentication Gates

None.

## Known Stubs

None. Empty-array and default-object matches are test-fixture initialization or checker accumulation, and the only `null` comparison is a fail-closed checker branch.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 133-03 can consume the opaque same-peer `[parent, child]` candidate without reconstructing origin eligibility.
- Plan 133-04 can build lifecycle and operator seams over one coherent pure-network owner.
- PPKG-01 bounds and the pure ordinary-message network boundary remain intact.
- No Plan 02 blockers remain.

## Self-Check: PASSED

- Summary and candidate source files exist.
- Task commits `3a90c465`, `d1e10825`, and `9f33dd63` exist in repository history.
- All 28 implementation files are represented by the atomic task commit range.
- Stub scan found no production placeholders; empty values were limited to test fixtures and checker state.
- No new network endpoint, authentication path, file-access boundary, or schema trust surface was introduced.
- Summary and implementation diffs are whitespace-clean.
