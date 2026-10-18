---
phase: 165-shared-exact-work-authority
reviewed: 2026-10-17T12:20:16Z
depth: standard
review_scope: preliminary-rust-tests-fixtures-benchmarks-only
base_commit: d0104a997c0362bf65b81de1c8fec8ddc3012a1b
files_reviewed: 234
files_reviewed_list:
  - packages/open-bitcoin-bench/src/cases/chainstate.rs
  - packages/open-bitcoin-bench/src/cases/network.rs
  - packages/open-bitcoin-bench/src/cases/storage_recovery.rs
  - packages/open-bitcoin-bench/src/cases/sync_runtime.rs
  - packages/open-bitcoin-bench/src/fixtures.rs
  - packages/open-bitcoin-bench/src/runtime_fixtures.rs
  - packages/open-bitcoin-chainstate/src/block_filter/tests.rs
  - packages/open-bitcoin-chainstate/src/block_filter/tests/fixtures.rs
  - packages/open-bitcoin-chainstate/src/block_filter/tests/validated.rs
  - packages/open-bitcoin-chainstate/src/engine/stage.rs
  - packages/open-bitcoin-chainstate/src/engine/tests.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/apply_and_connect_errors.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/connect_snapshot_and_cache.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/staged_connect_and_reorg.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/derives_contexts_from_chainstate_metadata.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/derives_contexts_from_chainstate_metadata/difficulty.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/disconnect_tip_skips_unspendable_outputs_and_reports_missing_created_out.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/phase165_rebind.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/phase165_tests.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/phase165_work_authority.rs
  - packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests/durability.rs
  - packages/open-bitcoin-chainstate/src/filter_index/tests.rs
  - packages/open-bitcoin-chainstate/src/lib.rs
  - packages/open-bitcoin-chainstate/src/types.rs
  - packages/open-bitcoin-chainstate/tests/parity.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model/tests/sync_and_recovery.rs
  - packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility/tests.rs
  - packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility/tests/phase165_work.rs
  - packages/open-bitcoin-cli/src/operator/soak/tests/recovery_resume_bounds.rs
  - packages/open-bitcoin-cli/src/operator/soak/tests/runtime/execution_and_checkpoints.rs
  - packages/open-bitcoin-cli/src/operator/soak/tests/runtime/runtime_fixtures.rs
  - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
  - packages/open-bitcoin-cli/src/operator/status/render/tests/inbound_and_sync.rs
  - packages/open-bitcoin-cli/src/operator/support/tests/inbound_status_fixtures.rs
  - packages/open-bitcoin-cli/src/operator/support/tests/sync_fixtures.rs
  - packages/open-bitcoin-cli/src/operator/support/tests/sync_soak_forensics.rs
  - packages/open-bitcoin-cli/src/operator/wallet/tests.rs
  - packages/open-bitcoin-cli/tests/operator_binary/process_fixtures.rs
  - packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs
  - packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs
  - packages/open-bitcoin-consensus/src/chain_proof/tests.rs
  - packages/open-bitcoin-consensus/src/chain_proof/tests/phase165_vectors.rs
  - packages/open-bitcoin-consensus/src/crypto.rs
  - packages/open-bitcoin-consensus/src/crypto/tests/phase165_tests.rs
  - packages/open-bitcoin-mempool/src/pool/tests.rs
  - packages/open-bitcoin-mempool/tests/parity.rs
  - packages/open-bitcoin-network/src/filter_serving/policy/tests.rs
  - packages/open-bitcoin-network/src/header_store.rs
  - packages/open-bitcoin-network/src/header_store/eligibility.rs
  - packages/open-bitcoin-network/src/header_store/eligibility/tests.rs
  - packages/open-bitcoin-network/src/header_store/eligibility/tests/phase165_authority.rs
  - packages/open-bitcoin-network/src/header_store/eligibility/tests/phase165_cached_order.rs
  - packages/open-bitcoin-network/src/header_store/proof/tests.rs
  - packages/open-bitcoin-network/src/header_store/tests.rs
  - packages/open-bitcoin-network/src/header_store/tests/phase165_tests.rs
  - packages/open-bitcoin-network/src/peer/tests/block_announcement_fallback_cases.rs
  - packages/open-bitcoin-network/src/peer/tests/block_fixtures.rs
  - packages/open-bitcoin-network/src/peer/tests/block_sync_request_cap_cases.rs
  - packages/open-bitcoin-network/src/peer/tests/block_sync_request_policy_cases.rs
  - packages/open-bitcoin-network/src/peer/tests/peer_action_cases.rs
  - packages/open-bitcoin-network/tests/parity.rs
  - packages/open-bitcoin-node/src/chainstate.rs
  - packages/open-bitcoin-node/src/chainstate/filter_reorg/tests.rs
  - packages/open-bitcoin-node/src/chainstate/filter_reorg/tests/faults.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store/tests.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store/tests/fixture.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store/tests/fixture/managed.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/fjall_sink.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/prune_apply/tests.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush/header_checkpoint.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/initialize.rs
  - packages/open-bitcoin-node/src/chainstate/live_fixture.rs
  - packages/open-bitcoin-node/src/chainstate/replay/tests.rs
  - packages/open-bitcoin-node/src/chainstate/tests.rs
  - packages/open-bitcoin-node/src/chainstate/tests/filter_index.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history/tests.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history/tests/coverage.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history/tests/fixtures.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history/tests/network.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history/tests/phase165_work_authority.rs
  - packages/open-bitcoin-node/src/chainstate/work_authority.rs
  - packages/open-bitcoin-node/src/mempool.rs
  - packages/open-bitcoin-node/src/network/canonical_admission/tests.rs
  - packages/open-bitcoin-node/src/network/compact_receive_candidates.rs
  - packages/open-bitcoin-node/src/network/limited_serve.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests/fixtures.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests/protection.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/query/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests/failures.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests/interleavings.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/transport/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/live_fixture.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/tests.rs
  - packages/open-bitcoin-node/src/network/tests.rs
  - packages/open-bitcoin-node/src/network/tests/block_connect_disposition.rs
  - packages/open-bitcoin-node/src/network/tests/block_serving.rs
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/maintenance.rs
  - packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases/reorg_reject_evidence.rs
  - packages/open-bitcoin-node/src/network/tests/phase165_eligibility.rs
  - packages/open-bitcoin-node/src/network/tests/phase165_headers.rs
  - packages/open-bitcoin-node/src/network/tests/phase165_rebind.rs
  - packages/open-bitcoin-node/src/network/tests/phase165_work_authority.rs
  - packages/open-bitcoin-node/src/network/tests/phase165_work_authority/fixtures.rs
  - packages/open-bitcoin-node/src/network/tests/phase165_work_authority/freshness.rs
  - packages/open-bitcoin-node/src/network/tests/phase165_work_authority/history.rs
  - packages/open-bitcoin-node/src/network/tests/runtime_projection.rs
  - packages/open-bitcoin-node/src/network/work_authority.rs
  - packages/open-bitcoin-node/src/status/tests/progress_guarantee_and_reorg.rs
  - packages/open-bitcoin-node/src/status/tests/snapshot_projection.rs
  - packages/open-bitcoin-node/src/storage/chain_compatibility.rs
  - packages/open-bitcoin-node/src/storage/chain_compatibility/tests.rs
  - packages/open-bitcoin-node/src/storage/chain_compatibility/tests/phase165_work.rs
  - packages/open-bitcoin-node/src/storage/chain_compatibility/tests/preservation.rs
  - packages/open-bitcoin-node/src/storage/fjall_store.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append/fencing.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/ownership.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/retention.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/retention/publication.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/payload_usage/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/prune_records.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/prune_unlink.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/prune_unlink/protection.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/confirmation_migration.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/reopen.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/tests/chainstate_wallet_and_headers.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/work/tests.rs
  - packages/open-bitcoin-node/src/sync/block_reconcile.rs
  - packages/open-bitcoin-node/src/sync/live_fixture.rs
  - packages/open-bitcoin-node/src/sync/runtime_state/tests.rs
  - packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint.rs
  - packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint/faults.rs
  - packages/open-bitcoin-node/src/sync/tests.rs
  - packages/open-bitcoin-node/src/sync/tests/block_requests.rs
  - packages/open-bitcoin-node/src/sync/tests/block_response/connection_progress.rs
  - packages/open-bitcoin-node/src/sync/tests/block_response/peer_tip_evidence.rs
  - packages/open-bitcoin-node/src/sync/tests/block_response/response_failures.rs
  - packages/open-bitcoin-node/src/sync/tests/durable_tip.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/accepted.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/failures.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/fixtures.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/rpc_faults.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/accepted_faults.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/retention.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/operator_status.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/peer_evidence.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/peer_fixtures.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/peer_fixtures/phase165_source.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/publication_faults.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/recovery.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches/manual_lifecycle.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/failures.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures/live.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures/return_branch.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/measurements.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/protection.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/retention.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/startup/recovery.rs
  - packages/open-bitcoin-node/src/sync/tests/inflight_failures.rs
  - packages/open-bitcoin-node/src/sync/tests/no_progress_diagnosis.rs
  - packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/phase162.rs
  - packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/wire_goldens.rs
  - packages/open-bitcoin-node/src/sync/tests/peer_resolution/resolver_rotation.rs
  - packages/open-bitcoin-node/src/sync/tests/phase158_preflight.rs
  - packages/open-bitcoin-node/src/sync/tests/phase165_candidates.rs
  - packages/open-bitcoin-node/src/sync/tests/phase165_candidates/fixtures.rs
  - packages/open-bitcoin-node/src/sync/tests/phase165_live_fixture.rs
  - packages/open-bitcoin-node/src/sync/tests/phase70_peer/failure_classification.rs
  - packages/open-bitcoin-node/src/sync/tests/phase70_peer/fallback_rotation.rs
  - packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs
  - packages/open-bitcoin-node/src/sync/tests/reorg_reconciliation.rs
  - packages/open-bitcoin-node/src/sync/tests/restart_and_recovery.rs
  - packages/open-bitcoin-node/src/sync/tests/restart_chainstate.rs
  - packages/open-bitcoin-node/src/sync/tests/restart_chainstate/download_and_invalid_body.rs
  - packages/open-bitcoin-node/src/sync/tests/restart_chainstate/persist_and_hydrate.rs
  - packages/open-bitcoin-node/src/sync/tests/restart_resume_matrix.rs
  - packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs
  - packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs
  - packages/open-bitcoin-node/src/sync/tests/soak.rs
  - packages/open-bitcoin-node/src/sync/tests/status_projection.rs
  - packages/open-bitcoin-node/src/sync/tests/stay_current_persistence.rs
  - packages/open-bitcoin-node/src/sync/tests/stay_current_progress.rs
  - packages/open-bitcoin-node/src/sync/tests/structured_runtime_logging.rs
  - packages/open-bitcoin-node/src/sync/tests/support_blocks.rs
  - packages/open-bitcoin-node/src/sync/tests/support_runtime.rs
  - packages/open-bitcoin-node/src/sync/tests/sync_cycles.rs
  - packages/open-bitcoin-node/src/sync/tests/synthetic_long_chain.rs
  - packages/open-bitcoin-node/src/sync/tests/wallet_rescan_runtime.rs
  - packages/open-bitcoin-node/src/sync/types/summary/tests.rs
  - packages/open-bitcoin-node/src/wallet.rs
  - packages/open-bitcoin-node/src/wallet_registry/rescan/tests.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/canonical_chain.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/daemon_sync.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/operator_evidence.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/peer_serving/evidence.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/inbound.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/lifecycle.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/rpc/retention.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/phase165_work.rs
  - packages/open-bitcoin-rpc/src/context/tests.rs
  - packages/open-bitcoin-rpc/src/context/tests/construction.rs
  - packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility.rs
  - packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility/fixtures.rs
  - packages/open-bitcoin-rpc/src/dispatch/prune/tests/ownership.rs
  - packages/open-bitcoin-rpc/src/dispatch/tests/chain_fixtures.rs
  - packages/open-bitcoin-rpc/src/dispatch/tests/sync_status.rs
  - packages/open-bitcoin-rpc/src/http/tests/filter_index.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs
  - packages/open-bitcoin-rpc/tests/phase165_work_startup.rs
  - packages/open-bitcoin-rpc/tests/support/phase165_live_source.rs
  - packages/open-bitcoin-wallet/src/wallet/tests.rs
findings:
  critical: 0
  warning: 1
  info: 0
  total: 1
status: issues_found
phase_qualified: false
native_qualification: false
security_qualification: false
audited_source_drift_observed: false
excluded_source_drift_observed: true
---

# Phase 165: Preliminary Rust Fixture Review

**One actionable warning:** a migrated Node preparation test now succeeds on an earlier untrusted-source refusal and no longer proves its intended parent-read failure path. No additional same-cause false-pass test was confirmed in the reviewed scope.

This is an independent preliminary static review of Rust test/fixture/benchmark changes. It is not canonical phase REVIEW/VERIFICATION, native execution, security qualification, phase completion or future-phase acceptance.

## Warning

### WR-01: Parent-cache preparation regression test passes before it reaches the parent

**Severity:** Warning — test reliability / lost regression coverage.

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/chainstate/tests.rs:375-403`

**Function:** `prepare_connect_then_drop_leaves_parent_unpopulated` (starts at line 371).

**Full-file SHA-256:** `a77c8ca532e9d14716adacbb8dd8f9bf786367a05da9a1502aef88a64c10e472`.

**Trigger and issue:** The test accepts a genuine genesis, converts it to a snapshot, then constructs `rematerialized` with `ManagedChainstate::from_store(MemoryChainstateStore::from_snapshot(snapshot))` at lines 375–377. The snapshot constructor intentionally installs an Untrusted core. The migrated preparation call at lines 392–398 now follows this path:

1. `ManagedChainstate::prepare_connect_block_with_current_time` calls `authorize_direct_connect`.
1. `chainstate/work_authority.rs:29` calls `bind_live_work_source(params)`.
1. `engine/work_authority.rs:290-295` refuses the actual private Untrusted origin with `ChainstateError::UntrustedWorkLineage`.
1. This occurs before staging, parent-coin lookup, or the deliberately missing second transaction input.
1. Lines 401–403 accept **any** error. The unchanged-cache and still-present-parent-coin assertions therefore pass without exercising the behavior described by the test's panic text: “missing second input must fail after peeking the parent coin.”

The production authority refusal is correct. The defect is the test's false success and loss of coverage for nonwarming preparation after the first input has been read. This is newly exposed by Phase 165's authority checks even though the generic error assertion predates this phase.

**Concrete fix:** Preserve the genuine accepted Memory core rather than rebuilding it from raw snapshot metadata. Consume the seeded manager into its parts, use the closed `CoinsFlush::flush` interface to move effective coins into the parent and clear cache occupancy while retaining live origin, then wrap that same core with `ManagedChainstate::from_chainstate` and the appropriate fixture lifecycle. Before Act, assert genuine live origin, the selected full `mature_params()`, a present parent coin and absent cache occupancy. Require `ChainstateError::MissingCoin` for the intended second outpoint (`Txid([9; 32]), vout: 0`), and retain the original unchanged-occupancy and spendable-parent assertions. Do not change production authority checks, use a mechanical fixture to bypass them, inflate work, normalize counts or silently admit the raw snapshot.

**Required follow-up evidence:** First strengthen the error assertion on the existing fixture and run the exact test to capture the actual RED signal (`UntrustedWorkLineage` instead of the required `MissingCoin`). Then repair genuine fixture setup and capture nonzero GREEN discovery for the same test. Verify that the first real input was encountered before the second-input refusal while cache occupancy remains unchanged, using the exact missing outpoint and, if needed, a narrowly scoped read observation. Run the relevant owning suite and applicable coverage after the heavy slot is available. This reviewer did **not** execute the test or reproduce the RED at runtime; the error path above is established from current source.

The repair belongs in a separately owned, independently checked bounded plan. It is not absorbed into pending Plan 24/28 ownership. No source-guard pin renewal is authorized by this report.

## Scope and boundaries

The exact Git base resolves to `d0104a997c0362bf65b81de1c8fec8ddc3012a1b`. Discovery combined tracked differences against that base with untracked files using normal ignore exclusions, then selected all changed/new first-party Rust paths under `packages/open-bitcoin-*`.

At closure there are 277 discovered Rust file identities:

- 234 in-scope paths: test/fixture/benchmark files, test-only source modules and 15 mixed-source files reviewed only for their changed test sections, doctests, test registration or test observability.
- 38 production-only paths excluded here because production changes belong to `165-PRE-REVIEW-PRODUCTION.md`.
- Five discovered changed paths excluded as pending Plan 24/28 owners. The full fixed exclusions below include unchanged pending owners as well.

The review used actual diffs and new source, changed assertion/function inventory, contextual test and helper reads, source-origin/error-path tracing and a same-cause search across raw snapshot/parent/cache constructions. Representation-only conversions and accessor migrations were checked as such; shared fixture migrations and changed behavior expectations received contextual inspection. The report does not assert a new audit of every unchanged line of the approximately 105,000-line discovered source set.

Applicable guidance and active lessons were already loaded during the immediately preceding production audit, then their current paths/sizes were checked for this continuation. Material inputs remain `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, the local standards index, architecture, code shape, verification, testing and Rust standards, locked Phase 165 D-01 through D-12, relevant implementation/repair objectives and the actual Plan 24/28 owner lists. Both active lesson files were fully read, total 7,188 bytes and summed conservative estimate 2,397 tokens; archives were excluded. No project skill directories were present.

## Related false-pass search

The following nearby cases were inspected specifically for the same cause:

| Case                                                                                              | Result                                                                                                                                                                            |
| ------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `chainstate/tests.rs::prepare_connect_then_drop_leaves_parent_unpopulated`                        | Confirmed warning WR-01. Raw rematerialization plus generic Err hides the intended MissingCoin path.                                                                              |
| `chainstate/tests.rs::prepare_connect_does_not_change_live_utxos_or_best_block`                   | Genuine source and a specific InvalidTipExtension assertion; no same-cause defect confirmed.                                                                                      |
| `chainstate/tests.rs::prepare_chainstate_failure_does_not_warm_live_cache_before_mempool_prepare` | Genuine empty owner and wrong-parent refusal, followed by a successful actual genesis connection; no same-cause defect confirmed.                                                 |
| `engine/tests/.../staged_connect_and_reorg.rs`                                                    | Genuine source stages still reach specific FreshFlagMisapplied or accepted absorption. Foreign preview refusal is deliberate.                                                     |
| `engine/tests/phase165_tests.rs::phase165_raw_snapshot_cannot_earn_parent_work`                   | Deliberate raw-origin refusal control, rather than a claimed parent-read test.                                                                                                    |
| Raw constructors in RPC rescan eligibility and confirmation migration / automatic prune fixtures  | Read-only or deliberately incomplete observer/recovery surfaces; no equivalent claimed connection-after-parent-read proof confirmed.                                              |
| Positive BASIC acceptance/readiness/transport and compact/standard reorg fixtures                 | Migrated to actual same-source acceptance, full selected params or explicit complete-prefix revalidation before strict handoff; no additional confirmed generic-error false pass. |

No additional confirmed finding was identified by this same-cause search. This is a source conclusion; it is not a runtime coverage receipt.

## Other inspected behavior

- Exact proof corpus assertions remain fixed, full-width and independently expected; byte/hex endian and compact sign/zero/overflow tests retain concrete expected results. Helper-derived proof in integration setup is not mislabeled independent arithmetic evidence.
- Raw legacy/exact DTO classification and hydration refusal tests retain malformed/version/width/u128 controls and do not promote numeric representation into live authority.
- Header selection tests now admit mined headers, retain strict first-admitted equal-work expectations and explicitly distinguish unproved imported projections.
- Body eligibility tests cover pending-parent release, FIFO breadth-first order, duplicate stability, unavailable/failed ancestry, capacity/counter refusal, full parameter binding, linkage identity and epoch invalidation. Cached-history tests preserve UNKNOWN order after duplicates and copies.
- Core/Node source and stage tests retain owner/generation/tip/parameter, clone, preview, stale authorization, rebind completeness, cache masks/tombstones, physical B/heads/undo/counts and pre-effect refusal controls. Raw/mechanical fixtures are explicitly unearned.
- Shared sync helpers require complete actual retained bodies and exact logical counts for positive revalidation; missing inputs do not get restored. Header-only readmission helpers explicitly preserve unearned core origin. Fixture-only saved-count enrichment occurs only on a genuine flushed owner and is not ordinary raw recovery.
- Reorg tests replace formerly synthetic/lower-preference activation with actual stronger branches or real earlier checked-body history. Weaker/unknown ties refuse; deliberate disconnect tests retain invalidated BASIC and unchanged physical checkpoints/rows rather than fabricate accepted empty history.
- Normal-maturity Phase 162 recipes retain full Regtest maturity 100 and actual per-block receipts. Retained-owner positive construction is separated from isolated all-handle-close read-only observations; the new public post-close combined proof remains excluded under Plan 28.
- Benchmarks distinguish explicit mechanical transaction-engine fixtures, actual admitted network headers, raw recovery without earned best work and genuine runtime acceptance. No benchmark was executed.
- CLI/RPC/operator fixtures retain fixed-width work expectations, four selector refusal policy, redaction, physical manifests, backend spy checks and clone-only destructive controls. The normal-daemon test invokes the built Open Bitcoin binary through bounded children and distinguishes those refusals from pinned-Knots qualification.
- Diff inspection found no newly ignored test, `should_panic` escape, `cfg(any())` disabling, or unconditional `if false` test bypass in the scoped changes. Removed/renamed test names were checked against their replacement intent, including header/body request policy, strict header ties and explicit disconnect behavior.

## Explicit pending-owner exclusions

These paths are outside this report, regardless of whether they were changed at initial capture:

- `packages/open-bitcoin-rpc/src/inbound_listener/tests/listener_fixtures.rs`
- `packages/open-bitcoin-rpc/src/http/tests/filter_index/fixtures.rs`
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs`
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/filter_index.rs`
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs`
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/fixtures.rs`
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/index_protection.rs`
- `packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs`
- `packages/open-bitcoin-node/tests/phase165_live_fixture.rs`
- `packages/open-bitcoin-node/tests/phase165_live_fixture/requalification.rs`

Plan 28's public parent and new `requalification.rs` child changed during this audit, within those declared exclusions. The public parent changed from SHA-256 `392ca0c03bf4dc095c6175ff05552beab3ad794cf78399c81d9d88d719720e4f` to `1c38f95572bae47ef45753fe3990d1517142d05821498c91bcae1599566f9ca9`; the new child is `881fba3b2d7581e34077971767da9ab2e4c12a99130546abc6b1e203399f547a`. These identities are recorded solely as exclusions and are **not reviewed or accepted** here. All 234 in-scope full-file hashes remained identical between initial capture and closure.

Plan 24's eight Rust owners and Plan 28's two Rust owners require their own later review after stabilization. The fixed list does not waive unresolved failures in those owners.

## Verification limits

No Cargo, Bazel, Bun, project import, test, build, source formatter write or source edit was performed. No nested agent was spawned. No shared metadata, source guard, task/lesson record, stage, commit, push, external message, PR or comment was changed. Only this report was written. Read-only Git discovery, exact-base resolution and `git diff --check` passed.

Current full heavy matrices remain parent/Plan 23 owned. This report does not reuse their results, predict their outcomes or claim native qualification. Root owns the confirmed warning's repair/actual failure signal/coverage follow-up and remaining full-phase gates.

The requirements remain CWOR-01/02/03/05. Canonical bootstrap, all-handle durable exact-work reopening and genesis BASIC (CWOR-04) belong to Phase 166; contextual completeness belongs to Phase 167; actual pinned-daemon qualification belongs to Phase 168. All four ordinary selectors remain refused and default networking remains off. Fixture evidence is not ordinary or pinned-daemon acceptance.

## Full-file identity inventory

Hashes cover complete files, including production text in mixed files and pending excluded text. `test scope` is full test/fixture/benchmark or test-only module scope; `mixed test sections` excludes production sections. Reuse only exact unchanged identities; any later full-file change requires ownership/drift analysis and renewed review of affected behavior. Production-only and pending-owner rows are exclusion records, not new review claims.

| Path                                                                                                                                          | SHA-256                                                            | Scope                    |
| --------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ | ------------------------ |
| `packages/open-bitcoin-bench/src/cases/chainstate.rs`                                                                                         | `eca138eba3518ceb60fce3cb1e069bf11e6d7582eed92ae9e606eb3d17eacfac` | test scope               |
| `packages/open-bitcoin-bench/src/cases/network.rs`                                                                                            | `8c690621a3c5e79610f854b286bef4ec469e4a1f9c9eccbf83c7577a9f694617` | test scope               |
| `packages/open-bitcoin-bench/src/cases/storage_recovery.rs`                                                                                   | `43eb1347f17f63431f98a542355a8352f2d51e5b34c7293f8057fe9c5eed116b` | test scope               |
| `packages/open-bitcoin-bench/src/cases/sync_runtime.rs`                                                                                       | `62668b6f6627ef7b7d7ebb376c16d453748afbeeea78cc02bb8e6d0fdf3902cd` | test scope               |
| `packages/open-bitcoin-bench/src/fixtures.rs`                                                                                                 | `8b540d61afe51db5f46ff926f1f715a8ea926c8af53a58dbe1295a4b0e9e7f96` | test scope               |
| `packages/open-bitcoin-bench/src/runtime_fixtures.rs`                                                                                         | `d21b6ca89b05af06a72764f227921798cf30385a5a29e58f0d3607e746478362` | test scope               |
| `packages/open-bitcoin-chainstate/src/block_filter/tests.rs`                                                                                  | `3bfc470a0442fff760dc16a4db183d94a57230c2a731cddb19d7e56863f6a2bd` | test scope               |
| `packages/open-bitcoin-chainstate/src/block_filter/tests/fixtures.rs`                                                                         | `b9c1ad54fef2a500104d55beade7cce3c7c1e6e274651990cbc6fe4c3bf15a01` | test scope               |
| `packages/open-bitcoin-chainstate/src/block_filter/tests/validated.rs`                                                                        | `cf2319d2c3e6061d4beeea92f411705bcfcf5154f25a81d34f2546891c352801` | test scope               |
| `packages/open-bitcoin-chainstate/src/coins.rs`                                                                                               | `cd59da7b3d47af02d06266ef1fcfaaa4e7d9984438d2a28ef36f201284aa54b2` | excluded production only |
| `packages/open-bitcoin-chainstate/src/coins/cache.rs`                                                                                         | `c4607eb78e30621e1edc093563cc7fb1fe650d72eadf3430a3aae78508222664` | excluded production only |
| `packages/open-bitcoin-chainstate/src/coins/memory.rs`                                                                                        | `b6919e5204326331f3f61970046b766ed454dec6403b7dadb4f219859edd4003` | excluded production only |
| `packages/open-bitcoin-chainstate/src/engine.rs`                                                                                              | `c1d99d5b719d3c04176442f5fe3faadf18dbd8d6bdc71865a92d17a367306156` | excluded production only |
| `packages/open-bitcoin-chainstate/src/engine/overlay_apply.rs`                                                                                | `0bce54ac9cbe8024fd80ea704e7f6a7a0813f963d400a971b52e2919bd0908f9` | excluded production only |
| `packages/open-bitcoin-chainstate/src/engine/stage.rs`                                                                                        | `475ac4c27ac612a61b69805c313d5ca8235b8720608dddf2a9584163a0c2641e` | mixed test sections      |
| `packages/open-bitcoin-chainstate/src/engine/tests.rs`                                                                                        | `9ff68e9f8ab2ed085c885c7ab706d4270aba3ecd09c8e18a252adc54cc4fd234` | test scope               |
| `packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/apply_and_connect_errors.rs`   | `491b3ad6a78719eac51702e44cb2f1fa8c5287930da9dc4cb0534bfaae976d24` | test scope               |
| `packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/connect_snapshot_and_cache.rs` | `c2f88d891c3856a75043c2ff38e46b8ddf102ac97a117de41964e02e776ee3cd` | test scope               |
| `packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/staged_connect_and_reorg.rs`   | `9c6747b6d4e1a9ff220a8c2b7aafa43385247a80c2669301be725d000733b95d` | test scope               |
| `packages/open-bitcoin-chainstate/src/engine/tests/derives_contexts_from_chainstate_metadata.rs`                                              | `f800289e4ab36c7fd1491ffe80381fee6c4c1db7e006fda24a8f709a9ef20c1a` | test scope               |
| `packages/open-bitcoin-chainstate/src/engine/tests/derives_contexts_from_chainstate_metadata/difficulty.rs`                                   | `951de46b475a1620eb639f80d3dd82a0477e000b882b9d315d6bcba189348346` | test scope               |
| `packages/open-bitcoin-chainstate/src/engine/tests/disconnect_tip_skips_unspendable_outputs_and_reports_missing_created_out.rs`               | `6f6fc595b1ec022b1be64f51583e1e55f15023c10f20ffebbec5207fba7d493a` | test scope               |
| `packages/open-bitcoin-chainstate/src/engine/tests/phase165_rebind.rs`                                                                        | `a3f4080591ed73bde274e1458198b7a3cae80a8873b04b11eb5a602ee226831b` | test scope               |
| `packages/open-bitcoin-chainstate/src/engine/tests/phase165_tests.rs`                                                                         | `dda6a8efff0c2d4bd23f9620bc8e9fc296cd5b9487aba4004436b75584819951` | test scope               |
| `packages/open-bitcoin-chainstate/src/engine/tests/phase165_work_authority.rs`                                                                | `dcc91e72fe8ab8f492dab9318c5176e864e11c64c2b87d22b9c06857fa42f653` | test scope               |
| `packages/open-bitcoin-chainstate/src/engine/work_authority.rs`                                                                               | `6c3412159f23e761a283adcccbaf45af3b0cf9aca62af607184d5fc612785dbb` | excluded production only |
| `packages/open-bitcoin-chainstate/src/error.rs`                                                                                               | `bcebfceedef8d23ab4defa74cfec2b4e59f07d89c8867faa52ac8f8c44b33e4f` | excluded production only |
| `packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests/durability.rs`                                                              | `28abb85fe12c90efed0e2fa9a9678ea9c463e011999c7b0b6f93b1cac4a989e9` | test scope               |
| `packages/open-bitcoin-chainstate/src/filter_index/tests.rs`                                                                                  | `4ad8180db268b8dc81b11a492d6ccd74489917bdea9426b0ca3eb03cf73b4d9a` | test scope               |
| `packages/open-bitcoin-chainstate/src/lib.rs`                                                                                                 | `d6a0d6b678ac45ac9e50bfbe68f23de9f4ffd2d8c476b0d67540b3110c16f189` | mixed test sections      |
| `packages/open-bitcoin-chainstate/src/types.rs`                                                                                               | `6ecbd780c0c0501a28b749a496ca4f7431f8ab89c36a0af1a5c487ca9c5b4f62` | mixed test sections      |
| `packages/open-bitcoin-chainstate/tests/parity.rs`                                                                                            | `b5f5324ff3e95afc2d7a61978a911f271c61d0805a5750733f1bc8d92fb45f12` | test scope               |
| `packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs`                                                                             | `b95ec57be535a2969533b9529f538bcbb2c528245bd91b8814d57490a8358c55` | test scope               |
| `packages/open-bitcoin-cli/src/operator/dashboard/model/tests/sync_and_recovery.rs`                                                           | `b9575a2780b352fcef84b172d74d416a90b328de55a285b6b0d38a13e2594ccd` | test scope               |
| `packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility/tests.rs`                                                                 | `4663d65914a5c860e2be7e15ef12f7ae056c80909b1ded9278fc6b6e239ec9f7` | test scope               |
| `packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility/tests/phase165_work.rs`                                                   | `1ba8622a4eb1282e71409ba8601b0cd798b01341b0f2409da40b161b3c4d6bfb` | test scope               |
| `packages/open-bitcoin-cli/src/operator/soak/tests/recovery_resume_bounds.rs`                                                                 | `38e44643f1f5656ad1150f21222936309726087e1cda383cdacbe007f5167c56` | test scope               |
| `packages/open-bitcoin-cli/src/operator/soak/tests/runtime/execution_and_checkpoints.rs`                                                      | `366c164a40c1cc70db94af2175bc3e526864a7e634342819c6c4c5607db88289` | test scope               |
| `packages/open-bitcoin-cli/src/operator/soak/tests/runtime/runtime_fixtures.rs`                                                               | `5fde90a1ecb2893958a48e6ad5db76c28ea99685e84e91ffb6701844ee03ae35` | test scope               |
| `packages/open-bitcoin-cli/src/operator/status/render/tests.rs`                                                                               | `8f7feee349b8663f4b99db69fffee84f420e483b259181f21778fd2158af546f` | test scope               |
| `packages/open-bitcoin-cli/src/operator/status/render/tests/inbound_and_sync.rs`                                                              | `dd0ff41fdac67dbc53913e4f7073a8e0b391ed836f40acedcdc66f5a8da2079d` | test scope               |
| `packages/open-bitcoin-cli/src/operator/support/tests/inbound_status_fixtures.rs`                                                             | `e679d121020f82f4ff4c7dcdb16a5ac188f9b9fcc14fb11764f0fd98442596c6` | test scope               |
| `packages/open-bitcoin-cli/src/operator/support/tests/sync_fixtures.rs`                                                                       | `c97147103df5ff292cab15f4b17dc4ad2dadf4c225224eaa525f5b7537d8c510` | test scope               |
| `packages/open-bitcoin-cli/src/operator/support/tests/sync_soak_forensics.rs`                                                                 | `59b28a4c9c11ee809dded3dff8f033b97c87ae34523c7832b0804fba48010f70` | test scope               |
| `packages/open-bitcoin-cli/src/operator/wallet/tests.rs`                                                                                      | `4319b078f29c6d8196f3d84736742a84456013752b392b6cbd5dc9554b5a8493` | test scope               |
| `packages/open-bitcoin-cli/tests/operator_binary/process_fixtures.rs`                                                                         | `fa7014e223b4dd65d11b94ee73ee5d3b286923f6d26db2bc74e511e9af8baa2f` | test scope               |
| `packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs`                                                                               | `d3b6c196c8ca8cca19d2ba64ae778e71b7207843bab7923a04781c1f008a99bc` | test scope               |
| `packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs`                                                                           | `fd0934bb72ed28362477092f6216b1aa4a78b093cce453d29f756e22f3276525` | test scope               |
| `packages/open-bitcoin-consensus/src/block.rs`                                                                                                | `a5219e5800c8b247afecbb509f481bc9d0cb8049ccd34fb48f10b07ea2bf8cea` | excluded production only |
| `packages/open-bitcoin-consensus/src/chain_proof.rs`                                                                                          | `9201f9f7604c66f6d3a514eb9f498ff658ebf7ec07e22fab04e311d7c55b785f` | excluded production only |
| `packages/open-bitcoin-consensus/src/chain_proof/tests.rs`                                                                                    | `ef7f17b25c47df0b49d481ac4386cba834a236f2c52abd64c6321c96731948c3` | test scope               |
| `packages/open-bitcoin-consensus/src/chain_proof/tests/phase165_vectors.rs`                                                                   | `abd0371668601b6e244068fedf05d00ce0e19340ef8de6955f34342d2425a2c0` | test scope               |
| `packages/open-bitcoin-consensus/src/crypto.rs`                                                                                               | `631710ae9083ab3a6acd21e923aba9fb052a016b70b2d4380f55530d01fb108d` | mixed test sections      |
| `packages/open-bitcoin-consensus/src/crypto/tests/phase165_tests.rs`                                                                          | `654623113c80f3cc0a71dc301ce3b805a407b7421db9ca7c21fb3d58582c33d9` | test scope               |
| `packages/open-bitcoin-mempool/src/pool/tests.rs`                                                                                             | `04162169077561c9e31d44134ecf972b803b31644b8269a593870a3a27be7cf3` | test scope               |
| `packages/open-bitcoin-mempool/tests/parity.rs`                                                                                               | `7ac351daaf97ab9ec02897952860a936ac94baef5235e7fe520e1336e6fb38af` | test scope               |
| `packages/open-bitcoin-network/src/filter_serving/policy/tests.rs`                                                                            | `c9cbb5eb2bb80f20dff1fb5dd562f5db83ec6cd4e407ecd73b1e55396bc3e228` | test scope               |
| `packages/open-bitcoin-network/src/header_store.rs`                                                                                           | `4955b6481396d6dcec0172921e8f10f1fcf4f4b7b655611a73a70f9fc79d9803` | mixed test sections      |
| `packages/open-bitcoin-network/src/header_store/eligibility.rs`                                                                               | `751b989eec04f97e280a0eccc2dc234a6fa413bd0804e5a3d1e802b0f120cc9c` | mixed test sections      |
| `packages/open-bitcoin-network/src/header_store/eligibility/tests.rs`                                                                         | `dea3cba79ff5c05e9752947798d1d909471b485bd65e34afa5a569e54e11a007` | test scope               |
| `packages/open-bitcoin-network/src/header_store/eligibility/tests/phase165_authority.rs`                                                      | `b157fdd575ff04f19d7df5c51cfd81dccc05253ee8a74bbdb2467ea4f6556151` | test scope               |
| `packages/open-bitcoin-network/src/header_store/eligibility/tests/phase165_cached_order.rs`                                                   | `7d4ae687ec178e401bf425bb43c92db70f5c17330e1a2070e44bc67e00723fb6` | test scope               |
| `packages/open-bitcoin-network/src/header_store/proof.rs`                                                                                     | `3fd34b9d5f0b48939e50a731a330065d84c427b12c312bb18da41e856b662d27` | excluded production only |
| `packages/open-bitcoin-network/src/header_store/proof/tests.rs`                                                                               | `39858dbe40d4beb551a29c8d7c2f1d2a29e359b1301efde99ab8b452424d3c09` | test scope               |
| `packages/open-bitcoin-network/src/header_store/tests.rs`                                                                                     | `93e1e1154e4eae7a3943b8156c9e87e06d1441cd880adfcc6eac14d6a9025275` | test scope               |
| `packages/open-bitcoin-network/src/header_store/tests/phase165_tests.rs`                                                                      | `1e05a4b55c2522ef66e08bc6a47b11398e7807ea9630e0f76124dda713880af8` | test scope               |
| `packages/open-bitcoin-network/src/lib.rs`                                                                                                    | `5f5250766167d46197f8f89f9355ac15cb97ae792f6935af1f2c7c5f58620fe0` | excluded production only |
| `packages/open-bitcoin-network/src/peer.rs`                                                                                                   | `4673cc6c9ab5d1885e304c20c5e0bf9ee67020330eea7d2f17a9177ee6f4a615` | excluded production only |
| `packages/open-bitcoin-network/src/peer/eligibility.rs`                                                                                       | `893c33ce182e4e9e5813a4a4be16f3e4f4ca1fcd5eb2a4dff59883a39aa78b85` | excluded production only |
| `packages/open-bitcoin-network/src/peer/message_dispatch.rs`                                                                                  | `53f2e79e42cf69072166e0b511009766d98d8fb83b561691548d7f3dfadf649c` | excluded production only |
| `packages/open-bitcoin-network/src/peer/tests/block_announcement_fallback_cases.rs`                                                           | `a5a08c1eb8555b13c9cfe101b1f15fd0239a41880ed6a5243f66a4a8039d3885` | test scope               |
| `packages/open-bitcoin-network/src/peer/tests/block_fixtures.rs`                                                                              | `4fcbf5c101d45ee679216bb3759f864fc12b98d90e4174d5d1a435d6ed0f8eee` | test scope               |
| `packages/open-bitcoin-network/src/peer/tests/block_sync_request_cap_cases.rs`                                                                | `ecdc36b13d67e0a87fa5b084440ce05ae5f25c90252617db42b7465fc2db75d5` | test scope               |
| `packages/open-bitcoin-network/src/peer/tests/block_sync_request_policy_cases.rs`                                                             | `125e768d8e4dddd8b54fe25a38598627564ebbed0efc1f71eaa000215e1b0f08` | test scope               |
| `packages/open-bitcoin-network/src/peer/tests/peer_action_cases.rs`                                                                           | `e2a485ad2e49b68cf5b393e41810c47b5c55a770b24a1fecacbcf146b32be7f0` | test scope               |
| `packages/open-bitcoin-network/tests/parity.rs`                                                                                               | `459eae10cf5244021c42476fc9080cae60d9a1c68e172ac26e1219957003f81a` | test scope               |
| `packages/open-bitcoin-node/src/chainstate.rs`                                                                                                | `1d25580fd120dd80c5129d065ae5e31da460392be9112f6c2ef767f2defb2ba8` | mixed test sections      |
| `packages/open-bitcoin-node/src/chainstate/filter_index.rs`                                                                                   | `9cf19c472ce8b25d0c3d0c33dc7e2465d689fe813333baeb50e558dced088c48` | excluded production only |
| `packages/open-bitcoin-node/src/chainstate/filter_reorg/tests.rs`                                                                             | `fdf3249d9f4bfbe5ac9a110675b0676941d2de64c3773cb8e4dd17c532c037a4` | test scope               |
| `packages/open-bitcoin-node/src/chainstate/filter_reorg/tests/faults.rs`                                                                      | `6de4474a52d4856a58d94b5001be91c679c3de020309ad7941eb2c1893788c10` | test scope               |
| `packages/open-bitcoin-node/src/chainstate/fjall_store.rs`                                                                                    | `57d814acd77d5f16741669ee2844f6e437410cd2f6dd999b839326d18f66d429` | mixed test sections      |
| `packages/open-bitcoin-node/src/chainstate/fjall_store/tests.rs`                                                                              | `2e94e38520387da17c325db62270b56d95ec7e2205efe835968670400d4a4772` | test scope               |
| `packages/open-bitcoin-node/src/chainstate/fjall_store/tests/fixture.rs`                                                                      | `69a4e4eb3eb24b64f6eed3b588fbf10418bb1432556c9941c4d8db465223e075` | test scope               |
| `packages/open-bitcoin-node/src/chainstate/fjall_store/tests/fixture/managed.rs`                                                              | `b91a18bb9faa231da62fa935a4f307b6a9ecc617f5b71b42172a1900b2abb37f` | test scope               |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs`                                                                                | `fddb8a4b1e730b29727842cc3ca743176ed24774dbc6b9e557de407aa525b04c` | mixed test sections      |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/fjall_sink.rs`                                                                     | `70f657e0b7cdaa5080e364e4878b51aaa7926315860bd666e0313eb5540b3645` | test scope               |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/prune_apply/tests.rs`                                                              | `57a10010fea74521a07ad01d1373a369058a467d2681580f521e0b7bc915cadc` | test scope               |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests.rs`                                                                          | `e854ece8a66141632e52d878b83aa2d033c4bd6894289484ff3c6a4fd3949f2f` | test scope               |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush.rs`                                                            | `0f659ca37d3ade0cbc44075bada76e072a3f2329ff8259c20c4fec43cddffb04` | test scope               |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush/header_checkpoint.rs`                                          | `d6368fa05c258b5be1804686e127b6a53b527934e00ecf6dd416660d084d0220` | test scope               |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/initialize.rs`                                                               | `7d795a9bd43746b8ebf5a417bff1e85af2e7e5093c4dd467969e58f1a13c9786` | test scope               |
| `packages/open-bitcoin-node/src/chainstate/live_fixture.rs`                                                                                   | `13ac5561c3e40eadc7cbb195c70dfd3efb6446af5509e38035e09e4062289a11` | test scope               |
| `packages/open-bitcoin-node/src/chainstate/replay.rs`                                                                                         | `2f3764dcb7a16eac00354cf22a1ea9c3c9376cb0b79e91ef0779267b74b3ffd1` | excluded production only |
| `packages/open-bitcoin-node/src/chainstate/replay/tests.rs`                                                                                   | `3c2f8ace82501a991915ef9c96ac4b0b122a611ac5af41f884dae60dec87fa8e` | test scope               |
| `packages/open-bitcoin-node/src/chainstate/tests.rs`                                                                                          | `a77c8ca532e9d14716adacbb8dd8f9bf786367a05da9a1502aef88a64c10e472` | test scope               |
| `packages/open-bitcoin-node/src/chainstate/tests/filter_index.rs`                                                                             | `710affcbc9848b91ec6f8c95fadaac7384626726902b315c165944c5b0ceaac3` | test scope               |
| `packages/open-bitcoin-node/src/chainstate/validation_history.rs`                                                                             | `de507925613e44fdea9cb5b3e8766c8aebc5070302a70db7fd59fd6c1dcbee62` | excluded production only |
| `packages/open-bitcoin-node/src/chainstate/validation_history/tests.rs`                                                                       | `84a9843591cf43d885c255149c2dd8af2adf48b5fdf89d3d241a0b17ad6b05a7` | test scope               |
| `packages/open-bitcoin-node/src/chainstate/validation_history/tests/coverage.rs`                                                              | `d609c385ee3f432abbcc9d9ebe874569dbab8f7c3cf05ce11a7530bf06f133b0` | test scope               |
| `packages/open-bitcoin-node/src/chainstate/validation_history/tests/fixtures.rs`                                                              | `f5b8403a5b2d179b974e3661db7a68be8cfdb27c5f37fc5f59fd145dd2aaa0a7` | test scope               |
| `packages/open-bitcoin-node/src/chainstate/validation_history/tests/network.rs`                                                               | `e1117611039a697ed11315f4f04852294da3e3fc51659e4c21017be7bf5b94d2` | test scope               |
| `packages/open-bitcoin-node/src/chainstate/validation_history/tests/phase165_work_authority.rs`                                               | `910688f61c3c4641eae86d0ca65d1e33591c5a3f96a748848819370536ab6a0e` | test scope               |
| `packages/open-bitcoin-node/src/chainstate/work_authority.rs`                                                                                 | `18fe2dc025deb1b556aa6570166e93d4951dc45e67b4ead69b99a12e255f9370` | mixed test sections      |
| `packages/open-bitcoin-node/src/mempool.rs`                                                                                                   | `4870cacb131adcdf4e9b8382622836deef1381600e07a06cf2f3e4d265541ba5` | test scope               |
| `packages/open-bitcoin-node/src/network.rs`                                                                                                   | `2a273fdf3b9278e4477fc0731ac338c2d40446bd45614d7248f46b63f32a89ab` | excluded production only |
| `packages/open-bitcoin-node/src/network/action_translation.rs`                                                                                | `92c8139de45c123ca759b6696ba70de41c6141ddec3e80e3f3738f4838cd5c0d` | excluded production only |
| `packages/open-bitcoin-node/src/network/candidate_eligibility.rs`                                                                             | `14e3334a388bdde9037fc1521a86e8a42d3ede428fc0fff8b4fe5be078b1ca1a` | excluded production only |
| `packages/open-bitcoin-node/src/network/canonical_admission.rs`                                                                               | `deb77a064624d2f0ecee6998849633c503aacf43ebd1106dd6adae7cdc409d19` | excluded production only |
| `packages/open-bitcoin-node/src/network/canonical_admission/tests.rs`                                                                         | `d4a12b2b790bef3de2b00ac33a70ed989bc66024f8ba646a927d6cba941dce7c` | test scope               |
| `packages/open-bitcoin-node/src/network/compact_receive_candidates.rs`                                                                        | `f28a565dc5895851d962d86b280bd289ebd397e61f72516f20b8b923d2c96a3a` | test scope               |
| `packages/open-bitcoin-node/src/network/header_sync.rs`                                                                                       | `dcd625d4dd241fb31f97d67d78ac8cd7886309f1927b91abb6fd4a1074564b8e` | excluded production only |
| `packages/open-bitcoin-node/src/network/inventory.rs`                                                                                         | `6eae72087d1cbf351576c08462351df2ec47c5ba271a45865b5965b6c684e7fc` | excluded production only |
| `packages/open-bitcoin-node/src/network/limited_serve.rs`                                                                                     | `3ef1325a9034e9445c5c87dd8522a4480b79f4a7aebc1da55abb7cc3ab335921` | test scope               |
| `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs`                                                                                 | `fa186eeafab8925c40006f7a20671c265af261a364698712b751562b6baa8bbb` | excluded production only |
| `packages/open-bitcoin-node/src/network/peer_network_clone.rs`                                                                                | `9afc6882d9ea027287f09513554f5679af12daa64de927a76ecc65352cdbbadb` | excluded production only |
| `packages/open-bitcoin-node/src/network/relay_serving.rs`                                                                                     | `87c1976f801bc4ddd25757ab608597ace0c574e970e8199f8ea6733274a37325` | excluded production only |
| `packages/open-bitcoin-node/src/network/runtime_authority.rs`                                                                                 | `65835d0307dd58ee1572efab8a8eb83794cc210c1748ad87f5218bb3cf082212` | mixed test sections      |
| `packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests/fixtures.rs`                                                  | `d2e193be731e06c9c219a0e58efa1497093a7f4f50282ad411a44cf944e1d7d7` | test scope               |
| `packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests/protection.rs`                                                | `ca9864497314e615af3a66dc5ffbd68b6aa3437d1828f13a972ca26985fa85ac` | test scope               |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs`                                                                    | `dd05bf16a0e4be7cbff5f4d05f5ba25984ed2fa9e366a1d0e2efa95a190508a6` | test scope               |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch/tests.rs`                                                       | `9915768162cb82758109fa6096e0fdff7b35a0adfeab19c268211c10d318c441` | test scope               |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs`                                                           | `d1afde6b9d01c22a2adf253945f3b5b0f6ed55d9974842aaf73cd9ae9203a621` | excluded production only |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/tests.rs`                                                     | `aa8286cbca8b9336217764562d5be181bd13556d4b7ea1d314d4b03d8d623fd9` | test scope               |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/query/tests.rs`                                                        | `9140f8c10abc5b86f2ec1d84eb528d1a8f320872e4e4c21f82d18d74f4720d16` | test scope               |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests.rs`                                                    | `bae3782a960294bbff64950dfeeb397cdd646877247189adf9ddd85bf277fb02` | test scope               |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests/failures.rs`                                           | `07bc502f429541f32b8969b2d1ab34d9a424445daf98ed638d7e11d49793ec58` | test scope               |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests/interleavings.rs`                                      | `80df8a74c64e197c459a7588e75f6af2ef56339138b753c7403cc8763c71626a` | test scope               |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/reorg.rs`                                                              | `0919a19fa34d573d21b3d1b4e8b17f10f49353f1f9b93b93f1ed8113fa88b85c` | excluded production only |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/transport/tests.rs`                                                    | `055e5bdcaf3d290ee59ea580d9ff4c15cea261a7baffab435ee114b8dbbdd2fb` | test scope               |
| `packages/open-bitcoin-node/src/network/runtime_authority/live_fixture.rs`                                                                    | `5fb001358e2938b00cf50f89335dcc40f099135f3b6649754c902f779306918a` | test scope               |
| `packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs`                                                                     | `262bbe4a989e96309282ed026cd82ed53d6b664b960024f3fd197d1ff747cecb` | excluded production only |
| `packages/open-bitcoin-node/src/network/runtime_authority/tests.rs`                                                                           | `43006899ee0a3c8eeca79a86646a259c8d217acccb6d2b2a50afd5fa03d4afb0` | test scope               |
| `packages/open-bitcoin-node/src/network/tests.rs`                                                                                             | `c4be346fb6c1f62a68e7f1cdc2f3b2c1dde78ea5f1b657abd8ecceb1de6a884d` | test scope               |
| `packages/open-bitcoin-node/src/network/tests/block_connect_disposition.rs`                                                                   | `058a37d3d501e740e059fff6c6d5468b5648dc9917a575c4b845bc2ba6936d96` | test scope               |
| `packages/open-bitcoin-node/src/network/tests/block_serving.rs`                                                                               | `4456f75e8726a6a9b0b1c96377c97e5f72ff278869cb03a1666a7b9e4fe347f8` | test scope               |
| `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/maintenance.rs`                                                      | `cc9cba526f4d1b5c7740506228a7880570ca1f85cfe4fe1ccfaa43615532b27f` | test scope               |
| `packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases/reorg_reject_evidence.rs`                                               | `69a417891f12f61df83745e74d5e1891d5bbb577b56d2c30c8fbafc326591b25` | test scope               |
| `packages/open-bitcoin-node/src/network/tests/phase165_eligibility.rs`                                                                        | `141042a5aa3e705bc329e04db0763350395e769383642df5c0bf6c84dcf4cda4` | test scope               |
| `packages/open-bitcoin-node/src/network/tests/phase165_headers.rs`                                                                            | `913dc6674074c8ee9f86f10696d294ef0ebf6bd72ca78e8ecd7467faf837b38c` | test scope               |
| `packages/open-bitcoin-node/src/network/tests/phase165_rebind.rs`                                                                             | `387f0e02c0991efa4dc6351a9a9c471a7109ca073943c6cc5b8acf18dc4a9629` | test scope               |
| `packages/open-bitcoin-node/src/network/tests/phase165_work_authority.rs`                                                                     | `26cdf05da44af6d10482f6163651ff790fa128f1deedd3330a34cf06852e3979` | test scope               |
| `packages/open-bitcoin-node/src/network/tests/phase165_work_authority/fixtures.rs`                                                            | `ab0eec5a42af1f5ffe8bb8cfaff9cd011911890a5839a8d4d3b3616731285c61` | test scope               |
| `packages/open-bitcoin-node/src/network/tests/phase165_work_authority/freshness.rs`                                                           | `f20173e226b2fccf614ed24d7a1a876f0150911a6470134f6446fb239cd7ff25` | test scope               |
| `packages/open-bitcoin-node/src/network/tests/phase165_work_authority/history.rs`                                                             | `63a7bd3e74120a3b450cddf5bf2839ab19e743818d8b00fefe98fa7fcc170839` | test scope               |
| `packages/open-bitcoin-node/src/network/tests/runtime_projection.rs`                                                                          | `7d25d6927a2cac24ec90a13e3b858b60f040bf99016f0ef9c15647d281394de4` | test scope               |
| `packages/open-bitcoin-node/src/network/validation_history.rs`                                                                                | `b2a9e5d489797818dada897d8af885c825b1ffa5cd5dac241ca608901f8f3e89` | excluded production only |
| `packages/open-bitcoin-node/src/network/work_authority.rs`                                                                                    | `a0952b43f0f4e6eb598421abfb6e6c2e8d01d30ebbb224ba80bd8c6fe8f71447` | mixed test sections      |
| `packages/open-bitcoin-node/src/status/tests/progress_guarantee_and_reorg.rs`                                                                 | `39f5c50a3384528d56fa7ab5b4ce89850ea8259d5962136d8d7b5f4cea220e19` | test scope               |
| `packages/open-bitcoin-node/src/status/tests/snapshot_projection.rs`                                                                          | `d4a19da5670cef0358efdffde7fcd3baa14a8e1af5e7be8320a6f256374bfaab` | test scope               |
| `packages/open-bitcoin-node/src/storage/chain_compatibility.rs`                                                                               | `37b3763d3cdfed99e60d11bf7ac705fcdcfb933934f5640a52033146d029370f` | mixed test sections      |
| `packages/open-bitcoin-node/src/storage/chain_compatibility/tests.rs`                                                                         | `455a7fb20a6791aa15fdfc3283692ee6124bc0e105434ebae21a7e45f2b34dc0` | test scope               |
| `packages/open-bitcoin-node/src/storage/chain_compatibility/tests/phase165_work.rs`                                                           | `92cd82ddc6e16814f3c42957a3411b406b4492585189ace252c001f5588430a3` | test scope               |
| `packages/open-bitcoin-node/src/storage/chain_compatibility/tests/preservation.rs`                                                            | `e6cb5740cb5c83655aed14c43b1206847d264431d944741a365771c3e8d76f5b` | test scope               |
| `packages/open-bitcoin-node/src/storage/coins_view.rs`                                                                                        | `f71ff78ac7a7c4e65d657964a7f415e3c11e9684388fb4456c93a974f1c5ee56` | excluded production only |
| `packages/open-bitcoin-node/src/storage/fjall_store.rs`                                                                                       | `033e588083230861e17e4fa8ce2aee86198616ecf5a3a23779eb4d296291dba1` | test scope               |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs`                                                                     | `ae563e872349666dee2a27075806327afe769cbb70d14778dd41124b020eb78d` | test scope               |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs`                                                                         | `f87ae3fcb97304959b325e9b1b2248848d83da4a7979f2aa54eba5f0724e027d` | test scope               |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs`                                                                         | `516e03132d17e15fe7ad4dc02f801ac49a711bffc2fc6b31ed580ef98bb298c9` | test scope               |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append/fencing.rs`                                                          | `fc58454dfcd883fd0b0980bda09cf31b8da8f719a045ba9f69b384b2bedee119` | test scope               |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/ownership.rs`                                                               | `d5e5fae9d988224a8f4ce9dcf6d7b113094b04fc5a8fac32b536864561fa7748` | test scope               |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg.rs`                                                                   | `a13a478cdaae20f3234c58a9bb5b6152a3c923a3024a8c521e6da427fb7fb944` | test scope               |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/retention.rs`                                                               | `bcd26a343ddc32d0d48e07846f9a8d13dfe9f188b09fe870f577b048c541d769` | test scope               |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/retention/publication.rs`                                                   | `33ddd24f9771d98f14418247eaa23184bf60ff8e15e051d6fb6dd3694f02fdd6` | test scope               |
| `packages/open-bitcoin-node/src/storage/fjall_store/payload_usage/tests.rs`                                                                   | `2a31f5ffe4574c78ce0f77805c46018c0f88698f89019ac7a0d4e1319076d760` | test scope               |
| `packages/open-bitcoin-node/src/storage/fjall_store/tests.rs`                                                                                 | `dedb807d23411bffacc31aaa0c0e53a4ef580c6b81f0681ed76277943cc29757` | test scope               |
| `packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs`                                                                 | `19f05f503375bb7df0a256cbc8dca31e40935464d861982c5bb9b315c7847050` | test scope               |
| `packages/open-bitcoin-node/src/storage/fjall_store/tests/prune_records.rs`                                                                   | `f2faf9ff3b17b4b70ed892fe466bf31c46523599ae308deda2e14dc620ea9a3b` | test scope               |
| `packages/open-bitcoin-node/src/storage/fjall_store/tests/prune_unlink.rs`                                                                    | `82491ab87310cb0c3520e1e270733f861b64e9d585b9efa66c60df870f6d8eaa` | test scope               |
| `packages/open-bitcoin-node/src/storage/fjall_store/tests/prune_unlink/protection.rs`                                                         | `3ba2e08467e40a7a4981b8e7c25d61bd2ba96306a3cf58382a250143aa9029de` | test scope               |
| `packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/confirmation_migration.rs`                                     | `813c619a42bebcc70b1919456e66bb2fbaa73c4665e04705c79727f7677d7c50` | test scope               |
| `packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/reopen.rs`                                                     | `7cf22cd67ff143cae696fb74f1c421fec2ddc0c66e95c7432479b5291d291a66` | test scope               |
| `packages/open-bitcoin-node/src/storage/snapshot_codec.rs`                                                                                    | `2fefee5664d1f447a649b864362aa04f6594195088c6623ef5e94a8bbd0e98f6` | excluded production only |
| `packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs`                                                                              | `0de7c7a8f9dccfc3b78e5de7b47a84ec141ecd967eca7222002ec254a55f6435` | test scope               |
| `packages/open-bitcoin-node/src/storage/snapshot_codec/tests/chainstate_wallet_and_headers.rs`                                                | `9cd6c31afee8d94beb6e0524f20842511aa8fdb02fd24483e90b50737b63fea7` | test scope               |
| `packages/open-bitcoin-node/src/storage/snapshot_codec/work.rs`                                                                               | `b7d2fc5e127e523ea05fc3aee33657f3466a1b6f7a234221e209047ad310d5cd` | excluded production only |
| `packages/open-bitcoin-node/src/storage/snapshot_codec/work/tests.rs`                                                                         | `ae92474d93ee2cfcf0e83ab530480c9a0a989e15caea3e920caef04ed39a9977` | test scope               |
| `packages/open-bitcoin-node/src/sync.rs`                                                                                                      | `88cf2a12bca8a236a4a8d821afe96e17cdd13a63c5ff8f159da5d24e3dfbe035` | excluded production only |
| `packages/open-bitcoin-node/src/sync/block_reconcile.rs`                                                                                      | `96341ba6541be2f3ed99d7a96745dbd88ce6ed254e4f5c01059fc4c21e2d70f0` | mixed test sections      |
| `packages/open-bitcoin-node/src/sync/block_response.rs`                                                                                       | `718f8eb2fa0e552cb26fd43e62cd92ade6145973fc96b957885d54121060d11c` | excluded production only |
| `packages/open-bitcoin-node/src/sync/live_fixture.rs`                                                                                         | `4da004e8b12a1b6bd0da8d460dde94b70a7c1119893fbe03ba2d048d707b9b9c` | mixed test sections      |
| `packages/open-bitcoin-node/src/sync/open_runtime.rs`                                                                                         | `e0d80123609b1f85a9edb06c18f1c1682268215bace22f343a2c420357c338d5` | excluded production only |
| `packages/open-bitcoin-node/src/sync/runtime_state.rs`                                                                                        | `610ad571f62023920b08666a5a0694aa081af610ebf1e637efd6bfc747007c25` | excluded production only |
| `packages/open-bitcoin-node/src/sync/runtime_state/tests.rs`                                                                                  | `56c5fbf0fbb5a256b9b1da1473c2ad32e24b30f31d2773fca99256c2c34bf667` | test scope               |
| `packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint.rs`                                                                | `e449aeed8ae0a65e23e7e654a20cccd4cc8609f982e2008b48aa372dbb777e8d` | test scope               |
| `packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint/faults.rs`                                                         | `418404faac133e33635924f4881458a73bc9402f50df1d414dfd571ed12373ac` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests.rs`                                                                                                | `a9b4b57990b015cee3909013834862c98e234ec121b7bee7467a460fdee21732` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/block_requests.rs`                                                                                 | `af81fb3a8d0060734decdce10d787e3017aae6a1dd64ea1df97f3f7672ecefd9` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/block_response/connection_progress.rs`                                                             | `5665b99089778826eead93420924bab58e3f441010469f00f08672110522712c` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/block_response/peer_tip_evidence.rs`                                                               | `a7497d7b4dd40b3d7c436e7e7ed53458b925dbd3a11c818254d81763bc905208` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/block_response/response_failures.rs`                                                               | `044f80175cb2ecebdf343c8e9fdd5dae54b4641f52badda38b7d6b0b270aa152` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/durable_tip.rs`                                                                                    | `4293c6da937a9df88a77532bcc1614d42a434f7a72f0960b9044d5d78fcc7286` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index.rs`                                                                                   | `ef891dd3429f628213f51d7ac234fbd1078a7074b5813f8c533d0f1ae3386fcc` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/accepted.rs`                                                                          | `5a261b2a919c7b0022166bd727296947ef9faf04111b3c0436306e2c1afb9d0e` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs`                                                                          | `d906e48ba644026f7c9905e27b2812089f7011a24e3f05d0b641389dae10cac6` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/failures.rs`                                                                 | `7bcf72e05d87049e85eb7d0a48c1ab7ab109a381a17a22d2ac45c5b5daa757d2` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/fixtures.rs`                                                                 | `93d6cb4e7d622083c8fff4dc7fd8c042b41efde895fe9f00288c45b5a56fb568` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/rpc_faults.rs`                                                               | `526debe0db3ea972f982f1b0ac88fb7c77dcd38f61831abb1b4fd0a7ac8f8d7f` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/accepted_faults.rs`                                                          | `40048b2bde5f3a1a2907b2d5e6182d9b79c39c2b9d8f60cabdb41357f34ac944` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/retention.rs`                                                                | `4ab8ee124dfd42263727b936491647cbe8d918ba00a7f6626a295d11d6a33426` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/operator_status.rs`                                                                   | `1e4c42611ee0c667c949012819d93046d07dba038c574d9a7928ea7ea72bc01d` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/peer_evidence.rs`                                                                     | `28ccf9d2294ee76ede6f383180fd220689874164138b0b7843b373f4c25be2d3` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/peer_fixtures.rs`                                                                     | `b3c194131c7133f9deadf001af1334ec660621e0f0c9a45a1f2f86ffabfbcd90` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/peer_fixtures/phase165_source.rs`                                                     | `5240131e624d9b04009b4970cc687f531e507874d0c6f24a299418e2cd5ba7ca` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs`                                                                          | `677a3a5def6d6be380454ee029bfecf18b1d0de0fd3a25a0248d7f9ee4638160` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/publication_faults.rs`                                                       | `3001c1bad93739a7e84e1fc9aea1d43a0634c2eb9882bc0d5f8a5abcaaa6e813` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/recovery.rs`                                                                          | `8f88c68e333ac1c0391c5d1d27ec24e245bcaf3cac083fe54da6fb1e2e5cabd8` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches.rs`                                                                    | `8e384b3b398d0ad7c10872858ac2a70906e352c253c6194230cff099b0ce55e7` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches/manual_lifecycle.rs`                                                   | `34de4e6cf2de803550da7bd16cd8cae52fd288e176b53987e77ac6aa6725e6c1` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/failures.rs`                                                                    | `ed0f8046f5664b536fc450841f4e0ccc65818884a5e3a975347183009dd21743` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs`                                                                    | `f2adb79ef209ab8fc63897ba6816e5201fb1c437d6d278aa787cec3a05ebe8b0` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures/live.rs`                                                               | `4e5376a4ff1b654f9532810d59d6d903cbc1e6ec7093e06b15f3108952327dcc` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures/return_branch.rs`                                                      | `6d6fa1911ea2263da025f1374d5fe9d1ef8f374eb97dc9313688a944e1099921` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/measurements.rs`                                                                | `3e17ca8ab2e45f2b31f4fc45daf3e68db7e9a678ca728beca740a748145bc007` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/protection.rs`                                                                  | `5df429f474195bc120b2fd5ea5db4f4e05bf559956e0416c5c9d4a189bab98e8` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/retention.rs`                                                                   | `2f13173c2aee79bb080adadfb494f59cf691838d10278a85c81c8d0a689a5951` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/startup/recovery.rs`                                                                  | `5b2273f8445aa056f96a3a37ab05b820e37134fa86352e4472905cb7fa25e54a` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/inflight_failures.rs`                                                                              | `bcd897d7a89429a0921ec339bb67b805b9d3bb0ca8f2fa189006b4d335b284b6` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/no_progress_diagnosis.rs`                                                                          | `4e1dfdd2bb5485c31217860499b5fb0e35010e0ca7009a12857c12e6b7cd465d` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/phase162.rs`                                                                 | `c35b11b189473843d2c34b95a75109c30bb064925019f8bf916c56fdf9241d39` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/wire_goldens.rs`                                                             | `a1e8b66a2f2c26a8b9995df947a4993cd6e8bda80a2f23b4a5227f01d6fcc1c4` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/peer_resolution/resolver_rotation.rs`                                                              | `44632b985da2d5ea00988aac12171fc369d34f135288aa61d7c7d8f478ad437b` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/phase158_preflight.rs`                                                                             | `6aae5f2c1e04139a0a464b396ae08430472e781cf7710bd670f070e49151b92c` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/phase165_candidates.rs`                                                                            | `acf61dc2106f647f99dbf950acfa7c446393bd90412646637910f4f5827dcb8b` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/phase165_candidates/fixtures.rs`                                                                   | `3a18443d0a228843d7f240c0c3db4a2791c01ae64664fd0f9a11032c6d37edd2` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/phase165_live_fixture.rs`                                                                          | `2768f24bf56b51acd8186a2e92eeb7feb21f7482e85c678f561ce03d75aba8f8` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/phase70_peer/failure_classification.rs`                                                            | `78d7020082fa177dc1c70b3731e440f04bf63a8066b07cb7a3ca6408765e44b4` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/phase70_peer/fallback_rotation.rs`                                                                 | `353567ef09b01758c751f7a2970a7fbbb873700e8f8507435a0b5eb137499f89` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/production_announcement_transport_cases.rs`                                                        | `669c1a6baf4ad3ee87bde3f00a97fd4dfe57c079c486c2f6d9c312ea88522e2e` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/reorg_reconciliation.rs`                                                                           | `0f0be100c432bc0d3b13ade42f34d240fd7fedc47f2e5c8f3f9b83e7c5e4b1f4` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/restart_and_recovery.rs`                                                                           | `2718625ec54f78271fa42d0f2529b9e4c1bbfe758023609427acf7a77d793a54` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/restart_chainstate.rs`                                                                             | `1aa1b5d75c2bb3007abbad68324ecfbc0d34f1d0b7c632410043f362ecc8e7b5` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/restart_chainstate/download_and_invalid_body.rs`                                                   | `cee8a5d7c4935e6404b5377ebb395acae1d7369f6fa1dc0ce5d111281c77a581` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/restart_chainstate/persist_and_hydrate.rs`                                                         | `54f840da0bff667fb57a3ab4bfe034774b335d21f39bda9047d6b9b18eb842e3` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/restart_resume_matrix.rs`                                                                          | `7fb23c23a06243af9a39bfb45f0f0451b31b7a9ced9b5e201eb3552095030310` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs`                                                                       | `525816c156cdf7e451e1e3877b11101cb71d534948fa9524c38ce80736840dce` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs`                                                                           | `d257b0180645de9b41262ea8fbe79970dfb16ce1994a8d054f953f6c275afd33` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/soak.rs`                                                                                           | `a060815ad8c2d5cfc71dc4d42e9d4aee97b4ef18f41b545c250adeb418cbc76c` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/status_projection.rs`                                                                              | `0e00fd0de9c06bd2136576b5288d89aef379b365a2f81545c34b2b7f228c5f0e` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/stay_current_persistence.rs`                                                                       | `3d6840b8c85cc25b3d9b7e6cd3a4228ed0b363818d49d4b19b94d0df18606b2a` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/stay_current_progress.rs`                                                                          | `43322767cdea3f93b90ef685b6d8f2177c850a6ff533f3162153749288760cdf` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/structured_runtime_logging.rs`                                                                     | `6bc90c323d7061afa408f050a9b9453999aaf61d378c65c132800e462b738935` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/support_blocks.rs`                                                                                 | `e2c553577d65bfe108975d138f99cb72a9ef6a2a81c86c95941027f05636a2f7` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/support_runtime.rs`                                                                                | `5a25857c552bb7dcfdd040ad15e19c4616f52ef8b35df69b4f11a42dcd5b1f05` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/sync_cycles.rs`                                                                                    | `0cc1856646f04a3a1e4f1cd308b3ffe40e444643fdfb180fd269fa0d77f8199e` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/synthetic_long_chain.rs`                                                                           | `3acf6932eca1783fa3e74cbd3840d08b77c28d6ff386ee0f655f1469184749a7` | test scope               |
| `packages/open-bitcoin-node/src/sync/tests/wallet_rescan_runtime.rs`                                                                          | `2bc4bd96e848413d3b6901eb79957e583968ed834072b1d52061344ef27e8ce2` | test scope               |
| `packages/open-bitcoin-node/src/sync/types/error.rs`                                                                                          | `18967e0b9b4c774c4d67e9c74712b37135203dd49c448096cabefb6ed340390c` | excluded production only |
| `packages/open-bitcoin-node/src/sync/types/summary/tests.rs`                                                                                  | `3d01d113435af23e31f899270595261438bc192465223407bd383c96f1484018` | test scope               |
| `packages/open-bitcoin-node/src/wallet.rs`                                                                                                    | `6185397f395ccde53880b31c122b59b614a62596cc015c4d6599cf09111318cf` | test scope               |
| `packages/open-bitcoin-node/src/wallet_registry/rescan/tests.rs`                                                                              | `c49dcb2d8b376eeb3fe2a4ac142ea3d893bd1bd6ca3317fb8a8767bd4f8e5f73` | test scope               |
| `packages/open-bitcoin-node/tests/phase165_live_fixture.rs`                                                                                   | `1c38f95572bae47ef45753fe3990d1517142d05821498c91bcae1599566f9ca9` | excluded pending 24/28   |
| `packages/open-bitcoin-node/tests/phase165_live_fixture/requalification.rs`                                                                   | `881fba3b2d7581e34077971767da9ab2e4c12a99130546abc6b1e203399f547a` | excluded pending 24/28   |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs`                                                                                    | `7e0a7c3e364fe7c4818e3404ec3fa8151e866815f348c66feaa3b7220341b712` | test scope               |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/fixtures.rs`                                                           | `965d7ab14dd929a99961b56c047d995d523511242351db01e491e44e5fa6801e` | excluded pending 24/28   |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/index_protection.rs`                                                   | `4f0e301c0ebb1932fde8dd3a08cc11c50e9ff50012fd3b615c74f77587514ebd` | excluded pending 24/28   |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/canonical_chain.rs`                                                                    | `46816c709e1ef46c1ae45d7918450942365254343c32c41c890b7354e40d10b5` | test scope               |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/daemon_sync.rs`                                                                        | `3345bbc4ae06d9ff9c270702cb64f56570b80500b0b3465aa95283e1e221cee9` | test scope               |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs`                                                              | `db1b79b1254029e3f2816a10359c65f707c13df91c45530c13677325a84640ae` | excluded pending 24/28   |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/operator_evidence.rs`                                                     | `aaae166c4ae00f0530b9ee383f543a8cef2d3ae1e5913d23d34013f83e1e9136` | test scope               |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/peer_serving/evidence.rs`                                                 | `57ef0c13471db770cb491d1949780730317e91ba82e4a5ecb38620ec3ef67199` | test scope               |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs`                                                     | `2d0a7e7d93034333e00f5c90977693bcfb7a03ff8ab704bf834d91fec680d307` | test scope               |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/inbound.rs`                                                      | `d89e0e47cd23861c17706ab36be8d6efb92528fa6e82f9ddfd8a731d774652ca` | test scope               |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/lifecycle.rs`                                                    | `d68dcc1f9a794b91441d3fe9686a55f1a23f94d4d31927059fc6965b1f4a91c2` | test scope               |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/rpc/retention.rs`                                                         | `b14247ea3c16cc99859cb38a4e746eddb1c5b842632f3b81d13f006a37512ea9` | test scope               |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/phase165_work.rs`                                                                      | `1028a3e30137f430bcc207d87c24c8e774615b1cfd0f5278aa1f79a5e9d8313f` | test scope               |
| `packages/open-bitcoin-rpc/src/context/tests.rs`                                                                                              | `72dd1502dec9a6c036fff69c466ea29b706fae35d62919187a5def02a4465b70` | test scope               |
| `packages/open-bitcoin-rpc/src/context/tests/construction.rs`                                                                                 | `1c81a53545b6330e9147945c81b35d96521bf2f0a450042451f1aa37b7663d73` | test scope               |
| `packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility.rs`                                                                           | `24603e5747f842872b86d6b987a6731df24de7402647255e53954259c4203afb` | test scope               |
| `packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility/fixtures.rs`                                                                  | `ab854ca7d9eb6a3806a9ed17f00749f3b1bb8da3de3a101d680c38e4a850dbd0` | test scope               |
| `packages/open-bitcoin-rpc/src/dispatch/prune/tests/ownership.rs`                                                                             | `e39d948ec2ca069fb8c8877e1d678f7f6fda8c388b404776c1fe975fe334d6eb` | test scope               |
| `packages/open-bitcoin-rpc/src/dispatch/tests/chain_fixtures.rs`                                                                              | `6330eeb79c8209d3f035e4169ae068c42d84d7465276c1c3f0d829bb13150182` | test scope               |
| `packages/open-bitcoin-rpc/src/dispatch/tests/sync_status.rs`                                                                                 | `571a8e9e5888da928eb4246f5c5f66a7657c5930f912962bda53abdde42b85e1` | test scope               |
| `packages/open-bitcoin-rpc/src/http/tests/filter_index.rs`                                                                                    | `276c6d6855e9c467511a09902a34d8ae227b4d59f39ec92c15ba37a823c6d9c5` | test scope               |
| `packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs`                                                               | `4c8250c11f1a2654b5c70c6c7594f238b5b68777a35992b6bb15933ff1e9951b` | test scope               |
| `packages/open-bitcoin-rpc/tests/phase165_work_startup.rs`                                                                                    | `53bc8a5159f44e069da2fadbbff3321def1f2a2ad65ced460c220b9dabaa1c25` | test scope               |
| `packages/open-bitcoin-rpc/tests/support/phase165_live_source.rs`                                                                             | `51d11bc50395c7b1b074f936c77752e1b7f997652dfc42cca03b990da8368248` | test scope               |
| `packages/open-bitcoin-wallet/src/wallet/tests.rs`                                                                                            | `0802602bbd23d79c8d42f6c5888af378795da01be836842361a9b095d0fa3b03` | test scope               |

______________________________________________________________________

_Reviewer: independent gsd-code-reviewer. Preliminary fixture audit only._
