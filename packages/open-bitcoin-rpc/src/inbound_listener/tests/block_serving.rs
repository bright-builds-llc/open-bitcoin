// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use open_bitcoin_node::core::chainstate::PruneMode;

use super::listener_fixtures::*;
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
    let mut response = responses.pop().expect("one encoded block response");
    assert!(matches!(response.message, WireNetworkMessage::Block(_)));
    let write_result = Ok(WriteWireMessageOutcome::Written);

    // Act
    assert!(acknowledge_inbound_response_write(&write_result, &mut response, &context).await);
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
    let mut response = responses
        .into_iter()
        .find(|response| matches!(response.message, WireNetworkMessage::Block(_)))
        .expect("enabled runtime should produce a typed Block response");
    let context = Arc::new(tokio::sync::Mutex::new(context));
    let write_result = Ok(WriteWireMessageOutcome::Written);

    // Act
    assert!(acknowledge_inbound_response_write(&write_result, &mut response, &context).await);
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
    let mut responses =
        resolve_inbound_wire_responses(&context, 123, durable_block_requests(&block), 2)
            .await
            .expect("resolve durable block responses");
    for response in &mut responses {
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

#[test]
fn durable_getdata_without_store_body_is_unavailable_at_gate() {
    // Arrange
    let (mut context, block, data_dir) = durable_block_serving_context(false);

    // Act
    let plan = context
        .prepare_inbound_wire_message(123, phase123_block_request(&block), 2)
        .expect("prepare missing-body getdata");
    let FieldAvailability::Available(status) = context
        .block_relay_evidence_status()
        .expect("block-serving evidence")
        .block_serving
        .status
    else {
        panic!("block-serving status should be available after the durable gate");
    };
    let has_durable_block_intent = plan
        .responses
        .iter()
        .any(|item| matches!(item, ManagedInboundResponsePlanItem::DurableBlock(_)));
    let resolved = plan.resolve();
    let served_count = context
        .block_served_write_count()
        .expect("authoritative block write count");

    // Assert
    assert!(
        !has_durable_block_intent,
        "missing store body must not produce a DurableBlock intent"
    );
    assert_eq!(status.unavailable_count, 1);
    assert_eq!(resolved.responses.len(), 1);
    assert!(matches!(
        resolved.responses[0].message,
        WireNetworkMessage::NotFound(_)
    ));
    assert_eq!(served_count, 0);
    drop(context);
    fs::remove_dir_all(data_dir).expect("remove missing-body gate store");
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

fn window_script_num(value: u32) -> Vec<u8> {
    if value == 0 {
        return vec![0x00];
    }
    let mut magnitude = u64::from(value);
    let mut encoded = Vec::new();
    while magnitude > 0 {
        encoded.push((magnitude & 0xff) as u8);
        magnitude >>= 8;
    }
    if encoded.last().is_some_and(|byte| byte & 0x80 != 0) {
        encoded.push(0x00);
    }
    let mut script = Vec::with_capacity(encoded.len() + 1);
    script.push(encoded.len() as u8);
    script.extend(encoded);
    script.push(0x51);
    script
}

fn mined_window_block(previous_block_hash: BlockHash, height: u32) -> Block {
    let mut script_sig_bytes = window_script_num(height);
    script_sig_bytes.push(0x51);
    let script_sig = ScriptBuf::from_bytes(script_sig_bytes).expect("coinbase script");
    let script_pubkey = ScriptBuf::from_bytes(vec![0x51]).expect("output script");
    let transaction = Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig,
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(5_000_000_000).expect("coinbase amount"),
            script_pubkey,
        }],
        lock_time: 0,
    };
    let (merkle_root, maybe_mutated) =
        block_merkle_root(core::slice::from_ref(&transaction)).expect("coinbase merkle root");
    assert!(!maybe_mutated);
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash,
            merkle_root,
            time: 1_231_006_500 + height,
            bits: PHASE123_EASY_BITS,
            nonce: 0,
        },
        transactions: vec![transaction],
    };
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("mined nonce");
    block
}

#[tokio::test]
async fn out_of_window_notfound_closes_the_inbound_socket() {
    // Arrange
    let runtime = RuntimeConfig {
        inbound: loopback_config(2),
        block_serving: BlockRelayActivationPolicy {
            block_serving: BlockServingActivationConfig { enabled: true },
            compact_relay: CompactRelayActivationConfig::default(),
        },
        ..RuntimeConfig::default()
    };
    let mut context = ManagedRpcContext::from_runtime_config(&runtime);
    context
        .set_prune_mode(PruneMode::ManualOnly)
        .expect("manual prune mode");
    let mut previous = BlockHash::default();
    let mut height_zero = Block::default();
    for height in 0..=291 {
        let block = mined_window_block(previous, height);
        context
            .connect_local_block(&block)
            .expect("connect limited-window block");
        if height == 0 {
            height_zero = block.clone();
        }
        previous = block_hash(&block.header);
    }
    let context = Arc::new(tokio::sync::Mutex::new(context));
    let activation = activate_inbound_listener(&runtime.inbound).await;
    let endpoint = activation
        .bound_endpoints()
        .first()
        .expect("bound loopback endpoint")
        .bound_endpoint
        .clone();
    let worker = start_inbound_accept_loop(activation, Arc::clone(&context))
        .expect("listener worker should start");
    let stream = TcpStream::connect(&endpoint)
        .await
        .expect("connect limited-window peer");
    send_message(
        &stream,
        WireNetworkMessage::Version(VersionMessage {
            nonce: 149_312,
            ..VersionMessage::default()
        }),
    )
    .await;
    for _ in 0..4 {
        let _ = receive_message(&stream).await;
    }
    send_message(&stream, WireNetworkMessage::Verack).await;

    // Act
    send_message(&stream, phase123_block_request(&height_zero)).await;
    let response = tokio::time::timeout(Duration::from_secs(2), receive_any_message(&stream))
        .await
        .expect("out-of-window response should arrive");
    stream
        .readable()
        .await
        .expect("socket should become readable after NotFound");
    let mut leftover = [0_u8; 64];
    let read = stream.try_read(&mut leftover);

    // Assert
    assert!(matches!(response, WireNetworkMessage::NotFound(_)));
    assert!(matches!(read, Ok(0)));
    worker.shutdown().await;
}
