// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/rpc/util.cpp

use super::{
    CanonicalChain, FjallNodeStore, MULTI_WALLET_SELECTION_MESSAGE, OperatorRuntimeError, Path,
    WalletOperatorError, WalletRegistry, to_invalid_request,
};

pub(super) fn resolve_managed_wallet_name(
    maybe_data_dir: Option<&Path>,
    maybe_requested_wallet_name: Option<&str>,
    selected_chain: CanonicalChain,
) -> Result<Option<String>, OperatorRuntimeError> {
    let Some(data_dir) = maybe_data_dir else {
        return Err(OperatorRuntimeError::InvalidRequest {
            message: "wallet name lookup requires a configured datadir".to_owned(),
        });
    };
    let store = FjallNodeStore::open_ordinary(data_dir, selected_chain)
        .map_err(|refusal| OperatorRuntimeError::CompatibilityRefused { refusal })?;
    let registry = WalletRegistry::load(&store).map_err(to_invalid_request)?;
    maybe_resolve_wallet_name_from_registry(&registry, maybe_requested_wallet_name)
        .map_err(to_invalid_request)
}

pub(super) fn maybe_resolve_wallet_name_from_registry(
    registry: &WalletRegistry,
    maybe_requested_wallet_name: Option<&str>,
) -> Result<Option<String>, WalletOperatorError> {
    if let Some(wallet_name) = maybe_requested_wallet_name {
        if registry
            .wallet_names()
            .iter()
            .any(|candidate| candidate == wallet_name)
        {
            return Ok(Some(wallet_name.to_string()));
        }
        return Err(WalletOperatorError::new(
            "Requested wallet does not exist or is not loaded",
        ));
    }
    if let Some(selected_wallet_name) = registry.selected_wallet_name() {
        return Ok(Some(selected_wallet_name.to_string()));
    }
    match registry.wallet_names() {
        [] => Ok(None),
        [wallet_name] => Ok(Some(wallet_name.clone())),
        _ => Err(WalletOperatorError::new(MULTI_WALLET_SELECTION_MESSAGE)),
    }
}
