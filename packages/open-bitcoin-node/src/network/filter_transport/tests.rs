// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

use super::*;

#[test]
fn phase160_filter_lease_invalidation_actually_drops_queued_bytes() {
    // Arrange
    let owner = FilterTransportOwner::default();
    let lease = owner.reserve(1, 48).expect("reserve");
    let response = lease.prepare(vec![vec![0; 24]; 2]).expect("prepare");
    // Act
    owner.invalidate(|_, _| Ok(false)).expect("invalidate");
    // Assert
    assert!(
        response
            .lease
            .inner
            .frames
            .lock()
            .expect("frames")
            .is_empty()
    );
    assert_eq!(
        owner.snapshot().expect("snapshot"),
        FilterTransportSnapshot::default()
    );
}

#[test]
fn phase160_filter_lease_combined_frame_pressure_refuses_before_handoff() {
    // Arrange
    let owner = FilterTransportOwner::default();
    let mut first = owner
        .reserve(1, 4_000_024)
        .expect("reserve")
        .prepare(vec![vec![0; 4_000_024]])
        .expect("prepare");
    let mut second = owner
        .reserve(2, 4_000_024)
        .expect("reserve")
        .prepare(vec![vec![0; 4_000_024]])
        .expect("prepare");
    let frame = first.next_frame().expect("frame").expect("present");
    let pressure = owner.reserve_pending(3, 388_561).expect("generic pressure");
    // Act / Assert
    assert!(second.next_frame().is_err());
    assert_eq!(second.progress().expect("progress").messages, 0);
    drop(pressure);
    let pressure = owner.reserve_pending(3, 388_560).expect("exact pressure");
    let second_frame = second
        .next_frame()
        .expect("exact aggregate")
        .expect("present");
    drop(frame);
    drop(second_frame);
    drop(pressure);
    assert_eq!(
        owner.snapshot().expect("snapshot"),
        FilterTransportSnapshot::default()
    );
}

#[test]
fn phase160_filter_receipts_wrong_length_partial_failure_and_replay_never_credit() {
    // Arrange
    let owner = FilterTransportOwner::default();
    let mut response = owner
        .reserve(1, 72)
        .expect("reserve")
        .prepare(vec![vec![0; 24]; 3])
        .expect("prepare");
    let first = response
        .next_frame()
        .expect("frame")
        .expect("present")
        .into_written_for_test();
    let replay = FilterFrameReceipt {
        lease: Arc::clone(&first.lease),
        sequence: first.sequence,
        bytes: first.bytes,
        consumed: false,
    };
    owner.complete(first, true).expect("complete");
    // Act / Assert
    assert!(owner.complete(replay, true).is_err());
    assert_eq!(response.progress().expect("progress").messages, 1);
    assert_eq!(
        owner.snapshot().expect("snapshot"),
        FilterTransportSnapshot::default()
    );
    let mut partial = owner
        .reserve(2, 48)
        .expect("reserve")
        .prepare(vec![vec![0; 24]; 2])
        .expect("prepare");
    let (_bytes, capability) = partial
        .next_frame()
        .expect("frame")
        .expect("present")
        .into_parts();
    assert!(capability.written(23).is_err());
    assert_eq!(
        partial.progress().expect("progress"),
        FilterWrittenPrefix::default()
    );
    assert_eq!(
        owner.snapshot().expect("snapshot"),
        FilterTransportSnapshot::default()
    );
}

#[test]
fn phase160_filter_receipts_foreign_and_out_of_order_never_credit() {
    // Arrange
    let owner = FilterTransportOwner::default();
    let foreign = FilterTransportOwner::default();
    let mut response = owner
        .reserve(1, 24)
        .expect("reserve")
        .prepare(vec![vec![0; 24]])
        .expect("prepare");
    let receipt = response
        .next_frame()
        .expect("frame")
        .expect("present")
        .into_written_for_test();
    // Act / Assert
    assert_eq!(
        foreign
            .complete(receipt, true)
            .expect("classify")
            .disposition,
        FilterTransportDisposition::DuplicateOrForeign
    );
    assert_eq!(
        response.progress().expect("progress"),
        FilterWrittenPrefix {
            messages: 1,
            bytes: 24
        }
    );
    let mut response = owner
        .reserve(1, 24)
        .expect("reserve")
        .prepare(vec![vec![0; 24]])
        .expect("prepare");
    let mut receipt = response
        .next_frame()
        .expect("frame")
        .expect("present")
        .into_written_for_test();
    receipt.sequence = 1;
    assert!(owner.complete(receipt, true).is_err());
    assert_eq!(
        response.progress().expect("progress"),
        FilterWrittenPrefix::default()
    );
}

#[test]
fn phase160_filter_lease_cancel_drop_and_terminal_history_are_bounded() {
    // Arrange
    let owner = FilterTransportOwner::default();
    // Act / Assert
    for _ in 0..1000 {
        owner
            .reserve(1, 24)
            .expect("reserve")
            .cancel()
            .expect("cancel");
    }
    assert_eq!(owner.registry.lock().expect("registry").terminals.len(), 64);
    let mut response = owner
        .reserve(1, 48)
        .expect("reserve")
        .prepare(vec![vec![0; 24]; 2])
        .expect("prepare");
    let frame = response.next_frame().expect("frame").expect("present");
    drop(response);
    assert_eq!(owner.snapshot().expect("snapshot").active_frame_bytes, 24);
    drop(frame);
    assert_eq!(
        owner.snapshot().expect("snapshot"),
        FilterTransportSnapshot::default()
    );
}

#[test]
fn phase160_filter_lease_peer_aggregate_and_drop() {
    // Arrange
    let owner = FilterTransportOwner::default();
    let leases: Vec<_> = (1..=4)
        .map(|peer| owner.reserve(peer, 8 * 1024 * 1024).expect("lease"))
        .collect();
    // Act / Assert
    assert!(owner.reserve(1, 1).is_err());
    assert!(owner.reserve(5, 1).is_err());
    assert_eq!(
        owner.snapshot().expect("snapshot").suffix_bytes,
        32 * 1024 * 1024
    );
    drop(leases);
    assert_eq!(
        owner.snapshot().expect("snapshot"),
        FilterTransportSnapshot::default()
    );
}

#[test]
fn phase160_filter_lease_thousand_frames_are_one_request() {
    // Arrange
    let owner = FilterTransportOwner::default();
    let lease = owner.reserve(1, 1000 * 24).expect("lease");
    let mut response = lease.prepare(vec![vec![0; 24]; 1000]).expect("prepare");
    // Act / Assert
    assert_eq!(owner.snapshot().expect("snapshot").requests, 1);
    assert_eq!(response.progress().expect("progress").messages, 0);
    for sequence in 0..1000 {
        let frame = response.next_frame().expect("frame").expect("next");
        let receipt = frame.into_written_for_test();
        let result = owner.complete(receipt, true).expect("complete");
        assert_eq!(result.progress.messages, sequence + 1);
    }
    assert_eq!(response.progress().expect("progress").bytes, 24_000);
    assert!(response.next_frame().expect("finished").is_none());
    assert_eq!(
        owner.snapshot().expect("snapshot"),
        FilterTransportSnapshot::default()
    );
}

#[test]
fn phase160_filter_receipts_registry_poison_settles_physical_frame_without_live_success() {
    // Arrange
    let owner = FilterTransportOwner::default();
    let mut response = owner
        .reserve(1, 48)
        .expect("reserve")
        .prepare(vec![vec![0; 24]; 2])
        .expect("prepare");
    let frame = response.next_frame().expect("frame").expect("present");
    let registry = Arc::clone(&owner.registry);
    assert!(
        std::thread::spawn(move || {
            let _guard = registry.lock().expect("guard");
            panic!("test poison");
        })
        .join()
        .is_err()
    );
    // Act / Assert
    assert_eq!(
        owner.complete(frame.acknowledge_write(), true),
        Err(FilterTransportError::Poisoned)
    );
    assert_eq!(response.progress().expect("physical").messages, 1);
    assert_eq!(owner.raw_snapshot(), FilterTransportSnapshot::default());
    assert_eq!(owner.snapshot(), Err(FilterTransportError::Poisoned));
}

#[test]
fn phase160_filter_lease_checked_growth_and_id_exhaustion_preserve_capacity() {
    // Arrange
    let owner = FilterTransportOwner::default();
    let lease = owner.reserve(1, 0).expect("reserve");
    // Act / Assert
    assert!(lease.resize_suffix(8 * 1024 * 1024 + 1).is_err());
    assert_eq!(owner.snapshot().expect("snapshot").suffix_bytes, 0);
    lease.resize_suffix(24).expect("grow");
    assert_eq!(owner.snapshot().expect("snapshot").suffix_bytes, 24);
    drop(lease);
    owner.registry.lock().expect("registry").next = u64::MAX;
    assert_eq!(
        owner.reserve(1, 0).expect_err("exhaustion"),
        FilterTransportError::Exhausted
    );
    assert_eq!(
        owner.snapshot().expect("snapshot"),
        FilterTransportSnapshot::default()
    );
}

#[test]
fn phase160_filter_receipts_failed_second_frame_keeps_exact_first_prefix() {
    // Arrange
    let owner = FilterTransportOwner::default();
    let mut response = owner
        .reserve(1, 72)
        .expect("reserve")
        .prepare(vec![vec![0; 24]; 3])
        .expect("prepare");
    let first = response.next_frame().expect("frame").expect("present");
    owner
        .complete(first.acknowledge_write(), true)
        .expect("complete");
    let second = response.next_frame().expect("frame").expect("present");
    // Act
    drop(second);
    // Assert
    assert_eq!(
        response.progress().expect("progress"),
        FilterWrittenPrefix {
            messages: 1,
            bytes: 24
        }
    );
    assert_eq!(
        owner.snapshot().expect("snapshot"),
        FilterTransportSnapshot::default()
    );
}

#[test]
fn phase160_filter_receipts_dropped_achieved_receipt_records_physical_only() {
    // Arrange
    let owner = FilterTransportOwner::default();
    let mut response = owner
        .reserve(1, 48)
        .expect("reserve")
        .prepare(vec![vec![0; 24]; 2])
        .expect("prepare");
    let receipt = response
        .next_frame()
        .expect("frame")
        .expect("present")
        .acknowledge_write();
    // Act
    drop(receipt);
    // Assert
    assert_eq!(
        response.progress().expect("physical"),
        FilterWrittenPrefix {
            messages: 1,
            bytes: 24
        }
    );
    assert!(response.next_frame().is_err());
    assert_eq!(
        owner.snapshot().expect("snapshot"),
        FilterTransportSnapshot::default()
    );
}
