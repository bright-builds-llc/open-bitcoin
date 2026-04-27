use open_bitcoin_node::core::primitives::Transaction;
use open_bitcoin_node::core::wallet::{
    Address, BuildRequest, BuiltTransaction, DescriptorRole, Wallet, WalletBalance, WalletError,
    WalletUtxo,
};
use open_bitcoin_node::wallet::ManagedWalletInfo;
use open_bitcoin_node::{
    FjallNodeStore, ManagedWallet, MemoryWalletStore, PersistMode, StorageError, WalletRegistry,
    WalletRegistryError,
};

use crate::config::{RuntimeConfig, WalletRuntimeScope};
use crate::error::{RpcErrorCode, RpcErrorDetail, RpcFailure, RpcFailureKind};

use super::ManagedRpcContext;

pub(super) enum WalletState {
    Local(ManagedWallet<MemoryWalletStore>),
    DurableNamedRegistry {
        store: FjallNodeStore,
        maybe_request_wallet_name: Option<String>,
    },
}

impl ManagedRpcContext {
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

    pub fn rescan_wallet(
        &mut self,
        snapshot: &open_bitcoin_node::core::chainstate::ChainstateSnapshot,
    ) -> Result<(), RpcFailure> {
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

    pub(super) fn request_wallet_name(&self) -> Option<&str> {
        match &self.wallet_state {
            WalletState::Local(_) => None,
            WalletState::DurableNamedRegistry {
                maybe_request_wallet_name,
                ..
            } => maybe_request_wallet_name.as_deref(),
        }
    }

    pub(super) fn selected_wallet_from_store(
        &self,
        store: &FjallNodeStore,
    ) -> Result<Wallet, RpcFailure> {
        let registry = load_wallet_registry(store)?;
        let wallet_name = resolve_selected_wallet_name(self.request_wallet_name(), &registry)?;
        registry
            .wallet(&wallet_name)
            .map_err(wallet_registry_error_to_failure)
    }
}

pub(super) fn build_wallet_state(config: &RuntimeConfig) -> WalletState {
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

pub(super) fn load_wallet_registry(store: &FjallNodeStore) -> Result<WalletRegistry, RpcFailure> {
    WalletRegistry::load(store).map_err(wallet_registry_error_to_failure)
}

pub(super) fn resolve_selected_wallet_name(
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

pub(super) fn wallet_error_to_failure(error: WalletError) -> RpcFailure {
    RpcFailure::wallet_error(error.to_string())
}

pub(super) fn wallet_registry_error_to_failure(error: WalletRegistryError) -> RpcFailure {
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
