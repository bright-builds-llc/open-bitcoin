// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp

use super::*;
use std::sync::atomic::{AtomicBool, Ordering};

struct ReentrantWake {
    handle: crate::ManagedNetworkHandle<crate::FjallChainstateStore, crate::FjallCoinsView>,
    store: FjallNodeStore,
    available: Arc<AtomicBool>,
}

struct PoisonWake {
    handle: crate::ManagedNetworkHandle<crate::FjallChainstateStore, crate::FjallCoinsView>,
    store: FjallNodeStore,
    count: Arc<WakeCount>,
    available: Arc<AtomicBool>,
}

impl Wake for PoisonWake {
    fn wake(self: Arc<Self>) {
        self.count.0.fetch_add(1, Ordering::SeqCst);
        let available = self.handle.basic_filter_read_locks_available_for_test()
            && self
                .store
                .basic_filter_publication_lock_available_for_test();
        self.available.store(available, Ordering::SeqCst);
        if available {
            assert_eq!(self.handle.basic_filter_waiter_count_for_test(), 1);
        }
    }
}

fn prepared_abort(runtime: &DurableSyncRuntime) -> crate::network::SnapshotWriteAbort {
    // A real prepared capability flows through the existing encoder-failure
    // adapter, which retains its exact abort on an injected dispatch failure.
    let prepared = runtime
        .network
        .prepare_mempool_snapshot_write(
            PolicyTime::from_unix_seconds(500_000),
            crate::network::CheckpointTrigger::Periodic,
        )
        .expect("genuine prepared snapshot");
    runtime
        .network
        .fail_next_checkpoint_abort_dispatch_for_test();
    let error = FjallNodeStore::execute_prepared_mempool_snapshot_write_with(
        &runtime.network,
        prepared,
        |_| {
            Err(crate::StorageError::Corruption {
                namespace: StorageNamespace::Mempool,
                detail: "injected encoder failure".to_owned(),
                action: crate::StorageRecoveryAction::Repair,
            })
        },
        |_, _| panic!("persistence cannot run after encoding failure"),
        || PolicyTime::from_unix_seconds(500_001),
    )
    .expect_err("real adapter retains exact abort");
    let (_, _, abort) = error
        .into_abort_dispatch_parts()
        .expect("retained abort carrier");
    abort
}

#[test]
fn phase159_basic_readiness_owner_direct_dispatch_poison_wakes_outside_guards() {
    for family in [
        "relay",
        "checkpoint-read",
        "checkpoint-complete",
        "checkpoint-abort",
        "stop",
    ] {
        // Arrange: genuine shared runtime and effect carriers precede poison.
        let history = ValidatedHistory::new("readiness-direct-dispatch-poison", false);
        let runtime = configured(&history);
        accept(&runtime, &history.blocks[2]);
        let maybe_receipt = if family == "checkpoint-complete" {
            let prepared = runtime
                .network
                .prepare_mempool_snapshot_write(
                    PolicyTime::from_unix_seconds(500_000),
                    crate::network::CheckpointTrigger::Periodic,
                )
                .expect("genuine prepared snapshot");
            Some(
                runtime
                    .store()
                    .execute_prepared_mempool_snapshot_write(&runtime.network, prepared, || {
                        PolicyTime::from_unix_seconds(500_001)
                    })
                    .expect("actual Sync persistence"),
            )
        } else {
            None
        };
        let maybe_abort = (family == "checkpoint-abort").then(|| prepared_abort(&runtime));
        let mut barrier = pending(&runtime, history.records[2].identity().block_hash());
        let count = Arc::new(WakeCount::default());
        let available = Arc::new(AtomicBool::new(false));
        let waker = Waker::from(Arc::new(PoisonWake {
            handle: runtime.network.clone(),
            store: runtime.store().clone(),
            count: Arc::clone(&count),
            available: Arc::clone(&available),
        }));
        assert!(
            Pin::new(&mut barrier)
                .poll(&mut Context::from_waker(&waker))
                .is_pending()
        );
        drop(waker);
        runtime.network.poison_for_test();
        // Act
        match family {
            "relay" => assert!(matches!(
                runtime.network.prepare_peer_relay_effect(1),
                Err(crate::ManagedNetworkAuthorityError::Poisoned)
            )),
            "checkpoint-read" => assert!(matches!(
                runtime
                    .network
                    .checkpoint_evidence(PolicyTime::from_unix_seconds(500_002), 60),
                Err(crate::ManagedNetworkAuthorityError::Poisoned)
            )),
            "checkpoint-complete" => {
                let receipt = maybe_receipt.expect("actual persisted receipt");
                let expected = receipt.duplicate_for_test();
                let error = runtime
                    .network
                    .complete_snapshot_write(receipt)
                    .expect_err("poisoned dispatch");
                assert_eq!(error.into_receipt(), expected);
            }
            "checkpoint-abort" => {
                let abort = maybe_abort.expect("actual retained abort");
                let expected = format!("{abort:?}");
                let error = runtime
                    .network
                    .abort_snapshot_write(abort)
                    .expect_err("poisoned dispatch");
                let (source, retained) = error.into_parts();
                assert!(matches!(
                    source,
                    crate::ManagedNetworkAuthorityError::Poisoned
                ));
                assert_eq!(format!("{retained:?}"), expected);
            }
            "stop" => assert!(matches!(
                runtime.network.stop_basic_filter_readiness(),
                Err(crate::ManagedNetworkAuthorityError::Poisoned)
            )),
            _ => panic!("unknown dispatcher family"),
        }
        // Assert: no second state read or Future poll is needed to earn the wake.
        assert_eq!(count.0.load(Ordering::SeqCst), 1, "{family}");
        assert!(
            available.load(Ordering::SeqCst),
            "callbacks held a guard: {family}"
        );
        assert!(matches!(
            poll(&mut barrier),
            Poll::Ready(Err(BasicFilterQueryError::Readiness(
                BasicFilterReadFailure::AuthorityUnavailable
            )))
        ));
        drop(runtime);
        history.cleanup();
    }
}

impl ReentrantWake {
    fn inspect(&self) {
        let available = self.handle.basic_filter_read_locks_available_for_test()
            && self
                .store
                .basic_filter_publication_lock_available_for_test();
        self.available.fetch_and(available, Ordering::SeqCst);
        if available {
            self.handle
                .maybe_basic_index_summary()
                .expect("reentrant summary read");
        }
    }
}

impl Wake for ReentrantWake {
    fn wake(self: Arc<Self>) {
        self.inspect();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.inspect();
    }
}

impl Drop for ReentrantWake {
    fn drop(&mut self) {
        self.inspect();
    }
}

fn waker(runtime: &DurableSyncRuntime, available: &Arc<AtomicBool>) -> Waker {
    Waker::from(Arc::new(ReentrantWake {
        handle: runtime.network.clone(),
        store: runtime.store().clone(),
        available: Arc::clone(available),
    }))
}

#[test]
fn phase159_basic_readiness_owner_waker_reenters_after_both_guards_release() {
    // Arrange
    let history = ValidatedHistory::new("readiness-reentrant", false);
    let runtime = configured(&history);
    accept(&runtime, &history.blocks[2]);
    let mut barrier = pending(&runtime, history.records[2].identity().block_hash());
    let available = Arc::new(AtomicBool::new(true));
    let waker = waker(&runtime, &available);
    assert!(
        Pin::new(&mut barrier)
            .poll(&mut Context::from_waker(&waker))
            .is_pending()
    );
    drop(waker);
    // Act
    runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("ordinary owner");
    // Assert
    ready(&mut barrier);
    assert!(available.load(Ordering::SeqCst));
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase159_basic_readiness_waker_replacement_and_cancel_drop_outside_registry() {
    // Arrange
    let history = ValidatedHistory::new("readiness-waker-drop", false);
    let runtime = configured(&history);
    accept(&runtime, &history.blocks[2]);
    let hash = history.records[2].identity().block_hash();
    let available = Arc::new(AtomicBool::new(true));
    // Act / Assert
    for cancel in [true, false] {
        let mut barrier = pending(&runtime, hash);
        let waker = waker(&runtime, &available);
        assert!(
            Pin::new(&mut barrier)
                .poll(&mut Context::from_waker(&waker))
                .is_pending()
        );
        drop(waker);
        if !cancel {
            assert!(poll(&mut barrier).is_pending());
        }
        drop(barrier);
        assert!(available.load(Ordering::SeqCst));
        assert_eq!(runtime.network.basic_filter_waiter_count_for_test(), 0);
    }
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase159_basic_readiness_owner_repoll_uses_latest_waker_once() {
    // Arrange
    let history = ValidatedHistory::new("readiness-repoll", false);
    let runtime = configured(&history);
    accept(&runtime, &history.blocks[2]);
    let mut barrier = pending(&runtime, history.records[2].identity().block_hash());
    let old = Arc::new(WakeCount::default());
    let new = Arc::new(WakeCount::default());
    let old_waker = Waker::from(Arc::clone(&old));
    let new_waker = Waker::from(Arc::clone(&new));
    assert!(
        Pin::new(&mut barrier)
            .poll(&mut Context::from_waker(&old_waker))
            .is_pending()
    );
    assert!(
        Pin::new(&mut barrier)
            .poll(&mut Context::from_waker(&new_waker))
            .is_pending()
    );
    // Act
    runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("owner");
    // Assert
    assert_eq!(old.0.load(std::sync::atomic::Ordering::SeqCst), 0);
    assert_eq!(new.0.load(std::sync::atomic::Ordering::SeqCst), 1);
    ready(&mut barrier);
    assert!(matches!(
        poll(&mut barrier),
        Poll::Ready(Err(BasicFilterQueryError::Readiness(
            BasicFilterReadFailure::AlreadyCompleted
        )))
    ));
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase159_basic_readiness_owner_never_connected_request_keeps_snapshot_after_acceptance() {
    use crate::chainstate::validation_history::tests::block;
    use open_bitcoin_core::chainstate::filter_index::catch_up::{BasicIndexTurnBudget, TurnWork};
    use open_bitcoin_core::chainstate::{Chainstate, CoinsCache, FlushMode, FlushPolicyTime};
    use open_bitcoin_network::{HeadersMessage, WireNetworkMessage};
    // Arrange: genuine genesis earns complete retained validation coverage.
    let dir = reserved_filter_path("readiness-never-connected");
    let store = FjallNodeStore::open(&dir).expect("fresh store");
    let state = Chainstate::from_coins_cache(
        CoinsCache::from_parent(store.coins_view()),
        Vec::new(),
        Default::default(),
        None,
    );
    let mut manager = crate::ManagedChainstate::from_recovered_chainstate(
        crate::FjallChainstateStore::from_store(store.clone()),
        state,
        crate::chainstate::FlushLifecycle::ready(
            FlushPolicyTime::from_unix_seconds(0),
            FlushPolicyTime::from_unix_seconds(0),
            0,
            false,
        ),
    )
    .expect("same-store authority");
    let genesis = block(BlockHash::default(), 0, 0x51);
    manager
        .connect_block(
            &genesis,
            1,
            open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
            Default::default(),
        )
        .expect("genuine genesis");
    store.save_block(&genesis, PersistMode::Sync).expect("body");
    manager
        .flush_with_mode(
            FlushMode::Always,
            FlushPolicyTime::from_unix_seconds(10),
            u64::MAX,
        )
        .expect("genuine coins fence");
    drop(manager);
    drop(store);
    let runtime = DurableSyncRuntime::open_configured(
        FjallNodeStore::open(&dir).expect("reopen"),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("runtime");
    let first = block(block_hash(&genesis.header), 1, 0x52);
    let second = block(block_hash(&first.header), 2, 0x53);
    accept(&runtime, &first);
    runtime
        .network
        .connect_outbound_peer(1, 10_000)
        .expect("peer");
    runtime
        .network
        .receive_sync_message(
            1,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![first.header.clone(), second.header.clone()],
            }),
            1_000_000,
            open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
            open_bitcoin_core::consensus::ConsensusParams {
                no_pow_retargeting: true,
                ..Default::default()
            },
        )
        .expect("genuine known-only header admission");
    let hash = block_hash(&second.header);
    assert_eq!(
        runtime
            .store()
            .validation_provenance(hash)
            .expect("before acceptance"),
        crate::storage::validation_history::ValidationProvenance::NeverConnected
    );
    let mut barrier = pending(&runtime, hash);
    // Act: the target was height 1, even though the requested block now connects.
    accept(&runtime, &second);
    let work = TurnWork {
        blocks: 1,
        ..injected_budget().normal()
    };
    runtime
        .network
        .drive_basic_filter_index_turn_with_budget(
            BasicIndexTurnBudget::new(work, work).expect("budget"),
        )
        .expect("original finite target");
    let result = runtime
        .network
        .complete_basic_filter_read(hash, ready(&mut barrier))
        .expect("snapshot classification");
    // Assert
    assert_eq!(
        runtime
            .store()
            .validation_provenance(hash)
            .expect("after acceptance"),
        crate::storage::validation_history::ValidationProvenance::ScriptsValid
    );
    assert!(matches!(
        result,
        BasicFilterQuery::Missing {
            provenance: crate::BasicBlockValidationProvenance::NeverConnected,
            initially_synchronized: true
        }
    ));
    drop(runtime);
    std::fs::remove_dir_all(dir).expect("cleanup");
}
