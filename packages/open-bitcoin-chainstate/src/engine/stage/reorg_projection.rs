// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::{Chainstate, ChainstateError, CoinsView, StagedChainstateReorg};

impl<V: CoinsView> Chainstate<V> {
    /// Inspect the complete staged replacement without granting acceptance authority.
    pub fn project_staged_reorg(
        &self,
        staged: &StagedChainstateReorg,
    ) -> Result<crate::ChainstateSnapshot, ChainstateError> {
        self.check_staged_reorg(staged)?;
        let mut snapshot = self.admission_snapshot()?;
        staged.overlay.overlay_unspent_into(&mut snapshot.utxos);
        snapshot.active_chain = staged.next_active_chain.clone();
        snapshot.undo_by_block = staged.next_undo_by_block.clone();
        snapshot.maybe_confirmed_txid_counts = staged.next_confirmed_txid_counts.clone();
        Ok(snapshot)
    }
}
