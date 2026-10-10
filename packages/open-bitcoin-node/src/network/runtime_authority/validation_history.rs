// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/headerssync.cpp

use super::*;

impl<S: ChainstateStore, V: CoinsView> ManagedNetworkHandle<S, V> {
    pub(crate) fn persist_validation_header_snapshot(
        &self,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.try_read(ManagedPeerNetwork::persist_validation_header_snapshot)
    }
}
