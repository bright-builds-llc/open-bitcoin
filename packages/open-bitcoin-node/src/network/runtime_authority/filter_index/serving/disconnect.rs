// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp

use super::*;
use crate::ManagedNetworkError;
use std::{sync::MutexGuard, task::Waker};

/// Only a real second authorization and raw disconnect can create this phase.
#[must_use = "release and finish the achieved disconnect"]
pub(super) struct PendingDisconnect<'guard, S, V: CoinsView> {
    handle: &'guard ManagedNetworkHandle<S, V>,
    network: MutexGuard<'guard, ManagedPeerNetwork<S, V>>,
    disconnected: Result<(), ManagedNetworkError>,
}

/// No authority reference, guard, pointer or guard-capturing closure survives release.
#[must_use = "finish the collected wake and error completion"]
pub(super) struct UnlockedDisconnect {
    disconnected: Result<(), ManagedNetworkError>,
    pruned: Result<(), ManagedNetworkAuthorityError>,
    wakes: Vec<Waker>,
}

impl<'guard, S: ChainstateStore, V: CoinsView> PendingDisconnect<'guard, S, V> {
    pub(super) fn begin(
        handle: &'guard ManagedNetworkHandle<S, V>,
        mut network: MutexGuard<'guard, ManagedPeerNetwork<S, V>>,
        token: &SyncPeerLifetime,
    ) -> Result<Self, ManagedNetworkAuthorityError> {
        let peer = handle.require_sync_peer_locked(&network, token)?;
        let disconnected = network.disconnect_peer(peer);
        Ok(Self {
            handle,
            network,
            disconnected,
        })
    }

    pub(super) fn release(self) -> UnlockedDisconnect {
        let Self {
            handle,
            network,
            disconnected,
        } = self;
        let pruned = handle.prune_sync_peers_locked(&network);
        let wakes = handle.collect_basic_filter_wakes(&network);
        drop(network);
        UnlockedDisconnect {
            disconnected,
            pruned,
            wakes,
        }
    }
}

impl UnlockedDisconnect {
    pub(super) fn finish(self) -> SyncContinuationFailure {
        super::super::readiness::wake_all(self.wakes);
        let primary = match self.disconnected {
            Ok(()) => ManagedNetworkAuthorityError::Operation(
                ManagedNetworkError::LifecycleEffect("invalid BASIC peer request"),
            ),
            Err(error) => error.into(),
        };
        let error = match self.pruned {
            Ok(()) => primary,
            Err(cleanup) => ManagedNetworkAuthorityError::LifecycleEffect(format!(
                "{primary}; sync registry cleanup also failed: {cleanup}"
            )),
        };
        SyncContinuationFailure::Operation(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::{DurableSyncRuntime, SyncNetwork, SyncRuntimeConfig};
    use crate::{FjallNodeStore, chainstate::BasicFilterStartupMode};
    use open_bitcoin_core::consensus::{CanonicalChain, ScriptVerifyFlags};
    use open_bitcoin_network::VersionMessage;
    use std::sync::Arc;

    #[test]
    fn phase167_peer_session_real_post_disconnect_prune_poison_is_unlocked_operation_error() {
        // Arrange: public provider, actual token and shared semantic preparation.
        let mut nonce = [0; 16];
        getrandom::fill(&mut nonce).expect("owned path");
        let path = std::fs::canonicalize(std::env::temp_dir())
            .expect("physical temp parent")
            .join(format!(
                "phase167-disconnect-prune-{:x}",
                u128::from_le_bytes(nonce)
            ));
        let opened = FjallNodeStore::open_canonical_genesis(&path, CanonicalChain::Regtest)
            .expect("proved genesis");
        let params = open_bitcoin_core::chainstate::engine::canonical_genesis::consensus_params(
            opened.genesis(),
        );
        let hash = opened.position().block_hash;
        let runtime = DurableSyncRuntime::open_canonical_genesis_configured(
            opened,
            SyncRuntimeConfig {
                network: SyncNetwork::Regtest,
                ..Default::default()
            },
            BasicFilterStartupMode::Enabled,
        )
        .expect("actual BASIC provider");
        let handle = runtime.network_handle();
        handle
            .configure_basic_peer_serving(true)
            .expect("explicit serving");
        let (token, outbound) = handle.connect_sync_peer(1, 0).expect("actual A factory");
        outbound.expect("version");
        for message in [
            WireNetworkMessage::Version(VersionMessage::default()),
            WireNetworkMessage::Verack,
        ] {
            handle
                .receive_sync_message(1, message, 0, ScriptVerifyFlags::P2SH, params)
                .expect("handshake");
        }
        let network = handle.lock_authority().expect("same actual guard");
        let peer = handle
            .require_sync_peer_locked(&network, &token)
            .expect("original token");
        let prepared = handle.prepare_basic_filter_response_locked(
            &network,
            peer,
            FilterRequest::Headers {
                filter_type: 1,
                start_height: 0,
                stop_hash: hash,
            },
            params,
            &mut BasicPeerServeBudget::default(),
            |_| {},
        );
        assert!(matches!(
            prepared,
            BasicFilterPrepareOutcome::InvalidDisconnect(_)
        ));
        // Act: genuine raw mutation precedes the real registry poison and prune error.
        let pending =
            PendingDisconnect::begin(&handle, network, &token).expect("real second authorization");
        let registry = Arc::clone(&handle.peer_sessions);
        assert!(
            std::thread::spawn(move || {
                let _guard = registry.lock().expect("actual registry lock");
                panic!("intentional post-disconnect registry poison");
            })
            .join()
            .is_err()
        );
        let unlocked = pending.release();
        // Assert BEFORE finish: real genesis has no pending wake, and actual locks are free.
        assert!(unlocked.wakes.is_empty());
        assert!(handle.basic_filter_read_locks_available_for_test());
        assert!(
            runtime
                .store()
                .basic_filter_publication_lock_available_for_test()
        );
        assert!(
            handle
                .peer_manager_snapshot()
                .expect("actual unlocked membership")
                .peer_state(1)
                .is_none()
        );
        let SyncContinuationFailure::Operation(error) = unlocked.finish() else {
            panic!("achieved disconnect must not become pre-effect denial");
        };
        assert!(error.to_string().contains("invalid BASIC peer request"));
        assert!(error.to_string().contains("RegistryUnavailable"));
        drop(handle);
        drop(runtime);
        std::fs::remove_dir_all(path).expect("owned cleanup");
    }
}
