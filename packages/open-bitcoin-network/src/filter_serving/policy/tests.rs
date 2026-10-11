// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/chain.cpp

use super::*;
use open_bitcoin_primitives::{BlockHeader, MerkleRoot};

fn chain(height: u32) -> (HeaderStore, Vec<BlockHash>) {
    let mut store = HeaderStore::default();
    let mut hashes = Vec::new();
    let mut parent = BlockHash::default();
    for nonce in 0..=height {
        let inserted = store
            .insert_header(BlockHeader {
                version: 1,
                previous_block_hash: parent,
                merkle_root: MerkleRoot::default(),
                time: nonce,
                bits: 0x207f_ffff,
                nonce,
            })
            .expect("chain");
        parent = inserted.block_hash;
        hashes.push(parent);
    }
    (store, hashes)
}

fn unlimited() -> BranchWorkLimit {
    BranchWorkLimit {
        ancestry_steps: 100_000,
        identity_bytes: 1_000_000,
    }
}
fn validate(
    request: FilterRequest,
    store: &HeaderStore,
) -> Result<ValidatedFilterRequest, FilterRequestInvalid> {
    validate_filter_request(
        request,
        ServiceFlags::COMPACT_FILTERS,
        store,
        StopProvenance::Active,
        600,
    )
}

#[test]
fn phase160_range_policy_inclusive_limits_and_genesis() {
    // Arrange
    let (store, hashes) = chain(2000);
    for (headers, limit) in [(false, 1000_usize), (true, 2000)] {
        let request = if headers {
            FilterRequest::Headers {
                filter_type: 0,
                start_height: 0,
                stop_hash: hashes[limit - 1],
            }
        } else {
            FilterRequest::Filters {
                filter_type: 0,
                start_height: 0,
                stop_hash: hashes[limit - 1],
            }
        };
        // Act
        let branch = resolve_filter_branch(
            validate(request, &store).expect("limit accepted"),
            &store,
            unlimited(),
        )
        .expect("resolved");
        // Assert
        assert_eq!(branch.identities.len(), limit);
        assert!(branch.maybe_predecessor.is_none());
        let oversized = if headers {
            FilterRequest::Headers {
                filter_type: 0,
                start_height: 0,
                stop_hash: hashes[limit],
            }
        } else {
            FilterRequest::Filters {
                filter_type: 0,
                start_height: 0,
                stop_hash: hashes[limit],
            }
        };
        assert_eq!(
            validate(oversized, &store),
            Err(FilterRequestInvalid::TooManyEntries)
        );
    }
}

#[test]
fn phase160_range_policy_gate_order_precedes_saturated_work() {
    // Arrange
    let (store, hashes) = chain(1);
    let unknown = BlockHash::from_byte_array([9; 32]);
    for (kind, stop, start, services, expected) in [
        (
            2,
            unknown,
            9,
            ServiceFlags::COMPACT_FILTERS,
            FilterRequestInvalid::UnsupportedTypeOrCapability,
        ),
        (
            0,
            unknown,
            9,
            ServiceFlags::NONE,
            FilterRequestInvalid::UnsupportedTypeOrCapability,
        ),
        (
            0,
            unknown,
            9,
            ServiceFlags::COMPACT_FILTERS,
            FilterRequestInvalid::UnknownOrDisallowedStop,
        ),
        (
            0,
            hashes[1],
            9,
            ServiceFlags::COMPACT_FILTERS,
            FilterRequestInvalid::ReversedRange,
        ),
    ] {
        // Act
        let result = validate_filter_request(
            FilterRequest::Filters {
                filter_type: kind,
                start_height: start,
                stop_hash: stop,
            },
            services,
            &store,
            StopProvenance::Active,
            600,
        );
        // Assert
        assert_eq!(result, Err(expected));
    }
    let valid = validate(
        FilterRequest::Filters {
            filter_type: 0,
            start_height: 0,
            stop_hash: hashes[0],
        },
        &store,
    )
    .expect("semantic");
    assert_eq!(
        resolve_filter_branch(
            valid,
            &store,
            BranchWorkLimit {
                ancestry_steps: 0,
                identity_bytes: 0
            }
        ),
        Err(BranchResourceRefusal::AncestryWork)
    );
}

#[test]
fn phase160_range_policy_checkpoint_positive_intervals_and_local_limits() {
    // Arrange
    let (store, hashes) = chain(3001);
    for height in [0, 999, 1000, 1001, 2000, 3001] {
        let request = FilterRequest::Checkpoints {
            filter_type: 0,
            stop_hash: hashes[height],
        };
        // Act
        let branch = resolve_filter_branch(
            validate(request, &store).expect("semantic"),
            &store,
            unlimited(),
        )
        .expect("branch");
        // Assert
        assert_eq!(
            branch
                .identities
                .iter()
                .map(|id| id.height)
                .collect::<Vec<_>>(),
            (1..=height as u32 / 1000)
                .map(|interval| interval * 1000)
                .collect::<Vec<_>>()
        );
    }
    let valid = validate(
        FilterRequest::Checkpoints {
            filter_type: 0,
            stop_hash: hashes[3001],
        },
        &store,
    )
    .expect("no protocol cap");
    assert_eq!(
        resolve_filter_branch(
            valid,
            &store,
            BranchWorkLimit {
                ancestry_steps: 2001,
                identity_bytes: 1000
            }
        ),
        Err(BranchResourceRefusal::AncestryWork)
    );
    assert_eq!(
        resolve_filter_branch(
            valid,
            &store,
            BranchWorkLimit {
                ancestry_steps: 2002,
                identity_bytes: 271
            }
        ),
        Err(BranchResourceRefusal::IdentityBytes)
    );
    assert!(
        resolve_filter_branch(
            valid,
            &store,
            BranchWorkLimit {
                ancestry_steps: 2002,
                identity_bytes: 272
            }
        )
        .is_ok()
    );
}

#[test]
fn phase160_range_policy_stale_signed_timestamp_and_exact_age() {
    // Arrange
    let (mut store, hashes) = chain(1);
    let request = FilterRequest::Filters {
        filter_type: 0,
        start_height: 1,
        stop_hash: hashes[1],
    };
    let add_best = |store: &mut HeaderStore, time: u32, bits: u32| {
        store
            .insert_header(BlockHeader {
                version: 1,
                previous_block_hash: hashes[0],
                merkle_root: MerkleRoot::default(),
                time,
                bits,
                nonce: time,
            })
            .expect("best")
    };
    add_best(&mut store, 2_592_001, 0x1d00_ffff);
    // Act / Assert: equal timestamp age rejects despite small exact-work age.
    assert_eq!(
        validate_filter_request(
            request,
            ServiceFlags::COMPACT_FILTERS,
            &store,
            StopProvenance::ScriptsValid,
            600
        ),
        Err(FilterRequestInvalid::UnknownOrDisallowedStop)
    );
    let (mut just_below, _) = chain(1);
    add_best(&mut just_below, 2_592_000, 0x1d00_ffff);
    assert!(
        validate_filter_request(
            request,
            ServiceFlags::COMPACT_FILTERS,
            &just_below,
            StopProvenance::ScriptsValid,
            600
        )
        .is_ok()
    );
    // A future stale timestamp yields signed negative age, which is allowed.
    let (mut future, future_hashes) = chain(1);
    add_best(&mut future, 0, 0x1d00_ffff);
    let future_request = FilterRequest::Filters {
        filter_type: 0,
        start_height: 1,
        stop_hash: future_hashes[1],
    };
    assert!(
        validate_filter_request(
            future_request,
            ServiceFlags::COMPACT_FILTERS,
            &future,
            StopProvenance::ScriptsValid,
            600
        )
        .is_ok()
    );
    // Equal exact proof age rejects even when timestamps are close.
    let (uniform, uniform_hashes) = chain(2);
    let uniform_request = FilterRequest::Filters {
        filter_type: 0,
        start_height: 1,
        stop_hash: uniform_hashes[1],
    };
    assert_eq!(
        validate_filter_request(
            uniform_request,
            ServiceFlags::COMPACT_FILTERS,
            &uniform,
            StopProvenance::ScriptsValid,
            2_592_000
        ),
        Err(FilterRequestInvalid::UnknownOrDisallowedStop)
    );
    assert!(
        validate_filter_request(
            uniform_request,
            ServiceFlags::COMPACT_FILTERS,
            &uniform,
            StopProvenance::ScriptsValid,
            2_591_999
        )
        .is_ok()
    );
}

#[test]
fn phase160_range_policy_nonuniform_proof_rejects_timestamp_and_height_shortcuts() {
    // Arrange
    let (mut store, hashes) = chain(1);
    let hard = store
        .insert_header(BlockHeader {
            version: 2,
            previous_block_hash: hashes[0],
            merkle_root: MerkleRoot::default(),
            time: 2,
            bits: 0x1d00_ffff,
            nonce: 999,
        })
        .expect("hard branch");
    let best = store
        .insert_header(BlockHeader {
            version: 2,
            previous_block_hash: hard.block_hash,
            merkle_root: MerkleRoot::default(),
            time: 3,
            bits: 0x207f_ffff,
            nonce: 1000,
        })
        .expect("easy best header after hard work");
    let request = FilterRequest::Filters {
        filter_type: 0,
        start_height: 1,
        stop_hash: hashes[1],
    };
    // Act
    let outcome = validate_filter_request(
        request,
        ServiceFlags::COMPACT_FILTERS,
        &store,
        StopProvenance::ScriptsValid,
        600,
    );
    // Assert
    assert_eq!(
        store
            .maybe_best_proof_header()
            .expect("selected")
            .block_hash,
        best.block_hash
    );
    assert_eq!(
        store
            .maybe_best_proof_header()
            .expect("best denominator")
            .block_proof,
        open_bitcoin_consensus::chain_proof::block_proof(0x207f_ffff).expect("easy proof")
    );
    assert_eq!(outcome, Err(FilterRequestInvalid::UnknownOrDisallowedStop));
}

#[test]
fn phase160_range_policy_uses_stop_branch_and_predecessor() {
    // Arrange
    let (mut store, hashes) = chain(3);
    let stale = store
        .insert_header(BlockHeader {
            version: 2,
            previous_block_hash: hashes[1],
            merkle_root: MerkleRoot::default(),
            time: 2,
            bits: 0x207f_ffff,
            nonce: 999,
        })
        .expect("fork");
    let request = FilterRequest::Headers {
        filter_type: 0,
        start_height: 2,
        stop_hash: stale.block_hash,
    };
    // Act
    let validated = validate_filter_request(
        request,
        ServiceFlags::COMPACT_FILTERS,
        &store,
        StopProvenance::ScriptsValid,
        600,
    )
    .expect("stale");
    let branch = resolve_filter_branch(validated, &store, unlimited()).expect("branch");
    // Assert
    assert_eq!(
        branch
            .identities
            .iter()
            .map(|id| id.hash)
            .collect::<Vec<_>>(),
        vec![stale.block_hash]
    );
    assert_eq!(
        branch.maybe_predecessor.expect("predecessor").hash,
        hashes[1]
    );
    assert_eq!(branch.ancestry_steps, 2);
    assert_eq!(branch.identity_bytes, 204);
}

#[test]
fn phase160_range_policy_missing_proof_and_invalid_spacing_fail_closed() {
    // Arrange
    let (store, hashes) = chain(1);
    let request = FilterRequest::Filters {
        filter_type: 0,
        start_height: 0,
        stop_hash: hashes[0],
    };
    let mut invalid_entries: Vec<_> = store.entries().cloned().collect();
    for entry in &mut invalid_entries {
        entry.header.bits = 0;
    }
    let invalid = HeaderStore::from_entries(invalid_entries).expect("legacy fixture");
    // Act / Assert
    for (headers, spacing) in [(&store, 0), (&invalid, 600)] {
        assert_eq!(
            validate_filter_request(
                request,
                ServiceFlags::COMPACT_FILTERS,
                headers,
                StopProvenance::ScriptsValid,
                spacing
            ),
            Err(FilterRequestInvalid::UnknownOrDisallowedStop)
        );
    }
    assert_eq!(
        validate_filter_request(
            request,
            ServiceFlags::COMPACT_FILTERS,
            &store,
            StopProvenance::Insufficient,
            600
        ),
        Err(FilterRequestInvalid::UnknownOrDisallowedStop)
    );
}

#[test]
fn phase160_range_policy_changed_or_missing_ancestry_is_local_refusal() {
    // Arrange
    let (store, hashes) = chain(2);
    let request = FilterRequest::Headers {
        filter_type: 0,
        start_height: 1,
        stop_hash: hashes[2],
    };
    let validated = validate(request, &store).expect("semantic");
    let empty = HeaderStore::default();
    let mut entries: Vec<_> = store.entries().cloned().collect();
    for entry in &mut entries {
        if entry.height == 1 {
            entry.height = 8;
        }
    }
    let wrong_height = HeaderStore::from_entries(entries).expect("legacy fixture");
    // Act / Assert
    assert_eq!(
        resolve_filter_branch(validated, &empty, unlimited()),
        Err(BranchResourceRefusal::UnavailableAncestry)
    );
    assert_eq!(
        resolve_filter_branch(validated, &wrong_height, unlimited()),
        Err(BranchResourceRefusal::UnavailableAncestry)
    );
}

#[test]
fn phase160_range_policy_maximum_checkpoint_height_has_no_protocol_cap() {
    // Arrange
    let (store, _) = chain(1);
    let mut entries: Vec<_> = store.entries().cloned().collect();
    let stop = entries
        .iter_mut()
        .find(|entry| entry.height == 1)
        .expect("child");
    stop.height = u32::MAX;
    let stop_hash = stop.block_hash;
    let legacy = HeaderStore::from_entries(entries).expect("fixture");
    // Act
    let validated = validate(
        FilterRequest::Checkpoints {
            filter_type: 0,
            stop_hash,
        },
        &legacy,
    )
    .expect("height alone not invalid");
    let outcome = resolve_filter_branch(validated, &legacy, unlimited());
    // Assert
    assert_eq!(outcome, Err(BranchResourceRefusal::AncestryWork));
}

#[test]
fn phase160_coverage_raw_request_fields_preserve_unsupported_types() {
    // Arrange
    let (store, hashes) = chain(2);
    let cases = [
        (
            FilterRequest::Filters {
                filter_type: 1,
                start_height: u32::MAX,
                stop_hash: hashes[0],
            },
            1,
            hashes[0],
        ),
        (
            FilterRequest::Headers {
                filter_type: 128,
                start_height: u32::MAX,
                stop_hash: hashes[1],
            },
            128,
            hashes[1],
        ),
        (
            FilterRequest::Checkpoints {
                filter_type: u8::MAX,
                stop_hash: hashes[2],
            },
            u8::MAX,
            hashes[2],
        ),
    ];
    for (request, expected_type, expected_stop) in cases {
        // Act
        let raw_fields = (request.filter_type(), request.stop_hash());
        let outcome = validate(request, &store);

        // Assert
        assert_eq!(raw_fields, (expected_type, expected_stop));
        assert_eq!(
            outcome,
            Err(FilterRequestInvalid::UnsupportedTypeOrCapability)
        );
    }
}

#[test]
fn phase160_coverage_stale_stop_without_exact_proof_fails_with_valid_best() {
    // Arrange
    let (mut store, hashes) = chain(0);
    let stop = store
        .insert_header(BlockHeader {
            version: 1,
            previous_block_hash: hashes[0],
            merkle_root: MerkleRoot::default(),
            time: 1,
            bits: 0,
            nonce: 1,
        })
        .expect("known malformed target fixture");
    let request = FilterRequest::Filters {
        filter_type: 0,
        start_height: 1,
        stop_hash: stop.block_hash,
    };

    // Act: injected provenance isolates refusal; no consensus-validation claim.
    let outcome = validate_filter_request(
        request,
        ServiceFlags::COMPACT_FILTERS,
        &store,
        StopProvenance::ScriptsValid,
        600,
    );

    // Assert
    assert!(store.entry(&stop.block_hash).is_some());
    assert_eq!(
        store
            .maybe_best_proof_header()
            .expect("valid best")
            .block_hash,
        hashes[0]
    );
    assert!(store.maybe_proof_facts(stop.block_hash).is_none());
    assert_eq!(outcome, Err(FilterRequestInvalid::UnknownOrDisallowedStop));
}
