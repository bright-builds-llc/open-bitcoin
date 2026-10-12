// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use crate::chainstate::FlushExecution;
use std::cell::RefCell;

type Events = Rc<RefCell<Vec<&'static str>>>;

struct CheckpointSink {
    events: Events,
    maybe_failure: Option<&'static str>,
    deletes: Cell<usize>,
}

impl CheckpointSink {
    fn effect(&self, stage: &'static str) -> Result<(), StorageError> {
        self.events.borrow_mut().push(stage);
        if self.maybe_failure == Some(stage) {
            return Err(checkpoint_error(stage));
        }
        Ok(())
    }
}

fn checkpoint_error(stage: &str) -> StorageError {
    StorageError::BackendFailure {
        namespace: StorageNamespace::Headers,
        message: stage.to_owned(),
        action: StorageRecoveryAction::Restart,
    }
}

impl FlushPersistSink for CheckpointSink {
    fn load_prune_protection(
        &self,
    ) -> Result<crate::chainstate::PruneProtectionSnapshot, StorageError> {
        Ok(crate::chainstate::PruneProtectionSnapshot::no_index(
            Vec::new(),
        ))
    }
    fn persist_block(&mut self, _: &Block) -> Result<(), StorageError> {
        self.effect("block")
    }
    fn persist_undo(&mut self, _: BlockHash, _: &BlockUndo) -> Result<(), StorageError> {
        self.effect("undo")
    }
    fn persist_header_entries(&mut self, _: &[HeaderEntry]) -> Result<(), StorageError> {
        self.effect("raw_headers")
    }
    fn persist_chain_meta(&mut self, _: &[ChainPosition]) -> Result<(), StorageError> {
        self.effect("metadata")
    }
    fn commit_paired_unlink(
        &mut self,
        _: u32,
        _: BlockHash,
    ) -> Result<PairedDeleteOutcome, StorageError> {
        self.deletes.set(self.deletes.get() + 1);
        self.effect(if self.deletes.get() == 1 {
            "unlink1"
        } else {
            "unlink2"
        })?;
        Ok(PairedDeleteOutcome::DeletedLiveMate)
    }
}

struct CheckpointCoins {
    events: Events,
    fail: bool,
}

impl CoinsView for CheckpointCoins {
    fn get_coin(&self, _: &OutPoint) -> Result<Option<Coin>, ChainstateError> {
        Ok(None)
    }
    fn have_coin(&self, _: &OutPoint) -> Result<bool, ChainstateError> {
        Ok(false)
    }
    fn best_block(&self) -> Result<Option<BlockHash>, ChainstateError> {
        Ok(None)
    }
    fn head_blocks(&self) -> Result<Vec<BlockHash>, ChainstateError> {
        Ok(Vec::new())
    }
    fn batch_write(&mut self, _: CoinsBatch, _: Option<BlockHash>) -> Result<(), ChainstateError> {
        self.events.borrow_mut().push("coins");
        if self.fail {
            return Err(ChainstateError::CoinsStorage {
                detail: "coins".to_owned(),
            });
        }
        Ok(())
    }
}

struct CheckpointRun {
    result: Result<FlushExecution, StorageError>,
    events: Vec<&'static str>,
    deleted: Vec<BlockHash>,
    attempts: [usize; 4],
}

fn run_checkpoint(
    mode: FlushMode,
    ready: bool,
    disk: u64,
    prune: bool,
    maybe_failure: Option<&'static str>,
) -> CheckpointRun {
    let events = Events::default();
    let mut sink = CheckpointSink {
        events: events.clone(),
        maybe_failure,
        deletes: Cell::new(0),
    };
    let mut cache = CoinsCache::from_parent(CheckpointCoins {
        events: events.clone(),
        fail: maybe_failure == Some("coins"),
    });
    cache
        .add_coin(
            OutPoint {
                txid: Txid::default(),
                vout: 0,
            },
            sample_coin(),
            true,
        )
        .expect("dirty");
    let mut lifecycle = if ready {
        FlushLifecycle::ready_for_test(
            default_coins_cache_byte_limit(),
            0,
            FlushPolicyTime::from_unix_seconds(u64::MAX),
            false,
        )
    } else {
        FlushLifecycle::not_ready_for_test()
    };
    let first = ChainPosition::new(header(BlockHash::default(), 1), 1, 1, 1);
    let second = ChainPosition::new(header(first.block_hash, 2), 2, 2, 1);
    let tip = ChainPosition::new(header(second.block_hash, 3), 400, 400, 1);
    let plan = PrunePlan {
        heights: if prune { vec![1, 2] } else { Vec::new() },
    };
    let mut deleted = Vec::new();
    let mut attempts = [0; 4];
    let result = lifecycle.execute_flush_applying_plan_with_header_checkpoint(
        &mut sink,
        &mut cache,
        mode,
        policy_now(),
        disk,
        &[(first.block_hash, BlockUndo::default())],
        &[],
        &[Block {
            header: first.header.clone(),
            transactions: Vec::new(),
        }],
        &[first, second, tip],
        &plan,
        &[],
        &mut |hash| deleted.push(hash),
        &mut |selected: &CheckpointSink| {
            attempts[0] += 1;
            selected.effect("supplier")?;
            attempts[1] += 1;
            selected.effect("capture")?;
            attempts[2] += 1;
            selected.effect("validation")?;
            attempts[3] += 1;
            selected.effect("snapshot")
        },
    );
    CheckpointRun {
        result,
        events: events.borrow().clone(),
        deleted,
        attempts,
    }
}

#[test]
fn phase162_header_checkpoint_order_selected_prefix_before_unlink_coins_metadata() {
    // Arrange / Act
    let run = run_checkpoint(FlushMode::Always, true, u64::MAX, true, None);
    // Assert
    assert_eq!(
        run.events,
        [
            "block",
            "undo",
            "raw_headers",
            "supplier",
            "capture",
            "validation",
            "snapshot",
            "unlink1",
            "unlink2",
            "coins",
            "metadata"
        ]
    );
    assert_eq!(run.attempts, [1; 4]);
    assert_eq!(
        run.result.expect("selected").deleted_block_hashes,
        run.deleted
    );
    assert_eq!(run.deleted.len(), 2);
}

#[test]
fn phase162_header_checkpoint_order_no_work_never_supplies_captures_validates_or_writes() {
    // Arrange
    for (mode, ready, disk, fails) in [
        (FlushMode::None, true, u64::MAX, false),
        (FlushMode::IfNeeded, true, u64::MAX, false),
        (FlushMode::Periodic, true, u64::MAX, false),
        (FlushMode::Always, false, u64::MAX, true),
        (FlushMode::Always, true, 0, true),
    ] {
        // Act
        let run = run_checkpoint(mode, ready, disk, false, Some("snapshot"));
        // Assert
        assert_eq!(run.result.is_err(), fails);
        assert_eq!(run.attempts, [0; 4]);
        assert!(run.events.is_empty());
        assert!(run.deleted.is_empty());
    }
}

#[test]
fn phase162_header_checkpoint_order_prefix_and_checkpoint_failures_preserve_mates_and_coins() {
    // Arrange
    let sequence = [
        "block",
        "undo",
        "raw_headers",
        "supplier",
        "capture",
        "validation",
        "snapshot",
    ];
    for (index, stage) in sequence.iter().enumerate() {
        // Act
        let run = run_checkpoint(FlushMode::Always, true, u64::MAX, true, Some(stage));
        // Assert
        assert!(
            matches!(run.result, Err(StorageError::BackendFailure { namespace: StorageNamespace::Headers, ref message, action: StorageRecoveryAction::Restart }) if message == stage)
        );
        assert_eq!(run.events, sequence[..=index]);
        assert!(run.deleted.is_empty());
    }
}

#[test]
fn phase162_header_checkpoint_order_later_failures_report_actual_deleted_prefix() {
    // Arrange
    let sequence = [
        "block",
        "undo",
        "raw_headers",
        "supplier",
        "capture",
        "validation",
        "snapshot",
        "unlink1",
        "unlink2",
        "coins",
        "metadata",
    ];
    for (stage, count) in [
        ("unlink1", 0),
        ("unlink2", 1),
        ("coins", 2),
        ("metadata", 2),
    ] {
        // Act
        let run = run_checkpoint(FlushMode::Always, true, u64::MAX, true, Some(stage));
        // Assert
        assert!(run.result.is_err());
        assert_eq!(run.attempts, [1; 4]);
        let end = sequence
            .iter()
            .position(|value| *value == stage)
            .expect("stage");
        assert_eq!(run.events, sequence[..=end]);
        assert_eq!(run.deleted.len(), count);
    }
}

#[test]
fn phase162_header_checkpoint_order_explicit_none_plan_keeps_existing_prefix_and_delete_behavior() {
    // Arrange / Act
    let run = run_checkpoint(FlushMode::None, true, u64::MAX, true, None);
    // Assert
    assert_eq!(run.attempts, [1; 4]);
    assert_eq!(run.deleted.len(), 2);
    assert!(!run.result.expect("explicit plan").wrote_coins);
    assert_eq!(run.events.last(), Some(&"unlink2"));
}

#[test]
fn phase162_header_checkpoint_order_explicit_disk_refusal_keeps_existing_deleted_prefix() {
    // Arrange / Act
    let run = run_checkpoint(FlushMode::Always, true, 0, true, None);
    // Assert
    assert!(run.result.is_err());
    assert_eq!(run.attempts, [1; 4]);
    assert_eq!(run.deleted.len(), 2);
    assert_eq!(run.events.last(), Some(&"unlink2"));
}
