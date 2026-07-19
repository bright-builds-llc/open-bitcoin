// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/context.h

use std::{
    fmt,
    sync::{Arc, Mutex},
};

use open_bitcoin_core::{
    chainstate::{AnchoredBlock, ChainPosition, ChainTransition, ChainstateSnapshot},
    consensus::{ConsensusParams, ScriptVerifyFlags},
    primitives::{Block, BlockHash},
};
use open_bitcoin_network::{HeaderEntry, PeerId, PeerManager, WireNetworkMessage};

use crate::{MemoryChainstateStore, status::BlockRelayEvidenceStatus, sync::SyncRuntimeError};

use super::{
    BlockConnectDisposition, BlockRelayRuntimeEvidenceSnapshot, ManagedNetworkError,
    ManagedNetworkInfo, ManagedPeerNetwork, ManagedSyncMessageResult,
};

type AuthoritativeNetwork = ManagedPeerNetwork<MemoryChainstateStore>;

#[derive(Debug)]
pub enum ManagedNetworkAuthorityError {
    Poisoned,
    Operation(ManagedNetworkError),
}

impl fmt::Display for ManagedNetworkAuthorityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Poisoned => formatter.write_str("authoritative network state is unavailable"),
            Self::Operation(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ManagedNetworkAuthorityError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Poisoned => None,
            Self::Operation(error) => Some(error),
        }
    }
}

impl From<ManagedNetworkError> for ManagedNetworkAuthorityError {
    fn from(value: ManagedNetworkError) -> Self {
        Self::Operation(value)
    }
}

impl From<ManagedNetworkAuthorityError> for SyncRuntimeError {
    fn from(value: ManagedNetworkAuthorityError) -> Self {
        match value {
            ManagedNetworkAuthorityError::Poisoned => Self::Network {
                message: "authoritative network state is unavailable".to_string(),
            },
            ManagedNetworkAuthorityError::Operation(error) => Self::from(error),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ManagedNetworkHandle {
    authority: Arc<Mutex<AuthoritativeNetwork>>,
}

impl ManagedNetworkHandle {
    pub(crate) fn new(network: AuthoritativeNetwork) -> Self {
        Self {
            authority: Arc::new(Mutex::new(network)),
        }
    }

    fn read<T>(
        &self,
        snapshot: impl FnOnce(&AuthoritativeNetwork) -> T,
    ) -> Result<T, ManagedNetworkAuthorityError> {
        let network = self
            .authority
            .lock()
            .map_err(|_| ManagedNetworkAuthorityError::Poisoned)?;
        Ok(snapshot(&network))
    }

    fn mutate<T>(
        &self,
        command: impl FnOnce(&mut AuthoritativeNetwork) -> T,
    ) -> Result<T, ManagedNetworkAuthorityError> {
        let mut network = self
            .authority
            .lock()
            .map_err(|_| ManagedNetworkAuthorityError::Poisoned)?;
        Ok(command(&mut network))
    }

    fn try_mutate<T>(
        &self,
        command: impl FnOnce(&mut AuthoritativeNetwork) -> Result<T, ManagedNetworkError>,
    ) -> Result<T, ManagedNetworkAuthorityError> {
        self.mutate(command)?.map_err(Into::into)
    }

    pub fn chainstate_snapshot(&self) -> Result<ChainstateSnapshot, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::chainstate_snapshot)
    }

    pub fn maybe_chain_tip(&self) -> Result<Option<ChainPosition>, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::maybe_chain_tip)
    }

    pub fn header_entries(&self) -> Result<Vec<HeaderEntry>, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::header_entries)
    }

    pub fn best_chain_entries(&self) -> Result<Vec<HeaderEntry>, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::best_chain_entries)
    }

    pub fn peer_manager_snapshot(&self) -> Result<PeerManager, ManagedNetworkAuthorityError> {
        self.read(|network| network.peer_manager().clone())
    }

    pub fn network_info(&self) -> Result<ManagedNetworkInfo, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::network_info)
    }

    pub fn connect_outbound_peer(
        &self,
        peer_id: PeerId,
        timestamp: i64,
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| network.connect_outbound_peer(peer_id, timestamp))
    }

    pub fn receive_sync_message(
        &self,
        peer_id: PeerId,
        message: WireNetworkMessage,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ManagedSyncMessageResult, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| {
            network.receive_sync_message(
                peer_id,
                message,
                timestamp,
                verify_flags,
                consensus_params,
            )
        })
    }

    pub fn expire_compact_download_timeouts(
        &self,
        timestamp: i64,
    ) -> Result<Vec<(PeerId, WireNetworkMessage)>, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| network.expire_compact_download_timeouts(timestamp))
    }

    pub fn peer_requested_blocks(
        &self,
        peer_id: PeerId,
    ) -> Result<Vec<BlockHash>, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| network.peer_requested_blocks(peer_id))
    }

    pub fn request_missing_blocks(
        &self,
        peer_id: PeerId,
        block_hashes: &[BlockHash],
    ) -> Result<Vec<WireNetworkMessage>, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| network.request_missing_blocks(peer_id, block_hashes))
    }

    pub fn disconnect_peer(&self, peer_id: PeerId) -> Result<(), ManagedNetworkAuthorityError> {
        self.try_mutate(|network| network.disconnect_peer(peer_id))
    }

    pub fn acknowledge_wire_message_written(
        &self,
        message: &WireNetworkMessage,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| network.acknowledge_wire_message_written(message))
    }

    pub fn connect_stored_block(
        &self,
        block: &Block,
        chain_work: u128,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<BlockConnectDisposition, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| {
            network.connect_stored_block(
                block,
                chain_work,
                timestamp,
                verify_flags,
                consensus_params,
            )
        })
    }

    pub fn reorg_to_branch(
        &self,
        disconnect_blocks: &[Block],
        replacement_branch: &[AnchoredBlock],
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ChainTransition, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| {
            network.reorg_to_branch(
                disconnect_blocks,
                replacement_branch,
                verify_flags,
                consensus_params,
            )
        })
    }

    pub fn note_local_block_hash(
        &self,
        block_hash: BlockHash,
    ) -> Result<(), ManagedNetworkAuthorityError> {
        self.mutate(|network| network.note_local_block_hash(block_hash))
    }

    pub(crate) fn block_relay_runtime_evidence_snapshot(
        &self,
    ) -> Result<BlockRelayRuntimeEvidenceSnapshot, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::block_relay_runtime_evidence_snapshot)
    }

    pub fn block_relay_evidence_status(
        &self,
    ) -> Result<BlockRelayEvidenceStatus, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::block_relay_evidence_status)
    }

    pub fn block_served_write_count(&self) -> Result<u64, ManagedNetworkAuthorityError> {
        self.read(ManagedPeerNetwork::block_served_write_count)
    }

    pub fn connect_local_block(
        &self,
        block: &Block,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ChainPosition, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| {
            network.connect_local_block(block, verify_flags, consensus_params)
        })
    }

    pub fn announce_block(
        &self,
        peer_id: PeerId,
        block: &Block,
    ) -> Result<Option<WireNetworkMessage>, ManagedNetworkAuthorityError> {
        self.try_mutate(|network| network.announce_block(peer_id, block))
    }

    #[cfg(test)]
    fn poison_for_test(&self) {
        let authority = Arc::clone(&self.authority);
        let result = std::thread::spawn(move || {
            let _network = authority.lock().expect("test authority should lock");
            panic!("poison authoritative network for test");
        })
        .join();
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod tests {
    use open_bitcoin_mempool::PolicyConfig;
    use open_bitcoin_network::LocalPeerConfig;

    use crate::{ManagedPeerNetwork, MemoryChainstateStore};

    use super::{ManagedNetworkAuthorityError, ManagedNetworkHandle};

    fn test_handle() -> ManagedNetworkHandle {
        let network = ManagedPeerNetwork::new(
            MemoryChainstateStore::default(),
            LocalPeerConfig::default(),
            PolicyConfig::default(),
        );
        ManagedNetworkHandle::new(network)
    }

    #[test]
    fn cloned_handles_share_mutations() {
        // Arrange
        let mutating_handle = test_handle();
        let snapshot_handle = mutating_handle.clone();

        // Act
        mutating_handle
            .connect_outbound_peer(1, 1_777_225_210)
            .expect("shared authority should accept the peer");
        let snapshot = snapshot_handle
            .network_info()
            .expect("shared authority should return an owned snapshot");

        // Assert
        assert_eq!(snapshot.outbound_peers, 1);
    }

    #[test]
    fn owned_snapshot_survives_authority_drop() {
        // Arrange
        let handle = test_handle();

        // Act
        let snapshot = handle
            .chainstate_snapshot()
            .expect("shared authority should return an owned snapshot");
        drop(handle);

        // Assert
        assert!(snapshot.active_chain.is_empty());
    }

    #[test]
    fn poisoned_authority_returns_typed_error() {
        // Arrange
        let handle = test_handle();
        handle.poison_for_test();

        // Act
        let result = handle.network_info();

        // Assert
        assert!(matches!(
            result,
            Err(ManagedNetworkAuthorityError::Poisoned)
        ));
    }
}
