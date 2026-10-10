// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/node.cpp
// - packages/bitcoin-knots/src/index/base.cpp

use super::*;
use crate::method::{
    MethodOrigin, MethodScope, RequestParameters, SupportedMethod, normalize_method_call,
};
use serde_json::json;

#[test]
fn phase159_filter_rpc_dispatch_exact_missing_matrix() {
    // Arrange
    let cases = [
        (
            BasicBlockValidationProvenance::NeverConnected,
            false,
            -5,
            "Filter not found. Block was not connected to active chain.",
        ),
        (
            BasicBlockValidationProvenance::NeverConnected,
            true,
            -5,
            "Filter not found. Block was not connected to active chain.",
        ),
        (
            BasicBlockValidationProvenance::ScriptsValid,
            false,
            -1,
            "Filter not found. Block filters are still in the process of being indexed.",
        ),
        (
            BasicBlockValidationProvenance::ScriptsValid,
            true,
            -32603,
            "Filter not found. This error is unexpected and indicates index corruption.",
        ),
        (
            BasicBlockValidationProvenance::UnknownLegacy,
            false,
            -32603,
            "Block filter index is unavailable",
        ),
        (
            BasicBlockValidationProvenance::UnknownLegacy,
            true,
            -32603,
            "Block filter index is unavailable",
        ),
    ];
    for (provenance, initially_synchronized, code, message) in cases {
        // Act
        let failure = missing_failure(provenance, initially_synchronized);
        // Assert
        let detail = failure.maybe_detail.expect("detail");
        assert_eq!(detail.code.as_i32(), code);
        assert_eq!(detail.message, message);
    }
}

#[test]
fn phase159_filter_rpc_dispatch_registry_and_node_scope() {
    for (name, method) in [
        ("getblockfilter", SupportedMethod::GetBlockFilter),
        ("getindexinfo", SupportedMethod::GetIndexInfo),
    ] {
        // Act / Assert
        assert_eq!(SupportedMethod::from_name(name), Some(method));
        assert_eq!(method.name(), name);
        assert_eq!(method.origin(), MethodOrigin::BaselineParity);
        assert_eq!(method.scope(), MethodScope::Node);
        assert!(SupportedMethod::all().contains(&method));
    }
}

#[test]
fn phase159_filter_rpc_dispatch_parameter_precedence_before_disabled() {
    // Arrange
    let mut context = ManagedRpcContext::from_runtime_config(&crate::RuntimeConfig::default());
    let cases = [
        (json!(["bad", "unknown"]), -8),
        (json!(["00".repeat(32), 3]), -3),
        (json!(["00".repeat(32), "BASIC"]), -5),
    ];
    for (params, code) in cases {
        // Act
        let failure = normalize_method_call("getblockfilter", RequestParameters::from_json(params))
            .expect_err("normalizer rejects before backend");
        // Assert
        assert_eq!(failure.maybe_detail.expect("detail").code.as_i32(), code);
    }
    let v0 = normalize_method_call(
        "getblockfilter",
        RequestParameters::Positional(vec![json!("00".repeat(32)), json!("v0")]),
    )
    .expect("recognized");
    let failure = super::super::dispatch(&mut context, v0).expect_err("disabled V0");
    assert_eq!(
        failure.maybe_detail.expect("detail").message,
        "Index is not enabled for filtertype v0"
    );
}

#[test]
fn phase159_filter_rpc_dispatch_query_faults_are_allowlisted() {
    // Arrange
    let private_marker = "/private/datadir/backend-marker";
    let cases = [
        BasicFilterQueryError::Storage(open_bitcoin_node::StorageError::BackendFailure {
            namespace: open_bitcoin_node::StorageNamespace::BlockIndex,
            message: private_marker.into(),
            action: open_bitcoin_node::StorageRecoveryAction::Restart,
        }),
        BasicFilterQueryError::Storage(open_bitcoin_node::StorageError::Corruption {
            namespace: open_bitcoin_node::StorageNamespace::BlockIndex,
            detail: private_marker.into(),
            action: open_bitcoin_node::StorageRecoveryAction::Repair,
        }),
        BasicFilterQueryError::Authority(
            open_bitcoin_node::ManagedNetworkAuthorityError::LifecycleEffect(private_marker.into()),
        ),
        BasicFilterQueryError::Readiness(open_bitcoin_node::BasicFilterReadFailure::Invalidated),
        BasicFilterQueryError::Readiness(open_bitcoin_node::BasicFilterReadFailure::OwnerStopped),
        BasicFilterQueryError::Readiness(open_bitcoin_node::BasicFilterReadFailure::Capacity),
    ];
    for error in cases {
        // Act
        let failure = query_failure(error);
        // Assert
        assert!(!format!("{failure:?}").contains(private_marker));
        assert_eq!(
            failure.maybe_detail.expect("detail"),
            RpcErrorDetail::new(
                RpcErrorCode::InternalError,
                "Block filter index is unavailable"
            )
        );
    }
}
