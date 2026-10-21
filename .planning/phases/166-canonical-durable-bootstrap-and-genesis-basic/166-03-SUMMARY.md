---
phase: 166-canonical-durable-bootstrap-and-genesis-basic
plan: "03"
subsystem: storage
tags: [fjall, lifetime-lease, contention, safe-paths, drop-order]
requires:
  - phase: 166-02
    provides: Earned canonical genesis owners; checked phase execution predecessor
provides:
  - Required opaque external application lease before every supported backend constructor
  - Shared lease retained until store and extracted coins-view engine handles close
  - Private owned-token handoff for later canonical inspection and original open
affects: [166-04, 166-06, 166-07, 166-10, 166-11, 166-12, 166-13]
tech-stack:
  added: []
  patterns: [owned constructor token, final-field lease retention, child-process contention barriers]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/open_lease.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/open_lease/tests.rs
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - packages/open-bitcoin-node/src/storage/coins_view.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Preserve strict source ancestor and symlink refusal; canonical TMPDIR is test preparation rather than an admission exception.
  - Preserve the established redacted lock-contention message and recovery classification.
  - Scope the lease to the documented macOS/Linux operator workflow and explicitly refuse non-Unix platforms.
patterns-established:
  - Backend construction consumes one private lease whose normalized source supplies the builder path.
  - Store lease drops after all engine fields; extracted view handles drop before the retained store.
requirements-completed: [CBOT-02, CWOR-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 166-2026-10-18T22-36-40Z
generated_at: "2026-10-18T23:30:21Z"
commits: false
duration: 14 min recorded implementation and checking window, initial reading excluded
completed: 2026-10-18
---

# Phase 166 Plan 03: Retained Store Operation Lease Summary

**One opaque external lease now gates backend construction and remains held through the final store or coins-view engine teardown, with actual contention and same-datadir reopen evidence.**

## Performance

- Recorded window: 2026-10-18T23:16:05Z through 2026-10-18T23:30:21Z; initial instruction/source reading is excluded.
- Tasks: 2/2. Five owned implementation/evidence files, plus this summary.
- No staging, commits, pushes, branch/worktree creation or hook bypass. Root retains shared state, roadmap, requirements, task progress and final Git saving.

## Accomplishments and Interfaces

Private `open_lease::OperationLease::acquire(&Path) -> Result<Self, StorageError>` checks a canonical existing ancestor and a missing suffix without creating source entries. Empty paths, parent traversal, non-directory components, symlink targets/ancestors and canonical aliases that change the checked spelling refuse. Absolute, relative, dot and initially missing sources share the same normalized identity. `source()` exposes only the checked path within the parent module's privacy boundary.

The empty lease file uses an opaque double-SHA256 source digest under the canonical fixed `/tmp` platform root, independently of caller `TMPDIR` or `HOME`. Unix creation requests 0600; regular-file, empty-content, private-mode, single-link and opened-versus-selected inode checks precede locking. Metadata and lock errors use fixed bounded redacted messages. Selecting an ancestor of that external location refuses before creating the file. Supported code neither deletes nor replaces the stable lease inode, including after source creation and repeated reopen.

`FjallNodeStore::open_unrecovered` acquires the lease and calls private `open_with_lease(OperationLease)`. The latter derives its builder path from the consumed token, preventing a different source from being paired with it. Generic and fixture entrypoints share this gate; ordinary admission still refuses before backend or lease creation. Later canonical inspection can retain and consume this same token without reacquiring its own lock. No canonical inspection, source copying or atomic bootstrap is implemented here.

The store's final `_lease: Arc<OperationLease>` field drops after every database/keyspace field. `FjallCoinsView` now declares its retained `filter_store` last, after its extracted database and coins keyspace. Store clones, the chainstate adapter and coins views therefore keep one lease until the last supported engine owner is gone. Tests arm a test-only observer at the exact final lease destructor and probe the real engine lock; this reproduced the old view drop order before the fix.

## Ownership Audit

- The only production caller cloning `database()` or `coins_keyspace()` is `FjallCoinsView::from_store`; its retained store now drops last. Other callers are test infrastructure in `packages/open-bitcoin-node/src/storage/fjall_store/tests/prune_unlink/protection.rs`, `packages/open-bitcoin-node/src/sync/tests/phase165_live_fixture.rs` and `packages/open-bitcoin-node/src/network/canonical_admission/tests/fixtures.rs`.
- Actual Fjall snapshots occur in `FjallCoinsView::complete_snapshot` and `fjall_store/payload_usage.rs::retained_payload_usage`. Both remain method-local while borrowing a live owner. Returned complete snapshots and usage totals are detached domain data, with no engine capability.
- BASIC query readers borrow a live store and publication guard; returned record views contain owned bytes and domain facts. The chainstate adapter and pending/completed validation objects retain store clones.
- The single production `Database::builder` remains in `fjall_store.rs` behind the required token. Existing direct-Fjall test mutation helpers remain fixture infrastructure, outside supported application admission. Trusted crate-private borrowed raw helpers were retained after parent review; no public raw-handle or bypass-token constructor was introduced.

## Automated Evidence

All Bun commands use the pinned Bun 1.3.9 PATH. Cargo runs serially through `scripts/command-timings.ts`; test commands prepare canonical existing TMPDIR to avoid macOS `/var` symlink fixture paths. Logs remain under ignored `.local/open-bitcoin-dev/phase166/plan03/`.

| Check | Actual result | Log |
| --- | --- | --- |
| Initial attempted stub RED | Exit 0; five passed because the compiler consumed the implementation edited during that invocation; recorded as an observation, not RED evidence | `task1-initial-observation.log` |
| Task 1 actual RED | Exit 101; four passed/two failed: external-root overlap and concurrent process-scenario release | `task1-red.log` |
| Task 1 attempted GREEN | Exit 101; five passed/one failed in concurrent release scenario | `task1-green.log` |
| Serialized diagnostic | Exit 0; six passed | `task1-serialized-diagnostic.log` |
| Task 1 final default-parallel GREEN | Exit 0; six passed after local process-scenario serialization | `task1-green-final.log` |
| Task 2 RED | Exit 101; six passed/one failed; exact final view lease destructor observed the engine lock still held | `task2-red.log` |
| Task 2 GREEN | Exit 0; twelve passed after view drop-order correction and additional lifetime controls | `task2-green.log` |
| Expanded test compilation | Exit 101; no tests executed, incorrect test-only `Testnet4` selector corrected to existing `Testnet3` | `task2-final.log` |
| Expanded retest | Exit 0; fourteen passed | `task2-final-retest.log` |
| Initial storage regression | Exit 101; 486 passed/one failed; new busy text lost existing lock-contention classification | `storage-regressions.log` |
| Exact recovery-category retest | Exit 0; one passed after restoring established fixed busy text | `lock-category-retest.log` |
| Final expanded default-parallel focused suite | Exit 0; fifteen passed | `focused-final.log` |
| Storage regression retest | Exit 0; 488 passed, zero failed, 205.81 seconds | `storage-regressions-retest.log` |
| Focused suite after unchanged fixture-helper relocation | Exit 0; fifteen passed, zero failed | `focused-after-simplification.log` |
| Node Clippy, all targets/features, warnings denied | Exit 0 | `clippy.log` |
| Final targeted rustfmt check | Exit 0 | `format.log` |
| Final breadcrumb check | Exit 0; 1212 Rust files verified | `breadcrumbs-final.log` |
| Bright Builds all | Exit 0; 1574 tracked source files, zero exceptions/findings; repository lesson check passes | `bright-builds.log` |

The focused suite has fourteen behavior scenarios plus the subprocess test entrypoint. It checks child-process generic/fixture/lease contention, deterministic completion barriers, final teardown, repeated actual reopen, failed schema construction, stable private inode reuse, missing suffix creation, relative/absolute identity, strict unsafe paths, ordinary refusal and source bytes/modes/mtimes. No timing-only sleeps are used. Zero-match integration harnesses are excluded from test counts. `git diff --check` passes; runtime logs are confirmed ignored.

## Decisions, Adjustments and Limits

The documented source-built operator scope is macOS/Linux (`docs/operator/runtime-guide.md:132`–135); the source-build support-matrix row delegates to that guide, and `.github/workflows/ci.yml` runs Ubuntu. Parent review chose an explicit fixed non-Unix unsupported-lease diagnostic instead of an accidental drive-prefix or `/tmp` error. Unix-only helpers and test-module references are cfg-gated; no Windows or Linux execution result is claimed from this macOS run. Plan 166-13 must carry this platform scope into current documentation.

The accepted guarantee is cooperative supported Open Bitcoin APIs plus Fjall's real engine lock. Arbitrary direct-Fjall and filesystem writers remain excluded. A lease is exclusion infrastructure, not proof of canonical identity, history or durable authority. Existing generic migration behavior remains explicitly untrusted; ordinary legacy/foreign admission remains refused.

Guidance informing the work: repository AGENTS/native verification/tool pins/parity rules; Bright Builds sidecar and relevant architecture, code-shape, verification, testing and Rust pages; no concrete standards override; both complete canonical active lesson inputs (7188 bytes / 2397 conservative estimated tokens). The checked plan, context, resolved research, plan-check and predecessor summaries define the retained authority boundaries.

## Deviations from Plan

**1. [Rule 1 - Bug] Preserve the existing contention recovery contract.** Found by the broad storage run: a distinct application-busy string mapped to `StorageBackendFailure`. Reusing the established redacted `database locked by another process` message preserves `StorageLockContention` and the existing test contract without broadening classifier/test ownership. The exact regression, focused suite and full storage retest pass.

**2. [Rule 3 - Blocking] Bound fixture environment and file size.** Parent confirmed strict source-path refusal, canonical local TMPDIR preparation and explicit non-Unix refusal within the documented operator scope. A local mutex serializes only process-spawn lifetime scenarios after the parallel-versus-serialized diagnostic; production contention remains immediate and unchanged. The unchanged test-only fixture opener moved into the lease module to keep the main store at 628 physical lines, with no checker exception or production-builder relocation. Future module declarations require a meaningful owned-module split.

The external-directory overlap refusal and view field-order correction directly implement planned safety/lifetime requirements. No production dependency, crate or architecture layer was added. No executor commits were made under the strict wrapper.

## Issues Encountered and Review

The initial intended RED was not a valid failing observation because implementation edits overlapped compilation; its actual passing result is preserved above. Subsequent real RED observations cover the unsafe overlap and exact teardown defect. Process-scenario contention disappeared with serialized execution; fork/exec descriptor inheritance is the working explanation, not a stronger platform guarantee. Final default-parallel runs pass with deterministic local scenario isolation. The initial Cargo invocation also waited on the artifact lock and then completed normally; no editor or compiler was terminated.

The simplification pass retained one private token and existing owner types, moved only the fixture opener for cohesive ownership and file length, and preserved the original production-builder anchor. Owned diffs and all created files were reviewed. No unfinished implementation or unmodeled security surface remains.

## Task Commits

Deferred to root after complete phase acceptance; `commits: false` is intentional. No commit, hook or push result is claimed. Requirement IDs record this plan's contribution; root owns final requirement and lifecycle closure.

## Next Plan Readiness

Ready for 166-04: use the retained private `OperationLease`, its checked `source()` and private `FjallNodeStore::open_with_lease` handoff. Copy/inspection limits, actual source-engine lock, cleanup and authority validation remain later plans. Full default native verification, historical source-contract renewal, LOC freshness, independent final reviews and normal-hook Git saving remain with their declared later owners.

## Self-Check: PASSED

All five owned implementation/evidence files and this summary exist. Final focused tests, the complete storage regression retest, scoped Clippy, targeted formatting, breadcrumbs, Bright Builds and diff whitespace checks pass. Both new Rust files are registered in the breadcrumb manifest.
