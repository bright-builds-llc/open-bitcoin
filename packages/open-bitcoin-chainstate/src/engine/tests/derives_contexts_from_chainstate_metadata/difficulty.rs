// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use super::*;

#[test]
fn connect_block_rejects_wrong_bits_at_retarget_boundary() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let consensus_params = ConsensusParams {
        coinbase_maturity: 1,
        allow_min_difficulty_blocks: false,
        no_pow_retargeting: false,
        pow_target_spacing_seconds: 10,
        pow_target_timespan_seconds: 20,
        ..ConsensusParams::default()
    };
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        100,
        vec![coinbase_transaction(0, 50)],
    );
    connect_block(&mut chainstate, &genesis_block, 1);
    let height_one_block = build_block(
        open_bitcoin_consensus::block_hash(&genesis_block.header),
        110,
        vec![coinbase_transaction(1, 50)],
    );
    let height_one_position = chainstate
        .connect_block(
            &height_one_block,
            2,
            ScriptVerifyFlags::P2SH,
            consensus_params,
        )
        .expect("height-one block should connect");
    let wrong_bits_block = build_block(
        height_one_position.block_hash,
        120,
        vec![coinbase_transaction(2, 50)],
    );

    // Act
    let error = chainstate
        .connect_block(
            &wrong_bits_block,
            3,
            ScriptVerifyFlags::P2SH,
            consensus_params,
        )
        .expect_err("stale retarget-boundary bits must fail");

    // Assert
    assert!(matches!(
        error,
        crate::ChainstateError::BlockValidation { source }
            if source.reject_reason == "bad-diffbits"
                && source.debug_message.as_deref() == Some("incorrect proof of work")
    ));
}

#[test]
fn connect_block_recovers_last_non_special_target_after_special_min_difficulty_block() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let consensus_params = ConsensusParams {
        coinbase_maturity: 1,
        allow_min_difficulty_blocks: true,
        no_pow_retargeting: false,
        pow_target_spacing_seconds: 10,
        pow_target_timespan_seconds: 40,
        ..ConsensusParams::default()
    };
    let recovered_bits = 0x205f_ffff;
    let genesis_block = build_block_with_bits(
        BlockHash::from_byte_array([0_u8; 32]),
        100,
        consensus_params.pow_limit_bits,
        vec![coinbase_transaction(0, 50)],
    );
    let genesis_position = chainstate
        .connect_block(&genesis_block, 1, ScriptVerifyFlags::P2SH, consensus_params)
        .expect("genesis block should connect");
    let on_time_block = build_block_with_bits(
        genesis_position.block_hash,
        110,
        consensus_params.pow_limit_bits,
        vec![coinbase_transaction(1, 50)],
    );
    let on_time_position = chainstate
        .connect_block(&on_time_block, 2, ScriptVerifyFlags::P2SH, consensus_params)
        .expect("non-special block should connect");
    let second_on_time_block = build_block_with_bits(
        on_time_position.block_hash,
        120,
        consensus_params.pow_limit_bits,
        vec![coinbase_transaction(2, 50)],
    );
    let second_on_time_position = chainstate
        .connect_block(
            &second_on_time_block,
            3,
            ScriptVerifyFlags::P2SH,
            consensus_params,
        )
        .expect("second non-special block should connect");
    let third_on_time_block = build_block_with_bits(
        second_on_time_position.block_hash,
        130,
        consensus_params.pow_limit_bits,
        vec![coinbase_transaction(3, 50)],
    );
    let third_on_time_position = chainstate
        .connect_block(
            &third_on_time_block,
            4,
            ScriptVerifyFlags::P2SH,
            consensus_params,
        )
        .expect("third non-special block should connect");
    let boundary_block = build_block_with_bits(
        third_on_time_position.block_hash,
        140,
        recovered_bits,
        vec![coinbase_transaction(4, 50)],
    );
    let boundary_position = chainstate
        .connect_block(
            &boundary_block,
            5,
            ScriptVerifyFlags::P2SH,
            consensus_params,
        )
        .expect("boundary block should connect");
    let special_block = build_block_with_bits(
        boundary_position.block_hash,
        161,
        consensus_params.pow_limit_bits,
        vec![coinbase_transaction(5, 50)],
    );
    let special_position = chainstate
        .connect_block(&special_block, 6, ScriptVerifyFlags::P2SH, consensus_params)
        .expect("late special block should connect");
    let wrong_bits_block = build_block_with_bits(
        special_position.block_hash,
        170,
        consensus_params.pow_limit_bits,
        vec![coinbase_transaction(6, 50)],
    );
    let recovered_bits_block = build_block_with_bits(
        special_position.block_hash,
        170,
        recovered_bits,
        vec![coinbase_transaction(6, 50)],
    );

    // Act
    let error = chainstate
        .connect_block(
            &wrong_bits_block,
            7,
            ScriptVerifyFlags::P2SH,
            consensus_params,
        )
        .expect_err("previous special bits must be rejected after recovery");
    let recovered_position = chainstate
        .connect_block(
            &recovered_bits_block,
            7,
            ScriptVerifyFlags::P2SH,
            consensus_params,
        )
        .expect("last non-special target should be accepted");

    // Assert
    assert!(matches!(
        error,
        crate::ChainstateError::BlockValidation { source }
            if source.reject_reason == "bad-diffbits"
                && source.debug_message.as_deref() == Some("incorrect proof of work")
    ));
    assert_eq!(recovered_position.height, 6);
    assert_eq!(recovered_position.header.bits, recovered_bits);
}

#[test]
fn difficulty_interval_helper_clamps_non_positive_spacing() {
    let interval = difficulty_adjustment_interval(&ConsensusParams {
        pow_target_spacing_seconds: 0,
        ..ConsensusParams::default()
    });

    assert_eq!(interval, 1);
}
