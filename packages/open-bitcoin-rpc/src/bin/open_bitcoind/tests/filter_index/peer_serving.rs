// Parity breadcrumbs:
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net_processing.cpp

use super::*;
use crate::tests::fixture_runtime::{
    fixture_open_authoritative_network_runtime as open_authoritative_network_runtime,
    fixture_open_runtime_store as open_runtime_store,
};
use open_bitcoin_network::{ServiceFlags, VersionMessage, WireNetworkMessage};

#[path = "peer_serving/evidence.rs"]
mod evidence;

fn own_services(messages: &[WireNetworkMessage]) -> ServiceFlags {
    messages
        .iter()
        .find_map(|message| match message {
            WireNetworkMessage::Version(version) => Some(version.services),
            _ => None,
        })
        .expect("actual local version")
}

#[test]
fn phase160_startup_general_reaches_real_authority_before_index_catchup() {
    // Arrange
    let history = History::new(40);
    let store = history.seed();
    fs::write(history.path.join("open-bitcoin.jsonc"), "{\"prune\":1}")
        .expect("actual prune config");
    let args = [
        format!("-datadir={}", history.path.display()).into(),
        "-blockfilterindex=basic".into(),
        "-peerblockfilters".into(),
    ];
    let config = open_bitcoin_rpc::config::load_runtime_config_for_args(&args, &history.path)
        .expect("actual general serving configuration");
    // Act
    let opened = open_authoritative_network_runtime(&config, Some(store.clone()))
        .expect("configured durable startup")
        .expect_durable();
    let network = opened.network.clone();
    let context = ManagedRpcContext::from_fixture_runtime_config_with_network_handle(
        &config,
        network.clone(),
        Some(store.clone()),
    )
    .expect("shared daemon context preserves configured capability");
    let outbound = network
        .connect_outbound_peer(160_301, 1)
        .expect("real outbound version constructor");
    network.add_inbound_peer(160_302).expect("inbound peer");
    let inbound = network
        .receive_message(
            160_302,
            WireNetworkMessage::Version(VersionMessage {
                nonce: 999,
                ..Default::default()
            }),
            1,
            open_bitcoin_node::core::consensus::ScriptVerifyFlags::P2SH,
            fixtures::params(),
        )
        .expect("real inbound version constructor");
    // Assert
    assert_ne!(
        network
            .network_info()
            .expect("actual configured authority")
            .local_services_bits
            & ServiceFlags::COMPACT_FILTERS.bits(),
        0
    );
    for services in [own_services(&outbound), own_services(&inbound.outbound)] {
        assert!(services.contains(ServiceFlags::COMPACT_FILTERS));
        assert!(services.contains(ServiceFlags::NETWORK_LIMITED));
        assert!(services.contains(ServiceFlags::WITNESS));
        assert!(!services.contains(ServiceFlags::NETWORK));
    }
    for id in [160_301, 160_302] {
        assert!(
            !network
                .peer_manager_snapshot()
                .expect("actual peers")
                .peer_state(id)
                .expect("actual peer")
                .sync_started
        );
    }
    drop(context);
    assert!(!config.sync.is_enabled());
    assert!(!config.inbound.enabled);
    let progress = network
        .drive_basic_filter_index_turn()
        .expect("ordinary bounded maintenance")
        .maybe_progress
        .expect("BASIC owner");
    assert!(!progress.initially_synchronized());
    assert!(
        progress
            .maybe_processed_endpoint()
            .expect("lagged prefix")
            .height()
            < 39
    );
}

#[test]
fn phase160_startup_invalid_permissions_preserve_retained_sources_and_index() {
    for tokens in [
        vec!["in", "blockfilters"],
        vec!["in", "all", "blockfilters"],
    ] {
        // Arrange
        let history = History::new(40);
        let store = history.seed();
        let mut config = history.config(None);
        let class = open_bitcoin_network::ParsedPeerPermissionClass::parse(
            "trusted",
            ["127.0.0.1"],
            tokens,
        )
        .expect("typed explicit permission");
        config.inbound.permission_classes =
            open_bitcoin_network::PeerPermissionClassRegistry::new([class]);
        let before = store.load_prune_protection().expect("prior protection");
        let locks = store.load_prune_locks().expect("prior locks");
        let sources = history
            .snapshot
            .active_chain
            .iter()
            .map(|position| {
                (
                    store.load_block(position.block_hash).expect("prior body"),
                    store.load_undo(position.block_hash).expect("prior undo"),
                )
            })
            .collect::<Vec<_>>();
        // Act
        let selected = open_runtime_store(&config);
        let opened = open_authoritative_network_runtime(&config, Some(store.clone()));
        // Assert
        assert!(selected.is_err());
        assert!(opened.is_err());
        assert_eq!(
            store.load_prune_protection().expect("unchanged protection"),
            before
        );
        assert_eq!(store.load_prune_locks().expect("unchanged locks"), locks);
        assert!(!store.load_have_pruned().expect("unchanged prune state"));
        for (position, expected) in history.snapshot.active_chain.iter().zip(sources) {
            assert_eq!(
                (
                    store
                        .load_block(position.block_hash)
                        .expect("unchanged body"),
                    store
                        .load_undo(position.block_hash)
                        .expect("unchanged undo")
                ),
                expected
            );
        }
    }
}

#[test]
fn phase160_startup_all_only_keeps_transient_backend_absent_and_base_services_off() {
    // Arrange
    let directory = temp_store_path("phase160-all-transient");
    let args = [
        "-openbitcoininboundpermissionclass=trusted@127.0.0.1=in,all".into(),
        "-blockfilterindex=0".into(),
    ];
    let config = open_bitcoin_rpc::config::load_runtime_config_for_args(&args, &directory)
        .expect("all-only actual parser");
    // Act
    let store = open_runtime_store(&config).expect("no durable trigger");
    assert!(store.is_none());
    let opened =
        open_authoritative_network_runtime(&config, store).expect("accepted all-only startup");
    // Assert
    let OpenedAuthoritativeRuntime::Transient(opened) = opened else {
        panic!("all does not initialize an index");
    };
    assert!(!directory.exists());
    assert!(opened.maybe_sync_runtime.is_none());
    assert_eq!(
        opened
            .network
            .network_info()
            .expect("global base")
            .local_services_bits
            & ServiceFlags::COMPACT_FILTERS.bits(),
        0
    );
}

#[tokio::test]
async fn phase160_startup_all_only_actual_loopback_version_without_backend() {
    use std::io::{Read, Write};
    // Arrange
    let directory = temp_store_path("phase160-all-loopback");
    let args = [
        "-openbitcoininboundpermissionclass=trusted@127.0.0.1=in,all".into(),
        "-openbitcoininbound=1".into(),
        "-openbitcoinlisten=127.0.0.1:0".into(),
        "-openbitcoinmaxinbound=2".into(),
        "-openbitcoinreservedslots=1".into(),
        "-blockfilterindex=0".into(),
    ];
    let config = open_bitcoin_rpc::config::load_runtime_config_for_args(&args, &directory)
        .expect("actual all-only listener config");
    let store = open_runtime_store(&config).expect("nonindex startup");
    assert!(store.is_none());
    let OpenedAuthoritativeRuntime::Transient(opened) =
        open_authoritative_network_runtime(&config, store).expect("actual startup")
    else {
        panic!("listener without datadir must retain transient authority");
    };
    let context = ManagedRpcContext::from_fixture_runtime_config_with_network_handle(
        &config,
        opened.network.clone(),
        None,
    )
    .expect("shared context");
    let shared = Arc::new(tokio::sync::Mutex::new(context));
    let mut listener =
        crate::inbound_startup::start_inbound_listener_for_runtime_with_context_and_announcements(
            &config,
            shared,
            opened.peer_identity_authority,
            opened.announcement_outboxes,
            opened.network.clone(),
        )
        .await;
    assert_eq!(listener.state, InboundListenerState::Listening);
    let endpoint = listener.bound_endpoints[0].clone();
    let request = WireNetworkMessage::Version(VersionMessage {
        nonce: 999,
        ..Default::default()
    })
    .encode_wire(config.sync.runtime.network.magic())
    .expect("remote version frame");
    // Act
    let message = tokio::task::spawn_blocking(move || {
        let mut stream = std::net::TcpStream::connect(endpoint).expect("actual loopback socket");
        stream
            .set_read_timeout(Some(std::time::Duration::from_secs(5)))
            .expect("bounded reads");
        stream
            .set_write_timeout(Some(std::time::Duration::from_secs(5)))
            .expect("bounded writes");
        stream.write_all(&request).expect("achieved remote version");
        let mut header = [0_u8; 24];
        stream
            .read_exact(&mut header)
            .expect("actual version header");
        let size = open_bitcoin_codec::parse_message_header(&header)
            .expect("valid version header")
            .payload_size as usize;
        assert!(size < 4_096, "bounded handshake frame");
        let mut bytes = header.to_vec();
        bytes.resize(24 + size, 0);
        stream
            .read_exact(&mut bytes[24..])
            .expect("actual version payload");
        open_bitcoin_network::ParsedNetworkMessage::decode_wire(&bytes)
            .expect("valid actual wire version")
            .message
    })
    .await
    .expect("bounded actual handshake completion");
    // Assert
    assert!(own_services(&[message]).contains(ServiceFlags::COMPACT_FILTERS));
    assert_eq!(
        opened
            .network
            .network_info()
            .expect("unmodified base")
            .local_services_bits
            & ServiceFlags::COMPACT_FILTERS.bits(),
        0
    );
    assert!(!config.block_filter_index.is_enabled());
    assert!(!directory.exists());
    listener.shutdown().await;
}

#[test]
fn phase160_startup_invalid_general_preserves_existing_saved_index_owner() {
    // Arrange
    let history = History::new(40);
    let store = history.seed();
    let enabled = history.config(Some("-blockfilterindex=1"));
    drop(open_authoritative_network_runtime(&enabled, Some(store.clone())).expect("saved owner"));
    let before = store.load_prune_protection().expect("saved owner");
    let locks = store.load_prune_locks().expect("saved locks");
    let mut invalid = history.config(Some("-blockfilterindex=0"));
    invalid.peer_block_filters = true;
    // Act
    let selected = open_runtime_store(&invalid);
    let opened = open_authoritative_network_runtime(&invalid, Some(store.clone()));
    // Assert
    assert!(selected.is_err());
    assert!(opened.is_err());
    assert_eq!(
        store
            .load_prune_protection()
            .expect("saved owner unchanged"),
        before
    );
    assert_eq!(
        store.load_prune_locks().expect("saved locks unchanged"),
        locks
    );
}

#[test]
fn phase160_startup_invalid_general_refuses_before_store_creation() {
    // Arrange
    let directory = temp_store_path("phase160-invalid-general");
    let config = RuntimeConfig {
        maybe_data_dir: Some(directory.clone()),
        peer_block_filters: true,
        ..Default::default()
    };
    // Act
    let result = open_runtime_store(&config);
    // Assert
    assert!(
        result.is_err(),
        "invalid general serving must refuse before choosing a transient runtime"
    );
    assert!(!directory.exists());
}
