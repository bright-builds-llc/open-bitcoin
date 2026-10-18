// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::fixtures::DaemonFixture;
use axum::{
    body::to_bytes,
    http::{HeaderMap, HeaderValue, Method, StatusCode},
};
use open_bitcoin_node::core::chainstate::{FlushMode, FlushPolicyTime};
use open_bitcoin_rpc::http::handle_http_request;
use serde_json::{Value, json};

async fn captured(fixture: &DaemonFixture) -> Value {
    let expected = serde_json::to_value(
        fixture
            .opened
            .network
            .operator_snapshot()
            .expect("frozen authority")
            .basic_index(),
    )
    .expect("BASIC JSON");
    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        HeaderValue::from_static("Basic cGhhc2UxNTk6cHJpdmF0ZS1wYXNzd29yZA=="),
    );
    let request = json!({"jsonrpc":"2.0","id":161,"method":"openbitcoinnetworkstatus"});
    let response = handle_http_request(
        &fixture.state,
        "/",
        Method::POST,
        &headers,
        request.to_string().as_bytes(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let wire: Value = serde_json::from_slice(
        &to_bytes(response.into_body(), 1024 * 1024)
            .await
            .expect("HTTP body"),
    )
    .expect("RPC JSON");
    assert_eq!(
        wire["result"]["basic_index"], expected,
        "same frozen authority"
    );
    assert!(expected.to_string().len() <= 4096);
    expected
}

#[tokio::test]
async fn phase161_operator_rpc_frozen_progress_auth_and_real_reopen() {
    // Arrange
    let mut fixture = DaemonFixture::new(40);
    // Act / Assert: every comparison uses a frozen epoch before any owner mutation.
    let catching = captured(&fixture).await;
    assert_eq!(catching["operational_state"]["value"], "catching_up");
    assert_eq!(catching["processed_height"]["value"], 7);
    assert_eq!(catching["initial_sync_complete"]["value"], false);
    assert_eq!(catching["retention"]["value"]["retained_record_count"], 8);
    fixture.finish();
    let current = captured(&fixture).await;
    assert_eq!(current["operational_state"]["value"], "current");
    assert_eq!(current["safe_durable_height"]["value"], 39);
    fixture.accept_next();
    let lagged = captured(&fixture).await;
    assert_eq!(lagged["operational_state"]["value"], "lagged");
    assert_eq!(lagged["initial_sync_complete"]["value"], true);
    assert_eq!(lagged["lag_blocks"]["value"], 1);
    fixture.finish();
    let ahead = captured(&fixture).await;
    assert_eq!(ahead["processed_height"]["value"], 40);
    assert_eq!(ahead["safe_durable_height"]["value"], 39);
    fixture
        .opened
        .network
        .flush_coins(FlushMode::Always, FlushPolicyTime::new(1), u64::MAX)
        .expect("actual flush");
    fixture.finish();
    let settled = captured(&fixture).await;
    let request = br#"{"jsonrpc":"2.0","id":161,"method":"openbitcoinnetworkstatus"}"#;
    let unauthorized = handle_http_request(
        &fixture.state,
        "/",
        Method::POST,
        &HeaderMap::new(),
        request,
    )
    .await;
    assert_eq!(unauthorized.status(), StatusCode::UNAUTHORIZED);
    assert!(
        to_bytes(unauthorized.into_body(), 4096)
            .await
            .expect("401 body")
            .is_empty()
    );
    let (path, blocks) = fixture.close();
    let fixture = DaemonFixture::open(path, blocks, true);
    assert_eq!(captured(&fixture).await, settled);
    fixture.cleanup();
}

#[tokio::test]
async fn phase161_operator_rpc_disabled_empty_and_raw_clone_invalidation() {
    use open_bitcoin_node::core::{chainstate::CoinsBatch, primitives::BlockHash};
    // Arrange: actual disabled startup may open a genuinely empty Fjall store.
    let history = super::fixtures::History::new(0);
    let fixture = DaemonFixture::open(history.path.clone(), Vec::new(), false);
    // Act / Assert
    let empty = captured(&fixture).await;
    assert_eq!(empty["operational_state"]["value"], "disabled");
    assert_eq!(empty["configured_indexing"]["value"], false);
    assert_eq!(empty["processed_height"]["state"], "unavailable");
    assert_eq!(empty["safe_durable_height"]["state"], "unavailable");
    drop(fixture.close());
    drop(history);
    let fixture = DaemonFixture::new(2);
    let before = captured(&fixture).await;
    assert_eq!(before["operational_state"]["value"], "current");
    let raw = fixture.store.clone();
    raw.coins_view()
        .batch_write_with_persist_mode(
            CoinsBatch {
                entries: Default::default(),
            },
            Some(BlockHash::from_byte_array([0x99; 32])),
            open_bitcoin_node::PersistMode::Sync,
        )
        .expect("actual raw clone fence mutation");
    assert!(
        fixture
            .opened
            .network
            .drive_basic_filter_index_turn()
            .is_err()
    );
    let invalid = captured(&fixture).await;
    assert_ne!(invalid["operational_state"]["value"], "current");
    assert_eq!(invalid["configured_indexing"]["value"], true);
    assert_eq!(invalid["initial_sync_complete"]["value"], true);
    assert_eq!(invalid["safe_durable_height"]["state"], "unavailable");
    assert_eq!(
        captured(&fixture).await,
        invalid,
        "status does not recover raw mutation"
    );
    drop(raw);
    fixture.cleanup();
}

#[tokio::test]
async fn phase161_operator_rpc_retained_pruned_stale_rows_and_reopen() {
    use open_bitcoin_node::core::{
        chainstate::{AnchoredBlock, PrunePlan},
        consensus::{ScriptVerifyFlags, block_hash, check_block_header},
    };
    // Arrange: the existing continuous maturity-one fixture permits a legal old-height prune.
    let mut fixture = DaemonFixture::new(401);
    let protected = captured(&fixture).await;
    let protected_hash = block_hash(&fixture.blocks[20].header);
    let protected_attempt = fixture
        .opened
        .network
        .flush_applying_prune_plan(
            FlushMode::Always,
            FlushPolicyTime::new(1),
            u64::MAX,
            &PrunePlan { heights: vec![20] },
            &[],
        )
        .expect("protected plan is safely filtered");
    assert!(
        protected_attempt.deleted_block_hashes.is_empty(),
        "BASIC-owned history is not deleted"
    );
    assert!(
        fixture
            .store
            .has_block(protected_hash)
            .expect("protected physical body survives")
    );
    assert_eq!(
        captured(&fixture).await,
        protected,
        "refused prune cannot fabricate index progress"
    );
    fixture.finish();
    let original = fixture.blocks.clone();
    let pruned = block_hash(&original[20].header);
    let before = captured(&fixture).await;
    // Act: actual serialized paired prune, then a same-height validated replacement.
    let deletion = fixture
        .opened
        .network
        .flush_applying_prune_plan(
            FlushMode::Always,
            FlushPolicyTime::new(2),
            u64::MAX,
            &PrunePlan { heights: vec![20] },
            &[],
        )
        .expect("legal manual owner prune");
    assert_eq!(deletion.deleted_block_hashes, vec![pruned]);
    assert_eq!(
        captured(&fixture).await,
        before,
        "payload prune does not subtract immutable filters"
    );
    let snapshot = fixture
        .opened
        .network
        .chainstate_snapshot()
        .expect("actual disconnect undo");
    let displaced = block_hash(&original[400].header);
    fixture
        .store
        .save_undo(
            displaced,
            &snapshot.undo_by_block[&displaced],
            open_bitcoin_node::PersistMode::Sync,
        )
        .expect("disconnect undo");
    let mut replacement = Vec::new();
    let mut parent = original[399].clone();
    for height in 400..=401 {
        let mut block = super::fixtures::daemon_block(Some(&parent), height);
        block.header.time += 1;
        block.header.nonce = (0..=u32::MAX)
            .find(|nonce| {
                block.header.nonce = *nonce;
                check_block_header(&block.header).is_ok()
            })
            .expect("replacement proof of work");
        fixture
            .store
            .save_block(&block, open_bitcoin_node::PersistMode::Sync)
            .expect("replacement body");
        replacement.push(AnchoredBlock {
            block: block.clone(),
            chain_work: open_bitcoin_node::core::consensus::chain_proof::ProofWork::ZERO,
        });
        parent = block;
    }
    fixture
        .opened
        .network
        .reorg_to_branch(
            &[original[400].clone()],
            &replacement,
            open_bitcoin_mempool::ReorgLifecycleContext::new(
                open_bitcoin_mempool::PolicyTime::from_unix_seconds(50_000),
            ),
            ScriptVerifyFlags::P2SH,
            super::fixtures::params(),
        )
        .expect("actual strictly stronger branch replacement");
    fixture.blocks.truncate(400);
    fixture
        .blocks
        .extend(replacement.iter().map(|row| row.block.clone()));
    let replaced = captured(&fixture).await;
    assert_eq!(replaced["initial_sync_complete"]["value"], true);
    assert_eq!(replaced["lag_blocks"]["value"], replacement.len());
    fixture
        .opened
        .network
        .flush_coins(FlushMode::Always, FlushPolicyTime::new(3), u64::MAX)
        .expect("actual replacement fence");
    fixture.finish();
    let settled = captured(&fixture).await;
    // Assert: independent enumeration of every known actual immutable query row.
    let mut payload_bytes = 0_u64;
    for block in original
        .iter()
        .chain(replacement.iter().map(|row| &row.block))
    {
        let open_bitcoin_node::BasicFilterQuery::Found(row) = fixture
            .opened
            .network
            .basic_filter_query(block_hash(&block.header))
            .expect("actual retained query")
        else {
            panic!("every original and displaced row retained")
        };
        assert!(row.work().record_reads <= 2);
        payload_bytes += row.encoded_bytes().len() as u64;
    }
    let measurement = &settled["retention"]["value"];
    let retained = original.len() + replacement.len();
    assert_eq!(measurement["retained_record_count"], retained);
    assert_eq!(measurement["encoded_filter_payload_bytes"], payload_bytes);
    // v1 persisted envelope is 170 bytes; actual record keys are prefix + 64 hex bytes.
    let key_bytes = "basic_filter:v1:record:".len() as u64 + 64;
    assert_eq!(
        measurement["retained_record_key_value_bytes"],
        payload_bytes + retained as u64 * (170 + key_bytes)
    );
    assert_eq!(fixture.store.load_block(pruned).expect("pruned body"), None);
    assert_eq!(fixture.store.load_undo(pruned).expect("pruned undo"), None);
    assert!(fixture.store.load_have_pruned().expect("earned prune bit"));
    let (path, blocks) = fixture.close();
    let fixture = DaemonFixture::open(path, blocks, true);
    assert_eq!(captured(&fixture).await, settled);
    assert_eq!(
        fixture
            .store
            .load_block(pruned)
            .expect("reopened body absence"),
        None
    );
    assert_eq!(
        fixture
            .store
            .load_undo(pruned)
            .expect("reopened undo absence"),
        None
    );
    eprintln!(
        "phase161 operator RPC retained evidence: accepted=401 replacement=2 immutable_rows={retained} payload_bytes={payload_bytes} key_value_bytes={} legal_manual_prune_height=20 all_handles_closed=true",
        payload_bytes + retained as u64 * (170 + key_bytes)
    );
    fixture.cleanup();
}
