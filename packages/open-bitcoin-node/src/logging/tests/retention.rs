// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn retention_prunes_by_max_files() {
    // Arrange
    let files = vec![
        log_metadata(10, 100),
        log_metadata(11, 100),
        log_metadata(12, 100),
        LogFileMetadata::new(PathBuf::from("debug.log"), 100),
    ];
    let policy = LogRetentionPolicy {
        max_files: 2,
        max_age_days: 30,
        max_total_bytes: 1_000,
        ..LogRetentionPolicy::default()
    };

    // Act
    let selected = plan_log_retention(&files, policy, 12 * 86_400);

    // Assert
    assert_eq!(selected, vec![managed_path(10)]);
}

#[test]
fn retention_prunes_by_max_age() {
    // Arrange
    let files = vec![
        log_metadata(17, 100),
        log_metadata(18, 100),
        log_metadata(20, 100),
    ];
    let policy = LogRetentionPolicy {
        max_files: 10,
        max_age_days: 2,
        max_total_bytes: 1_000,
        ..LogRetentionPolicy::default()
    };

    // Act
    let selected = plan_log_retention(&files, policy, 20 * 86_400);

    // Assert
    assert_eq!(selected, vec![managed_path(17)]);
}

#[test]
fn retention_prunes_by_total_bytes() {
    // Arrange
    let files = vec![
        log_metadata(10, 75),
        log_metadata(11, 75),
        log_metadata(12, 75),
    ];
    let policy = LogRetentionPolicy {
        max_files: 10,
        max_age_days: 30,
        max_total_bytes: 150,
        ..LogRetentionPolicy::default()
    };

    // Act
    let selected = plan_log_retention(&files, policy, 12 * 86_400);

    // Assert
    assert_eq!(selected, vec![managed_path(10)]);
}
