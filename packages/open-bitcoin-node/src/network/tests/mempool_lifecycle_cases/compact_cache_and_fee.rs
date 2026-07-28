// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/kernel/disconnected_transactions.cpp
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;
use crate::network::mempool_lifecycle;

#[test]
fn connected_block_mempool_removal_clears_matched_compact_partial_slot() {
    // Arrange — live CompactBlock leaves an in-flight partial with one mempool-matched slot
    let mut network = compact_relay_enabled_managed_network(119_301);
    let peer_id = 119_301;
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("connect spendable");

    let matched = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let matched_wtxid = wtxid(&matched);
    // Missing short-id body is not in the announced merkle tree; we never complete reconstruction.
    let still_missing = spend_transaction(txid(&genesis.transactions[0]), 499_998_000);
    let announced =
        build_block_with_transactions(block_hash(&spendable.header), 2, vec![matched.clone()]);
    let announced_hash = block_hash(&announced.header);
    let payload = compact_payload_matched_and_missing(&announced, &matched, &still_missing, 42);

    let outcome = network
        .submit_local_transaction_outcome_at(
            matched.clone(),
            verify_flags(),
            consensus_params(),
            40,
            RelayIntent::NotRequested,
        )
        .expect("admit matched tx");
    assert!(matches!(outcome, MempoolOutcome::Accepted { .. }));

    handshake_and_sendcmpct(&mut network, peer_id);
    let receive_time = i64::from(announced.header.time) + 60;
    let outbound = network
        .receive_message(
            peer_id,
            WireNetworkMessage::CompactBlock(payload),
            receive_time,
            verify_flags(),
            consensus_params(),
        )
        .expect("live compact receive with one match + one missing")
        .outbound;
    assert!(
        outbound
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::GetBlockTxn(_))),
        "expected GetBlockTxn so partial stays in-flight; outbound={outbound:?}"
    );

    let download_state = network
        .peer_manager()
        .compact_download_peer_state(peer_id)
        .expect("download state after receive");
    let in_flight = download_state
        .in_flight
        .get(&announced_hash)
        .expect("in-flight partial");
    assert!(in_flight.partial.is_transaction_available(1));
    assert!(!in_flight.partial.is_transaction_available(2));

    // Conflict block confirms removal of the matched mempool tx without connecting announced
    let conflict = spend_transaction(txid(&spendable.transactions[0]), 499_997_000);
    let conflict_block =
        build_block_with_transactions(block_hash(&spendable.header), 2, vec![conflict]);

    // Act — connected-block lifecycle must forward removal.wtxid into PeerManager
    network
        .connect_local_block(&conflict_block, verify_flags(), consensus_params())
        .expect("connect conflict block removing matched mempool tx");

    // Assert — matched volatile slot cleared; missing index remains
    let download_state = network
        .peer_manager()
        .compact_download_peer_state(peer_id)
        .expect("download state after lifecycle");
    let in_flight = download_state
        .in_flight
        .get(&announced_hash)
        .expect("in-flight partial retained");
    assert!(
        !in_flight.partial.is_transaction_available(1),
        "matched slot for wtxid {matched_wtxid:?} must clear after connected-block removal"
    );
    assert_eq!(in_flight.partial.missing_transaction_indexes(), vec![1, 2]);
    assert_lifecycle_authority(&network, 2);
}

#[test]
fn rolling_fee_decay_requires_connected_block_after_bump() {
    // Arrange — zero capacity keeps empty-pool occupancy on the default 12h half-life.
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(702),
        PolicyConfig {
            mempool_capacity: open_bitcoin_mempool::MempoolCapacity::new(0),
            ..PolicyConfig::default()
        },
    );
    let genesis = build_block_with_transactions(BlockHash::from_byte_array([0_u8; 32]), 0, vec![]);
    let spendable = build_block_with_transactions(block_hash(&genesis.header), 1, vec![]);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("connect spendable");
    let bumped = FeeRate::from_sats_per_kvb(10_000);
    network
        .track_package_removed_rolling_fee(bumped)
        .expect("revision remains available");
    let later = PolicyTime::new(ROLLING_FEE_HALFLIFE_SECONDS * 4);

    // Act — time advance alone must not decay after a pressure bump
    let without_block = network
        .materialize_rolling_mempool_fee_rate(later)
        .expect("revision remains available");

    // Assert
    assert_eq!(without_block.fee_rate(), bumped);

    // Act — connected-block lifecycle opens the gate; then one half-life halves the floor
    let connect_time = 1_700_000_100_i64;
    let empty_connect = build_block_with_transactions(block_hash(&spendable.header), 2, vec![]);
    let context = mempool_lifecycle::block_lifecycle_context(connect_time, 2);
    network
        .apply_connected_block_mempool_lifecycle(&empty_connect, context)
        .expect("open decay gate on connect");
    let after_halflife = PolicyTime::new(connect_time + ROLLING_FEE_HALFLIFE_SECONDS);
    let with_block = network
        .materialize_rolling_mempool_fee_rate(after_halflife)
        .expect("revision remains available");

    // Assert
    assert_eq!(with_block.fee_rate().sats_per_kvb(), 5_000);
    assert_lifecycle_authority(&network, 0);
}
