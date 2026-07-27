// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use super::*;

#[tokio::test]
async fn phase123_inbound_written_block_increments_served_once() {
    // Arrange
    let context = Arc::new(tokio::sync::Mutex::new(
        ManagedRpcContext::for_local_operator(AddressNetwork::Regtest),
    ));
    let mut responses = context
        .lock()
        .await
        .encode_wire_responses(vec![WireNetworkMessage::Block(Block::default())])
        .expect("block response should encode");
    let response = responses.pop().expect("one encoded block response");
    assert!(matches!(response.message, WireNetworkMessage::Block(_)));
    let write_result = Ok(WriteWireMessageOutcome::Written);

    // Act
    assert!(acknowledge_inbound_response_write(&write_result, &response, &context).await);
    let served_count = context
        .lock()
        .await
        .block_served_write_count()
        .expect("authoritative block write count");

    // Assert
    assert_eq!(served_count, 1);
}

#[tokio::test]
async fn phase123_enabled_runtime_config_serves_and_acknowledges_inbound_block() {
    // Arrange
    let (mut context, block) = phase123_block_serving_context(true);
    let responses = context
        .receive_inbound_wire_message(123, phase123_block_request(&block), 2)
        .expect("serve enabled block request");
    let response = responses
        .into_iter()
        .find(|response| matches!(response.message, WireNetworkMessage::Block(_)))
        .expect("enabled runtime should produce a typed Block response");
    let context = Arc::new(tokio::sync::Mutex::new(context));
    let write_result = Ok(WriteWireMessageOutcome::Written);

    // Act
    assert!(acknowledge_inbound_response_write(&write_result, &response, &context).await);
    let served_count = context
        .lock()
        .await
        .block_served_write_count()
        .expect("authoritative block write count");

    // Assert
    assert_eq!(served_count, 1);
}

#[tokio::test]
async fn phase123_disabled_runtime_config_does_not_serve_inbound_block() {
    // Arrange
    let (mut context, block) = phase123_block_serving_context(false);

    // Act
    let responses = context
        .receive_inbound_wire_message(123, phase123_block_request(&block), 2)
        .expect("handle disabled block request");
    let served_count = context
        .block_served_write_count()
        .expect("authoritative block write count");

    // Assert
    assert!(
        !responses
            .iter()
            .any(|response| matches!(response.message, WireNetworkMessage::Block(_)))
    );
    assert_eq!(served_count, 0);
}

#[tokio::test]
async fn durable_block_serving_survives_restart_without_cache_hydration() {
    // Arrange
    let (context, block, data_dir) = durable_block_serving_context(true);
    let context = Arc::new(tokio::sync::Mutex::new(context));

    // Act
    let responses =
        resolve_inbound_wire_responses(&context, 123, durable_block_requests(&block), 2)
            .await
            .expect("resolve durable block responses");
    for response in &responses {
        let written = Ok(WriteWireMessageOutcome::Written);
        assert!(acknowledge_inbound_response_write(&written, response, &context).await);
    }
    let served_count = context
        .lock()
        .await
        .block_served_write_count()
        .expect("authoritative block write count");

    // Assert
    assert_eq!(responses.len(), 3);
    assert!(matches!(responses[0].message, WireNetworkMessage::Block(_)));
    assert!(matches!(responses[1].message, WireNetworkMessage::Block(_)));
    assert!(matches!(
        responses[2].message,
        WireNetworkMessage::CompactBlock(_)
    ));
    assert_eq!(served_count, 3);
    drop(context);
    fs::remove_dir_all(data_dir).expect("remove durable block-serving store");
}

#[tokio::test]
async fn durable_block_serving_missing_body_returns_notfound_without_served_credit() {
    // Arrange
    let (context, block, data_dir) = durable_block_serving_context(false);
    let context = Arc::new(tokio::sync::Mutex::new(context));

    // Act
    let responses =
        resolve_inbound_wire_responses(&context, 123, phase123_block_request(&block), 2)
            .await
            .expect("resolve missing durable block response");
    let served_count = context
        .lock()
        .await
        .block_served_write_count()
        .expect("authoritative block write count");

    // Assert
    assert_eq!(responses.len(), 1);
    assert!(matches!(
        responses[0].message,
        WireNetworkMessage::NotFound(_)
    ));
    assert_eq!(served_count, 0);
    drop(context);
    fs::remove_dir_all(data_dir).expect("remove missing durable block-serving store");
}

#[tokio::test]
async fn durable_block_serving_corruption_is_redacted_as_notfound() {
    // Arrange
    let failure = ScriptedDurableBlockFailure::Corruption;

    // Act
    let (message, bytes, served_count) = durable_block_failure_outcome(failure).await;

    // Assert
    assert!(matches!(message, WireNetworkMessage::NotFound(_)));
    assert!(!bytes.windows(7).any(|window| window == b"private"));
    assert_eq!(served_count, 0);
}

#[tokio::test]
async fn durable_block_serving_store_error_is_redacted_as_notfound() {
    // Arrange
    let failure = ScriptedDurableBlockFailure::Backend;

    // Act
    let (message, bytes, served_count) = durable_block_failure_outcome(failure).await;

    // Assert
    assert!(matches!(message, WireNetworkMessage::NotFound(_)));
    assert!(!bytes.windows(7).any(|window| window == b"private"));
    assert_eq!(served_count, 0);
}

#[tokio::test]
async fn phase123_inbound_rejected_block_does_not_increment_served() {
    // Arrange
    let responses = vec![block_response()];
    let write_results = vec![rejected_write_result()];

    // Act
    let served_count = acknowledged_block_count(responses, write_results).await;

    // Assert
    assert_eq!(served_count, 0);
}

#[tokio::test]
async fn phase123_inbound_write_error_block_does_not_increment_served() {
    // Arrange
    let responses = vec![block_response()];
    let write_results = vec![Err(io::Error::other("scripted write failure"))];

    // Act
    let served_count = acknowledged_block_count(responses, write_results).await;

    // Assert
    assert_eq!(served_count, 0);
}

#[tokio::test]
async fn phase123_inbound_written_non_block_does_not_increment_served() {
    // Arrange
    let responses = vec![non_block_response()];
    let write_results = vec![Ok(WriteWireMessageOutcome::Written)];

    // Act
    let served_count = acknowledged_block_count(responses, write_results).await;

    // Assert
    assert_eq!(served_count, 0);
}

#[tokio::test]
async fn phase123_inbound_partial_batch_counts_successful_block_prefix() {
    // Arrange
    let responses = vec![block_response(), non_block_response(), block_response()];
    let write_results = vec![
        Ok(WriteWireMessageOutcome::Written),
        Ok(WriteWireMessageOutcome::Written),
        Err(io::Error::other("scripted later write failure")),
    ];

    // Act
    let served_count = acknowledged_block_count(responses, write_results).await;

    // Assert
    assert_eq!(served_count, 1);
}

#[tokio::test]
async fn phase123_inbound_two_blocks_before_later_failure_count_two() {
    // Arrange
    let responses = vec![block_response(), block_response(), non_block_response()];
    let write_results = vec![
        Ok(WriteWireMessageOutcome::Written),
        Ok(WriteWireMessageOutcome::Written),
        Err(io::Error::other("scripted later write failure")),
    ];

    // Act
    let served_count = acknowledged_block_count(responses, write_results).await;

    // Assert
    assert_eq!(served_count, 2);
}

#[tokio::test]
async fn phase123_inbound_encoding_failure_does_not_increment_served() {
    // Arrange
    let context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);
    let inventory = InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: BlockHash::default().into(),
    };
    let oversized = WireNetworkMessage::Inv(InventoryList::new(vec![inventory; MAX_INV_SIZE + 1]));

    // Act
    let result = context.encode_wire_responses(vec![oversized]);
    let served_count = context
        .block_served_write_count()
        .expect("authoritative block write count");

    // Assert
    assert!(result.is_err());
    assert_eq!(served_count, 0);
}
