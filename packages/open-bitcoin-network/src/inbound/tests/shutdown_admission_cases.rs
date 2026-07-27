// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_handshake.py

use super::*;

#[test]
fn shutdown_request_rejects_admission() {
    // Arrange
    let policy = InboundAdmissionPolicy::new(4, 1);
    let counters = InboundAdmissionCounters::default();
    let mut request = admission_request(
        15,
        "127.0.0.1:20015",
        InboundAdmissionSlotClass::Ordinary,
        counters,
    );
    request.is_shutdown_requested = true;

    // Act
    let decision = policy.decide(request);

    // Assert
    let InboundAdmissionDecision::Reject(rejection) = decision else {
        panic!("expected shutdown rejection");
    };
    assert_eq!(rejection.reason, InboundAdmissionRejectionReason::Shutdown);
}
