// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Public fixture assembly against the normally compiled Node library.

#[path = "phase165_live_fixture/requalification.rs"]
mod requalification;

use open_bitcoin_mempool::{PolicyTime, ReorgLifecycleContext};
use open_bitcoin_network::{
    BlockRelayActivationPolicy, HeaderStore, HeadersMessage, PeerId, RelayActivationConfig,
    VersionMessage, WireNetworkMessage,
};
use open_bitcoin_node::{
    DurableSyncRuntime, FjallChainstateStore, FjallCoinsView, FjallNodeStore, ManagedChainstate,
    SyncNetwork, SyncRuntimeConfig, SyncRuntimeError,
    chainstate::{BasicFilterStartupMode, FlushLifecycle},
    core::{
        chainstate::{
            AnchoredBlock, Chainstate, CoinsBatch, CoinsCacheEntry, CoinsView, FlushMode,
            FlushPolicyTime,
        },
        consensus::{
            CanonicalChain, ConsensusParams, ScriptVerifyFlags, block_hash, block_merkle_root,
            chain_proof::ProofWork, check_block_header,
        },
        primitives::{
            Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
            TransactionInput, TransactionOutput, Txid,
        },
    },
};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

type Source = ManagedChainstate<FjallChainstateStore, FjallCoinsView>;

struct FixtureDir(PathBuf);
impl FixtureDir {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        Self(std::env::temp_dir().join(format!(
                "phase165-public-{}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("clock")
                    .as_nanos(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            )))
    }
}
impl Drop for FixtureDir {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).expect("remove closed fixture");
    }
}

fn params() -> ConsensusParams {
    ConsensusParams {
        coinbase_maturity: 1,
        ..Default::default()
    }
}
fn config() -> SyncRuntimeConfig {
    SyncRuntimeConfig {
        network: SyncNetwork::Regtest,
        ..Default::default()
    }
}
fn lifecycle() -> FlushLifecycle {
    let now = FlushPolicyTime::from_unix_seconds(0);
    FlushLifecycle::ready(now, now, 0, false)
}
fn genuine(
    directory: &FixtureDir,
    blocks: &[Block],
    selected: ConsensusParams,
) -> (FjallNodeStore, Source) {
    let store = FjallNodeStore::open(&directory.0).expect("open fixture");
    let mut core = Chainstate::from_parent(store.coins_view(), vec![], HashMap::new(), None);
    Chainstate::new()
        .rebind_into(&mut core)
        .expect("genuine empty transfer");
    let mut managed = ManagedChainstate::from_chainstate(
        FjallChainstateStore::from_store(store.clone()),
        core,
        lifecycle(),
    );
    for block in blocks {
        managed
            .connect_block(block, ProofWork::ZERO, ScriptVerifyFlags::P2SH, selected)
            .expect("actual managed acceptance before extraction");
    }
    if blocks.is_empty() {
        return (store, managed);
    }
    let (mut sink, mut same_core) = managed.into_parts();
    let positions = same_core.active_chain().to_vec();
    let undo = same_core
        .undo_by_block()
        .iter()
        .map(|(hash, undo)| (*hash, undo.clone()))
        .collect::<Vec<_>>();
    let mut flush = lifecycle();
    flush
        .execute_flush(
            &mut sink,
            &mut same_core,
            FlushMode::Always,
            FlushPolicyTime::from_unix_seconds(1),
            u64::MAX,
            &undo,
            &[],
            blocks,
            &positions,
        )
        .expect("real flush without raw headers");
    same_core.validate_live_origin().expect("same genuine core");
    same_core
        .bind_live_work_source(selected)
        .expect("full selected params");
    (
        store,
        ManagedChainstate::from_chainstate(sink, same_core, flush),
    )
}
fn open(
    source: Source,
    selected: ConsensusParams,
    mode: BasicFilterStartupMode,
) -> Result<DurableSyncRuntime, SyncRuntimeError> {
    DurableSyncRuntime::from_live_fixture_source(
        source,
        config(),
        selected,
        mode,
        RelayActivationConfig::default(),
        BlockRelayActivationPolicy::default(),
        false,
    )
}
fn block(parent: BlockHash, height: u32, offset: u32) -> Block {
    let tx = Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(if height == 0 {
                vec![0, 0x51]
            } else {
                vec![1, height as u8, 0x51]
            })
            .expect("coinbase script"),
            sequence: u32::MAX,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(50).expect("amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("script"),
        }],
        lock_time: 0,
    };
    let mut result = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: parent,
            merkle_root: block_merkle_root(std::slice::from_ref(&tx))
                .expect("merkle")
                .0,
            time: 1_231_006_500 + height + offset,
            bits: 0x207f_ffff,
            nonce: 0,
        },
        transactions: vec![tx],
    };
    result.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            result.header.nonce = *nonce;
            check_block_header(&result.header).is_ok()
        })
        .expect("mine easy header");
    result
}
fn ingress(runtime: &DurableSyncRuntime, batches: &[Vec<Block>]) {
    let handle = runtime.network_handle();
    let peer: PeerId = 70;
    let now = 2_000_000_000;
    handle
        .connect_outbound_peer(peer, now)
        .expect("modeled peer");
    for message in [
        WireNetworkMessage::Version(VersionMessage::default()),
        WireNetworkMessage::Verack,
    ] {
        handle
            .receive_sync_message(peer, message, now, ScriptVerifyFlags::P2SH, params())
            .expect("handshake");
    }
    for batch in batches {
        handle
            .receive_sync_message(
                peer,
                WireNetworkMessage::Headers(HeadersMessage {
                    headers: batch.iter().map(|block| block.header.clone()).collect(),
                }),
                now,
                ScriptVerifyFlags::P2SH,
                params(),
            )
            .expect("normal future headers");
    }
    handle
        .disconnect_peer(peer)
        .expect("disconnect modeled peer");
}

#[test]
fn phase165_live_fixture_empty_source_accepts_first_actual_body() {
    // Arrange
    let dir = FixtureDir::new();
    let (store, source) = genuine(&dir, &[], params());
    let root = block(BlockHash::default(), 0, 0);
    let runtime = open(source, params(), BasicFilterStartupMode::Disabled).expect("runtime");
    assert!(
        runtime
            .snapshot_summary()
            .maybe_connected_block_hash
            .is_none()
    );
    let handle = runtime.network_handle();
    let mut headers = HeaderStore::default();
    headers
        .insert_header_with_params(root.header.clone(), &params())
        .expect("root header");
    handle
        .try_admit_fixture_headers(headers, 2_000_000_000, params())
        .expect("explicit root");
    // Act
    let accepted = handle
        .connect_local_block(&root, ScriptVerifyFlags::P2SH, params())
        .expect("actual first connection");
    // Assert
    assert_eq!(accepted.block_hash, block_hash(&root.header));
    assert_eq!(accepted.chain_work.to_le_bytes()[0], 2);
    drop((handle, runtime, store));
}

#[test]
fn phase165_live_fixture_populated_basic_keeps_params_and_known_counts() {
    // Arrange
    let dir = FixtureDir::new();
    let root = block(BlockHash::default(), 0, 0);
    let child = block(block_hash(&root.header), 1, 0);
    let (store, source) = genuine(&dir, std::slice::from_ref(&root), params());
    let before = source.chainstate().overlay_snapshot();
    // Act
    let runtime = open(source, params(), BasicFilterStartupMode::Enabled).expect("normal BASIC");
    let handle = runtime.network_handle();
    // Assert
    let actual = handle.chainstate_snapshot().expect("same source");
    assert_eq!(actual.active_chain, before.active_chain);
    assert_eq!(
        actual.maybe_confirmed_txid_counts,
        before.maybe_confirmed_txid_counts
    );
    handle
        .connect_local_block(&child, ScriptVerifyFlags::P2SH, params())
        .expect("explicit maturity-one connection");
    assert_eq!(
        handle
            .maybe_chain_tip()
            .expect("tip")
            .expect("active")
            .height,
        1
    );
    drop((handle, runtime, store));
}

#[test]
fn phase165_live_fixture_raw_replayable_source_refuses_origin() {
    // Arrange
    let dir = FixtureDir::new();
    let root = block(BlockHash::default(), 0, 0);
    let (store, source) = genuine(&dir, &[root], params());
    let (sink, core) = source.into_parts();
    let snapshot = core.overlay_snapshot();
    let raw = Chainstate::from_parent(
        store.coins_view(),
        snapshot.active_chain,
        snapshot.undo_by_block,
        snapshot.maybe_confirmed_txid_counts,
    );
    let before = store.coins_view().complete_snapshot().expect("physical");
    // Act
    let result = open(
        ManagedChainstate::from_chainstate(sink, raw, lifecycle()),
        params(),
        BasicFilterStartupMode::Disabled,
    );
    // Assert
    assert!(
        result
            .err()
            .expect("raw refusal")
            .to_string()
            .contains("lineage")
    );
    assert_eq!(
        store.coins_view().complete_snapshot().expect("physical"),
        before
    );
}

#[test]
fn phase165_live_fixture_wrong_full_parameters_refuse() {
    // Arrange
    let dir = FixtureDir::new();
    let (store, source) = genuine(&dir, &[block(BlockHash::default(), 0, 0)], params());
    let wrong = ConsensusParams {
        coinbase_maturity: 100,
        ..params()
    };
    let before = store.coins_view().complete_snapshot().expect("physical");
    // Act
    let result = open(source, wrong, BasicFilterStartupMode::Disabled);
    // Assert
    assert!(result.is_err());
    assert_eq!(
        store.coins_view().complete_snapshot().expect("physical"),
        before
    );
}

fn alias_poison(kind: u8) {
    let dir = FixtureDir::new();
    let (store, source) = genuine(&dir, &[block(BlockHash::default(), 0, 0)], params());
    let before = source
        .chainstate()
        .coins()
        .complete_snapshot()
        .expect("clean source");
    let (outpoint, coin) = before.coins.iter().next().expect("actual coin");
    let mut changed = coin.clone();
    changed.created_height += 1;
    let (key, entry) = match kind {
        0 => (
            OutPoint {
                txid: Txid::from_byte_array([99; 32]),
                vout: 0,
            },
            CoinsCacheEntry::unspent_dirty(changed),
        ),
        1 => (outpoint.clone(), CoinsCacheEntry::spent_dirty()),
        _ => (outpoint.clone(), CoinsCacheEntry::unspent_dirty(changed)),
    };
    store
        .coins_view()
        .batch_write(
            CoinsBatch {
                entries: HashMap::from([(key, entry)]),
            },
            before.maybe_best_block,
        )
        .expect("ordinary public aliased writer with SAME B");
    source
        .chainstate()
        .validate_live_origin()
        .expect("origin remains Live");
    source
        .chainstate()
        .bind_live_work_source(params())
        .expect("binding still valid");
    let poisoned = store
        .coins_view()
        .complete_snapshot()
        .expect("poisoned physical");
    assert_ne!(before, poisoned);
    assert_eq!(
        source
            .chainstate()
            .coins()
            .complete_snapshot()
            .expect("aliased source"),
        poisoned
    );
    let result = open(source, params(), BasicFilterStartupMode::Disabled);
    assert!(
        result
            .err()
            .expect("refuse independent mismatch")
            .to_string()
            .contains("independent fixture physical identity mismatch")
    );
    assert_eq!(
        store
            .coins_view()
            .complete_snapshot()
            .expect("unchanged poison"),
        poisoned
    );
}
#[test]
fn phase165_live_fixture_public_alias_extra_coin_refuses() {
    alias_poison(0);
}
#[test]
fn phase165_live_fixture_public_alias_missing_coin_refuses() {
    alias_poison(1);
}
#[test]
fn phase165_live_fixture_public_alias_changed_coin_refuses() {
    alias_poison(2);
}

fn canonical_refuses(chain: CanonicalChain) {
    let dir = FixtureDir::new();
    let genesis = chain.spec().genesis_block();
    let selected = ConsensusParams {
        pow_limit_bits: genesis.header.bits,
        enforce_bip34_height_in_coinbase: false,
        ..Default::default()
    };
    let (store, source) = genuine(&dir, &[genesis], selected);
    source
        .chainstate()
        .validate_live_origin()
        .expect("actual genuine canonical source");
    source
        .chainstate()
        .bind_live_work_source(selected)
        .expect("same canonical test params");
    let before = store.coins_view().complete_snapshot().expect("physical");
    let headers = store.load_header_store().expect("saved headers");
    let pruned = store.load_have_pruned().expect("prune state");
    let result = open(source, selected, BasicFilterStartupMode::Enabled);
    assert!(
        matches!(result, Err(SyncRuntimeError::InvalidData { ref message })
        if message == "canonical root is not a live fixture source")
    );
    assert_eq!(
        store.coins_view().complete_snapshot().expect("physical"),
        before
    );
    assert_eq!(
        format!("{:?}", store.load_header_store().expect("headers")),
        format!("{headers:?}")
    );
    assert_eq!(store.load_have_pruned().expect("pruned"), pruned);
}
#[test]
fn phase165_live_fixture_canonical_mainnet_refuses_specifically() {
    canonical_refuses(CanonicalChain::Mainnet);
}
#[test]
fn phase165_live_fixture_canonical_testnet3_refuses_specifically() {
    canonical_refuses(CanonicalChain::Testnet3);
}
#[test]
fn phase165_live_fixture_canonical_signet_refuses_specifically() {
    canonical_refuses(CanonicalChain::Signet);
}
#[test]
fn phase165_live_fixture_canonical_regtest_refuses_specifically() {
    canonical_refuses(CanonicalChain::Regtest);
}

#[test]
fn phase165_live_fixture_populated_graph_extends_but_unknown_equal_refuses() {
    // Arrange
    let dir = FixtureDir::new();
    let root = block(BlockHash::default(), 0, 0);
    let active = block(block_hash(&root.header), 1, 0);
    let child = block(block_hash(&active.header), 2, 0);
    let side = block(block_hash(&active.header), 2, 100);
    let (store, source) = genuine(&dir, &[root, active], params());
    let runtime =
        open(source, params(), BasicFilterStartupMode::Disabled).expect("populated graph");
    ingress(&runtime, &[vec![child.clone()], vec![side.clone()]]);
    let handle = runtime.network_handle();
    // Act
    handle
        .connect_local_block(&child, ScriptVerifyFlags::P2SH, params())
        .expect("strict extension");
    let before = handle.chainstate_snapshot().expect("before refusal");
    let result = handle.reorg_to_branch(
        &[child],
        &[AnchoredBlock {
            block: side,
            chain_work: ProofWork::ZERO,
        }],
        ReorgLifecycleContext::new(PolicyTime::from_unix_seconds(2_000_000_000)),
        ScriptVerifyFlags::P2SH,
        params(),
    );
    // Assert
    assert!(
        result.is_err(),
        "unknown historical order cannot earn equal preference"
    );
    assert_eq!(handle.chainstate_snapshot().expect("unchanged"), before);
    drop((handle, runtime, store));
}

#[test]
fn phase165_live_fixture_public_pruned_prefix_refuses() {
    // Arrange
    let dir = FixtureDir::new();
    let root = block(BlockHash::default(), 0, 0);
    let hash = block_hash(&root.header);
    let (store, source) = genuine(&dir, &[root], params());
    open_bitcoin_node::chainstate::FlushPersistSink::commit_paired_unlink(
        &mut FjallChainstateStore::from_store(store.clone()),
        0,
        hash,
    )
    .expect("actual public sink prune");
    let before = store.coins_view().complete_snapshot().expect("physical");
    // Act
    let result = open(source, params(), BasicFilterStartupMode::Disabled);
    // Assert
    assert!(
        result
            .err()
            .expect("missing prefix refusal")
            .to_string()
            .contains("missing retained fixture active body")
    );
    assert!(!store.has_block(hash).expect("no body restoration"));
    assert!(!store.has_undo(hash).expect("no undo restoration"));
    assert_eq!(
        store.coins_view().complete_snapshot().expect("physical"),
        before
    );
}

#[test]
fn phase165_live_fixture_public_changed_undo_refuses() {
    // Arrange
    let dir = FixtureDir::new();
    let root = block(BlockHash::default(), 0, 0);
    let hash = block_hash(&root.header);
    let (store, source) = genuine(&dir, &[root], params());
    let changed = open_bitcoin_node::core::chainstate::BlockUndo {
        transactions: vec![open_bitcoin_node::core::chainstate::TxUndo::default()],
    };
    store
        .save_undo(hash, &changed, open_bitcoin_node::PersistMode::Sync)
        .expect("ordinary undo writer");
    // Act
    let result = open(source, params(), BasicFilterStartupMode::Disabled);
    // Assert
    assert!(
        result
            .err()
            .expect("changed undo refusal")
            .to_string()
            .contains("independent fixture physical")
    );
    assert_eq!(
        store.load_undo(hash).expect("unchanged physical undo"),
        Some(changed)
    );
}
