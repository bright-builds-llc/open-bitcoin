// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.h

use super::*;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct SpyStore {
    inner: crate::MemoryChainstateStore,
    calls: std::rc::Rc<std::cell::Cell<u64>>,
}

impl SpyStore {
    fn observe(&self) {
        self.calls.set(self.calls.get() + 1);
    }
}

impl ChainstateStore for SpyStore {
    fn maybe_basic_filter_query_store(&self) -> Option<&FjallNodeStore> {
        self.observe();
        None
    }
    fn maybe_validation_history_store(&self) -> Option<FjallNodeStore> {
        self.observe();
        None
    }
    fn load_snapshot(&self) -> Option<ChainstateSnapshot> {
        self.observe();
        self.inner.load_snapshot()
    }
    fn save_snapshot(&mut self, snapshot: ChainstateSnapshot) {
        self.observe();
        self.inner.save_snapshot(snapshot);
    }
    fn get_coin(&self, outpoint: &OutPoint) -> Result<Option<Coin>, ChainstateError> {
        self.observe();
        self.inner.get_coin(outpoint)
    }
    fn have_coin(&self, outpoint: &OutPoint) -> Result<bool, ChainstateError> {
        self.observe();
        self.inner.have_coin(outpoint)
    }
    fn best_block(&self) -> Result<Option<BlockHash>, ChainstateError> {
        self.observe();
        self.inner.best_block()
    }
    fn head_blocks(&self) -> Result<Vec<BlockHash>, ChainstateError> {
        self.observe();
        self.inner.head_blocks()
    }
    fn batch_write(
        &mut self,
        batch: CoinsBatch,
        maybe_best: Option<BlockHash>,
    ) -> Result<(), ChainstateError> {
        self.observe();
        self.inner.batch_write(batch, maybe_best)
    }
    fn load_undo(&self, hash: BlockHash) -> Result<Option<BlockUndo>, ChainstateError> {
        self.observe();
        self.inner.load_undo(hash)
    }
    fn save_undo(&mut self, hash: BlockHash, undo: BlockUndo) -> Result<(), ChainstateError> {
        self.observe();
        self.inner.save_undo(hash, undo)
    }
}

impl FlushPersistSink for SpyStore {
    fn load_prune_protection(&self) -> Result<PruneProtectionSnapshot, StorageError> {
        self.observe();
        self.inner.load_prune_protection()
    }
    fn persist_block(&mut self, block: &Block) -> Result<(), StorageError> {
        self.observe();
        self.inner.persist_block(block)
    }
    fn persist_undo(&mut self, hash: BlockHash, undo: &BlockUndo) -> Result<(), StorageError> {
        self.observe();
        self.inner.persist_undo(hash, undo)
    }
    fn persist_header_entries(
        &mut self,
        entries: &[open_bitcoin_network::HeaderEntry],
    ) -> Result<(), StorageError> {
        self.observe();
        self.inner.persist_header_entries(entries)
    }
    fn persist_chain_meta(&mut self, chain: &[ChainPosition]) -> Result<(), StorageError> {
        self.observe();
        self.inner.persist_chain_meta(chain)
    }
}

#[test]
fn phase165_work_authority_lower_public_reorg_refuses_genuine_weaker_and_equal_suffixes() {
    for active_len in [2_u32, 3] {
        // Arrange
        let mut manager = ManagedChainstate::from_store(SpyStore::default());
        let mut active = Vec::new();
        let mut parent = BlockHash::default();
        for height in 0..active_len {
            let candidate = block(parent, height, 0x51);
            manager
                .connect_block(
                    &candidate,
                    open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
                    ScriptVerifyFlags::P2SH,
                    ConsensusParams::default(),
                )
                .expect("actual live prefix");
            parent = block_hash(&candidate.header);
            active.push(candidate);
        }
        let candidate = block(block_hash(&active[0].header), 1, 0x52);
        let replacement = [AnchoredBlock {
            block: candidate,
            chain_work: open_bitcoin_core::consensus::chain_proof::ProofWork::from_le_bytes(
                [255; 32],
            ),
        }];
        let before = manager.clone();
        let calls = manager.store().calls.get();
        let preparation = manager.work_preparation_observations();
        let disconnect = active[1..].iter().rev().cloned().collect::<Vec<_>>();
        let staged = manager
            .chainstate()
            .stage_reorg(
                &disconnect,
                &replacement,
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default(),
            )
            .expect("genuine validated weaker or equal suffix");
        assert!(staged.has_earned_work());
        // Act
        let result = manager.reorg(
            &disconnect,
            &replacement,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        );
        // Assert
        assert!(
            result.is_err(),
            "lower public owner requires preference authorization"
        );
        assert_eq!(manager, before);
        assert_eq!(
            manager.store().calls.get(),
            calls,
            "zero durable/query calls"
        );
        assert_eq!(
            manager.work_preparation_observations(),
            preparation,
            "zero stage/history/BASIC/preview/commit calls"
        );
        assert!(manager.maybe_validated_lineage.is_none());
        assert!(manager.maybe_basic_index_owner.is_none());
        assert!(manager.maybe_pending_validation.is_none());
    }
}

#[test]
fn phase165_work_authority_lower_connect_wrong_parent_refuses_before_all_preparation() {
    // Arrange
    let mut manager = ManagedChainstate::from_store(SpyStore::default());
    let genesis = block(BlockHash::default(), 0, 0x51);
    manager
        .connect_block(
            &genesis,
            open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("genuine root");
    let orphan = block(BlockHash::from_byte_array([22; 32]), 1, 0x51);
    let before = manager.clone();
    let calls = manager.store().calls.get();
    let preparation = manager.work_preparation_observations();
    // Act
    let result = manager.connect_block(
        &orphan,
        open_bitcoin_core::consensus::chain_proof::ProofWork::from_le_bytes([255; 32]),
        ScriptVerifyFlags::P2SH,
        ConsensusParams::default(),
    );
    // Assert
    assert!(matches!(
        result,
        Err(ChainstateError::InvalidTipExtension { .. })
    ));
    assert_eq!(manager, before);
    assert_eq!(manager.store().calls.get(), calls);
    assert_eq!(manager.work_preparation_observations(), preparation);
}

#[test]
fn phase165_work_authority_lower_invalid_target_and_pow_refuse_before_preparation() {
    for invalid_target in [false, true] {
        // Arrange
        let mut manager = ManagedChainstate::from_store(SpyStore::default());
        let mut candidate = block(BlockHash::default(), 0, 0x51);
        if invalid_target {
            candidate.header.bits = 0;
        } else {
            candidate.header.nonce = (0..=u32::MAX)
                .find(|nonce| {
                    candidate.header.nonce = *nonce;
                    check_block_header(&candidate.header).is_err()
                })
                .expect("actual invalid PoW nonce at easy target");
        }
        let before = manager.clone();
        let calls = manager.store().calls.get();
        let preparation = manager.work_preparation_observations();
        // Act
        let result = manager.connect_block(
            &candidate,
            open_bitcoin_core::consensus::chain_proof::ProofWork::from_le_bytes([255; 32]),
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        );
        // Assert
        assert!(result.is_err());
        assert_eq!(manager, before);
        assert_eq!(manager.store().calls.get(), calls);
        assert_eq!(manager.work_preparation_observations(), preparation);
    }
}

#[test]
fn phase165_work_authority_lower_prepare_without_authorization_refuses_genuine_stronger_stage() {
    // Arrange
    let mut manager = ManagedChainstate::from_store(SpyStore::default());
    let genesis = block(BlockHash::default(), 0, 0x51);
    manager
        .connect_block(
            &genesis,
            open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("root");
    let candidate = block(block_hash(&genesis.header), 1, 0x51);
    let replacement = [AnchoredBlock {
        block: candidate,
        chain_work: open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
    }];
    assert!(
        manager
            .chainstate()
            .stage_reorg(
                &[],
                &replacement,
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default()
            )
            .expect("genuine stronger core stage")
            .has_earned_work()
    );
    let before = manager.clone();
    let calls = manager.store().calls.get();
    let preparation = manager.work_preparation_observations();
    // Act
    let result = manager.prepare_reorg(
        &[],
        &replacement,
        ScriptVerifyFlags::P2SH,
        ConsensusParams::default(),
    );
    // Assert
    assert!(matches!(
        result,
        Err(ChainstateError::PreferenceAuthorizationRequired)
    ));
    assert_eq!(manager, before);
    assert_eq!(manager.store().calls.get(), calls);
    assert_eq!(manager.work_preparation_observations(), preparation);
}

#[test]
fn phase165_work_authority_actual_fjall_lower_refusals_preserve_basic_and_all_accepted_state() {
    for replacement_len in [1, 2] {
        // Arrange
        let fixture = ReorgFixture::new("work-lower-physical-refusal", 3);
        let path = fixture.path().to_owned();
        let store = fixture.store.clone();
        let mut network = fixture.manager.into_network();
        let params = ConsensusParams {
            coinbase_maturity: 1,
            ..Default::default()
        };
        let active = network.chainstate().chainstate().active_chain().to_vec();
        let disconnect = active[1..]
            .iter()
            .rev()
            .map(|position| {
                store
                    .load_block(position.block_hash)
                    .expect("actual active bytes")
                    .expect("retained")
            })
            .collect::<Vec<_>>();
        let mut replacement = Vec::new();
        let mut parent = active[0].block_hash;
        for height in 1..=replacement_len {
            let body = block(parent, height, 0x53);
            parent = block_hash(&body.header);
            network
                .retain_checked_candidate(&body, params, 2_000_000_000)
                .expect("actual later body eligibility and retention");
            replacement.push(AnchoredBlock {
                block: body,
                chain_work: open_bitcoin_core::consensus::chain_proof::ProofWork::from_le_bytes(
                    [255; 32],
                ),
            });
        }
        assert!(
            network
                .maybe_body_eligibility(parent)
                .expect("actual candidate sequence")
                .maybe_sequence
                > network
                    .maybe_body_eligibility(active.last().expect("accepted tip").block_hash)
                    .expect("actual accepted sequence")
                    .maybe_sequence
        );
        let owner = network.chainstate_mut();
        owner
            .initialize_basic_index_owner(ReorgFixture::budget())
            .expect("real BASIC owner");
        assert!(
            owner
                .chainstate()
                .stage_reorg(&disconnect, &replacement, ScriptVerifyFlags::P2SH, params)
                .expect("genuine weaker or later equal validated suffix")
                .has_earned_work()
        );
        let source = owner
            .chainstate()
            .bind_live_work_source(params)
            .expect("private actual source generation");
        let before = owner
            .export_chainstate_snapshot()
            .expect("active/undo/counts");
        let coins = owner
            .chainstate()
            .coins()
            .complete_snapshot()
            .expect("complete effective coins/B/heads");
        let cache = (
            owner.chainstate().coins().cache_entry_count(),
            owner.chainstate().coins().estimated_cache_bytes(),
            coins
                .coins
                .keys()
                .map(|outpoint| {
                    (
                        outpoint.clone(),
                        owner.chainstate().coins().cached_entry(outpoint).cloned(),
                    )
                })
                .collect::<HashMap<_, _>>(),
        );
        let progress = owner.maybe_basic_index_progress();
        let target = owner.maybe_basic_index_accepted_target();
        let checkpoint = store
            .maybe_basic_filter_checkpoint()
            .expect("physical BASIC checkpoint");
        let records = active
            .iter()
            .map(|position| {
                store
                    .load_basic_filter_record(position.block_hash)
                    .expect("physical filter row")
            })
            .collect::<Vec<_>>();
        let stored_undo = store.load_all_undo_records().expect("physical undo");
        let validation = active
            .iter()
            .map(|position| {
                store
                    .validation_provenance(position.block_hash)
                    .expect("actual historical validation classification")
            })
            .collect::<Vec<_>>();
        let preparation = owner.work_preparation_observations();
        // Act
        let result = owner.reorg(&disconnect, &replacement, ScriptVerifyFlags::P2SH, params);
        // Assert
        assert!(matches!(
            result,
            Err(ChainstateError::PreferenceAuthorizationRequired)
        ));
        assert_eq!(owner.work_preparation_observations(), preparation);
        assert_eq!(owner.chainstate().check_live_work_source(&source), Ok(()));
        assert_eq!(
            owner
                .export_chainstate_snapshot()
                .expect("all active facts unchanged"),
            before
        );
        assert_eq!(
            owner
                .chainstate()
                .coins()
                .complete_snapshot()
                .expect("all coins unchanged"),
            coins
        );
        assert_eq!(
            (
                owner.chainstate().coins().cache_entry_count(),
                owner.chainstate().coins().estimated_cache_bytes(),
                coins
                    .coins
                    .keys()
                    .map(|outpoint| (
                        outpoint.clone(),
                        owner.chainstate().coins().cached_entry(outpoint).cloned()
                    ))
                    .collect::<HashMap<_, _>>()
            ),
            cache
        );
        assert_eq!(owner.maybe_basic_index_progress(), progress);
        assert_eq!(owner.maybe_basic_index_accepted_target(), target);
        assert!(owner.maybe_basic_index_failure().is_none());
        assert!(!owner.has_pending_validation_history());
        assert_eq!(
            store
                .maybe_basic_filter_checkpoint()
                .expect("physical checkpoint unchanged"),
            checkpoint
        );
        assert_eq!(
            active
                .iter()
                .map(|position| store
                    .load_basic_filter_record(position.block_hash)
                    .expect("physical row unchanged"))
                .collect::<Vec<_>>(),
            records
        );
        assert_eq!(
            store
                .load_all_undo_records()
                .expect("physical undo unchanged"),
            stored_undo
        );
        assert_eq!(
            active
                .iter()
                .map(|position| store
                    .validation_provenance(position.block_hash)
                    .expect("accepted validation lineage unchanged"))
                .collect::<Vec<_>>(),
            validation
        );
        drop(network);
        drop(store);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
}
