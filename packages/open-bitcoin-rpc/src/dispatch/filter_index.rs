// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/node.cpp
// - packages/bitcoin-knots/src/index/base.cpp

use crate::method::{
    BasicFilterIndexSummary, BlockFilterSelection, GetBlockFilterRequest, GetBlockFilterResult,
    GetIndexInfoRequest, MethodCall,
};
use crate::{ManagedRpcContext, RpcErrorCode, RpcErrorDetail, RpcFailure, RpcFailureKind};
use open_bitcoin_node::core::chainstate::CoinsView;
use open_bitcoin_node::{
    BasicBlockValidationProvenance, BasicFilterQuery, BasicFilterQueryError,
    BasicFilterReadBarrier, ChainstateStore,
};
use serde_json::Value;

/// Pending stays typed until the authenticated async shell can release its lock.
pub(crate) enum PreparedDispatch {
    Complete(Value),
    Pending {
        request: GetBlockFilterRequest,
        barrier: BasicFilterReadBarrier,
    },
}

pub(crate) fn prepare<S: ChainstateStore, V: CoinsView>(
    context: &mut ManagedRpcContext<S, V>,
    call: MethodCall,
) -> Result<PreparedDispatch, RpcFailure> {
    match call {
        MethodCall::GetBlockFilter(request) => prepare_filter(context, request),
        call => super::dispatch(context, call).map(PreparedDispatch::Complete),
    }
}

pub(super) fn prepare_filter<S: ChainstateStore, V: CoinsView>(
    context: &ManagedRpcContext<S, V>,
    request: GetBlockFilterRequest,
) -> Result<PreparedDispatch, RpcFailure> {
    if request.filter_type == BlockFilterSelection::V0 {
        return Err(request.filter_type.disabled_failure());
    }
    let query = context
        .basic_filter_network_handle()
        .basic_filter_query(request.block_hash)
        .map_err(query_failure)?;
    project_query(request, query)
}

pub(crate) fn project_query(
    request: GetBlockFilterRequest,
    query: BasicFilterQuery,
) -> Result<PreparedDispatch, RpcFailure> {
    match query {
        BasicFilterQuery::Disabled => Err(request.filter_type.disabled_failure()),
        BasicFilterQuery::UnknownBlock => Err(failure(
            RpcErrorCode::InvalidAddressOrKey,
            "Block not found",
        )),
        BasicFilterQuery::Missing {
            provenance,
            initially_synchronized,
        } => Err(missing_failure(provenance, initially_synchronized)),
        BasicFilterQuery::Found(record) => {
            serde_json::to_value(GetBlockFilterResult::from_encoded(
                record.encoded_bytes(),
                *record.filter_header().as_bytes(),
            ))
            .map(PreparedDispatch::Complete)
            .map_err(|_| internal_failure())
        }
        BasicFilterQuery::Pending(barrier) => Ok(PreparedDispatch::Pending { request, barrier }),
    }
}

pub(crate) fn completed(prepared: PreparedDispatch) -> Result<Value, RpcFailure> {
    match prepared {
        PreparedDispatch::Complete(value) => Ok(value),
        PreparedDispatch::Pending { .. } => Err(RpcFailure::internal_error(
            "BASIC filter read requires asynchronous dispatch",
        )),
    }
}

pub(super) fn index_info<S: ChainstateStore, V: CoinsView>(
    context: &ManagedRpcContext<S, V>,
    request: GetIndexInfoRequest,
) -> Result<Value, RpcFailure> {
    if request
        .maybe_index_name
        .as_deref()
        .is_some_and(|name| !name.is_empty() && name != crate::method::BASIC_FILTER_INDEX_NAME)
    {
        return Ok(serde_json::json!({}));
    }
    let maybe_summary = context
        .basic_filter_network_handle()
        .maybe_basic_index_summary()
        .map_err(query_failure)?
        .map(|summary| BasicFilterIndexSummary {
            synced: summary.synced,
            best_block_height: summary.best_block_height,
        });
    serde_json::to_value(request.project_basic(maybe_summary)).map_err(|_| internal_failure())
}

fn missing_failure(
    provenance: BasicBlockValidationProvenance,
    initially_synchronized: bool,
) -> RpcFailure {
    match provenance {
        BasicBlockValidationProvenance::NeverConnected => failure(
            RpcErrorCode::InvalidAddressOrKey,
            "Filter not found. Block was not connected to active chain.",
        ),
        BasicBlockValidationProvenance::ScriptsValid if !initially_synchronized => failure(
            RpcErrorCode::MiscError,
            "Filter not found. Block filters are still in the process of being indexed.",
        ),
        BasicBlockValidationProvenance::ScriptsValid => failure(
            RpcErrorCode::InternalError,
            "Filter not found. This error is unexpected and indicates index corruption.",
        ),
        BasicBlockValidationProvenance::UnknownLegacy => internal_failure(),
    }
}

pub(crate) fn query_failure(_error: BasicFilterQueryError) -> RpcFailure {
    internal_failure()
}

fn internal_failure() -> RpcFailure {
    RpcFailure::internal_error("Block filter index is unavailable")
}

fn failure(code: RpcErrorCode, message: &'static str) -> RpcFailure {
    RpcFailure::new(
        if code == RpcErrorCode::InternalError {
            RpcFailureKind::InternalError
        } else {
            RpcFailureKind::InvalidParams
        },
        Some(RpcErrorDetail::new(code, message)),
    )
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod initial_tests {
    use crate::{
        ManagedRpcContext,
        dispatch::dispatch,
        method::{RequestParameters, normalize_method_call},
    };
    use serde_json::json;

    #[test]
    fn phase159_filter_rpc_dispatch_disabled_and_summary() {
        // Arrange
        let mut context =
            ManagedRpcContext::from_runtime_config(&crate::config::RuntimeConfig::default());
        let filter = normalize_method_call(
            "getblockfilter",
            RequestParameters::Positional(vec![json!("00".repeat(32))]),
        )
        .expect("typed request");
        let summary =
            normalize_method_call("getindexinfo", RequestParameters::None).expect("typed request");
        // Act
        let failure = dispatch(&mut context, filter).expect_err("disabled");
        let value = dispatch(&mut context, summary).expect("summary");
        // Assert
        let detail = failure.maybe_detail.expect("detail");
        assert_eq!(detail.code.as_i32(), -1);
        assert_eq!(detail.message, "Index is not enabled for filtertype basic");
        assert_eq!(value, json!({}));
    }
}
