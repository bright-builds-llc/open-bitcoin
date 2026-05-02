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

use open_bitcoin_node::core::chainstate::ChainstateSnapshot;
use open_bitcoin_node::core::consensus::{ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_node::core::mempool::PolicyConfig;
use open_bitcoin_node::core::network::{LocalPeerConfig, ServiceFlags, WireNetworkMessage};
use open_bitcoin_node::core::primitives::{Block, NetworkAddress, NetworkMagic, Transaction};
use open_bitcoin_node::core::wallet::AddressNetwork;
use open_bitcoin_node::network::{ManagedMempoolInfo, ManagedNetworkInfo};
use open_bitcoin_node::{DurableSyncState, FjallNodeStore};
use open_bitcoin_node::{
    ManagedNetworkError, ManagedPeerNetwork, ManagedWallet, MemoryChainstateStore,
    MemoryWalletStore,
};

use crate::config::RuntimeConfig;

use super::ManagedRpcContext;
use super::wallet_state::build_wallet_state;

impl ManagedRpcContext {
    pub fn new(
        chain: AddressNetwork,
        consensus_params: ConsensusParams,
        verify_flags: ScriptVerifyFlags,
        network: ManagedPeerNetwork<MemoryChainstateStore>,
        wallet: ManagedWallet<MemoryWalletStore>,
    ) -> Self {
        Self {
            chain,
            consensus_params,
            verify_flags,
            network,
            maybe_durable_sync_state: None,
            wallet_state: super::wallet_state::WalletState::Local(wallet),
        }
    }

    pub fn from_runtime_config(config: &RuntimeConfig) -> Self {
        let consensus_params = ConsensusParams {
            coinbase_maturity: config.wallet.coinbase_maturity,
            ..ConsensusParams::default()
        };
        let local_config = LocalPeerConfig {
            magic: network_magic(config.chain),
            services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
            address: NetworkAddress {
                services: 0,
                address_bytes: [0_u8; 16],
                port: default_p2p_port(config.chain),
            },
            nonce: 0,
            relay: true,
            user_agent: "/open-bitcoin:0.1.0/".to_string(),
        };
        let managed_network = ManagedPeerNetwork::new(
            MemoryChainstateStore::default(),
            local_config,
            PolicyConfig::default(),
        );
        match build_wallet_state(config) {
            super::wallet_state::WalletState::Local(wallet) => Self {
                chain: config.chain,
                consensus_params,
                verify_flags: default_verify_flags(),
                network: managed_network,
                maybe_durable_sync_state: load_durable_sync_state(config),
                wallet_state: super::wallet_state::WalletState::Local(wallet),
            },
            super::wallet_state::WalletState::DurableNamedRegistry {
                store,
                maybe_request_wallet_name,
            } => Self {
                chain: config.chain,
                consensus_params,
                verify_flags: default_verify_flags(),
                network: managed_network,
                maybe_durable_sync_state: store
                    .load_runtime_metadata()
                    .ok()
                    .flatten()
                    .and_then(|metadata| metadata.maybe_sync_state),
                wallet_state: super::wallet_state::WalletState::DurableNamedRegistry {
                    store,
                    maybe_request_wallet_name,
                },
            },
        }
    }

    pub fn for_local_operator(network: AddressNetwork) -> Self {
        Self::from_runtime_config(&RuntimeConfig {
            chain: network,
            ..RuntimeConfig::default()
        })
    }

    pub fn chain(&self) -> AddressNetwork {
        self.chain
    }

    pub fn chain_name(&self) -> &'static str {
        match self.chain {
            AddressNetwork::Mainnet => "main",
            AddressNetwork::Testnet => "test",
            AddressNetwork::Signet => "signet",
            AddressNetwork::Regtest => "regtest",
        }
    }

    pub fn consensus_params(&self) -> ConsensusParams {
        self.consensus_params
    }

    pub fn verify_flags(&self) -> ScriptVerifyFlags {
        self.verify_flags
    }

    pub fn coinbase_maturity(&self) -> u32 {
        self.consensus_params.coinbase_maturity
    }

    pub fn blockchain_snapshot(&self) -> ChainstateSnapshot {
        self.network.chainstate_snapshot()
    }

    pub fn maybe_chain_tip(&self) -> Option<open_bitcoin_node::core::chainstate::ChainPosition> {
        self.network.maybe_chain_tip()
    }

    pub fn maybe_durable_sync_state(&self) -> Option<&DurableSyncState> {
        self.maybe_durable_sync_state.as_ref()
    }

    pub fn mempool_info(&self) -> ManagedMempoolInfo {
        self.network.mempool_info()
    }

    pub fn network_info(&self) -> ManagedNetworkInfo {
        self.network.network_info()
    }

    pub fn add_inbound_peer(&mut self, peer_id: u64) -> Result<(), ManagedNetworkError> {
        self.network.add_inbound_peer(peer_id)
    }

    pub fn connect_outbound_peer(
        &mut self,
        peer_id: u64,
        timestamp: i64,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkError> {
        self.network.connect_outbound_peer(peer_id, timestamp)
    }

    pub fn receive_network_message(
        &mut self,
        peer_id: u64,
        message: WireNetworkMessage,
        timestamp: i64,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkError> {
        self.network.receive_message(
            peer_id,
            message,
            timestamp,
            self.verify_flags,
            self.consensus_params,
        )
    }

    pub fn connect_local_block(
        &mut self,
        block: &Block,
    ) -> Result<open_bitcoin_node::core::chainstate::ChainPosition, ManagedNetworkError> {
        self.network
            .connect_local_block(block, self.verify_flags, self.consensus_params)
    }

    pub fn submit_local_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<open_bitcoin_node::core::mempool::AdmissionResult, ManagedNetworkError> {
        self.network
            .submit_local_transaction(transaction, self.verify_flags, self.consensus_params)
    }
}

fn load_durable_sync_state(config: &RuntimeConfig) -> Option<DurableSyncState> {
    let data_dir = config.maybe_data_dir.as_ref()?;
    let store = FjallNodeStore::open(data_dir).ok()?;
    let metadata = store.load_runtime_metadata().ok()??;
    metadata.maybe_sync_state
}

pub(super) fn default_verify_flags() -> ScriptVerifyFlags {
    ScriptVerifyFlags::P2SH
        | ScriptVerifyFlags::STRICTENC
        | ScriptVerifyFlags::DERSIG
        | ScriptVerifyFlags::LOW_S
        | ScriptVerifyFlags::NULLDUMMY
        | ScriptVerifyFlags::SIGPUSHONLY
        | ScriptVerifyFlags::MINIMALDATA
        | ScriptVerifyFlags::CLEANSTACK
        | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
        | ScriptVerifyFlags::CHECKSEQUENCEVERIFY
        | ScriptVerifyFlags::WITNESS
        | ScriptVerifyFlags::MINIMALIF
        | ScriptVerifyFlags::NULLFAIL
        | ScriptVerifyFlags::WITNESS_PUBKEYTYPE
        | ScriptVerifyFlags::TAPROOT
}

pub(super) fn network_magic(chain: AddressNetwork) -> NetworkMagic {
    match chain {
        AddressNetwork::Mainnet => NetworkMagic::MAINNET,
        AddressNetwork::Testnet => NetworkMagic::from_bytes([0x0b, 0x11, 0x09, 0x07]),
        AddressNetwork::Signet => NetworkMagic::from_bytes([0x0a, 0x03, 0xcf, 0x40]),
        AddressNetwork::Regtest => NetworkMagic::from_bytes([0xfa, 0xbf, 0xb5, 0xda]),
    }
}

pub(super) fn default_p2p_port(chain: AddressNetwork) -> u16 {
    match chain {
        AddressNetwork::Mainnet => 8333,
        AddressNetwork::Testnet => 18_333,
        AddressNetwork::Signet => 38_333,
        AddressNetwork::Regtest => 18_444,
    }
}
