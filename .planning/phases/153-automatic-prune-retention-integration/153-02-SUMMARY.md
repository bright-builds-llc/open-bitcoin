---
phase: 153-automatic-prune-retention-integration
plan: "02"
subsystem: database
tags: [rust, fjall, pruning, rpc, concurrency]
requires:
  - phase: 153-01
    provides: Exact guarded payload facts and captured revisions with durable lock forwarding
provides:
  - Measured automatic planning on the ordinary serialized flush owner
  - Full existing coins and chain-metadata checkpoints for nonempty automatic plans
  - Authority-owned durable lock publication and current-lock manual revalidation
  - Conservative completed-revision reuse with 60-second Periodic coalescing
affects: [153-03, 153-04]
tech-stack:
  added: []
  patterns: [authority-state-storage lock ordering, captured-revision idle keys, receipt-owned cache cleanup]
key-files:
  created:
    - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests/fixtures.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests/writers.rs
  modified:
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
    - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/payload_usage.rs
    - packages/open-bitcoin-node/src/sync/types.rs
    - packages/open-bitcoin-rpc/src/context.rs
    - packages/open-bitcoin-rpc/src/context/prune.rs
    - packages/open-bitcoin-rpc/src/dispatch/prune/tests.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Only an equal reusable revision captured with measured facts can authorize an idle scan skip.
  - Changed Periodic input is coalesced for 60 seconds without cached candidate application; first eligible activity and Always measure immediately.
  - The existing authority owns lock read-modify-SyncAll publication and reloads current durable locks for manual application.
  - Nonempty automatic plans request the existing Always policy rather than constructing a new flush decision.
patterns-established:
  - Lock acquisition stays authority then automatic state then storage payload guard; storage guards are released before nested flush effects.
  - Operator status and explicit lock RPCs share the authority-backed lock read.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 153-2026-10-03T04-01-54
generated_at: 2026-10-03T05:34:03Z
duration: 31 min
completed: 2026-10-03
---

# Phase 153 Plan 02: Automatic Retention Owner Integration Summary

**Ordinary flush activity now measures retained payloads, calls the existing automatic planner, and selects full existing checkpoints while durable lock publication shares the prune owner.**

## Performance

- Verification-record window: 2026-10-03T05:03:30Z through 2026-10-03T05:34:03Z; initial context loading preceded this window.
- Tasks: 2/2.
- Files: fourteen assigned source/manifest paths plus this summary.

## Accomplishments

- `ManagedNetworkHandle::flush_coins` invokes measured automatic orchestration inside its existing `mutate` guard. Inputs are actual active positions and tip/hash, configured mode, injected chain threshold, current durable locks and Plan 01's returned snapshot facts. The existing pure planner and cache/undo receipt wrapper remain the only planning/application/delete path.
- Every nonempty automatic plan requests `FlushMode::Always`, forcing the existing coins plus chain-metadata checkpoint even when Periodic has a future write deadline. Empty plans retain the requested policy. None, nonautomatic modes, short chains and overflowing targets gate before exact scans; protected windows, under-target totals and fully protected history retain the pure planner's behavior.
- Clone-shared transient state saves only completed measurement identities, never candidate plans. Equal reusable revisions plus tip height/hash, mode and locks skip unchanged non-Always scans. Changed Periodic activity coalesces to a named 60-second minimum; first eligible activity and Always bypass that interval. A deferred cycle applies no cached candidates. Synchronized pre-effect and post-effect revision reads refuse unstable facts or clear reuse after mutations/errors.
- List, replace and clear lock operations acquire the same authority as planning/deletion. Replacement preserves other names; missing clear performs no write. Production RPC CRUD and operator-status reads route through that owner. Manual plans reload durable locks under the owner and retain caller protection as an additional restriction.
- `SyncNetwork::prune_after_height()` centralizes mainnet 100000 and other-network 1000 thresholds. RPC reuses this mapping, and the handle exposes a narrow startup setter for Plan 03.

## Pending Atomic Finalization Records

D-10 overrides ordinary atomic/TDD commits. This executor made no commits, pushes, source-content staging, STATE/ROADMAP/REQUIREMENTS edits, or hook bypasses. Root owns phase verification, lifecycle validation, requirement completion and finalization. `requirements-completed` remains empty.

| Task | Intended record | Responsibility | Status |
| --- | --- | --- | --- |
| 1 | `fix(153-02): serialize durable prune lock publication` | Sink read-only receiver forwarding, authority CRUD/manual lock reload, RPC/status routes and lock regressions | Pending finalization |
| 2 | `feat(153-02): drive measured automatic retention from ordinary flushes` | Clone-shared gate, planner/checkpoint integration, network threshold, guarded writer helper, small behavior/concurrency tests and breadcrumbs | Pending finalization |

Base HEAD remains `6dd6f18c9b16158b16391fcd288aadc94728cfcd`. Root prepared intent-to-add entries for the new Rust files; `git diff --cached --stat` was empty at handoff.

## Verification and Local RED/GREEN Evidence

All ad-hoc Cargo and rustfmt commands used `bun run scripts/command-timings.ts run --key <key> -- <command>` sequentially. Resumable sessions were polled and unrelated app-owned processes were preserved.

| Key/check | Command or evidence | Result |
| --- | --- | --- |
| `phase153-lock-routes-red` | RPC `metrics_store_does_not_grant` filter against the former metrics-clone write seam | Meaningful RED: 1 failed because the unsupported transient authority could publish a durable lock; command 132.7s, test 4.41s |
| `phase153-automatic-small-red` | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node automatic_prune -- --test-threads=1` with empty-plan scaffold | Meaningful RED: 6 passed, 1 failed on ordinary Periodic `wrote_coins`; command 208.3s |
| `phase153-automatic-small` | Same node filter after integration; final run adds `--nocapture` | Final source GREEN: 23/23 passed, test 2.33s, command 41.3s |
| `phase153-lock-routes` | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc prune -- --test-threads=1` | 32/32 library tests passed, test 9.26s; zero matches in binary/integration targets also completed; command 211.2s |
| `phase153-lock-status-red` | RPC `--lib operator_status_reads_locks_from_the_prune_authority` with distinct authority/metrics stores | Meaningful RED: status returned `Available([])` instead of the authority's wallet lock; command 39.4s, test 1.23s |
| `phase153-lock-routes-lib` | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib prune -- --test-threads=1` after status-read routing | Final RPC source GREEN: 33/33 passed, test 11.02s, command 48.1s |
| `phase153-owner-clippy` | `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node -p open-bitcoin-rpc --all-targets --all-features -- -D warnings` | Passed, command 55.9s |
| `phase153-lock-status-clippy` | Same strict Clippy scope for RPC after the final status-read change | Passed, command 18.7s |
| Formatting | Timed `cargo fmt --manifest-path packages/Cargo.toml --all` and scoped rustfmt | Passed on final source |
| Breadcrumbs | `bun scripts/check-parity-breadcrumbs.ts` | Passed for 917 Rust files |
| File lengths | `bun scripts/bright-builds-check.ts file-lengths` | Passed: 1228 files, zero findings |
| Diff | `git diff --check` and assigned-path source review | Passed; shared Plan 01/root changes preserved |

Node behavior tests cover exact threshold 999/1000 gating, 288-block boundary, ten-block lock buffer, fully protected usage above target, no-op modes, overflow, first/Always measurement, unchanged clone reuse, changed revision/tip/hash/mode/locks, coalescing without cached deletion, accounting failure and cache/undo receipt cleanup after a later metadata failure. RPC regressions include CRUD/reopen, invalid names/ranges/buffer overflow, legal/refused manual behavior, configuration and Phase 152 wallet eligibility.

Small fault/counting sinks inject logical over-target facts only in unit fixtures. Real-store writer tests use one three-byte raw block value to isolate metadata accounting and synchronization; they are not codec or genuine 550 MiB retention proof. Plan 03 still owns the legal-target destructive production-runtime fixture.

## Guard and Lock Audit

The fixed acquisition order is authority, automatic state, then storage payload mutex. Accounting/current-revision methods acquire and release the storage guard internally; the ordinary flush does not retain that guard across effects that reacquire it. Storage writers never acquire the authority.

The new `#[cfg(test)]` helper performs an actual block-index insert under Plan 01's guarded mutation gateway. Channels pause after invalidation and before insertion, including an injected error after insertion. The owner signals before its synchronized revision read; while a clone writer holds the guard, no measurement completes and scan count remains one. Release precedes assertions; after completion, same-tip activity measures again. Two overlapping writers serialize complete attempts, and actual mutex poisoning visibly refuses the next owner cycle. The error case is controlled injection rather than a real disk fault.

Final real-store first-pass observations used five active positions, one payload key and one candidate key: 82.667µs for the ordinary equal-size case and 147.459µs for the error scenario's initial pass. These are whole small owner-pass durations, not a production scan or mutex-hold SLA.

All production `sync_prune_locks` call sites now consist of the existing Fjall forwarding methods and authority CRUD operations; the RPC metrics clone is not a writer. Recovery loads locks and resumes intent before readiness. Remaining direct store writes are fixtures/tests. Unsupported transient sinks expose an empty lock read but refuse lock writes and eligible automatic accounting; adding a metrics clone cannot grant write capability. No production fallback was added.

## Decisions and Simplification

The explicit simplification pass kept orchestration in one 185-line production module, reused `store()` with a read-only `sync_prune_locks` receiver, and moved the status/list read into the existing prune context module. Test support is split into 219-line fixtures and 222-line writer cases; main behavior tests are 431 lines, and the authority entrypoint stays 623 lines. No dependency, schema, worker, persisted byte cache or alternate unlink owner was added.

Local AGENTS instructions, the Bright Builds sidecar/overrides, architecture/code-shape/testing/verification/Rust standards and both active lesson inputs informed the work. Root already performed repository/submodule synchronization. Contributor evidence and README freshness remain Plan 04's assigned closure work.

## Deviations from Plan

- **[Rule 3 - Blocking, authorized scope addition]** Added one narrow `#[cfg(test)]` guarded writer helper in Plan 01's `payload_usage.rs`, because the real storage guard was intentionally private to its module. Production visibility stayed unchanged. Root added the path to the plan before implementation.
- **[Rule 2 - Required conventions, authorized split]** Split counting fixtures and real writer cases into two child test files before the combined test file exceeded the managed file-length limit. Root added their paths; pinned breadcrumbs cover all four new Rust files.
- **[Rule 2 - Missing contract coverage, authorized scope addition]** Moved the read-only operator-status lock helper out of `context.rs` and into the owned prune context so every production lock read uses the authority. A distinct-store RED/GREEN regression proves the route. The private error channel changed to the authority error; existing wire/unavailable handling is preserved.

## Issues Encountered

An initial RED fixture attempted to access a private context field; it was corrected to drop/reopen the public store before the meaningful behavioral RED. Early test compilation reported an unused import and the temporarily unused writer helper; final tests use the helper and strict Clippy has no warnings.

One green command waited before harness startup. The captured process sample at `/tmp/open-bitcoin-phase153-node-sample.txt` showed only `_dyld_start` and a 112 KiB footprint, not Rust/test frames. It subsequently completed without termination or security-metadata changes. App-owned Cargo checks also contended for the same artifact directory and were preserved. Later final runs completed normally.

## Known Stubs and Threat Scan

No goal-blocking stubs remain in the completed Plan 02 surface. No security-relevant surface outside the plan's existing lock/accounting/application boundaries was introduced. New pause/fault capabilities compile only in node tests; RPC wire/authentication shapes stay unchanged.

## Next Plan Readiness

Plan 03 must call `ManagedNetworkHandle::set_prune_network(&self, network: SyncNetwork) -> Result<(), ManagedNetworkAuthorityError>` with `config.sync.runtime.network` before Periodic/Always worker startup. The default is conservative mainnet; the setter resets transient gate history and supplies `SyncNetwork::prune_after_height()`. Production activation/bootstrap placement remains Plan 03-owned.

Small fixtures live in `automatic_prune/tests/fixtures.rs`; real guarded writer cases live in `automatic_prune/tests/writers.rs`. Genuine legal-target deletion, startup/reopen behavior, full default native verification with coverage/Bazel, lifecycle validation, final requirement completion and all commits/push remain Plan 03/04/root work.

## Self-Check: PASSED

All fourteen assigned source/manifest paths and this summary exist. The unchanged base commit exists. Final node/RPC tests, strict affected Clippy, formatting, breadcrumbs, file lengths and diff checks passed. Source changes stay within the root-revised assignment, including the three authorized scope adjustments. The staging diff is empty. Atomic commits/state/requirement updates are intentionally absent under D-10 and root ownership; full phase verification and finalization remain pending.
