// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

/// Observe only a separately proved genesis or complete default-Regtest history.
pub(super) fn observe_canonical_status(
    data_dir: &Path,
    selected_chain: CanonicalChain,
    format: OperatorOutputFormat,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    use open_bitcoin_node::storage::chain_compatibility::{
        CompatibilityRefusal, inspect_datadir_without_backend,
    };
    use open_bitcoin_node::{
        DurableSyncRuntime, SyncNetwork, SyncRuntimeConfig, chainstate::BasicFilterStartupMode,
    };
    let refusal = || OperatorRuntimeError::CompatibilityRefused {
        refusal: CompatibilityRefusal::from_observation(
            selected_chain,
            inspect_datadir_without_backend(data_dir),
        ),
    };
    let network = match selected_chain {
        CanonicalChain::Mainnet => SyncNetwork::Mainnet,
        CanonicalChain::Testnet3 => SyncNetwork::Testnet,
        CanonicalChain::Signet => SyncNetwork::Signet,
        CanonicalChain::Regtest => SyncNetwork::Regtest,
    };
    let config = SyncRuntimeConfig {
        network,
        ..Default::default()
    };
    let (runtime, observed_evidence_class) =
        match FjallNodeStore::reopen_canonical_genesis_existing(data_dir, selected_chain) {
            Ok(opened) => (
                DurableSyncRuntime::open_canonical_genesis_configured(
                    opened,
                    config,
                    BasicFilterStartupMode::PreserveSaved,
                )
                .map_err(|_| OperatorRuntimeError::CanonicalObservationUnavailable)?,
                "proved_genesis",
            ),
            Err(_) if selected_chain == CanonicalChain::Regtest => {
                // A failed genesis proof grants no authority. History has its own
                // complete copied/original replay proof before opening the owner.
                let opened =
                    FjallNodeStore::reopen_canonical_history_existing(data_dir, selected_chain)
                        .map_err(|_| refusal())?;
                (
                    DurableSyncRuntime::open_canonical_history_configured(
                        opened,
                        config,
                        BasicFilterStartupMode::PreserveSaved,
                    )
                    .map_err(|_| OperatorRuntimeError::CanonicalObservationUnavailable)?,
                    "complete_retained_body_reopen",
                )
            }
            Err(_) => return Err(refusal()),
        };
    let mut metadata = runtime
        .store()
        .load_runtime_metadata()
        .map_err(|_| OperatorRuntimeError::CanonicalObservationUnavailable)?
        .unwrap_or_default();
    let timestamp = metadata
        .maybe_sync_state
        .as_ref()
        .map_or(0, |state| state.updated_at_unix_seconds);
    let timestamp = i64::try_from(timestamp)
        .map_err(|_| OperatorRuntimeError::CanonicalObservationUnavailable)?;
    metadata.maybe_sync_state = Some(
        runtime
            .durable_sync_state(SyncLifecycleState::Stopped, None, timestamp)
            .map_err(|_| OperatorRuntimeError::CanonicalObservationUnavailable)?,
    );
    let snapshot = runtime
        .network_handle()
        .operator_snapshot()
        .map_err(|_| OperatorRuntimeError::CanonicalObservationUnavailable)?;
    let scope = selected_chain.spec().validation_scope();
    let implementation = scope.local_ordinary_implementation();
    let generic_gate_missing = selected_chain
        .spec()
        .require_ordinary_history()
        .err()
        .map_or_else(Vec::new, |error| {
            error
                .missing_prerequisites()
                .iter()
                .map(|prerequisite| prerequisite.code())
                .collect()
        });
    let implementation_missing = scope
        .missing_implementation_prerequisites()
        .iter()
        .map(|prerequisite| prerequisite.code())
        .collect::<Vec<_>>();
    if format == OperatorOutputFormat::Json {
        let mut value = serde_json::to_value(&metadata)
            .map_err(|_| OperatorRuntimeError::CanonicalObservationUnavailable)?;
        value["selected_network"] = json!(selected_chain.to_string());
        value["basic_index"] = json!(snapshot.basic_index());
        value["support"] = json!({
            "selected_identity": selected_chain.to_string(),
            "observed_evidence_class": observed_evidence_class,
            "local_ordinary_implementation": implementation.code(),
            "retained_body_reopen_bound": implementation.retained_body_reopen_bound(),
            "missing_implementation_prerequisites": implementation_missing,
            "generic_ordinary_gate_missing": generic_gate_missing,
            "actual_knots_daemon_differential_qualified": scope.actual_daemon_differential_qualified(),
        });
        return Ok(OperatorCommandOutcome::success(format!(
            "{}\n",
            serde_json::to_string_pretty(&value)
                .map_err(|_| OperatorRuntimeError::CanonicalObservationUnavailable)?
        )));
    }
    let mut outcome = render_sync_outcome(&SyncCommand::Status, format, &metadata)?;
    let basic_state = match &snapshot.basic_index().operational_state {
        FieldAvailability::Available(value) => format!("{value:?}").to_lowercase(),
        FieldAvailability::Unavailable { .. } => "unavailable".to_owned(),
    };
    outcome.stdout.text.push_str(&format!(
        "Selected network: {selected_chain}\nObserved evidence: {observed_evidence_class}\nLocal ordinary implementation: {}\nRetained-body reopen bound: {}\nActual Knots-daemon differential qualified: {}\nBASIC: {basic_state}\n",
        implementation.code(),
        implementation.retained_body_reopen_bound(),
        scope.actual_daemon_differential_qualified(),
    ));
    Ok(outcome)
}
