// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/wallet.cpp
// - packages/bitcoin-knots/src/wallet/rpc/util.cpp
// - packages/bitcoin-knots/src/wallet/rpc/transactions.cpp

//! Durable named-wallet registry and rescan-job shell state.

use std::collections::BTreeMap;

use open_bitcoin_core::{
    primitives::BlockHash,
    wallet::{Wallet, WalletError, WalletSnapshot},
};
use open_bitcoin_wallet::wallet::WalletRescanState;

use crate::{FjallNodeStore, PersistMode, StorageError, StorageNamespace, StorageRecoveryAction};

const MISSING_SELECTED_WALLET_DETAIL: &str = "selected wallet metadata missing";

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WalletRegistrySnapshot {
    pub wallet_names: Vec<String>,
}

impl WalletRegistrySnapshot {
    pub fn new(wallet_names: impl IntoIterator<Item = String>) -> Self {
        let mut wallet_names = wallet_names.into_iter().collect::<Vec<_>>();
        wallet_names.sort();
        wallet_names.dedup();

        Self { wallet_names }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectedWalletRecord {
    pub wallet_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletRescanFreshness {
    Fresh,
    Partial,
    Scanning,
}

impl WalletRescanFreshness {
    pub fn from_wallet_state(state: WalletRescanState) -> Self {
        match state {
            WalletRescanState::Fresh { .. } => Self::Fresh,
            WalletRescanState::Partial { .. } => Self::Partial,
            WalletRescanState::Scanning { .. } => Self::Scanning,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletRescanJobState {
    Pending,
    Scanning,
    Complete,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletRescanJob {
    pub wallet_name: String,
    pub target_tip_hash: BlockHash,
    pub target_tip_height: u32,
    pub next_height: u32,
    pub maybe_scanned_through_height: Option<u32>,
    pub maybe_tip_median_time_past: Option<i64>,
    pub freshness: WalletRescanFreshness,
    pub state: WalletRescanJobState,
    pub maybe_error: Option<String>,
}

impl WalletRescanJob {
    pub fn new(
        wallet_name: impl Into<String>,
        target_tip_hash: BlockHash,
        target_tip_height: u32,
        next_height: u32,
        maybe_scanned_through_height: Option<u32>,
    ) -> Result<Self, WalletRegistryError> {
        let freshness = WalletRescanFreshness::from_wallet_state(WalletRescanState::from_progress(
            maybe_scanned_through_height,
            Some(target_tip_height),
            Some(next_height),
            true,
        )?);

        Ok(Self {
            wallet_name: wallet_name.into(),
            target_tip_hash,
            target_tip_height,
            next_height,
            maybe_scanned_through_height,
            maybe_tip_median_time_past: None,
            freshness,
            state: WalletRescanJobState::Pending,
            maybe_error: None,
        })
    }

    pub fn mark_chunk_progress(
        &mut self,
        scanned_through_height: u32,
        maybe_tip_median_time_past: Option<i64>,
    ) {
        self.maybe_scanned_through_height = Some(scanned_through_height);
        self.maybe_tip_median_time_past = maybe_tip_median_time_past;
        if scanned_through_height >= self.target_tip_height {
            self.next_height = self.target_tip_height.saturating_add(1);
            self.freshness = WalletRescanFreshness::Fresh;
            self.state = WalletRescanJobState::Complete;
            self.maybe_error = None;
            return;
        }

        self.next_height = scanned_through_height.saturating_add(1);
        self.freshness = WalletRescanFreshness::Partial;
        self.state = WalletRescanJobState::Scanning;
        self.maybe_error = None;
    }

    pub fn mark_failed(&mut self, error: impl Into<String>) {
        self.state = WalletRescanJobState::Failed;
        self.maybe_error = Some(error.into());
    }

    pub fn requires_resume(&self) -> bool {
        matches!(
            self.state,
            WalletRescanJobState::Pending | WalletRescanJobState::Scanning
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WalletRegistryError {
    DuplicateWalletName(String),
    UnknownWallet(String),
    StaleSelection(String),
    Wallet(String),
    Storage(StorageError),
}

impl std::fmt::Display for WalletRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateWalletName(wallet_name) => {
                write!(f, "duplicate wallet name: {wallet_name}")
            }
            Self::UnknownWallet(wallet_name) => write!(f, "unknown wallet: {wallet_name}"),
            Self::StaleSelection(detail) => write!(f, "stale wallet selection: {detail}"),
            Self::Wallet(detail) => write!(f, "wallet runtime failure: {detail}"),
            Self::Storage(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for WalletRegistryError {}

impl From<StorageError> for WalletRegistryError {
    fn from(value: StorageError) -> Self {
        Self::Storage(value)
    }
}

impl From<WalletError> for WalletRegistryError {
    fn from(value: WalletError) -> Self {
        Self::Wallet(value.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WalletRegistry {
    snapshot: WalletRegistrySnapshot,
    maybe_selected_wallet_name: Option<String>,
    wallet_snapshots: BTreeMap<String, WalletSnapshot>,
    rescan_jobs: BTreeMap<String, WalletRescanJob>,
}

impl WalletRegistry {
    pub fn load(store: &FjallNodeStore) -> Result<Self, WalletRegistryError> {
        let snapshot = store.load_wallet_registry()?.unwrap_or_default();
        let maybe_selected_wallet_name = store
            .load_selected_wallet()?
            .map(|record| record.wallet_name);

        let mut wallet_snapshots = BTreeMap::new();
        for wallet_name in &snapshot.wallet_names {
            let Some(wallet_snapshot) = store.load_named_wallet_snapshot(wallet_name)? else {
                return Err(StorageError::Corruption {
                    namespace: StorageNamespace::Wallet,
                    detail: format!(
                        "wallet registry references missing wallet snapshot for {wallet_name}"
                    ),
                    action: StorageRecoveryAction::RestoreFromBackup,
                }
                .into());
            };
            wallet_snapshots.insert(wallet_name.clone(), wallet_snapshot);
        }

        if let Some(selected_wallet_name) = maybe_selected_wallet_name.as_deref()
            && !wallet_snapshots.contains_key(selected_wallet_name)
        {
            return Err(WalletRegistryError::StaleSelection(
                selected_wallet_name.to_string(),
            ));
        }

        let mut rescan_jobs = BTreeMap::new();
        for job in store.load_wallet_rescan_jobs()? {
            if !wallet_snapshots.contains_key(job.wallet_name.as_str()) {
                return Err(StorageError::Corruption {
                    namespace: StorageNamespace::Wallet,
                    detail: format!(
                        "wallet rescan job references unknown wallet {}",
                        job.wallet_name
                    ),
                    action: StorageRecoveryAction::RestoreFromBackup,
                }
                .into());
            }
            rescan_jobs.insert(job.wallet_name.clone(), job);
        }

        Ok(Self {
            snapshot,
            maybe_selected_wallet_name,
            wallet_snapshots,
            rescan_jobs,
        })
    }

    pub fn wallet_names(&self) -> &[String] {
        &self.snapshot.wallet_names
    }

    pub fn selected_wallet_name(&self) -> Option<&str> {
        self.maybe_selected_wallet_name.as_deref()
    }

    pub fn require_selected_wallet_name(&self) -> Result<&str, WalletRegistryError> {
        self.selected_wallet_name().ok_or_else(|| {
            WalletRegistryError::StaleSelection(MISSING_SELECTED_WALLET_DETAIL.to_string())
        })
    }

    pub fn wallet_snapshot(
        &self,
        wallet_name: &str,
    ) -> Result<&WalletSnapshot, WalletRegistryError> {
        self.wallet_snapshots
            .get(wallet_name)
            .ok_or_else(|| WalletRegistryError::UnknownWallet(wallet_name.to_string()))
    }

    pub fn wallet(&self, wallet_name: &str) -> Result<Wallet, WalletRegistryError> {
        Ok(Wallet::from_snapshot(
            self.wallet_snapshot(wallet_name)?.clone(),
        ))
    }

    pub fn rescan_job(&self, wallet_name: &str) -> Option<&WalletRescanJob> {
        self.rescan_jobs.get(wallet_name)
    }

    pub fn rescan_jobs(&self) -> impl Iterator<Item = &WalletRescanJob> {
        self.rescan_jobs.values()
    }

    pub fn create_wallet(
        &mut self,
        store: &FjallNodeStore,
        wallet_name: impl Into<String>,
        wallet: Wallet,
        mode: PersistMode,
    ) -> Result<(), WalletRegistryError> {
        let wallet_name = wallet_name.into();
        if self.wallet_snapshots.contains_key(wallet_name.as_str()) {
            return Err(WalletRegistryError::DuplicateWalletName(wallet_name));
        }

        let snapshot = wallet.snapshot();
        store.save_named_wallet_snapshot(&wallet_name, &snapshot, mode)?;
        self.wallet_snapshots.insert(wallet_name.clone(), snapshot);
        self.snapshot = WalletRegistrySnapshot::new(self.wallet_snapshots.keys().cloned());
        store.save_wallet_registry(&self.snapshot, mode)?;
        Ok(())
    }

    pub fn save_wallet(
        &mut self,
        store: &FjallNodeStore,
        wallet_name: &str,
        wallet: &Wallet,
        mode: PersistMode,
    ) -> Result<(), WalletRegistryError> {
        if !self.wallet_snapshots.contains_key(wallet_name) {
            return Err(WalletRegistryError::UnknownWallet(wallet_name.to_string()));
        }

        let snapshot = wallet.snapshot();
        store.save_named_wallet_snapshot(wallet_name, &snapshot, mode)?;
        self.wallet_snapshots
            .insert(wallet_name.to_string(), snapshot);
        Ok(())
    }

    pub fn set_selected_wallet(
        &mut self,
        store: &FjallNodeStore,
        wallet_name: &str,
        mode: PersistMode,
    ) -> Result<(), WalletRegistryError> {
        if !self.wallet_snapshots.contains_key(wallet_name) {
            return Err(WalletRegistryError::UnknownWallet(wallet_name.to_string()));
        }

        let record = SelectedWalletRecord {
            wallet_name: wallet_name.to_string(),
        };
        store.save_selected_wallet(&record, mode)?;
        self.maybe_selected_wallet_name = Some(record.wallet_name);
        Ok(())
    }

    pub fn save_rescan_job(
        &mut self,
        store: &FjallNodeStore,
        job: WalletRescanJob,
        mode: PersistMode,
    ) -> Result<(), WalletRegistryError> {
        if !self.wallet_snapshots.contains_key(job.wallet_name.as_str()) {
            return Err(WalletRegistryError::UnknownWallet(job.wallet_name));
        }

        store.save_wallet_rescan_job(&job, mode)?;
        self.rescan_jobs.insert(job.wallet_name.clone(), job);
        Ok(())
    }

    pub fn clear_rescan_job(
        &mut self,
        store: &FjallNodeStore,
        wallet_name: &str,
        mode: PersistMode,
    ) -> Result<(), WalletRegistryError> {
        store.clear_wallet_rescan_job(wallet_name, mode)?;
        self.rescan_jobs.remove(wallet_name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs, io,
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    };

    use open_bitcoin_core::{
        primitives::BlockHash,
        wallet::{AddressNetwork, Wallet},
    };

    use super::{WalletRegistry, WalletRegistryError, WalletRescanJob};
    use crate::{FjallNodeStore, PersistMode};

    fn temp_store_path(test_name: &str) -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "open-bitcoin-wallet-registry-{test_name}-{}-{timestamp}",
            std::process::id()
        ))
    }

    fn remove_dir_if_exists(path: &Path) {
        match fs::remove_dir_all(path) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => panic!("failed to remove {}: {error}", path.display()),
        }
    }

    #[test]
    fn missing_selected_wallet_metadata_and_unknown_lookups_return_typed_errors() {
        // Arrange
        let path = temp_store_path("typed-errors");
        remove_dir_if_exists(&path);
        let store = FjallNodeStore::open(&path).expect("open store");
        let mut registry = WalletRegistry::default();
        registry
            .create_wallet(
                &store,
                "alpha",
                Wallet::new(AddressNetwork::Regtest),
                PersistMode::Sync,
            )
            .expect("save alpha");

        // Act
        let stale_selection = registry
            .require_selected_wallet_name()
            .expect_err("missing selected wallet metadata");
        let unknown_wallet = registry
            .wallet_snapshot("missing")
            .expect_err("missing wallet lookup");

        // Assert
        assert_eq!(
            stale_selection,
            WalletRegistryError::StaleSelection("selected wallet metadata missing".to_string())
        );
        assert_eq!(
            unknown_wallet,
            WalletRegistryError::UnknownWallet("missing".to_string())
        );

        remove_dir_if_exists(&path);
    }

    #[test]
    fn duplicate_wallet_names_and_reopen_visible_jobs_are_rejected_or_restored() {
        // Arrange
        let path = temp_store_path("duplicates-and-jobs");
        remove_dir_if_exists(&path);
        let store = FjallNodeStore::open(&path).expect("open store");
        let mut registry = WalletRegistry::default();
        registry
            .create_wallet(
                &store,
                "alpha",
                Wallet::new(AddressNetwork::Regtest),
                PersistMode::Sync,
            )
            .expect("save alpha");
        registry
            .set_selected_wallet(&store, "alpha", PersistMode::Sync)
            .expect("select alpha");
        let job =
            WalletRescanJob::new("alpha", BlockHash::from_byte_array([9_u8; 32]), 10, 0, None)
                .expect("job");
        registry
            .save_rescan_job(&store, job.clone(), PersistMode::Sync)
            .expect("save job");

        // Act
        let duplicate = registry
            .create_wallet(
                &store,
                "alpha",
                Wallet::new(AddressNetwork::Regtest),
                PersistMode::Sync,
            )
            .expect_err("duplicate wallet");
        let reopened = WalletRegistry::load(&store).expect("reload registry");

        // Assert
        assert_eq!(
            duplicate,
            WalletRegistryError::DuplicateWalletName("alpha".to_string())
        );
        assert_eq!(reopened.selected_wallet_name(), Some("alpha"));
        assert_eq!(reopened.wallet_names(), &["alpha".to_string()]);
        assert_eq!(reopened.rescan_job("alpha"), Some(&job));

        remove_dir_if_exists(&path);
    }
}
