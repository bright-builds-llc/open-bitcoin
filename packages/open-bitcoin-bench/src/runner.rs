use std::{
    hint::black_box,
    time::{Duration, Instant},
};

use crate::error::BenchError;
use crate::{
    registry::{BenchCase, BenchGroup, KnotsMapping},
    report::{BenchCaseReport, BenchGroupReport, BenchReport, KnotsMappingReport, KnotsSource},
};

pub const SMOKE_MIN_ITERATIONS: u64 = 1;
pub const SMOKE_MAX_ITERATIONS: u64 = 10;
pub const FULL_MIN_ITERATIONS: u64 = 1;
pub const FULL_MAX_ITERATIONS: u64 = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunMode {
    Smoke,
    Full { iterations: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunConfig {
    pub mode: RunMode,
    pub iterations_per_case: u64,
}

impl RunConfig {
    pub fn smoke(iterations: u64) -> Result<Self, BenchError> {
        validate_iterations(
            "smoke",
            iterations,
            SMOKE_MIN_ITERATIONS,
            SMOKE_MAX_ITERATIONS,
        )?;
        Ok(Self {
            mode: RunMode::Smoke,
            iterations_per_case: iterations,
        })
    }

    pub fn full(iterations: u64) -> Result<Self, BenchError> {
        validate_iterations("full", iterations, FULL_MIN_ITERATIONS, FULL_MAX_ITERATIONS)?;
        Ok(Self {
            mode: RunMode::Full { iterations },
            iterations_per_case: iterations,
        })
    }

    pub fn mode_label(self) -> String {
        match self.mode {
            RunMode::Smoke => format!("smoke:{}", self.iterations_per_case),
            RunMode::Full { iterations } => format!("full:{iterations}"),
        }
    }
}

pub fn run_benchmarks(
    groups: &[BenchGroup],
    config: RunConfig,
    baseline: impl Into<String>,
    generated_at_unix_seconds: u64,
    optional_knots_source: Option<KnotsSource>,
) -> Result<BenchReport, BenchError> {
    let mut group_reports = Vec::with_capacity(groups.len());

    for group in groups {
        let mut case_reports = Vec::with_capacity(group.cases.len());
        for case in group.cases {
            let elapsed = run_case(case, config.iterations_per_case)?;
            case_reports.push(case_report(case, config.iterations_per_case, elapsed));
        }

        group_reports.push(BenchGroupReport {
            id: group.id.as_str().to_string(),
            description: group.description.to_string(),
            cases: case_reports,
        });
    }

    Ok(BenchReport {
        schema_version: 1,
        baseline: baseline.into(),
        mode: config.mode_label(),
        generated_at_unix_seconds,
        groups: group_reports,
        optional_knots_source,
    })
}

fn validate_iterations(
    mode: &'static str,
    iterations: u64,
    min: u64,
    max: u64,
) -> Result<(), BenchError> {
    if (min..=max).contains(&iterations) {
        return Ok(());
    }

    Err(BenchError::InvalidRunMode {
        mode,
        iterations,
        min,
        max,
    })
}

fn run_case(case: &BenchCase, iterations: u64) -> Result<Duration, BenchError> {
    let start = Instant::now();
    for _ in 0..iterations {
        black_box((case.run_once)()?);
    }
    Ok(start.elapsed())
}

fn case_report(case: &BenchCase, iterations: u64, elapsed: Duration) -> BenchCaseReport {
    BenchCaseReport {
        id: case.id.to_string(),
        group: case.group.as_str().to_string(),
        description: case.description.to_string(),
        iterations,
        elapsed_nanos: duration_nanos(elapsed),
        knots_mapping: mapping_report(case.knots_mapping),
    }
}

fn mapping_report(mapping: &KnotsMapping) -> KnotsMappingReport {
    KnotsMappingReport {
        benchmark_names: mapping
            .benchmark_names
            .iter()
            .map(|name| (*name).to_string())
            .collect(),
        source_files: mapping
            .source_files
            .iter()
            .map(|source_file| (*source_file).to_string())
            .collect(),
        notes: mapping.notes.to_string(),
    }
}

fn duration_nanos(duration: Duration) -> u64 {
    let nanos = duration.as_nanos();
    if nanos > u128::from(u64::MAX) {
        return u64::MAX;
    }
    nanos as u64
}

#[cfg(test)]
mod tests {
    use crate::registry::benchmark_groups;

    use super::{FULL_MAX_ITERATIONS, RunConfig, run_benchmarks};

    #[test]
    fn smoke_mode_rejects_unbounded_iteration_counts() {
        // Arrange / Act
        let zero = RunConfig::smoke(0);
        let above_cap = RunConfig::smoke(11);

        // Assert
        assert!(zero.is_err());
        assert!(above_cap.is_err());
    }

    #[test]
    fn full_mode_rejects_iterations_above_hard_cap() {
        // Arrange / Act
        let result = RunConfig::full(10_001);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn full_mode_accepts_iterations_at_hard_cap() {
        // Arrange / Act
        let result = RunConfig::full(FULL_MAX_ITERATIONS);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn smoke_run_executes_registered_cases_without_thresholds() {
        // Arrange
        let config = match RunConfig::smoke(1) {
            Ok(config) => config,
            Err(error) => panic!("smoke config should be valid: {error}"),
        };

        // Act
        let report = run_benchmarks(
            benchmark_groups(),
            config,
            "Bitcoin Knots 29.3.knots20260210",
            1,
            None,
        );

        // Assert
        let Ok(report) = report else {
            panic!("smoke benchmark run should succeed");
        };
        assert_eq!(report.groups.len(), 7);
        assert!(
            report
                .groups
                .iter()
                .all(|group| group.cases.iter().all(|case| case.iterations == 1))
        );
    }
}
