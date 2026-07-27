// Parity breadcrumbs:
// - packages/bitcoin-knots/src/policy/packages.h
// - packages/bitcoin-knots/src/policy/packages.cpp
// - packages/bitcoin-knots/src/validation.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/test/txpackage_tests.cpp

use super::*;

#[test]
fn ephemeral_policy_failure_preserves_requested_identity() {
    // Arrange
    let requested = MempoolMemberIdentity {
        txid: Txid::from_byte_array([0x99; 32]),
        wtxid: Wtxid::from_byte_array([0x9a; 32]),
    };
    let result = PackageMemberResult::HardRejected(HardMemberFailure::EphemeralPolicy {
        requested,
        reason: "missing ephemeral spends".to_string(),
    });

    // Act / Assert
    assert_eq!(result.requested_identity(), requested);
}
