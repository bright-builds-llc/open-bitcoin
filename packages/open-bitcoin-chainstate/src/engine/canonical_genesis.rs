// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/kernel/chainparams.cpp

use super::*;
use open_bitcoin_consensus::chain_spec::CanonicalGenesisProof;

/// Complete selected-network parameters at height zero for genesis owner binding only.
/// This does not qualify contextual history or implement Signet challenge checks.
pub fn consensus_params(proof: &CanonicalGenesisProof) -> ConsensusParams {
    let rules = proof.spec().rules();
    ConsensusParams {
        coinbase_maturity: rules.coinbase_maturity(),
        subsidy_halving_interval: rules.subsidy_halving_interval(),
        locktime_threshold: rules.locktime_threshold(),
        sequence_locktime_granularity: rules.sequence_locktime_granularity(),
        pow_limit_bits: rules.pow_limit_bits(),
        pow_target_spacing_seconds: rules.pow_target_spacing_seconds(),
        pow_target_timespan_seconds: rules.pow_target_timespan_seconds(),
        allow_min_difficulty_blocks: rules.allow_min_difficulty_blocks(),
        no_pow_retargeting: rules.no_pow_retargeting(),
        enforce_bip34_height_in_coinbase: rules.bip34_height() == 0,
        enforce_bip113_median_time_past: rules.csv_height() == 0,
        enforce_segwit: rules.segwit_height() == 0,
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
        work_authority.maybe_canonical_rules = Some(proof.spec().rules());
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
