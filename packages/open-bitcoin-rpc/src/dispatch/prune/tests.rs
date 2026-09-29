// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/node/blockstorage.h

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use open_bitcoin_node::FjallNodeStore;
use open_bitcoin_node::core::chainstate::PruneMode;
use open_bitcoin_node::core::consensus::block_hash;
use open_bitcoin_node::core::primitives::BlockHash;
use open_bitcoin_node::status::{FieldAvailability, ManualPruneRefusalCode, ManualPruneSurface};
use serde_json::{Value, json};

use super::super::dispatch;
use super::operator_prune_status;
use crate::ManagedRpcContext;
use crate::dispatch::tests::chain_fixtures::{build_block, empty_context, p2sh_script};
use crate::error::RpcErrorCode;
use crate::method::{
    ClearPruneLockRequest, ListPruneLocksRequest, MethodCall, PruneBlockchainRequest,
    SetPruneLockRequest,
};

struct TempStore {
    path: PathBuf,
}

impl TempStore {
    fn new(test_name: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "open-bitcoin-rpc-prune-locks-{test_name}-{}-{nanos}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&path);
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempStore {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

fn context_with_store(path: &Path) -> ManagedRpcContext {
    let store = FjallNodeStore::open(path).expect("open prune lock store");
    let mut context = empty_context();
    context.set_metrics_store(store);
    context
}

fn list_locks(context: &mut ManagedRpcContext) -> Value {
    dispatch(
        context,
        MethodCall::ListPruneLocks(ListPruneLocksRequest {}),
    )
    .expect("list prune locks")
}

fn set_lock(
    context: &mut ManagedRpcContext,
    name: &str,
    height_first: u32,
    height_last: u32,
) -> Result<Value, crate::RpcFailure> {
    dispatch(
        context,
        MethodCall::SetPruneLock(SetPruneLockRequest {
            name: name.to_owned(),
            height_first,
            height_last,
        }),
    )
}

fn clear_lock(context: &mut ManagedRpcContext, name: &str) -> Value {
    dispatch(
        context,
        MethodCall::ClearPruneLock(ClearPruneLockRequest {
            name: name.to_owned(),
        }),
    )
    .expect("clear prune lock")
}

#[test]
fn list_on_an_empty_record_returns_an_empty_array() {
    // Arrange
    let temp = TempStore::new("empty-list");
    let mut context = context_with_store(temp.path());

    // Act
    let listed = list_locks(&mut context);

    // Assert
    assert_eq!(listed, json!([]));
}

#[test]
fn set_replaces_the_same_name_and_survives_reopen() {
    // Arrange
    let temp = TempStore::new("replace");
    let listed = {
        let mut context = context_with_store(temp.path());
        let first = set_lock(&mut context, "wallet", 10, 20).expect("first set");
        assert_eq!(
            first,
            json!({ "name": "wallet", "height_first": 10, "height_last": 20 })
        );
        let replaced = set_lock(&mut context, "wallet", 12, 18).expect("replace");
        assert_eq!(
            replaced,
            json!({ "name": "wallet", "height_first": 12, "height_last": 18 })
        );
        list_locks(&mut context)
    };

    // Act
    let reopened = FjallNodeStore::open(temp.path()).expect("reopen store");
    let durable = reopened.load_prune_locks().expect("load locks");

    // Assert
    assert_eq!(
        listed,
        json!([{ "name": "wallet", "height_first": 12, "height_last": 18 }])
    );
    assert_eq!(durable.len(), 1);
    assert_eq!(durable[0].name, "wallet");
    assert_eq!(durable[0].height_first, 12);
    assert_eq!(durable[0].height_last, 18);
}

#[test]
fn clear_removes_a_present_name_and_missing_name_does_not_write() {
    // Arrange
    let temp = TempStore::new("clear");
    let mut context = context_with_store(temp.path());
    set_lock(&mut context, "wallet", 12, 18).expect("wallet");
    set_lock(&mut context, "other", 1, 2).expect("other");

    // Act
    let missing = clear_lock(&mut context, "missing");
    let after_missing = list_locks(&mut context);
    let removed = clear_lock(&mut context, "wallet");
    let after_wallet = list_locks(&mut context);

    // Assert
    assert_eq!(missing, json!({ "success": false }));
    assert_eq!(
        after_missing,
        json!([
            { "name": "other", "height_first": 1, "height_last": 2 },
            { "name": "wallet", "height_first": 12, "height_last": 18 }
        ])
    );
    assert_eq!(removed, json!({ "success": true }));
    assert_eq!(
        after_wallet,
        json!([{ "name": "other", "height_first": 1, "height_last": 2 }])
    );
}

#[test]
fn clear_of_the_only_row_leaves_an_empty_list() {
    // Arrange
    let temp = TempStore::new("clear-only");
    let mut context = context_with_store(temp.path());
    set_lock(&mut context, "wallet", 12, 18).expect("wallet");

    // Act
    let removed = clear_lock(&mut context, "wallet");
    let listed = list_locks(&mut context);

    // Assert
    assert_eq!(removed, json!({ "success": true }));
    assert_eq!(listed, json!([]));
}

#[test]
fn invalid_ranges_return_invalid_parameter_and_do_not_write() {
    // Arrange
    let temp = TempStore::new("refuse");
    let mut context = context_with_store(temp.path());
    set_lock(&mut context, "keep", 1, 2).expect("keeper");

    // Act
    let empty_name = set_lock(&mut context, "", 1, 2).expect_err("empty name");
    let reversed = set_lock(&mut context, "bad", 20, 10).expect_err("reversed");
    let overflow = set_lock(&mut context, "bad", 0, u32::MAX).expect_err("overflow");
    let listed = list_locks(&mut context);
    drop(context);
    let reopened = FjallNodeStore::open(temp.path()).expect("reopen");
    let durable = reopened.load_prune_locks().expect("load");

    // Assert
    for failure in [&empty_name, &reversed, &overflow] {
        let detail = failure.maybe_detail.as_ref().expect("detail");
        assert_eq!(detail.code, RpcErrorCode::InvalidParameter);
    }
    assert_eq!(
        listed,
        json!([{ "name": "keep", "height_first": 1, "height_last": 2 }])
    );
    assert_eq!(durable.len(), 1);
    assert_eq!(durable[0].name, "keep");
}

fn prune(context: &mut ManagedRpcContext, height: i64) -> Result<Value, crate::RpcFailure> {
    dispatch(
        context,
        MethodCall::PruneBlockchain(PruneBlockchainRequest { height }),
    )
}

fn failure_message(failure: &crate::RpcFailure) -> String {
    failure
        .maybe_detail
        .as_ref()
        .expect("rpc detail")
        .message
        .clone()
}

fn enable_manual(context: &mut ManagedRpcContext) {
    context
        .set_prune_mode(PruneMode::ManualOnly)
        .expect("manual prune mode");
}

#[test]
fn disabled_pruneblockchain_does_not_delete_or_set_have_pruned() {
    // Arrange
    let temp = TempStore::new("disabled-prune");
    let mut context = context_with_store(temp.path());
    let block = build_block(
        BlockHash::from_byte_array([0; 32]),
        0,
        500_000_000,
        p2sh_script(),
    );
    let hash = block_hash(&block.header);
    context.connect_local_block(&block).expect("connect");

    // Act
    let failure = prune(&mut context, 1).expect_err("disabled");
    let still_cached = context.cached_block_present(hash).expect("cache");
    let status = operator_prune_status(&context).expect("status");
    drop(context);
    let reopened = FjallNodeStore::open(temp.path()).expect("reopen");

    // Assert
    assert_eq!(
        failure_message(&failure),
        "Cannot prune blocks because node is not in prune mode."
    );
    assert_eq!(
        failure.maybe_detail.expect("detail").code,
        RpcErrorCode::MiscError
    );
    assert!(still_cached);
    assert_eq!(
        status.manual_prune,
        FieldAvailability::available(ManualPruneSurface::Refused {
            reason: ManualPruneRefusalCode::Disabled,
        })
    );
    assert!(!reopened.load_have_pruned().expect("have-pruned"));
}

#[test]
fn height_zero_returns_zero_without_planning_a_prune() {
    // Arrange
    let mut context = empty_context();
    enable_manual(&mut context);
    let block = build_block(
        BlockHash::from_byte_array([0; 32]),
        0,
        500_000_000,
        p2sh_script(),
    );
    context.connect_local_block(&block).expect("connect");

    // Act
    let height = prune(&mut context, 0).expect("zero");

    // Assert
    assert_eq!(height, json!(0));
}

#[test]
fn negative_height_is_invalid_parameter() {
    // Arrange
    let mut context = empty_context();
    enable_manual(&mut context);

    // Act
    let failure = prune(&mut context, -1).expect_err("negative");

    // Assert
    assert_eq!(failure_message(&failure), "Negative block height.");
    assert_eq!(
        failure.maybe_detail.expect("detail").code,
        RpcErrorCode::InvalidParameter
    );
}

#[test]
fn chain_too_short_is_misc_error_and_timestamp_miss_does_not_plan() {
    // Arrange
    let mut context = empty_context();
    enable_manual(&mut context);

    // Act
    let short = prune(&mut context, 10).expect_err("short");
    let timestamp = prune(&mut context, 1_000_000_001).expect_err("timestamp");

    // Assert
    assert_eq!(
        failure_message(&short),
        "Blockchain is too short for pruning."
    );
    assert_eq!(
        short.maybe_detail.expect("detail").code,
        RpcErrorCode::MiscError
    );
    assert_eq!(
        failure_message(&timestamp),
        "block at the requested timestamp was not found"
    );
    assert_eq!(
        timestamp.maybe_detail.expect("detail").code,
        RpcErrorCode::InvalidParameter
    );
}

#[test]
fn keep_window_refuses_without_deleting_and_legal_height_returns_helper() {
    // Arrange — regtest prune-after height is 1000, so the tip must reach it.
    let mut context = empty_context();
    enable_manual(&mut context);
    let tip_hash = connect_chain(&mut context, 1_000);

    // Act
    let above = prune(&mut context, 1_001).expect_err("above tip");
    let keep = prune(&mut context, 1_000).expect_err("keep window");
    let still_cached = context.cached_block_present(tip_hash).expect("cache");
    let legal = prune(&mut context, 100).expect("legal manual");
    context
        .set_prune_mode(PruneMode::Automatic { target_mib: 550 })
        .expect("automatic");
    let automatic = prune(&mut context, 100).expect("automatic manual prune");

    // Assert
    assert_eq!(
        failure_message(&above),
        "Blockchain is shorter than the attempted prune height."
    );
    assert_eq!(
        failure_message(&keep),
        "target is inside the 288-block keep window"
    );
    assert!(still_cached);
    assert_eq!(legal, json!(-1));
    assert_eq!(automatic, json!(-1));
    let status = operator_prune_status(&context).expect("status");
    assert_eq!(
        status.manual_prune,
        FieldAvailability::available(ManualPruneSurface::Height { height: -1 })
    );
}

fn connect_chain(context: &mut ManagedRpcContext, tip: u32) -> BlockHash {
    let script = p2sh_script();
    let mut previous = BlockHash::from_byte_array([0; 32]);
    let mut last = previous;
    for height in 0..=tip {
        let block = build_block(previous, height, 500_000_000, script.clone());
        last = block_hash(&block.header);
        context.connect_local_block(&block).expect("connect");
        previous = last;
    }
    last
}
