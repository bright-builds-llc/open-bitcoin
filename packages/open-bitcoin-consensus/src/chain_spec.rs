// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Canonical identity facts are separate from ordinary validation authority.

use core::fmt;
use open_bitcoin_primitives::{Block, BlockHash, BlockHeader};

pub mod bootstrap;
mod genesis;
mod rules;
pub use bootstrap::CanonicalGenesisProof;
pub use rules::{NetworkRules, ScriptFlagException};

/// The four pinned profiles already exposed by Open Bitcoin's selectors.
/// Signet denotes the default challenge only; custom Signets are outside this contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalChain {
    Mainnet,
    Testnet3,
    Signet,
    Regtest,
}

impl CanonicalChain {
    /// Select immutable identity and validation-support facts.
    pub const fn spec(self) -> ChainSpec {
        ChainSpec { chain: self }
    }

    /// Canonical identity spelling; operator adapters retain their existing `testnet` selector.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Mainnet => "mainnet",
            Self::Testnet3 => "testnet3",
            Self::Signet => "signet",
            Self::Regtest => "regtest",
        }
    }
}

impl fmt::Display for CanonicalChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Selected canonical facts, with no caller-provided validation capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChainSpec {
    chain: CanonicalChain,
}

impl ChainSpec {
    /// Canonical profile selected by this specification.
    pub const fn chain(&self) -> CanonicalChain {
        self.chain
    }

    /// Exact pinned root header; this is an identity fact, not validated history.
    pub fn genesis_header(&self) -> BlockHeader {
        genesis::header(self.chain)
    }

    /// Exact pinned body; genesis outputs are excluded from the UTXO set by special handling.
    pub fn genesis_block(&self) -> Block {
        genesis::block(self.chain)
    }

    /// Bitcoin block hash of the pinned header, in the existing typed raw-byte order.
    pub fn genesis_hash(&self) -> BlockHash {
        crate::block_hash(&self.genesis_header())
    }

    /// Compare every field, rather than accepting a shared merkle root or valid PoW.
    pub fn check_genesis_header(&self, candidate: &BlockHeader) -> Result<(), RootIdentityError> {
        if candidate != &self.genesis_header() {
            return Err(RootIdentityError::NonCanonicalHeader);
        }
        Ok(())
    }

    /// Compare the entire canonical body, including witness and transaction metadata.
    pub fn check_genesis_block(&self, candidate: &Block) -> Result<(), RootIdentityError> {
        self.check_genesis_header(&candidate.header)?;
        if candidate != &self.genesis_block() {
            return Err(RootIdentityError::NonCanonicalBody);
        }
        Ok(())
    }

    /// Pinned default Signet profile only. Genesis alone cannot prove this challenge,
    /// and providing the fact does not implement Signet solution validation.
    pub const fn maybe_default_signet_challenge(&self) -> Option<&'static [u8]> {
        match self.chain {
            CanonicalChain::Signet => Some(&genesis::DEFAULT_SIGNET_CHALLENGE),
            CanonicalChain::Mainnet | CanonicalChain::Testnet3 | CanonicalChain::Regtest => None,
        }
    }

    /// Selected identity and qualification facts, separate from local implementation.
    pub const fn validation_scope(&self) -> ValidationScope {
        ValidationScope { chain: self.chain }
    }

    /// Refuse ordinary history until independently verified prerequisites exist.
    pub const fn require_ordinary_history(&self) -> Result<(), UnsupportedValidationScope> {
        Err(UnsupportedValidationScope {
            scope: self.validation_scope(),
        })
    }
}

/// Selected identity alone proves no ordinary-history qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationEvidenceClass {
    IdentityOnly,
}

/// Local implementation is not an admission capability or differential qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalOrdinaryImplementation {
    Unavailable,
    BoundedDefaultRegtest,
}

impl LocalOrdinaryImplementation {
    /// The retained-body replay and safe reopen bound exists only for default Regtest.
    pub const fn retained_body_reopen_bound(self) -> bool {
        matches!(self, Self::BoundedDefaultRegtest)
    }

    /// Stable status spelling that does not imply network activation.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Unavailable => "unavailable",
            Self::BoundedDefaultRegtest => "bounded_default_regtest",
        }
    }
}

/// Implementation work still missing outside the bounded default-Regtest path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissingImplementationPrerequisite {
    HistoricalRules,
    CheckpointContext,
    SignetChallengeValidation,
}

impl MissingImplementationPrerequisite {
    /// Stable bounded code for operator support diagnostics.
    pub const fn code(self) -> &'static str {
        match self {
            Self::HistoricalRules => "historical_rules",
            Self::CheckpointContext => "checkpoint_context",
            Self::SignetChallengeValidation => "signet_challenge_validation",
        }
    }
}

/// Required checks that are not yet implemented and qualified for ordinary history.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissingValidationPrerequisite {
    CanonicalBootstrap,
    ExactWork,
    CandidateHistory,
    SignetChallengeValidation,
}

impl MissingValidationPrerequisite {
    /// Stable bounded code for adapter diagnostics.
    pub const fn code(self) -> &'static str {
        match self {
            Self::CanonicalBootstrap => "canonical_bootstrap",
            Self::ExactWork => "exact_work",
            Self::CandidateHistory => "candidate_history",
            Self::SignetChallengeValidation => "signet_challenge_validation",
        }
    }
}

impl fmt::Display for MissingValidationPrerequisite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

const MISSING_HISTORY: [MissingValidationPrerequisite; 3] = [
    MissingValidationPrerequisite::CanonicalBootstrap,
    MissingValidationPrerequisite::ExactWork,
    MissingValidationPrerequisite::CandidateHistory,
];
const MISSING_SIGNET_HISTORY: [MissingValidationPrerequisite; 4] = [
    MissingValidationPrerequisite::CanonicalBootstrap,
    MissingValidationPrerequisite::ExactWork,
    MissingValidationPrerequisite::CandidateHistory,
    MissingValidationPrerequisite::SignetChallengeValidation,
];
const MISSING_CHECKPOINT_IMPLEMENTATION: [MissingImplementationPrerequisite; 2] = [
    MissingImplementationPrerequisite::HistoricalRules,
    MissingImplementationPrerequisite::CheckpointContext,
];
const MISSING_SIGNET_IMPLEMENTATION: [MissingImplementationPrerequisite; 2] = [
    MissingImplementationPrerequisite::HistoricalRules,
    MissingImplementationPrerequisite::SignetChallengeValidation,
];
const NO_MISSING_IMPLEMENTATION: [MissingImplementationPrerequisite; 0] = [];

/// Read-only support facts derived solely from the selected canonical profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidationScope {
    chain: CanonicalChain,
}

impl ValidationScope {
    /// Canonical profile to which these support facts apply.
    pub const fn chain(&self) -> CanonicalChain {
        self.chain
    }

    /// Selected identity does not make local implementation a canonical owner.
    pub const fn evidence_class(&self) -> ValidationEvidenceClass {
        ValidationEvidenceClass::IdentityOnly
    }

    /// Whether ordinary history is qualified; currently false for every profile.
    pub const fn ordinary_history(&self) -> bool {
        false
    }

    /// Bounded local implementation facts; a sealed runtime remains required for acceptance.
    pub const fn local_ordinary_implementation(&self) -> LocalOrdinaryImplementation {
        match self.chain {
            CanonicalChain::Regtest => LocalOrdinaryImplementation::BoundedDefaultRegtest,
            CanonicalChain::Mainnet | CanonicalChain::Testnet3 | CanonicalChain::Signet => {
                LocalOrdinaryImplementation::Unavailable
            }
        }
    }

    /// Missing implementation work is distinct from generic qualification prerequisites.
    pub const fn missing_implementation_prerequisites(
        &self,
    ) -> &'static [MissingImplementationPrerequisite] {
        match self.chain {
            CanonicalChain::Mainnet | CanonicalChain::Testnet3 => {
                &MISSING_CHECKPOINT_IMPLEMENTATION
            }
            CanonicalChain::Signet => &MISSING_SIGNET_IMPLEMENTATION,
            CanonicalChain::Regtest => &NO_MISSING_IMPLEMENTATION,
        }
    }

    /// No actual pinned-daemon differential qualification has been performed.
    pub const fn actual_daemon_differential_qualified(&self) -> bool {
        false
    }

    /// Immutable prerequisites, including absent challenge validation for default Signet.
    pub const fn missing_prerequisites(&self) -> &'static [MissingValidationPrerequisite] {
        match self.chain {
            CanonicalChain::Mainnet | CanonicalChain::Testnet3 | CanonicalChain::Regtest => {
                &MISSING_HISTORY
            }
            CanonicalChain::Signet => &MISSING_SIGNET_HISTORY,
        }
    }
}

/// Bounded typed refusal; it never includes candidate-controlled bytes or strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnsupportedValidationScope {
    scope: ValidationScope,
}

impl UnsupportedValidationScope {
    /// Complete immutable support facts that caused the refusal.
    pub const fn scope(&self) -> ValidationScope {
        self.scope
    }

    /// Refused canonical profile.
    pub const fn chain(&self) -> CanonicalChain {
        self.scope.chain()
    }

    /// Typed reasons ordinary history remains unsupported.
    pub const fn missing_prerequisites(&self) -> &'static [MissingValidationPrerequisite] {
        self.scope.missing_prerequisites()
    }
}

impl fmt::Display for UnsupportedValidationScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ordinary history validation is unsupported for {}: ",
            self.chain()
        )?;
        for (index, prerequisite) in self.missing_prerequisites().iter().enumerate() {
            if index != 0 {
                f.write_str(", ")?;
            }
            write!(f, "{prerequisite}")?;
        }
        Ok(())
    }
}

impl std::error::Error for UnsupportedValidationScope {}

/// A candidate differs from the selected profile's exact genesis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootIdentityError {
    NonCanonicalHeader,
    NonCanonicalBody,
}

impl fmt::Display for RootIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::NonCanonicalHeader => "noncanonical genesis header",
            Self::NonCanonicalBody => "noncanonical genesis body",
        })
    }
}

impl std::error::Error for RootIdentityError {}

#[cfg(test)]
mod tests;
