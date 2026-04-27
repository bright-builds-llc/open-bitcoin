// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Operator runtime contract surface.

use std::fmt;

/// Typed process-style outcome for an operator command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorCommandOutcome {
    /// Standard output.
    pub stdout: OperatorStdout,
    /// Standard error.
    pub stderr: OperatorStderr,
    /// Exit status.
    pub exit_code: OperatorExitCode,
}

/// Standard output stream text.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OperatorStdout {
    /// Captured stdout text.
    pub text: String,
}

/// Standard error stream text.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OperatorStderr {
    /// Captured stderr text.
    pub text: String,
}

/// Typed exit code for operator command outcomes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperatorExitCode {
    /// Successful command.
    Success,
    /// Failed command with a process-compatible code.
    Failure(u8),
}

impl OperatorExitCode {
    /// Numeric process exit code.
    pub const fn code(self) -> u8 {
        match self {
            Self::Success => 0,
            Self::Failure(code) => code,
        }
    }
}

/// Runtime error contract for operator command execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatorRuntimeError {
    /// Request was invalid before command execution.
    InvalidRequest {
        /// Human-readable error message.
        message: String,
    },
    /// Command is parsed but intentionally not executable in the current phase.
    UnsupportedCommand {
        /// Stable command name.
        command: &'static str,
    },
    /// Command failed and produced a typed outcome.
    CommandFailed {
        /// Failed command outcome.
        outcome: OperatorCommandOutcome,
    },
}

impl fmt::Display for OperatorRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest { message } => f.write_str(message),
            Self::UnsupportedCommand { command } => {
                write!(f, "operator command is not supported yet: {command}")
            }
            Self::CommandFailed { outcome } => {
                write!(
                    f,
                    "operator command failed with exit {}",
                    outcome.exit_code.code()
                )
            }
        }
    }
}

impl std::error::Error for OperatorRuntimeError {}
