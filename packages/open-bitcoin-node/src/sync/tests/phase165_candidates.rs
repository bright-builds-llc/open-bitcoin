// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use super::*;
mod fixtures;
use fixtures::CandidateFixture;

#[test]
fn phase165_candidates_partial_extension_stops_turn_and_notifies_only_accepted_prefix() {
    // Arrange
    let mut fixture = CandidateFixture::new("165-partial-extension");
    let mut branch = fixture.branch(3, 20);
    branch[1] = fixtures::missing_coin_body(branch[1].clone());
    branch[2].header.previous_block_hash = block_hash(&branch[1].header);
    mine_header(&mut branch[2]);
    let sibling = build_branch_block(block_hash(&branch[0].header), 2, 30);
    fixture.headers(&branch);
    fixture.headers(std::slice::from_ref(&sibling));
    for block in branch.iter().chain(std::iter::once(&sibling)) {
        fixture.cache(block);
    }
    let captured = durable_tip_capture(&mut fixture.runtime);
    // Act
    let first = crate::sync::block_reconcile::reconcile_best_chain_for_live_session(
        &mut fixture.runtime,
        2_000_000_000,
    );
    // Assert: no stale ancestor snapshot can select the sibling in this turn.
    assert!(
        matches!(
            first,
            Ok(SyncReconcileProgress::ExtendedActiveChain { connected_count: 1 })
        ),
        "{first:?}"
    );
    assert_eq!(fixture.tip(), block_hash(&branch[0].header));
    assert!(captured.lock().expect("capture").is_empty());
    fixture
        .runtime
        .persist_progress_and_dispatch_tip()
        .expect("persist real prefix before notification");
    assert_eq!(
        *captured.lock().expect("capture"),
        vec![block_hash(&branch[0].header)]
    );
    assert!(
        fixture
            .runtime
            .network
            .maybe_body_eligibility(block_hash(&branch[1].header))
            .expect("read")
            .expect("checked body")
            .failed
    );
    // Act: a new turn obtains the new actual source/ancestry.
    let next = crate::sync::block_reconcile::reconcile_best_chain_for_live_session(
        &mut fixture.runtime,
        2_000_000_000,
    )
    .expect("fresh sibling turn");
    fixture
        .runtime
        .persist_progress_and_dispatch_tip()
        .expect("persist actual sibling");
    // Assert
    assert!(matches!(
        next,
        SyncReconcileProgress::ExtendedActiveChain { connected_count: 1 }
    ));
    assert_eq!(fixture.tip(), block_hash(&sibling.header));
    assert_eq!(
        *captured.lock().expect("capture"),
        vec![block_hash(&branch[0].header), block_hash(&sibling.header)]
    );
    fixture.cleanup();
}

#[test]
fn phase165_candidates_invalid_suffix_preserves_prefix_and_activates_valid_sibling() {
    // Arrange
    let mut fixture = CandidateFixture::new("165-failed-suffix");
    let active = fixture.branch(1, 10);
    let mut invalid = fixture.branch(3, 20);
    invalid[1].transactions.push(Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: open_bitcoin_core::primitives::Txid::from_byte_array([0x77; 32]),
                vout: 0,
            },
            script_sig: ScriptBuf::default(),
            sequence: u32::MAX,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(1).expect("amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    });
    invalid[1].header.merkle_root = block_merkle_root(&invalid[1].transactions)
        .expect("merkle")
        .0;
    mine_header(&mut invalid[1]);
    invalid[2].header.previous_block_hash = block_hash(&invalid[1].header);
    mine_header(&mut invalid[2]);
    let sibling = build_branch_block(block_hash(&invalid[0].header), 2, 30);
    fixture.headers(&active);
    fixture.body(&active[0]);
    fixture.headers(&invalid);
    fixture.headers(std::slice::from_ref(&sibling));
    for block in &invalid {
        fixture.body(block);
    }
    fixture.body(&sibling);
    // Act
    let result = fixture.reconcile();
    // Assert
    assert!(
        matches!(result, SyncReconcileProgress::ReorgPersisted(_)),
        "{result:?}"
    );
    assert_eq!(fixture.tip(), block_hash(&sibling.header));
    for (block, failed) in [(&invalid[0], false), (&invalid[1], true), (&sibling, false)] {
        assert_eq!(
            fixture
                .runtime
                .network
                .maybe_body_eligibility(block_hash(&block.header))
                .expect("read")
                .expect("checked linkage")
                .failed,
            failed
        );
    }
    fixture.cleanup();
}

#[test]
fn phase165_candidates_requested_side_bodies_persist_and_notify_actual_active_tip() {
    // Arrange
    let mut fixture = CandidateFixture::new("165-live-side");
    let active = fixture.branch(1, 10);
    fixture.headers(&active);
    fixture.body(&active[0]);
    let unavailable = fixture.branch(4, 20);
    let side = fixture.branch(2, 30);
    let captured = durable_tip_capture(&mut fixture.runtime);
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 4,
            nonce: 165_909,
            ..Default::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: unavailable
                .iter()
                .map(|block| block.header.clone())
                .collect(),
        }),
        WireNetworkMessage::Headers(HeadersMessage {
            headers: side.iter().map(|block| block.header.clone()).collect(),
        }),
        WireNetworkMessage::Block(side[0].clone()),
        WireNetworkMessage::Block(side[1].clone()),
    ]]);
    // Act
    let summary = fixture
        .runtime
        .sync_once(&mut transport, 2_000_000_000)
        .expect("finite real sync session");
    // Assert
    assert_eq!(fixture.tip(), block_hash(&side[1].header));
    assert_eq!(
        summary.maybe_connected_block_hash,
        Some(block_hash_hex(block_hash(&side[1].header)))
    );
    assert_eq!(
        *captured.lock().expect("capture"),
        vec![block_hash(&side[1].header)]
    );
    for block in &side {
        assert_eq!(
            fixture
                .runtime
                .store
                .load_block(block_hash(&block.header))
                .expect("durable read"),
            Some(block.clone())
        );
    }
    fixture.cleanup();
}

#[test]
fn phase165_candidates_missing_active_body_refuses_before_any_reorg_effect() {
    // Arrange
    let mut fixture = CandidateFixture::new("165-missing-active");
    let active = fixture.branch(1, 10);
    let side = fixture.branch(2, 20);
    fixture.headers(&active);
    fixture.headers(&side);
    fixture.body(&active[0]);
    for block in &side {
        fixture.body(block);
    }
    let before = fixture
        .runtime
        .network
        .chainstate_snapshot()
        .expect("before");
    fixture
        .runtime
        .store
        .delete_basic_input_for_test(block_hash(&active[0].header), false)
        .expect("actual physical body loss");
    // Act
    let result =
        crate::sync::block_reconcile::reconcile_best_chain(&mut fixture.runtime, 2_000_000_000);
    // Assert
    assert!(
        matches!(
            result,
            Err(SyncRuntimeError::Storage(StorageError::Corruption { .. }))
        ),
        "{result:?}"
    );
    assert_eq!(
        fixture
            .runtime
            .network
            .chainstate_snapshot()
            .expect("after"),
        before
    );
    fixture.cleanup();
}

#[test]
fn phase165_candidates_raw_cache_is_checked_after_headers_and_unknown_ties_refuse() {
    // Arrange
    let mut fixture = CandidateFixture::new("165-raw-cache");
    let active = fixture.branch(1, 10);
    let cached = fixture.branch(2, 20);
    let later = fixture.branch(2, 30);
    fixture.headers(&active);
    fixture.body(&active[0]);
    for block in cached.iter().rev() {
        fixture.cache(block);
    }
    assert!(
        fixture
            .runtime
            .network
            .maybe_body_eligibility(block_hash(&cached[1].header))
            .expect("read")
            .is_none()
    );
    fixture.headers(&cached);
    // Act
    let stronger = fixture.reconcile();
    fixture.headers(&later);
    for block in &later {
        fixture.body(block);
    }
    let tied = fixture.reconcile();
    // Assert
    assert!(matches!(stronger, SyncReconcileProgress::ReorgPersisted(_)));
    assert!(!tied.should_persist_progress());
    assert_eq!(fixture.tip(), block_hash(&cached[1].header));
    for block in &cached {
        assert_eq!(
            fixture
                .runtime
                .network
                .maybe_body_eligibility(block_hash(&block.header))
                .expect("read")
                .expect("checked cached linkage")
                .maybe_sequence,
            None
        );
    }
    fixture.cleanup();
}

#[test]
fn phase165_candidates_invalid_raw_cache_does_not_hide_valid_sibling() {
    // Arrange
    let mut fixture = CandidateFixture::new("165-invalid-cache");
    let active = fixture.branch(1, 10);
    let invalid = fixture.branch(3, 20);
    let valid = fixture.branch(2, 30);
    fixture.headers(&active);
    fixture.body(&active[0]);
    fixture.headers(&invalid);
    fixture.headers(&valid);
    let mut bad = invalid[0].clone();
    bad.transactions.clear();
    fixture.cache(&bad);
    for block in &valid {
        fixture.body(block);
    }
    // Act
    let result = fixture.reconcile();
    // Assert
    assert!(
        matches!(result, SyncReconcileProgress::ReorgPersisted(_)),
        "{result:?}"
    );
    assert_eq!(fixture.tip(), block_hash(&valid[1].header));
    assert!(
        fixture
            .runtime
            .network
            .maybe_body_eligibility(block_hash(&bad.header))
            .expect("read")
            .is_none()
    );
    fixture.cleanup();
}

#[test]
fn phase165_candidates_later_equal_body_cannot_displace_earlier_active() {
    // Arrange
    let mut fixture = CandidateFixture::new("165-later-equal");
    let active = fixture.branch(2, 10);
    let later = fixture.branch(2, 20);
    fixture.headers(&active);
    fixture.headers(&later);
    for block in &active {
        fixture.body(block);
    }
    for block in &later {
        fixture.body(block);
    }
    let before = fixture
        .runtime
        .network
        .chainstate_snapshot()
        .expect("before");
    // Act
    let result = fixture.reconcile();
    // Assert
    assert!(!result.should_persist_progress());
    assert_eq!(
        fixture
            .runtime
            .network
            .chainstate_snapshot()
            .expect("after"),
        before
    );
    fixture.cleanup();
}

#[test]
fn phase165_candidates_shorter_harder_branch_beats_longer_easier_branch() {
    // Arrange
    let params = open_bitcoin_core::consensus::ConsensusParams {
        no_pow_retargeting: false,
        pow_target_timespan_seconds: 1_200,
        ..Default::default()
    };
    let mut fixture = CandidateFixture::with_params("165-shorter-harder", params);
    let child = |parent: &Block, height, time, bits| {
        let mut block = build_block(block_hash(&parent.header), height);
        block.header.time = time;
        block.header.bits = bits;
        mine_header(&mut block);
        block
    };
    let time = fixture.root.header.time;
    let parent = child(&fixture.root, 1, time + 1, EASY_BITS);
    let base = child(&parent, 2, time + 2, 0x201f_ffff);
    fixture.headers(&[parent.clone(), base.clone()]);
    fixture.body(&parent);
    fixture.body(&base);
    let mut easy = base.clone();
    let mut easy_branch = Vec::new();
    for height in 3..=5 {
        easy = child(&easy, height, easy.header.time + 1_201, EASY_BITS);
        easy_branch.push(easy.clone());
    }
    fixture.headers(&easy_branch);
    for block in &easy_branch {
        fixture.body(block);
    }
    let hard = child(&base, 3, time + 3, 0x201f_ffff);
    fixture.headers(std::slice::from_ref(&hard));
    fixture.body(&hard);
    // Act
    let result = fixture.reconcile();
    // Assert
    assert!(
        matches!(result, SyncReconcileProgress::ReorgPersisted(_)),
        "{result:?}"
    );
    let tip = fixture
        .runtime
        .network
        .maybe_chain_tip()
        .expect("read")
        .expect("tip");
    assert_eq!(tip.block_hash, block_hash(&hard.header));
    assert_eq!(tip.height, 3);
    let mut expected_work = [0; 32];
    expected_work[0] = 20;
    assert_eq!(tip.chain_work.to_le_bytes(), expected_work);
    assert!(
        fixture
            .runtime
            .store
            .load_block(block_hash(&easy.header))
            .expect("read")
            .is_some()
    );
    fixture.cleanup();
}

#[test]
fn phase165_candidates_body_order_beats_first_header_equal_work() {
    // Arrange
    let mut fixture = CandidateFixture::new("165-body-order");
    let first = fixture.branch(2, 10);
    let earlier_body = fixture.branch(2, 20);
    fixture.headers(&first);
    fixture.headers(&earlier_body);
    fixture.body(&first[0]);
    fixture.body(&earlier_body[0]);
    fixture.body(&earlier_body[1]);
    fixture
        .runtime
        .store
        .save_block(&first[1], PersistMode::Sync)
        .expect("later raw bytes");
    assert_eq!(
        fixture
            .runtime
            .network
            .best_chain_entries()
            .expect("headers")
            .last()
            .expect("tip")
            .block_hash,
        block_hash(&first[1].header)
    );
    // Act
    let result = fixture.reconcile();
    // Assert
    assert!(
        matches!(result, SyncReconcileProgress::ReorgPersisted(_)),
        "{result:?}"
    );
    assert_eq!(fixture.tip(), block_hash(&earlier_body[1].header));
    fixture.cleanup();
}

#[test]
fn phase165_candidates_missing_stronger_headers_do_not_hide_available_branch() {
    // Arrange
    let mut fixture = CandidateFixture::new("165-unavailable-best");
    let active = fixture.branch(1, 10);
    let unavailable = fixture.branch(4, 20);
    let available = fixture.branch(2, 30);
    fixture.headers(&active);
    fixture.headers(&unavailable);
    fixture.headers(&available);
    fixture.body(&active[0]);
    fixture.body(&available[0]);
    fixture.body(&available[1]);
    // Act
    let result = fixture.reconcile();
    // Assert
    assert!(
        matches!(result, SyncReconcileProgress::ReorgPersisted(_)),
        "{result:?}"
    );
    assert_eq!(fixture.tip(), block_hash(&available[1].header));
    fixture.cleanup();
}

#[test]
fn phase165_candidates_late_parent_releases_child_without_duplicate_progress() {
    // Arrange
    let mut fixture = CandidateFixture::new("165-late-parent");
    let active = fixture.branch(1, 10);
    let unavailable = fixture.branch(4, 20);
    let side = fixture.branch(2, 30);
    fixture.headers(&active);
    fixture.headers(&unavailable);
    fixture.headers(&side);
    fixture.body(&active[0]);
    fixture.body(&side[1]);
    assert_eq!(
        fixture
            .runtime
            .network
            .maybe_body_eligibility(block_hash(&side[1].header))
            .expect("read")
            .expect("checked child")
            .maybe_sequence,
        None
    );
    fixture.body(&side[0]);
    let before = fixture
        .runtime
        .network
        .maybe_body_eligibility(block_hash(&side[1].header))
        .expect("read");
    fixture.body(&side[1]);
    // Act
    let result = fixture.reconcile();
    let duplicate = fixture.reconcile();
    // Assert
    assert!(
        matches!(result, SyncReconcileProgress::ReorgPersisted(_)),
        "{result:?}"
    );
    assert!(!duplicate.should_persist_progress());
    assert_eq!(fixture.tip(), block_hash(&side[1].header));
    assert_eq!(
        fixture
            .runtime
            .network
            .maybe_body_eligibility(block_hash(&side[1].header))
            .expect("read"),
        before
    );
    fixture.cleanup();
}
