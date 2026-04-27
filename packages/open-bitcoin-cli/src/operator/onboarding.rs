// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Operator onboarding contract surface.

use std::path::PathBuf;

use super::NetworkSelection;

/// Request shape for interactive and non-interactive onboarding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnboardingRequest {
    /// Prompt-driven onboarding with injectable answers for tests and shells.
    Interactive {
        /// Answers collected by the prompt adapter.
        answers: OnboardingPromptAnswers,
    },
    /// Flag-driven onboarding that must not prompt.
    NonInteractive {
        /// Explicit answers supplied by CLI flags or automation.
        answers: OnboardingPromptAnswers,
        /// Whether an approved write may replace an existing managed file.
        force_overwrite: bool,
    },
}

/// Answers collected from prompt or non-interactive flag input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnboardingPromptAnswers {
    /// Requested network.
    pub maybe_network: Option<NetworkSelection>,
    /// Requested Open Bitcoin datadir.
    pub maybe_data_dir: Option<PathBuf>,
    /// Requested Open Bitcoin JSONC config path.
    pub maybe_config_path: Option<PathBuf>,
    /// Whether read-only Core/Knots detection should run.
    pub detect_existing_installations: bool,
    /// Whether metrics should be enabled in the proposed config.
    pub metrics_enabled: bool,
    /// Whether structured logs should be enabled in the proposed config.
    pub logs_enabled: bool,
    /// Whether the user or automation approved the proposed config write.
    pub approve_write: bool,
}

/// Planned onboarding result before any shell writes are attempted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnboardingPlan {
    /// Original onboarding request.
    pub request: OnboardingRequest,
    /// Write policy derived from the request and current filesystem evidence.
    pub write_decision: OnboardingWriteDecision,
    /// User-facing messages the shell may render.
    pub messages: Vec<OnboardingMessage>,
}

/// Explicit onboarding write decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnboardingWriteDecision {
    /// No write should be attempted.
    NoWrite {
        /// Human-readable reason.
        reason: String,
    },
    /// A config write is proposed but not approved.
    ProposedWrite {
        /// Proposed config write.
        write: ProposedConfigWrite,
    },
    /// A config write is explicitly approved.
    ApprovedWrite {
        /// Approved config write.
        write: ProposedConfigWrite,
    },
}

/// Proposed Open Bitcoin JSONC write.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProposedConfigWrite {
    /// Destination `open-bitcoin.jsonc` path.
    pub path: PathBuf,
    /// Full proposed JSONC content.
    pub contents: String,
    /// Whether the destination already exists.
    pub replaces_existing: bool,
}

/// Message emitted by the pure onboarding planner for shell rendering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnboardingMessage {
    /// Severity level.
    pub level: OnboardingMessageLevel,
    /// Message text.
    pub text: String,
}

/// User-facing onboarding message level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OnboardingMessageLevel {
    /// Informational message.
    Info,
    /// Warning message requiring operator attention.
    Warning,
    /// Error message that should stop the flow.
    Error,
}

#[cfg(test)]
mod tests;
