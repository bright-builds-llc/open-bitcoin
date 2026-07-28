// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use std::collections::BTreeSet;

use open_bitcoin_mempool::MempoolMemberIdentity;

use super::ManagedRelayFanoutState;

impl ManagedRelayFanoutState {
    pub(in crate::network) fn lifecycle_mismatch_count(
        &self,
        canonical: &BTreeSet<MempoolMemberIdentity>,
    ) -> usize {
        self.lifecycle_members()
            .symmetric_difference(canonical)
            .count()
    }

    fn lifecycle_members(&self) -> BTreeSet<MempoolMemberIdentity> {
        self.wtxids_by_txid
            .iter()
            .map(|(txid, wtxid)| MempoolMemberIdentity {
                txid: *txid,
                wtxid: *wtxid,
            })
            .collect()
    }

    #[cfg(test)]
    pub(in crate::network) fn lifecycle_members_for_test(&self) -> BTreeSet<MempoolMemberIdentity> {
        self.lifecycle_members()
    }
}
