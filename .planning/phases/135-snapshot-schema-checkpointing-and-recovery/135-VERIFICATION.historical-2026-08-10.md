---
phase: 135-snapshot-schema-checkpointing-and-recovery
verified: 2026-08-10T03:49:56Z
status: gaps_found
score: 15/19 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 135-2026-08-02T17-41-48
generated_at: 2026-08-10T03:49:56Z
lifecycle_validated: true
overrides_applied: 0
command_protocol:
  command: "bun run scripts/command-timings.ts run --key phase135-11-full-verify -- bash scripts/verify.sh"
  ran: false
  result: "not_run_source_truth_failed"
  reason: "The transactional Plan 135-11 gate forbids the reserved full verifier after any source-truth gap; writer/reader closure and persisted-generation safety both failed before the command boundary."
warnings:
  - id: WR-02
    severity: warning
    summary: "The Phase 135 checker does not guard the RPC startup call site that selects the fixed persisted-input reader contract."
  - id: WR-03
    severity: warning
    summary: "The checker can pass when production topology vertex or per-record edge enforcement is bypassed."
  - id: WR-04
    severity: warning
    summary: "The checker treats an attribute-disabled raw-key preflight as a valid direct statement."
gaps:
  - truth: "Every snapshot accepted and durably written by the checkpoint path is admissible to the same binary's bounded startup reader."
    status: failed
    reason: "The writer derives an encoding ceiling from the snapshot's actual dimensions and never enforces the fixed persisted-input contract used by startup. A supported live mempool can therefore receive a successful Sync checkpoint that a restart rejects as resource exhaustion."
    artifacts:
      - path: "packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs"
        issue: "encode_mempool_snapshot checks bytes only against an upper bound computed from the snapshot itself, not persisted_mempool_input_limits()."
      - path: "packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs"
        issue: "Both snapshot write paths save successful encoder output, while load enforces the fixed 256 MiB / 220,096 record / 4 MiB per transaction / 64 MiB aggregate transaction-byte contract."
      - path: "packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs"
        issue: "Capture applies current live record accounting but does not prove the snapshot is representable by the persisted reader's aggregate byte contract."
    missing:
      - "Enforce one versioned representability contract before any save, including record, unbroadcast, per-transaction, checked aggregate transaction-byte, topology, and final encoded-byte limits."
      - "Preserve the prior durable snapshot and exact-abort the unachieved write capability when representability fails."
      - "Make every supported live policy state durably representable, or explicitly constrain/version/chunk the supported persistence surface so writer and reader are closed."
      - "Add exact-boundary and one-over prepare -> encode -> Sync write -> reopen regressions plus a writer-side checker mutation."
  - truth: "A structurally admissible persisted generation cannot poison the recovered authority or prevent its next lifecycle mutation."
    status: failed
    reason: "Current-v2 decoding accepts captured_generation = u64::MAX, recovery installs it directly as LifecycleGeneration::MAX, and the next non-empty lifecycle transition fails checked_next with LifecycleGenerationExhausted."
    artifacts:
      - path: "packages/open-bitcoin-node/src/storage/mempool_snapshot.rs"
        issue: "CapturedMempoolGeneration::new is infallible for every u64 and try_new_current has no reserved-terminal-value invariant."
      - path: "packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs"
        issue: "The v2 DTO accepts a raw u64 and converts it directly into CapturedMempoolGeneration."
      - path: "packages/open-bitcoin-node/src/network/lifecycle_projection/recovery.rs"
        issue: "Recovery maps the captured value directly into LifecycleGeneration and installs it without proving a successor exists."
    missing:
      - "Reject u64::MAX at the snapshot domain/decoder boundary and classify it as structural corruption before staging or installation."
      - "Add a crafted-v2 regression proving the corrupt recovery is not installed and a subsequent ordinary lifecycle mutation succeeds."
  - truth: "Deterministic Phase 135 structural checks fail when the live persisted-input, topology, or raw-key safety statements are disabled."
    status: partial
    reason: "Four copied-corpus probes each returned zero diagnostics after disabling an intended runtime guard: the RPC fixed reader contract, topology vertex enforcement, per-record edge enforcement, and raw-key preflight through cfg(any())."
    artifacts:
      - path: "scripts/check-phase135-snapshot-recovery/persisted-input.ts"
        issue: "The checker validates store-limit construction and source-wide topology tokens, but not the RPC reader constructor or exact production-body use of vertex/per-record guards."
      - path: "scripts/check-phase135-snapshot-recovery/source.ts"
        issue: "directStatementIndex models brace depth but not outer Rust attributes that can compile a direct statement out."
      - path: "scripts/check-phase135-snapshot-recovery/persisted-input-mutations.ts"
        issue: "The mutation matrix lacks all four reproduced false-negative cases."
    missing:
      - "Require the fixed persisted-input constructor in the actual RPC recovery body."
      - "Require vertex and per-record edge guards at their production use sites, not only source-wide helper tokens."
      - "Make direct-statement validation attribute-aware or reject attributes on the guarded preflight/Serde statements."
      - "Add exact one-diagnostic mutations for all reproduced bypasses."
deferred:
  - truth: "Broad RPC, CLI, dashboard, metrics, logs, and support-bundle presentation of checkpoint and recovery evidence is available to operators."
    addressed_in: "Phase 137"
    evidence: "Phase 137 success criterion 2 assigns broad checkpoint and recovery reporting to RPC and Sanitized Operator Evidence."
---

# Phase 135: Snapshot Schema, Checkpointing, and Recovery Verification Report

**Phase Goal:** Supported restarts recover valid mempool source state and local initial-broadcast intent truthfully without persisting volatile or derived policy state.
**Verified:** 2026-08-10T03:49:56Z
**Status:** gaps_found
**Re-verification:** No — fresh cycle-3 verification. The quarantined earlier report was consulted only as historical input.

## Goal Achievement

### Observable Truths

The roadmap success criteria and all eleven plan frontmatter contracts were merged and deduplicated into the phase's established 19 observable truths. Summary claims were not accepted as evidence; results below come from the live source, focused behavior, copied-corpus mutation probes, and parity/checker inspection.

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Current v2 persists canonical witness transactions, known or explicit unknown acceptance times, capture generation/time, and exact unbroadcast membership without derived state. | ✓ VERIFIED | `MempoolSnapshot` and `MempoolSnapshotV2Dto` contain only source-authority fields; identity and unbroadcast subset checks are substantive. |
| 2 | Explicit v1 migration preserves unknown age conservatively and remains checkpointable without inventing source facts. | ✓ VERIFIED | Missing legacy age maps to `LegacyUnknown`; current v2 represents it as explicit nullable age. |
| 3 | Unsupported, corrupt, oversized, malformed-key, identity-invalid, and lifecycle-invalid snapshots fail closed before install. | ✗ FAILED | Byte/count/key/identity guards exist, but v2 `captured_generation: u64::MAX` is accepted and installed as an authority generation with no successor. |
| 4 | Recovery derives deterministic bounded parent-before-child topology while treating absent external prevouts separately. | ✓ VERIFIED | Format-owned finite vertices/edges feed deterministic `BTreeSet` Kahn traversal; current-policy topology regression now passes. |
| 5 | Invalid components do not block independent valid components, and all seven outcomes are assigned from final membership. | ✓ VERIFIED | Plan 10's separation is live; the current capacity-trim regression passes and reaches `DroppedEvicted`. |
| 6 | Known/unknown entry age and exact surviving unbroadcast membership survive recovery while rolling floor and decay-gate state reset. | ✓ VERIFIED | Recovery preserves acceptance-time values, intersects unbroadcast with final membership, and rebuilds a fresh mempool baseline. |
| 7 | A prepared recovery installs exactly once through the managed authority without partial live state. | ✓ VERIFIED | Epoch, chainstate, freshness, pending-effect, membership, and derived projection checks precede the atomic aggregate replacement. |
| 8 | Canonical survivors rebuild derived projections and initialize current/last-durable generation with an empty loss interval. | ✓ VERIFIED | Serving, identity, fanout seed, peer, unbroadcast, evidence, and checkpoint state are rebuilt from the prepared survivors. |
| 9 | Recovery evidence and restored unbroadcast truth describe final survivors only and schedule no relay side effect. | ✓ VERIFIED | Final membership drives classifications and unbroadcast intersection; recovery seeds mappings without transmission. |
| 10 | Snapshot preparation captures every supported live source record and every successful Sync checkpoint is readable by the same format contract. | ✗ FAILED | Capture is live-policy bounded, but the encoder's self-derived ceiling can admit data outside the reader's fixed 64 MiB aggregate transaction-byte and 256 MiB encoded ceilings. |
| 11 | Checkpoint evidence distinguishes current, dirty, in-flight, and durable generations and reports the exact loss interval. | ✓ VERIFIED | Generation boundaries, trigger, strength, timestamps, failure/outcome, overdue state, and `(last_durable, current]` are modeled. |
| 12 | A successful Sync write yields an achieved receipt that survives completion-dispatch failure. | ✓ VERIFIED | Successful persistence consumes the capability into a retained achieved receipt; post-Sync completion remains retryable. |
| 13 | The coordinator skips clean periodic ticks, coalesces newer work, and permits only one write in flight. | ✓ VERIFIED | Coordinator state and tests enforce clean skip, one flight, retained completion, and one bounded follow-up. |
| 14 | Encode/storage failures abort only unachieved capabilities, retain dirtiness, and remain retryable. | ✓ VERIFIED | The shell exact-aborts pre-achievement failures; achieved writes are not converted back into abortable work. |
| 15 | Startup performs policy-independent bounded load followed by current-chainstate/current-policy staging before publication. | ✓ VERIFIED | RPC constructs `for_persisted_input`, Fjall size-checks before load, staging replays through the handle's current policy, and publication follows installation. |
| 16 | A private 300-second worker drives Sync checkpoints and shutdown quiesces producers before settle/force-current and clean marking. | ✓ VERIFIED | Daemon ordering and checkpoint coordinator wiring remain substantive and connected. |
| 17 | Deterministic mutation/structural checks enforce the actual Phase 135 safety boundaries. | ✗ FAILED | The live suite is green, but four independent copied-corpus guard bypasses each yield zero diagnostics (WR-02 through WR-04). |
| 18 | Parity evidence names bounded Knots anchors and strengthened durability without broad relay, propagation, or readiness claims. | ✓ VERIFIED | Catalog, checklist, index, and breadcrumbs remain `in_progress`/Pending with explicit exclusions; breadcrumb check passes for 762 Rust files. |
| 19 | The exact final default repository verifier passes on the canonical post-summary candidate tree. | ✗ BLOCKED / NOT RUN | Source truth failed first. Plan 135-11 therefore required stopping before the reserved one-shot `phase135-11-full-verify` command. |

**Score:** 15/19 truths verified

### Advisory Review Adjudication

| Finding | Independent result | Evidence |
| --- | --- | --- |
| CR-01 writer/reader closure | 🛑 CONFIRMED | Encoder acceptance is based on actual snapshot dimensions (`snapshot_codec/mempool.rs:168-187`); write paths persist it (`fjall_store/mempool.rs:255-300`); startup uses the fixed contract (`snapshot_codec/mempool.rs:30-84`, `fjall_store/mempool.rs:303-354`, RPC lines 104-111). |
| WR-01 terminal generation wedge | 🛑 CONFIRMED | `CapturedMempoolGeneration::new` accepts every `u64`; decode passes the raw value; recovery installs it; `LifecycleGeneration::checked_next` fails at `u64::MAX`. |
| WR-02 RPC checker blind spot | ⚠ CONFIRMED | Replacing the RPC fixed constructor with five `usize::MAX` arguments in a copied corpus produced `0 diagnostics []`. |
| WR-03 topology checker blind spots | ⚠ CONFIRMED | Replacing the production vertex guard with `if false` and replacing per-record validation with `validate_parent_edges(0)` independently produced `0 diagnostics []`. |
| WR-04 outer-attribute blind spot | ⚠ CONFIRMED | Adding `#[cfg(any())]` directly above the production raw-key preflight produced `0 diagnostics []`. |

CR-01 and WR-01 are source-truth goal blockers. WR-02 through WR-04 are structural-proof gaps: the live source currently retains those guards, but Plan 07/11's deterministic enforcement contract is incomplete.

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `packages/open-bitcoin-node/src/storage/mempool_snapshot.rs` | Invariant-bearing source-only snapshot domain | ⚠ PARTIAL | Schema/source invariants are substantive and wired; the captured generation admits the terminal `u64::MAX` value. |
| `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs` | Bounded v2 codec and explicit v1 migration | ✗ HOLLOW CONTRACT | Reader bounds are substantive, but encoder output is not closed under them; generation conversion also lacks the terminal-value check. |
| `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/decode.rs` | Allocation-bounded streaming decoder | ✓ VERIFIED | Raw-key preflight precedes Serde in live code; streaming record, unbroadcast, per-record, and aggregate counters are connected. |
| `packages/open-bitcoin-node/src/network/recovery/topology.rs` | Deterministic format-bounded dependency analysis | ✓ VERIFIED | Plan 10's format/current-policy separation and exact/one-over edge bounds are live. |
| `packages/open-bitcoin-node/src/network/recovery/staging.rs` | Side-effect-free current-policy staged recovery | ✓ VERIFIED | Current chainstate/policy replay, final membership, age, and unbroadcast intersection flow to a complete prepared result. |
| `packages/open-bitcoin-node/src/network/lifecycle_projection/recovery.rs` | Atomic complete recovery projection | ⚠ PARTIAL | Projection is substantive, but imports persisted `u64::MAX` directly into the authoritative generation. |
| `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs` | Guarded capture and sole install command | ⚠ PARTIAL | Capture/install authority is correct; capture does not prove persisted-format representability before reserving a write. |
| `packages/open-bitcoin-node/src/network/checkpoint.rs` | Single-flight checkpoint coordinator | ✓ VERIFIED | Clean skip, coalescing, retained receipt, and bounded follow-up are wired. |
| `packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs` | Sync write and fixed bounded read | ⚠ PARTIAL | Each side is substantive, but writer and reader accept different value sets. |
| `packages/open-bitcoin-rpc/src/context/mempool_recovery.rs` | Pre-publication bounded startup recovery | ✓ VERIFIED | Fixed read -> current-policy stage -> one install is connected, though the checker does not protect the fixed read call. |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/checkpoint.rs` | Private periodic and shutdown orchestration | ✓ VERIFIED | Periodic and final Sync paths are connected outside runtime authority. |
| `scripts/check-phase135-snapshot-recovery.ts` and child modules | Fail-closed structural/claim checker | ⚠ PARTIAL | Live checker and 75 named mutations pass; four independently reproduced safety bypasses are unguarded. |
| `docs/parity/catalog/mempool-policy.md` and companions | Bounded auditable parity evidence | ✓ VERIFIED | Exact Knots anchors, pending status, breadcrumbs, and scope exclusions remain consistent. |

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| live mempool capture | v2 encoder | guarded canonical record clone | ✓ WIRED | Current source facts flow into the owned snapshot. |
| v2 encoder / Fjall save | fixed Fjall/RPC reader | one versioned representability contract | ✗ NOT WIRED | Writer uses a snapshot-relative ceiling; reader uses fixed format limits. |
| persisted generation | recovery authority | decode -> prepare -> install | ✗ UNSAFE | The raw terminal value reaches `LifecycleGeneration::MAX` with no successor proof. |
| persisted records | topology -> current-policy staging | format bounds before current replay | ✓ WIRED | Plan 10's repaired two-boundary path is live and focused regression-tested. |
| prepared recovery | authoritative aggregate | consumed sole lifecycle command | ✓ WIRED | All replacement projections are prepared before mutation and installed together. |
| checkpoint capability | Fjall Sync | encode/save outside the authority guard | ✓ WIRED | I/O stays outside runtime authority, but successful output may be unreadable. |
| achieved receipt | checkpoint evidence | retain and retry completion | ✓ WIRED | Durable acknowledgement cannot be converted into an abort. |
| structural checker | RPC/topology/raw-key production statements | exact mutation guards | ⚠ PARTIAL | Four copied-corpus bypasses evade diagnostics. |
| canonical report | reserved full verifier | Plan 135-11 transaction | ⊘ NOT RUN | Transaction stopped at the required source-gap escalation gate. |

The generic plan key-link helper reported several multiline-pattern false negatives; the table above comes from direct call-chain inspection rather than those textual misses.

### Data-Flow Trace (Level 4)

| Artifact | Data variable | Source | Produces real data | Status |
| --- | --- | --- | --- | --- |
| Snapshot preparation | records, age, generation/time, unbroadcast | live authoritative mempool under guard | Yes | ✓ FLOWING |
| Snapshot write | encoded checkpoint bytes | prepared snapshot | Yes, but not always reader-admissible | ✗ CONTRACT BREAK |
| Startup decode | bounded source records | atomic Fjall snapshot under fixed limits | Yes for reader-admissible snapshots | ✓ FLOWING |
| Generation install | authoritative lifecycle generation | persisted `captured_generation` | Yes, including poisonous terminal value | ✗ UNSAFE FLOW |
| Staged recovery | final membership/classifications | decoded records + current chainstate/policy/time | Yes | ✓ FLOWING |
| Checkpoint evidence | generation, strength, outcome, loss interval | authority and write receipt transitions | Yes, but can label an unreadable write durable | ⚠ MISLEADING EDGE |
| Broad operator presentation | checkpoint/recovery evidence | Phase 135 projections | Deferred | ➜ Phase 137 |

### Behavioral Spot-Checks

| Behavior | Command / probe | Result | Status |
| --- | --- | --- | --- |
| Phase checker mutation matrix | `bun test scripts/check-phase135-snapshot-recovery.test.ts` | 75 passed, 0 failed, 132 assertions in 3.39s | ✓ PASS |
| Live Phase 135 checker | `bun run scripts/check-phase135-snapshot-recovery.ts` | Invariants reported verified | ✓ PASS, incomplete coverage |
| Persisted-input exact/one-over contract | timed node `persisted_input_limits` filter | 8 passed, 0 failed | ✓ PASS |
| Current-policy capacity trim reaches final classification | timed `staged_recovery_classifies_later_capacity_trim_from_final_membership` | 1 passed, 0 failed | ✓ PASS |
| Former fixed 50,000 record boundary | timed `former_50_000_record_limit_checkpoints_and_reopens_with_persisted_input_bounds` | 1 passed with 50,001 records in 5.85s | ✓ PASS; below CR-01 byte boundary |
| Parity breadcrumbs | `bun run scripts/check-parity-breadcrumbs.ts --check` | 762 Rust files verified | ✓ PASS |
| Active milestone traceability | `bun run scripts/check-active-milestone-verification-traceability.ts` | Exit 1: correctly rejects non-passed Phase 135 coverage for MPDUR-01, MPDUR-02, and MPDUR-04 | ✓ EXPECTED FAIL-CLOSED |
| RPC fixed-reader mutation | copied corpus, replace constructor with five `usize::MAX` arguments | 0 diagnostics | ✗ FAIL (WR-02) |
| Topology enforcement mutations | copied corpora, disable vertex and per-record guards | 0 diagnostics for each | ✗ FAIL (WR-03) |
| Attribute-disabled preflight mutation | copied corpus, add `#[cfg(any())]` before preflight | 0 diagnostics | ✗ FAIL (WR-04) |
| Reserved full repository verifier | exact `phase135-11-full-verify` command | Not run because source truth failed | ⊘ BLOCKED BY PROTOCOL |

No state-mutating runtime service was started. The two runtime gaps are conclusive static call-chain failures; no existing regression exercises either exact boundary.

### Requirements Coverage

| Requirement | Source plans | Status | Evidence |
| --- | --- | --- | --- |
| MPDUR-01 | 135-01 through 135-09 as declared | ✗ BLOCKED | The schema preserves the right fields, but a supported live source set is not guaranteed to survive a writer -> restart-reader round trip. |
| MPDUR-02 | 135-01 through 135-11 as declared | ✗ BLOCKED | Normal bounded/current-policy replay works, but a self-written over-contract snapshot never reaches replay and terminal generation can poison the installed authority. |
| MPDUR-03 | 135-02, 03, 06, 07, 09 | ✓ SATISFIED | Known/unknown age and survivor-local intent are retained while rolling state rebuilds at the pinned baseline. |
| MPDUR-04 | 135-03 through 135-09 as declared | ✗ BLOCKED | I/O separation, coalescing, strength, and loss evidence are implemented, but a successful unreadable Sync write can be reported as durable/current. |

All four Phase 135 requirement IDs appear in plan frontmatter and the requirements traceability table. No Phase 135 requirement is orphaned. Pending checkboxes remain accurate for this non-passing verification.

### Anti-Patterns Found

| File | Line / area | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| `storage/snapshot_codec/mempool.rs` | 168-187 | Writer validates against a bound derived from its own input instead of the reader contract | 🛑 Blocker | Successful durable output can be unreadable by the same binary. |
| `storage/mempool_snapshot.rs` | 47-54, 143-180 | Infallible persisted-generation constructor admits reserved terminal state | 🛑 Blocker | Crafted local snapshot can permanently wedge later mempool mutations. |
| `check-phase135-snapshot-recovery/persisted-input.ts` | persisted decode/topology checks | Source-wide token presence substitutes for exact production-use checks | ⚠ Warning | Safety guards can be bypassed without failing the checker. |
| `check-phase135-snapshot-recovery/source.ts` | `directStatementIndex` | Brace-only structure ignores compile-time attributes | ⚠ Warning | An attribute-disabled preflight is accepted as live proof. |

No blocker TODO/FIXME/placeholder, empty runtime implementation, static-empty data source, console-only handler, derived peer/topology/rolling serialization, authority lock across I/O, or duplicate checkpoint journal was found in the Phase 135 source union. Test fixtures and empty accumulators were inspected contextually and are not stubs.

### Human Verification Required

None. This is a headless persistence/recovery phase, and the observable contracts are source-traceable and deterministically testable. Broad operator-surface presentation is explicitly deferred.

### Deferred Items

| # | Item | Addressed In | Evidence |
| --- | --- | --- | --- |
| 1 | Broad RPC/CLI/dashboard/metrics/log/support-bundle presentation of checkpoint and recovery evidence | Phase 137 | Roadmap Phase 137 success criterion 2 assigns those operator surfaces. |

CR-01, WR-01, and checker enforcement are not deferred: no later phase names writer/reader closure, persisted-generation validity, or Phase 135 checker repair with enough specificity to move them out of this report.

### Lifecycle Provenance

`135-CONTEXT.md`, Plans 135-01 through 135-11, and Summaries 135-01 through 135-11 all use `lifecycle_mode: yolo` and `phase_lifecycle_id: 135-2026-08-02T17-41-48`. This report is newer than `135-11-SUMMARY.md` (`2026-08-10T03:27:36Z`). No formal phase artifact is marked `direct-fallback`; provenance is lifecycle-valid. The pre-existing dirty planning files and untracked Plans 08-11 were read but not edited.

### Disconfirmation Review

1. **Could the writer already be bounded indirectly by live policy?** No. Capture checks only record count from current accounted capacity; default live capacity is 300,000,000 accounted bytes, while the persisted reader accepts only 67,108,864 aggregate canonical transaction bytes. The encoder never imports the persisted limits.
2. **Could Fjall reject the mismatch before replacing durable data?** No. Both public save and prepared execution pass successful encoder bytes directly to `put_bytes`; the fixed contract first appears on load.
3. **Could `u64::MAX` be rejected as structural corruption elsewhere?** No. The DTO type is `u64`, conversion uses the infallible constructor, snapshot construction has no max guard, and recovery directly calls `LifecycleGeneration::from_raw`.
4. **Could the checker findings be theoretical only?** No. Four isolated copied-corpus mutations were evaluated by the live exported checker; all returned zero diagnostics.
5. **Could a later roadmap phase own these gaps?** No specific later goal or success criterion covers persistence representability, generation-domain validation, or checker repair. Only broad presentation is explicitly Phase 137 work.

### Gaps Summary

Plans 10 and 11 close the earlier current-policy/input-bound regression: finite persisted candidates now reach current-policy replay, capacity trimming is typed, and focused tests are green. The phase still does not achieve durable restart recovery end to end. The writer can acknowledge and replace a Sync checkpoint that the same binary's reader rejects, and a crafted supported-v2 generation can install a terminal authority state that rejects every later non-empty lifecycle transition. The structural checker also misses four concrete bypass classes. Because source truth failed before the reserved command boundary, the exact full verifier was correctly not run.

***

_Verified: 2026-08-10T03:49:56Z_
_Verifier: the agent (gsd-verifier)_
