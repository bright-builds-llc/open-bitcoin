// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Concrete storage effects forwarded into the existing paired-delete owner.

use super::*;

impl FlushPersistSink for FjallNodeStore {
    fn confirm_validated_flush(
        &mut self,
        completed: CompletedValidatedFlush,
    ) -> Result<(), StorageError> {
        self.confirm_basic_filter_flush(completed)
    }
    fn retained_payload_usage(
        &self,
        active_chain: &[ChainPosition],
    ) -> Result<RetainedPayloadUsage, StorageError> {
        FjallNodeStore::retained_payload_usage(self, active_chain)
    }
    fn payload_usage_revision(&self) -> Result<PayloadUsageRevision, StorageError> {
        FjallNodeStore::payload_usage_revision(self)
    }
    fn load_prune_locks(&self) -> Result<Vec<PruneLockInfo>, StorageError> {
        FjallNodeStore::load_prune_locks(self)
    }
    fn load_prune_protection(&self) -> Result<PruneProtectionSnapshot, StorageError> {
        FjallNodeStore::load_prune_protection(self)
    }
    fn sync_prune_locks(&self, locks: &[PruneLockInfo]) -> Result<(), StorageError> {
        FjallNodeStore::sync_prune_locks(self, locks)
    }
    fn persist_block(&mut self, block: &Block) -> Result<(), StorageError> {
        FjallNodeStore::save_block(self, block, PersistMode::Flush).map(|_| ())
    }
    fn persist_undo(&mut self, hash: BlockHash, undo: &BlockUndo) -> Result<(), StorageError> {
        FjallNodeStore::save_undo(self, hash, undo, PersistMode::Flush)
    }
    fn persist_header_entries(&mut self, entries: &[HeaderEntry]) -> Result<(), StorageError> {
        if entries.is_empty() {
            return Ok(());
        }
        FjallNodeStore::save_header_entries(self, entries, PersistMode::Flush)
    }
    fn persist_chain_meta(&mut self, active_chain: &[ChainPosition]) -> Result<(), StorageError> {
        FjallNodeStore::save_validated_chain_meta(self, active_chain, PersistMode::Flush)
    }
    fn disk_free_bytes(&self) -> u64 {
        probe_disk_free_bytes(self.datadir())
    }
    fn commit_paired_unlink(
        &mut self,
        height: u32,
        block_hash: BlockHash,
    ) -> Result<PairedDeleteOutcome, StorageError> {
        self.commit_paired_delete(height, block_hash)
    }
    fn record_successful_prune_batch(
        &mut self,
        deleted_heights: &[u32],
    ) -> Result<(), StorageError> {
        FjallNodeStore::record_successful_prune_batch(self, deleted_heights)
    }
}

#[cfg(test)]
pub(super) mod proof_tests {
    use super::super::tests::{coinbase, header, temp_store_path};
    use super::*;
    use crate::chainstate::{FjallChainstateStore, ManagedChainstate};
    use crate::storage::fjall_store::filters::{
        BasicFilterWriterInterleave, FilterPublicationFault,
    };
    use crate::{DurableSyncRuntime, SyncRuntimeConfig};
    use open_bitcoin_core::{
        chainstate::Chainstate,
        consensus::{ConsensusParams, ScriptVerifyFlags, block_merkle_root, check_block_header},
        primitives::ScriptBuf,
    };

    fn block(parent: BlockHash, height: u32) -> Block {
        let mut transaction = coinbase(height, 5_000_000_000);
        transaction.inputs[0].script_sig = ScriptBuf::from_bytes(if height == 0 {
            vec![0, 0x51]
        } else {
            vec![1, height as u8, 0x51]
        })
        .expect("height script");
        let transactions = vec![transaction];
        let mut block = Block {
            header: header(parent, 0),
            transactions,
        };
        block.header.time = 1_000 + height * 100;
        block.header.merkle_root = block_merkle_root(&block.transactions).expect("merkle").0;
        block.header.nonce = (0..=u32::MAX)
            .find(|nonce| {
                block.header.nonce = *nonce;
                check_block_header(&block.header).is_ok()
            })
            .expect("easy proof of work");
        block
    }

    fn runtime(name: &str) -> (std::path::PathBuf, DurableSyncRuntime, Block) {
        static NEXT_FIXTURE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let unique = NEXT_FIXTURE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = temp_store_path(&format!("{name}-{unique}"));
        let store = FjallNodeStore::open(&path).expect("store");
        let genesis = block(BlockHash::default(), 0);
        let mut chainstate = Chainstate::default();
        chainstate
            .connect_block_with_current_time(
                &genesis,
                1,
                1_001,
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default(),
            )
            .expect("validated genesis");
        store
            .seed_coins_from_snapshot(&chainstate.snapshot())
            .expect("real genesis coins/meta");
        store
            .save_block(&genesis, PersistMode::Sync)
            .expect("retained genesis");
        let runtime = DurableSyncRuntime::open_configured(
            store,
            SyncRuntimeConfig::default(),
            BasicFilterStartupMode::Enabled,
        )
        .expect("configured recovered runtime");
        (path, runtime, genesis)
    }

    #[test]
    fn phase157_proof_actual_tracked_runtime_flush_refreshes() {
        // Arrange
        let (path, runtime, _) = runtime("proof-managed-positive");
        let old = runtime
            .store()
            .maybe_basic_filter_append_proof()
            .expect("proof")
            .expect("recovered");
        // Act
        let execution = runtime
            .network_handle()
            .flush_coins(
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(10),
                u64::MAX,
            )
            .expect("actual flush");
        // Assert
        assert!(execution.wrote_coins);
        let current = runtime
            .store()
            .maybe_basic_filter_append_proof()
            .expect("proof")
            .expect("confirmed own flush");
        assert!(
            runtime
                .store()
                .check_basic_filter_append_proof(&old)
                .is_err()
        );
        runtime
            .store()
            .check_basic_filter_append_proof(&current)
            .expect("live new proof");
        drop(current);
        drop(old);
        drop(runtime);
        std::fs::remove_dir_all(path).expect("cleanup");
    }

    pub(in crate::chainstate) fn managed_fixture(
        name: &str,
    ) -> (
        std::path::PathBuf,
        FjallNodeStore,
        ManagedChainstate<FjallChainstateStore, FjallCoinsView>,
        Block,
    ) {
        let (path, runtime, genesis) = runtime(name);
        let store = runtime.store().clone();
        drop(runtime);
        let now = FlushPolicyTime::from_unix_seconds(0);
        let (lifecycle, _, cache) = initialize_configured(
            &store,
            now,
            now,
            0,
            false,
            u64::MAX,
            BasicFilterStartupMode::Enabled,
        )
        .expect("actual configured init");
        let (positions, counts) = store.load_chain_meta_for_open().expect("metadata");
        let state = Chainstate::from_coins_cache(
            cache,
            positions,
            store.load_all_undo_records().expect("undo"),
            counts,
        );
        let managed = ManagedChainstate::from_recovered_chainstate(
            FjallChainstateStore::from_store(store.clone()),
            state,
            lifecycle,
        )
        .expect("exact recovered constructor");
        (path, store, managed, genesis)
    }

    fn accept_child(
        managed: &mut ManagedChainstate<FjallChainstateStore, FjallCoinsView>,
        genesis: &Block,
        staged: bool,
    ) -> Block {
        let child = block(open_bitcoin_core::consensus::block_hash(&genesis.header), 1);
        if staged {
            let prepared = managed
                .prepare_connect_block_with_current_time(
                    &child,
                    2,
                    1_101,
                    ScriptVerifyFlags::P2SH,
                    ConsensusParams::default(),
                )
                .expect("actual stage");
            managed.commit_prepared_connect(prepared)
        } else {
            managed.connect_block_with_current_time(
                &child,
                2,
                1_101,
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default(),
            )
        }
        .expect("actual validated acceptance");
        managed
            .store()
            .inner()
            .save_block(&child, PersistMode::Sync)
            .expect("retained accepted body");
        child
    }

    fn accepted_flush(staged: bool) {
        // Arrange
        let (path, store, mut managed, genesis) = managed_fixture("proof-accepted");
        let child = accept_child(&mut managed, &genesis, staged);
        let old = store
            .maybe_basic_filter_append_proof()
            .expect("proof")
            .expect("still old durable authority");
        assert_eq!(old.durable_tip().0, 0);
        // Act
        managed
            .flush_with_mode(
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(10),
                u64::MAX,
            )
            .expect("tracked own flush");
        // Assert
        let current = store
            .maybe_basic_filter_append_proof()
            .expect("proof")
            .expect("confirmed");
        assert_eq!(
            current.durable_tip(),
            (1, open_bitcoin_core::consensus::block_hash(&child.header))
        );
        assert_eq!(current.safe_checkpoint(), old.safe_checkpoint());
        assert!(store.check_basic_filter_append_proof(&old).is_err());
        drop(current);
        drop(old);
        drop(managed);
        drop(store);
        let reopened = FjallNodeStore::open(&path).expect("real reopen");
        DurableSyncRuntime::open_configured(
            reopened,
            SyncRuntimeConfig::default(),
            BasicFilterStartupMode::Enabled,
        )
        .expect("full successful recovery");
        std::fs::remove_dir_all(path).expect("cleanup");
    }

    #[test]
    fn phase157_proof_direct_acceptance_tracks_before_deferred_flush() {
        accepted_flush(false);
    }
    #[test]
    fn phase157_proof_staged_acceptance_tracks_before_deferred_flush() {
        accepted_flush(true);
    }

    fn interleaved(point: BasicFilterWriterInterleave) {
        // Arrange
        let (path, runtime, _) = runtime("proof-interleaved");
        let before = runtime.store().maybe_basic_filter_state().expect("before");
        let locks = runtime.store().load_prune_locks().expect("locks");
        runtime.store().set_basic_filter_writer_interleave(point);
        // Act
        assert!(
            runtime
                .network_handle()
                .flush_coins(
                    FlushMode::Always,
                    FlushPolicyTime::from_unix_seconds(10),
                    u64::MAX
                )
                .is_err()
        );
        // Assert
        assert!(
            runtime
                .store()
                .clone()
                .maybe_basic_filter_append_proof()
                .expect("invalidated")
                .is_none()
        );
        assert_eq!(
            runtime
                .store()
                .maybe_basic_filter_state()
                .expect("unchanged"),
            before
        );
        assert_eq!(
            runtime.store().load_prune_locks().expect("unchanged locks"),
            locks
        );
        drop(runtime);
        std::fs::remove_dir_all(path).expect("cleanup");
    }

    #[test]
    fn phase157_proof_raw_clone_before_own_coins_refuses() {
        interleaved(BasicFilterWriterInterleave::BeforeCoins);
    }
    #[test]
    fn phase157_proof_raw_clone_between_coins_and_metadata_refuses() {
        interleaved(BasicFilterWriterInterleave::BeforeMetadata);
    }
    #[test]
    fn phase157_proof_raw_clone_after_metadata_refuses() {
        interleaved(BasicFilterWriterInterleave::BeforeConfirm);
    }
    #[test]
    fn phase157_proof_nominal_raw_pending_recreation_refuses() {
        interleaved(BasicFilterWriterInterleave::RecreateBeforeConfirm);
    }

    #[test]
    fn phase157_proof_public_snapshot_and_custom_parent_are_untracked() {
        // Arrange
        for custom in [false, true] {
            let (path, runtime, _) = runtime("proof-fake-manager");
            let snapshot = runtime
                .store()
                .wallet_scan_chainstate_snapshot()
                .expect("snapshot")
                .expect("present");
            let store = FjallChainstateStore::from_store(runtime.store().clone());
            let now = FlushPolicyTime::from_unix_seconds(0);
            // Act
            if custom {
                let cache = super::super::tests::dirty_recording_cache();
                let state = Chainstate::from_coins_cache(
                    cache,
                    snapshot.active_chain,
                    snapshot.undo_by_block,
                    snapshot.maybe_confirmed_txid_counts,
                );
                let mut fake = ManagedChainstate::from_chainstate(
                    store,
                    state,
                    FlushLifecycle::ready(now, now, 0, false),
                );
                assert!(fake.maybe_validated_lineage.is_none());
                fake.flush_with_mode(FlushMode::Always, now, u64::MAX)
                    .expect("nominal custom flush");
            } else {
                let mut fake = ManagedChainstate::from_chainstate(
                    store,
                    Chainstate::from_snapshot(snapshot),
                    FlushLifecycle::ready(now, now, 0, false),
                );
                assert!(fake.maybe_validated_lineage.is_none());
                fake.flush_with_mode(FlushMode::Always, now, u64::MAX)
                    .expect("nominal snapshot flush");
                assert!(fake.clone().maybe_validated_lineage.is_none());
            }
            // Assert
            assert!(
                runtime
                    .store()
                    .maybe_basic_filter_append_proof()
                    .expect("no forged completion")
                    .is_none()
            );
            drop(runtime);
            std::fs::remove_dir_all(path).expect("cleanup");
        }
    }

    #[test]
    fn phase157_proof_private_seed_refuses_foreign_genuine_parent() {
        // Arrange
        let (first_path, first, _) = runtime("proof-first-parent");
        let (second_path, second, _) = runtime("proof-second-parent");
        let positions = first
            .store()
            .load_chain_meta_for_open()
            .expect("metadata")
            .0;
        let state = Chainstate::from_coins_cache(
            CoinsCache::from_parent(second.store().coins_view()),
            positions,
            Default::default(),
            Some(Default::default()),
        );
        let now = FlushPolicyTime::from_unix_seconds(0);
        // Act
        let result = ManagedChainstate::from_recovered_chainstate(
            FjallChainstateStore::from_store(first.store().clone()),
            state,
            FlushLifecycle::ready(now, now, 0, false),
        );
        // Assert
        assert!(result.is_err());
        assert!(
            first
                .store()
                .maybe_basic_filter_append_proof()
                .expect("unchanged")
                .is_some()
        );
        drop(first);
        drop(second);
        std::fs::remove_dir_all(first_path).expect("cleanup");
        std::fs::remove_dir_all(second_path).expect("cleanup");
    }

    #[test]
    fn phase157_proof_actual_metadata_fault_preserves_safe_state_on_reopen() {
        // Arrange
        let (path, store, mut managed, genesis) = managed_fixture("proof-real-meta-fault");
        let child = accept_child(&mut managed, &genesis, true);
        let before = store.maybe_basic_filter_state().expect("before");
        store.set_basic_filter_fault(FilterPublicationFault::BeforeChainMeta);
        // Act
        assert!(
            managed
                .flush_with_mode(
                    FlushMode::Always,
                    FlushPolicyTime::from_unix_seconds(10),
                    u64::MAX
                )
                .is_err()
        );
        assert_eq!(
            store.coins_view().best_block().expect("actual B"),
            Some(open_bitcoin_core::consensus::block_hash(&child.header))
        );
        drop(managed);
        drop(store);
        let reopened = FjallNodeStore::open(&path).expect("real reopen");
        // Assert
        assert_eq!(
            reopened
                .maybe_basic_filter_state()
                .expect("saved safe state"),
            before
        );
        assert!(
            DurableSyncRuntime::open_configured(
                reopened,
                SyncRuntimeConfig::default(),
                BasicFilterStartupMode::Enabled
            )
            .is_err()
        );
        std::fs::remove_dir_all(path).expect("cleanup");
    }

    #[test]
    fn phase157_proof_no_write_flush_preserves_live_authority() {
        // Arrange
        let (path, runtime, _) = runtime("proof-noop");
        let old = runtime
            .store()
            .maybe_basic_filter_append_proof()
            .expect("proof")
            .expect("live");
        // Act
        let execution = runtime
            .network_handle()
            .flush_coins(
                FlushMode::None,
                FlushPolicyTime::from_unix_seconds(0),
                u64::MAX,
            )
            .expect("no write");
        // Assert
        assert!(!execution.wrote_coins);
        runtime
            .store()
            .check_basic_filter_append_proof(&old)
            .expect("unaffected live proof");
        drop(old);
        drop(runtime);
        std::fs::remove_dir_all(path).expect("cleanup");
    }

    #[test]
    fn phase157_proof_rejected_connect_leaves_validated_lineage_unchanged() {
        // Arrange
        let (path, store, mut managed, genesis) = managed_fixture("proof-rejected");
        let mut invalid = block(open_bitcoin_core::consensus::block_hash(&genesis.header), 1);
        invalid.header.merkle_root = Default::default();
        // Act
        assert!(
            managed
                .connect_block_with_current_time(
                    &invalid,
                    2,
                    1_101,
                    ScriptVerifyFlags::P2SH,
                    ConsensusParams::default()
                )
                .is_err()
        );
        managed
            .flush_with_mode(
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(10),
                u64::MAX,
            )
            .expect("unchanged accepted target flush");
        // Assert
        assert_eq!(
            store
                .maybe_basic_filter_append_proof()
                .expect("proof")
                .expect("live")
                .durable_tip()
                .0,
            0
        );
        drop(managed);
        drop(store);
        std::fs::remove_dir_all(path).expect("cleanup");
    }

    #[test]
    fn phase157_proof_own_revision_exhaustion_refuses_before_writes() {
        // Arrange
        let (path, runtime, _) = runtime("proof-exhausted");
        let before = runtime.store().maybe_basic_filter_state().expect("before");
        runtime
            .store()
            .set_basic_filter_revision_for_test(u64::MAX - 1);
        // Act
        assert!(
            runtime
                .network_handle()
                .flush_coins(
                    FlushMode::Always,
                    FlushPolicyTime::from_unix_seconds(10),
                    u64::MAX
                )
                .is_err()
        );
        // Assert
        assert!(
            runtime
                .store()
                .maybe_basic_filter_append_proof()
                .expect("no authority")
                .is_none()
        );
        assert_eq!(
            runtime
                .store()
                .maybe_basic_filter_state()
                .expect("unchanged"),
            before
        );
        drop(runtime);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
}
