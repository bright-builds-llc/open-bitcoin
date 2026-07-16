// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/netbase.cpp
// - packages/bitcoin-knots/src/netbase.h
// - packages/bitcoin-knots/src/protocol.h

use std::{
    io::{self, Read, Write},
    net::TcpStream,
    time::Duration,
};

use open_bitcoin_core::{
    codec::{CodecError, MAX_SIZE, parse_message_header},
    primitives::{MessageHeader, NetworkMagic},
};
use open_bitcoin_network::{ParsedNetworkMessage, WireNetworkMessage};

use super::{
    ResolvedSyncPeerAddress, SyncPeerReceiveOutcome, SyncPeerSession, SyncRuntimeConfig,
    SyncRuntimeError, SyncTransport,
};

const WIRE_HEADER_LEN: usize = 24;

#[derive(Debug, Clone, Copy, Default)]
pub struct TcpPeerTransport;

pub struct TcpPeerSession {
    peer: String,
    stream: TcpStream,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ReadStageOutcome {
    Complete,
    Idle,
    Closed,
}

enum MessageHeaderOutcome {
    Complete(MessageHeader),
    Idle,
    Closed,
}

impl SyncTransport for TcpPeerTransport {
    type Session = TcpPeerSession;

    fn connect(
        &mut self,
        peer: &ResolvedSyncPeerAddress,
        config: &SyncRuntimeConfig,
    ) -> Result<Self::Session, SyncRuntimeError> {
        let stream = TcpStream::connect_timeout(
            &peer.endpoint,
            Duration::from_millis(config.connect_timeout_ms),
        )
        .map_err(|error| io_error(peer.label(), error))?;
        stream
            .set_read_timeout(Some(Duration::from_millis(config.read_timeout_ms)))
            .map_err(|error| io_error(peer.label(), error))?;
        stream
            .set_write_timeout(Some(Duration::from_millis(config.read_timeout_ms)))
            .map_err(|error| io_error(peer.label(), error))?;

        Ok(TcpPeerSession {
            peer: peer.label(),
            stream,
        })
    }
}

impl SyncPeerSession for TcpPeerSession {
    fn send(
        &mut self,
        message: &WireNetworkMessage,
        magic: NetworkMagic,
    ) -> Result<(), SyncRuntimeError> {
        let encoded = message
            .encode_wire(magic)
            .map_err(|error| SyncRuntimeError::Network {
                message: error.to_string(),
            })?;
        self.stream
            .write_all(&encoded)
            .map_err(|error| io_error(self.peer.clone(), error))
    }

    fn receive(&mut self, magic: NetworkMagic) -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> {
        let header = match read_message_header(&mut self.stream, &self.peer, magic)? {
            MessageHeaderOutcome::Complete(header) => header,
            MessageHeaderOutcome::Idle => return Ok(SyncPeerReceiveOutcome::Idle),
            MessageHeaderOutcome::Closed => return Ok(SyncPeerReceiveOutcome::Closed),
        };
        let payload_len = header.payload_size as usize;
        if payload_len as u64 > MAX_SIZE {
            return Err(SyncRuntimeError::Network {
                message: CodecError::LengthOutOfRange {
                    field: "payload size",
                    value: payload_len as u64,
                }
                .to_string(),
            });
        }

        let mut payload = vec![0_u8; payload_len];
        match read_stage_for_peer(&mut self.stream, &mut payload, false, &self.peer)? {
            ReadStageOutcome::Complete => {}
            ReadStageOutcome::Idle | ReadStageOutcome::Closed => {
                return Err(io_message(
                    self.peer.clone(),
                    "payload read ended without a complete frame".to_string(),
                ));
            }
        }
        let mut wire = Vec::with_capacity(WIRE_HEADER_LEN + payload.len());
        wire.extend_from_slice(&open_bitcoin_core::codec::encode_message_header(&header));
        wire.extend_from_slice(&payload);
        Ok(SyncPeerReceiveOutcome::Message(
            ParsedNetworkMessage::decode_wire(&wire)?.message,
        ))
    }
}

fn read_message_header(
    stream: &mut TcpStream,
    peer: &str,
    expected_magic: NetworkMagic,
) -> Result<MessageHeaderOutcome, SyncRuntimeError> {
    let mut header_bytes = [0_u8; WIRE_HEADER_LEN];
    match read_stage_for_peer(stream, &mut header_bytes, true, peer)? {
        ReadStageOutcome::Complete => {}
        ReadStageOutcome::Idle => return Ok(MessageHeaderOutcome::Idle),
        ReadStageOutcome::Closed => return Ok(MessageHeaderOutcome::Closed),
    }
    let header =
        parse_message_header(&header_bytes).map_err(|error| SyncRuntimeError::Network {
            message: error.to_string(),
        })?;
    if header.magic != expected_magic {
        return Err(SyncRuntimeError::InvalidMagic {
            expected: expected_magic.to_bytes(),
            actual: header.magic.to_bytes(),
        });
    }

    Ok(MessageHeaderOutcome::Complete(header))
}

pub(super) fn read_stage<R: Read>(
    reader: &mut R,
    buffer: &mut [u8],
    allow_clean_idle: bool,
) -> Result<ReadStageOutcome, SyncRuntimeError> {
    let mut filled = 0_usize;
    while filled < buffer.len() {
        match reader.read(&mut buffer[filled..]) {
            Ok(0) if allow_clean_idle && filled == 0 => return Ok(ReadStageOutcome::Closed),
            Ok(0) => {
                return Err(io_message(
                    "peer stream".to_string(),
                    format!(
                        "unexpected EOF after {filled} of {} frame bytes",
                        buffer.len()
                    ),
                ));
            }
            Ok(read_count) => filled = filled.saturating_add(read_count),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error)
                if allow_clean_idle
                    && filled == 0
                    && matches!(
                        error.kind(),
                        io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
                    ) =>
            {
                return Ok(ReadStageOutcome::Idle);
            }
            Err(error) => return Err(io_error("peer stream".to_string(), error)),
        }
    }

    Ok(ReadStageOutcome::Complete)
}

fn read_stage_for_peer<R: Read>(
    reader: &mut R,
    buffer: &mut [u8],
    allow_clean_idle: bool,
    peer: &str,
) -> Result<ReadStageOutcome, SyncRuntimeError> {
    match read_stage(reader, buffer, allow_clean_idle) {
        Err(SyncRuntimeError::Io { message, .. }) => Err(io_message(peer.to_string(), message)),
        result => result,
    }
}

fn io_error(peer: String, error: io::Error) -> SyncRuntimeError {
    io_message(peer, error.to_string())
}

fn io_message(peer: String, message: String) -> SyncRuntimeError {
    SyncRuntimeError::Io { peer, message }
}
