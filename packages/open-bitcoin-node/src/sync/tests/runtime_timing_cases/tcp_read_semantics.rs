// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

#[test]
fn phase123_tcp_zero_progress_timeout_is_idle() {
    // Arrange
    let mut reader = ScriptedReader::new(vec![ReadAction::Error(io::ErrorKind::TimedOut)]);
    let mut buffer = [0_u8; 2];

    // Act
    let outcome = read_stage(&mut reader, &mut buffer, true).expect("idle outcome");

    // Assert
    assert_eq!(outcome, ReadStageOutcome::Idle);
}

#[test]
fn phase123_tcp_clean_eof_is_closed() {
    // Arrange
    let mut reader = ScriptedReader::new(vec![ReadAction::Eof]);
    let mut buffer = [0_u8; 2];

    // Act
    let outcome = read_stage(&mut reader, &mut buffer, true).expect("closed outcome");

    // Assert
    assert_eq!(outcome, ReadStageOutcome::Closed);
}

#[test]
fn phase123_partial_frame_timeout_is_not_clean_idle() {
    // Arrange
    let mut reader = ScriptedReader::new(vec![
        ReadAction::Bytes(vec![0x01]),
        ReadAction::Error(io::ErrorKind::TimedOut),
    ]);
    let mut buffer = [0_u8; 2];

    // Act
    let result = read_stage(&mut reader, &mut buffer, true);

    // Assert
    assert!(matches!(result, Err(SyncRuntimeError::Io { .. })));
}

#[test]
fn phase123_partial_frame_eof_is_not_clean_closed() {
    // Arrange
    let mut reader = ScriptedReader::new(vec![ReadAction::Bytes(vec![0x01]), ReadAction::Eof]);
    let mut buffer = [0_u8; 2];

    // Act
    let result = read_stage(&mut reader, &mut buffer, true);

    // Assert
    assert!(matches!(result, Err(SyncRuntimeError::Io { .. })));
}
