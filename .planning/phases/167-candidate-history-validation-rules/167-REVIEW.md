---
phase: 167-candidate-history-validation-rules
reviewed: "2026-11-05T06:01:13Z"
depth: standard
files_reviewed: 318
files_reviewed_list:
  - .bazelrc
  - packages/Cargo.toml
  - packages/open-bitcoin-chainstate/src/block_filter/tests.rs
  - packages/open-bitcoin-chainstate/src/coins.rs
  - packages/open-bitcoin-chainstate/src/engine.rs
  - packages/open-bitcoin-chainstate/src/engine/apply.rs
  - packages/open-bitcoin-chainstate/src/engine/apply/serialized_tests.rs
  - packages/open-bitcoin-chainstate/src/engine/candidate_rules.rs
  - packages/open-bitcoin-chainstate/src/engine/canonical_genesis.rs
  - packages/open-bitcoin-chainstate/src/engine/canonical_genesis/tests.rs
  - packages/open-bitcoin-chainstate/src/engine/overlay_apply.rs
  - packages/open-bitcoin-chainstate/src/engine/overwrite_undo.rs
  - packages/open-bitcoin-chainstate/src/engine/stage.rs
  - packages/open-bitcoin-chainstate/src/engine/tests.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/disconnect_tip_skips_unspendable_outputs_and_reports_missing_created_out.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/phase167_bip30.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/phase167_context.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/phase167_locktime.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/phase167_policy.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/phase167_scripts.rs
  - packages/open-bitcoin-chainstate/src/engine/work_authority.rs
  - packages/open-bitcoin-chainstate/src/types.rs
  - packages/open-bitcoin-cli/src/client.rs
  - packages/open-bitcoin-cli/src/client/tests.rs
  - packages/open-bitcoin-cli/src/operator/runtime.rs
  - packages/open-bitcoin-cli/src/operator/runtime/support.rs
  - packages/open-bitcoin-cli/src/operator/runtime/support/canonical_genesis.rs
  - packages/open-bitcoin-cli/src/operator/runtime/support/phase167_tests.rs
  - packages/open-bitcoin-codec/src/block.rs
  - packages/open-bitcoin-codec/src/block/wire.rs
  - packages/open-bitcoin-codec/src/transaction.rs
  - packages/open-bitcoin-codec/src/transaction/wire.rs
  - packages/open-bitcoin-codec/src/transaction/wire/tests.rs
  - packages/open-bitcoin-consensus/src/block.rs
  - packages/open-bitcoin-consensus/src/block/intrinsic.rs
  - packages/open-bitcoin-consensus/src/block/intrinsic/tests.rs
  - packages/open-bitcoin-consensus/src/block/mutation.rs
  - packages/open-bitcoin-consensus/src/block/mutation/tests.rs
  - packages/open-bitcoin-consensus/src/block/phase167_tests.rs
  - packages/open-bitcoin-consensus/src/block/sigops.rs
  - packages/open-bitcoin-consensus/src/block/sigops/tests.rs
  - packages/open-bitcoin-consensus/src/block/tests.rs
  - packages/open-bitcoin-consensus/src/block/witness.rs
  - packages/open-bitcoin-consensus/src/candidate_history.rs
  - packages/open-bitcoin-consensus/src/candidate_history/header_rules.rs
  - packages/open-bitcoin-consensus/src/candidate_history/rules.rs
  - packages/open-bitcoin-consensus/src/candidate_history/rules/tests.rs
  - packages/open-bitcoin-consensus/src/candidate_history/tests.rs
  - packages/open-bitcoin-consensus/src/chain_spec.rs
  - packages/open-bitcoin-consensus/src/chain_spec/rules.rs
  - packages/open-bitcoin-consensus/src/chain_spec/rules/tests.rs
  - packages/open-bitcoin-consensus/src/chain_spec/tests.rs
  - packages/open-bitcoin-consensus/src/classify.rs
  - packages/open-bitcoin-consensus/src/context.rs
  - packages/open-bitcoin-consensus/src/crypto.rs
  - packages/open-bitcoin-consensus/src/lib.rs
  - packages/open-bitcoin-consensus/src/script.rs
  - packages/open-bitcoin-consensus/src/script/encoding.rs
  - packages/open-bitcoin-consensus/src/script/legacy.rs
  - packages/open-bitcoin-consensus/src/script/locktime.rs
  - packages/open-bitcoin-consensus/src/script/opcodes.rs
  - packages/open-bitcoin-consensus/src/script/serialized_tests.rs
  - packages/open-bitcoin-consensus/src/script/sigops.rs
  - packages/open-bitcoin-consensus/src/script/stack.rs
  - packages/open-bitcoin-consensus/src/script/taproot.rs
  - packages/open-bitcoin-consensus/src/script/tests.rs
  - packages/open-bitcoin-consensus/src/script/tests/verify_script_matches_knots_equal_vector.rs
  - packages/open-bitcoin-consensus/src/script/witness.rs
  - packages/open-bitcoin-consensus/src/sighash.rs
  - packages/open-bitcoin-consensus/src/sighash/tests.rs
  - packages/open-bitcoin-consensus/src/signature.rs
  - packages/open-bitcoin-consensus/src/signature/tests.rs
  - packages/open-bitcoin-consensus/src/signature/tests/high_s.rs
  - packages/open-bitcoin-consensus/src/transaction.rs
  - packages/open-bitcoin-consensus/src/transaction/intrinsic.rs
  - packages/open-bitcoin-mempool/src/policy.rs
  - packages/open-bitcoin-network/src/compact_download.rs
  - packages/open-bitcoin-network/src/compact_download/completion.rs
  - packages/open-bitcoin-network/src/compact_download/tests.rs
  - packages/open-bitcoin-network/src/compact_download/tests/blocktxn_cases.rs
  - packages/open-bitcoin-network/src/compact_download/tests/completion_cases.rs
  - packages/open-bitcoin-network/src/compact_download/tests/initialization_cases.rs
  - packages/open-bitcoin-network/src/compact_download/tests/scheduling_cases.rs
  - packages/open-bitcoin-network/src/header_store.rs
  - packages/open-bitcoin-network/src/header_store/canonical_genesis.rs
  - packages/open-bitcoin-network/src/header_store/canonical_history.rs
  - packages/open-bitcoin-network/src/header_store/canonical_history/tests.rs
  - packages/open-bitcoin-network/src/header_store/eligibility.rs
  - packages/open-bitcoin-network/src/header_store/eligibility/tests.rs
  - packages/open-bitcoin-network/src/header_store/tests.rs
  - packages/open-bitcoin-network/src/lib.rs
  - packages/open-bitcoin-network/src/peer.rs
  - packages/open-bitcoin-network/src/peer/block_requests.rs
  - packages/open-bitcoin-network/src/peer/compact_download_state.rs
  - packages/open-bitcoin-network/src/peer/inventory_state.rs
  - packages/open-bitcoin-network/src/peer/message_dispatch.rs
  - packages/open-bitcoin-network/src/peer/tests.rs
  - packages/open-bitcoin-network/src/peer/tests/block_request_order_cases.rs
  - packages/open-bitcoin-network/src/peer/tests/block_sync_lifecycle_cases.rs
  - packages/open-bitcoin-network/src/peer/tests/compact_download_fallback_cases.rs
  - packages/open-bitcoin-network/src/peer/tests/compact_download_lifecycle_cases.rs
  - packages/open-bitcoin-network/src/peer/tests/compact_fixtures.rs
  - packages/open-bitcoin-network/src/peer/tests/compact_retry_cases.rs
  - packages/open-bitcoin-network/src/peer/tests/transaction_lifecycle_cases.rs
  - packages/open-bitcoin-network/src/test_block_commitments.rs
  - packages/open-bitcoin-node/src/chainstate.rs
  - packages/open-bitcoin-node/src/chainstate/filter_index.rs
  - packages/open-bitcoin-node/src/chainstate/filter_index/phase167_tests.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store/canonical_genesis.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store/phase167_undo_tests.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/fjall_sink.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/fjall_sink/proof_tests.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/metadata.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/phase167_counts.rs
  - packages/open-bitcoin-node/src/chainstate/live_fixture.rs
  - packages/open-bitcoin-node/src/chainstate/replay/tests.rs
  - packages/open-bitcoin-node/src/chainstate/tests.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history/tests.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history/tests/phase167.rs
  - packages/open-bitcoin-node/src/chainstate/work_authority.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/action_translation.rs
  - packages/open-bitcoin-node/src/network/canonical_admission.rs
  - packages/open-bitcoin-node/src/network/canonical_admission/tests.rs
  - packages/open-bitcoin-node/src/network/canonical_admission/tests/fixtures.rs
  - packages/open-bitcoin-node/src/network/canonical_admission/tests/phase167.rs
  - packages/open-bitcoin-node/src/network/canonical_admission/tests/phase167/compact_reconstruction.rs
  - packages/open-bitcoin-node/src/network/canonical_admission/tests/phase167/compact_stored_body.rs
  - packages/open-bitcoin-node/src/network/canonical_admission/tests/phase167/fixtures.rs
  - packages/open-bitcoin-node/src/network/compact_receive_candidates.rs
  - packages/open-bitcoin-node/src/network/header_sync.rs
  - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/phase167_tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/serving/disconnect.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/peer_sessions.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/peer_sessions/basic_tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/peer_sessions/registry.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/validation_history.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests/compact_download_cleanup.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests/observation.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests/reopen_bound.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests/resources.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/validation_history/tests/typed_boundaries.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/validation_history/workflow.rs
  - packages/open-bitcoin-node/src/network/tests.rs
  - packages/open-bitcoin-node/src/network/tests/block_connect_disposition.rs
  - packages/open-bitcoin-node/src/network/tests/compact_announcement.rs
  - packages/open-bitcoin-node/src/network/tests/compact_cleanup_cases.rs
  - packages/open-bitcoin-node/src/network/tests/compact_misbehavior_cases.rs
  - packages/open-bitcoin-node/src/network/tests/compact_stored_body_cases.rs
  - packages/open-bitcoin-node/src/network/tests/compact_timeout_cases.rs
  - packages/open-bitcoin-node/src/network/types.rs
  - packages/open-bitcoin-node/src/network/validation_history.rs
  - packages/open-bitcoin-node/src/storage.rs
  - packages/open-bitcoin-node/src/storage/canonical_bootstrap.rs
  - packages/open-bitcoin-node/src/storage/canonical_bootstrap/test_support.rs
  - packages/open-bitcoin-node/src/storage/canonical_history.rs
  - packages/open-bitcoin-node/src/storage/canonical_history/proof.rs
  - packages/open-bitcoin-node/src/storage/canonical_history/records.rs
  - packages/open-bitcoin-node/src/storage/canonical_history/records/projection.rs
  - packages/open-bitcoin-node/src/storage/canonical_history/records/projection/tests.rs
  - packages/open-bitcoin-node/src/storage/canonical_history/records/projection/tests/freshness.rs
  - packages/open-bitcoin-node/src/storage/canonical_history/tests.rs
  - packages/open-bitcoin-node/src/storage/chain_compatibility/tests/preservation.rs
  - packages/open-bitcoin-node/src/storage/coins_view.rs
  - packages/open-bitcoin-node/src/storage/fjall_store.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows/codec.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows/durable_tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis/tests/freshness.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/chain_meta.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/coins_access.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/append/rows.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/append/rows/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append_proof.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/canonical_history.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/copy.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/copy/identity.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/live.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/live/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/live/tests/refusals.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/live/tests/unlinked.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/phase167_tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/protocol.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/protocol/history.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/reader.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/inspection/worker.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/opening.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/opening/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/opening/tests/observation.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/payload_usage.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/serialized_body_tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tracked.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tracked/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/validation_history.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/serialized_tests.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/overwrite_undo.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/tests/chainstate_wallet_and_headers.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/tests/phase167_undo.rs
  - packages/open-bitcoin-node/src/sync.rs
  - packages/open-bitcoin-node/src/sync/block_reconcile.rs
  - packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs
  - packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests/tests.rs
  - packages/open-bitcoin-node/src/sync/block_response.rs
  - packages/open-bitcoin-node/src/sync/canonical_genesis.rs
  - packages/open-bitcoin-node/src/sync/canonical_genesis/basic_tests.rs
  - packages/open-bitcoin-node/src/sync/canonical_genesis/tests.rs
  - packages/open-bitcoin-node/src/sync/canonical_history.rs
  - packages/open-bitcoin-node/src/sync/canonical_history/tests.rs
  - packages/open-bitcoin-node/src/sync/open_runtime.rs
  - packages/open-bitcoin-node/src/sync/runtime_state.rs
  - packages/open-bitcoin-node/src/sync/session.rs
  - packages/open-bitcoin-node/src/sync/session/emission_terminal.rs
  - packages/open-bitcoin-node/src/sync/session/outbox.rs
  - packages/open-bitcoin-node/src/sync/tests.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/startup/recovery.rs
  - packages/open-bitcoin-node/src/sync/tests/peer_resolution/peer_protocol_failures.rs
  - packages/open-bitcoin-node/src/sync/tests/peer_resolution/peer_protocol_failures/lifecycle.rs
  - packages/open-bitcoin-node/src/sync/tests/phase165_live_fixture.rs
  - packages/open-bitcoin-node/src/sync/tests/phase167_historical.rs
  - packages/open-bitcoin-node/src/sync/tests/phase167_historical/bip30_repeats.rs
  - packages/open-bitcoin-node/src/sync/tests/phase167_historical/bip30_restore.rs
  - packages/open-bitcoin-node/src/sync/tests/phase167_historical/contexts.rs
  - packages/open-bitcoin-node/src/sync/tests/phase167_historical/ordinary_flags.rs
  - packages/open-bitcoin-node/src/sync/tests/phase167_historical/scripts.rs
  - packages/open-bitcoin-node/src/sync/tests/phase167_historical/source.rs
  - packages/open-bitcoin-node/src/sync/tests/phase167_ordinary.rs
  - packages/open-bitcoin-node/src/sync/tests/phase167_support.rs
  - packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs
  - packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases/compact_timeout.rs
  - packages/open-bitcoin-node/tests/phase165_live_fixture.rs
  - packages/open-bitcoin-primitives/src/script.rs
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/canonical_startup.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/phase167_tests.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/fixtures.rs
  - packages/open-bitcoin-rpc/src/context.rs
  - packages/open-bitcoin-rpc/src/context/canonical_chain.rs
  - packages/open-bitcoin-rpc/src/context/canonical_chain/tests.rs
  - packages/open-bitcoin-rpc/src/context/canonical_genesis.rs
  - packages/open-bitcoin-rpc/src/context/canonical_history.rs
  - packages/open-bitcoin-rpc/src/context/canonical_history/tests.rs
  - packages/open-bitcoin-rpc/src/context/mempool_recovery.rs
  - packages/open-bitcoin-rpc/src/context/network.rs
  - packages/open-bitcoin-rpc/src/context/wallet_state.rs
  - packages/open-bitcoin-rpc/src/dispatch.rs
  - packages/open-bitcoin-rpc/src/dispatch/node.rs
  - packages/open-bitcoin-rpc/src/dispatch/submit_block.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/block_serving.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/failures.rs
  - packages/open-bitcoin-rpc/src/method.rs
  - packages/open-bitcoin-rpc/src/method/tests.rs
  - packages/open-bitcoin-rpc/tests/phase167_canonical_startup.rs
  - packages/open-bitcoin-rpc/tests/phase167_canonical_startup/basic_evidence.rs
  - scripts/basic-filter-oracle.py
  - scripts/check-active-milestone-verification-traceability/checks.ts
  - scripts/check-active-milestone-verification-traceability/lifecycle-scenarios.ts
  - scripts/check-active-milestone-verification-traceability/ownership.ts
  - scripts/check-active-milestone-verification-traceability/success-and-coverage.ts
  - scripts/check-panic-sites.test.ts
  - scripts/check-panic-sites.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity/test-fixtures.ts
  - scripts/check-phase127-authoritative-network-state-unification.test.ts
  - scripts/check-phase127-authoritative-network-state-unification.ts
  - scripts/check-phase127-authoritative-network-state-unification/production-proof-tests.ts
  - scripts/check-phase128-production-compact-announcement-transport.test.ts
  - scripts/check-phase128-production-compact-announcement-transport.ts
  - scripts/check-phase128-production-compact-announcement-transport/durable-trigger.ts
  - scripts/check-phase135-snapshot-recovery.test.ts
  - scripts/check-phase135-snapshot-recovery/source.ts
  - scripts/check-phase157-index-catch-up/contracts.ts
  - scripts/check-phase159-filter-rpcs.test.ts
  - scripts/check-phase159-filter-rpcs/contracts.ts
  - scripts/check-phase160-peer-filter-serving.ts
  - scripts/check-phase164-canonical-identity.ts
  - scripts/check-phase164-canonical-identity/contracts.ts
  - scripts/check-phase164-canonical-identity/historical-tooling.ts
  - scripts/check-phase164-canonical-identity/source-files.ts
  - scripts/check-phase164-canonical-identity/test-modules.ts
  - scripts/check-phase165-shared-exact-work.ts
  - scripts/check-phase165-shared-exact-work/contracts.ts
  - scripts/check-phase167-candidate-history.test.ts
  - scripts/check-phase167-candidate-history.ts
  - scripts/check-phase167-candidate-history/contracts.ts
  - scripts/check-phase97-inbound-metrics.ts
  - scripts/check-phase98-traceability-reconciliation.test.ts
  - scripts/check-phase98-traceability-reconciliation.ts
  - scripts/phase167-bip30-fixtures.test.ts
  - scripts/phase167-bip30-fixtures.ts
  - scripts/phase167-historical-corpus.test.ts
  - scripts/phase167-historical-corpus.ts
  - scripts/phase167-regtest-corpus.test.ts
  - scripts/phase167-regtest-corpus.ts
  - scripts/phase167-regtest-serialization.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
source_scope_sha256: da888c8929643e35d5cd770c9dfa0beeabcd5d96d4cc843a4e6f120e9c39cbc6
source_product_paths: 1828
source_product_sha256: 715bd1ef82cce36afd29b177cdb604b8db796bcdccb0f35bd78ca5d55fe03fd5
source_frame_sha256: b98af995b4ce42116a7d732f948bf89eabc31e22b1351b3cf2e15057cc284901
default_verifier_receipt_sha256: 0728e7a86cc5589fa6edcb13cf08fafa8c6e3e0b4d4f763a7d7728e71c80a62f
revised_default_log_sha256: 74a5226365e916707b0e723f74b586d754cb25025775ed0b1dcee7201afdb339
phase_lifecycle_id: 167-2026-10-21T16-38-23
---

# Phase 167: Code Review Report

**Reviewed:** 2026-11-05T06:01:13Z
**Depth:** Standard, with focused cross-module authority tracing
**Files reviewed:** 318
**Status:** clean

## Summary

No actionable correctness, security, or mandatory code-quality finding remains in the reviewed Phase 167 source. The scope lists changed first-party Rust, Bun, Python, shell, and Cargo manifest files from baseline 1b472fb through accepted Plans 17–19/25 and Plan 20. The generated Bazel lock, parity fixtures, current documentation, and ignored local receipts were examined as supporting evidence and are not counted as code files. Every listed code file was read or scanned; the authority and error paths below received focused cross-file review.

The frozen frame contains 1828 product and evidence paths. SHA-256 of sorted `path + TAB + file SHA-256 + LF` rows, excluding generated LOC and the four final reports absent at capture, is `715bd1ef82cce36afd29b177cdb604b8db796bcdccb0f35bd78ca5d55fe03fd5`. The 318-file reviewed code-scope digest uses the same formula: `da888c8929643e35d5cd770c9dfa0beeabcd5d96d4cc843a4e6f120e9c39cbc6`; it adds the later `.bazelrc` correction to the 317-file v14 code scope. The frozen frame has 82 Plan 20 owners and 1,751 protected FINAL25 paths with zero drift. Seventy product paths changed from FINAL25, all within the frozen Plan 20 ownership.

The earlier default `bash scripts/verify.sh` [v14 receipt](../../../.local/open-bitcoin-dev/phase167/plan20/task1-execution-v1/coverage-c-default-verify-v14.json) has exit 0 and unchanged source-owner hashes. It passed pinned Bun guards, format, strict Clippy, all-target build, nine original 16-thread stress selectors, complete native suites, benchmark validation, six Bazel targets and provenance, and fresh eight-package coverage with no `Uncovered Lines:` heading. The [validation report](167-VALIDATION.md) records exact discovery and earlier failures. The revised-config v15 run below is the current verifier basis.

## Authority and behavior review

| Decisions | Reviewed source behavior |
| --- | --- |
| D-01–03 | Immutable four-network rules feed `CandidateHistory`, which checks candidate branch, time windows and BIP34 facts without creating canonical acceptance. Chainstate re-derives selected context from held candidate and active ancestry. Only bounded default-Regtest local ordinary history is supported. Rewards, halving, maturity, fees, BIP68 creating-MTP and witness rules consume selected context. |
| D-04–05 | The interpreter keeps flag-sensitive P2SH/WITNESS/TAPROOT execution and three exact flag replacements before buried additions. The 80,000-sigop budget uses checked consensus arithmetic and real spent outputs. Wave C retained public `Result` count APIs and gave mempool count-only accessors over the same private syntax counters. Difficulty, MTP and minimum-difficulty recovery stay bound to the candidate branch; missing anchors refuse. |
| D-06 | BIP30 scans the complete pre-block UTXO view and captures prior Coin fields and exact overwrite occurrence. Undo integrity, durable codec, replay and disconnect reject mismatched coins or provenance before publication. Complete prior-Coin restoration is documented as an intentional difference from pinned Knots. |
| D-07–08 | Proved genesis, admitted headers, staged chainstate, accepted receipts, BASIC and retained-history reopen use distinct owners. Seeded historical facts and imported status cannot mint authority. Typed refusals preserve tip, coins, work, filters and durable provenance. RPC, CLI and local daemon consume the same bounded owner. |
| D-09–10 | Serialized historical corpus and source pins bind actual bytes. Current Phase 97/98/127/128/135/157/159/160/164/165/167 guards were renewed for reviewed current bodies while historical inverses and negative mutations remain. Panic-site ownership, parity breadcrumbs, documentation reconciliation and the repo-native verifier pass. |

## Plan 20 changes and evidence

Coverage waves A1, A2, B and C closed 159 missing pure-core lines without changing the eight-package zero rule. Network tests exercise compact and header/peer refusals. Chainstate and consensus tests exercise selected ancestry, BIP30 prior Coins, storage-error propagation, script and sigop limits; codec tests exercise fallible allocation propagation. Wave C removed four unreachable mempool error mappings while retaining source-compatible public `Result` counters. The [A1](../../../.local/open-bitcoin-dev/phase167/plan20/task1-execution-v1/coverage-a1-final-v1/receipt.json), [A2](../../../.local/open-bitcoin-dev/phase167/plan20/task1-execution-v1/coverage-a2-final-v1/receipt.json), [B](../../../.local/open-bitcoin-dev/phase167/plan20/task1-execution-v1/coverage-b-final-v1/receipt.json), and [C](../../../.local/open-bitcoin-dev/phase167/plan20/task1-execution-v1/coverage-c-final-v1/receipt.json) receipts bind the reviewed source and outcomes.

The durable inspection change classifies a known unlinked regular descendant as `Changed` for bounded recheck; unknown sources and hardlinks still refuse. Test-only detached-copy timing measures physical copy and recovery before validation callbacks, retaining both strict 30-second assertions and the worker deadline. The checked test profile and verifier use four harness threads for complete native suites and nine named selectors at 16 threads. RPC socket tests retry only `WouldBlock` under bounded deadlines. The prune undo-size oracle includes overwrite batches. The Bazel lock changed only its recorded Cargo manifest digest.

## Source checks and limits

- The 318 scoped code and build-config files contain no unsafe block, unfinished `todo!` or `unimplemented!` path, or production `unwrap()`. The one literal `unwrap()` is a panic-site checker test fixture. All changed files fit the managed 628-line cap. New Rust files have required parity breadcrumbs or checked entrypoint exceptions; the repo-native breadcrumb check passed.
- Accepted FINAL25 and Plan 20 source frames were compared against live hashes. The v14 receipt reports no changed source path. Generated LOC remains a separate staged-index reconciliation task.
- Earlier failed verifier attempts retain their failed status. Independent security and goal reports, lifecycle validation, staged-index LOC and normal-hook Git saving are separate acceptance steps.

## Upstream managed update after v14

After the successful v14 run, local HEAD fast-forwarded to upstream Bright Builds Rules commit `34731eca61fcf3feac4eea9621ad4f36855f89d7`. The commit updates only the source commit pin in `AGENTS.bright-builds.md`, that pin and timestamp in `bright-builds-rules.audit.md`, and two root README pathspecs in `scripts/bright-builds-auto-update.sh`. The sidecar and audit were outside the frozen 1,828-path product frame. The script was inside it but was unchanged by Phase 167; its SHA-256 moved from `08e753689ab4f9990d86c7f52926423863142845369a2e048a769b11e57aded8` to `5ec3604c4439216ae178bd595fcbc0ea807b961b7c864d66e39366dd3f5b628b`. The new pathspec resolves to the root README in this checkout, and `bash -n` passes. This inspected upstream delta has no actionable Phase 167 source finding.

The frozen v14 product digest in frontmatter remains `715bd1ef82cce36afd29b177cdb604b8db796bcdccb0f35bd78ca5d55fe03fd5`. Replacing only that script row with its upstream hash yields combined-tree digest `82a2e42c455820c7e834755aa0c956284bc8b29a007f272b6c8fb3b3a08c1380` over the same 1,828 paths. The v14 verifier result applies to the frozen snapshot; ROOT's normal staged hook remains the verification gate for the combined tree and index-generated LOC.

## Late Bazel host-tool correction

The second normal commit hook passed the full native suites but failed Bazel host proc-macro loading with a malformed Mach-O LINKEDIT string-pool offset. Clean and expunged default builds reproduced the failure; a host `fastbuild` proc-macro dylib loaded. The checked Plan 20 [amendment](167-20-BAZEL-HOST-TOOLCHAIN-AMENDMENT.md) adds only `build --host_compilation_mode=fastbuild` to `.bazelrc`, SHA-256 `59227b79b943c837a5dfee5eacd52393616131994acc5258d836d4d761289321`. This selects host-tool compilation mode and leaves target mode, six Bazel smoke targets, Cargo, coverage and consensus behavior unchanged. The fresh expunged literal default six-target [build log](../../../.local/open-bitcoin-dev/phase167/plan20/bazel-fresh-default-host-fix-v1.log), SHA-256 `0054c9c6144b5e8fbe0a0f8c15073ac506d87e130934913c67bc9c7b36e38ddb`, ends with all six targets built successfully. The configuration diff is one line and has no actionable source-review finding.

`.bazelrc` was outside the frozen v14 1,828-path frame, so the frontmatter product digest remains the v14 basis. The 318-file code-scope digest above includes this reviewed late config. Current status and parity document changes, the upstream managed script, and staged LOC require the final combined-tree source/index and normal hook to be evaluated on their actual bytes; this report does not retag the v14 run as verification of those later changes.

## Revised-config default verification

The complete repo-native default verifier reran with the reviewed `.bazelrc` on the revised tree and exited 0 in 2h 34m 54.863s. Its retained [v15 log](../../../.local/open-bitcoin-dev/phase167/plan20/revised-default-v15.log) has SHA-256 `74a5226365e916707b0e723f74b586d754cb25025775ed0b1dcee7201afdb339`. It records passing current Phase 164/165 guards, all nine 16-thread stress selectors, complete native tests, benchmark validation, the literal default six-target Bazel build and provenance, and the original eight-package zero-uncovered coverage gate. The log ends with `verify.sh completed`; it contains no `Uncovered Lines:` heading. An independent live rehash of all 318 reviewed code/config paths equals the frontmatter code-scope digest `da888c8929643e35d5cd770c9dfa0beeabcd5d96d4cc843a4e6f120e9c39cbc6`, so no reviewed code changed after the successful run. Earlier failed hooks and diagnostics retain their failed status. Final staged-index LOC and normal-hook completion remain ROOT's separate saving gate.

## Residual boundary

This review does not qualify an actual pinned-Knots daemon differential, arbitrary whole-chain history, Mainnet/Testnet3/default-Signet ordinary product routes, public networking, canonical-history wallet availability or Phase 168. Independently seeded historical script and BIP30 cases support only their exact corpus claims. The bounded product claim is the local default-Regtest 301-descendant route and complete retained reopen.

All reviewed files meet the scoped quality requirements. No issues found.

_Reviewed: 2026-11-05T06:01:13Z_
_Reviewer: gsd-code-reviewer_
_Depth: standard with focused cross-module tracing_

