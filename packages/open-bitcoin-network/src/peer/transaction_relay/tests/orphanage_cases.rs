// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/txrequest.h
// - packages/bitcoin-knots/src/txrequest.cpp
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_opportunistic_1p1c.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use open_bitcoin_primitives::{
    Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
};

use super::*;

fn policy(
    max_total_orphans: usize,
    max_orphans_per_peer: usize,
    orphan_ttl_seconds: i64,
    max_reconsiderations_per_parent: usize,
) -> OrphanPolicy {
    OrphanPolicy {
        max_total_orphans,
        max_orphans_per_peer,
        max_announcers_per_orphan: 4,
        max_retained_bytes: PHASE133_MAX_ORPHAN_RETAINED_BYTES,
        orphan_ttl_seconds,
        max_reconsiderations_per_parent,
    }
}

fn orphan_input(
    _peer_id: PeerId,
    tx_byte: u8,
    wtx_byte: u8,
    missing_parent_bytes: impl IntoIterator<Item = u8>,
    now_unix_seconds: i64,
) -> OrphanStageInput {
    OrphanStageInput {
        transaction: Transaction::default(),
        txid: txid(tx_byte),
        wtxid: wtxid(wtx_byte),
        missing_parents: missing_parent_bytes.into_iter().map(txid).collect(),
        now_unix_seconds,
    }
}

fn stage_singleton(
    orphanage: &mut TxOrphanage,
    peer_id: PeerId,
    input: OrphanStageInput,
) -> Vec<OrphanAction> {
    orphanage.stage_missing_parent_with_provenance(input, provenance(peer_id, [peer_id]))
}

fn provenance(
    delivered_by: PeerId,
    announcers: impl IntoIterator<Item = PeerId>,
) -> ReceivedTransactionProvenance {
    ReceivedTransactionProvenance {
        delivered_by,
        announcers: announcers.into_iter().collect(),
    }
}

fn transaction(version: i32) -> Transaction {
    Transaction {
        version,
        ..Transaction::default()
    }
}

fn transaction_with_body_bytes(version: i32, body_bytes: usize) -> Transaction {
    Transaction {
        version,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(vec![0x51]).expect("bounded test script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::new(vec![vec![0x51; 16]]),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::ZERO,
            script_pubkey: ScriptBuf::from_bytes(vec![0x51; body_bytes])
                .expect("bounded test script"),
        }],
        ..Transaction::default()
    }
}

fn parent_request(peer_id: PeerId, parent_byte: u8) -> OrphanAction {
    OrphanAction::RequestParent {
        peer_id,
        relay_id: TxRelayId::Txid(txid(parent_byte)),
        label: OrphanEvidenceLabel::ParentRequested,
    }
}

fn evicted(peer_id: PeerId, tx_byte: u8, wtx_byte: u8) -> OrphanAction {
    OrphanAction::Evicted {
        peer_id,
        txid: txid(tx_byte),
        wtxid: wtxid(wtx_byte),
        label: OrphanEvidenceLabel::OrphanEvicted,
    }
}

fn expired(peer_id: PeerId, tx_byte: u8, wtx_byte: u8) -> OrphanAction {
    OrphanAction::Expired {
        peer_id,
        txid: txid(tx_byte),
        wtxid: wtxid(wtx_byte),
        label: OrphanEvidenceLabel::OrphanExpired,
    }
}

mod boundedness_cases;
mod candidate_cases;
mod lifecycle_cases;
mod policy_cases;
mod provenance_cases;
