// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/txdb.cpp

//! Store-bound recovery of an earned genesis chainstate owner.

use super::*;
use crate::{
    chainstate::{BasicFilterStartupMode, initialize_configured},
    storage::fjall_store::canonical_genesis::RecoveredCanonicalGenesis,
};
use open_bitcoin_core::{
    chainstate::FlushPolicyTime, consensus::chain_spec::CanonicalGenesisProof,
};

impl ManagedChainstate<FjallChainstateStore, FjallCoinsView> {
    /// Consume original-store evidence before recovery or index configuration writes.
    pub(crate) fn from_canonical_genesis(
        store: FjallNodeStore,
        recovered: RecoveredCanonicalGenesis,
        mode: BasicFilterStartupMode,
    ) -> Result<(Self, CanonicalGenesisProof), StorageError> {
        let proof = recovered
            .check(&store)
            .map_err(|_| receipt_error("canonical genesis store proof changed"))?
            .clone();
        let now = FlushPolicyTime::from_unix_seconds(0);
        let (lifecycle, _view, cache) =
            initialize_configured(&store, now, now, 0, false, u64::MAX, mode)?;
        let chainstate = Chainstate::from_canonical_genesis(&proof, cache)
            .map_err(|_| receipt_error("canonical genesis complete coins binding mismatch"))?;
        // BASIC lineage is conditional on the genuine active index proof. The
        // earned core genesis owner exists independently when indexing is off.
        let managed = Self::from_recovered_chainstate(
            FjallChainstateStore::from_store(store),
            chainstate,
            lifecycle,
        )?;
        Ok((managed, proof))
    }
}

#[cfg(test)]
mod tests;
