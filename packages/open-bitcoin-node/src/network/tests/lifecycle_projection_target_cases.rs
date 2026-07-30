// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use open_bitcoin_core::consensus::{block_hash, transaction_txid, transaction_wtxid};
use open_bitcoin_core::primitives::{BlockHash, Txid, Wtxid};
use open_bitcoin_mempool::{AdmissionContext, PolicyConfig, PolicyTime, RelayIntent};
use open_bitcoin_network::{
    AcceptedPeerPackageFingerprint, PeerTransactionIdentity, PeerTransactionLifecycleInput,
    PeerTransactionLifecyclePreparationError,
};

use super::*;
use crate::network::lifecycle_projection::{AuthorityEpoch, LifecycleProjectionPlan};

fn network_with_spendable_coinbase(
    nonce: u64,
    config: PolicyConfig,
) -> (ManagedPeerNetwork<MemoryChainstateStore>, Txid) {
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(nonce),
        config,
    );
    let genesis = build_block(BlockHash::from_byte_array([0; 32]), 0, 500_000_000);
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("connect spendable block");
    (
        network,
        transaction_txid(&spendable.transactions[0]).expect("coinbase txid"),
    )
}

fn identity(byte: u8) -> PeerTransactionIdentity {
    PeerTransactionIdentity::new(
        Txid::from_byte_array([byte; 32]),
        Wtxid::from_byte_array([byte.wrapping_add(128); 32]),
    )
}

fn target_snapshot(network: &ManagedPeerNetwork<MemoryChainstateStore>) -> String {
    format!(
        "{:?}|{:?}|{:?}|{:?}|{:?}|{:?}",
        network.compact_extra_txn.to_owned_pairs(),
        network.transactions_by_txid,
        network.transactions_by_wtxid,
        network.relay_serving_info(),
        network.relay_fanout_info(),
        network.peer_manager,
    )
}

#[test]
fn lifecycle_projection_target_cases_final_present_filter_removes_both_aliases() {
    // Arrange
    let accepted_at = 100;
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(
        134_041,
        PolicyConfig {
            mempool_expiry_hours: 1,
            ..PolicyConfig::default()
        },
    );
    let transaction = spend_transaction(coinbase_txid, 499_999_000);
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");
    network
        .submit_local_transaction_outcome_at(
            transaction,
            verify_flags(),
            consensus_params(),
            accepted_at,
            RelayIntent::NotRequested,
        )
        .expect("seed accepted targets");
    let core = network
        .mempool
        .prepare_expiry(PolicyTime::new(accepted_at + 3_601))
        .expect("prepare expiry");
    assert!(core.facts().final_present().is_empty());
    let prepared = LifecycleProjectionPlan::prepare(&network, AuthorityEpoch::INITIAL, core)
        .expect("prepare every target");

    // Act
    network.apply_prepared_compact(prepared.compact);
    network.apply_prepared_serving(prepared.serving);
    network.apply_prepared_fanout(prepared.fanout);
    network.apply_prepared_peer_lifecycle(prepared.peers);

    // Assert
    assert!(!network.transactions_by_txid.contains_key(&txid));
    assert!(!network.transactions_by_wtxid.contains_key(&wtxid));
    assert_eq!(network.relay_serving_info().serveable_transactions, 0);
    assert_eq!(network.relay_fanout_info().known_transactions, 0);
}

#[test]
fn lifecycle_projection_target_cases_replacement_retains_compact_body_before_serving_retirement() {
    // Arrange
    let (mut network, coinbase_txid) =
        network_with_spendable_coinbase(134_042, PolicyConfig::default());
    let original = spend_transaction(coinbase_txid, 499_999_000);
    let replacement = spend_transaction(coinbase_txid, 499_996_000);
    let original_txid = transaction_txid(&original).expect("original txid");
    let original_wtxid = transaction_wtxid(&original).expect("original wtxid");
    let replacement_txid = transaction_txid(&replacement).expect("replacement txid");
    let replacement_wtxid = transaction_wtxid(&replacement).expect("replacement wtxid");
    network
        .submit_local_transaction_outcome_at(
            original,
            verify_flags(),
            consensus_params(),
            200,
            RelayIntent::NotRequested,
        )
        .expect("seed replacement victim");
    let core = network
        .mempool
        .prepare_transaction_with_context(
            &network.chainstate,
            replacement,
            verify_flags(),
            consensus_params(),
            AdmissionContext::local(PolicyTime::new(201), RelayIntent::NotRequested),
        )
        .expect("prepare replacement");
    let prepared = LifecycleProjectionPlan::prepare(&network, AuthorityEpoch::INITIAL, core)
        .expect("prepare replacement targets");

    // Act
    network.apply_prepared_compact(prepared.compact);

    // Assert
    assert!(
        network
            .compact_extra_txn
            .iter_available()
            .any(|(wtxid, _)| *wtxid == original_wtxid)
    );
    assert!(network.transactions_by_txid.contains_key(&original_txid));

    // Act
    network.apply_prepared_serving(prepared.serving);

    // Assert
    assert!(!network.transactions_by_txid.contains_key(&original_txid));
    assert!(!network.transactions_by_wtxid.contains_key(&original_wtxid));
    assert!(network.transactions_by_txid.contains_key(&replacement_txid));
    assert!(
        network
            .transactions_by_wtxid
            .contains_key(&replacement_wtxid)
    );
}

#[test]
fn lifecycle_projection_target_cases_preparation_errors_preserve_complete_snapshot() {
    // Arrange
    let network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(134_043),
        PolicyConfig::default(),
    );
    let baseline = target_snapshot(&network);
    let same_txid_a = identity(1);
    let same_txid_b = PeerTransactionIdentity::new(same_txid_a.txid(), identity(2).wtxid());
    let overlap = identity(3);
    let body_a = identity(4);
    let body_b = identity(5);
    let fingerprint = [6; 32];
    let over_bound = (0..=100)
        .map(|index| AcceptedPeerPackageFingerprint::new([index as u8; 32], Vec::new()))
        .collect();
    let inputs = [
        PeerTransactionLifecycleInput::new(vec![same_txid_a, same_txid_b], Vec::new(), Vec::new()),
        PeerTransactionLifecycleInput::new(vec![overlap], vec![overlap], Vec::new()),
        PeerTransactionLifecycleInput::new(
            Vec::new(),
            Vec::new(),
            vec![
                AcceptedPeerPackageFingerprint::new(fingerprint, vec![body_a]),
                AcceptedPeerPackageFingerprint::new(fingerprint, vec![body_b]),
            ],
        ),
        PeerTransactionLifecycleInput::new(Vec::new(), Vec::new(), over_bound),
    ];

    // Act
    let errors = inputs.map(|input| {
        let Err(error) = network.peer_manager.prepare_transaction_lifecycle(input) else {
            panic!("invalid target work must fail in preparation");
        };
        error
    });

    // Assert
    assert!(matches!(
        errors[0],
        PeerTransactionLifecyclePreparationError::TxidAliasConflict { .. }
    ));
    assert!(matches!(
        errors[1],
        PeerTransactionLifecyclePreparationError::FinalMembershipConflict { .. }
    ));
    assert!(matches!(
        errors[2],
        PeerTransactionLifecyclePreparationError::FingerprintMembersConflict { .. }
    ));
    assert!(matches!(
        errors[3],
        PeerTransactionLifecyclePreparationError::AcceptedPackageCountLimit { .. }
    ));
    assert_eq!(target_snapshot(&network), baseline);
}
