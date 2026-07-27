// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use super::chain_fixtures::empty_context;
use super::storage_fixtures::{remove_dir_if_exists, temp_store_path};
use super::*;

#[test]
fn open_bitcoin_sync_rpc_control_updates_daemon_runtime_metadata() {
    // Arrange
    let (control, receiver) = DaemonSyncControl::channel();
    let join_handle = spawn_test_sync_control_worker(receiver);
    let mut context = empty_context();
    context.set_daemon_sync_control(control);

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinSyncStatus(OpenBitcoinSyncStatusRequest::default()),
    )
    .expect("sync status");
    let pause = dispatch(
        &mut context,
        MethodCall::OpenBitcoinSyncPause(OpenBitcoinSyncPauseRequest::default()),
    )
    .expect("sync pause");
    let resume = dispatch(
        &mut context,
        MethodCall::OpenBitcoinSyncResume(OpenBitcoinSyncResumeRequest::default()),
    )
    .expect("sync resume");
    drop(context);
    join_handle.join().expect("sync control worker");

    // Assert
    assert_eq!(status["metadata"]["sync_control"]["paused"], json!(false));
    assert_eq!(pause["metadata"]["sync_control"]["paused"], json!(true));
    assert_eq!(resume["metadata"]["sync_control"]["paused"], json!(false));
}

#[test]
fn open_bitcoin_sync_rpc_control_uses_daemon_store_backend() {
    // Arrange
    let path = temp_store_path("sync-control-store-backend");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    store
        .save_runtime_metadata(&RuntimeMetadata::default(), PersistMode::Sync)
        .expect("save metadata");
    let mut context = empty_context();
    context.set_daemon_sync_control(DaemonSyncControl::store_backed(
        store.clone(),
        PersistMode::Sync,
    ));

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinSyncStatus(OpenBitcoinSyncStatusRequest::default()),
    )
    .expect("sync status");
    let pause = dispatch(
        &mut context,
        MethodCall::OpenBitcoinSyncPause(OpenBitcoinSyncPauseRequest::default()),
    )
    .expect("sync pause");
    let resume = dispatch(
        &mut context,
        MethodCall::OpenBitcoinSyncResume(OpenBitcoinSyncResumeRequest::default()),
    )
    .expect("sync resume");

    // Assert
    assert_eq!(status["metadata"]["sync_control"]["paused"], json!(false));
    assert_eq!(pause["metadata"]["sync_control"]["paused"], json!(true));
    assert_eq!(resume["metadata"]["sync_control"]["paused"], json!(false));
    let stored_metadata = store
        .load_runtime_metadata()
        .expect("load metadata")
        .expect("metadata");
    assert!(!stored_metadata.sync_control.paused);
}

#[test]
fn open_bitcoin_sync_rpc_control_requires_daemon_handle() {
    // Arrange
    let mut context = empty_context();

    // Act
    let error = dispatch(
        &mut context,
        MethodCall::OpenBitcoinSyncStatus(OpenBitcoinSyncStatusRequest::default()),
    )
    .expect_err("sync control should require daemon handle");

    // Assert
    assert_eq!(error.kind, RpcFailureKind::ClientNotConnected);
    assert_eq!(
        error
            .maybe_detail
            .expect("sync control error detail")
            .message,
        "daemon sync control is unavailable"
    );
}

fn spawn_test_sync_control_worker(receiver: DaemonSyncControlReceiver) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut metadata = RuntimeMetadata::default();
        while let Ok(request) = receiver.recv_timeout(std::time::Duration::from_secs(1)) {
            match request.action() {
                DaemonSyncControlAction::Status => {}
                DaemonSyncControlAction::Pause => metadata.sync_control.paused = true,
                DaemonSyncControlAction::Resume => metadata.sync_control.paused = false,
            }
            request.respond(Ok(metadata.clone()));
        }
    })
}
