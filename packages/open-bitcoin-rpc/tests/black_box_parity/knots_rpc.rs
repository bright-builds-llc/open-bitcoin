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
