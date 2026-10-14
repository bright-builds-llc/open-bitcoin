// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

pub(super) fn arm_compact_download(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    block: &open_bitcoin_core::primitives::Block,
) {
    let payload = open_bitcoin_codec::CompactBlockPayload {
        header: block.header.clone(),
        nonce: 0,
        short_ids: vec![],
        prefilled_transactions: vec![open_bitcoin_codec::PrefilledTransaction {
            index_delta: 0,
            transaction: block.transactions[0].clone(),
        }],
    };
    let _fixture_actions = network
        .peer_manager
        .handle_compact_block_download(
            1,
            payload,
            open_bitcoin_network::CompactBlockReceiveFacts {
                candidates: &[],
                extra: &[],
            },
            0,
        )
        .expect("fixture compact owner");
    network
        .peer_manager
        .compact_download_peer_state_mut(1)
        .expect("fixture slot map")
        .in_flight
        .insert(
            block_hash(&block.header),
            open_bitcoin_network::CompactDownloadInFlight {
                partial: Default::default(),
                getblocktxn_in_flight: true,
                requested_indexes: vec![1],
                started_at_unix: 0,
            },
        );
}

pub(super) struct DurableFixture {
    dir: std::path::PathBuf,
    pub(super) store: crate::FjallNodeStore,
}

impl DurableFixture {
    pub(super) fn new() -> Self {
        let dir = std::env::temp_dir().join(format!(
            "phase164-admission-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        let store = crate::FjallNodeStore::open(&dir).expect("owned raw store");
        Self { dir, store }
    }

    pub(super) fn bytes(&self) -> Vec<(String, Vec<u8>, Vec<u8>)> {
        use crate::StorageNamespace;
        let mut rows = Vec::new();
        for namespace in [
            StorageNamespace::Headers,
            StorageNamespace::BlockIndex,
            StorageNamespace::Chainstate,
            StorageNamespace::Coins,
            StorageNamespace::Wallet,
            StorageNamespace::Metrics,
            StorageNamespace::Mempool,
            StorageNamespace::Runtime,
            StorageNamespace::Schema,
        ] {
            // These namespaces already exist; this test reads exact application bytes.
            let keyspace = self
                .store
                .database()
                .keyspace(namespace.as_str(), fjall::KeyspaceCreateOptions::default)
                .expect("existing namespace");
            for entry in keyspace.iter() {
                let (key, value) = entry.into_inner().expect("row");
                rows.push((namespace.as_str().to_owned(), key.to_vec(), value.to_vec()));
            }
        }
        rows
    }

    pub(super) fn network(
        &self,
    ) -> ManagedPeerNetwork<crate::FjallChainstateStore, crate::FjallCoinsView> {
        use open_bitcoin_core::chainstate::{Chainstate, CoinsCache, FlushPolicyTime};
        let state = Chainstate::from_coins_cache(
            CoinsCache::from_parent(self.store.coins_view()),
            Vec::new(),
            Default::default(),
            None,
        );
        let manager = crate::ManagedChainstate::from_recovered_chainstate(
            crate::FjallChainstateStore::from_store(self.store.clone()),
            state,
            crate::chainstate::FlushLifecycle::ready(
                FlushPolicyTime::from_unix_seconds(0),
                FlushPolicyTime::from_unix_seconds(0),
                0,
                false,
            ),
        )
        .expect("owned recovered fixture");
        ManagedPeerNetwork::from_initialized_chainstate(
            manager,
            Default::default(),
            Default::default(),
            16,
            Default::default(),
            Default::default(),
            false,
        )
    }

    pub(super) fn cleanup(self) {
        let Self { dir, store } = self;
        drop(store);
        std::fs::remove_dir_all(dir).expect("owned fixture cleanup");
    }
}
