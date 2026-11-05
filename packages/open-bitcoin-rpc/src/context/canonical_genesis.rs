// Parity breadcrumbs:
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/wallet/wallet.cpp

use super::{CanonicalContextError, CanonicalSubmissionBridge, ManagedRpcContext};
use crate::config::{BasicFilterIndexSetting, RuntimeConfig};
use open_bitcoin_node::chainstate::BasicFilterStartupMode;
use open_bitcoin_node::core::chainstate::engine::canonical_genesis::consensus_params;
use open_bitcoin_node::storage::{OpenedCanonicalGenesis, coins_view::FjallCoinsView};
use open_bitcoin_node::{DurableSyncRuntime, FjallChainstateStore};

impl ManagedRpcContext<FjallChainstateStore, FjallCoinsView> {
    /// Consume proved genesis storage without granting ordinary or wallet history.
    pub fn from_canonical_genesis_runtime_config(
        config: &RuntimeConfig,
        opened: OpenedCanonicalGenesis,
    ) -> Result<(Self, DurableSyncRuntime), CanonicalContextError> {
        preflight(config, &opened)?;
        let params = consensus_params(opened.genesis());
        #[cfg(test)]
        INITIALIZATIONS.set(INITIALIZATIONS.get() + 1);
        let (runtime, maybe_command) = DurableSyncRuntime::open_genesis_submission_configured(
            opened,
            config.sync.runtime.clone(),
            startup_mode(config.block_filter_index),
        )
        .map_err(CanonicalContextError::GenesisRuntime)?;
        let maybe_bridge = maybe_command.map(|command| {
            let command = std::sync::Arc::new(command);
            let reader = std::sync::Arc::clone(&command);
            #[cfg(test)]
            let observer = std::sync::Arc::clone(&command);
            CanonicalSubmissionBridge {
                active_tip: Box::new(move || {
                    // The ordinary observer returns detached facts after its guards drop.
                    reader
                        .observe_accepted_state(&[])
                        .map_err(|error| crate::RpcFailure::verify_error(error.to_string()))?
                        .fold(|(snapshot, _, _), _, _, _, _| {
                            snapshot.tip().cloned().ok_or_else(|| {
                                crate::RpcFailure::verify_error(
                                    "canonical accepted tip unavailable",
                                )
                            })
                        })
                }),
                submit: Box::new(move |wire| {
                    command
                        .submit(wire)
                        .map(|outcome| {
                            outcome.fold(
                                |_| serde_json::Value::Null,
                                |_| serde_json::json!("duplicate"),
                                |reason| {
                                    serde_json::json!(if reason.is_empty() {
                                        "rejected".to_owned()
                                    } else {
                                        reason
                                    })
                                },
                                || serde_json::json!("inconclusive"),
                            )
                        })
                        .map_err(|error| crate::RpcFailure::verify_error(error.to_string()))
                }),
                #[cfg(test)]
                observe: Box::new(move |hashes| {
                    observer
                        .observe_accepted_state(hashes)
                        .map(|facts| {
                            facts.fold(
                                |coins, maybe_progress, maybe_stored_basic, rows, provenance| {
                                    super::CanonicalObservation {
                                        coins,
                                        maybe_progress,
                                        maybe_stored_basic,
                                        rows,
                                        provenance,
                                    }
                                },
                            )
                        })
                        .map_err(|error| crate::RpcFailure::verify_error(error.to_string()))
                }),
            }
        });
        let context = super::canonical_history::assemble(config, &runtime, params, maybe_bridge)?;
        Ok((context, runtime))
    }
}

#[cfg(test)]
thread_local! {
    static INITIALIZATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

fn preflight(
    config: &RuntimeConfig,
    opened: &OpenedCanonicalGenesis,
) -> Result<(), CanonicalContextError> {
    let chain = super::canonical_chain::selected_chain(config)?;
    config
        .validate_peer_block_filters()
        .map_err(|_| CanonicalContextError::InvalidGenesisConfiguration)?;
    if opened.genesis().chain() != chain {
        return Err(CanonicalContextError::ChainMismatch);
    }
    if config.sync.is_enabled()
        || config.inbound.enabled
        || config.relay.enabled
        || config.block_serving.block_serving.enabled
        || config.block_serving.compact_relay.enabled
    {
        super::canonical_chain::require_ordinary_scope(chain)?;
    }
    Ok(())
}

fn startup_mode(setting: BasicFilterIndexSetting) -> BasicFilterStartupMode {
    match setting {
        BasicFilterIndexSetting::Basic => BasicFilterStartupMode::Enabled,
        BasicFilterIndexSetting::Disabled => BasicFilterStartupMode::Disabled,
        BasicFilterIndexSetting::Unspecified => BasicFilterStartupMode::PreserveSaved,
    }
}

#[cfg(test)]
mod tests;
