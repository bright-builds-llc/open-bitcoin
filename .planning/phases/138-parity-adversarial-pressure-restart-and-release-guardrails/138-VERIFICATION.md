---
phase: 138-parity-adversarial-pressure-restart-and-release-guardrails
verified: 2026-08-22T10:38:28Z
status: passed
score: 5/5 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 138-2026-08-22T04-13-58
generated_at: 2026-08-22T10:38:28Z
lifecycle_validated: true
overrides_applied: 0
re_verification:
  previous_status: passed
  previous_score: 4/4
  gaps_closed: []
  gaps_remaining: []
  regressions: []
---

# Phase 138: Parity, Adversarial Pressure, Restart, and Release Guardrails Verification Report

**Phase Goal:** Contributors and operators have deterministic proof that the integrated v2.2 behavior matches its pinned Knots anchors, remains bounded, and does not broaden release claims.
**Verified:** 2026-08-22T10:38:28Z
**Status:** passed
**Re-verification:** No — initial gsd-verifier pass. An executor-authored `138-VERIFICATION.md` already existed with `status: passed` and no `gaps:` section, so this run used initial-mode evidence checks rather than gap closure.

## Goal Achievement

Phase 138 owns MPVFY-01 through MPVFY-04. Live code, checkers, and claim docs deliver the named 4×6 inventory plus restart composition, Instant-free PRESS-05 work-count smoke, exactly-once surface owners, a committed UAT package, the locked D-21 sentence, and a last-gate checker after Phase 117. Leftover v2.2 requirement rows are Complete and the milestone remains unarchived.

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Pinned fixtures, fake-clock scenarios, randomized graph-oracle tests, and failure injection cover package, rolling-fee, pressure, expiry, recovery, and retry behavior, including the named restart composition. | ✓ VERIFIED | `matrix.ts` pins all 24 method×behavior cells; `COMPOSITION_SYMBOL` is `recovery_restart_preserves_local_package_unbroadcast_remints_retry_and_keeps_injected_install_failure_inert` in `restart_composition.rs`; `mod restart_composition` is registered; live checker names missing cells. |
| 2 | Package and sustained-pressure benchmarks enforce documented bounded-work and performance expectations without public-network or wall-clock gates in default verification. | ✓ VERIFIED | `SUSTAINED_PRESSURE_MAX_ELAPSED` and `Instant::now` are gone from `mempool.rs`; `SUSTAINED_PRESSURE_TRIM_CYCLES = 24`, one retained entry, and rolling-fee bump remain; `threshold_free` stays in `check-benchmark-report.ts`; `verify.sh` `run_step` lines have no `public-network` or `wall-clock`. |
| 3 | Parity catalogs, source breadcrumbs, operator docs, and repo-local Cargo and Bazel UAT commands identify exact Knots anchors, intentional differences, and evidence boundaries for every v2.2 surface. | ✓ VERIFIED | Index/checklist own PACK, PRESS, PPKG-04/IBR, and MPVFY exactly once; `138-UAT.md` publishes the ten Cargo/Bazel forms plus `status: not run`; D-21 sentence is in README, runtime-guide, release-readiness, and support-matrix; required Knots anchors sit on the closeout surface. |
| 4 | Deterministic guardrails require bounded local-package, same-peer 1P1C, ordinary transaction fanout, and initial-broadcast-retry wording while rejecting general package wire, whole-mempool rebroadcast, public/default/production relay, guaranteed propagation, public-network CI, and production-readiness claims. | ✓ VERIFIED | `checkPhase138ParityUatReleaseBoundary` is the last `check-phase*` after Phase 117 in both `VERIFY_COMMAND_ORDER` and `run_step`; claim corpus requires D-21 and denies D-22 plus unscoped `package relay`; checker source has no `fetch(`, `Bun.spawn`, or `node:child_process`. |
| 5 | Every leftover implemented-Pending v2.2 requirement is Complete, required surfaces are `done` without renaming the Phase 135 human title, and the milestone is not archived. | ✓ VERIFIED | REQUIREMENTS leftover PACK/PPKG-04/MPDUR/IBR/MPOBS/MPVFY rows are `- [x]` / Complete; nine v2.2 surfaces including `v2 snapshot schema, checkpointing, and recovery` are `done`; PROJECT/STATE keep `/gsd-complete-milestone v2.2` as a future command. |

**Score:** 5/5 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| -------- | -------- | ------ | ------- |
| `packages/open-bitcoin-node/src/network/tests/recovery_cases/restart_composition.rs` | D-04/D-05 restart × package × unbroadcast × remint × inject composition | ✓ VERIFIED | Named test uses `PolicyTime`, `RecoveryInstallFailureGuard::inject`, rolling-fee reset, and remint; no `Instant::now` / sleep / process. |
| `packages/open-bitcoin-bench/src/cases/mempool.rs` | Work-count sustained-pressure smoke without Instant ceiling | ✓ VERIFIED | Keeps `SUSTAINED_PRESSURE_TRIM_CYCLES`, one-entry, and rolling-fee bump checks. |
| `scripts/check-phase131-rolling-fee-expiry-pressure.ts` | PRESS-05 pin retargeted to oracle + work-count symbols | ✓ VERIFIED | Pins `SUSTAINED_PRESSURE_TRIM_CYCLES`; last-gate needle is now Phase 138 while still requiring Phase 117. |
| `docs/parity/index.json` | Backfilled PACK/PRESS/136/MPVFY owners; closeout `done` | ✓ VERIFIED | Four new surfaces plus closeout exist; all required v2.2 top-level statuses are `done`. |
| `docs/parity/checklist.md` | Mirrored human owners | ✓ VERIFIED | Same four IDs plus Phase 135 human title `v2 snapshot schema, checkpointing, and recovery` as Done. |
| `.planning/phases/138-parity-adversarial-pressure-restart-and-release-guardrails/138-UAT.md` | Committed deterministic UAT; optional public-network not run | ✓ VERIFIED | Contains the ten Cargo/Bazel strings, `bash scripts/verify.sh`, and `status: not run`. |
| `scripts/check-phase138-parity-uat-release-boundary.ts` | Thin CLI plus re-export | ✓ VERIFIED | Exports `checkPhase138ParityUatReleaseBoundary`; reads `OPEN_BITCOIN_PHASE138_REPO_ROOT`. |
| `scripts/check-phase138-parity-uat-release-boundary/matrix.ts` | Named 4×6 table plus D-04 composition | ✓ VERIFIED | 24 cells plus `COMPOSITION_CELL`; composition symbol lives in `constants.ts` and is imported. gsd-tools `contains` miss is a constant-indirection false negative. |
| `scripts/verify.sh` | 117 then 138 then reconciliation | ✓ VERIFIED | Visible and executable last `check-phase*` is the 138 check. |
| `.planning/REQUIREMENTS.md` | Flipped leftover checkboxes | ✓ VERIFIED | `- [x] **MPVFY-04**` and the other leftover IDs; no leftover `- [ ]` rows for those families. |
| `scripts/check-phase135-snapshot-recovery.ts` | Parity lock requires done/Complete | ✓ VERIFIED | `P135 parity: evidence is done and MPDUR requirements are complete`; checks `- [x] **MPDUR-0X**` individually. gsd-tools phrase-miss is a false negative. |

### Key Link Verification

| From | To | Via | Status | Details |
| ---- | --- | --- | ------ | ------- |
| `recovery_cases.rs` | `restart_composition.rs` | `mod restart_composition` | ✓ WIRED | Line 147. |
| `check-phase131-rolling-fee-expiry-pressure.ts` | `mempool.rs` | work-count PRESS-05 pin | ✓ WIRED | `SUSTAINED_PRESSURE_TRIM_CYCLES`. |
| `docs/parity/index.json` | `docs/parity/checklist.md` | matching v2-2 surface IDs | ✓ WIRED | All four closeout-era IDs present in both. |
| `138-UAT.md` | `docs/operator/runtime-guide.md` | Cargo/Bazel command forms | ✓ WIRED | Same `open-bitcoin` / `open-bitcoin-cli` forms. |
| `checks.ts` | `docs/parity/index.json` | exactly-once ownership | ✓ WIRED | `v2-2-parity-uat-release-boundary` required and present. |
| `scripts/verify.sh` | `check-phase138-parity-uat-release-boundary.ts` | last check-phase* gate | ✓ WIRED | `bun run scripts/check-phase138-parity-uat-release-boundary.ts` after 117, before reconciliation. |
| `checks.ts` | `.planning/REQUIREMENTS.md` | `[x]` exactly once | ✓ WIRED | `checkRequirementCheckboxes` matches `- [x] **${id}**`; live file has `- [x] **MPVFY-01**` through `04`. gsd-tools escaped-pattern miss is a false negative. |
| `check-phase135-snapshot-recovery.ts` | `docs/parity/index.json` | Phase 135 surface done + MPDUR Complete | ✓ WIRED | Top-level name `"v2 snapshot schema, checkpointing, and recovery"` status `done`. |

### Data-Flow Trace (Level 4)

This phase is a filesystem-only closeout gate, not a UI that renders fetched data. The last-gate checker reads live files and returns real failure strings.

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `checkPhase138ParityUatReleaseBoundary` | `failures: string[]` | Allow-listed repo files via `OPEN_BITCOIN_PHASE138_REPO_ROOT` | Yes — live run printed `Phase 138 parity UAT release boundary validated.` | ✓ FLOWING |
| `restart_composition.rs` | recovered members, unbroadcast set, rolling fee, retry due | `prepare_mempool_recovery_at` / `install_mempool_recovery` / `maintenance_tick` | Yes — asserts recovered count 2, both identities, fee 0, reminted due time | ✓ FLOWING |
| `138-UAT.md` | operator command strings | Published Cargo/Bazel forms | Yes — copy-pasteable commands, not alias-only | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Phase 138 mutation suite | `bun test scripts/check-phase138-parity-uat-release-boundary.test.ts` | 19 pass, 0 fail | ✓ PASS |
| Live last-gate checker | `bun run scripts/check-phase138-parity-uat-release-boundary.ts` | `Phase 138 parity UAT release boundary validated.` | ✓ PASS |
| Instant 2s gate retired | `rg SUSTAINED_PRESSURE_MAX_ELAPSED|Instant::now packages/open-bitcoin-bench/src/cases/mempool.rs` | no matches | ✓ PASS |
| Last check-phase* is 138 after 117 | `rg check-phase138\|check-phase117 scripts/verify.sh` | 117 pair, then 138 pair, then reconciliation in both surfaces | ✓ PASS |
| No leftover Pending IDs | `rg "^- \\[ \\] \\*\\*(PACK-\|PPKG-04\|MPDUR-\|IBR-\|MPOBS-\|MPVFY-)" .planning/REQUIREMENTS.md` | no matches | ✓ PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| MPVFY-01 | 138-01, 138-03 | 4×6 inventory plus D-04 restart composition | ✓ SATISFIED | Matrix cells and `restart_composition.rs` named by the last-gate checker; composition test exists and is breadcrumbed. |
| MPVFY-02 | 138-01, 138-03 | Work-count / `threshold_free` default smoke | ✓ SATISFIED | Instant 2s gate retired; N=24, one retained entry, rolling-fee bump, and `threshold_free` remain. |
| MPVFY-03 | 138-02, 138-04 | Parity owners, breadcrumbs, UAT commands, Knots anchors | ✓ SATISFIED | Exactly-once owners; `138-UAT.md` Cargo/Bazel forms; closeout Knots anchors; leftover rows Complete. |
| MPVFY-04 | 138-03, 138-04 | D-21 required sentence and D-22 overclaim denial | ✓ SATISFIED | Last-gate claim corpus; mutation that `supports package relay` fails 138 and 117; leftover Pending flipped after named evidence. |

No orphaned Phase 138 IDs. REQUIREMENTS.md maps only MPVFY-01 through MPVFY-04 to Phase 138, and every plan frontmatter ID is accounted for. FEEP, PRESS, PACK, PPKG, MPLIFE, MPDUR, IBR, and MPOBS remain exactly-once owned by Phases 130–137.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| `docs/parity/release-readiness.md` | 113 | Stale `remains in_progress` after closeout promotion | ℹ️ Info | Machine-readable index/checklist already say `done`. The sentence under-claims rather than broadening release scope. D-21 is present on the same line. |
| `138-UAT.md` | 24–57 | Required-test `result: pending` left from Plan 02 | ℹ️ Info | Optional public-network review is correctly `status: not run`. Deterministic proof is the last-gate checker, which passed on the live repo. |

No blocker stubs, no unscoped positive `package relay` clause, and no public-network or wall-clock default-verify gates.

Plan 03's documented deviation that pins pressure/failure-injection to `missing_pressure_victim_is_rejected` is live and honest: that symbol exists in `prospective_failure_cases.rs`. Treating `136-VERIFICATION.md` / `137-VERIFICATION.md` as sufficient is correctly rejected; runnable cargo filters are pinned instead.

### Human Verification Required

None. Required UAT is deterministic and filesystem-checkable. Optional public-network review is recorded as not run and is never a default, CI, or release gate.

### Gaps Summary

No goal-blocking gaps. The phase delivers deterministic proof that integrated v2.2 behavior matches its pinned Knots anchors, stays work-count bounded, and does not broaden release claims. Milestone archival remains a later `/gsd-complete-milestone v2.2` command.

---

_Verified: 2026-08-22T10:38:28Z_
_Verifier: Claude (gsd-verifier)_
