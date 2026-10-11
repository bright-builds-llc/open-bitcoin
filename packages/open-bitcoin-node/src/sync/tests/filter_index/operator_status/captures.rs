// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;

#[test]
fn phase161_operator_authority_detailed_private_owner_failures_coexist_with_genuine_retention() {
    // Arrange
    let history = ValidatedHistory::new("operator-failure-map", false);
    let runtime = opened(&history);
    for (failure, expected) in [
        (
            AcceptedBasicIndexFailure::InvalidFacts,
            BasicIndexFailureCategory::IntegrityOrInvalidFacts,
        ),
        (
            AcceptedBasicIndexFailure::FactLimit,
            BasicIndexFailureCategory::ResourceBound,
        ),
        (
            AcceptedBasicIndexFailure::Invalidated,
            BasicIndexFailureCategory::Invalidated,
        ),
    ] {
        runtime
            .network
            .note_basic_index_failure(failure)
            .expect("private controlled owner failure");
        let rows = runtime.store().basic_operator_namespaces_for_test();
        // Act
        let status = capture(&runtime);
        // Assert
        assert_eq!(
            status.maybe_failure,
            FieldAvailability::available(Some(expected))
        );
        assert_eq!(
            status.retained_availability,
            FieldAvailability::available(BasicRetainedAvailability::Observed)
        );
        assert_eq!(
            status.initial_sync_complete,
            FieldAvailability::available(true)
        );
        assert_eq!(runtime.store().basic_operator_namespaces_for_test(), rows);
        verify_caller_capture(
            match expected {
                BasicIndexFailureCategory::IntegrityOrInvalidFacts => "owner-integrity",
                BasicIndexFailureCategory::ResourceBound => "owner-resource",
                BasicIndexFailureCategory::Invalidated => "owner-invalidated",
                _ => unreachable!("enumerated owner controls"),
            },
            &status,
        );
    }
    drop(runtime);
    history.cleanup();
}

/// Committed safe captures bridge private producer controls to public clients.
/// Explicit generation must run serially; ordinary tests only check freshness.
pub(in crate::sync::tests::filter_index) fn verify_caller_capture(
    name: &str,
    status: &BasicIndexOperatorStatus,
) {
    let value = serde_json::to_value(status).expect("safe BASIC capture");
    assert!(value.to_string().len() <= 4096);
    assert!(CAPTURE_CASES.contains(&name), "unknown producer case");
    if let Some(generation) = std::env::var_os("OPEN_BITCOIN_GENERATE_BASIC_CAPTURES") {
        assert!(generation == "1", "capture generation requires exact env=1");
        assert!(
            std::env::args().any(|arg| arg == "--test-threads=1"),
            "capture generation must be serial"
        );
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("testdata/basic_operator_captures.json");
        let mut captures: serde_json::Value =
            serde_json::from_slice(&fs::read(&fixture).expect("existing capture fixture"))
                .expect("capture fixture JSON");
        captures[name] = value;
        fs::write(
            fixture,
            format!(
                "{}\n",
                serde_json::to_string_pretty(&captures).expect("capture JSON")
            ),
        )
        .expect("explicit safe fixture generation");
        return;
    }
    let captures: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../../testdata/basic_operator_captures.json"
    ))
    .expect("committed producer captures");
    compare_fixture(&captures, name, &value);
}

fn compare_fixture(captures: &serde_json::Value, name: &str, value: &serde_json::Value) {
    validate_fixture(captures);
    assert_eq!(
        captures[name], *value,
        "stale genuine producer capture: {name}"
    );
}

const CAPTURE_CASES: [&str; 9] = [
    "owner-integrity",
    "owner-invalidated",
    "owner-resource",
    "persistence-after-commit",
    "persistence-before-records",
    "read-backend",
    "read-integrity",
    "required-body",
    "required-undo",
];

fn validate_fixture(captures: &serde_json::Value) {
    let object = captures.as_object().expect("flat safe capture catalog");
    assert_eq!(
        object.keys().map(String::as_str).collect::<Vec<_>>(),
        CAPTURE_CASES
    );
    for value in object.values() {
        assert!(value.to_string().len() <= 4096);
        let typed: BasicIndexOperatorStatus =
            serde_json::from_value(value.clone()).expect("typed safe capture");
        assert_eq!(
            serde_json::to_value(typed).expect("safe round trip"),
            *value,
            "fixture has unknown or private fields"
        );
    }
}

#[test]
fn phase161_capture_fixture_rejects_missing_extra_private_malformed_oversized_and_stale() {
    // Arrange
    let baseline: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../../testdata/basic_operator_captures.json"
    ))
    .expect("committed captures");
    validate_fixture(&baseline);
    let mut missing = baseline.clone();
    missing
        .as_object_mut()
        .expect("catalog")
        .remove("read-backend");
    let mut extra = baseline.clone();
    extra["extra"] = baseline["read-backend"].clone();
    let mut private = baseline.clone();
    private["read-backend"]["storage_key"] = serde_json::json!("private-marker");
    let mut malformed = baseline.clone();
    malformed["read-backend"]["configured_indexing"]["value"] = serde_json::json!("true");
    let mut oversized = baseline.clone();
    oversized["read-backend"]["storage_key"] = serde_json::json!("x".repeat(4096));
    let mut stale = baseline.clone();
    stale["read-backend"]["accepted_height"]["value"] = serde_json::json!(99);
    // Act / Assert: mutations are rejected through the same normal-test comparator.
    for invalid in [missing, extra, private, malformed, oversized, stale] {
        assert!(
            std::panic::catch_unwind(|| compare_fixture(
                &invalid,
                "read-backend",
                &baseline["read-backend"]
            ))
            .is_err()
        );
    }
}
