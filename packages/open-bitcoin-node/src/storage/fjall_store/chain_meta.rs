// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txdb.cpp
// - packages/bitcoin-knots/src/coins.cpp

//! Complete metadata publication in the existing coins/validation transaction order.

use super::bounded_rows::{self, RowFamily};
use super::coins::{interrupted_coins_write, map_heads_error};
use super::*;
use crate::storage::snapshot_codec::{HydratedChainMeta, decode_chain_meta, encode_chain_meta};
use open_bitcoin_core::chainstate::CoinsView;
use std::collections::HashMap;

#[cfg(test)]
thread_local! {
    static METADATA_COUNTS_FAULT: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
}

#[cfg(test)]
fn metadata_counts_fault(after: bool) -> Result<(), StorageError> {
    if METADATA_COUNTS_FAULT.get() == Some(after) {
        METADATA_COUNTS_FAULT.set(None);
        return Err(StorageError::BackendFailure {
            namespace: StorageNamespace::Chainstate,
            message: "injected count metadata commit uncertainty".into(),
            action: StorageRecoveryAction::Restart,
        });
    }
    Ok(())
}

impl FjallNodeStore {
    pub fn save_chain_meta(
        &self,
        active_chain: &[open_bitcoin_core::chainstate::ChainPosition],
        mode: PersistMode,
    ) -> Result<(), StorageError> {
        self.save_unverified_chain_meta(active_chain, None, mode)
    }

    pub(super) fn save_unverified_chain_meta(
        &self,
        active_chain: &[open_bitcoin_core::chainstate::ChainPosition],
        maybe_counts: Option<&HashMap<open_bitcoin_core::primitives::Txid, u32>>,
        mode: PersistMode,
    ) -> Result<(), StorageError> {
        let mut control = self.filter_publication_guard()?;
        self.invalidate_validation_coverage()?;
        control.invalidate_append()?;
        control.chain_meta_fault()?;
        let meta_bytes = encode_chain_meta(active_chain, maybe_counts)?;
        self.replace_chain_meta_bytes(meta_bytes, mode)
    }

    /// Flush-only publication of already validated live ancestry. Full ancestry
    /// validation belongs to this full metadata writer, never to an index turn.
    pub(crate) fn save_validated_chain_meta(
        &self,
        active_chain: &[open_bitcoin_core::chainstate::ChainPosition],
        mode: PersistMode,
    ) -> Result<(), StorageError> {
        self.save_validated_chain_meta_with_counts(active_chain, None, mode)
    }

    /// Positions and the live owner's complete occurrences share one metadata batch.
    pub(crate) fn save_validated_chain_meta_with_counts(
        &self,
        active_chain: &[open_bitcoin_core::chainstate::ChainPosition],
        maybe_live_counts: Option<&HashMap<open_bitcoin_core::primitives::Txid, u32>>,
        mode: PersistMode,
    ) -> Result<(), StorageError> {
        if maybe_live_counts.is_some_and(|counts| counts.values().any(|count| *count == 0)) {
            return Err(corruption(
                StorageNamespace::Chainstate,
                "zero confirmed transaction count",
            ));
        }
        #[cfg(test)]
        self.run_basic_filter_writer_interleave(
            super::filters::BasicFilterWriterInterleave::BeforeMetadata,
        )?;
        let mut control = self.filter_publication_guard()?;
        // Counts describe the entire stored ancestry. Preserve only an existing
        // decoded map for an identical checkpoint; absence never means empty.
        let maybe_counts = self
            .read_chain_meta_bytes()?
            .map(|bytes| decode_chain_meta(&bytes))
            .transpose()?
            .and_then(|(stored, maybe_counts)| {
                if stored.as_slice() == active_chain {
                    return maybe_counts;
                }
                None
            });
        let maybe_pending = control.maybe_pending_coins;
        self.check_validation_coverage_for_metadata(active_chain)?;
        control.invalidate_append()?;
        control.chain_meta_fault()?;
        let view = self.coins_view();
        if !view.head_blocks().map_err(map_heads_error)?.is_empty() {
            return Err(interrupted_coins_write());
        }
        let maybe_refresh = match maybe_pending {
            Some(coins)
                if coins.completed
                    && coins.previous.revision.checked_add(1) == Some(coins.revision)
                    && coins.revision.checked_add(1) == Some(control.revision) =>
            {
                let mut identity = coins.previous;
                let fence = open_bitcoin_core::chainstate::VerifiedChainstateFence::new(
                    view.best_block().map_err(map_heads_error)?,
                    Some(active_chain),
                )
                .map_err(crate::storage::filter_index::index_corruption)?;
                if fence.tip().block_hash != coins.new_tip
                    || (!identity.maybe_replacement.is_some_and(|replacement| {
                        replacement.maybe_displaced_fence
                            == Some((identity.durable_height, identity.durable_hash))
                    }) && fence
                        .maybe_position(identity.durable_height)
                        .map(|p| p.block_hash)
                        != Some(identity.durable_hash))
                {
                    return Err(crate::storage::filter_index::index_corruption(
                        "incompatible BASIC durable metadata publication",
                    ));
                }
                identity.durable_height = fence.tip().height;
                identity.durable_hash = fence.tip().block_hash;
                identity.revision = control.revision;
                Some(super::filters::publication::CompletedMetadataPublication { coins, identity })
            }
            _ => None,
        };
        let bytes = encode_chain_meta(active_chain, maybe_live_counts.or(maybe_counts.as_ref()))?;
        self.replace_chain_meta_bytes(bytes, mode)?;
        // A publicly callable sink cannot confer live authority. Only the actual
        // managed validated flush owner can confirm these achieved pending facts.
        control.maybe_completed_metadata = maybe_refresh;
        Ok(())
    }

    pub(crate) fn load_chain_meta_for_open(&self) -> Result<HydratedChainMeta, StorageError> {
        if let Some(bytes) = self.read_chain_meta_bytes()? {
            return decode_chain_meta(&bytes);
        }
        Ok((Vec::new(), Some(HashMap::new())))
    }

    pub(super) fn load_hydrate_chain_meta(&self) -> Result<HydratedChainMeta, StorageError> {
        if let Some(bytes) = self.read_chain_meta_bytes()? {
            return decode_chain_meta(&bytes);
        }
        let Some(leftover) = self.load_chainstate_snapshot()? else {
            return Ok((Vec::new(), None));
        };
        Ok((leftover.active_chain, leftover.maybe_confirmed_txid_counts))
    }

    fn read_chain_meta_bytes(&self) -> Result<Option<Vec<u8>>, StorageError> {
        bounded_rows::read(self, RowFamily::ChainMeta)
    }

    /// Explicit synthetic fixture enrichment under the caller's same-store guard.
    /// This data replacement changes no coverage, BASIC or payload capability.
    #[cfg(test)]
    pub(crate) fn replace_live_fixture_chain_meta_under_guard_for_test(
        &self,
        positions: &[open_bitcoin_core::chainstate::ChainPosition],
        counts: &HashMap<open_bitcoin_core::primitives::Txid, u32>,
        _guard: &std::sync::MutexGuard<'_, super::filters::PublicationControl>,
    ) -> Result<(), StorageError> {
        self.replace_chain_meta_bytes(
            encode_chain_meta(positions, Some(counts))?,
            PersistMode::Sync,
        )
    }

    #[cfg(test)]
    pub(crate) fn chain_meta_sequence_for_test(&self) -> u64 {
        self.db.visible_seqno()
    }

    fn replace_chain_meta_bytes(
        &self,
        bytes: Vec<u8>,
        mode: PersistMode,
    ) -> Result<(), StorageError> {
        let prepared = bounded_rows::prepare(RowFamily::ChainMeta, bytes)?;
        let mut batch = self.db.batch();
        if let Some(mode) = fjall_persist_mode(mode) {
            batch = batch.durability(Some(mode));
        }
        bounded_rows::append_replacement(self, &mut batch, prepared)?;
        #[cfg(test)]
        bounded_rows::metadata_commit_fault(false)?;
        #[cfg(test)]
        metadata_counts_fault(false)?;
        batch
            .commit()
            .map_err(|error| backend_failure(StorageNamespace::Chainstate, error))?;
        #[cfg(test)]
        bounded_rows::metadata_commit_fault(true)?;
        #[cfg(test)]
        metadata_counts_fault(true)?;
        Ok(())
    }
}

#[cfg(test)]
mod phase167_counts {
    use super::*;
    use crate::chainstate::BasicFilterStartupMode;
    use crate::chainstate::validation_history::tests::phase167 as genuine;
    use open_bitcoin_core::{
        chainstate::{FlushMode, FlushPolicyTime},
        consensus::block_hash,
    };

    fn with_data_store(
        check: impl FnOnce(FjallNodeStore, Vec<open_bitcoin_core::chainstate::ChainPosition>),
    ) {
        let mut nonce = [0; 16];
        getrandom::fill(&mut nonce).expect("nonce");
        let root = std::fs::canonicalize(std::env::temp_dir())
            .expect("physical parent")
            .join(format!(
                "phase167-count-carrier-{:x}",
                u128::from_le_bytes(nonce)
            ));
        let store = FjallNodeStore::open(&root).expect("data-only store");
        let genesis = open_bitcoin_core::consensus::CanonicalChain::Regtest
            .spec()
            .genesis_block();
        let position = open_bitcoin_core::chainstate::ChainPosition::new(
            genesis.header.clone(),
            0,
            open_bitcoin_core::consensus::chain_proof::block_proof(genesis.header.bits)
                .expect("numeric proof"),
            i64::from(genesis.header.time),
        );
        check(store, vec![position]);
        std::fs::remove_dir_all(root).expect("all handles dropped");
    }

    #[test]
    fn phase167_durable_counts_typed_multiplicity_and_same_ancestry_updates() {
        with_data_store(|store, positions| {
            // Arrange: deliberately injected typed data, never canonical acceptance.
            let positions = vec![positions[0].clone(); 401];
            let key = open_bitcoin_core::primitives::Txid::from_byte_array([7; 32]);
            let other = open_bitcoin_core::primitives::Txid::from_byte_array([8; 32]);
            let first = HashMap::from([(key, 2), (other, 17)]);
            // Act / Assert
            store
                .save_validated_chain_meta_with_counts(&positions, Some(&first), PersistMode::Sync)
                .expect("data writer");
            assert_eq!(
                store.load_chain_meta_for_open().expect("all pairs"),
                (positions.clone(), Some(first))
            );
            let updated = HashMap::from([(key, 3), (other, u32::MAX)]);
            store
                .save_validated_chain_meta_with_counts(
                    &positions,
                    Some(&updated),
                    PersistMode::Sync,
                )
                .expect("same ancestry Some replaces");
            assert_eq!(
                store.load_chain_meta_for_open().expect("exact update"),
                (positions.clone(), Some(updated.clone()))
            );
            store
                .save_validated_chain_meta(&positions, PersistMode::Sync)
                .expect("legacy None same ancestry");
            assert_eq!(
                store
                    .load_chain_meta_for_open()
                    .expect("None retains known"),
                (positions.clone(), Some(updated))
            );
            let changed = positions[..1].to_vec();
            let replacement = HashMap::from([(key, 2)]);
            store
                .save_validated_chain_meta_with_counts(
                    &changed,
                    Some(&replacement),
                    PersistMode::Sync,
                )
                .expect("Some changed ancestry replaces map");
            let expected = (changed, Some(replacement));
            let root = store.datadir().to_path_buf();
            drop(store);
            let reopened = FjallNodeStore::open(root).expect("all handle data reopen");
            assert_eq!(
                reopened
                    .load_chain_meta_for_open()
                    .expect("typed carrier reopen"),
                expected
            );
        });
    }

    #[test]
    fn phase167_durable_counts_mixed_or_truncated_prior_family_refuses_some_without_repair() {
        with_data_store(|store, positions| {
            // Arrange: data-only large family; corruption cannot be repaired by live data.
            let large = vec![positions[0].clone(); 401];
            for mixed in [true, false] {
                store
                    .replace_chain_meta_bytes(
                        encode_chain_meta(&large, None).expect("large data"),
                        PersistMode::Sync,
                    )
                    .expect("bounded family");
                if mixed {
                    store
                        .write_raw_for_test(
                            StorageNamespace::Chainstate,
                            "chain_meta",
                            encode_chain_meta(&positions, None).expect("root"),
                        )
                        .expect("mixed root");
                } else {
                    let key = store
                        .chainstate
                        .prefix("chain_meta:segment:v1:")
                        .next()
                        .expect("fragment")
                        .key()
                        .expect("key");
                    store.chainstate.remove(key).expect("missing fragment");
                }
                let sequence = store.chain_meta_sequence_for_test();
                // Act
                assert!(
                    store
                        .save_validated_chain_meta_with_counts(
                            &positions,
                            Some(&HashMap::new()),
                            PersistMode::Sync
                        )
                        .is_err()
                );
                // Assert
                assert_eq!(store.chain_meta_sequence_for_test(), sequence);
                assert!(store.load_chain_meta_for_open().is_err());
            }
        });
    }

    #[test]
    fn phase167_durable_counts_none_changed_ancestry_and_no_previous_map_stay_unknown() {
        with_data_store(|store, positions| {
            // Arrange
            store
                .save_validated_chain_meta(&positions, PersistMode::Sync)
                .expect("no previous data");
            assert_eq!(
                store.load_chain_meta_for_open().expect("unknown"),
                (positions.clone(), None)
            );
            store
                .save_validated_chain_meta_with_counts(
                    &positions,
                    Some(&HashMap::new()),
                    PersistMode::Sync,
                )
                .expect("explicit known empty");
            let changed = vec![positions[0].clone(); 2];
            // Act
            store
                .save_validated_chain_meta(&changed, PersistMode::Sync)
                .expect("generic changed ancestry");
            // Assert
            assert_eq!(
                store
                    .load_chain_meta_for_open()
                    .expect("unknown remains unknown"),
                (changed, None)
            );
        });
    }

    #[test]
    fn phase167_durable_counts_zero_map_refuses_without_writes() {
        with_data_store(|store, positions| {
            // Arrange
            store
                .save_validated_chain_meta_with_counts(
                    &positions,
                    Some(&HashMap::new()),
                    PersistMode::Sync,
                )
                .expect("old known empty");
            let old = store.load_chain_meta_for_open().expect("prior");
            let sequence = store.chain_meta_sequence_for_test();
            let counts = HashMap::from([(
                open_bitcoin_core::primitives::Txid::from_byte_array([9; 32]),
                0,
            )]);
            // Act
            assert!(
                store
                    .save_validated_chain_meta_with_counts(
                        &positions,
                        Some(&counts),
                        PersistMode::Sync
                    )
                    .is_err()
            );
            // Assert
            assert_eq!(store.chain_meta_sequence_for_test(), sequence);
            assert_eq!(store.load_chain_meta_for_open().expect("prior intact"), old);
        });
    }

    #[test]
    fn phase167_durable_counts_malformed_prior_map_cannot_be_repaired_by_some() {
        with_data_store(|store, positions| {
            // Arrange / Act / Assert: exact codec refusals, with missing retained as unknown.
            let key = open_bitcoin_core::primitives::Txid::from_byte_array([7; 32]);
            let counts = HashMap::from([(key, 2)]);
            let mut json: serde_json::Value = serde_json::from_slice(
                &encode_chain_meta(&positions, Some(&counts)).expect("data"),
            )
            .expect("JSON");
            for duplicate in [false, true] {
                let mut damaged = json.clone();
                if duplicate {
                    let entry = damaged["payload"]["maybe_confirmed_txid_counts"][0].clone();
                    damaged["payload"]["maybe_confirmed_txid_counts"]
                        .as_array_mut()
                        .expect("entries")
                        .push(entry);
                } else {
                    damaged["payload"]["maybe_confirmed_txid_counts"][0]["active_chain_occurrences"] =
                        serde_json::json!(0);
                }
                let bytes = serde_json::to_vec(&damaged).expect("corrupt bytes");
                assert!(decode_chain_meta(&bytes).is_err());
                store
                    .write_raw_for_test(StorageNamespace::Chainstate, "chain_meta", bytes)
                    .expect("explicit raw corruption");
                let sequence = store.chain_meta_sequence_for_test();
                assert!(
                    store
                        .save_validated_chain_meta_with_counts(
                            &positions,
                            Some(&counts),
                            PersistMode::Sync
                        )
                        .is_err()
                );
                assert_eq!(store.chain_meta_sequence_for_test(), sequence);
            }
            json["payload"]
                .as_object_mut()
                .expect("payload")
                .remove("maybe_confirmed_txid_counts");
            assert_eq!(
                decode_chain_meta(&serde_json::to_vec(&json).expect("missing bytes"))
                    .expect("legacy absence"),
                (positions, None)
            );
        });
    }

    #[test]
    fn phase167_durable_counts_reopen_seeded_finite_suffix_remains_none() {
        genuine::with_manager(|manager, store, params| {
            // Arrange: explicit incomplete suffix, not a canonical root proof.
            let root = store.datadir().to_path_buf();
            let snapshot = manager.export_chainstate_snapshot().expect("genesis data");
            let block = genuine::child(&snapshot.active_chain[0].header, 1);
            let history = open_bitcoin_core::consensus::CandidateHistory::check_seeded(
                open_bitcoin_core::consensus::CanonicalChain::Regtest
                    .spec()
                    .rules(),
                block.header.clone(),
                open_bitcoin_core::consensus::HistoryHeader::new(
                    open_bitcoin_core::consensus::CanonicalChain::Regtest,
                    0,
                    snapshot.active_chain[0].header.clone(),
                ),
                vec![],
                None,
            )
            .expect("conditional suffix");
            let core =
                open_bitcoin_core::chainstate::Chainstate::from_coins_cache_for_mechanical_fixture(
                    open_bitcoin_core::chainstate::CoinsCache::from_parent(store.coins_view()),
                    snapshot.active_chain,
                    snapshot.undo_by_block,
                    None,
                );
            let mut fixture = crate::chainstate::ManagedChainstate::from_chainstate(
                crate::chainstate::FjallChainstateStore::from_store(store.clone()),
                core,
                manager.flush_lifecycle.clone(),
            );
            drop(manager);
            // Act
            fixture
                .connect_seeded_historical_fixture(&block, &history, i64::from(block.header.time))
                .expect("real conditional kernel/flush");
            let expected = fixture
                .export_chainstate_snapshot()
                .expect("full fixture state");
            assert!(fixture.chainstate().bind_live_work_source(params).is_err());
            assert!(fixture.chainstate().maybe_canonical_chain().is_none());
            assert!(expected.maybe_confirmed_txid_counts.is_none());
            drop(fixture);
            drop(store);
            let reopened = FjallNodeStore::open(root).expect("all handle seeded reopen");
            // Assert
            assert_eq!(
                reopened
                    .hydrate_chainstate_for_open()
                    .expect("data decode")
                    .expect("present"),
                expected
            );
            assert!(
                reopened
                    .load_chain_meta_for_open()
                    .expect("generic metadata")
                    .1
                    .is_none()
            );
        });
    }

    fn metadata_fault(after: bool) {
        genuine::with_mode(
            BasicFilterStartupMode::Enabled,
            |mut manager, store, params| {
                // Arrange
                let root = store.datadir().to_path_buf();
                let old = store
                    .load_chain_meta_for_open()
                    .expect("prior complete metadata");
                let block = genuine::child(&manager.chainstate().tip().expect("genesis").header, 1);
                genuine::connect(&mut manager, &block, params)
                    .expect("actual canonical absorption");
                let snapshot = manager.export_chainstate_snapshot().expect("accepted");
                let safe = store.maybe_basic_filter_state().expect("safe");
                METADATA_COUNTS_FAULT.set(Some(after));
                // Act
                assert!(
                    manager
                        .flush_with_mode(
                            FlushMode::Always,
                            FlushPolicyTime::from_unix_seconds(50_001),
                            u64::MAX
                        )
                        .is_err()
                );
                // Assert: completed coins stay written; failed metadata never confirms BASIC.
                assert_eq!(
                    manager
                        .export_chainstate_snapshot()
                        .expect("unchanged accepted state"),
                    snapshot
                );
                assert_eq!(
                    store.coins_view().best_block().expect("actual coins B"),
                    Some(block_hash(&block.header))
                );
                assert_eq!(
                    store.maybe_basic_filter_state().expect("safe unchanged"),
                    safe
                );
                assert!(
                    store
                        .maybe_basic_filter_append_proof()
                        .expect("no false completion")
                        .is_none()
                );
                let expected = if after {
                    (snapshot.active_chain, snapshot.maybe_confirmed_txid_counts)
                } else {
                    old
                };
                assert_eq!(
                    store.load_chain_meta_for_open().expect("one full family"),
                    expected
                );
                drop(manager);
                drop(store);
                let reopened =
                    FjallNodeStore::open(root).expect("physical reopen after uncertainty");
                assert_eq!(
                    reopened
                        .load_chain_meta_for_open()
                        .expect("complete family"),
                    expected
                );
                assert_eq!(reopened.maybe_basic_filter_state().expect("safe"), safe);
                assert!(
                    reopened
                        .maybe_basic_filter_append_proof()
                        .expect("raw data only")
                        .is_none()
                );
            },
        );
    }

    #[test]
    fn phase167_durable_counts_reopen_metadata_before_commit() {
        metadata_fault(false);
    }
    #[test]
    fn phase167_durable_counts_reopen_metadata_after_commit_uncertainty() {
        metadata_fault(true);
    }
}
