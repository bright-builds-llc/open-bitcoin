// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/txdb.cpp

use super::*;
use crate::chainstate::BasicFilterStartupMode;

#[test]
fn phase167_durable_counts_changed_canonical_checkpoint_preserves_exact_map() {
    genuine::with_manager(|manager, store, params| {
        // Arrange
        let mut network = genuine::network(manager);
        let block = genuine::child(&network.maybe_chain_tip().expect("genesis").header, 1);
        genuine::admit(&mut network, std::slice::from_ref(&block), params);
        network
            .connect_local_block(&block, ScriptVerifyFlags::NONE, params)
            .expect("actual canonical absorption");
        let live = network.chainstate_snapshot().expect("complete live state");
        assert_eq!(
            live.maybe_confirmed_txid_counts
                .as_ref()
                .expect("known")
                .len(),
            1
        );
        // Act
        network
            .chainstate_mut()
            .flush_with_mode(
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(50_001),
                u64::MAX,
            )
            .expect("actual managed flush");
        // Assert
        assert_eq!(
            store.load_chain_meta_for_open().expect("complete metadata"),
            (live.active_chain, live.maybe_confirmed_txid_counts)
        );
    });
}
use crate::chainstate::validation_history::tests::phase167 as genuine;
use open_bitcoin_core::{
    chainstate::AnchoredBlock,
    consensus::{CanonicalChain, ConsensusParams, ScriptVerifyFlags, transaction_txid},
};

fn checkpoint(network: &mut genuine::Network) {
    network
        .persist_validation_header_snapshot()
        .expect("trusted complete headers");
    network
        .chainstate_mut()
        .flush_with_mode(FlushMode::Always, policy_now(), u64::MAX)
        .expect("full managed coins and metadata checkpoint");
}

fn history(
    network: &mut genuine::Network,
    store: &FjallNodeStore,
    params: ConsensusParams,
) -> Vec<Block> {
    let mut blocks = vec![CanonicalChain::Regtest.spec().genesis_block()];
    for height in 1..=301 {
        let mut block = genuine::child(&blocks.last().expect("parent").header, height);
        if height == 101 {
            let mut spend = blocks[1].transactions[0].clone();
            spend.inputs[0].previous_output = OutPoint {
                txid: transaction_txid(&spend).expect("spent coinbase identity"),
                vout: 0,
            };
            spend.inputs[0].script_sig =
                ScriptBuf::from_bytes(Vec::new()).expect("empty input script");
            spend.outputs[0].value = Amount::from_sats(4_999_999_000).expect("fee");
            spend.lock_time = 0;
            block.transactions.push(spend);
            genuine::remine(&mut block);
        }
        genuine::admit(network, std::slice::from_ref(&block), params);
        store
            .save_block(&block, PersistMode::Sync)
            .expect("retained received body");
        network
            .connect_local_block(&block, ScriptVerifyFlags::NONE, params)
            .expect("actual absorption");
        blocks.push(block);
    }
    let state = network.chainstate_snapshot().expect("complete state");
    let spent = transaction_txid(&blocks[1].transactions[0]).expect("spent identity");
    assert!(!state.utxos.keys().any(|outpoint| outpoint.txid == spent));
    assert_eq!(
        state
            .maybe_confirmed_txid_counts
            .as_ref()
            .expect("complete")
            .get(&spent),
        Some(&1)
    );
    blocks
}

fn catch_up(
    handle: &crate::ManagedNetworkHandle<crate::chainstate::FjallChainstateStore, FjallCoinsView>,
    enabled: bool,
) {
    if enabled {
        let mut maybe_previous = None;
        for _ in 0..303 {
            let turn = handle
                .drive_basic_filter_index_turn()
                .expect("ordinary bounded turn");
            let height = turn
                .maybe_progress
                .expect("progress")
                .maybe_processed_endpoint()
                .expect("endpoint")
                .height();
            assert!(maybe_previous.is_none_or(|previous| height > previous));
            maybe_previous = Some(height);
            if turn.maybe_accepted_lag == Some(0) {
                break;
            }
        }
    } else {
        let turn = handle.drive_basic_filter_index_turn().expect("disabled");
        assert!(turn.maybe_progress.is_none());
    }
}

fn reopen_complete(mode: BasicFilterStartupMode, replacement: bool) {
    genuine::with_mode(mode, |manager, store, params| {
        // Arrange: real canonical301, with a fully spent historical transaction.
        let root = store.datadir().to_path_buf();
        let mut network = genuine::network(manager);
        let mut blocks = history(&mut network, &store, params);
        checkpoint(&mut network);
        let original = network.chainstate_snapshot().expect("original");
        assert_eq!(
            store
                .wallet_scan_chainstate_snapshot()
                .expect("stored")
                .expect("present"),
            original
        );
        let network = crate::ManagedNetworkHandle::new(network);
        if mode == BasicFilterStartupMode::Enabled {
            network
                .initialize_basic_filter_index_owner()
                .expect("actual ordered owner");
            catch_up(&network, true);
            let progress = network
                .maybe_basic_index_progress()
                .expect("progress")
                .expect("owner");
            let endpoint = progress
                .maybe_processed_endpoint()
                .expect("processed original301");
            let original_tip = original.tip().expect("original tip");
            assert_eq!(endpoint.height(), 301);
            assert_eq!(progress.maybe_safe_durable_endpoint(), Some(endpoint));
            assert_eq!(endpoint.block_hash(), original_tip.block_hash);
        }
        if replacement {
            let mut parent = blocks[299].header.clone();
            let mut branch_blocks = Vec::new();
            for height in 300..=302 {
                let mut block = genuine::child(&parent, height);
                block.header.time += 1_000;
                block.transactions[0].lock_time += 10_000;
                genuine::remine(&mut block);
                store
                    .save_block(&block, PersistMode::Sync)
                    .expect("received fork body");
                parent = block.header.clone();
                branch_blocks.push(block);
            }
            network
                .receive_message(
                    1,
                    open_bitcoin_network::WireNetworkMessage::Headers(
                        open_bitcoin_network::HeadersMessage {
                            headers: branch_blocks
                                .iter()
                                .map(|block| block.header.clone())
                                .collect(),
                        },
                    ),
                    i64::MIN,
                    ScriptVerifyFlags::NONE,
                    params,
                )
                .expect("actual contextual fork ingress");
            let entries = network.header_entries().expect("earned headers");
            let branch: Vec<_> = branch_blocks
                .iter()
                .map(|block| AnchoredBlock {
                    block: block.clone(),
                    chain_work: entries
                        .iter()
                        .find(|entry| entry.block_hash == block_hash(&block.header))
                        .expect("earned work")
                        .chain_work,
                })
                .collect();
            let disconnected: Vec<_> = blocks[300..].iter().rev().cloned().collect();
            let transition = network
                .reorg_to_branch(
                    &disconnected,
                    &branch,
                    open_bitcoin_mempool::ReorgLifecycleContext::new(
                        open_bitcoin_mempool::PolicyTime::from_unix_seconds(i64::MIN),
                    ),
                    ScriptVerifyFlags::NONE,
                    params,
                )
                .expect("genuine preferred replacement");
            assert_eq!(transition.disconnected.len(), 2);
            assert_eq!(transition.connected.len(), 3);
            let current = network.chainstate_snapshot().expect("replacement");
            assert!(
                current.tip().expect("tip").chain_work > original.tip().expect("old").chain_work
            );
            assert_ne!(
                current.tip().expect("tip").median_time_past,
                original.tip().expect("old").median_time_past
            );
            let counts = current
                .maybe_confirmed_txid_counts
                .as_ref()
                .expect("complete");
            for block in disconnected {
                assert!(!counts.contains_key(
                    &transaction_txid(&block.transactions[0]).expect("removed identity")
                ));
            }
            blocks.extend(branch_blocks);
            network
                .persist_validation_header_snapshot()
                .expect("trusted replacement headers");
            network
                .flush_coins(FlushMode::Always, policy_now(), u64::MAX)
                .expect("replacement checkpoint");
        }
        catch_up(&network, mode == BasicFilterStartupMode::Enabled);
        let snapshot = network
            .chainstate_snapshot()
            .expect("captured complete live state");
        let entries = network.header_entries().expect("complete headers");
        let all_statuses: Vec<_> = entries
            .iter()
            .map(|entry| {
                (
                    entry.block_hash,
                    store
                        .maybe_validation_history_record(entry.block_hash)
                        .expect("all known statuses"),
                    store
                        .validation_provenance(entry.block_hash)
                        .expect("all known provenance"),
                )
            })
            .collect();
        let progress = network.maybe_basic_index_progress().expect("progress");
        let basic = store.maybe_basic_filter_state().expect("BASIC");
        let records: Vec<_> = snapshot
            .active_chain
            .iter()
            .map(|position| {
                (
                    store
                        .load_basic_filter_record(position.block_hash)
                        .expect("every BASIC record"),
                    if mode == BasicFilterStartupMode::Enabled {
                        Some(
                            store
                                .basic_filter_projection(position.height)
                                .expect("active binding"),
                        )
                    } else {
                        None
                    },
                    store
                        .maybe_validation_history_record(position.block_hash)
                        .expect("status"),
                    store
                        .validation_provenance(position.block_hash)
                        .expect("provenance"),
                )
            })
            .collect();
        if let Some(progress) = progress {
            let proof = store
                .maybe_basic_filter_append_proof()
                .expect("proof")
                .expect("live owner");
            assert_eq!(proof.branch_identity(), progress.branch_identity());
            assert!(
                snapshot
                    .active_chain
                    .iter()
                    .any(|p| p.block_hash == proof.branch_identity())
            );
            assert_eq!(
                progress
                    .maybe_processed_endpoint()
                    .expect("processed")
                    .height(),
                snapshot.tip().expect("tip").height
            );
            assert_eq!(
                progress.maybe_safe_durable_endpoint(),
                progress.maybe_processed_endpoint()
            );
        }
        let mut expected = snapshot.clone();
        // Orphan undo remains retained data; every complete original record is compared too.
        expected.undo_by_block.extend(original.undo_by_block);
        assert_eq!(
            store
                .wallet_scan_chainstate_snapshot()
                .expect("wallet")
                .expect("present"),
            expected
        );
        let spent = transaction_txid(&blocks[1].transactions[0]).expect("spent identity");
        assert_eq!(
            snapshot
                .maybe_confirmed_txid_counts
                .as_ref()
                .expect("known")
                .get(&spent),
            Some(&1)
        );
        // Act: all owners and proofs die before copied inspection and physical reopen.
        drop(network);
        drop(store);
        let measured: crate::storage::fjall_store::DurableRowsMeasurement =
            crate::storage::fjall_store::measure_detached_rows_for_test(&root)
                .expect("bounded detached inspection");
        eprintln!(
            "DURABLE_COUNTS mode={mode:?} replacement={replacement} positions={} counts={} {measured:?}",
            snapshot.active_chain.len(),
            snapshot
                .maybe_confirmed_txid_counts
                .as_ref()
                .expect("known")
                .len()
        );
        assert!(measured.manifest_equal && measured.authority_max <= 65536);
        assert!(measured.logical_entries <= 4096 && measured.physical_entries <= 4096);
        assert!(measured.logical_bytes <= 536870912 && measured.physical_bytes <= 536870912);
        assert_eq!(measured.copy_bytes, measured.physical_bytes);
        assert!(measured.elapsed.as_secs_f64() < 30.0);
        assert!(measured.body_max > 0 && measured.undo_max > 0);
        assert_eq!(
            measured.basic_max > 0,
            mode == BasicFilterStartupMode::Enabled
        );
        let reopened = FjallNodeStore::open(&root).expect("all-handle storage reopen");
        // Assert: full positions/work/MTP, Coins, all undos, exact map and all bodies.
        assert_eq!(
            reopened
                .wallet_scan_chainstate_snapshot()
                .expect("wallet")
                .expect("present"),
            expected
        );
        assert_eq!(
            reopened
                .hydrate_chainstate_for_open()
                .expect("hydrate")
                .expect("present"),
            expected
        );
        assert_eq!(
            reopened.load_chain_meta_for_open().expect("metadata"),
            (
                snapshot.active_chain.clone(),
                snapshot.maybe_confirmed_txid_counts.clone()
            )
        );
        assert_eq!(
            reopened
                .load_header_entries()
                .expect("headers")
                .expect("present")
                .entries,
            entries
        );
        assert_eq!(
            reopened
                .load_block_index_entries()
                .expect("index")
                .expect("present")
                .entries,
            entries
        );
        for (hash, status, provenance) in all_statuses {
            assert_eq!(
                reopened
                    .maybe_validation_history_record(hash)
                    .expect("all statuses"),
                status
            );
            assert_eq!(
                reopened
                    .validation_provenance(hash)
                    .expect("all provenance"),
                provenance
            );
        }
        for block in blocks {
            assert_eq!(
                reopened
                    .load_block(block_hash(&block.header))
                    .expect("body"),
                Some(block)
            );
        }
        assert_eq!(reopened.maybe_basic_filter_state().expect("BASIC"), basic);
        for (position, record) in snapshot.active_chain.iter().zip(records) {
            assert_eq!(
                reopened
                    .load_basic_filter_record(position.block_hash)
                    .expect("record"),
                record.0
            );
            if let Some(hash) = record.1 {
                assert_eq!(
                    reopened
                        .basic_filter_projection(position.height)
                        .expect("binding"),
                    hash
                );
            }
            assert_eq!(
                reopened
                    .maybe_validation_history_record(position.block_hash)
                    .expect("status"),
                record.2
            );
            assert_eq!(
                reopened
                    .validation_provenance(position.block_hash)
                    .expect("provenance"),
                record.3
            );
        }
        if let Some(progress) = progress {
            assert!(
                reopened
                    .maybe_basic_filter_append_proof()
                    .expect("raw reopen")
                    .is_none()
            );
            reopened
                .configure_basic_filter_index_before_prune(
                    Some(snapshot.tip().expect("tip").block_hash),
                    mode,
                )
                .expect("explicit configured data recovery");
            let proof = reopened
                .maybe_basic_filter_append_proof()
                .expect("proof")
                .expect("new process anchor");
            assert_eq!(
                proof.branch_identity(),
                snapshot.tip().expect("tip").block_hash
            );
            assert_eq!(proof.generation(), progress.generation());
            assert_eq!(proof.protection(), progress.protection());
            assert_eq!(
                proof.processed().prefix(),
                open_bitcoin_core::chainstate::IndexPrefix::Committed(
                    progress.maybe_processed_endpoint().expect("processed")
                )
            );
            assert_eq!(
                proof.safe_checkpoint().prefix(),
                open_bitcoin_core::chainstate::IndexPrefix::Committed(
                    progress.maybe_safe_durable_endpoint().expect("safe")
                )
            );
        }
    });
}

#[test]
fn phase167_durable_counts_reopen_actual301_disabled() {
    reopen_complete(BasicFilterStartupMode::Disabled, false);
}
#[test]
fn phase167_durable_counts_reopen_actual301_enabled() {
    reopen_complete(BasicFilterStartupMode::Enabled, false);
}
#[test]
fn phase167_durable_counts_reopen_preferred_replacement() {
    reopen_complete(BasicFilterStartupMode::Enabled, true);
}

#[derive(Default)]
struct UnsupportedSink {
    effects: usize,
}
impl FlushPersistSink for UnsupportedSink {
    fn persist_block(&mut self, _: &Block) -> Result<(), StorageError> {
        self.effects += 1;
        Ok(())
    }
    fn persist_undo(&mut self, _: BlockHash, _: &BlockUndo) -> Result<(), StorageError> {
        self.effects += 1;
        Ok(())
    }
    fn persist_header_entries(&mut self, _: &[HeaderEntry]) -> Result<(), StorageError> {
        self.effects += 1;
        Ok(())
    }
    fn persist_chain_meta(
        &mut self,
        _: &[open_bitcoin_core::chainstate::ChainPosition],
    ) -> Result<(), StorageError> {
        self.effects += 1;
        Ok(())
    }
    fn commit_paired_unlink(
        &mut self,
        _: u32,
        _: BlockHash,
    ) -> Result<crate::storage::fjall_store::PairedDeleteOutcome, StorageError> {
        self.effects += 1;
        Ok(crate::storage::fjall_store::PairedDeleteOutcome::AlreadyAbsent)
    }
}

type PhysicalManifest = Vec<(
    PathBuf,
    bool,
    u64,
    SystemTime,
    std::fs::Permissions,
    Vec<u8>,
)>;

fn manifest(root: &Path) -> PhysicalManifest {
    fn visit(root: &Path, rows: &mut PhysicalManifest) {
        let meta = fs::symlink_metadata(root).expect("manifest metadata");
        assert!(!meta.file_type().is_symlink());
        rows.push((
            root.to_path_buf(),
            meta.is_dir(),
            meta.len(),
            meta.modified().expect("mtime"),
            meta.permissions(),
            if meta.is_file() {
                fs::read(root).expect("physical contents")
            } else {
                Vec::new()
            },
        ));
        if meta.is_dir() {
            for entry in fs::read_dir(root).expect("directory") {
                visit(&entry.expect("entry").path(), rows);
            }
        }
    }
    let mut rows = Vec::new();
    visit(root, &mut rows);
    rows.sort_by(|a, b| a.0.cmp(&b.0));
    rows
}

#[test]
fn phase167_durable_counts_unsupported_sink_refuses_before_first_effect() {
    genuine::with_manager(|mut manager, store, params| {
        // Arrange
        let block = genuine::child(&manager.chainstate().tip().expect("genesis").header, 1);
        genuine::connect(&mut manager, &block, params).expect("genuine canonical overlay");
        let accepted = genuine::accepted(&manager, &store);
        let physical = manifest(store.datadir());
        let mut sink = UnsupportedSink::default();
        let now = policy_now();
        let mut lifecycle = FlushLifecycle::ready(now, now, 0, false);
        let active = manager.chainstate().active_chain().to_vec();
        let undo: Vec<_> = manager
            .chainstate()
            .undo_by_block()
            .iter()
            .map(|(hash, undo)| (*hash, undo.clone()))
            .collect();
        // Act
        let result = lifecycle.execute_flush_applying_plan(
            &mut sink,
            &mut manager.chainstate,
            FlushMode::Always,
            now,
            u64::MAX,
            &undo,
            &[],
            &[block],
            &active,
            &open_bitcoin_core::chainstate::PrunePlan { heights: vec![0] },
            &[],
            &mut |_| panic!("prune callback before support refusal"),
        );
        // Assert
        assert!(matches!(
            result,
            Err(StorageError::UnavailableNamespace {
                namespace: StorageNamespace::Chainstate
            })
        ));
        assert_eq!(sink.effects, 0);
        assert_eq!(genuine::accepted(&manager, &store), accepted);
        assert_eq!(manifest(store.datadir()), physical);
        assert!(
            sink.persist_chain_meta_with_counts(&active, Some(&HashMap::new()))
                .is_err()
        );
        assert_eq!(sink.effects, 0);
    });
}
