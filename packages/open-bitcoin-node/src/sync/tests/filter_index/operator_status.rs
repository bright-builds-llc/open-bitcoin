// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Genuine validated owner observations through the production snapshot authority.

use super::{catch_up::fixtures::TurnHistory, recovery::ValidatedHistory, *};
use crate::{
    chainstate::{BasicFilterStartupMode, filter_index::AcceptedBasicIndexFailure},
    status::{
        BasicIndexFailureCategory, BasicIndexNextAction, BasicIndexOperationalState,
        BasicIndexOperatorStatus, BasicIndexProtection, BasicRetainedAvailability,
        FieldAvailability,
    },
    storage::fjall_store::filters::FilterPublicationFault,
};
use open_bitcoin_core::{
    chainstate::{FlushMode, FlushPolicyTime},
    consensus::{ConsensusParams, ScriptVerifyFlags},
};

fn capture(runtime: &DurableSyncRuntime) -> BasicIndexOperatorStatus {
    let snapshot = runtime
        .network
        .operator_snapshot()
        .expect("authoritative snapshot");
    let status = snapshot.basic_index().clone();
    assert!(serde_json::to_vec(&status).expect("BASIC encoding").len() <= 4096);
    let observation = runtime
        .store()
        .basic_filter_operator_observation()
        .expect("publication capture");
    assert!(observation.work.metadata_reads <= 8);
    assert!(observation.work.metadata_bytes <= 4096);
    status
}

mod captures;
pub(super) use captures::verify_caller_capture;

fn opened(history: &ValidatedHistory) -> DurableSyncRuntime {
    DurableSyncRuntime::open_configured(
        history.seed(1),
        sync_config(),
        BasicFilterStartupMode::Enabled,
    )
    .expect("genuine startup")
}

#[test]
fn phase161_operator_authority_actual_no_genesis_disabled_never_invents_zero_progress() {
    // Arrange
    let directory = reserved_filter_path("operator-no-genesis");
    let runtime = DurableSyncRuntime::open_configured(
        FjallNodeStore::open(&directory).expect("empty Fjall"),
        sync_config(),
        BasicFilterStartupMode::Disabled,
    )
    .expect("disabled empty startup");
    // Act
    let status = capture(&runtime);
    // Assert
    assert_eq!(
        status.configured_indexing,
        FieldAvailability::available(false)
    );
    assert_eq!(
        status.operational_state,
        FieldAvailability::available(BasicIndexOperationalState::Disabled)
    );
    assert_eq!(
        status.retained_availability,
        FieldAvailability::available(BasicRetainedAvailability::NoneSeen)
    );
    assert!(matches!(
        status.maybe_processed_height,
        FieldAvailability::Unavailable { .. }
    ));
    assert!(matches!(
        status.maybe_safe_durable_height,
        FieldAvailability::Unavailable { .. }
    ));
    assert!(matches!(
        status.initial_sync_complete,
        FieldAvailability::Unavailable { .. }
    ));
    drop(runtime);
    fs::remove_dir_all(directory).expect("cleanup");
}

#[test]
fn phase161_operator_authority_actual_current_and_readonly_polling_at_two_history_sizes() {
    for prefix in [16, 128] {
        // Arrange
        let fixture = TurnHistory::new(prefix, 1, 1);
        let runtime = DurableSyncRuntime::open_configured(
            fixture.seed(prefix),
            sync_config(),
            BasicFilterStartupMode::Enabled,
        )
        .expect("configured startup");
        let rows = runtime.store().basic_operator_namespaces_for_test();
        let scans = runtime.store().basic_filter_validation_scans_for_test();
        let point_reads = runtime.store().basic_filter_point_reads_for_test();
        let owner = runtime.network.maybe_basic_index_progress().expect("owner");
        let expected = capture(&runtime);
        // Act
        let polling = std::time::Instant::now();
        let mut maximum_capture_ns = 0;
        for _ in 0..64 {
            let started = std::time::Instant::now();
            assert_eq!(capture(&runtime), expected);
            maximum_capture_ns = maximum_capture_ns.max(started.elapsed().as_nanos());
        }
        let work = runtime
            .store()
            .basic_filter_operator_observation()
            .expect("final bounded work")
            .work;
        eprintln!(
            "phase161 operator polling: prefix={prefix} polls=64 metadata_reads={} metadata_bytes={} basic_json_bytes={} total_wall_ns={} maximum_capture_wall_ns={maximum_capture_ns} scans_delta=0 integrity_reads_delta=0 namespace_changes=0",
            work.metadata_reads,
            work.metadata_bytes,
            serde_json::to_vec(&expected).expect("JSON").len(),
            polling.elapsed().as_nanos()
        );
        // Assert
        assert_eq!(
            expected.operational_state,
            FieldAvailability::available(BasicIndexOperationalState::Current)
        );
        assert_eq!(
            expected.initial_sync_complete,
            FieldAvailability::available(true)
        );
        assert_eq!(
            expected.maybe_lag_blocks,
            FieldAvailability::available(Some(0))
        );
        assert_eq!(
            expected.maybe_safe_durable_height,
            FieldAvailability::available(Some(prefix as u32))
        );
        assert_eq!(
            expected.retained_availability,
            FieldAvailability::available(BasicRetainedAvailability::Observed)
        );
        assert_eq!(runtime.store().basic_operator_namespaces_for_test(), rows);
        assert_eq!(
            runtime.store().basic_filter_validation_scans_for_test(),
            scans
        );
        assert_eq!(
            runtime.store().basic_filter_point_reads_for_test(),
            point_reads
        );
        assert_eq!(
            runtime
                .network
                .maybe_basic_index_progress()
                .expect("unchanged owner"),
            owner
        );
        drop(runtime);
        fixture.history.cleanup();
    }
}

#[test]
fn phase161_operator_authority_actual_catchup_keeps_safe_fence_separate() {
    // Arrange
    let fixture = TurnHistory::new(16, 24, 1);
    let runtime = DurableSyncRuntime::open_configured(
        fixture.seed(16),
        sync_config(),
        BasicFilterStartupMode::Enabled,
    )
    .expect("startup turn");
    // Act
    let first = capture(&runtime);
    runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("second turn");
    runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("third turn");
    let complete = capture(&runtime);
    // Assert
    assert_eq!(
        first.operational_state,
        FieldAvailability::available(BasicIndexOperationalState::CatchingUp)
    );
    assert_eq!(
        first.initial_sync_complete,
        FieldAvailability::available(false)
    );
    assert_eq!(
        first.maybe_processed_height,
        FieldAvailability::available(Some(23))
    );
    assert_eq!(
        first.maybe_safe_durable_height,
        FieldAvailability::available(Some(15))
    );
    assert_eq!(
        first.maybe_accepted_height,
        FieldAvailability::available(Some(39))
    );
    assert_eq!(
        first.maybe_lag_blocks,
        FieldAvailability::available(Some(16))
    );
    assert_eq!(
        complete.initial_sync_complete,
        FieldAvailability::available(true)
    );
    assert_eq!(
        complete.maybe_safe_durable_height,
        FieldAvailability::available(Some(39))
    );
    drop(runtime);
    fixture.history.cleanup();
}

#[test]
fn phase161_operator_authority_actual_later_lag_processed_ahead_and_achieved_flush() {
    // Arrange
    let history = ValidatedHistory::new("operator-ahead", false);
    let runtime = history.live_runtime();
    runtime
        .network
        .connect_local_block(
            &history.blocks[2],
            ScriptVerifyFlags::P2SH,
            ConsensusParams {
                coinbase_maturity: 1,
                ..Default::default()
            },
        )
        .expect("actual accepted successor");
    // Act
    let lagged = capture(&runtime);
    runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("accepted facts turn");
    let ahead = capture(&runtime);
    runtime
        .network
        .flush_coins(FlushMode::Always, FlushPolicyTime::new(0), u64::MAX)
        .expect("genuine flush");
    runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("durable acknowledgement");
    let durable = capture(&runtime);
    // Assert
    assert_eq!(
        lagged.operational_state,
        FieldAvailability::available(BasicIndexOperationalState::Lagged)
    );
    assert_eq!(
        lagged.initial_sync_complete,
        FieldAvailability::available(true)
    );
    assert_eq!(
        lagged.maybe_lag_blocks,
        FieldAvailability::available(Some(1))
    );
    assert_eq!(
        ahead.maybe_processed_height,
        FieldAvailability::available(Some(2))
    );
    assert_eq!(
        ahead.maybe_safe_durable_height,
        FieldAvailability::available(Some(1))
    );
    assert_eq!(
        ahead.next_action,
        FieldAvailability::available(BasicIndexNextAction::ProcessedAheadOfDurable)
    );
    assert_eq!(
        durable.maybe_safe_durable_height,
        FieldAvailability::available(Some(2))
    );
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase161_operator_authority_actual_missing_body_and_undo_preserve_retained_evidence() {
    for undo in [false, true] {
        // Arrange
        let fixture = TurnHistory::new(16, 24, 1);
        let runtime = DurableSyncRuntime::open_configured(
            fixture.seed(16),
            sync_config(),
            BasicFilterStartupMode::Enabled,
        )
        .expect("startup");
        let hash = fixture.history.records[24].identity().block_hash();
        runtime
            .store()
            .delete_basic_input_for_test(hash, undo)
            .expect("remove required physical mate");
        if undo {
            runtime
                .network
                .forget_block_payload_and_undo_for_test(hash)
                .expect("forget actual memory undo");
        }
        // Act
        assert!(runtime.network.drive_basic_filter_index_turn().is_err());
        let rows = runtime.store().basic_operator_namespaces_for_test();
        let status = capture(&runtime);
        // Assert
        assert_eq!(
            status.maybe_failure,
            FieldAvailability::available(Some(if undo {
                BasicIndexFailureCategory::RequiredUndo
            } else {
                BasicIndexFailureCategory::RequiredBody
            }))
        );
        assert_eq!(
            status.operational_state,
            FieldAvailability::available(BasicIndexOperationalState::Paused)
        );
        assert_eq!(
            status.retained_availability,
            FieldAvailability::available(BasicRetainedAvailability::Observed)
        );
        assert_eq!(
            status.maybe_safe_durable_height,
            FieldAvailability::available(Some(15))
        );
        assert_eq!(capture(&runtime), status);
        assert_eq!(runtime.store().basic_operator_namespaces_for_test(), rows);
        verify_caller_capture(
            if undo {
                "required-undo"
            } else {
                "required-body"
            },
            &status,
        );
        drop(runtime);
        fixture.history.cleanup();
    }
}

#[test]
fn phase161_operator_authority_actual_persistence_poison_keeps_owner_failure_and_configuration() {
    for fault in [
        FilterPublicationFault::BeforeRecords,
        FilterPublicationFault::AfterCommit,
    ] {
        // Arrange
        let fixture = TurnHistory::new(16, 24, 1);
        let runtime = DurableSyncRuntime::open_configured(
            fixture.seed(16),
            sync_config(),
            BasicFilterStartupMode::Enabled,
        )
        .expect("startup");
        runtime.store().set_basic_filter_fault(fault);
        assert!(runtime.network.drive_basic_filter_index_turn().is_err());
        let rows = runtime.store().basic_operator_namespaces_for_test();
        // Act
        let snapshot = runtime
            .network
            .operator_snapshot()
            .expect("other snapshot groups survive");
        let status = snapshot.basic_index();
        // Assert
        assert_eq!(
            status.configured_indexing,
            FieldAvailability::available(true)
        );
        assert_eq!(
            status.maybe_failure,
            FieldAvailability::available(Some(BasicIndexFailureCategory::Backend))
        );
        assert!(matches!(
            status.retention,
            FieldAvailability::Unavailable { .. }
        ));
        assert!(matches!(
            status.maybe_safe_durable_height,
            FieldAvailability::Unavailable { .. }
        ));
        assert!(runtime.store().basic_filter_operator_observation().is_err());
        assert!(
            runtime.network.drive_basic_filter_index_turn().is_err(),
            "poll did not clear poison"
        );
        assert_eq!(runtime.store().basic_operator_namespaces_for_test(), rows);
        verify_caller_capture(
            match fault {
                FilterPublicationFault::BeforeRecords => "persistence-before-records",
                FilterPublicationFault::AfterCommit => "persistence-after-commit",
                _ => unreachable!("enumerated publication faults"),
            },
            status,
        );
        drop(runtime);
        fixture.history.cleanup();
    }
}

#[test]
fn phase161_operator_authority_actual_failed_disable_never_reuses_old_configuration() {
    for fault in [
        FilterPublicationFault::BeforeCommit,
        FilterPublicationFault::AfterDisable,
        FilterPublicationFault::AfterRelease,
    ] {
        // Arrange
        let history = ValidatedHistory::new("operator-disable-uncertain", false);
        let runtime = opened(&history);
        runtime.store().set_basic_filter_fault(fault);
        // Act
        assert!(runtime.network.disable_basic_filter_index().is_err());
        let snapshot = runtime.network.operator_snapshot().expect("snapshot");
        // Assert
        assert!(matches!(
            snapshot.basic_index().configured_indexing,
            FieldAvailability::Unavailable { .. }
        ));
        assert_ne!(
            snapshot.basic_index().operational_state,
            FieldAvailability::available(BasicIndexOperationalState::Current)
        );
        drop(runtime);
        history.cleanup();
    }
}

#[test]
fn phase161_operator_authority_actual_successful_transitions_record_configuration_and_protection() {
    // Arrange
    let history = ValidatedHistory::new("operator-transition", false);
    let runtime = opened(&history);
    // Act
    runtime
        .network
        .disable_basic_filter_index()
        .expect("actual disable");
    let disabled = capture(&runtime);
    runtime
        .network
        .enable_basic_filter_index()
        .expect("actual enable");
    let enabled = capture(&runtime);
    // Assert
    assert_eq!(
        disabled.configured_indexing,
        FieldAvailability::available(false)
    );
    assert_eq!(
        disabled.operational_state,
        FieldAvailability::available(BasicIndexOperationalState::Disabled)
    );
    assert_eq!(
        disabled.protection,
        FieldAvailability::available(BasicIndexProtection::None)
    );
    assert_eq!(
        disabled.retained_availability,
        FieldAvailability::available(BasicRetainedAvailability::Observed)
    );
    assert_eq!(
        enabled.configured_indexing,
        FieldAvailability::available(true)
    );
    assert_ne!(
        enabled.operational_state,
        FieldAvailability::available(BasicIndexOperationalState::Current)
    );
    assert!(matches!(
        enabled.protection,
        FieldAvailability::Available(BasicIndexProtection::InclusiveRange(_))
    ));
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase161_operator_authority_actual_equal_height_reorg_reports_verified_common_prefix() {
    // Arrange
    let fixture =
        super::reorg::fixtures::ForkFixture::compact_with_early_branch(16, 8, 24, 15, 8, 0x76);
    let before = capture(&fixture.runtime);
    let branch = fixture.fork(15, 8, 0x76);
    // Act
    fixture.apply(&branch);
    let replaced = capture(&fixture.runtime);
    fixture.flush();
    fixture.finish();
    let recovered = capture(&fixture.runtime);
    // Assert
    assert_eq!(before.maybe_accepted_height, replaced.maybe_accepted_height);
    assert_eq!(
        replaced.initial_sync_complete,
        FieldAvailability::available(true)
    );
    assert_eq!(
        replaced.operational_state,
        FieldAvailability::available(BasicIndexOperationalState::Lagged)
    );
    assert_eq!(
        replaced.maybe_lag_blocks,
        FieldAvailability::available(Some(8))
    );
    assert_eq!(
        replaced.maybe_processed_height,
        FieldAvailability::available(Some(15))
    );
    assert_eq!(
        replaced.maybe_safe_durable_height,
        FieldAvailability::available(Some(15))
    );
    assert_eq!(
        replaced.retained_availability,
        FieldAvailability::available(BasicRetainedAvailability::Observed)
    );
    assert_eq!(
        recovered.operational_state,
        FieldAvailability::available(BasicIndexOperationalState::Current)
    );
    let FieldAvailability::Available(measurement) = recovered.retention else {
        panic!("earned retention")
    };
    assert_eq!(
        measurement.retained_record_count, 32,
        "retained displaced branch remains counted"
    );
    fixture.cleanup();
}

#[test]
fn phase161_operator_authority_metadata_bounds_and_raw_integrity_revocation_are_readonly() {
    // Arrange
    let history = ValidatedHistory::new("operator-read-bounds", false);
    let runtime = opened(&history);
    let store = runtime.store();
    let baseline = store.basic_filter_operator_observation().expect("capture");
    let exact = baseline.work;
    // Act / Assert
    assert!(
        store
            .basic_filter_operator_observation_with_limits(
                exact.metadata_reads,
                exact.metadata_bytes
            )
            .expect("exact")
            .authority
            .is_ok()
    );
    assert!(
        store
            .basic_filter_operator_observation_with_limits(
                exact.metadata_reads - 1,
                exact.metadata_bytes
            )
            .expect("read bound")
            .authority
            .is_err()
    );
    assert!(
        store
            .basic_filter_operator_observation_with_limits(
                exact.metadata_reads,
                exact.metadata_bytes - 1
            )
            .expect("byte bound")
            .authority
            .is_err()
    );
    store
        .write_raw_for_test(
            StorageNamespace::BlockIndex,
            codec::STATE_KEY,
            vec![0; 4097],
        )
        .expect("oversized private raw value");
    let rows = store.basic_operator_namespaces_for_test();
    let oversized = store
        .basic_filter_operator_observation()
        .expect("bounded capture");
    assert!(oversized.authority.is_err());
    assert_eq!(oversized.work.metadata_reads, 1);
    assert_eq!(
        oversized.work.metadata_bytes, 0,
        "oversized borrowed value rejected before decode/copy"
    );
    assert!(
        oversized.retention.is_err(),
        "raw write revokes prior retained evidence"
    );
    let status = capture(&runtime);
    assert!(matches!(
        (status.operational_state, status.maybe_failure),
        (
            FieldAvailability::Unavailable { .. },
            FieldAvailability::Available(None)
        )
    ));
    assert!(matches!(
        status.retention,
        FieldAvailability::Unavailable { .. }
    ));
    assert_eq!(store.basic_operator_namespaces_for_test(), rows);
    drop(runtime);
    history.cleanup();
}
