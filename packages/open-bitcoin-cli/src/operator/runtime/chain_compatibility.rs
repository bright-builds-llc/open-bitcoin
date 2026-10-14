// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Selected operator identity and bounded compatibility diagnostics.

use open_bitcoin_node::{
    SyncNetwork,
    core::{consensus::CanonicalChain, wallet::AddressNetwork},
};
use open_bitcoin_rpc::config::{RuntimeConfig, selected_canonical_chain};

use super::OperatorRuntimeError;
use crate::operator::{NetworkSelection, config::OperatorConfigResolution};

pub(crate) fn canonical_chain_from_resolution(
    resolution: &OperatorConfigResolution,
) -> Result<CanonicalChain, OperatorRuntimeError> {
    let mut runtime = RuntimeConfig::default();
    if let Some(network) = resolution.maybe_network {
        let (chain, sync) = match network {
            NetworkSelection::Mainnet => (AddressNetwork::Mainnet, SyncNetwork::Mainnet),
            NetworkSelection::Testnet => (AddressNetwork::Testnet, SyncNetwork::Testnet),
            NetworkSelection::Signet => (AddressNetwork::Signet, SyncNetwork::Signet),
            NetworkSelection::Regtest => (AddressNetwork::Regtest, SyncNetwork::Regtest),
        };
        runtime.chain = chain;
        runtime.sync.runtime.network = sync;
    }
    selected_canonical_chain(&runtime).map_err(|error| OperatorRuntimeError::InvalidRequest {
        message: error.to_string(),
    })
}

#[cfg(test)]
pub(crate) mod tests;
