// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/txdb.cpp

//! Canonical genesis startup keeps ordinary history and networking unqualified.

use super::{DurableSyncRuntime, SyncNetwork, SyncRuntimeConfig, SyncRuntimeError, progress};
use crate::{
    ManagedChainstate, ManagedPeerNetwork, chainstate::BasicFilterStartupMode,
    storage::OpenedCanonicalGenesis,
};
use open_bitcoin_core::consensus::CanonicalChain;

impl DurableSyncRuntime {
    /// Consume an exclusively proved store before initialization or index writes.
    /// The resulting owner contains only canonical genesis and starts no workers.
    pub fn open_canonical_genesis_configured(
        opened: OpenedCanonicalGenesis,
        config: SyncRuntimeConfig,
        mode: BasicFilterStartupMode,
    ) -> Result<Self, SyncRuntimeError> {
        let selected = match config.network {
            SyncNetwork::Mainnet => CanonicalChain::Mainnet,
            SyncNetwork::Testnet => CanonicalChain::Testnet3,
            SyncNetwork::Signet => CanonicalChain::Signet,
            SyncNetwork::Regtest => CanonicalChain::Regtest,
        };
        if opened.genesis().chain() != selected {
            return Err(SyncRuntimeError::InvalidData {
                message: "canonical genesis selected network mismatch".to_owned(),
            });
        }
        let (store, recovered) =
            opened
                .into_parts()
                .map_err(|_| SyncRuntimeError::InvalidData {
                    message: "canonical genesis store proof changed".to_owned(),
                })?;
        #[cfg(test)]
        basic::inject(&store, basic::Stage::Configure);
        let mode = basic::resolve_startup_mode(&store, mode)?;
        let (managed, proof) =
            ManagedChainstate::from_canonical_genesis(store.clone(), recovered, mode)?;
        let mut network = ManagedPeerNetwork::from_canonical_genesis_initialized(
            managed,
            progress::local_peer_config(&config),
            config.max_blocks_in_flight_per_peer,
            &proof,
        )?;
        network.record_basic_indexing_configuration(
            store.resolved_basic_filter_indexing_after_startup()?,
        );
        #[cfg(test)]
        basic::inject(&store, basic::Stage::Append);
        Self::finish_initialized_runtime(
            store,
            network,
            config,
            open_bitcoin_core::chainstate::engine::canonical_genesis::consensus_params(&proof),
        )
    }
}

mod basic;
#[cfg(test)]
mod basic_tests;
#[cfg(test)]
mod tests;
