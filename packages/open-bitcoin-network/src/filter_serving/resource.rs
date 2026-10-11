// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net.h
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

//! Checked logical work and retained byte admission; these are not RSS guarantees.

/// Pinned protocol payload ceiling, independently of the codec/storage capacity.
pub const BASIC_PEER_MAX_PAYLOAD_BYTES: u64 = 4_000_000;
/// One retained response suffix, independent of the generic 128-effect queue.
pub const BASIC_PEER_MAX_SUFFIX_BYTES: u64 = 8 * 1024 * 1024;
/// Concurrent response suffixes across peers.
pub const BASIC_PEER_MAX_AGGREGATE_SUFFIX_BYTES: u64 = 32 * 1024 * 1024;
/// At most one affine request per peer and four per runtime.
pub const BASIC_PEER_MAX_REQUESTS: u64 = 1;
pub const BASIC_PEER_MAX_AGGREGATE_REQUESTS: u64 = 4;
/// Transport accounting includes all other pending frames on the connection.
pub const BASIC_PEER_MAX_FRAME_BYTES: u64 = BASIC_PEER_MAX_PAYLOAD_BYTES + 24;
pub const BASIC_PEER_MAX_AGGREGATE_FRAME_BYTES: u64 = 8 * 1024 * 1024;
/// Power-of-two headroom over legal complete range and sparse-checkpoint work.
pub const BASIC_PEER_MAX_ANCESTRY_LINKS: u64 = 8192;
pub const BASIC_PEER_MAX_HEADER_OPERATIONS: u64 = 8192;
/// Two reads per 2000-row range plus a bounded predecessor read.
pub const BASIC_PEER_MAX_RECORD_READS: u64 = 4096;
/// Covers the sealed 128-identity stale-provenance batch plus bookkeeping.
pub const BASIC_PEER_MAX_METADATA_READS: u64 = 256;
/// Two individually codec-capacity envelopes; stored capacity is unchanged.
pub const BASIC_PEER_MAX_VALIDATED_ENVELOPE_BYTES: u64 = 64 * 1024 * 1024;
pub const BASIC_PEER_MAX_HASH_PADDED_BYTES: u64 = 128 * 1024 * 1024;
pub const BASIC_PEER_MAX_LOGICAL_COPY_BYTES: u64 = 64 * 1024 * 1024;

/// Retained capacity is checked independently from cumulative request work.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasicPeerRetentionRefusal {
    PeerRequests,
    AggregateRequests,
    PeerSuffix,
    AggregateSuffix,
    PeerFrame,
    AggregateFrame,
}

/// Reserve the whole suffix as one request before building response collections.
pub fn admit_response_reservation(
    peer_requests: u64,
    aggregate_requests: u64,
    peer_suffix: u64,
    aggregate_suffix: u64,
    suffix: u64,
) -> Result<(), BasicPeerRetentionRefusal> {
    use BasicPeerRetentionRefusal as Refusal;
    capacity(
        peer_requests,
        1,
        BASIC_PEER_MAX_REQUESTS,
        Refusal::PeerRequests,
    )?;
    capacity(
        aggregate_requests,
        1,
        BASIC_PEER_MAX_AGGREGATE_REQUESTS,
        Refusal::AggregateRequests,
    )?;
    capacity(
        peer_suffix,
        suffix,
        BASIC_PEER_MAX_SUFFIX_BYTES,
        Refusal::PeerSuffix,
    )?;
    capacity(
        aggregate_suffix,
        suffix,
        BASIC_PEER_MAX_AGGREGATE_SUFFIX_BYTES,
        Refusal::AggregateSuffix,
    )
}

/// Include every other pending transport byte in the narrow CF frame override.
pub fn admit_active_filter_frame(
    frame: u64,
    peer_pending: u64,
    aggregate_pending: u64,
) -> Result<(), BasicPeerRetentionRefusal> {
    use BasicPeerRetentionRefusal as Refusal;
    capacity(
        peer_pending,
        frame,
        BASIC_PEER_MAX_FRAME_BYTES,
        Refusal::PeerFrame,
    )?;
    capacity(
        aggregate_pending,
        frame,
        BASIC_PEER_MAX_AGGREGATE_FRAME_BYTES,
        Refusal::AggregateFrame,
    )
}

fn capacity(
    used: u64,
    added: u64,
    limit: u64,
    refusal: BasicPeerRetentionRefusal,
) -> Result<(), BasicPeerRetentionRefusal> {
    used.checked_add(added)
        .filter(|value| *value <= limit)
        .map(|_| ())
        .ok_or(refusal)
}

/// Independent counters make hidden predecessor and sparse checkpoint work visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum BasicPeerServeAxis {
    AncestryLinks,
    HeaderOperations,
    RecordReads,
    MetadataReads,
    ValidatedEnvelopeBytes,
    HashPaddedBytes,
    LogicalCopyBytes,
    EncodedSuffixBytes,
}

impl BasicPeerServeAxis {
    pub const ALL: [Self; 8] = [
        Self::AncestryLinks,
        Self::HeaderOperations,
        Self::RecordReads,
        Self::MetadataReads,
        Self::ValidatedEnvelopeBytes,
        Self::HashPaddedBytes,
        Self::LogicalCopyBytes,
        Self::EncodedSuffixBytes,
    ];
}

/// Actual charged logical work; indexed only through the named axis enum.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BasicPeerServeWork([u64; 8]);

impl BasicPeerServeWork {
    pub const fn get(self, axis: BasicPeerServeAxis) -> u64 {
        self.0[axis as usize]
    }
}

/// A local resource refusal never classifies an otherwise valid request as invalid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BasicPeerResourceRefusal(pub BasicPeerServeAxis);

/// Finite per-request policy; limits may be narrowed for operator/test admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BasicPeerServeBudget {
    limits: BasicPeerServeWork,
    work: BasicPeerServeWork,
}

impl Default for BasicPeerServeBudget {
    fn default() -> Self {
        Self {
            // 4096 covers two reads per legal 2000-header range plus predecessor.
            // Byte budgets derive from two codec-capacity 32MiB envelopes; hashing
            // owns padded copies. 8192 gives finite power-of-two ancestry headroom.
            limits: BasicPeerServeWork([
                BASIC_PEER_MAX_ANCESTRY_LINKS,
                BASIC_PEER_MAX_HEADER_OPERATIONS,
                BASIC_PEER_MAX_RECORD_READS,
                BASIC_PEER_MAX_METADATA_READS,
                BASIC_PEER_MAX_VALIDATED_ENVELOPE_BYTES,
                BASIC_PEER_MAX_HASH_PADDED_BYTES,
                BASIC_PEER_MAX_LOGICAL_COPY_BYTES,
                BASIC_PEER_MAX_SUFFIX_BYTES,
            ]),
            work: BasicPeerServeWork::default(),
        }
    }
}

impl BasicPeerServeBudget {
    /// Narrow a counter before beginning work; cannot erase consumed work.
    pub fn with_limit(mut self, axis: BasicPeerServeAxis, limit: u64) -> Self {
        self.limits.0[axis as usize] = limit.min(self.limits.get(axis));
        self
    }
    pub const fn work(&self) -> BasicPeerServeWork {
        self.work
    }
    pub fn remaining(&self, axis: BasicPeerServeAxis) -> u64 {
        self.limits.get(axis).saturating_sub(self.work.get(axis))
    }
    /// Charge before the associated lookup, parse/hash, copy or allocation.
    pub fn charge(
        &mut self,
        axis: BasicPeerServeAxis,
        amount: u64,
    ) -> Result<(), BasicPeerResourceRefusal> {
        let value = self
            .work
            .get(axis)
            .checked_add(amount)
            .filter(|value| *value <= self.limits.get(axis))
            .ok_or(BasicPeerResourceRefusal(axis))?;
        self.work.0[axis as usize] = value;
        Ok(())
    }
}

fn compact_size_bytes(value: u64) -> u64 {
    match value {
        0..=252 => 1,
        253..=65535 => 3,
        65536..=4294967295 => 5,
        _ => 9,
    }
}
/// Full Bitcoin frame, including type, block hash and encoded-filter length.
pub fn cfilter_frame_bytes(encoded: u64) -> Option<u64> {
    encoded
        .checked_add(57)?
        .checked_add(compact_size_bytes(encoded))
}
/// Full cfheaders frame, including previous header and 32-byte filter hashes.
pub fn cfheaders_frame_bytes(count: u64) -> Option<u64> {
    count
        .checked_mul(32)?
        .checked_add(89)?
        .checked_add(compact_size_bytes(count))
}
/// Positive interval checkpoints; no protocol stop-height maximum is imposed.
pub fn cfcheckpt_frame_bytes(stop_height: u32) -> Option<u64> {
    let count = u64::from(stop_height / 1000);
    count
        .checked_mul(32)?
        .checked_add(57)?
        .checked_add(compact_size_bytes(count))
}

#[cfg(test)]
mod tests;
