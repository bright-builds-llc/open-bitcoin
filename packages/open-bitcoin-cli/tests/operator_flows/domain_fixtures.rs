// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/rpc/client.cpp
// - packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py

use super::*;

pub(super) fn durable_basic_runtime(
    data_dir: &Path,
    count: u32,
) -> (open_bitcoin_node::DurableSyncRuntime, Vec<Block>) {
    use open_bitcoin_node::core::{
        chainstate::{FlushMode, FlushPolicyTime},
        consensus::{ConsensusParams, ScriptVerifyFlags},
    };
    use open_bitcoin_node::{
        DurableSyncRuntime, FjallNodeStore, PersistMode, SyncNetwork, SyncRuntimeConfig,
    };
    let runtime = DurableSyncRuntime::open(
        FjallNodeStore::open(data_dir).expect("durable store"),
        SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            ..Default::default()
        },
    )
    .expect("real runtime");
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
        runtime
            .network_handle()
            .connect_local_block(
                &block,
                ScriptVerifyFlags::P2SH,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..Default::default()
                },
            )
            .expect("genuine continuous historical and same-block spends");
        runtime
            .store()
            .save_block(&block, PersistMode::Sync)
            .expect("actual body");
        blocks.push(block);
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

pub(super) fn replace_basic_tip(
    runtime: &open_bitcoin_node::DurableSyncRuntime,
    blocks: &[Block],
) -> BlockHash {
    use open_bitcoin_node::core::{
        chainstate::AnchoredBlock,
        consensus::{ConsensusParams, ScriptVerifyFlags},
    };
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
    let mut replacement = build_block(
        block_hash(&blocks[tip - 1].header),
        u32::try_from(tip).expect("bounded fixture height"),
        5_000_000_000,
        script(&[0x51]),
    );
    replacement.header.time += 1;
    mine_header(&mut replacement);
    runtime
        .store()
        .save_block(&replacement, open_bitcoin_node::PersistMode::Sync)
        .expect("replacement body");
    network
        .reorg_to_branch(
            &[blocks[tip].clone()],
            &[AnchoredBlock {
                block: replacement,
                chain_work: tip as u128 + 1_000,
            }],
            open_bitcoin_node::core::mempool::ReorgLifecycleContext::new(
                open_bitcoin_node::core::mempool::PolicyTime::from_unix_seconds(50_000),
            ),
            ScriptVerifyFlags::P2SH,
            ConsensusParams {
                coinbase_maturity: 1,
                ..Default::default()
            },
        )
        .expect("actual validated reorg");
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
