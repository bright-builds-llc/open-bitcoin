// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

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
