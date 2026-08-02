// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Immutable authority input for one coherent mempool checkpoint capture.

use open_bitcoin_mempool::PolicyTime;

use crate::network::CheckpointTrigger;

pub(in crate::network) struct SnapshotPreparationRequest {
    pub(in crate::network) captured_at: PolicyTime,
    pub(in crate::network) trigger: CheckpointTrigger,
}

impl SnapshotPreparationRequest {
    pub(in crate::network) const fn new(
        captured_at: PolicyTime,
        trigger: CheckpointTrigger,
    ) -> Self {
        Self {
            captured_at,
            trigger,
        }
    }
}
