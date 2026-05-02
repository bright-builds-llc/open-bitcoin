// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/netbase.cpp
// - packages/bitcoin-knots/src/netbase.h
// - packages/bitcoin-knots/src/protocol.h

use std::net::ToSocketAddrs;

use super::{ResolvedSyncPeerAddress, SyncPeerAddress, SyncRuntimeConfig, SyncRuntimeError};

pub trait SyncPeerResolver {
    fn resolve(
        &mut self,
        peer: &SyncPeerAddress,
        config: &SyncRuntimeConfig,
    ) -> Result<Vec<ResolvedSyncPeerAddress>, SyncRuntimeError>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemSyncPeerResolver;

impl SyncPeerResolver for SystemSyncPeerResolver {
    fn resolve(
        &mut self,
        peer: &SyncPeerAddress,
        _config: &SyncRuntimeConfig,
    ) -> Result<Vec<ResolvedSyncPeerAddress>, SyncRuntimeError> {
        let addresses = (peer.host.as_str(), peer.port)
            .to_socket_addrs()
            .map_err(|error| SyncRuntimeError::AddressResolution {
                peer: peer.label(),
                message: error.to_string(),
            })?;
        let resolved = addresses
            .map(|endpoint| ResolvedSyncPeerAddress::new(peer.clone(), endpoint))
            .collect::<Vec<_>>();
        if resolved.is_empty() {
            return Err(SyncRuntimeError::AddressResolution {
                peer: peer.label(),
                message: "no socket addresses returned".to_string(),
            });
        }
        Ok(resolved)
    }
}
