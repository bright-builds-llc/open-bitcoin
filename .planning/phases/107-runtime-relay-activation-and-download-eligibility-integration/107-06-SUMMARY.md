---
phase: 107-runtime-relay-activation-and-download-eligibility-integration
plan: 06
subsystem: verification-closeout
tags:
  - relay
  - runtime-activation
  - download-eligibility
  - verification
  - planning-state

requires:
  - phase: 107-runtime-relay-activation-and-download-eligibility-integration
    provides: Plans 107-01 through 107-05 implementation, docs, evidence, and guardrails
  - phase: 106-parity-traceability-uat-and-release-boundary-guardrails
    provides: Deterministic v2.0 release-boundary and parity guardrails
provides:
  - Phase 107 verification evidence with status passed
  - Completed Phase 107 requirement and roadmap state
  - Phase 108 pending handoff state
  - Refreshed LOC report after verification-driven fixes
affects:
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - .planning/STATE.md
  - .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-VERIFICATION.md
  - docs/metrics/lines-of-code.md

tech-stack:
  added: []
  patterns:
    - Verification-first closeout
    - No-commit GSD execution under parent workflow ownership
    - Scoped checker repair for split source ownership

key-files:
  created:
    - .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-VERIFICATION.md
    - .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-06-SUMMARY.md
  modified:
    - .planning/REQUIREMENTS.md
    - .planning/ROADMAP.md
    - .planning/STATE.md
    - docs/metrics/lines-of-code.md
    - packages/open-bitcoin-node/src/network/tests.rs
    - packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - packages/open-bitcoin-network/src/peer/relay_download.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs
    - scripts/check-phase102-orphan-admission-bridge.ts
    - scripts/check-phase102-orphan-admission-bridge.test.ts

key-decisions:
  - "Mark Phase 107 requirements complete only after the focused checker, parity checks, Cargo checks, full workspace tests, and full repo verifier pass."
  - "Leave MEM-04, MEM-05, MEM-06, REL-01, and REL-02 pending under Phase 108."
  - "Treat default-off relay behavior as the current truthful RPC and test expectation unless relay activation is explicit."
  - "Do not create commits or push; the parent workflow owns final git history."

patterns-established:
  - "Closeout verification records exact commands and residual boundaries before planning state advances."
  - "Legacy checker evidence follows source splits instead of requiring moved logic to stay in old files."
  - "Coverage-only fixes stay behavior-focused and do not widen relay scope."

requirements-completed:
  - ACT-01
  - ACT-02
  - INV-02
  - INV-03
  - DL-01
  - DL-02
  - REL-03
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 107-2026-07-03T02-54-20
generated_at: 2026-07-03T06:42:28Z

duration: 1h 34m
completed: 2026-07-03
---

# Phase 107 Plan 06: Verification Evidence and Closeout Summary

**Phase 107 runtime relay activation and download eligibility is verified, documented, and closed while Phase 108 durable mempool recovery remains pending.**

## Performance

- **Duration:** 1h 34m
- **Started:** 2026-07-03T05:08:53Z
- **Completed:** 2026-07-03T06:42:28Z
- **Tasks:** 2
- **Files modified/created:** 14 closeout or verification-fix files, plus pre-existing Wave 1-5 worktree changes

## Accomplishments

- Ran the full required Phase 107 closeout command set, including the Phase 107 Bun checker/test, parity JSON parse, parity breadcrumbs, Cargo format/clippy/build/test, and full `bash scripts/verify.sh`.
- Created `107-VERIFICATION.md` with `status: passed`, exact command evidence, seven Phase 107 requirement IDs, evidence roots, verification fixes, and residual no-claim boundaries.
- Updated `.planning/REQUIREMENTS.md` so only `ACT-01`, `ACT-02`, `INV-02`, `INV-03`, `DL-01`, `DL-02`, and `REL-03` moved from pending to complete.
- Updated `.planning/ROADMAP.md` so Phase 107 is complete with six completed plans and Phase 108 remains pending.
- Updated `.planning/STATE.md` so the next position is Phase 108 pending/next, with `MEM-04`, `MEM-05`, `MEM-06`, `REL-01`, and `REL-02` still owned by Phase 108.

## Task Commits

No commits were created. The execution request explicitly instructed this executor not to commit or push.

1. **Task 1: Run final verification and create Phase 107 verification evidence** - complete, not committed here.
2. **Task 2: Close Phase 107 planning state without changing Phase 108 ownership** - complete, not committed here.

## Files Created/Modified

- `.planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-VERIFICATION.md` - Final Phase 107 passed verification evidence.
- `.planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-06-SUMMARY.md` - This closeout summary.
- `.planning/REQUIREMENTS.md` - Marks the seven Phase 107 requirements complete and leaves Phase 108 requirements pending.
- `.planning/ROADMAP.md` - Marks Phase 107 and all six plans complete; keeps Phase 108 pending.
- `.planning/STATE.md` - Advances current position to Phase 108 pending/next.
- `docs/metrics/lines-of-code.md` - Regenerated after verification-driven changes.
- `packages/open-bitcoin-node/src/network/tests.rs` and `packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs` - Align relay/download expectations with explicit relay activation and eligible outbound peers.
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - Aligns `localrelay` evidence with default-off runtime relay activation.
- `packages/open-bitcoin-network/src/peer/relay_download.rs` and transaction relay scheduler test files - Add coverage for runtime policy mutation, missing inbound record fallback, and defensive ineligible mapping.
- `scripts/check-phase102-orphan-admission-bridge.ts` and `.test.ts` - Track the split `request_orphan_parent` evidence in `peer/inventory_state.rs`.

## Decisions Made

- Phase 107 verification can pass only after both focused guardrails and the repo-native verifier pass on the current worktree.
- Default-off relay activation is the expected value for baseline-compatible `localrelay` unless activation is explicitly enabled.
- The Phase 102 checker should follow the actual ownership split for orphan-parent request logic rather than pinning evidence to `peer.rs`.
- Phase 108 remains the owner for durable mempool recovery and relay serving/fanout lifecycle gaps after restart.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Updated tests that expected transaction download from default-off inbound peers**
- **Found during:** Task 1 full Cargo test.
- **Issue:** Several managed network tests still expected getdata/in-flight behavior from ordinary inbound/default relay-off fixtures after Phase 107 added the download eligibility gate.
- **Fix:** Switched those expectations to explicit relay-enabled/outbound peer fixtures where transaction download is intended.
- **Files modified:** `packages/open-bitcoin-node/src/network/tests.rs`, `packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs`
- **Commit:** Not committed by request.

**2. [Rule 1 - Bug] Updated RPC `localrelay` expectation for default-off relay**
- **Found during:** Task 1 full Cargo test.
- **Issue:** `node_info_methods_return_documented_phase_8_fields` still expected `localrelay: true` despite Phase 107 preserving public relay off by default.
- **Fix:** Updated the assertion to expect `false`.
- **Files modified:** `packages/open-bitcoin-rpc/src/dispatch/tests.rs`
- **Commit:** Not committed by request.

**3. [Rule 3 - Blocking] Refreshed stale LOC report**
- **Found during:** Task 1 full repo verifier.
- **Issue:** `bash scripts/verify.sh` rejected stale `docs/metrics/lines-of-code.md`.
- **Fix:** Regenerated the tracked LOC report from the current worktree.
- **Files modified:** `docs/metrics/lines-of-code.md`
- **Commit:** Not committed by request.

**4. [Rule 3 - Blocking] Repaired Phase 102 checker after source split**
- **Found during:** Task 1 full repo verifier.
- **Issue:** The Phase 102 checker still looked for `request_orphan_parent` evidence only in `peer.rs`, but Phase 107 work had moved that ownership into `peer/inventory_state.rs`.
- **Fix:** Added `peer/inventory_state.rs` to the checker corpus and test fixtures, and verified the Phase 102 checker again.
- **Files modified:** `scripts/check-phase102-orphan-admission-bridge.ts`, `scripts/check-phase102-orphan-admission-bridge.test.ts`
- **Commit:** Not committed by request.

**5. [Rule 1 - Bug] Added coverage for uncovered relay eligibility branches**
- **Found during:** Task 1 full repo verifier coverage gate.
- **Issue:** Coverage reported uncovered lines in `relay_download.rs` and the scheduler defensive mapping branch.
- **Fix:** Added behavior tests for policy setter mutation, missing inbound admission record fallback, and ineligible decision mapping.
- **Files modified:** `packages/open-bitcoin-network/src/peer/relay_download.rs`, `packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs`, `packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs`
- **Commit:** Not committed by request.

## Issues Encountered

- The first full `cargo test --manifest-path packages/Cargo.toml --all-features` run exposed stale tests after the Phase 107 default-off relay eligibility gate.
- The first full `bash scripts/verify.sh` run exposed LOC drift, Phase 102 checker drift from a source split, and pure coverage gaps.
- All blocking issues were fixed in scope and the required exact verification commands were rerun or completed successfully.

## Known Stubs

None. A targeted scan of the Plan 107-06 touched code/test/checker files found no `TODO`, `FIXME`, placeholder, coming-soon, not-available text, or hardcoded empty UI/data stubs.

## Threat Flags

None. This closeout added verification evidence, planning state updates, test/checker repairs, and coverage tests. It did not add a new network endpoint, auth path, schema boundary, file-access trust boundary, service-bit behavior, public relay default, compact block behavior, package relay, bloom/filter serving, public-network verifier gate, or durable mempool recovery implementation.

## Verification

- `bun test scripts/check-phase107-runtime-relay-activation-download-eligibility.test.ts` - passed.
- `bun run scripts/check-phase107-runtime-relay-activation-download-eligibility.ts` - passed.
- `node -e "JSON.parse(require('fs').readFileSync('docs/parity/index.json','utf8')); JSON.parse(require('fs').readFileSync('docs/parity/source-breadcrumbs.json','utf8'));"` - passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` - passed.
- `cargo fmt --manifest-path packages/Cargo.toml --all --check` - passed.
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` - passed.
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` - passed.
- `cargo test --manifest-path packages/Cargo.toml --all-features` - passed.
- `bash scripts/verify.sh` - passed.
- `git status --short` - run as the final closeout status check; no commit was created.

## User Setup Required

None.

## Next Phase Readiness

Phase 108 can start from a verified runtime activation/download eligibility boundary. It should keep ownership of durable mempool recovery plus `MEM-04`, `MEM-05`, `MEM-06`, `REL-01`, and `REL-02`.

## Self-Check: PASSED

- Created `.planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-VERIFICATION.md`.
- Created `.planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-06-SUMMARY.md`.
- Verified Phase 107 requirements are complete in `.planning/REQUIREMENTS.md` and Phase 108 requirements remain pending.
- Verified `.planning/ROADMAP.md` lists all six Phase 107 plans as complete and leaves Phase 108 pending.
- Verified `.planning/STATE.md` states Phase 107 Complete and Phase 108 Pending/next.
- No commits were created, matching the execution request.

*Phase: 107-runtime-relay-activation-and-download-eligibility-integration*
*Completed: 2026-07-03*
