// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/headerssync.cpp

use super::filter_index::BasicBlockValidationProvenance;
use super::*;
use crate::chainstate::BasicFilterStartupMode;
use crate::sync::{DurableSyncRuntime, SyncNetwork, SyncRuntimeConfig, SyncRuntimeError};
use crate::{FjallChainstateStore, FjallCoinsView, FjallNodeStore};
use open_bitcoin_core::chainstate::{
    ChainstateSnapshot, IndexInputProtection, coins::CompleteCoinsSnapshot,
    filter_index::catch_up::BasicIndexProgress,
};
use open_bitcoin_core::codec::block::wire::WireBlock;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
mod workflow;

type SubmissionNetwork = ManagedPeerNetwork<FjallChainstateStore, FjallCoinsView>;
type SubmissionResult<T> = Result<T, CanonicalSubmissionError>;

type ObservedCoins = (ChainstateSnapshot, Option<BlockHash>, CompleteCoinsSnapshot);
type ObservedStoredBasic = (
    Option<(u32, BlockHash)>,
    u32,
    BlockHash,
    IndexInputProtection,
);
type ObservedNativeRows = BTreeMap<String, BTreeMap<Vec<u8>, Vec<u8>>>;
type ObservedProvenance = BTreeMap<
    BlockHash,
    (
        BasicBlockValidationProvenance,
        BasicBlockValidationProvenance,
    ),
>;

/// Detached complete facts; construction grants no work or publication authority.
#[derive(Debug, PartialEq, Eq)]
pub struct CanonicalAcceptedStateObservation {
    coins: ObservedCoins,
    maybe_progress: Option<BasicIndexProgress>,
    maybe_stored_basic: Option<ObservedStoredBasic>,
    rows: ObservedNativeRows,
    provenance: ObservedProvenance,
}

impl CanonicalAcceptedStateObservation {
    /// Consume owned data after the command has released every authority guard.
    pub fn fold<T>(
        self,
        consume: impl FnOnce(
            ObservedCoins,
            Option<BasicIndexProgress>,
            Option<ObservedStoredBasic>,
            ObservedNativeRows,
            ObservedProvenance,
        ) -> T,
    ) -> T {
        consume(
            self.coins,
            self.maybe_progress,
            self.maybe_stored_basic,
            self.rows,
            self.provenance,
        )
    }
}

fn operational<E: std::error::Error + Send + Sync + 'static>(
    stage: &'static str,
) -> impl FnOnce(E) -> CanonicalSubmissionError {
    move |error| CanonicalSubmissionError::operation(stage, error)
}

/// Operational failure retains its actual source; it is never consensus invalidity.
#[derive(Debug)]
pub struct CanonicalSubmissionError {
    stage: &'static str,
    maybe_source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl CanonicalSubmissionError {
    fn support(stage: &'static str) -> Self {
        Self {
            stage,
            maybe_source: None,
        }
    }
    fn operation(
        stage: &'static str,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            stage,
            maybe_source: Some(Box::new(source)),
        }
    }
}
impl fmt::Display for CanonicalSubmissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.stage)?;
        if let Some(source) = &self.maybe_source {
            write!(f, ": {source}")?;
        }
        Ok(())
    }
}
impl std::error::Error for CanonicalSubmissionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.maybe_source
            .as_ref()
            .map(|source| source.as_ref() as _)
    }
}

/// Genuine results; availability and known headers alone do not imply acceptance.
#[derive(Debug)]
pub enum CanonicalSubmissionOutcome {
    Accepted(ChainPosition),
    Duplicate(BlockHash),
    Rejected(String),
    Inconclusive,
}
impl CanonicalSubmissionOutcome {
    /// Consume a result through inference across the private module boundary.
    pub fn fold<T>(
        self,
        accepted: impl FnOnce(ChainPosition) -> T,
        duplicate: impl FnOnce(BlockHash) -> T,
        rejected: impl FnOnce(String) -> T,
        inconclusive: impl FnOnce() -> T,
    ) -> T {
        match self {
            Self::Accepted(position) => accepted(position),
            Self::Duplicate(hash) => duplicate(hash),
            Self::Rejected(reason) => rejected(reason),
            Self::Inconclusive => inconclusive(),
        }
    }
}

/// Factory-earned command bound to the same held network, store and incarnation.
/// No public arbitrary-runtime constructor or Default exists.
pub struct CanonicalSubmissionCommand {
    network: ManagedNetworkHandle<FjallChainstateStore, FjallCoinsView>,
    store: FjallNodeStore,
    params: ConsensusParams,
    chain: CanonicalChain,
    incarnation: crate::network::lifecycle_projection::AuthorityEpoch,
    operation: Mutex<()>,
}

fn selected(config: &SyncRuntimeConfig) -> CanonicalChain {
    match config.network {
        SyncNetwork::Mainnet => CanonicalChain::Mainnet,
        SyncNetwork::Testnet => CanonicalChain::Testnet3,
        SyncNetwork::Signet => CanonicalChain::Signet,
        SyncNetwork::Regtest => CanonicalChain::Regtest,
    }
}
fn startup_error(error: impl fmt::Display) -> SyncRuntimeError {
    SyncRuntimeError::InvalidData {
        message: error.to_string(),
    }
}

impl DurableSyncRuntime {
    /// Open one proved configured owner and its optional Regtest submission command.
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use open_bitcoin_node::{DurableSyncRuntime, SyncRuntimeConfig};
    /// use open_bitcoin_node::chainstate::BasicFilterStartupMode;
    /// let (_runtime, command) = DurableSyncRuntime::open_canonical_submission_configured(
    ///     "node-data", SyncRuntimeConfig::default(), BasicFilterStartupMode::Disabled)?;
    /// if let Some(command) = command {
    ///     let (wire, _) = open_bitcoin_core::codec::block::wire::parse_submission_block_prefix(&[])?;
    ///     command.submit(wire)?.fold(|_| (), |_| (), |_| (), || ());
    /// }
    /// # Ok(()) }
    /// ```
    pub fn open_canonical_submission_configured(
        path: impl AsRef<Path>,
        config: SyncRuntimeConfig,
        mode: BasicFilterStartupMode,
    ) -> Result<(Self, Option<CanonicalSubmissionCommand>), SyncRuntimeError> {
        let chain = selected(&config);
        let empty = match std::fs::read_dir(path.as_ref()) {
            Ok(mut entries) => entries.next().transpose().map_err(startup_error)?.is_none(),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => true,
            Err(error) => return Err(startup_error(error)),
        };
        if empty || chain != CanonicalChain::Regtest {
            let opened =
                FjallNodeStore::open_canonical_genesis(path, chain).map_err(startup_error)?;
            return Self::open_genesis_submission_configured(opened, config, mode);
        }
        let runtime = Self::reopen_canonical_history_configured(path, config, mode)?;
        let network = runtime.network_handle();
        let (maybe_params, incarnation) = network.read(|network| {
            (
                network.maybe_header_consensus_params,
                network.authority_epoch(),
            )
        })?;
        let command = CanonicalSubmissionCommand {
            network,
            store: runtime.store().clone(),
            params: maybe_params.ok_or_else(|| startup_error("missing held parameters"))?,
            chain,
            incarnation,
            operation: Mutex::new(()),
        };
        command
            .network
            .read(|network| command.check_owner(network))?
            .map_err(startup_error)?;
        Ok((runtime, Some(command)))
    }

    /// Consume the genuine genesis seal directly without reopening its held lease.
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use open_bitcoin_node::{DurableSyncRuntime, FjallNodeStore, SyncRuntimeConfig};
    /// use open_bitcoin_node::chainstate::BasicFilterStartupMode;
    /// let opened = FjallNodeStore::open_canonical_genesis("node-data",
    ///     open_bitcoin_core::consensus::CanonicalChain::Regtest)?;
    /// let (_runtime, command) = DurableSyncRuntime::open_genesis_submission_configured(
    ///     opened, SyncRuntimeConfig { network: open_bitcoin_node::SyncNetwork::Regtest,
    ///     ..Default::default() }, BasicFilterStartupMode::Disabled)?;
    /// if let Some(command) = command {
    ///     let (wire, _) = open_bitcoin_core::codec::block::wire::parse_submission_block_prefix(&[])?;
    ///     command.submit(wire)?.fold(|_| (), |_| (), |_| (), || ());
    /// }
    /// # Ok(()) }
    /// ```
    pub fn open_genesis_submission_configured(
        opened: crate::storage::OpenedCanonicalGenesis,
        config: SyncRuntimeConfig,
        mode: BasicFilterStartupMode,
    ) -> Result<(Self, Option<CanonicalSubmissionCommand>), SyncRuntimeError> {
        let chain = selected(&config);
        let runtime = Self::open_canonical_genesis_configured(opened, config, mode)?;
        if chain != CanonicalChain::Regtest {
            return Ok((runtime, None));
        }
        let network = runtime.network_handle();
        let (maybe_params, incarnation) = network.read(|network| {
            (
                network.maybe_header_consensus_params,
                network.authority_epoch(),
            )
        })?;
        let command = CanonicalSubmissionCommand {
            network,
            store: runtime.store().clone(),
            params: maybe_params.ok_or_else(|| startup_error("missing held parameters"))?,
            chain,
            incarnation,
            operation: Mutex::new(()),
        };
        command
            .network
            .read(|network| command.check_owner(network))?
            .map_err(startup_error)?;
        Ok((runtime, Some(command)))
    }
}

impl CanonicalSubmissionCommand {
    /// Observe the same held owner without driving persistence or BASIC work.
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use std::collections::BTreeMap;
    /// use open_bitcoin_core::{primitives::BlockHash, chainstate::{
    ///     ChainstateSnapshot, IndexInputProtection, coins::CompleteCoinsSnapshot,
    ///     filter_index::catch_up::BasicIndexProgress}};
    /// use open_bitcoin_node::{DurableSyncRuntime, SyncRuntimeConfig, SyncNetwork};
    /// use open_bitcoin_node::chainstate::BasicFilterStartupMode;
    /// use open_bitcoin_node::network::BasicBlockValidationProvenance;
    /// let config = SyncRuntimeConfig { network: SyncNetwork::Regtest, ..Default::default() };
    /// let (_runtime, maybe_command) = DurableSyncRuntime::open_canonical_submission_configured(
    ///     "node-data", config, BasicFilterStartupMode::Enabled)?;
    /// if let Some(command) = maybe_command {
    ///     command.observe_accepted_state(&[])?.fold(|
    ///         coins: (ChainstateSnapshot, Option<BlockHash>, CompleteCoinsSnapshot),
    ///         progress: Option<BasicIndexProgress>,
    ///         stored: Option<(Option<(u32, BlockHash)>, u32, BlockHash, IndexInputProtection)>,
    ///         rows: BTreeMap<String, BTreeMap<Vec<u8>, Vec<u8>>>,
    ///         provenance: BTreeMap<BlockHash, (BasicBlockValidationProvenance,
    ///             BasicBlockValidationProvenance)>| {
    ///         let _ = (coins, progress, stored, rows, provenance);
    ///     });
    /// }
    /// # Ok(()) }
    /// ```
    pub fn observe_accepted_state(
        &self,
        queried_hashes: &[BlockHash],
    ) -> SubmissionResult<CanonicalAcceptedStateObservation> {
        let limit = FjallNodeStore::canonical_history_reopen_limit();
        if queried_hashes.len() > limit {
            return Err(CanonicalSubmissionError::support(
                "observation query bound exceeded",
            ));
        }
        let observed = {
            let _operation = self
                .operation
                .lock()
                .map_err(|_| CanonicalSubmissionError::support("submission mutex unavailable"))?;
            self.network
                .read(|network| {
                    self.check_owner(network)?;
                    let inventory = self
                        .store
                        .inspect_submission_inventory()
                        .map_err(operational("observation native inventory"))?;
                    let headers = network.peer_manager.header_store();
                    if headers.entries().count() > limit {
                        return Err(CanonicalSubmissionError::support(
                            "observation retained header bound exceeded",
                        ));
                    }
                    let mut hashes: BTreeSet<_> =
                        headers.entries().map(|entry| entry.block_hash).collect();
                    // Queries are observations, never novel-candidate admissions.
                    hashes.extend(queried_hashes.iter().copied());
                    let manager = network.chainstate();
                    let coins = (
                        manager
                            .export_chainstate_snapshot()
                            .map_err(operational("observation effective state"))?,
                        manager
                            .chainstate()
                            .coins_best_block()
                            .map_err(operational("observation live Coins"))?,
                        self.store
                            .coins_view()
                            .complete_snapshot()
                            .map_err(operational("observation durable Coins"))?,
                    );
                    let maybe_progress = manager.maybe_basic_index_progress();
                    let maybe_stored_basic = self
                        .store
                        .maybe_basic_filter_state()
                        .map_err(operational("observation stored BASIC"))?
                        .map(|state| {
                            (
                                state.maybe_endpoint,
                                state.fence_height,
                                state.fence_hash,
                                state.protection,
                            )
                        });
                    let provenance = hashes
                        .into_iter()
                        .map(|hash| {
                            Ok((
                                hash,
                                (
                                    manager
                                        .validation_provenance(hash)
                                        .map_err(operational("observation live provenance"))?
                                        .into(),
                                    self.store
                                        .validation_provenance(hash)
                                        .map_err(operational("observation durable provenance"))?
                                        .into(),
                                ),
                            ))
                        })
                        .collect::<SubmissionResult<ObservedProvenance>>()?;
                    self.check_owner(network)?;
                    let rows = inventory
                        .into_observed_rows()
                        .map_err(operational("observation final source"))?;
                    Ok(CanonicalAcceptedStateObservation {
                        coins,
                        maybe_progress,
                        maybe_stored_basic,
                        rows,
                        provenance,
                    })
                })
                .map_err(operational("observation authority"))??
        };
        Ok(observed)
    }

    fn check_owner(&self, network: &SubmissionNetwork) -> SubmissionResult<()> {
        let core = network.chainstate().chainstate();
        core.validate_live_origin()
            .map_err(operational("live origin"))?;
        core.bind_live_work_source(self.params)
            .map_err(operational("selected parameters"))?;
        if self.chain != CanonicalChain::Regtest
            || network.authority_epoch() != self.incarnation
            || network.maybe_canonical_chain() != Some(self.chain)
            || core.maybe_canonical_chain() != Some(self.chain)
            || network.peer_manager.header_store().maybe_canonical_chain() != Some(self.chain)
            || network.maybe_header_consensus_params != Some(self.params)
            || !network
                .chainstate()
                .store()
                .inner()
                .shares_validation_history_store(&self.store)
        {
            return Err(CanonicalSubmissionError::support(
                "submission owner mismatch",
            ));
        }
        Ok(())
    }
    /// Submit complete decoded raw data; all clocks, rules and work come from held owners.
    pub fn submit(&self, block: WireBlock) -> SubmissionResult<CanonicalSubmissionOutcome> {
        let _operation = self
            .operation
            .lock()
            .map_err(|_| CanonicalSubmissionError::support("submission mutex unavailable"))?;
        let mut maybe_achieved = None;
        let outcome = self
            .network
            .mutate(|network| self.execute(network, block, &mut maybe_achieved))
            .map_err(operational("authority"))??;
        if let CanonicalSubmissionOutcome::Accepted(position) = &outcome {
            use super::filter_index::BasicFilterQuery;
            let maybe_barrier = match self
                .network
                .basic_filter_query(position.block_hash)
                .map_err(operational("accepted BASIC capture"))?
            {
                BasicFilterQuery::Pending(barrier) => Some(barrier),
                BasicFilterQuery::Disabled | BasicFilterQuery::Found(_) => None,
                _ => {
                    return Err(CanonicalSubmissionError::support(
                        "accepted BASIC target unavailable",
                    ));
                }
            };
            self.network
                .drive_basic_filter_index_turn()
                .map_err(operational("accepted BASIC turn"))?;
            if let Some(mut barrier) = maybe_barrier {
                use std::future::Future;
                match std::pin::Pin::new(&mut barrier)
                    .poll(&mut std::task::Context::from_waker(std::task::Waker::noop()))
                {
                    std::task::Poll::Ready(result) => {
                        result.map_err(operational("accepted BASIC barrier"))?;
                    }
                    std::task::Poll::Pending => {
                        return Err(CanonicalSubmissionError::support(
                            "accepted BASIC work remains pending",
                        ));
                    }
                }
            }
            #[cfg(test)]
            tests::after_basic(self);
            let achieved = maybe_achieved.ok_or_else(|| {
                CanonicalSubmissionError::support("accepted inventory observation absent")
            })?;
            self.network
                .mutate(|network| {
                    self.confirm_durable(network, position)?;
                    achieved(&self.store).map_err(operational("accepted achieved inventory"))?;
                    self.store
                        .check_submission_physical_inventory()
                        .map_err(operational("accepted physical inventory"))?;
                    self.finish_retained_body(
                        network,
                        position.block_hash,
                        "accepted retained body",
                        "accepted retained body unavailable",
                    )
                })
                .map_err(operational("accepted final authority"))??;
        }
        if let CanonicalSubmissionOutcome::Duplicate(hash) = &outcome {
            self.network
                .mutate(|network| {
                    self.check_owner(network)?;
                    self.finish_retained_body(
                        network,
                        *hash,
                        "duplicate retained body",
                        "duplicate retained body unavailable",
                    )
                })
                .map_err(operational("duplicate final authority"))??;
        }
        Ok(outcome)
    }

    fn finish_retained_body(
        &self,
        network: &mut SubmissionNetwork,
        hash: BlockHash,
        read_stage: &'static str,
        missing_stage: &'static str,
    ) -> SubmissionResult<()> {
        let body = self
            .store
            .load_block(hash)
            .map_err(operational(read_stage))?
            .ok_or_else(|| CanonicalSubmissionError::support(missing_stage))?;
        if !network.finish_bookkeeping(&body) {
            return Err(CanonicalSubmissionError::support(
                "retained body eligibility unavailable",
            ));
        }
        Ok(())
    }

    fn accepted_identity(
        &self,
        network: &SubmissionNetwork,
        block: &Block,
    ) -> SubmissionResult<bool> {
        use crate::storage::validation_history::{StoredValidationStatus, ValidationProvenance};
        let hash = open_bitcoin_core::consensus::block_hash(&block.header);
        let manager = network.chainstate();
        let maybe_pending = manager.maybe_pending_validation_identity(hash);
        let maybe_record = self
            .store
            .maybe_validation_history_record(hash)
            .map_err(operational("accepted record"))?;
        let maybe_identity = maybe_pending.or_else(|| {
            maybe_record
                .filter(|record| record.status() == StoredValidationStatus::ScriptsValid)
                .map(|record| record.identity())
        });
        let Some(identity) = maybe_identity else {
            return Ok(false);
        };
        let headers = network.peer_manager.header_store();
        if identity.parent_hash() != block.header.previous_block_hash
            || manager
                .validation_provenance(hash)
                .map_err(operational("accepted provenance"))?
                != ValidationProvenance::ScriptsValid
            || !headers.is_canonically_admitted(hash)
            || headers.entry(&hash).is_none_or(|entry| {
                entry.header != block.header || entry.height != identity.height()
            })
        {
            return Err(CanonicalSubmissionError::support(
                "accepted identity correspondence",
            ));
        }
        let maybe_body = match network.blocks_by_hash.get(&hash) {
            Some(body) => Some(body.clone()),
            None => self
                .store
                .load_block(hash)
                .map_err(operational("accepted body"))?,
        };
        let Some(body) = maybe_body else {
            return Err(CanonicalSubmissionError::support(
                "accepted body unavailable",
            ));
        };
        if body.header != block.header
            || open_bitcoin_core::consensus::crypto::block_merkle_root(&body.transactions)
                .map_err(operational("accepted body identity"))?
                .0
                != block.header.merkle_root
        {
            return Err(CanonicalSubmissionError::support(
                "accepted body identity mismatch",
            ));
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests;

impl<S: ChainstateStore, V: CoinsView> ManagedNetworkHandle<S, V> {
    pub(crate) fn persist_validation_header_snapshot(
        &self,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.try_read(ManagedPeerNetwork::persist_validation_header_snapshot)
    }
}
