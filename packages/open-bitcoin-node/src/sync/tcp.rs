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
    ResolvedSyncPeerAddress, SyncPeerSession, SyncRuntimeConfig, SyncRuntimeError, SyncTransport,
};

const WIRE_HEADER_LEN: usize = 24;

#[derive(Debug, Clone, Copy, Default)]
pub struct TcpPeerTransport;

pub struct TcpPeerSession {
    peer: String,
    stream: TcpStream,
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

    fn receive(
        &mut self,
        magic: NetworkMagic,
    ) -> Result<Option<WireNetworkMessage>, SyncRuntimeError> {
        let Some(header) = read_message_header(&mut self.stream, &self.peer, magic)? else {
            return Ok(None);
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
        let Some(()) = read_exact_or_stall(&mut self.stream, &mut payload, &self.peer)? else {
            return Ok(None);
        };
        let mut wire = Vec::with_capacity(WIRE_HEADER_LEN + payload.len());
        wire.extend_from_slice(&open_bitcoin_core::codec::encode_message_header(&header));
        wire.extend_from_slice(&payload);
        Ok(Some(ParsedNetworkMessage::decode_wire(&wire)?.message))
    }
}

fn read_message_header(
    stream: &mut TcpStream,
    peer: &str,
    expected_magic: NetworkMagic,
) -> Result<Option<MessageHeader>, SyncRuntimeError> {
    let mut header_bytes = [0_u8; WIRE_HEADER_LEN];
    let Some(()) = read_exact_or_stall(stream, &mut header_bytes, peer)? else {
        return Ok(None);
    };
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

    Ok(Some(header))
}

fn read_exact_or_stall(
    stream: &mut TcpStream,
    buffer: &mut [u8],
    peer: &str,
) -> Result<Option<()>, SyncRuntimeError> {
    match stream.read_exact(buffer) {
        Ok(()) => Ok(Some(())),
        Err(error)
            if matches!(
                error.kind(),
                io::ErrorKind::UnexpectedEof | io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
            ) =>
        {
            Ok(None)
        }
        Err(error) => Err(io_error(peer.to_string(), error)),
    }
}

fn io_error(peer: String, error: io::Error) -> SyncRuntimeError {
    SyncRuntimeError::Io {
        peer,
        message: error.to_string(),
    }
}
