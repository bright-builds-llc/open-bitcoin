// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use crate::wallet_registry::rescan::{
    PreparedWalletRescan, WalletRescanEligibilityFailure, prepare_durable_wallet_rescan,
};
use open_bitcoin_core::{chainstate::ChainstateSnapshot, wallet::Wallet};
use open_bitcoin_wallet::wallet::WalletRescanState;

use crate::{
    FjallNodeStore, PersistMode, StorageError, StorageNamespace,
    wallet_registry::{
        WalletRegistry, WalletRegistryError, WalletRescanFreshness, WalletRescanJob,
        WalletRescanJobState,
    },
};

const DEFAULT_WALLET_RESCAN_CHUNK_SIZE: u32 = 128;

pub struct WalletRescanRuntime {
    store: FjallNodeStore,
    persist_mode: PersistMode,
    chunk_size: u32,
}

impl WalletRescanRuntime {
    pub fn open(
        store: FjallNodeStore,
        persist_mode: PersistMode,
    ) -> Result<Self, WalletRegistryError> {
        Self::open_with_chunk_size(store, persist_mode, DEFAULT_WALLET_RESCAN_CHUNK_SIZE)
    }

    pub(crate) fn open_with_chunk_size(
        store: FjallNodeStore,
        persist_mode: PersistMode,
        chunk_size: u32,
    ) -> Result<Self, WalletRegistryError> {
        let runtime = Self {
            store,
            persist_mode,
            chunk_size: chunk_size.max(1),
        };
        let _ = runtime.resume_pending_jobs()?;
        Ok(runtime)
    }

    pub fn store(&self) -> &FjallNodeStore {
        &self.store
    }

    pub fn enqueue_rescan(
        &self,
        wallet_name: &str,
    ) -> Result<WalletRescanJob, WalletRegistryError> {
        let mut registry = WalletRegistry::load(&self.store)?;
        let wallet_snapshot = registry.wallet_snapshot(wallet_name)?;
        let chainstate = self.required_chainstate_snapshot()?;
        let Some(target_tip) = chainstate.tip() else {
            return Err(WalletRegistryError::Storage(
                StorageError::UnavailableNamespace {
                    namespace: StorageNamespace::Chainstate,
                },
            ));
        };

        let mut job = WalletRescanJob::new(
            wallet_name,
            target_tip.block_hash,
            target_tip.height,
            wallet_snapshot
                .maybe_tip_height
                .map_or(0, |height| height.saturating_add(1)),
            wallet_snapshot.maybe_tip_height,
        )?;
        job.state = WalletRescanJobState::Pending;
        job.freshness = WalletRescanFreshness::from_wallet_state(WalletRescanState::from_progress(
            job.maybe_scanned_through_height,
            Some(job.target_tip_height),
            Some(job.next_height),
            true,
        )?);
        registry.save_rescan_job(&self.store, job, self.persist_mode)?;

        self.advance_wallet_rescan(wallet_name)
    }

    pub fn resume_pending_jobs(&self) -> Result<Vec<WalletRescanJob>, WalletRegistryError> {
        let registry = WalletRegistry::load(&self.store)?;
        let pending_wallet_names = registry
            .rescan_jobs()
            .filter(|job| job.requires_resume())
            .map(|job| job.wallet_name.clone())
            .collect::<Vec<_>>();

        let mut advanced_jobs = Vec::with_capacity(pending_wallet_names.len());
        for wallet_name in pending_wallet_names {
            advanced_jobs.push(self.advance_wallet_rescan(wallet_name.as_str())?);
        }
        Ok(advanced_jobs)
    }

    pub fn advance_wallet_rescan(
        &self,
        wallet_name: &str,
    ) -> Result<WalletRescanJob, WalletRegistryError> {
        self.advance_with_preparation(
            wallet_name,
            |wallet, authority, start, through| {
                prepare_durable_wallet_rescan(&self.store, wallet, authority, start, through)
            },
            |registry, job| registry.save_rescan_job(&self.store, job, self.persist_mode),
        )
    }

    fn advance_with_preparation(
        &self,
        wallet_name: &str,
        prepare: impl FnOnce(
            &Wallet,
            &ChainstateSnapshot,
            u32,
            u32,
        ) -> Result<PreparedWalletRescan, WalletRescanEligibilityFailure>,
        save_failure: impl FnOnce(
            &mut WalletRegistry,
            WalletRescanJob,
        ) -> Result<(), WalletRegistryError>,
    ) -> Result<WalletRescanJob, WalletRegistryError> {
        let mut registry = WalletRegistry::load(&self.store)?;
        let mut job = registry
            .rescan_job(wallet_name)
            .cloned()
            .ok_or_else(|| WalletRegistryError::UnknownWallet(wallet_name.to_string()))?;
        if !job.requires_resume() {
            return Ok(job);
        }
        let through = chunk_end_height(job.next_height, job.target_tip_height, self.chunk_size);
        let result = (|| {
            let authority = self
                .required_chainstate_snapshot()
                .map_err(WalletRescanEligibilityFailure::authority)?;
            let wallet = registry
                .wallet(wallet_name)
                .map_err(WalletRescanEligibilityFailure::authority)?;
            prepare(&wallet, &authority, job.next_height, through)
        })();
        let prepared = match result {
            Ok(prepared) => prepared,
            Err(failure) => {
                job.mark_failed(failure.safe_detail());
                save_failure(&mut registry, job)?;
                return Err(failure.error);
            }
        };
        registry.save_wallet(
            &self.store,
            wallet_name,
            &prepared.wallet,
            self.persist_mode,
        )?;
        job.mark_chunk_progress(prepared.through_height, prepared.maybe_tip_median_time_past);
        registry.save_rescan_job(&self.store, job.clone(), self.persist_mode)?;
        Ok(job)
    }

    #[cfg(test)]
    pub(crate) fn advance_with_probe_and_failure_save(
        &self,
        wallet_name: &str,
        probe: impl FnMut(open_bitcoin_core::primitives::BlockHash) -> Result<bool, StorageError>,
        save_failure: impl FnOnce(
            &mut WalletRegistry,
            WalletRescanJob,
        ) -> Result<(), WalletRegistryError>,
    ) -> Result<WalletRescanJob, WalletRegistryError> {
        self.advance_with_preparation(
            wallet_name,
            |wallet, authority, start, through| {
                crate::wallet_registry::rescan::guard_durable_wallet_rescan_authority(&self.store)?;
                crate::wallet_registry::rescan::prepare_wallet_rescan_with_probe(
                    wallet, authority, start, through, probe,
                )
            },
            save_failure,
        )
    }

    fn required_chainstate_snapshot(
        &self,
    ) -> Result<open_bitcoin_core::chainstate::ChainstateSnapshot, WalletRegistryError> {
        self.store.wallet_scan_chainstate_snapshot()?.ok_or({
            WalletRegistryError::Storage(StorageError::UnavailableNamespace {
                namespace: StorageNamespace::Chainstate,
            })
        })
    }
}

fn chunk_end_height(next_height: u32, target_tip_height: u32, chunk_size: u32) -> u32 {
    next_height
        .saturating_add(chunk_size.saturating_sub(1))
        .min(target_tip_height)
}
