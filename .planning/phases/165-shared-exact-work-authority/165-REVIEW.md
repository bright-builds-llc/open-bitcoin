---
phase: 165-shared-exact-work-authority
reviewed: "2026-10-18T13:04:25Z"
depth: deep
files_reviewed: 344
files_reviewed_list:
  - README.md
  - docs/architecture/status-snapshot.md
  - docs/metrics/lines-of-code.md
  - docs/operator/runtime-guide.md
  - docs/parity/fixtures/phase165-work-vectors.json
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - docs/parity/v2-6-canonical-chain-identity.md
  - docs/parity/v2-6-shared-exact-work.md
  - packages/open-bitcoin-bench/src/cases/chainstate.rs
  - packages/open-bitcoin-bench/src/cases/network.rs
  - packages/open-bitcoin-bench/src/cases/storage_recovery.rs
  - packages/open-bitcoin-bench/src/cases/sync_runtime.rs
  - packages/open-bitcoin-bench/src/fixtures.rs
  - packages/open-bitcoin-bench/src/runtime_fixtures.rs
  - packages/open-bitcoin-chainstate/src/block_filter/tests.rs
  - packages/open-bitcoin-chainstate/src/block_filter/tests/fixtures.rs
  - packages/open-bitcoin-chainstate/src/block_filter/tests/validated.rs
  - packages/open-bitcoin-chainstate/src/coins.rs
  - packages/open-bitcoin-chainstate/src/coins/cache.rs
  - packages/open-bitcoin-chainstate/src/coins/memory.rs
  - packages/open-bitcoin-chainstate/src/engine.rs
  - packages/open-bitcoin-chainstate/src/engine/overlay_apply.rs
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
  - packages/open-bitcoin-chainstate/src/engine/work_authority.rs
  - packages/open-bitcoin-chainstate/src/error.rs
  - packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests/durability.rs
  - packages/open-bitcoin-chainstate/src/filter_index/tests.rs
  - packages/open-bitcoin-chainstate/src/lib.rs
  - packages/open-bitcoin-chainstate/src/types.rs
  - packages/open-bitcoin-chainstate/tests/parity.rs
  - packages/open-bitcoin-cli/README.md
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
  - packages/open-bitcoin-consensus/src/block.rs
  - packages/open-bitcoin-consensus/src/chain_proof.rs
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
  - packages/open-bitcoin-network/src/header_store/proof.rs
  - packages/open-bitcoin-network/src/header_store/proof/tests.rs
  - packages/open-bitcoin-network/src/header_store/tests.rs
  - packages/open-bitcoin-network/src/header_store/tests/phase165_tests.rs
  - packages/open-bitcoin-network/src/lib.rs
  - packages/open-bitcoin-network/src/peer.rs
  - packages/open-bitcoin-network/src/peer/eligibility.rs
  - packages/open-bitcoin-network/src/peer/message_dispatch.rs
  - packages/open-bitcoin-network/src/peer/tests/block_announcement_fallback_cases.rs
  - packages/open-bitcoin-network/src/peer/tests/block_fixtures.rs
  - packages/open-bitcoin-network/src/peer/tests/block_sync_request_cap_cases.rs
  - packages/open-bitcoin-network/src/peer/tests/block_sync_request_policy_cases.rs
  - packages/open-bitcoin-network/src/peer/tests/peer_action_cases.rs
  - packages/open-bitcoin-network/tests/parity.rs
  - packages/open-bitcoin-node/README.md
  - packages/open-bitcoin-node/src/chainstate.rs
  - packages/open-bitcoin-node/src/chainstate/filter_index.rs
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
  - packages/open-bitcoin-node/src/chainstate/replay.rs
  - packages/open-bitcoin-node/src/chainstate/replay/tests.rs
  - packages/open-bitcoin-node/src/chainstate/tests.rs
  - packages/open-bitcoin-node/src/chainstate/tests/filter_index.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history/tests.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history/tests/coverage.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history/tests/fixtures.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history/tests/network.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history/tests/phase165_work_authority.rs
  - packages/open-bitcoin-node/src/chainstate/work_authority.rs
  - packages/open-bitcoin-node/src/mempool.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/action_translation.rs
  - packages/open-bitcoin-node/src/network/candidate_eligibility.rs
  - packages/open-bitcoin-node/src/network/canonical_admission.rs
  - packages/open-bitcoin-node/src/network/canonical_admission/tests.rs
  - packages/open-bitcoin-node/src/network/compact_receive_candidates.rs
  - packages/open-bitcoin-node/src/network/header_sync.rs
  - packages/open-bitcoin-node/src/network/inventory.rs
  - packages/open-bitcoin-node/src/network/limited_serve.rs
  - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
  - packages/open-bitcoin-node/src/network/peer_network_clone.rs
  - packages/open-bitcoin-node/src/network/relay_serving.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests/fixtures.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/automatic_prune/tests/protection.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/query/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests/failures.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests/interleavings.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/reorg.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/transport/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/live_fixture.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs
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
  - packages/open-bitcoin-node/src/network/validation_history.rs
  - packages/open-bitcoin-node/src/network/work_authority.rs
  - packages/open-bitcoin-node/src/status/tests/progress_guarantee_and_reorg.rs
  - packages/open-bitcoin-node/src/status/tests/snapshot_projection.rs
  - packages/open-bitcoin-node/src/storage/chain_compatibility.rs
  - packages/open-bitcoin-node/src/storage/chain_compatibility/tests.rs
  - packages/open-bitcoin-node/src/storage/chain_compatibility/tests/phase165_work.rs
  - packages/open-bitcoin-node/src/storage/chain_compatibility/tests/preservation.rs
  - packages/open-bitcoin-node/src/storage/coins_view.rs
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
  - packages/open-bitcoin-node/src/storage/snapshot_codec.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/tests/chainstate_wallet_and_headers.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/work.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/work/tests.rs
  - packages/open-bitcoin-node/src/sync.rs
  - packages/open-bitcoin-node/src/sync/block_reconcile.rs
  - packages/open-bitcoin-node/src/sync/block_response.rs
  - packages/open-bitcoin-node/src/sync/live_fixture.rs
  - packages/open-bitcoin-node/src/sync/open_runtime.rs
  - packages/open-bitcoin-node/src/sync/runtime_state.rs
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
  - packages/open-bitcoin-node/src/sync/types/error.rs
  - packages/open-bitcoin-node/src/sync/types/summary/tests.rs
  - packages/open-bitcoin-node/src/wallet.rs
  - packages/open-bitcoin-node/src/wallet_registry/rescan/tests.rs
  - packages/open-bitcoin-node/tests/phase165_live_fixture.rs
  - packages/open-bitcoin-node/tests/phase165_live_fixture/requalification.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/filter_index.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/fixtures.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/index_protection.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/canonical_chain.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/daemon_sync.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/operator_evidence.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/peer_serving/evidence.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/inbound.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/lifecycle.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/outbound.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/rpc/failures.rs
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
  - packages/open-bitcoin-rpc/src/http/tests/filter_index/fixtures.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/listener_fixtures.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs
  - packages/open-bitcoin-rpc/tests/black_box_parity/fixtures.rs
  - packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs
  - packages/open-bitcoin-rpc/tests/phase165_work_startup.rs
  - packages/open-bitcoin-rpc/tests/support/phase165_live_source.rs
  - packages/open-bitcoin-wallet/src/wallet/tests.rs
  - packages/testdata/phase165/legacy-c-fixtures.json
  - scripts/check-phase123-runtime-timing-evidence-integrity.test.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts
  - scripts/check-phase127-authoritative-network-state-unification.test.ts
  - scripts/check-phase127-authoritative-network-state-unification.ts
  - scripts/check-phase127-authoritative-network-state-unification/production-proof-tests.ts
  - scripts/check-phase128-production-compact-announcement-transport.test.ts
  - scripts/check-phase128-production-compact-announcement-transport.ts
  - scripts/check-phase128-production-compact-announcement-transport/durable-trigger.ts
  - scripts/check-phase134-apply-boundaries/aggregate-roots.ts
  - scripts/check-phase134-authoritative-lifecycle.test/apply-helpers.ts
  - scripts/check-phase155-filter-index.test.ts
  - scripts/check-phase155-filter-index.ts
  - scripts/check-phase156-prune-coordination.test.ts
  - scripts/check-phase156-prune-coordination.ts
  - scripts/check-phase156-prune-coordination/contracts.ts
  - scripts/check-phase157-index-catch-up.test.ts
  - scripts/check-phase157-index-catch-up/contracts.ts
  - scripts/check-phase158-validated-reorg.test.ts
  - scripts/check-phase158-validated-reorg.ts
  - scripts/check-phase159-filter-rpcs.test.ts
  - scripts/check-phase159-filter-rpcs.ts
  - scripts/check-phase159-filter-rpcs/contracts.ts
  - scripts/check-phase161-operator-index-evidence.ts
  - scripts/check-phase162-integrated-filter-proof.test.ts
  - scripts/check-phase162-integrated-filter-proof.ts
  - scripts/check-phase163-provenance/fixtures.ts
  - scripts/check-phase164-canonical-identity.test.ts
  - scripts/check-phase164-canonical-identity.ts
  - scripts/check-phase164-canonical-identity/contracts.ts
  - scripts/check-phase164-canonical-identity/fixture-source.ts
  - scripts/check-phase164-canonical-identity/historical-tooling.ts
  - scripts/check-phase164-canonical-identity/historical.test.ts
  - scripts/check-phase164-canonical-identity/source-files.ts
  - scripts/check-phase164-canonical-identity/test-modules.ts
  - scripts/check-phase165-shared-exact-work.test.ts
  - scripts/check-phase165-shared-exact-work.ts
  - scripts/check-phase165-shared-exact-work/contracts.ts
  - scripts/check-phase165-shared-exact-work/test-support.ts
  - scripts/check-phase68-active-chain-persistence.test.ts
  - scripts/check-phase68-active-chain-persistence.ts
  - scripts/check-phase72-observability-evidence.ts
  - scripts/check-phase73-uat-verification.test.ts
  - scripts/check-phase73-uat-verification/constants.ts
  - scripts/check-phase75-soak-runner.test.ts
  - scripts/check-phase75-soak-runner.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
generated_by: gsd-code-reviewer
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
lifecycle_mode: yolo
source_files: 335
documentation_files: 9
diff_base: d0104a997c0362bf65b81de1c8fec8ddc3012a1b
source_mapping_sha256: 7354f4d8a5dd9378a7d2fde02e8dc975cd708fb2fbc59324b24ce03cd3c4345e
document_mapping_sha256: f99c68949b835bc6bc02ba41a5c2c2256b811197adc89a4bcbce9453f829b6dd
phase_acceptance: false
---
# Phase 165: Code Review Report

**Reviewed:** 2026-10-18T13:04:25Z
**Depth:** deep
**Files reviewed:** 344 (335 source/data and nine documentation/catalog files)
**Status:** clean

## Summary

The complete changed and new-file phase scope is CLEAN with zero Critical,
Warning or Info findings. All reviewed files meet quality standards. No issues
found. The review covers correctness, security-relevant implementation and
maintainability; it does not grant later canonical bootstrap/reopen, contextual
consensus or pinned-daemon qualification.

The current source generation retains one full-width work model, explicit
header/body preference, private mutation authority, strict historical source
restoration and ordinary-selector refusal before backend effects. The actual
entire native verifier and independently accepted fresh coverage support these
source conclusions. Final phase verification and normal-hook Git finalization
remain distinct gates.

## Scope, guidance and method

The exact base is d0104a997c0362bf65b81de1c8fec8ddc3012a1b. Scope combines the
complete Git diff against that base with git ls-files --others --exclude-standard.
The fresh closing inventory is exactly the 344 paths in frontmatter. Planning
and task metadata, ignored files and third-party/generated build trees are not
silently counted as application source. The intentionally tracked LOC artifact
is included as documentation. No .claudeignore or project skill directory was
present.

Repo AGENTS.md, AGENTS.bright-builds.md, placeholder-only standards-overrides.md
and relevant architecture, code-shape, verification, testing, Rust and
TypeScript standards informed this review. Both active lesson files were read
completely: 7,188 bytes and 2,397 conservative estimated tokens; no archive lesson
content was loaded.

Earlier independent production, Node, tooling and scoped integration reviews
were reused only through complete matching file identities. The 333 sources
unchanged from accepted Plan40 retain their actual full-buffer bindings. Both
Plan41 changes were independently reviewed as complete source/data proposals,
then rebound to their exact applied bodies. All 335 source/data buffers still
match the final source prepass; there is no unexplained addition or source drift.

The source mapping is 7354f4d8a5dd9378a7d2fde02e8dc975cd708fb2fbc59324b24ce03cd3c4345e.
Its complete canonical path/byte/SHA rows are preserved in
.local/open-bitcoin-dev/phase165-final41-code-final-binding-prepass.json,
SHA256 7f5d995e4464b37cc189d2bebe6f08351a0b3678444618602d3cc2cd9e0e0888.
The nine current documentation identities are listed below. These bindings
describe review-time bytes, not future formal catalog or indexed LOC changes.

## Production and cross-file conclusions

- Exact arithmetic uses the existing 256-bit ProofWork carrier, normalized
  compact decoding, full-width ordering and modulo accumulation. Numerical
  byte/hex imports and source-derived expected vectors do not confer validated
  ancestry. Actual selected limit/hash checks remain distinct from decoding.
- Accepted headers derive work from accepted parents and selected parameters.
  Strictly greater work changes the best header; equal work retains its earlier
  accepted winner. Checked body/ancestor eligibility, insertion-FIFO release and
  retained side candidates remain separate from header arrival.
- Unknown historical body order stays unknown through cached readmission,
  copies and requalification. Greater work can qualify; equal-work activation
  requires the appropriate known linkage. No deterministic restart tie behavior
  is inferred from internal pointer or disk observations.
- Core stages bind private owner, generation, actual tip and parameters.
  Untrusted imports and mechanical fixtures cannot mint live authority.
  Mutable coins access and persistence invalidate stages; exclusive validated
  absorption and source-bound preview continuation preserve the effect boundary.
- Node preparation, preview and commit retain source, current eligibility,
  branch preference and exact replacement checks before effects. Typed actual
  consensus failures permit bounded candidate fallback; storage, stale, BASIC
  and local failures propagate. Durable errors are not misrepresented as an
  all-effects rollback guarantee.
- Complete physical rebind compares effective coins, best/head markers,
  positions, undo and logical counts before transferring genuine ownership.
  Public fixture requalification validates the complete retained prefix and
  physical identity; it does not silently upgrade raw reopen or canonical roots.
- Codec/import classification preserves full-width exact values separately from
  legacy scores and earned lineage. All four ordinary selectors retain
  identity-only pre-backend refusal. Networking remains default off.

Regression sources retain actual owner/generation/parameter, foreign/stale stage,
raw-import, greater/equal work, unknown-order, body failure, physical identity,
consumer, daemon and refusal assertions. Tests were assessed for meaningful
behavior and reliability, not implementation-mirroring style preferences.

## Tooling integrity and final repairs

Plan40's fixture copier preserves exact supplied evidence membership and bytes
in evidence-only mode while retaining executable copying as the default.
Canonical-path, finite-bound, pre-write registration and symlink-safe writer
checks remain shared. The strict missing-root assertion, original 900-case
matrix and historical snapshots were not relaxed.

Historical qualification still requires supplied current generations and
complete exact inverses before imported current classification. Unknown,
partial, reordered, duplicate or forged descriptors cannot obtain historical
authority. H's data-only historical view remains separate from the current
PASSED-generation adapter and does not mutate the caller's Map.

Plan41 removes only the wrapper around the independent historicalToolingView
try/catch. It restores the manifest-attributed refusal required by the unchanged
four-owner regression when the copier adapter already rejects a mutation.
The adapter refusal, historyBound return and imported checkCurrentClaims gate
remain exact; local checkWorkClaims still executes independently. The historical
view's returned Map is discarded and cannot grant current qualification.

Independent source CLEAN preceded actual application and both attributable
stale-row CLI refusals. Independent data review and Root stage acceptance then
preceded renewal of exactly the existing entrypoint SOURCE row. Current entry
SHA256 is cc8fb2ac578eb1a271e148d4d58ca54ac7ad40229669137ddc1bee6486b1302f;
the data host is
7263310cc12da65c08dcaa2980bf0eb5ed917a44a1864bc3523838accb8c8a27.
Other 191 SOURCE rows, all 88 TEST rows and all 307 ordered inputs remain exact.
Counts remain 192/88/307; the existing ordered pin is unchanged.

The explicit simplification pass found this wrapper removal and one-row renewal
to be the smallest complete repair. No additional abstraction, fallback,
dependency, cleanup or inverse is warranted. Two dated prose-only hash caveats
in earlier checkpoint reports remain historically attributed; authoritative
complete-buffer identities and later acceptance are not replaced by those prose
values.

## Actual verification evidence

The full default command was bash scripts/verify.sh, using pinned Bun 1.3.9 and
Rust 1.94.1. PID 87342 ran from 2026-10-18T10:55:11.685402Z to
2026-10-18T12:33:05.263120Z and exited 0, without signal. Internal duration was
5,872,826ms; normal timing 5,873,529ms; parent outcome 5,873,552ms.

All 76 single-file Bun summaries recorded 11,130 pass occurrences and 0 failures.
These are run counts, not deduplicated behaviors. All 59 Rust result occurrences
succeeded; workspace/coverage repeats are not summed as unique tests.
Six baseline ignored occurrences remain: public-network opt-in 1, timing-only 2,
bounded subprocess entrypoints 3 (Node 4 and RPC 2 overall). No new Phase 165 test
is ignored.

| Actual native suite | Pass / fail | Assertions |
| --- | --- | ---: |
| Phase158 complete | 900 / 0 | 2113 |
| Phase162 complete | 2620 / 0 | 9704 |
| Phase163 native file | 160 / 0 | 1534 |
| Phase164 main complete | 1179 / 0 | 2787 |
| Phase 165 complete | 1966 / 0 | 7351 |

The native command reached format/lint/all-target build/tests, CLI/bin,
benchmark smoke/report validation, Bazel smoke/provenance and fresh coverage.
The separate completed preflight passed all 85 eligible source CLIs from 87
registrations with exact serial argv and PIDs.

The retained actual llvm text report is 2,431,082 bytes at SHA256
44107b740baee0fe758c585da40dec865005174d85a09082a6005178ae59bcf3:
161 annotated source-file sections across exactly eight recipe crates, with no
Uncovered Lines: section. Its format provides no aggregate percentage table;
none is inferred.

The original evidence postprocessor validation remains false at
31b559aec8578c1c792ebf5b8ad814b10f5b9f15215862512ab5d85b9d81253d.
Its direct comparison mixed tuple-valued runtime data with parsed JSON arrays.
The executed tools and original false validation were preserved. A separately
reviewed data-only auditor exited 0 and checked all 9,504 inputs in complete
before/after/current frames (28,512 rows), all ten actual outputs, real
argv/exit/timing, recorded FD/writer lineage and exact whole report. Canonical
JSON comparison resolved only that serialization mismatch without rerunning the
native process, helper, application, tests, Cargo or Bazel.

Root's complete closure is
.local/open-bitcoin-dev/phase165-final41-root-r5b-native-closure.json,
SHA256 d464b6d90fb8f1cef1ea583bf22e631f4019a3ca83ae1ba30f1e59a04231e5bd.
Separate Root acceptance
.local/open-bitcoin-dev/phase165-final41-root-native-final-acceptance.json,
SHA256 abc239023ef67348b228b68315755f3424fd633d673567d789c0f9b780798b68,
accepts actual full native and fresh coverage. This finite provenance boundary
does not inventory the OS/compiler or requalify completed process memory.

Root also ran timing-wrapped cargo fmt --manifest-path packages/Cargo.toml --all
under key phase165-final-rust-fmt: actual exit0, with all 335 source/data buffers
unchanged. The current full diff whitespace check is recorded separately.

## Independently reviewed documentation and summary

The reviewer prepared the documentation proposal; independent Root review of its
complete new diff, exact whole-body inverses, missing-Summary proof and source
preservation supplies the separate documentation review. All live after bodies
match proposal v2 d7529e646918c725632a93eb62237745117c13be0a67370c0900a57a3b37bac4.
Root application 608d93fca00c3b9c9001b2bd58add04c5094af2df91f2919c5437f5a3e42ec2f
confirms the exact four document changes and new Summary 14.

Root regenerated LOC after the README and ran freshness: both actual exits 0,
current LOC unchanged at 6d9727a312f0dd85bdaec0b4edc509b1fdba4b83fb1c2919788be1447c295085.
Saved Summary 14 checked 6/6 meaningful files, self-check passed and errors were
empty; commits_exist=false is expected because commits remain deferred.
Root Summary 14 acceptance is 35953c9180172bd4b338289bf38fdc926a41b13b1a1c0b5ca2aea507e2c3364a.
Both complete original final14 Task XML buffers remain exact.

The acceptance record and Summary 14 are additional planning-context evidence,
not silently added to the primary 344-file source count. Native process success,
supplemental evidence acceptance and formal phase completion remain distinct.
The catalog currently remains in_progress pending formal verification.

| Primary documentation/catalog file | Current SHA256 |
| --- | --- |
| README.md | 6a5d40bac7e2b66e2885e2c5d399782cf68196cc59db7f87eb6af8637adfe0d5 |
| docs/architecture/status-snapshot.md | 8e79b2a4bb4e6a07b599f16d7615e770ea80effd803e206f022da107c9a9a092 |
| docs/metrics/lines-of-code.md | 6d9727a312f0dd85bdaec0b4edc509b1fdba4b83fb1c2919788be1447c295085 |
| docs/operator/runtime-guide.md | c6c497329f53f86e1e2435f664b06f743a0036d7bec052f0ce127401934b6057 |
| docs/parity/index.json | facaadb54e1093ed5d5c15ffffe1dc44c4fb0cbb79aa1e9735f8121a017ee7e7 |
| docs/parity/v2-6-canonical-chain-identity.md | 6c240395df4bda05237dc867281d8116d70d0eac565c38e1c6191f736990f43c |
| docs/parity/v2-6-shared-exact-work.md | 803dd2ffb79b16e1bd3a7e691bd142d0e2a9ce67699a2cc67e33954949acbf44 |
| packages/open-bitcoin-cli/README.md | 8db3c061cb9ea4a0dd4299ecbc4bf53659cbca418b8c188f2c97a090db91227f |
| packages/open-bitcoin-node/README.md | 89e6f4cb33400b59ad24eb0ba0d530f0e44e805f1714a8edc70a7578330b731e |

## Limits and finalization

CWOR-01/02/03/05 are the reviewed Phase 165 scope. CWOR-04/canonical bootstrap,
exact all-handle reopen and genesis BASIC belong to Phase 166; contextual
completeness to 167; actual pinned-daemon differential qualification to 168.
No ordinary production, public-network, funds-safety or deterministic restart
qualification is granted by fixture/source/native evidence.

This clean code review is not formal phase completion or Git finalization.
Root owns canonical security/formal/lifecycle disposition and the normal-hook
commit/push. Its later authorized catalog status/report links, metadata and
indexed LOC changes require a concise final documentation rebind. Such
documentation-only changes do not create a new domain source generation;
any actual source change would require renewed scoped review.

Only this canonical review artifact was written by this review step. No source,
test, documentation, shared metadata, hook, dependency, Git index or branch was
modified. No new native/test/Cargo/Bazel pipeline was run by the reviewer.

_Reviewed: 2026-10-18T13:04:25Z_
_Reviewer: gsd-code-reviewer_
_Depth: deep_
