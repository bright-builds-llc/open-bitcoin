// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

//! Operator wrapper for deterministic compatibility harness reports.

use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_network::{
    CompatibilityDiagnosis, ServiceFlags, TranscriptEvent, VersionMessage, WireNetworkMessage,
    evaluate_transcript,
};
use open_bitcoin_primitives::NetworkMagic;
use serde::Serialize;
use serde_json::json;

use super::{
    CompatibilityArgs, CompatibilityCommand, CompatibilityHarnessArgs, CompatibilityScenario,
    NetworkSelection, OperatorOutputFormat,
    config::OperatorConfigResolution,
    runtime::{OperatorCommandOutcome, OperatorRuntimeError},
};

const COMPATIBILITY_REPORT_JSON: &str = "compatibility-harness-report.json";
const COMPATIBILITY_REPORT_MARKDOWN: &str = "compatibility-harness-report.md";

pub(crate) fn execute_compatibility_command(
    args: &CompatibilityArgs,
    format: OperatorOutputFormat,
    config_resolution: &OperatorConfigResolution,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    match &args.command {
        CompatibilityCommand::Harness(harness) => {
            execute_compatibility_harness(harness, format, config_resolution)
        }
    }
}

fn execute_compatibility_harness(
    args: &CompatibilityHarnessArgs,
    format: OperatorOutputFormat,
    config_resolution: &OperatorConfigResolution,
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let output_dir = compatibility_output_dir(args, config_resolution)?;
    fs::create_dir_all(&output_dir).map_err(|error| OperatorRuntimeError::InvalidRequest {
        message: format!(
            "could not create compatibility report output directory {}: {error}",
            output_dir.display()
        ),
    })?;

    let json_path = output_dir.join(COMPATIBILITY_REPORT_JSON);
    let markdown_path = output_dir.join(COMPATIBILITY_REPORT_MARKDOWN);
    let report = build_report(args, config_resolution, &json_path, &markdown_path)?;
    let json_text = serde_json::to_string_pretty(&report).map_err(|error| {
        OperatorRuntimeError::InvalidRequest {
            message: format!("could not encode compatibility report JSON: {error}"),
        }
    })?;
    fs::write(&json_path, format!("{json_text}\n")).map_err(|error| {
        OperatorRuntimeError::InvalidRequest {
            message: format!(
                "could not write compatibility report JSON {}: {error}",
                json_path.display()
            ),
        }
    })?;
    fs::write(&markdown_path, render_compatibility_markdown(&report)).map_err(|error| {
        OperatorRuntimeError::InvalidRequest {
            message: format!(
                "could not write compatibility report Markdown {}: {error}",
                markdown_path.display()
            ),
        }
    })?;

    Ok(OperatorCommandOutcome::success(
        render_compatibility_outcome(&report, format)?,
    ))
}

fn compatibility_output_dir(
    args: &CompatibilityHarnessArgs,
    config_resolution: &OperatorConfigResolution,
) -> Result<PathBuf, OperatorRuntimeError> {
    if let Some(output_dir) = args.maybe_output_dir.as_ref() {
        return Ok(output_dir.clone());
    }
    let Some(data_dir) = config_resolution.maybe_data_dir.as_ref() else {
        return Err(OperatorRuntimeError::InvalidRequest {
            message: "compatibility harness requires --output-dir when no datadir is available"
                .to_string(),
        });
    };

    Ok(data_dir.join("compatibility"))
}

fn build_report(
    args: &CompatibilityHarnessArgs,
    config_resolution: &OperatorConfigResolution,
    json_path: &Path,
    markdown_path: &Path,
) -> Result<CompatibilityHarnessReport, OperatorRuntimeError> {
    let scenario = args.scenario;
    let network = config_resolution
        .maybe_network
        .unwrap_or(NetworkSelection::Mainnet);
    let report = evaluate_transcript(scenario_name(scenario), scenario_events(scenario)?);
    let steps = report
        .steps
        .iter()
        .map(|step| CompatibilityHarnessStep {
            event: step.event.clone(),
            observed_command: step.observed_command.clone(),
            sent_commands: step.sent_commands.clone(),
            diagnosis: diagnosis_name(step.diagnosis).to_string(),
            useful_progress: step.useful_progress,
        })
        .collect::<Vec<_>>();
    let maybe_failing_step = steps
        .iter()
        .find(|step| step.diagnosis != "compatible")
        .cloned();

    Ok(CompatibilityHarnessReport {
        generated_at_unix_seconds: current_unix_seconds(),
        generated_by: "open-bitcoin compatibility harness".to_string(),
        peer_endpoint: args.peer_endpoint.clone(),
        network: network_name(network).to_string(),
        scenario: scenario_name(scenario).to_string(),
        diagnosis: diagnosis_name(report.diagnosis).to_string(),
        useful_progress: report.useful_progress,
        negotiated_capabilities: negotiated_capabilities(&steps),
        failing_step: maybe_failing_step,
        transcript_summary: transcript_summary(&steps, report.useful_progress),
        redaction_boundaries: redaction_boundaries(),
        next_action: report.next_action,
        output: CompatibilityHarnessOutput {
            json_path: path_to_string(json_path),
            markdown_path: path_to_string(markdown_path),
        },
        steps,
    })
}

fn scenario_events(
    scenario: CompatibilityScenario,
) -> Result<Vec<TranscriptEvent>, OperatorRuntimeError> {
    match scenario {
        CompatibilityScenario::Compatible => Ok(vec![
            TranscriptEvent::OutboundConnect,
            TranscriptEvent::ReceiveMessage(WireNetworkMessage::Version(VersionMessage {
                start_height: 3,
                ..VersionMessage::default()
            })),
            TranscriptEvent::ReceiveMessage(WireNetworkMessage::Verack),
        ]),
        CompatibilityScenario::VersionRejected => Ok(vec![
            TranscriptEvent::OutboundConnect,
            TranscriptEvent::ReceiveMessage(WireNetworkMessage::Version(VersionMessage::default())),
            TranscriptEvent::ReceiveMessage(WireNetworkMessage::Version(VersionMessage::default())),
        ]),
        CompatibilityScenario::NetworkMismatch => {
            let wire = WireNetworkMessage::Version(VersionMessage::default())
                .encode_wire(NetworkMagic::from_bytes([1, 2, 3, 4]))
                .map_err(|error| OperatorRuntimeError::InvalidRequest {
                    message: format!("could not build wrong-network transcript: {error}"),
                })?;
            Ok(vec![
                TranscriptEvent::OutboundConnect,
                TranscriptEvent::ReceiveWire(wire),
            ])
        }
        CompatibilityScenario::ServiceBitMismatch => Ok(vec![
            TranscriptEvent::OutboundConnect,
            TranscriptEvent::ReceiveMessage(WireNetworkMessage::Version(VersionMessage {
                services: ServiceFlags::NETWORK,
                ..VersionMessage::default()
            })),
        ]),
        CompatibilityScenario::UnsupportedMessageOrder => Ok(vec![
            TranscriptEvent::OutboundConnect,
            TranscriptEvent::ReceiveMessage(WireNetworkMessage::Version(VersionMessage {
                start_height: 3,
                ..VersionMessage::default()
            })),
            TranscriptEvent::ReceiveMessage(WireNetworkMessage::Verack),
            TranscriptEvent::ReceiveMessage(WireNetworkMessage::WtxidRelay),
        ]),
        CompatibilityScenario::Timeout => Ok(vec![
            TranscriptEvent::OutboundConnect,
            TranscriptEvent::Timeout,
        ]),
        CompatibilityScenario::PeerDisconnect => Ok(vec![
            TranscriptEvent::OutboundConnect,
            TranscriptEvent::PeerDisconnect,
        ]),
        CompatibilityScenario::MalformedPayload => Ok(vec![
            TranscriptEvent::OutboundConnect,
            TranscriptEvent::ReceiveWire(vec![0, 1, 2]),
        ]),
        CompatibilityScenario::LocalConfigurationFailure => {
            Ok(vec![TranscriptEvent::ReceiveMessage(
                WireNetworkMessage::Verack,
            )])
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct CompatibilityHarnessReport {
    generated_at_unix_seconds: u64,
    generated_by: String,
    peer_endpoint: String,
    network: String,
    scenario: String,
    diagnosis: String,
    useful_progress: bool,
    negotiated_capabilities: NegotiatedCapabilities,
    failing_step: Option<CompatibilityHarnessStep>,
    transcript_summary: TranscriptSummary,
    redaction_boundaries: RedactionBoundaries,
    next_action: String,
    output: CompatibilityHarnessOutput,
    steps: Vec<CompatibilityHarnessStep>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct CompatibilityHarnessOutput {
    json_path: String,
    markdown_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct NegotiatedCapabilities {
    required_services: Vec<&'static str>,
    local_sent_commands: Vec<String>,
    remote_observed_commands: Vec<String>,
    wtxid_relay: bool,
    sendheaders: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct CompatibilityHarnessStep {
    event: String,
    observed_command: Option<String>,
    sent_commands: Vec<String>,
    diagnosis: String,
    useful_progress: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct TranscriptSummary {
    total_steps: usize,
    observed_commands: Vec<String>,
    sent_commands: Vec<String>,
    useful_progress: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct RedactionBoundaries {
    included: Vec<&'static str>,
    omitted: Vec<&'static str>,
}

fn negotiated_capabilities(steps: &[CompatibilityHarnessStep]) -> NegotiatedCapabilities {
    let sent_commands = unique_strings(
        steps
            .iter()
            .flat_map(|step| step.sent_commands.iter().cloned())
            .collect(),
    );
    let observed_commands = unique_strings(
        steps
            .iter()
            .filter_map(|step| step.observed_command.clone())
            .collect(),
    );
    NegotiatedCapabilities {
        required_services: vec!["NODE_NETWORK", "NODE_WITNESS"],
        wtxid_relay: sent_commands.iter().any(|command| command == "wtxidrelay")
            || observed_commands
                .iter()
                .any(|command| command == "wtxidrelay"),
        sendheaders: sent_commands.iter().any(|command| command == "sendheaders")
            || observed_commands
                .iter()
                .any(|command| command == "sendheaders"),
        local_sent_commands: sent_commands,
        remote_observed_commands: observed_commands,
    }
}

fn transcript_summary(
    steps: &[CompatibilityHarnessStep],
    useful_progress: bool,
) -> TranscriptSummary {
    TranscriptSummary {
        total_steps: steps.len(),
        observed_commands: unique_strings(
            steps
                .iter()
                .filter_map(|step| step.observed_command.clone())
                .collect(),
        ),
        sent_commands: unique_strings(
            steps
                .iter()
                .flat_map(|step| step.sent_commands.iter().cloned())
                .collect(),
        ),
        useful_progress,
    }
}

fn unique_strings(values: Vec<String>) -> Vec<String> {
    let mut unique = Vec::new();
    for value in values {
        if unique.iter().any(|existing| existing == &value) {
            continue;
        }
        unique.push(value);
    }
    unique
}

fn redaction_boundaries() -> RedactionBoundaries {
    RedactionBoundaries {
        included: vec![
            "peer endpoint label",
            "network",
            "scenario",
            "diagnosis",
            "step command names",
            "local next action",
        ],
        omitted: vec![
            "raw wire payloads",
            "daemon stdout/stderr tails",
            "RPC credentials and cookie contents",
            "wallet private material",
            "unbounded peer logs",
        ],
    }
}

fn render_compatibility_outcome(
    report: &CompatibilityHarnessReport,
    format: OperatorOutputFormat,
) -> Result<String, OperatorRuntimeError> {
    match format {
        OperatorOutputFormat::Human => Ok(format!(
            "Compatibility harness report written:\nJSON: {}\nMarkdown: {}\n",
            report.output.json_path, report.output.markdown_path
        )),
        OperatorOutputFormat::Json => {
            let output = json!({
                "json_path": report.output.json_path,
                "markdown_path": report.output.markdown_path,
                "diagnosis": report.diagnosis,
                "peer_endpoint": report.peer_endpoint,
                "network": report.network,
            });
            serde_json::to_string_pretty(&output)
                .map(|text| format!("{text}\n"))
                .map_err(|error| OperatorRuntimeError::InvalidRequest {
                    message: format!("could not encode compatibility command output: {error}"),
                })
        }
    }
}

fn render_compatibility_markdown(report: &CompatibilityHarnessReport) -> String {
    let mut output = String::new();
    output.push_str("# Open Bitcoin Compatibility Harness Report\n\n");
    output.push_str(&format!(
        "- Generated: {}\n",
        report.generated_at_unix_seconds
    ));
    output.push_str(&format!("- Peer endpoint: {}\n", report.peer_endpoint));
    output.push_str(&format!("- Network: {}\n", report.network));
    output.push_str(&format!("- Scenario: {}\n", report.scenario));
    output.push_str(&format!("- Diagnosis: {}\n", report.diagnosis));
    output.push_str(&format!(
        "- Useful progress: {}\n\n",
        report.useful_progress
    ));

    output.push_str("## Negotiated capabilities\n\n");
    output.push_str(&format!(
        "- Required services: {}\n",
        report.negotiated_capabilities.required_services.join(", ")
    ));
    output.push_str(&format!(
        "- Local sent commands: {}\n",
        display_list(&report.negotiated_capabilities.local_sent_commands)
    ));
    output.push_str(&format!(
        "- Remote observed commands: {}\n",
        display_list(&report.negotiated_capabilities.remote_observed_commands)
    ));
    output.push_str(&format!(
        "- wtxid relay: {}\n- sendheaders: {}\n\n",
        report.negotiated_capabilities.wtxid_relay, report.negotiated_capabilities.sendheaders
    ));

    output.push_str("## Failing step\n\n");
    match &report.failing_step {
        Some(step) => {
            output.push_str(&format!("- Event: {}\n", step.event));
            output.push_str(&format!(
                "- Observed command: {}\n",
                step.observed_command.as_deref().unwrap_or("unavailable")
            ));
            output.push_str(&format!("- Diagnosis: {}\n\n", step.diagnosis));
        }
        None => output.push_str("- None - transcript remained compatible.\n\n"),
    }

    output.push_str("## Transcript summary\n\n");
    output.push_str(&format!(
        "- Total steps: {}\n- Useful progress: {}\n",
        report.transcript_summary.total_steps, report.transcript_summary.useful_progress
    ));
    output.push_str(&format!(
        "- Observed commands: {}\n",
        display_list(&report.transcript_summary.observed_commands)
    ));
    output.push_str(&format!(
        "- Sent commands: {}\n\n",
        display_list(&report.transcript_summary.sent_commands)
    ));

    output.push_str("## Redaction boundaries\n\n");
    for item in &report.redaction_boundaries.included {
        output.push_str(&format!("- Included: {item}\n"));
    }
    for item in &report.redaction_boundaries.omitted {
        output.push_str(&format!("- Omitted: {item}\n"));
    }
    output.push_str(&format!("\n## Next action\n\n{}\n", report.next_action));
    output
}

fn display_list(values: &[String]) -> String {
    if values.is_empty() {
        return "none".to_string();
    }
    values.join(", ")
}

fn scenario_name(scenario: CompatibilityScenario) -> &'static str {
    match scenario {
        CompatibilityScenario::Compatible => "compatible",
        CompatibilityScenario::VersionRejected => "version_rejected",
        CompatibilityScenario::NetworkMismatch => "network_mismatch",
        CompatibilityScenario::ServiceBitMismatch => "service_bit_mismatch",
        CompatibilityScenario::UnsupportedMessageOrder => "unsupported_message_order",
        CompatibilityScenario::Timeout => "timeout",
        CompatibilityScenario::PeerDisconnect => "peer_disconnect",
        CompatibilityScenario::MalformedPayload => "malformed_payload",
        CompatibilityScenario::LocalConfigurationFailure => "local_configuration_failure",
    }
}

fn diagnosis_name(diagnosis: CompatibilityDiagnosis) -> &'static str {
    match diagnosis {
        CompatibilityDiagnosis::Compatible => "compatible",
        CompatibilityDiagnosis::VersionRejected => "version_rejected",
        CompatibilityDiagnosis::NetworkMismatch => "network_mismatch",
        CompatibilityDiagnosis::ServiceBitMismatch => "service_bit_mismatch",
        CompatibilityDiagnosis::UnsupportedMessageOrder => "unsupported_message_order",
        CompatibilityDiagnosis::Timeout => "timeout",
        CompatibilityDiagnosis::PeerDisconnect => "peer_disconnect",
        CompatibilityDiagnosis::MalformedPayload => "malformed_payload",
        CompatibilityDiagnosis::LocalConfigurationFailure => "local_configuration_failure",
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

fn current_unix_seconds() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(_) => 0,
    }
}

fn path_to_string(path: &Path) -> String {
    path.display().to_string()
}
