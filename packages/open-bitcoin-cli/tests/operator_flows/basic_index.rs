// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
#[path = "basic_index/captured.rs"]
mod captured;
use open_bitcoin_node::{
    DurableSyncRuntime, FjallNodeStore, SyncNetwork, SyncRuntimeConfig,
    chainstate::BasicFilterStartupMode,
};

fn operator(sandbox: &TestSandbox, data_dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_open-bitcoin"))
        .env("HOME", &sandbox.home)
        .args([
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "--no-color",
        ])
        .args(args)
        .output()
        .expect("actual operator binary")
}

fn body_request(
    runtime: &DurableSyncRuntime,
    peer: u64,
    hash: BlockHash,
) -> open_bitcoin_node::network::ManagedSyncMessageResult {
    use open_bitcoin_network::{InventoryList, VersionMessage, WireNetworkMessage};
    use open_bitcoin_node::core::{
        consensus::{ConsensusParams, ScriptVerifyFlags},
        primitives::{InventoryType, InventoryVector},
    };
    let network = runtime.network_handle();
    network
        .connect_outbound_peer(peer, 1)
        .expect("local modeled peer only");
    for message in [
        WireNetworkMessage::Version(VersionMessage::default()),
        WireNetworkMessage::Verack,
    ] {
        network
            .receive_message(
                peer,
                message,
                1,
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default(),
            )
            .expect("modeled handshake");
    }
    network
        .receive_message_for_durable_serving(
            peer,
            WireNetworkMessage::GetData(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::Block,
                object_hash: hash.into(),
            }])),
            2,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
            |candidate| {
                runtime
                    .store()
                    .has_block(candidate)
                    .expect("actual body probe")
            },
        )
        .expect("actual modeled body request")
}

fn assert_wallet_refusal(runtime: &DurableSyncRuntime) {
    let mut registry =
        open_bitcoin_node::WalletRegistry::load(runtime.store()).expect("wallet registry");
    registry
        .create_wallet(
            runtime.store(),
            "negative",
            Wallet::new(AddressNetwork::Regtest),
            open_bitcoin_node::PersistMode::Sync,
        )
        .expect("empty test wallet");
    let original_wallet = registry
        .wallet_snapshot("negative")
        .expect("wallet before")
        .clone();
    let rescan = open_bitcoin_node::WalletRescanRuntime::open(
        runtime.store().clone(),
        open_bitcoin_node::PersistMode::Sync,
    )
    .expect("rescan adapter");
    assert!(
        rescan.enqueue_rescan("negative").is_err(),
        "real requested-range payload is absent"
    );
    let failed_job = runtime
        .store()
        .load_wallet_rescan_job("negative")
        .expect("persisted failed job")
        .expect("job exists");
    assert!(
        failed_job
            .maybe_error
            .as_deref()
            .is_some_and(|detail| detail.contains("requested") && detail.contains("20")),
        "{failed_job:?}"
    );
    assert_eq!(
        open_bitcoin_node::WalletRegistry::load(runtime.store())
            .expect("wallet after")
            .wallet_snapshot("negative")
            .expect("preserved wallet"),
        &original_wallet
    );
    drop(rescan);
}

fn assert_exports(
    sandbox: &TestSandbox,
    server: &RpcTestServer,
    label: &str,
    expected: &Value,
) -> Value {
    let data_dir = sandbox.home.join(label);
    fs::create_dir_all(&data_dir).expect("client config dir");
    fs::write(
        data_dir.join("bitcoin.conf"),
        format!(
            "rpcconnect=127.0.0.1\nrpcport={}\nrpcuser=alice\nrpcpassword=secret\n",
            server.address.port()
        ),
    )
    .expect("client RPC config");
    // Act
    let status = assert_success_json(&operator(
        sandbox,
        &data_dir,
        &["status", "--format", "json"],
    ));
    let dashboard = assert_success_json(&operator(
        sandbox,
        &data_dir,
        &["dashboard", "--format", "json"],
    ));
    let human = operator(sandbox, &data_dir, &["status", "--format", "human"]);
    let text = operator(sandbox, &data_dir, &["dashboard", "--format", "human"]);
    let output_dir = data_dir.join("bundle");
    assert_success_json(&operator(
        sandbox,
        &data_dir,
        &[
            "support",
            "bundle",
            "--output-dir",
            output_dir.to_str().expect("output dir"),
            "--format",
            "json",
        ],
    ));
    let support: Value = serde_json::from_slice(
        &fs::read(output_dir.join("support-evidence.json")).expect("real support file"),
    )
    .expect("support JSON");
    let markdown =
        fs::read_to_string(output_dir.join("support-evidence.md")).expect("real support Markdown");
    // Assert
    assert_eq!(status["basic_index"], *expected, "status {label}");
    assert_eq!(dashboard["basic_index"], *expected, "dashboard {label}");
    assert_eq!(
        support["status"]["basic_index"], *expected,
        "support {label}"
    );
    assert!(human.status.success(), "{}", stderr_text(&human));
    assert!(text.status.success(), "{}", stderr_text(&text));
    let snapshot: open_bitcoin_node::OpenBitcoinStatusSnapshot =
        serde_json::from_value(status.clone()).expect("actual collected snapshot");
    let model =
        open_bitcoin_cli::operator::dashboard::model::DashboardState::from_snapshot(&snapshot);
    let section = model
        .sections
        .iter()
        .find(|section| section.title == "BASIC index")
        .expect("real dashboard model section");
    assert_eq!(section.rows.len(), 19);
    assert_eq!(
        section
            .rows
            .iter()
            .map(|row| row.label.as_str())
            .collect::<Vec<_>>(),
        [
            "State",
            "Init sync",
            "Lag",
            "Processed",
            "Durable",
            "Failure",
            "Retained",
            "Index bytes",
            "Filter bytes",
            "Records",
            "Indexing",
            "Peer serving",
            "Peer scope",
            "Target",
            "Protection",
            "Basis",
            "Next action",
            "Retention note",
            "Body/wallet note",
        ]
    );
    for row in &section.rows {
        let rendered = format!("{}: {}", row.label, row.value);
        assert!(
            stdout_text(&human).contains(&rendered),
            "human {label}: {rendered}"
        );
        assert!(
            stdout_text(&text).contains(&rendered),
            "dashboard {label}: {rendered}"
        );
        assert!(
            markdown.contains(&format!("- {rendered}")),
            "support {label}: {rendered}"
        );
    }
    assert!(expected.to_string().len() <= 4096);
    assert!(
        markdown.contains("Retained filters do not restore deleted bodies or wallet eligibility.")
    );
    status
}

fn assert_consumers(sandbox: &TestSandbox, runtime: &DurableSyncRuntime, label: &str) {
    // Arrange: no worker runs; each consumer reads this identical authority epoch.
    let network = runtime.network_handle();
    let before = network
        .operator_snapshot()
        .expect("capture")
        .basic_index()
        .clone();
    let expected = serde_json::to_value(&before).expect("independent captured BASIC");
    let config = RuntimeConfig {
        prune_mode: network.prune_mode().expect("actual prune mode"),
        ..Default::default()
    };
    let context = ManagedRpcContext::from_runtime_config_with_network_handle(
        &config,
        network.clone(),
        Some(runtime.store().clone()),
    )
    .expect("actual durable RPC context");
    let server = RpcTestServer::start(context);
    let rpc = assert_success_json(&run_cli_with_rpc(
        &server,
        sandbox,
        &["openbitcoinnetworkstatus".to_string()],
    ));
    assert_eq!(rpc["basic_index"], expected, "actual RPC client {label}");
    let status = assert_exports(sandbox, &server, label, &expected);
    if let open_bitcoin_node::core::chainstate::PruneMode::Automatic { target_mib } =
        config.prune_mode
    {
        assert_eq!(
            status["prune"]["maybe_prune_target_size"],
            target_mib * 1024 * 1024
        );
        assert_eq!(status["basic_index"]["retention"], expected["retention"]);
    }
    assert_eq!(
        network
            .operator_snapshot()
            .expect("unchanged frozen authority")
            .basic_index(),
        &before
    );
}
#[test]
fn phase161_operator_consumers_genuine_catchup_current_lag_ahead_reopen() {
    // Arrange
    let sandbox = TestSandbox::new("phase161-consumers");
    let store_dir = sandbox.home.join("store");
    let (runtime, mut blocks) = durable_basic_runtime(&store_dir, 24);
    drop(runtime);
    let runtime = DurableSyncRuntime::open_configured(
        FjallNodeStore::open(&store_dir).expect("all-handle reopen"),
        SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            ..Default::default()
        },
        BasicFilterStartupMode::Enabled,
    )
    .expect("actual configured startup");
    // Act / Assert
    assert_consumers(&sandbox, &runtime, "catching");
    for _ in 0..2 {
        runtime
            .network_handle()
            .drive_basic_filter_index_turn()
            .expect("scheduled turn");
    }
    assert_consumers(&sandbox, &runtime, "current");
    runtime
        .network_handle()
        .set_prune_mode(open_bitcoin_node::core::chainstate::PruneMode::Automatic {
            target_mib: 550,
        })
        .expect("explicit automatic target display policy");
    assert_consumers(&sandbox, &runtime, "automatic-target");
    let block = build_block(
        block_hash(&blocks.last().expect("tip").header),
        24,
        5_000_000_000,
        script(&[0x51]),
    );
    runtime
        .network_handle()
        .connect_local_block(
            &block,
            open_bitcoin_node::core::consensus::ScriptVerifyFlags::P2SH,
            open_bitcoin_node::core::consensus::ConsensusParams {
                coinbase_maturity: 1,
                ..Default::default()
            },
        )
        .expect("real later accepted block");
    runtime
        .store()
        .save_block(&block, open_bitcoin_node::PersistMode::Sync)
        .expect("body");
    blocks.push(block);
    assert_consumers(&sandbox, &runtime, "lagged");
    runtime
        .network_handle()
        .drive_basic_filter_index_turn()
        .expect("processed ahead of durable");
    assert_consumers(&sandbox, &runtime, "ahead");
    runtime
        .network_handle()
        .flush_coins(
            open_bitcoin_node::core::chainstate::FlushMode::Always,
            open_bitcoin_node::core::chainstate::FlushPolicyTime::new(2),
            u64::MAX,
        )
        .expect("real later fence");
    runtime
        .network_handle()
        .drive_basic_filter_index_turn()
        .expect("durable acknowledgement");
    let before = runtime
        .network_handle()
        .operator_snapshot()
        .expect("before reopen")
        .basic_index()
        .clone();
    drop(runtime);
    let runtime = DurableSyncRuntime::open_configured(
        FjallNodeStore::open(&store_dir).expect("every earlier clone closed"),
        SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            ..Default::default()
        },
        BasicFilterStartupMode::Enabled,
    )
    .expect("real verified reopen");
    assert_eq!(
        runtime
            .network_handle()
            .operator_snapshot()
            .expect("after reopen")
            .basic_index(),
        &before
    );
    assert_consumers(&sandbox, &runtime, "reopened");
    runtime
        .network_handle()
        .disable_basic_filter_index()
        .expect("explicit owner disable");
    assert_consumers(&sandbox, &runtime, "disabled-retained");
}

#[test]
fn phase161_operator_consumers_disabled_no_genesis_and_stopped() {
    // Arrange
    let sandbox = TestSandbox::new("phase161-empty");
    let store_dir = sandbox.home.join("empty-store");
    let runtime = DurableSyncRuntime::open_configured(
        FjallNodeStore::open(&store_dir).expect("empty Fjall"),
        SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            ..Default::default()
        },
        BasicFilterStartupMode::Disabled,
    )
    .expect("disabled empty startup");
    // Act / Assert
    let captured = serde_json::to_value(
        runtime
            .network_handle()
            .operator_snapshot()
            .expect("actual empty authority")
            .basic_index(),
    )
    .expect("JSON");
    assert_eq!(captured["configured_indexing"]["value"], false);
    assert_eq!(captured["operational_state"]["value"], "disabled");
    assert_eq!(captured["processed_height"]["state"], "unavailable");
    assert_eq!(captured["safe_durable_height"]["state"], "unavailable");
    assert_consumers(&sandbox, &runtime, "disabled-empty");
    let stopped_dir = sandbox.home.join("stopped");
    fs::create_dir_all(&stopped_dir).expect("stopped datadir");
    let output = operator(&sandbox, &stopped_dir, &["status", "--format", "json"]);
    let stopped = assert_success_json(&output);
    for (field, value) in stopped["basic_index"].as_object().expect("stopped BASIC") {
        assert_eq!(value["state"], "unavailable", "stopped invented {field}");
    }
    assert!(
        !stopped_dir.join("db").exists(),
        "status cannot create a Fjall store"
    );
    let mut closed = RpcTestServer::start(empty_context());
    closed.stop.send(()).expect("stop local listener");
    closed
        .join_handle
        .take()
        .expect("listener thread")
        .join()
        .expect("listener closed");
    assert_exports(&sandbox, &closed, "stopped-socket", &stopped["basic_index"]);
}

#[test]
fn phase161_operator_consumers_retained_pruned_stale_and_wallet_body_negatives() {
    use open_bitcoin_network::{
        BlockRelayActivationPolicy, BlockServingActivationConfig, RelayActivationConfig,
        WireNetworkMessage,
    };
    use open_bitcoin_node::core::chainstate::{FlushMode, FlushPolicyTime, PruneMode, PrunePlan};
    // Arrange: continuous actual accepted history and explicitly enabled modeled body serving.
    let sandbox = TestSandbox::new("phase161-retained-clients");
    let store_dir = sandbox.home.join("store");
    let (runtime, blocks) = durable_basic_runtime(&store_dir, 401);
    drop(runtime);
    let runtime = DurableSyncRuntime::open_with_configured_runtime_activation(
        FjallNodeStore::open(&store_dir).expect("all-handle open"),
        SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            ..Default::default()
        },
        RelayActivationConfig { enabled: true },
        BlockRelayActivationPolicy {
            block_serving: BlockServingActivationConfig { enabled: true },
            ..Default::default()
        },
        false,
        BasicFilterStartupMode::Enabled,
    )
    .expect("explicit local runtime policy");
    let mut network = runtime.network_handle();
    network
        .set_prune_mode(PruneMode::ManualOnly)
        .expect("manual prune policy");
    for _ in 0..50 {
        network
            .drive_basic_filter_index_turn()
            .expect("ordinary bounded turns");
    }
    let before = network
        .operator_snapshot()
        .expect("full current capture")
        .basic_index()
        .clone();
    let pruned = block_hash(&blocks[20].header);
    let present = body_request(&runtime, 161_800, block_hash(&blocks[400].header));
    assert!(present.inbound_response_plan.iter().any(|item| matches!(item, open_bitcoin_node::ManagedInboundResponsePlanItem::DurableBlock(intent) if intent.block_hash() == block_hash(&blocks[400].header))), "same enabled policy must admit a physically present in-window body before pruning");
    // Act
    let deletion = network
        .flush_applying_prune_plan(
            FlushMode::Always,
            FlushPolicyTime::new(3),
            u64::MAX,
            &PrunePlan { heights: vec![20] },
            &[],
        )
        .expect("legal actual owner prune");
    assert_eq!(deletion.deleted_block_hashes, vec![pruned]);
    assert_eq!(
        network
            .operator_snapshot()
            .expect("filter totals unchanged")
            .basic_index(),
        &before
    );
    assert_consumers(&sandbox, &runtime, "pruned-current");
    // Assert: retained filters never restore physical payload or real wallet eligibility.
    assert!(!runtime.store().has_block(pruned).expect("body absence"));
    assert!(!runtime.store().has_undo(pruned).expect("undo absence"));
    assert_wallet_refusal(&runtime);
    let served = body_request(&runtime, 161_801, pruned);
    assert!(
        !served
            .outbound
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::Block(_)))
    );
    assert!(served.inbound_response_plan.iter().any(|item| matches!(
        item,
        open_bitcoin_node::ManagedInboundResponsePlanItem::Immediate(WireNetworkMessage::NotFound(
            _
        ))
    )));
    assert!(!served.inbound_response_plan.iter().any(|item| matches!(
        item,
        open_bitcoin_node::ManagedInboundResponsePlanItem::DurableBlock(_)
    )));
    // A genuine same-height branch replacement retains its displaced immutable row.
    let displaced = replace_basic_tip(&runtime, &blocks);
    network
        .flush_coins(FlushMode::Always, FlushPolicyTime::new(4), u64::MAX)
        .expect("replacement durable fence");
    network
        .drive_basic_filter_index_turn()
        .expect("replacement indexing");
    assert_consumers(&sandbox, &runtime, "retained-stale");
    assert!(matches!(
        network
            .basic_filter_query(displaced)
            .expect("actual stale row"),
        open_bitcoin_node::BasicFilterQuery::Found(_)
    ));
    let expected = network
        .operator_snapshot()
        .expect("before reopen")
        .basic_index()
        .clone();
    drop(network);
    drop(runtime);
    let runtime = DurableSyncRuntime::open_with_configured_runtime_activation(
        FjallNodeStore::open(&store_dir).expect("closed every store/context clone"),
        SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            ..Default::default()
        },
        RelayActivationConfig { enabled: true },
        BlockRelayActivationPolicy {
            block_serving: BlockServingActivationConfig { enabled: true },
            ..Default::default()
        },
        false,
        BasicFilterStartupMode::Enabled,
    )
    .expect("real verified reopen");
    runtime
        .network_handle()
        .set_prune_mode(PruneMode::ManualOnly)
        .expect("same manual serving policy after reopen");
    assert_eq!(
        runtime
            .network_handle()
            .operator_snapshot()
            .expect("after reopen")
            .basic_index(),
        &expected
    );
    assert_consumers(&sandbox, &runtime, "pruned-stale-reopened");
    assert!(
        !runtime
            .store()
            .has_block(pruned)
            .expect("reopened body absent")
    );
    assert!(
        !runtime
            .store()
            .has_undo(pruned)
            .expect("reopened undo absent")
    );
    let served = body_request(&runtime, 161_802, pruned);
    assert!(served.inbound_response_plan.iter().any(|item| matches!(
        item,
        open_bitcoin_node::ManagedInboundResponsePlanItem::Immediate(WireNetworkMessage::NotFound(
            _
        ))
    )));
    assert!(!served.inbound_response_plan.iter().any(|item| matches!(
        item,
        open_bitcoin_node::ManagedInboundResponsePlanItem::DurableBlock(_)
    )));
}
