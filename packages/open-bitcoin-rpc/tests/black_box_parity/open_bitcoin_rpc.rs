// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/src/node/context.h
// - packages/bitcoin-knots/src/rpc/server_util.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use super::fixtures::*;
use super::*;

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
