// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_core::chainstate::CoinsView;
use open_bitcoin_core::{
    consensus::{ConsensusParams, ScriptVerifyFlags},
    primitives::Transaction,
};
use open_bitcoin_mempool::{
    AdmissionContext, DryRunPackageCommand, MempoolError, PackageReport, PolicyTime, RelayIntent,
    SubmissionPackage, SubmitPackageCommand, SubmittedPackageResult, WellFormedPackage,
};

use super::{ManagedNetworkError, ManagedPeerNetwork, lifecycle_admission_error};
use crate::ChainstateStore;
use crate::network::lifecycle_projection::{
    AdmissionProjectionSource, LifecycleCommand, LifecycleProjectionPlan,
};
use crate::network::runtime_authority::{LifecycleCommandResult, apply_lifecycle_command};

impl<S: ChainstateStore, V: CoinsView> ManagedPeerNetwork<S, V> {
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
            &self.chainstate_snapshot()?,
            verify_flags,
            consensus_params,
        )?;
        Ok(result.report)
    }

    /// Submits a local package through `SubmitPackageCommand` and PackageAdmission Local.
    pub fn submit_local_package(
        &mut self,
        transactions: Vec<Transaction>,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        now_unix_seconds: i64,
        relay_intent: RelayIntent,
    ) -> Result<SubmittedPackageResult, ManagedNetworkError> {
        let package = WellFormedPackage::try_from(transactions)?;
        let chainstate = self.chainstate_snapshot()?;
        let submission = SubmissionPackage::try_from_package(package, &chainstate)?;
        let prepared = self.mempool.prepare_package(
            SubmitPackageCommand {
                package: submission,
                context: AdmissionContext::local(
                    PolicyTime::from_unix_seconds(now_unix_seconds),
                    relay_intent,
                ),
            },
            &chainstate,
            verify_flags,
            consensus_params,
        )?;
        let report = prepared
            .facts()
            .maybe_package_report()
            .cloned()
            .ok_or_else(|| MempoolError::InternalInvariant {
                reason: "local package preparation omitted its report".to_string(),
            })?;
        let plan = LifecycleProjectionPlan::prepare_admission(
            self,
            self.authority_epoch(),
            prepared,
            AdmissionProjectionSource::Local,
        )
        .map_err(lifecycle_admission_error)?;
        let LifecycleCommandResult::Lifecycle(delta) =
            apply_lifecycle_command(self, LifecycleCommand::PackageAdmission(plan))
                .map_err(lifecycle_admission_error)?
        else {
            return Err(MempoolError::InternalInvariant {
                reason: "local package dispatcher returned a non-lifecycle result".to_string(),
            }
            .into());
        };
        Ok(SubmittedPackageResult { report, delta })
    }
}
