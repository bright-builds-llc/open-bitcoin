// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;

#[test]
fn phase161_capture_read_failures_preserve_genuine_sticky_projection() {
    for backend in [false, true] {
        // Arrange: a genuinely accepted retained row, then a source-local discovered fault.
        let (history, runtime) = genuine();
        runtime
            .network
            .drive_basic_filter_index_turn()
            .expect("actual publication");
        let before = read_fault_status(&runtime);
        assert_eq!(
            before.operational_state,
            crate::status::FieldAvailability::available(
                crate::status::BasicIndexOperationalState::Current
            )
        );
        let hash = history.records[2].identity().block_hash();
        if backend {
            runtime
                .store()
                .set_basic_filter_fault(FilterPublicationFault::BeforeQueryRead);
        } else {
            runtime
                .store()
                .inject_query_record_fault_for_test(hash, QueryRecordFault::CorruptTarget)
                .expect("actual target tamper");
        }
        // Act
        assert!(matches!(
            runtime.network.basic_filter_query(hash),
            Err(BasicFilterQueryError::Storage(_))
        ));
        // Assert: source-specific checks precede any optional fixture generation.
        assert_sticky_read_fault(&runtime, &before, backend);
        super::super::super::operator_status::verify_caller_capture(
            if backend {
                "read-backend"
            } else {
                "read-integrity"
            },
            &read_fault_status(&runtime),
        );
        drop(runtime);
        history.cleanup();
    }
}
