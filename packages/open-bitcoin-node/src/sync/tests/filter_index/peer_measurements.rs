// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

use super::peer_fixtures::PeerFilterFixture;
use crate::storage::fjall_store::filters::query::peer::BasicPeerReadError;
use open_bitcoin_network::{basic_peer_resource::*, policy::FilterRequest};

fn structural(label: &str, work: BasicPeerServeWork, records: usize) -> serde_json::Value {
    let mut counters = serde_json::Map::new();
    for axis in BasicPeerServeAxis::ALL {
        counters.insert(format!("{axis:?}"), work.get(axis).into());
        assert!(
            work.get(axis) <= BasicPeerServeBudget::default().remaining(axis),
            "{axis:?}"
        );
    }
    serde_json::json!({"case": label, "records": records, "work": counters})
}

fn observe(fixture: &PeerFilterFixture, label: &str, request: FilterRequest) -> serde_json::Value {
    let mut budget = BasicPeerServeBudget::default();
    let measured = fixture
        .runtime
        .network
        .measure_basic_peer_reads_for_test(request, &mut budget)
        .expect("authority");
    let records = measured
        .result
        .expect("admitted genuine request")
        .expect("complete records");
    eprintln!(
        "phase160 PEER {label} hold_us={} total_us={} work={:?}",
        measured.hold.as_micros(),
        measured.total.as_micros(),
        measured.work
    );
    let actual = structural(label, measured.work, records.len());
    let reference: serde_json::Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/parity/fixtures/peer-filter-resource-measurements.json"
    )))
    .expect("checked-in structural evidence");
    let expected = reference["measurements"]
        .as_array()
        .expect("measurements")
        .iter()
        .find(|measurement| measurement["case"] == label)
        .expect("measured case");
    assert_eq!(
        &actual, expected,
        "structural counters must remain reproducible"
    );
    actual
}

#[test]
fn phase160_peer_measurements_complete_genuine_persisted_matrix() {
    // Arrange
    let fixture = PeerFilterFixture::new(3002);
    let mut measurements = Vec::new();
    for count in [1, 20, 400, 1000] {
        let request = FilterRequest::Filters {
            filter_type: 0,
            start_height: 1,
            stop_hash: fixture.hash(count),
        };
        let label = format!("cfilters-{count}");
        // Act / Assert
        let cold = observe(&fixture, &label, request);
        assert_eq!(cold["records"], count);
        assert_eq!(observe(&fixture, &label, request), cold);
        measurements.push(cold);
    }
    let request = FilterRequest::Headers {
        filter_type: 0,
        start_height: 1,
        stop_hash: fixture.hash(2000),
    };
    let cold = observe(&fixture, "cfheaders-2000", request);
    assert_eq!(cold["records"], 2000);
    assert_eq!(observe(&fixture, "cfheaders-2000", request), cold);
    measurements.push(cold);
    let request = FilterRequest::Checkpoints {
        filter_type: 0,
        stop_hash: fixture.hash(3001),
    };
    let cold = observe(&fixture, "cfcheckpt-3001", request);
    assert_eq!(cold["records"], 3);
    assert_eq!(observe(&fixture, "cfcheckpt-3001", request), cold);
    measurements.push(cold);
    // Exact/one-under checks include the branch walk and all persisted reads.
    let request = FilterRequest::Filters {
        filter_type: 0,
        start_height: 1,
        stop_hash: fixture.hash(20),
    };
    let mut original = BasicPeerServeBudget::default();
    let observed = fixture
        .runtime
        .network
        .measure_basic_peer_reads_for_test(request, &mut original)
        .expect("measure");
    assert_eq!(
        observed.result.expect("success").expect("complete").len(),
        20
    );
    for axis in BasicPeerServeAxis::ALL {
        let exact = observed.work.get(axis);
        for (limit, expected) in [(exact, true), (exact - 1, false)] {
            let mut budget = BasicPeerServeBudget::default().with_limit(axis, limit);
            let result = fixture
                .runtime
                .network
                .measure_basic_peer_reads_for_test(request, &mut budget)
                .expect("authority")
                .result;
            assert_eq!(result.is_ok(), expected, "{axis:?} limit={limit}");
            if !expected {
                assert!(matches!(result, Err(BasicPeerReadError::Resource(_))));
            }
        }
    }
    let fixture = fixture.reopen();
    assert_eq!(
        observe(
            &fixture,
            "cfcheckpt-3001",
            FilterRequest::Checkpoints {
                filter_type: 0,
                stop_hash: fixture.hash(3001)
            }
        ),
        measurements[5]
    );
    eprintln!(
        "phase160 STRUCTURAL {}",
        serde_json::json!({"measurements": measurements})
    );
    fixture.cleanup();
}

#[test]
fn phase160_peer_measurements_genuine_large_filter_wire_admission() {
    // Arrange
    let fixture = PeerFilterFixture::large();
    let expected = fixture.history.history.records[16].encoded_bytes();
    assert!(
        expected.len() > 100_000,
        "distinct outputs must generate an actually large filter"
    );
    let request = FilterRequest::Filters {
        filter_type: 0,
        start_height: 16,
        stop_hash: fixture.hash(16),
    };
    // Act / Assert
    let result = observe(&fixture, "cfilter-genuine-large", request);
    assert_eq!(result["records"], 1);
    eprintln!("phase160 LARGE_STRUCTURAL {result}");
    fixture.cleanup();
}

#[test]
fn phase160_peer_measurements_sparse_genuine_stop_refuses_before_record_reads() {
    // Arrange
    let fixture = PeerFilterFixture::sparse(9302);
    let request = FilterRequest::Checkpoints {
        filter_type: 0,
        stop_hash: fixture.history.history.records[9301]
            .identity()
            .block_hash(),
    };
    let before = fixture.store.basic_filter_point_reads_for_test();
    let mut budget = BasicPeerServeBudget::default();
    // Act
    let measured = fixture
        .network
        .measure_basic_peer_reads_for_test(request, &mut budget)
        .expect("authority");
    // Assert
    assert!(matches!(
        measured.result,
        Err(BasicPeerReadError::Resource(BasicPeerResourceRefusal(
            BasicPeerServeAxis::AncestryLinks
        )))
    ));
    assert_eq!(measured.work.get(BasicPeerServeAxis::RecordReads), 0);
    assert_eq!(fixture.store.basic_filter_point_reads_for_test(), before);
    eprintln!(
        "phase160 SPARSE stop=9301 required_links=8302 admitted_links=8192 no_persisted_reads hold_us={} total_us={}",
        measured.hold.as_micros(),
        measured.total.as_micros()
    );
    fixture.cleanup();
}
