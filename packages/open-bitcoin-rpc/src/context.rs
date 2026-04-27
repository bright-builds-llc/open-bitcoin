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

use open_bitcoin_node::core::network::WireNetworkMessage;
use open_bitcoin_node::core::{
    chainstate::{ChainPosition, ChainstateSnapshot},
    consensus::{ConsensusParams, ScriptVerifyFlags},
    mempool::{AdmissionResult, PolicyConfig},
    network::{LocalPeerConfig, ServiceFlags},
    primitives::{Block, NetworkAddress, NetworkMagic, Transaction},
    wallet::{
        Address, AddressNetwork, BuildRequest, BuiltTransaction, DescriptorRole, Wallet,
        WalletBalance, WalletError, WalletUtxo,
    },
};
use open_bitcoin_node::network::{ManagedMempoolInfo, ManagedNetworkInfo};
use open_bitcoin_node::wallet::ManagedWalletInfo;
use open_bitcoin_node::{
    FjallNodeStore, ManagedNetworkError, ManagedPeerNetwork, ManagedWallet, MemoryChainstateStore,
    MemoryWalletStore, PersistMode, StorageError, WalletRegistry, WalletRegistryError,
    WalletRescanFreshness, WalletRescanJob, WalletRescanJobState,
};

use crate::{
    config::{RuntimeConfig, WalletRuntimeScope},
    error::{RpcErrorCode, RpcErrorDetail, RpcFailure, RpcFailureKind},
};

pub struct ManagedRpcContext {
    chain: AddressNetwork,
    consensus_params: ConsensusParams,
    verify_flags: ScriptVerifyFlags,
    network: ManagedPeerNetwork<MemoryChainstateStore>,
    wallet_state: WalletState,
}

enum WalletState {
    Local(ManagedWallet<MemoryWalletStore>),
    DurableNamedRegistry {
        store: FjallNodeStore,
        maybe_request_wallet_name: Option<String>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletFreshnessKind {
    Fresh,
    Partial,
    Scanning,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletFreshnessView {
    pub scanning: bool,
    pub freshness: WalletFreshnessKind,
    pub maybe_scanned_through_height: Option<u32>,
    pub maybe_target_height: Option<u32>,
    pub maybe_next_height: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletRescanExecution {
    pub start_height: u32,
    pub stop_height: u32,
    pub freshness: WalletFreshnessView,
}

impl core::fmt::Debug for ManagedRpcContext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let wallet_mode = match &self.wallet_state {
            WalletState::Local(_) => "local",
            WalletState::DurableNamedRegistry { .. } => "durable",
        };
        f.debug_struct("ManagedRpcContext")
            .field("chain", &self.chain)
            .field("consensus_params", &self.consensus_params)
            .field("verify_flags", &self.verify_flags)
            .field("wallet_mode", &wallet_mode)
            .finish()
    }
}

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
            wallet_state: WalletState::Local(wallet),
        }
    }
}

impl ManagedRpcContext {
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
            WalletState::Local(wallet) => Self::new(
                config.chain,
                consensus_params,
                default_verify_flags(),
                managed_network,
                wallet,
            ),
            WalletState::DurableNamedRegistry {
                store,
                maybe_request_wallet_name,
            } => Self {
                chain: config.chain,
                consensus_params,
                verify_flags: default_verify_flags(),
                network: managed_network,
                wallet_state: WalletState::DurableNamedRegistry {
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

    pub fn maybe_chain_tip(&self) -> Option<ChainPosition> {
        self.network.maybe_chain_tip()
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
    ) -> Result<ChainPosition, ManagedNetworkError> {
        self.network
            .connect_local_block(block, self.verify_flags, self.consensus_params)
    }

    pub fn submit_local_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<AdmissionResult, ManagedNetworkError> {
        self.network
            .submit_local_transaction(transaction, self.verify_flags, self.consensus_params)
    }

    pub fn wallet_info(&self) -> ManagedWalletInfo {
        self.selected_wallet_info().unwrap_or(ManagedWalletInfo {
            network: self.chain,
            descriptor_count: 0,
            utxo_count: 0,
            maybe_tip_height: None,
            maybe_tip_median_time_past: None,
        })
    }

    pub fn selected_wallet_info(&self) -> Result<ManagedWalletInfo, RpcFailure> {
        match &self.wallet_state {
            WalletState::Local(wallet) => Ok(wallet.wallet_info()),
            WalletState::DurableNamedRegistry { store, .. } => {
                let wallet = self.selected_wallet_from_store(store)?;
                let snapshot = wallet.snapshot();
                Ok(ManagedWalletInfo {
                    network: snapshot.network,
                    descriptor_count: snapshot.descriptors.len(),
                    utxo_count: snapshot.utxos.len(),
                    maybe_tip_height: snapshot.maybe_tip_height,
                    maybe_tip_median_time_past: snapshot.maybe_tip_median_time_past,
                })
            }
        }
    }

    pub fn selected_wallet_name(&self) -> Result<Option<String>, RpcFailure> {
        match &self.wallet_state {
            WalletState::Local(_) => Ok(None),
            WalletState::DurableNamedRegistry { store, .. } => {
                let registry = load_wallet_registry(store)?;
                resolve_selected_wallet_name(self.request_wallet_name(), &registry).map(Some)
            }
        }
    }

    pub fn require_wallet_selection(&self) -> Result<(), RpcFailure> {
        match &self.wallet_state {
            WalletState::Local(_) => Ok(()),
            WalletState::DurableNamedRegistry { store, .. } => {
                let registry = load_wallet_registry(store)?;
                let _ = resolve_selected_wallet_name(self.request_wallet_name(), &registry)?;
                Ok(())
            }
        }
    }

    pub fn set_request_wallet_name(&mut self, maybe_wallet_name: Option<String>) {
        if let WalletState::DurableNamedRegistry {
            maybe_request_wallet_name,
            ..
        } = &mut self.wallet_state
        {
            *maybe_request_wallet_name = maybe_wallet_name;
        }
    }

    pub fn clear_request_wallet_name(&mut self) {
        self.set_request_wallet_name(None);
    }

    pub fn wallet_balance(&self, coinbase_maturity: u32) -> Result<WalletBalance, RpcFailure> {
        match &self.wallet_state {
            WalletState::Local(wallet) => wallet
                .balance(coinbase_maturity)
                .map_err(wallet_error_to_failure),
            WalletState::DurableNamedRegistry { store, .. } => self
                .selected_wallet_from_store(store)?
                .balance(coinbase_maturity)
                .map_err(wallet_error_to_failure),
        }
    }

    pub fn wallet_utxos(&self) -> Result<Vec<WalletUtxo>, RpcFailure> {
        match &self.wallet_state {
            WalletState::Local(wallet) => Ok(wallet.utxos().to_vec()),
            WalletState::DurableNamedRegistry { store, .. } => {
                Ok(self.selected_wallet_from_store(store)?.utxos().to_vec())
            }
        }
    }

    pub fn descriptor_address(&self, descriptor_id: u32) -> Result<Address, RpcFailure> {
        match &self.wallet_state {
            WalletState::Local(wallet) => wallet
                .wallet()
                .address_for_descriptor(descriptor_id)
                .map_err(wallet_error_to_failure),
            WalletState::DurableNamedRegistry { store, .. } => self
                .selected_wallet_from_store(store)?
                .address_for_descriptor(descriptor_id)
                .map_err(wallet_error_to_failure),
        }
    }

    pub fn import_descriptor(
        &mut self,
        label: impl Into<String>,
        role: DescriptorRole,
        descriptor_text: &str,
    ) -> Result<u32, RpcFailure> {
        match &mut self.wallet_state {
            WalletState::Local(wallet) => wallet
                .import_descriptor(label, role, descriptor_text)
                .map_err(wallet_error_to_failure),
            WalletState::DurableNamedRegistry {
                store,
                maybe_request_wallet_name,
            } => {
                let mut registry = load_wallet_registry(store)?;
                let wallet_name =
                    resolve_selected_wallet_name(maybe_request_wallet_name.as_deref(), &registry)?;
                let mut wallet = registry
                    .wallet(&wallet_name)
                    .map_err(wallet_registry_error_to_failure)?;
                let descriptor_id = wallet
                    .import_descriptor(label, role, descriptor_text)
                    .map_err(wallet_error_to_failure)?;
                registry
                    .save_wallet(store, &wallet_name, &wallet, PersistMode::Sync)
                    .map_err(wallet_registry_error_to_failure)?;
                Ok(descriptor_id)
            }
        }
    }

    pub fn allocate_receive_address(&mut self) -> Result<Address, RpcFailure> {
        match &mut self.wallet_state {
            WalletState::Local(wallet) => wallet
                .allocate_receive_address()
                .map_err(wallet_error_to_failure),
            WalletState::DurableNamedRegistry {
                store,
                maybe_request_wallet_name,
            } => {
                let mut registry = load_wallet_registry(store)?;
                let wallet_name =
                    resolve_selected_wallet_name(maybe_request_wallet_name.as_deref(), &registry)?;
                let mut wallet = registry
                    .wallet(&wallet_name)
                    .map_err(wallet_registry_error_to_failure)?;
                let address = wallet
                    .allocate_receive_address()
                    .map_err(wallet_error_to_failure)?;
                registry
                    .save_wallet(store, &wallet_name, &wallet, PersistMode::Sync)
                    .map_err(wallet_registry_error_to_failure)?;
                Ok(address)
            }
        }
    }

    pub fn allocate_change_address(&mut self) -> Result<Address, RpcFailure> {
        match &mut self.wallet_state {
            WalletState::Local(wallet) => wallet
                .allocate_change_address()
                .map_err(wallet_error_to_failure),
            WalletState::DurableNamedRegistry {
                store,
                maybe_request_wallet_name,
            } => {
                let mut registry = load_wallet_registry(store)?;
                let wallet_name =
                    resolve_selected_wallet_name(maybe_request_wallet_name.as_deref(), &registry)?;
                let mut wallet = registry
                    .wallet(&wallet_name)
                    .map_err(wallet_registry_error_to_failure)?;
                let address = wallet
                    .allocate_change_address()
                    .map_err(wallet_error_to_failure)?;
                registry
                    .save_wallet(store, &wallet_name, &wallet, PersistMode::Sync)
                    .map_err(wallet_registry_error_to_failure)?;
                Ok(address)
            }
        }
    }

    pub fn rescan_wallet(&mut self, snapshot: &ChainstateSnapshot) -> Result<(), RpcFailure> {
        match &mut self.wallet_state {
            WalletState::Local(wallet) => wallet
                .rescan_chainstate(snapshot)
                .map_err(wallet_error_to_failure),
            WalletState::DurableNamedRegistry {
                store,
                maybe_request_wallet_name,
            } => {
                let mut registry = load_wallet_registry(store)?;
                let wallet_name =
                    resolve_selected_wallet_name(maybe_request_wallet_name.as_deref(), &registry)?;
                let mut wallet = registry
                    .wallet(&wallet_name)
                    .map_err(wallet_registry_error_to_failure)?;
                wallet
                    .rescan_chainstate(snapshot)
                    .map_err(wallet_error_to_failure)?;
                registry
                    .save_wallet(store, &wallet_name, &wallet, PersistMode::Sync)
                    .map_err(wallet_registry_error_to_failure)
            }
        }
    }

    pub fn build_transaction(
        &self,
        request: &BuildRequest,
        coinbase_maturity: u32,
    ) -> Result<BuiltTransaction, RpcFailure> {
        match &self.wallet_state {
            WalletState::Local(wallet) => wallet
                .build_transaction(request, coinbase_maturity)
                .map_err(wallet_error_to_failure),
            WalletState::DurableNamedRegistry { store, .. } => self
                .selected_wallet_from_store(store)?
                .build_transaction(request, coinbase_maturity)
                .map_err(wallet_error_to_failure),
        }
    }

    pub fn build_and_sign_transaction(
        &self,
        request: &BuildRequest,
        coinbase_maturity: u32,
    ) -> Result<BuiltTransaction, RpcFailure> {
        match &self.wallet_state {
            WalletState::Local(wallet) => wallet
                .build_and_sign(request, coinbase_maturity)
                .map_err(wallet_error_to_failure),
            WalletState::DurableNamedRegistry { store, .. } => self
                .selected_wallet_from_store(store)?
                .build_and_sign(request, coinbase_maturity)
                .map_err(wallet_error_to_failure),
        }
    }

    pub fn sign_transaction(&self, built: &BuiltTransaction) -> Result<Transaction, RpcFailure> {
        match &self.wallet_state {
            WalletState::Local(wallet) => wallet
                .sign_transaction(built)
                .map_err(wallet_error_to_failure),
            WalletState::DurableNamedRegistry { store, .. } => self
                .selected_wallet_from_store(store)?
                .sign_transaction(built)
                .map_err(wallet_error_to_failure),
        }
    }

    pub fn wallet_snapshot(
        &self,
    ) -> Result<open_bitcoin_node::core::wallet::WalletSnapshot, RpcFailure> {
        match &self.wallet_state {
            WalletState::Local(wallet) => Ok(wallet.snapshot()),
            WalletState::DurableNamedRegistry { store, .. } => {
                Ok(self.selected_wallet_from_store(store)?.snapshot())
            }
        }
    }

    pub fn wallet_rescan_job(&self) -> Result<Option<WalletRescanJob>, RpcFailure> {
        let WalletState::DurableNamedRegistry { store, .. } = &self.wallet_state else {
            return Ok(None);
        };
        let registry = load_wallet_registry(store)?;
        let wallet_name = resolve_selected_wallet_name(self.request_wallet_name(), &registry)?;
        Ok(registry.rescan_job(&wallet_name).cloned())
    }

    pub fn wallet_freshness(&self) -> Result<WalletFreshnessView, RpcFailure> {
        if let Some(job) = self.wallet_rescan_job()?
            && matches!(
                job.state,
                WalletRescanJobState::Pending | WalletRescanJobState::Scanning
            )
        {
            return Ok(WalletFreshnessView {
                scanning: true,
                freshness: match job.freshness {
                    WalletRescanFreshness::Fresh => WalletFreshnessKind::Fresh,
                    WalletRescanFreshness::Partial => WalletFreshnessKind::Partial,
                    WalletRescanFreshness::Scanning => WalletFreshnessKind::Scanning,
                },
                maybe_scanned_through_height: job.maybe_scanned_through_height,
                maybe_target_height: Some(job.target_tip_height),
                maybe_next_height: Some(job.next_height),
            });
        }

        let maybe_wallet_tip_height = self.wallet_snapshot()?.maybe_tip_height;
        let maybe_chain_tip_height = self.maybe_chain_tip().map(|tip| tip.height);
        let freshness = match (maybe_wallet_tip_height, maybe_chain_tip_height) {
            (_, None) => WalletFreshnessKind::Fresh,
            (Some(wallet_tip_height), Some(chain_tip_height))
                if wallet_tip_height >= chain_tip_height =>
            {
                WalletFreshnessKind::Fresh
            }
            _ => WalletFreshnessKind::Partial,
        };

        Ok(WalletFreshnessView {
            scanning: false,
            freshness,
            maybe_scanned_through_height: maybe_wallet_tip_height,
            maybe_target_height: maybe_chain_tip_height,
            maybe_next_height: maybe_wallet_tip_height.map(|height| height.saturating_add(1)),
        })
    }

    pub fn rescan_wallet_range(
        &mut self,
        maybe_start_height: Option<u32>,
        maybe_stop_height: Option<u32>,
    ) -> Result<WalletRescanExecution, RpcFailure> {
        let snapshot = self.blockchain_snapshot();
        let tip_height = snapshot.tip().map_or(0, |tip| tip.height);
        let current_wallet_tip = self.wallet_snapshot()?.maybe_tip_height;
        let start_height = maybe_start_height
            .unwrap_or_else(|| current_wallet_tip.map_or(0, |height| height.saturating_add(1)));
        let stop_height = maybe_stop_height.unwrap_or(tip_height);
        if start_height > stop_height {
            return Err(RpcFailure::invalid_params(
                "rescanblockchain start_height must be less than or equal to stop_height",
            ));
        }
        if stop_height > tip_height {
            return Err(RpcFailure::invalid_params(
                "rescanblockchain stop_height exceeds the active chain tip",
            ));
        }

        let partial_snapshot = partial_chainstate_snapshot(&snapshot, stop_height);
        let maybe_tip_median_time_past = partial_snapshot.tip().map(|tip| tip.median_time_past);

        match &mut self.wallet_state {
            WalletState::Local(wallet) => {
                wallet
                    .rescan_chainstate(&partial_snapshot)
                    .map_err(wallet_error_to_failure)?;
            }
            WalletState::DurableNamedRegistry {
                store,
                maybe_request_wallet_name,
            } => {
                let mut registry = load_wallet_registry(store)?;
                let wallet_name =
                    resolve_selected_wallet_name(maybe_request_wallet_name.as_deref(), &registry)?;
                let maybe_scanned_through_height = start_height.checked_sub(1);
                let target_tip_hash = partial_snapshot.tip().map_or_else(
                    || open_bitcoin_node::core::primitives::BlockHash::from_byte_array([0_u8; 32]),
                    |tip| tip.block_hash,
                );
                let mut job = WalletRescanJob::new(
                    wallet_name.clone(),
                    target_tip_hash,
                    stop_height,
                    start_height,
                    maybe_scanned_through_height,
                )
                .map_err(wallet_registry_error_to_failure)?;
                job.state = WalletRescanJobState::Pending;
                registry
                    .save_rescan_job(store, job.clone(), PersistMode::Sync)
                    .map_err(wallet_registry_error_to_failure)?;

                let mut wallet = registry
                    .wallet(&wallet_name)
                    .map_err(wallet_registry_error_to_failure)?;
                wallet
                    .rescan_chainstate(&partial_snapshot)
                    .map_err(wallet_error_to_failure)?;
                registry
                    .save_wallet(store, &wallet_name, &wallet, PersistMode::Sync)
                    .map_err(wallet_registry_error_to_failure)?;
                job.mark_chunk_progress(stop_height, maybe_tip_median_time_past);
                registry
                    .save_rescan_job(store, job, PersistMode::Sync)
                    .map_err(wallet_registry_error_to_failure)?;
            }
        }

        let freshness = WalletFreshnessView {
            scanning: false,
            freshness: if stop_height < tip_height {
                WalletFreshnessKind::Partial
            } else {
                WalletFreshnessKind::Fresh
            },
            maybe_scanned_through_height: Some(stop_height),
            maybe_target_height: Some(tip_height),
            maybe_next_height: Some(stop_height.saturating_add(1)),
        };

        Ok(WalletRescanExecution {
            start_height,
            stop_height,
            freshness,
        })
    }

    fn request_wallet_name(&self) -> Option<&str> {
        match &self.wallet_state {
            WalletState::Local(_) => None,
            WalletState::DurableNamedRegistry {
                maybe_request_wallet_name,
                ..
            } => maybe_request_wallet_name.as_deref(),
        }
    }

    fn selected_wallet_from_store(&self, store: &FjallNodeStore) -> Result<Wallet, RpcFailure> {
        let registry = load_wallet_registry(store)?;
        let wallet_name = resolve_selected_wallet_name(self.request_wallet_name(), &registry)?;
        registry
            .wallet(&wallet_name)
            .map_err(wallet_registry_error_to_failure)
    }
}

#[cfg(test)]
mod tests;

fn build_wallet_state(config: &RuntimeConfig) -> WalletState {
    let Some(data_dir) = config.maybe_data_dir.as_ref() else {
        return WalletState::Local(ManagedWallet::from_store(
            MemoryWalletStore::default(),
            Wallet::new(config.chain),
        ));
    };

    let should_use_registry = matches!(
        config.wallet.scope,
        WalletRuntimeScope::DurableNamedRegistry | WalletRuntimeScope::LocalOperatorSingleWallet
    );
    if should_use_registry && let Ok(store) = FjallNodeStore::open(data_dir) {
        return WalletState::DurableNamedRegistry {
            store,
            maybe_request_wallet_name: None,
        };
    }

    WalletState::Local(ManagedWallet::from_store(
        MemoryWalletStore::default(),
        Wallet::new(config.chain),
    ))
}

fn load_wallet_registry(store: &FjallNodeStore) -> Result<WalletRegistry, RpcFailure> {
    WalletRegistry::load(store).map_err(wallet_registry_error_to_failure)
}

fn resolve_selected_wallet_name(
    maybe_request_wallet_name: Option<&str>,
    registry: &WalletRegistry,
) -> Result<String, RpcFailure> {
    if let Some(wallet_name) = maybe_request_wallet_name {
        if registry
            .wallet_names()
            .iter()
            .any(|candidate| candidate == wallet_name)
        {
            return Ok(wallet_name.to_string());
        }
        return Err(RpcFailure::new(
            RpcFailureKind::InvalidParams,
            Some(RpcErrorDetail::new(
                RpcErrorCode::WalletNotFound,
                "Requested wallet does not exist or is not loaded",
            )),
        ));
    }

    match registry.wallet_names() {
        [] => Err(RpcFailure::new(
            RpcFailureKind::InvalidParams,
            Some(RpcErrorDetail::new(
                RpcErrorCode::WalletNotFound,
                "No wallet is loaded. Load a wallet using loadwallet or create a new one with createwallet. (Note: A default wallet is no longer automatically created)",
            )),
        )),
        [wallet_name] => Ok(wallet_name.clone()),
        _ => Err(RpcFailure::new(
            RpcFailureKind::InvalidParams,
            Some(RpcErrorDetail::new(
                RpcErrorCode::WalletNotSpecified,
                "Multiple wallets are loaded. Please select which wallet to use by requesting the RPC through the /wallet/<walletname> URI path.",
            )),
        )),
    }
}

fn wallet_error_to_failure(error: WalletError) -> RpcFailure {
    RpcFailure::wallet_error(error.to_string())
}

fn wallet_registry_error_to_failure(error: WalletRegistryError) -> RpcFailure {
    match error {
        WalletRegistryError::UnknownWallet(_) => RpcFailure::new(
            RpcFailureKind::InvalidParams,
            Some(RpcErrorDetail::new(
                RpcErrorCode::WalletNotFound,
                "Requested wallet does not exist or is not loaded",
            )),
        ),
        WalletRegistryError::Storage(StorageError::UnavailableNamespace { .. }) => {
            RpcFailure::wallet_error(error.to_string())
        }
        _ => RpcFailure::wallet_error(error.to_string()),
    }
}

fn partial_chainstate_snapshot(
    snapshot: &ChainstateSnapshot,
    through_height: u32,
) -> ChainstateSnapshot {
    let active_chain = snapshot
        .active_chain
        .iter()
        .filter(|position| position.height <= through_height)
        .cloned()
        .collect::<Vec<_>>();
    let active_hashes = active_chain
        .iter()
        .map(|position| position.block_hash)
        .collect::<std::collections::BTreeSet<_>>();
    let utxos = snapshot
        .utxos
        .iter()
        .filter(|(_, coin)| coin.created_height <= through_height)
        .map(|(outpoint, coin)| (outpoint.clone(), coin.clone()))
        .collect();
    let undo_by_block = snapshot
        .undo_by_block
        .iter()
        .filter(|(block_hash, _)| active_hashes.contains(block_hash))
        .map(|(block_hash, undo)| (*block_hash, undo.clone()))
        .collect();

    ChainstateSnapshot::new(active_chain, utxos, undo_by_block)
}

fn default_verify_flags() -> ScriptVerifyFlags {
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

fn network_magic(chain: AddressNetwork) -> NetworkMagic {
    match chain {
        AddressNetwork::Mainnet => NetworkMagic::MAINNET,
        AddressNetwork::Testnet => NetworkMagic::from_bytes([0x0b, 0x11, 0x09, 0x07]),
        AddressNetwork::Signet => NetworkMagic::from_bytes([0x0a, 0x03, 0xcf, 0x40]),
        AddressNetwork::Regtest => NetworkMagic::from_bytes([0xfa, 0xbf, 0xb5, 0xda]),
    }
}

fn default_p2p_port(chain: AddressNetwork) -> u16 {
    match chain {
        AddressNetwork::Mainnet => 8333,
        AddressNetwork::Testnet => 18_333,
        AddressNetwork::Signet => 38_333,
        AddressNetwork::Regtest => 18_444,
    }
}
