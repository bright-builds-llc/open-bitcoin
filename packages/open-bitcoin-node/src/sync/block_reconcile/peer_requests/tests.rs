// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_blockfilters.py

use super::*;

#[cfg(test)]
struct UnboundRequests<'a> {
    handle: &'a crate::DurableNetworkHandle,
    peer: PeerId,
}

#[cfg(test)]
impl RequestOwner for UnboundRequests<'_> {
    fn peer(&self) -> PeerId {
        self.peer
    }
    fn requested(&mut self) -> Result<Vec<BlockHash>, SyncRuntimeError> {
        self.handle
            .peer_requested_blocks(self.peer)
            .map_err(Into::into)
    }
    fn request(
        &mut self,
        hashes: &[BlockHash],
    ) -> Result<Vec<WireNetworkMessage>, SyncRuntimeError> {
        self.handle
            .request_missing_blocks(self.peer, hashes)
            .map_err(Into::into)
    }
}

#[cfg(test)]
pub(in crate::sync) fn request_missing_blocks(
    runtime: &mut DurableSyncRuntime,
    peer_id: PeerId,
) -> Result<Vec<WireNetworkMessage>, SyncRuntimeError> {
    if runtime
        .config
        .max_blocks_in_flight_total
        .saturating_sub(runtime.inflight_blocks.len())
        == 0
    {
        return Ok(Vec::new());
    }
    let requested = select_missing(runtime, |hash| {
        runtime
            .network
            .note_local_block_hash(hash)
            .map_err(Into::into)
    })?;
    request_tracked_blocks(runtime, peer_id, &requested)
}

#[cfg(test)]
pub(in crate::sync) fn request_tracked_blocks(
    runtime: &mut DurableSyncRuntime,
    peer_id: PeerId,
    hashes: &[BlockHash],
) -> Result<Vec<WireNetworkMessage>, SyncRuntimeError> {
    let network = runtime.network.clone();
    let mut owner = UnboundRequests {
        handle: &network,
        peer: peer_id,
    };
    request_core(
        &mut owner,
        &mut runtime.inflight_blocks,
        runtime.config.max_blocks_in_flight_total,
        hashes,
    )
}

#[cfg(test)]
pub(in crate::sync) fn handoff_requested_messages(
    runtime: &mut DurableSyncRuntime,
    peer_id: PeerId,
    targeted: Vec<(PeerId, WireNetworkMessage)>,
) -> Result<Vec<WireNetworkMessage>, SyncRuntimeError> {
    let network = runtime.network.clone();
    let mut owner = UnboundRequests {
        handle: &network,
        peer: peer_id,
    };
    handoff_core(
        &mut owner,
        &mut runtime.inflight_blocks,
        runtime.config.max_blocks_in_flight_total,
        targeted,
    )
}

use crate::sync::{ResolvedSyncPeerAddress, SyncNetwork, SyncPeerAddress, SyncRuntimeConfig};
use crate::{FjallNodeStore, chainstate::BasicFilterStartupMode};
use open_bitcoin_core::{consensus::CanonicalChain, primitives::InventoryVector};
use open_bitcoin_network::{GetCFHeaders, InventoryList, VersionMessage};
use std::path::PathBuf;

fn fixture() -> (PathBuf, DurableSyncRuntime, BlockHash) {
    let mut nonce = [0; 16];
    getrandom::fill(&mut nonce).expect("owned path");
    let path = std::fs::canonicalize(std::env::temp_dir())
        .expect("physical temp parent")
        .join(format!(
            "phase167-denied-consumer-{:x}",
            u128::from_le_bytes(nonce)
        ));
    let opened = FjallNodeStore::open_canonical_genesis(&path, CanonicalChain::Regtest)
        .expect("actual proved genesis");
    let hash = opened.position().block_hash;
    let runtime = DurableSyncRuntime::open_canonical_genesis_configured(
        opened,
        SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            max_blocks_in_flight_total: 8,
            ..Default::default()
        },
        BasicFilterStartupMode::Enabled,
    )
    .expect("actual BASIC provider");
    runtime
        .network
        .configure_basic_peer_serving(true)
        .expect("explicit serving");
    (path, runtime, hash)
}

fn peer(runtime: &DurableSyncRuntime, id: PeerId) -> PeerRequests {
    let (token, outbound) = runtime
        .network
        .connect_sync_peer(id, 0)
        .expect("actual factory");
    outbound.expect("actual version");
    for message in [
        WireNetworkMessage::Version(VersionMessage::default()),
        WireNetworkMessage::Verack,
    ] {
        runtime
            .network
            .receive_sync_message(
                id,
                message,
                0,
                runtime.verify_flags,
                runtime.consensus_params,
            )
            .expect("actual handshake");
    }
    PeerRequests::new(token)
}

fn handoff(
    runtime: &mut DurableSyncRuntime,
    id: PeerId,
    hashes: &[BlockHash],
) -> Vec<WireNetworkMessage> {
    let outbound = runtime
        .network
        .request_missing_blocks(id, hashes)
        .expect("actual manager request");
    assert!(!outbound.is_empty());
    handoff_requested_messages(
        runtime,
        id,
        outbound.into_iter().map(|message| (id, message)).collect(),
    )
    .expect("real shared production handoff, not inserted slots")
}

fn earned_pair(runtime: &mut DurableSyncRuntime) -> (PeerRequests, PeerRequests, [BlockHash; 3]) {
    let mut original = peer(runtime, 1);
    let other = peer(runtime, 2);
    let h1 = BlockHash::from_byte_array([31; 32]);
    let h2 = BlockHash::from_byte_array([32; 32]);
    let h3 = BlockHash::from_byte_array([33; 32]);
    let outbound = handoff(runtime, 1, &[h1]);
    assert!(
        original
            .renew(runtime, &outbound)
            .expect("earned predecessor")
    );
    handoff(runtime, 2, &[h1, h3]);
    runtime
        .network
        .receive_sync_message(
            1,
            WireNetworkMessage::NotFound(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::Block,
                object_hash: h1.into(),
            }])),
            1,
            runtime.verify_flags,
            runtime.consensus_params,
        )
        .expect("actual H1 cancel without consumer cleanup");
    handoff(runtime, 1, &[h2]);
    assert_eq!(original.retained, vec![h1]);
    assert_eq!(
        runtime
            .network
            .peer_requested_blocks(1)
            .expect("actual latest"),
        vec![h2]
    );
    assert_eq!(runtime.inflight_blocks, [h1, h2, h3].into_iter().collect());
    (original, other, [h1, h2, h3])
}

fn basic_intent(hash: BlockHash) -> WireNetworkMessage {
    WireNetworkMessage::GetCFHeaders(GetCFHeaders {
        filter_type: 0,
        start_height: 0,
        stop_hash: hash,
    })
}

fn assert_denied_conservation(
    requests: &mut PeerRequests,
    runtime: &mut DurableSyncRuntime,
    h1: BlockHash,
    h2: BlockHash,
) {
    let before = runtime.inflight_blocks.clone();
    assert_eq!(requests.retained, vec![h1]);
    assert_eq!(requests.maybe_denied_requests.as_ref(), Some(&vec![h2]));
    assert!(requests.renew(runtime, &[]).is_err());
    assert!(requests.request_missing(runtime).is_err());
    assert!(requests.expire(runtime, 9).is_err());
    assert!(requests.handoff(runtime, Vec::new()).is_err());
    assert!(requests.block_was_requested(runtime, h2).is_err());
    let peer = ResolvedSyncPeerAddress::new(
        SyncPeerAddress::manual("127.0.0.1", 18444),
        "127.0.0.1:18444".parse().expect("literal endpoint"),
    );
    let mut progress = PeerProgress::new(peer, SyncNetwork::Regtest, 1);
    assert!(
        requests
            .dispatch(
                runtime,
                WireNetworkMessage::Ping { nonce: 1 },
                9,
                &[],
                &mut progress
            )
            .is_err()
    );
    assert_eq!(runtime.inflight_blocks, before);
    assert_eq!(requests.retained, vec![h1]);
    assert_eq!(requests.maybe_denied_requests.as_ref(), Some(&vec![h2]));
}

#[test]
fn phase167_peer_session_genuine_denied_consumer_preserves_two_sets_until_actual_absence() {
    // Arrange: distinct earned predecessor/raw sets and a real shared/different owner C.
    let (path, mut runtime, genesis) = fixture();
    let (mut original, other, [h1, h2, h3]) = earned_pair(&mut runtime);
    let handle = runtime.network.clone();
    let continuation = handle
        .begin_sync_peer_dispatch(
            &original.token,
            basic_intent(genesis),
            2,
            runtime.verify_flags,
            runtime.consensus_params,
        )
        .expect("actual raw FilterRequestIntent");
    handle.clone().disconnect_peer(1).expect("actual A removal");
    let mut replacement = peer(&runtime, 1);
    let h4 = BlockHash::from_byte_array([34; 32]);
    handoff(&mut runtime, 1, &[h4]);
    let other_before = handle
        .peer_requested_blocks(2)
        .expect("C exact ownership/order");
    let replacement_before = format!("{:?}", handle.peer_manager_snapshot().expect("real B"));
    let slots = runtime.inflight_blocks.clone();
    // Act: resolve only the captured origin, then use the actual private consumer.
    let outcome = continuation.resolve();
    let primary = original
        .settle_dispatch(&mut runtime, outcome, &[])
        .expect_err("genuine Replaced denial");
    // Assert: denial itself never grants either cleanup.
    assert!(primary.to_string().contains("Replaced"));
    assert_denied_conservation(&mut original, &mut runtime, h1, h2);
    let cleanup = original.finish(&mut runtime);
    let joined: Result<(), SyncRuntimeError> = request_cleanup_result(Err(primary), cleanup);
    assert!(
        joined
            .expect_err("real first denial plus real replacement finish error")
            .to_string()
            .contains("Replaced")
    );
    assert_eq!(runtime.inflight_blocks, slots);
    assert_eq!(
        format!("{:?}", handle.peer_manager_snapshot().expect("B unchanged")),
        replacement_before
    );
    let (foreign_path, mut foreign, _) = fixture();
    let foreign_before = format!(
        "{:?}",
        foreign
            .network
            .peer_manager_snapshot()
            .expect("foreign original state")
    );
    assert!(
        original
            .finish(&mut foreign)
            .expect_err("foreign runtime owns no original cleanup")
            .to_string()
            .contains("Foreign")
    );
    assert!(foreign.inflight_blocks.is_empty());
    assert_eq!(
        format!(
            "{:?}",
            foreign
                .network
                .peer_manager_snapshot()
                .expect("foreign unchanged")
        ),
        foreign_before
    );
    assert_eq!(original.retained, vec![h1]);
    assert_eq!(original.maybe_denied_requests.as_ref(), Some(&vec![h2]));
    assert_eq!(runtime.inflight_blocks, slots);
    // B's own normal finish retires its H4; A's two candidates still remain untouched.
    replacement
        .finish(&mut runtime)
        .expect("actual B normal owner cleanup and removal");
    assert_eq!(runtime.inflight_blocks, [h1, h2, h3].into_iter().collect());
    let absent_before = format!(
        "{:?}",
        handle.peer_manager_snapshot().expect("A/B already absent")
    );
    original
        .finish(&mut runtime)
        .expect("fresh no-capture original absence reconciles both sets");
    assert!(original.retained.is_empty());
    assert!(original.maybe_denied_requests.is_none());
    assert_eq!(runtime.inflight_blocks, [h1, h3].into_iter().collect());
    assert_eq!(
        handle
            .peer_requested_blocks(2)
            .expect("C still owns exact shared/different hashes"),
        other_before
    );
    assert_eq!(
        format!(
            "{:?}",
            handle
                .peer_manager_snapshot()
                .expect("no second disconnect")
        ),
        absent_before
    );
    drop(foreign);
    std::fs::remove_dir_all(foreign_path).expect("foreign owned cleanup");
    drop(handle);
    drop(other);
    drop(runtime);
    std::fs::remove_dir_all(path).expect("owned cleanup");
}

#[test]
fn phase167_peer_session_bound_real_request_scope_poison_retains_both_consumer_sets() {
    // Arrange
    let (path, mut runtime, genesis) = fixture();
    let (mut original, other, [h1, h2, _h3]) = earned_pair(&mut runtime);
    let handle = runtime.network.clone();
    let continuation = handle
        .begin_sync_peer_dispatch(
            &original.token,
            basic_intent(genesis),
            2,
            runtime.verify_flags,
            runtime.consensus_params,
        )
        .expect("real raw BASIC intent");
    let slots = runtime.inflight_blocks.clone();
    // Act: existing real public request scope poisons the actual locks; no recovery/test bridge.
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _result = handle.with_sync_peer_requests(&original.token, |_owner| {
                panic!("actual request-scope poison")
            });
        }))
        .is_err()
    );
    let outcome = continuation.resolve();
    let primary = original
        .settle_dispatch(&mut runtime, outcome, &[])
        .expect_err("genuine bound denial");
    assert_denied_conservation(&mut original, &mut runtime, h1, h2);
    let cleanup = original.finish(&mut runtime);
    let joined: Result<(), SyncRuntimeError> = request_cleanup_result(Err(primary), cleanup);
    assert!(
        joined
            .expect_err("both genuine failures visible")
            .to_string()
            .contains("unavailable")
    );
    assert_eq!(runtime.inflight_blocks, slots);
    assert_eq!(original.retained, vec![h1]);
    assert_eq!(original.maybe_denied_requests.as_ref(), Some(&vec![h2]));
    drop(handle);
    drop(other);
    drop(runtime);
    std::fs::remove_dir_all(path).expect("owned cleanup after genuine poison");
}
