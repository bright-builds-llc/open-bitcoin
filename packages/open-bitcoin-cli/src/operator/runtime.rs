// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Operator runtime contract surface.

use std::{
    collections::BTreeMap,
    env, fmt, fs,
    path::{Path, PathBuf},
};

use open_bitcoin_rpc::RpcAuthConfig;
use serde_json::json;

use crate::{args::CliStartupArgs, startup::resolve_startup_config};

use super::{
    ConfigCommand, OnboardArgs, OperatorCli, OperatorCommand, OperatorOutputFormat,
    config::{
        OPEN_BITCOIN_CONFIG_ENV, OPEN_BITCOIN_DATADIR_ENV, OPEN_BITCOIN_NETWORK_ENV,
        OperatorConfigRequest, OperatorConfigResolution, OperatorConfigRoots,
        resolve_operator_config,
    },
    dashboard::{DashboardRuntimeContext, platform_dashboard_service_runtime, run_dashboard},
    detect::{DetectionRoots, detect_existing_installations},
    onboarding::{
        OnboardingError, OnboardingPromptAnswers, OnboardingRequest, StdIoOnboardingPrompter,
        apply_onboarding_plan, plan_onboarding, prompt_onboarding_answers,
        read_existing_open_bitcoin_config, render_onboarding_plan,
    },
    service::{execute_service_command, platform_service_manager},
    status::{
        HttpStatusRpcClient, StatusCollectorInput, StatusDetectionEvidence,
        StatusLiveRpcAdapterInput, StatusRenderMode, StatusRequest, StatusRpcAuthSource,
        StatusRpcClient, collect_status_snapshot, render_status,
    },
    wallet::execute_wallet_command,
};

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

impl OperatorCommandOutcome {
    pub fn new(
        stdout: impl Into<String>,
        stderr: impl Into<String>,
        exit_code: OperatorExitCode,
    ) -> Self {
        Self {
            stdout: OperatorStdout {
                text: stdout.into(),
            },
            stderr: OperatorStderr {
                text: stderr.into(),
            },
            exit_code,
        }
    }

    pub fn success(stdout: impl Into<String>) -> Self {
        Self::new(stdout, "", OperatorExitCode::Success)
    }

    pub fn failure(stderr: impl Into<String>) -> Self {
        Self::new("", stderr, OperatorExitCode::Failure(1))
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

/// Execute a parsed operator CLI using process environment and filesystem evidence.
pub fn execute_operator_cli(cli: OperatorCli) -> OperatorCommandOutcome {
    execute_operator_cli_with_default_data_dir(cli, default_operator_data_dir())
}

/// Execute a parsed operator CLI with an injected default datadir.
pub fn execute_operator_cli_with_default_data_dir(
    cli: OperatorCli,
    default_data_dir: PathBuf,
) -> OperatorCommandOutcome {
    match execute_operator_cli_inner(cli, default_data_dir) {
        Ok(outcome) => outcome,
        Err(error) => OperatorCommandOutcome::failure(error.to_string()),
    }
}

fn execute_operator_cli_inner(
    cli: OperatorCli,
    default_data_dir: PathBuf,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let environment = operator_environment();
    let roots = OperatorConfigRoots::new(
        cli.maybe_data_dir
            .clone()
            .unwrap_or_else(|| default_data_dir.clone()),
    );
    let config_resolution =
        resolve_operator_config(&OperatorConfigRequest::from(&cli), &environment, &roots).map_err(
            |error| OperatorRuntimeError::InvalidRequest {
                message: error.to_string(),
            },
        )?;
    let detections = detect_existing_installations(&detection_roots(&config_resolution));

    match &cli.command {
        OperatorCommand::Status(_) => execute_status(&cli, config_resolution, detections),
        OperatorCommand::Config(config) => match config.command {
            ConfigCommand::Paths => Ok(OperatorCommandOutcome::success(render_config_paths(
                &config_resolution,
                cli.format,
            )?)),
        },
        OperatorCommand::Onboard(args) => {
            execute_onboarding(args, &cli, config_resolution, detections)
        }
        OperatorCommand::Service(service) => {
            let manager = platform_service_manager(operator_home_dir());
            let binary_path =
                std::env::current_exe().unwrap_or_else(|_| PathBuf::from("open-bitcoin"));
            let data_dir = config_resolution
                .maybe_data_dir
                .clone()
                .unwrap_or_else(|| default_data_dir.clone());
            Ok(execute_service_command(
                service,
                binary_path,
                data_dir,
                config_resolution.maybe_config_path.clone(),
                config_resolution.maybe_log_dir.clone(),
                manager.as_ref(),
            ))
        }
        OperatorCommand::Dashboard(args) => {
            let binary_path =
                std::env::current_exe().unwrap_or_else(|_| PathBuf::from("open-bitcoin"));
            let data_dir = config_resolution
                .maybe_data_dir
                .clone()
                .unwrap_or_else(|| default_data_dir.clone());
            let service = platform_dashboard_service_runtime(
                binary_path,
                data_dir,
                config_resolution.maybe_config_path.clone(),
                config_resolution.maybe_log_dir.clone(),
                operator_home_dir(),
            );
            let status = status_runtime_parts(&cli, config_resolution, detections);
            Ok(run_dashboard(
                args,
                DashboardRuntimeContext {
                    render_mode: status_render_mode(cli.format),
                    status_input: status.input,
                    maybe_rpc_client: status.maybe_rpc_client,
                    service,
                },
            ))
        }
        OperatorCommand::Wallet(args) => execute_wallet_command(
            args,
            &cli,
            &config_resolution,
            &detections,
            &default_data_dir,
        ),
    }
}

struct StatusRuntimeParts {
    input: StatusCollectorInput,
    maybe_rpc_client: Option<Box<dyn StatusRpcClient>>,
}

fn execute_status(
    cli: &OperatorCli,
    config_resolution: OperatorConfigResolution,
    detections: Vec<super::detect::DetectedInstallation>,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let status = status_runtime_parts(cli, config_resolution, detections);
    let snapshot = collect_status_snapshot(&status.input, status.maybe_rpc_client.as_deref());
    let rendered = render_status(&snapshot, status_render_mode(cli.format)).map_err(|error| {
        OperatorRuntimeError::InvalidRequest {
            message: error.to_string(),
        }
    })?;
    Ok(OperatorCommandOutcome::success(format!("{rendered}\n")))
}

fn status_runtime_parts(
    cli: &OperatorCli,
    config_resolution: OperatorConfigResolution,
    detections: Vec<super::detect::DetectedInstallation>,
) -> StatusRuntimeParts {
    let maybe_startup = startup_config_for_status(&config_resolution);
    let maybe_rpc_client = maybe_startup
        .as_ref()
        .and_then(|startup| HttpStatusRpcClient::from_rpc_config(&startup.rpc).ok())
        .map(|client| Box::new(client) as Box<dyn StatusRpcClient>);
    let maybe_live_rpc = maybe_startup
        .as_ref()
        .map(|startup| StatusLiveRpcAdapterInput {
            endpoint: format!("{}:{}", startup.rpc.host, startup.rpc.port),
            auth_source: auth_source(&startup.rpc.auth),
            timeout: std::time::Duration::from_secs(2),
        });
    let maybe_service_manager: Option<Box<dyn super::service::ServiceManager>> =
        Some(super::service::platform_service_manager(operator_home_dir()));

    StatusRuntimeParts {
        input: StatusCollectorInput {
            request: StatusRequest {
                render_mode: status_render_mode(cli.format),
                maybe_config_path: config_resolution.maybe_config_path.clone(),
                maybe_data_dir: config_resolution.maybe_data_dir.clone(),
                maybe_network: config_resolution.maybe_network,
                include_live_rpc: maybe_rpc_client.is_some(),
                no_color: cli.no_color,
            },
            config_resolution,
            detection_evidence: StatusDetectionEvidence {
                detected_installations: detections,
            },
            maybe_live_rpc,
            maybe_service_manager,
        },
        maybe_rpc_client,
    }
}

fn execute_onboarding(
    args: &OnboardArgs,
    cli: &OperatorCli,
    config_resolution: OperatorConfigResolution,
    detections: Vec<super::detect::DetectedInstallation>,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let defaults = OnboardingPromptAnswers {
        maybe_network: cli.maybe_network,
        maybe_data_dir: cli.maybe_data_dir.clone(),
        maybe_config_path: cli.maybe_config_path.clone(),
        detect_existing_installations: args.detect_existing,
        metrics_enabled: !args.disable_metrics,
        logs_enabled: !args.disable_logs,
        approve_write: args.approve_write,
    };
    let answers = if args.non_interactive {
        defaults
    } else {
        let mut prompter = StdIoOnboardingPrompter;
        prompt_onboarding_answers(&mut prompter, &defaults).map_err(onboarding_error)?
    };
    let request = if args.non_interactive {
        OnboardingRequest::NonInteractive {
            answers,
            force_overwrite: args.force_overwrite,
        }
    } else {
        OnboardingRequest::Interactive { answers }
    };
    let existing = read_existing_open_bitcoin_config(config_resolution.maybe_config_path.as_ref())
        .map_err(onboarding_error)?;
    let plan = plan_onboarding(&config_resolution, existing, detections, request)
        .map_err(onboarding_error)?;
    apply_onboarding_plan(&plan).map_err(onboarding_error)?;
    Ok(OperatorCommandOutcome::success(format!(
        "{}\n",
        render_onboarding_plan(&plan)
    )))
}

fn onboarding_error(error: OnboardingError) -> OperatorRuntimeError {
    OperatorRuntimeError::InvalidRequest {
        message: error.to_string(),
    }
}

fn render_config_paths(
    resolution: &OperatorConfigResolution,
    format: OperatorOutputFormat,
) -> Result<String, OperatorRuntimeError> {
    let sources = resolution
        .source_names()
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    if format == OperatorOutputFormat::Json {
        return serde_json::to_string_pretty(&json!({
            "config_path": resolution.maybe_config_path.as_ref().map(|path| path.display().to_string()),
            "bitcoin_conf_path": resolution.maybe_bitcoin_conf_path.as_ref().map(|path| path.display().to_string()),
            "datadir": resolution.maybe_data_dir.as_ref().map(|path| path.display().to_string()),
            "log_dir": resolution.maybe_log_dir.as_ref().map(|path| path.display().to_string()),
            "metrics_store_path": resolution.maybe_metrics_store_path.as_ref().map(|path| path.display().to_string()),
            "sources_considered": sources,
        }))
        .map(|value| format!("{value}\n"))
        .map_err(|error| OperatorRuntimeError::InvalidRequest {
            message: error.to_string(),
        });
    }
    Ok(format!(
        "Config: {}\nBitcoin config: {}\nDatadir: {}\nLogs: {}\nMetrics: {}\nSources: {}\n",
        display_path(resolution.maybe_config_path.as_deref()),
        display_path(resolution.maybe_bitcoin_conf_path.as_deref()),
        display_path(resolution.maybe_data_dir.as_deref()),
        display_path(resolution.maybe_log_dir.as_deref()),
        display_path(resolution.maybe_metrics_store_path.as_deref()),
        sources.join(" > ")
    ))
}

fn startup_config_for_status(
    resolution: &OperatorConfigResolution,
) -> Option<crate::startup::CliStartupConfig> {
    let conf_path = resolution.maybe_bitcoin_conf_path.as_ref()?;
    if !conf_path.exists() {
        return None;
    }
    let startup = CliStartupArgs {
        maybe_conf_path: Some(conf_path.clone()),
        maybe_data_dir: resolution.maybe_data_dir.clone(),
        ..CliStartupArgs::default()
    };
    resolve_startup_config(
        &startup,
        resolution
            .maybe_data_dir
            .as_deref()
            .unwrap_or_else(|| Path::new(".")),
    )
    .ok()
}

fn detection_roots(resolution: &OperatorConfigResolution) -> DetectionRoots {
    let home_dir = env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let data_dirs = resolution
        .maybe_data_dir
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    let service_dirs = {
        #[cfg(target_os = "macos")]
        {
            vec![home_dir.join("Library/LaunchAgents")]
        }
        #[cfg(target_os = "linux")]
        {
            vec![home_dir.join(".config/systemd/user")]
        }
        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        {
            Vec::new()
        }
    };
    DetectionRoots {
        home_dir,
        config_dirs: data_dirs.clone(),
        data_dirs,
        service_dirs,
    }
}

fn operator_environment() -> BTreeMap<String, String> {
    [
        OPEN_BITCOIN_CONFIG_ENV,
        OPEN_BITCOIN_DATADIR_ENV,
        OPEN_BITCOIN_NETWORK_ENV,
    ]
    .into_iter()
    .filter_map(|name| env::var(name).ok().map(|value| (name.to_string(), value)))
    .collect()
}

fn operator_home_dir() -> PathBuf {
    env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn status_render_mode(format: OperatorOutputFormat) -> StatusRenderMode {
    match format {
        OperatorOutputFormat::Human => StatusRenderMode::Human,
        OperatorOutputFormat::Json => StatusRenderMode::Json,
    }
}

fn auth_source(auth: &RpcAuthConfig) -> StatusRpcAuthSource {
    match auth {
        RpcAuthConfig::Cookie { maybe_cookie_file } => StatusRpcAuthSource::CookieFile {
            path: maybe_cookie_file
                .clone()
                .unwrap_or_else(|| PathBuf::from(".cookie")),
        },
        RpcAuthConfig::UserPassword { .. } => StatusRpcAuthSource::UserCredentialsConfigured,
    }
}

pub(crate) fn format_host_for_url(host: &str, port: u16) -> String {
    if host.contains(':') && !host.starts_with('[') {
        format!("[{host}]:{port}")
    } else {
        format!("{host}:{port}")
    }
}

pub(crate) fn authorization_header(auth: &RpcAuthConfig) -> Result<String, OperatorRuntimeError> {
    let credentials = match auth {
        RpcAuthConfig::UserPassword { username, password } => {
            format!("{username}:{password}")
        }
        RpcAuthConfig::Cookie { maybe_cookie_file } => {
            let cookie_file = maybe_cookie_file
                .clone()
                .unwrap_or_else(|| PathBuf::from(".cookie"));
            let contents = fs::read_to_string(&cookie_file).map_err(|_| {
                OperatorRuntimeError::InvalidRequest {
                    message: format!(
                        "Could not locate RPC credentials. No authentication cookie was found at {}",
                        cookie_file.display()
                    ),
                }
            })?;
            let Some((username, password)) = contents.trim().split_once(':') else {
                return Err(OperatorRuntimeError::InvalidRequest {
                    message: format!(
                        "Could not parse RPC credentials from {}",
                        cookie_file.display()
                    ),
                });
            };
            format!("{username}:{password}")
        }
    };
    Ok(format!("Basic {}", base64_encode(credentials.as_bytes())))
}

fn base64_encode(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        let triple = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);
        output.push(TABLE[((triple >> 18) & 0x3f) as usize] as char);
        output.push(TABLE[((triple >> 12) & 0x3f) as usize] as char);
        output.push(if chunk.len() > 1 {
            TABLE[((triple >> 6) & 0x3f) as usize] as char
        } else {
            '='
        });
        output.push(if chunk.len() > 2 {
            TABLE[(triple & 0x3f) as usize] as char
        } else {
            '='
        });
    }
    output
}

fn display_path(maybe_path: Option<&Path>) -> String {
    maybe_path
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "Unavailable".to_string())
}

fn default_operator_data_dir() -> PathBuf {
    if let Some(path) = env::var_os(OPEN_BITCOIN_DATADIR_ENV) {
        return PathBuf::from(path);
    }
    if let Some(path) = env::var_os("HOME") {
        return PathBuf::from(path).join(".open-bitcoin");
    }
    PathBuf::from(".open-bitcoin")
}
