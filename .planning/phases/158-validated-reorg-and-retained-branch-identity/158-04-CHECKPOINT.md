---
phase: 158-validated-reorg-and-retained-branch-identity
plan: "04"
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 158-2026-10-08T02-17-15
generated_at: "2026-10-08T05:44:11Z"
status: verification-blocked
requirements-addressed: [CFIX-03]
requirements-completed: []
---

# Phase 158 Plan 04: Host Launch Checkpoint

**Required-source preflight and the genuine scheduled append consumer are implemented. Final Rust test evidence is blocked before harness entry by the host executable launch boundary. This is not a SUMMARY or plan-completion claim.**

## Current Position

- Execution started from the root's Plan 04 dispatch at approximately 2026-10-08T04:34:33Z.
- Both task implementations are persisted. Neither task is declared fully verified on the latest source.
- No staging, commit, push, worktree, hook bypass, source lint suppression, authority factory, host security change, or cache mutation occurred.
- Root owns STATE, ROADMAP, REQUIREMENTS, todo, final native verification, lifecycle/source/security review and Git finalization. CFIX-03 remains unactivated. No 158-04-SUMMARY.md was written.

## Exact Source Scope

| Path under `packages/open-bitcoin-node/src/` | Change |
| --- | --- |
| `chainstate.rs` | Narrow budget re-export and test-only genuine prepared-work observation |
| `chainstate/filter_reorg.rs` | Preflight before capture/preview; carried, checked composed preparation ledger |
| `chainstate/filter_reorg/preflight.rs` | New reusable retained-source preflight and unchanged common production budget definition |
| `chainstate/fjall_store/reorg.rs` | Narrow lineage wrapper passes its private same-store handle into preflight |
| `network/mempool_lifecycle.rs` | Actual reorg caller uses the thin preparation consumer before preview and mempool work |
| `network/runtime_authority.rs` | Only `filter_index` module visibility becomes `pub(in crate::network)` |
| `network/runtime_authority/filter_index.rs` | Registers restricted reorg consumer |
| `network/runtime_authority/filter_index/reorg.rs` | New thin preparation consumer; no authority constructor |
| `network/runtime_authority/filter_index/catch_up.rs` | All nonempty live/recovered turns use genuine accepted positions; bounded immutable reuse and explicit accepted target/lag; preserves Persistence pause |
| `network/runtime_authority/filter_index/catch_up/tests.rs` | New actual production handle tests, genuine preparation accounting, stale owned work, optional accepted target and later connect |
| `network/runtime_authority/filter_index/catch_up/tests/faults.rs` | New source-loss/corruption/foreign-undo and actual closed-reopen fault controls |
| `storage/fjall_store/filters/turn_inputs.rs` | Native admitted immutable reads, Missing/Deferred/Ready distinction, named physical-row deletion fault helper under cfg(test) |
| `storage/fjall_store/filters/turn_inputs/undo.rs` | New native undo envelope admission before serde, full equality to genuine historical undo |
| `sync/block_reconcile.rs` | Explicit required-source storage refusal and two genuine production reconciliation tests |

`docs/parity/source-breadcrumbs.json` registers all five new Rust modules. The existing `catch_up/inputs.rs` remains unchanged: its borrowed accepted-undo admission already supplies the later-turn source path. Root additionally granted ONLY observation-driven updates to the four exact counters in `sync/tests/filter_index/catch_up.rs`; that existing test file has not been edited.

## Implemented Contracts

The private lineage wrapper is:

```rust
ValidatedChainstateLineage::preflight_required_sources(
    &self,
    staged: &StagedChainstateReorg,
    replacements: &[AnchoredBlock],
    positions: &[ChainPosition],
    undo: &HashMap<BlockHash, BlockUndo>,
    processed: FilterCheckpoint,
) -> Result<TurnWork, StorageError>
```

It delegates to `chainstate/filter_reorg/preflight.rs::required_sources` with the private same-store handle. Manager preparation calls it after read-only sealed storage preparation and before fact capture, preview, mempool effects or accepted publication. The public direct manager path and serialized production network path share this preparation.

- Disconnected consensus inputs always require actual retained bodies and non-genesis undo. An immutable filter does not substitute for a consensus disconnect body/undo.
- The native body scanner admits wire length/work before decoding. Body/header/hash/merkle binding and complete undo shape are checked through `BasicFilterInputs::from_historical`.
- `FjallNodeStore::basic_filter_required_undo(hash, expected, work, maximum) -> Result<BlockUndo, StorageError>` admits the native JSON byte envelope and conservative complete container/count/copy cost before serde. The decoded row must equal the complete genuine old accepted `BlockUndo`, including script/value/height/coinbase metadata. Same-shape data is insufficient.
- Lagging shared heights may reuse an exact verified immutable record and its predecessor header without requiring payloads when consensus does not disconnect that height. Missing immutable rows require the actual historical body/undo pair.
- New replacement bodies use genuine staged undo, including historical and same-block spends; no already-durable replacement undo is demanded.
- No current coins, other-branch reconstruction, snapshot payload, redownload, repair, skipped height or fabricated empty filter replaces required evidence.

`FjallNodeStore::maybe_basic_filter_reusable_record(position, work, maximum, admit)` returns `BasicFilterReusableRead::{Missing, Deferred, Ready}`. Admission runs before clone/decode and can select the existing normal versus oversized-singleton policy. Deferred is distinct from missing, preventing fallback generation or duplicate probe loops. The driver checks the exact currently processed predecessor/header before reusing a returned record.

Every **nonempty** ordinary turn, for both `ValidatedReorg` and `RecoveredPrefix`, calls:

```rust
manager.authorize_basic_filter_append_positions(&proof, &records)?;
store.prepare_basic_filter_replacement_append(proof, positions, &records)?;
```

This does not select permission from `maybe_replacement_target`, which can be None for recovered authority. The current accepted chain supplies only a bounded borrowed slice; preparation returns owned immutable work and completion rechecks the normal same-store generation/revision/branch/frontier barriers. No stage/history copy is retained. Position authorization work is part of the complete achieved turn ledger.

Zero-record maintenance uses `prepare_basic_filter_append`: there is no forward projection to authorize, and checkpoint promotion still requires the existing actual own coins/metadata fence. Preview/pending work cannot acquire the ordinary proof. A full disconnect returns an absent accepted target and paused conservative progress without preparing a nonempty turn.

`BasicFilterTurnOutcome` adds `maybe_accepted_target`, `maybe_accepted_lag` and `reused_records`. A failed accepted publication cannot turn the old pure progress into new achievement: the driver observes the explicit accepted endpoint, refuses pending publication and preserves a Persistence pause. Genuine later connects update the existing accepted projection. No broad RPC/status/peer surface or additional worker was introduced.

`PreparedChainstateReorg::maybe_basic_filter_preparation_work() -> Option<TurnWork>` is a cfg(test) readout of a genuine preparation. Actual production composition uses `PreparedBasicFilterReorg::work()` plus admitted preflight work plus bounded next-height captured facts. Preview/commit validate this ledger against the checked **sum** of the independent existing storage, source/scheduler and accepted-fact ceilings. No newly guessed numeric cap was introduced. Core consensus staging/preview history costs and ordinary mempool/persistence costs remain separate; this is not a whole-runtime constant-memory or calibrated latency claim. Plan 06 must observe/calibrate the integrated work.

## Earned Evidence and Limits

| Timing key / check | Actual result |
| --- | --- |
| `phase158-production-red` | Compilation RED: two test setup mistakes (network magic/service field and private display helper); corrected |
| `phase158-production-behavior-red` | 0 passed / 2 failed: production reorg concealed corrupted physical undo; ordinary replacement turn lacked accepted-position permission |
| `phase158-production-green` | 2 passed / 0 failed / 0 ignored; 1.13s harness, on the earlier implementation |
| `phase158-production-matrix` | 7 passed / 3 failed / 0 ignored; 5.79s harness. Both sync fixtures and body-fault reopen incorrectly selected PreserveSaved, which installs no ordered append owner; all three were changed to explicit configured Enabled policy |
| `phase158-production-matrix-final` | Compiled 12-case version, then no harness entry for over 30min; stopped ONLY owned child PID 88359 after invariant dyld/non-progress evidence. Exit 101 / SIGTERM 15. No test pass claimed |
| Final timed `phase158-production-format-final` | Passed |
| First `phase158-production-style-strict` | Failed on two introduced issues: unused ordinary append API and production item after sync test module |
| Corrected `phase158-production-style-strict` | Passed `--all-targets --all-features -- -D warnings`, **8.12s**; no `-A`, allow/expect attribute, dummy consumer or authority factory. All seven predecessor strict diagnostic groups were closed genuinely |
| Pinned scoped rustfmt after latest fixture edit | Passed; syntax/format evidence only |
| Bright Builds `all` | Zero findings; source modules remain below 628 lines |
| Breadcrumb `--check` | 997 currently tracked Rust files passed; five untracked new modules have exact registered comments. Root must check again when final staging includes them |

**Latest edit needing compilation:** after the strict pass and after the current test binary linked, the foreign-undo fixture was strengthened. `genuine_foreign_undo` changes an alternate branch's coinbase output to OP_2, repairs historical/same-block spend txids and header/merkle/PoW commitments, runs actual core `stage_reorg` under the serialized handle, and borrows that genuine **same-height other-branch** undo. It no longer uses an older block from the same branch. This is a test data change, not an authority factory. It needs strict compilation and actual execution after host recovery.

The latest **15 named tests have not produced a final-source passing run**. They cover equal-height production reorg; separate required body/undo loss; genuine same-height foreign undo; corruption; a later one-record scheduled turn after stage consumption; unavailable common immutable-row reuse; post-accept concrete body-writer failure plus actual closed configured reopen/recovered rebinding; composed preparation accounting; stale already-owned append; new accepted identity with publication failure/Persistence pause; full disconnect absence; a later genuine validated connect/historical facts; and actual sync reconciliation success/refusal. New tests are not ignored. Physical deletion helper cases are **Unavailable**, not proof of actual paired pruning. Broader paired-prune/fork/fault/calibration evidence belongs to Plans 05/06.

## Live Host Boundary and Diagnostics

The first stalled matrix child had repeated invariant `_dyld_start` samples, zero CPU and 112KiB physical footprint. Exact binary `codesign --verify --verbose=2` passed. The root accepted stopping only that owned child after more than 30min of concrete non-progress. That result is a termination, not a test failure inside Rust and not an advisory threshold timeout.

- Preserved first runner diagnostics: `.local/open-bitcoin-dev/stall-diagnostics/2026-10-08T05-12-48.983Z-phase158-production-matrix-final/` (`process-tree.txt`, `active-cargo-jobs.txt`, `sample.txt`, `lsof.txt`, `disk.txt`). The runner's sample there is of waiting Cargo.
- Exact first child sample: `/tmp/open_bitcoin_node-c908eaefc6cece02_2026-10-08_000351_tgiP.sample.txt`.
- One permitted current-source retry key: `phase158-production-current-tests`.
- **Live unified execution session: `48034`.**
- **Live Cargo PID: `91463`; live test child PID: `91507`.** The child was still pre-harness at approximately 10min elapsed at this checkpoint.
- Latest child sample: `/tmp/open_bitcoin_node-c908eaefc6cece02_2026-10-08_004108_pSNh.sample.txt`: 790 samples only at `_dyld_start`, 112KiB footprint. Current binary codesign verification again passed. No Rust test frames or CPU progress.
- Current retry linked successfully in 7.41s, but no 15-case harness result exists. Keep this process alive until the human/root recovery disposition. Do not run another identical retry or overlap another Cargo job against this target.
- The runner currently holds the checkout's cooperative build lock. After an actual host restart the old PIDs/sessions will be stale; use the existing timing runner's stale-owner recovery rather than bypassing the lock. No target/cache quarantine or host security mutation is authorized here.

The root owns the human Mac restart request and later resumption. No source or host repair hypothesis is claimed beyond the sampled pre-Rust launch boundary and valid on-disk binary evidence.

## Exact Resume Sequence

After host recovery, first confirm the actual pending process/session disposition and source files still exist. Re-read this checkpoint and the amended Plan 04; preserve all shared edits. Run Cargo serially through the pinned timing runner:

```bash
PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH bun run scripts/command-timings.ts run --key phase158-production-format-resume -- cargo fmt --manifest-path packages/Cargo.toml --all
PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH bun run scripts/command-timings.ts run --key phase158-production-style-resume -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings
PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH bun run scripts/command-timings.ts run --key phase158-production-tests-resume -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib phase158_preflight
PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH bun run scripts/command-timings.ts run --key phase158-production-allphase-resume -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib phase158
PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH bun run scripts/command-timings.ts run --key phase158-production-phase157-regressions -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib phase157
```

Require the 15 new tests to execute, not a zero-test filtered pass. The existing fixed-workload regression in `sync/tests/filter_index/catch_up.rs` asserts record operations 70, projection operations 36, checkpoint operations 5415 and indexed point reads 40. Added genuine position-proof checks and native immutable probes are charged, so these four exact observations may change. Root granted only **actual RED/observation-driven** updates to those four exact assertions together. Keep the exact assertions and preserve eight body reads/decodes/undo borrows/generations, byte/clone bounds, batch count and prefix independence. Do not suppress or weaken bounds. Do not change historical Phase 157 measurement reports; record current counts/deltas in the future 158 summary and Plan 06 measurements. No new actual counter values are claimed here.

Run breadcrumbs and Bright Builds after any edits; inspect the final scoped diff under root's Git policy. Only after passing final-source tests and affected regressions should an executor write 158-04-SUMMARY.md with actual results and self-check. Root retains full native build/coverage/Bazel, lifecycle/source/security review, state updates, requirement activation and strict Git finalization.

## Source Routes for Later Guards

- `production_budget` moved unchanged from `network/runtime_authority/filter_index/catch_up.rs` to `chainstate/filter_reorg/preflight.rs`. The former delegates through `chainstate::basic_filter_turn_budget`, re-exported narrowly in `chainstate.rs`. Plan 07/native source guards must follow this real route rather than expect numeric declarations in the former file.
- Actual network reorg path: `mempool_lifecycle::reorg_to_branch -> runtime_authority/filter_index/reorg::prepare_reorg -> manager.prepare_reorg -> prepare_basic_index_reorg -> lineage.preflight_required_sources -> chainstate/filter_reorg/preflight::required_sources`, before `install_prepared_reorg_preview`.
- Actual sync path remains `sync/block_reconcile -> handle.reorg_to_branch`, with awaiting replacement bodies unchanged and explicit required-source refusal.
- The existing RPC ordinary BASIC maintenance adapter already calls `drive_basic_filter_index_turn`; no new worker or serving path was added.
- Plan 03's proof acquisition moved to `storage/fjall_store/filters/ownership/proofs.rs`; preserve that predecessor source-route amendment in later guards.

## Checkpoint Self-Check

All implementation and test changes are saved on the shared checkout. Exact registered source paths exist; no completion summary or nonexistent commit is claimed. Final tests, the latest test-only fixture compile, affected counter refresh and full native/source/security/lifecycle/Git gates remain outstanding. No known production stub, new dependency/crate/schema/key, external endpoint, authentication gate or unmodeled trust boundary was introduced.
