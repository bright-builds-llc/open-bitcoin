// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_core::{
    consensus::{ConsensusParams, ScriptVerifyFlags},
    primitives::Transaction,
};
use open_bitcoin_mempool::{
    AdmissionContext, DryRunPackageCommand, PackageReport, PolicyTime, RelayIntent,
    WellFormedPackage,
};

use super::{ManagedNetworkError, ManagedPeerNetwork};
use crate::ChainstateStore;

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    /// Evaluates a local package through `DryRunPackageCommand` without lifecycle mutation.
    pub fn dry_run_local_package(
        &self,
        transactions: Vec<Transaction>,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        now_unix_seconds: i64,
        relay_intent: RelayIntent,
    ) -> Result<PackageReport, ManagedNetworkError> {
        let package = WellFormedPackage::try_from(transactions)?;
        let result = self.mempool.dry_run_package(
            DryRunPackageCommand {
                package,
                context: AdmissionContext::local(
                    PolicyTime::from_unix_seconds(now_unix_seconds),
                    relay_intent,
                ),
            },
            &self.chainstate_snapshot(),
            verify_flags,
            consensus_params,
        )?;
        Ok(result.report)
    }
}
