// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use open_bitcoin_network::WireNetworkMessage;

fn context_payload() -> open_bitcoin_codec::CompactBlockPayload {
    let parent = CanonicalChain::Regtest.spec().genesis_header();
    let mut header = parent.clone();
    header.previous_block_hash = block_hash(&parent);
    header.time += 1;
    open_bitcoin_codec::CompactBlockPayload {
        header,
        nonce: 0,
        short_ids: vec![open_bitcoin_codec::ShortId::from_wire_bytes([1; 6])],
        prefilled_transactions: vec![open_bitcoin_codec::PrefilledTransaction {
            index_delta: 0,
            transaction: CanonicalChain::Regtest.spec().genesis_block().transactions[0].clone(),
        }],
    }
}

fn ready_compact_peer<S: crate::ChainstateStore, V: open_bitcoin_core::chainstate::CoinsView>(
    network: &mut ManagedPeerNetwork<S, V>,
) {
    network.peer_manager.set_block_relay_activation_policy(
        open_bitcoin_network::BlockRelayActivationPolicy {
            compact_relay: open_bitcoin_network::CompactRelayActivationConfig { enabled: true },
            ..Default::default()
        },
    );
    network
        .peer_manager
        .add_outbound_peer(1, 0)
        .expect("fixture peer");
    for message in [
        WireNetworkMessage::Version(open_bitcoin_network::VersionMessage::default()),
        WireNetworkMessage::Verack,
        WireNetworkMessage::SendCompact(open_bitcoin_codec::SendCompactMessage {
            announce: true,
            version: 2,
        }),
    ] {
        network
            .peer_manager
            .handle_message(1, message, 0)
            .expect("ready peer");
    }
}

#[test]
fn phase167_compact_provenance_context_node_missing_fixture_selection_refuses() {
    // Arrange
    let mut network = fixture();
    ready_compact_peer(&mut network);
    let before = snapshot(&network);
    // Act
    let result = network.handle_compact_block_receive(1, context_payload(), 0);
    // Assert
    assert!(result.is_err());
    assert_eq!(snapshot(&network), before);
}

#[test]
fn phase167_compact_provenance_context_node_every_canonical_binding_blocks_fixture_route() {
    // Arrange
    let raw = DurableFixture::new();
    let mut outcomes = Vec::new();
    let mut selected_controls = Vec::new();
    for mask in 0..8 {
        let directory = raw.dir.with_extension(format!("canonical-compact-{mask}"));
        {
            let (store, recovered) =
                crate::FjallNodeStore::open_canonical_genesis(&directory, CanonicalChain::Regtest)
                    .expect("proved storage")
                    .into_parts()
                    .expect("exclusive owner");
            let (managed, proof) = crate::ManagedChainstate::from_canonical_genesis(
                store,
                recovered,
                crate::chainstate::BasicFilterStartupMode::Disabled,
            )
            .expect("proved core owner");
            let params =
                open_bitcoin_core::chainstate::engine::canonical_genesis::consensus_params(&proof);
            let mut network = ManagedPeerNetwork::from_canonical_genesis_initialized(
                managed,
                open_bitcoin_network::LocalPeerConfig {
                    magic: open_bitcoin_core::primitives::NetworkMagic::from_bytes([
                        0xfa, 0xbf, 0xb5, 0xda,
                    ]),
                    ..Default::default()
                },
                16,
                &proof,
            )
            .expect("bound network");
            ready_compact_peer(&mut network);
            if mask & 1 == 0 {
                network.admission_scope = super::super::AdmissionScope::Fixture;
            }
            if mask & 2 == 0 {
                network.chainstate = raw.network().chainstate;
            }
            if mask & 4 == 0 {
                network
                    .peer_manager
                    .seed_header_store(HeaderStore::default());
            }
            network.maybe_header_consensus_params = Some(params);
            let before = snapshot(&network);
            // Act
            let result = network.handle_compact_block_receive(1, context_payload(), 0);
            outcomes.push((
                mask,
                result.is_err(),
                snapshot(&network) == before,
                network
                    .peer_manager
                    .maybe_block_request_enforce_segwit(1, block_hash(&context_payload().header))
                    == Some(params.enforce_segwit),
            ));
            if mask == 7 {
                let mut payload = context_payload();
                payload.header.version = 4;
                while open_bitcoin_core::consensus::check_block_header(&payload.header).is_err() {
                    payload.header.nonce += 1;
                }
                let hash = block_hash(&payload.header);
                let mut admitted = network.peer_manager.header_store().clone();
                admitted
                    .insert_canonical_header(
                        payload.header.clone(),
                        i64::from(payload.header.time) + 100,
                    )
                    .expect("earned actual header");
                network.peer_manager.seed_header_store(admitted);
                let mut foreign = params;
                foreign.enforce_segwit = !foreign.enforce_segwit;
                network.maybe_header_consensus_params = Some(foreign);
                let before = snapshot(&network);
                let refused = network.handle_compact_block_receive(1, payload.clone(), -100);
                selected_controls.push(refused.is_err() && snapshot(&network) == before);
                network.maybe_header_consensus_params = Some(params);
                let actual = network
                    .handle_compact_block_receive(1, payload, -100)
                    .expect("actual held selected context");
                selected_controls.push(
                    matches!(
                        actual.as_slice(),
                        [open_bitcoin_network::PeerAction::Send(
                            WireNetworkMessage::GetBlockTxn(_)
                        )]
                    ) && network
                        .peer_manager
                        .maybe_block_request_enforce_segwit(1, hash)
                        == Some(params.enforce_segwit),
                );
            }
        }
        std::fs::remove_dir_all(directory).expect("canonical fixture cleanup");
    }
    raw.cleanup();
    // Assert
    assert!(
        outcomes
            .iter()
            .all(|(mask, refused, unchanged, explicit_selected)| {
                if *mask == 0 {
                    !refused && *explicit_selected
                } else {
                    *refused && *unchanged
                }
            }),
        "actual binding controls: {outcomes:?}"
    );
    assert_eq!(selected_controls, vec![true, true]);
}

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
    // Synthetic refusal-test setup only: neither a canonical admission nor stored-body proof.
    assert!(
        network
            .peer_manager
            .header_store()
            .maybe_canonical_chain()
            .is_none()
    );
    let fixture_enforce_segwit = true;
    let _fixture_actions = network
        .peer_manager
        .handle_fixture_compact_block_download(
            1,
            payload,
            open_bitcoin_network::CompactBlockReceiveFacts {
                candidates: &[],
                extra: &[],
            },
            0,
            fixture_enforce_segwit,
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
