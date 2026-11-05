// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

const FIXTURE_ENFORCE_SEGWIT: bool = false;

fn canonical_compact(admitted: bool) -> (PeerManager, CompactBlockPayload, BlockHash) {
    use open_bitcoin_consensus::chain_spec::{CanonicalChain, CanonicalGenesisProof};
    let spec = CanonicalChain::Regtest.spec();
    let proof = CanonicalGenesisProof::prove(spec, &spec.genesis_block()).expect("proved root");
    let mut store = HeaderStore::from_canonical_genesis(&proof).expect("canonical store");
    let mut candidate = proof.header().clone();
    candidate.previous_block_hash = proof.hash();
    candidate.time += 1;
    candidate.version = 4;
    candidate.nonce = 0;
    while check_block_header(&candidate).is_err() {
        candidate.nonce += 1;
    }
    if admitted {
        store
            .insert_canonical_header(candidate.clone(), i64::from(candidate.time) + 100)
            .expect("earned candidate");
    }
    let (mut manager, mut payload, _, _) = phase115_prepare_compact_download_manager(90);
    payload.header = candidate;
    let hash = block_hash(&payload.header);
    manager.seed_header_store(store);
    (manager, payload, hash)
}

fn add_ready_peer(manager: &mut PeerManager, peer_id: PeerId) {
    manager.add_outbound_peer(peer_id, 0).expect("peer");
    complete_outbound_handshake(manager, peer_id, 2);
    process_high_bandwidth_sendcmpct(manager, peer_id);
}

fn requested_hashes(maybe_message: Option<WireNetworkMessage>) -> Vec<BlockHash> {
    let Some(WireNetworkMessage::GetData(inventory)) = maybe_message else {
        return Vec::new();
    };
    inventory
        .inventory
        .into_iter()
        .map(|item| BlockHash::from(item.object_hash))
        .collect()
}

fn notfound(manager: &mut PeerManager, peer_id: PeerId, hash: BlockHash) {
    manager
        .handle_message(
            peer_id,
            WireNetworkMessage::NotFound(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::Block,
                object_hash: hash.into(),
            }])),
            0,
        )
        .expect("notfound");
}

#[test]
fn phase167_compact_provenance_order_arrival_survives_duplicate_and_reenrollment() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    let hash = BlockHash::from(hash_from_index(9));
    for peer_id in [90, 2, 7] {
        add_ready_peer(&mut manager, peer_id);
        assert!(
            manager
                .request_missing_blocks(peer_id, &[hash])
                .expect("request")
                .is_some()
        );
    }

    // Act / Assert
    assert_eq!(manager.block_request_peer_order(hash), vec![90, 2, 7]);
    assert!(
        manager
            .request_missing_blocks(90, &[hash])
            .expect("duplicate")
            .is_none()
    );
    assert_eq!(manager.block_request_peer_order(hash), vec![90, 2, 7]);
    notfound(&mut manager, 2, hash);
    manager
        .request_missing_blocks(2, &[hash])
        .expect("reenroll");
    assert_eq!(manager.block_request_peer_order(hash), vec![90, 7, 2]);
    manager.remove_peer(90).expect("disconnect");
    assert_eq!(manager.block_request_peer_order(hash), vec![7, 2]);
}

#[test]
fn phase167_compact_provenance_order_explicit_dedup_cap_and_release() {
    // Arrange
    let mut manager = PeerManager::with_max_blocks_in_flight(local_config(), 2);
    add_ready_peer(&mut manager, 90);
    let hashes: Vec<_> = (1..=3)
        .map(|index| BlockHash::from(hash_from_index(index)))
        .collect();

    // Act
    let requested = manager
        .request_missing_blocks(90, &[hashes[1], hashes[1], hashes[0], hashes[2]])
        .expect("request");

    // Assert
    assert_eq!(requested_hashes(requested), vec![hashes[1], hashes[0]]);
    assert!(
        manager
            .request_missing_blocks(90, &hashes)
            .expect("at cap")
            .is_none()
    );
    notfound(&mut manager, 90, hashes[1]);
    assert_eq!(
        requested_hashes(
            manager
                .request_missing_blocks(90, &[hashes[2]])
                .expect("released")
        ),
        vec![hashes[2]]
    );
    assert_eq!(
        manager.peer_requested_blocks(90).expect("membership"),
        vec![hashes[0], hashes[2]]
    );
}

#[test]
fn phase167_compact_provenance_order_notfound_releases_sender_partial_only() {
    // Arrange
    let (mut manager, payload, _, hash) = phase115_prepare_compact_download_manager(90);
    add_ready_peer(&mut manager, 2);
    for peer in [90, 2] {
        manager
            .handle_fixture_compact_block_download(
                peer,
                payload.clone(),
                explicit_empty_compact_receive_facts(),
                100,
                FIXTURE_ENFORCE_SEGWIT,
            )
            .expect("real compact request");
        manager
            .request_missing_blocks(peer, &[hash])
            .expect("ordinary attachment");
    }
    let ordinary_hash = BlockHash::from(hash_from_index(10));
    manager
        .request_missing_blocks(90, &[ordinary_hash])
        .expect("unrelated ordinary request");
    let other_before = manager.compact_download_peer_state(2).cloned();

    // Act
    notfound(&mut manager, 90, hash);

    // Assert
    assert!(
        manager
            .compact_download_peer_state(90)
            .is_some_and(|state| !state.in_flight.contains_key(&hash))
    );
    assert_eq!(
        manager.compact_download_peer_state(2),
        other_before.as_ref()
    );
    assert_eq!(
        manager
            .peer_requested_blocks(90)
            .expect("sender membership"),
        vec![ordinary_hash]
    );
    assert_eq!(
        manager.peer_requested_blocks(2).expect("other membership"),
        vec![hash]
    );
}

#[test]
fn phase167_compact_provenance_order_compact_cleanup_preserves_ordinary_requests() {
    // Arrange
    let (mut manager, payload, _, hash) = phase115_prepare_compact_download_manager(90);
    let ordinary_hash = BlockHash::from(hash_from_index(10));
    manager
        .request_missing_blocks(90, &[ordinary_hash])
        .expect("ordinary request");
    manager
        .handle_fixture_compact_block_download(
            90,
            payload,
            explicit_empty_compact_receive_facts(),
            100,
            FIXTURE_ENFORCE_SEGWIT,
        )
        .expect("real compact request");

    // Act
    let removed = manager
        .cleanup_compact_download_for_peer(90, CompactDownloadCleanupCause::Reorg)
        .expect("scoped cleanup");

    // Assert
    assert_eq!(removed, 1);
    assert_eq!(
        manager
            .peer_requested_blocks(90)
            .expect("ordinary preserved"),
        vec![ordinary_hash]
    );
    assert!(
        manager
            .compact_download_peer_state(90)
            .is_some_and(|state| !state.in_flight.contains_key(&hash))
    );
}

#[test]
fn phase167_compact_provenance_compact_tracks_arrival_and_attachment_at_cap() {
    // Arrange
    let (mut manager, payload, _, hash) = phase115_prepare_compact_download_manager(90);
    add_ready_peer(&mut manager, 2);
    manager.max_blocks_in_flight_per_peer = 1;
    manager
        .request_missing_blocks(90, &[hash])
        .expect("ordinary first");

    // Act
    for (peer, now) in [(2, 100), (90, 10)] {
        let actions = manager
            .handle_fixture_compact_block_download(
                peer,
                payload.clone(),
                explicit_empty_compact_receive_facts(),
                now,
                FIXTURE_ENFORCE_SEGWIT,
            )
            .expect("compact");
        assert!(matches!(
            actions.as_slice(),
            [PeerAction::Send(WireNetworkMessage::GetBlockTxn(_))]
        ));
    }

    // Assert
    assert_eq!(manager.block_request_peer_order(hash), vec![90, 2]);
    assert_eq!(
        manager.maybe_block_request_enforce_segwit(90, hash),
        Some(FIXTURE_ENFORCE_SEGWIT)
    );
    assert_eq!(
        manager
            .peer_requested_blocks(2)
            .expect("compact membership"),
        vec![hash]
    );
    manager
        .handle_fixture_compact_block_download(
            90,
            payload,
            explicit_empty_compact_receive_facts(),
            -10,
            FIXTURE_ENFORCE_SEGWIT,
        )
        .expect("duplicate");
    assert_eq!(
        manager
            .compact_download_peer_state(90)
            .expect("partial")
            .in_flight[&hash]
            .started_at_unix,
        10
    );
    assert_eq!(manager.block_request_peer_order(hash), vec![90, 2]);
    manager.cleanup_all_compact_downloads(CompactDownloadCleanupCause::Reorg);
    assert_eq!(manager.block_request_peer_order(hash), vec![90]);
    assert!(
        manager
            .peer_requested_blocks(2)
            .expect("compact released")
            .is_empty()
    );
}

#[test]
fn phase167_compact_provenance_compact_new_request_and_fallback_at_cap_suppress() {
    // Arrange
    for collision in [false, true] {
        let (mut manager, mut payload, _, hash) = phase115_prepare_compact_download_manager(90);
        manager.max_blocks_in_flight_per_peer = 1;
        let ordinary = BlockHash::from(hash_from_index(40));
        manager
            .request_missing_blocks(90, &[ordinary])
            .expect("fill cap");
        if collision {
            payload.short_ids.push(payload.short_ids[0]);
        }

        // Act
        let actions = manager
            .handle_fixture_compact_block_download(
                90,
                payload,
                explicit_empty_compact_receive_facts(),
                100,
                FIXTURE_ENFORCE_SEGWIT,
            )
            .expect("capacity suppression");

        // Assert
        assert!(actions.is_empty(), "collision={collision}");
        assert!(manager.block_request_peer_order(hash).is_empty());
        assert_eq!(
            manager.peer_requested_blocks(90).expect("membership"),
            vec![ordinary]
        );
        assert!(
            manager
                .compact_download_peer_state(90)
                .is_none_or(|state| state.in_flight.is_empty())
        );
    }
}

#[test]
fn phase167_compact_provenance_compact_timeout_preserves_request_and_strict_age() {
    // Arrange
    let (mut manager, payload, _, hash) = phase115_prepare_compact_download_manager(90);
    manager.max_blocks_in_flight_per_peer = 1;
    manager
        .handle_fixture_compact_block_download(
            90,
            payload,
            explicit_empty_compact_receive_facts(),
            100,
            FIXTURE_ENFORCE_SEGWIT,
        )
        .expect("compact");

    // Act / Assert
    assert!(manager.expire_compact_download_timeouts(99).is_empty());
    assert!(manager.expire_compact_download_timeouts(160).is_empty());
    let actions = manager.expire_compact_download_timeouts(161);
    assert!(matches!(
        actions.as_slice(),
        [(90, PeerAction::Send(WireNetworkMessage::GetData(_)))]
    ));
    assert_eq!(
        manager
            .peer_requested_blocks(90)
            .expect("tracked full fallback"),
        vec![hash]
    );
    assert_eq!(manager.block_request_peer_order(hash), vec![90]);
    assert!(
        manager
            .compact_download_peer_state(90)
            .expect("expired partial")
            .in_flight
            .is_empty()
    );
    assert!(
        manager
            .expire_compact_download_timeouts(i64::MAX)
            .is_empty()
    );
}

#[test]
fn phase167_compact_provenance_context_unbound_production_refuses_before_effects() {
    // Arrange
    let (mut manager, payload, _, _) = phase115_prepare_compact_download_manager(90);
    let before = format!("{manager:?}");
    // Act
    let result = manager.handle_compact_block_download(
        90,
        payload,
        explicit_empty_compact_receive_facts(),
        100,
    );
    // Assert
    assert!(result.is_err());
    assert_eq!(format!("{manager:?}"), before);
}

#[test]
fn phase167_compact_provenance_context_unadmitted_and_foreign_candidate_refuse() {
    // Arrange
    for foreign in [false, true] {
        let (mut manager, mut payload, _) = canonical_compact(false);
        if foreign {
            payload.header.previous_block_hash = BlockHash::from_byte_array([77; 32]);
        }
        let before = format!("{manager:?}");
        // Act
        let result = manager.handle_compact_block_download(
            90,
            payload,
            explicit_empty_compact_receive_facts(),
            0,
        );
        // Assert
        assert!(result.is_err());
        assert_eq!(format!("{manager:?}"), before);
    }
}

#[test]
fn phase167_compact_provenance_context_admitted_uses_held_rules_not_protocol_or_clock() {
    use super::super::block_requests::CompactRequestRules;
    // Arrange
    let (mut manager, payload, hash) = canonical_compact(true);
    let held = manager
        .header_store()
        .canonical_header_validation_context(&payload.header)
        .expect("held context");
    let selected = manager
        .header_store()
        .canonical_header_history(hash)
        .expect("history")
        .rules();
    let before = format!("{manager:?}");
    assert!(
        manager
            .handle_fixture_compact_block_download(
                90,
                payload.clone(),
                explicit_empty_compact_receive_facts(),
                0,
                false
            )
            .is_err()
    );
    assert_eq!(format!("{manager:?}"), before);
    // Act
    let actions = manager
        .handle_compact_block_download(90, payload, explicit_empty_compact_receive_facts(), -500)
        .expect("earned compact context");
    // Assert
    assert!(matches!(
        actions.as_slice(),
        [PeerAction::Send(WireNetworkMessage::GetBlockTxn(_))]
    ));
    assert_eq!(
        manager.maybe_block_request_enforce_segwit(90, hash),
        Some(selected.enforce_segwit_commitment())
    );
    let Some(CompactRequestRules::Canonical(actual)) =
        manager.maybe_compact_request_rules(90, hash)
    else {
        panic!("actual canonical rule binding");
    };
    assert_eq!(&**actual, &(selected, held));
}

#[test]
fn phase167_compact_provenance_context_blocktxn_requires_real_requested_membership() {
    // Arrange
    let (mut manager, payload, missing, hash) = phase115_prepare_compact_download_manager(90);
    manager
        .handle_fixture_compact_block_download(
            90,
            payload,
            explicit_empty_compact_receive_facts(),
            0,
            FIXTURE_ENFORCE_SEGWIT,
        )
        .expect("real request");
    manager
        .peers
        .get_mut(&90)
        .expect("peer")
        .requested_blocks
        .clear();
    let before = format!("{manager:?}");
    // Act
    let result = manager.handle_message(
        90,
        WireNetworkMessage::BlockTxn(BlockTransactions {
            block_hash: hash,
            transactions: vec![missing],
        }),
        1,
    );
    // Assert
    assert!(result.is_err());
    assert_eq!(format!("{manager:?}"), before);
}

#[test]
fn phase167_compact_provenance_context_blocktxn_rechecks_held_canonical_owner() {
    // Arrange
    let (mut manager, payload, hash) = canonical_compact(true);
    manager
        .handle_compact_block_download(90, payload, explicit_empty_compact_receive_facts(), 0)
        .expect("real canonical request");
    manager.seed_header_store(HeaderStore::default());
    let before = format!("{manager:?}");
    // Act
    let result = manager.handle_message(
        90,
        WireNetworkMessage::BlockTxn(BlockTransactions {
            block_hash: hash,
            transactions: vec![phase115_sample_transaction(0x22)],
        }),
        1,
    );
    // Assert
    assert!(result.is_err());
    assert_eq!(format!("{manager:?}"), before);
}

#[test]
fn phase167_compact_provenance_headers_duplicate_and_no_request_controls() {
    // Arrange
    let (mut manager, payload, _, hash) = phase115_prepare_compact_download_manager(90);
    let headers = HeadersMessage {
        headers: vec![payload.header.clone()],
    };
    let insert =
        |store: &mut HeaderStore, header: &BlockHeader| store.insert_header(header.clone());
    // Act / Assert
    manager
        .handle_headers_with_policy(90, headers.clone(), HeaderSyncPolicy::HeadersOnly, insert)
        .expect("headers only");
    assert!(manager.block_request_peer_order(hash).is_empty());
    let requested = manager
        .handle_headers_with_policy(
            90,
            headers.clone(),
            HeaderSyncPolicy::HeadersAndBlocks,
            insert,
        )
        .expect("real request");
    assert!(
        requested
            .iter()
            .any(|action| matches!(action, PeerAction::Send(WireNetworkMessage::GetData(_))))
    );
    assert_eq!(manager.block_request_peer_order(hash), vec![90]);
    let before = format!("{manager:?}");
    let duplicate = manager
        .handle_headers_with_policy(90, headers, HeaderSyncPolicy::HeadersAndBlocks, insert)
        .expect("duplicate");
    assert!(duplicate.is_empty());
    assert_eq!(format!("{manager:?}"), before);
    notfound(&mut manager, 90, hash);
    manager.note_local_block_hash(hash);
    assert!(
        manager
            .request_missing_blocks(90, &[hash])
            .expect("known block")
            .is_none()
    );
    assert!(manager.block_request_peer_order(hash).is_empty());
}

#[test]
fn phase167_compact_provenance_compact_existing_fallback_at_cap_keeps_ordinary_order() {
    // Arrange
    let (mut manager, mut payload, _, hash) = phase115_prepare_compact_download_manager(90);
    manager.max_blocks_in_flight_per_peer = 1;
    manager
        .request_missing_blocks(90, &[hash])
        .expect("ordinary request");
    payload.short_ids.push(payload.short_ids[0]);
    // Act
    let actions = manager
        .handle_fixture_compact_block_download(
            90,
            payload,
            explicit_empty_compact_receive_facts(),
            0,
            FIXTURE_ENFORCE_SEGWIT,
        )
        .expect("existing full fallback");
    // Assert
    assert!(matches!(
        actions.as_slice(),
        [PeerAction::Send(WireNetworkMessage::GetData(_))]
    ));
    assert_eq!(manager.block_request_peer_order(hash), vec![90]);
    assert_eq!(
        manager
            .peer_requested_blocks(90)
            .expect("one capacity slot"),
        vec![hash]
    );
    manager.cleanup_all_compact_downloads(CompactDownloadCleanupCause::Reorg);
    assert_eq!(manager.block_request_peer_order(hash), vec![90]);
    assert_eq!(manager.maybe_block_request_enforce_segwit(90, hash), None);
}
