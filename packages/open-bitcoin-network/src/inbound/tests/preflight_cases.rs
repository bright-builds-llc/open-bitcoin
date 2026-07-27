// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_handshake.py

use super::*;

#[test]
fn preflight_reason_labels_are_stable() {
    // Arrange
    let reasons = [
        (InboundPreflightReason::Disabled, "disabled"),
        (
            InboundPreflightReason::NoListenAddresses,
            "no_listen_addresses",
        ),
        (InboundPreflightReason::InvalidEndpoint, "invalid_endpoint"),
        (InboundPreflightReason::UnsafeEndpoint, "unsafe_endpoint"),
        (InboundPreflightReason::BindUnavailable, "bind_unavailable"),
        (InboundPreflightReason::AlreadyBound, "already_bound"),
        (InboundPreflightReason::Ready, "ready"),
    ];

    // Act
    let labels: Vec<&str> = reasons
        .into_iter()
        .map(|(reason, _label)| reason.as_str())
        .collect();

    // Assert
    assert_eq!(
        labels,
        vec![
            "disabled",
            "no_listen_addresses",
            "invalid_endpoint",
            "unsafe_endpoint",
            "bind_unavailable",
            "already_bound",
            "ready",
        ],
    );
}

#[test]
fn disabled_preflight_does_not_attempt_bind() {
    // Arrange
    let config = InboundListenerConfig::default();

    // Act
    let plan = classify_inbound_preflight(&config);

    // Assert
    assert_eq!(plan.reason(), InboundPreflightReason::Disabled);
    assert!(!plan.should_attempt_bind());
    assert!(plan.ready_endpoints().is_empty());
    assert_eq!(plan.diagnostics()[0].maybe_endpoint, None);
}

#[test]
fn enabled_preflight_requires_listen_addresses() {
    // Arrange
    let config = enabled_config(Vec::new());

    // Act
    let plan = classify_inbound_preflight(&config);

    // Assert
    let diagnostic = &plan.diagnostics()[0];
    assert_eq!(diagnostic.reason, InboundPreflightReason::NoListenAddresses);
    assert_eq!(diagnostic.field, "inbound.listen_addresses");
    assert!(!plan.should_attempt_bind());
}

#[test]
fn loopback_preflight_returns_ready_normalized_endpoints() {
    // Arrange
    let config = enabled_config(vec!["127.0.0.1:18444", "[::1]:18444"]);

    // Act
    let plan = classify_inbound_preflight(&config);

    // Assert
    assert_eq!(plan.reason(), InboundPreflightReason::Ready);
    assert!(plan.should_attempt_bind());
    assert_eq!(plan.ready_endpoints()[0].normalized, "127.0.0.1:18444");
    assert_eq!(plan.ready_endpoints()[1].normalized, "[::1]:18444");
}

#[test]
fn activation_diagnostics_represent_os_observed_bind_results() {
    // Arrange
    let config = enabled_config(vec!["127.0.0.1:18444"]);
    let plan = classify_inbound_preflight(&config);
    let endpoint = &plan.ready_endpoints()[0];

    // Act
    let bind_unavailable =
        InboundListenerActivationDiagnostic::bind_unavailable(endpoint, "address unavailable");
    let already_bound =
        InboundListenerActivationDiagnostic::already_bound(endpoint, "address in use");

    // Assert
    assert_eq!(
        bind_unavailable.reason,
        InboundPreflightReason::BindUnavailable,
    );
    assert_eq!(already_bound.reason, InboundPreflightReason::AlreadyBound);
    assert_eq!(
        already_bound.maybe_endpoint.as_deref(),
        Some("127.0.0.1:18444"),
    );
}

#[test]
fn activation_diagnostic_converts_to_preflight_shape() {
    // Arrange
    let config = enabled_config(vec!["127.0.0.1:18444"]);
    let plan = classify_inbound_preflight(&config);
    let endpoint = &plan.ready_endpoints()[0];
    let activation = InboundListenerActivationDiagnostic::already_bound(endpoint, "address in use");

    // Act
    let diagnostic = activation.into_preflight_diagnostic();

    // Assert
    assert_eq!(diagnostic.reason, InboundPreflightReason::AlreadyBound);
    assert_eq!(
        diagnostic.maybe_endpoint.as_deref(),
        Some("127.0.0.1:18444"),
    );
    assert_eq!(diagnostic.field, "inbound.listen_addresses");
}
