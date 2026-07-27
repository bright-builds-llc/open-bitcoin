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

use std::{
    collections::VecDeque,
    env, fs,
    path::PathBuf,
    process,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use ::open_bitcoin_rpc::{
    ManagedRpcContext, RpcAuthConfig, RuntimeConfig,
    dispatch::dispatch,
    http::{build_http_state, build_http_state_with_shared_context, router},
    inbound_listener::{activate_inbound_listener, start_inbound_accept_loop},
    method::{GetBlockchainInfoRequest, MethodCall},
};
use open_bitcoin_codec::BIP152_COMPACT_BLOCKS_VERSION;
use open_bitcoin_network::{
    BlockRelayActivationPolicy, BlockServingActivationConfig, CompactRelayActivationConfig,
    HeadersMessage, InboundListenerConfig, InventoryList, ParsedNetworkMessage,
    ParsedPeerPermissionClass, PeerPermissionClassRegistry, RelayActivationConfig, VersionMessage,
    WireNetworkMessage,
};
use open_bitcoin_node::{
    DurableSyncRuntime, FieldAvailability, FjallNodeStore, ResolvedSyncPeerAddress,
    SyncLifecycleState, SyncNetwork, SyncPeerAddress, SyncPeerReceiveOutcome, SyncPeerSession,
    SyncRuntimeConfig, SyncRuntimeError, SyncTransport,
    core::{
        codec::parse_message_header,
        consensus::{
            block_hash, block_merkle_root, check_block_header, crypto::hash160, transaction_txid,
        },
        primitives::{
            Amount, Block, BlockHash, BlockHeader, InventoryType, InventoryVector, NetworkMagic,
            OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
            Txid,
        },
        wallet::AddressNetwork,
    },
};
use open_bitcoin_test_harness::{
    ExpectedOutcome, FunctionalCase, HarnessTarget, RpcHttpTarget, run_suite, skipped_suite,
    write_reports_from_env,
};
use serde_json::json;
use tokio::net::TcpStream;

#[path = "black_box_parity/fixtures.rs"]
mod fixtures;
#[path = "black_box_parity/knots_rpc.rs"]
mod knots_rpc;
#[path = "black_box_parity/open_bitcoin_rpc.rs"]
mod open_bitcoin_rpc;
#[path = "black_box_parity/phase127_composition.rs"]
mod phase127_composition;
