// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/kernel/chainparams.cpp

use super::*;
use open_bitcoin_consensus::CanonicalChain;
use open_bitcoin_consensus::chain_spec::CanonicalGenesisProof;

/// Existing selected-network parameters for genesis owner binding only.
/// This does not qualify contextual history or implement Signet challenge checks.
pub fn consensus_params(proof: &CanonicalGenesisProof) -> ConsensusParams {
    let (pow_limit_bits, allow_min_difficulty_blocks, no_pow_retargeting) = match proof.chain() {
        CanonicalChain::Mainnet => (0x1d00_ffff, false, false),
        CanonicalChain::Testnet3 => (0x1d00_ffff, true, false),
        CanonicalChain::Signet => (0x1e03_77ae, false, false),
        CanonicalChain::Regtest => (0x207f_ffff, true, true),
    };
    ConsensusParams {
        pow_limit_bits,
        allow_min_difficulty_blocks,
        no_pow_retargeting,
        ..ConsensusParams::default()
    }
}

impl<V: CoinsView> Chainstate<V> {
    /// Earn only the proved genesis root from a complete, empty effective coins view.
    pub fn from_canonical_genesis(
        proof: &CanonicalGenesisProof,
        coins: CoinsCache<V>,
    ) -> Result<Self, ChainstateError> {
        let snapshot = coins.complete_snapshot()?;
        if snapshot.maybe_best_block != Some(proof.hash())
            || !snapshot.coins.is_empty()
            || !snapshot.head_blocks.is_empty()
        {
            return Err(ChainstateError::WorkRebindMismatch);
        }
        // The sealed proof already guarantees the null genesis parent.
        let position = ChainPosition::new(
            proof.header().clone(),
            0,
            proof.work(),
            i64::from(proof.header().time),
        );
        let mut work_authority = WorkAuthority::new(WorkOrigin::Live);
        let binding = work_authority.bind(Some(&position), consensus_params(proof));
        work_authority.accept(&binding);
        // Knots skips genesis transaction connection entirely, including its outputs.
        Ok(Self {
            work_authority,
            active_chain: vec![position],
            coins,
            undo_by_block: HashMap::new(),
            maybe_confirmed_txid_counts: Some(HashMap::new()),
        })
    }
}

#[cfg(test)]
mod tests;
