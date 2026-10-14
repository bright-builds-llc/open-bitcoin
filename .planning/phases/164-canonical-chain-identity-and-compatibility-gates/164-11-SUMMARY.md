---
phase: 164-canonical-chain-identity-and-compatibility-gates
plan: "11"
subsystem: testing
tags: [rust, dispatch, benchmarks, explicit-fixtures]
requires:
  - phase: 164-10
    provides: Context fixture migration
provides: [Explicit dispatch and benchmark fixture contexts]
affects: [164-13]
tech-stack:
  added: []
  patterns: [Named synthetic context construction]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-rpc/src/dispatch/filter_index.rs
    - packages/open-bitcoin-rpc/src/dispatch/filter_index/tests.rs
    - packages/open-bitcoin-rpc/src/dispatch/prune/tests.rs
    - packages/open-bitcoin-rpc/src/dispatch/prune/tests/ownership.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests/chain_fixtures.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests/network_fixtures.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests/permissions.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests/storage_fixtures.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests/sync_status.rs
    - packages/open-bitcoin-bench/src/cases/rpc_cli.rs
key-decisions: [Keep product dispatch logic unchanged]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 164-2026-10-14T01-59-33Z
generated_at: "2026-10-14T04:38:36Z"
duration: 1min
completed: 2026-10-14
---

# Phase 164 Plan 11: Dispatch and Benchmark Fixture Calls Summary

**Thirteen synthetic dispatch/benchmark calls across ten paths now select explicit fixture factories, including two local operator aliases.**

## Accomplishments and Verification

Both tasks completed. The mixed `dispatch/filter_index.rs` change is confined to its existing `#[cfg(test)] initial_tests` module. Production dispatch remains unchanged. Factory arguments, assertions, corpora, receipts, default flags and evidence labels are preserved.

Scoped Rust formatting, whitespace and managed file-length checks passed (zero findings). A mechanical-equivalence comparison against HEAD passes for every owned file after normalizing only factory names, whitespace and formatter commas. The initial whitespace-only comparison flagged rustfmt's added trailing comma; the exact diff confirmed formatting alone. No implementation failure occurred. A simplification pass retained the direct API substitution.

Local AGENTS guidance, sidecar, overrides, architecture/code-shape/verification/testing/Rust standards and complete active lessons informed execution.

## Task Commits

Tasks 1 and 2: deferred parent-owned strict phase finalization. No staging, commit, push, hooks, worktree or shared tracker changes.

## Deviations from Plan

**[Rule 3 - Coupled fixture handle ownership] Reused the already open sync-status fixture store.** Plan 13's dispatch run exposed two tests sharing `context_with_runtime_metadata`: the old wallet fallback silently ignored a second-open lock error; the explicit fixture helper propagates it. Parent authorized replacing that helper's infallible fixture call with `from_fixture_runtime_config_with_store` and `Some(store.clone())`, retaining identical configuration, metadata, daemon control and assertions. This is a disclosed caller-ownership fix beyond factory renaming. The earlier mechanical-equivalence result predates this fix; all other owned changes remain mechanical. Focused and complete dispatch verification results are recorded in Plan 13.

No new security surface, stubs or authentication gates.

## Remaining Coupled Verification

Plan 13 owns compilation and runtime regression after all waves. No intermediate Cargo GREEN or requirement completion is claimed.

## Self-Check: PASSED

All ten source paths and this summary exist. Source checks passed; initial mechanical equivalence passed before the separately documented handle fix. Commit verification is intentionally inapplicable under parent-owned finalization.
