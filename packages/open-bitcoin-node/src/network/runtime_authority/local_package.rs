// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/context.h

use open_bitcoin_core::{
    consensus::{ConsensusParams, ScriptVerifyFlags},
    primitives::Transaction,
};
use open_bitcoin_mempool::{PackageReport, RelayIntent, SubmittedPackageResult};

use super::{ManagedNetworkAuthorityError, ManagedNetworkHandle};

impl<S: crate::ChainstateStore, V: open_bitcoin_core::chainstate::CoinsView>
    ManagedNetworkHandle<S, V>
{
    /// Evaluates a local package without committing mempool or lifecycle state.
    pub fn dry_run_local_package(
        &self,
        transactions: Vec<Transaction>,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        now_unix_seconds: i64,
        relay_intent: RelayIntent,
    ) -> Result<PackageReport, ManagedNetworkAuthorityError> {
        self.try_read(|network| {
            network.dry_run_local_package(
                transactions,
                verify_flags,
                consensus_params,
                now_unix_seconds,
                relay_intent,
            )
        })
    }

    /// Submits a local package through PackageAdmission Local.
    pub fn submit_local_package(
        &self,
        transactions: Vec<Transaction>,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        now_unix_seconds: i64,
        relay_intent: RelayIntent,
    ) -> Result<SubmittedPackageResult, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| {
            network.submit_local_package(
                transactions,
                verify_flags,
                consensus_params,
                now_unix_seconds,
                relay_intent,
            )
        })
    }
}
