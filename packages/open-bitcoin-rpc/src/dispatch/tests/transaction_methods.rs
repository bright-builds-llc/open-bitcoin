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

use super::chain_fixtures::*;
use super::wallet_fixtures::*;
use super::*;

#[test]
fn sendrawtransaction_returns_txid_and_maps_rejections() {
    // Arrange
    let mut context = empty_context();
    let genesis = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        0,
        500_000_000,
        p2sh_script(),
    );
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000, p2sh_script());
    context.connect_local_block(&genesis).expect("genesis");
    context.connect_local_block(&spendable).expect("spendable");
    let transaction = spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("txid"),
        499_999_000,
    );
    let transaction_hex = encode_hex(
        &encode_transaction(&transaction, TransactionEncoding::WithWitness).expect("encode"),
    );
    let expected_txid = encode_hex(transaction_txid(&transaction).expect("txid").as_bytes());

    // Act
    let success = dispatch(
        &mut context,
        MethodCall::SendRawTransaction(SendRawTransactionRequest {
            transaction_hex: transaction_hex.clone(),
            maybe_max_fee_rate_sat_per_kvb: None,
            maybe_max_burn_amount_sats: None,
            ignore_rejects: Vec::new(),
        }),
    )
    .expect("submit");
    let failure = dispatch(
        &mut context,
        MethodCall::SendRawTransaction(SendRawTransactionRequest {
            transaction_hex,
            maybe_max_fee_rate_sat_per_kvb: None,
            maybe_max_burn_amount_sats: None,
            ignore_rejects: Vec::new(),
        }),
    )
    .expect_err("duplicate");

    // Assert
    assert_eq!(success["txid_hex"], json!(expected_txid));
    assert_eq!(
        failure.maybe_detail.as_ref().map(|detail| detail.code),
        Some(RpcErrorCode::VerifyRejected),
    );
}

#[test]
fn sendrawtransaction_queues_internal_relay_evidence_without_propagation_claim() {
    // Arrange
    let mut context = relay_enabled_context(44);
    let genesis = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        0,
        500_000_000,
        p2sh_script(),
    );
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000, p2sh_script());
    context.connect_local_block(&genesis).expect("genesis");
    context.connect_local_block(&spendable).expect("spendable");
    context.connect_outbound_peer(44, 1).expect("outbound");
    let transaction = spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("txid"),
        499_999_000,
    );
    let transaction_hex = encode_hex(
        &encode_transaction(&transaction, TransactionEncoding::WithWitness).expect("encode"),
    );
    let submitted_transaction_hex = transaction_hex.clone();
    let expected_txid = encode_hex(transaction_txid(&transaction).expect("txid").as_bytes());

    // Act
    let success = dispatch(
        &mut context,
        MethodCall::SendRawTransaction(SendRawTransactionRequest {
            transaction_hex,
            maybe_max_fee_rate_sat_per_kvb: None,
            maybe_max_burn_amount_sats: None,
            ignore_rejects: Vec::new(),
        }),
    )
    .expect("submit");

    // Assert
    let response = success.as_object().expect("response object");
    assert_eq!(response.len(), 3);
    assert!(response.contains_key("txid_hex"));
    assert!(response.contains_key("replaced_txids"));
    assert!(response.contains_key("evicted_txids"));
    for forbidden_key in [
        "propagated",
        "broadcast",
        "public_relay",
        "production_ready",
    ] {
        assert!(!response.contains_key(forbidden_key));
    }
    let response_json = success.to_string();
    for forbidden in ["broadcast", "propagation", "public", "guaranteed"] {
        assert!(!response_json.contains(forbidden));
    }
    let evidence = context
        .latest_local_submission_evidence()
        .expect("authoritative relay evidence")
        .expect("relay evidence");
    assert_eq!(evidence.queued_count, 1);
    assert_eq!(
        evidence
            .labels
            .iter()
            .map(|label| label.as_str())
            .collect::<Vec<_>>(),
        vec!["accepted", "queued", "rebroadcast_deferred"],
    );
    let txid = transaction_txid(&transaction).expect("txid");
    let metadata = context
        .mempool_entry_metadata(&txid)
        .expect("authoritative metadata")
        .expect("accepted entry metadata");
    let MempoolAcceptanceTime::Known(accepted_at) = metadata.accepted_at else {
        panic!("expected known local acceptance time");
    };
    assert_ne!(accepted_at.unix_seconds(), 0);
    assert_eq!(metadata.origin, MempoolOrigin::Local);
    assert_eq!(metadata.relay_intent, RelayIntent::Requested);
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");
    assert_eq!(
        status["relay"]["outcome_counters"]["value"]["accepted_count"],
        json!(1)
    );
    assert_eq!(
        status["relay"]["outcome_counters"]["value"]["rebroadcast_deferred_count"],
        json!(1)
    );
    assert_eq!(
        status["relay"]["activation"]["value"]["enabled"],
        json!(true)
    );
    assert_eq!(
        status["relay"]["download_eligibility"]["value"]["eligible_peer_count"],
        json!(1)
    );
    assert_eq!(
        status["relay"]["local_submission"]["state"],
        json!("implemented")
    );
    assert_eq!(
        status["relay"]["rebroadcast"]["state"],
        json!("implemented")
    );
    let status_json = serde_json::to_string(&status).expect("network status json");
    assert!(!status_json.contains(&submitted_transaction_hex));
    assert!(!status_json.contains(&expected_txid));
}

#[test]
fn sendrawtransaction_duplicate_does_not_queue_new_fanout() {
    // Arrange
    let mut context = relay_enabled_context(45);
    let genesis = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        0,
        500_000_000,
        p2sh_script(),
    );
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000, p2sh_script());
    context.connect_local_block(&genesis).expect("genesis");
    context.connect_local_block(&spendable).expect("spendable");
    context.connect_outbound_peer(45, 1).expect("outbound");
    let transaction = spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("txid"),
        499_999_000,
    );
    let transaction_hex = encode_hex(
        &encode_transaction(&transaction, TransactionEncoding::WithWitness).expect("encode"),
    );
    dispatch(
        &mut context,
        MethodCall::SendRawTransaction(SendRawTransactionRequest {
            transaction_hex: transaction_hex.clone(),
            maybe_max_fee_rate_sat_per_kvb: None,
            maybe_max_burn_amount_sats: None,
            ignore_rejects: Vec::new(),
        }),
    )
    .expect("initial submit");

    // Act
    let duplicate = dispatch(
        &mut context,
        MethodCall::SendRawTransaction(SendRawTransactionRequest {
            transaction_hex,
            maybe_max_fee_rate_sat_per_kvb: None,
            maybe_max_burn_amount_sats: None,
            ignore_rejects: Vec::new(),
        }),
    )
    .expect_err("duplicate");

    // Assert
    assert_eq!(
        duplicate.maybe_detail.as_ref().map(|detail| detail.code),
        Some(RpcErrorCode::VerifyRejected),
    );
    let evidence = context
        .latest_local_submission_evidence()
        .expect("authoritative relay evidence")
        .expect("relay evidence");
    assert_eq!(evidence.queued_count, 0);
    assert_eq!(
        evidence
            .labels
            .iter()
            .map(|label| label.as_str())
            .collect::<Vec<_>>(),
        vec!["duplicate"],
    );
}

#[test]
fn sendrawtransaction_explicit_time_stores_local_requested_metadata() {
    // Arrange
    let mut context = relay_enabled_context(46);
    let genesis = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        0,
        500_000_000,
        p2sh_script(),
    );
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000, p2sh_script());
    context.connect_local_block(&genesis).expect("genesis");
    context.connect_local_block(&spendable).expect("spendable");
    let transaction = spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("txid"),
        499_999_000,
    );
    let txid = transaction_txid(&transaction).expect("txid");
    let expected_txid = encode_hex(txid.as_bytes());

    // Act
    let outcome = context
        .submit_local_transaction_with_relay_evidence_at(transaction, 60)
        .expect("explicit-time local submit");

    // Assert
    assert!(matches!(
        outcome,
        open_bitcoin_node::core::mempool::MempoolOutcome::Accepted { .. }
    ));
    let metadata = context
        .mempool_entry_metadata(&txid)
        .expect("authoritative metadata")
        .expect("accepted entry metadata");
    assert_eq!(
        metadata.accepted_at,
        MempoolAcceptanceTime::Known(PolicyTime::new(60))
    );
    assert_eq!(metadata.origin, MempoolOrigin::Local);
    assert_eq!(metadata.relay_intent, RelayIntent::Requested);
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");
    let status_json = serde_json::to_string(&status).expect("network status json");
    assert!(!status_json.contains(&expected_txid));
    assert!(!status_json.contains("accepted_at"));
    assert!(!status_json.contains("relay_intent"));
    assert!(!status_json.contains("MempoolOrigin"));
}

#[test]
fn sendrawtransaction_clock_before_epoch_returns_typed_internal_error() {
    // Arrange
    let before_epoch = UNIX_EPOCH
        .checked_sub(Duration::from_secs(1))
        .expect("construct pre-epoch clock");

    // Act
    let failure = super::node::timestamp_unix_seconds_from_system_time(before_epoch)
        .expect_err("pre-epoch clock must fail closed");

    // Assert
    assert_eq!(failure.kind, RpcFailureKind::InternalError);
    let detail = failure.maybe_detail.expect("internal detail");
    assert_eq!(detail.code, RpcErrorCode::InternalError);
    assert_eq!(
        detail.message,
        "system clock is unavailable for local transaction acceptance",
    );
}

#[test]
fn sendrawtransaction_rejects_unenforced_fee_limits_before_mempool_submission() {
    // Arrange
    let mut context = empty_context();

    // Act
    let fee_limit_failure = dispatch(
        &mut context,
        MethodCall::SendRawTransaction(SendRawTransactionRequest {
            transaction_hex: "not hex".to_string(),
            maybe_max_fee_rate_sat_per_kvb: Some(1),
            maybe_max_burn_amount_sats: None,
            ignore_rejects: Vec::new(),
        }),
    )
    .expect_err("maxfeerate");
    let burn_limit_failure = dispatch(
        &mut context,
        MethodCall::SendRawTransaction(SendRawTransactionRequest {
            transaction_hex: "not hex".to_string(),
            maybe_max_fee_rate_sat_per_kvb: None,
            maybe_max_burn_amount_sats: Some(1),
            ignore_rejects: Vec::new(),
        }),
    )
    .expect_err("maxburnamount");
    let mempool = dispatch(
        &mut context,
        MethodCall::GetMempoolInfo(GetMempoolInfoRequest::default()),
    )
    .expect("mempool");

    // Assert
    let fee_detail = fee_limit_failure.maybe_detail.expect("fee detail");
    assert_eq!(fee_detail.code, RpcErrorCode::InvalidParams);
    assert_eq!(
        fee_detail.message,
        "sendrawtransaction maxfeerate enforcement is not supported in Phase 8; omit maxfeerate",
    );
    let burn_detail = burn_limit_failure.maybe_detail.expect("burn detail");
    assert_eq!(burn_detail.code, RpcErrorCode::InvalidParams);
    assert_eq!(
        burn_detail.message,
        "sendrawtransaction maxburnamount enforcement is not supported in Phase 8; omit maxburnamount",
    );
    assert_eq!(mempool["size"], json!(0));
}

#[test]
fn buildandsigntransaction_returns_deterministic_hex_and_fee() {
    // Arrange
    let mut context = funded_wallet_context();
    let request = BuildAndSignTransactionRequest {
        recipients: vec![TransactionRecipient {
            script_pubkey_hex: "51".to_string(),
            amount_sats: 30_000,
        }],
        fee_rate_sat_per_kvb: 2_000,
        maybe_change_descriptor_id: None,
        maybe_lock_time: None,
        enable_rbf: true,
    };

    // Act
    let first = dispatch(
        &mut context,
        MethodCall::BuildAndSignTransaction(request.clone()),
    )
    .expect("first");
    let second =
        dispatch(&mut context, MethodCall::BuildAndSignTransaction(request)).expect("second");

    // Assert
    assert_eq!(first, second);
    assert_eq!(first["fee_sats"], json!(242));
    assert!(first["transaction_hex"].as_str().expect("hex").len() > 10);
    assert_eq!(first["inputs"][0]["amount_sats"], json!(75_000));
}

#[test]
fn sendtoaddress_reuses_the_build_and_sign_spend_path() {
    // Arrange
    let mut context = spendable_send_context();
    let destination_script = SingleKeyDescriptor::parse(
        "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)#8fhd9pwu",
        AddressNetwork::Regtest,
    )
    .expect("descriptor")
    .address(AddressNetwork::Regtest)
    .expect("destination address")
    .script_pubkey;
    let build_request = BuildAndSignTransactionRequest {
        recipients: vec![TransactionRecipient {
            script_pubkey_hex: encode_hex(destination_script.as_bytes()),
            amount_sats: 30_000,
        }],
        fee_rate_sat_per_kvb: 2_000,
        maybe_change_descriptor_id: None,
        maybe_lock_time: None,
        enable_rbf: true,
    };

    // Act
    let built = dispatch(
        &mut context,
        MethodCall::BuildAndSignTransaction(build_request),
    )
    .expect("build and sign");
    let expected_transaction = parse_transaction(&decode_hex(
        built["transaction_hex"].as_str().expect("transaction hex"),
    ))
    .expect("parse built transaction");
    let send = dispatch(
        &mut context,
        MethodCall::SendToAddress(SendToAddressRequest {
            address: "bcrt1qa0qwuze2h85zw7nqpsj3ga0z9geyrgwpf2m8je".to_string(),
            amount_sats: 30_000,
            maybe_fee_rate_sat_per_kvb: Some(2_000),
            maybe_conf_target: None,
            maybe_estimate_mode: None,
            maybe_change_descriptor_id: None,
            maybe_lock_time: None,
            enable_rbf: true,
            maybe_max_tx_fee_sats: Some(1_000),
        }),
    )
    .expect("sendtoaddress");

    // Assert
    assert_eq!(
        send,
        json!(encode_hex(
            transaction_txid(&expected_transaction)
                .expect("expected txid")
                .as_bytes()
        )),
    );
}
