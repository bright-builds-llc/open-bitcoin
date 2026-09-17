// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Last/aggregate have-bytes labels retained beside block-relay evidence.
//!
//! Stores last serving labels plus three counters only. This is not a request
//! log and does not keep block hashes or per-peer maps.

use crate::status::{HaveBytesEvidence, ServingStatusLabel};

use super::block_serving::BlockServingPresenceFacts;

/// Sibling have-bytes accumulator for CSOBS last labels and D-12 counters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HaveBytesAccumulator {
    last_serving_status: ServingStatusLabel,
    last_payload_present: bool,
    last_index_known: bool,
    last_validated_on_active_chain: bool,
    available_count: u64,
    unavailable_count: u64,
    index_known_without_payload_count: u64,
}

impl Default for HaveBytesAccumulator {
    fn default() -> Self {
        Self {
            last_serving_status: ServingStatusLabel::Unavailable,
            last_payload_present: false,
            last_index_known: false,
            last_validated_on_active_chain: false,
            available_count: 0,
            unavailable_count: 0,
            index_known_without_payload_count: 0,
        }
    }
}

impl HaveBytesAccumulator {
    /// Records one serve-seam presence decision into last labels and counters.
    pub fn record_have_bytes(&mut self, presence: BlockServingPresenceFacts) {
        self.last_payload_present = presence.payload_present;
        self.last_index_known = presence.index_known;
        self.last_validated_on_active_chain = presence.validated_on_active_chain;
        if !presence.payload_present {
            self.last_serving_status = ServingStatusLabel::Unavailable;
            self.unavailable_count = self.unavailable_count.saturating_add(1);
            if presence.index_known {
                self.index_known_without_payload_count =
                    self.index_known_without_payload_count.saturating_add(1);
            }
            return;
        }
        self.last_serving_status = ServingStatusLabel::Available;
        self.available_count = self.available_count.saturating_add(1);
    }

    /// Copies last labels and the three counters without I/O.
    pub fn snapshot(&self) -> HaveBytesEvidence {
        HaveBytesEvidence {
            last_serving_status: self.last_serving_status,
            last_payload_present: self.last_payload_present,
            last_index_known: self.last_index_known,
            last_validated_on_active_chain: self.last_validated_on_active_chain,
            available_count: self.available_count,
            unavailable_count: self.unavailable_count,
            index_known_without_payload_count: self.index_known_without_payload_count,
        }
    }
}

#[cfg(test)]
mod tests;
