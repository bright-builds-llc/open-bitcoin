// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txdb.h
// - packages/bitcoin-knots/src/txdb.cpp
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp

use open_bitcoin_core::{
    chainstate::Coin,
    primitives::{Amount, BlockHash, OutPoint, ScriptBuf, TransactionOutput, Txid},
};

use super::{
    decode_coin_value, decode_head_blocks_value, encode_best_block_key, encode_coin_key,
    encode_coin_value, encode_head_blocks_key,
};
use crate::storage::{StorageError, StorageNamespace};

#[test]
fn encode_coin_key_is_c_plus_txid_plus_le_vout() {
    // Arrange
    let outpoint = OutPoint {
        txid: Txid::from_byte_array([0xff; 32]),
        vout: 256,
    };

    // Act
    let key = encode_coin_key(&outpoint);

    // Assert
    assert_eq!(key.len(), 37);
    assert_eq!(key[0], b'C');
    assert_eq!(&key[1..33], &[0xff; 32]);
    assert_eq!(&key[33..37], &256_u32.to_le_bytes());
}

#[test]
fn best_block_and_head_blocks_keys_are_single_bytes() {
    // Arrange / Act
    let best_block_key = encode_best_block_key();
    let head_blocks_key = encode_head_blocks_key();

    // Assert
    assert_eq!(best_block_key, [b'B']);
    assert_eq!(head_blocks_key, [b'H']);
}

#[test]
fn compact_coin_round_trips_height_and_mtp() {
    // Arrange
    let coin = Coin {
        output: TransactionOutput {
            value: Amount::from_sats(50).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        },
        is_coinbase: true,
        created_height: 100,
        created_median_time_past: 1_700_000_000,
    };

    // Act
    let encoded = encode_coin_value(&coin).expect("encode compact coin");
    let decoded = decode_coin_value(&encoded).expect("decode compact coin");

    // Assert
    assert_eq!(decoded, coin);
    assert_eq!(decoded.created_height, 100);
    assert_eq!(decoded.created_median_time_past, 1_700_000_000);
}

#[test]
fn decode_coin_value_trailing_byte_is_corruption() {
    // Arrange
    let coin = Coin {
        output: TransactionOutput {
            value: Amount::from_sats(50).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        },
        is_coinbase: true,
        created_height: 100,
        created_median_time_past: 1_700_000_000,
    };
    let mut bytes = encode_coin_value(&coin).expect("encode compact coin");
    bytes.push(0x00);

    // Act
    let error = decode_coin_value(&bytes);

    // Assert
    assert!(matches!(
        error,
        Err(StorageError::Corruption {
            namespace: StorageNamespace::Coins,
            ..
        })
    ));
}

#[test]
fn decode_coin_value_empty_bytes_is_corruption() {
    // Arrange
    let bytes: &[u8] = &[];

    // Act
    let error = decode_coin_value(bytes);

    // Assert
    assert!(matches!(
        error,
        Err(StorageError::Corruption {
            namespace: StorageNamespace::Coins,
            ..
        })
    ));
}

#[test]
fn decode_head_blocks_rejects_count_one() {
    // Arrange
    let mut count_one = vec![0x01];
    count_one.extend_from_slice(&[0xab; 32]);
    let empty: &[u8] = &[];

    // Act
    let count_one_error = decode_head_blocks_value(&count_one);
    let empty_heads = decode_head_blocks_value(empty);

    // Assert
    assert!(matches!(
        count_one_error,
        Err(StorageError::Corruption {
            namespace: StorageNamespace::Coins,
            ..
        })
    ));
    assert_eq!(empty_heads, Ok(Vec::<BlockHash>::new()));
}
