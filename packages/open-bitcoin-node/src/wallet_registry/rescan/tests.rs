// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/wallet.cpp
// - packages/bitcoin-knots/src/wallet/rpc/transactions.cpp
// - packages/bitcoin-knots/src/coins.cpp

use super::*;
use crate::StorageRecoveryAction;
use open_bitcoin_core::{
    chainstate::{ChainPosition, Coin},
    primitives::{Amount, BlockHeader, OutPoint, ScriptBuf, TransactionOutput, Txid},
    wallet::{AddressNetwork, DescriptorRole},
};

fn fixture() -> (Wallet, ChainstateSnapshot) {
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    wallet.import_descriptor("receive", DescriptorRole::External, "wpkh(tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK/1/1/*)").expect("descriptor");
    let positions = (0..4)
        .map(|height| {
            ChainPosition::new(
                BlockHeader {
                    version: 1,
                    previous_block_hash: BlockHash::from_byte_array([0; 32]),
                    merkle_root: Default::default(),
                    time: height,
                    bits: 0x207fffff,
                    nonce: height,
                },
                height,
                u128::from(height) + 1,
                i64::from(height),
            )
        })
        .collect();
    let coin = Coin {
        output: TransactionOutput {
            value: Amount::from_sats(1000).expect("amount"),
            script_pubkey: wallet
                .default_receive_address()
                .expect("receive")
                .script_pubkey,
        },
        is_coinbase: false,
        created_height: 1,
        created_median_time_past: 1,
    };
    let utxos = [(
        OutPoint {
            txid: Txid::from_byte_array([1; 32]),
            vout: 0,
        },
        coin,
    )]
    .into_iter()
    .collect();
    (
        wallet,
        ChainstateSnapshot::new(positions, utxos, Default::default()),
    )
}

#[test]
fn creating_height_before_start_requires_payload_without_mutating_prior() {
    // Arrange
    let (wallet, truth) = fixture();
    let before = wallet.snapshot();
    let hash = truth.active_chain[1].block_hash;
    // Act
    let result = prepare_wallet_rescan_with_probe(&wallet, &truth, 2, 3, |probe| Ok(probe != hash));
    // Assert
    let failure = result.expect_err("creating payload required");
    assert_eq!(failure.boundary, WalletRescanEligibilityBoundary::Creating);
    assert_eq!(failure.maybe_height, Some(1));
    assert_eq!(failure.maybe_block_hash, Some(hash));
    assert_eq!(wallet.snapshot(), before);
}

#[test]
fn unrelated_old_coin_does_not_require_payload() {
    // Arrange
    let (wallet, mut truth) = fixture();
    let coin = truth.utxos.values_mut().next().expect("coin");
    coin.output.script_pubkey = ScriptBuf::from_bytes(vec![0x51]).expect("unrelated");
    let old_hash = truth.active_chain[1].block_hash;
    // Act
    let prepared = prepare_wallet_rescan_with_probe(&wallet, &truth, 2, 3, |hash| {
        assert_ne!(hash, old_hash);
        Ok(true)
    })
    .expect("prepare");
    // Assert
    assert!(prepared.wallet.utxos().is_empty());
}

#[test]
fn missing_creating_chain_position_refuses_replacement() {
    // Arrange
    let (wallet, mut truth) = fixture();
    truth.active_chain.retain(|position| position.height != 1);
    // Act
    let failure = prepare_wallet_rescan_with_probe(&wallet, &truth, 2, 3, |_| Ok(true))
        .expect_err("creating metadata");
    // Assert
    assert_eq!(failure.boundary, WalletRescanEligibilityBoundary::Creating);
    assert_eq!(failure.maybe_height, Some(1));
    assert_eq!(failure.maybe_block_hash, None);
}

#[test]
fn missing_requested_height_refuses_even_without_matching_coins() {
    // Arrange
    let (wallet, mut truth) = fixture();
    truth.utxos.clear();
    truth.active_chain.retain(|position| position.height != 2);
    // Act
    let failure = prepare_wallet_rescan_with_probe(&wallet, &truth, 2, 3, |_| Ok(true))
        .expect_err("requested metadata");
    // Assert
    assert_eq!(failure.boundary, WalletRescanEligibilityBoundary::Requested);
    assert_eq!(failure.maybe_height, Some(2));
}

#[test]
fn payload_probe_error_retains_facts_without_backend_path() {
    // Arrange
    let (wallet, truth) = fixture();
    let error = StorageError::BackendFailure {
        namespace: StorageNamespace::BlockIndex,
        message: "/private/secret/datadir".into(),
        action: StorageRecoveryAction::Repair,
    };
    let before = wallet.snapshot();
    // Act
    let failure = prepare_wallet_rescan_with_probe(&wallet, &truth, 2, 3, |_| Err(error.clone()))
        .expect_err("probe error");
    // Assert
    assert_eq!(failure.error, WalletRegistryError::Storage(error));
    assert_eq!(failure.maybe_height, Some(1));
    assert!(failure.safe_detail().contains("BackendFailure"));
    assert!(!failure.safe_detail().contains("private"));
    assert_eq!(wallet.snapshot(), before);
}

#[test]
fn duplicate_creating_heights_and_hashes_are_probed_once() {
    // Arrange
    let (wallet, mut truth) = fixture();
    let coin = truth.utxos.values().next().expect("coin").clone();
    truth.utxos.insert(
        OutPoint {
            txid: Txid::from_byte_array([2; 32]),
            vout: 0,
        },
        coin,
    );
    truth.active_chain[2].block_hash = truth.active_chain[1].block_hash;
    let before = wallet.snapshot();
    let mut hashes = BTreeSet::new();
    // Act
    let prepared = prepare_wallet_rescan_with_probe(&wallet, &truth, 1, 3, |hash| {
        assert!(hashes.insert(hash));
        Ok(true)
    })
    .expect("prepare");
    // Assert
    assert_eq!(hashes.len(), 2);
    assert_eq!(prepared.wallet.utxos().len(), 2);
    assert_eq!(wallet.snapshot(), before);
}

#[test]
fn coins_above_through_height_are_excluded() {
    // Arrange
    let (wallet, mut truth) = fixture();
    truth
        .utxos
        .values_mut()
        .next()
        .expect("coin")
        .created_height = 3;
    // Act
    let prepared =
        prepare_wallet_rescan_with_probe(&wallet, &truth, 2, 2, |_| Ok(true)).expect("prepare");
    // Assert
    assert!(prepared.wallet.utxos().is_empty());
    assert_eq!(prepared.through_height, 2);
    assert_eq!(prepared.maybe_tip_median_time_past, Some(2));
}

#[test]
fn empty_requested_interval_still_checks_replacement_candidates() {
    // Arrange
    let (wallet, truth) = fixture();
    // Act
    let failure = prepare_wallet_rescan_with_probe(&wallet, &truth, 4, 3, |_| Ok(false))
        .expect_err("candidate check");
    // Assert
    assert_eq!(failure.boundary, WalletRescanEligibilityBoundary::Creating);
}
