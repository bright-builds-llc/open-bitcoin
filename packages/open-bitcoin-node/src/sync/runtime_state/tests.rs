// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/headerssync.cpp

use super::*;
use crate::chainstate::validation_history::tests::block;
use crate::storage::fjall_store::validation_history::HistoryPublicationFault;
use crate::storage::validation_history::ValidationProvenance;
use open_bitcoin_core::consensus::{ConsensusParams, ScriptVerifyFlags, block_hash};
use open_bitcoin_network::{HeadersMessage, WireNetworkMessage};

fn runtime(name: &str) -> (std::path::PathBuf, DurableSyncRuntime) {
    let dir = std::env::temp_dir().join(format!(
        "phase159-headers-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ));
    let store = crate::FjallNodeStore::open(&dir).expect("fresh store");
    let runtime = DurableSyncRuntime::open(
        store,
        super::super::SyncRuntimeConfig {
            network: super::super::SyncNetwork::Regtest,
            ..Default::default()
        },
    )
    .expect("empty non-index runtime");
    runtime
        .network_handle()
        .connect_outbound_peer(1, 10_000)
        .expect("peer");
    (dir, runtime)
}

fn headers(
    runtime: &DurableSyncRuntime,
    headers: Vec<open_bitcoin_core::primitives::BlockHeader>,
) -> Result<crate::network::ManagedSyncMessageResult, crate::ManagedNetworkAuthorityError> {
    runtime.network_handle().receive_sync_message(
        1,
        WireNetworkMessage::Headers(HeadersMessage { headers }),
        1_000_000,
        ScriptVerifyFlags::P2SH,
        ConsensusParams {
            no_pow_retargeting: true,
            ..Default::default()
        },
    )
}

#[test]
fn phase159_validation_history_store_actual_persist_progress_preserves_trusted_header_coverage_after_reopen()
 {
    // Arrange
    let (dir, runtime) = runtime("progress");
    let genesis = block(BlockHash::default(), 0, 0x51);
    let hash = block_hash(&genesis.header);
    headers(&runtime, vec![genesis.header]).expect("genuine header validation and insertion");
    // Act
    runtime
        .persist_progress()
        .expect("actual runtime progress persistence");
    // Assert
    assert_eq!(
        runtime
            .store()
            .validation_provenance(hash)
            .expect("header only"),
        ValidationProvenance::NeverConnected
    );
    assert!(
        runtime
            .store()
            .recovered_validation_history()
            .expect("coverage")
            .is_complete()
            .expect("complete")
    );
    assert!(
        runtime
            .store()
            .load_header_store()
            .expect("durable headers")
            .expect("snapshot")
            .entry(&hash)
            .is_some()
    );
    drop(runtime);
    let reopened = crate::FjallNodeStore::open(&dir).expect("exclusive reopen");
    assert_eq!(
        reopened
            .validation_provenance(hash)
            .expect("still header only"),
        ValidationProvenance::NeverConnected
    );
    assert!(
        reopened
            .recovered_validation_history()
            .expect("recovered")
            .is_complete()
            .expect("complete")
    );
    drop(reopened);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase159_validation_history_store_2000_admitted_headers_are_not_restricted_by_acceptance_batch_cap()
 {
    // Arrange
    let (dir, runtime) = runtime("full-message");
    let mut parent = BlockHash::default();
    let mut admitted = Vec::new();
    for height in 0..2_000 {
        let next = block(parent, height, 0x51);
        parent = block_hash(&next.header);
        admitted.push(next.header);
    }
    // Act
    headers(&runtime, admitted).expect("entire normal header message");
    runtime.persist_progress().expect("full trusted snapshot");
    // Assert
    assert_eq!(
        runtime
            .network_handle()
            .header_entries()
            .expect("headers")
            .len(),
        2_000
    );
    assert_eq!(
        runtime
            .store()
            .validation_provenance(parent)
            .expect("last header"),
        ValidationProvenance::NeverConnected
    );
    assert!(
        runtime
            .store()
            .recovered_validation_history()
            .expect("recovered")
            .is_complete()
            .expect("complete")
    );
    drop(runtime);
    std::fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn phase159_validation_history_store_header_publication_fault_stays_closed() {
    // Arrange
    let (dir, runtime) = runtime("fault");
    let genesis = block(BlockHash::default(), 0, 0x51);
    runtime
        .store()
        .set_validation_history_fault(HistoryPublicationFault::BeforeCommit);
    // Act
    assert!(headers(&runtime, vec![genesis.header]).is_err());
    // Assert
    assert!(runtime.persist_progress().is_err());
    assert!(
        runtime
            .store()
            .load_header_store()
            .expect("no published snapshot")
            .is_none()
    );
    drop(runtime);
    std::fs::remove_dir_all(dir).expect("cleanup");
}
