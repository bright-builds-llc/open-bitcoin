// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/wallet.cpp
// - packages/bitcoin-knots/src/wallet/spend.cpp
// - packages/bitcoin-knots/src/wallet/receive.cpp

use open_bitcoin_core::{
    primitives::Transaction,
    wallet::{
        AddressNetwork, BuildRequest, BuiltTransaction, Wallet, WalletBalance, WalletSnapshot,
        WalletUtxo,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedWalletInfo {
    pub network: AddressNetwork,
    pub descriptor_count: usize,
    pub utxo_count: usize,
    pub maybe_tip_height: Option<u32>,
    pub maybe_tip_median_time_past: Option<i64>,
}

pub trait WalletStore {
    fn load_snapshot(&self) -> Option<WalletSnapshot>;
    fn save_snapshot(&mut self, snapshot: WalletSnapshot);
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MemoryWalletStore {
    maybe_snapshot: Option<WalletSnapshot>,
}

impl MemoryWalletStore {
    pub fn snapshot(&self) -> Option<&WalletSnapshot> {
        self.maybe_snapshot.as_ref()
    }
}

impl WalletStore for MemoryWalletStore {
    fn load_snapshot(&self) -> Option<WalletSnapshot> {
        self.maybe_snapshot.clone()
    }

    fn save_snapshot(&mut self, snapshot: WalletSnapshot) {
        self.maybe_snapshot = Some(snapshot);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedWallet<S> {
    store: S,
    wallet: Wallet,
}

impl<S: WalletStore> ManagedWallet<S> {
    pub fn from_store(store: S, fallback_wallet: Wallet) -> Self {
        let wallet = store
            .load_snapshot()
            .map(Wallet::from_snapshot)
            .unwrap_or(fallback_wallet);

        Self { store, wallet }
    }

    pub fn wallet(&self) -> &Wallet {
        &self.wallet
    }

    pub fn store(&self) -> &S {
        &self.store
    }

    pub fn wallet_info(&self) -> ManagedWalletInfo {
        let snapshot = self.wallet.snapshot();

        ManagedWalletInfo {
            network: snapshot.network,
            descriptor_count: snapshot.descriptors.len(),
            utxo_count: snapshot.utxos.len(),
            maybe_tip_height: snapshot.maybe_tip_height,
            maybe_tip_median_time_past: snapshot.maybe_tip_median_time_past,
        }
    }

    pub fn balance(
        &self,
        coinbase_maturity: u32,
    ) -> Result<WalletBalance, open_bitcoin_core::wallet::WalletError> {
        self.wallet.balance(coinbase_maturity)
    }

    pub fn utxos(&self) -> &[WalletUtxo] {
        self.wallet.utxos()
    }

    pub fn import_descriptor(
        &mut self,
        label: impl Into<String>,
        role: open_bitcoin_core::wallet::DescriptorRole,
        descriptor_text: &str,
    ) -> Result<u32, open_bitcoin_core::wallet::WalletError> {
        let descriptor_id = self
            .wallet
            .import_descriptor(label, role, descriptor_text)?;
        self.persist();
        Ok(descriptor_id)
    }

    pub fn rescan_chainstate(
        &mut self,
        snapshot: &open_bitcoin_core::chainstate::ChainstateSnapshot,
    ) -> Result<(), open_bitcoin_core::wallet::WalletError> {
        self.wallet.rescan_chainstate(snapshot)?;
        self.persist();
        Ok(())
    }

    pub fn build_and_sign(
        &self,
        request: &BuildRequest,
        coinbase_maturity: u32,
    ) -> Result<BuiltTransaction, open_bitcoin_core::wallet::WalletError> {
        self.wallet.build_and_sign(request, coinbase_maturity)
    }

    pub fn build_transaction(
        &self,
        request: &BuildRequest,
        coinbase_maturity: u32,
    ) -> Result<BuiltTransaction, open_bitcoin_core::wallet::WalletError> {
        self.wallet.build_transaction(request, coinbase_maturity)
    }

    pub fn sign_transaction(
        &self,
        built: &BuiltTransaction,
    ) -> Result<Transaction, open_bitcoin_core::wallet::WalletError> {
        self.wallet.sign_transaction(built)
    }

    pub fn into_parts(self) -> (S, Wallet) {
        (self.store, self.wallet)
    }

    fn persist(&mut self) {
        self.store.save_snapshot(self.wallet.snapshot());
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use open_bitcoin_core::{
        chainstate::{ChainPosition, ChainstateSnapshot, Coin},
        primitives::{BlockHash, BlockHeader, OutPoint, ScriptBuf, TransactionOutput, Txid},
        wallet::{AddressNetwork, BuildRequest, DescriptorRole, Recipient, Wallet},
    };
    use open_bitcoin_mempool::FeeRate;

    use crate::wallet::{ManagedWallet, MemoryWalletStore, WalletStore};

    fn sample_tip(height: u32) -> ChainPosition {
        ChainPosition::new(
            BlockHeader {
                version: 1,
                previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
                merkle_root: Default::default(),
                time: 1_700_000_000 + height,
                bits: 0x207f_ffff,
                nonce: 1,
            },
            height,
            1,
            i64::from(1_700_000_000 + height),
        )
    }

    fn script(bytes: &[u8]) -> ScriptBuf {
        ScriptBuf::from_bytes(bytes.to_vec()).expect("script")
    }

    fn wallet_with_descriptors() -> Wallet {
        let mut wallet = Wallet::new(AddressNetwork::Regtest);
        wallet
            .import_descriptor(
                "receive",
                DescriptorRole::External,
                "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
            )
            .expect("receive descriptor");
        wallet
            .import_descriptor(
                "change",
                DescriptorRole::Internal,
                "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))",
            )
            .expect("change descriptor");
        wallet
    }

    fn funded_snapshot(wallet: &Wallet) -> ChainstateSnapshot {
        let receive_script = wallet
            .default_receive_address()
            .expect("receive address")
            .script_pubkey;
        let mut utxos = HashMap::new();
        utxos.insert(
            OutPoint {
                txid: Txid::from_byte_array([7_u8; 32]),
                vout: 0,
            },
            Coin {
                output: TransactionOutput {
                    value: open_bitcoin_core::primitives::Amount::from_sats(75_000)
                        .expect("amount"),
                    script_pubkey: receive_script,
                },
                is_coinbase: false,
                created_height: 9,
                created_median_time_past: 1_700_000_009,
            },
        );

        ChainstateSnapshot::new(vec![sample_tip(10)], utxos, Default::default())
    }

    #[test]
    fn managed_wallet_persists_descriptor_imports() {
        let wallet = Wallet::new(AddressNetwork::Regtest);
        let mut managed = ManagedWallet::from_store(MemoryWalletStore::default(), wallet);

        managed
            .import_descriptor(
                "receive",
                DescriptorRole::External,
                "wpkh(cTe1f5rdT8A8DFgVWTjyPwACsDPJM9ff4QngFxUixCSvvbg1x6sh)",
            )
            .expect("descriptor import");

        assert_eq!(managed.wallet().descriptors().len(), 1);
        assert_eq!(
            managed
                .store()
                .load_snapshot()
                .expect("snapshot")
                .descriptors
                .len(),
            1
        );
    }

    #[test]
    fn managed_wallet_exposes_rpc_projection_and_build_helpers() {
        // Arrange
        let mut managed = ManagedWallet::from_store(
            MemoryWalletStore::default(),
            Wallet::new(AddressNetwork::Regtest),
        );
        managed
            .import_descriptor(
                "receive",
                DescriptorRole::External,
                "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
            )
            .expect("receive descriptor");
        managed
            .import_descriptor(
                "change",
                DescriptorRole::Internal,
                "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))",
            )
            .expect("change descriptor");
        let snapshot = funded_snapshot(&wallet_with_descriptors());
        managed.rescan_chainstate(&snapshot).expect("rescan");
        let request = BuildRequest {
            recipients: vec![Recipient {
                script_pubkey: script(&[0x51]),
                value: open_bitcoin_core::primitives::Amount::from_sats(30_000).expect("amount"),
            }],
            fee_rate: FeeRate::from_sats_per_kvb(2_000),
            maybe_change_descriptor_id: None,
            maybe_lock_time: None,
            enable_rbf: true,
        };

        // Act
        let wallet_info = managed.wallet_info();
        let balance = managed.balance(100).expect("balance");
        let utxos = managed.utxos();
        let built = managed
            .build_transaction(&request, 100)
            .expect("build transaction");
        let signed = managed.sign_transaction(&built).expect("sign transaction");
        let built_and_signed = managed
            .build_and_sign(&request, 100)
            .expect("build and sign");

        // Assert
        assert_eq!(wallet_info.network, AddressNetwork::Regtest);
        assert_eq!(wallet_info.descriptor_count, 2);
        assert_eq!(wallet_info.utxo_count, 1);
        assert_eq!(wallet_info.maybe_tip_height, Some(10));
        assert_eq!(balance.total.to_sats(), 75_000);
        assert_eq!(utxos.len(), 1);
        assert!(!signed.inputs[0].witness.is_empty());
        assert_eq!(built_and_signed.transaction, signed);
        assert_eq!(built_and_signed.selected_inputs.len(), 1);
        assert_eq!(
            managed
                .store()
                .load_snapshot()
                .expect("snapshot")
                .descriptors
                .len(),
            2
        );
    }
}
