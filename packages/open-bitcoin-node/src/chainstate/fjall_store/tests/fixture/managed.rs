// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.h

use super::*;
use crate::network::ManagedPeerNetwork;
use open_bitcoin_network::{
    BlockRelayActivationPolicy, HeaderStore, LocalPeerConfig, RelayActivationConfig,
};

/// Test ownership wrapper, using the same network decision and lower capability path.
pub(crate) struct FixtureManagedNetwork {
    network: ManagedPeerNetwork<FjallChainstateStore, FjallCoinsView>,
}

impl std::ops::Deref for FixtureManagedNetwork {
    type Target = ManagedChainstate<FjallChainstateStore, FjallCoinsView>;
    fn deref(&self) -> &Self::Target {
        self.network.chainstate()
    }
}

impl std::ops::DerefMut for FixtureManagedNetwork {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.network.chainstate_mut()
    }
}

impl FixtureManagedNetwork {
    pub(crate) fn into_parts(self) -> (FjallChainstateStore, Chainstate<FjallCoinsView>) {
        self.network.into_fixture_chainstate().into_parts()
    }
    pub(crate) fn into_network(self) -> ManagedPeerNetwork<FjallChainstateStore, FjallCoinsView> {
        self.network
    }
    pub(crate) fn into_manager(self) -> ManagedChainstate<FjallChainstateStore, FjallCoinsView> {
        self.network.into_fixture_chainstate()
    }
    pub(crate) fn from_live_owner(
        manager: ManagedChainstate<FjallChainstateStore, FjallCoinsView>,
        blocks: &[Block],
        params: open_bitcoin_core::consensus::ConsensusParams,
    ) -> Self {
        manager
            .chainstate()
            .validate_live_origin()
            .expect("actual live source required");
        let mut wrapper = Self::new(manager, blocks, false);
        let mut headers = HeaderStore::default();
        for block in blocks {
            headers
                .insert_header_with_params(block.header.clone(), &params)
                .expect("actual active headers");
        }
        wrapper
            .network
            .try_admit_fixture_headers(headers, 2_000_000_000, params)
            .expect("readmitted history");
        for block in blocks {
            wrapper
                .network
                .retain_checked_candidate(block, params, 2_000_000_000)
                .expect("actual retained active bytes");
        }
        wrapper
    }
    pub(super) fn new(
        manager: ManagedChainstate<FjallChainstateStore, FjallCoinsView>,
        blocks: &[Block],
        connect: bool,
    ) -> Self {
        let mut network = ManagedPeerNetwork::from_initialized_chainstate(
            manager,
            LocalPeerConfig::default(),
            Default::default(),
            16,
            RelayActivationConfig::default(),
            BlockRelayActivationPolicy::default(),
            false,
        );
        if !connect {
            return Self { network };
        }
        let params = ReorgFixture::params();
        let mut headers = HeaderStore::default();
        for block in blocks {
            headers
                .insert_header_with_params(block.header.clone(), &params)
                .expect("actual fixture headers");
        }
        network
            .try_admit_fixture_headers(headers, 2_000_000_000, params)
            .expect("explicit header model");
        network
            .retain_checked_candidate(&blocks[0], params, 2_000_000_000)
            .expect("actual retained root");
        // These actual bodies arrive before the active suffix. No sequence is supplied.
        let first = ReorgFixture::child(&blocks[0], 1, true);
        let second = ReorgFixture::child(&first, 2, true);
        let sibling = ReorgFixture::child(&blocks[1], 2, true);
        for block in [&first, &second, &sibling] {
            network
                .retain_checked_candidate(block, params, 2_000_000_000)
                .expect("checked retained earlier fork bytes");
        }
        for block in &blocks[1..] {
            network
                .connect_local_block(
                    block,
                    open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
                    params,
                )
                .expect("actual active body acceptance after side linkage");
        }
        Self { network }
    }

    pub(crate) fn prepare_reorg(
        &self,
        disconnect: &[Block],
        replacements: &[open_bitcoin_core::chainstate::AnchoredBlock],
        flags: open_bitcoin_core::consensus::ScriptVerifyFlags,
        params: open_bitcoin_core::consensus::ConsensusParams,
    ) -> Result<crate::chainstate::PreparedChainstateReorg, ChainstateError> {
        let authorization = self
            .network
            .authorize_reorg(disconnect, replacements, params)?;
        self.network.chainstate().prepare_authorized_reorg(
            authorization,
            disconnect,
            replacements,
            flags,
            params,
        )
    }

    pub(crate) fn reorg(
        &mut self,
        disconnect: &[Block],
        replacements: &[open_bitcoin_core::chainstate::AnchoredBlock],
        flags: open_bitcoin_core::consensus::ScriptVerifyFlags,
        params: open_bitcoin_core::consensus::ConsensusParams,
    ) -> Result<open_bitcoin_core::chainstate::ChainTransition, ChainstateError> {
        let prepared = self.prepare_reorg(disconnect, replacements, flags, params)?;
        self.network
            .chainstate_mut()
            .commit_prepared_reorg(prepared)
    }
}
