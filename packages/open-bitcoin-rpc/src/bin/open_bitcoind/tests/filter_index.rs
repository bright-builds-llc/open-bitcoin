// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use crate::{OpenedAuthoritativeRuntime, open_runtime_store};
use open_bitcoin_node::core::chainstate::{PruneMode, filter_index::lifecycle::IndexLifecycle};

#[path = "filter_index/fixtures.rs"]
pub(crate) mod fixtures;
use fixtures::History;

#[test]
fn phase157_daemon_explicit_zero_preserves_complete_saved_checkpoint_on_reopen() {
    // Arrange
    let history = History::new(40);
    let store = history.seed();
    let enabled = history.config(Some("-blockfilterindex=basic"));
    let opened = open_authoritative_network_runtime(&enabled, Some(store.clone()))
        .expect("actual enabled open")
        .expect_durable();
    for _ in 0..4 {
        opened
            .network
            .drive_basic_filter_index_turn()
            .expect("bounded saved-prefix progress");
    }
    let saved = store
        .load_prune_protection()
        .expect("earned saved prefix")
        .maybe_owner()
        .expect("saved owner")
        .checkpoint();
    assert!(
        matches!(saved.checkpoint().prefix(), open_bitcoin_node::core::chainstate::IndexPrefix::Committed(id) if id.height() == 39)
    );
    drop(opened);
    drop(store);
    let disabled = history.config(Some("-blockfilterindex=0"));
    // Act
    let store = open_runtime_store(&disabled)
        .expect("explicit zero selection")
        .expect("real durable open");
    let opened = open_authoritative_network_runtime(&disabled, Some(store.clone()))
        .expect("configured disable");
    // Assert
    let owner = store
        .load_prune_protection()
        .expect("prefix retained")
        .maybe_owner()
        .expect("saved owner");
    assert_eq!(owner.checkpoint(), saved);
    assert!(matches!(owner.lifecycle(), IndexLifecycle::Disabled { .. }));
    drop(opened);
    drop(store);
    let reopened = FjallNodeStore::open(&history.path).expect("closed actual reopen");
    assert_eq!(
        reopened
            .load_prune_protection()
            .expect("durable saved prefix")
            .maybe_owner()
            .expect("owner")
            .checkpoint(),
        saved
    );
}

#[test]
fn phase157_daemon_missing_required_body_or_undo_preserves_sources_and_absent_owner() {
    for missing_body in [true, false] {
        // Arrange
        let history = History::new(40);
        let store =
            history.seed_with_missing(missing_body.then_some(12), (!missing_body).then_some(12));
        let config = history.config(Some("-blockfilterindex=basic"));
        let sources = history
            .snapshot
            .active_chain
            .iter()
            .map(|position| {
                (
                    store.load_block(position.block_hash).expect("body before"),
                    store.load_undo(position.block_hash).expect("undo before"),
                )
            })
            .collect::<Vec<_>>();
        let before = store.load_prune_protection().expect("unowned");
        // Act
        let result = open_authoritative_network_runtime(&config, Some(store.clone()));
        // Assert
        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("missing input must refuse"),
        };
        assert!(
            error
                .to_string()
                .contains(if missing_body { "body" } else { "undo" }),
            "{error}"
        );
        assert!(error.to_string().contains("12"), "{error}");
        assert_eq!(
            store.load_prune_protection().expect("unchanged owner"),
            before
        );
        for (position, source) in history.snapshot.active_chain.iter().zip(&sources) {
            assert_eq!(
                (
                    store.load_block(position.block_hash).expect("body after"),
                    store.load_undo(position.block_hash).expect("undo after")
                ),
                *source
            );
        }
        drop(store);
        let reopened = FjallNodeStore::open(&history.path).expect("closed actual Fjall reopen");
        assert_eq!(
            reopened
                .load_prune_protection()
                .expect("durable unchanged owner"),
            before
        );
    }
}

#[test]
fn phase157_daemon_basic_nonexistent_datadir_refuses_without_creating_it() {
    // Arrange
    let data_dir = temp_store_path("phase157-missing-datadir");
    assert!(!data_dir.exists());
    let config = RuntimeConfig {
        maybe_data_dir: Some(data_dir.clone()),
        block_filter_index: open_bitcoin_rpc::config::BasicFilterIndexSetting::Basic,
        ..RuntimeConfig::default()
    };
    // Act
    let result = open_runtime_store(&config);
    // Assert
    assert!(result.is_err());
    assert!(!data_dir.exists());
}

#[test]
fn phase157_daemon_basic_forms_select_durable_and_one_strict_startup_prefix() {
    for option in [
        "-blockfilterindex",
        "-blockfilterindex=1",
        "-blockfilterindex=basic",
    ] {
        // Arrange
        let history = History::new(40);
        drop(history.seed());
        let config = history.config(Some(option));
        // Act
        let store = open_runtime_store(&config)
            .expect("selection")
            .expect("BASIC selects durable");
        let opened = open_authoritative_network_runtime(&config, Some(store.clone()))
            .expect("actual startup")
            .expect_durable();
        // Assert
        let first_idle = opened
            .network
            .drive_basic_filter_index_turn()
            .expect("one bounded ordinary turn");
        let progress = first_idle.maybe_progress.expect("actual owner");
        assert_eq!(first_idle.generations, 8);
        let endpoint = progress
            .maybe_processed_endpoint()
            .expect("processed prefix");
        assert_eq!(
            endpoint.height(),
            15,
            "startup plus exactly one eight-block ordinary turn"
        );
        assert_eq!(
            endpoint.block_hash(),
            history.snapshot.active_chain[15].block_hash
        );
        assert!(!progress.initially_synchronized());
        assert!(!config.sync.is_enabled());
        assert!(!config.inbound.enabled);
        assert_eq!(config.relay, RuntimeConfig::default().relay);
        assert_eq!(config.block_serving, RuntimeConfig::default().block_serving);
        assert!(
            opened
                .network
                .peer_manager_snapshot()
                .expect("peers")
                .peer_ids()
                .is_empty()
        );
        let network = opened
            .network
            .network_info()
            .expect("actual network policy");
        assert_eq!(
            network.local_services_bits & (1 << 6),
            0,
            "compact-filter peer service remains off"
        );
        assert_eq!(network.connected_peers, 0);
        assert!(
            store
                .has_block(history.snapshot.active_chain[0].block_hash)
                .expect("body intact")
        );
    }
}

#[test]
fn phase157_daemon_basic_without_datadir_refuses() {
    // Arrange
    let config = RuntimeConfig {
        block_filter_index: open_bitcoin_rpc::config::BasicFilterIndexSetting::Basic,
        ..RuntimeConfig::default()
    };
    // Act
    let result = open_runtime_store(&config)
        .and_then(|store| open_authoritative_network_runtime(&config, store));
    // Assert
    let error = match result {
        Err(error) => error,
        Ok(_) => panic!("BASIC requires datadir"),
    };
    assert!(error.to_string().contains("existing datadir"), "{error}");
}

#[test]
fn phase157_daemon_enabled_empty_store_refuses_without_index_mutation() {
    // Arrange
    let history = History::new(0);
    let config = history.config(Some("-blockfilterindex=basic"));
    let store = FjallNodeStore::open(&history.path).expect("empty store");
    // Act
    let result = open_authoritative_network_runtime(&config, Some(store.clone()));
    // Assert
    let error = match result {
        Err(error) => error,
        Ok(_) => panic!("empty BASIC requires validated genesis"),
    };
    assert!(
        error
            .to_string()
            .contains("validated genesis history required"),
        "{error}"
    );
    assert!(
        store
            .load_prune_protection()
            .expect("state")
            .maybe_owner()
            .is_none()
    );
    assert!(store.load_prune_locks().expect("locks").is_empty());
}

#[test]
fn phase157_daemon_explicit_zero_and_omitted_durable_trigger_disable_saved_owner() {
    for maybe_option in [Some("-blockfilterindex=0"), None] {
        // Arrange
        let history = History::new(40);
        let store = history.seed();
        let enabled = history.config(Some("-blockfilterindex=1"));
        let opened =
            open_authoritative_network_runtime(&enabled, Some(store.clone())).expect("enable");
        drop(opened);
        let checkpoint = store
            .load_prune_protection()
            .expect("saved prefix")
            .maybe_owner()
            .expect("owner")
            .checkpoint();
        assert!(!store.load_prune_locks().expect("saved owner").is_empty());
        drop(store);
        let mut config = history.config(maybe_option);
        if maybe_option.is_none() {
            config.prune_mode = PruneMode::ManualOnly;
        }
        // Act
        let store = open_runtime_store(&config)
            .expect("select")
            .expect("durable inspection");
        let opened = open_authoritative_network_runtime(&config, Some(store.clone()))
            .expect("disable")
            .expect_durable();
        // Assert
        assert!(
            store
                .load_prune_locks()
                .expect("released BASIC ownership")
                .is_empty()
        );
        let owner = store
            .load_prune_protection()
            .expect("retained prefix")
            .maybe_owner()
            .expect("owner");
        assert_eq!(owner.checkpoint(), checkpoint);
        assert!(matches!(owner.lifecycle(), IndexLifecycle::Disabled { .. }));
        drop(opened);
    }
}

#[test]
fn phase157_daemon_omitted_without_durable_trigger_preserves_saved_owner() {
    // Arrange
    let history = History::new(40);
    let store = history.seed();
    let enabled = history.config(Some("-blockfilterindex=1"));
    drop(open_authoritative_network_runtime(&enabled, Some(store.clone())).expect("enable"));
    let before = store.load_prune_locks().expect("saved owner");
    let cp = store.load_prune_protection().expect("saved prefix");
    let config = history.config(None);
    // Act
    let maybe_store = open_runtime_store(&config).expect("selection");
    assert!(maybe_store.is_none());
    let opened =
        open_authoritative_network_runtime(&config, maybe_store).expect("transient startup");
    // Assert
    assert!(matches!(opened, OpenedAuthoritativeRuntime::Transient(_)));
    assert_eq!(store.load_prune_locks().expect("owner intact"), before);
    assert_eq!(store.load_prune_protection().expect("prefix intact"), cp);
}

#[tokio::test]
async fn phase157_daemon_basic_keeps_actual_sync_and_inbound_workers_off() {
    // Arrange
    let history = History::new(40);
    drop(history.seed());
    let config = history.config(Some("-blockfilterindex=basic"));
    let store = open_runtime_store(&config)
        .expect("select")
        .expect("durable");
    let mut opened = open_authoritative_network_runtime(&config, Some(store.clone()))
        .expect("open")
        .expect_durable();
    let context = ManagedRpcContext::from_runtime_config_with_network_handle(
        &config,
        opened.network.clone(),
        Some(store),
    )
    .expect("context");
    let shared = Arc::new(tokio::sync::Mutex::new(context));
    // Act
    let maybe_sync = crate::start_daemon_sync_worker(
        &config,
        Arc::clone(&shared),
        opened.maybe_sync_runtime.take(),
    )
    .expect("sync selection");
    let listener = start_inbound_listener_for_runtime_with_context(&config, shared).await;
    // Assert
    assert!(maybe_sync.is_none());
    assert_eq!(listener.state, InboundListenerState::Disabled);
    assert!(listener.bound_endpoints.is_empty());
    assert!(listener.maybe_worker.is_none());
}
