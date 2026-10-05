// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Real stores and software failure boundaries; no hardware power-loss claim.

use super::{catch_up::fixtures::TurnHistory, recovery::ValidatedHistory, *};
use crate::chainstate::BasicFilterStartupMode;
use crate::storage::fjall_store::filters::FilterPublicationFault;
use open_bitcoin_core::{
    chainstate::{FlushMode, FlushPolicyTime, PrunePlan},
    consensus::{ConsensusParams, ScriptVerifyFlags},
};

mod accepted_faults;
mod history_loss;
mod retention;

type RawRows = Vec<(Vec<u8>, Vec<u8>)>;

/// Every key in these namespaces, including payloads, locks, intents and coins B/H.
fn snapshot_store(path: &Path) -> Vec<(&'static str, RawRows)> {
    ["block_index", "chainstate", "coins"]
        .into_iter()
        .map(|namespace| {
            let mut rows = Vec::new();
            raw_namespace(path, namespace, |space| {
                for guard in space.iter() {
                    let (key, value) = guard.into_inner().expect("raw stored row");
                    rows.push((key.to_vec(), value.to_vec()));
                }
            });
            (namespace, rows)
        })
        .collect()
}

fn configured(path: &Path) -> Result<DurableSyncRuntime, SyncRuntimeError> {
    DurableSyncRuntime::open_configured(
        FjallNodeStore::open(path).expect("all prior handles dropped"),
        sync_config(),
        BasicFilterStartupMode::Enabled,
    )
}

fn assert_refusal_unchanged(path: &Path, category: &str) {
    let before = snapshot_store(path);
    assert_filter_refusal(configured(path), category);
    assert_eq!(
        snapshot_store(path),
        before,
        "complete persisted key/value equality"
    );
}

fn manual(runtime: &DurableSyncRuntime, heights: &[u32]) -> crate::chainstate::FlushExecution {
    runtime
        .network
        .flush_applying_prune_plan(
            FlushMode::Always,
            FlushPolicyTime::new(0),
            u64::MAX,
            &PrunePlan {
                heights: heights.to_vec(),
            },
            &[],
        )
        .expect("existing serialized checkpoint and paired delete owner")
}

fn assert_pair(store: &FjallNodeStore, history: &ValidatedHistory, height: usize, present: bool) {
    let hash = history.full.active_chain[height].block_hash;
    assert_eq!(
        store.load_block(hash).expect("actual body"),
        present.then(|| history.blocks[height].clone())
    );
    assert_eq!(
        store.load_undo(hash).expect("actual undo"),
        present
            .then(|| history.full.undo_by_block.get(&hash).cloned())
            .flatten()
    );
}

fn remove_one_mate(path: &Path, hash: BlockHash, body: bool) {
    let namespace = if body { "block_index" } else { "chainstate" };
    let prefix = if body { "block:" } else { "undo:" };
    let key = codec::record_key(hash).replacen(codec::RECORD_PREFIX, prefix, 1);
    raw_namespace(path, namespace, |space| {
        space.remove(key).expect("independent missing-mate fault");
    });
}

fn params() -> ConsensusParams {
    ConsensusParams {
        coinbase_maturity: 1,
        ..Default::default()
    }
}

fn finish(runtime: &DurableSyncRuntime) {
    for _ in 0..256 {
        let turn = runtime
            .network
            .drive_basic_filter_index_turn()
            .expect("ordinary ordered turn");
        if turn.maybe_progress.expect("enabled").current_lag() == 0 {
            return;
        }
    }
    panic!("bounded fixture did not finish");
}
