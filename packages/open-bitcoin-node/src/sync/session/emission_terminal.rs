// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use std::collections::VecDeque;

use open_bitcoin_core::primitives::NetworkMagic;
use open_bitcoin_network::PeerId;

use crate::{
    ManagedNetworkHandle,
    network::{
        AnnouncementPreparationOutcome, EffectAbort, PeerEmission, PeerEmissionWriteCapability,
    },
};

use super::super::{SyncPeerSession, SyncRuntimeError};

pub(super) fn send_peer_emissions<S: SyncPeerSession>(
    network: &ManagedNetworkHandle,
    session: &mut S,
    peer_id: PeerId,
    network_magic: NetworkMagic,
    emissions: Vec<PeerEmission>,
) -> Result<(), SyncRuntimeError> {
    let mut emissions = VecDeque::from(emissions);
    while let Some(emission) = emissions.pop_front() {
        let (target_peer_id, message, capability) = emission.into_parts();
        if target_peer_id != peer_id {
            abort_current_and_suffix(network, capability, emissions)?;
            return Err(SyncRuntimeError::Network {
                message: "announcement outbox target does not match connected session".to_string(),
            });
        }
        if let Err(write_error) = session.send(&message, network_magic) {
            abort_current_and_suffix(network, capability, emissions)?;
            return Err(write_error);
        }
        let receipt = capability.acknowledge_write();
        if let Err(completion_error) = network.complete_peer_emission(receipt) {
            abort_emissions(network, emissions)?;
            return Err(completion_error.into());
        }
    }
    Ok(())
}

pub(super) fn abort_emissions(
    network: &ManagedNetworkHandle,
    emissions: impl IntoIterator<Item = PeerEmission>,
) -> Result<(), SyncRuntimeError> {
    abort_capabilities(
        network,
        emissions
            .into_iter()
            .map(|emission| emission.into_parts().2),
    )
}

pub(super) fn abort_outcomes(
    network: &ManagedNetworkHandle,
    outcomes: Vec<AnnouncementPreparationOutcome>,
) -> Result<(), SyncRuntimeError> {
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

fn abort_current_and_suffix(
    network: &ManagedNetworkHandle,
    current: PeerEmissionWriteCapability,
    suffix: VecDeque<PeerEmission>,
) -> Result<(), SyncRuntimeError> {
    abort_capabilities(
        network,
        std::iter::once(current).chain(suffix.into_iter().map(|emission| emission.into_parts().2)),
    )
}

fn abort_capabilities(
    network: &ManagedNetworkHandle,
    capabilities: impl IntoIterator<Item = PeerEmissionWriteCapability>,
) -> Result<(), SyncRuntimeError> {
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
