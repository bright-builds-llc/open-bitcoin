// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use open_bitcoin_node::core::{
    codec::parse_transaction,
    mempool::{MempoolError, MempoolOutcome},
    primitives::{OutPoint, Transaction},
    wallet::SingleKeyDescriptor,
};
use open_bitcoin_node::status::{
    FieldAvailability, SyncLifecycleState, SyncProgress, SyncProgressSignal,
};

use crate::{
    ManagedRpcContext,
    error::RpcFailure,
    method::{
        DeriveAddressesRequest, DeriveAddressesResponse, GetBlockchainInfoResponse,
        GetMempoolInfoResponse, GetNetworkInfoResponse, OpenBitcoinNetworkStatusResponse,
        OpenBitcoinSyncControlResponse, SendRawTransactionRequest, SendRawTransactionResponse,
    },
};

use super::{decode, network_authority_error_to_failure, version_number, wallet_error_to_failure};

const UNSUPPORTED_MAX_FEE_RATE_MESSAGE: &str =
    "sendrawtransaction maxfeerate enforcement is not supported in Phase 8; omit maxfeerate";
const UNSUPPORTED_MAX_BURN_AMOUNT_MESSAGE: &str =
    "sendrawtransaction maxburnamount enforcement is not supported in Phase 8; omit maxburnamount";

pub(super) fn get_blockchain_info(
    context: &ManagedRpcContext,
) -> Result<GetBlockchainInfoResponse, RpcFailure> {
    if let Some(durable_sync_state) = context.maybe_durable_sync_state() {
        return durable_blockchain_info(context, durable_sync_state);
    }

    let maybe_tip = context
        .maybe_chain_tip()
        .map_err(network_authority_error_to_failure)?;
    Ok(GetBlockchainInfoResponse {
        chain: context.chain_name().to_string(),
        blocks: maybe_tip.as_ref().map_or(0, |tip| tip.height),
        headers: maybe_tip.as_ref().map_or(0, |tip| tip.height),
        maybe_best_block_hash: maybe_tip
            .as_ref()
            .map(|tip| decode::encode_hex(tip.block_hash.as_bytes())),
        maybe_median_time_past: maybe_tip.as_ref().map(|tip| tip.median_time_past),
        verificationprogress: if maybe_tip.is_some() { 1.0 } else { 0.0 },
        initialblockdownload: false,
        warnings: Vec::new(),
    })
}

fn durable_blockchain_info(
    context: &ManagedRpcContext,
    durable_sync_state: &open_bitcoin_node::DurableSyncState,
) -> Result<GetBlockchainInfoResponse, RpcFailure> {
    let maybe_tip = context
        .maybe_chain_tip()
        .map_err(network_authority_error_to_failure)?;
    let maybe_sync_progress = match &durable_sync_state.sync.sync_progress {
        FieldAvailability::Available(value) => Some(value),
        FieldAvailability::Unavailable { .. } => None,
    };
    let headers = maybe_sync_progress.map_or(0, |value| u64_to_u32(value.header_height));
    let blocks = maybe_sync_progress.map_or(0, |value| u64_to_u32(value.block_height));
    let lifecycle = match durable_sync_state.sync.lifecycle {
        FieldAvailability::Available(value) => Some(value),
        FieldAvailability::Unavailable { .. } => None,
    };

    Ok(GetBlockchainInfoResponse {
        chain: context.chain_name().to_string(),
        blocks,
        headers,
        maybe_best_block_hash: maybe_tip
            .as_ref()
            .map(|tip| decode::encode_hex(tip.block_hash.as_bytes())),
        maybe_median_time_past: maybe_tip.as_ref().map(|tip| tip.median_time_past),
        verificationprogress: durable_verification_progress(maybe_sync_progress),
        initialblockdownload: durable_initial_block_download(headers, blocks, lifecycle),
        warnings: durable_warnings(durable_sync_state),
    })
}

pub(super) fn get_mempool_info(
    context: &ManagedRpcContext,
) -> Result<GetMempoolInfoResponse, RpcFailure> {
    let info = context
        .mempool_info()
        .map_err(network_authority_error_to_failure)?;
    Ok(GetMempoolInfoResponse {
        size: info.transaction_count,
        bytes: info.total_virtual_size,
        usage: info.total_virtual_size,
        total_fee_sats: info.total_fee_sats,
        maxmempool: info.max_mempool_virtual_size,
        mempoolminfee: info.min_relay_feerate_sats_per_kvb,
        minrelaytxfee: info.min_relay_feerate_sats_per_kvb,
        loaded: true,
    })
}

fn durable_verification_progress(maybe_sync_progress: Option<&SyncProgress>) -> f64 {
    let Some(sync_progress) = maybe_sync_progress else {
        return 0.0;
    };
    if sync_progress.header_height == 0 {
        return 0.0;
    }

    sync_progress.block_height as f64 / sync_progress.header_height as f64
}

fn durable_initial_block_download(
    headers: u32,
    blocks: u32,
    lifecycle: Option<SyncLifecycleState>,
) -> bool {
    if headers > blocks {
        return true;
    }

    matches!(
        lifecycle,
        Some(SyncLifecycleState::Recovering | SyncLifecycleState::Failed)
    )
}

fn durable_warnings(durable_sync_state: &open_bitcoin_node::DurableSyncState) -> Vec<String> {
    let mut warnings = Vec::new();
    if let FieldAvailability::Available(value) = &durable_sync_state.sync.last_error {
        warnings.push(value.clone());
    }
    if let FieldAvailability::Available(value) = &durable_sync_state.sync.progress_signal {
        warnings.push(format!(
            "progress_signal={}",
            sync_progress_signal_name(*value)
        ));
    }
    if let FieldAvailability::Available(value) = &durable_sync_state.sync.latest_stop_reason {
        warnings.push(format!("latest_stop_reason={}", value.label.as_str()));
    }
    if let FieldAvailability::Available(value) = &durable_sync_state.sync.recovery_category {
        warnings.push(format!("recovery_category={}", value.as_str()));
    }
    if let FieldAvailability::Available(value) = &durable_sync_state.sync.recovery_action {
        warnings.push(value.clone());
    }
    warnings
}

fn sync_progress_signal_name(signal: SyncProgressSignal) -> &'static str {
    match signal {
        SyncProgressSignal::HeaderProgress => "header_progress",
        SyncProgressSignal::BlockProgress => "block_progress",
        SyncProgressSignal::WaitingForPeers => "waiting_for_peers",
        SyncProgressSignal::PeerFailures => "peer_failures",
        SyncProgressSignal::AwaitingBlocks => "awaiting_blocks",
        SyncProgressSignal::Steady => "steady",
    }
}

fn u64_to_u32(value: u64) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

pub(super) fn get_network_info(
    context: &ManagedRpcContext,
) -> Result<GetNetworkInfoResponse, RpcFailure> {
    let snapshot = context
        .authoritative_operator_snapshot()
        .map_err(network_authority_error_to_failure)?;
    let network_info = snapshot.network();
    let mempool_info = snapshot.mempool();
    Ok(GetNetworkInfoResponse {
        version: version_number(),
        subversion: network_info.user_agent.clone(),
        protocolversion: network_info.protocol_version,
        localservices: format!("{:016x}", network_info.local_services_bits),
        localrelay: network_info.relay,
        connections: network_info.connected_peers,
        connections_in: network_info.inbound_peers,
        connections_out: network_info.outbound_peers,
        relayfee: mempool_info.min_relay_feerate_sats_per_kvb,
        incrementalfee: mempool_info.incremental_relay_feerate_sats_per_kvb,
        warnings: Vec::new(),
    })
}

pub(super) fn open_bitcoin_network_status(
    context: &ManagedRpcContext,
) -> Result<OpenBitcoinNetworkStatusResponse, RpcFailure> {
    /* Phase 116 compatibility anchor for the replaced direct projection:
    block_relay_evidence_status()
            .map_err(network_authority_error_to_failure)? */
    let snapshot = context
        .authoritative_operator_snapshot()
        .map_err(network_authority_error_to_failure)?;
    Ok(OpenBitcoinNetworkStatusResponse {
        inbound: snapshot.inbound().clone(),
        relay: snapshot.relay().clone(),
        block_relay: snapshot.block_relay().clone(),
        metrics: context.metrics_status(),
    })
}

pub(super) fn open_bitcoin_sync_status(
    context: &ManagedRpcContext,
) -> Result<OpenBitcoinSyncControlResponse, RpcFailure> {
    open_bitcoin_sync_response(context.daemon_sync_status()?)
}

pub(super) fn open_bitcoin_sync_pause(
    context: &ManagedRpcContext,
) -> Result<OpenBitcoinSyncControlResponse, RpcFailure> {
    open_bitcoin_sync_response(context.daemon_sync_pause()?)
}

pub(super) fn open_bitcoin_sync_resume(
    context: &ManagedRpcContext,
) -> Result<OpenBitcoinSyncControlResponse, RpcFailure> {
    open_bitcoin_sync_response(context.daemon_sync_resume()?)
}

fn open_bitcoin_sync_response(
    metadata: open_bitcoin_node::RuntimeMetadata,
) -> Result<OpenBitcoinSyncControlResponse, RpcFailure> {
    Ok(OpenBitcoinSyncControlResponse { metadata })
}

pub(super) fn derive_addresses(
    context: &ManagedRpcContext,
    request: DeriveAddressesRequest,
) -> Result<DeriveAddressesResponse, RpcFailure> {
    let descriptor = SingleKeyDescriptor::parse(&request.descriptor, context.chain())
        .map_err(wallet_error_to_failure)?;
    let address = descriptor
        .address(context.chain())
        .map_err(wallet_error_to_failure)?;
    Ok(DeriveAddressesResponse {
        addresses: vec![address.to_string()],
    })
}

pub(super) fn send_raw_transaction(
    context: &mut ManagedRpcContext,
    request: SendRawTransactionRequest,
) -> Result<SendRawTransactionResponse, RpcFailure> {
    if request.maybe_max_fee_rate_sat_per_kvb.is_some() {
        return Err(RpcFailure::invalid_params(UNSUPPORTED_MAX_FEE_RATE_MESSAGE));
    }
    if request.maybe_max_burn_amount_sats.is_some() {
        return Err(RpcFailure::invalid_params(
            UNSUPPORTED_MAX_BURN_AMOUNT_MESSAGE,
        ));
    }

    let transaction_bytes = decode::decode_hex(&request.transaction_hex)
        .map_err(|error| RpcFailure::invalid_params(error.to_string()))?;
    let transaction = parse_transaction(&transaction_bytes)
        .map_err(|error| RpcFailure::invalid_params(error.to_string()))?;
    let outcome = context
        .submit_local_transaction_with_relay_evidence(transaction.clone())
        .map_err(network_authority_error_to_failure)?;

    send_raw_transaction_response(outcome, &transaction)
}

fn send_raw_transaction_response(
    outcome: MempoolOutcome,
    transaction: &Transaction,
) -> Result<SendRawTransactionResponse, RpcFailure> {
    match outcome {
        MempoolOutcome::Accepted { txid, evicted, .. } => Ok(SendRawTransactionResponse {
            txid_hex: decode::encode_hex(txid.as_bytes()),
            replaced_txids: Vec::new(),
            evicted_txids: evicted
                .into_iter()
                .map(|txid| decode::encode_hex(txid.as_bytes()))
                .collect(),
        }),
        MempoolOutcome::Replaced {
            txid,
            replaced,
            evicted,
            ..
        } => Ok(SendRawTransactionResponse {
            txid_hex: decode::encode_hex(txid.as_bytes()),
            replaced_txids: replaced
                .into_iter()
                .map(|txid| decode::encode_hex(txid.as_bytes()))
                .collect(),
            evicted_txids: evicted
                .into_iter()
                .map(|txid| decode::encode_hex(txid.as_bytes()))
                .collect(),
        }),
        MempoolOutcome::Duplicate { txid } => Err(mempool_outcome_failure(
            MempoolError::DuplicateTransaction { txid },
        )),
        MempoolOutcome::Orphaned {
            missing_parents, ..
        } => Err(mempool_outcome_failure(orphaned_submission_error(
            transaction,
            &missing_parents,
        ))),
        MempoolOutcome::Rejected { category, .. } => {
            Err(mempool_outcome_failure(MempoolError::InternalInvariant {
                reason: format!("local submission rejected: {}", category.as_str()),
            }))
        }
        MempoolOutcome::Evicted { txid, .. } | MempoolOutcome::Expired { txid, .. } => {
            Err(mempool_outcome_failure(MempoolError::CandidateEvicted {
                txid,
            }))
        }
    }
}

fn orphaned_submission_error(
    transaction: &Transaction,
    missing_parents: &[open_bitcoin_node::core::primitives::Txid],
) -> MempoolError {
    let Some(outpoint) = missing_input_outpoint(transaction, missing_parents) else {
        return MempoolError::InternalInvariant {
            reason: "local submission orphaned without inspectable missing input".to_string(),
        };
    };
    MempoolError::MissingInput { outpoint }
}

fn missing_input_outpoint(
    transaction: &Transaction,
    missing_parents: &[open_bitcoin_node::core::primitives::Txid],
) -> Option<OutPoint> {
    transaction
        .inputs
        .iter()
        .find(|input| missing_parents.contains(&input.previous_output.txid))
        .or_else(|| transaction.inputs.first())
        .map(|input| input.previous_output.clone())
}

fn mempool_outcome_failure(error: MempoolError) -> RpcFailure {
    RpcFailure::verify_rejected(error.to_string())
}
