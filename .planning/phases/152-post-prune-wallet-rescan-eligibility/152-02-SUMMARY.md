---
phase: 152-post-prune-wallet-rescan-eligibility
plan: "02"
subsystem: rpc
tags: [wallet, rescan, fjall, prune, durable-evidence]
requires:
  - phase: 152-01
    provides: Shared staged replacement, creating-payload gate and safe typed refusal
provides:
  - Durable RPC range and direct helper consume shared eligibility
  - Refusal preserves the actual wallet/job checkpoint and persists safe Failed evidence
  - Real paired-prune, later request, reopen and missing-target regressions
affects: [152-03, SNAP-01]
tech-stack:
  added: []
  patterns: [identified job before authority reads, shared preparation before wallet persistence]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility.rs
    - packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility/fixtures.rs
  modified:
    - packages/open-bitcoin-rpc/src/context/rescan.rs
    - packages/open-bitcoin-rpc/src/context/wallet_state.rs
    - packages/open-bitcoin-rpc/src/context/tests.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "Preserve the live manager admission snapshot and validate durable head markers separately."
  - "Reuse the existing successful job checkpoint; never infer successful progress from a requested start height."
  - "Missing stop-position metadata refuses without inventing a target hash or a new job."
  - "Keep requirements-completed empty and defer commits/tracking/generated LOC to root full verification."
patterns-established:
  - "RPC private callbacks inject typed errors through the production refusal handler and real Fjall registry persistence."
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 152-2026-10-03T00-26-52
generated_at: 2026-10-03T01:46:17Z
duration: approximately 35 min including review fix
completed: 2026-10-03
---

# Phase 152 Plan 02: Durable RPC Wallet Rescan Eligibility Summary

**Durable RPC replacements now refuse unavailable older creating payloads, retain saved wallet/checkpoint state, and persist sanitized Failed evidence through the shared node eligibility contract.**

## Performance

- Tasks: 2/2 implemented; targeted verification recorded below.
- Source/manifest paths changed: six (two adapters, test registration, test module, fixture module and breadcrumb manifest).
- Full default repository verification, generated LOC freshness, canonical state/roadmap/requirements updates and git finalization belong to the root executor and remain outside this summary's success claim.

## Accomplishments

- Durable range scans resolve the named wallet and previous job before authority checks. They preserve the live manager admission snapshot, stage the replacement through `prepare_durable_wallet_rescan`, and save the wallet/progress only after eligibility succeeds.
- New Pending jobs inherit the actual saved wallet checkpoint and MTP. Existing jobs retain checkpoint, next height and MTP; freshness is recalculated against the new target before Pending is persisted. Known earlier checkpoints report Partial, while unknown checkpoints keep Scanning. Neither a later requested start nor missing stop-position metadata fabricates successful progress or a target hash.
- Identified authority failures save Failed through the same handler; authority failures without a known target/job refuse without creating invented metadata. Preparation failures record shared safe boundary/category/known height/hash. Failure-save errors propagate visibly.
- Durable `rescan_wallet(snapshot)` delegates to the authoritative range adapter, closing the caller-supplied snapshot bypass. Local fixture scans retain their supplied-snapshot behavior.
- Tests perform `FlushPersistSink::commit_paired_unlink` on the same Fjall store supplied to RPC and assert both mates absent, have-pruned true, durable coins/best-block retained and other payload mates retained. Complete persisted wallet equality covers balances, UTXOs, tip and MTP.
- Reopen tests drop all contexts/store clones, use the fallible with-store constructor, and prove conflicting leftover snapshots remain present but non-authoritative. Partial-success/later-request tests recheck earlier creating payloads both before and after reopen.
- Private adapter callbacks inject typed payload/authority/failure-save errors into the production handler. Real H=[new,old]/missing-B storage fixtures remain node-owned; RPC tests do not claim to plant those markers.

## RED and Verification Evidence

All executor Cargo commands ran sequentially through pinned Bun 1.3.9 and `scripts/command-timings.ts`. An external IDE Cargo workspace check held the artifact lock during setup; its live rustc children were observed and it finished without termination. No executor Cargo jobs were overlapped.

| Command/filter | Result |
| --- | --- |
| RPC `--lib context::tests::rescan_eligibility --all-features`, unchanged adapter, corrected durable fixture | **RED:** exit 101; retained control passed, real paired-prune reproducer failed because the old adapter returned `Ok(Fresh, through=3)` for start=3/stop=3 after deleting the matching coin's creating height 1. Durable variant and live/durable UTXO equality were explicitly asserted. |
| Same two tests after shared gate | 2 passed / 0 failed. |
| Extended matrix before fixture split | 13 passed / 0 failed. |
| Final split matrix including missing stop-position cases | 16 passed / 0 failed (session 66114, exit 0). |
| WR-01 changed-target Failed-save regression before review fix | **RED:** exit 101; persisted Pending job reported Fresh rather than Partial at checkpoint 1 / target 3. |
| Intermediate matrix after WR-01 review fix | 18 passed / 0 failed (session 47374, exit 0), including new/reused checkpoint freshness, unknown checkpoint behavior, Pending persistence on Failed-save error and wallet-freshness readback. |
| Final matrix with directly observed Fresh-arm refusal | 19 passed / 0 failed (session 68125, exit 0). Actual paired prune at height 1 followed by stop=1 preserves wallet/checkpoint and persists Failed/Fresh at target 1; this observes the >= target projection before progress can overwrite it. |
| WR-01 RPC `--lib freshness --all-features` | 4 passed / 0 failed, including wallet-info projection, range Partial/Fresh reporting and the new status regressions. |
| WR-01 scoped RPC Clippy before final Fresh-arm test | Passed, exit 0 (session 48483). |
| Final WR-01 RPC `clippy --all-targets --all-features -- -D warnings` | Passed, exit 0 (session 15019). |
| RPC `clippy --all-targets --all-features -- -D warnings` | Passed, exit 0. |
| RPC `--lib context::tests::construction --all-features` | 5 passed / 0 failed, including existing missing-payload and leftover-snapshot controls. |
| RPC `--lib rescanblockchain --all-features` | 1 passed / 0 failed, existing range/partial-freshness dispatch regression. |
| Node `--lib wallet --all-features` | 39 passed / 0 failed, including shared eligibility, runtime/resume, real H/B refusal and wallet registry persistence cases. |
| Workspace `cargo fmt --all` via timing wrapper | Passed. |
| `bun scripts/bright-builds-check.ts all` | Zero findings, 1,222 files scanned; no file-cap exception. |
| `bun run scripts/check-parity-breadcrumbs.ts --check` | 911 Rust files verified. |
| `git diff --check` | Passed. |

Setup compiler failures for the trait import/mutable sink and moved-module paths were corrected before final verification. An initial fixture omitted `maybe_data_dir`, selected Local, and therefore was explicitly excluded from durable RED evidence. The later corrected durable run above is the actual reproduction.

## Decisions Made and Concurrency Bounds

The implementation follows D-01 through D-09 and the resolved research dispositions. `blockchain_snapshot` continues to read the manager admission view that merges durable parent coins with its pending overlay; it is not replaced by an older disk-only snapshot and no persisted-best-block/live-tip equality check was added.

The owner trace confirms `http.rs::handle_single_request` holds the context mutex across synchronous dispatch; reachable rescan and manual-prune RPC calls on that context serialize. Manual prune enters `context/prune.rs::flush_applying_prune_plan` and the network authority's `mutate` mutex. Current automatic `flush_coins` supplies an empty prune plan. Separately exported node/store callers do not share an atomic wallet probe/save transaction. This implementation proves eligibility after completed pruning and fresh rechecks on later requests/reopen; a concurrent direct-library prune can still delete after probing and before persistence. New concurrent owners or stronger atomicity claims require replanning.

An explicit simplification pass extracted Pending-job construction, reused persisted jobs directly, removed a redundant registry reload, and consolidated fixture cleanup/job reads. Durable eligibility remains entirely in the shared helper; the local-only partial-snapshot filter does not implement a second durable gate.

## Deviations from Plan

**Organization-only fixture split:** Formatter-expanded tests exceeded the managed 628-line cap. The root updated Plan 02 ownership and authorized `context/tests/rescan_eligibility/fixtures.rs`; setup and shared assertions moved there, leaving runtime tests in the parent module. No behavior/dependency/storage API expansion or checker exception was introduced. Final source files are under the cap.

**Missing-target guard refinement:** Diff review identified the old all-zero hash fallback when the requested stop position is absent. It now refuses before creating a new job and updates only an identified existing job with safe Failed detail. Two actual network-fixture regressions prove target/checkpoint preservation and the absence of invented target metadata.

**[Rule 1 - Bug] WR-01 stale freshness during retarget:** Review found that reusing a Complete/Fresh job and advancing its target preserved Fresh even though its checkpoint was earlier. Pending construction now recomputes freshness for both reused jobs and new jobs from known wallet checkpoints. `mark_failed` remains unchanged; it retains the corrected freshness. A Failed-save error leaves durable Pending/Partial evidence and RPC readback reports scanning=true/Partial. Tests also retain Scanning for unknown checkpoints. An attempted domain-state import was unavailable through the node facade and RPC has no direct wallet dependency, so the implementation uses the equivalent three-case checkpoint projection without adding a dependency. Import compilation failures are not counted as behavioral RED. Plan 02 records this reviewed evidence hazard as T-152-17.

## Review Threat Flag

| Flag | File | Description and mitigation |
| --- | --- | --- |
| threat_flag: job-evidence-freshness (T-152-17) | `context/rescan.rs` | Reused/new Pending jobs must not claim Fresh when the saved checkpoint precedes a changed target. The checkpoint projection now runs before persistence, and runtime refusal/readback regressions verify truthful evidence even when saving Failed returns an error. |

## Task Commits and Tracking

Task commits and metadata commits are deliberately deferred by the strict lifecycle instruction: the root owns git and the final clean default verifier gate. This executor performed no staging, commit, push, branch/worktree operation, hook bypass or requirement completion. Root-created intent-to-add entries make the two new test files visible to tracked-only managed checkers. `requirements-completed` remains empty until lifecycle-valid root evidence closes SNAP-01.

## Stub and Threat Review

No blocking placeholders or unwired production stubs were found. Empty/favorable snapshots occur only as deliberate local/bypass test inputs. The work adds no endpoint, auth path, raw storage mutation API, schema or dependency; it uses the existing named registry, range validation and safe shared error formatter within the plan's threat model.

## Next Phase Readiness

Plan 03 can record parity/operator evidence for the guarded RPC surfaces and the real-prune matrix. Root must still run the default `bash scripts/verify.sh`, retain verifier-generated LOC freshness, perform lifecycle report/requirements closeout, and finalize git before overall phase completion.

## Self-Check: PASSED

All six source/manifest paths and this summary exist. Final managed checks, parity breadcrumbs and diff whitespace checks passed. Runtime result counts above are actual completed tool outputs. There are no executor commit hashes to check because commits were explicitly deferred to the root's clean full-verification gate; no commit success is claimed. Tracking files were left to the root as instructed.

The WR-01 follow-up self-check also passed: all three freshness arms are observed by runtime refusal tests, the final 19-test matrix and four relevant freshness regressions passed, final scoped Clippy passed, formatting passed, and managed/breadcrumb checks remain clean without a file-cap exception. No executor Cargo session remains.
