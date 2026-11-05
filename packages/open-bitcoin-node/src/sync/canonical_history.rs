// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

//! Consuming safe startup for completely replayed canonical history.

use super::{DurableSyncRuntime, SyncNetwork, SyncRuntimeConfig, SyncRuntimeError, progress};
use crate::{
    ManagedChainstate, ManagedPeerNetwork, chainstate::BasicFilterStartupMode,
    storage::fjall_store::canonical_genesis::OpenedCanonicalHistory,
};

impl DurableSyncRuntime {
    /// Consume a complete original-store history proof before runtime effects.
    pub fn open_canonical_history_configured(
        opened: OpenedCanonicalHistory,
        config: SyncRuntimeConfig,
        mode: BasicFilterStartupMode,
    ) -> Result<Self, SyncRuntimeError> {
        use open_bitcoin_core::consensus::CanonicalChain;
        let selected = match config.network {
            SyncNetwork::Mainnet => CanonicalChain::Mainnet,
            SyncNetwork::Testnet => CanonicalChain::Testnet3,
            SyncNetwork::Signet => CanonicalChain::Signet,
            SyncNetwork::Regtest => CanonicalChain::Regtest,
        };
        if opened.selected() != selected {
            return Err(invalid("canonical history selected network mismatch"));
        }
        let (store, recovered) = opened
            .into_parts()
            .map_err(|_| invalid("canonical history store proof changed"))?;
        // Resolve saved mode without writes; original full proof already exists.
        let mode = super::canonical_genesis::resolve_history_startup_mode(&store, mode)?;
        let (managed, genesis, mut headers) =
            ManagedChainstate::from_canonical_history(store.clone(), recovered, mode)?;
        let params =
            open_bitcoin_core::chainstate::engine::canonical_genesis::consensus_params(&genesis);
        let state = managed.chainstate();
        state
            .validate_live_origin()
            .map_err(|_| invalid("canonical history core origin mismatch"))?;
        state
            .bind_live_work_source(params)
            .map_err(|_| invalid("canonical history parameter mismatch"))?;
        let tip = state
            .tip()
            .ok_or_else(|| invalid("canonical history active tip absent"))?
            .clone();
        let active: Vec<_> = state
            .active_chain()
            .iter()
            .map(|position| position.block_hash)
            .collect();
        if genesis.chain() != selected
            || state.maybe_canonical_chain() != Some(selected)
            || headers.maybe_canonical_chain() != Some(selected)
            || headers.entry(&tip.block_hash).is_none_or(|entry| {
                entry.header != tip.header
                    || entry.height != tip.height
                    || entry.chain_work != tip.chain_work
            })
        {
            return Err(invalid("canonical history owner correspondence mismatch"));
        }
        if active.len() == 1
            && tip.height == 0
            && tip.block_hash == genesis.hash()
            && tip.header == *genesis.header()
            && tip.chain_work == genesis.work()
        {
            let mut network = root_network(
                managed,
                progress::local_peer_config(&config),
                config.max_blocks_in_flight_per_peer,
                &genesis,
                headers,
            )?;
            network.record_basic_indexing_configuration(
                store.resolved_basic_filter_indexing_after_startup()?,
            );
            return Self::finish_initialized_runtime(store, network, config, params);
        }
        let body = store
            .load_block(tip.block_hash)?
            .ok_or_else(|| invalid("canonical history proved tip body absent"))?;
        let observed = headers
            .canonical_header_validation_context(&body.header)
            .map_err(|_| invalid("canonical history held context absent"))?
            .current_time;
        // Genesis uses its existing distinct proof boundary, without asking
        // descendant context derivation to fabricate a parent for the root.
        headers.check_body_eligibility(genesis.block(), params, observed)?;
        let local = progress::local_peer_config(&config);
        let expected_magic = match selected {
            CanonicalChain::Mainnet => open_bitcoin_core::primitives::NetworkMagic::MAINNET,
            CanonicalChain::Testnet3 => {
                open_bitcoin_core::primitives::NetworkMagic::from_bytes([0x0b, 0x11, 0x09, 0x07])
            }
            CanonicalChain::Signet => {
                open_bitcoin_core::primitives::NetworkMagic::from_bytes([0x0a, 0x03, 0xcf, 0x40])
            }
            CanonicalChain::Regtest => {
                open_bitcoin_core::primitives::NetworkMagic::from_bytes([0xfa, 0xbf, 0xb5, 0xda])
            }
        };
        if local.magic != expected_magic {
            return Err(invalid("canonical history local network mismatch"));
        }
        let mut network = ManagedPeerNetwork::from_initialized_chainstate(
            managed,
            local,
            open_bitcoin_mempool::PolicyConfig::default(),
            config.max_blocks_in_flight_per_peer,
            open_bitcoin_network::RelayActivationConfig::default(),
            open_bitcoin_network::BlockRelayActivationPolicy::default(),
            false,
        )
        .into_canonical_scope(selected);
        network.peer_manager_mut().seed_header_store(headers);
        // Body linkage receipts require the real checked ancestors, independently
        // of header knowledge. Recheck those actual proved bytes in active order.
        for hash in active
            .into_iter()
            .filter(|hash| *hash != tip.block_hash && *hash != genesis.hash())
        {
            let ancestor = store
                .load_block(hash)?
                .ok_or_else(|| invalid("canonical history active body absent"))?;
            let now = if hash == genesis.hash() {
                observed
            } else {
                network
                    .peer_manager()
                    .header_store()
                    .canonical_header_validation_context(&ancestor.header)
                    .map_err(|_| invalid("canonical history active body context absent"))?
                    .current_time
            };
            network.retain_checked_candidate(&ancestor, params, now)?;
        }
        network.retain_checked_candidate(&body, params, observed)?;
        if network
            .maybe_body_eligibility(tip.block_hash)
            .is_none_or(|facts| {
                !facts.available
                    || facts.failed
                    || facts.chain_work != tip.chain_work
                    || facts.maybe_sequence.is_none()
            })
        {
            return Err(invalid(
                "canonical history tip eligibility/cache was not earned",
            ));
        }
        network.record_basic_indexing_configuration(
            store.resolved_basic_filter_indexing_after_startup()?,
        );
        Self::finish_initialized_runtime(store, network, config, params)
    }
}

fn root_network(
    managed: ManagedChainstate<crate::FjallChainstateStore, crate::FjallCoinsView>,
    local: open_bitcoin_network::LocalPeerConfig,
    max_blocks: usize,
    proof: &open_bitcoin_core::consensus::chain_spec::CanonicalGenesisProof,
    mut headers: open_bitcoin_network::HeaderStore,
) -> Result<ManagedPeerNetwork<crate::FjallChainstateStore, crate::FjallCoinsView>, SyncRuntimeError>
{
    let mut network =
        ManagedPeerNetwork::from_canonical_genesis_initialized(managed, local, max_blocks, proof)?;
    let maybe_known = headers
        .entries()
        .find(|entry| entry.block_hash != proof.hash())
        .cloned();
    if let Some(known) = maybe_known {
        let observed = headers
            .canonical_header_validation_context(&known.header)?
            .current_time;
        #[cfg(test)]
        let before = headers.clone();
        headers.check_body_eligibility(
            proof.block(),
            open_bitcoin_core::chainstate::engine::canonical_genesis::consensus_params(proof),
            observed,
        )?;
        #[cfg(test)]
        let transferred = headers.clone();
        network.peer_manager_mut().seed_header_store(headers);
        #[cfg(test)]
        {
            assert_eq!(network.peer_manager().header_store(), &transferred);
            root_tests::assert_context_transfer(
                &before,
                network.peer_manager().header_store(),
                proof.hash(),
            );
        }
    }
    Ok(network)
}

fn invalid(message: &str) -> SyncRuntimeError {
    SyncRuntimeError::InvalidData {
        message: message.into(),
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod root_tests {
    use super::*;
    use crate::{FjallNodeStore, chainstate::validation_history::tests::phase167 as genuine};
    use open_bitcoin_core::{
        chainstate::{CoinsView, FlushMode, FlushPolicyTime},
        consensus::{CanonicalChain, ScriptVerifyFlags, block_hash},
        primitives::{Amount, Block},
    };

    pub(super) fn assert_context_transfer(
        before: &open_bitcoin_network::HeaderStore,
        after: &open_bitcoin_network::HeaderStore,
        root: open_bitcoin_core::primitives::BlockHash,
    ) {
        assert_eq!(
            before.entries().collect::<Vec<_>>(),
            after.entries().collect::<Vec<_>>()
        );
        for entry in before.entries().filter(|entry| entry.block_hash != root) {
            assert_eq!(
                before
                    .canonical_header_validation_context(&entry.header)
                    .expect("original observation"),
                after
                    .canonical_header_validation_context(&entry.header)
                    .expect("same transferred observation")
            );
        }
        eprintln!(
            "ACTIVE_ROOT_HEADER_HANDOFF known={} entire_store_exact=true held_contexts_exact=true",
            before.entries().count()
        );
    }

    fn admit(runtime: &DurableSyncRuntime, block: &Block) {
        runtime
            .network
            .receive_message(
                1,
                open_bitcoin_network::WireNetworkMessage::Headers(
                    open_bitcoin_network::HeadersMessage {
                        headers: vec![block.header.clone()],
                    },
                ),
                i64::MIN,
                ScriptVerifyFlags::NONE,
                runtime.consensus_params,
            )
            .expect("real contextual header admission");
    }

    fn flush(runtime: &DurableSyncRuntime) {
        runtime
            .network
            .persist_validation_header_snapshot()
            .expect("trusted snapshot");
        runtime
            .network
            .flush_coins(
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(60_001),
                u64::MAX,
            )
            .expect("complete root or descendant checkpoint");
    }

    fn checkpoint(known: bool, mode: BasicFilterStartupMode, preserve: bool) {
        // Arrange: actual root runtime, actual header ingress and trusted persistence.
        let mut nonce = [0; 16];
        getrandom::fill(&mut nonce).expect("disposable source");
        let root = std::fs::canonicalize(std::env::temp_dir())
            .expect("physical parent")
            .join(format!(
                "phase167-root-reopen-{:x}",
                u128::from_le_bytes(nonce)
            ));
        let config = SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            ..Default::default()
        };
        let opened = FjallNodeStore::open_canonical_genesis(&root, CanonicalChain::Regtest)
            .expect("full canonical root proof");
        let runtime =
            DurableSyncRuntime::open_canonical_genesis_configured(opened, config.clone(), mode)
                .expect("actual canonical genesis runtime");
        runtime
            .network
            .connect_outbound_peer(1, 0)
            .expect("socket-free peer");
        let genesis = CanonicalChain::Regtest.spec().genesis_block();
        let accepted = runtime
            .network
            .chainstate_snapshot()
            .expect("root accepted state");
        let mut bad = genuine::child(&genesis.header, 1);
        bad.transactions[0].outputs[0].value =
            Amount::from_sats(5_000_000_001).expect("bad reward");
        genuine::remine(&mut bad);
        if known {
            admit(&runtime, &bad);
            runtime
                .store
                .save_block(&bad, crate::PersistMode::Sync)
                .expect("candidate body");
            let error = runtime
                .network
                .connect_local_block(&bad, ScriptVerifyFlags::NONE, runtime.consensus_params)
                .expect_err("real Coins reward validation");
            assert!(error.to_string().contains("bad-cb-amount"), "{error}");
            assert_eq!(
                runtime
                    .network
                    .chainstate_snapshot()
                    .expect("unaccepted bad reward"),
                accepted
            );
        }
        flush(&runtime);
        let enabled = mode == BasicFilterStartupMode::Enabled;
        tests::drain(&runtime.network, enabled, 0);
        let expected = runtime
            .network
            .chainstate_snapshot()
            .expect("complete root state");
        assert_eq!(expected.active_chain.len(), 1);
        assert!(expected.utxos.is_empty() && expected.undo_by_block.is_empty());
        assert!(
            expected
                .maybe_confirmed_txid_counts
                .as_ref()
                .expect("complete root counts")
                .is_empty()
        );
        let coins = runtime
            .store
            .coins_view()
            .complete_snapshot()
            .expect("complete Coins");
        let best = runtime.store.coins_view().best_block().expect("B");
        let entries = runtime
            .network
            .header_entries()
            .expect("all earned headers");
        assert_eq!(entries.len(), if known { 2 } else { 1 });
        let facts: Vec<_> = entries
            .iter()
            .map(|entry| {
                (
                    entry.block_hash,
                    runtime
                        .store
                        .maybe_validation_history_record(entry.block_hash)
                        .expect("status"),
                    runtime
                        .store
                        .validation_provenance(entry.block_hash)
                        .expect("provenance"),
                    runtime.store.load_undo(entry.block_hash).expect("undo"),
                    runtime
                        .store
                        .load_basic_filter_record(entry.block_hash)
                        .expect("filter"),
                )
            })
            .collect();
        if known {
            assert_eq!(
                facts
                    .iter()
                    .find(|row| row.0 == block_hash(&bad.header))
                    .expect("known status")
                    .1
                    .as_ref()
                    .expect("stored")
                    .status(),
                crate::storage::validation_history::StoredValidationStatus::KnownOnly
            );
        }
        let basic = runtime
            .store
            .maybe_basic_filter_state()
            .expect("BASIC state");
        let protection = runtime
            .store
            .load_prune_protection()
            .expect("retention")
            .locks;
        let endpoints = runtime
            .network
            .maybe_basic_index_progress()
            .expect("progress")
            .map(|progress| {
                (
                    progress.maybe_processed_endpoint(),
                    progress.maybe_safe_durable_endpoint(),
                )
            });
        drop(runtime);
        // Act: public safe producer proves copied and ORIGINAL data after every handle dropped.
        let requested = if preserve {
            BasicFilterStartupMode::PreserveSaved
        } else {
            mode
        };
        let result =
            DurableSyncRuntime::reopen_canonical_history_configured(&root, config, requested);
        eprintln!(
            "ACTIVE_ROOT known={known} mode={mode:?} preserve={preserve} error={:?}",
            result.as_ref().err()
        );
        let runtime = result.expect("genuine root checkpoint must reopen");
        // Assert: complete accepted/durable facts and all known metadata survive.
        assert_eq!(
            runtime.network.chainstate_snapshot().expect("accepted"),
            expected
        );
        assert_eq!(
            runtime
                .store
                .coins_view()
                .complete_snapshot()
                .expect("Coins"),
            coins
        );
        assert_eq!(runtime.store.coins_view().best_block().expect("B"), best);
        assert_eq!(
            runtime.network.header_entries().expect("all known"),
            entries
        );
        assert_eq!(
            runtime.store.maybe_basic_filter_state().expect("BASIC"),
            basic
        );
        assert_eq!(
            runtime
                .store
                .load_prune_protection()
                .expect("retention")
                .locks,
            protection
        );
        assert_eq!(
            runtime
                .network
                .maybe_basic_index_progress()
                .expect("progress")
                .map(|progress| (
                    progress.maybe_processed_endpoint(),
                    progress.maybe_safe_durable_endpoint()
                )),
            endpoints
        );
        for (hash, status, provenance, undo, filter) in facts {
            assert_eq!(
                runtime
                    .store
                    .maybe_validation_history_record(hash)
                    .expect("status"),
                status
            );
            assert_eq!(
                runtime
                    .store
                    .validation_provenance(hash)
                    .expect("provenance"),
                provenance
            );
            assert_eq!(runtime.store.load_undo(hash).expect("undo"), undo);
            assert_eq!(
                runtime
                    .store
                    .load_basic_filter_record(hash)
                    .expect("filter"),
                filter
            );
        }
        let manager = runtime
            .network
            .peer_manager_snapshot()
            .expect("actual owner snapshot");
        for entry in entries
            .iter()
            .filter(|entry| entry.block_hash != block_hash(&genesis.header))
        {
            assert!(
                manager
                    .header_store()
                    .is_canonically_admitted(entry.block_hash)
            );
            assert!(
                manager
                    .header_store()
                    .canonical_header_validation_context(&entry.header)
                    .is_ok()
            );
        }
        drop(manager);
        assert!(
            !runtime
                .network
                .relay_activation_enabled()
                .expect("default activation")
        );
        runtime
            .network
            .connect_outbound_peer(1, 0)
            .expect("new peer");
        admit(&runtime, &bad);
        let before_bad = runtime
            .network
            .chainstate_snapshot()
            .expect("accepted root");
        let basic_before_bad = runtime
            .store
            .maybe_basic_filter_state()
            .expect("BASIC before refusal");
        let error = runtime
            .network
            .connect_local_block(&bad, ScriptVerifyFlags::NONE, runtime.consensus_params)
            .expect_err("invalid first reward remains rejected");
        assert!(error.to_string().contains("bad-cb-amount"), "{error}");
        assert_eq!(
            runtime
                .network
                .chainstate_snapshot()
                .expect("unchanged root"),
            before_bad
        );
        assert_eq!(
            runtime
                .store
                .maybe_basic_filter_state()
                .expect("unchanged BASIC"),
            basic_before_bad
        );
        let next = genuine::child(&genesis.header, 1);
        admit(&runtime, &next);
        runtime
            .store
            .save_block(&next, crate::PersistMode::Sync)
            .expect("valid next body");
        runtime
            .network
            .connect_local_block(&next, ScriptVerifyFlags::NONE, runtime.consensus_params)
            .expect("actual next Coins acceptance");
        let after = runtime
            .network
            .chainstate_snapshot()
            .expect("accepted child");
        assert_eq!(
            after.tip().expect("tip").block_hash,
            block_hash(&next.header)
        );
        assert_eq!(after.active_chain.len(), 2);
        assert_eq!(
            after.tip().expect("tip").chain_work,
            expected.tip().expect("root").chain_work.wrapping_add(
                open_bitcoin_core::consensus::chain_proof::block_proof(next.header.bits)
                    .expect("actual work")
            )
        );
        flush(&runtime);
        tests::drain(&runtime.network, enabled, 1);
        assert_eq!(
            runtime
                .store
                .validation_provenance(block_hash(&next.header))
                .expect("next provenance"),
            crate::storage::validation_history::ValidationProvenance::ScriptsValid
        );
        drop(runtime);
        std::fs::remove_dir_all(root).expect("all handles released");
    }

    #[test]
    fn phase167_runtime_reopen_active_root_only() {
        for mode in [
            BasicFilterStartupMode::Disabled,
            BasicFilterStartupMode::Enabled,
        ] {
            for preserve in [false, true] {
                checkpoint(false, mode, preserve);
            }
        }
    }

    #[test]
    fn phase167_runtime_reopen_active_root_known_only() {
        for mode in [
            BasicFilterStartupMode::Disabled,
            BasicFilterStartupMode::Enabled,
        ] {
            for preserve in [false, true] {
                checkpoint(true, mode, preserve);
            }
        }
    }
}
