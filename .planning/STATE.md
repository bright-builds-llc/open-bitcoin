---
gsd_state_version: 1.0
milestone: v2.3
milestone_name: Chainstate Durability and Historical Serving
status: executing
stopped_at: Completed 143-01-PLAN.md
last_updated: "2026-09-17T03:54:56.129Z"
last_activity: 2026-09-17
progress:
  total_phases: 7
  completed_phases: 4
  total_plans: 21
  completed_plans: 18
  percent: 86
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-08-29 after starting milestone v2.3).

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Phase 143 — honest-stored-block-availability

## Current Position

Milestone: v2.3 Chainstate Durability and Historical Serving
Phase: 143 (honest-stored-block-availability) — EXECUTING
Plan: 2 of 4
Status: Ready to execute
Last activity: 2026-09-17

The milestone replaces snapshot-style coin persistence with disk-backed coins, cache-flush policy, fuller chainstate-manager behavior, and honest stored-block availability. Prune/archive product modes, assumeutxo, compact-filter serving, public defaults, and production claims remain deferred. Historical `.planning/phases/` directories stay tracked.

Progress: [░░░░░░░░░░] 0%

Next action: `/gsd-plan-phase 139`

## Performance Metrics

**Current milestone:** v2.3 has no completed plans yet.

**Previous milestone (v2.2 archive):**

- Total plans completed: 112
- Average duration: 42 min
- Total execution time: 9h 4m

| Phase | Plans | Total | Avg/Plan |
| --- | ---: | ---: | ---: |
| 130–138 | 13 | 9h 4m | 42 min |
| 130 | 13 | - | - |
| 131 | 5 | - | - |
| 132 | 8 | - | - |
| 133 | 4 | - | - |
| 133.1 | 6 | - | - |
| 134 | 24 | - | - |
| 135 | 14 | - | - |
| 136 | 6 | - | - |
| 137 | 11 | - | - |
| 138 | 4 | - | - |
| 139 | 4 | - | - |
| 140 | 3 | - | - |
| 141 | 4 | - | - |
| 142 | 6 | - | - |

### Plan Execution History

| Plan | Duration | Tasks | Files |
| --- | --- | --- | --- |
| Phase 133 P01 | 1h 13m | 3 tasks | 16 files |
| Phase 133 P02 | 1h 10m | 3 tasks | 28 files |
| Phase 133 P03 | 1h 4m | 3 tasks | 20 files |
| Phase 133 P04 | 38min | 3 tasks | 14 files |
| Phase 133.1 P01 | 10min | 2 tasks | 48 files |
| Phase 133.1 P02 | 13min | 3 tasks | 62 files |
| Phase 133.1 P03 | 51min | 3 tasks | 263 files |
| Phase 133.1 P04 | 27min | 3 tasks | 137 files |
| Phase 133.1 P05 | 21min | 3 tasks | 21 files |
| Phase 133.1 P06 | 53m | 2 tasks | 63 files |
| Phase 134 P01 | 1h 10m | 3 tasks | 13 files |
| Phase 134 P02 | 32min | 2 tasks | 5 files |
| Phase 134 P03 | 71min | 2 tasks | 11 files |
| Phase 134 P04 | 58min | 2 tasks | 12 files |
| Phase 134 P05 | 1h 41m | 2 tasks | 25 files |
| Phase 134 P06 | 1h 7m | 2 tasks | 19 files |
| Phase 134-authoritative-cross-cache-lifecycle-integration P07 | 1h 5m | 3 tasks | 10 files |
| Phase 134 P08 | 1h 17m | 2 tasks | 13 files |
| Phase 134 P09 | 1h 17m | 3 tasks | 31 files |
| Phase 134 P10 | 1h 10m | 2 tasks | 4 files |
| Phase 134 P11 | 94m | 3 tasks | 12 files |
| Phase 134 P12 | 1h 29m | 2 tasks | 5 files |
| Phase 134 P13 | 41m | 2 tasks | 5 files |
| Phase 134 P14 | 58m | 2 tasks | 9 files |
| Phase 134 P15 | 2h 7m | 2 tasks | 22 files |
| Phase 134 P16 | 2h 51m | 2 tasks | 23 files |
| Phase 134 P17 | 116m | 2 tasks | 12 files |
| Phase 134 P18 | 1h 52m | 2 tasks | 12 files |
| Phase 134 P19 | 125m | 2 tasks | 5 files |
| Phase 134 P20 | 2h 35m | 2 tasks | 7 files |
| Phase 134 P21 | 2h 39m | 2 tasks | 9 files |
| Phase 134 P22 | 27m | 1 tasks | 4 files |
| Phase 134 P23 | 69m | 1 tasks | 8 files |
| Phase 134 P24 | 1h 4m | 2 tasks | 5 files |
| Phase 135 P12 | 33min | 2 tasks | 16 files |
| Phase 135 P13 | 34 | 2 tasks | 11 files |
| Phase 135 P14 | 23 | 3 tasks | 7 files |
| Phase 136-receive-independent-maintenance-and-transport-receipts P01 | 33 | 2 tasks | 5 files |
| Phase 136 P02 | 21 | 2 tasks | 7 files |
| Phase 136 P03 | 71 | 2 tasks | 9 files |
| Phase 136 P04 | 55 | 2 tasks | 22 files |
| Phase 136-receive-independent-maintenance-and-transport-receipts P05 | 74 | 2 tasks | 10 files |
| Phase 136 P06 | 72min | 2 tasks | 21 files |
| Phase 138-parity-adversarial-pressure-restart-and-release-guardrails P01 | 32 min | 2 tasks | 8 files |
| Phase 138 P02 | 58 | 2 tasks | 7 files |
| Phase 138-parity-adversarial-pressure-restart-and-release-guardrails P03 | 86min | 2 tasks | 27 files |
| Phase 138-parity-adversarial-pressure-restart-and-release-guardrails P04 | 63 | 2 tasks | 21 files |
| Phase 140 P01 | 133 | 2 tasks | 6 files |
| Phase 140 P02 | 79 | 2 tasks | 3 files |
| Phase 140 P03 | 28 | 2 tasks | 5 files |
| Phase 141 P01 | 42 | 2 tasks | 13 files |
| Phase 141 P02 | 27 | 2 tasks | 5 files |
| Phase 141 P03 | 37 | 2 tasks | 6 files |
| Phase 141-durable-fjall-coins-adapter P04 | 118min | 2 tasks | 25 files |
| Phase 142-manager-flush-lifecycle-and-restart P01 | 21 min | 2 tasks | 6 files |
| Phase 142-manager-flush-lifecycle-and-restart P02 | 23 min | 2 tasks | 9 files |
| Phase 142 P03 | 53 | 2 tasks | 8 files |
| Phase 142-manager-flush-lifecycle-and-restart P04 | 43 | 2 tasks | 6 files |
| Phase 142 P05 | 60min | 2 tasks | 16 files |
| Phase 142 P06 | 120min | 2 tasks | 104 files |
| Phase 143-honest-stored-block-availability P01 | 23 | 2 tasks | 9 files |

## Accumulated Context

### Roadmap Evolution

- Phase 133.1 inserted after Phase 133: Bright Builds Verification Baseline Cleanup (URGENT)
- v2.3 roadmap continues numbering after Phase 138; Phases 139–145 own the 15 v2.3 requirements.

### Decisions

- [v2.3 roadmap]: Continue phase numbering at 139. Do not reset to Phase 1. Do not delete or archive historical `.planning/phases/` directories.
- [v2.3 roadmap]: Seven fine-granularity phases follow research order: typed coins-view/cache and engine apply, pure flush policy, Fjall coins adapter, manager lifecycle/restart, honest availability, operator evidence, then parity/no-claim guardrails.
- [v2.3 roadmap]: Merge research "view/cache" and "engine apply" into Phase 139 because CACHE-01 is the only requirement those two conceptual slices share.
- [v2.3 roadmap]: Assign CSOBS-03 to Phase 141 so coins disk-read errors fail closed at the first durable-read seam, not later as operator copy.
- [v2.3 roadmap]: Honest availability (Phase 143) comes after coins truth exists and before operator evidence, so docs cannot re-document a serving lie.
- [v2.3 roadmap]: Storage-first locked scope: no prune/archive product modes, no assumeutxo/assumevalid/IBD shortcuts, no LevelDB or rust-bitcoin, functional core stays I/O-free.
- [v2.3 milestone]: Initialized the new milestone through `/gsd-new-milestone` after the archived v2.2 closeout.
- [v2.3 milestone]: Storage-first scope: disk-backed coins, cache-flush, and chainstate-manager now; prune/archive product modes later.
- [v2.3 milestone]: Honest availability means serve or report a stored block only when the payload is present; refuse cleanly when it is not.
- [v2.3 milestone]: Keep assumeutxo, assumevalid, and IBD snapshot shortcuts out.
- [v2.2 milestone]: Initialized the new milestone through `/gsd-new-milestone` after the archived v2.1 closeout.
- [v2.2 roadmap]: Use the research-backed nine-phase dependency order across Phases 130–138 at fine granularity.
- [v2.2 roadmap]: Assign PPKG-04 to Phase 136, where parent-before-child package fanout becomes an achieved transport behavior after the peer bridge and lifecycle authority exist.
- [v2.2 roadmap]: Keep package handling to local package APIs and bounded same-peer 1P1C assembly over ordinary transaction messages; add no general package wire protocol.
- [v2.2 roadmap]: Persist canonical entries, acceptance times, and surviving local unbroadcast membership, but rebuild derived state and reset the rolling fee on restart.
- [v2.2 roadmap]: Keep default verification deterministic and hermetic; public/default/production relay and guaranteed-propagation claims remain deferred.
- [Phase 130]: Use deterministic Rust-owned logical mempool accounting rather than C++ allocator estimates.
- [Phase 130]: Keep Phase 130 trimming exclusively on legacy vsize while reporting distinct accounted usage and capacity.
- [Phase 130]: Map resource arithmetic failures to MempoolError::InternalInvariant at mutation boundaries.
- [Phase 130]: Keep FeeRate role-neutral for wallet arithmetic while requiring semantic wrappers at mempool policy boundaries.
- [Phase 130]: Initialize the rolling floor to zero and derive effective admission from static and rolling values at decision and summary boundaries.
- [Phase 130]: Keep package member-static and eligible aggregate-rolling obligations independent without a generic exception switch.
- [Phase 130]: Classify missing legacy metadata only as LegacyUnknown, RecoveryUnknown, and NotRequested; never infer local origin or current time.
- [Phase 130]: Require local origin, requested relay intent, and current authoritative membership together for retry eligibility.
- [Phase 130]: No-time local outcome adapters are removed; wallet AdmissionResult no-time path remains deprecated separately.
- [Phase 130]: Keep MempoolOutcome as attempt vocabulary and MempoolLifecycleDelta as committed fact vocabulary.
- [Phase 130]: Resolve retry clears with LifecycleRemoval > TransportWritten > EligibleServe precedence.
- [Phase 130]: Keep removal cause independent from direct-versus-descendant role.
- [Phase 130]: Peer admission uses exact receive or reconsideration time with Peer and NotRequested metadata.
- [Phase 130]: Local RPC admission now samples checked shell time with Local origin and activation-resolved relay intent.
- [Phase 130]: Bridge-owned admission cache effects consume lifecycle delta cause, role, identities, and final membership.
- [Phase 130]: Model only the injected variable retry delay in Phase 130; Phase 136 owns scheduling, fanout, receipts, and clearing.
- [Phase 130]: Require fallible 0-to-300-second jitter construction before creating a retry decision context.
- [Phase 130]: Use requested relay intent only for local relay and serving fixtures; non-relay admission setup remains explicitly not requested.
- [Phase 130]: Deterministic fixture time remains authoritative in tests; live RPC clock sampling is owned by Plan 130-11 and is complete.
- [Phase 130]: Use stored-block receive time and connected height while direct local blocks use explicit header time and connected height.
- [Phase 130]: Use one explicit reorg operation time for replacement-block cleanup and disconnected transaction reacceptance.
- [Phase 130]: Apply every reorg admission attempt through its semantic lifecycle delta without expanding Phase 134 cross-cache scope.
- [Phase 130]: Keep SchemaVersion::CURRENT unchanged and encode metadata as three optional mempool-record fields.
- [Phase 130]: All-absent decodes to LegacyUnknown, RecoveryUnknown, and NotRequested; any partial set is StorageError::Corruption in Mempool.
- [Phase 130]: Known capture and recovery pass metadata through AdmissionContext::recovery without substituting restart time or local origin.
- [Phase 130]: Sample SystemTime only in sendrawtransaction with checked conversion; never unwrap_or(0).
- [Phase 130]: Resolve RelayIntent::Requested from relay activation enabled; otherwise NotRequested.
- [Phase 130]: Migrate the final RPC caller and delete both no-time outcome adapters in one commit.
- [Phase 130]: Keep getmempoolinfo.bytes=vsize, usage=accounted memory, maxmempool=accounted capacity, and mempoolminfee=effective max(static, rolling).
- [Phase 130]: Serialize capacityenforcement as fixed legacy_vsize during Phase 130 without claiming accounted-capacity enforcement.
- [Phase 130]: Expose rollingmempoolfee, effectiveadmissionfee, and incrementalrelayfee as distinct exact fields so incremental never contaminates mempoolminfee.
- [Phase 130]: Register unique FEEP-01 through FEEP-05 ownership under v2-2-resource-time-fee-primitives with exact later-phase boundaries.
- [Phase 130]: Document intentional Rust-owned accounting difference from C++ allocator estimates while preserving Knots RPC meanings.
- [Phase 130]: Align documentation reconciliation with active v2.2 README truth instead of requiring /gsd-new-milestone in the root status block.
- [Phase 130]: Reuse the Phase 129 string[] failure-list contract with no separate result alias.
- [Phase 130]: Validate README freshness through three independent readTarget calls and dedicated stale-wording failures.
- [Phase 130]: Keep FEEP requirements Pending until Phase 130 VERIFICATION.md exists for milestone traceability.
- [Phase 133]: Reject evidence accepts only Wtxid or typed package fingerprints; txid-only inventory requires an authoritative txid-to-wtxid mapping.
- [Phase 133]: Ordinary inventory consults hard and reconsiderable evidence, while orphan-parent requests bypass reconsiderable evidence and still honor hard rejects.
- [Phase 133]: Both reject evidence domains reset together immediately after successful authoritative chainstate connect or reorg mutation.
- [Phase 133]: Production tweak entropy is derived in the node shell with RandomState while network constructors retain fixed-tweak deterministic seams.
- [Phase 133]: Capture receipt provenance before request cleanup, deterministically unioning bounded txid/wtxid announcers while retaining the delivering peer.
- [Phase 133]: Retain one orphan body with a policy-bounded announcer set; late inventory changes ownership evidence only and never replaces the body or refreshes TTL.
- [Phase 133]: Use an opaque consume-only same-peer 1P1C candidate as the proof of provenance and bounded eligibility.
- [Phase 133]: Co-locate scheduler, orphanage, reject evidence, and disconnect mutation under PeerManager.
- [Phase 133]: Classify peer singletons through typed package reports and preserve ordinary RBF only for the exact typed one-member package-replacement shape.
- [Phase 133]: Apply only bounded orphan and reject-evidence feedback in Phase 133; defer package lifecycle projection to Phase 134.
- [Phase 133]: Claim only bounded opportunistic same-peer 1P1C assembly over ordinary transaction messages; broader package relay surfaces remain deferred.
- [Phase 133]: Guard the exact node-owned Phase 132 handoff and exhaustive feedback boundary with a filesystem-only checker and 22 independent mutations.
- [Phase 133]: Treat probabilistic reject evidence as suppression-only, with active-tip reset and no peer punishment.
- [Phase 133.1]: Anchor all cleanup comparisons to phase-start commit 3e35678a9e3d623aad27893f9594a8ded152a722.
- [Phase 133.1]: Compare Rust and Bun test behavior with sorted multisets plus independently parsed counts.
- [Phase 133.1]: Group contiguous Rust test leaves by behavior while keeping shared fixtures in thin test roots.
- [Phase 133.1]: Preserve moved tests original super:: resolution through private test-root imports rather than rewriting test bodies.
- [Phase 133.1]: Assign every new Wave A Rust path explicitly to the parity breadcrumb group that owned its source offender.
- [Phase 133.1]: Use semantic behavior and phase families while retaining shared fixtures in thin test-only roots.
- [Phase 133.1]: Expose moved shared helpers only with pub(super), using absolute crate paths where module depth changes super resolution.
- [Phase 133.1]: Assign each new child to the parity group of its nearest original source offender.
- [Phase 133.1]: Preserve each oversized TypeScript root as the stable entrypoint and move concerns into same-named directories.
- [Phase 133.1]: Use explicit child-source maps when parity checkers follow Rust sources decomposed by earlier plans.
- [Phase 133.1]: Compare exact XML-escaped JUnit title multisets plus independent test counts for TypeScript decomposition evidence.
- [Phase 133.1]: Preserve the live-smoke TypeScript and shell roots as stable entrypoints backed by same-named concern directories.
- [Phase 133.1]: Keep extracted networking inert until parsed explicit CLI opt-in and successful local preflight.
- [Phase 133.1]: Confine all hermetic live-smoke fixtures, reports, and cleanup to one guarded temporary root.
- [Phase 133.1]: Structural checkers read the stable root plus its same-named extension-free child tree in bytewise order.
- [Phase 133.1]: Formatter repair stayed limited to Phase 133.1-owned Rust test paths.
- [Phase 134]: Keep MempoolPatch private behind one opaque non-Clone capability and separate revision validation from infallible consumption.
- [Phase 134]: Derive teardown order from canonical removed transaction inputs so descendants precede ancestors independently of removal cause or role.
- [Phase 134]: Retain existing mutating APIs as compatibility facades that prepare, validate, and consume exactly once.
- [Phase 134]: Use one exhaustive LifecycleCommand family for lifecycle mutation, effect preparation, relay preparation, and receipt completion.
- [Phase 134]: Require authority epoch, core capability, and all seven concrete projection targets in one private checked constructor.
- [Phase 134]: Keep ManagedMempool additions preparation-only and preserve the existing Arc<Mutex<AuthoritativeNetwork>> runtime authority without routing callers.
- [Phase 134]: Prepare exact peer-local operations before mutation and consume them through one unit-returning no-scan apply path.
- [Phase 134]: Use only a bounded fingerprint-to-members forward map and retire aliases through a capped preparation-time reverse scan.
- [Phase 134]: Validate candidate cursor cardinality before inspection while preserving resumable sibling traversal semantics.
- [Phase 134]: Prepare compact, serving, fanout, and peer target work completely from authoritative final membership before mutation.
- [Phase 134]: Restrict structural enforcement to exact brace-balanced target apply bodies while excluding fallible validation and aggregate dispatch.
- [Phase 134]: Make the validated lifecycle capability private and consumable so stale authority epoch or canonical revision fails before any target mutation.
- [Phase 134]: Apply canonical core, compact, serving, fanout, peer, unbroadcast, persistence, and evidence projections in one exact order under the sole authority lock.
- [Phase 134]: Keep reconciliation read-only and bounded: production exposes only seven fixed labels and capped counts, while exact identities remain test-only.
- [Phase 134]: Admission reports and Phase 133 feedback remain outward compatibility data; only prepared lifecycle facts drive projection.
- [Phase 134]: Exact package member-to-peer provenance is supplied only to preparation and cannot become a second projection authority.
- [Phase 134]: Production and exact package scenario coverage share one lifecycle dispatcher helper.
- [Phase 134]: Maintenance commands retain exact typed identity while sharing one complete prepared lifecycle projection path.
- [Phase 134]: Reorg replacement blocks and reconsidered transactions commit sequential ReorgStep commands so each preparation observes prior membership.
- [Phase 134]: Allocate effect IDs only after bounded reservation succeeds so pressure does not consume identity space.
- [Phase 134]: Check and record achieved completion identity before freshness while preserving newer authoritative state.
- [Phase 134]: Keep effect-facing methods as LifecycleCommand facades over the sole dispatcher.
- [Phase 134]: PeerEmission owns an affine write capability; only acknowledge_write can create its receipt after external success.
- [Phase 134]: Node sync and RPC inbound complete each written command before advancing, preserving exact successful-prefix truth.
- [Phase 134]: RPC failure coverage uses a private injected executor while production completion remains routed through ManagedNetworkHandle.
- [Phase 134]: Keep current-schema snapshot execution beside Fjall save_mempool_snapshot and outside lifecycle authority.
- [Phase 134]: Mint SnapshotWriteReceipt only after encoding and the requested persistence mode succeed.
- [Phase 134]: Use public snapshot prepare and complete facades for end-to-end persistence coverage while retaining exact dispatcher-state generation assertions.
- [Phase 134]: Use prepared admitted and teardown order as the exact consequence-order contract while canonical delta membership remains unordered.
- [Phase 134]: Keep the independent model std-only and production-helper-free, then compare its target vectors with production reconciliation outside that module.
- [Phase 134]: Compile failure injection only under cfg(test), scope it per thread, and invoke it only during preparation before mutation.
- [Phase 134]: Guard exact seven projection applies plus aggregate commit while leaving validation fallible.
- [Phase 134]: Use stable contract-family diagnostics and fresh filesystem fixtures for fail-closed mutation coverage.
- [Phase 134]: Keep Phase 134 verification deterministic and require identical Phase 133 to Phase 134 to Phase 117 order in both verifier surfaces.
- [Phase 134]: Keep MPLIFE-01 through MPLIFE-04 pending until phase-level verification.
- [Phase 134]: Published one ManagedNetworkHandle authority with a mandatory seven-target lifecycle projection and family-specific outside-lock effects.
- [Phase 134]: Preserved D-18 and all Phase 135-138, broad relay, public-network CI, and production-readiness deferrals.
- [Phase 134]: Install a unique non-initial authority epoch at the ManagedNetworkHandle construction boundary while retaining fixture-friendly initial epochs before handle installation.
- [Phase 134]: Store pending and completed truth as complete family-specific keys; use the monotonic family ID only as bounded eviction-order metadata.
- [Phase 134]: Reject foreign or immutable-mismatched receipts with a typed lifecycle error; reserve AchievedButStale for exact pending work whose current targets advanced.
- [Phase 134]: Bound peer session-generation history by connected or pending-effect ownership.
- [Phase 134]: Route emission receipts through one evidence-bearing lifecycle command and one authority guard.
- [Phase 134]: Exact pre-achievement peer abort validates immutable ownership while allowing capacity release after lifecycle or target-peer freshness advances.
- [Phase 134]: Every audited node and RPC peer emission terminates through achieved completion or explicit current-and-suffix abort.
- [Phase 134]: Fanout cleanup attempts every suffix abort and surfaces cleanup failure without rolling back successful-prefix evidence.
- [Phase 134]: Keep MPLIFE-01 and MPLIFE-04 pending until independent phase re-verification.
- [Phase 134]: Snapshot abort validates immutable ownership while ignoring later lifecycle and dirty freshness.
- [Phase 134]: Fjall snapshot execution owns encode, save, and exactly one complete-or-abort terminal dispatch.
- [Phase 134]: MPLIFE-01 through MPLIFE-04 remain pending until phase re-verification.
- [Phase 134]: Candidate cursors retain complete child txid+wtxid identities without retaining child transaction bodies.
- [Phase 134]: Expected unbroadcast membership is the retry-eligible subset of canonical mempool identities and is audited by symmetric difference.
- [Phase 134]: MPLIFE-01 through MPLIFE-04 remain pending until phase re-verification.
- [Phase 134]: Raw accepted-package command capacity is enforced before deduplication because duplicate inputs still consume bounded preprocessing work.
- [Phase 134]: Accepted-package fingerprint identities remain immutable across same-command retirement; conflicting members fail closed while identical duplicates are idempotent.
- [Phase 134]: Accepted-package retained capacity is evaluated on the deduplicated final map after bounded same-transition retirements.
- [Phase 134]: Revision validation and patch application execute inside one mutable consuming boundary.
- [Phase 134]: Legacy validated transition APIs remain covered through Plan 20 for Plan 21 node migration.
- [Phase 134]: Core commit is the sole fallible aggregate mutation and executes before dependent target application.
- [Phase 134]: The old validated transition surface was removed in the same commit that migrated the live node.
- [Phase 134]: Stale atomicity is proved through the production dispatcher with a complete eight-domain snapshot.
- [Phase 134]: Treat commit_sealed_lifecycle and apply_prepared_lifecycle as one explicit aggregate root because Plan 21 separated atomic commit from infallible dependent applies.
- [Phase 134]: Classify apply-call behavior only by exact fully qualified symbol; unresolved or unknown repo-owned helpers fail closed.
- [Phase 134]: Use visited fully qualified symbols to terminate recursive helper cycles without suppressing reachable violations.
- [Phase 134]: Keep MPLIFE-01 through MPLIFE-04 pending until phase re-verification.
- [Phase 134]: Normalize all five canonical Phase 134 claim surfaces before matching curated D-18 semantic claim families.
- [Phase 134]: Require all three Phase 134 parity records to remain in_progress while MPLIFE or recorded verification gaps remain pending.
- [Phase 134]: Run Phase 134 mutation, transitive apply, and live guards in dependency order immediately after Phase 133.
- [Phase 134]: Map every review finding to exact source and regression evidence without treating the audit as independent verification.
- [Phase 134]: Keep Phase 134 in progress and MPLIFE-01 through MPLIFE-04 pending until separate fresh re-verification.
- [Phase 134]: Preserve D-18 and Phase 135-138 deferrals while publishing only bounded repaired guarantees.
- [Phase 135]: Classify captured_generation = u64::MAX as snapshot-level StructuralCorruption at try_new, try_new_current, and v2 decode.
- [Phase 135]: Refuse LifecycleGeneration::MAX before recovery projection rebuild so a later ordinary mutation can still call checked_next.
- [Phase 135]: Keep MPDUR Pending and leave requirements-completed empty until lifecycle-valid phase verification.
- [Phase 135]: Do not recreate canonical 135-VERIFICATION.md; Plan 14 owns fresh verification.
- [Phase 135]: Capture and encode share one assert_mempool_snapshot_representable against persisted_mempool_input_limits().
- [Phase 135]: encode_mempool_snapshot compares bytes.len() to limits.max_encoded_bytes, not encoded_size_upper_bound of snapshot dimensions.
- [Phase 135]: Keep MPDUR Pending and leave requirements-completed empty until lifecycle-valid phase verification.
- [Phase 135]: Do not recreate canonical 135-VERIFICATION.md; Plan 14 owns fresh verification.
- [Phase 135]: Require for_persisted_input inside recover_mempool_snapshot_with_loader; store-only tokens are insufficient.
- [Phase 135]: Require vertex and per-record edge guards at prepare_recovery_topology use sites.
- [Phase 135]: directStatementIndex ignores statements disabled by a same-line or preceding Rust attribute.
- [Phase 135]: encode_mempool_snapshot must keep representability and limits.max_encoded_bytes; reject snapshot-derived encoded_size_upper_bound(total_transaction_bytes.
- [Phase 135]: Keep MPDUR Pending and leave requirements-completed empty until lifecycle-valid phase verification.
- [Phase 135]: Do not recreate canonical 135-VERIFICATION.md; the independent verifier owns phase135-14-full-verify.
- [Phase 136]: Cycle length is 600 plus injected jitter; next due uses checked_add and returns None on overflow.
- [Phase 136]: Production inspect/prepare are 256/32; new() accepts 1..=4999 and rejects 0 and >= 5000.
- [Phase 136]: select_maintenance_identities walks only the supplied BTreeSet and marks unprepared members leftover_unattempted.
- [Phase 136]: Keep cycle, budget, and cursor types in retry.rs under the repo 628-line production gate.
- [Phase 136]: Keep IBR-01 and IBR-02 Pending until lifecycle-valid phase verification.
- [Phase 136]: Insert unbroadcast members only when delta().admitted contains the identity and metadata is retry-eligible.
- [Phase 136]: expected_unbroadcast_members is the intersection of the live set with retry-eligible canonical members.
- [Phase 136]: A still-present TransportWritten-cleared member is not a reconciliation mismatch.
- [Phase 136]: Keep IBR-01 and IBR-04 Pending until lifecycle-valid phase verification.
- [Phase 136]: TX INV and TX response are distinct write kinds; INV is announcement, not acknowledgement.
- [Phase 136]: PeerEmission::new stays compact-only and still rejects WireNetworkMessage::Tx.
- [Phase 136]: record_peer_emission increments compact counters only when maybe_evidence_reason is Some.
- [Phase 136]: Keep IBR-03 Pending until lifecycle-valid phase verification.
- [Phase 136]: Clear unbroadcast only on Applied current-epoch TX-response writes as TransportWritten, or LifecycleRemoval.
- [Phase 136]: EligibleServe classify and TX INV writes must not clear membership.
- [Phase 136]: Keep IBR-04 Pending until lifecycle-valid phase verification.
- [Phase 136-receive-independent-maintenance-and-transport-receipts]: Keep the existing final_present() fanout loop; AlreadyPresent parents are not in that list.
- [Phase 136-receive-independent-maintenance-and-transport-receipts]: enqueue_retry_admissions iterates the caller slice only and reuses record_prepared_admission.
- [Phase 136-receive-independent-maintenance-and-transport-receipts]: rebroadcast_deferred means the first hop is recorded and the retry cycle has not yet run.
- [Phase 136-receive-independent-maintenance-and-transport-receipts]: Keep PPKG-04 and IBR-03 Pending until lifecycle-valid phase verification.
- [Phase 136]: Store last prepared identity as the walk cursor so leftovers progress when N < inspect 256.
- [Phase 136]: Drain first-hop INV after accept; abort unused write capabilities until a later socket write path exists.
- [Phase 136]: Always start the retry worker from open-bitcoind main beside the checkpoint worker, never from DurableSyncRuntime.
- [Phase 136]: Keep IBR-01 through IBR-04 Pending until lifecycle-valid phase verification.
- [Phase 138-parity-adversarial-pressure-restart-and-release-guardrails]: Place the D-04/D-05 composition in recovery_cases/restart_composition.rs next to staging, not a new mega-harness — D-05 allows at most one named Rust composition; staging.rs is already 483 lines
- [Phase 138-parity-adversarial-pressure-restart-and-release-guardrails]: Retire SUSTAINED_PRESSURE_MAX_ELAPSED and pin PRESS-05 to N=24 work-count symbols while Phase 117 remains last-gate until Plan 03 — D-07/D-08 require Instant-free default smoke; Plan 03 owns last-gate rewiring
- [Phase 138]: Keep the four new surfaces in_progress and leave existing in_progress surfaces unchanged until Plan 04
- [Phase 138]: Copy the D-21 sentence verbatim and keep the README P2P-row package-relay remain-deferred clause for Phase 117
- [Phase 138]: Leave SUMMARY requirements-completed empty so MPVFY-03 is not flipped before the last-gate checker exists
- [Phase 138-parity-adversarial-pressure-restart-and-release-guardrails]: Pin pressure/failure-injection to missing_pressure_victim_is_rejected because prospective_failure_cases.rs has no trim symbol — The plan pin was not present in the live file; using the existing pressure failure-injection test keeps the same file without inventing a trim harness
- [Phase 138-parity-adversarial-pressure-restart-and-release-guardrails]: Treat future, future-gated, and without broadening as D-22 no-claim markers so live README future-gated readiness wording stays valid — D-22 says future-gated wording remains valid; adding those markers avoids a false positive without globally allowing package relay
- [Phase 138-parity-adversarial-pressure-restart-and-release-guardrails]: Leave SUMMARY requirements-completed empty so MPVFY IDs are not flipped before Plan 04 — Putting MPVFY IDs in requirements-completed fails check-active-milestone-verification-traceability before Plan 04 verification exists
- [Phase 138]: Flip leftover Pending v2.2 rows only after the Plan 03 checker can name evidence. Phase 138 owns MPVFY-01 through MPVFY-04; do not archive the milestone.
- [Phase 138-parity-adversarial-pressure-restart-and-release-guardrails]: Flip leftover Pending v2.2 rows only after the Plan 03 checker can name evidence
- [Phase 138-parity-adversarial-pressure-restart-and-release-guardrails]: Retarget the Phase 134 pending lock so historical 134-GAPS.md does not block done after MPLIFE is Complete
- [Phase 138-parity-adversarial-pressure-restart-and-release-guardrails]: Phase 138 owns MPVFY-01 through MPVFY-04; do not archive the milestone
- [Phase 140]: FlushPolicyTime is a u64 unix-seconds newtype; core never samples a clock
- [Phase 140]: classify_cache_size is pub(crate); Plan 01 decide_flush always returns FlushDecision::None
- [Phase 140]: Combined 140-01 RED and GREEN into one hook-passing feat commit because pre-commit runs cargo test
- [Phase 140]: Periodic LARGE/CRITICAL flushes even when not due; IfNeeded LARGE without pressure returns None
- [Phase 140]: RefuseDiskSpace replaces an intended Flush or Sync only; guard is free_bytes < 192 * entry_count
- [Phase 140]: Combined 140-02 RED and GREEN into one hook-passing feat commit because pre-commit runs cargo test
- [Phase 140]: RecoveryDecision maps count 0/1/2/other; one-element is first-class, not InconsistentOtherCount
- [Phase 140]: Crate root re-exports flush and recovery types; classify_cache_size stays crate-private
- [Phase 140]: Combined 140-03 RED and GREEN into one hook-passing feat commit because pre-commit runs cargo test
- [Phase 140]: Leave MGR-03 Pending and requirements-completed empty until lifecycle-valid phase verification
- [Phase 141]: Combined 141-01 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh
- [Phase 141]: CoinsCache::from_parent does not probe parent.best_block; overlay starts with maybe_best_block None
- [Phase 141]: FjallNodeStore database() and coins_keyspace() live in fjall_store/coins_access.rs so fjall_store.rs stays under 628 lines
- [Phase 141]: SchemaVersion::CURRENT stays 1; COIN-01 and CSOBS-03 remain Pending until phase verification
- [Phase 141]: Combined 141-02 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh
- [Phase 141]: Split after adding an item when accumulated encoded bytes exceed the cap (Knots SizeEstimate-after-write)
- [Phase 141]: Fjall WriteBatch is named OwnedWriteBatch at the crate root; coins_view uses that type for commit_batch
- [Phase 141]: SchemaVersion::CURRENT stays 1; COIN-01 and CSOBS-03 remain Pending until phase verification
- [Phase 141]: Combined 141-03 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh
- [Phase 141]: MemoryCoinsView gained Debug/Clone/Default/PartialEq/Eq so MemoryChainstateStore can keep those derives
- [Phase 141]: encode_block_undo and decode_block_undo allow(dead_code) until Plan 04 writes undo: records
- [Phase 141]: SchemaVersion::CURRENT stays 1; COIN-01 remains Pending until phase verification
- [Phase 141]: Combined 141-04 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh
- [Phase 141]: persist_progress still writes leftover snapshots and also seeds coins so schema-2 reopen can hydrate until Phase 142 write-site cutover
- [Phase 141]: Phase 135 snapshot-recovery checks now require store CURRENT = 2 and DurableSyncRuntime hydrate_chainstate_for_open
- [Phase 142]: Combined 142-01 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh — Hooks run the workspace verifier, so a RED-only commit cannot pass pre-commit.
- [Phase 142]: classify_markers (2, None) is Ok(Interrupted); store open succeeds; hydrate still fail-closes — FLUSH-02 cannot replay if open fail-closes; leftover scanners must not invent a tip (D-09/D-10).
- [Phase 142]: map_heads_error matches ChainstateError::InterruptedWrite; leftover-empty skipped when InterruptedTwoHeads — IN-01/WR-02: Display remaps hide crash state; leftover-empty must not shadow interrupted H.
- [Phase 142]: Leave FLUSH-02 Pending until replay and lifecycle-valid phase verification — This plan only makes the interrupted marker observable; ReplayBlocks is Plan 03.
- [Phase 142]: Combined 142-02 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh — Hooks run the workspace verifier, so a RED-only commit cannot pass pre-commit.
- [Phase 142]: from_parent does not probe best_block; overlay occupancy starts at 0 while coins_best_block reads the parent tip — Empty overlay cannot pretend the parent tip is cache-dirty (141 / D-13).
- [Phase 142]: estimated_cache_bytes is first-party overlay math (48 + unspent payload), never Fjall len or LevelDB SizeEstimate — D-05 forbids Fjall item counts and LevelDB SizeEstimate; 450/4/8 MiB defaults stay in the shell.
- [Phase 142]: Leave MGR-01 and MGR-02 Pending until Fjall attach, manager flush, and lifecycle-valid phase verification — This plan only supplies the generic parent and occupancy facts; later plans own attach and flush wiring.
- [Phase 142]: Combined 142-03 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh
- [Phase 142]: batch_write_with_persist_mode allows present H so replay can finish markers; ordinary batch_write still refuses H
- [Phase 142]: CoinsCache::set_best_block and into_dirty_parent_write extract Sync writes without calling cache.flush
- [Phase 142]: Leave FLUSH-02 Pending until lifecycle-valid phase verification
- [Phase 142]: Combined 142-04 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh
- [Phase 142]: execute_flush is generic over FlushPersistSink so undo-abort uses UndoFailingSink + RecordingCoinsView
- [Phase 142]: probe_disk_free_bytes returns u64::MAX because the node crate forbids unsafe libc::statvfs
- [Phase 142]: Leave MGR-01 Pending until persist cutover and lifecycle-valid phase verification
- [Phase 142]: Combined 142-05 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh — Hooks run the workspace verifier, so a RED-only commit cannot pass pre-commit.
- [Phase 142]: FlushLifecycle is a required ManagedChainstate field; persist() always execute_flush(IfNeeded) — Plan revision forbids Option gating and when-installed hedges.
- [Phase 142]: Periodic/Always go through ManagedNetworkHandle; the worker does not own a second lifecycle — Same ownership pattern as checkpoint.rs.
- [Phase 142]: Leave MGR-01 Pending until persist_progress cutover and lifecycle-valid phase verification — persist_progress leftover snapshot writes remain for Plan 06.
- [Phase 142]: Open attaches initialize cache and lifecycle; leftover snapshot UTXOs stay unread — MGR-02 and D-13 require restart from coins B, not leftover hydrate
- [Phase 142]: persist_progress writes headers and runtime only; credit requires coins B == claimed tip — D-17 cutover: leftover snapshot is no longer live UTXO truth
- [Phase 142]: Flush persist_chain_meta after coins write; do not clobber the fork-aware header index — Reopen hydrates active_chain from chain_meta; active-chain-only header persist wiped competing branches
- [Phase 142]: Leave MGR-01 Pending; this plan completes MGR-02 only — Plan frontmatter requirements are MGR-02; manager CanFlush verification remains later
- [Phase 143]: Combined 143-01 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh
- [Phase 143]: BlockServingPresenceFacts sits beside BlockServingStatusFacts on the shell report seam
- [Phase 143]: SideChain chain_position still uses cache_present, not durable_payload_present
- [Phase 143]: Leave HAVL-02 and HAVL-03 Pending until later plans and lifecycle-valid phase verification

### Pending Todos

- Keep historical `.planning/phases/` directories tracked because repository verifiers consume selected evidence.
- Keep repo-local Cargo and Bazel command forms in UAT guidance.
- Preserve existing explicit relay activation and public-network opt-in boundaries.
- Keep prune/archive product modes, assumeutxo, compact-filter serving, public defaults, and production claims deferred.

### Blockers/Concerns

- Phase 141 planning should resolve Fjall coins keyspace, compact codec home, undo-record location, and schema-bump versus multi-namespace migration before the first production write.
- Phase 142 planning should name the allowed crash-loss window and the interrupted-flush replay-versus-fail-closed rule when bodies or undo are missing.
- Phase 143 planning should pin `durable_availability` and reserve `Pruned` so help text cannot be read as prune-mode.
- Historical Phase 130–138 planning notes remain in the v2.2 archive. No open v2.2 blockers remain after the passed milestone audit.

## Latest Milestone Archive

- Roadmap: `.planning/milestones/v2.2-ROADMAP.md`
- Requirements: `.planning/milestones/v2.2-REQUIREMENTS.md`
- Audit: `.planning/milestones/v2.2-MILESTONE-AUDIT.md`

## Session Continuity

Last session: 2026-09-17T03:54:56.122Z
Stopped at: Completed 143-01-PLAN.md
Resume file: None
