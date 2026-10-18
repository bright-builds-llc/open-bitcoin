// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

// Public-only source assembly included by mature RPC and daemon test modules.

use open_bitcoin_network::{
    BlockRelayActivationPolicy, HeadersMessage, RelayActivationConfig, VersionMessage,
    WireNetworkMessage,
};
use open_bitcoin_node::{
    DurableSyncRuntime, FjallChainstateStore, FjallNodeStore, ManagedChainstate, SyncRuntimeConfig,
    chainstate::{BasicFilterStartupMode, FlushLifecycle},
    core::{
        chainstate::{ChainPosition, Chainstate, CoinsView, FlushMode, FlushPolicyTime},
        consensus::{ConsensusParams, ScriptVerifyFlags, block_hash, chain_proof::ProofWork},
        primitives::{Block, BlockHash},
    },
};
use std::path::Path;

pub(crate) fn live_genesis_runtime(
    path: &Path,
    genesis: &Block,
    config: SyncRuntimeConfig,
    fullparams: ConsensusParams,
    flags: ScriptVerifyFlags,
    mode: BasicFilterStartupMode,
) -> (DurableSyncRuntime, ChainPosition) {
    let store = FjallNodeStore::open(path).expect("empty concrete source store");
    let mut core = Chainstate::from_parent(store.coins_view(), vec![], Default::default(), None);
    Chainstate::new()
        .rebind_into(&mut core)
        .expect("genuine empty source transfer");
    let now = FlushPolicyTime::from_unix_seconds(0);
    let mut source = ManagedChainstate::from_chainstate(
        FjallChainstateStore::from_store(store.clone()),
        core,
        FlushLifecycle::ready(now, now, 0, false),
    );
    let receipt = source
        .connect_block(genesis, ProofWork::ZERO, flags, fullparams)
        .expect("actual managed genesis receipt");
    let (mut sink, mut same_core) = source.into_parts();
    let positions = same_core.active_chain().to_vec();
    let undo = same_core
        .undo_by_block()
        .iter()
        .map(|(hash, undo)| (*hash, undo.clone()))
        .collect::<Vec<_>>();
    let mut lifecycle = FlushLifecycle::ready(now, now, 0, false);
    assert!(
        lifecycle
            .execute_flush(
                &mut sink,
                &mut same_core,
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(2000),
                u64::MAX,
                &undo,
                &[],
                std::slice::from_ref(genesis),
                &positions,
            )
            .expect("actual genesis source flush")
            .wrote_coins
    );
    assert_eq!(
        store.coins_view().best_block().expect("durable genesis B"),
        Some(receipt.block_hash)
    );
    let runtime = DurableSyncRuntime::from_live_fixture_source(
        ManagedChainstate::from_chainstate(sink, same_core, lifecycle),
        config,
        fullparams,
        mode,
        RelayActivationConfig::default(),
        BlockRelayActivationPolicy::default(),
        false,
    )
    .expect("same genuine public fixture source");
    let headers = runtime
        .network_handle()
        .header_entries()
        .expect("captured root graph");
    assert_eq!(headers.len(), 1);
    assert_eq!(headers[0].header, genesis.header);
    (runtime, receipt)
}

pub(crate) fn admit_descendant_headers(
    runtime: &DurableSyncRuntime,
    blocks: &[Block],
    flags: ScriptVerifyFlags,
    fullparams: ConsensusParams,
) {
    let Some(first) = blocks.first() else {
        return;
    };
    assert_ne!(first.header.previous_block_hash, BlockHash::default());
    let network = runtime.network_handle();
    let before = network
        .header_entries()
        .expect("already bound source graph");
    assert!(
        before
            .iter()
            .any(|entry| entry.block_hash == first.header.previous_block_hash)
    );
    assert!(blocks.iter().all(|block| {
        !before
            .iter()
            .any(|entry| entry.block_hash == block_hash(&block.header))
    }));
    assert!(
        blocks
            .windows(2)
            .all(|pair| pair[1].header.previous_block_hash == block_hash(&pair[0].header))
    );
    let now = blocks
        .iter()
        .map(|block| i64::from(block.header.time))
        .max()
        .expect("nonempty header batch");
    let peer = 165_019;
    network
        .connect_outbound_peer(peer, now)
        .expect("modeled descendant peer");
    for message in [
        WireNetworkMessage::Version(VersionMessage {
            nonce: peer,
            start_height: 1100,
            ..Default::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: blocks.iter().map(|block| block.header.clone()).collect(),
        }),
    ] {
        network
            .receive_sync_message(peer, message, now, flags, fullparams)
            .expect("normal new descendant ingress");
    }
    let after = network.header_entries().expect("accepted descendant graph");
    for block in blocks {
        assert!(
            after
                .iter()
                .any(|entry| entry.block_hash == block_hash(&block.header)
                    && entry.header == block.header)
        );
    }
    network
        .disconnect_peer(peer)
        .expect("modeled peer released");
}
