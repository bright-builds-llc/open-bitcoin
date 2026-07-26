// Parity breadcrumbs:
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/validation.cpp

//! Package-facing policy checks over the sparse prospective view.

use open_bitcoin_primitives::Txid;

use crate::{
    LimitDirection, LimitKind, MempoolEntry, MempoolError, PolicyConfig, TransactionVirtualSize,
    effective_admission_fee_rate,
};

use super::ProspectiveMempool;
use crate::pool::candidate::CandidateMempoolView;

impl ProspectiveMempool<'_> {
    pub(in crate::pool) fn enforce_admission_fee(
        &self,
        modified_fee_sats: i64,
        virtual_size: TransactionVirtualSize,
    ) -> Result<(), MempoolError> {
        let effective_fee_rate = effective_admission_fee_rate(
            self.base.config.static_relay_fee_rate,
            self.rolling_fee_state.rolling_fee_rate(),
        );
        crate::pool::enforce_min_relay_fee(effective_fee_rate, modified_fee_sats, virtual_size)
    }

    pub(in crate::pool) fn validate_candidate_limits(
        &self,
        txid: Txid,
    ) -> Result<(), MempoolError> {
        let Some(candidate) = self.maybe_entry(&txid) else {
            return Err(invariant("prospective candidate is missing"));
        };
        validate_limit(
            candidate.ancestor_stats.count,
            self.base.config.max_ancestor_count,
            LimitDirection::Ancestor,
            LimitKind::Count,
            txid,
        )?;
        validate_limit(
            candidate.ancestor_stats.virtual_size.as_usize(),
            self.base.config.max_ancestor_virtual_size,
            LimitDirection::Ancestor,
            LimitKind::VirtualSize,
            txid,
        )?;

        for ancestor_txid in self.collect_ancestors(txid) {
            let Some(ancestor) = self.maybe_entry(&ancestor_txid) else {
                return Err(invariant("prospective ancestor is missing"));
            };
            validate_limit(
                ancestor.descendant_stats.count,
                self.base.config.max_descendant_count,
                LimitDirection::Descendant,
                LimitKind::Count,
                ancestor_txid,
            )?;
            validate_limit(
                ancestor.descendant_stats.virtual_size.as_usize(),
                self.base.config.max_descendant_virtual_size,
                LimitDirection::Descendant,
                LimitKind::VirtualSize,
                ancestor_txid,
            )?;
        }
        Ok(())
    }
}

impl CandidateMempoolView for ProspectiveMempool<'_> {
    fn config(&self) -> &PolicyConfig {
        &self.base.config
    }

    fn maybe_entry(&self, txid: &Txid) -> Option<&MempoolEntry> {
        ProspectiveMempool::maybe_entry(self, txid)
    }
}

fn validate_limit(
    attempted: usize,
    max: usize,
    direction: LimitDirection,
    kind: LimitKind,
    txid: Txid,
) -> Result<(), MempoolError> {
    if attempted <= max {
        return Ok(());
    }
    Err(MempoolError::LimitExceeded {
        direction,
        kind,
        txid: Some(txid),
        attempted,
        max,
    })
}

fn invariant(reason: &'static str) -> MempoolError {
    MempoolError::InternalInvariant {
        reason: reason.to_string(),
    }
}
