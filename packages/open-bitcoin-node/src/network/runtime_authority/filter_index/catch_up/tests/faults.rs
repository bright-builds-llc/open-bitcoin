// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;

#[test]
fn phase158_preflight_missing_durable_undo_refuses_before_production_preview() {
    required_input_refusal("undo");
}

#[test]
fn phase158_preflight_missing_durable_body_refuses_before_production_preview() {
    required_input_refusal("body");
}

#[test]
fn phase158_preflight_same_shape_other_branch_undo_refuses_before_production_preview() {
    required_input_refusal("foreign");
}

#[test]
fn phase158_preflight_corrupt_undo_refuses_before_production_preview() {
    required_input_refusal("corrupt");
}

fn required_input_refusal(kind: &str) {
    // Arrange
    let fixture = ReorgFixture::new("driver-missing-undo", 3);
    let path = fixture.path().to_owned();
    let store = fixture.store.clone();
    let (disconnect, replacement) = branch(&fixture, 2);
    let network = handle(fixture);
    network
        .initialize_basic_filter_index_owner()
        .expect("owner");
    let old = network.chainstate_snapshot().expect("before");
    let old_progress = network.maybe_basic_index_progress().expect("before");
    let old_pool = network.mempool_info().expect("mempool before");
    let old_checkpoint = store.maybe_basic_filter_checkpoint().expect("checkpoint");
    let old_locks = store.load_prune_locks().expect("locks");
    let hash = open_bitcoin_core::consensus::block_hash(&disconnect[0].header);
    match kind {
        "undo" | "body" => store
            .delete_basic_input_for_test(hash, kind == "undo")
            .expect("remove actual durable mate"),
        "foreign" => {
            let undo = genuine_foreign_undo(&network, &disconnect, &replacement);
            store
                .save_undo(hash, &undo, crate::storage::PersistMode::Sync)
                .expect("different genuine block undo");
        }
        "corrupt" => {
            let key = format!(
                "undo:{}",
                hash.as_bytes()
                    .iter()
                    .map(|byte| format!("{byte:02x}"))
                    .collect::<String>()
            );
            store
                .write_raw_for_test(crate::StorageNamespace::Chainstate, &key, Vec::new())
                .expect("actual corrupted undo row");
        }
        _ => panic!("unknown fixture"),
    }
    // Act
    let result = network.reorg_to_branch(
        &disconnect,
        &replacement,
        open_bitcoin_mempool::ReorgLifecycleContext::new(
            open_bitcoin_mempool::PolicyTime::from_unix_seconds(2000),
        ),
        open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
        open_bitcoin_core::consensus::ConsensusParams {
            coinbase_maturity: 1,
            ..Default::default()
        },
    );
    // Assert
    let error = result.expect_err("preflight refuses").to_string();
    assert!(
        error.contains(if kind == "body" {
            "required body"
        } else {
            "required undo"
        }),
        "{error}"
    );
    assert_eq!(network.chainstate_snapshot().expect("after"), old);
    assert_eq!(
        network.maybe_basic_index_progress().expect("after"),
        old_progress
    );
    assert!(
        store
            .maybe_basic_filter_append_proof()
            .expect("not frozen")
            .is_some()
    );
    assert_eq!(network.mempool_info().expect("mempool after"), old_pool);
    assert_eq!(
        store.maybe_basic_filter_checkpoint().expect("checkpoint"),
        old_checkpoint
    );
    assert_eq!(store.load_prune_locks().expect("locks"), old_locks);
    drop(network);
    drop(store);
    std::fs::remove_dir_all(path).expect("cleanup");
}

fn genuine_foreign_undo(
    network: &ManagedNetworkHandle<FjallChainstateStore, FjallCoinsView>,
    disconnect: &[open_bitcoin_core::primitives::Block],
    replacement: &[open_bitcoin_core::chainstate::AnchoredBlock],
) -> open_bitcoin_core::chainstate::BlockUndo {
    use open_bitcoin_core::consensus::{
        block_hash, block_merkle_root, check_block_header, transaction_txid,
    };
    let mut foreign = replacement.to_vec();
    foreign[0].block.transactions[0].outputs[0].script_pubkey =
        open_bitcoin_core::primitives::ScriptBuf::from_bytes(vec![0x52])
            .expect("different true script");
    for height in 0..foreign.len() {
        if height != 0 {
            let parent_hash = block_hash(&foreign[height - 1].block.header);
            let funding_txid = transaction_txid(&foreign[height - 1].block.transactions[0])
                .expect("foreign funding");
            foreign[height].block.header.previous_block_hash = parent_hash;
            foreign[height].block.transactions[1].inputs[0]
                .previous_output
                .txid = funding_txid;
            let same_block_txid = transaction_txid(&foreign[height].block.transactions[1])
                .expect("genuine same-block spend");
            foreign[height].block.transactions[2].inputs[0]
                .previous_output
                .txid = same_block_txid;
        }
        let block = &mut foreign[height].block;
        block.header.merkle_root = block_merkle_root(&block.transactions)
            .expect("body commitment")
            .0;
        block.header.nonce = (0..=u32::MAX)
            .find(|nonce| {
                block.header.nonce = *nonce;
                check_block_header(&block.header).is_ok()
            })
            .expect("easy proof of work");
    }
    let staged = network
        .read(|network| {
            network.chainstate().chainstate().stage_reorg(
                disconnect,
                &foreign,
                open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
                open_bitcoin_core::consensus::ConsensusParams {
                    coinbase_maturity: 1,
                    ..Default::default()
                },
            )
        })
        .expect("serialized genuine staging")
        .expect("validated foreign branch");
    staged
        .maybe_replacement_undo(block_hash(&foreign[1].block.header))
        .expect("same-height foreign undo")
        .clone()
}

#[test]
fn phase158_preflight_after_accept_body_error_pauses_then_recovered_driver_rebinds() {
    // Arrange
    let fixture = ReorgFixture::new("driver-body-recovery", 3);
    let path = fixture.path().to_owned();
    let store = fixture.store.clone();
    let old_tip = fixture.records[2].identity().block_hash();
    let (disconnect, replacement) = branch(&fixture, 2);
    let network = handle(fixture);
    network
        .initialize_basic_filter_index_owner()
        .expect("owner");
    reorg(&network, &disconnect, &replacement).expect("accepted");
    let first = network
        .drive_basic_filter_index_turn_with_budget(one_block_budget())
        .expect("first replacement ahead of coins");
    store.set_basic_filter_fault(
        crate::storage::fjall_store::filters::FilterPublicationFault::BeforeBody,
    );
    // Act
    assert!(
        store
            .save_block(&replacement[1].block, crate::storage::PersistMode::Sync)
            .is_err()
    );
    network
        .note_basic_index_failure(
            crate::chainstate::filter_index::AcceptedBasicIndexFailure::Persistence,
        )
        .expect("production host failure projection");
    assert!(network.drive_basic_filter_index_turn().is_err());
    let paused = network
        .maybe_basic_index_progress()
        .expect("paused")
        .expect("owner");
    assert_eq!(
        paused.maybe_processed_endpoint(),
        first
            .maybe_progress
            .expect("progress")
            .maybe_processed_endpoint()
    );
    assert_eq!(
        paused.state(),
        BasicIndexState::Paused(
            open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexPause::Persistence
        )
    );
    drop(network);
    drop(store);
    let reopened = crate::storage::FjallNodeStore::open(&path).expect("actual closed reopen");
    let runtime = crate::sync::DurableSyncRuntime::open_configured(
        reopened,
        crate::sync::SyncRuntimeConfig {
            network: crate::sync::SyncNetwork::Regtest,
            ..Default::default()
        },
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("production startup drives recovered prefix");
    // Assert
    let progress = runtime
        .network_handle()
        .maybe_basic_index_progress()
        .expect("progress")
        .expect("owner");
    assert_eq!(progress.accepted_target().block_hash(), old_tip);
    assert_eq!(
        progress
            .maybe_processed_endpoint()
            .expect("processed")
            .block_hash(),
        old_tip
    );
    drop(runtime);
    std::fs::remove_dir_all(path).expect("cleanup");
}
