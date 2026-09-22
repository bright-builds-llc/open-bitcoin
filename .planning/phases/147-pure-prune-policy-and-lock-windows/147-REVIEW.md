---
phase: 147-pure-prune-policy-and-lock-windows
reviewed: 2026-09-22T18:57:00Z
depth: standard
files_reviewed: 13
files_reviewed_list:
  - packages/open-bitcoin-chainstate/src/lib.rs
  - packages/open-bitcoin-chainstate/src/prune.rs
  - packages/open-bitcoin-chainstate/src/prune/mode.rs
  - packages/open-bitcoin-chainstate/src/prune/range.rs
  - packages/open-bitcoin-chainstate/src/prune/locks.rs
  - packages/open-bitcoin-chainstate/src/prune/plan.rs
  - packages/open-bitcoin-chainstate/src/prune/tests.rs
  - packages/open-bitcoin-chainstate/src/prune/tests/mode.rs
  - packages/open-bitcoin-chainstate/src/prune/tests/range.rs
  - packages/open-bitcoin-chainstate/src/prune/tests/locks.rs
  - packages/open-bitcoin-chainstate/src/prune/tests/automatic.rs
  - packages/open-bitcoin-chainstate/src/prune/tests/manual.rs
  - docs/parity/source-breadcrumbs.json
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
---

# Phase 147: Code Review Report

**Reviewed:** 2026-09-22T18:57:00Z
**Depth:** standard
**Files Reviewed:** 13
**Status:** clean

## Summary

Reviewed the pure prune policy surface in `open-bitcoin-chainstate` (`prune/` module, crate re-exports, tests, and `chainstate-prune` breadcrumbs) against CONTEXT D-01–D-17 and the pinned Knots anchors (`GetPruneRange`, `ParsePruneOption`, `DoPruneLocksForbidPruning`, `FindFilesToPrune`).

Focus checks all pass:

| Focus | Verdict |
| --- | --- |
| Keep window (`tip − 288`) | `last_prunable_height` / `height_inside_keep_window` match Knots (`height > tip − 288`); tip=1000 → last=712, keep=`713..=1000` |
| Lock buffer `[first−10, last+10]` | Knots formula with `height_first ≤ 11 → lock_height=1` and `height_first − 11` otherwise; lock 100 forbids `90..=110` |
| Manual refuse vs clamp | `TargetInsideKeepWindow` refusal; no silent clamp to `tip−288` |
| Automatic empty when `tip ≤ prune_after` | `automatic_prune_allowed` is `tip > prune_after`; planner returns empty plan (not error) |
| Byte budget stop at `remaining ≤ target` | Subtract then break when `remaining_usage <= target_bytes`; covered by `[700, 701]` fixture |
| I/O into chainstate | Injected facts only (`BTreeMap` / `BTreeSet`); no filesystem, Fjall, or network |
| `unwrap` | None in production prune code; crate denies `clippy::unwrap_used` outside tests |
| `maybe_` prefixes | Sole `Option` field is `maybe_present_heights` |
| Tests vs claimed behavior | Mode, range, locks, automatic, and manual suites assert the contracts above, including manual/automatic prune-after asymmetry |

Parity breadcrumbs for all new prune sources point at the Knots validation/blockstorage/args anchors. Module layout uses `prune.rs` + `prune/` per Rust standards.

All reviewed files meet quality standards. No issues found.

---

_Reviewed: 2026-09-22T18:57:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
