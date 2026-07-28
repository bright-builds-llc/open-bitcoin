// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Typed singleton admission facade for local and peer transaction sources.

use open_bitcoin_core::{
    consensus::{ConsensusParams, ScriptVerifyFlags, transaction_wtxid},
    primitives::Transaction,
};
use open_bitcoin_mempool::{AdmissionContext, MempoolError, MempoolOutcome, MempoolTransition};

use super::lifecycle_admission_error;
use crate::network::lifecycle_projection::{
    AdmissionProjectionSource, LifecycleCommand, LifecycleProjectionPlan,
};
use crate::network::runtime_authority::{LifecycleCommandResult, apply_lifecycle_command};
use crate::{ChainstateStore, ManagedNetworkError, ManagedPeerNetwork};

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub(super) fn submit_singleton_transition(
        &mut self,
        transaction: Transaction,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        context: AdmissionContext,
        source: AdmissionProjectionSource,
    ) -> Result<MempoolTransition, ManagedNetworkError> {
        let prepared = match self.mempool.prepare_transaction_with_context(
            &self.chainstate,
            transaction.clone(),
            verify_flags,
            consensus_params,
            context,
        ) {
            Ok(prepared) => prepared,
            Err(_) => {
                return self
                    .mempool
                    .submit_transaction_transition_with_context(
                        &self.chainstate,
                        transaction,
                        verify_flags,
                        consensus_params,
                        context,
                    )
                    .map_err(ManagedNetworkError::from);
            }
        };
        let admission = prepared
            .facts()
            .maybe_admission_result()
            .cloned()
            .ok_or_else(|| MempoolError::InternalInvariant {
                reason: "singleton preparation omitted its admission result".to_string(),
            })?;
        let wtxid = transaction_wtxid(&transaction)?;
        let outcome = if admission.replaced.is_empty() {
            MempoolOutcome::Accepted {
                txid: admission.accepted,
                wtxid,
                evicted: admission.evicted,
            }
        } else {
            MempoolOutcome::Replaced {
                txid: admission.accepted,
                wtxid,
                replaced: admission.replaced,
                evicted: admission.evicted,
            }
        };
        let plan = LifecycleProjectionPlan::prepare_admission(
            self,
            self.authority_epoch(),
            prepared,
            source,
        )
        .map_err(lifecycle_admission_error)?;
        let LifecycleCommandResult::Lifecycle(delta) =
            apply_lifecycle_command(self, LifecycleCommand::SingletonAdmission(plan))
                .map_err(lifecycle_admission_error)?
        else {
            return Err(MempoolError::InternalInvariant {
                reason: "singleton admission dispatcher returned a non-lifecycle result"
                    .to_string(),
            }
            .into());
        };
        Ok(MempoolTransition { outcome, delta })
    }
}
