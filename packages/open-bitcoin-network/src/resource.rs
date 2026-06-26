// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_invalid_messages.py

use open_bitcoin_codec::{CodecError, parse_message_header};
use open_bitcoin_consensus::crypto::double_sha256;
use open_bitcoin_primitives::{MessageHeader, NetworkMagic};

use crate::{NetworkError, ParsedNetworkMessage, WireNetworkMessage};

pub const INBOUND_MESSAGE_HEADER_LEN: usize = 24;
pub const PHASE94_MAX_INBOUND_RUNTIME_PAYLOAD_BYTES: usize = 32 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceViolationLabel {
    WrongNetworkMagic,
    MalformedHeader,
    PayloadOversized,
    InvalidChecksum,
    UnsupportedCommand,
    MalformedPayload,
    TrailingPayload,
}

impl ResourceViolationLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongNetworkMagic => "wrong_network_magic",
            Self::MalformedHeader => "malformed_header",
            Self::PayloadOversized => "payload_oversized",
            Self::InvalidChecksum => "invalid_checksum",
            Self::UnsupportedCommand => "unsupported_command",
            Self::MalformedPayload => "malformed_payload",
            Self::TrailingPayload => "trailing_payload",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceGovernanceSource {
    EnvelopeGate,
    PayloadDecoder,
    RuntimeRead,
}

impl ResourceGovernanceSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnvelopeGate => "source_envelope_gate",
            Self::PayloadDecoder => "source_payload_decoder",
            Self::RuntimeRead => "source_runtime_read",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboundResourceEvent {
    pub outcome: String,
    pub reason: String,
    pub label: String,
    pub source: String,
    pub message: String,
    pub next_action: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InboundEnvelopeDecision {
    ReadPayload { payload_len: usize },
    Reject(InboundResourceEvent),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InboundEnvelopePolicy {
    pub expected_magic: NetworkMagic,
    pub max_payload_bytes: usize,
}

impl InboundEnvelopePolicy {
    pub const fn new(expected_magic: NetworkMagic) -> Self {
        Self {
            expected_magic,
            max_payload_bytes: PHASE94_MAX_INBOUND_RUNTIME_PAYLOAD_BYTES,
        }
    }

    pub fn evaluate_header(&self, header_bytes: &[u8]) -> InboundEnvelopeDecision {
        if header_bytes.len() != INBOUND_MESSAGE_HEADER_LEN {
            return InboundEnvelopeDecision::Reject(Self::event(
                ResourceViolationLabel::MalformedHeader,
                ResourceGovernanceSource::EnvelopeGate,
                "message header is not exactly 24 bytes",
            ));
        }

        let header = match parse_message_header(header_bytes) {
            Ok(header) => header,
            Err(_) => {
                return InboundEnvelopeDecision::Reject(Self::event(
                    ResourceViolationLabel::MalformedHeader,
                    ResourceGovernanceSource::EnvelopeGate,
                    "message header parse failed",
                ));
            }
        };

        if header.magic != self.expected_magic {
            return InboundEnvelopeDecision::Reject(Self::event(
                ResourceViolationLabel::WrongNetworkMagic,
                ResourceGovernanceSource::EnvelopeGate,
                "message network magic did not match local network",
            ));
        }

        let payload_len = header.payload_size as usize;
        if payload_len > self.max_payload_bytes {
            return InboundEnvelopeDecision::Reject(Self::event(
                ResourceViolationLabel::PayloadOversized,
                ResourceGovernanceSource::EnvelopeGate,
                "message payload length exceeds inbound runtime cap",
            ));
        }

        if !is_supported_command(header.command.as_str()) {
            return InboundEnvelopeDecision::Reject(Self::event(
                ResourceViolationLabel::UnsupportedCommand,
                ResourceGovernanceSource::EnvelopeGate,
                "message command is outside the supported inbound surface",
            ));
        }

        InboundEnvelopeDecision::ReadPayload { payload_len }
    }

    #[allow(clippy::result_large_err)]
    pub fn decode_payload(
        &self,
        header: &MessageHeader,
        payload: &[u8],
    ) -> Result<ParsedNetworkMessage, InboundResourceEvent> {
        if header.magic != self.expected_magic {
            return Err(Self::event(
                ResourceViolationLabel::WrongNetworkMagic,
                ResourceGovernanceSource::EnvelopeGate,
                "message network magic did not match local network",
            ));
        }

        if !is_supported_command(header.command.as_str()) {
            return Err(Self::event(
                ResourceViolationLabel::UnsupportedCommand,
                ResourceGovernanceSource::EnvelopeGate,
                "message command is outside the supported inbound surface",
            ));
        }

        let expected_payload_len = header.payload_size as usize;
        if expected_payload_len > self.max_payload_bytes {
            return Err(Self::event(
                ResourceViolationLabel::PayloadOversized,
                ResourceGovernanceSource::EnvelopeGate,
                "message payload length exceeds inbound runtime cap",
            ));
        }

        if payload.len() > expected_payload_len {
            return Err(Self::event(
                ResourceViolationLabel::TrailingPayload,
                ResourceGovernanceSource::PayloadDecoder,
                "payload contains trailing bytes beyond header length",
            ));
        }

        if payload.len() < expected_payload_len {
            return Err(Self::event(
                ResourceViolationLabel::MalformedPayload,
                ResourceGovernanceSource::PayloadDecoder,
                "payload ended before header length",
            ));
        }

        if checksum(payload) != header.checksum {
            return Err(Self::event(
                ResourceViolationLabel::InvalidChecksum,
                ResourceGovernanceSource::EnvelopeGate,
                "payload checksum did not match message header",
            ));
        }

        let message = WireNetworkMessage::decode_payload(&header.command, payload)
            .map_err(resource_event_for_decode_error)?;
        Ok(ParsedNetworkMessage {
            header: header.clone(),
            message,
        })
    }

    pub fn event(
        label: ResourceViolationLabel,
        source: ResourceGovernanceSource,
        reason: impl Into<String>,
    ) -> InboundResourceEvent {
        InboundResourceEvent {
            outcome: "rejected".to_string(),
            reason: reason.into(),
            label: label.as_str().to_string(),
            source: source.as_str().to_string(),
            message: "inbound_message_resource_governance".to_string(),
            next_action: "payload_rejected".to_string(),
        }
    }
}

fn resource_event_for_decode_error(error: NetworkError) -> InboundResourceEvent {
    match error {
        NetworkError::UnknownCommand(_) => InboundEnvelopePolicy::event(
            ResourceViolationLabel::UnsupportedCommand,
            ResourceGovernanceSource::PayloadDecoder,
            "message command is outside the supported inbound surface",
        ),
        NetworkError::InvalidChecksum => InboundEnvelopePolicy::event(
            ResourceViolationLabel::InvalidChecksum,
            ResourceGovernanceSource::EnvelopeGate,
            "payload checksum did not match message header",
        ),
        NetworkError::Codec(CodecError::TrailingData { .. }) => InboundEnvelopePolicy::event(
            ResourceViolationLabel::TrailingPayload,
            ResourceGovernanceSource::PayloadDecoder,
            "payload decoder found trailing bytes",
        ),
        _ => InboundEnvelopePolicy::event(
            ResourceViolationLabel::MalformedPayload,
            ResourceGovernanceSource::PayloadDecoder,
            "payload decoder rejected malformed payload",
        ),
    }
}

fn is_supported_command(command: &str) -> bool {
    matches!(
        command,
        "version"
            | "verack"
            | "wtxidrelay"
            | "sendheaders"
            | "getaddr"
            | "ping"
            | "pong"
            | "getheaders"
            | "headers"
            | "addr"
            | "inv"
            | "getdata"
            | "notfound"
            | "tx"
            | "block"
    )
}

fn checksum(payload: &[u8]) -> [u8; 4] {
    let digest = double_sha256(payload);
    [digest[0], digest[1], digest[2], digest[3]]
}

#[cfg(test)]
mod tests;
