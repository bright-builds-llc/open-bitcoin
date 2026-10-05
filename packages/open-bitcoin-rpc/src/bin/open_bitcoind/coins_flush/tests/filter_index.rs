// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::super::*;
use crate::tests::filter_index::fixtures::{History, next_block, params};
use crate::{open_authoritative_network_runtime, open_runtime_store};
use open_bitcoin_node::core::chainstate::{
    CoinsView, FlushPolicyTime, IndexInputProtection, IndexPrefix,
};
use open_bitcoin_node::core::consensus::ScriptVerifyFlags;

#[test]
fn phase157_shutdown_retained_basic_error_settles_all_workers_without_clean_marker() {
    // Arrange
    let history = History::new(40);
    drop(history.seed());
    let config = history.config(Some("-blockfilterindex=1"));
    let store = open_runtime_store(&config)
        .expect("store")
        .expect("durable");
    let opened = open_authoritative_network_runtime(&config, Some(store.clone()))
        .expect("startup")
        .expect_durable();
    let handle = opened.network.clone();
    let successor = next_block(history.blocks.last(), 40);
    let accepted = handle
        .connect_local_block(&successor, ScriptVerifyFlags::P2SH, params())
        .expect("accepted successor whose body is not yet persisted");
    for height in [15, 23, 31, 39] {
        let progress = handle
            .drive_basic_filter_index_turn()
            .expect("catch up backlog")
            .maybe_progress
            .expect("Active owner");
        assert_eq!(
            progress
                .maybe_processed_endpoint()
                .expect("prefix")
                .height(),
            height
        );
        assert!(!progress.initially_synchronized());
    }
    let protected = store.load_prune_protection().expect("Active protection");
    let mut maybe_checkpoint =
        crate::checkpoint::start_mempool_checkpoint_worker(handle.clone(), Some(store.clone()));
    let mut maybe_retry = Some(crate::retry::start_initial_broadcast_retry_worker(
        handle.clone(),
    ));
    let mut retry_joined = false;
    let mut checkpoint_settled = false;
    let (shutdown_sender, shutdown_receiver) = mpsc::channel();
    let worker_handle = handle.clone();
    let worker_store = store.clone();
    let join_handle = thread::spawn(move || {
        let mut waits = 0;
        let result = coins_flush_worker_loop(worker_handle, worker_store, |_| {
            waits += 1;
            if waits == 1 {
                return CheckpointWait::Elapsed;
            }
            shutdown_receiver.recv().expect("actual shutdown signal");
            CheckpointWait::Shutdown
        });
        assert!(
            matches!(result, Err(CoinsFlushError::BasicFilter(_))),
            "retained real index failure after successful Always"
        );
        assert_eq!(waits, 2);
        result
    });
    let coins_worker = CoinsFlushWorker {
        shutdown_sender,
        join_handle,
    };

    // Act
    let result = crate::checkpoint::settle_daemon_shutdown(
        Ok(()),
        || Ok(()),
        || {
            let result = coins_worker.shutdown_always();
            assert!(
                matches!(result, Err(DaemonCheckpointError::ShutdownCheckpoint)),
                "actual coins worker joined and retained the index failure"
            );
            assert_eq!(
                store.coins_view().best_block().expect("Always fence"),
                Some(accepted.block_hash)
            );
            result.map_err(Into::into)
        },
        || {
            maybe_retry.take().expect("retry worker").shutdown()?;
            retry_joined = true;
            Ok(())
        },
        || {
            maybe_checkpoint
                .take()
                .expect("checkpoint worker")
                .shutdown_settle()?;
            checkpoint_settled = true;
            Ok(())
        },
        || store.mark_clean_shutdown(open_bitcoin_node::PersistMode::Sync),
    );
    let evidence = handle
        .checkpoint_evidence(open_bitcoin_mempool::PolicyTime::new(301), 1)
        .expect("checkpoint evidence");
    let maybe_metadata = store.load_runtime_metadata().expect("metadata");
    // Clean up any workers skipped by a regressed coordinator before asserting.
    if let Some(worker) = maybe_retry {
        worker.shutdown().expect("test cleanup retry");
    }
    if let Some(worker) = maybe_checkpoint {
        worker.shutdown_settle().expect("test cleanup checkpoint");
    }

    // Assert
    assert!(
        retry_joined,
        "outer shutdown must join retry despite retained BASIC error"
    );
    assert!(
        checkpoint_settled,
        "outer shutdown must settle and join final mempool checkpoint"
    );
    assert_eq!(
        evidence.maybe_last_durable_generation,
        Some(evidence.current_generation)
    );
    assert!(maybe_metadata.is_none_or(|metadata| !metadata.last_clean_shutdown));
    assert!(matches!(
        result
            .expect_err("failure remains visible")
            .downcast_ref::<DaemonCheckpointError>(),
        Some(DaemonCheckpointError::ShutdownCheckpoint)
    ));
    assert_eq!(
        store
            .load_prune_protection()
            .expect("Active protection survives"),
        protected
    );
    assert!(
        !store
            .load_prune_locks()
            .expect("required inputs protected")
            .is_empty()
    );
}

#[test]
fn phase157_idle_always_shutdown_advances_real_accepted_fence_before_reopen() {
    // Arrange
    let history = History::new(40);
    drop(history.seed());
    let config = history.config(Some("-blockfilterindex=1"));
    let store = open_runtime_store(&config)
        .expect("store")
        .expect("durable");
    let opened = open_authoritative_network_runtime(&config, Some(store.clone()))
        .expect("startup")
        .expect_durable();
    let handle = opened.network.clone();
    handle
        .set_coins_next_write(FlushPolicyTime::new(u64::MAX))
        .expect("future deadline");
    let successor = next_block(history.blocks.last(), 40);
    let accepted = handle
        .connect_local_block(&successor, ScriptVerifyFlags::P2SH, params())
        .expect("genuine unflushed acceptance");
    store
        .save_block(&successor, open_bitcoin_node::PersistMode::Sync)
        .expect("ordinary caller body");
    assert_eq!(
        store.coins_view().best_block().expect("old B"),
        Some(history.snapshot.active_chain[39].block_hash)
    );
    // Act
    coins_flush_worker_loop(handle.clone(), store.clone(), |_| CheckpointWait::Shutdown)
        .expect("Always after stop");
    // Assert
    assert_eq!(
        store.coins_view().best_block().expect("earned Always B"),
        Some(accepted.block_hash)
    );
    assert!(matches!(
        store
            .load_prune_protection()
            .expect("no shutdown index turn")
            .maybe_owner()
            .expect("saved Active")
            .checkpoint()
            .checkpoint()
            .prefix(),
        IndexPrefix::Empty
    ));
    drop(handle);
    drop(opened);
    drop(store);
    let store = open_runtime_store(&config)
        .expect("closed Fjall reopen")
        .expect("durable");
    let opened = open_authoritative_network_runtime(&config, Some(store.clone()))
        .expect("actual configured reopen")
        .expect_durable();
    assert_eq!(
        store
            .coins_view()
            .best_block()
            .expect("durable accepted fence"),
        Some(accepted.block_hash)
    );
    let outcome = opened
        .network
        .drive_basic_filter_index_turn()
        .expect("ordinary public turn");
    assert_eq!(
        outcome
            .maybe_progress
            .expect("owner")
            .accepted_target()
            .height(),
        40
    );
    assert!(
        !outcome
            .maybe_progress
            .expect("owner")
            .initially_synchronized()
    );
}

#[test]
fn phase157_idle_injected_elapsed_ticks_advance_without_receives_and_yield() {
    // Arrange
    let history = History::new(40);
    drop(history.seed());
    let config = history.config(Some("-blockfilterindex=1"));
    let store = open_runtime_store(&config)
        .expect("store")
        .expect("durable");
    let opened = open_authoritative_network_runtime(&config, Some(store.clone()))
        .expect("startup")
        .expect_durable();
    let handle = opened.network.clone();
    handle
        .set_coins_next_write(FlushPolicyTime::new(u64::MAX))
        .expect("future deadline");
    let mut waits = 0;
    // Act
    coins_flush_worker_loop(handle.clone(), store.clone(), |duration| {
        assert_eq!(duration, Duration::from_secs(1));
        waits += 1;
        assert_eq!(
            handle
                .maybe_chain_tip()
                .expect("authority usable between turns")
                .expect("tip")
                .height,
            39
        );
        assert!(
            handle
                .peer_manager_snapshot()
                .expect("zero peers")
                .peer_ids()
                .is_empty()
        );
        if waits <= 2 {
            CheckpointWait::Elapsed
        } else {
            CheckpointWait::Shutdown
        }
    })
    .expect("ordinary worker settles");
    let outcome = handle
        .drive_basic_filter_index_turn()
        .expect("observe next public turn");
    // Assert
    assert_eq!(waits, 3);
    assert_eq!(outcome.generations, 8);
    let progress = outcome.maybe_progress.expect("owner");
    assert_eq!(
        progress
            .maybe_processed_endpoint()
            .expect("prefix")
            .height(),
        31,
        "startup 8 + two ordinary elapsed 8 + observation 8; shutdown adds none"
    );
    assert!(!progress.initially_synchronized());
    assert!(!store.load_have_pruned().expect("no deletion"));
}

#[test]
fn phase157_idle_shutdown_performs_always_without_an_index_turn() {
    // Arrange
    let history = History::new(40);
    drop(history.seed());
    let config = history.config(Some("-blockfilterindex=basic"));
    let store = open_runtime_store(&config)
        .expect("store")
        .expect("durable");
    let opened = open_authoritative_network_runtime(&config, Some(store.clone()))
        .expect("startup")
        .expect_durable();
    let handle = opened.network.clone();
    let owner = store.load_prune_protection().expect("saved owner");
    // Act
    coins_flush_worker_loop(handle.clone(), store.clone(), |_| CheckpointWait::Shutdown)
        .expect("Always settles");
    let outcome = handle
        .drive_basic_filter_index_turn()
        .expect("one subsequent observation turn");
    // Assert
    assert_eq!(
        outcome
            .maybe_progress
            .expect("owner")
            .maybe_processed_endpoint()
            .expect("prefix")
            .height(),
        15
    );
    assert_eq!(
        store
            .load_prune_protection()
            .expect("conservative Active survives"),
        owner
    );
    assert_eq!(
        store.coins_view().best_block().expect("Always B"),
        Some(history.snapshot.active_chain[39].block_hash)
    );
}

#[test]
fn phase157_idle_ordered_turns_extend_target_then_genuine_periodic_fence_releases() {
    // Arrange
    let history = History::new(40);
    drop(history.seed());
    let config = history.config(Some("-blockfilterindex=1"));
    let store = open_runtime_store(&config)
        .expect("store")
        .expect("durable");
    let opened = open_authoritative_network_runtime(&config, Some(store.clone()))
        .expect("startup")
        .expect_durable();
    let handle = opened.network.clone();
    handle
        .set_coins_next_write(FlushPolicyTime::new(u64::MAX))
        .expect("future ordinary deadline");
    let successor = next_block(history.blocks.last(), 40);
    let accepted = handle
        .connect_local_block(&successor, ScriptVerifyFlags::P2SH, params())
        .expect("genuine accepted connect while behind");
    store
        .save_block(&successor, open_bitcoin_node::PersistMode::Sync)
        .expect("normal caller's accepted body persistence");
    // Act / Assert: each ordinary elapsed body returns after one bounded turn.
    let mut prefixes = Vec::new();
    for (height, blocks) in [(15, 8), (23, 8), (31, 8), (39, 8), (40, 1)] {
        let outcome = maybe_drive_elapsed(&handle, &store)
            .expect("ordinary elapsed")
            .expect("durable driver");
        let progress = outcome.maybe_progress.expect("owner");
        prefixes.push(
            progress
                .maybe_processed_endpoint()
                .expect("ordered prefix")
                .height(),
        );
        assert_eq!(prefixes.last(), Some(&height));
        assert_eq!(outcome.generations, blocks);
        assert_eq!(outcome.work.blocks, blocks);
        assert!(outcome.body_reads <= 8);
        assert!(outcome.persistence_batches <= 1);
        assert!(outcome.work.record_operations <= 512);
        assert!(outcome.work.projection_operations <= 256);
        assert!(outcome.work.checkpoint_operations <= 1_000_000);
        assert_eq!(progress.accepted_target().height(), 40);
        assert_eq!(progress.initially_synchronized(), height == 40);
        if height < 39 {
            assert!(progress.maybe_safe_durable_endpoint().is_none());
        } else {
            assert_eq!(
                progress
                    .maybe_safe_durable_endpoint()
                    .expect("exact old durable tip earned")
                    .height(),
                39,
                "unflushed B40 cannot advance safe progress beyond genuine B39"
            );
        }
        assert!(
            handle
                .peer_manager_snapshot()
                .expect("no receive prerequisite")
                .peer_ids()
                .is_empty()
        );
        assert_eq!(
            handle.maybe_chain_tip().expect("usable between turns"),
            Some(accepted.clone())
        );
        assert!(
            !store
                .load_prune_locks()
                .expect("retain conservative input protection")
                .is_empty()
        );
    }
    let checkpoint = flush_cycle(
        &handle,
        FlushMode::Periodic,
        FlushPolicyTime::new(u64::MAX),
        u64::MAX,
    )
    .expect("genuine due Periodic fence");
    assert!(checkpoint.wrote_coins);
    assert!(checkpoint.deleted_block_hashes.is_empty());
    let release = maybe_drive_elapsed(&handle, &store)
        .expect("next ordinary elapsed")
        .expect("durable");
    assert_eq!(release.generations, 0);
    assert_eq!(release.body_reads, 0);
    assert_eq!(
        release
            .maybe_progress
            .expect("owner")
            .maybe_safe_durable_endpoint()
            .expect("earned safe tip")
            .block_hash(),
        accepted.block_hash
    );
    assert_eq!(
        store.load_prune_locks().expect("earned historical release"),
        vec![
            IndexInputProtection::FromHeight(41)
                .maybe_prune_lock()
                .expect("protect next required input")
        ]
    );
    let owner = store
        .load_prune_protection()
        .expect("checkpoint")
        .maybe_owner()
        .expect("owner");
    assert!(
        matches!(owner.checkpoint().checkpoint().prefix(), IndexPrefix::Committed(id) if id.height() == 40 && id.block_hash() == accepted.block_hash)
    );
    coins_flush_worker_loop(handle.clone(), store.clone(), |_| CheckpointWait::Shutdown)
        .expect("same owner's Always shutdown");
    drop(handle);
    drop(opened);
    drop(store);
    // Actual closed Fjall/configured daemon reopen grants no invented extra work.
    let store = open_runtime_store(&config)
        .expect("reopen store")
        .expect("durable");
    let reopened = open_authoritative_network_runtime(&config, Some(store.clone()))
        .expect("actual reopen")
        .expect_durable();
    assert_eq!(
        store
            .load_prune_protection()
            .expect("durable recovered checkpoint")
            .maybe_owner(),
        Some(owner)
    );
    let idle = maybe_drive_elapsed(&reopened.network, &store)
        .expect("reopened idle")
        .expect("durable");
    assert_eq!(idle.generations, 0);
    assert!(idle.maybe_progress.expect("owner").initially_synchronized());
}

#[test]
fn phase157_idle_explicit_disable_refuses_stale_turn_and_keeps_always_cleanup() {
    // Arrange
    let history = History::new(40);
    drop(history.seed());
    let config = history.config(Some("-blockfilterindex=1"));
    let store = open_runtime_store(&config)
        .expect("store")
        .expect("durable");
    let opened = open_authoritative_network_runtime(&config, Some(store.clone()))
        .expect("startup")
        .expect_durable();
    let handle = opened.network.clone();
    handle
        .disable_basic_filter_index()
        .expect("invalidate before release");
    let before = store.load_prune_protection().expect("disabled artifacts");
    let mut waits = 0;
    // Act
    let result = coins_flush_worker_loop(handle.clone(), store.clone(), |_| {
        waits += 1;
        if waits == 1 {
            CheckpointWait::Elapsed
        } else {
            CheckpointWait::Shutdown
        }
    });
    // Assert
    assert!(
        matches!(result, Err(CoinsFlushError::BasicFilter(_))),
        "index failure retained through Always"
    );
    assert_eq!(waits, 2, "failure still yields and obeys shutdown");
    assert_eq!(
        store.load_prune_protection().expect("no stale publication"),
        before
    );
    assert!(
        store
            .load_prune_locks()
            .expect("release stays disabled")
            .is_empty()
    );
    assert_eq!(
        store.coins_view().best_block().expect("Always still runs"),
        Some(history.snapshot.active_chain[39].block_hash)
    );
    assert!(handle.drive_basic_filter_index_turn().is_err());
}

#[test]
fn phase157_idle_offline_activation_starts_existing_worker_and_joins() {
    // Arrange
    let history = History::new(40);
    drop(history.seed());
    let config = history.config(Some("-blockfilterindex=basic"));
    let store = open_runtime_store(&config)
        .expect("store")
        .expect("durable");
    let opened = open_authoritative_network_runtime(&config, Some(store.clone()))
        .expect("startup")
        .expect_durable();
    // Act
    let worker = start_coins_flush_worker(opened.network.clone(), Some(store.clone()))
        .expect("index-only durable worker");
    worker.shutdown_always().expect("signal and actual join");
    // Assert
    assert!(!config.sync.is_enabled());
    assert!(!config.inbound.enabled);
    assert!(
        !store
            .load_prune_locks()
            .expect("saved protection survives join")
            .is_empty()
    );
    assert!(
        opened
            .network
            .peer_manager_snapshot()
            .expect("zero peers")
            .peer_ids()
            .is_empty()
    );
}

#[test]
fn phase157_idle_transient_adapter_has_no_index_and_starts_no_durable_worker() {
    // Arrange
    let config = open_bitcoin_rpc::config::RuntimeConfig::default();
    let opened = open_authoritative_network_runtime(&config, None).expect("transient");
    let crate::OpenedAuthoritativeRuntime::Transient(opened) = opened else {
        panic!("default stays transient")
    };
    // Act / Assert
    assert!(
        opened
            .network
            .maybe_drive_index_turn()
            .expect("typed no-index")
            .is_none()
    );
    assert!(start_coins_flush_worker(opened.network, None).is_none());
}
