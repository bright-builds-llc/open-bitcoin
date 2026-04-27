// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Operator onboarding planning and write shell.

use std::{
    collections::BTreeMap,
    fmt, fs,
    io::{self, Write},
    path::PathBuf,
};

use open_bitcoin_rpc::config::OpenBitcoinConfig;

use super::{
    NetworkSelection,
    config::OperatorConfigResolution,
    detect::{DetectedInstallation, DetectionSourcePathKind},
};

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

/// Existing local state considered by the onboarding planner.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OnboardingExistingState {
    /// Existing Open Bitcoin JSONC config, when parsed.
    pub maybe_config: Option<OpenBitcoinConfig>,
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
    /// Existing parsed config state, if any.
    pub existing: OnboardingExistingState,
    /// Proposed answers that would be written.
    pub proposed_answers: OnboardingPromptAnswers,
    /// Read-only detection evidence surfaced to the user.
    pub detected_installations: Vec<DetectedInstallation>,
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

/// Prompt adapter used by interactive onboarding.
pub trait OnboardingPrompter {
    fn prompt(
        &mut self,
        question: &str,
        maybe_default: Option<&str>,
    ) -> Result<String, OnboardingError>;
}

/// Onboarding failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnboardingError {
    message: String,
}

impl OnboardingError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for OnboardingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for OnboardingError {}

/// Stdout/stdin prompt adapter for the actual operator binary.
#[derive(Debug, Default)]
pub struct StdIoOnboardingPrompter;

impl OnboardingPrompter for StdIoOnboardingPrompter {
    fn prompt(
        &mut self,
        question: &str,
        maybe_default: Option<&str>,
    ) -> Result<String, OnboardingError> {
        match maybe_default {
            Some(default) => print!("{question} [{default}]: "),
            None => print!("{question}: "),
        }
        io::stdout()
            .flush()
            .map_err(|error| OnboardingError::new(format!("failed to write prompt: {error}")))?;
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .map_err(|error| OnboardingError::new(format!("failed to read prompt: {error}")))?;
        let trimmed = answer.trim();
        if trimmed.is_empty()
            && let Some(default) = maybe_default
        {
            return Ok(default.to_string());
        }
        Ok(trimmed.to_string())
    }
}

/// Ask the practical first-run questions through an injected prompt adapter.
pub fn prompt_onboarding_answers(
    prompter: &mut dyn OnboardingPrompter,
    defaults: &OnboardingPromptAnswers,
) -> Result<OnboardingPromptAnswers, OnboardingError> {
    let network_default = defaults.maybe_network.map(network_name);
    let network = prompter.prompt("Network", network_default)?;
    let data_dir_default = defaults
        .maybe_data_dir
        .as_ref()
        .map(|path| path.display().to_string());
    let data_dir = prompter.prompt("Datadir", data_dir_default.as_deref())?;
    let config_path_default = defaults
        .maybe_config_path
        .as_ref()
        .map(|path| path.display().to_string());
    let config_path = prompter.prompt("Open Bitcoin JSONC path", config_path_default.as_deref())?;
    let metrics_enabled = parse_yes_no(&prompter.prompt(
        "Enable metrics by default",
        Some(if defaults.metrics_enabled {
            "yes"
        } else {
            "no"
        }),
    )?)?;
    let logs_enabled = parse_yes_no(&prompter.prompt(
        "Enable structured logs by default",
        Some(if defaults.logs_enabled { "yes" } else { "no" }),
    )?)?;
    let detect_existing_installations = parse_yes_no(&prompter.prompt(
        "Detect existing Core/Knots installations",
        Some(if defaults.detect_existing_installations {
            "yes"
        } else {
            "no"
        }),
    )?)?;
    let approve_write = parse_yes_no(&prompter.prompt("Write open-bitcoin.jsonc", Some("no"))?)?;

    Ok(OnboardingPromptAnswers {
        maybe_network: Some(parse_network(&network)?),
        maybe_data_dir: Some(PathBuf::from(data_dir)),
        maybe_config_path: Some(PathBuf::from(config_path)),
        detect_existing_installations,
        metrics_enabled,
        logs_enabled,
        approve_write,
    })
}

/// Create an onboarding plan without mutating the filesystem.
pub fn plan_onboarding(
    config_resolution: &OperatorConfigResolution,
    existing: OnboardingExistingState,
    detected_installations: Vec<DetectedInstallation>,
    request: OnboardingRequest,
) -> Result<OnboardingPlan, OnboardingError> {
    let answers = answers_from_request(&request).clone();
    validate_required_answers(&answers)?;
    let config_path = answers
        .maybe_config_path
        .clone()
        .ok_or_else(|| OnboardingError::new("missing required onboarding value: config"))?;
    let force_overwrite = matches!(
        request,
        OnboardingRequest::NonInteractive {
            force_overwrite: true,
            ..
        }
    );
    let replaces_existing = config_path.exists() || existing.maybe_config.is_some();
    let non_interactive = matches!(request, OnboardingRequest::NonInteractive { .. });
    let proposed_config = proposed_open_bitcoin_config(&answers, non_interactive)?;
    let contents = serde_json::to_string_pretty(&proposed_config)
        .map_err(|error| OnboardingError::new(format!("failed to serialize JSONC: {error}")))?;
    let write = ProposedConfigWrite {
        path: config_path.clone(),
        contents,
        replaces_existing,
    };

    let mut messages = vec![OnboardingMessage {
        level: OnboardingMessageLevel::Info,
        text: format!("Open Bitcoin config path: {}", config_path.display()),
    }];
    if let Some(data_dir) = config_resolution.maybe_data_dir.as_ref() {
        messages.push(OnboardingMessage {
            level: OnboardingMessageLevel::Info,
            text: format!("Open Bitcoin datadir: {}", data_dir.display()),
        });
    }
    messages.extend(detection_messages(&detected_installations));

    let write_decision = if replaces_existing && !force_overwrite {
        messages.push(OnboardingMessage {
            level: OnboardingMessageLevel::Warning,
            text: "Existing open-bitcoin.jsonc left unchanged; rerun with --force-overwrite to replace it."
                .to_string(),
        });
        OnboardingWriteDecision::NoWrite {
            reason: "existing open-bitcoin.jsonc left unchanged".to_string(),
        }
    } else if answers.approve_write {
        OnboardingWriteDecision::ApprovedWrite { write }
    } else {
        OnboardingWriteDecision::ProposedWrite { write }
    };

    Ok(OnboardingPlan {
        request,
        existing,
        proposed_answers: answers,
        detected_installations,
        write_decision,
        messages,
    })
}

/// Apply an approved onboarding write to the selected Open Bitcoin JSONC path.
pub fn apply_onboarding_plan(plan: &OnboardingPlan) -> Result<Option<PathBuf>, OnboardingError> {
    let OnboardingWriteDecision::ApprovedWrite { write } = &plan.write_decision else {
        return Ok(None);
    };
    if let Some(parent) = write.path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            OnboardingError::new(format!("failed to create config dir: {error}"))
        })?;
    }
    fs::write(&write.path, &write.contents).map_err(|error| {
        OnboardingError::new(format!("failed to write open-bitcoin.jsonc: {error}"))
    })?;
    Ok(Some(write.path.clone()))
}

pub fn render_onboarding_plan(plan: &OnboardingPlan) -> String {
    let mut lines = plan
        .messages
        .iter()
        .map(|message| message.text.clone())
        .collect::<Vec<_>>();
    match &plan.write_decision {
        OnboardingWriteDecision::NoWrite { reason } => lines.push(format!("No write: {reason}")),
        OnboardingWriteDecision::ProposedWrite { write } => {
            lines.push(format!("Proposed write: {}", write.path.display()))
        }
        OnboardingWriteDecision::ApprovedWrite { write } => lines.push(format!(
            "Wrote open-bitcoin.jsonc: {}",
            write.path.display()
        )),
    }
    lines.join("\n")
}

pub fn read_existing_open_bitcoin_config(
    maybe_config_path: Option<&PathBuf>,
) -> Result<OnboardingExistingState, OnboardingError> {
    let Some(path) = maybe_config_path else {
        return Ok(OnboardingExistingState::default());
    };
    if !path.exists() {
        return Ok(OnboardingExistingState::default());
    }
    let text = fs::read_to_string(path).map_err(|error| {
        OnboardingError::new(format!("failed to read existing config: {error}"))
    })?;
    let config = open_bitcoin_rpc::config::parse_open_bitcoin_jsonc_config(&text)
        .map_err(|error| OnboardingError::new(error.to_string()))?;
    Ok(OnboardingExistingState {
        maybe_config: Some(config),
    })
}

fn answers_from_request(request: &OnboardingRequest) -> &OnboardingPromptAnswers {
    match request {
        OnboardingRequest::Interactive { answers }
        | OnboardingRequest::NonInteractive { answers, .. } => answers,
    }
}

fn validate_required_answers(answers: &OnboardingPromptAnswers) -> Result<(), OnboardingError> {
    if answers.maybe_network.is_none() {
        return Err(OnboardingError::new(
            "missing required onboarding value: network",
        ));
    }
    if answers.maybe_data_dir.is_none() {
        return Err(OnboardingError::new(
            "missing required onboarding value: datadir",
        ));
    }
    if answers.maybe_config_path.is_none() {
        return Err(OnboardingError::new(
            "missing required onboarding value: config",
        ));
    }
    Ok(())
}

fn proposed_open_bitcoin_config(
    answers: &OnboardingPromptAnswers,
    non_interactive: bool,
) -> Result<OpenBitcoinConfig, OnboardingError> {
    let network = answers
        .maybe_network
        .map(network_name)
        .ok_or_else(|| OnboardingError::new("missing required onboarding value: network"))?;
    let data_dir = answers
        .maybe_data_dir
        .as_ref()
        .ok_or_else(|| OnboardingError::new("missing required onboarding value: datadir"))?;
    let config_path = answers
        .maybe_config_path
        .as_ref()
        .ok_or_else(|| OnboardingError::new("missing required onboarding value: config"))?;
    let mut wizard_answers = BTreeMap::new();
    wizard_answers.insert("network".to_string(), network.to_string());
    wizard_answers.insert("datadir".to_string(), data_dir.display().to_string());
    wizard_answers.insert("config".to_string(), config_path.display().to_string());

    let mut config = OpenBitcoinConfig::default();
    config.onboarding.wizard_answers = wizard_answers;
    config.onboarding.completed_steps = vec![
        "network".to_string(),
        "datadir".to_string(),
        "config".to_string(),
    ];
    config.onboarding.non_interactive = non_interactive;
    config.metrics.enabled = answers.metrics_enabled;
    config.logs.enabled = answers.logs_enabled;
    config.migration.detect_existing_installations = answers.detect_existing_installations;
    Ok(config)
}

fn detection_messages(detected_installations: &[DetectedInstallation]) -> Vec<OnboardingMessage> {
    detected_installations
        .iter()
        .map(|installation| {
            let paths = installation
                .source_paths
                .iter()
                .filter(|source_path| source_path.present)
                .map(|source_path| {
                    format!(
                        "{}:{}",
                        source_path_kind(source_path.kind),
                        source_path.path.display()
                    )
                })
                .collect::<Vec<_>>()
                .join(", ");
            OnboardingMessage {
                level: OnboardingMessageLevel::Info,
                text: format!(
                    "Detected existing Bitcoin candidate with uncertain identity: {paths}"
                ),
            }
        })
        .collect()
}

fn source_path_kind(kind: DetectionSourcePathKind) -> &'static str {
    match kind {
        DetectionSourcePathKind::DataDir => "datadir",
        DetectionSourcePathKind::ConfigFile => "config",
        DetectionSourcePathKind::CookieFile => "cookie",
        DetectionSourcePathKind::ServiceDefinition => "service",
        DetectionSourcePathKind::WalletDirectory => "wallet_dir",
        DetectionSourcePathKind::WalletFile => "wallet_file",
    }
}

fn parse_yes_no(value: &str) -> Result<bool, OnboardingError> {
    match value.trim().to_ascii_lowercase().as_str() {
        "y" | "yes" | "true" | "1" => Ok(true),
        "n" | "no" | "false" | "0" => Ok(false),
        _ => Err(OnboardingError::new(format!(
            "invalid yes/no answer: {value}"
        ))),
    }
}

fn parse_network(value: &str) -> Result<NetworkSelection, OnboardingError> {
    match value {
        "main" | "mainnet" => Ok(NetworkSelection::Mainnet),
        "test" | "testnet" | "testnet3" => Ok(NetworkSelection::Testnet),
        "signet" => Ok(NetworkSelection::Signet),
        "regtest" => Ok(NetworkSelection::Regtest),
        _ => Err(OnboardingError::new(format!("invalid network: {value}"))),
    }
}

fn network_name(network: NetworkSelection) -> &'static str {
    match network {
        NetworkSelection::Mainnet => "mainnet",
        NetworkSelection::Testnet => "testnet",
        NetworkSelection::Signet => "signet",
        NetworkSelection::Regtest => "regtest",
    }
}

#[cfg(test)]
mod tests;
