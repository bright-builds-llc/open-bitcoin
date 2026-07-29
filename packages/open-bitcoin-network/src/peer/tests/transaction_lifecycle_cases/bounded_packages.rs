// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.cpp

use super::*;

const ACCEPTED_PACKAGE_CAP: usize = 100;
const ACCEPTED_PACKAGE_MEMBER_CAP: usize = 25;

fn empty_package(index: usize) -> AcceptedPeerPackageFingerprint {
    AcceptedPeerPackageFingerprint::new([index as u8; 32], Vec::new())
}

fn package(index: usize, members: Vec<PeerTransactionIdentity>) -> AcceptedPeerPackageFingerprint {
    AcceptedPeerPackageFingerprint::new([index as u8; 32], members)
}

fn accepted_package_input(
    packages: Vec<AcceptedPeerPackageFingerprint>,
) -> PeerTransactionLifecycleInput {
    PeerTransactionLifecycleInput::new(Vec::new(), Vec::new(), packages)
}

fn apply_packages(manager: &mut PeerManager, packages: Vec<AcceptedPeerPackageFingerprint>) {
    let prepared = manager
        .prepare_transaction_lifecycle(accepted_package_input(packages))
        .expect("bounded accepted packages should prepare");
    manager.apply_prepared_transaction_lifecycle(prepared);
}

#[test]
fn accepts_raw_accepted_package_command_at_exact_cap() {
    // Arrange
    let manager = PeerManager::new(local_config());
    let packages = (0..ACCEPTED_PACKAGE_CAP)
        .map(empty_package)
        .collect::<Vec<_>>();

    // Act
    let result = manager.prepare_transaction_lifecycle(accepted_package_input(packages));

    // Assert
    assert!(result.is_ok());
}

#[test]
fn rejects_raw_unique_accepted_package_command_at_cap_plus_one_without_mutation() {
    // Arrange
    let manager = PeerManager::new(local_config());
    let packages = (0..=ACCEPTED_PACKAGE_CAP)
        .map(empty_package)
        .collect::<Vec<_>>();
    let baseline = format!("{manager:?}");

    // Act
    let result = manager.prepare_transaction_lifecycle(accepted_package_input(packages));

    // Assert
    assert_eq!(
        result.err(),
        Some(
            PeerTransactionLifecyclePreparationError::AcceptedPackageCountLimit {
                count: ACCEPTED_PACKAGE_CAP + 1,
                maximum: ACCEPTED_PACKAGE_CAP,
            }
        )
    );
    assert_eq!(format!("{manager:?}"), baseline);
}

#[test]
fn rejects_raw_duplicate_only_accepted_package_command_at_cap_plus_one_before_deduplication() {
    // Arrange
    let manager = PeerManager::new(local_config());
    let duplicate = package(1, vec![identity(1)]);
    let packages = vec![duplicate; ACCEPTED_PACKAGE_CAP + 1];
    let baseline = format!("{manager:?}");

    // Act
    let result = manager.prepare_transaction_lifecycle(accepted_package_input(packages));

    // Assert
    assert_eq!(
        result.err(),
        Some(
            PeerTransactionLifecyclePreparationError::AcceptedPackageCountLimit {
                count: ACCEPTED_PACKAGE_CAP + 1,
                maximum: ACCEPTED_PACKAGE_CAP,
            }
        )
    );
    assert_eq!(format!("{manager:?}"), baseline);
}

#[test]
fn bounded_duplicate_admission_is_idempotent_in_prepared_work_and_state() {
    // Arrange
    let accepted = package(2, vec![identity(2), identity(3)]);
    let mut manager = PeerManager::new(local_config());
    apply_packages(&mut manager, vec![accepted.clone()]);
    let baseline = format!("{manager:?}");

    // Act
    let prepared = manager
        .prepare_transaction_lifecycle(accepted_package_input(vec![accepted.clone(), accepted]))
        .expect("bounded duplicates should prepare");
    let prepared_admission_count = prepared.debug_fingerprint_admission_count();
    manager.apply_prepared_transaction_lifecycle(prepared);

    // Assert
    assert_eq!(prepared_admission_count, 0);
    assert_eq!(manager.debug_accepted_package_fingerprint_count(), 1);
    assert_eq!(format!("{manager:?}"), baseline);
}

#[test]
fn accepts_exact_stored_fingerprint_cap() {
    // Arrange
    let packages = (0..ACCEPTED_PACKAGE_CAP)
        .map(empty_package)
        .collect::<Vec<_>>();
    let mut manager = PeerManager::new(local_config());

    // Act
    apply_packages(&mut manager, packages);

    // Assert
    assert_eq!(
        manager.debug_accepted_package_fingerprint_count(),
        ACCEPTED_PACKAGE_CAP
    );
}

#[test]
fn rejects_stored_fingerprint_cap_plus_one_without_mutation() {
    // Arrange
    let packages = (0..ACCEPTED_PACKAGE_CAP)
        .map(empty_package)
        .collect::<Vec<_>>();
    let mut manager = PeerManager::new(local_config());
    apply_packages(&mut manager, packages);
    let baseline = format!("{manager:?}");

    // Act
    let result =
        manager.prepare_transaction_lifecycle(accepted_package_input(vec![empty_package(
            ACCEPTED_PACKAGE_CAP,
        )]));

    // Assert
    assert_eq!(
        result.err(),
        Some(PeerTransactionLifecyclePreparationError::FingerprintLimit {
            count: ACCEPTED_PACKAGE_CAP + 1,
            maximum: ACCEPTED_PACKAGE_CAP,
        })
    );
    assert_eq!(format!("{manager:?}"), baseline);
}

#[test]
fn package_member_cap_is_independent_of_fingerprint_count() {
    // Arrange
    let manager = PeerManager::new(local_config());
    let exact_members = (0..ACCEPTED_PACKAGE_MEMBER_CAP)
        .map(|index| identity(index as u8))
        .collect::<Vec<_>>();
    let oversized_members = (0..=ACCEPTED_PACKAGE_MEMBER_CAP)
        .map(|index| identity(index as u8))
        .collect::<Vec<_>>();
    let exact = accepted_package_input(vec![package(40, exact_members), empty_package(41)]);
    let oversized = accepted_package_input(vec![package(42, oversized_members)]);
    let baseline = format!("{manager:?}");

    // Act
    let exact_result = manager.prepare_transaction_lifecycle(exact);
    let oversized_result = manager.prepare_transaction_lifecycle(oversized);

    // Assert
    assert!(exact_result.is_ok());
    assert_eq!(
        oversized_result.err(),
        Some(
            PeerTransactionLifecyclePreparationError::PackageMemberLimit {
                count: ACCEPTED_PACKAGE_MEMBER_CAP + 1,
                maximum: ACCEPTED_PACKAGE_MEMBER_CAP,
            }
        )
    );
    assert_eq!(format!("{manager:?}"), baseline);
}

#[test]
fn full_cache_can_retire_and_replace_at_exact_capacity() {
    // Arrange
    let teardown = identity(220);
    let retired = package(0, vec![teardown]);
    let replacement = empty_package(100);
    let mut initial = vec![retired.clone()];
    initial.extend((1..ACCEPTED_PACKAGE_CAP).map(empty_package));
    let mut manager = PeerManager::new(local_config());
    apply_packages(&mut manager, initial);

    // Act
    let prepared = manager
        .prepare_transaction_lifecycle(PeerTransactionLifecycleInput::new(
            Vec::new(),
            vec![teardown],
            vec![replacement.clone()],
        ))
        .expect("same-transition retirement should free replacement capacity");
    manager.apply_prepared_transaction_lifecycle(prepared);

    // Assert
    assert_eq!(
        manager.debug_accepted_package_fingerprint_count(),
        ACCEPTED_PACKAGE_CAP
    );
    assert!(
        !manager
            .orphanage
            .accepted_package_fingerprints()
            .any(|(fingerprint, _)| *fingerprint == retired.fingerprint())
    );
    assert!(
        manager
            .orphanage
            .accepted_package_fingerprints()
            .any(|(fingerprint, _)| *fingerprint == replacement.fingerprint())
    );
}

#[test]
fn partial_retirement_still_over_cap_fails_without_mutation() {
    // Arrange
    let teardown = identity(221);
    let mut initial = vec![package(0, vec![teardown])];
    initial.extend((1..ACCEPTED_PACKAGE_CAP).map(empty_package));
    let manager = {
        let mut manager = PeerManager::new(local_config());
        apply_packages(&mut manager, initial);
        manager
    };
    let baseline = format!("{manager:?}");

    // Act
    let result = manager.prepare_transaction_lifecycle(PeerTransactionLifecycleInput::new(
        Vec::new(),
        vec![teardown],
        vec![
            empty_package(ACCEPTED_PACKAGE_CAP),
            empty_package(ACCEPTED_PACKAGE_CAP + 1),
        ],
    ));

    // Assert
    assert_eq!(
        result.err(),
        Some(PeerTransactionLifecyclePreparationError::FingerprintLimit {
            count: ACCEPTED_PACKAGE_CAP + 1,
            maximum: ACCEPTED_PACKAGE_CAP,
        })
    );
    assert_eq!(format!("{manager:?}"), baseline);
}

#[test]
fn duplicate_replacement_does_not_consume_final_capacity() {
    // Arrange
    let teardown = identity(222);
    let existing = empty_package(1);
    let replacement = empty_package(100);
    let mut initial = vec![package(0, vec![teardown])];
    initial.extend((1..ACCEPTED_PACKAGE_CAP).map(empty_package));
    let mut manager = PeerManager::new(local_config());
    apply_packages(&mut manager, initial);

    // Act
    let prepared = manager
        .prepare_transaction_lifecycle(PeerTransactionLifecycleInput::new(
            Vec::new(),
            vec![teardown],
            vec![existing, replacement.clone(), replacement],
        ))
        .expect("identical duplicates should not consume replacement capacity");
    let prepared_admission_count = prepared.debug_fingerprint_admission_count();
    manager.apply_prepared_transaction_lifecycle(prepared);

    // Assert
    assert_eq!(prepared_admission_count, 1);
    assert_eq!(
        manager.debug_accepted_package_fingerprint_count(),
        ACCEPTED_PACKAGE_CAP
    );
}
