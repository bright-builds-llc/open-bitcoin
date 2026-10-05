---
phase: 156-index-owned-manual-and-automatic-prune-coordination
plan: "04"
subsystem: authorization
tags: [basic-filter, prune-locks, ownership, rpc, authentication, fjall]
requires:
  - phase: 156-03
    provides: Trusted durable enable/disable host calls and guarded lifecycle publication
provides:
  - Unconditional reserved-name refusal at authoritative handle and RPC boundaries
  - Exact fresh reserved-entry preservation in public whole-map replacement
  - Real-store, authenticated HTTP, channel race and close/reopen ownership proof
affects: [156-05, 156-06, 156-07, 156-08, 157]
tech-stack:
  added: []
  patterns: [guarded exact map comparison, internally owned identity, authenticated durable fixtures]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/dispatch/prune/tests/ownership.rs
    - packages/open-bitcoin-rpc/src/http/tests/prune_ownership.rs
  modified:
    - packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/prune/records.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/prune_records.rs
    - packages/open-bitcoin-rpc/src/context/prune.rs
    - packages/open-bitcoin-rpc/src/dispatch.rs
    - packages/open-bitcoin-rpc/src/dispatch/prune.rs
    - packages/open-bitcoin-rpc/src/dispatch/prune/tests.rs
    - packages/open-bitcoin-rpc/src/http/tests.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Validate fresh effective ownership then compare raw reserved PruneLockInfo entries exactly under the existing publication guard.
  - Refuse the reserved identity before handle mutation or absent-name no-op, regardless of lifecycle state.
  - RPC returns the fixed invalid-parameter diagnostic while authentication retains its existing precedence over parsing and dispatch.
patterns-established:
  - Ordinary whole-map writes may preserve current ownership but cannot create, remove or alter the reserved entry.
  - RPC and HTTP share one test-only durable fixture using parameter-free trusted lifecycle calls.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 156-2026-10-04T20-28-36
generated_at: 2026-10-04T22:53:42Z
commits: []
git_finalization: pending consolidated root commit after clean whole-phase verification and full native gate
duration: 31min from first recorded TDD command
completed: 2026-10-04
---

# Phase 156 Plan 04: Reserved Operator Ownership Summary

**Reserved BASIC CRUD refusal and exact guarded lock-map preservation, proved through authenticated HTTP, real Fjall reopen and stale cloned-store maps.**

## Performance

- First recorded TDD command: 2026-10-04T22:22:40.062Z.
- Final targeted completion: 2026-10-04T22:53:42.028Z.
- Tasks completed: 3/3.
- Source, test and mapping files changed: 12; summary added separately.
- Both active lesson files read completely: 7,188 bytes and 2,397 conservative estimated tokens. No lesson audit trigger applied.

## Accomplishments

- Authoritative handle set/clear reject `BASIC_INDEX_PRUNE_LOCK` before acquiring mutation authority or returning absent-name `false`. They cannot create, replace or remove the reserved identity, even with absent ownership, Active or Disabled ownership, retained Disabled protection or an exhausted generation. Ordinary set/replace/clear/list and missing-name `false` remain functional.
- Public `sync_prune_locks` retains its shared publication guard across existing duplicate/name-length encoding validation, fresh bounded effective-owner validation, exact raw reserved-entry comparison and SyncAll write. Identical current entries and identical absence are permitted; forged creation, omission, any range change, duplicate reserved entries and stale resurrection refuse before durable mutation. Comparing raw entries preserves protection stronger than the saved minimum exactly.
- RPC set/clear return `InvalidParameter` (`-8`) with `reserved BASIC index prune lock is internally owned`. The name guard precedes ordinary range checks for a reserved name; existing empty-name, reversed-range, buffer-overflow and ordinary-name validation tests continue to pass.
- Added eight node ownership/map tests and five dispatch/HTTP tests. New RPC ownership fixtures use the real durable runtime and Plan 03 trusted parameter-free host lifecycle calls over a bounded genesis fixture. No ordinary reserved CRUD seeds legitimate index ownership.
- HTTP authentication remains before parsing/dispatch: missing or wrong credentials return 401 and the authentication challenge, including malformed JSON. Authenticated v2 reserved calls return HTTP 200 with `-8`; legacy calls retain HTTP 500 with `-8`. Ordinary authenticated set/replace/list/clear and missing-name controls persist through actual close/reopen.

## Task Finalization Records

The strict wrapper defers every task, RED, GREEN and metadata commit. No commit, push or hook bypass occurred.

1. Task 1, exact owner-preserving map replacement and authoritative name refusal: node RED and final 90/90 GREEN complete; pending consolidated root finalization.
1. Task 2, authenticated operator ownership proof: RPC RED and final 38/38 GREEN complete; pending consolidated root finalization.
1. Task 3, source ownership provenance: both new Rust paths mapped and in-source breadcrumbs synchronized; checker, source formatting, file lengths and diff review passed; pending consolidated root finalization.

STATE, ROADMAP, REQUIREMENTS and config remain root-owned. CFPR-01 completion is not activated by this summary.

## Verification

Pinned Bun 1.3.9 was selected and reprobed from `/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64`. All Cargo commands were timing-wrapped and serialized; no Bazel command was run by this plan.

| Check               | Exact command                                                                                                                                                      | Evidence                                                                                                                                                        |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Task 1 RED          | `bun run scripts/command-timings.ts run --key phase156-authoritative-locks-red -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib prune` | Expected exit 101: 82 passed, four new tests failed on reserved handle creation/clear, forged map creation, map omission and stale resurrection.                |
| Task 1 GREEN        | `bun run scripts/command-timings.ts run --key phase156-authoritative-locks -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib prune`     | 90/90 passed, 994 filtered out, 18.77s test body; timing duration 794.326s including the Cursor artifact-lock wait.                                             |
| Task 2 RED          | `bun run scripts/command-timings.ts run --key phase156-authenticated-locks-red -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib prune`  | Expected exit 101: 36 passed, two new reserved tests observed InternalError/-32603 instead of InvalidParameter/-8; authentication and ordinary controls passed. |
| Task 2 GREEN        | `bun run scripts/command-timings.ts run --key phase156-authenticated-locks -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib prune`      | 38/38 passed, 232 filtered out, 10.56s test body; timing duration 626.550s including slow host compilation/launch.                                              |
| Source breadcrumbs  | `bun run scripts/check-parity-breadcrumbs.ts --check`                                                                                                              | Passed for 958 first-party Rust files. The two new RPC paths use exact `git add -N` inventory only.                                                             |
| File lengths        | `bun run scripts/bright-builds-check.ts file-lengths`                                                                                                              | Passed: 1,277 scanned files, zero findings.                                                                                                                     |
| Source formatting   | Scoped `rustfmt --check --edition 2024 --config skip_children=true` on all eleven owned Rust paths                                                                 | Passed; unrelated module children preserved.                                                                                                                    |
| Diff and call graph | Owned diff review, reserved guard/publication caller search and `git diff --check`                                                                                 | Passed; no success/no-op route precedes the reserved name guard, and internal lifecycle/checkpoint effects do not call public map replacement as authorization. |

Final production/test source was unchanged after the respective GREEN runs. The root-owned native verifier, full lifecycle proof and final Git hook remain mandatory before phase finalization. Markdown verification is recorded below.

## Persistence, Race and Reopen Observations

- Store refusal tests compare complete block-index key/value snapshots, including saved owner/state and any immutable/projected filter rows. Forged creation and stale progress maps preserve identical snapshots after actual close/reopen. Ordinary exact-preservation writes keep all unrelated durable index rows intact.
- Two channel-ordered clone tests prepare maps before trusted disable or safe checkpoint progress, then apply only after the fresh mutation completes. Neither uses sleeps. Disable cannot be undone by the old map, and the stale stronger range cannot replace the newer exact reserved entry after progress.
- A valid current reserved entry stronger than the saved minimum is preserved exactly. Weakening it to the minimum or omitting it refuses in both Active and conservative Disabled recovery states. Disabled without a lock accepts ordinary updates and cannot resurrect old protection through public replacement.
- Legacy owner absence with valid saved state preserves its reserved entry and remains unmaterialized by an ordinary write. Malformed current owner bytes refuse an unrelated update without changing the block-index snapshot. Active exhausted generation and Disabled retained/released cases preserve state, lifecycle and locks through handle CRUD and reopen.
- RPC/HTTP tests close all handles before reopening Fjall. They assert exact lock maps, unchanged authoritative metadata/coins snapshots, coins best block and genesis body, then successfully reopen the production runtime against the retained lifecycle. Ordinary HTTP replacement and clear are each checked after an actual close/reopen.

No new commit/reply fault was injected in this plan. Interrupted Disabled and stronger retained-protection cases use labeled cfg(test) raw recovery fixtures only; Plan 03 supplies actual transition fault/reopen evidence and Plan 07 owns the broader final fault matrix. The bounded genesis fixture proves ownership and CRUD, not consensus history or client-after-prune behavior. Private raw index readers/writers were not exported for RPC tests; byte-exact index nonmutation evidence belongs to the node tests.

## Decisions Made

Repo-local AGENTS guidance, the Bright Builds sidecar, placeholder-only overrides and architecture/code-shape/testing/verification/Rust standards informed the implementation. The active GSD phase owns preparation/sync and final tracking.

Exact raw map comparison follows bounded effective-owner validation under the existing mutex. It does not reconstruct a minimum lock and silently replace stronger current protection. All index-owned changes continue through guarded internal publication/lifecycle batches; caller-provided names never grant lifecycle authority.

The parent approved a narrow test-only fixture re-export in `dispatch.rs` and crate-visible test child in `dispatch/prune.rs`, then added those files to Plan 04. The production prune module remains private and no raw index writer is exposed. One shared fixture avoids duplicated durable setup across dispatch and HTTP.

## Threat Mitigations

| Threat   | Concrete mitigation and evidence                                                                                                                                                                                                                                                         |
| -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T-156-12 | Handle and RPC reject the reserved identity independently of existence or lifecycle before mutation/no-op. Direct handle, dispatch and authenticated HTTP cases prove explicit refusal and ordinary CRUD controls.                                                                       |
| T-156-13 | Public whole-map replacement validates fresh bounded ownership and compares exact raw reserved entries while holding the shared guard through write. Forged/omitted/changed/duplicate, stronger-current, malformed-owner and channel-ordered stale maps refuse without durable mutation. |
| T-156-14 | Existing authentication precedes JSON parsing and dispatch. Real durable HTTP fixtures prove 401/challenge for missing/wrong credentials, exact authenticated -8 diagnostics, existing legacy/v2 status mapping and ordinary persistence.                                                |

No new network endpoint, authentication path, filesystem trust boundary or schema was introduced. No unresolved HIGH finding was identified in the owned diff; whole-phase security review remains root-owned.

## Simplification Review

One existing ownership model and one shared publication guard cover the store comparison. Two small boundary helpers make unconditional name refusal explicit. There is no new dependency, crate, mutex, cache, full-index scan per candidate, caller-name authority, paired-delete owner, fabricated accounting or swallowed failure. Complete integrity scans remain at startup/publication; this plan adds only bounded validation and raw equality.

## Deviations from Plan

The parent authorized the two test-only dispatch visibility files for shared durable fixture reuse and updated the checked plan. No production visibility expansion occurred. The wrapper replaces task/TDD commits and shared state updates with pending root finalization records.

## Issues Encountered

- Both RED runs produced only the expected missing enforcement/diagnostic failures. The final GREEN runs passed without a behavioral repair iteration.
- Cursor's existing all-target check held the artifact lock during node GREEN. Its compiler children advanced while the queued command stayed live. Host paging subsequently slowed RPC compilation/launch. Commands were polled within 60 seconds; no primary process, editor check, cache or host setting was changed, and elapsed estimates were not treated as timeouts.

## Known Stubs

None. The scoped placeholder/TODO/FIXME/unimplemented scan found no introduced stub. Public activation, scheduler, actual deletion gates and automatic measurement changes remain declared dependent work.

## Next Plan Readiness and Limits

Plan 05 can wire actual deletion against fresh ownership and migrate its deliberately raw corruption fixtures away from public reserved lock-map mutation. This plan does not execute that migration or claim proof of the dependent manual/automatic payload-deletion contract. Broader parity/README updates and final claim checking remain assigned to the later phase evidence plan.

Full native formatting/Clippy/build/test/doctest/coverage/Bazel/provenance checks, lifecycle validation and final Git finalization remain root gates. No requirement completion or commit hash is claimed here.

## Self-Check: PASSED

All twelve declared source/test/mapping paths and this summary exist. Node 90/90 and RPC 38/38 passed with nonzero matching counts; final source breadcrumbs, file lengths, all eleven scoped Rust formatting checks and diff whitespace checks passed. The summary was checked before targeted formatting with installed explicit GFM/frontmatter extensions, then its final Markdown check passed. No commit existence claim applies: `commits: []` and consolidated root Git finalization remain pending.
