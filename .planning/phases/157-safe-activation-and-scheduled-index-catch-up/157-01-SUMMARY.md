---
phase: 157-safe-activation-and-scheduled-index-catch-up
plan: "01"
subsystem: config
tags: [rust, rpc, basic, blockfilterindex, knots-parity]
requires:
  - phase: 156-index-owned-manual-and-automatic-prune-coordination
    provides: Trusted index lifecycle and reserved prune protection
provides:
  - Typed default-off BASIC setting with explicit option presence
  - Ordered CLI/network/default scalar and named-list resolver
  - Production CLI and bitcoin.conf loader integration
affects: [157-08, 157-10, CFAC-01]
tech-stack:
  added: []
  patterns: [pure ordered option resolver, typed boundary configuration]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/config/blockfilter.rs
    - packages/open-bitcoin-rpc/src/config/loader/blockfilter.rs
    - packages/open-bitcoin-rpc/src/config/tests/blockfilter.rs
  modified:
    - packages/open-bitcoin-rpc/src/config.rs
    - packages/open-bitcoin-rpc/src/config/loader.rs
    - packages/open-bitcoin-rpc/src/config/tests.rs
key-decisions:
  - Preserve false negation identity separately from string zero and resolve scalar mode before named validation
  - Emit fixed bounded refusal categories without echoing operator values or paths
  - Keep index input in CLI and bitcoin.conf; add no JSONC index field
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T15:56:09Z
duration: 18m 23s
completed: 2026-10-05
---

# Phase 157 Plan 01: Exact BASIC Option Selection Summary

**Default-off typed BASIC selection now preserves pinned CLI-last/config-first scalar selection, merged named lists, negation resets and active-section precedence through the production config loader.**

## Performance

- Measured implementation start: 2026-10-05T15:37:46Z.
- Final verification completed: 2026-10-05T15:56:09Z.
- Tasks: 2/2.
- Source files created/modified: 6; summary: 1.

## Accomplishments

- `RuntimeConfig.block_filter_index` exposes `BasicFilterIndexSetting::{Unspecified, Disabled, Basic}`, with `is_enabled()` and `is_explicit()`. Omission is Unspecified/off; explicit zero or negation is Disabled.
- Scalar zero/one/empty bypasses stale invalid entries exactly as pinned. Named mode merges CLI, active network section and default config entries, retaining reset and revived-config behavior instead of flattening repeated values.
- Actual loader paths support single/double dash bare and equals syntax, bitcoin.conf, includeconf order, dotted and active network sections. Separate next-token values refuse. Unknown selected types and excluded `v0`/type 2 produce bounded, redacted diagnostics.
- Successful option forms preserve sync, inbound, relay, block serving and prune policy. A whole-config equality test proves explicit RPC authentication/port, relay and manual-prune settings remain intact.
- All three new Rust modules carry exact pinned `init.cpp`, `common/args.cpp` and `common/settings.cpp` source breadcrumbs. Plan 10 owns consolidated manifest registration.

## Task Commits

Git finalization is pending the root-owned clean whole-phase verification. No task, TDD or metadata commits were created, and no files were staged. Shared STATE, ROADMAP, REQUIREMENTS and config.json were not mutated by this executor.

## Verification and TDD Evidence

All Cargo commands used pinned Bun 1.3.9 and `scripts/command-timings.ts`; shared-target commands ran serially.

| Check | Observed result |
| --- | --- |
| Task 1 RED: exact saved Unspecified resolver and original pure-test snapshot | Exit 101; 8 executed, 0 passed, 8 failed; 270 filtered; compile 11.05s; tests 0.00s |
| Task 1 initial GREEN | Exit 0; 17 passed, 0 failed; 270 filtered; compile 5.95s; tests 0.03s |
| Task 2 RED: temporary loader hands empty source lists to resolver | Exit 101; 9 executed, 0 passed, 9 failed; 278 filtered; compile 2.62s; tests 0.01s |
| Final `phase157-options-core`: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib phase157_blockfilter_` | Exit 0; 18 passed, 0 failed; 270 filtered; compile 3.23s; tests 0.02s |
| Final `phase157-options-loader`: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib config::tests` | Exit 0; 79 passed, 0 failed; 209 filtered; compile 0.28s; tests 0.05s |
| `phase157-options-clippy`: RPC all-targets/all-features Clippy with `-D warnings` | Exit 0; finished in 3.40s after replacing one test-only unnecessary clone with a borrowed slice |
| Scoped Rust 2024 rustfmt check on all six owned source files | Passed |
| Diff review and `git diff --check` | Passed |

The final suite contains eight pure tests and ten loader tests. Every research truth-table row is represented by behavioral assertions, with further integer-prefix negation, include order, explicit-policy preservation and credential/path redaction cases. Existing auth, sync, inbound and prune controls pass in the 79-test regression suite. The root owns the full native verifier and phase lifecycle/source/security review; these targeted results do not claim those later gates passed.

## Decisions Made and Simplification

- A pure resolver owns selection; a thin loader module retains per-source ordering and interprets pinned negation semantics. No production dependency or crate was added.
- Fixed refusal categories avoid reflecting arbitrary input, credentials or file paths. Scalar bypass remains lazy rather than eagerly validating every string.
- Moved the existing supported-config-key predicate unchanged, plus the two index keys, into the new loader helper. This preserves generic precedence and keeps loader.rs at 620 lines, below its existing 628-line boundary.
- The simplification pass replaced an unnecessary named-value counter with an emptiness boolean, removed an unnecessary extra config-file load from policy assertions, and retained the explicit pinned list-merge state transitions for auditability.

## Deviations from Plan

### Auto-fixed Blocking Issue

**1. [Rule 3 - Blocking] Recoverably quarantined a saturated ignored dependency directory before replaying RED.**

- Found during Task 1: the initial command never reached tests. Its rustc dependency compile stopped advancing CPU time; process sampling also blocked during symbol processing.
- The root's debugger confirmed blocked directory enumeration with 65,535 links while the exact loaded proc-macro file remained readable. Under root authorization it stopped the owned build/sampler and preserved the entire ignored dependency directory by atomic rename, without deletion or security-setting changes.
- Independent implementation continued while exact RED snapshots were preserved outside tracked production. After recovery and quiescence, the saved Task 1 RED ran and failed all eight tests; proper implementation was restored before GREEN. Task 2's nine-test RED then failed at the actual loader boundary before restoration and final GREEN.
- The aborted pre-test command is not counted as observed RED. No RED snapshot or temporary loader fault remains in production.
- Source scope stayed within the six declared files; no constructor adaptations were required. Git finalization remains root-owned.

## Known Limits and Parity Handoff

- Double negatives select the same state as pinned Knots, including integer-prefix interpretation. This loader does not emit Knots' potentially-confusing-double-negative warning: the RPC config crate has no diagnostic sink/logging dependency. The root explicitly accepted documenting this benign warning difference in Plan 10; no raw operator values are logged.
- The tested parity syntax is single/double dash, bare or equals values. Existing generic broad dash normalization remains unchanged; other dash counts are outside this parity guarantee. JSONC has no index field.
- This plan resolves configuration only. Durable selection, pre-prune application and scheduled index execution remain the later plans' production consumers; CFAC-01 phase closure still requires that integration and root verification.
- No daemon/public network was started, and no networking, serving, history acquisition or repair behavior was added.

## Security Review

- T-157-01: source/reset identity is retained until typed resolution; actual loader table tests pass.
- T-157-02: refusal messages are fixed and bounded, with explicit tests excluding long supplied credentials, Unicode payloads, newlines and config paths.
- T-157-03: Unspecified is off; successful loader forms preserve network activation policy and explicit unrelated controls.
- Stub scan found no production placeholders or unwired mock inputs. The omitted/empty collections in tests are intentional truth-table fixtures. No unmodeled security surface was introduced.

## Next Plan Readiness

The typed setting is ready for Plan 08's daemon/store startup path. Plan 10 must register the three new source paths, document option/diagnostic semantics and retain this warning limitation. Consolidated commits, phase state updates and push remain pending root finalization.

## Self-Check: PASSED

All six source paths and this summary exist, both targeted final test commands passed with nonzero counts, scoped Clippy/formatting passed, and the final source diff was reviewed. No commits exist for this plan by design; no commit hashes are asserted.
