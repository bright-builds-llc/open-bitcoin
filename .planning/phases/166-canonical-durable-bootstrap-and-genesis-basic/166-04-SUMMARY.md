---
phase: 166-canonical-durable-bootstrap-and-genesis-basic
plan: "04"
subsystem: storage
tags: [fjall, bounded-copy, inspection, source-preservation, checked-cleanup]
requires:
  - phase: 166-03
    provides: Retained private application lease and guarded backend construction
provides:
  - Bounded independent private source copies under the existing engine lock
  - Checked owned-root cleanup and missing-row-preserving detached reader foundation
  - Fixed non-authorizing refusal with no parent raw scratch recovery
affects: [166-15, 166-05, 166-06, 166-11, 166-12, 166-14]
tech-stack:
  added: []
  patterns: [bounded physical inventories, checked exact-root cleanup, child-only raw characterization]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection/copy.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection/reader.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection/tests/copy.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/opening.rs
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Complete only the amended copying and cleanup foundation; mandatory 166-15 must finish the original inspection goal before 166-05.
  - Raw scratch recovery is compiled only for controlled test-child characterization until 166-15 installs its process controller.
  - Preserve the active malformed-refusal behavior test with a valid-recovery positive control; unconditional refusal cannot satisfy full inspection acceptance.
patterns-established:
  - The parent retains source application and engine locks through final source recheck and checked cleanup.
  - Raw-pin abort characterization and safe refusal acceptance have different names and evidence.
requirements-completed: [CBOT-02, CWOR-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 166-2026-10-18T22-36-40Z
generated_at: "2026-10-20T03:20:10Z"
commits: false
duration: 11 min continuation verification window; earlier implementation receipts retained separately
completed: 2026-10-20
---

# Phase 166 Plan 04: Bounded Copying and Non-Authorizing Inspection Foundation Summary

**Existing storage can be copied into a bounded independent private tree under both source locks, then rechecked and removed with explicit failure reporting; the foundation refuses before raw recovery or validation.**

## Completion Boundary

Tasks: **2/2 amended foundation tasks complete**. The former full inspection goal is **unfinished until mandatory 166-15**. This summary does not authorize 166-05, compatible original reopen, original admission, or closure of CBOT-02/CWOR-04. The required frontmatter IDs describe this plan's contribution; root owns requirement closure after combined acceptance.

Eight owned implementation/evidence files and this summary are present. Earlier implementation happened before the process-isolation amendment; the continuation's test-first verification window is 2026-10-20T03:09:02Z through 2026-10-20T03:20:10Z. All historical logs remain intact. No executor staging, commits, push, branches, hook bypass, shared task changes, or STATE/ROADMAP/REQUIREMENTS updates occurred. Root retains final normal-hook Git saving after whole-phase acceptance.

## Accomplishments and Interfaces

`packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs` retains the private `with_inspection<T>(&OperationLease, FnOnce(&InspectedRecords) -> Result<T, StorageError>) -> Result<T, StorageError>` seam. Its production callback now returns the fixed redacted `inspection isolated recovery unavailable` refusal. It never calls the supplied validator or raw reader. It still exercises the checked copy, source recheck and cleanup path, so source or cleanup errors remain visible. No accepted records or original-store capability can leave this foundation.

Private `inspect_path` opens only the existing source `lock` without create/truncate, verifies selected/opened identity and takes the actual engine lock while the caller retains the external application lease. That engine lock remains live through inventory, copying, pure test callbacks, source recheck and checked cleanup. Missing locks, contention, symlinks, special files, hardlinks, unsafe traversal, identity/content/mode/mtime changes, oversized trees and injected copy I/O refuse. Tests use pure callbacks here; those callbacks are not backend recovery evidence.

`packages/open-bitcoin-node/src/storage/fjall_store/inspection/copy.rs` centralizes 4096 entries including root, 512 MiB regular-file payload, depth 16 and 4096 encoded path bytes. Arithmetic is checked; physical payload hashing/copying uses a fixed 64 KiB buffer. Entry identity and content digests detect replacement and same-length changes even with restored mtimes. Copies use independent create-new files, Unix 0600 files and 0700 directories. Source preservation compares bytes/digests, identities, modes and mtimes; atime is deliberately excluded.

`ScratchGuard` owns one random directory under canonical fixed `/tmp`, rejects overlap with the source, records device/inode and removes only that checked root. Result errors still run final source recheck and cleanup. Cleanup errors visibly override success and block any result. Replaced-root tests prove cleanup does not follow a replacement symlink. This is a live-process Result-error guarantee, not a parent/system-crash, forced-termination or OOM guarantee.

`packages/open-bitcoin-node/src/storage/fjall_store/inspection/reader.rs` preserves the restricted `InspectedRecords` namespace/key/value maps, `namespaces()`, `maybe_records()` and `read_existing(&Database)`. Missing namespaces, empty namespaces and missing rows remain distinct; no node schema/default-producing wrappers participate. Its raw `recover(&Path)` is now `cfg(test)` and its only caller is the controlled raw-characterization child. Plan 166-15 must enable raw recovery only behind its compiled child dispatcher, provide the bounded protocol/controller and consume the staged recovered-tree check. Plan 166-06 retains immediate original logical revalidation after lawful admission.

The guarded existing constructors moved intact into `packages/open-bitcoin-node/src/storage/fjall_store/opening.rs`: `open`, `open_unrecovered` and `open_with_lease`. They retain their required lease and original generic fixture/recovery semantics. The current builder anchor is now in this module; 166-11 must review that moved owner and 166-12 must renew only approved current contracts. Historical pins remain intentionally untouched. The main store is 583 lines; the inspection tests are 589 lines. All six new Rust paths have the explicit infrastructure breadcrumb registered in `docs/parity/source-breadcrumbs.json`.

## Historical and Unclosed Evidence

The original implementation's focused/storage passes did not cover the abort boundary. They are retained as their actual observations, not converted into safe-recovery acceptance:

| Observation | Actual result | Full repository-relative log |
| --- | --- | --- |
| Initial copy RED | Exit 101; one passed, one failed | `.local/open-bitcoin-dev/phase166/plan04/task1-red-actual.log` |
| Earlier copy GREEN | Exit 0; nine passed | `.local/open-bitcoin-dev/phase166/plan04/task1-green-final.log` |
| Initial reader RED | Exit 101; nine passed, one failed | `.local/open-bitcoin-dev/phase166/plan04/task2-red.log` |
| Earlier expanded reader run | Exit 0; fourteen passed | `.local/open-bitcoin-dev/phase166/plan04/task2-green.log` |
| Earlier focused run | Exit 0; sixteen passed | `.local/open-bitcoin-dev/phase166/plan04/task2-final.log` |
| Earlier broad storage run | Exit 0; 504 passed | `.local/open-bitcoin-dev/phase166/plan04/storage-regression.log` |
| Original malformed recovery | Test executable SIGABRT 6 | `.local/open-bitcoin-dev/phase166/plan04/malformed-engine-red.log` |
| Controlled original malformed refusal RED | Exit 101; zero passed, one failed; both integer/overflow children SIGABRT 6, source preserved, scratch retained | `.local/open-bitcoin-dev/phase166/plan04/malformed-engine-child-red-final.log` |
| New foundation RED | Exit 101; zero passed, one failed; old code invoked the validator before isolation | `.local/open-bitcoin-dev/phase166/plan04/foundation-red.log` |
| Original behavior remains pending 166-15 | Exit 101; zero passed, one failed on required valid-recovery control | `.local/open-bitcoin-dev/phase166/plan04/isolation-still-pending-red.log` |
| Initial continuation Clippy | Exit 101; changed unit-result assertion required direct `expect_err` | `.local/open-bitcoin-dev/phase166/plan04/foundation-clippy-final.log` |
| Old-target Clippy retest | Interrupted/unfinished; session exited 130 after coordinator-authorized graceful SIGINT, not a code failure or success | `.local/open-bitcoin-dev/phase166/plan04/foundation-clippy-retest.log` |

The active `phase166_inspection_malformed_engine_names_refuse_without_panics_or_retained_scratch` case is neither ignored nor disabled. It now requires a valid-record recovery control before exercising malformed children, preventing the temporary blanket refusal from masquerading as full inspection GREEN. Mandatory 166-15 must earn its genuine final GREEN and independently validate combined 04+15 behavior. The foundation-only focused command intentionally does not claim that original full behavior passed.

The separately named raw-pin characterization launches only the exact controlled test child. Before backend work, the child writes source/scratch/device/inode into its parent-selected create-new 0600 receipt. After confirmed child completion, the live parent verifies the receipt, source identity, canonical fixed scratch parent and exact scratch inode/private mode, then removes only that root. The new characterization reproduced both integer/overflow SIGABRT 6 outcomes with source equality and then successful checked parent cleanup. Its passing assertion means the abort was characterized; it does not mean safe refusal was implemented. No other scratch directory is scanned or deleted.

APFS's previously recorded non-UTF8 fixture creation failure (OS 92) is setup evidence only. Linux-only non-UTF8 setup remains available on a supporting filesystem; no Linux execution or non-UTF8 runtime result is claimed here. Existing forensic and exact-owned cleanup receipts are retained under `.local/open-bitcoin-dev/phase166/plan04/`; this continuation required no additional forensic cleanup.

## Decisions, Deviations and Review

The abort-triggered structural change was already approved in amended 166-04 and mandatory 166-15, with the current checked plan-set digest `fdd5fa6c714fe6ba8e52d62caf2d860b9b79e11293371cdde1ac87dfbc1f0dc1` recorded in `.planning/phases/166-canonical-durable-bootstrap-and-genesis-basic/166-PLAN-CHECK.md`. This execution follows that foundation boundary without inventing another architecture or product worker.

**[Rule 1 - Bug] Adapt the cleanup assertion to its pure callback result.** After replacing the raw reader callback with `Ok(())`, Clippy correctly required `result.expect_err(...)` instead of `result.err().expect(...)`. The localized test assertion was fixed; its final scoped Clippy and cleanup regression evidence are recorded below. No code was committed under the strict wrapper.

**[Rule 3 - Blocking] Isolate a host-stalled build target.** The old-target Clippy retest stopped progressing in Rust compiler search-path directory enumeration. `.local/open-bitcoin-dev/phase166/plan04/foundation-clippy-liveness.txt` records `SearchPath::new → ReadDir → readdir_r`; the process was in U state with little CPU time, matching the active macOS cache-stall lesson. The coordinator authorized graceful interruption of only this timed Clippy job and a fresh isolated target, without a source workaround. The session exited 130 and none of the four verified job PIDs remained; the header-only `.local/open-bitcoin-dev/phase166/plan04/foundation-clippy-interrupted-processes.log` records the post-interruption check. No old cache or editor process was changed. Subsequent required Cargo checks use `/Users/peterryszkiewicz/Repos/open-bitcoin/.local/open-bitcoin-dev/phase166/cargo-target` through the same timing wrapper, with no same-target overlap. The stalled attempt remains unfinished evidence.

The simplification pass kept a single copy/cleanup path and a constant production refusal. Nine physical-copy tests moved intact in behavior to `packages/open-bitcoin-node/src/storage/fjall_store/inspection/tests/copy.rs`; the module is named `copy` to match the plan's exact discovery filter. Other foundation tests use pure copy callbacks, while the known-valid fixture tests the detached existing-record decoder directly without recovering copied storage in the parent. Existing absence semantics and the complete malformed behavior obligation remain explicit.

Guidance materially applied: repository `AGENTS.md` native-verifier/tool-pin/parity rules, `AGENTS.bright-builds.md`, `standards-overrides.md` (no concrete exception), `standards/index.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/verification.md`, `standards/core/testing.md`, `standards/core/local-guidance.md`, `standards/languages/rust.md`, and the formal GSD execute-plan workflow. Both canonical active lesson files were fully read: 7977 bytes, 2660 conservative estimated tokens. No project skills exist. Coordinated shared-main ownership forbids executor Git mutation; root retains synchronization and final saving.

No new production dependency, crate, endpoint, user-facing command or unmodeled trust surface was added. File operations and test-child execution are covered by T-166-04-01/02/03. The guarantee remains cooperative supported application openers plus the actual engine lock; arbitrary direct-Fjall/filesystem writers are excluded. Contributor/operator documentation remains owned by 166-13 because this foundation exposes no new product workflow.

## Known Stubs and Mandatory Follow-Up

`packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs:33` intentionally returns `inspection isolated recovery unavailable`. Mandatory 166-15 must replace this planned refusal with process-isolated bounded recovery, successful normal child teardown/exit, validated complete detached records, parent source recheck and exact-root cleanup before invoking a validator. Its protocol, lifecycle/concurrency/error tests, compiled product/test entrypoints and fresh Cargo/Bazel packaging evidence are still required. The amended foundation goal is complete; the original full inspection goal is not.

No CBOT/CWOR requirement or phase lifecycle is closed here. Execution order remains **04 → 15 → 05**. Full native verification, reviewed source-contract renewal, tracked LOC freshness, independent code/security/goal review and normal-hook Git saving remain with 166-11/12/14/root. No external service setup is needed.

## Task Commits

None. `commits: false` follows the strict wrapper. Root owns all final staging/commits/push after the entire phase passes; no commit or hook success is claimed.

## Automated Evidence

All Bun commands used pinned Bun 1.3.9 PATH. Cargo commands ran serially through `scripts/command-timings.ts`; fixture commands canonicalized process-local TMPDIR with `pwd -P`. Nonzero behavior discovery is recorded separately from helper/no-job discovery. Runtime logs are ignored and preserved.

| Final focused check | Actual result | Full repository-relative log |
| --- | --- | --- |
| `storage::fjall_store::inspection::tests::copy::` | Exit 0; nine passed, zero failed | `.local/open-bitcoin-dev/phase166/plan04/foundation-copy-final.log` |
| `phase166_inspection_foundation` | Exit 0; eight passed, zero failed | `.local/open-bitcoin-dev/phase166/plan04/foundation-green.log` |
| Fresh isolated-target copy filter | Exit 0; nine passed, zero failed, actual clean-target test compilation | `.local/open-bitcoin-dev/phase166/plan04/foundation-copy-isolated.log` |
| Fresh isolated-target foundation filter | Exit 0; eight passed, zero failed; 49.30 seconds | `.local/open-bitcoin-dev/phase166/plan04/foundation-green-isolated.log` |
| Fresh isolated-target node Clippy, all targets/features, warnings denied | Exit 0; 14.37 seconds | `.local/open-bitcoin-dev/phase166/plan04/foundation-clippy-isolated.log` |
| Raw-pin abort characterization only | Exit 0; one passed, zero failed; both actual child aborts characterized and exact owned roots removed | `.local/open-bitcoin-dev/phase166/plan04/raw-pin-characterization-final.log` |
| Final breadcrumb check | Exit 0; 1218 Rust files verified | `.local/open-bitcoin-dev/phase166/plan04/foundation-breadcrumbs-final.log` |
| Final Bright Builds all | Exit 0; zero findings, 1574 tracked source files scanned | `.local/open-bitcoin-dev/phase166/plan04/foundation-managed-final.log` |
| Cargo formatting check | Exit 0 | `.local/open-bitcoin-dev/phase166/plan04/foundation-format-final.log` |
| Final scoped rustfmt after assertion correction | Exit 0 | `.local/open-bitcoin-dev/phase166/plan04/foundation-rustfmt-final.log` |

The canonical genesis+BASIC sizing fixture measures actual complete physical state before and after real all-handle fixture reopen: fresh **60 entries / 67,128,198 bytes**, reopened **46 entries / 21,691 bytes**. Both fit the unchanged finite limits and are physically preserved by pure copying. This is sizing/copy evidence, not canonical admission or safe backend recovery evidence.

## Self-Check: PASSED

All eight owned implementation/evidence files and this summary exist. Static checks confirm the production foundation has no raw recovery call, raw recovery is test-only with one controlled-child caller, every new Rust path is breadcrumb-registered, and standalone YAML delimiters appear only at the top. The required fresh-target copy/foundation filters discover and pass nine/eight cases; fresh-target scoped Clippy, final formatting, breadcrumb/managed checks and diff whitespace checks pass. Owned source changes were reviewed, and runtime artifacts remain ignored. No commits are required or claimed under the strict wrapper. This self-check accepts only the amended foundation; mandatory 166-15 still owns full inspection and the active original behavior test's final GREEN.
