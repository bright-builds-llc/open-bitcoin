use open_bitcoin_node::core::{
    chainstate::{ChainPosition, ChainstateSnapshot},
    consensus::{ConsensusParams, ScriptVerifyFlags},
    mempool::{AdmissionResult, PolicyConfig},
    network::LocalPeerConfig,
    primitives::Transaction,
    wallet::{
        AddressNetwork, BuildRequest, BuiltTransaction, Wallet, WalletBalance, WalletError,
        WalletUtxo,
    },
};
use open_bitcoin_node::network::{ManagedMempoolInfo, ManagedNetworkInfo};
use open_bitcoin_node::wallet::ManagedWalletInfo;
use open_bitcoin_node::{
    ManagedNetworkError, ManagedPeerNetwork, ManagedWallet, MemoryChainstateStore,
    MemoryWalletStore,
};

#[derive(Debug, Clone)]
pub struct ManagedRpcContext {
    network: ManagedPeerNetwork<MemoryChainstateStore>,
    wallet: ManagedWallet<MemoryWalletStore>,
}

impl ManagedRpcContext {
    pub fn new(
        network: ManagedPeerNetwork<MemoryChainstateStore>,
        wallet: ManagedWallet<MemoryWalletStore>,
    ) -> Self {
        Self { network, wallet }
    }

    pub fn for_local_operator(network: AddressNetwork) -> Self {
        let managed_network = ManagedPeerNetwork::new(
            MemoryChainstateStore::default(),
            LocalPeerConfig::default(),
            PolicyConfig::default(),
        );
        let managed_wallet =
            ManagedWallet::from_store(MemoryWalletStore::default(), Wallet::new(network));

        Self::new(managed_network, managed_wallet)
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

    pub fn submit_local_transaction(
        &mut self,
        transaction: Transaction,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<AdmissionResult, ManagedNetworkError> {
        self.network
            .submit_local_transaction(transaction, verify_flags, consensus_params)
    }

    pub fn wallet_info(&self) -> ManagedWalletInfo {
        self.wallet.wallet_info()
    }

    pub fn wallet_balance(&self, coinbase_maturity: u32) -> Result<WalletBalance, WalletError> {
        self.wallet.balance(coinbase_maturity)
    }

    pub fn wallet_utxos(&self) -> &[WalletUtxo] {
        self.wallet.utxos()
    }

    pub fn import_descriptor(
        &mut self,
        label: impl Into<String>,
        role: open_bitcoin_node::core::wallet::DescriptorRole,
        descriptor_text: &str,
    ) -> Result<u32, WalletError> {
        self.wallet.import_descriptor(label, role, descriptor_text)
    }

    pub fn rescan_wallet(&mut self, snapshot: &ChainstateSnapshot) -> Result<(), WalletError> {
        self.wallet.rescan_chainstate(snapshot)
    }

    pub fn build_transaction(
        &self,
        request: &BuildRequest,
        coinbase_maturity: u32,
    ) -> Result<BuiltTransaction, WalletError> {
        self.wallet.build_transaction(request, coinbase_maturity)
    }

    pub fn build_and_sign_transaction(
        &self,
        request: &BuildRequest,
        coinbase_maturity: u32,
    ) -> Result<BuiltTransaction, WalletError> {
        self.wallet.build_and_sign(request, coinbase_maturity)
    }

    pub fn sign_transaction(&self, built: &BuiltTransaction) -> Result<Transaction, WalletError> {
        self.wallet.sign_transaction(built)
    }
}

#[cfg(test)]
mod tests;
