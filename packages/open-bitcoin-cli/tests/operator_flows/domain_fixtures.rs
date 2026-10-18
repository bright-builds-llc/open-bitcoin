// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/rpc/client.cpp
// - packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py

use super::*;

pub(super) fn durable_basic_runtime(
    data_dir: &Path,
    count: u32,
) -> (open_bitcoin_node::DurableSyncRuntime, Vec<Block>) {
    durable_basic_runtime_with_policy(data_dir, count, Default::default(), Default::default())
}

pub(super) fn durable_basic_runtime_with_policy(
    data_dir: &Path,
    count: u32,
    relay: open_bitcoin_network::RelayActivationConfig,
    block_relay: open_bitcoin_network::BlockRelayActivationPolicy,
) -> (open_bitcoin_node::DurableSyncRuntime, Vec<Block>) {
    use open_bitcoin_node::PersistMode;
    use open_bitcoin_node::core::{
        chainstate::{FlushMode, FlushPolicyTime},
        consensus::ScriptVerifyFlags,
    };
    let mut blocks: Vec<Block> = Vec::new();
    for height in 0..count {
        let transactions = if height > 1 {
            let previous = blocks.last().expect("funding history");
            let first = spend_to_script(
                transaction_txid(&previous.transactions[0]).expect("funding txid"),
                4_999_999_000,
                script(&[0x52, 0x75, 0x51]),
            );
            let second = spend_to_script(
                transaction_txid(&first).expect("same-block txid"),
                4_999_998_000,
                script(&[0x53, 0x75, 0x51]),
            );
            vec![first, second]
        } else {
            Vec::new()
        };
        let block = build_block_with_transactions(
            blocks
                .last()
                .map_or(BlockHash::default(), |block| block_hash(&block.header)),
            height,
            5_000_000_000,
            script(&[0x01, height as u8, 0x75, 0x51]),
            transactions,
        );
        blocks.push(block);
    }
    let genesis = blocks.first().expect("populated fixture");
    let runtime = open_basic_source(data_dir, genesis, relay, block_relay);
    let maybe_replacement = (count == 401).then(|| basic_replacement(&blocks));
    admit_basic_headers(&runtime, &blocks[1..], maybe_replacement.as_ref());
    let network = runtime.network_handle();
    let root_before_duplicate = network.chainstate_snapshot().expect("actual root endpoint");
    assert!(
        matches!(network.connect_stored_block(genesis, fixture_work(), 2_000_000_000, ScriptVerifyFlags::P2SH, basic_params()).expect("checked duplicate root linkage"), open_bitcoin_node::network::BlockConnectDisposition::Duplicate(hash) if hash == block_hash(&genesis.header))
    );
    assert_eq!(
        network
            .chainstate_snapshot()
            .expect("duplicate root changes no active position"),
        root_before_duplicate
    );
    let root = network
        .chainstate_snapshot()
        .expect("root before pending intake");
    if let Some(replacement) = maybe_replacement.as_ref() {
        runtime
            .store()
            .save_block(replacement, PersistMode::Sync)
            .expect("retain exact early replacement bytes");
        assert!(
            matches!(network.connect_stored_block(replacement, fixture_work(), 2_000_000_000, ScriptVerifyFlags::P2SH, basic_params()).expect("actual early B400 body"), open_bitcoin_node::network::BlockConnectDisposition::NonExtending { block_hash: hash, .. } if hash == block_hash(&replacement.header))
        );
        assert_eq!(
            network
                .chainstate_snapshot()
                .expect("pending body does not connect")
                .active_chain,
            root.active_chain
        );
    }
    for block in blocks.iter().skip(1) {
        network
            .connect_local_block(block, ScriptVerifyFlags::P2SH, basic_params())
            .expect("genuine continuous historical and same-block spends");
        runtime
            .store()
            .save_block(block, PersistMode::Sync)
            .expect("actual body");
    }
    runtime
        .network_handle()
        .flush_coins(FlushMode::Always, FlushPolicyTime::new(1), u64::MAX)
        .expect("actual coins flush");
    if count > 2 {
        use open_bitcoin_node::core::chainstate::{BasicFilterInputs, HistoricalBlockUndo};
        let snapshot = runtime
            .network_handle()
            .chainstate_snapshot()
            .expect("actual staged spend undo");
        let last = blocks.last().expect("nonempty accepted history");
        let position = snapshot.active_chain.last().expect("accepted position");
        let inputs = BasicFilterInputs::from_historical(
            last,
            position,
            snapshot
                .undo_by_block
                .get(&position.block_hash)
                .map(|undo| HistoricalBlockUndo {
                    block_hash: position.block_hash,
                    undo,
                }),
        )
        .expect("actual accepted historical filter inputs");
        assert_eq!(
            inputs
                .spent_scripts()
                .map(<[u8]>::to_vec)
                .collect::<Vec<_>>(),
            vec![
                blocks[blocks.len() - 2].transactions[0].outputs[0]
                    .script_pubkey
                    .as_bytes()
                    .to_vec(),
                last.transactions[1].outputs[0]
                    .script_pubkey
                    .as_bytes()
                    .to_vec()
            ]
        );
        assert_ne!(
            blocks[blocks.len() - 2].transactions[0].outputs[0].script_pubkey,
            last.transactions[0].outputs[0].script_pubkey,
            "historical script is distinct from current outputs"
        );
    }
    (runtime, blocks)
}

fn open_basic_source(
    data_dir: &Path,
    genesis: &Block,
    relay: open_bitcoin_network::RelayActivationConfig,
    block_relay: open_bitcoin_network::BlockRelayActivationPolicy,
) -> open_bitcoin_node::DurableSyncRuntime {
    use open_bitcoin_node::core::{
        chainstate::{FlushMode, FlushPolicyTime},
        consensus::ScriptVerifyFlags,
    };
    use open_bitcoin_node::{DurableSyncRuntime, FjallNodeStore, SyncNetwork, SyncRuntimeConfig};
    let store = FjallNodeStore::open(data_dir).expect("durable store");
    let mut core = open_bitcoin_node::core::chainstate::Chainstate::from_parent(
        store.coins_view(),
        vec![],
        HashMap::new(),
        None,
    );
    open_bitcoin_node::core::chainstate::Chainstate::new()
        .rebind_into(&mut core)
        .expect("genuine empty transfer");
    let now = FlushPolicyTime::new(0);
    let mut lifecycle = open_bitcoin_node::chainstate::FlushLifecycle::ready(now, now, 0, false);
    let mut source = open_bitcoin_node::ManagedChainstate::from_chainstate(
        open_bitcoin_node::FjallChainstateStore::from_store(store),
        core,
        lifecycle.clone(),
    );
    source
        .connect_block(
            genesis,
            fixture_work(),
            ScriptVerifyFlags::P2SH,
            basic_params(),
        )
        .expect("actual managed genesis acceptance");
    let (mut sink, mut same_core) = source.into_parts();
    let positions = same_core.active_chain().to_vec();
    let undo = same_core
        .undo_by_block()
        .iter()
        .map(|(hash, undo)| (*hash, undo.clone()))
        .collect::<Vec<_>>();
    lifecycle
        .execute_flush(
            &mut sink,
            &mut same_core,
            FlushMode::Always,
            FlushPolicyTime::new(1),
            u64::MAX,
            &undo,
            &[],
            std::slice::from_ref(genesis),
            &positions,
        )
        .expect("real genesis flush without raw future headers");
    let source = open_bitcoin_node::ManagedChainstate::from_chainstate(sink, same_core, lifecycle);
    DurableSyncRuntime::from_live_fixture_source(
        source,
        SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            ..Default::default()
        },
        basic_params(),
        open_bitcoin_node::chainstate::BasicFilterStartupMode::Enabled,
        relay,
        block_relay,
        false,
    )
    .expect("same genuine core with explicit maturity-1 BASIC")
}

pub(super) fn basic_params() -> open_bitcoin_node::core::consensus::ConsensusParams {
    open_bitcoin_node::core::consensus::ConsensusParams {
        coinbase_maturity: 1,
        ..Default::default()
    }
}

pub(super) fn assert_raw_reopen_refuses_work(runtime: &open_bitcoin_node::DurableSyncRuntime) {
    use open_bitcoin_node::core::{
        chainstate::{ChainstateError, CoinsView},
        consensus::ScriptVerifyFlags,
    };
    let network = runtime.network_handle();
    let before = network
        .chainstate_snapshot()
        .expect("raw snapshot before refusal");
    let physical = runtime
        .store()
        .coins_view()
        .complete_snapshot()
        .expect("physical state before refusal");
    let tip = before.tip().expect("reopened actual endpoint");
    let later = build_block(
        tip.block_hash,
        tip.height + 1,
        5_000_000_000,
        script(&[0x51]),
    );
    assert!(matches!(
        network.connect_local_block(&later, ScriptVerifyFlags::P2SH, basic_params()),
        Err(
            open_bitcoin_node::network::ManagedNetworkAuthorityError::Operation(
                open_bitcoin_node::network::ManagedNetworkError::Chainstate(
                    ChainstateError::UntrustedWorkLineage
                )
            )
        )
    ));
    assert_eq!(
        network
            .chainstate_snapshot()
            .expect("unchanged raw snapshot"),
        before
    );
    assert_eq!(
        runtime
            .store()
            .coins_view()
            .complete_snapshot()
            .expect("unchanged physical state"),
        physical
    );
    assert!(
        !runtime
            .store()
            .has_block(block_hash(&later.header))
            .expect("refused body absent")
    );
}
fn fixture_work() -> open_bitcoin_node::core::consensus::chain_proof::ProofWork {
    open_bitcoin_node::core::consensus::chain_proof::ProofWork::ZERO
}
fn basic_replacement(blocks: &[Block]) -> Block {
    let tip = blocks.len() - 1;
    let mut replacement = build_block(
        block_hash(&blocks[tip - 1].header),
        u32::try_from(tip).expect("bounded height"),
        5_000_000_000,
        script(&[0x51]),
    );
    replacement.header.time += 1;
    mine_header(&mut replacement);
    replacement
}
fn admit_basic_headers(
    runtime: &open_bitcoin_node::DurableSyncRuntime,
    future: &[Block],
    maybe_replacement: Option<&Block>,
) {
    use open_bitcoin_network::{HeadersMessage, VersionMessage, WireNetworkMessage};
    use open_bitcoin_node::core::consensus::ScriptVerifyFlags;
    let network = runtime.network_handle();
    let peer = 165_016;
    let now = 2_000_000_000;
    network
        .connect_outbound_peer(peer, now)
        .expect("modeled header peer");
    for message in [
        WireNetworkMessage::Version(VersionMessage {
            start_height: future.len() as i32,
            ..Default::default()
        }),
        WireNetworkMessage::Verack,
    ] {
        network
            .receive_sync_message(peer, message, now, ScriptVerifyFlags::P2SH, basic_params())
            .expect("actual header handshake");
    }
    for batch in [
        future,
        maybe_replacement.map_or(&[][..], std::slice::from_ref),
    ] {
        if batch.is_empty() {
            continue;
        }
        let result = network
            .receive_sync_message(
                peer,
                WireNetworkMessage::Headers(HeadersMessage {
                    headers: batch.iter().map(|block| block.header.clone()).collect(),
                }),
                now,
                ScriptVerifyFlags::P2SH,
                basic_params(),
            )
            .expect("separate contiguous future header batch");
        assert!(result.maybe_block_disposition.is_none());
        let entries = network.header_entries().expect("accepted exact headers");
        for block in batch {
            let entry = entries
                .iter()
                .find(|entry| entry.block_hash == block_hash(&block.header))
                .expect("actual admitted future header");
            assert_eq!(entry.header, block.header);
            assert_eq!(
                entry.chain_work,
                entries
                    .iter()
                    .find(|entry| entry.block_hash == block.header.previous_block_hash)
                    .expect("accepted parent")
                    .chain_work
                    .wrapping_add(
                        open_bitcoin_node::core::consensus::chain_proof::block_proof(
                            block.header.bits
                        )
                        .expect("proof")
                    )
            );
        }
    }
    network
        .disconnect_peer(peer)
        .expect("clean modeled header peer");
}

pub(super) fn replace_basic_tip(
    runtime: &open_bitcoin_node::DurableSyncRuntime,
    blocks: &[Block],
) -> BlockHash {
    use open_bitcoin_node::core::{chainstate::AnchoredBlock, consensus::ScriptVerifyFlags};
    let network = runtime.network_handle();
    let tip = blocks.len() - 1;
    let displaced = block_hash(&blocks[tip].header);
    let snapshot = network.chainstate_snapshot().expect("disconnect undo");
    runtime
        .store()
        .save_undo(
            displaced,
            &snapshot.undo_by_block[&displaced],
            open_bitcoin_node::PersistMode::Sync,
        )
        .expect("disconnect undo");
    let replacement = runtime
        .store()
        .load_block(block_hash(&basic_replacement(blocks).header))
        .expect("retained early body read")
        .expect("same precomputed replacement bytes");
    let before_tip = snapshot.tip().expect("active tip");
    let expected_work =
        open_bitcoin_node::core::consensus::chain_proof::block_proof(replacement.header.bits)
            .expect("actual proof");
    assert_eq!(
        before_tip.chain_work,
        snapshot.active_chain[tip - 1]
            .chain_work
            .wrapping_add(expected_work)
    );
    runtime
        .store()
        .save_block(&replacement, open_bitcoin_node::PersistMode::Sync)
        .expect("replacement body");
    network
        .reorg_to_branch(
            &[blocks[tip].clone()],
            &[AnchoredBlock {
                block: replacement,
                chain_work: fixture_work(),
            }],
            open_bitcoin_node::core::mempool::ReorgLifecycleContext::new(
                open_bitcoin_node::core::mempool::PolicyTime::from_unix_seconds(50_000),
            ),
            ScriptVerifyFlags::P2SH,
            basic_params(),
        )
        .expect("actual validated reorg");
    let after = network
        .chainstate_snapshot()
        .expect("actual equal-work reorg endpoint");
    assert_eq!(
        after.tip().expect("replacement tip").height,
        before_tip.height
    );
    assert_eq!(
        after.tip().expect("replacement tip").chain_work,
        before_tip.chain_work
    );
    assert_eq!(
        after.tip().expect("replacement tip").block_hash,
        block_hash(&basic_replacement(blocks).header)
    );
    displaced
}

pub(super) fn run_raw_cli(sandbox: &TestSandbox, args: &[String]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_open-bitcoin-cli"));
    command.env("HOME", &sandbox.home);
    for arg in args {
        command.arg(arg);
    }
    command.output().expect("cli output")
}

pub(super) fn stdout_text(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout")
}

pub(super) fn stderr_text(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).expect("stderr")
}

pub(super) fn assert_success_json(output: &Output) -> Value {
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        stdout_text(output),
        stderr_text(output),
    );
    serde_json::from_slice(&output.stdout).expect("stdout json")
}
