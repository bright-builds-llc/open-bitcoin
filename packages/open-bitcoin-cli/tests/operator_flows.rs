// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/rpc/client.cpp
// - packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py

use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

use open_bitcoin_cli::operator::migration::migration_deviation_definitions;
use open_bitcoin_node::core::{
    consensus::{
        block_hash, block_merkle_root, check_block_header, crypto::hash160, transaction_txid,
    },
    primitives::{
        Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
        TransactionInput, TransactionOutput, Txid,
    },
    wallet::{AddressNetwork, DescriptorRole, Wallet},
};
use open_bitcoin_rpc::{
    ManagedRpcContext, RpcErrorCode, RpcErrorDetail, RpcFailure,
    config::{RuntimeConfig, WalletRuntimeConfig},
    dispatch::dispatch,
    method::{RequestParameters, normalize_method_call},
};
use serde_json::{Value, json};

#[path = "operator_flows/rpc_harness.rs"]
mod rpc_harness;
use rpc_harness::*;
#[path = "operator_flows/domain_fixtures.rs"]
mod domain_fixtures;
use domain_fixtures::*;
#[path = "operator_flows/wallet_and_parity.rs"]
mod wallet_and_parity;
