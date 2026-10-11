// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp

use super::*;

fn assert_family_messages(fixture: &PeerFilterFixture, peer: u64, inbound: bool) {
    let runtime = &fixture.runtime;
    // Act / Assert: whole inclusive limit and exact persisted byte order.
    let filters = decoded_preparation(
        runtime,
        managed_receive(
            runtime,
            peer,
            request_filters(1, fixture.hash(1000)),
            inbound,
        ),
    );
    assert_eq!(filters.len(), 1000);
    for (message, expected) in filters
        .iter()
        .zip(&fixture.history.history.records[1..=1000])
    {
        assert_eq!(
            message,
            &WireNetworkMessage::CFilter(open_bitcoin_network::CFilter {
                filter_type: 0,
                block_hash: expected.identity().block_hash(),
                filter: expected.encoded_bytes().to_vec(),
            })
        );
    }
    let headers = decoded_preparation(
        runtime,
        managed_receive(
            runtime,
            peer,
            WireNetworkMessage::GetCFHeaders(GetCFHeaders {
                filter_type: 0,
                start_height: 1,
                stop_hash: fixture.hash(2000),
            }),
            inbound,
        ),
    );
    assert_eq!(
        headers,
        vec![WireNetworkMessage::CFHeaders(
            open_bitcoin_network::CFHeaders {
                filter_type: 0,
                stop_hash: fixture.hash(2000),
                previous_filter_header: fixture.history.history.records[0]
                    .identity()
                    .filter_header()
                    .into(),
                filter_hashes: fixture.history.history.records[1..=2000]
                    .iter()
                    .map(|record| record.identity().filter_hash().into())
                    .collect(),
            }
        )]
    );
    let genesis = decoded_preparation(
        runtime,
        managed_receive(
            runtime,
            peer,
            WireNetworkMessage::GetCFHeaders(GetCFHeaders {
                filter_type: 0,
                start_height: 0,
                stop_hash: fixture.hash(0),
            }),
            inbound,
        ),
    );
    assert!(
        matches!(&genesis[..], [WireNetworkMessage::CFHeaders(headers)] if headers.previous_filter_header == Default::default() && headers.filter_hashes.len() == 1)
    );
    for stop in [999, 3001] {
        let checkpoints = decoded_preparation(
            runtime,
            managed_receive(
                runtime,
                peer,
                WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
                    filter_type: 0,
                    stop_hash: fixture.hash(stop),
                }),
                inbound,
            ),
        );
        assert_eq!(
            checkpoints,
            vec![WireNetworkMessage::CFCheckpt(
                open_bitcoin_network::CFCheckpt {
                    filter_type: 0,
                    stop_hash: fixture.hash(stop),
                    filter_headers: (1000..=stop)
                        .step_by(1000)
                        .map(|height| fixture.history.history.records[height]
                            .identity()
                            .filter_header()
                            .into())
                        .collect(),
                }
            )]
        );
    }
}

#[test]
fn phase160_complete_preparation_aggregate_pressure_preserves_semantic_disconnect() {
    use open_bitcoin_network::policy::{FilterRequest, FilterRequestInvalid};
    // Arrange
    let fixture = PeerFilterFixture::new(16);
    let runtime = &fixture.runtime;
    for peer in 2..=5 {
        runtime
            .network
            .connect_outbound_peer(peer, 0)
            .expect("peer");
        super::super::complete_peer_handshake(runtime, peer);
    }
    let held: Vec<_> = (1..=4)
        .map(|peer| managed_receive(runtime, peer, request_filters(1, fixture.hash(2)), false))
        .collect();
    assert_eq!(
        runtime
            .network
            .basic_filter_transport_snapshot()
            .expect("four reservations")
            .requests,
        4
    );
    // Act
    let refused = managed_receive(runtime, 5, request_filters(1, fixture.hash(2)), false);
    let invalid = runtime
        .network
        .prepare_basic_filter_response(
            5,
            FilterRequest::Filters {
                filter_type: 255,
                start_height: u32::MAX,
                stop_hash: BlockHash::default(),
            },
            runtime.consensus_params,
        )
        .expect("ordered rejection");
    let unadvertised = runtime
        .network
        .prepare_basic_filter_response(
            99,
            FilterRequest::Checkpoints {
                filter_type: 0,
                stop_hash: BlockHash::default(),
            },
            runtime.consensus_params,
        )
        .expect("first capability gate");
    // Assert
    assert!(refused.outbound.is_empty());
    assert!(refused.targeted_outbound.is_empty());
    assert_eq!(
        refused.inbound_response_plan,
        vec![PlanItem::FilterServeUnavailable(
            Unavailable::ResourceRefused
        )]
    );
    assert_eq!(
        invalid,
        Prepare::InvalidDisconnect(FilterRequestInvalid::UnsupportedTypeOrCapability)
    );
    assert_eq!(
        unadvertised,
        Prepare::InvalidDisconnect(FilterRequestInvalid::UnsupportedTypeOrCapability)
    );
    assert_eq!(
        runtime
            .network
            .basic_filter_transport_snapshot()
            .expect("unchanged held reservations")
            .requests,
        4
    );
    drop(held);
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
fn phase160_complete_preparation_genuine_legal_families_both_managed_directions() {
    // Arrange
    let fixture = PeerFilterFixture::new(3002);
    let runtime = &fixture.runtime;
    runtime.network.add_inbound_peer(2).expect("inbound");
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
    let before = runtime
        .network
        .maybe_basic_index_progress()
        .expect("progress");
    let durable = runtime.network.chainstate_snapshot().expect("snapshot");
    for (peer, inbound) in [(1, false), (2, true)] {
        assert_family_messages(&fixture, peer, inbound);
    }
    assert_eq!(
        runtime
            .network
            .maybe_basic_index_progress()
            .expect("progress"),
        before
    );
    assert_eq!(
        runtime.network.chainstate_snapshot().expect("snapshot"),
        durable
    );
    // Saturation does not turn pinned semantic violations into local refusal.
    let held = managed_receive(runtime, 1, request_filters(0, fixture.hash(1)), false);
    use open_bitcoin_network::policy::{FilterRequest, FilterRequestInvalid as Invalid};
    for (filter_type, start_height, stop_hash, invalid) in [
        (
            2,
            u32::MAX,
            BlockHash::default(),
            Invalid::UnsupportedTypeOrCapability,
        ),
        (
            0,
            u32::MAX,
            BlockHash::default(),
            Invalid::UnknownOrDisallowedStop,
        ),
        (0, 1002, fixture.hash(1001), Invalid::ReversedRange),
        (0, 0, fixture.hash(1000), Invalid::TooManyEntries),
    ] {
        assert_eq!(
            runtime
                .network
                .prepare_basic_filter_response(
                    1,
                    FilterRequest::Filters {
                        filter_type,
                        start_height,
                        stop_hash
                    },
                    runtime.consensus_params
                )
                .expect("typed outcome"),
            Prepare::InvalidDisconnect(invalid)
        );
    }
    assert_eq!(
        runtime
            .network
            .prepare_basic_filter_response(
                1,
                FilterRequest::Filters {
                    filter_type: 0,
                    start_height: 0,
                    stop_hash: fixture.hash(1)
                },
                runtime.consensus_params
            )
            .expect("local outcome"),
        Prepare::Unavailable(Unavailable::ResourceRefused)
    );
    drop(held);
    assert_eq!(
        runtime
            .network
            .basic_filter_transport_snapshot()
            .expect("released"),
        FilterTransportSnapshot::default()
    );
    // Complete checkpoint lookup exposes no prefix when a later checkpoint is absent.
    runtime
        .store()
        .inject_query_record_fault_for_test(
            fixture.hash(2000),
            crate::storage::fjall_store::filters::query::QueryRecordFault::MissingTarget,
        )
        .expect("checkpoint gap");
    assert_unavailable(
        runtime,
        managed_receive(
            runtime,
            1,
            WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
                filter_type: 0,
                stop_hash: fixture.hash(3001),
            }),
            false,
        ),
        Unavailable::Missing,
    );
    fixture.cleanup();
}
