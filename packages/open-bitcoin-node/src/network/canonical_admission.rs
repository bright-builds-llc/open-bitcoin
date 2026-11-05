// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Identity and support checks precede managed admission effects.

use super::{ManagedNetworkError, ManagedNetworkHandle, ManagedPeerNetwork, ManagedResult};
use crate::{ChainstateStore, MemoryChainstateStore};
use open_bitcoin_core::{
    chainstate::{CoinsView, MemoryCoinsView, PruneMode},
    consensus::{CanonicalChain, RootIdentityError, UnsupportedValidationScope},
    primitives::{Block, BlockHeader, NetworkAddress, NetworkMagic},
};
use open_bitcoin_mempool::PolicyConfig;
use open_bitcoin_network::{
    BlockRelayActivationPolicy, HeaderStore, LocalPeerConfig, NetworkError, RelayActivationConfig,
    WireNetworkMessage, advertised_service_flags,
};

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
}

impl<S: ChainstateStore, V: CoinsView> ManagedPeerNetwork<S, V> {
    /// Install an earned genesis header root for the same proved managed owner.
    /// This grants no ordinary-history or network-activation capability.
    pub(crate) fn from_canonical_genesis_initialized(
        managed: crate::ManagedChainstate<S, V>,
        local_config: LocalPeerConfig,
        max_blocks_in_flight: usize,
        proof: &open_bitcoin_core::consensus::chain_spec::CanonicalGenesisProof,
    ) -> ManagedResult<Self> {
        let state = managed.chainstate();
        let params =
            open_bitcoin_core::chainstate::engine::canonical_genesis::consensus_params(proof);
        let magic = match proof.chain() {
            CanonicalChain::Mainnet => NetworkMagic::MAINNET,
            CanonicalChain::Testnet3 => NetworkMagic::from_bytes([0x0b, 0x11, 0x09, 0x07]),
            CanonicalChain::Signet => NetworkMagic::from_bytes([0x0a, 0x03, 0xcf, 0x40]),
            CanonicalChain::Regtest => NetworkMagic::from_bytes([0xfa, 0xbf, 0xb5, 0xda]),
        };
        state.validate_live_origin()?;
        state.bind_live_work_source(params)?;
        if state.active_chain().len() != 1
            || state.tip().is_none_or(|tip| {
                tip.height != 0
                    || tip.block_hash != proof.hash()
                    || &tip.header != proof.header()
                    || tip.chain_work != proof.work()
            })
            || state.coins_best_block()? != Some(proof.hash())
            || local_config.magic != magic
        {
            return Err(ManagedNetworkError::LifecycleEffect(
                "canonical genesis owner binding mismatch",
            ));
        }
        let mut headers = HeaderStore::from_canonical_genesis(proof)?;
        if proof.chain() == CanonicalChain::Regtest {
            // Header lineage alone has no body seal. This adapter holds the full proved root.
            headers.check_body_eligibility(
                proof.block(),
                params,
                i64::from(proof.header().time),
            )?;
        }
        let mut network = Self::from_initialized_chainstate(
            managed,
            local_config,
            PolicyConfig::default(),
            max_blocks_in_flight,
            RelayActivationConfig::default(),
            BlockRelayActivationPolicy::default(),
            false,
        );
        network.admission_scope = AdmissionScope::Canonical(proof.chain());
        network.peer_manager.seed_header_store(headers);
        network.maybe_header_consensus_params = Some(params);
        Ok(network)
    }

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
            // Fixture receipts cannot transfer to another immutable root/network owner.
            self.peer_manager.seed_header_store(HeaderStore::default());
            self.maybe_header_consensus_params = None;
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
        self.require_owned_history()?;
        let proof = open_bitcoin_core::consensus::chain_spec::CanonicalGenesisProof::prove(
            chain.spec(),
            &chain.spec().genesis_block(),
        )
        .map_err(|error| CanonicalAdmissionError::from(error).network_error())?;
        // Resolve order from actual parent identities, never supplied heights/scores.
        let mut pending: Vec<_> = headers.entries().cloned().collect();
        let mut ordered = Vec::with_capacity(pending.len());
        let mut known = std::collections::BTreeSet::from([proof.hash()]);
        while !pending.is_empty() {
            let Some(index) = pending.iter().position(|entry| {
                entry.header == *proof.header() || known.contains(&entry.header.previous_block_hash)
            }) else {
                return Err(NetworkError::InvalidHeader {
                    reject_reason: "unlinked-canonical-header-import".to_owned(),
                    maybe_debug_message: None,
                }
                .into());
            };
            let entry = pending.remove(index);
            let hash = open_bitcoin_core::consensus::block_hash(&entry.header);
            if hash != entry.block_hash {
                return Err(NetworkError::InvalidHeader {
                    reject_reason: "unproved-header-identity".to_owned(),
                    maybe_debug_message: None,
                }
                .into());
            }
            known.insert(hash);
            ordered.push(entry);
        }
        let now = super::header_sync::shell_observed_now()?;
        HeaderStore::readmit_canonical_entries(&proof, ordered.clone(), now)?;
        // Retain prior successful clock observations and arrival order on this owner.
        let mut admitted = self.peer_manager.header_store().clone();
        for entry in ordered {
            if entry.header != *proof.header() {
                admitted.insert_canonical_header(entry.header, now)?;
            }
        }
        self.peer_manager.seed_header_store(admitted);
        Ok(())
    }

    /// Lower-level fixture import; cannot overwrite a canonical owner's store.
    pub fn seed_header_store(&mut self, headers: HeaderStore) {
        if self.admission_scope == AdmissionScope::Fixture
            && self.maybe_header_consensus_params.is_none()
        {
            let mut imported = HeaderStore::default();
            for entry in headers.entries() {
                imported.record_position(&open_bitcoin_core::chainstate::ChainPosition {
                    block_hash: entry.block_hash,
                    header: entry.header.clone(),
                    height: entry.height,
                    chain_work: entry.chain_work,
                    median_time_past: 0,
                });
            }
            self.peer_manager.seed_header_store(imported);
            // An import cannot replace the already selected parameter binding.
        }
    }

    /// Re-admits actual fixture headers under explicit parameters, without importing scores.
    /// Canonical support and root checks still run first; this grants no body lineage.
    /// A bound owner receives further headers through ingress, preserving earned tie order.
    pub fn try_admit_fixture_headers(
        &mut self,
        headers: HeaderStore,
        current_time: i64,
        params: open_bitcoin_core::consensus::ConsensusParams,
    ) -> ManagedResult<()> {
        let candidates: Vec<_> = headers
            .entries()
            .map(|entry| entry.header.clone())
            .collect();
        self.guard_header_batch(&candidates)?;
        if self.admission_scope != AdmissionScope::Fixture {
            self.admission_scope
                .require_history()
                .map_err(|error| error.network_error())?;
            return Err(ManagedNetworkError::LifecycleEffect(
                "canonical owner refuses fixture admission",
            ));
        }
        if self
            .maybe_header_consensus_params
            .is_some_and(|selected| selected != params)
        {
            return Err(NetworkError::InvalidHeader {
                reject_reason: "header-consensus-params-mismatch".to_owned(),
                maybe_debug_message: None,
            }
            .into());
        }
        if self.maybe_header_consensus_params.is_some() {
            return Err(NetworkError::InvalidHeader {
                reject_reason: "header-owner-already-bound".to_owned(),
                maybe_debug_message: None,
            }
            .into());
        }
        let admitted = super::header_sync::readmit_fixture_headers(&headers, current_time, params)?;
        if let Some(root) = self
            .peer_manager
            .header_store()
            .best_chain_entries()
            .first()
            && admitted
                .best_chain_entries()
                .first()
                .is_none_or(|candidate| candidate.block_hash != root.block_hash)
        {
            return Err(NetworkError::InvalidHeader {
                reject_reason: "header-root-mismatch".to_owned(),
                maybe_debug_message: None,
            }
            .into());
        }
        let identities = admitted
            .entries()
            .map(|entry| {
                crate::storage::validation_history::BlockValidationIdentity::new(
                    entry.block_hash,
                    entry.header.previous_block_hash,
                    entry.height,
                )
                .map_err(|_| NetworkError::InvalidHeader {
                    reject_reason: "invalid-validation-identity".to_owned(),
                    maybe_debug_message: None,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        self.peer_manager.seed_header_store(admitted);
        self.maybe_header_consensus_params = Some(params);
        self.publish_admitted_header_identities(identities)
    }

    pub(super) fn require_owned_history(&self) -> ManagedResult<()> {
        if self.maybe_canonical_chain() == Some(CanonicalChain::Regtest)
            && self.chainstate.chainstate().maybe_canonical_chain() == Some(CanonicalChain::Regtest)
            && self.peer_manager.header_store().maybe_canonical_chain()
                == Some(CanonicalChain::Regtest)
        {
            return Ok(());
        }
        self.admission_scope
            .require_history()
            .map_err(|error| error.network_error().into())
    }

    pub(super) fn guard_body(&mut self, block: &Block) -> ManagedResult<()> {
        if self.admission_scope == AdmissionScope::Fixture {
            return Ok(());
        }
        if let Some(chain) = self.maybe_canonical_chain()
            && block.header.previous_block_hash.to_byte_array() == [0; 32]
        {
            chain
                .spec()
                .check_genesis_block(block)
                .map_err(|error| CanonicalAdmissionError::from(error).network_error())?;
        }
        self.require_owned_history()?;
        self.admit_canonical_body_context(block)
    }

    pub(super) fn guard_header_batch(&self, headers: &[BlockHeader]) -> ManagedResult<()> {
        for header in headers {
            if let Some(chain) = self.maybe_canonical_chain()
                && header.previous_block_hash.to_byte_array() == [0; 32]
            {
                chain
                    .spec()
                    .check_genesis_header(header)
                    .map_err(|error| CanonicalAdmissionError::from(error).network_error())?;
            }
        }
        self.require_owned_history()
    }

    pub(super) fn guard_message_admission(
        &mut self,
        message: &WireNetworkMessage,
    ) -> ManagedResult<()> {
        match message {
            WireNetworkMessage::Headers(message) => self.guard_header_batch(&message.headers),
            WireNetworkMessage::Block(block) => self.guard_body(block),
            WireNetworkMessage::CompactBlock(payload) => {
                self.guard_header_batch(std::slice::from_ref(&payload.header))
            }
            WireNetworkMessage::BlockTxn(_) => self.require_owned_history(),
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

impl ManagedNetworkHandle<MemoryChainstateStore, MemoryCoinsView> {
    /// Creates an unbound transient authority for lower-level fixtures.
    /// Canonical binding and supported validation are required for ordinary use.
    pub fn transient_runtime(
        magic: NetworkMagic,
        port: u16,
        relay_activation: RelayActivationConfig,
        block_relay_activation: BlockRelayActivationPolicy,
        inbound_enabled: bool,
    ) -> Self {
        let local_config = LocalPeerConfig {
            magic,
            services: advertised_service_flags(PruneMode::Disabled),
            address: NetworkAddress {
                services: 0,
                address_bytes: [0_u8; 16],
                port,
            },
            nonce: 0,
            relay: true,
            user_agent: "/open-bitcoin:0.1.0/".to_string(),
        };
        let network = ManagedPeerNetwork::new_with_block_relay_activation(
            MemoryChainstateStore::default(),
            local_config,
            PolicyConfig::default(),
            relay_activation,
            block_relay_activation,
            inbound_enabled,
        );
        Self::new(network)
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod canonical_genesis_tests {
    use super::*;
    use crate::{FjallNodeStore, ManagedChainstate, chainstate::BasicFilterStartupMode};

    fn source() -> std::path::PathBuf {
        let mut nonce = [0_u8; 16];
        getrandom::fill(&mut nonce).expect("fixture nonce");
        std::fs::canonicalize(std::env::temp_dir())
            .expect("canonical parent")
            .join(format!(
                "phase166-network-owner-{:x}",
                u128::from_le_bytes(nonce)
            ))
    }

    #[test]
    fn phase166_runtime_network_constructor_keeps_all_activation_flags_disabled() {
        for (chain, selector) in [
            (CanonicalChain::Mainnet, crate::sync::SyncNetwork::Mainnet),
            (CanonicalChain::Testnet3, crate::sync::SyncNetwork::Testnet),
            (CanonicalChain::Signet, crate::sync::SyncNetwork::Signet),
            (CanonicalChain::Regtest, crate::sync::SyncNetwork::Regtest),
        ] {
            // Arrange
            let source = source();
            let (store, recovered) = FjallNodeStore::open_canonical_genesis(&source, chain)
                .expect("proved")
                .into_parts()
                .expect("token");
            let (managed, proof) = ManagedChainstate::from_canonical_genesis(
                store,
                recovered,
                BasicFilterStartupMode::Disabled,
            )
            .expect("manager");
            let config = LocalPeerConfig {
                magic: selector.magic(),
                ..Default::default()
            };
            // Act
            let network =
                ManagedPeerNetwork::from_canonical_genesis_initialized(managed, config, 16, &proof)
                    .expect("earned network owner");
            // Assert
            assert_eq!(network.relay_activation, RelayActivationConfig::default());
            assert_eq!(
                network.block_relay_activation,
                BlockRelayActivationPolicy::default()
            );
            assert!(!network.inbound_serving_enabled);
            assert_eq!(network.maybe_canonical_chain(), Some(chain));
            assert_eq!(network.best_chain_entries().len(), 1);
            drop(network);
            std::fs::remove_dir_all(source).expect("cleanup");
        }
    }

    #[test]
    fn phase166_runtime_network_constructor_rejects_foreign_proof_or_magic() {
        for foreign_proof in [false, true] {
            // Arrange
            let source = source();
            let (store, recovered) =
                FjallNodeStore::open_canonical_genesis(&source, CanonicalChain::Mainnet)
                    .expect("proved")
                    .into_parts()
                    .expect("token");
            let (managed, proof) = ManagedChainstate::from_canonical_genesis(
                store,
                recovered,
                BasicFilterStartupMode::Disabled,
            )
            .expect("manager");
            let other = open_bitcoin_core::consensus::chain_spec::CanonicalGenesisProof::prove(
                CanonicalChain::Testnet3.spec(),
                &CanonicalChain::Testnet3.spec().genesis_block(),
            )
            .expect("other proof");
            let config = LocalPeerConfig {
                magic: if foreign_proof {
                    NetworkMagic::MAINNET
                } else {
                    NetworkMagic::from_bytes([0xfa, 0xbf, 0xb5, 0xda])
                },
                ..Default::default()
            };
            // Act
            let result = ManagedPeerNetwork::from_canonical_genesis_initialized(
                managed,
                config,
                16,
                if foreign_proof { &other } else { &proof },
            );
            // Assert
            assert!(result.is_err());
            drop(result);
            std::fs::remove_dir_all(source).expect("cleanup");
        }
    }

    #[test]
    fn phase166_runtime_mechanical_append_proof_cannot_promote_raw_chainstate() {
        use open_bitcoin_core::chainstate::{Chainstate, CoinsCache};
        // Arrange
        let source = source();
        let (store, recovered) =
            FjallNodeStore::open_canonical_genesis(&source, CanonicalChain::Mainnet)
                .expect("proved")
                .into_parts()
                .expect("token");
        let (genuine, proof) = ManagedChainstate::from_canonical_genesis(
            store,
            recovered,
            BasicFilterStartupMode::Enabled,
        )
        .expect("genuine manager");
        let store = genuine.store().inner().clone();
        assert!(
            store
                .maybe_basic_filter_append_proof()
                .expect("mechanical append proof")
                .is_some()
        );
        let raw = Chainstate::from_coins_cache(
            CoinsCache::from_parent(store.coins_view()),
            genuine.chainstate().active_chain().to_vec(),
            Default::default(),
            None,
        );
        let reconstructed = ManagedChainstate::from_recovered_chainstate(
            crate::FjallChainstateStore::from_store(store),
            raw,
            genuine.flush_lifecycle.clone(),
        )
        .expect("mechanical recovered fixture");
        // Act
        let result = ManagedPeerNetwork::from_canonical_genesis_initialized(
            reconstructed,
            LocalPeerConfig {
                magic: NetworkMagic::MAINNET,
                ..Default::default()
            },
            16,
            &proof,
        );
        // Assert
        assert!(
            result.is_err(),
            "index ownership alone cannot earn core genesis authority"
        );
        drop(result);
        drop(genuine);
        std::fs::remove_dir_all(source).expect("cleanup");
    }
}
