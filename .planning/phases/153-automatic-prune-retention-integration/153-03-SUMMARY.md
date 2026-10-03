---
phase: 153-automatic-prune-retention-integration
plan: "03"
subsystem: testing
tags:

  - rust
  - fjall
  - pruning
  - daemon
  - wallet
  - recovery

requires: ["phase: 153-02"]
provides:

  - Explicit offline prune selection through the existing recovered durable runtime
  - "Genuine legal-target ordinary retention, checkpoint, downstream and reopen evidence"
  - Recovered current-best-block carry for idle full checkpoints
  - "Small current-lock recovery, worker and idle-checkpoint regressions"

affects:

  - 153-04
  - PRUN-01
  - PRUN-02
  - INT-01

tech-stack:
  added: []
  patterns:

    - ordinary-cycle effect helper
    - real-store metadata fault delegation
    - bounded reusable payload fixture

key-files:
  created:

    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/fixtures.rs
  modified:

    - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/initialize.rs
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:

  - Explicit prune mode selects existing durable recovery independently from sync and inbound activation.
  - "A full checkpoint carries current coins best-block when the cache has no staged tip, preserving a newer overlay tip and propagating read errors."
  - Genuine threshold evidence uses actual serialized nonactive bodies and undo values; protected history may keep the soft target unreachable.
  - Later failure injection changes only metadata persistence while delegating real accounting and paired deletion.

patterns-established:

  - Production and deterministic daemon tests call the same ordinary flush cycle helper.
  - Existing public paired-delete outcome is nameable by dependent FlushPersistSink implementations.

requirements-completed: [PRUN-01, PRUN-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 153-2026-10-03T04-01-54
generated_at: "2026-10-03T06:10:00Z"
duration: 35 min
completed: "2026-10-03"
---

# Phase 153 Plan 03: Offline Ordinary Retention Evidence Summary

**Explicit offline pruning now opens the recovered durable owner, and an ordinary legal 550 MiB cycle performs real paired deletes, checkpoints coins, preserves protected history and refuses unsafe wallet rescans.**

## Performance

- Verification-record window: 2026-10-03T05:39:01Z through 2026-10-03T06:09:01Z; initial context loading preceded this window.
- Tasks: 3/3.
- Files: eleven source/manifest paths plus this summary. Root revised the plan to include the three required existing node paths.
- Final genuine scenario: 17.891221375 seconds; harness 18.08 seconds; timed command 52.781 seconds including compilation and host startup. The first fully passing scenario measured 17.887695208 seconds, harness 18.06 seconds.

## Accomplishments

- `open_runtime_store` selects durability for resolved ManualOnly or Automatic mode with a datadir even when sync and inbound activation are disabled. The existing `DurableSyncRuntime::open_with_runtime_activation` path still completes initialization/recovery before returning the owner. Startup installs configured `SyncNetwork` and prune mode before any workers. Disabled/no-network and no-datadir cases retain transient selection; no networking transport or listener is enabled by pruning.
- The daemon's existing Periodic and Always paths use a thin `flush_cycle` helper that accepts deterministic policy time and disk facts. The helper preserves the underlying authority error for tests; the worker retains its existing categorized error channel. No separate retention worker or lifecycle was added.
- The genuine fixture drives that helper against the production offline durable owner, with a future coins write deadline. Actual retained usage above the legal target selects a nonempty plan, earns paired deletion receipts and forces the existing full checkpoint. Repeated ordinary activity does not advance counters or resurrect deleted data.
- The genuine run exposed a full-checkpoint bug: an opened cache, or a cache cleared by an earlier full flush, had no overlay best-block, so Fjall rejected the write after actual pruning. The shell now reads current cache coins authority and stages it before a real write when present. This preserves a newer overlay tip, carries the durable parent tip for idle writes and propagates read failures. It fabricates no genesis or snapshot fallback.
- A dependent fixture could not implement the existing public `FlushPersistSink` unlink method because its return enum was hidden behind a crate-private re-export. Publicly re-exporting the existing `PairedDeleteOutcome` fixes that public interface without changing unlink behavior or exposing raw storage/codec hooks.

## Genuine Fixture Evidence

| Observation | Actual result |
| --- | --- |
| Configured legal target | Automatic 550 MiB = 576,716,800 bytes |
| Initial retained logical bytes | 578,359,864 |
| Nonactive encoded payload bytes | 578,358,970 from 235 block/undo pairs |
| Reused encoded body / undo size | 2,461,034 / 68 bytes per nonactive pair |
| Initial active paired payload bytes | 894 across six small pairs |
| Initial ordinary committed deletes | Heights 1, 500 and 713; 447 logical bytes |
| Protected active survivors | 510, 714 and 1001; body, undo and cached controls remain |
| Boundary / lock evidence | Tip 1001 makes 713 eligible and 714 recent; lock 520 protects 510 through its ten-block buffer |
| Retained logical bytes after initial deletion | 578,359,417 |
| Earned initial support | One batch, three heights, last height 713; have-pruned true |
| Later replenished actual deletion | Height 1 is rewritten at the same tip, then deleted again before injected metadata failure |
| Final earned support | Two batches, four height deletions; retry does not advance counts |
| Final startup | Production owner reopens after retry with consistent durable chain metadata and removed pairs still absent |

One reusable approximately 2.5 MiB block construction buffer contains 300 bounded 8192-byte output scripts. Each stored body has a distinct canonical header hash, and each mate is a real encoded default undo value. Exact stored accounting equals the accumulated encoded body-plus-undo sums; undo size is derived from an actual measured small pair minus its independently encoded body. No synthetic usage override, lowered target, ignored test or opt-out is used.

The production owner opens before bulk body seeding. Only six small active bodies are cached through the existing duplicate `connect_stored_block` path. Modest undo records keep reopen hydration small. The single large fixture is default-suite and hermetic, and its temporary directory uses RAII cleanup on success and unwinding. Construction memory is bounded; this is not a total Fjall resident-memory measurement.

The 235 bulk pairs are nonactive; the six active bodies are small. Consequently eligible deletion cannot reach the target, which is intentional evidence that protected/nonactive retained bytes still count and never authorize protected deletion. The sparse chain/body fixture is codec-valid retention integration, not consensus validation of a continuous chain or full-sync acceptance. These are logical live-value bytes, not a measured physical allocation cap or evidence of immediate disk-space reclamation.

## Downstream, Failure and Reopen Evidence

- Both durable RPC and node wallet adapters succeed with retained creating payloads before automatic deletion. After height 1 is automatically removed, later full-replacement requests at retained height 1001 refuse the older creating payload, preserve the saved wallet and persist safe Failed evidence. The node refusal retains its prior checkpoint fields. The existing Phase 152 suites also pass.
- Actual removed-body lookup returns absence. An in-memory serving peer receives NotFound for deleted boundary height 713, earning Pruned evidence. An unrelated unknown hash earns Unknown rather than Pruned. An explicitly injected lookup-unavailable completion for a still-present retained body earns Unavailable separately. These checks use no socket. Operator dispatch reports automatic mode, the exact target and earned support counts.
- Successful production checkpoints reopen from the same datadir. A poison leftover snapshot with the original undo records and incorrect tip height does not replace durable coins/chain metadata and does not resurrect removed undo.
- The later failure stage reuses that datadir after production reopen, with one real replenished body/undo pair. A fixture-only `MetadataFaultStore` delegates actual accounting, revisions, durable locks, payload persistence, paired unlink and counters to Fjall. Its sole injected effect is `persist_chain_meta` refusal. Its cache is built from current durable wallet-scan truth; this stage uses a fixture owner/coins view and is distinguished from the earlier production durable checkpoint proof.
- Actual unlink precedes that controlled metadata failure. Receipts remove the positive cached body and actual in-memory undo; both durable mates remain absent. Removing the injected fault permits Always retry and a final production reopen with no resurrection or double counting. This is an injected persistence failure, not an actual hardware/disk fault.
- A small actual-store recovery test injects only the existing interrupted intent record. Reopen finishes the real pair when allowed; a newly current durable buffered lock refuses readiness and preserves both mates and unearned prune state. Existing initialize tests cover hash mismatch, keep-window, absent/one-mate cases and coins recovery.

## Pending Atomic Finalization Records

D-10 defers all commits and push until root completes phase verification and lifecycle validation. This executor created no commits, content staging, hook bypasses or STATE/ROADMAP/REQUIREMENTS edits. Root prepared intent-to-add for new Rust paths; the cached content diff is empty. Requirement completion remains empty.

| Task | Intended record | Status |
| --- | --- | --- |
| 1 | `feat(153-03): activate recovered durable offline pruning` | Pending root finalization |
| 2 | `test(153-03): prove genuine legal-target ordinary retention` | Pending root finalization |
| 3 | `fix(153-03): preserve current coins tip during idle checkpoints` plus deterministic recovery/worker/interface regressions | Pending root finalization |

Base HEAD remains `6dd6f18c9b16158b16391fcd288aadc94728cfcd`. No task commit hashes exist under the explicit deferred-finalization rule.

## Verification and Local RED/GREEN Evidence

All ad-hoc Cargo commands used the timing wrapper sequentially, with resumable polling. Targeted library commands use `--lib`; daemon filters use `--bin open-bitcoind`. Final source has no unresolved test or lint failures.

| Key / check | Result |
| --- | --- |
| `phase153-offline-startup-red` | Meaningful RED: explicit offline mode failed the durable-store selection assertion; default transient control passed. Command 72.161s |
| `phase153-offline-startup` | Final four startup controls pass: explicit modes, disabled default, disabled sync/listener workers and no datadir; harness 1.79s |
| `phase153-genuine-retention` | Meaningful production RED after three real deletes: `batch_write requires a new best-block`. Current-best-block shell fix resolves it |
| `phase153-genuine-retention-final` | Final genuine scenario passes; harness 18.08s, command 52.781s; exact measurements above |
| `phase153-node-regressions` | 25 automatic-prune tests pass; harness 3.86s, command 42.4s |
| `phase153-flush-regressions` | 42 lifecycle/recovery tests pass, including repeated idle/staged-tip writes and current-best read-error refusal; harness 19.82s, command 58.006s |
| `phase153-daemon-flush-regressions` | Four tests pass, including behavioral ordinary Periodic/Always and worker shutdown; harness 0.63s, command 42.018s |
| `phase153-rpc-prune-regressions` | 33 RPC/config/prune regressions pass; harness 11.79s, command 53.793s |
| `phase153-node-wallet-regressions` | 15 node wallet-rescan regressions pass; harness 10.12s, command 10.541s |
| `phase153-rpc-wallet-regressions` | 19 RPC rescan-eligibility regressions pass; harness 14.13s, command 14.467s |
| `phase153-plan03-clippy` | Strict affected node/RPC `--all-targets --all-features -- -D warnings` passes; command 28.459s |
| Formatting | Timed Cargo formatting passed on final source |
| Breadcrumbs | 919 Rust paths pass |
| File lengths | 1230 files scanned; zero findings |
| Diff | Scoped diff review and `git diff --check` pass; other agents' changes preserved |

The genuine test was rerun after the simplification pass. Later focused suites and strict Clippy compile the final source. Full native verification, coverage, build and Bazel remain Plan 04/root gates.

## Decisions and Simplification

Startup changes retain one durable recovery path. The cycle helper stays thin. Ordinary-success and later-error/reopen assertions are separate named stages; fixtures are isolated from behavior assertions. Final main test file is 539 lines, fixture file 362, initialize tests 607 and node automatic tests 535. No new dependency, schema, worker or alternate deletion implementation was introduced.

AGENTS local guidance, the Bright Builds sidecar/overrides, architecture/code-shape/testing/verification/Rust standards and both active lesson files informed the work. Root had already completed remote and pinned submodule bootstrap; this executor did not repeat it. No project skills were present. GSD execution context and D-10 ownership govern finalization.

## Deviations from Plan

1. **[Rule 1 - Bug, root-authorized scope]** Added the current-best carry in `flush_lifecycle.rs` and focused `tests/initialize.rs` guards after real pruning revealed failed idle durable checkpoints. An initialization-only fix would fail again after a cache-clearing full flush, so the repair is at the existing write effect boundary.
2. **[Rule 2 - Missing public contract, root-authorized scope]** Re-exported existing `PairedDeleteOutcome` after compiler E0603 proved the public trait's return type could not be named in a dependent implementation. The real-store fault delegate now compiles. No raw helper/codec export was added.

Root amended the plan for these paths/actions. Commit deferral and state ownership follow D-10 rather than ordinary executor commit steps.

## Issues Encountered

Fixture compilation caught an explicit-path child module requirement, a private undo encoder, test-only raw-coins helpers unavailable in a dependent crate, and shared daemon helper visibility. These were corrected using existing measurements and fixture seams. A CoinsBatch default assumption was corrected to an explicit empty batch.

The initially proposed zero-free-byte injection did not refuse a zero-entry cache because its required byte guard was zero; the final test instead uses the narrowly delegated metadata persistence fault. An unknown hash correctly produces Unknown, so a separate explicit lookup-unavailable completion proves the unrelated Unavailable category. These are fixture corrections, not product failures or fabricated fault claims.

Host pre-harness startup delays were polled normally; no process was terminated and no security metadata was changed. The local timing report contains failed fixture iterations as well as the meaningful RED runs and final passing runs.

## Known Stubs and Threat Scan

No goal-blocking stubs remain. Sparse codec fixtures, controlled fault behavior and synthetic small counting sinks are intentional test support, not production placeholders. No security surface outside the plan's offline datadir/recovery and serving/wallet/cleanup trust boundaries was introduced.

## Next Plan Readiness

Plan 04 can consume the actual byte/timing evidence and perform parity/readme reconciliation, independent review, integration re-audit and the full default verifier. This plan does not claim phase release readiness, public-mainnet unattended sync, consensus-chain validation, physical storage bounds or production-funds wallet readiness. All metadata updates, requirement completion, commits and push remain root-owned.

## Self-Check: PASSED

Both created Rust files and this summary exist. The unchanged base commit exists. Final genuine behavior, related targeted suites, strict affected Clippy, formatting, breadcrumbs, file lengths and diff checks passed. The staged content diff is empty. Atomic commits and state/requirement updates are intentionally absent under D-10; full phase verification and finalization remain root-owned.
