// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp

//! Explicit fixture transfer of a genuine live owner, never raw recovery acceptance.

use super::{
    Chainstate, ChainstateError, FjallChainstateStore, HashMap, ManagedChainstate, map_persist,
};
use crate::storage::coins_view::FjallCoinsView;
use open_bitcoin_core::chainstate::CoinsView;

impl ManagedChainstate<FjallChainstateStore, FjallCoinsView> {
    /// Explicit synthetic fixture enrichment after a real flush. These rows
    /// grant no canonical authority.
    pub(crate) fn persist_missing_live_fixture_counts(&mut self) -> Result<(), ChainstateError> {
        self.chainstate.validate_live_origin()?;
        let store = self.store.inner().clone();
        if !self.chainstate.coins().parent().belongs_to(&store)
            || self.has_pending_validation_history()
        {
            return Err(ChainstateError::WorkRebindMismatch);
        }
        let _guard = store.filter_publication_guard().map_err(map_persist)?;
        let current = self.chainstate.overlay_snapshot();
        let Some(counts) = current.maybe_confirmed_txid_counts.as_ref() else {
            return Err(ChainstateError::WorkRebindMismatch);
        };
        let (positions, maybe_saved_counts) =
            store.load_chain_meta_for_open().map_err(map_persist)?;
        let live_coins = self.chainstate.coins().complete_snapshot()?;
        let physical_coins = store.coins_view().complete_snapshot()?;
        if positions != current.active_chain
            || live_coins != physical_coins
            || !physical_coins.head_blocks.is_empty()
            || physical_coins.maybe_best_block != self.chainstate.tip().map(|tip| tip.block_hash)
        {
            return Err(ChainstateError::WorkRebindMismatch);
        }
        if let Some(saved) = maybe_saved_counts {
            return if saved == *counts {
                Ok(())
            } else {
                Err(ChainstateError::WorkRebindMismatch)
            };
        }
        store
            .replace_live_fixture_chain_meta_under_guard_for_test(&positions, counts, &_guard)
            .map_err(map_persist)
    }

    pub(crate) fn adopt_live_fixture_owner(
        &mut self,
        source: Chainstate,
    ) -> Result<(), ChainstateError> {
        let store = self.store.inner().clone();
        if !self.chainstate.coins().parent().belongs_to(&store)
            || self.has_pending_validation_history()
        {
            return Err(ChainstateError::WorkRebindMismatch);
        }
        let _guard = store.filter_publication_guard().map_err(map_persist)?;
        let (positions, maybe_counts) = store.load_chain_meta_for_open().map_err(map_persist)?;
        let undo = store.load_all_undo_records().map_err(map_persist)?;
        let current = self.chainstate.overlay_snapshot();
        let empty_counts = positions.is_empty()
            && undo.is_empty()
            && maybe_counts.as_ref().is_none_or(HashMap::is_empty)
            && current
                .maybe_confirmed_txid_counts
                .as_ref()
                .is_none_or(HashMap::is_empty);
        if positions != current.active_chain
            || undo != current.undo_by_block
            || (!empty_counts && maybe_counts != current.maybe_confirmed_txid_counts)
        {
            return Err(ChainstateError::WorkRebindMismatch);
        }
        source.rebind_into(&mut self.chainstate)
    }
}

#[cfg(test)]
impl ManagedChainstate<FjallChainstateStore, FjallCoinsView> {
    /// Attach BASIC to the same genuine live fixture after its actual full flush.
    /// No core source, body history, durable work or ordinary support is imported.
    #[cfg(test)]
    pub(crate) fn attach_fixture_basic_owner_after_flush(
        &mut self,
        params: open_bitcoin_core::consensus::ConsensusParams,
    ) -> Result<(), ChainstateError> {
        use open_bitcoin_core::chainstate::{
            IndexPrefix,
            filter_index::catch_up::{AcceptedIndexTarget, BasicIndexProgress},
        };
        self.chainstate.bind_live_work_source(params)?;
        let store = self.store.inner().clone();
        if !self.chainstate.coins().parent().belongs_to(&store)
            || self.has_pending_validation_history()
            || self.maybe_validated_lineage.is_some()
            || self.maybe_basic_index_owner.is_some()
        {
            return Err(ChainstateError::WorkRebindMismatch);
        }
        let proof = store
            .maybe_basic_filter_append_proof()
            .map_err(map_persist)?
            .ok_or(ChainstateError::WorkRebindMismatch)?;
        let control = store.filter_publication_guard().map_err(map_persist)?;
        let current = self.chainstate.overlay_snapshot();
        let (positions, counts) = store.load_chain_meta_for_open().map_err(map_persist)?;
        let physical = store.coins_view().complete_snapshot()?;
        if positions != current.active_chain
            || counts != current.maybe_confirmed_txid_counts
            || store.load_all_undo_records().map_err(map_persist)? != current.undo_by_block
            || self.chainstate.coins().complete_snapshot()? != physical
            || !physical.head_blocks.is_empty()
            || physical.maybe_best_block != self.chainstate.tip().map(|tip| tip.block_hash)
        {
            return Err(ChainstateError::WorkRebindMismatch);
        }
        let tip = self.chainstate.tip().ok_or(ChainstateError::MissingTip)?;
        let lineage = super::fjall_store::ValidatedChainstateLineage::from_fixture_flush_proof(
            &store, &proof, tip, &control,
        )
        .map_err(map_persist)?;
        let endpoint =
            |checkpoint: open_bitcoin_core::chainstate::FilterCheckpoint| match checkpoint.prefix()
            {
                IndexPrefix::Empty => None,
                IndexPrefix::Committed(identity) => Some(identity),
            };
        let progress = BasicIndexProgress::new(
            proof.generation(),
            proof.branch_identity(),
            AcceptedIndexTarget::new(tip.height, tip.block_hash),
            endpoint(proof.processed()),
            endpoint(proof.safe_checkpoint()),
            proof.protection(),
        )
        .map_err(|_| ChainstateError::WorkRebindMismatch)?;
        self.install_basic_index_owner(progress)
            .map_err(|_| ChainstateError::WorkRebindMismatch)?;
        self.maybe_validated_lineage = Some(lineage);
        Ok(())
    }
}

#[cfg(test)]
mod bounded_row_tests {
    use super::*;
    use crate::chainstate::validation_history::tests::phase167;
    use open_bitcoin_core::{
        chainstate::{FlushMode, FlushPolicyTime},
        consensus::{ConsensusParams, ScriptVerifyFlags, chain_proof::block_proof},
    };

    #[test]
    fn phase167_durable_rows_reopen_fixture_counts_preserve_full_synthetic_state_and_proofs() {
        // Arrange: an actual synthetic Live401-position owner, never canonical.
        let params = ConsensusParams {
            subsidy_halving_interval: 150,
            ..Default::default()
        };
        let (path, store, mut manager, _) =
            crate::chainstate::flush_lifecycle::managed_fixture_with_params(
                "phase167-bounded-fixture-counts",
                params,
            );
        for height in 1..=400 {
            let parent = manager.chainstate().tip().expect("synthetic parent");
            let block = phase167::child(&parent.header, height);
            let work = parent
                .chain_work
                .wrapping_add(block_proof(block.header.bits).expect("work"));
            manager
                .connect_block_with_current_time(
                    &block,
                    work,
                    2_000_000_000,
                    ScriptVerifyFlags::NONE,
                    params,
                )
                .expect("real managed synthetic absorption");
        }
        let headers: Vec<_> = manager
            .chainstate()
            .active_chain()
            .iter()
            .map(|p| open_bitcoin_network::HeaderEntry {
                block_hash: p.block_hash,
                header: p.header.clone(),
                height: p.height,
                chain_work: p.chain_work,
            })
            .collect();
        store
            .save_header_entries(&headers, crate::PersistMode::Sync)
            .expect("complete fixture headers");
        manager
            .flush_with_mode(
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(50_001),
                u64::MAX,
            )
            .expect("actual complete checkpoint");
        assert!(
            store
                .load_chain_meta_for_open()
                .expect("None policy")
                .1
                .is_none()
        );
        let complete = manager
            .export_chainstate_snapshot()
            .expect("full live state");
        assert_eq!(complete.active_chain.len(), 401);
        assert_eq!(
            complete
                .maybe_confirmed_txid_counts
                .as_ref()
                .expect("full existing map")
                .len(),
            401
        );
        let coins = store
            .coins_view()
            .complete_snapshot()
            .expect("physical Coins");
        let undos = store.load_all_undo_records().expect("complete undos");
        let basic = store.maybe_basic_filter_state().expect("BASIC");
        let progress = manager.maybe_basic_index_progress();
        let provenance: Vec<_> = complete
            .active_chain
            .iter()
            .map(|p| {
                store
                    .validation_provenance(p.block_hash)
                    .expect("provenance")
            })
            .collect();
        let payload = store.payload_usage_revision().expect("payload epoch");
        let proof = store
            .maybe_basic_filter_append_proof()
            .expect("proof")
            .expect("held append proof");
        let observations = (
            proof.revision(),
            proof.generation(),
            proof.branch_identity(),
            proof.processed(),
            proof.safe_checkpoint(),
            proof.protection(),
        );
        {
            let guard = store.filter_publication_guard().expect("same-store guard");
            store
                .check_fixture_append_proof_guarded(&proof, &guard)
                .expect("existing proof valid before enrichment");
        }
        assert!(manager.chainstate().maybe_canonical_chain().is_none());
        // Act
        manager
            .persist_missing_live_fixture_counts()
            .expect("explicit exact synthetic map enrichment");
        let sequence = store.chain_meta_sequence_for_test();
        manager
            .persist_missing_live_fixture_counts()
            .expect("equal map is write-free");
        // Assert
        assert_eq!(store.chain_meta_sequence_for_test(), sequence);
        assert_eq!(
            manager
                .export_chainstate_snapshot()
                .expect("unchanged live"),
            complete
        );
        assert_eq!(
            store.load_chain_meta_for_open().expect("all exact pairs"),
            (
                complete.active_chain.clone(),
                complete.maybe_confirmed_txid_counts.clone()
            )
        );
        assert_eq!(
            store.coins_view().complete_snapshot().expect("Coins"),
            coins
        );
        assert_eq!(store.load_all_undo_records().expect("undos"), undos);
        assert_eq!(store.maybe_basic_filter_state().expect("BASIC"), basic);
        assert_eq!(manager.maybe_basic_index_progress(), progress);
        assert_eq!(store.payload_usage_revision().expect("payload"), payload);
        assert_eq!(
            (
                proof.revision(),
                proof.generation(),
                proof.branch_identity(),
                proof.processed(),
                proof.safe_checkpoint(),
                proof.protection()
            ),
            observations
        );
        {
            let guard = store.filter_publication_guard().expect("same-store guard");
            store
                .check_fixture_append_proof_guarded(&proof, &guard)
                .expect("already-held proof remains valid");
        }
        for (p, expected) in complete.active_chain.iter().zip(&provenance) {
            assert_eq!(
                &store
                    .validation_provenance(p.block_hash)
                    .expect("unchanged provenance"),
                expected
            );
        }
        assert!(manager.chainstate().maybe_canonical_chain().is_none());
        drop(proof);
        drop(manager);
        drop(store);
        let measured = crate::storage::fjall_store::measure_detached_rows_for_test(&path)
            .expect("segmented synthetic storage data");
        assert_eq!(measured.authority_max, 65536);
        let reopened = crate::FjallNodeStore::open(&path).expect("all-handle physical reopen");
        assert_eq!(
            reopened
                .wallet_scan_chainstate_snapshot()
                .expect("wallet")
                .expect("full"),
            complete
        );
        assert_eq!(
            reopened
                .hydrate_chainstate_for_open()
                .expect("hydrate")
                .expect("full"),
            complete
        );
        assert_eq!(
            reopened
                .load_header_entries()
                .expect("headers")
                .expect("present")
                .entries,
            headers
        );
        assert_eq!(
            reopened
                .load_block_index_entries()
                .expect("index")
                .expect("present")
                .entries,
            headers
        );
        assert_eq!(reopened.maybe_basic_filter_state().expect("BASIC"), basic);
        for (p, expected) in complete.active_chain.iter().zip(provenance) {
            assert_eq!(
                reopened
                    .validation_provenance(p.block_hash)
                    .expect("provenance"),
                expected
            );
        }
        drop(reopened);
        assert!(
            crate::FjallNodeStore::open_canonical_genesis(
                &path,
                open_bitcoin_core::consensus::CanonicalChain::Regtest
            )
            .is_err(),
            "synthetic storage cannot become ordinary canonical history"
        );
        let reopened = crate::FjallNodeStore::open(&path).expect("unchanged synthetic data");
        // An unchanged raw corruption API must still introduce a mixed family.
        let raw = crate::storage::snapshot_codec::encode_chain_meta(
            &complete.active_chain,
            complete.maybe_confirmed_txid_counts.as_ref(),
        )
        .expect("raw aggregate");
        reopened
            .write_raw_for_test(crate::StorageNamespace::Chainstate, "chain_meta", raw)
            .expect("raw remains raw");
        let sequence = reopened.chain_meta_sequence_for_test();
        assert!(reopened.load_chain_meta_for_open().is_err());
        assert!(reopened.load_chain_meta_for_open().is_err());
        assert_eq!(
            reopened.chain_meta_sequence_for_test(),
            sequence,
            "no read-time repair"
        );
        drop(reopened);
        std::fs::remove_dir_all(path).expect("closed all owners");
    }

    #[test]
    fn phase167_durable_rows_reopen_fixture_counts_refusals_are_nonmutating() {
        for scenario in [
            "origin",
            "unflushed",
            "positions",
            "coins",
            "heads",
            "best",
            "conflict",
        ] {
            // Arrange: perturb exactly one existing guard input.
            let params = ConsensusParams::default();
            let (path, store, mut manager, _) =
                crate::chainstate::flush_lifecycle::managed_fixture_with_params(
                    &format!("phase167-count-refusal-{scenario}"),
                    params,
                );
            let parent = manager.chainstate().tip().expect("parent");
            let mut child = phase167::child(&parent.header, 1);
            child.transactions[0].outputs[0].value =
                open_bitcoin_core::primitives::Amount::from_sats(5_000_000_000).expect("subsidy");
            phase167::remine(&mut child);
            let work = parent
                .chain_work
                .wrapping_add(block_proof(child.header.bits).expect("work"));
            manager
                .connect_block_with_current_time(
                    &child,
                    work,
                    2_000_000_000,
                    ScriptVerifyFlags::NONE,
                    params,
                )
                .expect("actual child");
            if scenario != "unflushed" {
                manager
                    .flush_with_mode(
                        FlushMode::Always,
                        FlushPolicyTime::from_unix_seconds(50_001),
                        u64::MAX,
                    )
                    .expect("full flush");
            }
            let snapshot = manager
                .export_chainstate_snapshot()
                .expect("live complete state");
            match scenario {
                "origin" => {
                    manager.chainstate = Chainstate::from_coins_cache(
                        open_bitcoin_core::chainstate::CoinsCache::from_parent(store.coins_view()),
                        snapshot.active_chain.clone(),
                        snapshot.undo_by_block.clone(),
                        snapshot.maybe_confirmed_txid_counts.clone(),
                    )
                }
                "positions" => {
                    let bytes = crate::storage::snapshot_codec::encode_chain_meta(
                        &snapshot.active_chain[..1],
                        None,
                    )
                    .expect("different positions");
                    store
                        .write_raw_for_test(
                            crate::StorageNamespace::Chainstate,
                            "chain_meta",
                            bytes,
                        )
                        .expect("raw positions");
                }
                "coins" => {
                    let parent = manager.chainstate().tip().expect("parent");
                    let next = phase167::child(&parent.header, 2);
                    let work = parent
                        .chain_work
                        .wrapping_add(block_proof(next.header.bits).expect("work"));
                    manager
                        .connect_block_with_current_time(
                            &next,
                            work,
                            2_000_000_000,
                            ScriptVerifyFlags::NONE,
                            params,
                        )
                        .expect("actual unflushed Coin overlay");
                    let positions = manager.chainstate().active_chain();
                    let bytes = crate::storage::snapshot_codec::encode_chain_meta(positions, None)
                        .expect("matching positions");
                    store
                        .write_raw_for_test(
                            crate::StorageNamespace::Chainstate,
                            "chain_meta",
                            bytes,
                        )
                        .expect("matching metadata");
                    store
                        .write_raw_for_test(
                            crate::StorageNamespace::Coins,
                            "B",
                            crate::storage::coins_codec::encode_best_block_value(
                                positions.last().expect("tip").block_hash,
                            ),
                        )
                        .expect("matching best, missing physical Coin");
                }
                "heads" => store
                    .write_raw_for_test(
                        crate::StorageNamespace::Coins,
                        "H",
                        crate::storage::coins_codec::encode_head_blocks_value(&[
                            snapshot.active_chain[1].block_hash,
                            snapshot.active_chain[0].block_hash,
                        ])
                        .expect("heads"),
                    )
                    .expect("heads marker"),
                "best" => store
                    .write_raw_for_test(
                        crate::StorageNamespace::Coins,
                        "B",
                        crate::storage::coins_codec::encode_best_block_value(
                            snapshot.active_chain[0].block_hash,
                        ),
                    )
                    .expect("different best"),
                "conflict" => {
                    let mut counts = snapshot
                        .maybe_confirmed_txid_counts
                        .clone()
                        .expect("full map");
                    *counts.values_mut().next().expect("pair") += 1;
                    let bytes = crate::storage::snapshot_codec::encode_chain_meta(
                        &snapshot.active_chain,
                        Some(&counts),
                    )
                    .expect("conflict");
                    store
                        .write_raw_for_test(
                            crate::StorageNamespace::Chainstate,
                            "chain_meta",
                            bytes,
                        )
                        .expect("raw conflict");
                }
                _ => {}
            }
            let before = manager.export_chainstate_snapshot().expect("live snapshot");
            let sequence = store.chain_meta_sequence_for_test();
            let metadata = store.load_chain_meta_for_open().expect("metadata");
            let coins = store.coins_view().complete_snapshot();
            let basic = store.maybe_basic_filter_state().expect("BASIC");
            let payload = store.payload_usage_revision().expect("payload");
            // Act
            let result = manager.persist_missing_live_fixture_counts();
            // Assert
            let expected = if scenario == "origin" {
                ChainstateError::UntrustedWorkLineage
            } else if scenario == "heads" {
                ChainstateError::CoinsStorage {
                    detail: "storage corruption in coins: head_blocks still present after best-block write; Run the storage repair flow before restarting normal operation.".to_owned(),
                }
            } else {
                ChainstateError::WorkRebindMismatch
            };
            assert_eq!(result, Err(expected), "{scenario}");
            assert_eq!(store.chain_meta_sequence_for_test(), sequence, "{scenario}");
            assert_eq!(
                manager.export_chainstate_snapshot().expect("live snapshot"),
                before
            );
            assert_eq!(
                store.load_chain_meta_for_open().expect("metadata"),
                metadata
            );
            assert_eq!(store.coins_view().complete_snapshot(), coins);
            assert_eq!(store.maybe_basic_filter_state().expect("BASIC"), basic);
            assert_eq!(store.payload_usage_revision().expect("payload"), payload);
            drop(manager);
            drop(store);
            std::fs::remove_dir_all(path).expect("closed all owners");
        }
    }
}
