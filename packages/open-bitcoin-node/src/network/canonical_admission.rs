// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Identity and support checks precede managed admission effects.

use super::{ManagedNetworkError, ManagedNetworkHandle, ManagedPeerNetwork, ManagedResult};
use crate::ChainstateStore;
use open_bitcoin_core::{
    chainstate::CoinsView,
    consensus::{CanonicalChain, RootIdentityError, UnsupportedValidationScope},
    primitives::{Block, BlockHeader},
};
use open_bitcoin_network::{HeaderStore, NetworkError, WireNetworkMessage};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum AdmissionScope {
    Fixture,
    Canonical(CanonicalChain),
}

/// Fixed refusal categories; candidate bytes never enter diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalAdmissionError {
    NonCanonicalHeader,
    NonCanonicalBody,
    UnsupportedValidationScope(UnsupportedValidationScope),
}

impl core::fmt::Display for CanonicalAdmissionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonCanonicalHeader => f.write_str("noncanonical genesis header"),
            Self::NonCanonicalBody => f.write_str("noncanonical genesis body"),
            Self::UnsupportedValidationScope(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for CanonicalAdmissionError {}

impl From<RootIdentityError> for CanonicalAdmissionError {
    fn from(error: RootIdentityError) -> Self {
        match error {
            RootIdentityError::NonCanonicalHeader => Self::NonCanonicalHeader,
            RootIdentityError::NonCanonicalBody => Self::NonCanonicalBody,
        }
    }
}

impl CanonicalAdmissionError {
    /// Stable category for existing network error adapters.
    pub const fn code(self) -> &'static str {
        match self {
            Self::NonCanonicalHeader => "noncanonical-genesis-header",
            Self::NonCanonicalBody => "noncanonical-genesis-body",
            Self::UnsupportedValidationScope(_) => "unsupported-validation-scope",
        }
    }
    pub(super) fn network_error(self) -> NetworkError {
        NetworkError::InvalidHeader {
            reject_reason: self.code().to_owned(),
            maybe_debug_message: Some(self.to_string()),
        }
    }
}

impl AdmissionScope {
    pub(super) fn require_history(self) -> Result<(), CanonicalAdmissionError> {
        match self {
            Self::Fixture => Ok(()),
            Self::Canonical(chain) => chain
                .spec()
                .require_ordinary_history()
                .map_err(CanonicalAdmissionError::UnsupportedValidationScope),
        }
    }

    pub(super) fn check_header(self, header: &BlockHeader) -> Result<(), CanonicalAdmissionError> {
        if let Self::Canonical(chain) = self
            && header.previous_block_hash.to_byte_array() == [0; 32]
        {
            chain.spec().check_genesis_header(header)?;
        }
        self.require_history()
    }

    pub(super) fn check_body(self, block: &Block) -> Result<(), CanonicalAdmissionError> {
        if let Self::Canonical(chain) = self
            && block.header.previous_block_hash.to_byte_array() == [0; 32]
        {
            chain.spec().check_genesis_block(block)?;
        }
        self.require_history()
    }
}

impl<S: ChainstateStore, V: CoinsView> ManagedPeerNetwork<S, V> {
    pub(super) const fn maybe_canonical_chain(&self) -> Option<CanonicalChain> {
        match self.admission_scope {
            AdmissionScope::Fixture => None,
            AdmissionScope::Canonical(chain) => Some(chain),
        }
    }
    /// Binds a lower-level fixture owner once. A bound owner's selection is immutable.
    pub fn into_canonical_scope(mut self, chain: CanonicalChain) -> Self {
        if self.admission_scope == AdmissionScope::Fixture {
            self.admission_scope = AdmissionScope::Canonical(chain);
        }
        self
    }

    /// Imports only after canonical root recognition and ordinary support qualification.
    pub fn try_seed_canonical_headers(&mut self, headers: HeaderStore) -> ManagedResult<()> {
        let AdmissionScope::Canonical(chain) = self.admission_scope else {
            return Err(ManagedNetworkError::LifecycleEffect(
                "canonical scope is required",
            ));
        };
        for entry in headers.entries() {
            if entry.header.previous_block_hash.to_byte_array() == [0; 32] {
                chain
                    .spec()
                    .check_genesis_header(&entry.header)
                    .map_err(|error| CanonicalAdmissionError::from(error).network_error())?;
            }
        }
        self.admission_scope
            .require_history()
            .map_err(|error| error.network_error())?;
        self.peer_manager.seed_header_store(headers);
        Ok(())
    }

    /// Lower-level fixture import; cannot overwrite a canonical owner's store.
    pub fn seed_header_store(&mut self, headers: HeaderStore) {
        if self.admission_scope == AdmissionScope::Fixture {
            self.peer_manager.seed_header_store(headers);
        }
    }

    pub(super) fn guard_body(&self, block: &Block) -> ManagedResult<()> {
        self.admission_scope
            .check_body(block)
            .map_err(|error| error.network_error().into())
    }

    pub(super) fn guard_header_batch(&self, headers: &[BlockHeader]) -> ManagedResult<()> {
        for header in headers {
            self.admission_scope
                .check_header(header)
                .map_err(|error| error.network_error())?;
        }
        self.admission_scope
            .require_history()
            .map_err(|error| error.network_error().into())
    }

    pub(super) fn guard_message_admission(
        &self,
        message: &WireNetworkMessage,
    ) -> ManagedResult<()> {
        match message {
            WireNetworkMessage::Headers(message) => self.guard_header_batch(&message.headers),
            WireNetworkMessage::Block(block) => self.guard_body(block),
            WireNetworkMessage::CompactBlock(payload) => self
                .admission_scope
                .check_header(&payload.header)
                .map_err(|error| error.network_error().into()),
            WireNetworkMessage::BlockTxn(_) => self
                .admission_scope
                .require_history()
                .map_err(|error| error.network_error().into()),
            _ => Ok(()),
        }
    }
}

impl<S: ChainstateStore, V: CoinsView> ManagedNetworkHandle<S, V> {
    /// Lower-level fixture wrapping preserves any existing canonical binding.
    pub fn from_network_fixture(network: ManagedPeerNetwork<S, V>) -> Self {
        Self::new(network)
    }

    /// Wraps an owner with an immutable canonical binding, without fixture routing.
    pub fn from_canonical_network(
        network: ManagedPeerNetwork<S, V>,
        chain: CanonicalChain,
    ) -> Self {
        Self::new(network.into_canonical_scope(chain))
    }
}

#[cfg(test)]
mod tests;
