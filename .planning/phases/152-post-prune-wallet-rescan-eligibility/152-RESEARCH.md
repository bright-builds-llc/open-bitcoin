---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 152-2026-10-03T00-26-52
generated_at: 2026-10-03T00:35:00Z
---

# Phase 152: Post-Prune Wallet Rescan Eligibility - Research

**Researched:** 2026-10-02 CDT
**Domain:** Durable wallet replacement eligibility after Fjall paired pruning
**Confidence:** HIGH for implementation seams; MEDIUM for concurrent probe/save ordering

<user-constraints>
## User Constraints (from CONTEXT.md)

The following decisions and discretion/deferred text are copied verbatim. [VERIFIED: 152-CONTEXT.md]

### Locked Decisions

### Full replacement eligibility

- **D-01:** Preserve full replacement semantics. A requested midrange or
  chunk controls progress and stop height, not permission to admit older
  entries without checking their creating payloads. Carry forward Phase
  146 D-04 and D-08 together.
- **D-02:** Check the requested chunk/range payloads and the creating
  payload of every candidate wallet entry through the replacement height.
  Check older creating heights even when outside the range. Deduplicate
  probes where useful. Do not require unrelated older payloads merely
  because unrelated durable coins exist.
- **D-03:** Use one shared eligibility contract for both durable adapters.
  Wallet matching/selection remains pure; Fjall payload probes stay in the
  shell. Durable coins best-block and coins remain chain truth; never
  fall back to the leftover snapshot blob.

### Refusal and durable evidence

- **D-04:** Missing creating metadata, absent payloads, and payload read
  errors fail closed before persisting any replacement wallet. Preserve
  prior persisted wallet balances, UTXOs, and tip/progress on refusal.
- **D-05:** Save a durable Failed rescan-job record for eligibility
  failures, including payload-probe errors. Keep existing error categories
  where possible; record the failed boundary and height/hash when known,
  without raw storage paths or snapshot fallback. Propagate persistence
  errors visibly.
- **D-06:** Re-evaluate eligibility on every resumed chunk and after
  reopen; prior successful probes are not permanent permission after
  prune. A failed later chunk preserves the last successful wallet state.

### Proof and scope

- **D-07:** Runtime regressions must use real paired block/undo deletion,
  retained durable coins, and have-pruned evidence. Cover older creating
  heights outside the range, in-range absence, chunk resume/reopen,
  conflicting leftover snapshots, and retained-payload success controls
  in both adapters. Source-string checks cannot substitute for behavior.
- **D-08:** Reuse existing registry/job and storage interfaces; add no
  dependencies or broad wallet schema redesign. Investigate the audit's
  InterruptedTwoHeads and probe-error notes at touched scan seams;
  close directly related authority/refusal holes without expanding into
  prune retention or repair work.
- **D-09:** Run default `bash scripts/verify.sh`, including managed
  checks, coverage and Bazel smoke. Register new first-party Rust files
  in parity breadcrumbs and refresh relevant parity/operator docs.
  Commits and push wait for clean verification, per this strict wrapper.

### Agent's Discretion

Choose the smallest shared pure/shell seam, typed refusal shape, fixture
organization, and bounded probe strategy consistent with these decisions.
No pending todos matched this phase.

### Deferred Ideas (OUT OF SCOPE)

Automatic target retention is Phase 153. Incremental wallet scanning,
snapshot deletion, archive serving, assumeutxo, public defaults, repair,
and production-funds claims remain outside this closure.
</user-constraints>

## Summary

The audit reproducer follows directly from existing code: `Wallet::rescan_chainstate` selects matching coins from the complete supplied UTXO map and replaces `wallet.utxos`; node and RPC filter coins through their stop height but only probe payloads inside their current chunk/range. A matching coin created before the start therefore remains a replacement candidate after its payload is pruned. This is an eligibility defect, not evidence that durable coins are invalid. [VERIFIED: packages/open-bitcoin-wallet/src/wallet/scan.rs; packages/open-bitcoin-node/src/sync/wallet_rescan.rs; packages/open-bitcoin-rpc/src/context/rescan.rs; .planning/v2.4-MILESTONE-AUDIT.md INT-02]

Use a staged replacement wallet selected by the existing pure scan, then verify the union of requested heights and that staged wallet's creating heights through one shared node-shell helper. Reject absent active-chain metadata or payload/read errors before `save_wallet`; persist Failed evidence on the job. Re-run this helper for every resume. Keep unrelated old coins outside eligibility, because they are not admitted wallet entries. This recommendation implements the locked contract using the existing selection API. [VERIFIED: 152-CONTEXT.md D-01–D-06; packages/open-bitcoin-wallet/src/wallet.rs::utxos; packages/open-bitcoin-node/src/wallet_registry.rs]

**Primary recommendation:** Two dependent plans: shared staged eligibility plus node runtime/authority regressions, then durable RPC integration plus real-prune regressions and evidence/docs. [VERIFIED: .planning/ROADMAP.md Phase 152 planning tasks; implementation seams above]

<phase-requirements>
## Phase Requirements

| ID | Description | Research Support |
| --- | --- | --- |
| SNAP-01 | Wallet rescan reads durable coins and payload-present blocks, and does not treat leftover snapshot bytes as chain truth. | Shared creating-height gate, no-leftover authority guard, real paired-delete midrange/resume/reopen tests. [VERIFIED: .planning/REQUIREMENTS.md; 152-CONTEXT.md] |
</phase-requirements>

## Project Constraints (from AGENTS.md)

- Follow pinned Knots `29.3.knots20260210`, preserve auditable behavioral differences, keep functional-core crates I/O-free, and add no Rust Bitcoin library or new dependency. [VERIFIED: AGENTS.md; .planning/REQUIREMENTS.md]
- This work is already inside the parent's GSD lifecycle. Research owns only this document; code, task ledgers, commits, and push belong to the orchestrator. All commits are deferred until strict-wrapper verification passes. [VERIFIED: delegated task; 152-CONTEXT.md D-09]
- Use Rust `1.94.1`/edition 2024, Bun automation, Bazel/Bzlmod; use `bash scripts/verify.sh` as the default completion/pre-commit contract. Ad-hoc Cargo/Bazel commands must use the timing wrapper; do not overlap Cargo against one target directory. Poll running sessions at least every 60 seconds and capture liveness before termination. [VERIFIED: AGENTS.md Repo-Local Guidance]
- Register touched/new Rust files in `docs/parity/source-breadcrumbs.json`, run the parity checker, refresh relevant README/parity docs, and retain generated `docs/metrics/lines-of-code.md` freshness changes. [VERIFIED: AGENTS.md Repo-Local Guidance]
- Keep guards shallow, use `maybe_` optional names and `foo.rs` plus `foo/`, propagate errors visibly, and avoid `unwrap`. Test behavior with one concern and Arrange/Act/Assert. Preserve parsed Markdown frontmatter: no standalone body `---` separators. [VERIFIED: supplied global AGENTS instructions; standards/core/code-shape.md; standards/core/testing.md; standards/languages/rust.md]
- Managed standards informed this research: architecture, code shape, verification, testing, and Rust. `standards-overrides.md` contains only its placeholder row. Project skill directories `.claude/skills` and `.agents/skills` were absent. Both active lesson files were read completely: 5,230 + 1,958 bytes, 2,397 estimated tokens; no archives were loaded. [VERIFIED: session filesystem reads]

## Standard Stack

### Core

| Component | Pinned version | Purpose | Instruction |
| --- | --- | --- | --- |
| Rust | 1.94.1 / edition 2024 | Existing first-party implementation | Keep pinned toolchain. [VERIFIED: rust-toolchain.toml; packages/Cargo.toml] |
| Fjall | 3.1.4 | Durable coins, wallet/job and paired payload keys | Reuse existing adapters. [VERIFIED: packages/Cargo.lock; packages/open-bitcoin-node/Cargo.toml] |
| First-party wallet/node/RPC | Workspace source | Pure selection and durable shell integration | No new production crate. [VERIFIED: packages/Cargo.toml; 152-CONTEXT.md D-08] |

### Supporting

| Component | Version | Purpose | Instruction |
| --- | --- | --- | --- |
| serde / serde_json | 1.0.228 / 1.0.149 | Existing durable wallet/job shapes | Reuse unchanged shapes. [VERIFIED: packages/Cargo.lock; packages/open-bitcoin-node/src/storage/fjall_store.rs] |
| Bun | Repo pin/execution 1.3.9; global host 1.4.2 | Timing/check automation | Use the isolated pinned binary with the PATH precondition below. [VERIFIED: .bun-version; both version probes] |
| rules_rust | 0.69.0 | Existing Bazel smoke build | New node Rust files enter the existing recursive src glob. [VERIFIED: MODULE.bazel; packages/open-bitcoin-node/BUILD.bazel] |

**Installation:** None. This is pinned code research, not a package selection phase; no registry upgrade or npm package is recommended. Package publication dates were not queried because the phase explicitly adds no dependencies. [VERIFIED: 152-CONTEXT.md D-08; packages/Cargo.lock]

### Alternatives Rejected by the Locked Contract

| Alternative | Why rejected |
| --- | --- |
| Probe every durable coin's old creating block | Refuses wallets for unrelated old coins, contrary to D-02. [VERIFIED: 152-CONTEXT.md] |
| Skip missing candidates / incrementally retain old entries | Changes full replacement and refusal semantics. [VERIFIED: 152-CONTEXT.md D-01/D-04; Deferred Ideas] |
| Share only range filtering, leave probes separate | Does not encode one shared eligibility contract. [VERIFIED: 152-CONTEXT.md D-03] |

## Architecture Patterns

### Shared Staged Replacement

Recommended seam: one small node-shell module, exposed to RPC through the existing node dependency. It may live beside `sync/wallet_rescan.rs` and be re-exported from `sync.rs`; keep the pure wallet crate unchanged unless a small pure helper clearly reduces duplication. This is a proposed implementation shape, grounded in existing exports and pure APIs. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/lib.rs; packages/open-bitcoin-rpc/Cargo.toml]

1. Filter the authority snapshot through replacement stop height once, preserving the current active-chain/UTXO/undo behavior. Remove the duplicated node/RPC filter when the shared helper replaces it. [VERIFIED: both adapters' partial_chainstate_snapshot functions]
2. Build a local candidate `Wallet::from_snapshot(prior.snapshot())`, call its pure `rescan_chainstate`, and inspect `candidate.utxos()`. No persisted wallet mutation occurs during this step. [VERIFIED: packages/open-bitcoin-wallet/src/wallet.rs; packages/open-bitcoin-wallet/src/wallet/scan.rs]
3. Gather requested heights and each candidate `created_height` in deterministic order. Every required height must map to an active-chain position; use actual heights, not an unchecked vector offset. Probe each distinct hash via `has_block` and return a typed refusal with boundary/height/hash where known. [VERIFIED: 152-CONTEXT.md D-02/D-04/D-05; packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs]
4. Return the candidate only after all probes succeed. The adapter then saves the candidate and job progress. If selection, creating metadata, authority, or payload probing fails, mark/save Failed and return the original failure; if saving Failed itself fails, return that persistence error visibly. [VERIFIED: 152-CONTEXT.md D-04/D-05; packages/open-bitcoin-node/src/wallet_registry.rs::save_wallet/save_rescan_job]

### Node Job Ordering

`advance_wallet_rescan` currently loads chain truth before loading its persisted job, and `has_block(...)?` escapes before `mark_failed`. Load/resolve the job first, return completed/failed jobs without unrelated scan reads, then perform authority assembly and staged eligibility inside an error-handled boundary. This allows existing Pending/Scanning jobs to become Failed even on authority/probe errors. Preserve the last successful wallet and checkpoint on a later chunk refusal. [VERIFIED: packages/open-bitcoin-node/src/sync/wallet_rescan.rs::advance_wallet_rescan; packages/open-bitcoin-node/src/wallet_registry.rs::mark_failed/requires_resume]

Enqueue requires chain tip information to create a truthful new target job. Do not invent a target hash/height if initial chain authority is wholly unavailable. Distinguish this from errors after an existing job is identified; the latter must record Failed. Creating-height eligibility happens after the normal Pending job is persisted. [VERIFIED: existing enqueue_rescan ordering; WalletRescanJob::new; 152-CONTEXT.md D-05]

### RPC Integration

`rescan_wallet_range` obtains a network-authority snapshot, validates bounds, saves Pending, probes only requested heights, scans, and saves. Replace the durable branch's probe/scan path with the shared staged gate and common failure handling; keep memory/local behavior compatible. **Resolved authority decision:** keep `blockchain_snapshot()` for the live RPC range. It reads `ManagedNetworkHandle::chainstate_snapshot`, which exports `Chainstate::admission_snapshot`: the durable parent coins plus active manager overlay, active positions, undo and confirmation counts. A durable-only reload may omit newly connected unflushed coins and use an older best-block; it must not replace the live admission view. Store-only construction/node scans continue using the guarded wallet loader. Add an interrupted-head guard to durable RPC operations without requiring the durable best-block to equal a newer live overlay tip. [VERIFIED: context/network.rs::blockchain_snapshot; network/runtime_authority.rs::chainstate_snapshot; node/chainstate.rs::export_chainstate_snapshot; chainstate/engine.rs::admission_snapshot; coins/cache.rs::collect_admission_unspent]

`wallet_state.rs::rescan_wallet(snapshot)` has a public durable branch bypassing range eligibility, although repository references found only dispatch test fixtures. Prevent a durable escape hatch when touching this seam; preserve the local fixture API. This observation does not establish an external RPC bypass. [VERIFIED: rg rescan_wallet references; packages/open-bitcoin-rpc/src/context/wallet_state.rs]

### InterruptedTwoHeads Authority Guard

`wallet_scan_chainstate_snapshot` checks `best_block` only when it is `Some`. `FjallCoinsView::best_block` returns `Ok(None)` for a valid two-head interrupted state; `head_blocks` returns both heads. `hydrate_chainstate_for_open` rejects this state before reading coins, whereas the wallet loader currently does not. Add the same head/recovery guard to the wallet loader before accepting a scan snapshot; preserve typed `InterruptedWrite` and do not repair or read leftovers. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/coins.rs; packages/open-bitcoin-node/src/storage/coins_view.rs::best_block/head_blocks]

### Probe/Save Concurrency Boundary

**Resolved scope:** prove eligibility after completed pruning and repeat probes on every chunk/reopen; this phase does not claim an atomic probe/save transaction. `http.rs::handle_single_request` holds the shared context mutex across synchronous dispatch, so `rescanblockchain` and `pruneblockchain` on that HTTP context serialize. Manual prune enters `context::flush_applying_prune_plan` and the network authority's `mutate` mutex. Automatic `flush_coins` currently uses an empty plan, and repository callers of planned prune outside the RPC path are test fixtures. Node `WalletRescanRuntime` is a separately exported API with direct store access; it does not participate in the RPC/authority mutex. Thus no global serialization guarantee is inferred for arbitrary direct library callers. [VERIFIED: rpc/http.rs:277–290; rpc/dispatch/wallet.rs; rpc/dispatch/prune.rs; rpc/context/prune.rs; node/network/runtime_authority.rs::mutate/flush_coins; node/network/runtime_authority/prune_flush.rs; rg planned-prune and WalletRescanRuntime callers]

**Current limitation and replan trigger:** concurrent direct-library prune or a future automatic retention owner can delete after a probe but before wallet persistence. Retain existing ownership for the completed-prune closure. If implementation introduces a new concurrent prune/rescan caller or claims atomic presence-at-save, replan around common owner serialization or an atomic store operation before making that claim. This bounds the implementation without leaving an unspecified planning choice. [VERIFIED: separate existing storage operations; Phase 153 owns automatic retention; 152-CONTEXT.md phase boundary]

## Don't Hand-Roll

| Problem | Don't build | Use instead | Why |
| --- | --- | --- | --- |
| Wallet candidate matching | Another descriptor matcher | Existing pure `rescan_chainstate` on staged wallet | One selection truth, including ranged descriptors. [VERIFIED: wallet/scan.rs] |
| Presence | Decode bodies or parse leftover snapshot | `has_block` | Existing byte-presence contract; no snapshot authority. [VERIFIED: fjall_store/blocks.rs] |
| Real pruning fixture | Remove an arbitrary key / set have-pruned manually | Existing paired-delete adapter | Deletes block+undo in SyncAll batch and records marker. [VERIFIED: fjall_store/prune.rs] |
| Resume/failure protocol | New wallet schema or job engine | `WalletRescanJob` / registry | Existing Pending/Scanning/Complete/Failed and persisted checkpoints. [VERIFIED: wallet_registry.rs] |

## Runtime State Inventory

This is a localized adapter refactor with no string rename or schema migration. Inventory is limited to state the changed paths consume. [VERIFIED: 152-CONTEXT.md D-08; traced code]

| Category | Items found | Action required |
| --- | --- | --- |
| Stored data | Named wallets, rescan jobs, coins, chain_meta, block/undo keys and leftover snapshot. [VERIFIED: Fjall store APIs] | No migration. Preserve wallet bytes on refusal; persist Failed through existing job shape; reevaluate pending jobs. |
| Live service config | Existing RPC/node runtime construction reads store/authority. [VERIFIED: context/network.rs; sync/open_runtime.rs] | Code edit only. External UI/service inventory not performed; no external config change planned. |
| OS-registered state | No registration surface touched. [VERIFIED: phase scope and traced paths] | None planned; no OS enumeration was performed. |
| Secrets/env vars | No secret/env rename requested. [VERIFIED: phase scope] | Existing HTTP auth retained; no secret mutation. |
| Build artifacts | Existing Cargo/Bazel outputs. [VERIFIED: workspace manifests/BUILD.bazel] | Rebuild through native verifier; no package rename or reinstall migration. |

## Common Pitfalls

- **Range-only validation:** Requested start controls progress, not which old coins the pure full replacement selects. Test a surviving matching coin created before start. [VERIFIED: node/RPC filter and wallet scan]
- **Overbroad old-history refusal:** Checking all old chain heights or all durable coin creating heights rejects unrelated pruned history. Include an unrelated old coin success control. [VERIFIED: 152-CONTEXT.md D-02]
- **Failure evidence bypass:** `?` on probes/authority escapes before job failure persistence. Test deterministic read-error injection at the shared callback seam and existing-job adapter handling. [VERIFIED: current node/RPC probe code; audit WR-02]
- **Missing metadata ignored:** Iterating only positions that exist never rejects a candidate whose creating height lacks a chain position. Validate required height membership explicitly. [VERIFIED: current iterator gates; 152-CONTEXT.md D-04]
- **Historical test false confidence:** Phase 146 source-string assertions do not execute real pruning; its missing-payload fixtures omit saves rather than delete paired keys. Keep those controls, add runtime deletion regressions. [VERIFIED: sync/tests/wallet_rescan_runtime.rs; context/tests/construction.rs]
- **Freshness versus failure:** `mark_failed` changes state/error but leaves last freshness/checkpoint. Preserve this existing protocol; assert Failed and unchanged checkpoint, rather than requiring unrelated freshness redesign. [VERIFIED: wallet_registry.rs::mark_failed]
- **Legacy snapshot resurrection:** A poison snapshot can disagree on UTXOs/tip and must remain ignored after reopen. Assert payload-derived refusal and unchanged durable coins, not merely a source-string absence. [VERIFIED: Phase 146 fixtures; 152-CONTEXT.md]

## Code Examples

### Existing Pure Staging API

Verified existing API usage; the eligibility helper is to be implemented, not an existing function. [VERIFIED: packages/open-bitcoin-wallet/src/wallet.rs]

```rust
let mut candidate = Wallet::from_snapshot(prior_wallet.snapshot());
candidate.rescan_chainstate(&partial_snapshot)?;
for utxo in candidate.utxos() {
    let creating_height = utxo.created_height;
    // Resolve creating_height in the active chain and probe in the node shell.
}
// Save candidate only after requested-range and candidate-creating probes pass.
```

### Real Paired Delete in RPC Fixture

The node unit tests can call crate-visible `commit_paired_delete`; RPC tests use the public `chainstate::FlushPersistSink` trait implemented for the store. `save_undo` and the real paired unlink are existing APIs. [VERIFIED: chainstate/flush_lifecycle.rs; chainstate/fjall_store.rs; storage/fjall_store/prune.rs]

```rust
use open_bitcoin_node::chainstate::FlushPersistSink;
store.save_undo(hash, &BlockUndo::default(), PersistMode::Sync)?;
let outcome = store.commit_paired_unlink(height, hash)?;
assert!(!store.has_block(hash)?);
assert!(!store.has_undo(hash)?);
assert!(store.load_have_pruned()?);
```

## Recommended Plan Split and Exact Regressions

These are proposed test names and destinations, not claims of existing tests. Each plan implements SNAP-01; split at the shared node/RPC dependency. [VERIFIED: roadmap requirements; traced adapter ownership]

### 152-01: Shared Eligibility and Node Runtime

- Add the shared staged replacement/filter/probe helper with deterministic typed errors and callback-based probe injection for tests. Add the two-head guard and load existing jobs before error-prone chain reads. Register new files. [VERIFIED: required seams above; recommended action]
- In the helper's unit tests: `creating_height_before_start_requires_payload`, `unrelated_old_coin_does_not_require_payload`, `missing_creating_chain_position_refuses_replacement`, `payload_probe_error_retains_height_and_hash`, and `duplicate_creating_height_is_probed_once`. [VERIFIED: D-02/D-04/D-05; recommended tests]
- In `sync/tests/wallet_rescan_runtime.rs` or a sibling module: `post_prune_midrange_rescan_preserves_wallet_and_fails_job`, `post_prune_resume_rechecks_earlier_creating_payload_after_reopen`, `post_prune_in_range_missing_payload_preserves_wallet`, and `retained_creating_payload_midrange_rescan_succeeds`. Include poison leftover/no-fallback assertions and full WalletSnapshot equality on failure. [VERIFIED: D-06/D-07; existing fixture/test module]
- In storage wallet-loader tests: `wallet_scan_two_heads_fails_closed_without_leftover_authority`; plant encoded H=[new,old], remove B, retain a C coin and chain_meta. Reuse the existing raw marker test facilities. [VERIFIED: coins_migration.rs interrupted-H fixtures; coins_view.rs cfg(test) raw APIs]

Node resume fixture: seed heights 0..3 and matching coins at 1/3; save all blocks plus undo; chunk size 2 completes the first chunk through 1. Record the wallet/job snapshot, pair-delete creating height 1, plant poison leftover, drop every store/runtime clone, reopen. Automatic resume through 2..3 must fail while preserving the first successful wallet and job checkpoint. A subsequent store-only reopen must still show Failed. [VERIFIED: existing restart_resume fixture; real delete implementation; D-06/D-07; recommended fixture]

### 152-02: Durable RPC Integration and Closure Evidence

- Route durable range scans through the shared gate, save Failed on missing metadata/probe/selection failures, and audit the durable `rescan_wallet(snapshot)` helper. Keep local memory fixture behavior. [VERIFIED: RPC seams above; recommended action]
- Add RPC tests in `context/tests/wallet_rescan.rs` or split the existing construction fixtures: `durable_post_prune_midrange_rescan_preserves_wallet_and_fails_job`, `durable_post_prune_reopen_refuses_poison_leftover`, `durable_post_prune_in_range_absence_preserves_wallet`, `durable_retained_payload_midrange_rescan_succeeds`, and `durable_unrelated_pruned_coin_does_not_block_wallet_rescan`. [VERIFIED: D-02/D-07; existing construction fixture API; recommended tests]
- For error handling, keep the literal H/B interruption fixture in node storage tests. `FjallCoinsView::write_raw_bytes/delete_raw_bytes` are `cfg(test)` and unavailable when node is an RPC dependency; store `put_bytes/get_bytes` are private and database/coins handles are crate-visible. Normal `CoinsView::batch_write` completes H/B transitions and offers no deterministic public pause between them. Do not widen raw mutation APIs or add a dependency solely for this test. [VERIFIED: coins_view.rs; fjall_store.rs::put_bytes/get_bytes; fjall_store/coins_access.rs; CoinsView trait]
- Feasible RPC propagation proof: extract a private, injectable authority/head-check closure at the durable rescan operation boundary; production passes the real Fjall head guard, while RPC unit tests pass typed `StorageError::InterruptedWrite { namespace: Coins, action: Reindex }` or a probe BackendFailure. Persist a real wallet/job, invoke the operation through this private seam, and assert Failed/unchanged wallet/reopen. Node tests separately prove real H=[new,old], missing B produces that exact error. No new public raw storage API is needed. [VERIFIED: existing typed error shape; node guard implementation pattern; recommended test seam]
- Update README phase status and parity catalog/index/checklist as required; preserve historical Phase 146 evidence while naming Phase 152 as the closure owner. Leave other audit gaps/Phase 153 requirements pending. [VERIFIED: docs/parity/index.json current Phase 146 ownership; scripts/check-phase151-parity-uat-release-boundary/gap-closure.ts]

RPC reproducer: persist a prior wallet already scanned through height 1, retain a matching durable coin created at 1, pair-delete block/undo at 1, then request start=2 stop=3. Assert RpcFailure, full prior wallet equality, Failed/error diagnostic, retained coins, both deleted keys absent, have-pruned true, and no authority from the poison leftover. Repeat after datadir reopen and retained-payload control. [VERIFIED: existing context construction fixtures; D-04/D-07; recommended fixture]

### Verification Commands

Run these sequentially through the cooperative timing wrapper; filters refer to existing module names or proposed post-prune test names. Test timings were not measured during research. Prefix every implementation and verification invocation with `PATH=/Users/peterryszkiewicz/Repos/open-bitcoin/packages/target/phase152/tooling/bun-darwin-aarch64:$PATH` so the isolated Bun 1.3.9 binary is used. The root prepared this ignored binary; its version was independently probed as 1.3.9. [VERIFIED: AGENTS.md; parent preparation message; isolated binary --version]

```bash
bun run scripts/command-timings.ts run --key phase152-node-wallet-rescan -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node wallet_rescan --all-features
bun run scripts/command-timings.ts run --key phase152-node-coins-migration -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node coins_migration --all-features
bun run scripts/command-timings.ts run --key phase152-rpc-post-prune -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc post_prune --all-features
bun run scripts/command-timings.ts run --key phase152-rpc-construction -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc context::tests --all-features
bash scripts/verify.sh
```

No Nyquist Validation Architecture section: `workflow.nyquist_validation` is explicitly false. The phase still requires runtime tests, coverage and default verifier. Completion checkers require Phase 152 summary plus lifecycle-valid verification, uniquely mapped SNAP-01 completion in roadmap/requirements; old Phase 146 completion cannot substitute. [VERIFIED: .planning/config.json; gap-closure.ts]

## State of the Art

| Existing approach | Required replacement | Impact |
| --- | --- | --- |
| Requested-range presence only | Range plus staged candidates' creating heights | Closes actual post-prune midrange gap. [VERIFIED: audit INT-02; CONTEXT D-02] |
| Duplicated snapshot filters | Shared filtering/staging/gating seam | One replacement contract in durable adapters. [VERIFIED: audit IN-02; CONTEXT D-03] |
| Unchecked absent best-block in interrupted state | Explicit head/recovery guard | Direct wallet/store consumers refuse interrupted coins truth. [VERIFIED: audit CR-01; coins loader/view] |

Pinned Knots range RPC checks `hasBlocks` and uses `ScanForWalletTransactions` to update transactions as it scans. Open Bitcoin's pure full replacement is different; its stronger creating-entry gate follows the project-locked contract, not a claim that Knots reconstructs the wallet from the UTXO set. Document this distinction in parity evidence. [VERIFIED: packages/bitcoin-knots/src/wallet/rpc/transactions.cpp:920; packages/bitcoin-knots/src/wallet/wallet.cpp:2000; first-party wallet/scan.rs]

## Assumptions Log

No locked implementation claims rely on training knowledge. The separate probe/save boundary is a documented scoped limitation with a replan trigger, not an assumed safety guarantee. [VERIFIED: evidence and resolution above]

| # | Claim | Section | Risk if wrong |
| --- | --- | --- | --- |
| — | None tagged ASSUMED | — | — |

## Open Questions (RESOLVED)

1. **RESOLVED — Concurrent prune ordering:** The HTTP context mutex spans both RPC dispatches; manual prune also holds the network mutation mutex while deleting. The current automatic flush submits no deletions. Separately exported node/store APIs have no shared wallet/prune transaction. Planning therefore targets completed-prune eligibility and fresh resume probes, with no atomic probe/save claim; a new concurrent owner or stronger claim triggers replanning. [VERIFIED: HTTP dispatch, authority flush and caller trace detailed above]
2. **RESOLVED — RPC authority source:** Keep the live manager admission snapshot, which merges durable parent coins with pending overlay updates. Add the head interruption check separately; do not compare a possibly older persisted B to the newer live snapshot tip. Direct store construction and node scans use the guarded durable loader. Test literal H/B at node scope and error propagation via a private injected guard at RPC scope. [VERIFIED: admission_snapshot/cache trace; private/cfg(test) API visibility; recommendations detailed above]
3. **RESOLVED — Bun execution precondition:** `.bun-version` pins 1.3.9; global `bun --version` reports 1.4.2. The native verifier requires Bun availability and does not compare versions; CI installs the pin through `setup-bun`. The root prepared ignored `packages/target/phase152/tooling/bun-darwin-aarch64/bun`, independently reporting 1.3.9. All execution/verifier commands use that directory first on PATH, leaving global tooling and repository pin unchanged. Required runtime availability is resolved; only successful default verification can establish suite compatibility/pass status. [VERIFIED: .bun-version; both version probes; scripts/verify.sh:177–192; .github/workflows/ci.yml:39–45; parent preparation message]

## Environment Availability

No external service or public network is needed for these local Fjall tests. Availability probes only establish installed tools, not passing builds. [VERIFIED: existing fixture architecture; session command probes]

| Dependency | Required by | Available | Version | Fallback |
| --- | --- | --- | --- | --- |
| Cargo/Rust | Local tests/verifier | Yes | Cargo 1.94.1 | None required. [VERIFIED: cargo --version] |
| Bun | Timing/native scripts | Yes | Isolated execution 1.3.9 / global 1.4.2 | Prefix PATH with ignored isolated binary directory. [VERIFIED: both --version probes; .bun-version] |
| Bazel entrypoint | Native smoke | Yes | Binary at /opt/homebrew/bin/bazel; runtime version not probed | Native verifier establishes pinned smoke. [VERIFIED: command -v bazel] |
| Bitcoin Knots submodule | Anchors/parity | Yes | a9aee730466ac67d35a3c03ee24676be5e045878 | Existing materialized baseline. [VERIFIED: git submodule status] |

**Missing dependencies:** None detected by these probes; no claim of full environment health or current test success. [VERIFIED: session probes; tests not run by researcher]

## Security Domain

Security enforcement is enabled by absence of an explicit false configuration. Category names below are intentionally the template's ASVS 4.0.3 taxonomy, verified against the OWASP-hosted category index; this is a scoped threat review, not a certification target or claim of the latest ASVS version. [VERIFIED: .planning/config.json; CITED: https://cornucopia.owasp.org/taxonomy/asvs-4.0.3]

| ASVS category | Applies | Existing control / scoped action |
| --- | --- | --- |
| V2 Authentication | Existing RPC boundary | Preserve auth before JSON parsing/dispatch. [VERIFIED: packages/open-bitcoin-rpc/src/http.rs:136] |
| V3 Session Management | No new session protocol | Existing context/state handling; phase adds no session flow. [VERIFIED: phase scope; http.rs] |
| V4 Access Control | Named wallet selection | Preserve selected/request wallet resolution before registry mutation. [VERIFIED: context/wallet_state.rs; context/rescan.rs] |
| V5 Input Validation | Yes | Height bounds, chain-position membership, typed probe/read refusal. [VERIFIED: context/rescan.rs; CONTEXT D-04] |
| V6 Stored Cryptography | No new crypto | Existing descriptor/signing remains pure; no algorithm change. [VERIFIED: phase scope; wallet scan] |
| V7 Error Handling and Logging | Yes | Durable Failed evidence without raw paths; visible persistence failure. [VERIFIED: CONTEXT D-05] |

| Threat pattern | STRIDE concern | Mitigation |
| --- | --- | --- |
| Missing/pruned creating payload admitted by full rebuild | Tampering/integrity | Shared candidate gate before save; preserve old snapshot. [VERIFIED: INT-02; D-04] |
| Read failure leaves Pending/Scanning without refusal evidence | Repudiation | Save Failed and retain checkpoint. [VERIFIED: audit WR-02; D-05] |
| Interrupted coins markers accepted as settled truth | Tampering/integrity | Head/recovery guard; explicit refusal, no repair. [VERIFIED: audit CR-01; coins_view.rs] |
| Reusing stale success across prune/reopen | Integrity | Reprobe each chunk; real paired-delete/reopen regression. [VERIFIED: D-06/D-07] |

## Sources

### Primary (HIGH confidence)

- `152-CONTEXT.md`, `.planning/ROADMAP.md`, `.planning/REQUIREMENTS.md`, `.planning/PROJECT.md`, and `.planning/v2.4-MILESTONE-AUDIT.md`: locked scope, INT-02 and residuals. [VERIFIED: session reads]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, relevant architecture/code-shape/testing/verification/Rust standards: constraints. [VERIFIED: session reads]
- Node `sync/wallet_rescan.rs`, `sync/tests/wallet_rescan_runtime.rs`, `wallet_registry.rs`, storage `fjall_store/coins.rs`, `blocks.rs`, `prune.rs`, `tests/prune_unlink.rs`, `tests/coins_migration.rs`, `coins_view.rs`: runtime/gate/job/storage behavior and fixtures. [VERIFIED: session reads]
- RPC `context/rescan.rs`, `wallet_state.rs`, `network.rs`, `context/tests/construction.rs`, `http.rs`: durable range and authority/HTTP seams. [VERIFIED: session reads]
- Wallet `wallet.rs`, `wallet/scan.rs`; pinned Knots `src/wallet/wallet.cpp`, `src/wallet/rpc/transactions.cpp`: pure replacement and external reference distinction. [VERIFIED: session reads]
- `scripts/check-phase151-parity-uat-release-boundary/gap-closure.ts`: new closure-owner evidence requirements. [VERIFIED: session read]
- [OWASP ASVS 4.0.3 category index](https://cornucopia.owasp.org/taxonomy/asvs-4.0.3): scoped category names. [CITED: cornucopia.owasp.org/taxonomy/asvs-4.0.3]

### Secondary / Tertiary

None used for implementation recommendations. Library capability claims come from pinned project code; no package upgrade or external ecosystem recommendation was made. [VERIFIED: research source set]

## Metadata

| Area | Confidence | Reason |
| --- | --- | --- |
| Standard stack | HIGH | Pinned manifests/lock and tool probes. [VERIFIED: cited local sources] |
| Architecture | HIGH, global atomicity explicitly outside claim | Adapter/manager-overlay and RPC serialization traced; direct-library race limitation has a replan trigger. [VERIFIED: cited local sources] |
| Pitfalls | HIGH | Audit defects match source and real deletion fixtures. [VERIFIED: cited local sources] |

**Research date:** 2026-10-02 CDT. **Validity:** Until traced adapter/storage contracts or concurrency owners change; use the recorded replan trigger. Research performed source investigation and tool availability probes, not behavioral test execution. All previously listed planning questions have resolved decisions or scoped limitations. [VERIFIED: session actions; resolution section]
