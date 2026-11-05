// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

use super::*;
use crate::network::{
    ManagedInboundResponsePlanItem as Item,
    filter_transport::{
        BasicFilterServeUnavailable, FilterTransportDisposition, PreparedFilterResponse,
    },
};
use crate::sync::{DurableSyncRuntime, SyncNetwork, SyncRuntimeConfig};
use crate::{FjallNodeStore, chainstate::BasicFilterStartupMode};
use open_bitcoin_network::{GetCFCheckpt, GetCFHeaders, GetCFilters, VersionMessage};
use std::{
    io::{Read, Write},
    path::PathBuf,
};

struct BasicFixture {
    path: PathBuf,
    maybe_runtime: Option<DurableSyncRuntime>,
    params: ConsensusParams,
    hash: BlockHash,
}

impl BasicFixture {
    fn new() -> Self {
        let mut nonce = [0_u8; 16];
        getrandom::fill(&mut nonce).expect("owned scratch nonce");
        let path = std::fs::canonicalize(std::env::temp_dir())
            .expect("physical temp parent")
            .join(format!(
                "phase167-basic-lifetime-{:x}",
                u128::from_le_bytes(nonce)
            ));
        let opened = FjallNodeStore::open_canonical_genesis(&path, CanonicalChain::Regtest)
            .expect("proved genesis");
        let params = open_bitcoin_core::chainstate::engine::canonical_genesis::consensus_params(
            opened.genesis(),
        );
        let hash = opened.position().block_hash;
        let runtime = DurableSyncRuntime::open_canonical_genesis_configured(
            opened,
            SyncRuntimeConfig {
                network: SyncNetwork::Regtest,
                ..Default::default()
            },
            BasicFilterStartupMode::Enabled,
        )
        .expect("real BASIC provider");
        runtime
            .network_handle()
            .configure_basic_peer_serving(true)
            .expect("explicit serving");
        Self {
            path,
            maybe_runtime: Some(runtime),
            params,
            hash,
        }
    }
    fn runtime(&self) -> &DurableSyncRuntime {
        self.maybe_runtime.as_ref().expect("owned live runtime")
    }
    fn handle(&self) -> crate::DurableNetworkHandle {
        self.runtime().network_handle()
    }
    fn peer(&self, id: PeerId) -> SyncPeerLifetime {
        let handle = self.handle();
        let (token, outbound) = handle.connect_sync_peer(id, 0).expect("real admission");
        outbound.expect("real version");
        for message in [
            WireNetworkMessage::Version(VersionMessage::default()),
            WireNetworkMessage::Verack,
        ] {
            handle
                .receive_sync_message(id, message, 0, ScriptVerifyFlags::P2SH, self.params)
                .expect("real handshake");
        }
        token
    }
    fn raw(&self, token: &SyncPeerLifetime, message: WireNetworkMessage) -> RawSyncPeerDispatch {
        self.handle()
            .produce_sync_peer_dispatch(token, message, 0, ScriptVerifyFlags::P2SH, self.params)
            .expect("actual raw producer")
    }
    fn flight_count(&self) -> u64 {
        let state = self
            .runtime()
            .durable_sync_state(crate::status::SyncLifecycleState::Stopped, None, 1)
            .expect("public resource facts");
        let crate::status::FieldAvailability::Available(pressure) = state.sync.resource_pressure
        else {
            panic!("available actual runtime pressure");
        };
        pressure.blocks_in_flight
    }
}
impl Drop for BasicFixture {
    fn drop(&mut self) {
        drop(self.maybe_runtime.take());
        std::fs::remove_dir_all(&self.path).expect("owned scratch cleanup after all handles drop");
    }
}

fn request(family: u8, filter_type: u8, start: u32, stop: BlockHash) -> WireNetworkMessage {
    match family {
        0 => WireNetworkMessage::GetCFilters(GetCFilters {
            filter_type,
            start_height: start,
            stop_hash: stop,
        }),
        1 => WireNetworkMessage::GetCFHeaders(GetCFHeaders {
            filter_type,
            start_height: start,
            stop_hash: stop,
        }),
        _ => WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
            filter_type,
            stop_hash: stop,
        }),
    }
}
fn response(result: ManagedSyncMessageResult) -> Box<PreparedFilterResponse> {
    result
        .inbound_response_plan
        .into_iter()
        .find_map(|item| {
            let Item::PreparedFilterServe(response) = item else {
                return None;
            };
            Some(response)
        })
        .expect("actual prepared response")
}
fn completed_frames(
    handle: &crate::DurableNetworkHandle,
    mut response: PreparedFilterResponse,
) -> Vec<Vec<u8>> {
    let mut frames = Vec::new();
    while let Some(frame) = response.next_frame().expect("actual frame handoff") {
        frames.push(frame.bytes().to_vec());
        std::io::sink()
            .write_all(frame.bytes())
            .expect("actual complete sink write");
        handle
            .complete_basic_filter_frame(frame.acknowledge_write())
            .expect("actual completed receipt");
    }
    frames
}

#[test]
fn phase167_peer_session_basic_bound_and_generic_genesis_families_have_exact_bytes() {
    for family in 0..3 {
        // Arrange
        let fixture = BasicFixture::new();
        let handle = fixture.handle();
        let token = fixture.peer(1);
        let _generic = fixture.peer(2);
        let message = request(family, 0, 0, fixture.hash);
        // Act
        let bound = handle
            .begin_sync_peer_dispatch(
                &token,
                message.clone(),
                0,
                ScriptVerifyFlags::P2SH,
                fixture.params,
            )
            .expect("raw authorized")
            .resolve()
            .into_parts()
            .expect("BASIC authorized")
            .into_parts()
            .1
            .expect("prepared bound family");
        let generic = handle
            .receive_sync_message(2, message, 0, ScriptVerifyFlags::P2SH, fixture.params)
            .expect("unchanged public generic family");
        let bound = completed_frames(&handle, *response(bound));
        let generic = completed_frames(&handle, *response(generic));
        // Assert
        assert_eq!(bound, generic);
        assert_eq!(bound.len(), 1);
        let record = fixture
            .runtime()
            .store()
            .load_basic_filter_record(fixture.hash)
            .expect("real row")
            .expect("genesis row");
        let expected = match family {
            0 => WireNetworkMessage::CFilter(open_bitcoin_network::CFilter {
                filter_type: 0,
                block_hash: fixture.hash,
                filter: record.encoded_bytes().to_vec(),
            }),
            1 => WireNetworkMessage::CFHeaders(open_bitcoin_network::CFHeaders {
                filter_type: 0,
                stop_hash: fixture.hash,
                previous_filter_header: Default::default(),
                filter_hashes: vec![record.identity().filter_hash().into()],
            }),
            _ => WireNetworkMessage::CFCheckpt(open_bitcoin_network::CFCheckpt {
                filter_type: 0,
                stop_hash: fixture.hash,
                filter_headers: Vec::new(),
            }),
        };
        assert_eq!(
            open_bitcoin_network::ParsedNetworkMessage::decode_wire(&bound[0])
                .expect("actual framed bytes")
                .message,
            expected
        );
        assert_eq!(
            handle
                .basic_filter_transport_snapshot()
                .expect("settled")
                .requests,
            0
        );
        assert_eq!(fixture.flight_count(), 0);
    }
}

#[test]
fn phase167_peer_session_basic_old_actual_intent_cannot_prepare_or_disconnect_replacement() {
    for route in 0..6 {
        // Arrange
        let fixture = BasicFixture::new();
        let handle = fixture.handle();
        let token = fixture.peer(1);
        let old_request = BlockHash::from_byte_array([19; 32]);
        handle
            .request_missing_blocks(1, &[old_request])
            .expect("actual old membership");
        let message = request(
            if route == 5 { 0 } else { route.min(2) },
            if route == 3 { 1 } else { 0 },
            if route == 5 { 1 } else { 0 },
            if route == 4 {
                BlockHash::from_byte_array([20; 32])
            } else {
                fixture.hash
            },
        );
        let raw = fixture.raw(&token, message);
        handle
            .clone()
            .disconnect_peer(1)
            .expect("actual old removal");
        let _replacement = fixture.peer(1);
        handle
            .request_missing_blocks(1, &[BlockHash::from_byte_array([21; 32])])
            .expect("actual replacement request");
        let before = format!(
            "{:?}",
            handle
                .peer_manager_snapshot()
                .expect("complete replacement state")
        );
        let transport = handle
            .basic_filter_transport_snapshot()
            .expect("before transport");
        assert_eq!(fixture.flight_count(), 0);
        // Act
        let denied = handle
            .continue_sync_peer_dispatch(&token, raw, fixture.params)
            .into_parts()
            .expect_err("old actual intent is denied");
        // Assert
        let (observed, error, maybe_primary) = denied.into_parts();
        assert!(
            matches!(observed, SyncPeerObservation::Present { requests, .. } if requests.contains(&old_request))
        );
        assert!(error.to_string().contains("Replaced"));
        assert!(maybe_primary.is_none());
        assert_eq!(
            format!(
                "{:?}",
                handle.peer_manager_snapshot().expect("replacement exact")
            ),
            before
        );
        assert_eq!(
            handle
                .basic_filter_transport_snapshot()
                .expect("transport exact"),
            transport
        );
        assert_eq!(fixture.flight_count(), 0);
    }
}

#[test]
fn phase167_peer_session_basic_semantic_errors_remove_only_current_original_once() {
    for route in 0..4 {
        // Arrange
        let fixture = BasicFixture::new();
        let handle = fixture.handle();
        if route == 1 {
            handle
                .configure_basic_peer_serving(false)
                .expect("actual capability disabled before admission");
        }
        let token = fixture.peer(1);
        let before = fixture
            .runtime()
            .store()
            .basic_operator_namespaces_for_test();
        let message = request(
            0,
            if route == 0 { 1 } else { 0 },
            if route == 3 { 1 } else { 0 },
            if route == 2 {
                BlockHash::from_byte_array([22; 32])
            } else {
                fixture.hash
            },
        );
        // Act
        let authorized = handle
            .continue_sync_peer_dispatch(&token, fixture.raw(&token, message), fixture.params)
            .into_parts()
            .expect("actual semantic outcome is authorized");
        let error = authorized
            .into_parts()
            .1
            .expect_err("actual invalid BASIC request");
        // Assert
        assert!(error.to_string().contains("invalid BASIC peer request"));
        assert!(
            handle
                .peer_manager_snapshot()
                .expect("after actual disconnect")
                .peer_state(1)
                .is_none()
        );
        assert!(matches!(
            handle.finish_sync_peer(&token),
            Ok(SyncPeerObservation::Absent)
        ));
        assert_eq!(
            fixture
                .runtime()
                .store()
                .basic_operator_namespaces_for_test(),
            before
        );
        assert_eq!(
            handle
                .basic_filter_transport_snapshot()
                .expect("no reserved prefix")
                .requests,
            0
        );
    }
}

#[test]
fn phase167_peer_session_basic_prepared_old_handed_frame_keeps_charge_but_not_live_credit() {
    // Arrange
    let fixture = BasicFixture::new();
    let handle = fixture.handle();
    let token = fixture.peer(1);
    let result = handle
        .begin_sync_peer_dispatch(
            &token,
            request(0, 0, 0, fixture.hash),
            0,
            ScriptVerifyFlags::P2SH,
            fixture.params,
        )
        .expect("raw authorized")
        .resolve()
        .into_parts()
        .expect("BASIC authorized")
        .into_parts()
        .1
        .expect("actual response");
    let mut response = response(result);
    let frame = response
        .next_frame()
        .expect("handoff")
        .expect("real genesis frame");
    let bytes = frame.bytes().to_vec();
    handle.disconnect_peer(1).expect("actual old removal");
    let _replacement = fixture.peer(1);
    // Act
    assert!(
        !handle
            .basic_filter_response_is_current(&response)
            .expect("old lease cannot revive")
    );
    assert_eq!(
        handle
            .basic_filter_transport_snapshot()
            .expect("handed charge")
            .active_frame_bytes,
        bytes.len() as u64
    );
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("owned loopback");
    let mut writer = std::net::TcpStream::connect(listener.local_addr().expect("bound endpoint"))
        .expect("actual transport");
    let (mut reader, _) = listener.accept().expect("actual peer");
    writer
        .write_all(frame.bytes())
        .expect("actual complete socket frame");
    let mut received = vec![0; bytes.len()];
    reader.read_exact(&mut received).expect("actual full bytes");
    let completed = handle
        .complete_basic_filter_frame(frame.acknowledge_write())
        .expect("physical receipt");
    // Assert
    assert_eq!(received, bytes);
    assert_eq!(
        completed.disposition,
        FilterTransportDisposition::PhysicalPrefixInvalidated
    );
    assert_eq!(completed.progress.messages, 1);
    assert_eq!(completed.progress.bytes, bytes.len() as u64);
    assert_eq!(
        handle
            .basic_filter_transport_snapshot()
            .expect("physical charge settled")
            .active_frame_bytes,
        0
    );
    assert!(
        handle
            .peer_manager_snapshot()
            .expect("replacement survives")
            .peer_state(1)
            .is_some()
    );
}

#[test]
fn phase167_peer_session_basic_actual_backend_failure_is_authorized_without_prepared_prefix() {
    // Arrange
    let fixture = BasicFixture::new();
    let handle = fixture.handle();
    let token = fixture.peer(1);
    let raw = fixture.raw(&token, request(0, 0, 0, fixture.hash));
    fixture.runtime().store().set_basic_filter_fault(
        crate::storage::fjall_store::filters::FilterPublicationFault::BeforeQueryRead,
    );
    // Act
    let (observed, result) = handle
        .continue_sync_peer_dispatch(&token, raw, fixture.params)
        .into_parts()
        .expect("real read failure is not a lifetime denial")
        .into_parts();
    let result = result.expect("existing structured unavailable contract");
    // Assert
    assert!(matches!(observed, SyncPeerObservation::Present { .. }));
    assert!(matches!(
        &result.inbound_response_plan[..],
        [Item::FilterServeUnavailable(
            BasicFilterServeUnavailable::StorageFailure
        )]
    ));
    assert_eq!(
        handle
            .basic_filter_transport_snapshot()
            .expect("no prefix exposure")
            .requests,
        0
    );
}

#[test]
fn phase167_peer_session_basic_actual_authority_or_registry_poison_denies_without_forged_intent() {
    for registry_poison in [false, true] {
        // Arrange
        let fixture = BasicFixture::new();
        let handle = fixture.handle();
        let token = fixture.peer(1);
        let raw = fixture.raw(&token, request(0, 0, 0, fixture.hash));
        if registry_poison {
            let registry = Arc::clone(&handle.peer_sessions);
            assert!(
                std::thread::spawn(move || {
                    let _guard = registry.lock().expect("real registry");
                    panic!("intentional registry poison");
                })
                .join()
                .is_err()
            );
        } else {
            handle.poison_for_test();
        }
        // Act
        let denied = handle
            .continue_sync_peer_dispatch(&token, raw, fixture.params)
            .into_parts()
            .expect_err("actual authority denial");
        // Assert
        let (observed, _, maybe_primary) = denied.into_parts();
        assert!(matches!(observed, SyncPeerObservation::Present { .. }));
        assert!(maybe_primary.is_none());
        assert_eq!(
            handle.basic_filter_transport_snapshot(),
            if registry_poison {
                Ok(Default::default())
            } else {
                Err(crate::network::filter_transport::FilterTransportError::Poisoned)
            }
        );
        assert_eq!(
            handle.basic_filter_transport.raw_snapshot(),
            Default::default()
        );
    }
}

#[test]
fn phase167_peer_session_raw_compatibility_without_basic_is_not_continuation_denied() {
    // Arrange
    let fixture = BasicFixture::new();
    let handle = fixture.handle();
    let token = fixture.peer(1);
    // Act
    let (_, result) = handle
        .begin_sync_peer_dispatch(
            &token,
            WireNetworkMessage::Version(VersionMessage::default()),
            0,
            ScriptVerifyFlags::P2SH,
            fixture.params,
        )
        .expect("actual authorized raw error")
        .resolve()
        .into_parts()
        .expect("no spurious BASIC denial")
        .into_parts();
    // Assert
    assert!(matches!(
        crate::sync::SyncRuntimeError::from(result.expect_err("actual duplicate version")),
        crate::sync::SyncRuntimeError::PeerCompatibility { .. }
    ));
    assert!(matches!(
        handle.finish_sync_peer(&token),
        Ok(SyncPeerObservation::Absent)
    ));
}

#[test]
fn phase167_peer_session_duplicate_foreign_epoch_and_absence_are_distinct() {
    // Arrange
    let handle = memory_handle();
    let foreign = memory_handle();
    let (token, outbound) = handle.connect_sync_peer(1, 0).expect("original");
    outbound.expect("version");
    // Act / Assert
    assert!(handle.connect_outbound_peer(1, 0).is_err());
    assert!(handle.add_inbound_peer(1).is_err());
    assert!(matches!(
        handle.observe_sync_peer(&token),
        Ok(SyncPeerObservation::Present { .. })
    ));
    assert!(matches!(
        handle
            .admit_inbound_peer(InboundAdmissionRequest::ordinary(1, "127.0.0.1:18447"))
            .expect("real duplicate policy"),
        InboundAdmissionDecision::Reject(_)
    ));
    assert!(matches!(
        handle.observe_sync_peer(&token),
        Ok(SyncPeerObservation::Present { .. })
    ));
    assert!(foreign.observe_sync_peer(&token).is_err());
    handle.disconnect_peer(1).expect("original removed");
    assert!(matches!(
        handle.finish_sync_peer(&token),
        Ok(SyncPeerObservation::Absent)
    ));
    handle
        .mutate(|network| network.install_authority_incarnation())
        .expect("actual epoch transition");
    assert!(handle.finish_sync_peer(&token).is_err());
}

#[test]
fn phase167_peer_session_real_registry_poison_denies_before_manager_effects() {
    // Arrange
    let handle = memory_handle();
    let before = format!("{:?}", handle.peer_manager_snapshot().expect("before"));
    let registry = Arc::clone(&handle.peer_sessions);
    let poisoned = std::thread::spawn(move || {
        let _guard = registry.lock().expect("actual registry lock");
        panic!("intentional private registry poison");
    })
    .join();
    assert!(poisoned.is_err());
    // Act
    let result = handle.connect_outbound_peer(1, 0);
    // Assert
    assert!(result.is_err());
    assert_eq!(
        format!(
            "{:?}",
            handle
                .peer_manager_snapshot()
                .expect("unchanged actual manager")
        ),
        before
    );
}
