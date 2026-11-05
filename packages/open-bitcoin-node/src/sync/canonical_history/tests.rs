// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp

use super::*;
use crate::{FjallNodeStore, chainstate::validation_history::tests::phase167 as genuine};
use open_bitcoin_core::{
    chainstate::{FlushMode, FlushPolicyTime},
    consensus::{CanonicalChain, ScriptVerifyFlags},
};

pub(super) type Handle =
    crate::ManagedNetworkHandle<crate::FjallChainstateStore, crate::FjallCoinsView>;

pub(super) fn drain(handle: &Handle, enabled: bool, tip: u32) {
    if !enabled {
        assert!(
            handle
                .drive_basic_filter_index_turn()
                .expect("disabled turn")
                .maybe_progress
                .is_none()
        );
        return;
    }
    let mut maybe_previous = None;
    for _ in 0..306 {
        let turn = handle
            .drive_basic_filter_index_turn()
            .expect("bounded ordinary turn");
        let height = turn
            .maybe_progress
            .expect("enabled")
            .maybe_processed_endpoint()
            .expect("processed")
            .height();
        assert!(maybe_previous.is_none_or(|previous| height > previous));
        maybe_previous = Some(height);
        if turn.maybe_accepted_lag == Some(0) {
            break;
        }
    }
    let progress = handle
        .maybe_basic_index_progress()
        .expect("progress")
        .expect("enabled");
    assert_eq!(
        progress
            .maybe_processed_endpoint()
            .expect("processed")
            .height(),
        tip
    );
    assert_eq!(
        progress.maybe_processed_endpoint(),
        progress.maybe_safe_durable_endpoint()
    );
}

fn add_spend(
    block: &mut open_bitcoin_core::primitives::Block,
    funding: &open_bitcoin_core::primitives::Block,
) {
    use open_bitcoin_core::{
        consensus::transaction_txid,
        primitives::{Amount, OutPoint, ScriptBuf},
    };
    let mut spend = funding.transactions[0].clone();
    spend.inputs[0].previous_output = OutPoint {
        txid: transaction_txid(&spend).expect("funded identity"),
        vout: 0,
    };
    spend.inputs[0].script_sig = ScriptBuf::from_bytes(Vec::new()).expect("empty script");
    spend.lock_time = 0;
    spend.outputs[0].value =
        Amount::from_sats(funding.transactions[0].outputs[0].value.to_sats() - 1)
            .expect("one sat fee");
    block.transactions.push(spend);
    block.transactions[0].outputs[0].value =
        Amount::from_sats(block.transactions[0].outputs[0].value.to_sats() + 1)
            .expect("reward plus fee");
    genuine::remine(block);
}

fn reopen_checkpoint(mode: BasicFilterStartupMode, replacement: bool) {
    use open_bitcoin_core::{
        chainstate::AnchoredBlock,
        consensus::{block_hash, transaction_txid},
        primitives::Amount,
    };
    genuine::with_mode(mode, |manager, store, params| {
        // Arrange: actual selected rules, ordinary maturity and both halvings.
        let root = store.datadir().to_path_buf();
        let mut network = genuine::network(manager);
        let mut blocks = vec![CanonicalChain::Regtest.spec().genesis_block()];
        for height in 1..=301 {
            let mut block = genuine::child(&blocks.last().expect("parent").header, height);
            assert_eq!(
                block.transactions[0].outputs[0].value,
                Amount::from_sats(5_000_000_000 >> (height / 150)).expect("selected subsidy")
            );
            if height == 101 {
                add_spend(&mut block, &blocks[1]);
            }
            genuine::admit(&mut network, std::slice::from_ref(&block), params);
            store
                .save_block(&block, crate::PersistMode::Sync)
                .expect("retained actual body");
            network
                .connect_local_block(&block, ScriptVerifyFlags::NONE, params)
                .expect("ordinary full acceptance");
            blocks.push(block);
        }
        network
            .persist_validation_header_snapshot()
            .expect("earned header checkpoint");
        network
            .chainstate_mut()
            .flush_with_mode(
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(60_001),
                u64::MAX,
            )
            .expect("complete checkpoint");
        let original = network.chainstate_snapshot().expect("original");
        let network = crate::ManagedNetworkHandle::new(network);
        let enabled = mode == BasicFilterStartupMode::Enabled;
        if enabled {
            network
                .initialize_basic_filter_index_owner()
                .expect("actual BASIC owner");
        }
        drain(&network, enabled, 301);
        if replacement {
            let mut branch = Vec::new();
            let mut parent = blocks[299].header.clone();
            for height in 300..=302 {
                let mut block = genuine::child(&parent, height);
                block.header.time += 1000;
                block.transactions[0].lock_time += 10000;
                genuine::remine(&mut block);
                store
                    .save_block(&block, crate::PersistMode::Sync)
                    .expect("fork body");
                parent = block.header.clone();
                branch.push(block);
            }
            network
                .receive_message(
                    1,
                    open_bitcoin_network::WireNetworkMessage::Headers(
                        open_bitcoin_network::HeadersMessage {
                            headers: branch.iter().map(|block| block.header.clone()).collect(),
                        },
                    ),
                    i64::MIN,
                    ScriptVerifyFlags::NONE,
                    params,
                )
                .expect("actual fork header ingress");
            let headers = network.header_entries().expect("earned fork work");
            let anchored: Vec<_> = branch
                .iter()
                .map(|block| AnchoredBlock {
                    block: block.clone(),
                    chain_work: headers
                        .iter()
                        .find(|entry| entry.block_hash == block_hash(&block.header))
                        .expect("actual admitted work")
                        .chain_work,
                })
                .collect();
            let disconnected: Vec<_> = blocks[300..].iter().rev().cloned().collect();
            let transition = network
                .reorg_to_branch(
                    &disconnected,
                    &anchored,
                    open_bitcoin_mempool::ReorgLifecycleContext::new(
                        open_bitcoin_mempool::PolicyTime::from_unix_seconds(i64::MIN),
                    ),
                    ScriptVerifyFlags::NONE,
                    params,
                )
                .expect("actual preferred replacement");
            assert_eq!(transition.disconnected.len(), 2);
            assert_eq!(transition.connected.len(), 3);
            let active = network.chainstate_snapshot().expect("replacement");
            assert!(
                active.tip().expect("new").chain_work > original.tip().expect("old").chain_work
            );
            assert_ne!(
                active.tip().expect("new").median_time_past,
                original.tip().expect("old").median_time_past
            );
            for block in &disconnected {
                assert!(
                    !active
                        .maybe_confirmed_txid_counts
                        .as_ref()
                        .expect("counts")
                        .contains_key(
                            &transaction_txid(&block.transactions[0]).expect("removed occurrence")
                        )
                );
            }
            network
                .persist_validation_header_snapshot()
                .expect("replacement trusted headers");
            network
                .flush_coins(
                    FlushMode::Always,
                    FlushPolicyTime::from_unix_seconds(60_002),
                    u64::MAX,
                )
                .expect("replacement checkpoint");
            drain(&network, true, 302);
        }
        let expected = network
            .chainstate_snapshot()
            .expect("complete actual active state");
        let entries = network.header_entries().expect("all earned headers");
        assert_eq!(entries.len(), if replacement { 305 } else { 302 });
        assert_eq!(
            expected.active_chain.len(),
            if replacement { 303 } else { 302 }
        );
        assert_eq!(
            expected.undo_by_block.len(),
            if replacement { 302 } else { 301 }
        );
        let mut physical = expected.clone();
        physical.undo_by_block.extend(original.undo_by_block);
        assert_eq!(
            physical.undo_by_block.len(),
            if replacement { 304 } else { 301 }
        );
        assert_eq!(
            store
                .wallet_scan_chainstate_snapshot()
                .expect("stored")
                .expect("checkpoint"),
            physical
        );
        let spent = transaction_txid(&blocks[1].transactions[0]).expect("spent coinbase");
        assert!(!expected.utxos.keys().any(|outpoint| outpoint.txid == spent));
        assert_eq!(
            expected
                .maybe_confirmed_txid_counts
                .as_ref()
                .expect("counts")
                .get(&spent),
            Some(&1)
        );
        let basic = store
            .maybe_basic_filter_state()
            .expect("complete BASIC state");
        let statuses: Vec<_> = entries
            .iter()
            .map(|entry| {
                (
                    entry.block_hash,
                    store
                        .maybe_validation_history_record(entry.block_hash)
                        .expect("status"),
                    store
                        .validation_provenance(entry.block_hash)
                        .expect("provenance"),
                    store
                        .load_basic_filter_record(entry.block_hash)
                        .expect("complete retained BASIC record"),
                )
            })
            .collect();
        let projections: Vec<_> = if enabled {
            expected
                .active_chain
                .iter()
                .map(|position| {
                    store
                        .basic_filter_projection(position.height)
                        .expect("every active binding")
                })
                .collect()
        } else {
            Vec::new()
        };
        // All live handles drop before actual production copied/original reproof.
        drop(network);
        drop(store);
        let measured = crate::storage::fjall_store::measure_detached_rows_for_test(&root)
            .expect("complete physical/logical measurements");
        assert!(measured.manifest_equal && measured.authority_max <= 65536);
        assert!(measured.logical_entries <= 4096 && measured.physical_entries <= 4096);
        assert!(measured.logical_bytes <= 536870912 && measured.physical_bytes <= 536870912);
        assert_eq!(measured.copy_bytes, measured.physical_bytes);
        // Act
        let started = std::time::Instant::now();
        let runtime = DurableSyncRuntime::reopen_canonical_history_configured(
            &root,
            SyncRuntimeConfig {
                network: super::super::SyncNetwork::Regtest,
                ..Default::default()
            },
            BasicFilterStartupMode::PreserveSaved,
        )
        .expect("actual product history reopen");
        let (stage, child) = FjallNodeStore::history_worker_observation_for_test();
        // Assert complete state unchanged by recovery and body cache preparation.
        assert_eq!(stage, Some(2));
        assert!(child < std::time::Duration::from_secs(30));
        assert_eq!(
            runtime
                .network
                .chainstate_snapshot()
                .expect("all reopened core state"),
            expected
        );
        assert_eq!(
            runtime
                .store
                .wallet_scan_chainstate_snapshot()
                .expect("all durable state")
                .expect("present"),
            physical
        );
        assert_eq!(
            runtime
                .network
                .header_entries()
                .expect("all actual headers"),
            entries
        );
        assert_eq!(
            runtime
                .store
                .maybe_basic_filter_state()
                .expect("BASIC preserved"),
            basic
        );
        for (hash, status, provenance, filter) in statuses {
            assert_eq!(
                runtime
                    .store
                    .maybe_validation_history_record(hash)
                    .expect("same status"),
                status
            );
            assert_eq!(
                runtime
                    .store
                    .validation_provenance(hash)
                    .expect("same provenance"),
                provenance
            );
            assert_eq!(
                runtime
                    .store
                    .load_basic_filter_record(hash)
                    .expect("same complete filter"),
                filter
            );
            assert_eq!(
                runtime
                    .store
                    .load_undo(hash)
                    .expect("retained physical undo"),
                physical.undo_by_block.get(&hash).cloned()
            );
        }
        for (height, hash) in projections.into_iter().enumerate() {
            assert_eq!(
                runtime
                    .store
                    .basic_filter_projection(height as u32)
                    .expect("same active binding"),
                hash
            );
        }
        eprintln!(
            "PRODUCT_HISTORY replacement={replacement} mode={mode:?} known={} active={} physical_undo={} active_undo={} child={child:?} product={:?} {measured:?}",
            entries.len(),
            expected.active_chain.len(),
            physical.undo_by_block.len(),
            expected.undo_by_block.len(),
            started.elapsed()
        );
        let tip = expected.tip().expect("tip");
        let next_height = tip.height + 1;
        let mut next = genuine::child(&tip.header, next_height);
        add_spend(&mut next, &blocks[(next_height - 100) as usize]);
        runtime
            .network
            .connect_outbound_peer(1, 0)
            .expect("fresh socket-free peer");
        runtime
            .network
            .receive_message(
                1,
                open_bitcoin_network::WireNetworkMessage::Headers(
                    open_bitcoin_network::HeadersMessage {
                        headers: vec![next.header.clone()],
                    },
                ),
                i64::MIN,
                ScriptVerifyFlags::NONE,
                runtime.consensus_params,
            )
            .expect("normal next header");
        runtime
            .store
            .save_block(&next, crate::PersistMode::Sync)
            .expect("next body");
        runtime
            .network
            .connect_local_block(&next, ScriptVerifyFlags::NONE, runtime.consensus_params)
            .expect("normal maturity/halving next acceptance");
        let after = runtime
            .network
            .chainstate_snapshot()
            .expect("next complete state");
        assert_eq!(after.tip().expect("tip").height, next_height);
        assert_eq!(
            after.tip().expect("tip").block_hash,
            block_hash(&next.header)
        );
        assert_eq!(
            after.tip().expect("tip").chain_work,
            tip.chain_work.wrapping_add(
                open_bitcoin_core::consensus::chain_proof::block_proof(next.header.bits)
                    .expect("exact work")
            )
        );
        assert_eq!(
            after.active_chain.len(),
            if replacement { 304 } else { 303 }
        );
        assert_eq!(
            runtime.network.header_entries().expect("next known").len(),
            if replacement { 306 } else { 303 }
        );
        runtime
            .network
            .persist_validation_header_snapshot()
            .expect("next earned headers");
        runtime
            .network
            .flush_coins(
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(60_003),
                u64::MAX,
            )
            .expect("next durable acceptance");
        drain(&runtime.network, enabled, next_height);
        assert_eq!(
            runtime
                .store
                .validation_provenance(block_hash(&next.header))
                .expect("durable next provenance"),
            crate::storage::validation_history::ValidationProvenance::ScriptsValid
        );
        // A newly known invalid body may not change any accepted complete state.
        let mut immature = genuine::child(&next.header, next_height + 1);
        add_spend(&mut immature, &next);
        runtime
            .network
            .receive_message(
                1,
                open_bitcoin_network::WireNetworkMessage::Headers(
                    open_bitcoin_network::HeadersMessage {
                        headers: vec![immature.header.clone()],
                    },
                ),
                i64::MIN,
                ScriptVerifyFlags::NONE,
                runtime.consensus_params,
            )
            .expect("known immature-spend header");
        let basic_before = runtime
            .store
            .maybe_basic_filter_state()
            .expect("safe BASIC before refusal");
        assert!(
            runtime
                .network
                .connect_local_block(&immature, ScriptVerifyFlags::NONE, runtime.consensus_params)
                .is_err()
        );
        assert_eq!(
            runtime
                .network
                .chainstate_snapshot()
                .expect("unchanged accepted state"),
            after
        );
        assert_eq!(
            runtime
                .store
                .maybe_basic_filter_state()
                .expect("unchanged safe BASIC"),
            basic_before
        );
        assert_ne!(
            runtime
                .store
                .validation_provenance(block_hash(&immature.header))
                .expect("not accepted"),
            crate::storage::validation_history::ValidationProvenance::ScriptsValid
        );
        drop(runtime);
    });
}

#[test]
fn phase167_runtime_reopen_301_basic_disabled() {
    reopen_checkpoint(BasicFilterStartupMode::Disabled, false);
}

#[test]
fn phase167_runtime_reopen_301_basic_enabled() {
    reopen_checkpoint(BasicFilterStartupMode::Enabled, false);
}

#[test]
fn phase167_runtime_reopen_retained_305_known_303_active_and_next_306_304() {
    reopen_checkpoint(BasicFilterStartupMode::Enabled, true);
}

#[test]
fn phase167_runtime_reopen_actual_all_handle_descendant() {
    genuine::with_manager(|manager, store, params| {
        // Arrange
        let root = store.datadir().to_path_buf();
        let mut network = genuine::network(manager);
        let block = genuine::child(&network.maybe_chain_tip().expect("root").header, 1);
        genuine::admit(&mut network, std::slice::from_ref(&block), params);
        store
            .save_block(&block, crate::PersistMode::Sync)
            .expect("body");
        network
            .connect_local_block(&block, ScriptVerifyFlags::NONE, params)
            .expect("ordinary acceptance");
        network
            .persist_validation_header_snapshot()
            .expect("earned headers");
        network
            .chainstate_mut()
            .flush_with_mode(
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(60001),
                u64::MAX,
            )
            .expect("checkpoint");
        let expected = network
            .chainstate()
            .export_chainstate_snapshot()
            .expect("complete state");
        drop(network);
        drop(store);
        let opened =
            FjallNodeStore::reopen_canonical_history_existing(&root, CanonicalChain::Regtest)
                .expect("safe history storage");
        // Act
        let runtime = DurableSyncRuntime::open_canonical_history_configured(
            opened,
            SyncRuntimeConfig {
                network: super::super::SyncNetwork::Regtest,
                ..Default::default()
            },
            BasicFilterStartupMode::Disabled,
        );
        // Assert
        assert!(
            runtime.is_ok(),
            "history assembly must consume the earned owners: {:?}",
            runtime.as_ref().err()
        );
        assert_eq!(
            runtime
                .expect("reopened")
                .network
                .chainstate_snapshot()
                .expect("complete reopened"),
            expected
        );
    });
}
