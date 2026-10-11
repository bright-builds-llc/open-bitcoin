// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp

use super::*;

#[path = "tests/families.rs"]
mod families;

use crate::network::{
    ManagedInboundResponsePlanItem as PlanItem,
    filter_transport::{
        BasicFilterPrepareOutcome as Prepare, BasicFilterServeUnavailable as Unavailable,
        FilterTransportSnapshot,
    },
};
use open_bitcoin_network::{GetCFCheckpt, GetCFHeaders, GetCFilters};

#[test]
fn phase161_disable_validation_genuine_managed_failure_hides_old_growth() {
    use crate::status::FieldAvailability;
    use crate::storage::fjall_store::filters::query::QueryRecordFault;
    // Arrange
    let fixture = PeerFilterFixture::new(16);
    let runtime = &fixture.runtime;
    assert!(runtime.store().basic_filter_retention().is_ok());
    let old = runtime
        .store()
        .with_basic_filter_query(|reader| reader.integrity_epoch())
        .expect("healthy epoch");
    let scans = runtime.store().basic_filter_validation_scans_for_test();
    runtime
        .store()
        .inject_query_record_fault_for_test(fixture.hash(4), QueryRecordFault::CorruptTarget)
        .expect("undiscovered forest fault");
    // Act
    assert!(runtime.network.disable_basic_filter_index().is_err());
    // Assert
    let status = runtime.network.operator_snapshot().expect("snapshot");
    assert!(matches!(
        status.basic_index().retention,
        FieldAvailability::Unavailable { .. }
    ));
    assert!(matches!(
        status.basic_index().retained_availability,
        FieldAvailability::Unavailable { .. }
    ));
    assert!(
        runtime
            .store()
            .with_basic_filter_query(|reader| reader.integrity_epoch())
            .is_err()
    );
    assert!(
        !runtime
            .store()
            .basic_peer_epoch_is_current(&old)
            .expect("old epoch")
    );
    assert_eq!(
        runtime.store().basic_filter_validation_scans_for_test() - scans,
        1
    );
    drop(old);
    fixture.cleanup();
}

#[test]
fn phase161_read_fault_callers_managed_peer_revokes_prepared_receipt_and_suffix() {
    use crate::network::filter_transport::FilterTransportDisposition;
    use crate::status::{
        BasicIndexFailureCategory as Failure, BasicIndexOperationalState as State,
        FieldAvailability as Field,
    };
    use crate::storage::fjall_store::filters::{FilterPublicationFault, query::QueryRecordFault};
    for kind in 0..4 {
        // Arrange
        let fixture = PeerFilterFixture::new(16);
        let runtime = &fixture.runtime;
        runtime
            .network
            .connect_outbound_peer(2, 0)
            .expect("independent request slot");
        super::complete_peer_handshake(runtime, 2);
        let before = runtime
            .network
            .operator_snapshot()
            .expect("snapshot")
            .basic_index()
            .clone();
        let result = managed_receive(runtime, 1, request_filters(1, fixture.hash(3)), false);
        let PlanItem::PreparedFilterServe(mut prepared) = result
            .inbound_response_plan
            .into_iter()
            .next()
            .expect("prepared")
        else {
            panic!("prepared response");
        };
        let frame = prepared.next_frame().expect("frame").expect("present");
        let frame_bytes = frame.bytes().len() as u64;
        if kind == 3 {
            runtime
                .store()
                .set_basic_filter_fault(FilterPublicationFault::BeforeQueryRead);
        } else {
            let fault = match kind {
                0 => QueryRecordFault::CorruptTarget,
                1 => QueryRecordFault::CorruptParent,
                _ => QueryRecordFault::MissingParent,
            };
            runtime
                .store()
                .inject_query_record_fault_for_test(fixture.hash(8), fault)
                .expect("fault");
        }
        // Act
        let failure = managed_receive(runtime, 2, request_filters(8, fixture.hash(10)), false);
        assert_eq!(
            failure.inbound_response_plan,
            vec![PlanItem::FilterServeUnavailable(
                Unavailable::StorageFailure
            )]
        );
        // Assert
        let status = runtime
            .network
            .operator_snapshot()
            .expect("snapshot")
            .basic_index()
            .clone();
        assert_eq!(
            status.maybe_failure,
            Field::available(Some(if kind == 3 {
                Failure::Backend
            } else {
                Failure::IntegrityOrInvalidFacts
            }))
        );
        assert_eq!(status.operational_state, Field::available(State::Failed));
        assert_eq!(status.protection, before.protection);
        assert_eq!(status.initial_sync_complete, before.initial_sync_complete);
        let rows = runtime.store().basic_operator_namespaces_for_test();
        let scans = runtime.store().basic_filter_validation_scans_for_test();
        let reads = runtime.store().basic_filter_point_reads_for_test();
        for _ in 0..32 {
            assert_eq!(
                runtime
                    .network
                    .operator_snapshot()
                    .expect("poll")
                    .basic_index(),
                &status
            );
        }
        assert_eq!(runtime.store().basic_operator_namespaces_for_test(), rows);
        assert_eq!(
            runtime.store().basic_filter_validation_scans_for_test(),
            scans
        );
        assert_eq!(runtime.store().basic_filter_point_reads_for_test(), reads);
        assert!(
            !runtime
                .network
                .basic_filter_response_is_current(&prepared)
                .expect("revoked")
        );
        let held = runtime
            .network
            .basic_filter_transport_snapshot()
            .expect("held physical frame");
        // The handed frame remains charged until its physical receipt settles.
        assert_eq!(held.suffix_bytes, frame_bytes);
        assert_eq!(held.active_frame_bytes, frame_bytes);
        assert_eq!(prepared.progress().expect("no invented bytes").bytes, 0);
        let completed = runtime
            .network
            .complete_basic_filter_frame(frame.acknowledge_write())
            .expect("isolated physical receipt");
        assert_eq!(
            completed.disposition,
            FilterTransportDisposition::PhysicalPrefixInvalidated
        );
        assert_eq!(completed.progress.messages, 1);
        assert_eq!(completed.progress.bytes, frame_bytes);
        assert!(matches!(
            prepared.next_frame(),
            Err(crate::network::filter_transport::FilterTransportError::Invalidated)
        ));
        drop(prepared);
        assert_eq!(
            runtime
                .network
                .basic_filter_transport_snapshot()
                .expect("released"),
            FilterTransportSnapshot::default()
        );
        runtime
            .network
            .disconnect_peer(1)
            .expect("disconnect cleanup");
        assert_eq!(
            runtime
                .network
                .basic_filter_transport_snapshot()
                .expect("disconnected"),
            FilterTransportSnapshot::default()
        );
        fixture.cleanup();
    }
}

pub(super) fn managed_receive(
    runtime: &DurableSyncRuntime,
    peer: u64,
    message: WireNetworkMessage,
    inbound: bool,
) -> crate::network::ManagedSyncMessageResult {
    if inbound {
        runtime.network.receive_message_for_durable_serving(
            peer,
            message,
            50_000,
            runtime.verify_flags,
            runtime.consensus_params,
            |_| false,
        )
    } else {
        runtime.network.receive_sync_message(
            peer,
            message,
            50_000,
            runtime.verify_flags,
            runtime.consensus_params,
        )
    }
    .expect("managed receive")
}

pub(super) fn decoded_preparation(
    runtime: &DurableSyncRuntime,
    result: crate::network::ManagedSyncMessageResult,
) -> Vec<WireNetworkMessage> {
    assert!(result.outbound.is_empty());
    assert!(result.targeted_outbound.is_empty());
    let mut plan = result.inbound_response_plan;
    assert_eq!(plan.len(), 1);
    let PlanItem::PreparedFilterServe(mut response) = plan.pop().expect("plan") else {
        panic!("complete prepared family required: {plan:?}");
    };
    assert_eq!(response.progress().expect("prefix").messages, 0);
    assert!(
        runtime
            .network
            .basic_filter_response_is_current(&response)
            .expect("current")
    );
    let mut messages = Vec::new();
    // Preparation-only decoding uses the existing isolated test acknowledgement;
    // this is not socket-write evidence and no adapter is exercised here.
    while let Some(frame) = response.next_frame().expect("isolated frame") {
        messages.push(
            open_bitcoin_network::ParsedNetworkMessage::decode_wire(frame.bytes())
                .expect("complete valid wire")
                .message,
        );
        std::io::Write::write_all(&mut std::io::sink(), frame.bytes())
            .expect("complete isolated sink write");
        runtime
            .network
            .complete_basic_filter_frame(frame.acknowledge_write())
            .expect("test-only drain");
    }
    drop(response);
    assert_eq!(
        runtime
            .network
            .basic_filter_transport_snapshot()
            .expect("released"),
        FilterTransportSnapshot::default()
    );
    messages
}

pub(super) fn assert_unavailable(
    runtime: &DurableSyncRuntime,
    result: crate::network::ManagedSyncMessageResult,
    expected: Unavailable,
) {
    assert!(result.outbound.is_empty());
    assert!(result.targeted_outbound.is_empty());
    assert_eq!(
        result.inbound_response_plan,
        vec![PlanItem::FilterServeUnavailable(expected)]
    );
    assert_eq!(
        runtime
            .network
            .basic_filter_transport_snapshot()
            .expect("released"),
        FilterTransportSnapshot::default()
    );
}

pub(super) fn request_filters(start: u32, stop: BlockHash) -> WireNetworkMessage {
    WireNetworkMessage::GetCFilters(GetCFilters {
        filter_type: 0,
        start_height: start,
        stop_hash: stop,
    })
}

#[test]
fn phase160_managed_dispatch_permission_only_all_uses_live_peer_capability() {
    permission_only_case("all");
    permission_only_case("blockfilters");
}

fn permission_only_case(token: &str) {
    // Arrange
    let fixture = PeerFilterFixture::new(16);
    let runtime = &fixture.runtime;
    runtime
        .network
        .configure_basic_peer_serving(false)
        .expect("general off");
    let class = open_bitcoin_network::ParsedPeerPermissionClass::parse(
        "all-test",
        ["203.0.113.7"],
        ["in", token],
    )
    .expect("class");
    let decision = open_bitcoin_network::PeerPermissionClassRegistry::new([class])
        .resolve_inbound("203.0.113.7".parse().expect("address"));
    runtime
        .network
        .set_inbound_admission_policy(open_bitcoin_network::InboundAdmissionPolicy::new(8, 2))
        .expect("policy");
    runtime
        .network
        .admit_inbound_peer(
            open_bitcoin_network::InboundAdmissionRequest::from_permission_decision(
                2,
                "203.0.113.7:12000",
                decision,
            ),
        )
        .expect("admission");
    managed_receive(
        runtime,
        2,
        WireNetworkMessage::Version(open_bitcoin_network::VersionMessage {
            nonce: 999,
            ..Default::default()
        }),
        true,
    );
    managed_receive(runtime, 2, WireNetworkMessage::Verack, true);
    // Act
    let prepared = managed_receive(runtime, 2, request_filters(1, fixture.hash(2)), true);
    runtime
        .network
        .configure_basic_peer_serving(false)
        .expect("live collector retains the permission grant");
    let messages = decoded_preparation(runtime, prepared);
    // Assert
    assert_eq!(messages.len(), 2);
    let ordinary = runtime
        .network
        .receive_message(
            2,
            WireNetworkMessage::GetCFHeaders(GetCFHeaders {
                filter_type: 0,
                start_height: 1,
                stop_hash: fixture.hash(2),
            }),
            50_000,
            runtime.verify_flags,
            runtime.consensus_params,
        )
        .expect("ordinary handle wrapper");
    assert!(
        matches!(&decoded_preparation(runtime, ordinary)[..], [WireNetworkMessage::CFHeaders(headers)] if headers.filter_hashes.len() == 2)
    );
    // The all-only exception advertises capability even after backend disable.
    runtime
        .network
        .disable_basic_filter_index()
        .expect("disable");
    for request in [
        request_filters(1, fixture.hash(2)),
        WireNetworkMessage::GetCFHeaders(GetCFHeaders {
            filter_type: 0,
            start_height: 1,
            stop_hash: fixture.hash(2),
        }),
        WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
            filter_type: 0,
            stop_hash: fixture.hash(2),
        }),
    ] {
        assert_unavailable(
            runtime,
            managed_receive(runtime, 2, request, true),
            Unavailable::NoBackend,
        );
    }
    for kind in [
        open_bitcoin_network::CFRequestKind::Filters,
        open_bitcoin_network::CFRequestKind::Headers,
        open_bitcoin_network::CFRequestKind::Checkpoints,
    ] {
        let short = managed_receive(
            runtime,
            2,
            WireNetworkMessage::MalformedCFRequest(kind),
            true,
        );
        assert!(short.inbound_response_plan.is_empty());
        assert!(short.outbound.is_empty());
    }
    let invalid = runtime.network.receive_message_for_durable_serving(
        2,
        WireNetworkMessage::GetCFilters(GetCFilters {
            filter_type: 2,
            start_height: u32::MAX,
            stop_hash: BlockHash::default(),
        }),
        50_000,
        runtime.verify_flags,
        runtime.consensus_params,
        |_| false,
    );
    assert!(invalid.is_err());
    assert!(
        runtime
            .store()
            .basic_filter_operator_observation()
            .expect("invalid request")
            .maybe_read_fault
            .is_none()
    );
    assert!(
        runtime
            .network
            .peer_manager_snapshot()
            .expect("peers")
            .peer_state(2)
            .is_none()
    );
    assert_eq!(
        runtime
            .network
            .basic_filter_transport_snapshot()
            .expect("released"),
        FilterTransportSnapshot::default()
    );
    fixture.cleanup();
}

#[test]
fn phase160_managed_completeness_catchup_prefix_and_crossing_missing_are_immediate() {
    // Arrange
    let history = TurnHistory::new(16, 400, 1);
    let runtime = DurableSyncRuntime::open_configured(
        history.seed(16),
        sync_config(),
        BasicFilterStartupMode::Enabled,
    )
    .expect("catchup owner");
    runtime
        .network
        .configure_basic_peer_serving(true)
        .expect("capability");
    runtime.network.connect_outbound_peer(1, 0).expect("peer");
    super::complete_peer_handshake(&runtime, 1);
    let before = runtime
        .network
        .maybe_basic_index_progress()
        .expect("progress");
    let summary = runtime
        .network
        .maybe_basic_index_summary()
        .expect("summary")
        .expect("enabled");
    assert!(!summary.synced);
    let hash = |height: usize| history.history.records[height].identity().block_hash();
    // Act
    let prefix = decoded_preparation(
        &runtime,
        managed_receive(&runtime, 1, request_filters(0, hash(15)), false),
    );
    let missing = managed_receive(&runtime, 1, request_filters(0, hash(415)), false);
    // Assert
    assert_eq!(prefix.len(), 16);
    assert_unavailable(&runtime, missing, Unavailable::Missing);
    assert!(
        runtime
            .store()
            .basic_filter_operator_observation()
            .expect("initial catchup absence")
            .maybe_read_fault
            .is_none()
    );
    assert_eq!(
        runtime
            .network
            .maybe_basic_index_progress()
            .expect("progress"),
        before
    );
    assert!(
        !runtime
            .network
            .maybe_basic_index_summary()
            .expect("summary")
            .expect("enabled")
            .synced
    );
    drop(runtime);
    history.history.cleanup();
}

#[test]
fn phase160_managed_completeness_missing_corrupt_and_backend_faults_never_expose_prefix() {
    use crate::storage::fjall_store::filters::{FilterPublicationFault, query::QueryRecordFault};
    for (fault, expected) in [
        (QueryRecordFault::MissingTarget, Unavailable::Missing),
        (QueryRecordFault::MissingParent, Unavailable::StorageFailure),
        (QueryRecordFault::CorruptTarget, Unavailable::StorageFailure),
        (QueryRecordFault::CorruptParent, Unavailable::StorageFailure),
    ] {
        // Arrange
        let fixture = PeerFilterFixture::new(16);
        let runtime = &fixture.runtime;
        let before = runtime
            .network
            .maybe_basic_index_progress()
            .expect("progress");
        runtime
            .store()
            .inject_query_record_fault_for_test(fixture.hash(8), fault)
            .expect("narrow fault");
        // Act
        let result = managed_receive(runtime, 1, request_filters(8, fixture.hash(10)), false);
        // Assert
        assert_unavailable(runtime, result, expected);
        assert_eq!(
            runtime
                .network
                .maybe_basic_index_progress()
                .expect("progress"),
            before
        );
        fixture.cleanup();
    }
    // Arrange
    let fixture = PeerFilterFixture::new(16);
    fixture
        .runtime
        .store()
        .set_basic_filter_fault(FilterPublicationFault::BeforeQueryRead);
    // Act
    let result = managed_receive(
        &fixture.runtime,
        1,
        request_filters(1, fixture.hash(2)),
        false,
    );
    // Assert
    assert_unavailable(&fixture.runtime, result, Unavailable::StorageFailure);
    fixture.cleanup();
}

#[test]
fn phase160_managed_completeness_final_middle_and_metadata_faults_release_every_reservation() {
    use crate::storage::fjall_store::filters::query::QueryRecordFault;
    for (maybe_missing, expected) in [
        (Some(8), Unavailable::Missing),
        (Some(10), Unavailable::Missing),
        (None, Unavailable::StorageFailure),
    ] {
        // Arrange
        let fixture = PeerFilterFixture::new(16);
        let runtime = &fixture.runtime;
        let before = runtime
            .network
            .maybe_basic_index_progress()
            .expect("progress");
        if let Some(height) = maybe_missing {
            runtime
                .store()
                .inject_query_record_fault_for_test(
                    fixture.hash(height),
                    QueryRecordFault::MissingTarget,
                )
                .expect("target gap");
        } else {
            runtime
                .store()
                .write_raw_for_test(
                    StorageNamespace::BlockIndex,
                    codec::ownership::OWNER_KEY,
                    vec![255],
                )
                .expect("malformed owner metadata");
        }
        // Act
        let result = managed_receive(runtime, 1, request_filters(1, fixture.hash(10)), false);
        // Assert
        assert_unavailable(runtime, result, expected);
        assert_eq!(
            runtime
                .network
                .maybe_basic_index_progress()
                .expect("progress"),
            before
        );
        fixture.cleanup();
    }
}
