// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

use super::*;
use crate::network::filter_transport::{FilterTransportDisposition, FilterTransportOwner};

#[test]
fn phase160_mixed_admission_handle_clones_share_actual_generic_buffers() {
    // Arrange
    let handle = transient();
    let clone = handle.clone();
    let message = open_bitcoin_network::WireNetworkMessage::Ping { nonce: 7 };
    // Act
    let first = handle
        .encode_pending_network_message(1, &message)
        .expect("first encoded owner");
    let second = clone
        .encode_pending_network_message(1, &message)
        .expect("second encoded owner");
    // Assert
    assert_eq!(first.bytes(), second.bytes());
    assert_eq!(&first.bytes()[..4], &[0xfa, 0xbf, 0xb5, 0xda]);
    assert_eq!(
        handle
            .basic_filter_transport_snapshot()
            .expect("shared")
            .generic_pending_bytes,
        64
    );
    drop(first);
    assert_eq!(
        clone
            .basic_filter_transport_snapshot()
            .expect("shared release")
            .generic_pending_bytes,
        32
    );
    let authority = handle.lock_authority().expect("held authority");
    drop(second);
    assert_eq!(
        clone
            .basic_filter_transport_snapshot()
            .expect("drop does not acquire authority"),
        FilterTransportSnapshot::default()
    );
    drop(authority);
}

#[test]
fn phase160_mixed_admission_failed_registry_preserves_handed_generic_charge() {
    // Arrange
    let handle = transient();
    let clone = handle.clone();
    let buffer = handle
        .encode_pending_network_message(1, &open_bitcoin_network::WireNetworkMessage::Verack)
        .expect("buffer");
    // Act
    clone.basic_filter_transport.mark_failed();
    // Assert
    assert_eq!(buffer.bytes().len(), 24);
    assert_eq!(
        clone
            .basic_filter_transport
            .raw_snapshot()
            .generic_pending_bytes,
        24
    );
    assert_eq!(
        handle.basic_filter_transport_snapshot(),
        Err(FilterTransportError::Poisoned)
    );
    assert!(matches!(
        clone.encode_pending_network_message(1, &open_bitcoin_network::WireNetworkMessage::Verack),
        Err(FilterTransportError::Poisoned)
    ));
    drop(buffer);
    assert_eq!(
        clone.basic_filter_transport.raw_snapshot(),
        FilterTransportSnapshot::default()
    );
    assert_eq!(
        handle.basic_filter_transport_snapshot(),
        Err(FilterTransportError::Poisoned)
    );
}

#[test]
fn phase160_filter_receipts_invalidation_during_write_preserves_only_physical_prefix() {
    // Arrange
    let owner = FilterTransportOwner::default();
    let lease = owner.reserve(1, 72).expect("lease");
    let mut response = lease.prepare(vec![vec![0; 24]; 3]).expect("response");
    let first = response
        .next_frame()
        .expect("frame")
        .expect("present")
        .into_written_for_test();
    owner.complete(first, true).expect("first");
    let second = response.next_frame().expect("frame").expect("present");
    // Act
    owner.invalidate(|_, _| Ok(false)).expect("invalidate");
    // Assert
    let snapshot = owner.snapshot().expect("snapshot");
    assert_eq!(snapshot.requests, 1);
    assert_eq!(snapshot.suffix_bytes, 24);
    assert_eq!(snapshot.active_frame_bytes, 24);
    assert!(owner.reserve(1, 24).is_err());
    let result = owner
        .complete(second.into_written_for_test(), false)
        .expect("late write");
    assert_eq!(
        result.disposition,
        FilterTransportDisposition::PhysicalPrefixInvalidated
    );
    assert_eq!(result.progress.messages, 2);
    assert_eq!(result.progress.bytes, 48);
    assert_eq!(
        owner.snapshot().expect("snapshot"),
        FilterTransportSnapshot::default()
    );
    assert!(response.next_frame().is_err());
}

fn transient() -> crate::MemoryNetworkHandle {
    let handle = crate::MemoryNetworkHandle::transient_runtime(
        open_bitcoin_core::primitives::NetworkMagic::from_bytes([0xfa, 0xbf, 0xb5, 0xda]),
        18444,
        Default::default(),
        Default::default(),
        false,
    );
    handle
        .configure_basic_peer_serving(true)
        .expect("configure");
    handle.connect_outbound_peer(1, 0).expect("connect");
    handle
}

#[test]
fn phase160_filter_receipts_unbound_disconnect_releases_request() {
    // Arrange
    let handle = transient();
    let clone = handle.clone();
    let lease = handle.reserve_basic_filter_response(1).expect("reserve");
    // Act
    clone.disconnect_peer(1).expect("disconnect");
    // Assert
    assert_eq!(
        handle.basic_filter_transport_snapshot().expect("snapshot"),
        FilterTransportSnapshot::default()
    );
    assert!(lease.resize_suffix(24).is_err());
}

fn persistent(
    name: &str,
) -> (
    std::path::PathBuf,
    crate::DurableSyncRuntime,
    crate::DurableNetworkHandle,
) {
    let (path, store, _) =
        crate::storage::fjall_store::filters::tests::query::recovered_fixture(name, 4);
    let positions = store.load_chain_meta_for_open().expect("positions").0;
    let runtime = crate::DurableSyncRuntime::open_configured(
        store,
        Default::default(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("runtime");
    let handle = runtime.network_handle();
    handle
        .mutate(|network| {
            let mut headers = open_bitcoin_network::HeaderStore::default();
            for position in positions {
                headers.record_position(&position);
            }
            network.seed_header_store(headers);
        })
        .expect("headers");
    handle
        .configure_basic_peer_serving(true)
        .expect("configure");
    handle.connect_outbound_peer(1, 0).expect("connect");
    handle.connect_outbound_peer(2, 0).expect("connect");
    (path, runtime, handle)
}

fn captured(
    handle: &crate::DurableNetworkHandle,
    store: &crate::FjallNodeStore,
) -> (
    BasicFilterBranchCapture,
    crate::storage::fjall_store::filters::query::BasicFilterReadEpoch,
) {
    let stop = handle
        .maybe_chain_tip()
        .expect("tip")
        .expect("present")
        .block_hash;
    let outcome = handle
        .capture_basic_filter_branch(
            1,
            open_bitcoin_network::policy::FilterRequest::Checkpoints {
                filter_type: 0,
                stop_hash: stop,
            },
            Default::default(),
            super::super::BasicFilterBranchLimit {
                branch: open_bitcoin_network::policy::BranchWorkLimit {
                    ancestry_steps: 8192,
                    identity_bytes: 1_000_000,
                },
                metadata_units: 256,
            },
        )
        .expect("capture");
    let super::super::BasicFilterBranchOutcome::Authorized(capture) = outcome else {
        panic!("authorized");
    };
    let epoch = store
        .with_basic_peer_query(&mut Default::default(), |reader, meter| {
            reader.capture_basic_peer_epoch(meter)
        })
        .expect("epoch");
    (*capture, epoch)
}

fn prepared(
    handle: &crate::DurableNetworkHandle,
    store: &crate::FjallNodeStore,
    frames: u64,
) -> PreparedFilterResponse {
    let lease = handle.reserve_basic_filter_response(1).expect("reserve");
    let (capture, epoch) = captured(handle, store);
    handle
        .bind_basic_filter_response(&lease, &capture, epoch)
        .expect("bind");
    lease.resize_suffix(frames * 24).expect("suffix");
    lease
        .prepare(vec![vec![0; 24]; frames as usize])
        .expect("prepare")
}

#[test]
fn phase160_filter_receipts_peer_and_authority_binding_are_exact() {
    // Arrange
    let (path, runtime, handle) = persistent("transport-bind");
    let (capture, epoch) = captured(&handle, runtime.store());
    let wrong_peer = handle.reserve_basic_filter_response(2).expect("reserve");
    // Act / Assert
    assert_eq!(
        handle.bind_basic_filter_response(&wrong_peer, &capture, epoch.clone()),
        Err(FilterTransportError::Foreign)
    );
    let foreign = transient();
    let foreign_lease = foreign
        .reserve_basic_filter_response(1)
        .expect("foreign reserve");
    assert_eq!(
        handle.bind_basic_filter_response(&foreign_lease, &capture, epoch),
        Err(FilterTransportError::Foreign)
    );
    let (_, fresh_epoch) = captured(&handle, runtime.store());
    let cancelled = handle.reserve_basic_filter_response(1).expect("reserve");
    cancelled.resize_suffix(24).expect("reserve suffix");
    let mut cancelled = cancelled
        .prepare(vec![vec![0; 24]])
        .expect("unbound preparation");
    let held = cancelled.next_frame().expect("frame").expect("present");
    cancelled.lease().abort().expect("cancel");
    assert_eq!(
        handle.bind_basic_filter_response(cancelled.lease(), &capture, fresh_epoch),
        Err(FilterTransportError::Invalidated)
    );
    drop(held);
    drop(cancelled);
    drop(wrong_peer);
    drop(foreign_lease);
    drop(foreign);
    drop(handle);
    drop(runtime);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase160_filter_receipts_empty_checkpoint_one_frame_requires_complete_write() {
    // Arrange
    let (path, runtime, handle) = persistent("transport-checkpoint");
    let mut response = prepared(&handle, runtime.store(), 1);
    assert_eq!(response.progress().expect("progress").messages, 0);
    assert!(
        handle
            .basic_filter_response_is_current(&response)
            .expect("current")
    );
    // Act
    let frame = response.next_frame().expect("frame").expect("present");
    let completion = handle
        .complete_basic_filter_frame(frame.acknowledge_write())
        .expect("complete");
    // Assert
    assert_eq!(
        completion.disposition,
        FilterTransportDisposition::CurrentRangeComplete
    );
    assert_eq!(completion.progress.messages, 1);
    assert_eq!(completion.progress.bytes, 24);
    drop(response);
    drop(handle);
    drop(runtime);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase160_filter_receipts_config_and_disconnect_during_write_preserve_physical_only() {
    for disconnect in [false, true] {
        // Arrange
        let (path, runtime, handle) = persistent(if disconnect {
            "transport-disconnect"
        } else {
            "transport-config"
        });
        let mut response = prepared(&handle, runtime.store(), 2);
        let frame = response.next_frame().expect("frame").expect("present");
        // Act
        if disconnect {
            handle.disconnect_peer(1).expect("disconnect");
            handle.connect_outbound_peer(1, 1).expect("reconnect");
        } else {
            handle
                .configure_basic_peer_serving(false)
                .expect("disable serving");
            handle
                .configure_basic_peer_serving(true)
                .expect("reenable serving");
        }
        // Assert
        assert!(
            !handle
                .basic_filter_response_is_current(&response)
                .expect("cancelled request stays revoked")
        );
        assert_eq!(
            handle
                .basic_filter_transport_snapshot()
                .expect("held")
                .active_frame_bytes,
            24
        );
        let result = handle
            .complete_basic_filter_frame(frame.acknowledge_write())
            .expect("physical");
        assert_eq!(
            result.disposition,
            FilterTransportDisposition::PhysicalPrefixInvalidated
        );
        assert_eq!(result.progress.messages, 1);
        assert_eq!(
            handle.basic_filter_transport_snapshot().expect("released"),
            FilterTransportSnapshot::default()
        );
        drop(response);
        drop(handle);
        drop(runtime);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
}

#[test]
fn phase160_filter_receipts_authority_poison_retains_handed_charge_and_physical_fact() {
    // Arrange
    let (path, runtime, handle) = persistent("transport-authority-poison");
    let mut response = prepared(&handle, runtime.store(), 2);
    let frame = response.next_frame().expect("frame").expect("present");
    handle.poison_for_test();
    // Act / Assert
    assert_eq!(
        handle.basic_filter_response_is_current(&response),
        Err(FilterTransportError::AuthorityUnavailable)
    );
    assert_eq!(
        handle
            .basic_filter_transport
            .raw_snapshot()
            .active_frame_bytes,
        24
    );
    assert_eq!(
        handle.complete_basic_filter_frame(frame.acknowledge_write()),
        Err(FilterTransportError::AuthorityUnavailable)
    );
    assert_eq!(response.progress().expect("physical").messages, 1);
    assert_eq!(
        handle.basic_filter_transport.raw_snapshot(),
        FilterTransportSnapshot::default()
    );
    assert_eq!(
        handle.basic_filter_transport_snapshot(),
        Err(FilterTransportError::Poisoned)
    );
    drop(response);
    drop(handle);
    drop(runtime);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase160_filter_receipts_store_poison_is_typed_and_preserves_achieved_frame() {
    // Arrange
    let (path, runtime, handle) = persistent("transport-store-poison");
    let mut response = prepared(&handle, runtime.store(), 2);
    let frame = response.next_frame().expect("frame").expect("present");
    runtime.store().set_basic_filter_fault(
        crate::storage::fjall_store::filters::FilterPublicationFault::BeforeDisable,
    );
    assert!(runtime.store().disable_basic_filter_index().is_err());
    // Act / Assert
    assert_eq!(
        handle.complete_basic_filter_frame(frame.acknowledge_write()),
        Err(FilterTransportError::PublicationUnavailable)
    );
    assert_eq!(response.progress().expect("physical").messages, 1);
    assert_eq!(
        handle.basic_filter_transport.raw_snapshot(),
        FilterTransportSnapshot::default()
    );
    drop(response);
    drop(handle);
    drop(runtime);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase160_filter_receipts_captured_identity_rejects_every_foreign_axis() {
    // Arrange
    let (path, runtime, handle) = persistent("transport-identity");
    let response = prepared(&handle, runtime.store(), 1);
    let identity = response
        .lease()
        .identity()
        .expect("identity")
        .expect("bound");
    // Act / Assert
    for axis in 0..5 {
        let mut changed = identity.clone();
        match axis {
            0 => changed.authority += 1,
            1 => changed.session = changed.session.checked_next().expect("next session"),
            2 => {
                changed.generation =
                    open_bitcoin_core::chainstate::filter_index::lifecycle::IndexGeneration::new(
                        changed.generation.value() + 1,
                    )
            }
            3 => {
                changed.branch_anchor =
                    open_bitcoin_core::primitives::BlockHash::from_byte_array([0x77; 32])
            }
            _ => {
                changed.stop = open_bitcoin_core::primitives::BlockHash::from_byte_array([0x88; 32])
            }
        }
        assert!(
            !handle
                .read(|network| is_current(network, 1, &changed))
                .expect("authority")
                .expect("freshness")
        );
    }
    assert!(
        handle
            .basic_filter_response_is_current(&response)
            .expect("original")
    );
    drop(response);
    drop(handle);
    drop(runtime);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase160_filter_receipts_mempool_and_accepted_extension_preserve_immutable_range() {
    // Arrange
    let (path, runtime, handle) = persistent("transport-extension");
    let response = prepared(&handle, runtime.store(), 1);
    let tip = handle.maybe_chain_tip().expect("tip").expect("present");
    let block =
        crate::chainstate::validation_history::tests::block(tip.block_hash, tip.height + 1, 0x51);
    // Act
    handle
        .expire_mempool(open_bitcoin_mempool::PolicyTime::new(1_000))
        .expect("expiry");
    handle
        .connect_local_block(
            &block,
            open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
            Default::default(),
        )
        .expect("genuine accepted extension");
    // Assert
    assert!(
        handle
            .basic_filter_response_is_current(&response)
            .expect("unchanged range")
    );
    drop(response);
    drop(handle);
    drop(runtime);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase160_filter_receipts_same_generation_recovery_during_write_is_physical_only() {
    // Arrange
    let (path, runtime, handle) = persistent("transport-aba");
    let mut response = prepared(&handle, runtime.store(), 2);
    let frame = response.next_frame().expect("frame").expect("present");
    let tip = handle.maybe_chain_tip().expect("tip").expect("present");
    // Act
    runtime
        .store()
        .recover_basic_filter_index_before_prune(Some(tip.block_hash))
        .expect("same generation recovery");
    let completion = handle
        .complete_basic_filter_frame(frame.acknowledge_write())
        .expect("complete physical");
    // Assert
    assert_eq!(
        completion.disposition,
        FilterTransportDisposition::PhysicalPrefixInvalidated
    );
    assert_eq!(completion.progress.messages, 1);
    assert_eq!(
        handle.basic_filter_transport_snapshot().expect("released"),
        FilterTransportSnapshot::default()
    );
    drop(response);
    drop(handle);
    drop(runtime);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase160_filter_lease_cancel_never_acquires_held_authority() {
    // Arrange
    let handle = transient();
    let lease = handle.reserve_basic_filter_response(1).expect("reserve");
    let guard = handle.lock_authority().expect("held authority");
    // Act
    lease.cancel().expect("memory-only cancellation");
    // Assert
    assert_eq!(
        handle.basic_filter_transport_snapshot().expect("released"),
        FilterTransportSnapshot::default()
    );
    drop(guard);
}

#[test]
fn phase160_filter_receipts_failed_transport_cleanup_still_stops_readiness() {
    // Arrange
    let handle = transient();
    handle.basic_filter_transport.mark_failed();
    // Act
    assert!(handle.stop_basic_filter_readiness().is_err());
    // Assert
    assert!(handle.basic_filter_readiness.is_stopped());
}
