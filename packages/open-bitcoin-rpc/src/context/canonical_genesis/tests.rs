// Parity breadcrumbs:
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/wallet/wallet.cpp

use super::*;
use crate::{RpcFailure, dispatch::dispatch, method::*};
use open_bitcoin_node::{
    FjallNodeStore, SyncNetwork,
    core::{
        chainstate::CoinsView,
        consensus::CanonicalChain,
        mempool::FeeRate,
        primitives::{Amount, Transaction},
        wallet::{AddressNetwork, BuildRequest, BuiltTransaction, DescriptorRole},
    },
};
use serde_json::{Value, json};
use std::{fs, path::PathBuf};

mod refusals;

// Keep inspection-child spawns from inheriting another scenario's live lock fd.
static PROCESS_SCENARIO: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn scenario() -> std::sync::MutexGuard<'static, ()> {
    PROCESS_SCENARIO.lock().expect("canonical process scenario")
}

const CHAINS: [(CanonicalChain, AddressNetwork, SyncNetwork, &str); 4] = [
    (
        CanonicalChain::Mainnet,
        AddressNetwork::Mainnet,
        SyncNetwork::Mainnet,
        "mainnet",
    ),
    (
        CanonicalChain::Testnet3,
        AddressNetwork::Testnet,
        SyncNetwork::Testnet,
        "testnet3",
    ),
    (
        CanonicalChain::Signet,
        AddressNetwork::Signet,
        SyncNetwork::Signet,
        "signet",
    ),
    (
        CanonicalChain::Regtest,
        AddressNetwork::Regtest,
        SyncNetwork::Regtest,
        "regtest",
    ),
];
type Context = ManagedRpcContext<FjallChainstateStore, FjallCoinsView>;

struct Source(PathBuf);
impl Source {
    fn new() -> Self {
        let mut nonce = [0u8; 16];
        getrandom::fill(&mut nonce).expect("nonce");
        Self(
            std::env::temp_dir()
                .canonicalize()
                .expect("temp parent")
                .join(format!("phase166-rpc-{:x}", u128::from_le_bytes(nonce))),
        )
    }
}
impl Drop for Source {
    fn drop(&mut self) {
        if self.0.exists() {
            fs::remove_dir_all(&self.0).expect("owned cleanup");
        }
    }
}
fn config(index: usize, mode: BasicFilterIndexSetting) -> RuntimeConfig {
    let mut config = RuntimeConfig {
        chain: CHAINS[index].1,
        block_filter_index: mode,
        ..Default::default()
    };
    config.sync.runtime.network = CHAINS[index].2;
    config
}
fn open(
    source: &Source,
    index: usize,
    mode: BasicFilterIndexSetting,
) -> (Context, DurableSyncRuntime) {
    let opened =
        FjallNodeStore::open_canonical_genesis(&source.0, CHAINS[index].0).expect("sealed opener");
    Context::from_canonical_genesis_runtime_config(&config(index, mode), opened)
        .expect("canonical context")
}
fn vector(index: usize) -> Value {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../../../docs/parity/fixtures/phase166-genesis-vectors.json"
    ))
    .expect("frozen vectors");
    fixture["genesis"]
        .as_array()
        .expect("vectors")
        .iter()
        .find(|row| row["chain"] == CHAINS[index].3)
        .expect("selector")
        .clone()
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
fn assert_refusal<T>(result: Result<T, RpcFailure>) {
    assert_eq!(
        result.map(|_| ()),
        Err(super::super::wallet_state::unavailable_wallet())
    );
}
fn assert_genesis(
    context: &mut Context,
    runtime: &DurableSyncRuntime,
    index: usize,
    enabled: bool,
) {
    let expected = vector(index);
    let tip = context.maybe_chain_tip().expect("tip").expect("genesis");
    let spec = CHAINS[index].0.spec();
    let proof = open_bitcoin_node::core::consensus::chain_spec::CanonicalGenesisProof::prove(
        spec,
        &spec.genesis_block(),
    )
    .expect("selected genesis");
    assert_eq!(context.consensus_params(), consensus_params(&proof));
    assert_eq!(tip.height, 0);
    assert_eq!(hex(tip.block_hash.as_bytes()), expected["block_hash_raw"]);
    assert_eq!(tip.chain_work.to_hex(), expected["proof_hex"]);
    assert_eq!(
        runtime.store().coins_view().best_block().expect("coins B"),
        Some(tip.block_hash)
    );
    assert!(
        runtime
            .store()
            .coins_view()
            .complete_snapshot()
            .expect("coins")
            .coins
            .is_empty()
    );
    assert_eq!(context.network_info().expect("network").connected_peers, 0);
    assert!(!context.network_info().expect("network").relay);
    assert_refusal(context.wallet_info());
    let info = dispatch(context, MethodCall::GetBlockchainInfo(Default::default()))
        .expect("blockchain RPC");
    assert_eq!(info["blocks"], 0);
    assert_eq!(info["headers"], 0);
    assert_eq!(info["bestblockhash"], expected["block_hash_display"]);
    let summary = dispatch(
        context,
        MethodCall::GetIndexInfo(GetIndexInfoRequest {
            maybe_index_name: None,
        }),
    )
    .expect("index RPC");
    let filter = dispatch(
        context,
        MethodCall::GetBlockFilter(GetBlockFilterRequest {
            block_hash: tip.block_hash,
            filter_type: BlockFilterSelection::Basic,
        }),
    );
    if enabled {
        assert_eq!(
            summary[BASIC_FILTER_INDEX_NAME],
            json!({"synced":true,"best_block_height":0})
        );
        assert_eq!(
            filter.expect("filter RPC"),
            json!({"filter":expected["basic"]["encoded"], "header":expected["basic"]["header_display"]})
        );
        let open_bitcoin_node::BasicFilterQuery::Found(record) = runtime
            .network_handle()
            .basic_filter_query(tip.block_hash)
            .expect("stored record")
        else {
            panic!("saved record");
        };
        assert_eq!(hex(record.encoded_bytes()), expected["basic"]["encoded"]);
        assert_eq!(
            hex(record.identity().filter_hash().as_bytes()),
            expected["basic"]["hash_raw"]
        );
        assert_eq!(
            hex(record.identity().filter_header().as_bytes()),
            expected["basic"]["header_raw"]
        );
        assert_eq!(
            hex(record.identity().previous_header().as_bytes()),
            expected["basic"]["predecessor_raw"]
        );
        assert!(
            runtime
                .network_handle()
                .maybe_basic_index_summary()
                .expect("owner")
                .is_some()
        );
    } else {
        assert_eq!(summary, json!({}));
        assert!(filter.is_err());
        assert!(
            runtime
                .network_handle()
                .maybe_basic_index_summary()
                .expect("no owner")
                .is_none()
        );
    }
}
fn fresh(index: usize) {
    let _scenario = scenario();
    // Arrange / Act: real sealed storage, selected params and runtime.
    let source = Source::new();
    let opened = FjallNodeStore::open_canonical_genesis(&source.0, CHAINS[index].0)
        .expect("canonical opener");
    let position = opened.position().clone();
    let params = consensus_params(opened.genesis());
    INITIALIZATIONS.set(0);
    let (mut context, runtime) = Context::from_canonical_genesis_runtime_config(
        &config(index, BasicFilterIndexSetting::Basic),
        opened,
    )
    .expect("canonical context");
    // Assert: retain all initial factory assertions and extend actual dispatch.
    assert_eq!(context.consensus_params(), params);
    assert_eq!(INITIALIZATIONS.get(), 1, "positive initialization control");
    assert_eq!(
        context.maybe_chain_tip().expect("tip"),
        Some(position.clone())
    );
    assert_eq!(
        runtime.store().coins_view().best_block().expect("B"),
        Some(position.block_hash)
    );
    assert_refusal(context.require_wallet_selection());
    assert_genesis(&mut context, &runtime, index, true);
}

#[test]
fn phase166_genesis_context_mainnet() {
    fresh(0);
}
#[test]
fn phase166_genesis_context_testnet() {
    fresh(1);
}
#[test]
fn phase166_genesis_context_signet() {
    fresh(2);
}
#[test]
fn phase166_genesis_context_regtest() {
    fresh(3);
}

#[test]
fn phase166_genesis_rpc_hash_display_roundtrips_both_metadata_branches() {
    let _scenario = scenario();
    for index in 0..CHAINS.len() {
        // Arrange
        let source = Source::new();
        let (mut context, runtime) = open(&source, index, BasicFilterIndexSetting::Basic);
        let expected = vector(index);
        assert!(
            runtime
                .store()
                .load_runtime_metadata()
                .expect("metadata")
                .and_then(|metadata| metadata.maybe_sync_state)
                .is_none()
        );
        // Act: capture both actual projectors before checking either display result.
        let first = dispatch(
            &mut context,
            MethodCall::GetBlockchainInfo(Default::default()),
        )
        .expect("unpersisted RPC");
        let state = runtime
            .durable_sync_state(open_bitcoin_node::SyncLifecycleState::Stopped, None, 0)
            .expect("proved state");
        runtime
            .persist_durable_sync_state(state)
            .expect("saved metadata");
        let second = dispatch(
            &mut context,
            MethodCall::GetBlockchainInfo(Default::default()),
        )
        .expect("persisted RPC");
        // Assert
        assert_eq!(
            [&first["bestblockhash"], &second["bestblockhash"]],
            [
                &expected["block_hash_display"],
                &expected["block_hash_display"]
            ]
        );
        for info in [first, second] {
            let request = normalize_getblockfilter(RequestParameters::Positional(vec![
                info["bestblockhash"].clone(),
            ]))
            .expect("real display normalizer");
            let filter = dispatch(&mut context, MethodCall::GetBlockFilter(request))
                .expect("roundtrip query");
            assert_eq!(
                filter,
                json!({"filter":expected["basic"]["encoded"],"header":expected["basic"]["header_display"]})
            );
        }
    }
}

#[test]
fn phase166_genesis_context_error_categories_are_fixed() {
    // Arrange
    let error =
        CanonicalContextError::GenesisRuntime(open_bitcoin_node::SyncRuntimeError::InvalidData {
            message: "PRIVATE path and payload".into(),
        });
    // Act / Assert
    assert_eq!(error.to_string(), "canonical_genesis_runtime_unavailable");
    assert_eq!(
        CanonicalContextError::InvalidGenesisConfiguration.to_string(),
        "canonical_genesis_configuration_unavailable"
    );
    assert!(matches!(
        error,
        CanonicalContextError::GenesisRuntime(
            open_bitcoin_node::SyncRuntimeError::InvalidData { .. }
        )
    ));
}

#[test]
fn phase166_genesis_context_actual_all_handles_reopen() {
    let _scenario = scenario();
    for (index, profile) in CHAINS.iter().enumerate() {
        // Arrange
        let source = Source::new();
        let (context, runtime) = open(&source, index, BasicFilterIndexSetting::Basic);
        let handle = runtime.network_handle();
        let store = runtime.store().clone();
        let position = context.maybe_chain_tip().expect("tip");
        let before = logical(&store);
        // Act / Assert: retaining any context/network/store owner blocks independent open.
        drop(runtime);
        assert!(FjallNodeStore::reopen_canonical_genesis_existing(&source.0, profile.0).is_err());
        drop(context);
        drop(handle);
        assert!(FjallNodeStore::reopen_canonical_genesis_existing(&source.0, profile.0).is_err());
        drop(store);
        let opened = FjallNodeStore::reopen_canonical_genesis_existing(&source.0, profile.0)
            .expect("real child-inspected reopen");
        assert_eq!(Some(opened.position().clone()), position);
        let (mut context, runtime) = Context::from_canonical_genesis_runtime_config(
            &config(index, BasicFilterIndexSetting::Unspecified),
            opened,
        )
        .expect("preserved saved Active");
        assert_eq!(logical(runtime.store()), before);
        assert_genesis(&mut context, &runtime, index, true);
    }
}

fn logical(store: &FjallNodeStore) -> String {
    format!("{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}",
        store.load_header_entries().expect("headers"), store.load_block_index_entries().expect("index"),
        store.coins_view().complete_snapshot().expect("coins"), store.load_chainstate_snapshot().expect("chain"),
        store.load_wallet_registry().expect("registry"), store.load_selected_wallet().expect("selection"),
        store.load_wallet_snapshot().expect("wallet"), store.load_wallet_rescan_jobs().expect("jobs"),
        store.load_runtime_metadata().expect("runtime"), store.load_recovery_marker().expect("recovery"),
        store.load_mempool_snapshot_with_limits(open_bitcoin_node::storage::fjall_store::MempoolSnapshotDecodeLimits::for_persisted_input().expect("limits")).expect("mempool"),
        store.load_prune_protection().expect("prune"), store.load_prune_locks().expect("locks"),
        store.load_have_pruned().expect("pruned"))
}

#[test]
fn phase166_genesis_context_disabled_and_unspecified_remain_absent() {
    let _scenario = scenario();
    for index in 0..4 {
        for mode in [
            BasicFilterIndexSetting::Disabled,
            BasicFilterIndexSetting::Unspecified,
        ] {
            // Arrange / Act
            let source = Source::new();
            let (mut context, runtime) = open(&source, index, mode);
            // Assert
            assert_genesis(&mut context, &runtime, index, false);
            assert!(
                runtime
                    .store()
                    .load_wallet_registry()
                    .expect("no registry")
                    .is_none()
            );
            assert!(
                runtime
                    .store()
                    .load_wallet_snapshot()
                    .expect("no wallet")
                    .is_none()
            );
            assert!(
                context
                    .network
                    .latest_mempool_recovery_summary()
                    .expect("no recovery")
                    .is_none()
            );
        }
    }
}

#[test]
fn phase166_genesis_context_effect_directory_is_proved_storage() {
    let _scenario = scenario();
    for (index, profile) in CHAINS.iter().enumerate() {
        for divergent in [false, true] {
            // Arrange
            let source = Source::new();
            let alternate = Source::new();
            let mut selected = config(index, BasicFilterIndexSetting::Disabled);
            selected.maybe_data_dir = divergent.then(|| alternate.0.clone());
            let opened =
                FjallNodeStore::open_canonical_genesis(&source.0, profile.0).expect("proof");
            // Act
            let (context, runtime) =
                Context::from_canonical_genesis_runtime_config(&selected, opened).expect("factory");
            // Assert
            assert_eq!(
                context.maybe_resource_governance_log_dir,
                Some(runtime.store().datadir().join("logs"))
            );
            assert!(!alternate.0.exists());
        }
    }
}

fn direct_wallet_refusals(context: &mut Context, store: &FjallNodeStore) {
    let before = logical(store);
    let request = BuildRequest {
        recipients: vec![],
        fee_rate: FeeRate::from_sats_per_kvb(1000),
        maybe_change_descriptor_id: None,
        maybe_lock_time: None,
        enable_rbf: false,
    };
    let built = BuiltTransaction {
        transaction: Transaction::default(),
        selected_inputs: vec![],
        fee: Amount::from_sats(0).expect("zero"),
        change_output_index: None,
    };
    let snapshot = context.blockchain_snapshot().expect("snapshot");
    assert_refusal(context.wallet_info());
    assert_refusal(context.selected_wallet_info());
    assert_refusal(context.selected_wallet_name());
    assert_refusal(context.require_wallet_selection());
    assert_refusal(context.wallet_balance(100));
    assert_refusal(context.wallet_utxos());
    assert_refusal(context.descriptor_address(0));
    assert_refusal(context.import_descriptor("PRIVATE", DescriptorRole::External, "invalid"));
    assert_refusal(context.allocate_receive_address());
    assert_refusal(context.allocate_change_address());
    assert_refusal(context.rescan_wallet(&snapshot));
    assert_refusal(context.build_transaction(&request, 100));
    assert_refusal(context.build_and_sign_transaction(&request, 100));
    assert_refusal(context.sign_transaction(&built));
    assert_refusal(context.wallet_snapshot());
    assert_refusal(context.wallet_rescan_job());
    assert_refusal(context.wallet_freshness());
    assert_refusal(context.selected_wallet_from_store(store));
    let calls = std::cell::Cell::new(0);
    assert_refusal(context.rescan_wallet_range_with(
        None,
        None,
        |_| {
            calls.set(calls.get() + 1);
            panic!("guard")
        },
        |_, _, _, _, _| {
            calls.set(calls.get() + 1);
            panic!("prepare")
        },
        |_, _, _| {
            calls.set(calls.get() + 1);
            panic!("save_failed")
        },
    ));
    assert_eq!(calls.get(), 0);
    context.set_request_wallet_name(Some("PRIVATE".into()));
    assert_eq!(context.request_wallet_name(), None);
    context.clear_request_wallet_name();
    assert_eq!(logical(store), before);
}

#[test]
fn phase166_genesis_context_wallet_http_and_dispatch_refuse_without_effects() {
    let _scenario = scenario();
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("HTTP test runtime")
        .block_on(async {
            use axum::{
                body::to_bytes,
                http::{HeaderMap, HeaderValue, Method},
            };
            use base64::Engine as _;
            for index in 0..4 {
                // Arrange
                let source = Source::new();
                let (mut context, runtime) = open(&source, index, BasicFilterIndexSetting::Basic);
                let before = logical(runtime.store());
                direct_wallet_refusals(&mut context, runtime.store());
                // Act / Assert: actual direct dispatch preserves the existing per-item import protocol.
                for method in [
                    "getwalletinfo",
                    "getnewaddress",
                    "getrawchangeaddress",
                    "listdescriptors",
                    "getbalances",
                    "listunspent",
                    "rescanblockchain",
                ] {
                    let call = normalize_method_call(method, RequestParameters::None)
                        .expect("wallet call");
                    assert_refusal(dispatch(&mut context, call));
                }
                let params =
                    json!({"requests":[{"desc":"invalid", "internal":false,"label":"PRIVATE"}]});
                let call = normalize_method_call(
                    "importdescriptors",
                    RequestParameters::Named(vec![("requests".into(), params["requests"].clone())]),
                )
                .expect("import call");
                let result =
                    dispatch(&mut context, call).expect("existing per-item failure response");
                assert_eq!(result["results"][0]["success"], false);
                assert_eq!(
                    result["results"][0]["maybe_message"],
                    "wallet unavailable: canonical genesis history is not qualified"
                );
                let state = crate::http::build_http_state(
                    crate::RpcAuthConfig::UserPassword {
                        username: "test".into(),
                        password: "test".into(),
                    },
                    context,
                )
                .expect("HTTP state");
                let mut headers = HeaderMap::new();
                headers.insert(
                    "authorization",
                    HeaderValue::from_str(&format!(
                        "Basic {}",
                        base64::engine::general_purpose::STANDARD.encode("test:test")
                    ))
                    .expect("auth"),
                );
                for method in [
                    "getwalletinfo",
                    "getnewaddress",
                    "importdescriptors",
                    "rescanblockchain",
                ] {
                    let params = if method == "importdescriptors" {
                        params.clone()
                    } else {
                        json!({})
                    };
                    let body = serde_json::to_vec(
                        &json!({"jsonrpc":"2.0","id":1,"method":method,"params":params}),
                    )
                    .expect("request");
                    let response = crate::http::handle_http_request(
                        &state,
                        "/",
                        Method::POST,
                        &headers,
                        &body,
                    )
                    .await;
                    let bytes = to_bytes(response.into_body(), 4096)
                        .await
                        .expect("bounded response");
                    let value: Value = serde_json::from_slice(&bytes).expect("JSON");
                    assert_eq!(value["error"]["code"], -4);
                    assert_eq!(
                        value["error"]["message"],
                        "wallet unavailable: canonical genesis history is not qualified"
                    );
                }
                assert_eq!(logical(runtime.store()), before);
                drop(state);
            }
        });
}
