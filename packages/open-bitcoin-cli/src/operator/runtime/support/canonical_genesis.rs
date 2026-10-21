// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

pub(super) fn observe_canonical_genesis(
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
    let opened = FjallNodeStore::reopen_canonical_genesis_existing(data_dir, selected_chain)
        .map_err(|_| OperatorRuntimeError::CompatibilityRefused {
            refusal: CompatibilityRefusal::from_observation(
                selected_chain,
                inspect_datadir_without_backend(data_dir),
            ),
        })?;
    let network = match selected_chain {
        CanonicalChain::Mainnet => SyncNetwork::Mainnet,
        CanonicalChain::Testnet3 => SyncNetwork::Testnet,
        CanonicalChain::Signet => SyncNetwork::Signet,
        CanonicalChain::Regtest => SyncNetwork::Regtest,
    };
    let runtime = DurableSyncRuntime::open_canonical_genesis_configured(
        opened,
        SyncRuntimeConfig {
            network,
            ..Default::default()
        },
        BasicFilterStartupMode::PreserveSaved,
    )
    .map_err(|_| OperatorRuntimeError::CanonicalObservationUnavailable)?;
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
    if format == OperatorOutputFormat::Json {
        let mut value = serde_json::to_value(&metadata)
            .map_err(|_| OperatorRuntimeError::CanonicalObservationUnavailable)?;
        value["selected_network"] = json!(selected_chain.to_string());
        value["basic_index"] = json!(snapshot.basic_index());
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
        "Selected network: {selected_chain}\nBASIC: {basic_state}\n"
    ));
    Ok(outcome)
}
