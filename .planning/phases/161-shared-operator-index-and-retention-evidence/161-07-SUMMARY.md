---
phase: 161-shared-operator-index-and-retention-evidence
plan: "07"
subsystem: ui
tags: [basic-filters, dashboard, ratatui, terminal-evidence]
requires:
  - phase: 161-06
    provides: Shared nineteen-row BASIC formatter and exact safe unavailable reasons
provides:
  - Complete shared BASIC dashboard model and text output
  - Bounded interactive BASIC presentation in the existing right column
  - Actual TestBackend evidence at normal, narrow and blocked geometries
affects: [161-08, 161-09]
tech-stack:
  added: []
  patterns: [Shared full rows, interactive-only abbreviations, actual terminal-cell assertions]
key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/dashboard/model/basic_index_section.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests/basic_index.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/app/tests/basic_index.rs
  modified:
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests/projection.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests/prune.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/app.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/app/tests.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Consume basic_index_entries directly; full model and text never abbreviate or infer facts.
  - Keep compaction private to interactive rendering, including all five supplied peer counts.
  - Use bounded existing-status guidance when complete critical values cannot fit.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 161-2026-10-11T04-48-29
generated_at: "2026-10-11T07:39:51Z"
duration: 11min including context and evidence
completed: 2026-10-11
---

# Phase 161 Plan 07: BASIC Dashboard Visibility Summary

**The existing dashboard now consumes the nineteen shared BASIC rows, with actual terminal cells proving readable latch, lag, processed, durable, failure and retained evidence.**

## Accomplishments and API

Both tasks are complete across ten source/manifest paths, including three new registered Rust children. The model child calls `crate::operator::status::basic_index_entries(&BasicIndexOperatorStatus)` directly. The section order is Node, Sync and Peers, Mempool and Wallet, Service, BASIC index, Logs and Health, Prune. Existing dashboard text retains all nineteen complete rows, exact shared values and full unavailable reasons; JSON continues to serialize the captured typed snapshot through its existing path.

The private interactive renderer abbreviates supplied strings only. It keeps the full short failure categories, decimal heights, observed absence, earned latch and separate lag. It retains the complete word `Unavailable`, never substitutes false, zero or no failure, and checks critical row lengths before rendering. Basis and both notes use the approved compact wording. Peer scope shows G/E/A/I in order plus O for the supplied offered count. Full text preserves the modeled-origin disclosure: implicit origins are not modeled/currently zero, ordinary peers are not relabeled, and all capability can advertise without a backend. Fixed Next action abbreviations follow the supplied shared guidance, including backend failure without restricting that category to writes.

No state collector, renderer-owned status schema, data source, RPC, storage read, retry, recovery, activation, mutation or dependency was added. Initial latch and current lag stay separate; processed never upgrades safe durable progress. Existing refresh code (`tick_ms.max(250)`), four vertical bands (3/minimum 10/8/4), 35/35/30 columns, eight charts, action keys/order, service confirmations and blocked-window destructive-action guard remain intact.

## Actual Render Evidence

All cell tests construct `Terminal<TestBackend>` and call the real `draw_dashboard` with a state projected from the shared snapshot. Assertions inspect exact adjacent label/value rows and coordinates rather than searching model/source strings.

| Geometry | Actual visible evidence |
| --- | --- |
| 160x55 | All nineteen BASIC facts in order at x=113, y=4 through y=23, within the 46-cell interior; exact bytes/counts, false peer serving, all five peer counts, target, protection, basis, guidance and notes. Logs and Health follows at y=24. |
| 80x30 | Heading plus seven critical rows in order at x=57, y=4 through y=11, within the 22-cell interior. |
| 80x25 | The same eight critical lines fit exactly before the status bottom border at y=12. |
| 80x24 | Exact existing Window too small blocker, current/required heights and resize/quit text; no Actions or BASIC panel. Existing blocked action-policy test also passes. |
| 40x30 | Bounded BASIC / Use status guidance in the 10-cell right interior; no misleading clipped facts. Existing complete dashboard text retains the failure, latch/lag, endpoints, peer-scope disclosure and body/wallet note. |

At both supported narrow heights, earned synchronization coexists with nonzero lag and processed 840021 remains distinct from durable 840018. Every failure category is asserted in full alongside `Retained: observed`. Separate draws preserve not earned, observed `none`, actual genesis `0`, `none seen`, no failure and complete unavailable words. Normal draws test all fourteen supplied guidance actions, hostile direct Rust reasons, unavailable measurements/basis and optional overflow guidance with twenty actual prune-lock rows. A maximum-u64 lag that cannot fit uses truthful status-output guidance instead of truncating the number.

Chart top borders are counted in the actual buffer (exactly eight), column boundaries are checked at 28/56 or 56/112, and Actions is verified at y=51, y=26 and y=21 for the three supported heights. The status bands contain no Actions label. Existing action confirmation, cancellation, service routing, wallet, prune and unavailable behavior tests remain intact.

## Executed Verification

Pinned `/tmp/open-bitcoin-bun-1.3.9-fresh/bun` was probed as 1.3.9 and its directory prepended to PATH for runner/checker children. Every Cargo command ran serially through `bun run scripts/command-timings.ts run --key KEY -- ...`. No yielded command was terminated.

| Timing key / check | Actual result | Duration |
| --- | --- | ---: |
| `phase161-dashboard-model-red` | Four discovered, all four failed on the missing BASIC section/order | 3,480 ms |
| `phase161-dashboard-model` | Four discovered, four passed | 5,727 ms |
| `phase161-dashboard-cells-red` | Eight discovered, five passed and three failed on actual clipping/full-copy behavior | 3,616 ms |
| `phase161-dashboard-cells` | Eight discovered, eight passed | 5,390 ms |
| `phase161-dashboard-regression` | Sixty discovered, fifty-two passed; eight stale section count/Prune-position expectations failed | 366 ms |
| `phase161-dashboard-cells-final` | Twelve discovered, eleven passed; overflow test fixture had not actually overflowed | 3,220 ms |
| `phase161-dashboard-regression-final` | Sixty-four discovered, sixty-four passed after focused compatibility/fixture corrections | 3,485 ms |
| `phase161-dashboard-clippy` | CLI all targets/all features with `-D warnings` passed | 4,432 ms |
| `phase161-dashboard-operator-final` | CLI all-features `operator::`: 396 discovered, 396 passed | 7,668 ms |
| `phase161-dashboard-cells-final-green` | Twelve discovered, twelve passed on final formatted source | 407 ms |
| Scoped Rustfmt, edition 2024, `skip_children=true` | Nine owned Rust paths passed check mode | Passed |
| Parity breadcrumbs | 1,117 Rust paths verified, including all three new children | Passed |
| Managed checks | 1,439 inputs; zero file-length findings; three repository lessons with zero findings | Passed |
| Diff/whitespace and stub review | `git diff --check` passed; no implementation stubs | Passed |

The final operator selection covers **396 distinct tests**, including sixteen new cases from this plan: four model and twelve real-buffer cases. Earlier focused runs overlap that selection and are not additional distinct cases. The relevant final commands were:

```bash
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features operator:: -- --nocapture
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli phase161_basic_dashboard_cells -- --nocapture
cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-targets --all-features -- -D warnings
```

These are presentation and existing consumer regression tests using controlled captured fixtures. They do not claim new genuine Fjall-to-consumer authority proof; Plan08 owns that evidence. Root owns the frozen-source full `bash scripts/verify.sh` gate and independent source/security review.

## Threat Mitigations

| Threat | Evidence |
| --- | --- |
| T-161-07-1: terminal disclosure/control injection | Shared formatter exact safe-reason vocabulary; actual hostile direct Rust reason draw/text test; fixed categories and scalar rows only. |
| T-161-07-2: misleading narrow evidence | Actual exact adjacent cell values/order; all failure categories; complete unavailable words; distinct endpoints/latch/lag; safe fallback for insufficient widths/oversized numbers. |
| T-161-07-3: refresh-side work or mutation | Pure captured-row projection and private string compaction; existing capture/cadence unchanged; no RPC/store/runtime access added. |

No new network endpoint, authentication path, filesystem access or trust-boundary schema was introduced. No additional threat flag was identified.

## Deviations and Simplification

Three extra source paths were explicitly authorized by the parent:

1. **Rule 3, maintainable test organization:** `app/tests/basic_index.rs` isolates real-buffer tests because existing `app/tests.rs` was already 529 lines. The parent module has only a small declaration; its existing tests are preserved. Its new breadcrumb is registered without changing other manifest mappings.
2. **Rule 1, compatibility expectation:** `model/tests/projection.rs` updates two exact expected section-order/count checks to seven sections. Wallet/durability/chart/action assertions remain intact.
3. **Rule 1, compatibility expectation:** `model/tests/prune.rs` finds Prune by title instead of assuming `sections[5]`. All six prune tests retain their semantics, including height zero, negative-one manual outcome, unavailable evidence, byte targets and lock/refusal behavior.

One added overflow test initially expected a details row without overflowing the status interior. The fixture now supplies twenty real prune-lock rows, exercises the overflow branch, and passes without a production workaround.

The explicit simplification pass retained one direct model adapter, the existing List renderer and two private copy helpers. There is no new compact-mode API, screen, key, alternate application, data collector or generic rendering framework. All abbreviation decisions are presentation-only and unknown future guidance safely directs readers to complete status output.

Material guidance included AGENTS/sidecar, placeholder-only overrides, architecture/code-shape/frontend/testing/verification/operability/Rust/local-guidance standards, finalized context/research, the approved UI contract and scoped Backend/Peer scope reconciliation, and Plan06's current exported formatter API. Both canonical active lessons were loaded in full (7,188 bytes; 2,397 conservative estimated tokens); existing baselines have no new audit trigger. No project skills exist. The parent GSD lifecycle owns synchronization, trackers and finalization.

## Known Stubs and Residual Limits

No stubs prevent this plan's goal. Very narrow widths and oversized critical values intentionally direct readers to the existing complete text output. Lower-priority details may overflow interactively; all shared rows and explanations remain complete in dashboard text, human status and support. Unknown or uncollected evidence remains unavailable.

CFOP-01 remains pending for root closure. D-16 and Phase162 integrated-proof ownership are unchanged. No repair/download, wallet acceleration, public-default, archive-scale, production-service/readiness, funds-safety or stronger crash-window claim is made. README/operator documentation integration belongs to the later phase documentation/checker gate.

## Task Commits

None under the explicit outer-wrapper contract. This executor staged, committed and pushed nothing and made no STATE, ROADMAP, REQUIREMENTS, config, todo or lesson edits.

## Self-Check: PASSED

All ten source/manifest paths and this summary exist. All three new Rust paths are registered. Final nonzero model/buffer/dashboard/operator tests, strict CLI lint, scoped formatting, parity/managed checks and whitespace checks passed. Commit checks are inapplicable because finalization belongs to the parent.
