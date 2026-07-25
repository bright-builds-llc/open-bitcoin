---
phase: 131-rolling-fee-expiry-and-descendant-eviction-core
verified: 2026-07-25T10:28:23.000Z
status: passed
score: 5/5 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 131-2026-07-24T22-07-47
generated_at: 2026-07-25T10:28:23.000Z
lifecycle_validated: true
overrides_applied: 0
re_verification: false
---

# Phase 131: Rolling Fee, Expiry, and Descendant Eviction Core Verification Report

**Phase Goal:** The mempool remains bounded and internally consistent during sustained pressure while its rolling fee follows pinned Knots bump and decay behavior.
**Verified:** 2026-07-25T10:04:13Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

Roadmap success criteria are the contract. Plan frontmatter truths were merged as supporting detail; none reduced roadmap scope.

| # | Truth | Status | Evidence |
| --- | ------- | ---------- | -------------- |
| 1 | Operators see capacity enforced from accounted memory usage while virtual size remains a distinct fee and reporting measure. | ✓ VERIFIED | `pool/pressure.rs` trims on `accounted_memory() > mempool_capacity`; `legacy_vsize_trim_limit` absent from `packages/**/*.rs`; `MempoolCapacityEnforcement::AccountedMemory` / `"accounted_memory"` in `lifecycle.rs`; `MempoolPressureSummary` still exposes `total_virtual_size` separately from `accounted_memory` and fee roles. |
| 2 | Pressure removes complete descendant packages in pinned descendant-score order and raises the rolling floor from the package actually evicted. | ✓ VERIFIED | `select_eviction_package` min by `descendant_score` then txid; `collect_descendants` + victim removed; `track_package_removed(package_feerate + incremental)` before remove; tests `pressure_removes_victim_and_descendants_with_roles`, `pressure_bump_uses_descendant_package_feerate_plus_incremental` pass. |
| 3 | The rolling floor does not decay before a block is connected and then follows the pinned 12-hour, 6-hour, or 3-hour half-life and rounding behavior for current occupancy. | ✓ VERIFIED | `RollingFeeState::decay_toward` gated on `block_since_last_rolling_fee_bump`; `open_decay_gate_after_block` from `remove_for_connected_block_transition`; `ROLLING_FEE_HALFLIFE_SECONDS=43200` with /2 and /4 occupancy; 10s update gate; zero below incremental/2; `rolling_fee_cases` + node `rolling_fee_decay_requires_connected_block_after_bump` pass. |
| 4 | Expiry and pressure removal leave no stale descendants or derived indexes and preserve graph and fee-aggregate invariants. | ✓ VERIFIED | Expiry and pressure both use `collect_descendants` + `recompute_state`; Expiry emits `MempoolRemovalCause::Expiry` Direct/Descendant; LegacyUnknown skipped; shell `ManagedNetworkHandle::expire_mempool` → `Mempool::expire`; node serving caches cleared in `mempool_lifecycle.rs`; expiry + sustained oracle tests pass. |
| 5 | Deterministic fill, trim, block, decay, expiry, refill, and reorg scenarios remain within documented resource and performance bounds and agree with recomputation. | ✓ VERIFIED | `sustained_pressure_oracle_agrees_across_fill_trim_block_decay_expiry_refill_reorg` asserts `recompute_resource_ledger` after each step; `rolling_fee_restarts_at_zero_without_durability`; bench case `mempool-policy.sustained-pressure-trim` with 2s hermetic threshold; Phase 131 checker wired in `verify.sh`. |

**Score:** 5/5 truths verified

### Supporting Plan Truths (all verified)

| Plan | Truth | Status |
| ---- | ----- | ------ |
| 01 | Effective admission remains `max(static, rolling)`; incremental is bump input only | ✓ VERIFIED — `effective_admission_fee_rate` uses `max`; bump adds incremental outside effective |
| 02 | Rolling collapses to zero below incremental/2; effective never becomes incremental mid-decay | ✓ VERIFIED — `rolling_fee_decay_zeros_below_incremental_half` |
| 03 | Pure core never reads `SystemTime`; LegacyUnknown not invented | ✓ VERIFIED — no `SystemTime` under `open-bitcoin-mempool/src`; expiry skip path |
| 04 | `PolicyConfig` has no `legacy_vsize_trim_limit`; rolling parity `active`; Phase 130 checker historicalized | ✓ VERIFIED — rg 0 in Rust; enums Active/AccountedMemory; checker pattern present |
| 05 | Breadcrumbs for new sources; Phase 131 checker in `verify.sh` | ✓ VERIFIED — breadcrumbs for `rolling.rs`/`pressure.rs`/`expiry.rs`; verify steps at L420–421 and L574–575 |

### Required Artifacts

| Artifact | Expected | Status | Details |
| -------- | ----------- | ------ | ------- |
| `packages/open-bitcoin-mempool/src/fee/rolling.rs` | RollingFeeState bump/decay | ✓ VERIFIED | Substantive; wired from pressure + lifecycle |
| `packages/open-bitcoin-mempool/src/pool/pressure.rs` | Accounted-capacity trim | ✓ VERIFIED | Accounted limiter + package bump |
| `packages/open-bitcoin-mempool/src/pool/tests/pressure_cases.rs` | PRESS-01/02 fixtures | ✓ VERIFIED | `accounted_capacity_trim_*` tests pass |
| `packages/open-bitcoin-mempool/src/pool/tests/rolling_fee_cases.rs` | Half-life/gate fixtures | ✓ VERIFIED | 12h/6h/3h + gate tests pass |
| `packages/open-bitcoin-mempool/src/pool/expiry.rs` | Expire-shaped pure API | ✓ VERIFIED | `expire` + `DEFAULT_MEMPOOL_EXPIRY_HOURS=336` |
| `packages/open-bitcoin-mempool/src/pool/tests/expiry_cases.rs` | Age/descendant/LegacyUnknown | ✓ VERIFIED | All expiry_* tests pass |
| `packages/open-bitcoin-node/src/network/runtime_authority.rs` | `expire_mempool` authority | ✓ VERIFIED | Mutates via managed handle |
| `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` | AccountedMemory + Active labels | ✓ VERIFIED | Enums + pressure_summary |
| `docs/parity/catalog/mempool-policy.md` | Phase 131 ownership prose | ✓ VERIFIED | accounted_memory / active / non-durable |
| `packages/open-bitcoin-mempool/src/pool/tests/sustained_pressure_cases.rs` | Multi-step oracle | ✓ VERIFIED | Oracle + restart-zero tests |
| `packages/open-bitcoin-bench/src/cases/mempool.rs` | Hermetic pressure threshold | ✓ VERIFIED | Case id `mempool-policy.sustained-pressure-trim` (PLAN body ID; frontmatter `contains: mempool-pressure` is a pattern mismatch only — behavior present) |
| `scripts/check-phase131-rolling-fee-expiry-pressure.ts` | Verifier ownership checks | ✓ VERIFIED | Bun checker + tests pass |

### Key Link Verification

| From | To | Via | Status | Details |
| ---- | --- | --- | ------ | ------- |
| `pool/admission.rs` | `pool/pressure.rs` | `trim_to_size` after prospective admit | ✓ WIRED | Import + call at admission commit (gsd-tools regex false-negative on `trim_to_size\\(`) |
| `pool/pressure.rs` | `fee/rolling.rs` | `track_package_removed` | ✓ WIRED | Called before package remove |
| `mempool_lifecycle.rs` (node) | `pool/lifecycle.rs` | `remove_for_connected_block_transition` | ✓ WIRED | Opens decay gate |
| `fee/rolling.rs` | `PolicyTime` | Injected clocks only | ✓ WIRED | No `SystemTime` in mempool src |
| `runtime_authority.rs` | `pool/expiry.rs` | `expire_mempool` → `Mempool::expire` | ✓ WIRED | Authority + network path |
| `pool/expiry.rs` | topology | `collect_descendants` + recompute | ✓ WIRED | Cleanup path present |
| `lifecycle.rs` | RPC/node info | enforcement/parity `as_str` | ✓ WIRED | AccountedMemory / Active |
| Phase 130 checker | historical artifacts | legacy seam off live pool | ✓ WIRED | `checkLegacyEnforcementSeam` / Phase 130 wording |
| `scripts/verify.sh` | phase 131 checker | run_step after Phase 130 | ✓ WIRED | bun test + bun run steps |
| `sustained_pressure_cases.rs` | `recompute_resource_ledger` | oracle after each step | ✓ WIRED | Assert helper (gsd-tools missed relative path) |

### Data-Flow Trace (Level 4)

Not applicable as UI/dashboard rendering. Pressure/expiry/decay mutate real mempool state from entry graphs, accounted ledger, and injected `PolicyTime` / block contexts — no hardcoded empty membership in production paths. Sustained oracle asserts ledger recomputation after each transition.

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `trim_to_size` | `resource_ledger.accounted_memory` | live ledger + recompute | Yes | ✓ FLOWING |
| `RollingFeeState` | `rolling_minimum_fee_rate_f64` | bump/decay transitions | Yes | ✓ FLOWING |
| `expire` | acceptance ages + topology | Known `PolicyTime` metadata | Yes | ✓ FLOWING |
| `pressure_summary` | accounted + fee roles | mempool state | Yes | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Phase 131 bun unit tests | `bun test scripts/check-phase131-rolling-fee-expiry-pressure.test.ts` | 13 pass, 0 fail | ✓ PASS |
| Phase 131 checker | `bun run scripts/check-phase131-rolling-fee-expiry-pressure.ts` | `Phase 131 rolling fee expiry and pressure validated.` | ✓ PASS |
| Mempool PRESS unit tests | `cargo test -p open-bitcoin-mempool --lib -- pressure_ rolling_fee_ expiry_ sustained_pressure_` | 24 passed, 0 failed | ✓ PASS |
| Node expiry/decay wiring | `cargo test -p open-bitcoin-node --lib -- expire_mempool rolling_fee mempool_lifecycle` | 12 passed, 0 failed | ✓ PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| PRESS-01 | 131-01, 131-04 | Accounted-memory capacity enforcement; vsize distinct | ✓ SATISFIED | Accounted trim + evidence labels + tests |
| PRESS-02 | 131-01 | Descendant-score package eviction + rolling bump | ✓ SATISFIED | pressure.rs + bump tests |
| PRESS-03 | 131-02, 131-04 | Block-gated occupancy-sensitive decay + active parity | ✓ SATISFIED | rolling.rs + rolling_fee_cases + Active label |
| PRESS-04 | 131-03 | Expiry/pressure clean descendants + invariants | ✓ SATISFIED | expiry.rs + authority hook + tests |
| PRESS-05 | 131-05 | Sustained oracle + hermetic perf bounds | ✓ SATISFIED | sustained_pressure_cases + bench + checker |

**Orphaned requirements:** None. REQUIREMENTS.md maps PRESS-01..05 only to Phase 131; all five appear in PLAN frontmatter.

Note: REQUIREMENTS.md checklist rows still show `Pending` — tracking metadata, not an implementation gap (orchestrator may flip on phase close).

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| — | — | No TODO/FIXME/placeholder stubs in `rolling.rs`, `pressure.rs`, `expiry.rs` | — | None |
| PLAN 05 frontmatter | — | `contains: "mempool-pressure"` vs case id `mempool-policy.sustained-pressure-trim` | ℹ️ Info | Tool-level pattern miss only; case exists and checker validates threshold |

### Human Verification Required

None. Behaviors are covered by hermetic unit/node tests and the Phase 131 verifier checker.

### Gaps Summary

No actionable gaps. Automated artifact tooling reported two false negatives (escaped regex on `trim_to_size(`, relative path for sustained_pressure key-link) and one pattern mismatch on the bench case id; manual inspection and passing tests confirm wiring and PRESS-05 performance ownership.

Inversion / confirmation-bias notes (non-blocking):
1. Full receive-independent expiry timers remain Phase 136 by design — minimum authority hook is present.
2. Rolling fee non-durability is intentional (D-15 / Phase 135 territory) and tested via restart-zero.
3. Package admission remains Phase 132 — not required for this phase goal.

---

_Verified: 2026-07-25T10:04:13Z_
_Verifier: Claude (gsd-verifier)_
