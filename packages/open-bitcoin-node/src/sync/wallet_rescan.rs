// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use std::collections::BTreeSet;

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
        let chainstate = self.required_chainstate_snapshot()?;
        let mut registry = WalletRegistry::load(&self.store)?;
        let mut job = registry
            .rescan_job(wallet_name)
            .cloned()
            .ok_or_else(|| WalletRegistryError::UnknownWallet(wallet_name.to_string()))?;
        if !job.requires_resume() {
            return Ok(job);
        }

        let chunk_end_height =
            chunk_end_height(job.next_height, job.target_tip_height, self.chunk_size);
        let partial_snapshot = partial_chainstate_snapshot(&chainstate, chunk_end_height);
        let maybe_tip_median_time_past = partial_snapshot.tip().map(|tip| tip.median_time_past);
        let mut wallet = registry.wallet(wallet_name)?;
        if let Err(error) = wallet.rescan_chainstate(&partial_snapshot) {
            job.mark_failed(error.to_string());
            registry.save_rescan_job(&self.store, job.clone(), self.persist_mode)?;
            return Err(error.into());
        }

        registry.save_wallet(&self.store, wallet_name, &wallet, self.persist_mode)?;
        job.mark_chunk_progress(chunk_end_height, maybe_tip_median_time_past);
        registry.save_rescan_job(&self.store, job.clone(), self.persist_mode)?;
        Ok(job)
    }

    fn required_chainstate_snapshot(
        &self,
    ) -> Result<open_bitcoin_core::chainstate::ChainstateSnapshot, WalletRegistryError> {
        self.store.load_chainstate_snapshot()?.ok_or({
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

fn partial_chainstate_snapshot(
    snapshot: &open_bitcoin_core::chainstate::ChainstateSnapshot,
    through_height: u32,
) -> open_bitcoin_core::chainstate::ChainstateSnapshot {
    let active_chain = snapshot
        .active_chain
        .iter()
        .filter(|position| position.height <= through_height)
        .cloned()
        .collect::<Vec<_>>();
    let active_hashes = active_chain
        .iter()
        .map(|position| position.block_hash)
        .collect::<BTreeSet<_>>();
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

    open_bitcoin_core::chainstate::ChainstateSnapshot::new(active_chain, utxos, undo_by_block)
}
