export const NODE = "packages/open-bitcoin-node/src/";
export const CORE = "packages/open-bitcoin-chainstate/src/";
export const RPC = "packages/open-bitcoin-rpc/src/";
export const PHASE = ".planning/phases/157-safe-activation-and-scheduled-index-catch-up/";
export const DOC = "docs/parity/catalog/basic-compact-filters.md";
export const SURFACE = "v2-5-safe-basic-activation-and-scheduled-catch-up";
export const DOCUMENTED_LIMITS = [
  ["Blocks", "8 blocks", "1"], ["Body logical/wire bytes", "1 MiB", "128,000,000"],
  ["Borrowed undo logical bytes", "4 MiB", "256 MiB"], ["Copy/allocation reservation", "16 MiB", "1 GiB"],
  ["Item-work reservation", "4 Mi units", "128 Mi units"], ["Byte-work reservation", "32 Mi units", "64 Gi units"],
  ["Encoded record envelopes", "1 MiB", "33,554,602"], ["Record operations", "512", "512"],
  ["Checkpoint/map/structural work", "1,000,000", "1,000,000"], ["Projection operations", "256", "256"],
] as const;
export const NEW_RUST = [
  "packages/open-bitcoin-chainstate/src/filter_index/catch_up.rs",
  "packages/open-bitcoin-chainstate/src/filter_index/catch_up/budget.rs",
  "packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests.rs",
  "packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests/budget.rs",
  "packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests/durability.rs",
  "packages/open-bitcoin-node/src/chainstate/filter_index.rs",
  "packages/open-bitcoin-node/src/chainstate/tests/filter_index.rs",
  "packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs",
  "packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/inputs.rs",
  "packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs",
  "packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append.rs",
  "packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append/admission.rs",
  "packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append/fencing.rs",
  "packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append_proof.rs",
  "packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs.rs",
  "packages/open-bitcoin-node/src/sync/tests/filter_index/accepted.rs",
  "packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs",
  "packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/failures.rs",
  "packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/fixtures.rs",
  "packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/measurements.rs",
  "packages/open-bitcoin-node/src/sync/tests/filter_index/evidence.rs",
  "packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/accepted_faults.rs",
  "packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/history_loss.rs",
  "packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/retention.rs",
  "packages/open-bitcoin-node/src/sync/tests/filter_index/startup/configured.rs",
  "packages/open-bitcoin-node/src/sync/tests/filter_index/startup/configured/faults.rs",
  "packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/filter_index.rs",
  "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs",
  "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs",
  "packages/open-bitcoin-rpc/src/config/blockfilter.rs",
  "packages/open-bitcoin-rpc/src/config/loader/blockfilter.rs",
  "packages/open-bitcoin-rpc/src/config/tests/blockfilter.rs"
];
const DRIVER = NODE + "network/runtime_authority/filter_index/catch_up.rs";
const OWNER = NODE + "storage/fjall_store/filters/ownership.rs";
const APPEND = NODE + "storage/fjall_store/filters/append.rs";
const PROOFS = NODE + "storage/fjall_store/filters/ownership/proofs.rs";
export const PRODUCTION_BUDGET = NODE + "chainstate/filter_reorg/preflight.rs";
const TEST = NODE + "sync/tests/filter_index/";
const DAEMON = RPC + "bin/open-bitcoind.rs";
const TIMER = RPC + "bin/open_bitcoind/coins_flush.rs";
export type Contract = { decision: string; threat: string; file: string; symbol: string;
  anchors: readonly string[]; maybeForbidden?: readonly string[] };
const row = (decision: string, file: string, symbol: string, anchors: string[], maybeForbidden?: string[]): Contract =>
  ({ decision, threat: "T-157-30", file, symbol, anchors, maybeForbidden });
export const SCANS = ["load_chain_meta_for_open(", "validate_basic_filter_records(",
  "scan_basic_filter_checkpoint(", "maybe_basic_filter_checkpoint(", "prepare_basic_filter_projection(",
  "load_basic_filter_record(", "verify_basic_filter_fence(", "VerifiedChainstateFence::new(",
  "load_all_undo_records(", "active_chain().to_vec(", "undo_by_block().clone("];
/** D-01–D-14 ordinary consumers; mutation controls remove each boundary independently. */
export const CONTRACTS: Contract[] = [
  row("D-01", RPC + "config/blockfilter.rs", "resolve_filter_index", ["after_last_negation(values)", "remaining.last()", "remaining.first()", "match scalar", "for (source, values) in sources"]),
  row("D-01", RPC + "config/loader.rs", "load_runtime_config_for_args", ["blockfilter::resolve(&cli.block_filter_values, &config_entries, chain)?"]),
  row("D-01", RPC + "config/loader.rs", "parse_cli_args", ["blockfilter::parse_cli(&mut settings.block_filter_values, key, maybe_value, negated)"]),
  row("D-01", RPC + "config/loader/blockfilter.rs", "resolve", ["config_section_name(chain)", "entry.maybe_section.as_deref()", "resolve_filter_index(cli, &network, &defaults)"]),
  row("D-02", DAEMON, "open_runtime_store", ["runtime.block_filter_index.is_enabled() && !has_valid_datadir", "runtime.block_filter_index.is_explicit() && has_valid_datadir", "if !has_durable_trigger", "FjallNodeStore::open(data_dir)"]),
  row("D-03", DAEMON, "open_authoritative_network_runtime", ["runtime.block_filter_index.is_enabled()", "BasicFilterStartupMode::Enabled", "BasicFilterStartupMode::Disabled", "DurableSyncRuntime::open_with_configured_runtime_activation("]),
  row("D-03", NODE + "chainstate/flush_lifecycle.rs", "initialize_configured", ["apply_recovery_decision(", "coins_recovery_outcome_after_success(", "match basic_filter_mode", "configure_basic_filter_index_before_prune(", "store.load_prune_locks()?", "resume_prune_intent(store, &locks)?", "ManagerReadiness::ReadyToFlush", "CoinsCache::from_parent(recovered)"]),
  row("D-04", NODE + "storage/fjall_store/filters/lifecycle.rs", "enable_basic_filter_index", ["self.preflight_basic_filter_history(IndexInputProtection::FromHeight(0), fence)?", "self.initialize_basic_filter_state_guarded(", "self.inspect_basic_filter_recovery(owner, fence)?", "self.preflight_basic_filter_history(checkpoint.input_protection(), fence)?", "if active == owner.lifecycle()", "self.finish_basic_filter_batch("]),
  row("D-05", NODE + "storage/fjall_store/filters/lifecycle.rs", "preflight_basic_filter_history", ["let IndexInputProtection::FromHeight(first)", ".load_block(", "if height == 0", ".load_undo(", "BasicFilterInputs::from_historical("]),
  row("D-06", OWNER, "check_basic_filter_append_proof_guarded_counted", ["Arc::ptr_eq(&proof.publication, &self.filter_publication)", "control.maybe_append_identity != Some(proof.identity)", "control.revision != proof.identity.revision", "owner != proof.identity.owner", "proof.identity.generation", "proof.identity.safe_checkpoint", "view.head_blocks()", "view.best_block()"], SCANS),
  row("D-14", OWNER, "with_remaining_budget", ["self.maybe_maximum_work", "!maximum_work.fits(previous) || !self.preparation_work.fits(maximum_work)", "self.maybe_maximum_work = Some(maximum_work)"]),
  row("D-14", OWNER, "basic_filter_lock_map_cost", ["checked_mul(", "checked_add(1)", "checked_div(2)", "checkpoint_operations:", "checked_mul(1024)", "cloned_bytes:"]),
  row("D-14", PROOFS, "maybe_basic_filter_append_proof_with_limits", ["self.filter_publication_guard()?", "control.maybe_reorg_suspension.is_some()", "self.maybe_basic_filter_reorg_proof_guarded("], SCANS),
  row("D-14", PROOFS, "maybe_basic_filter_reorg_proof_guarded", ["control.maybe_append_identity", "self.check_basic_filter_append_proof_guarded_counted(", "proof.preparation_work = work"], SCANS),
  ...["maybe_basic_filter_owner_guarded_counted", "bounded_basic_filter_checkpoint", "load_basic_filter_append_locks"].map(symbol => row("D-14", OWNER, symbol, [], SCANS)),
  ...["read_basic_filter_local_identity", "maybe_basic_filter_append_projection", "maybe_bounded_basic_filter_append_row"].map(symbol => row("D-14", APPEND, symbol, [], SCANS)),
  row("D-07", NODE + "chainstate.rs", "commit_prepared_connect", ["absorb_staged_connect(", "self.observe_basic_filter_acceptance(", "if let Err(error) = self.persist()", "note_basic_index_failure("]),
  row("D-07", NODE + "chainstate/filter_index.rs", "observe_basic_filter_acceptance", ["observe_validated_connect(", "Some(Ok(facts)) => owner.maybe_facts = Some(facts)", "Some(Err(failure)) => self.note_basic_index_failure(failure)"]),
  row("D-08", NODE + "sync/open_runtime.rs", "open_with_configured_runtime_activation", ["initialize_configured(", "Chainstate::from_coins_cache(", "ManagedChainstate::from_recovered_chainstate(", "network.initialize_basic_filter_index_owner()?", "network.drive_basic_filter_index_turn()?"]),
  row("D-08", DAEMON, "main", ["open_runtime_store(", "open_authoritative_network_runtime(", "serve_authoritative_runtime("]),
  row("D-08", DAEMON, "serve_authoritative_runtime", ["authoritative_runtime.network.clone()", "start_coins_flush_worker(handle.clone(), store)", "settle_daemon_shutdown(", "serve_result.map_err(Into::into)", "maybe_sync_worker.map_or(Ok(()), |worker| worker.shutdown())", "maybe_coins_flush_worker.map_or(Ok(()), |worker| worker.shutdown_always())", "retry_worker.shutdown()?", "maybe_checkpoint_worker.map_or(Ok(()), |worker| worker.shutdown_settle())", "store.mark_clean_shutdown(open_bitcoin_node::PersistMode::Sync)"]),
  row("D-13", RPC + "bin/open_bitcoind/checkpoint.rs", "settle_daemon_shutdown", ["let results = [sync(), coins(), retry(), checkpoint(), serve_result];", "for result in results", "if let Err(error) = result", "if maybe_error.is_none()", "if let Some(error) = maybe_error", "return Err(error)", "mark_clean().map_err("], ["coins()?", "sync()?", "retry()?", "checkpoint()?"]),
  row("D-08", TIMER, "coins_flush_worker_loop", ["match wait(Duration::from_secs(TICK_SECS))", "CheckpointWait::Elapsed", "maybe_drive_elapsed(&handle, &store)", "CheckpointWait::Shutdown", "drive_always(&handle, &store)?", "maybe_index_error.map_or(Ok(()), Err)"]),
  row("D-13", TIMER, "maybe_drive_elapsed", ["drive_periodic(handle, store)", "handle.maybe_drive_index_turn()"]),
  row("D-09", DRIVER, "drive_basic_filter_index_turn", ["self.drive_basic_filter_index_turn_with_budget(", "production_budget().map_err(lifecycle_error)?"]),
  row("D-09", DRIVER, "drive_turn", ["maybe_basic_filter_append_proof_with_budget(", "proof.generation() != progress.generation()", "proof.branch_identity() != progress.branch_identity()", "proof.preparation_work()", "generate_records(", "subtract(total_work, captured_work)?", "proof.with_remaining_budget(remaining)?", "current.prepare_turn()", "store.prepare_basic_filter_append(", "store.complete_basic_filter_append(", "confirm_basic_index_checkpoint(", "work.checked_add(achieved.work)"], SCANS),
  row("D-10", DRIVER, "generate_records", ["let mut work = captured_work", "for height in first..=progress.accepted_target().height()", "records.len() as u64 >= maximum.blocks", "reserve_generation(&mut candidate_work)?", "inputs::undo_work(", "store.maybe_basic_filter_turn_body(", "choose_work(", "BasicFilterInputs::from_historical(", "StoredFilterRecord::generate(", "if outcome.oversized_singleton"], SCANS),
  row("D-14", DRIVER, "reserve_generation", ["checked_mul(4)", "checked_add(179)", "cloned_bytes: copies", "script_items: work", "checked_mul(256)", "script_bytes: work", "checked_mul(256)"]),
  row("D-14", DRIVER, "choose_work", ["candidate.fits(budget.absolute_singleton())", "work.checked_add(candidate)", "total.fits(*maximum)", "outcome.generations != 0", "total.fits(budget.absolute_singleton())", "outcome.oversized_singleton = true"]),
  row("D-14", NODE + "network/runtime_authority/filter_index/catch_up/inputs.rs", "undo_work", ["charge_metadata(work, transactions, maximum)?", "undo.transactions.iter().try_fold(", "charge_metadata(", "borrowed_undo_work(maybe_undo)"], SCANS),
  row("D-14", NODE + "network/runtime_authority/filter_index/catch_up/inputs.rs", "charge_metadata", ["checked_add(TurnWork", "checkpoint_operations: count", "if !total.fits(maximum)", "*work = total"]),
  row("D-14", APPEND, "prepare_basic_filter_append", ["self.prepare_basic_filter_append_guarded_positions(proof, records, None)"], SCANS),
  row("D-14", APPEND, "prepare_basic_filter_append_guarded_positions", ["check_envelope_bounds(records)?", "proof.maybe_maximum_work()", "self.check_basic_filter_append_proof(&proof)?", "charge_append_work(", "verify_filter_record_predecessor(", "maybe_bounded_basic_filter_append_row(", "earned_basic_filter_safe_checkpoint("], SCANS),
  row("D-14", DRIVER, "production_budget", ["crate::chainstate::basic_filter_turn_budget()"]),
  row("D-14", PRODUCTION_BUDGET, "production_budget", ["BasicIndexTurnBudget::new("]),
  row("D-14", APPEND, "complete_basic_filter_append", ["self.filter_publication_guard()?", "check_basic_filter_append_proof_guarded_counted(", "load_basic_filter_append_locks(", "SyncAll", "self.finish_basic_filter_batch("], SCANS),
  row("D-06", APPEND, "earned_basic_filter_safe_checkpoint", ["prepared.proof.durable_tip()", "processed.height() < height", "maybe_basic_filter_append_projection(", "endpoint.height() != height || endpoint.block_hash() != hash"], SCANS),
  row("D-14", NODE + "storage/fjall_store/filters/turn_inputs.rs", "maybe_basic_filter_turn_body", ["bytes.len() > 4_000_000", "bytes.len() as u64 > remaining_body_bytes", "if !admit(work)?", "body_work(", "codec::parse_block("], SCANS),
];
export const TESTS: [string, string][] = [
  [RPC + "config/tests/blockfilter.rs", "phase157_blockfilter_loader_repeated_cli_matches_scalar_list_table"],
  [RPC + "config/tests/blockfilter.rs", "phase157_blockfilter_loader_includeconf_preserves_parent_then_include_order"],
  [RPC + "bin/open_bitcoind/tests/filter_index.rs", "phase157_daemon_basic_forms_select_durable_and_one_strict_startup_prefix"],
  [RPC + "bin/open_bitcoind/tests/filter_index.rs", "phase157_daemon_enabled_empty_store_refuses_without_index_mutation"],
  [RPC + "bin/open_bitcoind/tests/filter_index.rs", "phase157_daemon_basic_keeps_actual_sync_and_inbound_workers_off"],
  [RPC + "bin/open_bitcoind/coins_flush/tests/filter_index.rs", "phase157_idle_injected_elapsed_ticks_advance_without_receives_and_yield"],
  [RPC + "bin/open_bitcoind/coins_flush/tests/filter_index.rs", "phase157_idle_ordered_turns_extend_target_then_genuine_periodic_fence_releases"],
  [RPC + "bin/open_bitcoind/coins_flush/tests/filter_index.rs", "phase157_idle_explicit_disable_refuses_stale_turn_and_keeps_always_cleanup"],
  [RPC + "bin/open_bitcoind/coins_flush/tests/filter_index.rs", "phase157_shutdown_retained_basic_error_settles_all_workers_without_clean_marker"],
  ...["attempts_every_settlement_before_returning_first_failure", "http_failure_withholds_clean_marker_after_settlement", "success_marks_clean_only_after_all_settlement"].map(name => [RPC + "bin/open_bitcoind/tests/checkpoint.rs", "phase157_shutdown_" + name] as [string, string]),
  [TEST + "catch_up.rs", "phase157_turn_prefix_independent_complete_operation_counts"],
  [TEST + "catch_up.rs", "phase157_turn_production_legal_large_singleton_makes_progress"],
  [TEST + "catch_up.rs", "phase157_turn_remaining_budget_cannot_inflate_or_discard_acquisition"],
  [TEST + "catch_up/measurements.rs", "phase157_measure_turns"],
  ...["fresh_legal_paired_genesis_and_spending_loss_refuse_unchanged", "disabled_prefix_required_legal_paired_suffix_loss_refuses_unchanged", "independent_body_and_undo_missing_mates_preserve_live_intent", "indexed_legal_paired_loss_and_interrupted_disable_resume_suffix_only"].map(name => [TEST + "evidence/history_loss.rs", "phase157_store_history_" + name] as [string, string]),
  ...["ordinary_accepted_later_undo_coins_metadata_errors_reopen_truthfully", "requested_body_error_keeps_accepted_target_and_real_old_reopen", "append_before_and_after_commit_reopen_identical_retry_without_duplicate_rows"].map(name => [TEST + "evidence/accepted_faults.rs", "phase157_store_fault_" + name] as [string, string]),
  [TEST + "evidence/retention.rs", "phase157_store_retention_ordinary_automatic_owner_earns_actual_pairs_after_checkpoint"],
];
/** Concrete ordinary effects and helper resolution, beyond test-name/lexical assertion presence. */
export const EVIDENCE: [string, string, string[]][] = [
  [RPC + "bin/open_bitcoind/coins_flush/tests/filter_index.rs", "phase157_shutdown_retained_basic_error_settles_all_workers_without_clean_marker", ["open_authoritative_network_runtime(", "connect_local_block(", "[15, 23, 31, 39]", "thread::spawn(", "coins_flush_worker_loop(", "CheckpointWait::Elapsed", "shutdown_receiver.recv()", "CheckpointWait::Shutdown", "let coins_worker = CoinsFlushWorker", "settle_daemon_shutdown(", "coins_worker.shutdown_always()", "store.coins_view().best_block()", "retry_joined = true", "shutdown_settle()?", "checkpoint_settled = true", "store.mark_clean_shutdown("]],
  [TEST + "evidence/history_loss.rs", "phase157_store_history_fresh_legal_paired_genesis_and_spending_loss_refuse_unchanged", ["TurnHistory::new(16, 384, 1)", "manual(&runtime", "deletion.deleted_block_hashes", "assert_pair(", "drop(runtime)", "assert_refusal_unchanged("]],
  [TEST + "evidence/history_loss.rs", "phase157_store_history_disabled_prefix_required_legal_paired_suffix_loss_refuses_unchanged", ["disable_basic_filter_index()", "manual(&runtime", "deletion.deleted_block_hashes", "drop(runtime)", "assert_refusal_unchanged("]],
  [TEST + "evidence.rs", "manual", ["flush_applying_prune_plan("]],
  [TEST + "evidence.rs", "assert_refusal_unchanged", ["snapshot_store(path)", "assert_filter_refusal(configured(path), category)", "assert_eq!(", "snapshot_store(path)"]],
  [TEST + "evidence.rs", "configured", ["DurableSyncRuntime::open_configured(", "FjallNodeStore::open(path)", "BasicFilterStartupMode::Enabled"]],
  [TEST + "evidence/retention.rs", "phase157_store_retention_ordinary_automatic_owner_earns_actual_pairs_after_checkpoint", ["TurnHistory::new(0, 1_002, 1)", "target_mib: 550", "FlushMode::Periodic", "FlushMode::Always", "deletion.deleted_block_hashes.len(), 714", "drop(runtime)", "body_commitments(", "configured("]],
];
export const SOURCES = ["init.cpp", "common/args.cpp", "common/settings.cpp", "index/base.cpp", "index/blockfilterindex.cpp", "node/blockstorage.cpp"].map(file => "packages/bitcoin-knots/src/" + file);
export const CORPUS = ["rpc_getblockfilter.py", "feature_index_prune.py", "feature_pruning.py"].map(file => "packages/bitcoin-knots/test/functional/" + file);
export const REGISTRATIONS: [string, string[]][] = [
  [CORE + "filter_index.rs", ["pub mod catch_up;"]], [CORE + "filter_index/catch_up.rs", ["mod budget;", "mod tests;"]],
  [CORE + "filter_index/catch_up/tests.rs", ["mod budget;", "mod durability;"]],
  [NODE + "chainstate.rs", ["mod filter_index;", "pub(crate) use filter_reorg::preflight::production_budget as basic_filter_turn_budget;"]], [NODE + "chainstate/tests.rs", ["mod filter_index;"]],
  [NODE + "network/runtime_authority/filter_index.rs", ["mod catch_up;"]], [DRIVER, ["mod inputs;"]],
  [NODE + "storage/fjall_store/filters.rs", ["mod append;", "mod turn_inputs;"]],
  [NODE + "storage/fjall_store/filters/tests.rs", ["mod append;", "mod append_proof;"]],
  [NODE + "storage/fjall_store/filters/tests/append.rs", ["mod admission;", "mod fencing;"]],
  [NODE + "sync/tests/filter_index.rs", ["mod accepted;", "mod catch_up;", "mod evidence;"]],
  [TEST + "catch_up.rs", ["mod failures;", "mod fixtures;", "mod measurements;"]],
  [TEST + "startup.rs", ["mod configured;"]], [TEST + "startup/configured.rs", ["mod faults;"]],
  [TEST + "evidence.rs", ["mod history_loss;", "mod accepted_faults;", "mod retention;"]],
  [RPC + "config.rs", ["mod blockfilter;"]], [RPC + "config/tests.rs", ["mod blockfilter;"]],
  [RPC + "config/loader.rs", ["mod blockfilter;"]], [RPC + "bin/open_bitcoind/tests.rs", ["mod filter_index;"]],
  [RPC + "bin/open_bitcoind/tests/filter_index.rs", ["mod fixtures;"]],
  [RPC + "bin/open_bitcoind/coins_flush/tests.rs", ["mod filter_index;"]],
  [RPC + "bin/open_bitcoind/tests.rs", ["mod checkpoint;"]],
];
