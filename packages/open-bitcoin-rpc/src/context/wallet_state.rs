// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/wallet.cpp
// - packages/bitcoin-knots/src/wallet/rpc/transactions.cpp
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

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

use super::canonical_chain::selected_chain;
use super::{CanonicalContextError, ManagedRpcContext};

pub(super) enum WalletState {
    Unavailable,
    Local(ManagedWallet<MemoryWalletStore>),
    DurableNamedRegistry {
        store: FjallNodeStore,
        maybe_request_wallet_name: Option<String>,
    },
}

impl<S: open_bitcoin_node::ChainstateStore, V: open_bitcoin_node::core::chainstate::CoinsView>
    ManagedRpcContext<S, V>
{
    pub fn wallet_info(&self) -> Result<ManagedWalletInfo, RpcFailure> {
        self.selected_wallet_info()
    }

    pub fn selected_wallet_info(&self) -> Result<ManagedWalletInfo, RpcFailure> {
        match &self.wallet_state {
            WalletState::Unavailable => Err(unavailable_wallet()),
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
            WalletState::Unavailable => Err(unavailable_wallet()),
            WalletState::Local(_) => Ok(None),
            WalletState::DurableNamedRegistry { store, .. } => {
                let registry = load_wallet_registry(store)?;
                resolve_selected_wallet_name(self.request_wallet_name(), &registry).map(Some)
            }
        }
    }

    pub fn require_wallet_selection(&self) -> Result<(), RpcFailure> {
        match &self.wallet_state {
            WalletState::Unavailable => Err(unavailable_wallet()),
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
            WalletState::Unavailable => Err(unavailable_wallet()),
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
            WalletState::Unavailable => Err(unavailable_wallet()),
            WalletState::Local(wallet) => Ok(wallet.utxos().to_vec()),
            WalletState::DurableNamedRegistry { store, .. } => {
                Ok(self.selected_wallet_from_store(store)?.utxos().to_vec())
            }
        }
    }

    pub fn descriptor_address(&self, descriptor_id: u32) -> Result<Address, RpcFailure> {
        match &self.wallet_state {
            WalletState::Unavailable => Err(unavailable_wallet()),
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
            WalletState::Unavailable => Err(unavailable_wallet()),
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
            WalletState::Unavailable => Err(unavailable_wallet()),
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
            WalletState::Unavailable => Err(unavailable_wallet()),
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
            WalletState::Unavailable => Err(unavailable_wallet()),
            WalletState::Local(wallet) => wallet
                .rescan_chainstate(snapshot)
                .map_err(wallet_error_to_failure),
            WalletState::DurableNamedRegistry { .. } => {
                self.rescan_wallet_range(Some(0), None).map(|_| ())
            }
        }
    }

    pub fn build_transaction(
        &self,
        request: &BuildRequest,
        coinbase_maturity: u32,
    ) -> Result<BuiltTransaction, RpcFailure> {
        match &self.wallet_state {
            WalletState::Unavailable => Err(unavailable_wallet()),
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
            WalletState::Unavailable => Err(unavailable_wallet()),
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
            WalletState::Unavailable => Err(unavailable_wallet()),
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
            WalletState::Unavailable => Err(unavailable_wallet()),
            WalletState::Local(wallet) => Ok(wallet.snapshot()),
            WalletState::DurableNamedRegistry { store, .. } => {
                Ok(self.selected_wallet_from_store(store)?.snapshot())
            }
        }
    }

    pub(super) fn request_wallet_name(&self) -> Option<&str> {
        match &self.wallet_state {
            WalletState::Unavailable => None,
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
        if matches!(self.wallet_state, WalletState::Unavailable) {
            return Err(unavailable_wallet());
        }
        let registry = load_wallet_registry(store)?;
        let wallet_name = resolve_selected_wallet_name(self.request_wallet_name(), &registry)?;
        registry
            .wallet(&wallet_name)
            .map_err(wallet_registry_error_to_failure)
    }
}

pub(super) fn unavailable_wallet() -> RpcFailure {
    RpcFailure::wallet_error("wallet unavailable: canonical genesis history is not qualified")
}

pub(super) fn build_wallet_state_with_store(
    config: &RuntimeConfig,
    maybe_store: Option<FjallNodeStore>,
) -> Result<WalletState, CanonicalContextError> {
    let chain = selected_chain(config)?;
    let should_use_registry = matches!(
        config.wallet.scope,
        WalletRuntimeScope::DurableNamedRegistry | WalletRuntimeScope::LocalOperatorSingleWallet
    );
    if should_use_registry {
        if let Some(store) = maybe_store {
            super::canonical_chain::require_ordinary_scope(chain)?;
            return Ok(registry_wallet_state(store));
        }
        if let Some(data_dir) = config.maybe_data_dir.as_ref() {
            return Ok(registry_wallet_state(FjallNodeStore::open_ordinary(
                data_dir, chain,
            )?));
        }
    }
    super::canonical_chain::require_ordinary_scope(chain)?;
    Ok(local_wallet_state(config))
}

pub(super) fn build_fixture_wallet_state_with_store(
    config: &RuntimeConfig,
    maybe_store: Option<FjallNodeStore>,
) -> Result<WalletState, StorageError> {
    let Some(data_dir) = config.maybe_data_dir.as_ref() else {
        return Ok(local_wallet_state(config));
    };

    let should_use_registry = matches!(
        config.wallet.scope,
        WalletRuntimeScope::DurableNamedRegistry | WalletRuntimeScope::LocalOperatorSingleWallet
    );
    if should_use_registry && let Some(store) = maybe_store {
        return Ok(registry_wallet_state(store));
    }
    if should_use_registry {
        return Ok(registry_wallet_state(FjallNodeStore::open(data_dir)?));
    }

    Ok(local_wallet_state(config))
}

fn registry_wallet_state(store: FjallNodeStore) -> WalletState {
    WalletState::DurableNamedRegistry {
        store,
        maybe_request_wallet_name: None,
    }
}

fn local_wallet_state(config: &RuntimeConfig) -> WalletState {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn unavailable_fixture() -> ManagedRpcContext {
        // Memory-only state-machine evidence; this grants no durable authority.
        let mut context = ManagedRpcContext::from_fixture_runtime_config(&RuntimeConfig::default());
        context.wallet_state = WalletState::Unavailable;
        context
    }

    #[test]
    fn phase166_unavailable_wallet_queries_refuse() {
        // Arrange
        let context = unavailable_fixture();
        let expected = RpcFailure::wallet_error(
            "wallet unavailable: canonical genesis history is not qualified",
        );
        // Act / Assert
        assert_eq!(context.wallet_info().map(|_| ()), Err(expected.clone()));
        assert_eq!(
            context.selected_wallet_info().map(|_| ()),
            Err(expected.clone())
        );
        assert_eq!(
            context.selected_wallet_name().map(|_| ()),
            Err(expected.clone())
        );
        assert_eq!(context.require_wallet_selection(), Err(expected.clone()));
        assert_eq!(
            context.wallet_balance(100).map(|_| ()),
            Err(expected.clone())
        );
        assert_eq!(context.wallet_utxos().map(|_| ()), Err(expected.clone()));
        assert_eq!(context.wallet_snapshot().map(|_| ()), Err(expected.clone()));
        assert_eq!(
            context.wallet_rescan_job().map(|_| ()),
            Err(expected.clone())
        );
        assert_eq!(context.wallet_freshness().map(|_| ()), Err(expected));
    }

    #[test]
    fn phase166_unavailable_wallet_rescan_calls_no_effects() {
        // Arrange
        let mut context = unavailable_fixture();
        let calls = std::cell::Cell::new(0);
        // Act
        let result = context.rescan_wallet_range_with(
            None,
            None,
            |_| {
                calls.set(calls.get() + 1);
                panic!("guard must not run")
            },
            |_, _, _, _, _| {
                calls.set(calls.get() + 1);
                panic!("prepare must not run")
            },
            |_, _, _| {
                calls.set(calls.get() + 1);
                panic!("save_failed must not run")
            },
        );
        // Assert
        assert_eq!(
            result,
            Err(RpcFailure::wallet_error(
                "wallet unavailable: canonical genesis history is not qualified",
            ))
        );
        assert_eq!(calls.get(), 0);
    }

    #[test]
    fn phase166_unavailable_wallet_mutations_refuse() {
        // Arrange
        let mut context = unavailable_fixture();
        let expected = unavailable_wallet();
        let request = BuildRequest {
            recipients: vec![],
            fee_rate: open_bitcoin_node::core::mempool::FeeRate::from_sats_per_kvb(1000),
            maybe_change_descriptor_id: None,
            maybe_lock_time: None,
            enable_rbf: false,
        };
        let built = BuiltTransaction {
            transaction: Transaction::default(),
            selected_inputs: vec![],
            fee: open_bitcoin_node::core::primitives::Amount::from_sats(0).expect("zero fee"),
            change_output_index: None,
        };
        let snapshot = open_bitcoin_node::core::chainstate::ChainstateSnapshot::new(
            vec![],
            Default::default(),
            Default::default(),
        );
        // Act / Assert
        assert_eq!(
            context.descriptor_address(0).map(|_| ()),
            Err(expected.clone())
        );
        assert_eq!(
            context.import_descriptor("private", DescriptorRole::External, "invalid"),
            Err(expected.clone())
        );
        assert_eq!(
            context.allocate_receive_address().map(|_| ()),
            Err(expected.clone())
        );
        assert_eq!(
            context.allocate_change_address().map(|_| ()),
            Err(expected.clone())
        );
        assert_eq!(context.rescan_wallet(&snapshot), Err(expected.clone()));
        assert_eq!(
            context.build_transaction(&request, 100).map(|_| ()),
            Err(expected.clone())
        );
        assert_eq!(
            context
                .build_and_sign_transaction(&request, 100)
                .map(|_| ()),
            Err(expected.clone())
        );
        assert_eq!(context.sign_transaction(&built).map(|_| ()), Err(expected));
        context.set_request_wallet_name(Some("private".into()));
        assert_eq!(context.request_wallet_name(), None);
        context.clear_request_wallet_name();
        assert_eq!(context.request_wallet_name(), None);
        assert!(format!("{context:?}").contains("\"unavailable\""));
    }
}
