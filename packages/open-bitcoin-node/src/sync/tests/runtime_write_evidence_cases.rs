// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use open_bitcoin_core::primitives::{BlockHash, NetworkMagic};
use open_bitcoin_network::WireNetworkMessage;

use super::*;

#[derive(Debug)]
struct ExactFailingSendSession {
    send_calls: usize,
    maybe_fail_on_call: Option<usize>,
}

impl ExactFailingSendSession {
    fn succeeding() -> Self {
        Self {
            send_calls: 0,
            maybe_fail_on_call: None,
        }
    }

    fn failing_on_call(call: usize) -> Self {
        Self {
            send_calls: 0,
            maybe_fail_on_call: Some(call),
        }
    }
}

impl SyncPeerSession for ExactFailingSendSession {
    fn send(
        &mut self,
        _message: &WireNetworkMessage,
        _magic: NetworkMagic,
    ) -> Result<(), SyncRuntimeError> {
        self.send_calls = self.send_calls.saturating_add(1);
        if self.maybe_fail_on_call == Some(self.send_calls) {
            return Err(SyncRuntimeError::Io {
                peer: "phase123-scripted-peer".to_string(),
                message: format!("scripted send failure on call {}", self.send_calls),
            });
        }

        Ok(())
    }

    fn receive(
        &mut self,
        _magic: NetworkMagic,
    ) -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> {
        Ok(SyncPeerReceiveOutcome::Closed)
    }
}

fn block_message(previous_hash_byte: u8, height: u32) -> WireNetworkMessage {
    WireNetworkMessage::Block(build_block(
        BlockHash::from_byte_array([previous_hash_byte; 32]),
        height,
    ))
}

fn runtime_for_write_evidence(test_name: &str) -> (DurableSyncRuntime, PathBuf) {
    let path = temp_store_path(test_name);
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    (runtime, path)
}

#[test]
fn phase123_sync_block_write_success_increments_served_once() {
    // Arrange
    let (mut runtime, path) = runtime_for_write_evidence("phase123-block-write-success");
    let mut session = ExactFailingSendSession::succeeding();
    let messages = [block_message(1, 1)];

    // Act
    let result = runtime.send_all(&mut session, &messages);

    // Assert
    assert!(result.is_ok());
    assert_eq!(runtime.network.block_served_write_count(), 1);
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_sync_block_write_failure_does_not_increment_served() {
    // Arrange
    let (mut runtime, path) = runtime_for_write_evidence("phase123-block-write-failure");
    let mut session = ExactFailingSendSession::failing_on_call(1);
    let messages = [block_message(2, 1)];

    // Act
    let result = runtime.send_all(&mut session, &messages);

    // Assert
    assert!(matches!(result, Err(SyncRuntimeError::Io { .. })));
    assert_eq!(runtime.network.block_served_write_count(), 0);
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_sync_non_block_write_does_not_increment_served() {
    // Arrange
    let (mut runtime, path) = runtime_for_write_evidence("phase123-non-block-write");
    let mut session = ExactFailingSendSession::succeeding();
    let messages = [WireNetworkMessage::Ping { nonce: 123 }];

    // Act
    let result = runtime.send_all(&mut session, &messages);

    // Assert
    assert!(result.is_ok());
    assert_eq!(runtime.network.block_served_write_count(), 0);
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_sync_partial_batch_counts_each_successful_block_before_failure() {
    // Arrange
    let (mut runtime, path) = runtime_for_write_evidence("phase123-partial-batch");
    let mut session = ExactFailingSendSession::failing_on_call(3);
    let messages = [
        block_message(3, 1),
        WireNetworkMessage::Ping { nonce: 123 },
        block_message(4, 2),
    ];

    // Act
    let result = runtime.send_all(&mut session, &messages);

    // Assert
    assert!(matches!(result, Err(SyncRuntimeError::Io { .. })));
    assert_eq!(runtime.network.block_served_write_count(), 1);
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_sync_two_successful_blocks_before_later_failure_count_two() {
    // Arrange
    let (mut runtime, path) = runtime_for_write_evidence("phase123-two-block-prefix");
    let mut session = ExactFailingSendSession::failing_on_call(3);
    let messages = [
        block_message(5, 1),
        block_message(6, 2),
        WireNetworkMessage::Ping { nonce: 123 },
    ];

    // Act
    let result = runtime.send_all(&mut session, &messages);

    // Assert
    assert!(matches!(result, Err(SyncRuntimeError::Io { .. })));
    assert_eq!(runtime.network.block_served_write_count(), 2);
    remove_dir_if_exists(&path);
}
