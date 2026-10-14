// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::{FieldAvailability, FjallNodeStore, OpenBitcoinStatusSnapshot};

use crate::operator::{
    config::OperatorConfigResolution,
    runtime::chain_compatibility::canonical_chain_from_resolution, status::collect_resource_bounds,
};

pub(crate) fn enrich_soak_resource_bounds_from_store(
    snapshot: &mut OpenBitcoinStatusSnapshot,
    resolution: &OperatorConfigResolution,
) {
    let Some(datadir) = resolution.maybe_data_dir.as_deref() else {
        snapshot.resource_bounds =
            FieldAvailability::unavailable("chain_compatibility_unavailable: datadir_unavailable");
        return;
    };
    let chain = match canonical_chain_from_resolution(resolution) {
        Ok(chain) => chain,
        Err(error) => {
            snapshot.resource_bounds =
                FieldAvailability::unavailable(format!("chain_compatibility_unavailable: {error}"));
            return;
        }
    };
    let store = match FjallNodeStore::open_ordinary(datadir, chain) {
        Ok(store) => store,
        Err(refusal) => {
            snapshot.resource_bounds = FieldAvailability::unavailable(format!(
                "chain_compatibility_unavailable: {refusal}"
            ));
            return;
        }
    };
    let metadata = match store.load_runtime_metadata() {
        Ok(Some(metadata)) => metadata,
        Ok(None) | Err(_) => {
            snapshot.resource_bounds = FieldAvailability::unavailable(
                "durable_resource_bounds_unavailable: runtime_metadata_unavailable",
            );
            return;
        }
    };
    if let Some(sync_state) = metadata.maybe_sync_state.as_ref() {
        snapshot.sync = sync_state.sync.clone();
        snapshot.peers = sync_state.peers.clone();
    }
    snapshot.resource_bounds =
        collect_resource_bounds(resolution, metadata.maybe_sync_state.as_ref());
}
