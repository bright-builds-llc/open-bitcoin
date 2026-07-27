// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/p2p_opportunistic_1p1c.py

use open_bitcoin_mempool::{
    EffectiveFeeGroupId, ExistingMember, HardMemberFailure, MempoolMemberIdentity, MempoolOutcome,
    MempoolRejectionCategory, NewlyPresent, PackageMemberResult, PackageStatus, PostTrimAbsence,
    PriorMemberSuccess, ReconsiderableMemberFailure, RollingMempoolFeeRate, WellFormedPackage,
    WitnessAlias,
};
use open_bitcoin_network::ReceivedTransactionProvenance;

use super::*;
use crate::network::admission_bridge::ManagedPeerAdmissionResult;

fn cpfp_pair(parent_input: Txid) -> (Transaction, Transaction) {
    let parent = spend_transaction(parent_input, 499_999_900);
    let parent_txid = transaction_txid(&parent).expect("parent txid");
    let child = spend_transaction(parent_txid, 499_998_900);
    (parent, child)
}

fn identity(transaction: &Transaction) -> MempoolMemberIdentity {
    MempoolMemberIdentity {
        txid: transaction_txid(transaction).expect("txid"),
        wtxid: transaction_wtxid(transaction).expect("wtxid"),
    }
}

fn provenance(delivered_by: u64, announcers: &[u64]) -> ReceivedTransactionProvenance {
    ReceivedTransactionProvenance {
        delivered_by,
        announcers: announcers.to_vec(),
    }
}

fn package_network(
    peer_id: u64,
) -> (
    ManagedPeerNetwork<MemoryChainstateStore>,
    open_bitcoin_core::primitives::Txid,
) {
    let mut network = relay_enabled_managed_network(peer_id);
    network.add_inbound_peer(peer_id).expect("peer");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("spendable");
    network
        .set_rolling_mempool_fee_rate(RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(
            5_000,
        )))
        .expect("rolling floor");
    (
        network,
        transaction_txid(&genesis.transactions[0]).expect("coinbase txid"),
    )
}

mod candidate_selection;
mod feedback_domains;
mod singleton_and_submission;
