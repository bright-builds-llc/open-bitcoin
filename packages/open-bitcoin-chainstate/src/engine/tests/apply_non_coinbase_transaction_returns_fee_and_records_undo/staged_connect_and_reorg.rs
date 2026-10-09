// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/undo.h

use super::*;

#[test]
fn commit_staged_connect_rejects_fresh_flag_misapplied_to_live_cache() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let genesis_coinbase = coinbase_transaction(0, 50);
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![genesis_coinbase.clone()],
    );
    connect_block(&mut chainstate, &genesis_block, 1);
    let genesis_txid = open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid");
    let genesis_outpoint = OutPoint {
        txid: genesis_txid,
        vout: 0,
    };
    let genesis_coin = chainstate
        .utxos()
        .remove(&genesis_outpoint)
        .expect("genesis coin");
    let child = build_block(
        chainstate.tip().expect("genesis tip").block_hash,
        1_231_006_600,
        vec![coinbase_transaction(1, 50)],
    );
    let mut staged = chainstate
        .stage_connect_block_with_current_time(
            &child,
            2,
            i64::from(child.header.time),
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
        .expect("child should stage");
    let empty_parent = CoinsCache::from_parent(MemoryCoinsView::from_coins(HashMap::new(), None));
    staged
        .overlay
        .add_coin(&empty_parent, genesis_outpoint, genesis_coin, false)
        .expect("fresh write against an empty peek parent");

    // Act
    let error = chainstate
        .commit_staged_connect(staged)
        .expect_err("fresh write over live unspent occupancy must fail");

    // Assert
    assert!(matches!(
        error,
        crate::ChainstateError::FreshFlagMisapplied { .. }
    ));
}

#[test]
fn absorb_staged_connect_overwrites_fresh_misapplied_and_installs_tip() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let genesis_coinbase = coinbase_transaction(0, 50);
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![genesis_coinbase.clone()],
    );
    connect_block(&mut chainstate, &genesis_block, 1);
    let genesis_txid = open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid");
    let genesis_outpoint = OutPoint {
        txid: genesis_txid,
        vout: 0,
    };
    let genesis_coin = chainstate
        .utxos()
        .remove(&genesis_outpoint)
        .expect("genesis coin");
    let child = build_block(
        chainstate.tip().expect("genesis tip").block_hash,
        1_231_006_600,
        vec![coinbase_transaction(1, 50)],
    );
    let mut staged = chainstate
        .stage_connect_block_with_current_time(
            &child,
            2,
            i64::from(child.header.time),
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
        .expect("child should stage");
    let empty_parent = CoinsCache::from_parent(MemoryCoinsView::from_coins(HashMap::new(), None));
    staged
        .overlay
        .add_coin(&empty_parent, genesis_outpoint.clone(), genesis_coin, false)
        .expect("fresh write against an empty peek parent");
    assert_eq!(staged.position().height, 1);

    // Act
    let position = chainstate.absorb_staged_connect(staged);

    // Assert
    assert_eq!(position.height, 1);
    assert_eq!(chainstate.tip().map(|tip| tip.height), Some(1));
    assert!(
        chainstate
            .have_coin(&genesis_outpoint)
            .expect("absorbed overwrite keeps the live coin")
    );
}

#[test]
fn absorb_and_preview_staged_reorg_install_replacement_tip() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![coinbase_transaction(0, 50)],
    );
    connect_block(&mut chainstate, &genesis_block, 1);
    let child = build_block(
        chainstate.tip().expect("genesis tip").block_hash,
        1_231_006_600,
        vec![coinbase_transaction(1, 50)],
    );
    let staged = chainstate
        .stage_reorg(
            &[],
            &[crate::AnchoredBlock {
                block: child.clone(),
                chain_work: 2,
            }],
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
        .expect("extending reorg should stage");
    assert_eq!(staged.transition().connected.len(), 1);
    let mut preview = Chainstate::from_snapshot(chainstate.snapshot());

    // Act
    preview.install_staged_reorg_preview(&staged);
    let transition = chainstate.absorb_staged_reorg(staged);

    // Assert
    assert_eq!(transition.connected.len(), 1);
    assert_eq!(preview.tip().map(|tip| tip.height), Some(1));
    assert_eq!(chainstate.tip().map(|tip| tip.height), Some(1));
    assert_eq!(preview.snapshot(), chainstate.snapshot());
}

#[test]
fn phase158_reorg_preview_then_absorb_preserves_captured_old_ancestor_and_undo() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let genesis = build_block(
        BlockHash::default(),
        1_231_006_500,
        vec![coinbase_transaction(0, 50)],
    );
    connect_block(&mut chainstate, &genesis, 1);
    let ancestor = chainstate.tip().expect("genesis").clone();
    let child = build_block(
        ancestor.block_hash,
        1_231_006_600,
        vec![coinbase_transaction(1, 50)],
    );
    connect_block(&mut chainstate, &child, 2);
    let old = chainstate.tip().expect("old child").clone();
    let genesis_txid =
        open_bitcoin_consensus::transaction_txid(&genesis.transactions[0]).expect("genesis txid");
    let spend = spend_transaction(genesis_txid, 0, 48, TransactionInput::SEQUENCE_FINAL);
    let same_block_spend = spend_transaction(
        open_bitcoin_consensus::transaction_txid(&spend).expect("spend txid"),
        0,
        47,
        TransactionInput::SEQUENCE_FINAL,
    );
    let replacement = build_block(
        ancestor.block_hash,
        1_231_006_601,
        vec![coinbase_transaction(1, 49), spend, same_block_spend],
    );
    let staged = chainstate
        .stage_reorg(
            &[child],
            &[crate::AnchoredBlock {
                block: replacement.clone(),
                chain_work: 3,
            }],
            ScriptVerifyFlags::P2SH,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
        .expect("genuine fork");
    let new = staged
        .maybe_position_at_height(1)
        .expect("replacement")
        .clone();
    let undo = staged
        .maybe_replacement_undo(new.block_hash)
        .expect("genuine undo")
        .clone();
    assert_eq!(staged.maybe_old_tip(), Some(&old));
    assert_eq!(staged.maybe_common_ancestor(), Some(&ancestor));
    assert_eq!(staged.maybe_replacement_undo(ancestor.block_hash), None);
    assert_eq!(staged.maybe_replacement_undo(old.block_hash), None);
    assert_eq!(staged.maybe_position_at_height(2), None);
    let inputs = crate::BasicFilterInputs::from_historical(
        &replacement,
        &new,
        Some(crate::HistoricalBlockUndo {
            block_hash: new.block_hash,
            undo: &undo,
        }),
    );
    assert!(inputs.is_ok());
    assert_eq!(undo.transactions.len(), 2);
    assert!(undo.transactions[0].restored_inputs[0].is_coinbase);
    assert_eq!(undo.transactions[0].restored_inputs[0].created_height, 0);
    assert!(!undo.transactions[1].restored_inputs[0].is_coinbase);
    assert_eq!(undo.transactions[1].restored_inputs[0].created_height, 1);
    assert_eq!(
        inputs
            .expect("complete historical projection")
            .spent_scripts()
            .count(),
        2
    );
    // Act
    chainstate.install_staged_reorg_preview(&staged);
    let (transition, accepted) = chainstate.absorb_staged_reorg_with_receipt(staged);
    // Assert
    assert_eq!(chainstate.tip(), Some(&new));
    assert_eq!(chainstate.undo_by_block().get(&new.block_hash), Some(&undo));
    assert_eq!(transition.disconnected, vec![old.clone()]);
    assert_eq!(
        accepted.maybe_old_endpoint(),
        Some((old.height, old.block_hash))
    );
    assert_eq!(
        accepted.maybe_new_endpoint(),
        Some((new.height, new.block_hash))
    );
    assert_eq!(
        accepted.maybe_common_ancestor_endpoint(),
        Some((ancestor.height, ancestor.block_hash))
    );
}

#[test]
fn phase158_reorg_disconnect_to_empty_receipt_has_no_new_endpoint() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let genesis = build_block(
        BlockHash::default(),
        1_231_006_500,
        vec![coinbase_transaction(0, 50)],
    );
    connect_block(&mut chainstate, &genesis, 1);
    let old = chainstate.tip().expect("genesis").clone();
    let staged = chainstate
        .stage_reorg(
            &[genesis],
            &[],
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("disconnect");
    // Act
    let (_, accepted) = chainstate.absorb_staged_reorg_with_receipt(staged);
    // Assert
    assert_eq!(
        accepted.maybe_old_endpoint(),
        Some((old.height, old.block_hash))
    );
    assert_eq!(accepted.maybe_new_endpoint(), None);
    assert_eq!(accepted.maybe_common_ancestor_endpoint(), None);
    assert_eq!(chainstate.tip(), None);
}
