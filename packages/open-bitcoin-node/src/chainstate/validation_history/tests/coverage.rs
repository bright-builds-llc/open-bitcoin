// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txdb.cpp

use super::*;

fn raw_snapshot(tag: u8) -> ChainstateSnapshot {
    let genesis = block(BlockHash::default(), 0, tag);
    let mut state = Chainstate::default();
    state
        .connect_block(
            &genesis,
            1,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("independent raw snapshot");
    state.snapshot()
}

#[test]
fn phase159_validation_history_coverage_raw_seed_undo_fault_invalidates_before_coins_mutation() {
    // Arrange
    let (dir, store, mut manager) = fresh("raw-seed-failure", false);
    let accepted = block(BlockHash::default(), 0, 0x51);
    let accepted_hash = block_hash(&accepted.header);
    accept(&mut manager, &accepted, 0).expect("genuine retained acceptance");
    let raw = raw_snapshot(0x52);
    let raw_hash = raw.active_chain[0].block_hash;
    assert!(!raw.undo_by_block.is_empty());
    assert!(
        store
            .recovered_validation_history()
            .expect("before")
            .is_complete()
            .expect("complete")
    );
    store.set_basic_filter_fault(
        crate::storage::fjall_store::filters::FilterPublicationFault::BeforeUndo,
    );
    // Act
    assert!(store.seed_coins_from_snapshot(&raw).is_err());
    // Assert
    assert_eq!(
        store
            .coins_view()
            .best_block()
            .expect("raw coins already changed"),
        Some(raw_hash)
    );
    let complete_live = store
        .recovered_validation_history()
        .expect("live")
        .is_complete()
        .expect("coverage");
    assert_eq!(
        store
            .validation_provenance(accepted_hash)
            .expect("positive retained"),
        ValidationProvenance::ScriptsValid
    );
    let raw_live = store.validation_provenance(raw_hash).expect("raw identity");
    drop(manager);
    drop(store);
    let reopened = FjallNodeStore::open(&dir).expect("exclusive reopen");
    let complete_reopened = reopened
        .recovered_validation_history()
        .expect("disk")
        .is_complete()
        .expect("coverage");
    assert!(
        !complete_live && !complete_reopened,
        "raw seed coverage live={complete_live}, reopened={complete_reopened}"
    );
    assert_eq!(raw_live, ValidationProvenance::UnknownLegacy);
    assert_eq!(
        reopened
            .validation_provenance(accepted_hash)
            .expect("retained"),
        ValidationProvenance::ScriptsValid
    );
    assert_eq!(
        reopened
            .validation_provenance(raw_hash)
            .expect("raw unknown"),
        ValidationProvenance::UnknownLegacy
    );
    drop(reopened);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase159_validation_history_coverage_public_metadata_sink_cannot_preserve_unaccepted_positions()
{
    // Arrange / Act / Assert
    for wrapped in [false, true] {
        let (dir, store, mut manager) = fresh("raw-metadata-sink", false);
        let accepted = block(BlockHash::default(), 0, 0x51);
        let accepted_hash = block_hash(&accepted.header);
        accept(&mut manager, &accepted, 0).expect("genuine accepted row");
        let raw = raw_snapshot(0x52);
        let raw_hash = raw.active_chain[0].block_hash;
        assert!(
            store
                .recovered_validation_history()
                .expect("before")
                .is_complete()
                .expect("complete")
        );
        if wrapped {
            let mut sink = FjallChainstateStore::from_store(store.clone());
            FlushPersistSink::persist_chain_meta(&mut sink, &raw.active_chain)
                .expect("public forwarded sink");
        } else {
            FlushPersistSink::persist_chain_meta(&mut store.clone(), &raw.active_chain)
                .expect("public direct sink");
        }
        assert_eq!(
            store
                .load_chain_meta_for_open()
                .expect("raw metadata written")
                .0,
            raw.active_chain
        );
        let complete_live = store
            .recovered_validation_history()
            .expect("live")
            .is_complete()
            .expect("coverage");
        assert_eq!(
            store
                .validation_provenance(accepted_hash)
                .expect("positive retained"),
            ValidationProvenance::ScriptsValid
        );
        let raw_live = store.validation_provenance(raw_hash).expect("raw identity");
        drop(manager);
        drop(store);
        let reopened = FjallNodeStore::open(&dir).expect("actual reopen");
        let complete_reopened = reopened
            .recovered_validation_history()
            .expect("disk")
            .is_complete()
            .expect("coverage");
        assert!(
            !complete_live && !complete_reopened,
            "public metadata coverage live={complete_live}, reopened={complete_reopened}, wrapped={wrapped}"
        );
        assert_eq!(raw_live, ValidationProvenance::UnknownLegacy);
        assert_eq!(
            reopened
                .validation_provenance(accepted_hash)
                .expect("retained"),
            ValidationProvenance::ScriptsValid
        );
        assert_eq!(
            reopened.validation_provenance(raw_hash).expect("unknown"),
            ValidationProvenance::UnknownLegacy
        );
        drop(reopened);
        std::fs::remove_dir_all(dir).expect("cleanup");
    }
}

#[test]
fn phase159_validation_history_coverage_successful_raw_seed_retains_only_positive_acceptance() {
    // Arrange
    let (dir, store, mut manager) = fresh("raw-seed-success", false);
    let accepted = block(BlockHash::default(), 0, 0x51);
    let accepted_hash = block_hash(&accepted.header);
    accept(&mut manager, &accepted, 0).expect("genuine accepted row");
    let raw = raw_snapshot(0x52);
    let raw_hash = raw.active_chain[0].block_hash;
    // Act
    store
        .seed_coins_from_snapshot(&raw)
        .expect("successful raw seed");
    // Assert
    assert!(
        !store
            .recovered_validation_history()
            .expect("live")
            .is_complete()
            .expect("unknown coverage")
    );
    assert_eq!(
        store
            .validation_provenance(accepted_hash)
            .expect("retained positive"),
        ValidationProvenance::ScriptsValid
    );
    assert_eq!(
        store.validation_provenance(raw_hash).expect("raw unknown"),
        ValidationProvenance::UnknownLegacy
    );
    drop(manager);
    drop(store);
    let reopened = FjallNodeStore::open(&dir).expect("exclusive reopen");
    assert!(
        !reopened
            .recovered_validation_history()
            .expect("disk")
            .is_complete()
            .expect("unknown coverage")
    );
    assert_eq!(
        reopened
            .validation_provenance(accepted_hash)
            .expect("retained positive"),
        ValidationProvenance::ScriptsValid
    );
    assert_eq!(
        reopened
            .validation_provenance(raw_hash)
            .expect("raw unknown"),
        ValidationProvenance::UnknownLegacy
    );
    drop(reopened);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase159_validation_history_coverage_sparse_raw_metadata_invalidates_without_identity_parse_error()
 {
    // Arrange / Act / Assert
    for matching_hash in [false, true] {
        let (dir, store, mut manager) = fresh("sparse-raw-metadata", false);
        let accepted = block(BlockHash::default(), 0, 0x51);
        let accepted_hash = block_hash(&accepted.header);
        accept(&mut manager, &accepted, 0).expect("genuine accepted row");
        let raw = if matching_hash {
            accepted.clone()
        } else {
            block(BlockHash::default(), 0, 0x52)
        };
        let sparse = ChainPosition::new(raw.header, 50, 50, 1_700_000_000);
        let raw_hash = sparse.block_hash;
        assert!(
            crate::storage::validation_history::BlockValidationIdentity::new(
                raw_hash,
                sparse.previous_block_hash(),
                sparse.height,
            )
            .is_err(),
            "accepted identity parsing must remain strict"
        );
        FlushPersistSink::persist_chain_meta(&mut store.clone(), std::slice::from_ref(&sparse))
            .expect("legacy sparse metadata remains persistable without history authority");
        assert_eq!(
            store.load_chain_meta_for_open().expect("raw metadata").0,
            vec![sparse]
        );
        assert!(
            !store
                .recovered_validation_history()
                .expect("live coverage")
                .is_complete()
                .expect("coverage")
        );
        assert_eq!(
            store
                .validation_provenance(accepted_hash)
                .expect("retained accepted row"),
            ValidationProvenance::ScriptsValid
        );
        if !matching_hash {
            assert_eq!(
                store
                    .validation_provenance(raw_hash)
                    .expect("raw absent row"),
                ValidationProvenance::UnknownLegacy
            );
        }
        drop(manager);
        drop(store);
        let reopened = FjallNodeStore::open(&dir).expect("exclusive reopen");
        assert!(
            !reopened
                .recovered_validation_history()
                .expect("disk coverage")
                .is_complete()
                .expect("coverage")
        );
        assert_eq!(
            reopened
                .validation_provenance(accepted_hash)
                .expect("retained positive"),
            ValidationProvenance::ScriptsValid
        );
        if !matching_hash {
            assert_eq!(
                reopened
                    .validation_provenance(raw_hash)
                    .expect("raw unknown"),
                ValidationProvenance::UnknownLegacy
            );
        }
        drop(reopened);
        std::fs::remove_dir_all(dir).expect("cleanup");
    }
}
