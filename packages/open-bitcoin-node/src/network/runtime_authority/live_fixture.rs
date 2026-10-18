// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp

//! Explicit fixture transfers under the real runtime mutex; ordinary opens never invoke them.

use super::{ManagedNetworkAuthorityError, ManagedNetworkHandle};
use crate::network::canonical_admission::AdmissionScope;
use open_bitcoin_core::chainstate::{Chainstate, ChainstateError};

impl ManagedNetworkHandle<crate::FjallChainstateStore, crate::storage::coins_view::FjallCoinsView> {
    pub(crate) fn persist_missing_live_fixture_counts(
        &self,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.try_mutate(|network| {
            if !matches!(network.admission_scope, AdmissionScope::Fixture) {
                return Err(ChainstateError::UntrustedWorkLineage.into());
            }
            network.chainstate.persist_missing_live_fixture_counts()?;
            Ok(())
        })
    }

    pub(crate) fn has_earned_fixture_work(&self) -> Result<bool, ManagedNetworkAuthorityError> {
        self.read(|network| network.chainstate.chainstate().has_earned_work())
    }

    pub(crate) fn adopt_live_fixture_owner(
        &self,
        source: Chainstate,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.try_mutate(|network| {
            if !matches!(network.admission_scope, AdmissionScope::Fixture) {
                return Err(ChainstateError::UntrustedWorkLineage.into());
            }
            network.chainstate.adopt_live_fixture_owner(source)?;
            network.install_authority_incarnation();
            Ok(())
        })
    }
}
