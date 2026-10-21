// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use open_bitcoin_chainstate::engine::canonical_genesis::consensus_params;
use open_bitcoin_consensus::chain_spec::CanonicalGenesisProof;

impl HeaderStore {
    /// Construct only the selected proved root, without ordinary-history qualification.
    pub fn from_canonical_genesis(proof: &CanonicalGenesisProof) -> Result<Self, NetworkError> {
        let mut store = Self::default();
        store.insert_header_with_params(proof.header().clone(), &consensus_params(proof))?;
        Ok(store)
    }
}

#[cfg(test)]
mod tests;
