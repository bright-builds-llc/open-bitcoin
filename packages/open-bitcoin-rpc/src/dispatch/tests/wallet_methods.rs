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
use super::storage_fixtures::*;
use super::wallet_fixtures::*;
use super::*;

#[test]
fn deriveaddresses_returns_expected_addresses_for_supported_descriptors() {
    // Arrange
    let mut context = empty_context();
    let request = DeriveAddressesRequest {
        descriptor: "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)#8fhd9pwu"
            .to_string(),
        maybe_range: None,
    };

    // Act
    let response = dispatch(&mut context, MethodCall::DeriveAddresses(request)).expect("derive");

    // Assert
    assert_eq!(
        response,
        json!({
            "addresses": ["bcrt1qa0qwuze2h85zw7nqpsj3ga0z9geyrgwpf2m8je"]
        })
    );
}

#[test]
fn getwalletinfo_reports_wallet_identity_and_freshness_fields() {
    // Arrange
    let mut context = funded_wallet_context();

    // Act
    let response = dispatch(
        &mut context,
        MethodCall::GetWalletInfo(GetWalletInfoRequest::default()),
    )
    .expect("wallet info");

    // Assert
    assert_eq!(response["network"], json!("regtest"));
    assert_eq!(response["descriptor_count"], json!(2));
    assert_eq!(response["utxo_count"], json!(1));
    assert_eq!(response["maybe_tip_height"], json!(10));
    assert_eq!(
        response["maybe_tip_median_time_past"],
        json!(1700000010_i64)
    );
    assert_eq!(response["walletname"], json!(null));
    assert_eq!(response["scanning"], json!(false));
    assert_eq!(response["freshness"], json!("fresh"));
}

#[test]
fn wallet_descriptor_and_rescan_methods_update_wallet_views() {
    // Arrange
    let mut context = empty_context();
    let import_request = ImportDescriptorsRequest {
        requests: vec![
            crate::method::DescriptorImportItem {
                descriptor: "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)"
                    .to_string(),
                label: "receive".to_string(),
                internal: false,
                maybe_rescan_since_height: Some(0),
            },
            crate::method::DescriptorImportItem {
                descriptor: "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))"
                    .to_string(),
                label: "change".to_string(),
                internal: true,
                maybe_rescan_since_height: Some(0),
            },
        ],
    };
    let reference_wallet = wallet_with_descriptors();
    let receive_script = reference_wallet
        .default_receive_address()
        .expect("receive")
        .script_pubkey;
    let genesis = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        0,
        75_000,
        receive_script,
    );
    context.connect_local_block(&genesis).expect("genesis");

    // Act
    let import_response =
        dispatch(&mut context, MethodCall::ImportDescriptors(import_request)).expect("import");
    let rescan_response = dispatch(
        &mut context,
        MethodCall::RescanBlockchain(RescanBlockchainRequest {
            maybe_start_height: Some(0),
            maybe_stop_height: Some(0),
        }),
    )
    .expect("rescan");
    let balances = dispatch(
        &mut context,
        MethodCall::GetBalances(GetBalancesRequest::default()),
    )
    .expect("balances");
    let unspent = dispatch(
        &mut context,
        MethodCall::ListUnspent(ListUnspentRequest::default()),
    )
    .expect("listunspent");

    // Assert
    assert_eq!(import_response["results"][0]["success"], json!(true));
    assert_eq!(rescan_response["start_height"], json!(0));
    assert_eq!(rescan_response["stop_height"], json!(0));
    assert_eq!(balances["mine"]["trusted_sats"], json!(75_000));
    assert_eq!(balances["mine"]["immature_sats"], json!(0));
    assert_eq!(unspent["entries"][0]["descriptor_id"], json!(0));
    assert_eq!(unspent["entries"][0]["amount_sats"], json!(75_000));
}

#[test]
fn durable_wallet_methods_persist_address_cursors_and_descriptor_metadata() {
    // Arrange
    let mut context = durable_wallet_context("descriptor-cursors", "alpha");

    // Act
    let first_receive = dispatch(&mut context, MethodCall::GetNewAddress(Default::default()))
        .expect("first receive");
    let second_receive = dispatch(&mut context, MethodCall::GetNewAddress(Default::default()))
        .expect("second receive");
    let change = dispatch(
        &mut context,
        MethodCall::GetRawChangeAddress(Default::default()),
    )
    .expect("change");
    let descriptors = dispatch(
        &mut context,
        MethodCall::ListDescriptors(Default::default()),
    )
    .expect("descriptors");
    let wallet_info = dispatch(
        &mut context,
        MethodCall::GetWalletInfo(GetWalletInfoRequest::default()),
    )
    .expect("wallet info");

    // Assert
    assert_ne!(first_receive, second_receive);
    assert_ne!(first_receive, change);
    assert_eq!(descriptors["walletname"], json!("alpha"));
    assert_eq!(descriptors["descriptors"][0]["internal"], json!(false));
    assert_eq!(descriptors["descriptors"][0]["maybe_next_index"], json!(2));
    assert_eq!(descriptors["descriptors"][1]["internal"], json!(true));
    assert_eq!(descriptors["descriptors"][1]["maybe_next_index"], json!(1));
    assert_eq!(wallet_info["walletname"], json!("alpha"));
    assert_eq!(wallet_info["freshness"], json!("fresh"));
}

#[test]
fn rescanblockchain_accepts_ranges_and_records_partial_freshness() {
    // Arrange
    let mut context = durable_wallet_context("range-rescan", "alpha");
    let receive_script = context
        .descriptor_address(0)
        .expect("receive address")
        .script_pubkey;
    let genesis = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        0,
        75_000,
        receive_script.clone(),
    );
    let block_one = build_block(block_hash(&genesis.header), 1, 75_000, receive_script);
    let block_two = build_block(block_hash(&block_one.header), 2, 75_000, p2sh_script());
    context.connect_local_block(&genesis).expect("genesis");
    context.connect_local_block(&block_one).expect("block one");
    context.connect_local_block(&block_two).expect("block two");

    // Act
    let partial_rescan = dispatch(
        &mut context,
        MethodCall::RescanBlockchain(RescanBlockchainRequest {
            maybe_start_height: Some(1),
            maybe_stop_height: Some(1),
        }),
    )
    .expect("partial range");
    let wallet_info_after_partial = dispatch(
        &mut context,
        MethodCall::GetWalletInfo(GetWalletInfoRequest::default()),
    )
    .expect("wallet info after partial");
    let full_rescan = dispatch(
        &mut context,
        MethodCall::RescanBlockchain(RescanBlockchainRequest {
            maybe_start_height: Some(0),
            maybe_stop_height: Some(2),
        }),
    )
    .expect("full rescan");

    // Assert
    assert_eq!(partial_rescan["start_height"], json!(1));
    assert_eq!(partial_rescan["stop_height"], json!(1));
    assert_eq!(partial_rescan["freshness"], json!("partial"));
    assert_eq!(wallet_info_after_partial["freshness"], json!("partial"));
    assert_eq!(wallet_info_after_partial["walletname"], json!("alpha"));
    assert_eq!(full_rescan["freshness"], json!("fresh"));
    assert_eq!(full_rescan["maybe_scanned_through_height"], json!(2));
}
