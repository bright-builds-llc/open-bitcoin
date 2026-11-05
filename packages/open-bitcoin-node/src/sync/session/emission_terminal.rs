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

use super::super::block_reconcile::PeerRequests;
use super::super::{SyncPeerSession, SyncRuntimeError};
use super::AnnouncementOutboxNotification;

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

    #[cfg(test)]
    pub(in crate::sync) fn send_all_for_peer_with_cancel<S: SyncPeerSession>(
        &mut self,
        session: &mut S,
        peer_id: PeerId,
        messages: &[WireNetworkMessage],
        should_cancel: &mut impl FnMut() -> bool,
    ) -> Result<(), SyncRuntimeError> {
        self.send_all_for_registration(session, peer_id, messages, should_cancel, None)
            .map(|_| ())
    }

    pub(super) fn send_all_for_registered_peer_with_cancel<S: SyncPeerSession>(
        &mut self,
        session: &mut S,
        peer_id: PeerId,
        messages: &[WireNetworkMessage],
        should_cancel: &mut impl FnMut() -> bool,
        registration: &AnnouncementOutboxNotification,
        requests: &PeerRequests,
    ) -> Result<bool, SyncRuntimeError> {
        self.send_all_for_registration(
            session,
            peer_id,
            messages,
            should_cancel,
            Some((registration, requests)),
        )
    }

    fn send_all_for_registration<S: SyncPeerSession>(
        &mut self,
        session: &mut S,
        peer_id: PeerId,
        messages: &[WireNetworkMessage],
        should_cancel: &mut impl FnMut() -> bool,
        maybe_registration: Option<(&AnnouncementOutboxNotification, &PeerRequests)>,
    ) -> Result<bool, SyncRuntimeError> {
        self.send_all_with_cancel(session, peer_id, messages, should_cancel)?;
        let emissions = match maybe_registration {
            Some((registration, requests)) => {
                let Some(emissions) = requests.with_outbox(&self.network, |owned_peer| {
                    self.announcement_outboxes.take_registration(
                        owned_peer,
                        registration,
                        false,
                        false,
                    )
                })?
                else {
                    return Ok(false);
                };
                emissions?
            }
            None => self.announcement_outboxes.take_peer_emissions(peer_id)?,
        };
        send_peer_emissions(
            &self.network,
            session,
            peer_id,
            self.config.network.magic(),
            emissions,
            should_cancel,
        )?;
        Ok(true)
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

#[cfg(test)]
mod tests {
    use super::super::AnnouncementOutboxRegistry;
    use super::*;
    use crate::sync::{DurableSyncRuntime, SyncNetwork, SyncRuntimeConfig};
    use crate::{FjallNodeStore, chainstate::BasicFilterStartupMode};
    use open_bitcoin_core::{consensus::CanonicalChain, primitives::Block};
    use open_bitcoin_network::VersionMessage;
    use std::{
        cell::Cell,
        io::{Read, Write},
        net::{TcpListener, TcpStream},
        path::PathBuf,
        rc::Rc,
    };

    fn runtime() -> (PathBuf, DurableSyncRuntime, Block) {
        let mut nonce = [0; 16];
        getrandom::fill(&mut nonce).expect("owned nonce");
        let path = std::fs::canonicalize(std::env::temp_dir())
            .expect("physical temp parent")
            .join(format!(
                "phase167-owned-outbox-{:x}",
                u128::from_le_bytes(nonce)
            ));
        let config = SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            ..Default::default()
        };
        let opened = FjallNodeStore::open_canonical_genesis(&path, CanonicalChain::Regtest)
            .expect("proved genesis");
        let runtime = DurableSyncRuntime::open_canonical_genesis_configured(
            opened,
            config,
            BasicFilterStartupMode::Disabled,
        )
        .expect("proved owner");
        let genesis = CanonicalChain::Regtest.spec().genesis_block().clone();
        (path, runtime, genesis)
    }
    fn handshake(handle: &crate::DurableNetworkHandle, peer: PeerId) {
        for message in [
            WireNetworkMessage::Version(VersionMessage {
                nonce: 1,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
        ] {
            handle
                .receive_sync_message(
                    peer,
                    message,
                    0,
                    open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
                    Default::default(),
                )
                .expect("real handshake");
        }
    }
    fn prepare(
        handle: &crate::DurableNetworkHandle,
        registry: &AnnouncementOutboxRegistry,
        block: &Block,
    ) -> Vec<WireNetworkMessage> {
        let mut expected = Vec::new();
        for headers in [false, true] {
            if headers {
                handle
                    .receive_sync_message(
                        1,
                        WireNetworkMessage::SendHeaders,
                        0,
                        open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
                        Default::default(),
                    )
                    .expect("real header preference");
            }
            let outcomes = handle
                .prepare_block_announcements(block, &registry.snapshots().expect("actual queue"))
                .expect("actual emission producer");
            expected.extend(outcomes.iter().filter_map(|outcome| match outcome {
                AnnouncementPreparationOutcome::Ready(emission) => Some(emission.message().clone()),
                _ => None,
            }));
            registry
                .enqueue_prepared(handle, outcomes)
                .expect("actual enqueue");
        }
        assert_eq!(expected.len(), 2);
        assert_ne!(expected[0], expected[1]);
        expected
    }
    fn credits(handle: &crate::DurableNetworkHandle) -> u64 {
        let crate::status::FieldAvailability::Available(counters) = handle
            .block_relay_evidence_status()
            .expect("real credit facts")
            .announcement
        else {
            panic!("actual available counters");
        };
        counters.compact_announced_count
            + counters.compact_inventory_fallback_count
            + counters.compact_headers_fallback_count
    }
    struct ReplacingWriter {
        stream: TcpStream,
        handle: crate::DurableNetworkHandle,
        registry: AnnouncementOutboxRegistry,
        block: Block,
        route: usize,
        admission: usize,
        replaced: bool,
        cancel: Rc<Cell<bool>>,
        expected: Vec<WireNetworkMessage>,
        maybe_registration: Option<AnnouncementOutboxNotification>,
    }
    impl SyncPeerSession for ReplacingWriter {
        fn send(
            &mut self,
            message: &WireNetworkMessage,
            magic: NetworkMagic,
        ) -> Result<(), SyncRuntimeError> {
            self.send_encoded(&message.encode_wire(magic)?, magic)
        }
        fn send_encoded(
            &mut self,
            bytes: &[u8],
            _magic: NetworkMagic,
        ) -> Result<(), SyncRuntimeError> {
            if self.route != 3 && !self.replaced {
                if self.route < 3 {
                    self.registry.unregister_peer(&self.handle, 1)?;
                    self.maybe_registration = Some(self.registry.register_peer(1)?);
                } else {
                    self.handle.disconnect_peer(1)?;
                    match self.admission {
                        0 => {
                            self.handle.connect_outbound_peer(1, 1)?;
                        }
                        1 => self.handle.add_inbound_peer(1)?,
                        _ => assert!(matches!(
                            self.handle.admit_inbound_peer(
                                open_bitcoin_network::InboundAdmissionRequest::ordinary(
                                    1,
                                    "127.0.0.1:18448"
                                )
                            )?,
                            open_bitcoin_network::InboundAdmissionDecision::Admit(_)
                        )),
                    }
                    handshake(&self.handle, 1);
                }
                self.expected = prepare(&self.handle, &self.registry, &self.block);
                if self.route >= 7 {
                    self.handle.disconnect_peer(1)?;
                }
                self.replaced = true;
                let outcome = if self.route < 3 {
                    self.route
                } else {
                    (self.route - 4) % 3
                };
                if outcome == 1 {
                    self.stream
                        .shutdown(std::net::Shutdown::Write)
                        .map_err(|error| SyncRuntimeError::Io {
                            peer: "owned-loopback".to_owned(),
                            message: error.to_string(),
                        })?;
                }
                if outcome == 2 {
                    self.cancel.set(true);
                }
            }
            self.stream
                .write_all(bytes)
                .map_err(|error| SyncRuntimeError::Io {
                    peer: "owned-loopback".to_owned(),
                    message: error.to_string(),
                })
        }
        fn receive(
            &mut self,
            _magic: NetworkMagic,
        ) -> Result<crate::sync::SyncPeerReceiveOutcome, SyncRuntimeError> {
            Ok(crate::sync::SyncPeerReceiveOutcome::Closed)
        }
    }

    #[test]
    fn phase167_peer_session_replace_during_real_send_preserves_replacement_on_success_error_and_cancel()
     {
        for admission in 0..3 {
            for route in 0..10 {
                // Arrange
                let (path, mut runtime, block) = runtime();
                let (token, outbound) = runtime.network.connect_sync_peer(1, 0).expect("real peer");
                outbound.expect("real version");
                let requests = PeerRequests::new(token);
                handshake(&runtime.network, 1);
                let owner = runtime
                    .announcement_outboxes
                    .register_peer(1)
                    .expect("actual old owner");
                let listener = TcpListener::bind("127.0.0.1:0").expect("owned loopback");
                let stream = TcpStream::connect(listener.local_addr().expect("endpoint"))
                    .expect("real socket");
                let (mut reader, _) = listener.accept().expect("accepted");
                let cancel = Rc::new(Cell::new(false));
                let mut session = ReplacingWriter {
                    stream,
                    handle: runtime.network.clone(),
                    registry: runtime.announcement_outboxes.clone(),
                    block,
                    route,
                    admission,
                    replaced: false,
                    cancel: Rc::clone(&cancel),
                    expected: Vec::new(),
                    maybe_registration: None,
                };
                if route == 3 {
                    session.expected = prepare(&session.handle, &session.registry, &session.block);
                }
                let credit_before = credits(&runtime.network);
                let ping = WireNetworkMessage::Ping { nonce: 29 };
                // Act
                let result = runtime.send_all_for_registered_peer_with_cancel(
                    &mut session,
                    1,
                    std::slice::from_ref(&ping),
                    &mut || cancel.get(),
                    &owner,
                    &requests,
                );
                // Assert
                let outcome = if route < 4 { route } else { (route - 4) % 3 };
                if route != 3 {
                    let error = result.expect_err("actual replacement or actual socket error");
                    if outcome == 1 {
                        assert!(matches!(error, SyncRuntimeError::Io { .. }));
                    } else if route < 3 {
                        assert!(error.to_string().contains("registration was replaced"));
                    } else {
                        assert!(error.to_string().contains("sync peer lifetime"));
                    }
                    let queued = runtime
                        .announcement_outboxes
                        .take_peer_emissions(1)
                        .expect("replacement exact queue");
                    assert_eq!(
                        queued
                            .iter()
                            .map(|emission| emission.message().clone())
                            .collect::<Vec<_>>(),
                        session.expected
                    );
                    assert_eq!(credits(&runtime.network), credit_before);
                    let mut registration = session.maybe_registration.take().unwrap_or(owner);
                    let mut ready = std::pin::pin!(registration.notified());
                    assert!(
                        std::future::Future::poll(
                            ready.as_mut(),
                            &mut std::task::Context::from_waker(std::task::Waker::noop())
                        )
                        .is_ready()
                    );
                    for emission in queued {
                        runtime
                            .network
                            .abort_peer_emission(emission.into_parts().2)
                            .expect("replacement owner abort");
                    }
                } else {
                    assert!(result.expect("owned positive sender"));
                    assert_eq!(credits(&runtime.network), credit_before + 2);
                }
                let expected = if outcome == 1 {
                    Vec::new()
                } else if route == 3 {
                    std::iter::once(ping)
                        .chain(session.expected.clone())
                        .flat_map(|message| {
                            message
                                .encode_wire(runtime.config.network.magic())
                                .expect("wire")
                        })
                        .collect()
                } else {
                    ping.encode_wire(runtime.config.network.magic())
                        .expect("wire")
                };
                drop(session);
                let mut wire = Vec::new();
                reader
                    .read_to_end(&mut wire)
                    .expect("actual physical bytes");
                assert_eq!(wire, expected);
                runtime
                    .announcement_outboxes
                    .unregister_peer(&runtime.network, 1)
                    .expect("actual owner cleanup");
                drop(runtime);
                std::fs::remove_dir_all(path).expect("owned cleanup");
            }
        }
    }
}
