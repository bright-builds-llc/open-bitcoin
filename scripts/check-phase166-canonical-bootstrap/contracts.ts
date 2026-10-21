/** Fixed current-source boundaries; these are wiring guards, never runtime receipts. */
export const NODE = "packages/open-bitcoin-node/src/";
export const RPC = "packages/open-bitcoin-rpc/src/";
export const CLI = "packages/open-bitcoin-cli/src/";
export const CORE = "packages/open-bitcoin-chainstate/src/";
export const CONSENSUS = "packages/open-bitcoin-consensus/src/";
export const VECTOR = "docs/parity/fixtures/phase166-genesis-vectors.json";
export const DAEMON = RPC + "bin/open_bitcoind/canonical_startup.rs";
export const FACTORY = RPC + "context/canonical_genesis.rs";
export const OBSERVER = CLI + "operator/runtime/support/canonical_genesis.rs";
export const OPENER = NODE + "storage/fjall_store/canonical_genesis.rs";
export const INSPECTION = NODE + "storage/fjall_store/inspection.rs";
export const CHECKPOINT = NODE + "storage/fjall_store/coins.rs";
export type Contract = {
  file: string; symbol: string; rule: string; anchors: readonly string[];
  maybeTopLevel?: readonly string[]; maybeForbidden?: readonly string[];
};
const row = (file: string, symbol: string, rule: string, anchors: string[],
  maybeTopLevel?: string[], maybeForbidden?: string[]): Contract =>
  ({ file, symbol, rule, anchors, maybeTopLevel, maybeForbidden });

export const CONTRACTS: readonly Contract[] = [
  row(CONSENSUS + "chain_spec/bootstrap.rs", "prove", "genesis-proof", [
    "spec.check_genesis_block(body)?;", "crate::block_proof(body.header.bits)", "block: body.clone()", "work,",
  ], ["spec.check_genesis_block(body)?;"], ["return Ok("]),
  row(CORE + "engine/canonical_genesis.rs", "from_canonical_genesis", "complete-coins", [
    "let snapshot = coins.complete_snapshot()?;", "snapshot.maybe_best_block != Some(proof.hash())",
    "|| !snapshot.coins.is_empty()", "|| !snapshot.head_blocks.is_empty()",
    "return Err(ChainstateError::WorkRebindMismatch);", "proof.work()", "WorkAuthority::new(WorkOrigin::Live)",
    "work_authority.bind(Some(&position), consensus_params(proof))", "work_authority.accept(&binding)",
  ], ["let snapshot = coins.complete_snapshot()?;", `if snapshot.maybe_best_block != Some(proof.hash())
    || !snapshot.coins.is_empty() || !snapshot.head_blocks.is_empty() {
      return Err(ChainstateError::WorkRebindMismatch);
    }`], ["add_transaction_outputs", "apply_transaction", "return Ok("]),
  row(NODE + "storage/canonical_bootstrap/records.rs", "parse_rows", "strict-records", [
    "inventory(&namespaces)?;", "snapshots::decode_header_entries(header_bytes)",
    "strict_json(header_bytes, snapshots::encode_header_entries(&headers))?;",
    "snapshots::decode_chain_meta(meta_bytes)", "maybe_counts.ok_or(Refusal::MissingRecord)?",
    "if !counts.is_empty()", "Refusal::UnsupportedHistory", "coins_codec::decode_best_block_value",
    "parse_block(body_bytes)", "encode_block(&body)",
  ], ["inventory(&namespaces)?;"]),
  row(NODE + "storage/canonical_bootstrap/records.rs", "inventory", "no-genesis-coins", [
    '"coins" => key == "B",', "if !supported", "return Err(Refusal::UnsupportedRecord)",
  ]),
  row(NODE + "storage/canonical_bootstrap/proof.rs", "validate_genesis_records", "stored-authority", [
    "if records.chain != selected", "CanonicalGenesisProof::prove(selected.spec(), &records.body)",
    "if records.headers.len() != 1 || records.index.len() != 1 || records.active.len() != 1",
    "Refusal::UnsupportedHistory", "records.identity_hash != genesis.hash()",
    "records.best_block != genesis.hash()", "ProofWork::ZERO.wrapping_add(genesis.work())",
    "records.headers[0] != expected_header", "records.index[0] != expected_header",
    "records.active[0] != position", "records.coverage != history::ValidationCoverage::Complete",
    "validate_basic(&genesis, &position, &records.basic)?;",
  ], [
    "if records.chain != selected { return Err(Refusal::InconsistentGenesis); }",
    `if records.headers.len() != 1 || records.index.len() != 1 || records.active.len() != 1 {
      return Err(Refusal::UnsupportedHistory);
    }`,
    `if records.identity_hash != genesis.hash() || records.best_block != genesis.hash()
      || records.body_key != fjall_store::block_key(genesis.hash()) {
      return Err(Refusal::InconsistentGenesis);
    }`,
    `if records.headers[0] != expected_header || records.index[0] != expected_header
      || records.active[0] != position || records.coverage != history::ValidationCoverage::Complete
      || records.validation.status() != history::StoredValidationStatus::ScriptsValid
      || identity.hash() != genesis.hash() || identity.parent_hash() != position.previous_block_hash()
      || identity.height() != 0 { return Err(Refusal::InconsistentGenesis); }`,
  ], ["return Ok("]),
  row(NODE + "storage/canonical_bootstrap/proof.rs", "validate_basic", "stored-basic", [
    "filters::decode_state(state_bytes)", "filters::ownership::decode_owner(owner_bytes)",
    "state.fence_height != 0 || state.fence_hash != genesis.hash()", "state.protection.covers(required_protection)",
    "matches!(owner, IndexLifecycle::Active { .. }) && maybe_lock_protection.is_none()",
    "rows.len() != expected_count", "BasicFilterInputs::from_historical(genesis.block(), position, None)",
    "filters::StoredFilterRecord::generate(&inputs, position, None)", "if saved != expected",
    "filters::decode_projection(&key, bytes)", "!= genesis.hash()",
  ], [
    "if state.fence_height != 0 || state.fence_hash != genesis.hash() { return Err(Refusal::InconsistentIndex); }",
    "if !state.protection.covers(required_protection) { return Err(Refusal::InconsistentIndex); }",
    `if maybe_lock_protection.is_some_and(|protection| !protection.covers(state.protection))
      || matches!(owner, IndexLifecycle::Active { .. }) && maybe_lock_protection.is_none() {
      return Err(Refusal::InconsistentIndex);
    }`,
    "if rows.len() != expected_count { return Err(Refusal::InconsistentIndex); }",
    "if saved != expected { return Err(Refusal::InconsistentIndex); }",
  ]),
  row(OPENER, "into_parts", "consuming-owner", ["self.recovered.check(&self.store)?;", "Ok((self.store, self.recovered))"],
    ["self.recovered.check(&self.store)?;"], ["return Ok("]),
  row(NODE + "storage/fjall_store/opening.rs", "open_with_lease", "retained-store-lock", [
    "Database::builder(lease.source())", ".open()", "path: lease.source().to_path_buf()", "_lease: Arc::new(lease)",
  ]),
  row(OPENER, "check", "store-generation", [
    "!Arc::ptr_eq(&self.publication, &store.filter_publication)", "store.filter_publication_guard()",
    "store.db.seqno() != self.sequence", "prove_live(store, self.proof.genesis().chain())? != self.proof",
    "store.db.seqno() != self.sequence", "Ok(self.proof.genesis())",
  ], [
    "if !Arc::ptr_eq(&self.publication, &store.filter_publication) { return Err(Refusal::ChangedStore); }",
    `if store.db.seqno() != self.sequence || prove_live(store, self.proof.genesis().chain())? != self.proof
      || store.db.seqno() != self.sequence { return Err(Refusal::ChangedStore); }`,
  ], ["return Ok("]),
  row(OPENER, "open_selected", "atomic-publication", [
    "open_lease::OperationLease::acquire(path)", "inspect_datadir_without_backend(lease.source())",
    "bootstrap::prepare_genesis_rows(selected)?", "FjallNodeStore::open_with_lease(lease)",
    "store.db.batch().durability(Some(FjallPersistMode::SyncAll))", "batch.insert(keyspace, key, value)",
    "batch.commit().map_err(|_| Refusal::CommitUncertain)?;", "prove_live(&store, selected)?", "seal(store, proof)",
  ]),
  row(OPENER, "reopen", "pre-effect-proof", [
    "inspection::with_inspection(&lease, |records|", "prove_detached(records, selected)",
    "FjallNodeStore::open_with_lease(lease)", "prove_live(&store, selected)?", "if actual != expected",
    "return Err(Refusal::ChangedStore);", "seal(store, actual)",
  ], undefined, ["return Ok("]),
  row(INSPECTION, "with_inspection", "isolated-recovery", ["inspect_owned_then(", "lease,", "worker::recover,", "|records| validate(&records)"],
    ["inspect_owned_then("], ["raw_recover"]),
  row(INSPECTION, "inspect_owned_then", "cleanup-before-proof", [
    "let source = lease.source();", "lock_source(source)?", "copy::Inventory::capture(source)?",
    "ScratchGuard::create(source)?", "inventory.copy_to(source, &database_path)?",
    "match recover(&scratch)", "worker::Completion::Uncertain(error)", "reaped = false;",
    ".recheck(source)", "if !reaped", "scratch.cleanup()?;", "source_check?;", "recovered_check?;", "validate(result?)",
  ], ["scratch.cleanup()?;", "source_check?;", "recovered_check?;", "validate(result?)"]),
  row(INSPECTION, "lock_source", "engine-lock", ["identity.check_opened(&file, &path)?;", "file.try_lock()"]),
  row(NODE + "storage/fjall_store/inspection/worker.rs", "recover", "child-process", [
    "std::env::current_exe()", "Command::new(executable)", ".args(ARGS)", "command.spawn()",
    "monitor(&mut child, &job, started, &receiver)", "Completion::Uncertain(_)", "writer.join()",
  ], undefined, ["raw_recover("]),
  row(NODE + "storage/fjall_store/inspection/worker.rs", "finish", "child-result", [
    "if !status.success() || !delivered", "return Err(refused());", "protocol::read_result(&file, length, job)?",
  ]),
  row(NODE + "storage/fjall_store/inspection/worker.rs", "terminate", "child-reaping", [
    "child.try_wait()", "child.kill().is_err()", "child.wait()",
  ]),
  row(NODE + "chainstate/fjall_store/canonical_genesis.rs", "from_canonical_genesis", "recovery-after-proof", [
    "recovered.check(&store)", "initialize_configured(&store, now, now, 0, false, u64::MAX, mode)?",
    "Chainstate::from_canonical_genesis(&proof, cache)", "Self::from_recovered_chainstate(",
  ], undefined, ["return Ok("]),
  row(NODE + "sync/canonical_genesis.rs", "open_canonical_genesis_configured", "runtime-owner", [
    "if opened.genesis().chain() != selected", "opened.into_parts()", "basic::resolve_startup_mode(&store, mode)?",
    "ManagedChainstate::from_canonical_genesis(store.clone(), recovered, mode)?",
    "ManagedPeerNetwork::from_canonical_genesis_initialized(", "&proof,",
    "Self::finish_initialized_runtime(", "canonical_genesis::consensus_params(&proof)",
  ], undefined, ["from_snapshot", "open_configured(", "from_fixture", "return Ok("]),
  row(NODE + "sync/canonical_genesis/basic.rs", "resolve_startup_mode", "saved-basic-owner", [
    "matches!(requested, BasicFilterStartupMode::PreserveSaved)",
    "store.resolved_basic_filter_indexing_after_startup()? == Some(true)",
    "return Ok(BasicFilterStartupMode::Enabled);", "Ok(requested)",
  ]),
  row(CHECKPOINT, "save_validated_chain_meta", "checkpoint-counts", [
    "self.filter_publication_guard()?", ".get_bytes(StorageNamespace::Chainstate, CHAIN_META_KEY)?",
    ".map(|bytes| decode_chain_meta(&bytes))", ".transpose()?", "if stored.as_slice() == active_chain",
    "return maybe_counts;", "None", "self.check_validation_coverage_for_metadata(active_chain)?;",
    "control.invalidate_append()?;", "encode_chain_meta(active_chain, maybe_counts.as_ref())?",
    "self.put_bytes(StorageNamespace::Chainstate, CHAIN_META_KEY, bytes, mode)?;",
  ], undefined, ["unwrap_or_default", "Some(HashMap::new())"]),
  row(DAEMON, "preflight", "activation-before-open", [
    "runtime.validate_peer_block_filters()", "selected_canonical_chain(runtime)",
    "runtime.sync.is_enabled()", "runtime.inbound.enabled", "runtime.relay.enabled",
    "runtime.block_serving.block_serving.enabled", "runtime.block_serving.compact_relay.enabled",
    "if activation", ".require_ordinary_history()", "Ok(chain)",
  ], [`let activation = runtime.sync.is_enabled() || runtime.inbound.enabled || runtime.relay.enabled
    || runtime.block_serving.block_serving.enabled || runtime.block_serving.compact_relay.enabled;`]),
  row(DAEMON, "open_runtime_store", "daemon-sealed-opener", [
    "let chain = preflight(runtime)?;", "runtime.block_filter_index.is_explicit()", "if !has_durable_trigger",
    "return Ok(None);", "runtime.maybe_data_dir.as_ref()", "FjallNodeStore::open_canonical_genesis(data_dir, chain)",
  ], ["let chain = preflight(runtime)?;"], ["FjallNodeStore::open(", "open_ordinary("]),
  row(DAEMON, "open_authoritative_network_runtime", "daemon-consumption", [
    "preflight(runtime)?", "let Some(opened) = maybe_opened", "ManagedRpcContext::from_canonical_genesis_runtime_config(runtime, opened)",
    "assemble_durable_runtime(runtime, sync_runtime)?", "authoritative.maybe_prebuilt_context = Some(context)",
  ], undefined, ["from_fixture", "DurableSyncRuntime::open_configured"]),
  row(RPC + "bin/open-bitcoind.rs", "main", "pre-tokio-dispatch", [
    "FjallNodeStore::maybe_run_inspection_worker()", "std::process::exit(code)", "run_application()",
  ], ["run_application()"]),
  row(RPC + "bin/open-bitcoind.rs", "run_application", "daemon-wiring", [
    "load_runtime_config()?", "open_runtime_store(&runtime)?", "open_authoritative_network_runtime(&runtime, maybe_runtime_store)?",
    "serve_authoritative_runtime(",
  ]),
  row(RPC + "bin/open-bitcoind.rs", "serve_authoritative_runtime", "genesis-maintenance", [
    "let canonical_genesis = authoritative_runtime.maybe_prebuilt_context.is_some();",
    "match authoritative_runtime.maybe_prebuilt_context.take()", "Some(context) => context",
    "let maybe_checkpoint_worker = if canonical_genesis { None } else { start_mempool_checkpoint_worker(handle.clone(), store.clone()) };",
    "let maybe_coins_flush_worker = if canonical_genesis { None } else { start_coins_flush_worker(handle.clone(), store) };",
    "(!canonical_genesis).then(|| start_initial_broadcast_retry_worker(handle))",
    "tokio::net::TcpListener::bind(bind_address).await?",
  ]),
  row(FACTORY, "from_canonical_genesis_runtime_config", "sealed-rpc-factory", [
    "preflight(config, &opened)?;", "let params = consensus_params(opened.genesis());",
    "DurableSyncRuntime::open_canonical_genesis_configured(", "opened,", "startup_mode(config.block_filter_index)",
    "bound_config.maybe_data_dir = Some(runtime.store().datadir().to_path_buf());",
    "Self::assemble_context(", "Some(runtime.store().clone())", "WalletState::Unavailable",
    "context.consensus_params = params;", "Ok((context, runtime))",
  ], ["preflight(config, &opened)?;"], ["from_fixture", "build_wallet_state_with_store", "recover_mempool", "WalletState::Local", "return Ok("]),
  row(FACTORY, "preflight", "rpc-selected-history", [
    "selected_chain(config)?", "config.validate_peer_block_filters()", "opened.genesis().chain() != chain",
    "return Err(CanonicalContextError::ChainMismatch);", "config.sync.is_enabled()", "config.inbound.enabled",
    "config.relay.enabled", "config.block_serving.block_serving.enabled", "config.block_serving.compact_relay.enabled",
    "require_ordinary_scope(chain)?;",
  ], [
    "if opened.genesis().chain() != chain { return Err(CanonicalContextError::ChainMismatch); }",
    `if config.sync.is_enabled() || config.inbound.enabled || config.relay.enabled
      || config.block_serving.block_serving.enabled || config.block_serving.compact_relay.enabled {
      super::canonical_chain::require_ordinary_scope(chain)?;
    }`,
  ], ["return Ok("]),
  row(FACTORY, "startup_mode", "basic-mode", [
    "BasicFilterIndexSetting::Basic => BasicFilterStartupMode::Enabled",
    "BasicFilterIndexSetting::Disabled => BasicFilterStartupMode::Disabled",
    "BasicFilterIndexSetting::Unspecified => BasicFilterStartupMode::PreserveSaved",
  ]),
  row(CLI + "bin/open-bitcoin.rs", "main", "pre-parser-dispatch", [
    "FjallNodeStore::maybe_run_inspection_worker()", "std::process::exit(code)", "OperatorCli::parse()", "execute_operator_cli(cli)",
  ]),
  row(CLI + "operator/runtime/support.rs", "execute_offline_sync_command", "status-only", [
    "if matches!(args.command, SyncCommand::Status)", "return observe_canonical_genesis(data_dir, selected_chain, format);",
    "FjallNodeStore::open_ordinary(data_dir, selected_chain)",
  ]),
  row(OBSERVER, "observe_canonical_genesis", "existing-observation", [
    "FjallNodeStore::reopen_canonical_genesis_existing(data_dir, selected_chain)",
    "DurableSyncRuntime::open_canonical_genesis_configured(", "opened,", "BasicFilterStartupMode::PreserveSaved",
    "runtime.durable_sync_state(", "runtime.network_handle().operator_snapshot()",
  ], undefined, ["FjallNodeStore::open(", "FjallNodeStore::open_canonical_genesis(", "save_runtime_metadata", "from_fixture"]),
];

/** Complete finite predicates, refusal/success bodies and lexical depths for the
 * promised authority/recovery gates. Substring order alone cannot qualify them. */
export const DEPTH_GUARDS: readonly (readonly [string, string, string, string, number])[] = [
  [RPC + "bin/open-bitcoind.rs", "main", "pre-tokio-dispatch",
    "if let Some(code) = FjallNodeStore::maybe_run_inspection_worker() { std::process::exit(code); }", 0],
  [CLI + "bin/open-bitcoin.rs", "main", "pre-parser-dispatch",
    "if let Some(code) = open_bitcoin_node::FjallNodeStore::maybe_run_inspection_worker() { std::process::exit(code); }", 0],
  [NODE + "storage/canonical_bootstrap/records.rs", "parse_rows", "strict-records",
    "if !counts.is_empty() { return Err(Refusal::UnsupportedHistory); }", 0],
  [NODE + "storage/canonical_bootstrap/records.rs", "parse_rows", "strict-records",
    "if schema != b\"2\" { return Err(Refusal::MalformedRecord); }", 0],
  [NODE + "storage/canonical_bootstrap/records.rs", "parse_rows", "strict-records",
    "if encode_block(&body).map_err(|_| Refusal::MalformedRecord)? != body_bytes { return Err(Refusal::MalformedRecord); }", 0],
  [NODE + "storage/canonical_bootstrap/records.rs", "inventory", "no-genesis-coins",
    "if !supported { return Err(Refusal::UnsupportedRecord); }", 2],
  [NODE + "storage/canonical_bootstrap/proof.rs", "validate_basic", "stored-basic",
    "if filters::decode_projection(&key, bytes).map_err(|_| Refusal::InconsistentIndex)? != genesis.hash() { return Err(Refusal::InconsistentIndex); }", 0],
  [NODE + "storage/canonical_bootstrap/proof.rs", "validate_basic", "stored-basic", `let Some(state_bytes) = maybe_state else {
    if rows
      .keys()
      .any(|key| key.as_slice() != fjall_store::PRUNE_LOCKS_KEY.as_bytes())
      || maybe_lock.is_some()
    { return Err(Refusal::InconsistentIndex); }
    return Ok(());
  };`, 0],
  [NODE + "storage/canonical_bootstrap/proof.rs", "validate_basic", "stored-basic",
    "let Some(_) = state.maybe_endpoint else { return Ok(()); };", 0],
  [OPENER, "reopen", "pre-effect-proof", "if actual != expected { return Err(Refusal::ChangedStore); }", 0],
  [INSPECTION, "inspect_owned_then", "cleanup-before-proof", `if !reaped {
    if source_check.is_err() {
      return Err(inspection_error(
        "inspection child completion uncertain; source recheck failed",
      ));
    }
    return match result {
      Err(error) => Err(error),
      Ok(_) => Err(inspection_error("inspection child completion uncertain")),
    };
  }`, 0],
  [INSPECTION, "inspect_owned_then", "cleanup-before-proof", `let value = match recover(&scratch) {
    worker::Completion::Reaped(result) => result?,
    worker::Completion::Uncertain(error) => { reaped = false; return Err(error); }
  };`, 1],
  [INSPECTION, "inspect_owned_then", "cleanup-before-proof",
    "let source_check = inventory.recheck(source).and_then(|()| check_source_path(source));", 0],
  [NODE + "storage/fjall_store/inspection/worker.rs", "finish", "child-result",
    "if !status.success() || !delivered { return Err(refused()); }", 0],
  [NODE + "storage/fjall_store/inspection/worker.rs", "recover", "child-process",
    "if matches!(outcome, Completion::Uncertain(_)) { return outcome; }", 0],
  [NODE + "storage/fjall_store/inspection/worker.rs", "terminate", "child-reaping", `if child.kill().is_err() {
    return Completion::Uncertain(inspection_error(
      "inspection worker kill failed; completion uncertain",
    ));
  }`, 0],
  [NODE + "sync/canonical_genesis.rs", "open_canonical_genesis_configured", "runtime-owner", `if opened.genesis().chain() != selected {
    return Err(SyncRuntimeError::InvalidData { message: "canonical genesis selected network mismatch".to_owned(), });
  }`, 0],
  [NODE + "sync/canonical_genesis/basic.rs", "resolve_startup_mode", "saved-basic-owner",
    `if matches!(requested, BasicFilterStartupMode::PreserveSaved)
      && store.resolved_basic_filter_indexing_after_startup()? == Some(true) {
      return Ok(BasicFilterStartupMode::Enabled);
    }`, 0],
  [DAEMON, "preflight", "activation-before-open", `if activation {
    chain.spec().require_ordinary_history().map_err(|_| compatibility_refusal(runtime, chain))?;
  }`, 0],
  [DAEMON, "open_runtime_store", "daemon-sealed-opener", "if !has_durable_trigger { return Ok(None); }", 0],
  [CHECKPOINT, "save_validated_chain_meta", "checkpoint-counts", `.and_then(|(stored, maybe_counts)| {
    if stored.as_slice() == active_chain { return maybe_counts; }
    None
  });`, 0],
  [RPC + "context/wallet_state.rs", "selected_wallet_from_store", "wallet-unavailable",
    "if matches!(self.wallet_state, WalletState::Unavailable) { return Err(unavailable_wallet()); }", 0],
  [CLI + "operator/runtime/support.rs", "execute_offline_sync_command", "status-only",
    "if matches!(args.command, SyncCommand::Status) { return observe_canonical_genesis(data_dir, selected_chain, format); }", 0],
];

/** Mandatory first operations keep proof and dispatch ahead of early effects or exits. */
export const ENTRY_GUARDS: readonly (readonly [string, string, string, string])[] = [
  [CONSENSUS + "chain_spec/bootstrap.rs", "prove", "genesis-proof", "spec.check_genesis_block(body)?;"],
  [CORE + "engine/canonical_genesis.rs", "from_canonical_genesis", "complete-coins", "let snapshot = coins.complete_snapshot()?;"],
  [OPENER, "into_parts", "consuming-owner", "self.recovered.check(&self.store)?;"],
  [OPENER, "check", "store-generation", "if !Arc::ptr_eq(&self.publication, &store.filter_publication)"],
  [DAEMON, "open_runtime_store", "daemon-sealed-opener", "let chain = preflight(runtime)?;"],
  [DAEMON, "open_authoritative_network_runtime", "daemon-consumption", "let chain = preflight(runtime)?;"],
  [FACTORY, "from_canonical_genesis_runtime_config", "sealed-rpc-factory", "preflight(config, &opened)?;"],
  ...DEPTH_GUARDS.filter(row => row[1] === "main").map(([file, symbol, rule, body]) => [file, symbol, rule, body] as const),
];

/** Existing Rust behavioral tests stay executable; this inventory does not claim to run them. */
export const TESTS: readonly (readonly [string, string])[] = [
  [NODE + "storage/canonical_bootstrap/qualification_tests.rs", "phase166_qualification_independent_durable_authority_binding_matrix"],
  [NODE + "storage/canonical_bootstrap/qualification_tests.rs", "phase166_qualification_all_32_coordinated_work_bytes_recompute_from_actual_headers"],
  [NODE + "storage/canonical_bootstrap/qualification_tests.rs", "phase166_qualification_four_selector_managed_checkpoint_and_independent_reopen"],
  [NODE + "storage/fjall_store/tests/checkpoint_metadata.rs", "phase166_checkpoint_metadata_identical_ancestry_preserves_explicit_empty_counts"],
  [NODE + "storage/fjall_store/tests/checkpoint_metadata.rs", "phase166_checkpoint_metadata_identical_ancestry_preserves_nonempty_counts"],
  [NODE + "storage/fjall_store/tests/checkpoint_metadata.rs", "phase166_checkpoint_metadata_changed_ancestry_discards_counts_even_when_tip_agrees"],
  [NODE + "storage/fjall_store/tests/checkpoint_metadata.rs", "phase166_checkpoint_metadata_missing_or_legacy_counts_are_never_inferred_empty"],
  [NODE + "storage/fjall_store/tests/checkpoint_metadata.rs", "phase166_checkpoint_metadata_malformed_bytes_fail_before_replacement_or_invalidation"],
  [NODE + "storage/fjall_store/inspection/worker/tests.rs", "phase166_inspection_worker_valid_records_only_after_cleanup_and_retained_locks"],
  [NODE + "storage/fjall_store/inspection/worker/tests.rs", "phase166_inspection_worker_transport_exit_protocol_failures_block_callback"],
  [NODE + "storage/fjall_store/inspection/worker/tests.rs", "phase166_inspection_worker_cleanup_failure_blocks_successful_records"],
  [NODE + "sync/canonical_genesis/basic_tests.rs", "phase166_basic_enabled_uses_full_genesis_and_four_independent_vectors"],
  [NODE + "sync/canonical_genesis/basic_tests.rs", "phase166_basic_disabled_and_preserve_absence_create_no_index_rows_or_owner"],
  [NODE + "sync/canonical_genesis/basic_tests.rs", "phase166_basic_preserve_saved_restores_actual_accepted_owner_after_all_handle_reopen"],
  [RPC + "context/canonical_genesis/tests.rs", "phase166_genesis_context_actual_all_handles_reopen"],
  [RPC + "context/canonical_genesis/tests.rs", "phase166_genesis_context_wallet_http_and_dispatch_refuse_without_effects"],
];

export const REGISTRATIONS: readonly (readonly [string, string])[] = [
  [CONSENSUS + "chain_spec.rs", "mod bootstrap;"],
  [CORE + "engine.rs", "pub mod canonical_genesis;"],
  [NODE + "storage/canonical_bootstrap.rs", "#[cfg(test)] mod tests;"],
  [NODE + "storage/fjall_store.rs", "mod open_lease; mod opening;"],
  [NODE + "storage/canonical_bootstrap/tests.rs", '#[path = "qualification_tests.rs"] mod qualification_tests;'],
  [NODE + "storage/fjall_store/tests/coins_migration.rs", '#[path = "checkpoint_metadata.rs"] mod checkpoint_metadata;'],
  [NODE + "sync/canonical_genesis.rs", "#[cfg(test)] mod basic_tests;"],
  [RPC + "context.rs", "mod canonical_genesis;"],
  [RPC + "bin/open-bitcoind.rs", '#[path = "open_bitcoind/canonical_startup.rs"] mod canonical_startup;'],
  [RPC + "bin/open-bitcoind.rs", "use canonical_startup::{open_authoritative_network_runtime, open_runtime_store};"],
  [CLI + "operator/runtime/support.rs", "mod canonical_genesis; use canonical_genesis::observe_canonical_genesis;"],
  [NODE + "lib.rs", "#[cfg(test)] register_inspection_worker!();"],
  ...[RPC + "lib.rs", CLI + "lib.rs", RPC + "bin/open-bitcoind.rs", CLI + "bin/open-bitcoin.rs"].map(file =>
    [file, "#[cfg(test)] open_bitcoin_node::register_inspection_worker!();"] as const),
];

export const EXTRA_FILES = [
  NODE + "storage/fjall_store/open_lease.rs", RPC + "context/wallet_state.rs",
  CLI + "operator/wallet.rs", CLI + "operator/wallet/name_resolution.rs", CLI + "operator/soak/runtime/chain_compatibility.rs",
];
