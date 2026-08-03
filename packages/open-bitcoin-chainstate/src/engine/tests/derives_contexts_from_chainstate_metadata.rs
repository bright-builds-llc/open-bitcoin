// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use super::*;
use crate::{ChainstateError, ChainstateSnapshot};

mod difficulty;

#[test]
fn derives_contexts_from_chainstate_metadata() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let genesis_coinbase = coinbase_transaction(0, 50);
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![genesis_coinbase.clone()],
    );
    let genesis_position = connect_block(&mut chainstate, &genesis_block, 1);
    let spend = spend_transaction(
        Txid::from_byte_array(
            open_bitcoin_consensus::transaction_txid(&genesis_coinbase)
                .expect("txid")
                .to_byte_array(),
        ),
        0,
        40,
        1,
    );
    let block = build_block(
        genesis_position.block_hash,
        1_231_006_600,
        vec![coinbase_transaction(1, 50), spend],
    );

    // Act
    let next_position = connect_block(&mut chainstate, &block, 2);

    // Assert
    assert_eq!(next_position.height, 1);
    let spendable = chainstate
        .utxos()
        .values()
        .find(|coin| !coin.is_coinbase)
        .expect("expected transaction output to be added");
    assert_eq!(spendable.created_height, 1);
    assert_eq!(
        spendable.created_median_time_past,
        genesis_position.median_time_past
    );
}

#[test]
fn connect_and_disconnect_round_trip_utxos_and_tip() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let genesis_coinbase = coinbase_transaction(0, 50);
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![genesis_coinbase.clone()],
    );
    let genesis_position = connect_block(&mut chainstate, &genesis_block, 1);
    let spend = spend_transaction(
        open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid"),
        0,
        40,
        TransactionInput::SEQUENCE_FINAL,
    );
    let block = build_block(
        genesis_position.block_hash,
        1_231_006_600,
        vec![coinbase_transaction(1, 50), spend],
    );
    let block_txids = block
        .transactions
        .iter()
        .map(|transaction| open_bitcoin_consensus::transaction_txid(transaction).expect("txid"))
        .collect::<Vec<_>>();
    let connected_position = connect_block(&mut chainstate, &block, 2);
    let connected_snapshot = chainstate.snapshot();
    let confirmed_txid_counts = connected_snapshot
        .maybe_confirmed_txid_counts
        .as_ref()
        .expect("fresh chainstate should retain exact confirmed transaction identity");
    assert!(
        block_txids
            .iter()
            .all(|txid| confirmed_txid_counts.get(txid) == Some(&1))
    );

    // Act
    let disconnected = chainstate
        .disconnect_tip(&block)
        .expect("block should disconnect cleanly");

    // Assert
    assert_eq!(disconnected, connected_position);
    assert_active_tip(&chainstate, &genesis_position);
    assert_eq!(chainstate.utxos().len(), 1);
    let disconnected_snapshot = chainstate.snapshot();
    let confirmed_txid_counts = disconnected_snapshot
        .maybe_confirmed_txid_counts
        .as_ref()
        .expect("fresh chainstate should retain exact confirmed transaction identity");
    assert!(
        block_txids
            .iter()
            .all(|txid| !confirmed_txid_counts.contains_key(txid))
    );
}

#[test]
fn legacy_snapshot_connect_and_disconnect_preserve_unknown_confirmed_identity() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![coinbase_transaction(0, 50)],
    );
    let genesis_position = connect_block(&mut chainstate, &genesis_block, 1);
    let mut legacy_snapshot = chainstate.snapshot();
    legacy_snapshot.maybe_confirmed_txid_counts = None;
    let mut legacy_chainstate = Chainstate::from_snapshot(legacy_snapshot);
    let block = build_block(
        genesis_position.block_hash,
        1_231_006_600,
        vec![coinbase_transaction(1, 50)],
    );

    // Act
    connect_block(&mut legacy_chainstate, &block, 2);
    legacy_chainstate
        .disconnect_tip(&block)
        .expect("block should disconnect cleanly");

    // Assert
    assert!(
        legacy_chainstate
            .snapshot()
            .maybe_confirmed_txid_counts
            .is_none()
    );
}

#[test]
fn connect_rejects_confirmed_txid_count_overflow_without_mutation() {
    // Arrange
    let coinbase = coinbase_transaction(0, 50);
    let txid = open_bitcoin_consensus::transaction_txid(&coinbase).expect("txid");
    let block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![coinbase],
    );
    let mut snapshot = ChainstateSnapshot::new(Vec::new(), HashMap::new(), HashMap::new());
    snapshot.maybe_confirmed_txid_counts = Some(HashMap::from([(txid, u32::MAX)]));
    let expected = snapshot.clone();
    let mut chainstate = Chainstate::from_snapshot(snapshot);

    // Act
    let error = chainstate
        .connect_block(
            &block,
            1,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect_err("overflowing confirmation count must fail closed");

    // Assert
    assert!(matches!(
        error,
        ChainstateError::Serialization {
            context: "confirmed transaction count",
            ref reason,
        } if reason.contains("occurrence count overflow")
    ));
    assert_eq!(chainstate.snapshot(), expected);
}

#[test]
fn disconnect_rejects_missing_confirmed_txid_count_without_mutation() {
    // Arrange
    let block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![coinbase_transaction(0, 50)],
    );
    let mut chainstate = Chainstate::new();
    connect_block(&mut chainstate, &block, 1);
    let mut snapshot = chainstate.snapshot();
    snapshot
        .maybe_confirmed_txid_counts
        .as_mut()
        .expect("exact confirmation counts")
        .clear();
    let expected = snapshot.clone();
    let mut corrupted = Chainstate::from_snapshot(snapshot);

    // Act
    let error = corrupted
        .disconnect_tip(&block)
        .expect_err("missing confirmation count must fail closed");

    // Assert
    assert!(matches!(
        error,
        ChainstateError::Serialization {
            context: "confirmed transaction count",
            ref reason,
        } if reason.contains("missing active-chain occurrence")
    ));
    assert_eq!(corrupted.snapshot(), expected);
}

#[test]
fn duplicate_txid_counts_remain_exact_across_disconnect_and_snapshot_restore() {
    // Arrange
    let duplicate = coinbase_transaction(0, 50);
    let duplicate_txid = open_bitcoin_consensus::transaction_txid(&duplicate).expect("txid");
    let genesis = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![duplicate.clone()],
    );
    let middle = build_block(
        open_bitcoin_consensus::block_hash(&genesis.header),
        1_231_006_600,
        vec![
            coinbase_transaction(1, 50),
            spend_transaction(duplicate_txid, 0, 40, TransactionInput::SEQUENCE_FINAL),
        ],
    );
    let later_duplicate = build_block(
        open_bitcoin_consensus::block_hash(&middle.header),
        1_231_006_700,
        vec![duplicate],
    );
    let params = ConsensusParams {
        coinbase_maturity: 1,
        enforce_bip34_height_in_coinbase: false,
        ..ConsensusParams::default()
    };
    let flags = ScriptVerifyFlags::P2SH
        | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
        | ScriptVerifyFlags::CHECKSEQUENCEVERIFY;
    let mut chainstate = Chainstate::new();
    chainstate
        .connect_block(&genesis, 1, flags, params)
        .expect("connect first occurrence");
    chainstate
        .connect_block(&middle, 2, flags, params)
        .expect("spend first occurrence outputs");
    chainstate
        .connect_block(&later_duplicate, 3, flags, params)
        .expect("connect allowed later occurrence");

    // Act
    chainstate
        .disconnect_tip(&later_duplicate)
        .expect("disconnect later duplicate");
    let mut restored = Chainstate::from_snapshot(chainstate.snapshot());

    // Assert
    let counts = restored
        .snapshot()
        .maybe_confirmed_txid_counts
        .expect("exact confirmation counts");
    assert_eq!(counts.get(&duplicate_txid), Some(&1));
    restored.disconnect_tip(&middle).expect("disconnect middle");
    assert_eq!(
        restored
            .snapshot()
            .maybe_confirmed_txid_counts
            .expect("exact confirmation counts")
            .get(&duplicate_txid),
        Some(&1)
    );
    restored
        .disconnect_tip(&genesis)
        .expect("disconnect genesis");
    assert!(
        !restored
            .snapshot()
            .maybe_confirmed_txid_counts
            .expect("exact confirmation counts")
            .contains_key(&duplicate_txid)
    );
}

#[test]
fn reorg_prefers_heavier_branch_and_preserves_expected_utxos() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let genesis_coinbase = coinbase_transaction(0, 50);
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![genesis_coinbase.clone()],
    );
    let genesis_position = connect_block(&mut chainstate, &genesis_block, 1);

    let branch_a_coinbase = coinbase_transaction(1, 50);
    let branch_a = build_block(
        genesis_position.block_hash,
        1_231_006_600,
        vec![branch_a_coinbase.clone()],
    );
    let branch_a_position = connect_block(&mut chainstate, &branch_a, 2);

    let branch_b_spend = spend_transaction(
        open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid"),
        0,
        30,
        TransactionInput::SEQUENCE_FINAL,
    );
    let branch_b = build_block(
        genesis_position.block_hash,
        1_231_006_650,
        vec![coinbase_transaction(1, 50), branch_b_spend],
    );
    let branch_b_tip = ChainPosition::new(branch_b.header.clone(), 1, 3, 1_231_006_650);
    assert!(prefer_candidate_tip(&branch_a_position, &branch_b_tip));

    // Act
    let transition = chainstate
        .reorg(
            std::slice::from_ref(&branch_a),
            &[AnchoredBlock {
                block: branch_b.clone(),
                chain_work: 3,
            }],
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
        .expect("reorg should succeed");

    // Assert
    assert_eq!(transition.disconnected, vec![branch_a_position]);
    assert_eq!(transition.connected.len(), 1);
    assert_eq!(chainstate.tip(), Some(&transition.connected[0]));
    assert_eq!(chainstate.utxos().len(), 2);
}

#[test]
fn connect_block_rejects_premature_coinbase_spend() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let genesis_coinbase = coinbase_transaction(0, 50);
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![genesis_coinbase.clone()],
    );
    let genesis_position = connect_block(&mut chainstate, &genesis_block, 1);
    let premature_spend = spend_transaction(
        open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid"),
        0,
        40,
        TransactionInput::SEQUENCE_FINAL,
    );
    let block = build_block(
        genesis_position.block_hash,
        1_231_006_600,
        vec![coinbase_transaction(1, 50), premature_spend],
    );

    // Act
    let error = chainstate
        .connect_block(
            &block,
            2,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams::default(),
        )
        .expect_err("premature coinbase spend must fail");

    // Assert
    assert!(matches!(
        error,
        crate::ChainstateError::TransactionValidation { .. }
    ));
}

#[test]
fn connect_block_rejects_missing_prevouts_from_chainstate() {
    let mut chainstate = Chainstate::new();
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![coinbase_transaction(0, 50)],
    );
    connect_block(&mut chainstate, &genesis_block, 1);

    let missing_prevout = spend_transaction(
        Txid::from_byte_array([4_u8; 32]),
        0,
        40,
        TransactionInput::SEQUENCE_FINAL,
    );
    let block = build_block(
        open_bitcoin_consensus::block_hash(&genesis_block.header),
        1_231_006_600,
        vec![coinbase_transaction(1, 50), missing_prevout],
    );

    let error = chainstate
        .connect_block(
            &block,
            2,
            ScriptVerifyFlags::P2SH,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
        .expect_err("missing prevout must fail before mutation");

    assert!(matches!(error, crate::ChainstateError::MissingCoin { .. }));
}

#[test]
fn connect_block_uses_explicit_current_time_for_future_time_rejection() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![coinbase_transaction(0, 50)],
    );
    connect_block(&mut chainstate, &genesis_block, 1);
    let future_block = build_block(
        open_bitcoin_consensus::block_hash(&genesis_block.header),
        1_231_016_500,
        vec![coinbase_transaction(1, 50)],
    );

    // Act
    let error = chainstate
        .connect_block_with_current_time(
            &future_block,
            2,
            i64::from(future_block.header.time) - 7_201,
            ScriptVerifyFlags::P2SH,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
        .expect_err("future block must respect the caller-provided time");

    // Assert
    assert!(matches!(
        error,
        crate::ChainstateError::BlockValidation { source }
            if source.reject_reason == "time-too-new"
    ));
}
