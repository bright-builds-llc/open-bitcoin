---
phase: 140-pure-flush-policy-and-typed-decisions
reviewed: 2026-09-02T10:28:00Z
depth: standard
files_reviewed: 6
files_reviewed_list:
  - packages/open-bitcoin-chainstate/src/coins/flush.rs
  - packages/open-bitcoin-chainstate/src/coins/tests/flush.rs
  - packages/open-bitcoin-chainstate/src/coins.rs
  - packages/open-bitcoin-chainstate/src/coins/tests.rs
  - packages/open-bitcoin-chainstate/src/lib.rs
  - docs/parity/source-breadcrumbs.json
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
---

# Phase 140: Code Review Report

**Reviewed:** 2026-09-02T10:28:00Z
**Depth:** standard
**Files Reviewed:** 6
**Status:** clean

## Summary

Reviewed the Phase 140 flush-policy surface: `coins/flush.rs`, its unit matrix, `coins.rs` / `lib.rs` re-exports, test-module wiring, and parity breadcrumbs. Cross-checked leftover node persist (`ManagedChainstate::persist`, `DurableSyncRuntime::persist_progress`) and Knots `FlushStateToDisk` / `GetCoinsCacheSizeState` so the focused invariants could be judged against the pinned baseline.

`decide_flush` follows the locked Knots boolean split (`fCacheLarge` Periodic-only, `fCacheCritical` IfNeeded-only, `fPeriodicWrite`, write-kind from empty-cache). `RefuseDiskSpace` is a distinct variant returned only after an intended write, so a write and a refusal cannot coexist. `classify_cache_size` uses injected occupancy, saturating totals, `u128` 90% math, and strict `>`. `decide_recovery` is count-only. `open-bitcoin-chainstate` has no clock, filesystem, Fjall, or Tokio in the policy path. Leftover snapshot persist is unchanged and does not call the policy.

All reviewed files meet quality standards. No issues found.

---

_Reviewed: 2026-09-02T10:28:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
