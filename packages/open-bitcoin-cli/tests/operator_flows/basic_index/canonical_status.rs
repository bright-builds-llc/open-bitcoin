// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use open_bitcoin_node::core::chainstate::{PruneLockInfo, PruneMode};
use open_bitcoin_rpc::config::BasicFilterIndexSetting;

mod fixture {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/cint02/fixture.rs"
    ));
}

const PRIVATE_PRUNE_LOCK: &str = "cint03-fixture-private-lock";

#[test]
fn cint03_operator_exports_observe_the_same_canonical_tip_and_refusal() {
    // Arrange: a complete archive and the actually pruned reorg source.
    let archive = fixture::Source::new();
    let retained = fixture::Source::new();
    fixture::build(&archive, false);
    let evidence = fixture::build(&retained, true);
    let config = RuntimeConfig {
        chain: AddressNetwork::Regtest,
        maybe_data_dir: Some(retained.path().to_path_buf()),
        sync: open_bitcoin_rpc::config::DaemonSyncConfig {
            runtime: SyncRuntimeConfig {
                network: SyncNetwork::Regtest,
                dns_seeds: Vec::new(),
                ..Default::default()
            },
            ..Default::default()
        },
        prune_mode: PruneMode::ManualOnly,
        block_filter_index: BasicFilterIndexSetting::Basic,
        ..Default::default()
    };
    let (context, runtime) =
        ManagedRpcContext::from_pruned_regtest_runtime_config(&config, archive.path())
            .expect("proved canonical status context");
    drop(archive);
    assert!(
        !runtime
            .store()
            .has_block(evidence.pruned.identity().block_hash())
            .expect("pruned boundary body")
    );
    assert_eq!(
        runtime
            .store()
            .load_block(evidence.retained.identity().block_hash())
            .expect("retained boundary lookup"),
        Some(evidence.retained_block)
    );
    assert_ne!(
        evidence.active.filter_header(),
        evidence.displaced.filter_header()
    );
    let handle = runtime.network_handle();
    handle
        .replace_prune_lock(PruneLockInfo {
            name: PRIVATE_PRUNE_LOCK.to_string(),
            height_first: fixture::TIP,
            height_last: fixture::TIP,
        })
        .expect("private fixture lock");
    let expected_basic = serde_json::to_value(
        handle
            .operator_snapshot()
            .expect("proved authority snapshot")
            .basic_index(),
    )
    .expect("BASIC expectation");
    let server = RpcTestServer::start(context);
    let sandbox = TestSandbox::new("cint03-canonical-operator");

    // Act: a real authenticated request records the keep-window refusal.
    let refuse = run_cli_with_rpc(
        &server,
        &sandbox,
        &[
            "pruneblockchain".to_string(),
            (fixture::BOUNDARY + 1).to_string(),
        ],
    );
    assert!(!refuse.status.success(), "keep-window prune must refuse");
    let rpc = assert_success_json(&run_cli_with_rpc(
        &server,
        &sandbox,
        &["getblockchaininfo".to_string()],
    ));
    let operator_rpc = assert_success_json(&run_cli_with_rpc(
        &server,
        &sandbox,
        &["openbitcoinnetworkstatus".to_string()],
    ));
    let label = "canonical-pruned";
    let status = assert_exports(&sandbox, &server, label, &expected_basic);
    let client_dir = sandbox.home.join(label);
    let dashboard = assert_success_json(&operator(
        &sandbox,
        &client_dir,
        &["dashboard", "--format", "json"],
    ));
    let dashboard_text = stdout_text(&operator(
        &sandbox,
        &client_dir,
        &["dashboard", "--format", "human"],
    ));
    let bundle_dir = client_dir.join("bundle");
    let support_json =
        fs::read_to_string(bundle_dir.join("support-evidence.json")).expect("real support bundle");
    let support: Value = serde_json::from_str(&support_json).expect("support JSON");
    let markdown =
        fs::read_to_string(bundle_dir.join("support-evidence.md")).expect("support Markdown");

    // Assert: independent fixed-target work and the consensus tip agree on every surface.
    let hash = evidence
        .tip_hash
        .as_bytes()
        .iter()
        .rev()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let work = format!("{:064x}", 2 * (fixture::TIP + 1));
    assert_eq!(rpc["blocks"], fixture::TIP);
    assert_eq!(rpc["bestblockhash"], hash);
    assert_eq!(rpc["chainwork"], work);
    assert_eq!(rpc["pruneheight"], fixture::BOUNDARY + 1);
    for snapshot in [&status, &dashboard, &support["status"]] {
        assert_eq!(snapshot["sync"]["chain_tip"]["value"]["block_hash"], hash);
        assert_eq!(
            snapshot["sync"]["chain_tip"]["value"]["height"],
            fixture::TIP
        );
        assert_eq!(
            snapshot["sync"]["sync_progress"]["value"]["maybe_validated_active_chain_work"],
            work
        );
        assert_eq!(snapshot["basic_index"], expected_basic);
        assert_eq!(
            snapshot["prune"]["pruneheight"]["value"],
            fixture::BOUNDARY + 1
        );
        assert_eq!(
            snapshot["prune"]["support_counts"]["value"]["pruned_height_count"],
            fixture::BOUNDARY
        );
        assert_eq!(
            snapshot["prune"]["manual_prune"]["value"],
            json!({"refused": {"reason": "keep_window"}})
        );
    }
    assert_eq!(operator_rpc["basic_index"], expected_basic);
    assert_eq!(operator_rpc["prune"], status["prune"]);
    assert!(dashboard_text.contains(&format!("validated_active_chain_work={work}")));
    assert_eq!(
        support["full_sync_evidence"]["validated_active_chain"]["work"],
        work
    );
    assert_eq!(support["status"]["prune"]["locks"]["value"], json!([]));
    assert!(
        status["prune"]["locks"]["value"]
            .to_string()
            .contains(PRIVATE_PRUNE_LOCK)
    );
    assert!(!support_json.contains(PRIVATE_PRUNE_LOCK));
    assert!(!markdown.contains(PRIVATE_PRUNE_LOCK));
    eprintln!(
        "CINT-03 operator evidence: height={} work={work} pruneheight={} refusal=keep_window support_prune_locks=redacted",
        fixture::TIP,
        fixture::BOUNDARY + 1
    );
}
