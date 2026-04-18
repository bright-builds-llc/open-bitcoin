use open_bitcoin_core::wallet::{BuildRequest, BuiltTransaction, Wallet, WalletSnapshot};

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

    pub fn into_parts(self) -> (S, Wallet) {
        (self.store, self.wallet)
    }

    fn persist(&mut self) {
        self.store.save_snapshot(self.wallet.snapshot());
    }
}

#[cfg(test)]
mod tests {
    use open_bitcoin_core::wallet::{AddressNetwork, DescriptorRole, Wallet};

    use crate::wallet::{ManagedWallet, MemoryWalletStore, WalletStore};

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
}
