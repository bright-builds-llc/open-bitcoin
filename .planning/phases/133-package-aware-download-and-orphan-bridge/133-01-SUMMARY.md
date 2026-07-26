---
phase: 133-package-aware-download-and-orphan-bridge
plan: "01"
subsystem: network
tags: [rust, transaction-relay, rolling-filter, package-policy, bitcoin-knots-parity]

requires: []
provides:
  - Fixed-memory three-generation hard and reconsiderable reject evidence
  - Wtxid-authoritative scheduler suppression with parent reconsideration bypass
  - Shell-seeded paired evidence reset at authoritative active-tip seams
affects: [133-02, 133-03, 133-04, transaction-download, orphan-assembly]

tech-stack:
  added: []
  patterns:
    - Typed probabilistic evidence wrappers over a private fixed-allocation core
    - Shell-owned entropy injected into deterministic network state
    - Boolean local scheduling facts instead of cloned reject collections

key-files:
  created:
    - packages/open-bitcoin-network/src/peer/transaction_relay/reject_evidence.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/tests/reject_evidence_cases.rs
  modified:
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/peer/inventory_state.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs
    - packages/open-bitcoin-node/src/network/relay_serving.rs
    - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
    - packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Reject evidence accepts only Wtxid or typed package fingerprints; txid-only inventory consults evidence only through an authoritative txid-to-wtxid mapping."
  - "Ordinary inventory consults hard and reconsiderable evidence, while orphan-parent requests bypass reconsiderable evidence and still honor hard rejects."
  - "Both evidence domains reset together through one PeerManager method immediately after successful chainstate connect or reorg mutation."
  - "Production tweak entropy is derived in the node shell with RandomState, while network constructors retain fixed-tweak deterministic seams."

patterns-established:
  - "Probabilistic evidence may suppress redundant work but cannot produce peer punishment, disconnect, or misbehavior decisions."
  - "Active-tip reset pattern: mutate authoritative chainstate successfully, inject a fresh shell tweak, then reset both evidence domains atomically."

requirements-completed: []
requirements-addressed: [PPKG-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 133-2026-07-26T16-12-51
generated_at: 2026-07-26T18:41:26Z

duration: 1h 13m
completed: 2026-07-26
---

# Phase 133 Plan 01: Fixed-Memory Reject Evidence and Tip Reset Summary

**Typed three-generation reject evidence now bounds adversarial memory, preserves parent-fetch reconsideration, and resets both domains only at authoritative active-tip mutations**

## Performance

- **Duration:** 1h 13m
- **Started:** 2026-07-26T17:28:12Z
- **Completed:** 2026-07-26T18:41:26Z
- **Tasks:** 3
- **Files modified:** 16

## Accomplishments

- Replaced the exact recent-reject collection with a preallocated three-generation rolling filter sized for 120,000 entries at a 0.000001 false-positive target.
- Added separate typed hard-transaction and reconsiderable transaction/package domains without a public raw-byte insertion API.
- Changed scheduler integration to consume boolean evidence facts, suppress ordinary inventory with both domains, and allow orphan-parent requests through reconsiderable evidence.
- Prevented txid bytes from being substituted as wtxids; txid-only inventory consults reject evidence only when an authoritative local mapping exists.
- Seeded production filters from node-shell entropy and reset both filters together after successful local connect, stored connect, and reorg transitions.
- Proved fixed allocation under one million unique insertions and preserved evidence across duplicate, non-extending, disconnected, and failed chain transitions.

## Task Commits

Each TDD task was committed as a RED/GREEN pair:

1. **Task 1 RED: Rolling reject evidence behavior** - `12c975b2`
2. **Task 1 GREEN: Fixed-memory typed reject evidence** - `23ed2ed3`
3. **Task 2 RED: Semantic scheduler suppression behavior** - `02830614`
4. **Task 2 GREEN: PeerManager and scheduler evidence integration** - `802663bb`
5. **Task 3 RED: Active-tip reset behavior** - `671f1493`
6. **Task 3 GREEN: Production tweak injection and authoritative reset wiring** - `7270ce03`

## Files Created/Modified

- `packages/open-bitcoin-network/src/peer/transaction_relay/reject_evidence.rs` - Private fixed-allocation rolling core and typed evidence wrappers.
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests/reject_evidence_cases.rs` - Sizing, rotation, domain, reset, deterministic-vector, and million-insert regressions.
- `packages/open-bitcoin-network/src/peer.rs` - Node-global evidence ownership, seeded constructors, authoritative identity mapping, and paired reset API.
- `packages/open-bitcoin-network/src/peer/inventory_state.rs` - Wtxid-authoritative boolean fact projection.
- `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs` - Ordinary-versus-parent reconsiderable suppression semantics.
- `packages/open-bitcoin-network/src/peer/tests.rs` - Integration, no-punishment, txid identity, and paired-reset coverage.
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs` - Hard/reconsiderable scheduling matrix and parent bypass tests.
- `packages/open-bitcoin-node/src/network/admission_bridge.rs` - Admission rejection recording through the typed hard-reject API.
- `packages/open-bitcoin-node/src/network/relay_serving.rs` - Production RandomState tweak injection at the node shell.
- `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` - Successful local, stored, and reorg active-tip reset seams.
- `packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs` - Positive and negative authoritative reset tests.
- `docs/parity/source-breadcrumbs.json` - Direct Knots bloom and transaction-download evidence registration.
- `docs/metrics/lines-of-code.md` - Hook-regenerated tracked line-count report.

## Decisions Made

- The rolling core stores only probabilistic membership bits; no exact backup set, per-peer map, TTL clock, or mempool dependency was retained.
- Hard and reconsiderable filters use separate typed hash domains even when seeded with the same injected tweak.
- Unknown txid-only inventory never hashes txid bytes into a wtxid filter; only locally observed transaction identity can authorize that lookup.
- The active-tip reset happens immediately after authoritative chainstate success, before downstream projection, so evidence tracks actual chain mutation rather than receipt classification.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Completed strict coverage for the rolling filter**

- **Found during:** Task 1 pre-commit verification
- **Issue:** The repository hook rejected uncovered defensive construction, generation, and wrapper branches in the new pure network core.
- **Fix:** Added behavioral coverage for invalid sizing, deterministic hashing, generation expiry, reset/reseed, domain separation, and fixed allocation.
- **Files modified:** `packages/open-bitcoin-network/src/peer/transaction_relay/tests/reject_evidence_cases.rs`
- **Verification:** The normal hook's network coverage gate and full `bash scripts/verify.sh` passed.
- **Committed in:** `12c975b2`, `23ed2ed3`

**2. [Rule 3 - Blocking] Updated the admission bridge and peer integration tests**

- **Found during:** Task 2 compilation
- **Issue:** Replacing `note_recent_reject` made the existing node admission bridge and peer integration suite fail to compile even though those files were omitted from the task's narrow file list.
- **Fix:** Routed rejection outcomes through `record_hard_reject(Wtxid)` and updated integration tests to exercise typed hard/reconsiderable evidence without punishment.
- **Files modified:** `packages/open-bitcoin-node/src/network/admission_bridge.rs`, `packages/open-bitcoin-network/src/peer/tests.rs`
- **Verification:** The 77 focused transaction-relay tests and full repository verifier passed.
- **Committed in:** `02830614`, `802663bb`

**3. [Rule 3 - Blocking] Preserved the Phase 101 stable test evidence anchor**

- **Found during:** Task 2 RED commit hook
- **Issue:** Renaming the expanded scheduler regression caused the Phase 101 static verifier to lose its required historical test-name anchor.
- **Fix:** Restored `already_have_recent_reject_and_mempool_known_suppress_requests` as the wrapper name while retaining the new hard/reconsiderable and parent-bypass assertions.
- **Files modified:** `packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs`
- **Verification:** The Phase 101 checker and full normal hook passed.
- **Committed in:** `02830614`

**4. [Rule 3 - Blocking] Added direct coverage for the paired network reset**

- **Found during:** Task 3 RED commit hook
- **Issue:** Node lifecycle tests proved the production behavior, but the repository's network-only coverage profile still reported `PeerManager::on_active_tip_changed` as uncovered.
- **Fix:** Added a narrow network Arrange/Act/Assert test using fixed tweaks and both evidence domains.
- **Files modified:** `packages/open-bitcoin-network/src/peer/tests.rs`
- **Verification:** The normal hook reported no uncovered network lines and the full verifier passed.
- **Committed in:** `671f1493`

**5. [Rule 3 - Blocking] Staged the addressed requirement for phase-level verification**

- **Found during:** Summary metadata closeout
- **Issue:** Phase lifecycle rules reserve requirement completion for the phase verifier after all Phase 133 plans finish.
- **Fix:** Recorded PPKG-01 under `requirements-addressed` and left `requirements-completed` empty.
- **Files modified:** `.planning/phases/133-package-aware-download-and-orphan-bridge/133-01-SUMMARY.md`
- **Verification:** Summary self-check and metadata commit hook.

**6. [Rule 3 - Blocking] Preserved metrics in the repository's aggregate state format**

- **Found during:** State closeout
- **Issue:** The GSD `state record-metric` command could not parse the repository's colon-aligned aggregate performance table and returned `recorded: false`.
- **Fix:** Left the aggregate table intact and added a separate Plan Execution History table containing the exact Plan 133-01 duration, task count, and file count.
- **Files modified:** `.planning/STATE.md`
- **Verification:** State self-check confirms Plan 2 of 4, 90% milestone progress, and the Phase 133 P01 metric row.

**Total deviations:** 6 auto-fixed blocking issues.
**Impact on plan:** Each change enforced existing compilation, parity, coverage, or lifecycle contracts without expanding the runtime surface.

## Issues Encountered

- The first Task 2 hook rejected a renamed Phase 101 test anchor; restoring the stable wrapper name preserved historical verifier evidence.
- The first Task 3 hook exposed a difference between node behavior coverage and the network-only coverage profile; one direct network test closed the gap.
- All verification used the checkout-isolated target directory `/private/tmp/open-bitcoin-phase133-target.xgN5jr`; hook-created `packages/target` artifacts were moved out of the repository after each run.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plans 133-02 and 133-03 can consume bounded typed rejection evidence while assembling same-peer orphan/package candidates.
- Plan 133-04 can verify PPKG-01 at the phase lifecycle boundary.
- No blockers or threat flags remain; this plan adds no endpoint, authentication path, filesystem trust boundary, or schema change.

## Self-Check: PASSED

- All 16 created or modified implementation/evidence files and this summary exist.
- Task commits `12c975b2`, `23ed2ed3`, `02830614`, `802663bb`, `671f1493`, and `7270ce03` resolve as commits.
- The three focused suites, parity breadcrumb checker, whitespace checks, and latest full normal-hook verifier pass.
- The summary has exactly one YAML frontmatter block and no body `---` separators.

*Phase: 133-package-aware-download-and-orphan-bridge*
*Completed: 2026-07-26*
