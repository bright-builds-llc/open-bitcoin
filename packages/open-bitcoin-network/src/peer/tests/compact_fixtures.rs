// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

pub(super) fn compact_announcement_activation(enabled: bool) -> BlockRelayActivationPolicy {
    BlockRelayActivationPolicy {
        compact_relay: CompactRelayActivationConfig { enabled },
        ..BlockRelayActivationPolicy::default()
    }
}

pub(super) fn compact_announcement_input(
    compact_relay_enabled: bool,
    peer_has_previous_header: bool,
    peer_has_current_header: bool,
    status: BlockServingStatusDecision,
    resource_gate: BlockServingResourceGateDecision,
) -> PeerCompactAnnouncementInput {
    PeerCompactAnnouncementInput {
        activation: compact_announcement_activation(compact_relay_enabled),
        peer_has_previous_header,
        peer_has_current_header,
        status,
        resource_gate,
    }
}

pub(super) fn compact_available_status() -> BlockServingStatusDecision {
    BlockServingStatusDecision {
        label: BlockServingStatusLabel::Available,
        allow_storage_read: true,
        may_serve_block: true,
    }
}

pub(super) fn compact_unavailable_status() -> BlockServingStatusDecision {
    BlockServingStatusDecision {
        label: BlockServingStatusLabel::Unavailable,
        allow_storage_read: false,
        may_serve_block: false,
    }
}

pub(super) fn compact_available_resource_gate() -> BlockServingResourceGateDecision {
    BlockServingResourceGateDecision {
        label: BlockServingOutcomeLabel::BlockServingEligible,
        allow_storage_read: true,
        may_serve_block: true,
        maybe_resource_event: None,
        maybe_cleanup: None,
    }
}

pub(super) fn compact_limited_resource_gate() -> BlockServingResourceGateDecision {
    BlockServingResourceGateDecision {
        label: BlockServingOutcomeLabel::BlockRequestCapReached,
        allow_storage_read: false,
        may_serve_block: false,
        maybe_resource_event: None,
        maybe_cleanup: None,
    }
}

pub(super) fn process_high_bandwidth_sendcmpct(manager: &mut PeerManager, peer_id: PeerId) {
    manager
        .handle_message(
            peer_id,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: true,
                version: 2,
            }),
            1,
        )
        .expect("sendcmpct high-bandwidth should process");
}

pub(super) fn process_low_bandwidth_sendcmpct(manager: &mut PeerManager, peer_id: PeerId) {
    manager
        .handle_message(
            peer_id,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: false,
                version: 2,
            }),
            1,
        )
        .expect("sendcmpct low-bandwidth should process");
}

pub(super) fn phase115_seed_header_chain(manager: &mut PeerManager, headers: &[BlockHeader]) {
    let mut store = HeaderStore::default();
    for header in headers {
        let _ = store
            .insert_header(header.clone())
            .expect("header should insert");
    }
    manager.seed_header_store(store);
}

pub(super) fn phase115_coinbase_transaction() -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(vec![0x01, 0x02]).expect("valid script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(50_000_000_000).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        }],
        lock_time: 0,
    }
}

pub(super) fn phase115_sample_transaction(previous_txid_byte: u8) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([previous_txid_byte; 32]),
                vout: 0,
            },
            script_sig: ScriptBuf::from_bytes(Vec::new()).expect("valid script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::new(vec![vec![0x01, 0x02]]),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(5_000).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51, 0xac]).expect("valid script"),
        }],
        lock_time: 0,
    }
}

pub(super) fn phase115_compact_payload_with_missing_short_id()
-> (CompactBlockPayload, Transaction, Wtxid) {
    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let tip = mined_header(block_hash(&genesis), 2);
    let header = mined_header(block_hash(&tip), 3);
    let coinbase = phase115_coinbase_transaction();
    let missing = phase115_sample_transaction(0x22);
    let wtxid = transaction_wtxid(&missing).expect("wtxid");
    let selector = open_bitcoin_codec::short_id_selector_from_header_and_nonce(&header, 42);
    let short_id = open_bitcoin_consensus::compact_short_id_for_wtxid(selector, &wtxid);

    let payload = CompactBlockPayload {
        header,
        nonce: 42,
        short_ids: vec![short_id],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase,
        }],
    };

    (payload, missing, wtxid)
}

pub(super) fn phase115_prepare_compact_download_manager(
    peer_id: PeerId,
) -> (PeerManager, CompactBlockPayload, Transaction, BlockHash) {
    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let tip = mined_header(block_hash(&genesis), 2);
    let (payload, missing, _) = phase115_compact_payload_with_missing_short_id();
    let block_hash = block_hash(&payload.header);
    let mut manager = PeerManager::new(local_config());
    manager.set_block_relay_activation_policy(compact_announcement_activation(true));
    phase115_seed_header_chain(&mut manager, &[genesis, tip]);
    manager.add_outbound_peer(peer_id, 0).expect("peer");
    complete_outbound_handshake(&mut manager, peer_id, 2);
    process_high_bandwidth_sendcmpct(&mut manager, peer_id);
    (manager, payload, missing, block_hash)
}

pub(super) fn explicit_empty_compact_receive_facts() -> CompactBlockReceiveFacts<'static> {
    CompactBlockReceiveFacts {
        candidates: &[],
        extra: &[],
    }
}

pub(super) fn phase119_compact_payload_with_one_matched_and_one_missing()
-> (CompactBlockPayload, Transaction, Wtxid, Transaction, Wtxid) {
    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let tip = mined_header(block_hash(&genesis), 2);
    let header = mined_header(block_hash(&tip), 3);
    let coinbase = phase115_coinbase_transaction();
    let matched = phase115_sample_transaction(0x31);
    let still_missing = phase115_sample_transaction(0x32);
    let matched_wtxid = transaction_wtxid(&matched).expect("matched wtxid");
    let missing_wtxid = transaction_wtxid(&still_missing).expect("missing wtxid");
    let selector = open_bitcoin_codec::short_id_selector_from_header_and_nonce(&header, 42);
    let matched_short_id =
        open_bitcoin_consensus::compact_short_id_for_wtxid(selector, &matched_wtxid);
    let missing_short_id =
        open_bitcoin_consensus::compact_short_id_for_wtxid(selector, &missing_wtxid);

    let payload = CompactBlockPayload {
        header,
        nonce: 42,
        short_ids: vec![matched_short_id, missing_short_id],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase,
        }],
    };

    (
        payload,
        matched,
        matched_wtxid,
        still_missing,
        missing_wtxid,
    )
}

pub(super) fn announce_with_action_coinbase_block() -> Block {
    Block {
        header: mined_header(BlockHash::from_byte_array([0_u8; 32]), 4),
        transactions: vec![phase115_coinbase_transaction()],
    }
}

pub(super) fn compact_block_inventory(count: usize) -> InventoryList {
    InventoryList::new(
        (0..count)
            .map(|index| InventoryVector {
                inventory_type: InventoryType::CompactBlock,
                object_hash: hash_from_index(111_500 + index),
            })
            .collect(),
    )
}
