---
phase: 147-pure-prune-policy-and-lock-windows
verified: 2026-09-22T18:58:13Z
status: passed
score: 4/4 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 147-2026-09-22T12-05-43
generated_at: 2026-09-22T18:58:13Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 147: Pure Prune Policy and Lock Windows Verification Report

**Phase Goal:** Operators and later unlink get Knots-aligned prune mode, height-window, and lock-buffer decisions without any disk I/O in core.
**Verified:** 2026-09-22T18:58:13Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Typed shapes for disabled, manual-only, and automatic target of at least 550 MiB; refuse `2..=549`; default disabled | ✓ VERIFIED | `PruneMode::{Disabled, ManualOnly, Automatic { target_mib }}` in `mode.rs`; `parse_prune_arg` maps 0/1/`N≥550`, refuses negatives and `2..=549`; `#[default]` on `Disabled` |
| 2 | Automatic plans keep last 288 blocks and do not start before injected prune-after (empty plan, not error, when tip ≤ prune_after) | ✓ VERIFIED | `plan_automatic_prune` uses `last_prunable_height` / `automatic_prune_allowed`; returns empty `PrunePlan` when tip ≤ prune_after; tests pin tip=1000 never plans ≥713 |
| 3 | Manual prune refuses a target inside the 288-block keep window (no clamp) | ✓ VERIFIED | `plan_manual_prune` returns `Err(TargetInsideKeepWindow)` when `height_inside_keep_window`; tip=1000/target=713 refuses; no silent clamp to tip−288 |
| 4 | A prune lock forbids the locked range plus a 10-block buffer, decided in `open-bitcoin-chainstate` without Fjall or filesystem | ✓ VERIFIED | `PRUNE_LOCK_BUFFER=10` + Knots `DoPruneLocksForbidPruning` formula in `locks.rs`; auto/manual planners omit via `height_forbidden_by_any_lock`; no `std::fs` / Fjall in `prune/` |

**Score:** 4/4 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| -------- | -------- | ------ | ------- |
| `packages/open-bitcoin-chainstate/src/prune/mode.rs` | PruneMode, parse_prune_arg, MIN_DISK_SPACE_FOR_BLOCK_FILES_MIB | ✓ VERIFIED | Exists, substantive, re-exported from crate root |
| `packages/open-bitcoin-chainstate/src/prune/range.rs` | MIN_BLOCKS_TO_KEEP, last_prunable_height | ✓ VERIFIED | `288`; keep-window + prune-after helpers |
| `packages/open-bitcoin-chainstate/src/prune/locks.rs` | PruneLockInfo, PRUNE_LOCK_BUFFER, height_forbidden_by_lock | ✓ VERIFIED | Buffer=10; Knots low-height branch |
| `packages/open-bitcoin-chainstate/src/prune/plan.rs` | plan_automatic_prune, plan_manual_prune | ✓ VERIFIED | Both planners; ManualPruneRefusal enum |
| `packages/open-bitcoin-chainstate/src/prune/tests/automatic.rs` | Auto planner coverage | ✓ VERIFIED | prune-after empty, keep-window, byte budget, lock omit |
| `packages/open-bitcoin-chainstate/src/prune/tests/manual.rs` | Manual planner coverage | ✓ VERIFIED | TargetInsideKeepWindow, lock omit, no byte budget |
| `docs/parity/source-breadcrumbs.json` | chainstate-prune group | ✓ VERIFIED | All prune Rust paths listed with Knots anchors |

### Key Link Verification

| From | To | Via | Status | Details |
| ---- | --- | --- | ------ | ------- |
| `lib.rs` | `prune.rs` | `pub mod prune` + re-exports | ✓ WIRED | Crate root exports mode/range/locks/plan API |
| `mode.rs` | Knots `blockmanager_args.cpp` | `parse_prune_arg` + breadcrumbs | ✓ WIRED | Parity header + integer contract |
| `locks.rs` | Knots `blockstorage.cpp` | `height_forbidden_by_lock` | ✓ WIRED | DoPruneLocksForbidPruning analogue |
| `plan.rs` | `range.rs` | last_prunable / automatic_prune_allowed / keep window | ✓ WIRED | Used by both planners |
| `plan.rs` | `locks.rs` | `height_forbidden_by_any_lock` omit | ✓ WIRED | Auto + manual paths |
| `plan.rs` | `AutomaticPruneInput.height_sizes` | injected sizes only | ✓ WIRED | No storage walk |
| `plan_manual_prune` | byte budget | intentionally absent | ✓ WIRED | `ManualPruneInput` has no `height_sizes` / `current_usage_bytes` (gsd-tools path miss is false negative) |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `plan_automatic_prune` | `heights` | Injected `height_sizes`, tip, locks, usage | Yes — computed from caller facts | ✓ FLOWING |
| `plan_manual_prune` | `heights` / `ManualPruneRefusal` | Injected tip, target, locks, optional present set | Yes — range walk or typed refusal | ✓ FLOWING |
| `parse_prune_arg` | `PruneMode` | Operator integer input | Yes — typed enum mapping | ✓ FLOWING |

Pure decision core; no hollow UI props. Downstream unlink/operator surfaces are Phases 148/150.

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Full prune unit suite | `cargo test -p open-bitcoin-chainstate -- prune::` | 39 passed, 0 failed | ✓ PASS |
| No I/O in prune modules | `rg std::fs\|fjall\|Fjall` under `src/prune/` | No matches | ✓ PASS |
| Mode parse contract | Tests map 0/1/550 and refuse 2/549 | Covered in `tests/mode.rs` | ✓ PASS |
| Manual keep-window refusal | tip=1000 target=713 → `TargetInsideKeepWindow` | Covered in `tests/manual.rs` | ✓ PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| PRUN-01 | 147-01 | Disable / manual-only / automatic ≥550 MiB | ✓ SATISFIED | `PruneMode` + `parse_prune_arg`; default Disabled |
| PRUN-02 | 147-02 | Automatic keep 288; no start before prune-after | ✓ SATISFIED | `plan_automatic_prune` + range helpers; empty plan when tip ≤ prune_after |
| PRUN-03 | 147-03 | Manual refuse target inside keep window | ✓ SATISFIED | `ManualPruneRefusal::TargetInsideKeepWindow`; no clamp |
| LOCK-01 | 147-01, 147-02, 147-03 | Lock range + 10-block buffer from deletion | ✓ SATISFIED | `height_forbidden_by_lock` + omit in both planners |

All Phase 147 requirement IDs from PLAN frontmatter and REQUIREMENTS.md are accounted for. No orphaned Phase 147 requirements. REQUIREMENTS.md rows left Pending (phase-complete owns Complete flips).

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| `prune/tests/manual.rs` | several | `.expect(...)` in tests | ℹ️ Info | Test-only; production prune code has no `unwrap`/`expect` |
| — | — | TODO/FIXME/placeholder in production `prune/` | — | None found |
| — | — | `std::fs` / Fjall / `have_pruned` / `FlushMode` in `prune/` | — | None found |

### Human Verification Required

None. Phase deliverable is an I/O-free library decision surface; success criteria are covered by unit tests and static wiring checks. Operator UI/RPC is deferred to Phase 150.

### Gaps Summary

No gaps. Phase goal achieved: typed prune mode, automatic/manual height planners, and lock-buffer predicates exist in `open-bitcoin-chainstate` without disk I/O.

---

_Verified: 2026-09-22T18:58:13Z_
_Verifier: Claude (gsd-verifier)_
