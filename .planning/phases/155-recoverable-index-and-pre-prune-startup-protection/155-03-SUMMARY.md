---
phase: 155-recoverable-index-and-pre-prune-startup-protection
plan: "03"
subsystem: storage
tags: [basic-filters, startup, prune-protection, fjall, rust]
requires:
  - phase: 155-01
    provides: Pure coins-fenced recovery and direct input protection
  - phase: 155-02
    provides: Streamed durable integrity checks and atomic checkpoint publication
provides:
  - Mandatory recovered-coins index guard before startup prune resume
  - Real production reopen refusal, conservative rewind and safe legacy controls
  - Same-height saved fence and checkpoint consistency validation
affects: [155-04, compact-filter-index, durable-startup]
tech-stack:
  added: []
  patterns: [exclusive pre-readiness startup guard, reuse of SyncAll reconciliation publisher]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/startup.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/startup/faults.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/startup/recovery.rs
  modified:
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/faults.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Check live intents against recovered protection before any reconciliation mutation.
  - Reject same-height saved fence and endpoint hash conflicts; historical higher fences are provenance rather than recovery authority.
  - Reuse the concrete atomic publisher without adding an activation or pruning owner.
requirements-completed: [CFPR-03]
requirements-addressed: [CFIX-02, CFIX-04, CFPR-03]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 155-2026-10-04T16-01-08
generated_at: 2026-10-04T17:58:00Z
commits: []
git_finalization: pending consolidated parent commit after phase verification and full native gate
duration: 14min
completed: 2026-10-04
---

# Phase 155 Plan 03: Production Pre-Prune Protection Summary

**Production initialize validates and reconciles the BASIC index after coins recovery and before prune resume, preserving required history and interrupted intent on refusal.**

## Performance

- First recorded test command: 2026-10-04T17:43:52.788Z; summary written 2026-10-04T17:58:00Z.
- Both implementation tasks complete; 11 owned source/test/parity paths plus this summary.
- Final matching node suite: 56 tests passed in 21.87s test execution time. This includes 26 new production runtime tests and one new concrete storage consistency test, alongside the 29 existing storage/codec tests.
- Existing initialize suite: 23 matching tests passed in 11.85s. Existing restart_chainstate suite: 10 tests passed in 4.34s. These filtered suites have overlapping coverage and are not a distinct-test total.
- Normal Cargo artifact-lock and executable launch waits were polled; no process was terminated or execution security metadata changed.

## Accomplishments

- Added crate-internal `FjallNodeStore::recover_basic_filter_index_before_prune(Option<BlockHash>) -> Result<(), StorageError>`. Every production initialize calls it after successful H/B recovery and recovery-outcome projection, before loading resume locks, resuming an intent, constructing the cache or declaring readiness.
- Legacy absence requires state, immutable record prefix, active projection prefix and reserved identity all to be absent. A lone record, projection or reserved lock refuses rather than silently initializing authority.
- Saved state, complete immutable/projection integrity, saved checkpoint and the reserved protection pair are validated before the recovered coins B/full durable metadata fence determines Keep/Reconcile/Refuse. Missing, corrupt, malformed or weaker protection never gets automatically recreated. Same-height saved fence/endpoint hash disagreement now refuses in the common checkpoint parser.
- Unsafe intent checks use the pure recovered protection directly before mutation. Tests cover genesis, height one and u32::MAX, including stronger saved protection that the buffered ordinary helper alone cannot enforce at low heights. Reserved lock endpoints that would overflow the existing buffer calculation refuse before the generic helper runs.
- Reconciliation uses the existing publisher with no candidate rows. It rereads B and metadata and atomically SyncAll-publishes the recovered endpoint/fence and stronger protection. Immutable records and hidden projection suffix remain intact. The complete integrity scan retains the Plan 02 linear direct-edge design; no per-row ancestry walk was reintroduced.
- `sync/open_runtime.rs` is unchanged, including the literal initialize, `Chainstate::from_coins_cache` and `ManagedChainstate::from_chainstate` constructor anchors. Only the two temporary unused-method allowances were removed; four narrow future activation/catch-up/serving allowances remain.

## Production Persistence Evidence

The fixtures are explicitly sparse codec-valid historical coinbase-only data, without a claim of consensus validation. Matching chain metadata extends to height 400 so height-20 unsafe and height-1 safe intents are outside the existing keep window. All production reopen paths use real Fjall drop/reopen and `DurableSyncRuntime::open`, without a post-construction test lock.

Refusal tests independently reopen storage and assert exact body/undo values and presence, a live unchanged intent, checkpoint/projection/protection facts where decodable, and byte-for-byte equality of the entire BlockIndex namespace. That snapshot includes all immutable envelopes, projection, saved state, protection, intent and the selected block body. A corrupt intent test separately asserts it remains corrupt rather than claiming a successful decode.

The matrix covers required history; missing, corrupt, weak, overflowing and ordinary-only reserved protection; corrupt state/record/projection; missing state; each lone partial artifact; missing coins B; missing metadata; B/metadata disagreement; corrupt intent; low/max intent heights; and both one-body and one-undo interrupted-prune variants. Safe indexed and fully absent legacy controls actually delete both payload mates and clear the intent while preserving filter rows. Safe single-mate variants finish the remaining deletion.

Ahead saved checkpoints recover from historical fence height 400 to recovered B/metadata height 1, then persist endpoint 1 and protection from height 2 while preserving every immutable/projection row. Wrong-branch recovery retains only the matching indexed prefix; no common recovered genesis refuses with unchanged index bytes. An unsafe intent at the newly required rewind boundary refuses before publication, leaving the old checkpoint and protection unchanged.

Startup publication faults before checkpoint, before protection and after successful SyncAll commit prevent readiness and retain payloads. Real reopen observes either the complete old authority/protection pair or the complete stronger reconciled pair. These software boundary tests do not claim hardware power-loss simulation.

## Saved Fence Authority and Downstream Handoff

Historical `StoredFilterState.fence_height/fence_hash` records publication provenance. A historical higher fence is not an authority to advance a cursor and is not authenticated by reconstructing an unavailable historical header chain. Current recovered B, full compatible durable active metadata and the verified contiguous saved projection independently decide safe progress/protection. A saved fence at exactly the endpoint height must identify that endpoint; a conflicting same-height hash refuses. The production ahead and wrong-branch tests prove legitimate historical higher-fence state can reconcile conservatively without inventing historical proof.

Startup is exclusive before runtime construction exposes coins/metadata writers. The existing runtime owner must continue to serialize later coins/metadata/prune operations; the concrete filter publisher's mutex and reread are not CAS against arbitrary competing raw writers. No public activation, scheduling, worker, disable path, prune-lock CRUD bypass or runtime reorg orchestration was added.

## Verification

- RED `phase155-startup-guard-red`: the new production reopen test failed because existing startup produced only the generic prune-resume diagnostic. The fixture was subsequently extended from height 300 to 400 to exclude keep-window refusal as a confounder.
- Initial GREEN `phase155-startup-initial-green`: 33 matching node library tests passed.
- Expanded matrix: initial 46-test run had one overly specific test diagnostic expectation (`prune lock` versus actual `truncated prune record`); the expectation was corrected without changing production behavior. The next 51-test matrix passed.
- Final `phase155-production-startup-filter-tests`: all 56 matching node library tests passed, including the added storage/production fence-conflict and recovered-ancestry evidence.
- `phase155-existing-initialize`: all 23 matching node library tests passed, including successful interrupted coins replay and existing finish-or-Repair behavior.
- `phase155-existing-runtime-restart`: all 10 restart_chainstate library tests passed.
- `phase155-startup-clippy`: node crate, all targets/all features, `-D warnings`, passed.
- Phase 123/135 live checkers passed; their combined explicit-file Bun mutation suites passed all 122 tests and 185 assertions.
- Parity breadcrumbs passed for 945 tracked Rust files. All five new Rust paths and their mapping are staged for inventory.
- Pure-core dependency/import checks, production panic-site checks, scoped rustfmt check and staged/unstaged diff whitespace checks passed.
- Managed file-length checker scanned 1,262 tracked source files with zero exceptions/findings. The largest new file has 592 lines.

All Cargo commands used the timing wrapper and library scoping. No overlapping owned Cargo jobs were launched. Full native/integration/Bazel verification remains the consolidated parent gate.

## Simplification and Threat Review

One early shell consumer reuses the pure recovery reducer, existing bounded full scan and concrete atomic publisher. It owns no second index cache, manager, deletion implementation or repair machinery. The simplification pass kept fail-fast typed protection extraction and a single recovered-fence lifetime.

T-155-10: the mandatory initialize call precedes resume/readiness, with real production reopen evidence. T-155-11: saved protection validation precedes rewind and atomic strengthening; refusal snapshots show no automatic recreation or relaxation. T-155-12: direct 0/1/max intent checks and malformed max-endpoint refusal avoid buffered helper holes/overflow. T-155-13: index-specific fail_closed/Repair diagnostics are required independently of generic keep-window/ordinary-lock gates, with persisted payload/intent/index assertions. No unresolved HIGH finding was identified.

No new unmodeled endpoint, authentication path, filesystem access pattern or schema trust boundary was introduced. Stub scan found no placeholder or unwired path needed for this plan's goal. The four future-scope allowances remain intentional and narrowly documented.

## Deviations from Plan

No scope expansion. Parent review requested the same-height saved fence/endpoint consistency check in the shared storage parser and matching storage/production tests; this is a correctness refinement of saved-state validation. Tests were split into small fault/recovery child modules to preserve the 628-line limit, with all added paths registered for parity.

The authorized plan rules defer commits, canonical state/roadmap/requirement updates and requirement activation to the parent. No auth gate or external setup was required. STATE, ROADMAP and REQUIREMENTS were not edited by this executor.

## Next Plan Readiness

Plan 155-04 can check the actual pre-prune consumer, final test matrix and historical-fence authority distinction, then finish parity/docs and the full native verifier. Requirement IDs remain Pending until lifecycle-valid phase verification and the consolidated native gate pass. Git finalization is pending the parent commit.

## Self-Check: PASSED

All 11 owned source/test/parity paths and this summary exist; focused behavior, lint and static checks passed. No commits were created, so commit verification is intentionally inapplicable to the authorized consolidated-parent workflow.
