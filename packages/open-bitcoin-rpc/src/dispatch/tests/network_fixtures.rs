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

pub(super) fn resource_fee_evidence_context() -> (ManagedRpcContext, usize) {
    let local_config = LocalPeerConfig {
        magic: NetworkMagic::MAINNET,
        services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
        address: NetworkAddress {
            services: 0,
            address_bytes: [0_u8; 16],
            port: 18_444,
        },
        nonce: 13_009,
        relay: true,
        user_agent: "/open-bitcoin:rpc-mempool-evidence/".to_string(),
    };
    let policy = PolicyConfig {
        mempool_capacity: MempoolCapacity::new(12_345_678),
        static_relay_fee_rate: StaticRelayFeeRate::new(FeeRate::from_sats_per_kvb(1_000)),
        incremental_relay_fee_rate: IncrementalRelayFeeRate::new(FeeRate::from_sats_per_kvb(7_000)),
        ..PolicyConfig::default()
    };
    let mut network =
        ManagedPeerNetwork::new(MemoryChainstateStore::default(), local_config, policy);
    let consensus = ConsensusParams {
        coinbase_maturity: 1,
        ..ConsensusParams::default()
    };
    let genesis = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        0,
        500_000_000,
        p2sh_script(),
    );
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000, p2sh_script());
    network
        .connect_local_block(&genesis, rpc_verify_flags(), consensus)
        .expect("genesis");
    network
        .connect_local_block(&spendable, rpc_verify_flags(), consensus)
        .expect("spendable");
    let transaction = script_heavy_spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("txid"),
        499_998_000,
    );
    let expected_virtual_size =
        open_bitcoin_node::core::mempool::transaction_weight_and_virtual_size(&transaction)
            .expect("weight")
            .1;
    network
        .submit_local_transaction_outcome_at(
            transaction,
            rpc_verify_flags(),
            consensus,
            60,
            RelayIntent::NotRequested,
        )
        .expect("submit");
    network
        .set_rolling_mempool_fee_rate(RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(
            3_000,
        )))
        .expect("revision remains available");
    let wallet = ManagedWallet::from_store(
        MemoryWalletStore::default(),
        Wallet::new(AddressNetwork::Regtest),
    );
    let context = ManagedRpcContext::new(
        AddressNetwork::Regtest,
        consensus,
        rpc_verify_flags(),
        network,
        wallet,
    );
    (context, expected_virtual_size)
}

pub(super) fn inbound_context(max_peers: usize, reserved_slots: usize) -> ManagedRpcContext {
    ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Regtest,
        inbound: InboundListenerConfig {
            enabled: true,
            listen_addresses: vec!["127.0.0.1:18444".to_string()],
            max_peers,
            reserved_slots,
            allow_public: false,
            permission_classes: Default::default(),
        },
        wallet: WalletRuntimeConfig {
            coinbase_maturity: 1,
            ..WalletRuntimeConfig::default()
        },
        ..RuntimeConfig::default()
    })
}

pub(super) fn permission_context(classes: Vec<ParsedPeerPermissionClass>) -> ManagedRpcContext {
    permission_context_with_limits(classes, 8, 1)
}

pub(super) fn permission_context_with_limits(
    classes: Vec<ParsedPeerPermissionClass>,
    max_peers: usize,
    reserved_slots: usize,
) -> ManagedRpcContext {
    ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Regtest,
        inbound: InboundListenerConfig {
            enabled: true,
            listen_addresses: vec!["127.0.0.1:18444".to_string()],
            max_peers,
            reserved_slots,
            allow_public: false,
            permission_classes: open_bitcoin_network::PeerPermissionClassRegistry::new(classes),
        },
        wallet: WalletRuntimeConfig {
            coinbase_maturity: 1,
            ..WalletRuntimeConfig::default()
        },
        ..RuntimeConfig::default()
    })
}

pub(super) fn parsed_permission_class(
    name: &str,
    address: &str,
    permissions: &[&str],
) -> ParsedPeerPermissionClass {
    ParsedPeerPermissionClass::parse(name, [address], permissions.iter().copied())
        .expect("permission class should parse")
}

pub(super) fn address_boundary_context() -> ManagedRpcContext {
    ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Regtest,
        inbound: InboundListenerConfig {
            enabled: true,
            listen_addresses: vec!["8.8.8.8:18444".to_string(), "127.0.0.1:18445".to_string()],
            max_peers: 8,
            reserved_slots: 1,
            allow_public: true,
            permission_classes: open_bitcoin_network::PeerPermissionClassRegistry::new(vec![
                parsed_permission_class(
                    "operator-private-addr-secret",
                    "127.0.0.1",
                    &["in", "addr"],
                ),
            ]),
        },
        wallet: WalletRuntimeConfig {
            coinbase_maturity: 1,
            ..WalletRuntimeConfig::default()
        },
        ..RuntimeConfig::default()
    })
}

pub(super) fn address_announcement(
    time_unix_seconds: u64,
    address: NetworkAddress,
) -> AddressAnnouncement {
    AddressAnnouncement {
        time_unix_seconds: time_unix_seconds as u32,
        address,
    }
}

pub(super) fn public_ipv4_network_address(a: u8, b: u8, c: u8, d: u8, port: u16) -> NetworkAddress {
    NetworkAddress {
        services: ServiceFlags::NETWORK.bits(),
        address_bytes: ipv4_mapped_address_bytes([a, b, c, d]),
        port,
    }
}

pub(super) fn ipv4_mapped_address_bytes(octets: [u8; 4]) -> [u8; 16] {
    let mut bytes = [0_u8; 16];
    bytes[10] = 0xff;
    bytes[11] = 0xff;
    bytes[12..].copy_from_slice(&octets);
    bytes
}

pub(super) fn node_context_with_chain_and_mempool() -> ManagedRpcContext {
    let mut context = empty_context();
    let genesis = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        0,
        500_000_000,
        p2sh_script(),
    );
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000, p2sh_script());
    context.connect_local_block(&genesis).expect("genesis");
    context.connect_local_block(&spendable).expect("spendable");
    context.add_inbound_peer(7).expect("peer");
    context
        .receive_network_message(7, WireNetworkMessage::WtxidRelay, 1)
        .expect("wtxidrelay");
    context
        .receive_network_message(7, WireNetworkMessage::SendHeaders, 1)
        .expect("sendheaders");
    context.connect_outbound_peer(8, 2).expect("outbound");
    let transaction = spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("txid"),
        499_999_000,
    );
    context
        .submit_local_transaction(transaction)
        .expect("submit");
    context
}
