// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/txdb.cpp

use super::super::*;
use crate::{StorageNamespace, chainstate::BasicFilterStartupMode};
use open_bitcoin_core::{
    chainstate::{AnchoredBlock, CoinsView},
    consensus::{CanonicalChain, chain_proof::ProofWork},
};
use open_bitcoin_network::{HeaderStore, HeadersMessage, WireNetworkMessage};

const SELECTORS: [(CanonicalChain, SyncNetwork); 4] = [
    (CanonicalChain::Mainnet, SyncNetwork::Mainnet),
    (CanonicalChain::Testnet3, SyncNetwork::Testnet),
    (CanonicalChain::Signet, SyncNetwork::Signet),
    (CanonicalChain::Regtest, SyncNetwork::Regtest),
];

fn source() -> std::path::PathBuf {
    let mut nonce = [0_u8; 16];
    getrandom::fill(&mut nonce).expect("fixture nonce");
    std::fs::canonicalize(std::env::temp_dir())
        .expect("canonical temp parent")
        .join(format!("phase166-runtime-{:x}", u128::from_le_bytes(nonce)))
}

fn config(network: SyncNetwork) -> SyncRuntimeConfig {
    SyncRuntimeConfig {
        network,
        dns_seeds: vec![],
        manual_peers: vec![SyncPeerAddress::manual("fixture.invalid", 1)],
        ..Default::default()
    }
}

type Rows = Vec<(String, Vec<u8>, Vec<u8>)>;
fn rows(store: &FjallNodeStore) -> Rows {
    let mut rows = vec![];
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
        let keyspace = store
            .database()
            .keyspace(namespace.as_str(), fjall::KeyspaceCreateOptions::default)
            .expect("existing keyspace");
        for row in keyspace.iter() {
            let (key, value) = row.into_inner().expect("row");
            rows.push((namespace.as_str().to_owned(), key.to_vec(), value.to_vec()));
        }
    }
    rows
}

#[test]
fn phase166_runtime_earned_root_projections_agree_and_activation_is_disabled() {
    for (chain, selector) in SELECTORS {
        // Arrange
        let source = source();
        let opened = FjallNodeStore::open_canonical_genesis(&source, chain).expect("proved");
        let proof = opened.genesis().clone();
        // Act
        let runtime = DurableSyncRuntime::open_canonical_genesis_configured(
            opened,
            config(selector),
            BasicFilterStartupMode::Disabled,
        )
        .expect("runtime");
        // Assert
        let handle = runtime.network_handle();
        assert_eq!(handle.maybe_canonical_chain().expect("scope"), Some(chain));
        let tip = handle.maybe_chain_tip().expect("tip").expect("genesis");
        let headers = handle.best_chain_entries().expect("headers");
        assert_eq!(headers.len(), 1);
        assert_eq!(headers[0].block_hash, proof.hash());
        assert_eq!(headers[0].chain_work, proof.work());
        assert_eq!(
            handle
                .requestable_header_entries(runtime.consensus_params)
                .expect("earned requestable root"),
            headers
        );
        assert_eq!(tip.chain_work, proof.work());
        assert_eq!(tip.block_hash, proof.hash());
        assert_eq!(
            runtime.store.coins_view().best_block().expect("B"),
            Some(proof.hash())
        );
        assert!(
            runtime
                .store
                .coins_view()
                .complete_snapshot()
                .expect("coins")
                .coins
                .is_empty()
        );
        assert_eq!(
            runtime.snapshot_summary().maybe_validated_active_chain_work,
            Some(proof.work().to_string())
        );
        assert!(!handle.relay_activation_enabled().expect("relay off"));
        assert!(
            handle
                .peer_manager_snapshot()
                .expect("peers")
                .peer_ids()
                .is_empty()
        );
        assert_eq!(runtime.best_heights().expect("heights"), (0, 0));
        drop(handle);
        drop(runtime);
        std::fs::remove_dir_all(source).expect("cleanup");
    }
}

#[test]
fn phase166_runtime_config_mismatch_refuses_before_initialization() {
    // Arrange
    let source = source();
    let opened =
        FjallNodeStore::open_canonical_genesis(&source, CanonicalChain::Regtest).expect("proved");
    // Act
    let result = DurableSyncRuntime::open_canonical_genesis_configured(
        opened,
        config(SyncNetwork::Mainnet),
        BasicFilterStartupMode::Enabled,
    );
    // Assert
    assert!(matches!(result, Err(SyncRuntimeError::InvalidData { .. })));
    let reopened =
        FjallNodeStore::reopen_canonical_genesis_existing(&source, CanonicalChain::Regtest)
            .expect("unchanged complete original");
    let (store, _) = reopened.into_parts().expect("unchanged proof");
    assert!(
        store
            .maybe_basic_filter_checkpoint()
            .expect("index absent")
            .is_none()
    );
    drop(store);
    std::fs::remove_dir_all(source).expect("cleanup");
}

#[test]
fn phase166_runtime_descendant_routes_refuse_without_owner_or_storage_effects() {
    for (chain, selector) in SELECTORS {
        // Arrange
        let source = source();
        let opened = FjallNodeStore::open_canonical_genesis(&source, chain).expect("proved");
        let mut child = chain.spec().genesis_block();
        child.header.previous_block_hash = opened.genesis().hash();
        let params = open_bitcoin_core::chainstate::engine::canonical_genesis::consensus_params(
            opened.genesis(),
        );
        let runtime = DurableSyncRuntime::open_canonical_genesis_configured(
            opened,
            config(selector),
            BasicFilterStartupMode::Disabled,
        )
        .expect("runtime");
        let handle = runtime.network_handle();
        let before = handle.chainstate_snapshot().expect("before");
        let headers = handle.header_entries().expect("headers");
        let peers = format!("{:?}", handle.peer_manager_snapshot().expect("peers"));
        let durable = rows(&runtime.store);
        let assert_unchanged = || {
            assert_eq!(handle.chainstate_snapshot().expect("after"), before);
            assert_eq!(handle.header_entries().expect("after headers"), headers);
            assert_eq!(
                format!("{:?}", handle.peer_manager_snapshot().expect("after peers")),
                peers
            );
            assert_eq!(rows(&runtime.store), durable);
        };
        // Act / Assert
        for message in [
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![child.header.clone()],
            }),
            WireNetworkMessage::Block(child.clone()),
            WireNetworkMessage::CompactBlock(open_bitcoin_codec::CompactBlockPayload {
                header: child.header.clone(),
                nonce: 0,
                short_ids: vec![],
                prefilled_transactions: vec![],
            }),
            WireNetworkMessage::BlockTxn(open_bitcoin_codec::BlockTransactions {
                block_hash: chain.spec().genesis_hash(),
                transactions: vec![],
            }),
        ] {
            let error = handle
                .receive_sync_message(
                    1,
                    message.clone(),
                    2_000_000_000,
                    ScriptVerifyFlags::P2SH,
                    params,
                )
                .expect_err("ordinary history refused");
            assert!(error.to_string().contains("unsupported-validation-scope"));
            assert_unchanged();
            assert!(
                handle
                    .receive_message(
                        1,
                        message.clone(),
                        2_000_000_000,
                        ScriptVerifyFlags::P2SH,
                        params
                    )
                    .is_err()
            );
            assert_unchanged();
            assert!(
                handle
                    .receive_message_for_durable_serving(
                        1,
                        message,
                        2_000_000_000,
                        ScriptVerifyFlags::P2SH,
                        params,
                        |_| panic!("refuse before durable payload access")
                    )
                    .is_err()
            );
            assert_unchanged();
        }
        assert!(
            handle
                .connect_stored_block(
                    &child,
                    ProofWork::ZERO,
                    2_000_000_000,
                    ScriptVerifyFlags::P2SH,
                    params
                )
                .is_err()
        );
        assert_unchanged();
        assert!(
            handle
                .connect_local_block(&child, ScriptVerifyFlags::P2SH, params)
                .is_err()
        );
        assert_unchanged();
        assert!(
            handle
                .try_admit_fixture_headers(HeaderStore::default(), 2_000_000_000, params)
                .is_err()
        );
        assert_unchanged();
        assert!(
            handle
                .reorg_to_branch(
                    &[],
                    &[AnchoredBlock {
                        block: child,
                        chain_work: ProofWork::ZERO
                    }],
                    open_bitcoin_mempool::ReorgLifecycleContext::new(
                        open_bitcoin_mempool::PolicyTime::from_unix_seconds(1)
                    ),
                    ScriptVerifyFlags::P2SH,
                    params
                )
                .is_err()
        );
        assert_unchanged();
        drop(handle);
        drop(runtime);
        std::fs::remove_dir_all(source).expect("cleanup");
    }
}

struct NoNetwork;
impl SyncPeerResolver for NoNetwork {
    fn resolve(
        &mut self,
        _: &SyncPeerAddress,
        _: &SyncRuntimeConfig,
    ) -> Result<Vec<ResolvedSyncPeerAddress>, SyncRuntimeError> {
        panic!("canonical sync must not resolve")
    }
}
impl SyncTransport for NoNetwork {
    type Session = Self;
    fn connect(
        &mut self,
        _: &ResolvedSyncPeerAddress,
        _: &SyncRuntimeConfig,
    ) -> Result<Self, SyncRuntimeError> {
        panic!("canonical sync must not connect")
    }
}
impl SyncPeerSession for NoNetwork {
    fn send(
        &mut self,
        _: &WireNetworkMessage,
        _: open_bitcoin_core::primitives::NetworkMagic,
    ) -> Result<(), SyncRuntimeError> {
        panic!("canonical sync must not send")
    }
    fn receive(
        &mut self,
        _: open_bitcoin_core::primitives::NetworkMagic,
    ) -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> {
        panic!("canonical sync must not receive")
    }
}

#[test]
fn phase166_runtime_sync_refuses_before_resolver_transport_or_persistence() {
    for (chain, selector) in SELECTORS {
        // Arrange
        let source = source();
        let opened = FjallNodeStore::open_canonical_genesis(&source, chain).expect("proved");
        let mut runtime = DurableSyncRuntime::open_canonical_genesis_configured(
            opened,
            config(selector),
            BasicFilterStartupMode::Disabled,
        )
        .expect("runtime");
        runtime.maybe_reconcile_progress = Some(types::SyncReconcileProgress::NoChange);
        let before = rows(&runtime.store);
        let snapshot = runtime.snapshot_summary();
        // Act
        let error = runtime
            .sync_once_with_resolver(&mut NoNetwork, &mut NoNetwork, 2_000_000_000)
            .expect_err("scope refused");
        // Assert
        assert!(matches!(error, SyncRuntimeError::InvalidData { .. }));
        if chain == CanonicalChain::Signet {
            assert!(error.to_string().contains("signet_challenge_validation"));
        }
        assert_eq!(rows(&runtime.store), before);
        assert_eq!(runtime.snapshot_summary(), snapshot);
        drop(runtime);
        std::fs::remove_dir_all(source).expect("cleanup");
    }
}

struct CountingResolver(usize);
impl SyncPeerResolver for CountingResolver {
    fn resolve(
        &mut self,
        _: &SyncPeerAddress,
        _: &SyncRuntimeConfig,
    ) -> Result<Vec<ResolvedSyncPeerAddress>, SyncRuntimeError> {
        self.0 += 1;
        Ok(vec![])
    }
}

#[test]
fn phase166_runtime_fixture_sync_still_reaches_resolver() {
    // Arrange
    let source = source();
    let store = FjallNodeStore::open(&source).expect("raw fixture");
    let mut runtime = DurableSyncRuntime::open_configured(
        store,
        config(SyncNetwork::Regtest),
        BasicFilterStartupMode::Disabled,
    )
    .expect("fixture runtime");
    let mut resolver = CountingResolver(0);
    // Act
    let result = runtime.sync_once_with_resolver(&mut NoNetwork, &mut resolver, 2_000_000_000);
    // Assert
    assert!(result.is_ok(), "fixture sync control: {result:?}");
    assert!(resolver.0 > 0, "ordinary fixture path reached resolution");
    assert_eq!(
        runtime
            .network
            .maybe_canonical_chain()
            .expect("fixture scope"),
        None
    );
    drop(runtime);
    std::fs::remove_dir_all(source).expect("cleanup");
}

#[test]
fn phase166_runtime_preserve_saved_without_basic_creates_no_index_authority() {
    for (chain, selector) in SELECTORS {
        // Arrange
        let source = source();
        let opened = FjallNodeStore::open_canonical_genesis(&source, chain).expect("proved");
        // Act
        let runtime = DurableSyncRuntime::open_canonical_genesis_configured(
            opened,
            config(selector),
            BasicFilterStartupMode::PreserveSaved,
        )
        .expect("runtime");
        // Assert
        assert!(
            runtime
                .store
                .maybe_basic_filter_checkpoint()
                .expect("absent BASIC")
                .is_none()
        );
        assert_eq!(
            runtime.network.maybe_canonical_chain().expect("scope"),
            Some(chain)
        );
        assert!(
            runtime
                .snapshot_summary()
                .maybe_validated_active_chain_work
                .is_some()
        );
        drop(runtime);
        std::fs::remove_dir_all(source).expect("cleanup");
    }
}
