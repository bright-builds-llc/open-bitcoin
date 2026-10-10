// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use crate::chainstate::ReorgFixture;
use crate::storage::fjall_store::validation_history::HistoryPublicationFault;
use crate::storage::validation_history::ValidationProvenance;
use crate::{FjallChainstateStore, FjallCoinsView, FjallNodeStore, PersistMode};
use open_bitcoin_core::{
    chainstate::AnchoredBlock,
    consensus::{ConsensusParams, ScriptVerifyFlags},
};
use open_bitcoin_core::{
    chainstate::{CoinsCache, FlushMode, FlushPolicyTime},
    consensus::{block_hash, block_merkle_root, check_block_header},
    primitives::{
        Amount, BlockHeader, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
        TransactionOutput,
    },
};

mod coverage;
mod fixtures;
mod network;
pub(crate) use fixtures::block;
use fixtures::{accept, fresh};

#[test]
fn phase159_validation_history_accept_ordinary_fault_retains_live_bounded_fact_and_refuses_absorb()
{
    // Arrange / Act / Assert
    for (name, fault, survives) in [
        ("before", HistoryPublicationFault::BeforeCommit, false),
        ("after", HistoryPublicationFault::AfterCommit, true),
    ] {
        let (dir, store, mut manager) = fresh(name, false);
        let genesis = block(BlockHash::default(), 0, 0x51);
        let hash = block_hash(&genesis.header);
        store.set_validation_history_fault(fault);
        assert!(accept(&mut manager, &genesis, 0).is_err());
        assert_eq!(
            manager.chainstate().tip().expect("absorbed").block_hash,
            hash
        );
        assert_eq!(
            manager.validation_provenance(hash).expect("live accepted"),
            ValidationProvenance::ScriptsValid
        );
        let child = block(hash, 1, 0x51);
        assert!(accept(&mut manager, &child, 1).is_err());
        assert_eq!(manager.chainstate().tip().expect("unchanged").height, 0);
        assert_eq!(
            manager
                .maybe_pending_validation
                .as_ref()
                .expect("one batch")
                .identities()
                .len(),
            1
        );
        drop(manager);
        drop(store);
        let reopened = FjallNodeStore::open(&dir).expect("all handles dropped");
        assert_eq!(
            reopened
                .validation_provenance(hash)
                .expect("actual disk fact"),
            if survives {
                ValidationProvenance::ScriptsValid
            } else {
                ValidationProvenance::NeverConnected
            }
        );
        drop(reopened);
        std::fs::remove_dir_all(dir).expect("cleanup");
    }
}

#[test]
fn phase159_validation_history_accept_before_coins_persistence_failure_is_durable() {
    // Arrange
    let (dir, store, mut manager) = fresh("coins-failure", true);
    let genesis = block(BlockHash::default(), 0, 0x51);
    let hash = block_hash(&genesis.header);
    store.set_basic_filter_fault(
        crate::storage::fjall_store::filters::FilterPublicationFault::BeforeChainMeta,
    );
    // Act
    let result = accept(&mut manager, &genesis, 0);
    // Assert
    assert!(result.is_err());
    assert!(!manager.has_pending_validation_history());
    assert_eq!(
        manager.validation_provenance(hash).expect("accepted"),
        ValidationProvenance::ScriptsValid
    );
    drop(manager);
    drop(store);
    let reopened = FjallNodeStore::open(&dir).expect("reopen independent ledger");
    assert_eq!(
        reopened
            .validation_provenance(hash)
            .expect("durable accepted"),
        ValidationProvenance::ScriptsValid
    );
    drop(reopened);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase159_validation_history_accept_fresh_genesis_forced_flush_keeps_complete_coverage() {
    // Arrange
    let (dir, store, mut manager) = fresh("coverage", false);
    let genesis = block(BlockHash::default(), 0, 0x51);
    let hash = block_hash(&genesis.header);
    accept(&mut manager, &genesis, 0).expect("genuine genesis absorption");
    store.save_block(&genesis, PersistMode::Sync).expect("body");
    // Act
    let flushed = manager
        .flush_with_mode(
            FlushMode::Always,
            FlushPolicyTime::from_unix_seconds(10),
            u64::MAX,
        )
        .expect("actual coins flush");
    // Assert
    assert!(flushed.wrote_coins);
    assert!(
        store
            .recovered_validation_history()
            .expect("recovery")
            .is_complete()
            .expect("coverage survives trusted metadata")
    );
    assert_eq!(
        store.validation_provenance(hash).expect("genuine genesis"),
        ValidationProvenance::ScriptsValid
    );
    drop(manager);
    drop(store);
    let reopened = FjallNodeStore::open(&dir).expect("reopen");
    assert_eq!(
        reopened
            .validation_provenance(hash)
            .expect("retained genesis"),
        ValidationProvenance::ScriptsValid
    );
    assert!(
        reopened
            .recovered_validation_history()
            .expect("recovery")
            .is_complete()
            .expect("complete")
    );
    let runtime = crate::DurableSyncRuntime::open_configured(
        reopened,
        crate::SyncRuntimeConfig::default(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("configured startup from genuine genesis");
    assert!(
        runtime
            .store()
            .load_basic_filter_record(hash)
            .expect("startup owner turn")
            .is_some()
    );
    drop(runtime);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase159_validation_history_accept_reorg_retains_every_replacement_on_publication_failure() {
    // Arrange
    let mut fixture = ReorgFixture::new("history-accept-failure", 3);
    let path = fixture.path().to_owned();
    let (staged, _) = fixture.stage(3);
    let replacement: Vec<_> = staged
        .transition()
        .connected
        .iter()
        .map(|position| AnchoredBlock {
            block: fixture
                .store
                .load_block(position.block_hash)
                .expect("body")
                .expect("present"),
            chain_work: position.chain_work,
        })
        .collect();
    let disconnect: Vec<_> = fixture.manager.chainstate().active_chain()[1..]
        .iter()
        .rev()
        .map(|position| {
            fixture
                .store
                .load_block(position.block_hash)
                .expect("body")
                .expect("present")
        })
        .collect();
    fixture
        .store
        .set_validation_history_fault(HistoryPublicationFault::BeforeCommit);
    // Act
    let result = fixture.manager.reorg(
        &disconnect,
        &replacement,
        ScriptVerifyFlags::P2SH,
        ConsensusParams {
            coinbase_maturity: 1,
            ..Default::default()
        },
    );
    // Assert
    assert!(result.is_err());
    for position in &staged.transition().connected {
        assert_eq!(
            fixture
                .manager
                .validation_provenance(position.block_hash)
                .expect("live fact"),
            ValidationProvenance::ScriptsValid
        );
    }
    assert!(fixture.manager.has_pending_validation_history());
    let before = fixture.manager.chainstate().tip().cloned();
    assert!(
        fixture
            .manager
            .reorg(
                &[],
                &[],
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default()
            )
            .is_err()
    );
    assert_eq!(fixture.manager.chainstate().tip(), before.as_ref());
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

fn fork(fixture: &ReorgFixture, count: usize) -> (Vec<Block>, Vec<AnchoredBlock>, Vec<BlockHash>) {
    let (staged, _) = fixture.stage(count);
    let replacement: Vec<_> = staged
        .transition()
        .connected
        .iter()
        .map(|position| AnchoredBlock {
            block: fixture
                .store
                .load_block(position.block_hash)
                .expect("body")
                .expect("present"),
            chain_work: position.chain_work,
        })
        .collect();
    let disconnect = fixture.manager.chainstate().active_chain()[1..]
        .iter()
        .rev()
        .map(|position| {
            fixture
                .store
                .load_block(position.block_hash)
                .expect("body")
                .expect("present")
        })
        .collect();
    let hashes = staged
        .transition()
        .connected
        .iter()
        .map(|p| p.block_hash)
        .collect();
    (disconnect, replacement, hashes)
}

#[test]
fn phase159_validation_history_accept_equal_and_longer_reorg_retains_displaced_and_every_new_identity()
 {
    // Arrange / Act / Assert
    for count in [2, 3] {
        let mut fixture = ReorgFixture::new("history-replacement", 3);
        let path = fixture.path().to_owned();
        let old: Vec<_> = fixture.manager.chainstate().active_chain()[1..]
            .iter()
            .map(|p| p.block_hash)
            .collect();
        let (disconnect, replacement, hashes) = fork(&fixture, count);
        fixture
            .manager
            .reorg(
                &disconnect,
                &replacement,
                ScriptVerifyFlags::P2SH,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..Default::default()
                },
            )
            .expect("actual complete acceptance");
        for hash in old.iter().chain(&hashes) {
            assert_eq!(
                fixture
                    .manager
                    .validation_provenance(*hash)
                    .expect("retained live"),
                ValidationProvenance::ScriptsValid
            );
        }
        assert!(!fixture.manager.has_pending_validation_history());
        drop(fixture);
        let reopened =
            FjallNodeStore::open(&path).expect("exclusive reopen even with old durable coins");
        for hash in old.iter().chain(&hashes) {
            assert_eq!(
                reopened
                    .validation_provenance(*hash)
                    .expect("retained disk acceptance"),
                ValidationProvenance::ScriptsValid
            );
        }
        drop(reopened);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
}

#[test]
fn phase159_validation_history_accept_reorg_basic_failure_cannot_erase_absorbed_history() {
    // Arrange
    let mut fixture = ReorgFixture::new("history-basic-failure", 3);
    let path = fixture.path().to_owned();
    let (disconnect, replacement, hashes) = fork(&fixture, 3);
    fixture.store.set_basic_filter_fault(
        crate::storage::fjall_store::filters::FilterPublicationFault::BeforeProtection,
    );
    // Act
    let result = fixture.manager.reorg(
        &disconnect,
        &replacement,
        ScriptVerifyFlags::P2SH,
        ConsensusParams {
            coinbase_maturity: 1,
            ..Default::default()
        },
    );
    // Assert
    assert!(result.is_err());
    assert_eq!(
        fixture
            .manager
            .chainstate()
            .tip()
            .expect("absorbed")
            .block_hash,
        *hashes.last().expect("replacement")
    );
    assert!(!fixture.manager.has_pending_validation_history());
    assert!(fixture.store.maybe_basic_filter_append_proof().is_err());
    for hash in &hashes {
        assert_eq!(
            fixture.manager.validation_provenance(*hash).expect("live"),
            ValidationProvenance::ScriptsValid
        );
    }
    drop(fixture);
    let reopened = FjallNodeStore::open(&path).expect("history independent recovery");
    for hash in &hashes {
        assert_eq!(
            reopened.validation_provenance(*hash).expect("disk"),
            ValidationProvenance::ScriptsValid
        );
    }
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase159_validation_history_accept_preview_and_rejected_stage_cannot_mint() {
    // Arrange
    let mut fixture = ReorgFixture::new("history-preview", 3);
    let path = fixture.path().to_owned();
    let (disconnect, replacement, hashes) = fork(&fixture, 2);
    let mut invalid = replacement.clone();
    invalid[0].block.header.merkle_root = Default::default();
    // Act
    assert!(
        fixture
            .manager
            .prepare_reorg(
                &disconnect,
                &invalid,
                ScriptVerifyFlags::P2SH,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..Default::default()
                }
            )
            .is_err()
    );
    let prepared = fixture
        .manager
        .prepare_reorg(
            &disconnect,
            &replacement,
            ScriptVerifyFlags::P2SH,
            ConsensusParams {
                coinbase_maturity: 1,
                ..Default::default()
            },
        )
        .expect("valid stage");
    fixture
        .manager
        .install_prepared_reorg_preview(&prepared)
        .expect("preview only");
    // Assert
    for hash in hashes {
        assert!(
            fixture
                .store
                .maybe_validation_history_record(hash)
                .expect("ledger")
                .is_none()
        );
        assert_eq!(
            fixture
                .manager
                .validation_provenance(hash)
                .expect("legacy uncertainty"),
            ValidationProvenance::UnknownLegacy
        );
    }
    assert!(!fixture.manager.has_pending_validation_history());
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase159_validation_history_accept_exact_batch_bound_admitted_one_over_refused_before_absorb() {
    // Arrange
    let (dir, store, mut manager) = fresh("batch-bound", false);
    let genesis = block(BlockHash::default(), 0, 0x51);
    accept(&mut manager, &genesis, 0).expect("genuine genesis");
    let mut parent = block_hash(&genesis.header);
    let mut branch = Vec::new();
    for height in 1..=129 {
        let next = block(parent, height, 0x51);
        parent = block_hash(&next.header);
        branch.push(AnchoredBlock {
            block: next,
            chain_work: u128::from(height) + 1,
        });
    }
    // Act
    let oversized = manager.reorg(
        &[],
        &branch,
        ScriptVerifyFlags::P2SH,
        ConsensusParams::default(),
    );
    // Assert
    assert!(oversized.is_err());
    assert_eq!(manager.chainstate().tip().expect("unchanged").height, 0);
    assert!(!manager.has_pending_validation_history());
    manager
        .reorg(
            &[],
            &branch[..128],
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("exact 128 accepted");
    for anchored in &branch[..128] {
        assert_eq!(
            store
                .validation_provenance(block_hash(&anchored.block.header))
                .expect("every position"),
            ValidationProvenance::ScriptsValid
        );
    }
    assert_eq!(
        store
            .validation_provenance(parent)
            .expect("one over never absorbed"),
        ValidationProvenance::NeverConnected
    );
    drop(manager);
    drop(store);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase159_validation_history_accept_missing_filter_and_basic_disable_preserve_retained_acceptance()
 {
    // Arrange
    let fixture = ReorgFixture::new("history-retained", 3);
    let path = fixture.path().to_owned();
    let hash = fixture.records[1].identity().block_hash();
    assert!(
        fixture
            .store
            .load_basic_filter_record(hash)
            .expect("indexed accepted row")
            .is_some()
    );
    // Act
    fixture
        .store
        .disable_basic_filter_index()
        .expect("disable before removing active projection row");
    fixture
        .store
        .remove_validation_filter_for_test(hash)
        .expect("remove filter only");
    fixture
        .store
        .delete_basic_input_for_test(hash, false)
        .expect("remove body only");
    fixture
        .store
        .delete_basic_input_for_test(hash, true)
        .expect("remove undo only");
    // Assert
    assert_eq!(
        fixture
            .manager
            .validation_provenance(hash)
            .expect("accepted independent of payloads"),
        ValidationProvenance::ScriptsValid
    );
    drop(fixture);
    let reopened = FjallNodeStore::open(&path).expect("real reopen");
    assert!(
        reopened
            .load_basic_filter_record(hash)
            .expect("missing row")
            .is_none()
    );
    assert!(reopened.load_block(hash).expect("missing body").is_none());
    assert!(reopened.load_undo(hash).expect("missing undo").is_none());
    assert_eq!(
        reopened
            .validation_provenance(hash)
            .expect("retained accepted fact"),
        ValidationProvenance::ScriptsValid
    );
    drop(reopened);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase159_validation_history_accept_zero_replacement_disconnect_cannot_erase_history() {
    // Arrange
    let (dir, store, mut manager) = fresh("disconnect", false);
    let genesis = block(BlockHash::default(), 0, 0x51);
    let hash = block_hash(&genesis.header);
    accept(&mut manager, &genesis, 0).expect("genuine acceptance");
    // Act
    manager
        .reorg(
            &[genesis],
            &[],
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("genuine disconnect without replacement");
    // Assert
    assert!(manager.chainstate().tip().is_none());
    assert_eq!(
        manager.validation_provenance(hash).expect("retained"),
        ValidationProvenance::ScriptsValid
    );
    assert!(!manager.has_pending_validation_history());
    drop(manager);
    drop(store);
    std::fs::remove_dir_all(dir).expect("cleanup");
}
