---
phase: 157-safe-activation-and-scheduled-index-catch-up
reviewed: 2026-10-05T21:16:13Z
depth: standard
files_reviewed: 58
files_reviewed_list:
  - packages/open-bitcoin-chainstate/src/filter_index.rs
  - packages/open-bitcoin-chainstate/src/filter_index/catch_up.rs
  - packages/open-bitcoin-chainstate/src/filter_index/catch_up/budget.rs
  - packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests.rs
  - packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests/budget.rs
  - packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests/durability.rs
  - packages/open-bitcoin-node/src/chainstate.rs
  - packages/open-bitcoin-node/src/chainstate/filter_index.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/fjall_sink.rs
  - packages/open-bitcoin-node/src/chainstate/tests.rs
  - packages/open-bitcoin-node/src/chainstate/tests/filter_index.rs
  - packages/open-bitcoin-node/src/lib.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/inputs.rs
  - packages/open-bitcoin-node/src/storage/coins_view.rs
  - packages/open-bitcoin-node/src/storage/fjall_store.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/lifecycle.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append/admission.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append/fencing.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append_proof.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/prune.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/prune/records.rs
  - packages/open-bitcoin-node/src/sync/block_response.rs
  - packages/open-bitcoin-node/src/sync/open_runtime.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/accepted.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/failures.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/fixtures.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/measurements.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/evidence.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/accepted_faults.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/history_loss.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/retention.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/startup.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/startup/configured.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/startup/configured/faults.rs
  - packages/open-bitcoin-rpc/src/config.rs
  - packages/open-bitcoin-rpc/src/config/blockfilter.rs
  - packages/open-bitcoin-rpc/src/config/loader.rs
  - packages/open-bitcoin-rpc/src/config/loader/blockfilter.rs
  - packages/open-bitcoin-rpc/src/config/tests.rs
  - packages/open-bitcoin-rpc/src/config/tests/blockfilter.rs
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T21:16:13Z
review_partition: runtime-config-real-store
scope_source_sha256: 999e91e833808e3de1b338105b19ce97d6336457216c5b619e052ef4a8d4e9e7
base_commit: 26bc454a66d6d7a241fc01be84dc6fbf0416d15a
source_state: uncommitted frozen implementation including explicitly scoped untracked source
whole_phase_review_complete: false
verification_owner: root
---

# Phase 157: Runtime Partition Code Review Report

**Reviewed:** 2026-10-05T21:16:13Z\
**Depth:** standard\
**Files Reviewed:** 58\
**Status:** clean within this explicit partition

## Summary

No evidenced bug, security vulnerability or actionable quality defect was found in the 58 runtime/configuration/store Rust files listed above. Source and registered tests were inspected directly; plan summaries were used for intent and reported verification provenance, not as substitutes for implementation evidence.

This is the first explicit review partition covering Plans 01–07 and 09. It does not establish whole-phase completion. Plan 08 daemon/bin paths and Plan 10 scripts/docs/catalog work, including historical Phase 155 guards and the two legacy constructor source assertions, are excluded and require the root's later review. Full native verification, coverage, Bazel, final source/security/lifecycle closure and Git finalization remain root-owned.

## Material Guidance and Scope

AGENTS.md, AGENTS.bright-builds.md, placeholder-only standards-overrides.md and standards/index.md informed the review, together with architecture, code-shape, testing, verification, local-guidance and Rust standards. The GSD code-review skill and reviewer contract were applied. Project skill directories are absent.

Both canonical active lesson files were read completely: the global ledger is 5,230 bytes and the repository ledger is 1,958 bytes, totaling 7,188 bytes and 2,397 conservative estimated tokens. No archive was loaded. The existing phase context records an audit baseline; this read-only review adds no lesson or audit mutation.

Phase 157 CONTEXT, RESEARCH, applicable Plans 01–07 and 09, their existing summaries and TURN-MEASUREMENTS were reconciled with source. The explicit 58-file scope contains no ignored/generated/planning source exclusions. It includes current uncommitted and untracked source rather than deriving scope from committed history.

The source fingerprint is SHA256 over each inventory entry in its supplied order, concatenating its UTF-8 repository-relative path, NUL, lowercase SHA256 of its bytes, and newline. This records the inspected source independently of the temporary inventory file.

## Reviewed Safety and Correctness Boundaries

| Boundary | Concrete evidence and assessment |
| --- | --- |
| Exact default-off configuration | `config/blockfilter.rs` preserves source ordering, false-negation identity, CLI-last/config-first scalar choice and merged named-mode validation. `config/loader/blockfilter.rs` carries ordered active/default occurrences. Actual loader tests cover include order, mixed forms, double negatives, excluded types, redaction and unchanged activation policies. The pinned local `common/args.cpp`, `common/settings.cpp` and `util/strencodings.h` were cross-referenced; saturating integer-prefix behavior remains nonzero on overflow. |
| Configured pre-prune startup | `filters/lifecycle.rs:97` preflights the recovered checkpoint's required suffix before owner/checkpoint/protection publication. Stronger retention is separate from required inputs. `flush_lifecycle.rs:267` resumes prune only after the configured transition. Actual missing-mate, paired-loss, rewind, ahead-row and stronger-lock tests compare persisted state after all prior handles close. |
| Sealed recovered provenance | `chainstate/fjall_store.rs:128` seeds only the specialized recovered manager, requiring the genuine same-store coins parent and current recovered proof. Public constructors, Clone and reconstruction are untracked; replacement/reorg clears lineage. Pure targets and fences confer no storage capability. |
| Exact owned flush receipt | `chainstate/fjall_store.rs:53` captures a current proof and checked expected R+2. The achieved same-invocation flush creates consuming completion. `filters/ownership.rs:133` checks the original proof, completed R+1 coins, current R+2 metadata, same store/branch/generation/endpoints, actual B and absent H. Public/raw/pending writes cannot reinstall authority; extra writers and errors invalidate shared clone authority. Tests cover fake/custom/foreign managers, all interleave windows, nominal pending recreation, noop, replay and exhaustion. |
| Accepted-before-persist facts | `chainstate.rs:332` absorbs once, observes lineage and complete staged facts, then performs fallible persistence. `chainstate/filter_index.rs:263` enlarges the accepted target without moving ordered progress. Next-height historical/same-block inputs survive later errors. Network dependent projections are applied before an already-accepted persistence error returns; requested body failures retain explicit error state. |
| Bounded turn and publication | `catch_up.rs:262` selects consecutive canonical heights and borrows existing undo, with native body-length and count admission before decode. `filters/ownership.rs:49` permits only monotonic remaining-budget reduction that retains acquisition cost. `filters/append.rs:185` rechecks current proof under publication and commits achieved suffix/state/protection atomically. Acquisition, recheck, source, generation, candidate and completion accounting are combined without duplicating selected-block/output counts. Full forest/ancestry/metadata/projection scans stay in recovery/general readers and existing full flushes. |
| Safe checkpoint and stronger protection | Processed rows remain separate from safe durability. Release intentionally waits for the exact authenticated durable tip; intermediate appends preserve the actual stronger covering lock. A genuine later owned flush can earn a bounded zero-record release. Ahead immutable rows remain replay inputs rather than restart authority. Existing Periodic/Always pruning ownership and accepted-unflushed behavior are preserved. |

## Verification Provenance and Limits

No Cargo, Bazel, test, Git, source mutation or quarantined dependency enumeration was performed by this reviewer. The root supplied passing focused controls for Plans 01–07 and 09 and strict Node Clippy; summaries and TURN-MEASUREMENTS record their commands, counts and observed results. This report independently assesses source and assertions, not a fresh execution of those checks. Test counts overlap and are not summed.

The registered tests exercise genuine continuously validated history, software writer/publication faults and closed Fjall reopen. Sparse metadata fixtures and nonactive volume are explicitly labeled as codec/storage evidence. Reported reservations are software admission accounting rather than allocator RSS or measured comparison counts. Startup integrity and whole-suffix preflight remain chain-wide; generation under authority and SyncAll latency have measured observations rather than hard wall-clock guarantees.

The existing ScriptBuf representation limit, explicit resource/map refusal, 384 MiB retained live-fact cap, absence of automatic repair and conservative exact-tip retention are documented contracts. They are not newly classified defects. Runtime branch replacement, public filter RPC/peer serving and broad operator projections remain later phases.

## Balanced Simplification Pass

The implementation reuses one publication mutex, one ordered reducer, actual staged undo and the existing coins/prune owner. Small helpers separate native admission, generation, receipt validation and achieved publication. The single next-height fact slot avoids a second history queue; constant-size proof identities avoid full-history copies in turns. No simplification was identified that removes meaningful checks while preserving raw-writer/interleaving, nonmutation and bounded-work guarantees. No source edits were made.

## Result and Remaining Review

All reviewed files meet the applicable correctness and security review criteria at standard depth. No issues found in this partition. The exact 58-file list is retained in frontmatter for downstream re-review.

Root must separately close the excluded Plan 08/10 partition and required whole-phase verification before any phase-complete or commit/push claim.

***

_Reviewed: 2026-10-05T21:16:13Z_\
_Reviewer: gsd-code-reviewer_\
_Depth: standard; runtime/config/real-store partition only_
