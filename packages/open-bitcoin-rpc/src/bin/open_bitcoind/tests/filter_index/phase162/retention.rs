// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Actual planner/paired loss and configured reopen; pressure is never accepted history.

use super::{core_types, fixtures::Phase162Fixture, lifecycle};
use axum::{
    body::to_bytes,
    http::{HeaderMap, HeaderValue, Method, StatusCode},
};
use base64::Engine;
use core_types::{
    chainstate::{ChainstateSnapshot, CoinsView, FilterCheckpoint},
    codec::encode_block,
    consensus::{block_hash, block_merkle_root, crypto::double_sha256},
    primitives::{Amount, BlockHash, ScriptBuf, TransactionOutput},
};
use open_bitcoin_node::{BasicFilterRecordView, FjallNodeStore, PersistMode, SyncNetwork};
use serde_json::{Value, json};
use std::{path::Path, time::Instant};

pub(crate) async fn invoke(fixture: &Phase162Fixture, method: &str, params: Value) -> Value {
    let mut headers = HeaderMap::new();
    let credentials =
        base64::engine::general_purpose::STANDARD.encode("phase162:private-test-password");
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Basic {credentials}")).expect("auth header"),
    );
    let request = json!({"jsonrpc":"2.0", "id":"phase162", "method":method, "params":params});
    let response = open_bitcoin_rpc::http::handle_http_request(
        &fixture.state,
        "/",
        Method::POST,
        &headers,
        request.to_string().as_bytes(),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    serde_json::from_slice(
        &to_bytes(response.into_body(), 1024 * 1024)
            .await
            .expect("bounded RPC body"),
    )
    .expect("RPC JSON")
}

pub(crate) async fn protected_request(fixture: &Phase162Fixture) {
    assert_eq!(SyncNetwork::Regtest.prune_after_height(), 1000);
    let saved = fixture
        .store
        .load_prune_protection()
        .expect("partial safe identity");
    assert!(
        fixture
            .store
            .load_prune_protection()
            .expect("partial protection")
            .protects_height(20)
    );
    let response = invoke(fixture, "pruneblockchain", json!([20])).await;
    assert!(response["error"].is_null(), "{response}");
    // The shipped caller can delete an eligible prefix; required input is the invariant.
    for height in 8..=20 {
        assert_pair(
            fixture,
            fixture.provenance.receipts[height].block_hash,
            true,
        );
    }
    assert_eq!(
        fixture
            .store
            .load_prune_protection()
            .expect("preserved required-input protection"),
        saved
    );
    eprintln!(
        "phase162 protected manual request: requested=20 actual_return={} required8..20_retained=true",
        response["result"]
    );
}

pub(crate) struct SettledIdentity {
    // Undo rows are the deliberately deleted payload, not settled chain/coins identity.
    metadata_and_coins: ChainstateSnapshot,
    checkpoint: FilterCheckpoint,
    records: Vec<BasicFilterRecordView>,
}

impl SettledIdentity {
    pub fn capture(fixture: &Phase162Fixture, stale: &[BasicFilterRecordView]) -> Self {
        let mut snapshot = fixture
            .store
            .hydrate_chainstate_for_open()
            .expect("durable metadata/coins")
            .expect("settled state");
        let mut records: Vec<_> = snapshot
            .active_chain
            .iter()
            .map(|position| lifecycle::record(fixture, position.block_hash))
            .collect();
        records.extend_from_slice(stale);
        assert_eq!(snapshot.active_chain.len(), 1003);
        assert_eq!(records.len(), 1006);
        assert_eq!(
            fixture.store.coins_view().best_block().expect("coins B"),
            snapshot
                .active_chain
                .last()
                .map(|position| position.block_hash)
        );
        assert!(
            fixture
                .store
                .coins_view()
                .head_blocks()
                .expect("coins heads")
                .is_empty()
        );
        snapshot.undo_by_block.clear();
        let checkpoint = fixture
            .store
            .load_prune_protection()
            .expect("settled protection")
            .maybe_owner()
            .expect("owner")
            .checkpoint()
            .checkpoint();
        Self {
            metadata_and_coins: snapshot,
            checkpoint,
            records,
        }
    }

    pub fn assert_unchanged(&self, fixture: &Phase162Fixture) {
        let mut snapshot = fixture
            .store
            .hydrate_chainstate_for_open()
            .expect("actual reopened identity")
            .expect("durable state");
        snapshot.undo_by_block.clear();
        assert_eq!(snapshot, self.metadata_and_coins);
        assert_eq!(
            fixture
                .store
                .coins_view()
                .best_block()
                .expect("exact coins B"),
            snapshot
                .active_chain
                .last()
                .map(|position| position.block_hash)
        );
        assert!(
            fixture
                .store
                .coins_view()
                .head_blocks()
                .expect("settled heads")
                .is_empty()
        );
        assert_eq!(
            fixture
                .store
                .load_prune_protection()
                .expect("safe identity")
                .maybe_owner()
                .expect("owner")
                .checkpoint()
                .checkpoint(),
            self.checkpoint
        );
        for original in &self.records {
            let retained = lifecycle::record(fixture, original.identity().block_hash());
            assert_eq!(retained.identity(), original.identity());
            assert_eq!(retained.encoded_bytes(), original.encoded_bytes());
        }
    }
}

pub(crate) fn assert_pair(fixture: &Phase162Fixture, hash: BlockHash, present: bool) {
    assert_eq!(
        fixture
            .store
            .load_block(hash)
            .expect("real body read")
            .is_some(),
        present
    );
    assert_eq!(
        fixture
            .store
            .load_undo(hash)
            .expect("real undo read")
            .is_some(),
        present
    );
}

fn usage(fixture: &Phase162Fixture) -> u64 {
    let active = fixture
        .opened
        .network
        .chainstate_snapshot()
        .expect("accepted active chain")
        .active_chain;
    fixture
        .store
        .retained_payload_usage(&active)
        .expect("actual all-key usage")
        .current_usage_bytes
}

pub(crate) async fn manual(fixture: &Phase162Fixture, settled: &SettledIdentity) {
    let before = usage(fixture);
    let refused = invoke(fixture, "pruneblockchain", json!([1000])).await;
    assert_eq!(refused["error"]["code"], -8);
    assert!(
        refused["error"]["message"]
            .as_str()
            .expect("refusal")
            .contains("288-block keep window")
    );
    assert_eq!(usage(fixture), before);
    let response = invoke(fixture, "pruneblockchain", json!([20])).await;
    assert!(response["error"].is_null(), "{response}");
    assert_eq!(response["result"], 20);
    for position in &fixture.provenance.receipts[..=20] {
        assert_pair(fixture, position.block_hash, false);
    }
    for position in &settled.metadata_and_coins.active_chain[21..] {
        assert_pair(fixture, position.block_hash, true);
    }
    assert!(fixture.store.load_have_pruned().expect("earned prune bit"));
    settled.assert_unchanged(fixture);
    let after = usage(fixture);
    assert!(after < before);
    eprintln!(
        "phase162 manual retention: target=20 before={before} after={after} paired0..20_absent=true keep_window_refused=true records=1006"
    );
}

type BodyCommitments = Vec<(BlockHash, usize, [u8; 32])>;

/// Stream known auxiliary keys; no raw namespace reader or duplicate payload cache.
fn pressure_commitments(fixture: &Phase162Fixture, hashes: &[BlockHash]) -> BodyCommitments {
    hashes
        .iter()
        .map(|hash| {
            let body = fixture
                .store
                .load_block(*hash)
                .expect("auxiliary read")
                .expect("nonactive body retained");
            assert_eq!(block_hash(&body.header), *hash);
            let bytes = encode_block(&body).expect("exact canonical re-encoding");
            (*hash, bytes.len(), double_sha256(&bytes))
        })
        .collect()
}

fn add_pressure(fixture: &Phase162Fixture) -> Vec<BlockHash> {
    let mut body = fixture.recipe.blocks[1000].clone();
    let output = TransactionOutput {
        value: Amount::ZERO,
        script_pubkey: ScriptBuf::from_bytes(vec![0x61; 9985]).expect("bounded codec-only script"),
    };
    body.transactions[0]
        .outputs
        .extend(std::iter::repeat_n(output, 99));
    body.header.merkle_root = block_merkle_root(&body.transactions)
        .expect("codec-only merkle")
        .0;
    assert!((900_000..1_100_000).contains(&encode_block(&body).expect("codec body").len()));
    let active = fixture
        .opened
        .network
        .chainstate_snapshot()
        .expect("actual candidates")
        .active_chain;
    let mut hashes = Vec::with_capacity(584);
    for nonce in 100_000..100_584 {
        body.header.nonce = nonce;
        let hash = fixture
            .store
            .save_block(&body, PersistMode::Buffered)
            .expect("nonactive storage pressure");
        assert!(active.iter().all(|position| position.block_hash != hash));
        assert!(!hashes.contains(&hash));
        hashes.push(hash);
    }
    hashes
}

pub(crate) fn automatic(fixture: Phase162Fixture, settled: &SettledIdentity) -> Phase162Fixture {
    assert!(
        FjallNodeStore::open(&fixture.path).is_err(),
        "live owner lock refusal"
    );
    let fixture = Phase162Fixture::open_with_prune(fixture.close(), 550);
    settled.assert_unchanged(&fixture);
    let started = Instant::now();
    let hashes = add_pressure(&fixture);
    let pressure = pressure_commitments(&fixture, &hashes);
    let pressure_bytes: usize = pressure.iter().map(|(_, len, _)| len).sum();
    let before = usage(&fixture);
    assert!(
        before > 550 * 1024 * 1024,
        "actual usage must earn target pressure"
    );
    let deletion = fixture
        .opened
        .network
        .flush_coins(
            core_types::chainstate::FlushMode::Always,
            core_types::chainstate::FlushPolicyTime::new(500_004),
            u64::MAX,
        )
        .expect("ordinary automatic owner");
    assert!(deletion.wrote_coins);
    assert!(!deletion.deleted_block_hashes.is_empty());
    assert_eq!(deletion.deleted_block_hashes.len(), 694);
    for position in &settled.metadata_and_coins.active_chain[21..=714] {
        assert!(deletion.deleted_block_hashes.contains(&position.block_hash));
        assert_pair(&fixture, position.block_hash, false);
    }
    for position in &settled.metadata_and_coins.active_chain[715..] {
        assert_pair(&fixture, position.block_hash, true);
    }
    assert_eq!(pressure_commitments(&fixture, &hashes), pressure);
    let after = usage(&fixture);
    assert!(after < before);
    assert!(
        after > 550 * 1024 * 1024,
        "ineligible auxiliary rows can exceed the soft target"
    );
    settled.assert_unchanged(&fixture);
    let footprint = allocated_bytes(&fixture.path);
    assert!(
        FjallNodeStore::open(&fixture.path).is_err(),
        "all live configured handles lock the datadir"
    );
    let fixture = Phase162Fixture::open_with_prune(fixture.close(), 550);
    settled.assert_unchanged(&fixture);
    assert_eq!(
        usage(&fixture),
        after,
        "configured reopen does not invent further deletion"
    );
    assert_eq!(pressure_commitments(&fixture, &hashes), pressure);
    for position in &settled.metadata_and_coins.active_chain[..=714] {
        assert_pair(&fixture, position.block_hash, false);
    }
    for position in &settled.metadata_and_coins.active_chain[715..] {
        assert_pair(&fixture, position.block_hash, true);
    }
    assert!(
        fixture
            .store
            .load_have_pruned()
            .expect("reopened prune bit")
    );
    eprintln!(
        "phase162 automatic retention: configured_target_mib=550 nonactive_codec_rows={} pressure_value_bytes={pressure_bytes} before={before} after={after} automatic_pairs={} last_deleted=714 retained_from=715 allocated_file_bytes={footprint} pressure_apply_reopen_ms={} same_path=true records=1006 target_attained=false",
        hashes.len(),
        deletion.deleted_block_hashes.len(),
        started.elapsed().as_millis()
    );
    fixture
}

pub(crate) fn allocated_bytes(path: &Path) -> u64 {
    use std::os::unix::fs::MetadataExt;
    std::fs::read_dir(path)
        .expect("fixture directory")
        .map(|entry| {
            let entry = entry.expect("fixture entry");
            let metadata = entry.metadata().expect("fixture metadata");
            if metadata.is_dir() {
                allocated_bytes(&entry.path())
            } else {
                metadata.blocks() * 512
            }
        })
        .sum()
}
