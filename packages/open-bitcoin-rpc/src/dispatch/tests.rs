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

use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use serde_json::json;

use open_bitcoin_network::{
    AddressAnnouncement, AddressList, InboundAdmissionSlotClass, InboundListenerConfig,
    ParsedPeerPermissionClass, PeerConnectionClass, PermissionEffectLabel, RelayActivationConfig,
    RelayPermissionEffectLabel, VersionMessage,
};
use open_bitcoin_node::{
    DurableSyncState, FjallNodeStore, ManagedNetworkError, ManagedPeerNetwork, ManagedWallet,
    MemoryChainstateStore, MemoryWalletStore, PersistMode, RuntimeMetadata, WalletRegistry,
    core::{
        chainstate::{ChainPosition, ChainstateSnapshot, Coin},
        codec::{TransactionEncoding, encode_transaction, parse_transaction},
        consensus::{
            ConsensusParams, ScriptVerifyFlags, block_hash, block_merkle_root, check_block_header,
            crypto::hash160, transaction_txid,
        },
        mempool::{
            FeeRate, IncrementalRelayFeeRate, MempoolAcceptanceTime, MempoolCapacity,
            MempoolOrigin, PackageShapeError, PolicyConfig, PolicyTime, RelayIntent,
            RollingMempoolFeeRate, StaticRelayFeeRate,
        },
        network::{LocalPeerConfig, ServiceFlags, WireNetworkMessage},
        primitives::{
            Amount, Block, BlockHash, BlockHeader, NetworkAddress, NetworkMagic, OutPoint,
            ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput, Txid,
        },
        wallet::{AddressNetwork, DescriptorRole, SingleKeyDescriptor, Wallet},
    },
    status::{
        BestKnownTipSource, BestKnownTipStatus, ChainTipStatus, FieldAvailability,
        INBOUND_STATUS_UNAVAILABLE_REASON, InboundPeerServingStatus, NoProgressDiagnosis,
        PeerCounts, PeerStatus, StayCurrentStatus, SyncAttemptCounters, SyncConfiguredTargets,
        SyncLagStatus, SyncLifecycleState, SyncProgress, SyncProgressSignal,
        SyncReconcileProgressStatus, SyncRecoveryCategory, SyncReorgEvidence, SyncResourcePressure,
        SyncStatus, SyncStopReasonStatus, TipFreshnessStatus,
    },
};

use crate::{
    DaemonSyncControl, DaemonSyncControlAction, DaemonSyncControlReceiver, ManagedRpcContext,
    RpcErrorCode, RpcFailureKind,
    config::{RuntimeConfig, WalletRuntimeConfig},
    dispatch::dispatch,
    inbound_listener::InboundListenerEvidence,
    method::{
        BuildAndSignTransactionRequest, DeriveAddressesRequest, GetBalancesRequest,
        GetBlockchainInfoRequest, GetMempoolInfoRequest, GetNetworkInfoRequest,
        GetWalletInfoRequest, ImportDescriptorsRequest, ListUnspentRequest, MethodCall,
        OpenBitcoinNetworkStatusRequest, OpenBitcoinSyncPauseRequest, OpenBitcoinSyncResumeRequest,
        OpenBitcoinSyncStatusRequest, RescanBlockchainRequest, SendRawTransactionRequest,
        SendToAddressRequest, TransactionRecipient,
    },
};

const EASY_BITS: u32 = 0x207f_ffff;
const RANGED_TPRV: &str = "tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK";
const RANGED_TPUB: &str = "tpubD6NzVbkrYhZ4WaWSyoBvQwbpLkojyoTZPRsgXELWz3Popb3qkjcJyJUGLnL4qHHoQvao8ESaAstxYSnhyswJ76uZPStJRJCTKvosUCJZL5B";

use super::{network_error_to_failure, node};

mod chain_fixtures;
mod network_errors;
mod network_fixtures;
mod network_status_admission;
mod network_status_schema;
mod node_info;
mod permissions;
mod storage_fixtures;
mod sync_control;
mod sync_status;
mod transaction_methods;
mod wallet_fixtures;
mod wallet_methods;
