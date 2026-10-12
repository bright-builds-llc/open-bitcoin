// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/wallet.cpp
// - packages/bitcoin-knots/src/wallet/rpc/transactions.cpp
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

//! Actual funded-wallet and permissioned body callers on the retained-filter datadir.

use super::{fixtures::Phase162Fixture, recipe};
use super::{oracle_case, oracle_hash, retention};
use open_bitcoin_network::{
    BlockServingEligibilityInput, InventoryList, ParsedNetworkMessage, PeerConnectionClass,
    PermissionEffectLabel, ServiceFlags, VersionMessage, WireNetworkMessage,
    block_request_exceeds_limited_serve_window, classify_block_serving_eligibility,
};
use open_bitcoin_node::core::primitives::NetworkMagic;
use open_bitcoin_node::core::{
    chainstate::CoinsView,
    wallet::{AddressNetwork, DescriptorRole, Wallet, WalletSnapshot},
};
use open_bitcoin_node::core::{
    codec::encode_block,
    consensus::block_hash,
    primitives::{InventoryType, InventoryVector},
};
use open_bitcoin_node::network::filter_transport::FilterTransportSnapshot;
use open_bitcoin_node::{PersistMode, WalletRegistry, WalletRescanJob, WalletRescanJobState};
use std::{io, sync::Arc, time::Duration};
use tokio::net::TcpStream;

const WALLET_NAME: &str = "phase162-funded";

struct Cleanup(std::path::PathBuf);

impl Drop for Cleanup {
    fn drop(&mut self) {
        crate::tests::remove_dir_if_exists(&self.0);
    }
}

/// Only values survive close/reopen; no registry, context or store handle escapes.
pub(crate) struct WalletEvidence {
    snapshot: WalletSnapshot,
    job: WalletRescanJob,
}

pub(crate) async fn fund_wallet(fixture: &Phase162Fixture) -> WalletEvidence {
    let funding = fixture
        .store
        .coins_view()
        .get_coin(&fixture.recipe.wallet_outpoint)
        .expect("durable accepted funding read")
        .expect("genuine unspent funding");
    assert_eq!(funding.created_height, 20);
    assert_eq!(funding.output.value.to_sats(), recipe::WALLET_SATS);
    assert_eq!(funding.output.script_pubkey, fixture.recipe.wallet_script);
    let creating = fixture
        .store
        .load_block(fixture.provenance.receipts[20].block_hash)
        .expect("actual creating body read")
        .expect("pre-prune creating body");
    assert_eq!(creating, fixture.recipe.blocks[20]);
    assert_eq!(creating.transactions[0].outputs[1], funding.output);
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    // The recipe funds derivation zero; public metadata bounds only this test import.
    let descriptor_text = format!("{}#ob:0:0:0", recipe::WALLET_DESCRIPTOR);
    wallet
        .import_descriptor("receive", DescriptorRole::External, &descriptor_text)
        .expect("shared fixed descriptor");
    let descriptor = &wallet.descriptors()[0];
    assert_eq!(
        descriptor.original_text.split('#').next(),
        Some(recipe::WALLET_DESCRIPTOR)
    );
    assert_eq!(descriptor.original_text, descriptor_text);
    assert_eq!(descriptor.descriptor.range_start(), Some(0));
    assert_eq!(descriptor.descriptor.range_end(), Some(0));
    assert_eq!(descriptor.descriptor.next_index(), Some(0));
    assert_eq!(
        descriptor
            .descriptor
            .script_pubkey_at(0)
            .expect("funded derivation"),
        fixture.recipe.wallet_script
    );
    assert_eq!(
        wallet
            .default_receive_address()
            .expect("descriptor address")
            .script_pubkey,
        fixture.recipe.wallet_script
    );
    WalletRegistry::load(&fixture.store)
        .expect("registry load")
        .create_wallet(&fixture.store, WALLET_NAME, wallet, PersistMode::Sync)
        .expect("real named durable wallet");
    let evidence = checkpoint_wallet(fixture, 0).await;
    let wallet = WalletRegistry::load(&fixture.store)
        .expect("reload funded registry")
        .wallet(WALLET_NAME)
        .expect("persisted named wallet");
    let owned = wallet.utxos();
    assert_eq!(owned.len(), 1);
    assert_eq!(owned[0].outpoint, fixture.recipe.wallet_outpoint);
    assert_eq!(owned[0].created_height, 20);
    assert_eq!(owned[0].output, funding.output);
    evidence.assert_persisted(fixture, false);
    evidence
}

pub(crate) async fn checkpoint_wallet(fixture: &Phase162Fixture, start: u32) -> WalletEvidence {
    let mut context = fixture.shared.lock().await;
    context.set_request_wallet_name(Some(WALLET_NAME.to_owned()));
    let tip = fixture
        .opened
        .network
        .maybe_chain_tip()
        .expect("tip")
        .expect("accepted tip");
    let scanned = context
        .rescan_wallet_range(Some(start), Some(tip.height))
        .expect("ordinary pre-prune durable rescan");
    assert_eq!(scanned.stop_height, tip.height);
    let evidence = WalletEvidence {
        snapshot: context
            .wallet_snapshot()
            .expect("persisted funded snapshot"),
        job: context
            .wallet_rescan_job()
            .expect("prior job read")
            .expect("real prior job"),
    };
    assert_eq!(evidence.snapshot.maybe_tip_height, Some(tip.height));
    assert_eq!(evidence.job.state, WalletRescanJobState::Complete);
    assert_eq!(evidence.job.maybe_scanned_through_height, Some(tip.height));
    assert_eq!(evidence.job.next_height, tip.height + 1);
    assert_eq!(evidence.job.target_tip_hash, tip.block_hash);
    assert_eq!(evidence.job.target_tip_height, tip.height);
    drop(context);
    evidence.assert_persisted(fixture, false);
    evidence
}

#[tokio::test]
async fn phase162_wallet_funded_short_prefix_preflight() {
    // Arrange: accepted ordinary-maturity history, without retention pressure.
    let cleanup;
    let fixture = Phase162Fixture::seed_count(102);
    cleanup = Cleanup(fixture.path.clone());
    // Act
    let evidence = fund_wallet(&fixture).await;
    let fixture = Phase162Fixture::open(fixture.close());
    // Assert
    evidence.assert_persisted(&fixture, false);
    let handoff = fixture.close();
    drop(open_bitcoin_node::FjallNodeStore::open(&handoff.path).expect("preflight handles closed"));
    drop(cleanup);
    assert!(!handoff.path.exists());
}

impl WalletEvidence {
    pub(crate) fn assert_persisted(&self, fixture: &Phase162Fixture, failed: bool) {
        let registry = WalletRegistry::load(&fixture.store).expect("cold durable wallet registry");
        assert_eq!(
            registry
                .wallet(WALLET_NAME)
                .expect("actual named wallet")
                .snapshot(),
            self.snapshot
        );
        let mut job = registry
            .rescan_job(WALLET_NAME)
            .expect("actual persisted job")
            .clone();
        if failed {
            assert_eq!(job.state, WalletRescanJobState::Failed);
            let detail = job.maybe_error.take().expect("safe refusal category");
            assert!(
                detail.contains("creating")
                    && detail.contains("UnavailableNamespace")
                    && detail.contains("height 20")
            );
            job.state = self.job.state;
        }
        assert_eq!(
            job, self.job,
            "refusal cannot advance checkpoint, target or freshness"
        );
    }

    pub(crate) async fn assert_refused(&self, fixture: &Phase162Fixture) {
        self.assert_persisted(fixture, false);
        assert!(
            !fixture
                .store
                .has_block(fixture.provenance.receipts[20].block_hash)
                .expect("deleted creating body")
        );
        let mut context = fixture.shared.lock().await;
        context.set_request_wallet_name(Some(WALLET_NAME.to_owned()));
        // Requested bodies are retained; the selected unspent coin still needs h20.
        let error = context
            .rescan_wallet_range(Some(715), Some(self.job.target_tip_height))
            .expect_err("later-range rescan must check deleted creating history");
        assert!(
            error
                .maybe_detail
                .expect("safe RPC category")
                .message
                .contains("UnavailableNamespace")
        );
        assert_eq!(
            context.wallet_snapshot().expect("unchanged wallet"),
            self.snapshot
        );
        drop(context);
        self.assert_persisted(fixture, true);
    }
}

async fn retained_filter(fixture: &Phase162Fixture) {
    let row = oracle_case(fixture, "active-714");
    assert_eq!(
        retention::invoke(
            fixture,
            "getblockfilter",
            serde_json::json!([row["block_hash_display"], "basic"])
        )
        .await,
        serde_json::json!({"jsonrpc":"2.0", "id":"phase162", "result":{
            "filter": row["encoded"], "header": row["header_display"]}})
    );
}

async fn send(stream: &mut TcpStream, magic: NetworkMagic, message: WireNetworkMessage) {
    let bytes = message.encode_wire(magic).expect("body request frame");
    let mut offset = 0;
    while offset < bytes.len() {
        stream.writable().await.expect("body socket writable");
        match stream.try_write(&bytes[offset..]) {
            Ok(0) => panic!("zero body socket write"),
            Ok(count) => offset += count,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
            Err(error) => panic!("body socket write: {error}"),
        }
    }
}

async fn exact(stream: &TcpStream, buffer: &mut [u8]) {
    let mut offset = 0;
    while offset < buffer.len() {
        stream.readable().await.expect("body socket readable");
        match stream.try_read(&mut buffer[offset..]) {
            Ok(0) => panic!("closed incomplete body frame"),
            Ok(count) => offset += count,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
            Err(error) => panic!("body socket read: {error}"),
        }
    }
}

async fn read_frame(stream: &mut TcpStream) -> Vec<u8> {
    let mut bytes = vec![0; 24];
    exact(stream, &mut bytes).await;
    let size = open_bitcoin_codec::parse_message_header(&bytes)
        .expect("valid frame header")
        .payload_size as usize;
    assert!(size <= 16 * 1024, "recipe response frame bound");
    bytes.resize(24 + size, 0);
    exact(stream, &mut bytes[24..]).await;
    bytes
}

async fn message(stream: &mut TcpStream) -> WireNetworkMessage {
    ParsedNetworkMessage::decode_wire(&read_frame(stream).await)
        .expect("physical message")
        .message
}

async fn sentinel(stream: &mut TcpStream, magic: NetworkMagic, nonce: u64) {
    send(stream, magic, WireNetworkMessage::Ping { nonce }).await;
    assert_eq!(message(stream).await, WireNetworkMessage::Pong { nonce });
}

async fn handshake(stream: &mut TcpStream, magic: NetworkMagic) {
    send(
        stream,
        magic,
        WireNetworkMessage::Version(VersionMessage {
            nonce: 162_501,
            ..Default::default()
        }),
    )
    .await;
    let WireNetworkMessage::Version(version) = message(stream).await else {
        panic!("body caller must receive version first")
    };
    assert!(version.services.contains(ServiceFlags::NETWORK_LIMITED));
    assert!(version.services.contains(ServiceFlags::COMPACT_FILTERS));
    assert!(!version.services.contains(ServiceFlags::NETWORK));
    let mut acknowledged = false;
    for _ in 0..8 {
        if message(stream).await == WireNetworkMessage::Verack {
            acknowledged = true;
            break;
        }
    }
    assert!(acknowledged, "bounded body handshake");
    send(stream, magic, WireNetworkMessage::Verack).await;
    send(stream, magic, WireNetworkMessage::Ping { nonce: 162_502 }).await;
    for _ in 0..8 {
        let incoming = message(stream).await;
        if incoming == (WireNetworkMessage::Pong { nonce: 162_502 }) {
            return;
        }
        assert!(!matches!(
            incoming,
            WireNetworkMessage::Block(_) | WireNetworkMessage::NotFound(_)
        ));
    }
    panic!("body handshake sentinel was not received");
}

fn assert_body_permission(fixture: &Phase162Fixture) {
    let decision = fixture
        .config
        .inbound
        .permission_classes
        .resolve_inbound(std::net::Ipv4Addr::LOCALHOST.into());
    assert_eq!(
        decision.connection_class(),
        PeerConnectionClass::PermissionedInbound
    );
    assert!(
        decision
            .active_effects()
            .contains(&PermissionEffectLabel::DownloadServingPolicyInput)
    );
    let eligibility = classify_block_serving_eligibility(&BlockServingEligibilityInput {
        activation: fixture.config.block_serving,
        inbound_serving_enabled: fixture.config.inbound.enabled,
        connection_class: decision.connection_class(),
        active_permission_effects: decision.active_effects().to_vec(),
        inactive_permission_effects: decision.inactive_effects().to_vec(),
        status_available: true,
    });
    assert!(eligibility.eligible);
}

async fn body_session(
    fixture: &Phase162Fixture,
    retained_height: usize,
    missing: open_bitcoin_node::core::primitives::BlockHash,
) {
    assert_body_permission(fixture);
    let mut listener =
        crate::inbound_startup::start_inbound_listener_for_runtime_with_context_and_announcements(
            &fixture.config,
            Arc::clone(&fixture.shared),
            fixture.opened.peer_identity_authority.clone(),
            fixture.opened.announcement_outboxes.clone(),
            fixture.opened.network.clone(),
        )
        .await;
    assert_eq!(
        listener.state,
        open_bitcoin_rpc::inbound_listener::InboundListenerState::Listening
    );
    assert_eq!(listener.bound_endpoints.len(), 1);
    let endpoint: std::net::SocketAddr =
        listener.bound_endpoints[0].parse().expect("body endpoint");
    assert!(endpoint.ip().is_loopback() && endpoint.port() != 0);
    let network = fixture.opened.network.clone();
    let magic = fixture.config.sync.runtime.network.magic();
    let expected = fixture.recipe.blocks[retained_height].clone();
    let mut client = tokio::spawn(async move {
        let mut stream = TcpStream::connect(endpoint)
            .await
            .expect("body caller connect");
        handshake(&mut stream, magic).await;
        let peers = network.peer_manager_snapshot().expect("actual body peer");
        assert_eq!(peers.peer_ids().len(), 1);
        let peer_id = *peers.peer_ids().first().expect("live peer ID");
        let peer = peers.peer_state(peer_id).expect("admitted body peer");
        assert!(peer.remote_version_received && peer.remote_verack_received);
        let record = peer
            .maybe_inbound_record
            .as_ref()
            .expect("actual inbound record");
        assert_eq!(
            record.connection_class,
            PeerConnectionClass::PermissionedInbound
        );
        assert!(
            record
                .permission_decision
                .active_effects()
                .contains(&PermissionEffectLabel::DownloadServingPolicyInput)
        );
        for (index, hash) in [block_hash(&expected.header), missing]
            .into_iter()
            .enumerate()
        {
            let inventory = InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::Block,
                object_hash: hash.into(),
            }]);
            send(
                &mut stream,
                magic,
                WireNetworkMessage::GetData(inventory.clone()),
            )
            .await;
            let bytes = read_frame(&mut stream).await;
            let incoming = ParsedNetworkMessage::decode_wire(&bytes)
                .expect("body response")
                .message;
            if index == 0 {
                assert_eq!(
                    bytes,
                    WireNetworkMessage::Block(expected.clone())
                        .encode_wire(magic)
                        .expect("exact retained frame")
                );
                let WireNetworkMessage::Block(block) = incoming else {
                    panic!("retained body positive control")
                };
                assert_eq!(block_hash(&block.header), hash);
                assert_eq!(
                    encode_block(&block).expect("response body"),
                    encode_block(&expected).expect("original accepted body")
                );
            } else {
                assert_eq!(incoming, WireNetworkMessage::NotFound(inventory.clone()));
                assert_eq!(
                    bytes,
                    WireNetworkMessage::NotFound(inventory)
                        .encode_wire(magic)
                        .expect("exact NotFound frame")
                );
            }
            sentinel(&mut stream, magic, 162_510 + index as u64).await;
            assert!(
                network
                    .peer_manager_snapshot()
                    .expect("live after response")
                    .peer_ids()
                    .contains(&peer_id)
            );
            assert_eq!(
                network
                    .basic_filter_transport_snapshot()
                    .expect("released filter charges"),
                FilterTransportSnapshot::default()
            );
        }
        drop(stream);
    });
    let outcome = tokio::time::timeout(Duration::from_secs(30), &mut client).await;
    let maybe_cancelled = if outcome.is_err() {
        client.abort();
        Some(client.await)
    } else {
        None
    };
    let cleanup = tokio::time::timeout(Duration::from_secs(5), async {
        while !fixture
            .opened
            .network
            .peer_manager_snapshot()
            .expect("peer cleanup")
            .peer_ids()
            .is_empty()
        {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await;
    tokio::time::timeout(Duration::from_secs(5), listener.shutdown())
        .await
        .expect("body listener joined");
    cleanup.expect("body peer removed");
    if let Some(cancelled) = maybe_cancelled {
        assert!(
            cancelled
                .expect_err("aborted client must settle")
                .is_cancelled()
        );
    }
    outcome
        .expect("finite body session")
        .expect("body assertions");
    assert_eq!(
        fixture
            .opened
            .network
            .basic_filter_transport_snapshot()
            .expect("final charge cleanup"),
        FilterTransportSnapshot::default()
    );
}

pub(crate) async fn assert_restrictions(
    fixture: Phase162Fixture,
    settled: &retention::SettledIdentity,
    wallet: &WalletEvidence,
) -> Phase162Fixture {
    let fixture = Phase162Fixture::open_body_serving(fixture.close());
    settled.assert_unchanged(&fixture);
    assert!(fixture.config.block_serving.block_serving.enabled);
    assert!(!fixture.config.block_serving.compact_relay.enabled);
    let tip = fixture
        .opened
        .network
        .maybe_chain_tip()
        .expect("body tip")
        .expect("accepted tip");
    assert_eq!(tip.height, 1002);
    for (height, distance, present) in [(714, 288, false), (715, 287, true)] {
        assert_eq!(tip.height - height, distance);
        assert!(!block_request_exceeds_limited_serve_window(
            tip.height, height
        ));
        retention::assert_pair(
            &fixture,
            block_hash(&fixture.recipe.blocks[height as usize].header),
            present,
        );
    }
    assert!(
        fixture
            .store
            .load_have_pruned()
            .expect("earned physical prune marker")
    );
    assert_eq!(
        oracle_hash(oracle_case(&fixture, "active-714")),
        block_hash(&fixture.recipe.blocks[714].header)
    );
    retained_filter(&fixture).await;
    body_session(
        &fixture,
        715,
        block_hash(&fixture.recipe.blocks[714].header),
    )
    .await;
    retained_filter(&fixture).await;
    settled.assert_unchanged(&fixture);
    wallet.assert_refused(&fixture).await;
    for height in [20, 714] {
        retention::assert_pair(
            &fixture,
            block_hash(&fixture.recipe.blocks[height].header),
            false,
        );
    }
    retained_filter(&fixture).await;
    settled.assert_unchanged(&fixture);
    let fixture = Phase162Fixture::open_with_prune(fixture.close(), 550);
    wallet.assert_persisted(&fixture, true);
    for height in [20, 714] {
        retention::assert_pair(
            &fixture,
            block_hash(&fixture.recipe.blocks[height].header),
            false,
        );
    }
    retained_filter(&fixture).await;
    settled.assert_unchanged(&fixture);
    eprintln!(
        "phase162 restrictions: funded_wallets=1 funding_height=20 funding_sats=25000 prior_checkpoint=1002 later_range_start=715 creating_history_refused=true snapshot_unchanged=true checkpoint_unchanged=true retained_body_height=715 deleted_body_height=714 distances=287,288 serving_window=290 block_frames=1 notfound_frames=1 sentinel_pongs=2 charges=0 peer_removed=true joined=true reopened=true"
    );
    fixture
}

#[tokio::test]
async fn phase162_body_serving_short_prefix_preflight() {
    // Arrange: genuine accepted prefix; unknown hash is only a transport control.
    let cleanup;
    let fixture = Phase162Fixture::seed_count(102);
    cleanup = Cleanup(fixture.path.clone());
    let fixture = Phase162Fixture::open_body_serving(fixture.close());
    // Act / Assert: this is not the full actually-pruned-history acceptance proof.
    body_session(&fixture, 101, Default::default()).await;
    let handoff = fixture.close();
    drop(
        open_bitcoin_node::FjallNodeStore::open(&handoff.path)
            .expect("body preflight handles closed"),
    );
    drop(cleanup);
    assert!(!handoff.path.exists());
}
