// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{fmt, io, time::SystemTimeError};

#[derive(Debug)]
pub enum BenchError {
    InvalidRunMode {
        mode: &'static str,
        iterations: u64,
        min: u64,
        max: u64,
    },
    MissingFullIterations,
    InvalidArgument(String),
    CaseFailed {
        case: &'static str,
        reason: String,
    },
    Io(io::Error),
    Json(serde_json::Error),
    Time(SystemTimeError),
}

impl BenchError {
    pub fn case_failed(case: &'static str, reason: impl Into<String>) -> Self {
        Self::CaseFailed {
            case,
            reason: reason.into(),
        }
    }
}

impl fmt::Display for BenchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRunMode {
                mode,
                iterations,
                min,
                max,
            } => write!(
                f,
                "{mode} benchmark mode requires {min}..={max} iterations, got {iterations}"
            ),
            Self::MissingFullIterations => {
                f.write_str("full benchmark mode requires --iterations <N>")
            }
            Self::InvalidArgument(message) => f.write_str(message),
            Self::CaseFailed { case, reason } => {
                write!(f, "benchmark case `{case}` failed: {reason}")
            }
            Self::Io(error) => write!(f, "benchmark I/O failed: {error}"),
            Self::Json(error) => write!(f, "benchmark JSON failed: {error}"),
            Self::Time(error) => write!(f, "benchmark timestamp failed: {error}"),
        }
    }
}

impl std::error::Error for BenchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Json(error) => Some(error),
            Self::Time(error) => Some(error),
            Self::InvalidRunMode { .. }
            | Self::MissingFullIterations
            | Self::InvalidArgument(_)
            | Self::CaseFailed { .. } => None,
        }
    }
}

impl From<io::Error> for BenchError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for BenchError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<SystemTimeError> for BenchError {
    fn from(error: SystemTimeError) -> Self {
        Self::Time(error)
    }
}
