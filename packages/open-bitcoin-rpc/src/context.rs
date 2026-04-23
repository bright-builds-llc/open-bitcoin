use open_bitcoin_node::core::{
    chainstate::{ChainPosition, ChainstateSnapshot},
    consensus::{ConsensusParams, ScriptVerifyFlags},
    mempool::{AdmissionResult, PolicyConfig},
    network::{LocalPeerConfig, ServiceFlags},
    primitives::{NetworkAddress, NetworkMagic, Transaction},
    wallet::{
        AddressNetwork, BuildRequest, BuiltTransaction, DescriptorRole, Wallet, WalletBalance,
        WalletError, WalletUtxo,
    },
};
use open_bitcoin_node::network::{ManagedMempoolInfo, ManagedNetworkInfo};
use open_bitcoin_node::wallet::ManagedWalletInfo;
use open_bitcoin_node::{
    ManagedNetworkError, ManagedPeerNetwork, ManagedWallet, MemoryChainstateStore,
    MemoryWalletStore,
};

use crate::config::RuntimeConfig;

#[derive(Debug, Clone)]
pub struct ManagedRpcContext {
    chain: AddressNetwork,
    consensus_params: ConsensusParams,
    verify_flags: ScriptVerifyFlags,
    network: ManagedPeerNetwork<MemoryChainstateStore>,
    wallet: ManagedWallet<MemoryWalletStore>,
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
            wallet,
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
        let managed_wallet =
            ManagedWallet::from_store(MemoryWalletStore::default(), Wallet::new(config.chain));

        Self::new(
            config.chain,
            consensus_params,
            default_verify_flags(),
            managed_network,
            managed_wallet,
        )
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
        role: DescriptorRole,
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
