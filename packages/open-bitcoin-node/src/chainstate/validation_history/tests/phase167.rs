// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/pow.cpp
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp

use super::*;
use crate::chainstate::BasicFilterStartupMode;
use open_bitcoin_core::{
    consensus::{CanonicalChain, chain_proof::ProofWork},
    primitives::OutPoint,
};

pub(crate) type Manager = ManagedChainstate<FjallChainstateStore, FjallCoinsView>;
pub(crate) type Network = crate::ManagedPeerNetwork<FjallChainstateStore, FjallCoinsView>;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Accepted {
    chain: open_bitcoin_core::chainstate::ChainstateSnapshot,
    maybe_best: Option<BlockHash>,
    maybe_basic: Option<crate::storage::filter_index::StoredFilterState>,
    maybe_progress:
        Option<open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexProgress>,
    provenance: Vec<(ValidationProvenance, ValidationProvenance)>,
}

pub(crate) fn accepted(manager: &Manager, store: &FjallNodeStore) -> Accepted {
    Accepted {
        chain: manager
            .export_chainstate_snapshot()
            .expect("complete effective coins"),
        maybe_best: manager.chainstate().coins_best_block().expect("coins best"),
        maybe_basic: store.maybe_basic_filter_state().expect("stored BASIC"),
        maybe_progress: manager.maybe_basic_index_progress(),
        provenance: manager
            .chainstate()
            .active_chain()
            .iter()
            .map(|position| {
                (
                    manager
                        .validation_provenance(position.block_hash)
                        .expect("live provenance"),
                    store
                        .validation_provenance(position.block_hash)
                        .expect("durable provenance"),
                )
            })
            .collect(),
    }
}

pub(crate) fn assert_unaccepted(
    manager: &Manager,
    store: &FjallNodeStore,
    block: &Block,
    before: &Accepted,
) {
    assert_eq!(&accepted(manager, store), before);
    assert_ne!(
        manager
            .validation_provenance(block_hash(&block.header))
            .expect("live"),
        ValidationProvenance::ScriptsValid
    );
    assert_ne!(
        store
            .validation_provenance(block_hash(&block.header))
            .expect("durable"),
        ValidationProvenance::ScriptsValid
    );
}

pub(crate) fn network(manager: Manager) -> Network {
    let spec = CanonicalChain::Regtest.spec();
    let proof = open_bitcoin_core::consensus::chain_spec::CanonicalGenesisProof::prove(
        spec,
        &spec.genesis_block(),
    )
    .expect("real selected genesis");
    let mut network = crate::ManagedPeerNetwork::from_canonical_genesis_initialized(
        manager,
        open_bitcoin_network::LocalPeerConfig {
            magic: open_bitcoin_core::primitives::NetworkMagic::from_bytes([
                0xfa, 0xbf, 0xb5, 0xda,
            ]),
            ..Default::default()
        },
        16,
        &proof,
    )
    .expect("proved network owner");
    network
        .connect_outbound_peer(1, 0)
        .expect("socket-free peer");
    network
}

pub(crate) fn admit(network: &mut Network, blocks: &[Block], params: ConsensusParams) {
    network
        .receive_message(
            1,
            open_bitcoin_network::WireNetworkMessage::Headers(
                open_bitcoin_network::HeadersMessage {
                    headers: blocks.iter().map(|b| b.header.clone()).collect(),
                },
            ),
            i64::MIN,
            ScriptVerifyFlags::NONE,
            params,
        )
        .expect("actual contextual header ingress");
}

pub(crate) fn with_manager(check: impl FnOnce(Manager, FjallNodeStore, ConsensusParams)) {
    with_mode(BasicFilterStartupMode::Disabled, check);
}

pub(crate) fn with_mode(
    mode: BasicFilterStartupMode,
    check: impl FnOnce(Manager, FjallNodeStore, ConsensusParams),
) {
    let mut nonce = [0; 16];
    getrandom::fill(&mut nonce).expect("disposable directory");
    let dir = std::fs::canonicalize(std::env::temp_dir())
        .expect("physical temporary parent")
        .join(format!("phase167-managed-{:x}", u128::from_le_bytes(nonce)));
    {
        let (store, recovered) =
            FjallNodeStore::open_canonical_genesis(&dir, CanonicalChain::Regtest)
                .expect("proved canonical genesis")
                .into_parts()
                .expect("exclusive store");
        let (manager, proof) =
            ManagedChainstate::from_canonical_genesis(store.clone(), recovered, mode)
                .expect("genuine coins owner");
        let params =
            open_bitcoin_core::chainstate::engine::canonical_genesis::consensus_params(&proof);
        check(manager, store, params);
    }
    std::fs::remove_dir_all(dir).expect("all handles dropped");
}

pub(crate) fn child(parent: &BlockHeader, height: u32) -> Block {
    let mut number = Vec::new();
    let mut value = height;
    while value > 0 {
        number.push(value as u8);
        value >>= 8;
    }
    if number.last().is_some_and(|byte| byte & 0x80 != 0) {
        number.push(0);
    }
    let mut script = if height <= 16 {
        vec![0x50 + height as u8]
    } else {
        let mut encoded = vec![number.len() as u8];
        encoded.extend(number);
        encoded
    };
    script.push(0);
    let transaction = Transaction {
        version: 1,
        lock_time: height,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(script).expect("height"),
            sequence: u32::MAX,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(5_000_000_000 >> (height / 150)).expect("subsidy"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("true"),
        }],
    };
    let mut block = Block {
        header: BlockHeader {
            version: 4,
            previous_block_hash: block_hash(parent),
            merkle_root: Default::default(),
            time: parent.time + 1,
            bits: parent.bits,
            nonce: 0,
        },
        transactions: vec![transaction],
    };
    remine(&mut block);
    block
}

pub(crate) fn remine(block: &mut Block) {
    block.header.merkle_root = block_merkle_root(&block.transactions).expect("merkle").0;
    block.header.nonce = (0..100_000)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("bounded genuine regtest proof");
}

pub(crate) fn connect(
    manager: &mut Manager,
    block: &Block,
    params: ConsensusParams,
) -> Result<ChainPosition, ChainstateError> {
    manager.connect_block_with_current_time(
        block,
        ProofWork::ZERO,
        i64::MIN,
        ScriptVerifyFlags::NONE,
        params,
    )
}

#[test]
fn phase167_managed_connect_direct_captures_trusted_clock_before_immutable_stage() {
    with_manager(|mut manager, store, params| {
        // Arrange
        let block = child(&manager.chainstate().tip().expect("genesis").header, 1);
        let hash = block_hash(&block.header);
        // Act
        let position = connect(&mut manager, &block, params)
            .expect("actual direct absorption despite replay clock");
        // Assert
        assert_eq!(position.block_hash, hash);
        assert_eq!(
            position.chain_work,
            manager.chainstate().active_chain()[0]
                .chain_work
                .wrapping_add(
                    open_bitcoin_core::consensus::chain_proof::block_proof(block.header.bits)
                        .expect("proof")
                )
        );
        assert_eq!(
            store
                .validation_provenance(hash)
                .expect("durable accepted fact"),
            ValidationProvenance::ScriptsValid
        );
        assert!(manager.chainstate().undo_by_block().contains_key(&hash));
    });
}

#[test]
fn phase167_managed_connect_preferred_replacement_needs_complete_owned_batch() {
    with_manager(|manager, store, params| {
        // Arrange
        let mut network = network(manager);
        let genesis = network.maybe_chain_tip().expect("genesis").header;
        let active = child(&genesis, 1);
        admit(&mut network, std::slice::from_ref(&active), params);
        network
            .connect_local_block(&active, ScriptVerifyFlags::NONE, params)
            .expect("active body");
        let mut first = child(&genesis, 1);
        first.header.time += 100;
        remine(&mut first);
        let second = child(&first.header, 2);
        let replacements = vec![first, second];
        admit(&mut network, &replacements, params);
        let branch: Vec<_> = replacements
            .iter()
            .map(|block| AnchoredBlock {
                block: block.clone(),
                chain_work: network
                    .header_entries()
                    .iter()
                    .find(|entry| entry.block_hash == block_hash(&block.header))
                    .expect("actual earned header work")
                    .chain_work,
            })
            .collect();
        // Act
        let transition = network
            .reorg_to_branch(
                &[active],
                &branch,
                open_bitcoin_mempool::ReorgLifecycleContext::new(
                    open_bitcoin_mempool::PolicyTime::from_unix_seconds(i64::MIN),
                ),
                ScriptVerifyFlags::NONE,
                params,
            )
            .expect("actual preferred two-header fork absorption");
        // Assert
        assert_eq!(transition.disconnected.len(), 1);
        assert_eq!(transition.connected.len(), 2);
        assert_eq!(
            transition.connected[1].median_time_past,
            i64::from(replacements[0].header.time)
        );
        let second_coin = OutPoint {
            txid: open_bitcoin_core::consensus::transaction_txid(&replacements[1].transactions[0])
                .expect("txid"),
            vout: 0,
        };
        assert_eq!(
            network
                .chainstate()
                .chainstate()
                .get_coin(&second_coin)
                .expect("lookup")
                .expect("accepted coin")
                .created_median_time_past,
            i64::from(replacements[0].header.time),
            "creating MTP follows replacement branch"
        );
        assert_eq!(
            network.maybe_chain_tip().expect("replacement").block_hash,
            block_hash(&replacements[1].header)
        );
        for block in &replacements {
            assert_eq!(
                store
                    .validation_provenance(block_hash(&block.header))
                    .expect("durable"),
                ValidationProvenance::ScriptsValid
            );
        }
    });
}

#[test]
fn phase167_managed_connect_nonextending_body_is_eligible_without_coins_admission() {
    with_manager(|manager, store, params| {
        // Arrange
        let mut network = network(manager);
        let genesis = network.maybe_chain_tip().expect("genesis").header;
        let active = child(&genesis, 1);
        admit(&mut network, std::slice::from_ref(&active), params);
        network
            .connect_local_block(&active, ScriptVerifyFlags::NONE, params)
            .expect("active");
        let mut fork = child(&genesis, 1);
        fork.header.time += 50;
        remine(&mut fork);
        admit(&mut network, std::slice::from_ref(&fork), params);
        let before = network
            .chainstate_snapshot()
            .expect("complete accepted state");
        let before_basic = store.maybe_basic_filter_state().expect("BASIC");
        // Act
        let disposition = network
            .connect_stored_block(
                &fork,
                ProofWork::ZERO,
                i64::MIN,
                ScriptVerifyFlags::NONE,
                params,
            )
            .expect("known nonextending body can be cached");
        // Assert
        assert!(matches!(
            disposition,
            crate::network::BlockConnectDisposition::NonExtending { .. }
        ));
        assert_eq!(network.chainstate_snapshot().expect("unchanged"), before);
        assert_eq!(
            store.maybe_basic_filter_state().expect("unchanged BASIC"),
            before_basic
        );
        assert_ne!(
            store
                .validation_provenance(block_hash(&fork.header))
                .expect("no receipt"),
            ValidationProvenance::ScriptsValid
        );
        assert!(
            network
                .maybe_body_eligibility(block_hash(&fork.header))
                .expect("eligible")
                .available
        );
    });
}

#[test]
fn phase167_managed_connect_no_clock_and_prepare_need_exact_held_admission() {
    with_manager(|mut manager, store, params| {
        // Arrange
        let block = child(&manager.chainstate().tip().expect("genesis").header, 1);
        let before = accepted(&manager, &store);
        // Act / Assert
        assert!(
            manager
                .connect_block(&block, ProofWork::ZERO, ScriptVerifyFlags::NONE, params)
                .is_err()
        );
        assert!(
            manager
                .prepare_connect_block_with_current_time(
                    &block,
                    ProofWork::ZERO,
                    i64::MAX,
                    ScriptVerifyFlags::NONE,
                    params
                )
                .is_err()
        );
        assert_unaccepted(&manager, &store, &block, &before);
        manager
            .admit_direct_canonical_header(&block.header)
            .expect("actual shell observation");
        let source = manager
            .chainstate()
            .bind_live_work_source(params)
            .expect("source");
        manager
            .admit_direct_canonical_header(&block.header)
            .expect("identical observation preserves generation");
        manager
            .chainstate()
            .check_live_work_source(&source)
            .expect("same exact admission");
        manager
            .connect_block(&block, ProofWork::ZERO, ScriptVerifyFlags::NONE, params)
            .expect("held genuine context usable without clock");
    });
}

#[test]
fn phase167_managed_connect_future_header_cannot_use_caller_now() {
    with_manager(|mut manager, store, params| {
        // Arrange
        let mut block = child(&manager.chainstate().tip().expect("genesis").header, 1);
        block.header.time = u32::try_from(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_secs()
                + 7_300,
        )
        .expect("timestamp");
        remine(&mut block);
        let before = accepted(&manager, &store);
        // Act
        let error = manager
            .connect_block_with_current_time(
                &block,
                ProofWork::ZERO,
                i64::MAX,
                ScriptVerifyFlags::NONE,
                params,
            )
            .expect_err("caller future time is not evidence");
        // Assert
        assert!(error.to_string().contains("time-too-new"));
        assert_unaccepted(&manager, &store, &block, &before);
    });
}

#[test]
fn phase167_managed_connect_selected_params_and_public_reorg_remain_immutable() {
    with_manager(|mut manager, store, params| {
        // Arrange
        let block = child(&manager.chainstate().tip().expect("genesis").header, 1);
        let before = accepted(&manager, &store);
        // Act / Assert
        for changed in [
            ConsensusParams {
                coinbase_maturity: 1,
                ..params
            },
            ConsensusParams {
                subsidy_halving_interval: 210_000,
                ..params
            },
        ] {
            assert!(connect(&mut manager, &block, changed).is_err());
            assert_unaccepted(&manager, &store, &block, &before);
        }
        assert!(matches!(
            manager.reorg(&[], &[], ScriptVerifyFlags::NONE, params),
            Err(ChainstateError::PreferenceAuthorizationRequired)
        ));
        assert_unaccepted(&manager, &store, &block, &before);
        connect(&mut manager, &block, params).expect("selected parameters still usable");
    });
}

#[test]
fn phase167_managed_connect_changed_admission_invalidates_stage_without_acceptance() {
    with_manager(|mut manager, store, params| {
        // Arrange
        let block = child(&manager.chainstate().tip().expect("genesis").header, 1);
        manager
            .admit_direct_canonical_header(&block.header)
            .expect("shell context");
        let staged = manager
            .prepare_connect_block_with_current_time(
                &block,
                ProofWork::ZERO,
                i64::MIN,
                ScriptVerifyFlags::NONE,
                params,
            )
            .expect("immutable stage");
        let mut other = block.clone();
        other.header.time += 1;
        remine(&mut other);
        let before = accepted(&manager, &store);
        // Act
        manager
            .admit_direct_canonical_header(&other.header)
            .expect("different exact header");
        let result = manager.commit_prepared_connect(staged);
        // Assert
        assert!(matches!(result, Err(ChainstateError::StaleStage)));
        assert_unaccepted(&manager, &store, &block, &before);
    });
}

#[test]
fn phase167_managed_connect_foreign_prepared_stage_cannot_publish_receipt() {
    with_manager(|mut first, _, params| {
        with_manager(|mut second, store, _| {
            // Arrange
            let block = child(&first.chainstate().tip().expect("root").header, 1);
            first
                .admit_direct_canonical_header(&block.header)
                .expect("actual shell context");
            let stage = first
                .prepare_connect_block_with_current_time(
                    &block,
                    ProofWork::ZERO,
                    i64::MIN,
                    ScriptVerifyFlags::NONE,
                    params,
                )
                .expect("first-owner stage");
            let before = accepted(&second, &store);
            // Act
            let result = second.commit_prepared_connect(stage);
            // Assert
            assert!(matches!(result, Err(ChainstateError::StageOwnerMismatch)));
            assert_unaccepted(&second, &store, &block, &before);
        });
    });
}

#[test]
fn phase167_managed_connect_seeded_adapter_stays_fixture_classified_and_flushes_full_undo() {
    with_manager(|manager, store, params| {
        // Arrange
        let snapshot = manager
            .export_chainstate_snapshot()
            .expect("initial complete state");
        let header = snapshot.active_chain[0].header.clone();
        let block = child(&header, 1);
        let history = open_bitcoin_core::consensus::CandidateHistory::check_seeded(
            CanonicalChain::Regtest.spec().rules(),
            block.header.clone(),
            open_bitcoin_core::consensus::HistoryHeader::new(CanonicalChain::Regtest, 0, header),
            vec![],
            None,
        )
        .expect("explicit conditional history");
        let core = Chainstate::from_coins_cache_for_mechanical_fixture(
            CoinsCache::from_parent(store.coins_view()),
            snapshot.active_chain,
            snapshot.undo_by_block,
            snapshot.maybe_confirmed_txid_counts,
        );
        let mut fixture = ManagedChainstate::from_chainstate(
            FjallChainstateStore::from_store(store.clone()),
            core,
            manager.flush_lifecycle.clone(),
        );
        drop(manager);
        // Act
        fixture
            .connect_seeded_historical_fixture(&block, &history, i64::from(block.header.time))
            .expect("conditional managed kernel and flush");
        // Assert
        assert_eq!(fixture.chainstate().maybe_canonical_chain(), None);
        assert!(fixture.chainstate().validate_live_origin().is_err());
        assert!(fixture.chainstate().bind_live_work_source(params).is_err());
        let hash = block_hash(&block.header);
        assert_eq!(
            store
                .load_undo(hash)
                .expect("complete persisted undo")
                .as_ref(),
            fixture.chainstate().undo_by_block().get(&hash)
        );
        assert_ne!(
            store
                .validation_provenance(hash)
                .expect("no canonical publication"),
            ValidationProvenance::ScriptsValid
        );
    });
}
