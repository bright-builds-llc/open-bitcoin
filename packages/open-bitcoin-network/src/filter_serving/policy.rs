// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/chain.cpp

//! Ordered semantic validation precedes local ancestry admission and backend lookup.

use crate::{HeaderStore, ServiceFlags};
use open_bitcoin_consensus::chain_proof::proof_equivalent_time;
use open_bitcoin_primitives::BlockHash;

/// Raw successfully extracted request fields; malformed extraction is handled upstream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterRequest {
    Filters {
        filter_type: u8,
        start_height: u32,
        stop_hash: BlockHash,
    },
    Headers {
        filter_type: u8,
        start_height: u32,
        stop_hash: BlockHash,
    },
    Checkpoints {
        filter_type: u8,
        stop_hash: BlockHash,
    },
}

impl FilterRequest {
    /// Raw type stays untrusted until the BASIC capability gate.
    pub const fn filter_type(self) -> u8 {
        match self {
            Self::Filters { filter_type, .. }
            | Self::Headers { filter_type, .. }
            | Self::Checkpoints { filter_type, .. } => filter_type,
        }
    }
    /// Requested branch endpoint, before authorization.
    pub const fn stop_hash(self) -> BlockHash {
        match self {
            Self::Filters { stop_hash, .. }
            | Self::Headers { stop_hash, .. }
            | Self::Checkpoints { stop_hash, .. } => stop_hash,
        }
    }
    fn fields(self) -> (u8, u32, u32) {
        match self {
            Self::Filters {
                filter_type,
                start_height,
                ..
            } => (filter_type, start_height, 1000),
            Self::Headers {
                filter_type,
                start_height,
                ..
            } => (filter_type, start_height, 2000),
            Self::Checkpoints { filter_type, .. } => (filter_type, 0, u32::MAX),
        }
    }
}

/// Facts captured by the adapter, without implying any block-body availability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopProvenance {
    Active,
    ScriptsValid,
    Insufficient,
}

/// Pinned semantic failures require peer disconnect, before any local resource check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterRequestInvalid {
    UnsupportedTypeOrCapability,
    UnknownOrDisallowedStop,
    ReversedRange,
    TooManyEntries,
}

/// Local finite work failure never claims the peer violated a protocol height cap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchResourceRefusal {
    MetadataWork,
    AncestryWork,
    IdentityBytes,
    UnavailableAncestry,
}

/// Explicit local ceilings supplied by the serving resource owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BranchWorkLimit {
    pub ancestry_steps: u64,
    pub identity_bytes: u64,
}

/// Exact persisted-record identity on the requested stop branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FilterBranchIdentity {
    pub hash: BlockHash,
    pub parent: BlockHash,
    pub height: u32,
}

/// Semantically valid fields; construct only through the ordered validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidatedFilterRequest {
    request: FilterRequest,
    stop: FilterBranchIdentity,
}

/// Complete forward identities and the cfheaders predecessor, with measured ancestry work.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedFilterBranch {
    pub request: FilterRequest,
    pub stop: FilterBranchIdentity,
    pub identities: Vec<FilterBranchIdentity>,
    pub maybe_predecessor: Option<FilterBranchIdentity>,
    pub ancestry_steps: u64,
    pub identity_bytes: u64,
}

/// Validate fixed-cost pinned semantics, before reservation and backend availability.
pub fn validate_filter_request(
    request: FilterRequest,
    own_services: ServiceFlags,
    headers: &HeaderStore,
    provenance: StopProvenance,
    target_spacing_seconds: i64,
) -> Result<ValidatedFilterRequest, FilterRequestInvalid> {
    let (filter_type, start, maximum) = request.fields();
    if filter_type != 0 || !own_services.contains(ServiceFlags::COMPACT_FILTERS) {
        return Err(FilterRequestInvalid::UnsupportedTypeOrCapability);
    }
    let Some(stop) = headers.entry(&request.stop_hash()) else {
        return Err(FilterRequestInvalid::UnknownOrDisallowedStop);
    };
    let allowed = match provenance {
        StopProvenance::Active => true,
        StopProvenance::ScriptsValid => {
            stale_allowed(headers, stop.block_hash, target_spacing_seconds)
        }
        StopProvenance::Insufficient => false,
    };
    if !allowed {
        return Err(FilterRequestInvalid::UnknownOrDisallowedStop);
    }
    if start > stop.height {
        return Err(FilterRequestInvalid::ReversedRange);
    }
    if !matches!(request, FilterRequest::Checkpoints { .. }) && stop.height - start >= maximum {
        return Err(FilterRequestInvalid::TooManyEntries);
    }
    Ok(ValidatedFilterRequest {
        request,
        stop: FilterBranchIdentity {
            hash: stop.block_hash,
            parent: stop.header.previous_block_hash,
            height: stop.height,
        },
    })
}

fn stale_allowed(headers: &HeaderStore, stop: BlockHash, spacing: i64) -> bool {
    let Some(best) = headers.maybe_best_proof_header() else {
        return false;
    };
    let Some(stop) = headers.maybe_proof_facts(stop) else {
        return false;
    };
    i64::from(best.timestamp) - i64::from(stop.timestamp) < 2_592_000
        && proof_equivalent_time(best.cumulative, stop.cumulative, best.block_proof, spacing)
            .is_ok_and(|age| age < 2_592_000)
}

/// Reserve work and identity bytes before walking the stop's own decreasing-height ancestry.
pub fn resolve_filter_branch(
    validated: ValidatedFilterRequest,
    headers: &HeaderStore,
    limit: BranchWorkLimit,
) -> Result<AuthorizedFilterBranch, BranchResourceRefusal> {
    let ValidatedFilterRequest { request, stop } = validated;
    let (_, start, _) = request.fields();
    let checkpoints = matches!(request, FilterRequest::Checkpoints { .. });
    let predecessor = matches!(request, FilterRequest::Headers { .. }) && start > 0;
    let count = if checkpoints {
        u64::from(stop.height / 1000)
    } else {
        u64::from(stop.height - start) + 1
    };
    let lowest = if checkpoints {
        1000
    } else if predecessor {
        start - 1
    } else {
        start
    };
    let ancestry_steps = if count == 0 {
        0
    } else {
        u64::from(stop.height - lowest) + 1
    };
    let identity_bytes = (count + u64::from(predecessor) + 1) * 68;
    if ancestry_steps > limit.ancestry_steps {
        return Err(BranchResourceRefusal::AncestryWork);
    }
    if identity_bytes > limit.identity_bytes {
        return Err(BranchResourceRefusal::IdentityBytes);
    }
    let capacity = usize::try_from(count).map_err(|_| BranchResourceRefusal::IdentityBytes)?;
    let mut identities = Vec::with_capacity(capacity);
    let mut maybe_predecessor = None;
    let mut current_hash = stop.hash;
    let mut expected_height = stop.height;
    for _ in 0..ancestry_steps {
        let Some(entry) = headers.entry(&current_hash) else {
            return Err(BranchResourceRefusal::UnavailableAncestry);
        };
        let current = FilterBranchIdentity {
            hash: entry.block_hash,
            parent: entry.header.previous_block_hash,
            height: entry.height,
        };
        if current.height != expected_height || (current_hash == stop.hash && current != stop) {
            return Err(BranchResourceRefusal::UnavailableAncestry);
        }
        if predecessor && current.height == lowest {
            maybe_predecessor = Some(current);
        } else if !checkpoints || current.height.is_multiple_of(1000) {
            identities.push(current);
        }
        if current.height == lowest {
            break;
        }
        current_hash = current.parent;
        expected_height = current
            .height
            .checked_sub(1)
            .ok_or(BranchResourceRefusal::UnavailableAncestry)?;
    }
    identities.reverse();
    Ok(AuthorizedFilterBranch {
        request,
        stop,
        identities,
        maybe_predecessor,
        ancestry_steps,
        identity_bytes,
    })
}

#[cfg(test)]
mod tests;
