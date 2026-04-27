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

use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::json;

use open_bitcoin_node::{
    FjallNodeStore, PersistMode, WalletRegistry,
    core::{
        chainstate::{ChainPosition, ChainstateSnapshot, Coin},
        codec::{TransactionEncoding, encode_transaction, parse_transaction},
        consensus::{
            block_hash, block_merkle_root, check_block_header, crypto::hash160, transaction_txid,
        },
        network::WireNetworkMessage,
        primitives::{
            Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
            TransactionInput, TransactionOutput, Txid,
        },
        wallet::{AddressNetwork, DescriptorRole, SingleKeyDescriptor, Wallet},
    },
};

use crate::{
    ManagedRpcContext, RpcErrorCode,
    config::{RuntimeConfig, WalletRuntimeConfig},
    dispatch::dispatch,
    method::{
        BuildAndSignTransactionRequest, DeriveAddressesRequest, GetBalancesRequest,
        GetBlockchainInfoRequest, GetMempoolInfoRequest, GetNetworkInfoRequest,
        GetWalletInfoRequest, ImportDescriptorsRequest, ListUnspentRequest, MethodCall,
        RescanBlockchainRequest, SendRawTransactionRequest, SendToAddressRequest,
        TransactionRecipient,
    },
};

const EASY_BITS: u32 = 0x207f_ffff;
const RANGED_TPRV: &str = "tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK";
const RANGED_TPUB: &str = "tpubD6NzVbkrYhZ4WaWSyoBvQwbpLkojyoTZPRsgXELWz3Popb3qkjcJyJUGLnL4qHHoQvao8ESaAstxYSnhyswJ76uZPStJRJCTKvosUCJZL5B";

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("script")
}

fn redeem_script() -> ScriptBuf {
    script(&[0x51])
}

fn p2sh_script() -> ScriptBuf {
    let redeem_hash = hash160(redeem_script().as_bytes());
    let mut bytes = vec![0xa9, 20];
    bytes.extend_from_slice(&redeem_hash);
    bytes.push(0x87);
    script(&bytes)
}

fn sample_tip(height: u32) -> ChainPosition {
    ChainPosition::new(
        BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: Default::default(),
            time: 1_700_000_000 + height,
            bits: EASY_BITS,
            nonce: 1,
        },
        height,
        1,
        i64::from(1_700_000_000 + height),
    )
}

fn wallet_with_descriptors() -> Wallet {
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    wallet
        .import_descriptor(
            "receive",
            DescriptorRole::External,
            "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
        )
        .expect("receive descriptor");
    wallet
        .import_descriptor(
            "change",
            DescriptorRole::Internal,
            "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))",
        )
        .expect("change descriptor");
    wallet
}

fn funded_snapshot(wallet: &Wallet) -> ChainstateSnapshot {
    let receive_script = wallet
        .default_receive_address()
        .expect("receive address")
        .script_pubkey;
    let mut utxos = HashMap::new();
    utxos.insert(
        OutPoint {
            txid: Txid::from_byte_array([7_u8; 32]),
            vout: 0,
        },
        Coin {
            output: TransactionOutput {
                value: Amount::from_sats(75_000).expect("amount"),
                script_pubkey: receive_script,
            },
            is_coinbase: false,
            created_height: 9,
            created_median_time_past: 1_700_000_009,
        },
    );

    ChainstateSnapshot::new(vec![sample_tip(10)], utxos, Default::default())
}

fn serialized_script_num(value: i64) -> Vec<u8> {
    if value == 0 {
        return vec![0x00];
    }

    let mut magnitude = value as u64;
    let mut encoded = Vec::new();
    while magnitude > 0 {
        encoded.push((magnitude & 0xff) as u8);
        magnitude >>= 8;
    }

    let mut script = Vec::with_capacity(encoded.len() + 2);
    script.push(encoded.len() as u8);
    script.extend(encoded);
    script.push(0x51);
    script
}

fn coinbase_transaction(height: u32, value: i64, script_pubkey: ScriptBuf) -> Transaction {
    let mut script_sig = serialized_script_num(i64::from(height));
    script_sig.push(0x51);
    Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: script(&script_sig),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value).expect("amount"),
            script_pubkey,
        }],
        lock_time: 0,
    }
}

fn mine_header(block: &mut Block) {
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("nonce");
}

fn build_block(
    previous_block_hash: BlockHash,
    height: u32,
    value: i64,
    script_pubkey: ScriptBuf,
) -> Block {
    let transactions = vec![coinbase_transaction(height, value, script_pubkey)];
    let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
    assert!(!maybe_mutated);

    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash,
            merkle_root,
            time: 1_231_006_500 + height,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions,
    };
    mine_header(&mut block);
    block
}

fn spend_transaction(previous_txid: Txid, value: i64) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout: 0,
            },
            script_sig: script(&[0x01, 0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value).expect("amount"),
            script_pubkey: p2sh_script(),
        }],
        lock_time: 0,
    }
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn decode_hex(text: &str) -> Vec<u8> {
    let trimmed = text.trim();
    trimmed
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = char::from(pair[0]).to_digit(16).expect("hex") as u8;
            let low = char::from(pair[1]).to_digit(16).expect("hex") as u8;
            (high << 4) | low
        })
        .collect()
}

fn empty_context() -> ManagedRpcContext {
    ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Regtest,
        wallet: WalletRuntimeConfig {
            coinbase_maturity: 1,
            ..WalletRuntimeConfig::default()
        },
        ..RuntimeConfig::default()
    })
}

fn funded_wallet_context() -> ManagedRpcContext {
    let mut context = empty_context();
    context
        .import_descriptor(
            "receive",
            DescriptorRole::External,
            "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
        )
        .expect("receive");
    context
        .import_descriptor(
            "change",
            DescriptorRole::Internal,
            "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))",
        )
        .expect("change");
    let snapshot = funded_snapshot(&wallet_with_descriptors());
    context.rescan_wallet(&snapshot).expect("rescan");
    context
}

fn spendable_send_context() -> ManagedRpcContext {
    let mut context = empty_context();
    context
        .import_descriptor(
            "receive",
            DescriptorRole::External,
            "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
        )
        .expect("receive");
    context
        .import_descriptor(
            "change",
            DescriptorRole::Internal,
            "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))",
        )
        .expect("change");
    let receive_script = context
        .descriptor_address(0)
        .expect("receive address")
        .script_pubkey;
    let genesis = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        0,
        500_000_000,
        p2sh_script(),
    );
    let funding = build_block(block_hash(&genesis.header), 1, 75_000, receive_script);
    context.connect_local_block(&genesis).expect("genesis");
    context.connect_local_block(&funding).expect("funding");
    let snapshot = context.blockchain_snapshot();
    context.rescan_wallet(&snapshot).expect("rescan");
    context
}

fn temp_store_path(test_name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "open-bitcoin-rpc-dispatch-{test_name}-{}-{timestamp}",
        std::process::id()
    ))
}

fn remove_dir_if_exists(path: &Path) {
    match fs::remove_dir_all(path) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => panic!("failed to remove {}: {error}", path.display()),
    }
}

fn durable_wallet_context(test_name: &str, wallet_name: &str) -> ManagedRpcContext {
    let path = temp_store_path(test_name);
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut registry = WalletRegistry::default();
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    wallet
        .import_descriptor(
            "receive-ranged",
            DescriptorRole::External,
            &format!("wpkh({RANGED_TPRV}/1/1/*)"),
        )
        .expect("receive descriptor");
    wallet
        .import_descriptor(
            "change-ranged",
            DescriptorRole::Internal,
            &format!("sh(wpkh({RANGED_TPUB}/1/*))"),
        )
        .expect("change descriptor");
    registry
        .create_wallet(&store, wallet_name.to_string(), wallet, PersistMode::Sync)
        .expect("create wallet");
    drop(store);

    let mut context = ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Regtest,
        maybe_data_dir: Some(path),
        wallet: WalletRuntimeConfig {
            coinbase_maturity: 1,
            ..WalletRuntimeConfig::default()
        },
        ..RuntimeConfig::default()
    });
    context.set_request_wallet_name(Some(wallet_name.to_string()));
    context
}

fn node_context_with_chain_and_mempool() -> ManagedRpcContext {
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
    context.add_inbound_peer(7).expect("peer");
    context
        .receive_network_message(7, WireNetworkMessage::WtxidRelay, 1)
        .expect("wtxidrelay");
    context
        .receive_network_message(7, WireNetworkMessage::SendHeaders, 1)
        .expect("sendheaders");
    context.connect_outbound_peer(8, 2).expect("outbound");
    let transaction = spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("txid"),
        499_999_000,
    );
    context
        .submit_local_transaction(transaction)
        .expect("submit");
    context
}

#[test]
fn node_info_methods_return_documented_phase_8_fields() {
    // Arrange
    let mut context = node_context_with_chain_and_mempool();

    // Act
    let network = dispatch(
        &mut context,
        MethodCall::GetNetworkInfo(GetNetworkInfoRequest::default()),
    )
    .expect("network");
    let blockchain = dispatch(
        &mut context,
        MethodCall::GetBlockchainInfo(GetBlockchainInfoRequest::default()),
    )
    .expect("blockchain");
    let mempool = dispatch(
        &mut context,
        MethodCall::GetMempoolInfo(GetMempoolInfoRequest::default()),
    )
    .expect("mempool");

    // Assert
    assert_eq!(network["connections"], json!(2));
    assert_eq!(network["connections_in"], json!(1));
    assert_eq!(network["connections_out"], json!(1));
    assert_eq!(network["localrelay"], json!(true));
    assert_eq!(blockchain["chain"], json!("regtest"));
    assert_eq!(blockchain["blocks"], json!(1));
    assert_eq!(blockchain["headers"], json!(1));
    assert_eq!(blockchain["mediantime"], json!(1231006501));
    assert_eq!(mempool["size"], json!(1));
    assert_eq!(mempool["total_fee_sats"], json!(1000));
    assert_eq!(mempool["loaded"], json!(true));
}

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
