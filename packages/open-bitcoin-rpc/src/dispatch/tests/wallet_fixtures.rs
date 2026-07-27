// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use super::chain_fixtures::*;
use super::*;

pub(super) fn funded_wallet_context() -> ManagedRpcContext {
    let mut context = empty_context();
    context
        .import_descriptor(
            "receive",
            DescriptorRole::External,
            "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
        )
        .expect("receive");
    context
        .import_descriptor(
            "change",
            DescriptorRole::Internal,
            "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))",
        )
        .expect("change");
    let snapshot = funded_snapshot(&wallet_with_descriptors());
    context.rescan_wallet(&snapshot).expect("rescan");
    context
}

pub(super) fn spendable_send_context() -> ManagedRpcContext {
    let mut context = empty_context();
    context
        .import_descriptor(
            "receive",
            DescriptorRole::External,
            "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
        )
        .expect("receive");
    context
        .import_descriptor(
            "change",
            DescriptorRole::Internal,
            "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))",
        )
        .expect("change");
    let receive_script = context
        .descriptor_address(0)
        .expect("receive address")
        .script_pubkey;
    let genesis = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        0,
        500_000_000,
        p2sh_script(),
    );
    let funding = build_block(block_hash(&genesis.header), 1, 75_000, receive_script);
    context.connect_local_block(&genesis).expect("genesis");
    context.connect_local_block(&funding).expect("funding");
    let snapshot = context
        .blockchain_snapshot()
        .expect("authoritative chainstate snapshot");
    context.rescan_wallet(&snapshot).expect("rescan");
    context
}
