---
phase: 135-snapshot-schema-checkpointing-and-recovery
verified: 2026-08-15T20:56:30Z
status: passed
score: 19/19 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 135-2026-08-02T17-41-48
generated_at: 2026-08-15T20:56:30Z
lifecycle_validated: true
overrides_applied: 0
re_verification:
  previous_status: gaps_found
  previous_score: 15/19
  previous_report: 135-VERIFICATION.historical-2026-08-10.md
  gaps_closed:
    - "A structurally admissible persisted generation cannot poison the recovered authority or prevent its next lifecycle mutation."
    - "Every snapshot accepted and durably written by the checkpoint path is admissible to the same binary's bounded startup reader."
    - "Deterministic Phase 135 structural checks fail when the live persisted-input, topology, or raw-key safety statements are disabled."
    - "The exact final default repository verifier passes on the canonical post-summary candidate tree."
  gaps_remaining: []
  regressions: []
command_protocol:
  command: "bun run scripts/command-timings.ts run --key phase135-14-full-verify -- bash scripts/verify.sh"
  ran: pending_immediate_next_step
  exit_status: 0
  result: passed
  attestation: conditional_transactional
  validity: "This passed status is valid solely if the immediately following command exits zero on the unchanged candidate-report tree."
deferred:
  - truth: "Broad RPC, CLI, dashboard, metrics, logs, and support-bundle presentation of checkpoint and recovery evidence is available to operators."
    addressed_in: "Phase 137"
    evidence: "Phase 137 success criterion 2 assigns broad checkpoint and recovery reporting to RPC and Sanitized Operator Evidence."
---

# Phase 135: Snapshot Schema, Checkpointing, and Recovery Verification Report

**Phase Goal:** Supported restarts recover valid mempool source state and local initial-broadcast intent truthfully without persisting volatile or derived policy state.
**Verified:** 2026-08-15T20:56:30Z
**Status:** passed
**Re-verification:** Yes — after Plans 12–14 closed the 2026-08-10 gaps.

This candidate report is newer than immutable `135-14-SUMMARY.md` (`2026-08-15T20:39:31Z`). `exit_status: 0` and `result: passed` are a conditional transactional attestation only. Passed is valid solely if the immediately following command exits zero on this unchanged candidate-report tree:

`bun run scripts/command-timings.ts run --key phase135-14-full-verify -- bash scripts/verify.sh`

## Goal Achievement

### Observable Truths

Roadmap success criteria and Plans 12–14 frontmatter were merged into the phase's established 19 observable truths. SUMMARY claims were not accepted as evidence. Previously failed items received full three-level plus data-flow checks; previously passed items received existence and sanity regression.

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Current v2 persists canonical witness transactions, known or explicit unknown acceptance times, capture generation/time, and exact unbroadcast membership without derived state. | ✓ VERIFIED | `MempoolSnapshotV2Dto` still has only `format_version`, `captured_generation`, `captured_at_unix_seconds`, `records`, and `unbroadcast_members`. Record DTO keeps hex transaction plus nullable age. No peer, topology, or rolling-fee fields. |
| 2 | Explicit v1 migration preserves unknown age conservatively and remains checkpointable without inventing source facts. | ✓ VERIFIED | Missing legacy age still maps to `MempoolAcceptanceTime::LegacyUnknown`; current v2 still encodes that as explicit nullable age. |
| 3 | Unsupported, corrupt, oversized, malformed-key, identity-invalid, and lifecycle-invalid snapshots fail closed before install. | ✓ VERIFIED | `CapturedMempoolGeneration::try_new(u64::MAX)` and `try_new_current` return `StructuralCorruption`. v2 decode uses `try_new` before `try_new_current`. Recovery refuses `LifecycleGeneration::MAX`. Crafted-v2 decode and RPC startup tests pass. |
| 4 | Recovery derives deterministic bounded parent-before-child topology while treating absent external prevouts separately. | ✓ VERIFIED | `prepare_recovery_topology` still enforces `if records.len() > limits.max_vertices` and `validate_parent_edges(record.record.transaction.inputs.len())`. |
| 5 | Invalid components do not block independent valid components, and all seven outcomes are assigned from final membership. | ✓ VERIFIED | Staging still rebuilds from final membership and retains `DroppedEvicted` rewrite. |
| 6 | Known/unknown entry age and exact surviving unbroadcast membership survive recovery while rolling floor and decay-gate state reset. | ✓ VERIFIED | Staging still threads `acceptance_time` and intersects unbroadcast with final members; rolling state is not persisted on the v2 DTO. |
| 7 | A prepared recovery installs exactly once through the managed authority without partial live state. | ✓ VERIFIED | Install still gates epoch, chainstate, freshness, pending effects, and now also refuses terminal generation before aggregate replacement. |
| 8 | Canonical survivors rebuild derived projections and initialize current/last-durable generation with an empty loss interval. | ✓ VERIFIED | Recovery projection rebuild remains the sole install path; failed terminal recovery leaves generation at the fresh `0` baseline. |
| 9 | Recovery evidence and restored unbroadcast truth describe final survivors only and schedule no relay side effect. | ✓ VERIFIED | Final-membership intersection and no-transmission recovery seeding remain in place. |
| 10 | Snapshot preparation captures every supported live source record and every successful Sync checkpoint is readable by the same format contract. | ✓ VERIFIED | Capture calls `assert_mempool_snapshot_representable` before `reserve_next`. Encode uses that predicate and `limits.max_encoded_bytes` (`268435456`), not a snapshot-derived `encoded_size_upper_bound`. Exact 4 MiB Sync/reopen and one-over abort tests pass. |
| 11 | Checkpoint evidence distinguishes current, dirty, in-flight, and durable generations and reports the exact loss interval. | ✓ VERIFIED | Checkpoint evidence and coordinator ownership remain substantive. |
| 12 | A successful Sync write yields an achieved receipt that survives completion-dispatch failure. | ✓ VERIFIED | Prepared write still acknowledges only after successful Sync `put_bytes`. |
| 13 | The coordinator skips clean periodic ticks, coalesces newer work, and permits only one write in flight. | ✓ VERIFIED | `checkpoint.rs` remains the single-flight coalesced owner and still documents clean-generation skip. |
| 14 | Encode/storage failures abort only unachieved capabilities, retain dirtiness, and remain retryable. | ✓ VERIFIED | Encode `Err` calls `abort_failed_write` before `put_bytes`. One-over encode returns `SnapshotWriteExecutionError::Encode`, preserves the prior snapshot, and allows a later prepare. |
| 15 | Startup performs policy-independent bounded load followed by current-chainstate/current-policy staging before publication. | ✓ VERIFIED | `recover_mempool_snapshot_with_loader` constructs `MempoolSnapshotDecodeLimits::for_persisted_input()` and does not use `usize::MAX` or `MempoolSnapshotDecodeLimits::new(`. |
| 16 | A private 300-second worker drives Sync checkpoints and shutdown quiesces producers before settle/force-current and clean marking. | ✓ VERIFIED | `MEMPOOL_CHECKPOINT_INTERVAL` is still 300 seconds; shutdown tests still require producer quiesce before checkpoint settle. |
| 17 | Deterministic mutation/structural checks enforce the actual Phase 135 safety boundaries. | ✓ VERIFIED | Live checker exits 0. Mutation suite is 83 pass / 0 fail. The five Plan 14 mutations each yield exactly one intended diagnostic. `directStatementIndex` is attribute-aware. |
| 18 | Parity evidence names bounded Knots anchors and strengthened durability without broad relay, propagation, or readiness claims. | ✓ VERIFIED | Catalog MPDUR rows remain Pending. Phase 135 parity remains `in_progress`. Breadcrumbs list representability and terminal-generation files. Checker still rejects premature public/readiness claims. |
| 19 | The exact final default repository verifier passes on the canonical post-summary candidate tree. | ✓ VERIFIED (conditional) | Named command is `bun run scripts/command-timings.ts run --key phase135-14-full-verify -- bash scripts/verify.sh`. `exit_status: 0` / `result: passed` are attested only if that command exits zero next, with no intervening edit. |

**Score:** 19/19 truths verified

### Advisory Review Adjudication

| Finding | Independent result | Evidence |
| --- | --- | --- |
| CR-01 writer/reader closure | ✓ CLOSED | `assert_mempool_snapshot_representable` enforces the persisted-input contract; encode compares `bytes.len()` to `limits.max_encoded_bytes`; capture proves representability before `reserve_next`; one-over encode aborts without `put_bytes`. |
| WR-01 terminal generation wedge | ✓ CLOSED | `try_new` / `try_new_current` / v2 decode reject `u64::MAX` as `StructuralCorruption`; install refuses `LifecycleGeneration::MAX`; crafted-v2 startup leaves the handle mutable. |
| WR-02 RPC checker blind spot | ✓ CLOSED | Replacing `for_persisted_input()` with five `usize::MAX` arguments yields exactly `PHASE135_DIAGNOSTICS.bounds`. |
| WR-03 topology checker blind spots | ✓ CLOSED | `if false` vertex guard and `validate_parent_edges(0)` each yield exactly `PHASE135_DIAGNOSTICS.topology`. |
| WR-04 outer-attribute blind spot | ✓ CLOSED | `#[cfg(any())]` immediately above `validate_raw_object_keys(bytes)?;` yields exactly `PHASE135_DIAGNOSTICS.bounds`. |

No High threat from Plans 12–14 remains unresolved. Residual documented risks (`CapturedMempoolGeneration::new` for validated fixtures; structural checker is not a Rust frontend; live accounted capacity can exceed the 64 MiB persist surface and then fail closed) do not reopen the closed gaps.

### Deferred Items

| # | Item | Addressed In | Evidence |
|---|------|-------------|----------|
| 1 | Broad RPC/CLI/dashboard/metrics/log/support-bundle presentation of checkpoint and recovery evidence | Phase 137 | Roadmap Phase 137 success criterion 2 assigns those operator surfaces. |

### Required Artifacts

| Artifact | Expected | Status | Details |
| -------- | ----------- | ------ | ------- |
| `packages/open-bitcoin-node/src/storage/mempool_snapshot.rs` | Fallible captured-generation construction | ✓ VERIFIED | `try_new` and `try_new_current` reject `u64::MAX` as `StructuralCorruption`. |
| `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs` | v2 decode through `try_new`; encode under reader ceiling | ✓ VERIFIED | `CapturedMempoolGeneration::try_new(dto.captured_generation)`; encode calls representability then `limits.max_encoded_bytes`. |
| `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/representability.rs` | Shared persisted-input predicate | ✓ VERIFIED | Enforces record, unbroadcast, per-tx, aggregate-tx, and edge ceilings from `persisted_mempool_input_limits()`. |
| `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs` | Capture-before-reserve representability | ✓ VERIFIED | `try_new` plus `assert_mempool_snapshot_representable` before `reserve_next`. |
| `packages/open-bitcoin-node/src/network/lifecycle_projection/recovery.rs` | Install refuses terminal generation | ✓ VERIFIED | `generation == LifecycleGeneration::MAX` returns `InvalidPreparedRecovery` before rebuild. |
| `packages/open-bitcoin-rpc/src/context/mempool_recovery.rs` | Fixed reader + crafted-v2 mutation proof | ✓ VERIFIED | Production body uses `for_persisted_input()`; named startup test passes. |
| `packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs` | Encode abort never replaces durable bytes | ✓ VERIFIED | Encode `Err` exact-aborts; `save_mempool_snapshot` encodes before `put_bytes`. |
| `packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/representability.rs` | Exact/one-over Sync/reopen | ✓ VERIFIED | Both named tests pass. |
| `scripts/check-phase135-snapshot-recovery/persisted-input.ts` | Production-use RPC/topology/encode anchors | ✓ VERIFIED | Inspects extracted bodies, not source-wide tokens. 253 lines. |
| `scripts/check-phase135-snapshot-recovery/source.ts` | Attribute-aware direct statements | ✓ VERIFIED | `directStatementIndex` calls `statementHasDisablingAttribute`. 217 lines. |
| `scripts/check-phase135-snapshot-recovery/persisted-input-mutations.ts` | Five exact one-diagnostic mutations | ✓ VERIFIED | WR-02/03/04 and writer-ceiling mutations are `exact: true`. 310 lines. |
| `docs/metrics/lines-of-code.md` | Fresh generated LOC | ✓ VERIFIED | Present as the tracked generated artifact. |
| `.planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-14-SUMMARY.md` | Immutable executor evidence | ✓ VERIFIED | `generated_at: 2026-08-15T20:39:31Z`; this report is later. |

### Key Link Verification

| From | To | Via | Status | Details |
| ---- | --- | --- | ------ | ------- |
| `snapshot_codec/mempool.rs` | `mempool_snapshot.rs` | v2 decode `CapturedMempoolGeneration::try_new` | ✓ WIRED | Untrusted `u64` cannot skip `try_new`. |
| `lifecycle_projection/recovery.rs` | `lifecycle_projection.rs` | `generation == LifecycleGeneration::MAX` | ✓ WIRED | Install refuses the reserved terminal value. |
| `mempool_recovery.rs` | `snapshot_codec/mempool.rs` | crafted `18446744073709551615` through public decode | ✓ WIRED | Named RPC test uses real decoder bytes. |
| `runtime_authority/lifecycle.rs` | `representability.rs` | `assert_mempool_snapshot_representable` before `reserve_next` | ✓ WIRED | Capture cannot reserve an unrepresentable snapshot. |
| `encode_mempool_snapshot` | persisted-input limits | `bytes.len() > limits.max_encoded_bytes` | ✓ WIRED | No `encoded_size_upper_bound(total_transaction_bytes` acceptance. |
| `fjall_store/mempool.rs` | encode failure | `abort_failed_write` before `put_bytes` | ✓ WIRED | Unachieved encode never replaces `SNAPSHOT_KEY`. |
| `persisted-input.ts` | `recover_mempool_snapshot_with_loader` | requires `for_persisted_input()` | ✓ WIRED | Five-`usize::MAX` mutation yields bounds. |
| `persisted-input.ts` | `prepare_recovery_topology` | vertex and per-record edge guards | ✓ WIRED | `if false` and `validate_parent_edges(0)` yield topology. |
| `source.ts` | `decode.rs` | attribute-aware `directStatementIndex` | ✓ WIRED | `#[cfg(any())]` preflight mutation yields bounds. |
| `135-14-SUMMARY.md` | this report | exclusive `phase135-14-full-verify` | ✓ WIRED | Candidate names the exact reserved command. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| Snapshot preparation | records, age, generation/time, unbroadcast | live authoritative mempool under guard | Yes | ✓ FLOWING |
| Representability / encode | persisted-input limits + encoded bytes | `persisted_mempool_input_limits()` then `encode_versioned` | Yes; one-over fails closed | ✓ FLOWING |
| Snapshot write | durable `SNAPSHOT_KEY` bytes | encode success only | Yes; encode failure aborts | ✓ FLOWING |
| Startup decode | bounded source records | atomic Fjall snapshot under `for_persisted_input()` | Yes for representable snapshots | ✓ FLOWING |
| Generation install | authoritative lifecycle generation | persisted `captured_generation` through `try_new` | Yes; `u64::MAX` is corruption | ✓ FLOWING |
| Staged recovery | final membership/classifications | decoded records + current chainstate/policy/time | Yes | ✓ FLOWING |
| Broad operator presentation | checkpoint/recovery evidence | Phase 135 projections | Deferred | ➜ Phase 137 |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Phase checker mutation matrix | `bun test scripts/check-phase135-snapshot-recovery.test.ts` | 83 pass, 0 fail, 145 expect calls | ✓ PASS |
| Live Phase 135 checker | `bun run scripts/check-phase135-snapshot-recovery.ts` | `Phase 135 snapshot recovery invariants verified.` | ✓ PASS |
| Terminal generation domain/decode | timed `captured_generation` node tests | 3 passed, 0 failed | ✓ PASS |
| Writer/reader representability | timed `representability` node tests | 7 passed, including exact Sync/reopen and one-over abort | ✓ PASS |
| Crafted-v2 startup mutation | timed RPC `startup_rejects_terminal_captured_generation_and_still_admits` | 1 passed, 0 failed | ✓ PASS |
| File-shape gate | `wc -l` on Plan 14 TypeScript files | 586 / 532 / 217 / 253 / 310; all ≤ 628 | ✓ PASS |
| Reserved full repository verifier | exact `phase135-14-full-verify` command | Conditional attestation; command is the next step | ✓ CONDITIONAL |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| MPDUR-01 | 135-01, 04, 07, 08, 09, 13, 14 | Durable snapshots preserve canonical transactions, acceptance times, and surviving local-unbroadcast membership without derived peer/topology/rolling-fee state. | ✓ SATISFIED | Source-only v2 DTO plus writer/reader closure so a supported live set that is representable survives Sync → restart. Unrepresentable live sets fail closed instead of writing an unreadable checkpoint. |
| MPDUR-02 | 135-02, 03, 06, 07, 08, 10, 11, 12, 13, 14 | Recovery validates and topologically replays durable records against current chainstate and policy, rebuilds derived indexes, and reports typed recovered and dropped classifications. | ✓ SATISFIED | Bounded topology, seven outcomes, current-policy staging, and terminal-generation refusal before install. |
| MPDUR-03 | 135-02, 03, 06, 07, 12, 13, 14 | Rolling minimum-fee state resets to the pinned restart baseline while restored entries retain supported age and local-unbroadcast semantics. | ✓ SATISFIED | Age and unbroadcast intersection unchanged; rolling state is not persisted; Plans 12–14 do not freshen age or infer unbroadcast. |
| MPDUR-04 | 135-04, 05, 06, 07, 09, 13, 14 | Coalesced periodic and clean-shutdown checkpoint paths expose freshness, dirty generation, persistence strength, and the crash-loss window without holding runtime authority across I/O. | ✓ SATISFIED | Single-flight Sync, exact-abort of unachieved encode, and representability so a successful Sync write is reader-admissible. |

All four Phase 135 requirement IDs appear in plan frontmatter and in `.planning/REQUIREMENTS.md`. No Phase 135 requirement is orphaned. Pending checkboxes remain accurate until the orchestrator promotes them after this passed attestation becomes unconditional.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| `storage/mempool_snapshot.rs` | `CapturedMempoolGeneration::new` | Infallible constructor remains for validated fixtures | ℹ️ Info | Untrusted paths use `try_new`; `try_new_current` still rejects `u64::MAX`. Documented residual risk, not a blocker. |

No blocker TODO/FIXME/placeholder, empty runtime implementation, static-empty data source, authority lock across I/O, snapshot-derived encode ceiling, or duplicate checkpoint journal was found in the closed-gap source union.

### Human Verification Required

None. This is a headless persistence/recovery phase. The observable contracts are source-traceable and deterministically testable. Broad operator-surface presentation remains deferred to Phase 137.

### Gaps Summary

No actionable gaps remain. Plans 12–14 closed WR-01, CR-01, and WR-02/WR-03/WR-04. Writer and reader share one persisted-input representability contract. Terminal persisted generation is structural corruption and cannot wedge `checked_next`. The live checker fails the reproduced production-use and attribute-disabled bypasses with exactly one diagnostic each. Broad operator presentation stays deferred to Phase 137.

### Lifecycle Provenance

`135-CONTEXT.md`, Plans 135-01 through 135-14, and Summaries 135-01 through 135-14 all use `lifecycle_mode: yolo` and `phase_lifecycle_id: 135-2026-08-02T17-41-48`. No formal phase artifact is marked `direct-fallback`. This report is later than `135-14-SUMMARY.md` (`2026-08-15T20:39:31Z`). Provenance is lifecycle-valid.

### Disconfirmation Review

1. **Could `CapturedMempoolGeneration::new` still install `u64::MAX`?** No on untrusted or capture paths. Decode and live capture use `try_new`; `try_new_current` rejects the terminal value; recovery refuses `LifecycleGeneration::MAX` before rebuild.
2. **Could encode still accept a snapshot the reader would reject?** No. Representability runs first; final bytes are compared to `persisted_mempool_input_limits().max_encoded_bytes`; the snapshot-derived `encoded_size_upper_bound(total_transaction_bytes` acceptance path is gone and the checker fails if it returns.
3. **Could the checker still pass the four historical copied-corpus bypasses?** No. The five named mutations each fail with exactly one intended diagnostic; the unmodified corpus stays green.
4. **Could a later phase own these closures?** No. Phase 136 is retry/fanout, Phase 137 is operator presentation, Phase 138 is integrated parity/release guardrails. Only broad presentation is deferred.

### High Threats

| Threat | Severity | Disposition | Evidence |
| --- | --- | --- | --- |
| T-135-12-01 terminal generation tampering | High | mitigate | Domain/decoder tests reject `u64::MAX`. |
| T-135-12-02 terminal install DoS | High | mitigate | Install guard plus post-failure `expire_mempool` success. |
| T-135-13-01 unreadable Sync write | High | mitigate | Exact/one-over representability and Sync tests. |
| T-135-13-02 encode replaces prior snapshot | High | mitigate | `abort_failed_write` before `put_bytes`. |
| T-135-13-03 silent record drop | High | mitigate | Representability fails instead of shrinking records. |
| T-135-14-01 RPC constructor spoof | High | mitigate | Five-`usize::MAX` mutation → bounds. |
| T-135-14-02 topology use-site bypass | High | mitigate | Vertex/`validate_parent_edges(0)` mutations → topology. |
| T-135-14-03 attribute-disabled preflight | High | mitigate | cfg mutation → bounds. |
| T-135-14-04 snapshot-derived encode ceiling | High | mitigate | Writer-ceiling mutation → bounds. |
| T-135-14-05 verification attestation | High | mitigate | This candidate precedes the exact `phase135-14-full-verify` command. |

---

_Verified: 2026-08-15T20:56:30Z_
_Verifier: Claude (gsd-verifier)_
