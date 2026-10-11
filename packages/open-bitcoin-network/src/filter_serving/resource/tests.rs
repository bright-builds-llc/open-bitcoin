// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net.h
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

use super::*;

#[test]
fn phase160_resources_every_axis_exact_over_and_overflow_is_atomic() {
    // Arrange
    for axis in BasicPeerServeAxis::ALL {
        let mut meter = BasicPeerServeBudget::default();
        let limit = meter.remaining(axis);
        // Act / Assert
        meter.charge(axis, limit).expect("exact boundary");
        let before = meter.work();
        assert_eq!(meter.charge(axis, 1), Err(BasicPeerResourceRefusal(axis)));
        assert_eq!(meter.work(), before);
        assert_eq!(
            meter.charge(axis, u64::MAX),
            Err(BasicPeerResourceRefusal(axis))
        );
    }
}

#[test]
fn phase160_resources_frame_formulas_and_checkpoint_has_no_height_cap() {
    // Arrange / Act / Assert
    assert_eq!(cfilter_frame_bytes(252), Some(310));
    assert_eq!(cfilter_frame_bytes(253), Some(313));
    assert_eq!(cfheaders_frame_bytes(2000), Some(64_092));
    assert_eq!(cfcheckpt_frame_bytes(3001), Some(154));
    assert_eq!(cfcheckpt_frame_bytes(u32::MAX), Some(137_439_006));
    assert_eq!(cfilter_frame_bytes(u64::MAX), None);
    assert_eq!(cfheaders_frame_bytes(u64::MAX), None);
}

#[test]
fn phase160_resources_retention_and_pending_frames_exact_over_overflow() {
    // Arrange / Act / Assert
    use BasicPeerRetentionRefusal as Refusal;
    let suffix = BASIC_PEER_MAX_SUFFIX_BYTES;
    assert_eq!(
        admit_response_reservation(0, 3, 0, 3 * suffix, suffix),
        Ok(())
    );
    for (peer, aggregate, peer_bytes, aggregate_bytes, bytes, error) in [
        (1, 1, 0, 0, 1, Refusal::PeerRequests),
        (0, 4, 0, 0, 1, Refusal::AggregateRequests),
        (0, 0, 0, 0, suffix + 1, Refusal::PeerSuffix),
        (0, 0, 0, 3 * suffix + 1, suffix, Refusal::AggregateSuffix),
        (0, 0, u64::MAX, 0, 1, Refusal::PeerSuffix),
        (0, 0, 0, u64::MAX, 1, Refusal::AggregateSuffix),
    ] {
        assert_eq!(
            admit_response_reservation(peer, aggregate, peer_bytes, aggregate_bytes, bytes),
            Err(error)
        );
    }
    let frame = BASIC_PEER_MAX_FRAME_BYTES;
    assert_eq!(
        admit_active_filter_frame(frame, 0, BASIC_PEER_MAX_AGGREGATE_FRAME_BYTES - frame),
        Ok(())
    );
    assert_eq!(
        admit_active_filter_frame(frame, 1, 0),
        Err(Refusal::PeerFrame)
    );
    assert_eq!(
        admit_active_filter_frame(frame, 0, BASIC_PEER_MAX_AGGREGATE_FRAME_BYTES - frame + 1),
        Err(Refusal::AggregateFrame)
    );
    assert_eq!(
        admit_active_filter_frame(1, u64::MAX, 0),
        Err(Refusal::PeerFrame)
    );
    assert_eq!(
        admit_active_filter_frame(1, 0, u64::MAX),
        Err(Refusal::AggregateFrame)
    );
}

#[test]
fn phase160_coverage_narrowed_budget_preserves_consumed_work_and_cannot_reopen() {
    // Arrange
    for axis in BasicPeerServeAxis::ALL {
        let mut meter = BasicPeerServeBudget::default().with_limit(axis, 10);
        meter.charge(axis, 4).expect("initial consumed work");

        // Act
        let mut narrowed = meter.with_limit(axis, 6).with_limit(axis, 10);

        // Assert
        assert_eq!(narrowed.work(), meter.work());
        assert_eq!(narrowed.remaining(axis), 2);
        narrowed.charge(axis, 2).expect("remaining narrowed work");
        assert_eq!(narrowed.work().get(axis), 6);
        assert_eq!(
            narrowed.charge(axis, 1),
            Err(BasicPeerResourceRefusal(axis))
        );

        // Act
        let mut below_consumed = narrowed.with_limit(axis, 5).with_limit(axis, u64::MAX);
        let outcome = below_consumed.charge(axis, 1);

        // Assert
        assert_eq!(below_consumed.remaining(axis), 0);
        assert_eq!(outcome, Err(BasicPeerResourceRefusal(axis)));
        assert_eq!(below_consumed.work(), narrowed.work());
    }
}

#[test]
fn phase160_coverage_frame_sizing_crosses_u32_compact_size_boundary_without_allocation() {
    // Arrange
    let cases = [
        (u64::from(u32::MAX), 4_294_967_357, 137_438_953_534),
        (u64::from(u32::MAX) + 1, 4_294_967_362, 137_438_953_570),
    ];
    for (count, expected_filter_frame, expected_headers_frame) in cases {
        // Act
        let maybe_filter_frame = cfilter_frame_bytes(count);
        let maybe_headers_frame = cfheaders_frame_bytes(count);

        // Assert
        assert_eq!(maybe_filter_frame, Some(expected_filter_frame));
        assert_eq!(maybe_headers_frame, Some(expected_headers_frame));
    }
}
