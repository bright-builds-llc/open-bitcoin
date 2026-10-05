// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use crate::chainstate::PruneProtectionSnapshot;
use open_bitcoin_core::chainstate::{
    FilterCheckpoint, IndexInputProtection, IndexPrefix,
    filter_index::lifecycle::{
        EffectiveIndexOwnership, IndexCheckpointIdentity, IndexGeneration, IndexLifecycle,
    },
};

fn protected(generation: u64, disabled: bool, fence_nonce: u32) -> PruneProtectionSnapshot {
    let protection = IndexInputProtection::FromHeight(0);
    let lock = protection.maybe_prune_lock().expect("reserved lock");
    let lifecycle = if disabled {
        IndexLifecycle::Disabled {
            generation: IndexGeneration::new(generation),
        }
    } else {
        IndexLifecycle::Active {
            generation: IndexGeneration::new(generation),
        }
    };
    let maybe_owner = EffectiveIndexOwnership::maybe_from_artifacts(
        Some(lifecycle),
        Some(IndexCheckpointIdentity::new(
            FilterCheckpoint::new(IndexPrefix::Empty),
            1_001,
            position(fence_nonce).block_hash,
        )),
        Some(protection),
        Some(&lock),
        false,
        false,
    )
    .expect("validated policy facts");
    PruneProtectionSnapshot {
        maybe_owner,
        locks: vec![lock],
    }
}

#[test]
fn automatic_prune_same_second_ordinary_lock_changes_invalidate_timer() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET);
    tick(&handle, FlushMode::Periodic, 100).expect("first");
    let clone = handle.clone();
    // Act
    clone
        .replace_prune_lock(PruneLockInfo {
            name: "wallet".into(),
            height_first: 20,
            height_last: 30,
        })
        .expect("strengthen");
    let strengthened = tick(&handle, FlushMode::Periodic, 100).expect("changed");
    clone.clear_prune_lock("wallet").expect("release");
    let released = tick(&handle, FlushMode::Periodic, 100).expect("released");
    // Assert
    assert_eq!(facts.lock().expect("facts").scans, 3);
    assert!(!strengthened.wrote_coins);
    assert!(!released.wrote_coins);
}

#[test]
fn automatic_prune_owner_generation_lifecycle_and_fence_changes_invalidate_both_gates() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET);
    facts.lock().expect("facts").maybe_protection = Some(protected(0, false, 1_001));
    tick(&handle, FlushMode::Periodic, 100).expect("first");
    // Act / Assert: same range, unchanged payload revision and same second.
    for (index, snapshot) in [
        protected(1, false, 1_001),
        protected(2, true, 1_001),
        protected(3, false, 1_001),
        protected(3, false, 1_002),
    ]
    .into_iter()
    .enumerate()
    {
        facts.lock().expect("facts").maybe_protection = Some(snapshot);
        tick(&handle.clone(), FlushMode::Periodic, 100).expect("fresh ownership");
        assert_eq!(facts.lock().expect("facts").scans, index + 2);
    }
    tick(&handle, FlushMode::Periodic, 200).expect("unchanged");
    assert_eq!(facts.lock().expect("facts").scans, 5);
}

#[test]
fn automatic_prune_lock_order_only_preserves_idle_and_timer_coalescing() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET);
    facts.lock().expect("facts").locks = vec![
        PruneLockInfo {
            name: "a".into(),
            height_first: 20,
            height_last: 30,
        },
        PruneLockInfo {
            name: "b".into(),
            height_first: 40,
            height_last: 50,
        },
    ];
    tick(&handle, FlushMode::Periodic, 100).expect("first");
    // Act
    facts.lock().expect("facts").locks.reverse();
    tick(&handle, FlushMode::Periodic, 200).expect("equivalent");
    // Assert
    assert_eq!(facts.lock().expect("facts").scans, 1);
}

#[test]
fn automatic_prune_protection_and_accounting_errors_clear_timer_and_reuse() {
    // Arrange / Act
    for protection_error in [false, true] {
        let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET);
        tick(&handle, FlushMode::Periodic, 100).expect("first");
        {
            let mut facts = facts.lock().expect("facts");
            facts.fail_protection = protection_error;
            facts.fail_accounting = !protection_error;
        }
        assert!(tick(&handle, FlushMode::Always, 100).is_err());
        let scans = facts.lock().expect("facts").scans;
        {
            let mut facts = facts.lock().expect("facts");
            facts.fail_protection = false;
            facts.fail_accounting = false;
        }
        tick(&handle, FlushMode::Periodic, 100).expect("recovery remeasures");
        // Assert
        assert_eq!(facts.lock().expect("facts").scans, scans + 1);
        assert!(facts.lock().expect("facts").deletes.is_empty());
    }
}

#[test]
fn automatic_prune_direct_low_inputs_are_not_candidate_budget_or_forced_checkpoint() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET + 150);
    facts.lock().expect("facts").maybe_protection = Some(protected(0, false, 1_001));
    facts.lock().expect("facts").sizes = BTreeMap::from([(0, 100), (1, 100), (10, 100)]);
    let store = TestStore {
        memory: MemoryChainstateStore::default(),
        facts: Arc::clone(&facts),
    };
    let mut state = super::super::AutomaticPruneState::default();
    state.set_network(crate::SyncNetwork::Regtest);
    // Act
    let (plan, _, _) = super::super::prepare(
        &store,
        &[position(0), position(1), position(10), position(1_001)],
        PruneMode::Automatic { target_mib: 550 },
        FlushMode::Periodic,
        FlushPolicyTime::new(100),
        &mut state,
    )
    .expect("protected plan");
    let outcome = tick(&handle, FlushMode::Periodic, 100).expect("protected flush");
    // Assert
    assert!(plan.heights.is_empty());
    assert!(!outcome.wrote_coins);
    assert_eq!(facts.lock().expect("facts").usage, TARGET + 150);
    assert!(facts.lock().expect("facts").deletes.is_empty());
}

#[test]
fn automatic_prune_unchanged_always_still_measures_and_network_reset_clears_timer() {
    // Arrange
    let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, TARGET);
    tick(&handle, FlushMode::Periodic, 100).expect("first");
    // Act
    tick(&handle, FlushMode::Always, 100).expect("always");
    handle
        .set_prune_network(crate::SyncNetwork::Regtest)
        .expect("reset");
    tick(&handle, FlushMode::Periodic, 100).expect("reset scan");
    // Assert
    assert_eq!(facts.lock().expect("facts").scans, 3);
}

struct RealProtectionFixture {
    temp: std::path::PathBuf,
    runtime: crate::DurableSyncRuntime,
    handle: ManagedNetworkHandle<TestStore>,
    facts: Arc<Mutex<Facts>>,
    positions: Vec<ChainPosition>,
    records: Vec<crate::storage::filter_index::StoredFilterRecord>,
}

impl RealProtectionFixture {
    fn new(name: &str) -> Self {
        use crate::{FjallNodeStore, PersistMode};
        use open_bitcoin_core::{
            chainstate::{BasicFilterInputs, HistoricalBlockUndo},
            consensus::block_merkle_root,
            primitives::*,
        };
        // Reserve exclusive ownership even if concurrent test clocks are equal.
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let mut suffix = 0_u64;
        let temp = loop {
            let candidate = std::env::temp_dir().join(format!(
                "automatic-protection-{name}-{}-{nanos}-{suffix}",
                std::process::id()
            ));
            match std::fs::create_dir(&candidate) {
                Ok(()) => break candidate,
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                    suffix = suffix.checked_add(1).expect("fixture suffix")
                }
                Err(error) => panic!("reserve fixture: {error}"),
            }
        };
        let store = FjallNodeStore::open(&temp).expect("store");
        let mut positions = Vec::new();
        let mut records: Vec<crate::storage::filter_index::StoredFilterRecord> = Vec::new();
        let mut parent = BlockHash::default();
        // Dense metadata and actual coinbase inputs; this is deletion-order evidence,
        // not a consensus-validated sync or a legal-target automatic-deletion fixture.
        for height in 0..=1_001 {
            let transactions = vec![Transaction {
                version: 1,
                inputs: vec![TransactionInput {
                    previous_output: OutPoint::null(),
                    script_sig: ScriptBuf::from_bytes(vec![1, height as u8]).expect("script"),
                    sequence: u32::MAX,
                    witness: ScriptWitness::default(),
                }],
                outputs: vec![TransactionOutput {
                    value: Amount::from_sats(5_000_000_000).expect("amount"),
                    script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("script"),
                }],
                lock_time: 0,
            }];
            let block = Block {
                header: BlockHeader {
                    version: 1,
                    previous_block_hash: parent,
                    merkle_root: block_merkle_root(&transactions).expect("merkle").0,
                    time: 1_000 + height,
                    bits: 0x207fffff,
                    nonce: height,
                },
                transactions,
            };
            let position = ChainPosition::new(
                block.header.clone(),
                height,
                u128::from(height) + 1,
                i64::from(block.header.time),
            );
            let undo = BlockUndo::default();
            store
                .save_block(&block, PersistMode::Buffered)
                .expect("body");
            if height != 0 {
                store
                    .save_undo(position.block_hash, &undo, PersistMode::Buffered)
                    .expect("undo");
            }
            if height <= 16 {
                let maybe_history = (height != 0).then_some(HistoricalBlockUndo {
                    block_hash: position.block_hash,
                    undo: &undo,
                });
                let inputs = BasicFilterInputs::from_historical(&block, &position, maybe_history)
                    .expect("historical inputs");
                records.push(
                    crate::storage::filter_index::StoredFilterRecord::generate(
                        &inputs,
                        &position,
                        records.last().map(|record| record.identity()).as_ref(),
                    )
                    .expect("record"),
                );
            }
            parent = position.block_hash;
            positions.push(position);
        }
        let snapshot = ChainstateSnapshot::new(positions.clone(), HashMap::new(), HashMap::new());
        store
            .seed_coins_from_snapshot(&snapshot)
            .expect("durable coins/metadata");
        let runtime = crate::DurableSyncRuntime::open(
            store.clone(),
            crate::SyncRuntimeConfig {
                network: crate::SyncNetwork::Regtest,
                dns_seeds: Vec::new(),
                ..Default::default()
            },
        )
        .expect("production runtime");
        let (handle, facts) = fixture(PruneMode::Automatic { target_mib: 550 }, 0);
        handle
            .mutate(|network| {
                network.chainstate_mut().install_chainstate_for_test(
                    open_bitcoin_core::chainstate::Chainstate::from_snapshot(snapshot),
                )
            })
            .expect("same dense active metadata");
        facts.lock().expect("facts").maybe_real_store = Some(store);
        Self {
            temp,
            runtime,
            handle,
            facts,
            positions,
            records,
        }
    }

    fn fence(&self) -> open_bitcoin_core::chainstate::VerifiedChainstateFence<'_> {
        open_bitcoin_core::chainstate::VerifiedChainstateFence::new(
            Some(self.positions.last().expect("tip").block_hash),
            Some(&self.positions),
        )
        .expect("durable fence")
    }

    fn publish(&self) {
        let store = self.runtime.store().clone();
        let checkpoint = FilterCheckpoint::new(IndexPrefix::Committed(
            self.records.last().expect("endpoint").identity(),
        ));
        let work = store
            .maybe_basic_filter_work(&self.fence())
            .expect("work")
            .expect("active");
        store
            .publish_basic_filter_checkpoint(
                &work,
                &self.fence(),
                checkpoint,
                checkpoint.input_protection(),
                &self.records,
            )
            .expect("fenced publication");
    }

    fn scans(&self) -> usize {
        self.facts.lock().expect("facts").scans
    }

    fn cleanup(self) {
        let temp = self.temp.clone();
        drop(self);
        std::fs::remove_dir_all(temp).expect("closed fixture cleanup");
    }
}

#[test]
fn automatic_prune_real_clone_lifecycle_and_checkpoint_changes_are_visible_same_second() {
    // Arrange
    let fixture = RealProtectionFixture::new("lifecycle");
    let before = fixture
        .runtime
        .store()
        .retained_payload_usage(&fixture.positions)
        .expect("actual bytes");
    tick(&fixture.handle, FlushMode::Periodic, 100).expect("absent pass");
    let clone = fixture.runtime.store().clone();
    clone
        .initialize_basic_filter_state(&fixture.fence())
        .expect("clone acquisition");
    tick(&fixture.handle, FlushMode::Periodic, 100).expect("acquired");
    fixture.publish();
    tick(&fixture.handle, FlushMode::Periodic, 100).expect("released prefix");
    assert_eq!(fixture.scans(), 3);

    // Act: external clone publication and trusted production host calls share no callback.
    clone
        .disable_basic_filter_index()
        .expect("external clone disable");
    tick(&fixture.handle, FlushMode::Periodic, 100).expect("disabled");
    fixture
        .runtime
        .network_handle()
        .enable_basic_filter_index()
        .expect("host re-enable");
    let execution = tick(&fixture.handle.clone(), FlushMode::Periodic, 100).expect("re-enabled");
    tick(&fixture.handle, FlushMode::Periodic, 200).expect("unchanged coalesces");

    // Assert
    assert_eq!(fixture.scans(), 5);
    assert!(!execution.wrote_coins);
    assert!(execution.deleted_block_hashes.is_empty());
    assert_eq!(
        clone
            .retained_payload_usage(&fixture.positions)
            .expect("after")
            .current_usage_bytes,
        before.current_usage_bytes
    );
    for height in [0, 1, 20] {
        assert!(
            clone
                .has_block(fixture.positions[height].block_hash)
                .expect("body survivor")
        );
        if height != 0 {
            assert!(
                clone
                    .has_undo(fixture.positions[height].block_hash)
                    .expect("undo survivor")
            );
        }
    }
    assert!(!clone.load_have_pruned().expect("no unearned marker"));
    assert_eq!(
        clone
            .load_operator_support_counts()
            .expect("no receipts")
            .pruned_height_count,
        0
    );
    drop(clone);
    fixture.cleanup();
}

#[test]
fn automatic_prune_real_stalled_and_failed_publication_keeps_required_mates_and_refuses_reuse() {
    use crate::storage::fjall_store::filters::FilterPublicationFault;
    // Arrange
    let fixture = RealProtectionFixture::new("failed-publication");
    let store = fixture.runtime.store().clone();
    store
        .initialize_basic_filter_state(&fixture.fence())
        .expect("index stalls at empty");
    tick(&fixture.handle, FlushMode::Periodic, 100).expect("protected pass");
    let usage = store
        .retained_payload_usage(&fixture.positions)
        .expect("real bytes");
    let checkpoint = FilterCheckpoint::new(IndexPrefix::Committed(
        fixture.records.last().expect("record").identity(),
    ));
    let work = store
        .maybe_basic_filter_work(&fixture.fence())
        .expect("work")
        .expect("active");
    store.set_basic_filter_fault(FilterPublicationFault::BeforeCheckpoint);

    // Act
    assert!(
        store
            .publish_basic_filter_checkpoint(
                &work,
                &fixture.fence(),
                checkpoint,
                checkpoint.input_protection(),
                &fixture.records
            )
            .is_err()
    );
    let result = tick(&fixture.handle, FlushMode::Periodic, 100);

    // Assert
    assert!(result.is_err());
    assert_eq!(fixture.scans(), 1);
    assert_eq!(
        store
            .retained_payload_usage(&fixture.positions)
            .expect("retained bytes"),
        usage
    );
    for height in [0, 1, 20] {
        assert!(
            store
                .has_block(fixture.positions[height].block_hash)
                .expect("required body")
        );
    }
    assert!(
        store
            .has_undo(fixture.positions[1].block_hash)
            .expect("required undo")
    );
    assert_eq!(store.maybe_prune_intent().expect("no intent"), None);
    assert!(!store.load_have_pruned().expect("no marker"));
    let temp = fixture.temp.clone();
    let positions = fixture.positions.clone();
    drop(work);
    drop(store);
    drop(fixture);
    let reopened = crate::DurableSyncRuntime::open(
        crate::FjallNodeStore::open(&temp).expect("real reopen"),
        crate::SyncRuntimeConfig::default(),
    )
    .expect("protected production recovery");
    assert!(
        reopened
            .store()
            .load_prune_protection()
            .expect("recovered")
            .protects_height(1)
    );
    assert_eq!(
        reopened
            .store()
            .retained_payload_usage(&positions)
            .expect("reopened bytes")
            .current_usage_bytes,
        usage.current_usage_bytes
    );
    drop(reopened);
    std::fs::remove_dir_all(temp).expect("cleanup");
}

#[test]
fn automatic_prune_real_reenable_blocks_preplanned_pair_then_fenced_prefix_earns_safe_receipt() {
    // Arrange
    let fixture = RealProtectionFixture::new("preplanned");
    let stale = open_bitcoin_core::chainstate::PrunePlan { heights: vec![20] };
    let store = fixture.runtime.store().clone();
    store
        .initialize_basic_filter_state(&fixture.fence())
        .expect("initial protection");
    store.disable_basic_filter_index().expect("clone disable");
    let host = fixture.runtime.network_handle();
    host.enable_basic_filter_index()
        .expect("protection before stale application");
    tick(&fixture.handle, FlushMode::Periodic, 100).expect("re-enabled measurement");

    // Act
    let retained = host
        .flush_applying_prune_plan(
            FlushMode::IfNeeded,
            FlushPolicyTime::new(100),
            u64::MAX,
            &stale,
            &[],
        )
        .expect("stale candidate skips");
    fixture.publish();
    tick(&fixture.handle.clone(), FlushMode::Periodic, 100)
        .expect("same-second release measurement");
    let deleted = host
        .flush_applying_prune_plan(
            FlushMode::Always,
            FlushPolicyTime::new(100),
            u64::MAX,
            &open_bitcoin_core::chainstate::PrunePlan { heights: vec![2] },
            &[],
        )
        .expect("safe higher prefix pair");

    // Assert
    assert!(retained.deleted_block_hashes.is_empty());
    assert_eq!(fixture.scans(), 2);
    assert_eq!(
        deleted.deleted_block_hashes,
        vec![fixture.positions[2].block_hash]
    );
    assert!(
        !store
            .has_block(fixture.positions[2].block_hash)
            .expect("deleted body")
    );
    assert!(
        !store
            .has_undo(fixture.positions[2].block_hash)
            .expect("deleted undo")
    );
    assert!(
        store
            .has_block(fixture.positions[20].block_hash)
            .expect("required body")
    );
    assert!(
        store
            .has_undo(fixture.positions[20].block_hash)
            .expect("required undo")
    );
    assert_eq!(
        store
            .load_operator_support_counts()
            .expect("receipt")
            .pruned_height_count,
        1
    );
    let temp = fixture.temp.clone();
    let required_hash = fixture.positions[20].block_hash;
    let deleted_hash = fixture.positions[2].block_hash;
    drop(host);
    drop(store);
    drop(fixture);
    let reopened = crate::DurableSyncRuntime::open(
        crate::FjallNodeStore::open(&temp).expect("real reopen"),
        crate::SyncRuntimeConfig::default(),
    )
    .expect("production recovery");
    assert!(
        !reopened
            .store()
            .has_block(deleted_hash)
            .expect("durable receipt")
    );
    assert!(
        reopened
            .store()
            .has_block(required_hash)
            .expect("durable retention")
    );
    assert!(
        reopened
            .store()
            .has_undo(required_hash)
            .expect("durable undo")
    );
    assert_eq!(
        reopened
            .store()
            .load_operator_support_counts()
            .expect("durable receipt")
            .pruned_height_count,
        1
    );
    drop(reopened);
    std::fs::remove_dir_all(temp).expect("cleanup");
}
