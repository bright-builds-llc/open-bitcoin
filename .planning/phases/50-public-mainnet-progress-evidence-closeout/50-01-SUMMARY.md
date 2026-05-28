---
phase: 50-public-mainnet-progress-evidence-closeout
plan: 01
subsystem: public-mainnet-evidence-closeout
tags:
  - live-mainnet-smoke
  - parity
  - release-readiness
requires:
  - .planning/phases/50-public-mainnet-progress-evidence-closeout/50-CONTEXT.md
  - .planning/phases/50-public-mainnet-progress-evidence-closeout/50-RESEARCH.md
provides:
  - .planning/phases/50-public-mainnet-progress-evidence-closeout/50-UAT.md
  - docs/parity/release-readiness.md
  - docs/parity/checklist.md
  - docs/parity/index.json
affects:
  - docs/parity/threat-model-v1.3.md
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
tech-stack:
  added: []
  patterns:
    - opt-in live public-mainnet evidence remains local under packages/target
    - diagnosed blocker closeout records typed cause, endpoint outcomes, snapshots, and next action
key-files:
  created:
    - .planning/phases/50-public-mainnet-progress-evidence-closeout/50-UAT.md
  modified:
    - docs/parity/release-readiness.md
    - docs/parity/checklist.md
    - docs/parity/index.json
    - docs/parity/threat-model-v1.3.md
    - .planning/REQUIREMENTS.md
    - .planning/ROADMAP.md
key-decisions:
  - Phase 50 closed through diagnosed blocker evidence, not successful header/block progress.
  - Same-datadir restart/resume evidence is not claimed as success because no header or block progress was observed.
  - Generated live-smoke and support-bundle reports remain local artifacts outside git.
requirements-completed:
  - PROOF-03
  - PROOF-04
  - PROOF-05
  - SEC-03
duration: 10 min
completed: 2026-05-28
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 50-2026-05-28T03-06-48
generated_at: 2026-05-28T03:38:28Z
---

# Phase 50 Plan 01: Public Mainnet Evidence Closeout Summary

Closed v1.3 public-mainnet evidence through a diagnosed blocker UAT packet with
local live-smoke reports, support evidence, parity-root updates, and requirement
traceability.

## Execution

- Started: 2026-05-28T03:28:19Z
- Finished: 2026-05-28T03:38:28Z
- Tasks completed: 3/3
- Files changed: 8 tracked files plus local generated evidence under
  `packages/target`

## Local Evidence

Generated live-smoke and support artifacts are local review evidence and are not
checked into git.

| Artifact | Path | Result |
| --- | --- | --- |
| Default live-smoke JSON | `packages/target/live-mainnet-smoke-reports/phase50-default/open-bitcoin-live-mainnet-smoke.json` | `no_progress`, `handshake_failure` |
| Relative manual-peer live-smoke JSON | `packages/target/live-mainnet-smoke-reports/phase50-manual-peer/open-bitcoin-live-mainnet-smoke.json` | `runtime_failed`; rejected as selected evidence because a relative generated-config path was interpreted under the datadir |
| Selected manual-peer live-smoke JSON | `packages/target/live-mainnet-smoke-reports/phase50-manual-peer-absolute/open-bitcoin-live-mainnet-smoke.json` | `no_progress`, `handshake_failure` |
| Support evidence JSON | `packages/target/phase50-support/support-evidence.json` | Generated |
| Support evidence Markdown | `packages/target/phase50-support/support-evidence.md` | Generated |

## UAT Result

`.planning/phases/50-public-mainnet-progress-evidence-closeout/50-UAT.md`
records the selected closeout report:

- `result.status=no_progress`
- `result.progressDetected=false`
- `result.headerDelta=0`
- `result.blockDelta=0`
- `result.maybeNoProgressCause=handshake_failure`
- `result.nextAction=Inspect daemon stderr and peer endpoint outcomes; retry with a different manual peer if the endpoint accepts TCP but does not complete the Bitcoin handshake.`
- 79 manual-peer endpoint outcomes
- 24 durable status snapshots
- final durable status: header height 0, block height 0, messages processed 0,
  outbound peers 0

The same datadir was reused across attempts. Restart/resume is recorded as
`satisfied-by-diagnosed-blocker`; no restart/resume success is claimed.

## Tracked Updates

- Added Phase 50 UAT with commands, artifact paths, selected report fields,
  endpoint outcomes, durable status evidence, support bundle evidence,
  requirement verdicts, and next operator action.
- Added `## Phase 50 Evidence Closeout` to
  `docs/parity/release-readiness.md`.
- Added `v1-3-public-mainnet-progress-evidence-closeout` to
  `docs/parity/checklist.md` and `docs/parity/index.json`.
- Replaced stale "Phase 50 still owns closeout" language in
  `docs/parity/threat-model-v1.3.md`.
- Marked PROOF-03, PROOF-04, PROOF-05, SEC-03, and Phase 50 complete in GSD
  requirements and roadmap artifacts.

## Commits

Per-task commits are represented by the final plan completion commit for this
docs/evidence closeout plan.

## Deviations from Plan

**[Rule 1 - Bug] Relative manual-peer generated-config path** - Found during:
Task 1. The relative manual-peer command generated a config path that
`open-bitcoind` interpreted relative to the datadir, so the daemon exited before
the first status snapshot. Fix: reran the same-datadir manual-peer smoke with
absolute datadir and output paths. Verification: the absolute-path report
generated 24 snapshots and a typed `handshake_failure` no-progress diagnosis.

**[Rule 2 - Missing Critical] Stale threat-model closeout language** - Found
during: Task 3. `docs/parity/threat-model-v1.3.md` still said Phase 50 owned the
final closeout after UAT existed. Fix: updated the threat model residual risk and
threat register row to reference the diagnosed-blocker closeout.

**Total deviations:** 2 auto-fixed. **Impact:** Evidence is more accurate and
does not overclaim progress or restart/resume success.

## Verification To Run

```bash
rg -n "Phase 50 Public Mainnet Evidence UAT|Selected Closeout Report|Requirement Verdicts|Next Operator Action" .planning/phases/50-public-mainnet-progress-evidence-closeout/50-UAT.md
rg -n "Phase 50 Evidence Closeout|v1-3-public-mainnet-progress-evidence-closeout|50-UAT.md" docs/parity/release-readiness.md docs/parity/checklist.md docs/parity/index.json
rg -n "run-live-mainnet-smoke" scripts/verify.sh
cargo fmt --manifest-path packages/Cargo.toml --all
cargo clippy --manifest-path packages/Cargo.toml --workspace --all-targets --all-features -- -D warnings
cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features
cargo test --manifest-path packages/Cargo.toml --workspace --all-features
bash scripts/verify.sh
```

The `rg -n "run-live-mainnet-smoke" scripts/verify.sh` command must return no
matches.

## Self-Check: PASSED

- Phase UAT exists and records the selected diagnosed-blocker closeout.
- Same-datadir second valid invocation is recorded without claiming progress.
- Support evidence was generated and linked from UAT.
- Parity roots link Phase 50 UAT and no longer say Phase 50 remains open.
- Requirements and roadmap mark Phase 50 complete.
