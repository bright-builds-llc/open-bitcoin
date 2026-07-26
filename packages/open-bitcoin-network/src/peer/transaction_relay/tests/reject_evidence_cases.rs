// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/bloom.h
// - packages/bitcoin-knots/src/common/bloom.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.h
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp

use open_bitcoin_primitives::Wtxid;

use super::super::{
    HardRejectEvidence, PHASE133_REJECT_FILTER_CAPACITY,
    PHASE133_REJECT_FILTER_FALSE_POSITIVE_RATE, ReconsiderableEvidenceKey,
    ReconsiderableRejectEvidence, RejectEvidenceConfigError, RejectEvidenceTweak,
};

const EXPECTED_WORD_COUNT: usize = 161_750;
const EXPECTED_ENTRIES_PER_GENERATION: usize = 60_000;
const EXPECTED_PROBE_COUNT: usize = 20;

fn wtxid_from_index(index: u64) -> Wtxid {
    let mut bytes = [0_u8; 32];
    bytes[..8].copy_from_slice(&index.to_le_bytes());
    bytes[8..16].copy_from_slice(&index.rotate_left(17).to_le_bytes());
    bytes[16..24].copy_from_slice(&index.rotate_left(31).to_le_bytes());
    bytes[24..].copy_from_slice(&index.rotate_left(47).to_le_bytes());
    Wtxid::from_byte_array(bytes)
}

#[test]
fn locked_parameters_derive_knots_sizing() {
    // Arrange
    let evidence = HardRejectEvidence::new(RejectEvidenceTweak::new(7));

    // Act
    let derived = HardRejectEvidence::try_with_parameters(
        PHASE133_REJECT_FILTER_CAPACITY,
        PHASE133_REJECT_FILTER_FALSE_POSITIVE_RATE,
        RejectEvidenceTweak::new(7),
    );
    let Ok(derived) = derived else {
        panic!("locked reject evidence parameters must be valid");
    };

    // Assert
    assert_eq!(PHASE133_REJECT_FILTER_CAPACITY, 120_000);
    assert_eq!(PHASE133_REJECT_FILTER_FALSE_POSITIVE_RATE, 0.000_001);
    assert_eq!(
        evidence.debug_entries_per_generation(),
        EXPECTED_ENTRIES_PER_GENERATION,
    );
    assert_eq!(evidence.debug_probe_count(), EXPECTED_PROBE_COUNT);
    assert_eq!(evidence.debug_storage_len(), EXPECTED_WORD_COUNT);
    assert_eq!(evidence.debug_storage_capacity(), EXPECTED_WORD_COUNT);
    assert_eq!(
        derived.debug_entries_per_generation(),
        evidence.debug_entries_per_generation(),
    );
    assert_eq!(derived.debug_probe_count(), evidence.debug_probe_count());
    assert_eq!(derived.debug_storage_len(), evidence.debug_storage_len());
}

#[test]
fn invalid_parameters_return_typed_errors() {
    // Arrange / Act
    let zero_capacity = HardRejectEvidence::try_with_parameters(
        0,
        PHASE133_REJECT_FILTER_FALSE_POSITIVE_RATE,
        RejectEvidenceTweak::new(1),
    );
    let zero_false_positive_rate = HardRejectEvidence::try_with_parameters(
        PHASE133_REJECT_FILTER_CAPACITY,
        0.0,
        RejectEvidenceTweak::new(1),
    );
    let unit_false_positive_rate = HardRejectEvidence::try_with_parameters(
        PHASE133_REJECT_FILTER_CAPACITY,
        1.0,
        RejectEvidenceTweak::new(1),
    );
    let overflowing_capacity = HardRejectEvidence::try_with_parameters(
        usize::MAX,
        PHASE133_REJECT_FILTER_FALSE_POSITIVE_RATE,
        RejectEvidenceTweak::new(1),
    );
    let oversized_filter = HardRejectEvidence::try_with_parameters(
        200_000_000,
        PHASE133_REJECT_FILTER_FALSE_POSITIVE_RATE,
        RejectEvidenceTweak::new(1),
    );

    // Assert
    assert_eq!(zero_capacity, Err(RejectEvidenceConfigError::ZeroCapacity),);
    assert_eq!(
        zero_false_positive_rate,
        Err(RejectEvidenceConfigError::InvalidFalsePositiveRate),
    );
    assert_eq!(
        unit_false_positive_rate,
        Err(RejectEvidenceConfigError::InvalidFalsePositiveRate),
    );
    assert_eq!(
        overflowing_capacity,
        Err(RejectEvidenceConfigError::ArithmeticOverflow),
    );
    assert_eq!(
        oversized_filter,
        Err(RejectEvidenceConfigError::FilterTooLarge),
    );
}

#[test]
fn generation_rotation_retains_the_guaranteed_window_and_reuses_labels() {
    // Arrange
    let mut evidence = HardRejectEvidence::new(RejectEvidenceTweak::new(11));
    let first = wtxid_from_index(0);
    let newest_before_reuse = wtxid_from_index(180_000);

    // Act
    for index in 0..=180_000 {
        evidence.record(wtxid_from_index(index));
    }

    // Assert
    assert!(!evidence.contains(first));
    assert!(evidence.contains(wtxid_from_index(60_000)));
    assert!(evidence.contains(wtxid_from_index(120_000)));
    assert!(evidence.contains(newest_before_reuse));
    assert_eq!(evidence.debug_generation(), 1);
    assert_eq!(evidence.debug_entries_this_generation(), 1);
}

#[test]
fn reset_clears_membership_and_reseeds_deterministically() {
    // Arrange
    let transaction = wtxid_from_index(42);
    let mut evidence = HardRejectEvidence::new(RejectEvidenceTweak::new(19));
    evidence.record(transaction);
    assert!(evidence.contains(transaction));

    // Act
    evidence.reset(RejectEvidenceTweak::new(23));

    // Assert
    assert!(!evidence.contains(transaction));
    assert_eq!(evidence.debug_generation(), 1);
    assert_eq!(evidence.debug_entries_this_generation(), 0);
    assert_eq!(evidence.debug_tweak(), RejectEvidenceTweak::new(23));
}

#[test]
fn typed_domains_keep_transactions_and_packages_separate() {
    // Arrange
    let transaction = wtxid_from_index(73);
    let package = *transaction.as_bytes();
    let mut hard = HardRejectEvidence::new(RejectEvidenceTweak::new(29));
    let mut reconsiderable = ReconsiderableRejectEvidence::new(RejectEvidenceTweak::new(29));

    // Act
    hard.record(transaction);
    reconsiderable.record(ReconsiderableEvidenceKey::Transaction(transaction));
    reconsiderable.record(ReconsiderableEvidenceKey::Package(package));

    // Assert
    assert!(hard.contains(transaction));
    assert!(reconsiderable.contains(ReconsiderableEvidenceKey::Transaction(transaction,)));
    assert!(reconsiderable.contains(ReconsiderableEvidenceKey::Package(package,)));

    // Act
    reconsiderable.reset(RejectEvidenceTweak::new(30));

    // Assert
    assert!(!reconsiderable.contains(ReconsiderableEvidenceKey::Transaction(transaction,)));
    assert!(!reconsiderable.contains(ReconsiderableEvidenceKey::Package(package,)));
}

#[test]
fn fixed_tweak_membership_vectors_are_deterministic() {
    // Arrange
    let inserted = [
        wtxid_from_index(1),
        wtxid_from_index(2),
        wtxid_from_index(3),
    ];
    let absent = [wtxid_from_index(4), wtxid_from_index(5)];
    let mut first = HardRejectEvidence::new(RejectEvidenceTweak::new(0x1330_0001));
    let mut second = HardRejectEvidence::new(RejectEvidenceTweak::new(0x1330_0001));

    // Act
    for transaction in inserted {
        first.record(transaction);
        second.record(transaction);
    }

    // Assert
    for transaction in inserted {
        assert!(first.contains(transaction));
        assert!(second.contains(transaction));
    }
    for transaction in absent {
        assert!(!first.contains(transaction));
        assert!(!second.contains(transaction));
    }
    assert_eq!(first.debug_storage_checksum(), 342_094_158_989_959_191);
    assert_eq!(
        first.debug_storage_checksum(),
        second.debug_storage_checksum(),
    );
}

#[test]
fn one_million_unique_insertions_do_not_grow_allocation() {
    // Arrange
    let mut evidence = HardRejectEvidence::new(RejectEvidenceTweak::new(31));
    let initial_len = evidence.debug_storage_len();
    let initial_capacity = evidence.debug_storage_capacity();

    // Act
    for index in 0..1_000_000 {
        evidence.record(wtxid_from_index(index));
    }

    // Assert
    assert_eq!(evidence.debug_storage_len(), initial_len);
    assert_eq!(evidence.debug_storage_capacity(), initial_capacity);
    assert!(evidence.contains(wtxid_from_index(999_999)));
    assert!(evidence.contains(wtxid_from_index(940_000)));
}
