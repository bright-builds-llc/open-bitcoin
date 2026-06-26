// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use open_bitcoin_primitives::NetworkMagic;

use crate::{
    DisconnectReason, LocalPeerConfig, NetworkError, ParsedNetworkMessage, PeerAction, PeerId,
    PeerManager, ServiceFlags, WireNetworkMessage,
};

const HARNESS_PEER_ID: PeerId = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibilityDiagnosis {
    Compatible,
    VersionRejected,
    NetworkMismatch,
    ServiceBitMismatch,
    UnsupportedMessageOrder,
    Timeout,
    PeerDisconnect,
    MalformedPayload,
    LocalConfigurationFailure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranscriptEvent {
    OutboundConnect,
    ReceiveMessage(WireNetworkMessage),
    ReceiveWire(Vec<u8>),
    Timeout,
    PeerDisconnect,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranscriptStep {
    pub event: String,
    pub observed_command: Option<String>,
    pub sent_commands: Vec<String>,
    pub diagnosis: CompatibilityDiagnosis,
    pub useful_progress: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityReport {
    pub transcript_name: String,
    pub steps: Vec<TranscriptStep>,
    pub diagnosis: CompatibilityDiagnosis,
    pub useful_progress: bool,
    pub next_action: String,
}

pub fn evaluate_transcript(
    name: impl Into<String>,
    events: impl IntoIterator<Item = TranscriptEvent>,
) -> CompatibilityReport {
    let mut harness = CompatibilityHarness::default();
    let mut steps = Vec::new();
    let mut final_diagnosis = CompatibilityDiagnosis::Compatible;

    for event in events {
        let step = harness.apply_event(event);
        if step.diagnosis != CompatibilityDiagnosis::Compatible {
            final_diagnosis = step.diagnosis;
            steps.push(step);
            break;
        }
        steps.push(step);
    }

    let useful_progress = steps.iter().any(|step| step.useful_progress);
    CompatibilityReport {
        transcript_name: name.into(),
        steps,
        diagnosis: final_diagnosis,
        useful_progress: useful_progress && final_diagnosis == CompatibilityDiagnosis::Compatible,
        next_action: next_action(final_diagnosis).to_string(),
    }
}

struct CompatibilityHarness {
    manager: PeerManager,
    connected: bool,
}

impl Default for CompatibilityHarness {
    fn default() -> Self {
        Self {
            manager: PeerManager::new(LocalPeerConfig::default()),
            connected: false,
        }
    }
}

impl CompatibilityHarness {
    fn apply_event(&mut self, event: TranscriptEvent) -> TranscriptStep {
        match event {
            TranscriptEvent::OutboundConnect => self.apply_outbound_connect(),
            TranscriptEvent::ReceiveMessage(message) => self.apply_message(message),
            TranscriptEvent::ReceiveWire(bytes) => self.apply_wire(bytes),
            TranscriptEvent::Timeout => {
                terminal_step("timeout", None, CompatibilityDiagnosis::Timeout)
            }
            TranscriptEvent::PeerDisconnect => terminal_step(
                "peer-disconnect",
                None,
                CompatibilityDiagnosis::PeerDisconnect,
            ),
        }
    }

    fn apply_outbound_connect(&mut self) -> TranscriptStep {
        match self.manager.add_outbound_peer(HARNESS_PEER_ID, 0) {
            Ok(actions) => {
                self.connected = true;
                step_from_actions(
                    "outbound-connect",
                    None,
                    actions,
                    CompatibilityDiagnosis::Compatible,
                )
            }
            Err(_) => terminal_step(
                "outbound-connect",
                None,
                CompatibilityDiagnosis::LocalConfigurationFailure,
            ),
        }
    }

    fn apply_wire(&mut self, bytes: Vec<u8>) -> TranscriptStep {
        match ParsedNetworkMessage::decode_wire(&bytes) {
            Ok(parsed) => {
                if parsed.header.magic != NetworkMagic::MAINNET {
                    return terminal_step(
                        "receive-wire",
                        Some(parsed.message.command_name().to_string()),
                        CompatibilityDiagnosis::NetworkMismatch,
                    );
                }
                self.apply_decoded_message(parsed.message, "receive-wire")
            }
            Err(_) => terminal_step(
                "receive-wire",
                None,
                CompatibilityDiagnosis::MalformedPayload,
            ),
        }
    }

    fn apply_message(&mut self, message: WireNetworkMessage) -> TranscriptStep {
        let event = format!("receive:{}", message.command_name());
        self.apply_decoded_message(message, event)
    }

    fn apply_decoded_message(
        &mut self,
        message: WireNetworkMessage,
        event: impl Into<String>,
    ) -> TranscriptStep {
        let event = event.into();
        let observed_command = Some(message.command_name().to_string());

        if !self.connected {
            return terminal_step(
                event,
                observed_command,
                CompatibilityDiagnosis::LocalConfigurationFailure,
            );
        }

        if self.is_unsupported_order(&message) {
            return terminal_step(
                event,
                observed_command,
                CompatibilityDiagnosis::UnsupportedMessageOrder,
            );
        }

        if let WireNetworkMessage::Version(version) = &message
            && (!version.services.contains(ServiceFlags::NETWORK)
                || !version.services.contains(ServiceFlags::WITNESS))
        {
            return terminal_step(
                event,
                observed_command,
                CompatibilityDiagnosis::ServiceBitMismatch,
            );
        }

        match self.manager.handle_message(HARNESS_PEER_ID, message, 1) {
            Ok(actions) => {
                let diagnosis = diagnose_actions(&actions);
                step_from_actions(event, observed_command, actions, diagnosis)
            }
            Err(error) => terminal_step(event, observed_command, diagnose_error(&error)),
        }
    }

    fn is_unsupported_order(&self, message: &WireNetworkMessage) -> bool {
        if !matches!(message, WireNetworkMessage::WtxidRelay) {
            return false;
        }
        self.manager
            .peer_state(HARNESS_PEER_ID)
            .is_some_and(|peer| peer.remote_verack_received)
    }
}

fn diagnose_actions(actions: &[PeerAction]) -> CompatibilityDiagnosis {
    let maybe_disconnect = actions.iter().find_map(|action| match action {
        PeerAction::Disconnect(reason) => Some(reason),
        _ => None,
    });
    match maybe_disconnect {
        Some(DisconnectReason::DuplicateVersion) => CompatibilityDiagnosis::VersionRejected,
        Some(DisconnectReason::SelfConnection) => CompatibilityDiagnosis::VersionRejected,
        Some(DisconnectReason::ResourceLimit) => CompatibilityDiagnosis::VersionRejected,
        Some(DisconnectReason::MissingHeaderAncestor(_)) => {
            CompatibilityDiagnosis::MalformedPayload
        }
        None => CompatibilityDiagnosis::Compatible,
    }
}

fn diagnose_error(error: &NetworkError) -> CompatibilityDiagnosis {
    match error {
        NetworkError::PeerAlreadyExists(_) | NetworkError::UnknownPeer(_) => {
            CompatibilityDiagnosis::LocalConfigurationFailure
        }
        NetworkError::DuplicateVersion(_) | NetworkError::SelfConnection(_) => {
            CompatibilityDiagnosis::VersionRejected
        }
        _ => CompatibilityDiagnosis::MalformedPayload,
    }
}

fn step_from_actions(
    event: impl Into<String>,
    observed_command: Option<String>,
    actions: Vec<PeerAction>,
    diagnosis: CompatibilityDiagnosis,
) -> TranscriptStep {
    let sent_commands = sent_commands(actions);
    let useful_progress = diagnosis == CompatibilityDiagnosis::Compatible
        && sent_commands
            .iter()
            .any(|command| command == "getheaders" || command == "getdata");
    TranscriptStep {
        event: event.into(),
        observed_command,
        sent_commands,
        diagnosis,
        useful_progress,
    }
}

fn terminal_step(
    event: impl Into<String>,
    observed_command: Option<String>,
    diagnosis: CompatibilityDiagnosis,
) -> TranscriptStep {
    TranscriptStep {
        event: event.into(),
        observed_command,
        sent_commands: Vec::new(),
        diagnosis,
        useful_progress: false,
    }
}

fn sent_commands(actions: Vec<PeerAction>) -> Vec<String> {
    actions
        .into_iter()
        .filter_map(|action| match action {
            PeerAction::Send(message) => Some(message.command_name().to_string()),
            _ => None,
        })
        .collect()
}

fn next_action(diagnosis: CompatibilityDiagnosis) -> &'static str {
    match diagnosis {
        CompatibilityDiagnosis::Compatible => "Continue with header or block sync evidence.",
        CompatibilityDiagnosis::VersionRejected => {
            "Compare version fields with the Knots baseline and preserve the rejection reason."
        }
        CompatibilityDiagnosis::NetworkMismatch => {
            "Check selected network magic and retry with a peer on the configured network."
        }
        CompatibilityDiagnosis::ServiceBitMismatch => {
            "Retry with a peer advertising NODE_NETWORK and NODE_WITNESS service bits."
        }
        CompatibilityDiagnosis::UnsupportedMessageOrder => {
            "Inspect peer message order; capability negotiation must happen before verack."
        }
        CompatibilityDiagnosis::Timeout => {
            "Increase the deterministic timeout or retry with a scripted peer that responds."
        }
        CompatibilityDiagnosis::PeerDisconnect => {
            "Inspect the preceding transcript step and retry with a peer that keeps the connection open."
        }
        CompatibilityDiagnosis::MalformedPayload => {
            "Inspect the wire payload, command, size, checksum, and compact-size fields."
        }
        CompatibilityDiagnosis::LocalConfigurationFailure => {
            "Fix local harness configuration before retrying the transcript."
        }
    }
}

#[cfg(test)]
mod tests {
    use open_bitcoin_primitives::{BlockHash, BlockHeader, MerkleRoot, NetworkMagic};

    use super::{CompatibilityDiagnosis, TranscriptEvent, evaluate_transcript};
    use crate::{
        DisconnectReason, HeadersMessage, NetworkError, PeerAction, ServiceFlags, VersionMessage,
        WireNetworkMessage,
    };

    fn transcript_with(events: Vec<TranscriptEvent>) -> super::CompatibilityReport {
        evaluate_transcript("test transcript", events)
    }

    #[test]
    fn compatible_outbound_transcript_records_knots_like_early_messages() {
        // Arrange
        let events = vec![
            TranscriptEvent::OutboundConnect,
            TranscriptEvent::ReceiveMessage(WireNetworkMessage::Version(VersionMessage {
                start_height: 3,
                ..VersionMessage::default()
            })),
            TranscriptEvent::ReceiveMessage(WireNetworkMessage::Verack),
        ];

        // Act
        let report = transcript_with(events);

        // Assert
        assert_eq!(report.diagnosis, CompatibilityDiagnosis::Compatible);
        assert!(report.useful_progress);
        assert_eq!(report.steps[0].sent_commands, vec!["version"]);
        assert_eq!(
            report.steps[1].sent_commands,
            vec!["wtxidrelay", "verack", "sendheaders"],
        );
        assert_eq!(report.steps[2].sent_commands, vec!["getheaders"]);
    }

    #[test]
    fn duplicate_version_maps_to_version_rejected() {
        // Arrange
        let events = vec![
            TranscriptEvent::OutboundConnect,
            TranscriptEvent::ReceiveMessage(WireNetworkMessage::Version(VersionMessage::default())),
            TranscriptEvent::ReceiveMessage(WireNetworkMessage::Version(VersionMessage::default())),
        ];

        // Act
        let report = transcript_with(events);

        // Assert
        assert_eq!(report.diagnosis, CompatibilityDiagnosis::VersionRejected);
        assert!(!report.useful_progress);
    }

    #[test]
    fn wrong_network_magic_maps_to_network_mismatch() {
        // Arrange
        let wire = WireNetworkMessage::Version(VersionMessage::default())
            .encode_wire(NetworkMagic::from_bytes([1, 2, 3, 4]))
            .expect("valid wire message");
        let events = vec![
            TranscriptEvent::OutboundConnect,
            TranscriptEvent::ReceiveWire(wire),
        ];

        // Act
        let report = transcript_with(events);

        // Assert
        assert_eq!(report.diagnosis, CompatibilityDiagnosis::NetworkMismatch);
        assert!(!report.useful_progress);
    }

    #[test]
    fn mainnet_wire_messages_flow_through_the_transcript_harness() {
        // Arrange
        let wire = WireNetworkMessage::Version(VersionMessage::default())
            .encode_wire(NetworkMagic::MAINNET)
            .expect("valid mainnet wire message");
        let events = vec![
            TranscriptEvent::OutboundConnect,
            TranscriptEvent::ReceiveWire(wire),
        ];

        // Act
        let report = transcript_with(events);

        // Assert
        assert_eq!(report.diagnosis, CompatibilityDiagnosis::Compatible);
        assert_eq!(
            report.steps[1].sent_commands,
            vec!["wtxidrelay", "verack", "sendheaders"],
        );
    }

    #[test]
    fn malformed_wire_maps_to_malformed_payload() {
        // Arrange
        let events = vec![
            TranscriptEvent::OutboundConnect,
            TranscriptEvent::ReceiveWire(vec![0, 1, 2]),
        ];

        // Act
        let report = transcript_with(events);

        // Assert
        assert_eq!(report.diagnosis, CompatibilityDiagnosis::MalformedPayload);
        assert!(!report.useful_progress);
    }

    #[test]
    fn missing_required_services_maps_to_service_bit_mismatch() {
        // Arrange
        let events = vec![
            TranscriptEvent::OutboundConnect,
            TranscriptEvent::ReceiveMessage(WireNetworkMessage::Version(VersionMessage {
                services: ServiceFlags::NETWORK,
                ..VersionMessage::default()
            })),
        ];

        // Act
        let report = transcript_with(events);

        // Assert
        assert_eq!(report.diagnosis, CompatibilityDiagnosis::ServiceBitMismatch);
        assert!(!report.useful_progress);
    }

    #[test]
    fn wtxidrelay_after_verack_maps_to_unsupported_message_order() {
        // Arrange
        let events = vec![
            TranscriptEvent::OutboundConnect,
            TranscriptEvent::ReceiveMessage(WireNetworkMessage::Version(VersionMessage {
                start_height: 3,
                ..VersionMessage::default()
            })),
            TranscriptEvent::ReceiveMessage(WireNetworkMessage::Verack),
            TranscriptEvent::ReceiveMessage(WireNetworkMessage::WtxidRelay),
        ];

        // Act
        let report = transcript_with(events);

        // Assert
        assert_eq!(
            report.diagnosis,
            CompatibilityDiagnosis::UnsupportedMessageOrder,
        );
        assert!(!report.useful_progress);
    }

    #[test]
    fn scripted_timeout_and_disconnect_have_distinct_diagnoses() {
        // Arrange
        let timeout_events = vec![TranscriptEvent::OutboundConnect, TranscriptEvent::Timeout];
        let disconnect_events = vec![
            TranscriptEvent::OutboundConnect,
            TranscriptEvent::PeerDisconnect,
        ];

        // Act
        let timeout_report = transcript_with(timeout_events);
        let disconnect_report = transcript_with(disconnect_events);

        // Assert
        assert_eq!(timeout_report.diagnosis, CompatibilityDiagnosis::Timeout);
        assert_eq!(
            disconnect_report.diagnosis,
            CompatibilityDiagnosis::PeerDisconnect,
        );
        assert!(!timeout_report.useful_progress);
        assert!(!disconnect_report.useful_progress);
    }

    #[test]
    fn local_configuration_failures_are_distinct_from_peer_failures() {
        // Arrange
        let missing_connect_events =
            vec![TranscriptEvent::ReceiveMessage(WireNetworkMessage::Verack)];
        let duplicate_connect_events = vec![
            TranscriptEvent::OutboundConnect,
            TranscriptEvent::OutboundConnect,
        ];

        // Act
        let missing_connect_report = transcript_with(missing_connect_events);
        let duplicate_connect_report = transcript_with(duplicate_connect_events);

        // Assert
        assert_eq!(
            missing_connect_report.diagnosis,
            CompatibilityDiagnosis::LocalConfigurationFailure,
        );
        assert_eq!(
            duplicate_connect_report.diagnosis,
            CompatibilityDiagnosis::LocalConfigurationFailure,
        );
        assert!(missing_connect_report.next_action.contains("local harness"));
        assert!(!duplicate_connect_report.useful_progress);
    }

    #[test]
    fn invalid_header_payload_maps_to_malformed_payload() {
        // Arrange
        let invalid_header = BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([9_u8; 32]),
            merkle_root: MerkleRoot::from_byte_array([8_u8; 32]),
            time: 1,
            bits: 0,
            nonce: 0,
        };
        let events = vec![
            TranscriptEvent::OutboundConnect,
            TranscriptEvent::ReceiveMessage(WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![invalid_header],
            })),
        ];

        // Act
        let report = transcript_with(events);

        // Assert
        assert_eq!(report.diagnosis, CompatibilityDiagnosis::MalformedPayload);
        assert!(!report.useful_progress);
    }

    #[test]
    fn helper_diagnostics_cover_unreachable_peer_manager_variants() {
        // Arrange
        let missing_parent = BlockHash::from_byte_array([4_u8; 32]);

        // Act
        let missing_ancestor_diagnosis = super::diagnose_actions(&[PeerAction::Disconnect(
            DisconnectReason::MissingHeaderAncestor(missing_parent),
        )]);
        let self_connection_diagnosis =
            super::diagnose_actions(&[PeerAction::Disconnect(DisconnectReason::SelfConnection)]);
        let unknown_peer_diagnosis = super::diagnose_error(&NetworkError::UnknownPeer(77));
        let duplicate_version_diagnosis =
            super::diagnose_error(&NetworkError::DuplicateVersion(77));

        // Assert
        assert_eq!(
            missing_ancestor_diagnosis,
            CompatibilityDiagnosis::MalformedPayload,
        );
        assert_eq!(
            self_connection_diagnosis,
            CompatibilityDiagnosis::VersionRejected,
        );
        assert_eq!(
            unknown_peer_diagnosis,
            CompatibilityDiagnosis::LocalConfigurationFailure,
        );
        assert_eq!(
            duplicate_version_diagnosis,
            CompatibilityDiagnosis::VersionRejected,
        );
    }

    #[test]
    fn resource_limit_disconnect_maps_to_version_rejected() {
        // Arrange
        let actions = vec![PeerAction::Disconnect(DisconnectReason::ResourceLimit)];

        // Act
        let diagnosis = super::diagnose_actions(&actions);

        // Assert
        assert_eq!(diagnosis, CompatibilityDiagnosis::VersionRejected);
    }
}
