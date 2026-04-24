use std::env;

use open_bitcoin_node::core::wallet::AddressNetwork;
use open_bitcoin_rpc::{
    ManagedRpcContext, RpcAuthConfig,
    http::{build_http_state, router},
};
use open_bitcoin_test_harness::{
    ExpectedOutcome, FunctionalCase, RpcHttpTarget, run_suite, skipped_suite,
    write_reports_from_env,
};
use serde_json::json;

const RPC_USERNAME: &str = "alice";
const RPC_PASSWORD: &str = "secret";
const SUITE_NAME: &str = "rpc-black-box-parity";

fn functional_cases() -> Vec<FunctionalCase> {
    vec![
        FunctionalCase {
            name: "getblockchaininfo shape",
            method: "getblockchaininfo",
            params: json!([]),
            expected: ExpectedOutcome::ResultHasKeys(vec![
                "chain",
                "blocks",
                "headers",
                "initialblockdownload",
            ]),
        },
        FunctionalCase {
            name: "getnetworkinfo shape",
            method: "getnetworkinfo",
            params: json!([]),
            expected: ExpectedOutcome::ResultHasKeys(vec![
                "version",
                "subversion",
                "protocolversion",
                "connections",
            ]),
        },
        FunctionalCase {
            name: "getmempoolinfo shape",
            method: "getmempoolinfo",
            params: json!([]),
            expected: ExpectedOutcome::ResultHasKeys(vec!["size", "bytes", "loaded"]),
        },
        FunctionalCase {
            name: "unknown method error shape",
            method: "openbitcoin_does_not_exist",
            params: json!([]),
            expected: ExpectedOutcome::ErrorCode(-32601),
        },
    ]
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn same_black_box_suite_targets_open_bitcoin_rpc() {
    // Arrange
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("listener should bind");
    let address = listener.local_addr().expect("local address");
    let context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);
    let state = build_http_state(
        RpcAuthConfig::user_password(RPC_USERNAME, RPC_PASSWORD),
        context,
    )
    .expect("state should build");
    let server = tokio::spawn(async move {
        axum::serve(listener, router(state))
            .await
            .expect("server should run");
    });
    let mut target = RpcHttpTarget::new(
        "open-bitcoin",
        address.to_string(),
        RPC_USERNAME,
        RPC_PASSWORD,
    );
    let cases = functional_cases();

    // Act
    let report = run_suite(SUITE_NAME, &mut target, &cases);
    write_reports_from_env(&report).expect("report write should succeed");
    server.abort();

    // Assert
    assert!(report.passed(), "{report:#?}");
    assert_eq!(report.outcomes.len(), cases.len());
}

#[test]
fn same_black_box_suite_can_target_configured_knots_rpc() {
    // Arrange
    let maybe_addr = env::var("OPEN_BITCOIN_KNOTS_RPC_ADDR").ok();
    let maybe_user = env::var("OPEN_BITCOIN_KNOTS_RPC_USER").ok();
    let maybe_password = env::var("OPEN_BITCOIN_KNOTS_RPC_PASSWORD").ok();
    let cases = functional_cases();

    let (Some(addr), Some(user), Some(password)) = (maybe_addr, maybe_user, maybe_password) else {
        let report = skipped_suite(
            SUITE_NAME,
            "bitcoin-knots",
            "set OPEN_BITCOIN_KNOTS_RPC_ADDR, OPEN_BITCOIN_KNOTS_RPC_USER, and OPEN_BITCOIN_KNOTS_RPC_PASSWORD to run the same suite against Knots",
        );
        write_reports_from_env(&report).expect("skipped report should write");
        return;
    };
    let mut target = RpcHttpTarget::new("bitcoin-knots", addr, user, password);

    // Act
    let report = run_suite(SUITE_NAME, &mut target, &cases);
    write_reports_from_env(&report).expect("report write should succeed");

    // Assert
    assert!(report.passed(), "{report:#?}");
    assert_eq!(report.outcomes.len(), cases.len());
}
