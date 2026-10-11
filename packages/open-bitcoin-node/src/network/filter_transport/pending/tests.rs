// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net.h
// - packages/bitcoin-knots/src/net_processing.cpp

use super::super::*;
use super::{GenericPendingBuffer, WireSize, wire_size};
use open_bitcoin_core::primitives::{BlockHash, NetworkMagic};
use open_bitcoin_network::{CFCheckpt, CFHeaders, CFilter, WireNetworkMessage};

fn retained(owner: &FilterTransportOwner, peer: PeerId, bytes: u64) -> GenericPendingBuffer {
    let reservation = owner
        .reserve_pending(peer, bytes)
        .expect("reserve before allocation");
    GenericPendingBuffer {
        bytes: vec![0; bytes as usize],
        _reservation: reservation,
    }
}

#[test]
fn phase160_mixed_pending_exact_limit_and_release() {
    // Arrange
    let owner = FilterTransportOwner::default();
    // Act
    let pending = owner.reserve_pending(1, 8 * 1024 * 1024).expect("exact");
    // Assert
    assert!(owner.reserve_pending(2, 1).is_err());
    assert_eq!(
        owner.snapshot().expect("snapshot").generic_pending_bytes,
        8 * 1024 * 1024
    );
    drop(pending);
    assert_eq!(
        owner.snapshot().expect("released"),
        FilterTransportSnapshot::default()
    );
}

#[test]
fn phase160_mixed_pending_zero_byte_descriptors_are_bounded() {
    // Arrange
    let owner = FilterTransportOwner::default();
    let mut guards = Vec::new();
    // Act
    for peer in 0..8 {
        for _ in 0..128 {
            guards.push(owner.reserve_pending(peer, 0).expect("descriptor"));
        }
        assert!(owner.reserve_pending(peer, 0).is_err());
    }
    // Assert
    assert!(owner.reserve_pending(99, 0).is_err());
    assert_eq!(
        owner.snapshot().expect("snapshot").generic_descriptors,
        1024
    );
    drop(guards);
    assert_eq!(
        owner.snapshot().expect("released"),
        FilterTransportSnapshot::default()
    );
}

#[test]
fn phase160_mixed_pending_checked_overflow_and_id_exhaustion_do_not_grow_maps() {
    // Arrange
    let owner = FilterTransportOwner::default();
    let buffer = retained(&owner, 1, 1);
    // Act / Assert
    assert!(owner.reserve_pending(2, u64::MAX).is_err());
    assert_eq!(owner.snapshot().expect("no growth").generic_descriptors, 1);
    drop(buffer);
    owner.registry.lock().expect("registry").next = u64::MAX;
    assert!(matches!(
        owner.reserve_pending(2, 0),
        Err(FilterTransportError::Exhausted)
    ));
    assert!(owner.registry.lock().expect("registry").generic.is_empty());
}

#[test]
fn phase160_mixed_pending_failed_owner_retains_actual_buffer_until_drop() {
    // Arrange
    let owner = FilterTransportOwner::default();
    let buffer = owner
        .encode_pending(
            1,
            &WireNetworkMessage::Ping { nonce: 9 },
            NetworkMagic::MAINNET,
        )
        .expect("encode");
    // Act
    owner.mark_failed();
    // Assert
    assert_eq!(buffer.bytes().len(), 32);
    assert_eq!(owner.raw_snapshot().generic_pending_bytes, 32);
    assert_eq!(owner.snapshot(), Err(FilterTransportError::Poisoned));
    assert!(
        owner
            .encode_pending(2, &WireNetworkMessage::Verack, NetworkMagic::MAINNET)
            .is_err()
    );
    drop(buffer);
    assert_eq!(owner.raw_snapshot(), FilterTransportSnapshot::default());
    assert_eq!(owner.snapshot(), Err(FilterTransportError::Poisoned));
}

#[test]
fn phase160_mixed_pending_poisoned_drop_releases_without_hiding_failure() {
    // Arrange
    let owner = FilterTransportOwner::default();
    let buffer = retained(&owner, 1, 32);
    let registry = Arc::clone(&owner.registry);
    // Act
    assert!(
        std::thread::spawn(move || {
            let _held = registry.lock().expect("held");
            panic!("injected registry poison");
        })
        .join()
        .is_err()
    );
    // Assert
    assert_eq!(buffer.bytes().len(), 32);
    assert_eq!(owner.raw_snapshot().generic_pending_bytes, 32);
    assert!(matches!(
        owner.reserve_pending(2, 1),
        Err(FilterTransportError::Poisoned)
    ));
    drop(buffer);
    assert_eq!(owner.raw_snapshot(), FilterTransportSnapshot::default());
    assert_eq!(owner.snapshot(), Err(FilterTransportError::Poisoned));
}

#[test]
fn phase160_pending_encoding_scratch_reserved_before_encode_and_released_after_return() {
    // Arrange
    let owner = FilterTransportOwner::default();
    let message = WireNetworkMessage::Ping { nonce: 11 };
    let size = wire_size(&message).expect("size");
    // Act
    let encoded = owner
        .encode_reserved(1, size, || {
            assert_eq!(
                owner
                    .snapshot()
                    .expect("during encoding")
                    .generic_pending_bytes,
                72
            );
            message
                .encode_wire(NetworkMagic::MAINNET)
                .map_err(|_| FilterTransportError::Encoding)
        })
        .expect("encode");
    // Assert
    assert_eq!(
        encoded.bytes(),
        message.encode_wire(NetworkMagic::MAINNET).expect("codec")
    );
    assert_eq!(
        owner
            .snapshot()
            .expect("only actual output")
            .generic_pending_bytes,
        32
    );
    drop(encoded);
    assert_eq!(
        owner.snapshot().expect("released"),
        FilterTransportSnapshot::default()
    );
}

#[test]
fn phase160_pending_encoding_failure_and_length_drift_release_reservation() {
    // Arrange
    let owner = FilterTransportOwner::default();
    let size = WireSize {
        payload: 8,
        frame: 32,
        peak: 72,
    };
    // Act / Assert
    assert!(matches!(
        owner.encode_reserved(1, size, || Err(FilterTransportError::Encoding)),
        Err(FilterTransportError::Encoding)
    ));
    assert_eq!(
        owner.snapshot().expect("error cleanup"),
        FilterTransportSnapshot::default()
    );
    assert!(matches!(
        owner.encode_reserved(1, size, || Ok(vec![0; 31])),
        Err(FilterTransportError::Length)
    ));
    assert_eq!(
        owner.snapshot().expect("drift cleanup"),
        FilterTransportSnapshot::default()
    );
}

#[test]
fn phase160_pending_encoding_cf_factory_cannot_bypass_request_authority() {
    // Arrange
    let owner = FilterTransportOwner::default();
    let hash = BlockHash::default();
    let messages = [
        WireNetworkMessage::CFilter(CFilter {
            filter_type: 0,
            block_hash: hash,
            filter: Vec::new(),
        }),
        WireNetworkMessage::CFHeaders(CFHeaders {
            filter_type: 0,
            stop_hash: hash,
            previous_filter_header: Default::default(),
            filter_hashes: Vec::new(),
        }),
        WireNetworkMessage::CFCheckpt(CFCheckpt {
            filter_type: 0,
            stop_hash: hash,
            filter_headers: Vec::new(),
        }),
    ];
    // Act / Assert
    for message in messages {
        assert!(matches!(
            owner.encode_pending(1, &message, NetworkMagic::MAINNET),
            Err(FilterTransportError::FilterResponseRequiresLease)
        ));
        assert_eq!(
            owner.snapshot().expect("no charge"),
            FilterTransportSnapshot::default()
        );
    }
}

#[test]
fn phase160_pending_encoding_tiny_and_command_string_exact_headroom() {
    // Arrange
    let messages = [
        WireNetworkMessage::Ping { nonce: 1 },
        WireNetworkMessage::Verack,
        WireNetworkMessage::GetBlockTxn(open_bitcoin_codec::BlockTransactionsRequest {
            block_hash: BlockHash::default(),
            index_deltas: vec![0; 22],
        }),
    ];
    // Act / Assert
    for message in messages {
        let owner = FilterTransportOwner::default();
        let peak = wire_size(&message).expect("size").peak;
        if matches!(message, WireNetworkMessage::GetBlockTxn(_)) {
            assert_eq!(peak, 145);
        }
        let mut first = owner
            .reserve(1, 4_000_024)
            .expect("first")
            .prepare(vec![vec![0; 4_000_024]])
            .expect("prepare");
        let mut second = owner
            .reserve(2, 4_000_024)
            .expect("second")
            .prepare(vec![vec![0; 4_000_024]])
            .expect("prepare");
        let frame1 = first.next_frame().expect("first handoff").expect("frame");
        let frame2 = second.next_frame().expect("second handoff").expect("frame");
        let pressure = retained(&owner, 3, 8 * 1024 * 1024 - 8_000_048 - peak + 1);
        assert!(
            owner
                .encode_pending(4, &message, NetworkMagic::MAINNET)
                .is_err()
        );
        drop(pressure);
        let pressure = retained(&owner, 3, 8 * 1024 * 1024 - 8_000_048 - peak);
        let encoded = owner
            .encode_pending(4, &message, NetworkMagic::MAINNET)
            .expect("exact scratch headroom");
        assert_eq!(
            encoded.bytes(),
            message
                .encode_wire(NetworkMagic::MAINNET)
                .expect("same bytes")
        );
        drop(encoded);
        drop(pressure);
        drop(frame1);
        drop(frame2);
        assert_eq!(
            owner.snapshot().expect("released"),
            FilterTransportSnapshot::default()
        );
    }
}

#[test]
fn phase160_mixed_admission_both_orders_exact_boundary_and_one_over() {
    // Arrange / Act / Assert
    for generic_first in [true, false] {
        let owner = FilterTransportOwner::default();
        let mut response = owner
            .reserve(1, 4_000_024)
            .expect("CF reserve")
            .prepare(vec![vec![0; 4_000_024]])
            .expect("prepare");
        let (generic, frame) = if generic_first {
            let generic = retained(&owner, 2, 8 * 1024 * 1024 - 4_000_024);
            let frame = response
                .next_frame()
                .expect("exact mixed boundary")
                .expect("CF frame");
            (generic, frame)
        } else {
            let frame = response.next_frame().expect("CF frame").expect("present");
            let generic = retained(&owner, 2, 8 * 1024 * 1024 - 4_000_024);
            (generic, frame)
        };
        assert!(owner.reserve_pending(3, 1).is_err());
        response.cancel().expect("cancel queued authority");
        assert_eq!(
            owner.snapshot().expect("still physical").active_frame_bytes,
            frame.bytes().len() as u64
        );
        assert_eq!(
            owner
                .snapshot()
                .expect("still physical")
                .generic_pending_bytes,
            generic.bytes().len() as u64
        );
        assert!(owner.reserve_pending(3, 1).is_err());
        drop(frame);
        drop(generic);
        assert_eq!(
            owner.snapshot().expect("released"),
            FilterTransportSnapshot::default()
        );
    }
}

#[test]
fn phase160_mixed_admission_barrier_race_cannot_over_admit() {
    // Arrange / Act / Assert
    for _ in 0..32 {
        let owner = FilterTransportOwner::default();
        let pressure = retained(&owner, 3, 4 * 1024 * 1024);
        let mut response = owner
            .reserve(1, 3 * 1024 * 1024)
            .expect("CF reserve")
            .prepare(vec![vec![0; 3 * 1024 * 1024]])
            .expect("prepare");
        let start = std::sync::Barrier::new(3);
        let settled = std::sync::Barrier::new(3);
        std::thread::scope(|scope| {
            let generic = scope.spawn(|| {
                start.wait();
                let result = owner
                    .reserve_pending(2, 3 * 1024 * 1024)
                    .map(|reservation| GenericPendingBuffer {
                        bytes: vec![0; 3 * 1024 * 1024],
                        _reservation: reservation,
                    });
                settled.wait();
                result
            });
            let cf = scope.spawn(|| {
                start.wait();
                let result = response.next_frame();
                settled.wait();
                result
            });
            start.wait();
            settled.wait();
            let generic = generic.join().expect("generic thread");
            let frame = cf.join().expect("CF thread");
            assert_ne!(generic.is_ok(), frame.is_ok());
            assert_eq!(
                owner
                    .registry
                    .lock()
                    .expect("registry")
                    .pending_bytes()
                    .expect("checked"),
                7 * 1024 * 1024
            );
            drop(generic);
            drop(frame);
        });
        drop(response);
        drop(pressure);
        assert_eq!(
            owner.snapshot().expect("released"),
            FilterTransportSnapshot::default()
        );
    }
}

#[test]
fn phase160_mixed_admission_full_encoder_honors_scratch_in_reverse_order() {
    use open_bitcoin_core::primitives::{
        Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
        TransactionOutput,
    };
    // Arrange
    let owner = FilterTransportOwner::default();
    let message = WireNetworkMessage::Tx(Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::default(),
            sequence: u32::MAX,
            witness: ScriptWitness::new(vec![vec![1; 4_000_000 - 68]]),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(1).expect("amount"),
            script_pubkey: ScriptBuf::default(),
        }],
        lock_time: 0,
    });
    let mut response = owner
        .reserve(2, 4_000_024)
        .expect("CF")
        .prepare(vec![vec![0; 4_000_024]])
        .expect("prepare");
    let frame = response.next_frame().expect("first CF").expect("present");
    // Act / Assert: retained frames would fit, but simultaneous encoder scratch would not.
    assert!(
        owner
            .encode_pending(1, &message, NetworkMagic::MAINNET)
            .is_err()
    );
    assert_eq!(
        owner
            .snapshot()
            .expect("no generic allocation")
            .generic_descriptors,
        0
    );
    drop(frame);
    drop(response);
    let generic = owner
        .encode_pending(1, &message, NetworkMagic::MAINNET)
        .expect("encode first");
    let mut response = owner
        .reserve(2, 4_000_024)
        .expect("CF")
        .prepare(vec![vec![0; 4_000_024]])
        .expect("prepare");
    let frame = response
        .next_frame()
        .expect("steady retained output fits")
        .expect("frame");
    assert_eq!(
        owner
            .registry
            .lock()
            .expect("registry")
            .pending_bytes()
            .expect("checked"),
        8_000_048
    );
    drop(frame);
    drop(generic);
    assert_eq!(
        owner.snapshot().expect("released"),
        FilterTransportSnapshot::default()
    );
}
