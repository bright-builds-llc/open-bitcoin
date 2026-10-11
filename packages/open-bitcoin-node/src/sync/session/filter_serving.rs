// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

//! Consume complete prepared responses outside every authority and registry lock.

use super::super::{SyncPeerSession, SyncRuntimeError};
use crate::network::{
    ManagedInboundResponsePlanItem,
    filter_transport::{
        FilterTransportCompletion, FilterTransportDisposition, FilterTransportError,
        FilterWrittenPrefix, PreparedFilterResponse,
    },
};
use crate::{ChainstateStore, ManagedNetworkHandle};
use open_bitcoin_core::{chainstate::CoinsView, primitives::NetworkMagic};
use open_bitcoin_network::PeerId;

pub(super) fn transport_error(error: FilterTransportError) -> SyncRuntimeError {
    SyncRuntimeError::Network {
        message: error.to_string(),
    }
}

pub(crate) fn send_filter_plans<S: ChainstateStore, V: CoinsView>(
    network: &ManagedNetworkHandle<S, V>,
    session: &mut impl SyncPeerSession,
    peer: PeerId,
    magic: NetworkMagic,
    plans: Vec<ManagedInboundResponsePlanItem>,
    should_cancel: &mut impl FnMut() -> bool,
) -> Result<(), SyncRuntimeError> {
    for plan in plans {
        match plan {
            ManagedInboundResponsePlanItem::PreparedFilterServe(response) => {
                send_response(network, session, peer, magic, *response, should_cancel).0?;
            }
            ManagedInboundResponsePlanItem::FilterServeUnavailable(_) => {}
            _ => {
                return Err(SyncRuntimeError::Network {
                    message: "outbound response plan contains unresolved non-filter intent"
                        .to_string(),
                });
            }
        }
    }
    Ok(())
}

/// Return factual complete-frame progress even when the terminal write fails.
pub(crate) fn send_response<S: ChainstateStore, V: CoinsView>(
    network: &ManagedNetworkHandle<S, V>,
    session: &mut impl SyncPeerSession,
    peer: PeerId,
    magic: NetworkMagic,
    response: PreparedFilterResponse,
    should_cancel: &mut impl FnMut() -> bool,
) -> (
    Result<(), SyncRuntimeError>,
    Result<FilterTransportCompletion, FilterTransportError>,
) {
    let mut maybe_response = Some(response);
    let mut physical_prefix = Ok(FilterWrittenPrefix::default());
    let mut disposition = FilterTransportDisposition::Aborted;
    let result = (|| {
        let response = maybe_response
            .as_mut()
            .ok_or_else(|| transport_error(FilterTransportError::InvalidFrames))?;
        if response.peer_id() != peer {
            return Err(SyncRuntimeError::Network {
                message: "filter response target mismatch".to_string(),
            });
        }
        loop {
            if should_cancel() {
                disposition = FilterTransportDisposition::Aborted;
                return Ok(());
            }
            if !network
                .basic_filter_response_is_current(response)
                .map_err(transport_error)?
            {
                return Err(transport_error(FilterTransportError::Invalidated));
            }
            let Some(frame) = response.next_frame().map_err(transport_error)? else {
                return Ok(());
            };
            session.send_encoded(frame.bytes(), magic)?;
            let receipt = frame.acknowledge_write();
            if should_cancel() {
                let response = maybe_response
                    .take()
                    .ok_or_else(|| transport_error(FilterTransportError::InvalidFrames))?;
                if let Err(error) = response.cancel() {
                    physical_prefix = Err(error);
                    return Err(transport_error(error));
                }
                let completion = match network.complete_basic_filter_frame(receipt) {
                    Ok(completion) => completion,
                    Err(error) => {
                        physical_prefix = Err(error);
                        return Err(transport_error(error));
                    }
                };
                physical_prefix = Ok(completion.progress);
                disposition = completion.disposition;
                return Ok(());
            }
            let completion = network
                .complete_basic_filter_frame(receipt)
                .map_err(transport_error)?;
            disposition = completion.disposition;
            match completion.disposition {
                FilterTransportDisposition::CurrentRangeComplete => return Ok(()),
                FilterTransportDisposition::WrittenPrefix => {}
                _ => return Err(transport_error(FilterTransportError::Invalidated)),
            }
        }
    })();
    let prefix = maybe_response
        .as_ref()
        .map_or(physical_prefix, PreparedFilterResponse::progress);
    let result = match (&result, &prefix) {
        (Ok(()), Err(error)) => Err(transport_error(*error)),
        _ => result,
    };
    let cleanup = maybe_response.map_or(Ok(()), |response| {
        response.cancel().map_err(transport_error)
    });
    let result = match (result, cleanup) {
        (Err(error), Err(cleanup)) => Err(SyncRuntimeError::Network {
            message: format!("{error}; filter abort cleanup also failed: {cleanup}"),
        }),
        (Err(error), _) | (_, Err(error)) => Err(error),
        (Ok(()), Ok(())) => Ok(()),
    };
    if result.is_err() && disposition != FilterTransportDisposition::PhysicalPrefixInvalidated {
        disposition = FilterTransportDisposition::Aborted;
    }
    (
        result,
        prefix.map(|progress| FilterTransportCompletion {
            disposition,
            progress,
        }),
    )
}
