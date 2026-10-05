// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;

fn work(value: u64) -> TurnWork {
    TurnWork {
        blocks: 1,
        body_bytes: value,
        undo_bytes: value,
        cloned_bytes: value,
        script_items: value,
        script_bytes: value,
        encoded_bytes: value,
        record_operations: value,
        checkpoint_operations: value,
        projection_operations: value,
    }
}

fn budget() -> BasicIndexTurnBudget {
    BasicIndexTurnBudget::new(
        TurnWork {
            blocks: 4,
            ..work(10)
        },
        work(100),
    )
    .expect("bounded budget")
}

#[test]
fn phase157_budget_exact_normal_limit_is_admitted() {
    // Arrange
    let budget = budget();
    // Act
    let result = budget.admit(TurnWork::default(), work(10));
    // Assert
    assert_eq!(result, Ok(TurnAdmission::Normal(work(10))));
}

#[test]
fn phase157_budget_oversized_singleton_is_explicit_and_yields_afterward() {
    // Arrange
    let budget = budget();
    // Act
    let result = budget.admit(TurnWork::default(), work(11));
    // Assert
    assert_eq!(result, Ok(TurnAdmission::OversizedSingleton(work(11))));
    assert_eq!(budget.admit(work(11), work(1)), Ok(TurnAdmission::Yield));
}

#[test]
fn phase157_budget_absolute_overrun_is_a_failure_not_no_progress() {
    // Arrange
    let budget = budget();
    // Act
    let result = budget.admit(TurnWork::default(), work(101));
    // Assert
    assert_eq!(result, Err(TurnBudgetError::AbsoluteLimit));
}

fn set_counter(value: &mut TurnWork, index: usize, count: u64) {
    let counters: [fn(&mut TurnWork, u64); 10] = [
        |v, n| v.blocks = n,
        |v, n| v.body_bytes = n,
        |v, n| v.undo_bytes = n,
        |v, n| v.cloned_bytes = n,
        |v, n| v.script_items = n,
        |v, n| v.script_bytes = n,
        |v, n| v.encoded_bytes = n,
        |v, n| v.record_operations = n,
        |v, n| v.checkpoint_operations = n,
        |v, n| v.projection_operations = n,
    ];
    counters[index](value, count);
}

#[test]
fn phase157_budget_every_normal_counter_has_exact_and_one_over_boundary() {
    // Arrange
    let budget = budget();
    // Act / Assert
    for index in 0..10 {
        let mut used = TurnWork::default();
        let mut candidate = work(1);
        if index == 0 {
            used.blocks = 3;
        } else {
            set_counter(&mut candidate, index, 10);
        }
        let exact = used.checked_add(candidate).expect("small exact work");
        assert_eq!(
            budget.admit(used, candidate),
            Ok(TurnAdmission::Normal(exact))
        );
        if index == 0 {
            used.blocks = 4;
            assert_eq!(budget.admit(used, candidate), Ok(TurnAdmission::Yield));
        } else {
            set_counter(&mut candidate, index, 11);
            assert_eq!(
                budget.admit(used, candidate),
                Ok(TurnAdmission::OversizedSingleton(candidate))
            );
        }
    }
}

#[test]
fn phase157_budget_every_absolute_counter_has_exact_and_one_over_boundary() {
    // Arrange
    let budget = budget();
    // Act / Assert
    for index in 0..10 {
        let mut candidate = work(1);
        set_counter(&mut candidate, index, if index == 0 { 1 } else { 100 });
        let expected = if index == 0 {
            TurnAdmission::Normal(candidate)
        } else {
            TurnAdmission::OversizedSingleton(candidate)
        };
        assert_eq!(budget.admit(TurnWork::default(), candidate), Ok(expected));
        set_counter(&mut candidate, index, if index == 0 { 2 } else { 101 });
        let error = if index == 0 {
            TurnBudgetError::InvalidWork
        } else {
            TurnBudgetError::AbsoluteLimit
        };
        assert_eq!(budget.admit(TurnWork::default(), candidate), Err(error));
    }
}

#[test]
fn phase157_budget_every_counter_refuses_arithmetic_overflow() {
    // Arrange
    let increment = work(1);
    // Act / Assert
    for index in 0..10 {
        let mut used = TurnWork::default();
        set_counter(&mut used, index, u64::MAX);
        assert_eq!(used.checked_add(increment), Err(TurnBudgetError::Overflow));
    }
}

#[test]
fn phase157_budget_admission_reports_overflow_without_mutating_used() {
    // Arrange
    let used = TurnWork {
        script_items: u64::MAX,
        ..TurnWork::default()
    };
    // Act
    let result = budget().admit(used, work(1));
    // Assert
    assert_eq!(result, Err(TurnBudgetError::Overflow));
    assert_eq!(used.script_items, u64::MAX);
}

#[test]
fn phase157_budget_aggregate_saturation_yields_at_previous_reservation() {
    // Arrange
    let used = work(6);
    // Act
    let result = budget().admit(used, work(5));
    // Assert
    assert_eq!(result, Ok(TurnAdmission::Yield));
    assert_eq!(used, work(6));
}

#[test]
fn phase157_budget_empty_candidate_is_invalid_work() {
    // Arrange
    let budget = budget();
    // Act
    let result = budget.admit(TurnWork::default(), TurnWork::default());
    // Assert
    assert_eq!(result, Err(TurnBudgetError::InvalidWork));
}

#[test]
fn phase157_budget_required_candidate_reservations_cannot_be_zero() {
    // Arrange
    let budget = budget();
    // Act / Assert
    for index in [0, 1, 6, 7, 8, 9] {
        let mut candidate = work(1);
        set_counter(&mut candidate, index, 0);
        assert_eq!(
            budget.admit(TurnWork::default(), candidate),
            Err(TurnBudgetError::InvalidWork)
        );
    }
}

#[test]
fn phase157_budget_genesis_and_empty_scripts_allow_zero_optional_work() {
    // Arrange
    let candidate = TurnWork {
        undo_bytes: 0,
        cloned_bytes: 0,
        script_items: 0,
        script_bytes: 0,
        ..work(1)
    };
    // Act
    let result = budget().admit(TurnWork::default(), candidate);
    // Assert
    assert_eq!(result, Ok(TurnAdmission::Normal(candidate)));
}

#[test]
fn phase157_budget_zero_limits_are_invalid_for_each_counter() {
    // Arrange
    let normal = TurnWork {
        blocks: 4,
        ..work(10)
    };
    let absolute = work(100);
    // Act / Assert
    for index in 0..10 {
        let mut invalid = normal;
        set_counter(&mut invalid, index, 0);
        assert_eq!(
            BasicIndexTurnBudget::new(invalid, absolute),
            Err(TurnBudgetError::InvalidBudget)
        );
        let mut invalid = absolute;
        set_counter(&mut invalid, index, 0);
        assert_eq!(
            BasicIndexTurnBudget::new(normal, invalid),
            Err(TurnBudgetError::InvalidBudget)
        );
    }
}

#[test]
fn phase157_budget_absolute_limits_are_independent_of_aggregate_limits() {
    // Arrange
    let normal = TurnWork {
        blocks: 4,
        ..work(100)
    };
    // Act
    let budget = BasicIndexTurnBudget::new(normal, work(10))
        .expect("aggregate can exceed one-block legal maximum");
    // Assert
    assert_eq!(budget.normal(), normal);
    assert_eq!(budget.absolute_singleton(), work(10));
    assert_eq!(
        budget.admit(work(10), work(10)),
        Ok(TurnAdmission::Normal(TurnWork {
            blocks: 2,
            ..work(20)
        }))
    );
    assert_eq!(
        budget.admit(TurnWork::default(), work(11)),
        Err(TurnBudgetError::AbsoluteLimit)
    );
}

#[test]
fn phase157_budget_codec_envelope_limits_are_enforced() {
    // Arrange
    let normal = TurnWork {
        blocks: BASIC_INDEX_MAX_CANDIDATES,
        encoded_bytes: BASIC_INDEX_MAX_ENCODED_BYTES,
        ..work(10)
    };
    let absolute = TurnWork {
        encoded_bytes: BASIC_INDEX_MAX_SINGLETON_ENCODED_BYTES,
        ..work(100)
    };
    // Act / Assert
    assert!(BasicIndexTurnBudget::new(normal, absolute).is_ok());
    assert_eq!(
        BasicIndexTurnBudget::new(
            TurnWork {
                blocks: 129,
                ..normal
            },
            absolute
        ),
        Err(TurnBudgetError::InvalidBudget)
    );
    assert_eq!(
        BasicIndexTurnBudget::new(
            normal,
            TurnWork {
                blocks: 2,
                ..absolute
            }
        ),
        Err(TurnBudgetError::InvalidBudget)
    );
    assert_eq!(
        BasicIndexTurnBudget::new(
            TurnWork {
                encoded_bytes: BASIC_INDEX_MAX_ENCODED_BYTES + 1,
                ..normal
            },
            absolute
        ),
        Err(TurnBudgetError::InvalidBudget)
    );
    assert_eq!(
        BasicIndexTurnBudget::new(
            normal,
            TurnWork {
                encoded_bytes: BASIC_INDEX_MAX_SINGLETON_ENCODED_BYTES + 1,
                ..absolute
            }
        ),
        Err(TurnBudgetError::InvalidBudget)
    );
}

#[test]
fn phase157_budget_singleton_includes_constant_validation_and_publication_reservations() {
    // Arrange
    let base = TurnWork {
        record_operations: 5,
        checkpoint_operations: 5,
        projection_operations: 5,
        ..TurnWork::default()
    };
    let candidate = TurnWork {
        record_operations: 96,
        ..work(11)
    };
    // Act
    let result = budget().admit(base, candidate);
    // Assert
    assert_eq!(result, Err(TurnBudgetError::AbsoluteLimit));
}

#[test]
fn phase157_budget_normal_turn_includes_zero_block_preparation_work() {
    // Arrange
    let base = TurnWork {
        checkpoint_operations: 5,
        ..TurnWork::default()
    };
    let candidate = work(1);
    // Act
    let result = budget().admit(base, candidate);
    // Assert
    assert_eq!(
        result,
        Ok(TurnAdmission::Normal(TurnWork {
            checkpoint_operations: 6,
            ..candidate
        }))
    );
}

#[test]
fn phase157_budget_invalid_prior_aggregate_is_refused() {
    // Arrange
    let used = TurnWork {
        blocks: 2,
        ..work(11)
    };
    // Act
    let result = budget().admit(used, work(1));
    // Assert
    assert_eq!(result, Err(TurnBudgetError::InvalidWork));
}

#[test]
fn phase157_budget_error_categories_have_bounded_diagnostics() {
    // Arrange
    let errors = [
        TurnBudgetError::InvalidBudget,
        TurnBudgetError::InvalidWork,
        TurnBudgetError::Overflow,
        TurnBudgetError::AbsoluteLimit,
    ];
    // Act / Assert
    for error in errors {
        assert!(
            error.to_string().starts_with("BASIC")
                || error.to_string().starts_with("invalid BASIC")
        );
    }
}
