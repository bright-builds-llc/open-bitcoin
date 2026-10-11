// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use std::collections::VecDeque;

use open_bitcoin_core::primitives::NetworkMagic;
use open_bitcoin_network::{PeerId, WireNetworkMessage};

use crate::{
    ManagedNetworkHandle,
    network::{
        AnnouncementPreparationOutcome, EffectAbort, PeerEmission, PeerEmissionWriteCapability,
    },
};

use super::super::{SyncPeerSession, SyncRuntimeError};

impl super::DurableSyncRuntime {
    #[cfg(test)]
    pub(in crate::sync) fn send_all<S: SyncPeerSession>(
        &mut self,
        session: &mut S,
        peer_id: PeerId,
        messages: &[WireNetworkMessage],
    ) -> Result<(), SyncRuntimeError> {
        self.send_all_with_cancel(session, peer_id, messages, &mut || false)
    }

    fn send_all_with_cancel<S: SyncPeerSession>(
        &mut self,
        session: &mut S,
        peer_id: PeerId,
        messages: &[WireNetworkMessage],
        should_cancel: &mut impl FnMut() -> bool,
    ) -> Result<(), SyncRuntimeError> {
        let buffers = messages
            .iter()
            .map(|message| {
                self.network
                    .encode_pending_network_message(peer_id, message)
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(super::filter_serving::transport_error)?;
        for (message, buffer) in messages.iter().zip(buffers) {
            if should_cancel() {
                return Ok(());
            }
            session.send_encoded(buffer.bytes(), self.config.network.magic())?;
            self.network.acknowledge_wire_message_written(message)?;
        }
        Ok(())
    }

    #[cfg(test)]
    pub(in crate::sync) fn send_all_for_peer<S: SyncPeerSession>(
        &mut self,
        session: &mut S,
        peer_id: PeerId,
        messages: &[WireNetworkMessage],
    ) -> Result<(), SyncRuntimeError> {
        self.send_all_for_peer_with_cancel(session, peer_id, messages, &mut || false)
    }

    pub(in crate::sync) fn send_all_for_peer_with_cancel<S: SyncPeerSession>(
        &mut self,
        session: &mut S,
        peer_id: PeerId,
        messages: &[WireNetworkMessage],
        should_cancel: &mut impl FnMut() -> bool,
    ) -> Result<(), SyncRuntimeError> {
        self.send_all_with_cancel(session, peer_id, messages, should_cancel)?;
        let emissions = self.announcement_outboxes.take_peer_emissions(peer_id)?;
        send_peer_emissions(
            &self.network,
            session,
            peer_id,
            self.config.network.magic(),
            emissions,
            should_cancel,
        )
    }
}

pub(super) fn send_peer_emissions<Sess, Store, View>(
    network: &ManagedNetworkHandle<Store, View>,
    session: &mut Sess,
    peer_id: PeerId,
    network_magic: NetworkMagic,
    emissions: Vec<PeerEmission>,
    should_cancel: &mut impl FnMut() -> bool,
) -> Result<(), SyncRuntimeError>
where
    Sess: SyncPeerSession,
    Store: crate::ChainstateStore,
    View: open_bitcoin_core::chainstate::CoinsView,
{
    let mut buffers = VecDeque::new();
    let preparation = emissions.iter().try_for_each(|emission| {
        if emission.peer_id() != peer_id {
            return Err(SyncRuntimeError::Network {
                message: "announcement outbox target does not match connected session".to_string(),
            });
        }
        let buffer = network
            .encode_pending_network_message(peer_id, emission.message())
            .map_err(super::filter_serving::transport_error)?;
        buffers.push_back(buffer);
        Ok(())
    });
    if let Err(error) = preparation {
        drop(buffers);
        return combined_error(error, abort_emissions(network, emissions));
    }
    let mut emissions = VecDeque::from(emissions);
    while let Some(emission) = emissions.pop_front() {
        let (_target_peer_id, message, capability) = emission.into_parts();
        drop(message);
        if should_cancel() {
            drop(buffers);
            return abort_current_and_suffix(network, capability, emissions);
        }
        let Some(buffer) = buffers.pop_front() else {
            drop(buffers);
            return combined_error(
                SyncRuntimeError::Network {
                    message: "announcement encoded batch is incomplete".to_string(),
                },
                abort_current_and_suffix(network, capability, emissions),
            );
        };
        if let Err(write_error) = session.send_encoded(buffer.bytes(), network_magic) {
            drop(buffer);
            drop(buffers);
            return combined_error(
                write_error,
                abort_current_and_suffix(network, capability, emissions),
            );
        }
        drop(buffer);
        let receipt = capability.acknowledge_write();
        if let Err(completion_error) = network.complete_peer_emission(receipt) {
            drop(buffers);
            return combined_error(completion_error.into(), abort_emissions(network, emissions));
        }
    }
    Ok(())
}

fn combined_error(
    error: SyncRuntimeError,
    cleanup: Result<(), SyncRuntimeError>,
) -> Result<(), SyncRuntimeError> {
    match cleanup {
        Ok(()) => Err(error),
        Err(cleanup) => Err(SyncRuntimeError::Network {
            message: format!("{error}; announcement abort cleanup also failed: {cleanup}"),
        }),
    }
}

pub(super) fn abort_emissions<S, V>(
    network: &ManagedNetworkHandle<S, V>,
    emissions: impl IntoIterator<Item = PeerEmission>,
) -> Result<(), SyncRuntimeError>
where
    S: crate::ChainstateStore,
    V: open_bitcoin_core::chainstate::CoinsView,
{
    abort_capabilities(
        network,
        emissions
            .into_iter()
            .map(|emission| emission.into_parts().2),
    )
}

pub(super) fn abort_outcomes<S, V>(
    network: &ManagedNetworkHandle<S, V>,
    outcomes: Vec<AnnouncementPreparationOutcome>,
) -> Result<(), SyncRuntimeError>
where
    S: crate::ChainstateStore,
    V: open_bitcoin_core::chainstate::CoinsView,
{
    abort_emissions(
        network,
        outcomes.into_iter().filter_map(|outcome| match outcome {
            AnnouncementPreparationOutcome::Ready(emission) => Some(*emission),
            _ => None,
        }),
    )
}

pub(super) fn surface_abort_cleanup_error(
    result: Result<(), SyncRuntimeError>,
    cleanup: Result<(), SyncRuntimeError>,
) -> (Result<(), SyncRuntimeError>, Result<(), SyncRuntimeError>) {
    match (result, cleanup) {
        (Err(error), Err(abort_error)) => (
            Err(SyncRuntimeError::Network {
                message: format!("{error}; announcement abort cleanup also failed: {abort_error}"),
            }),
            Ok(()),
        ),
        pair => pair,
    }
}

fn abort_current_and_suffix<S, V>(
    network: &ManagedNetworkHandle<S, V>,
    current: PeerEmissionWriteCapability,
    suffix: VecDeque<PeerEmission>,
) -> Result<(), SyncRuntimeError>
where
    S: crate::ChainstateStore,
    V: open_bitcoin_core::chainstate::CoinsView,
{
    abort_capabilities(
        network,
        std::iter::once(current).chain(suffix.into_iter().map(|emission| emission.into_parts().2)),
    )
}

fn abort_capabilities<S, V>(
    network: &ManagedNetworkHandle<S, V>,
    capabilities: impl IntoIterator<Item = PeerEmissionWriteCapability>,
) -> Result<(), SyncRuntimeError>
where
    S: crate::ChainstateStore,
    V: open_bitcoin_core::chainstate::CoinsView,
{
    let mut maybe_first_error = None;
    for capability in capabilities {
        let abort_result = network
            .abort_peer_emission(capability)
            .map_err(SyncRuntimeError::from)
            .and_then(|abort| match abort {
                EffectAbort::Aborted => Ok(()),
                EffectAbort::AlreadyCompleted | EffectAbort::NotPending => {
                    Err(SyncRuntimeError::Network {
                        message: format!(
                            "prepared peer emission abort returned unexpected classification {abort:?}"
                        ),
                    })
                }
            });
        if maybe_first_error.is_none() {
            maybe_first_error = abort_result.err();
        }
    }
    match maybe_first_error {
        Some(error) => Err(error),
        None => Ok(()),
    }
}
