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
use super::*;
use crate::package_projection::{
    admission_state_for_member, submit_relay_state_for_member, PackageAdmissionState,
    PackageRelayState, PackageSubmitRelayFacts,
};
use open_bitcoin_mempool::{
    EffectiveFeeGroupId, MempoolMemberIdentity, NewlyPresent, PackageMemberResult,
};

const FORBIDDEN_PROPAGATION_KEYS: [&str; 3] = ["propagated", "broadcast", "public_relay"];

fn spendable_package_hex() -> (ManagedRpcContext, String) {
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
        transaction_txid(&genesis.transactions[0]).expect("genesis txid"),
        499_999_000,
    );
    let transaction_hex = encode_hex(
        &encode_transaction(&transaction, TransactionEncoding::WithWitness).expect("encode"),
    );
    (context, transaction_hex)
}

fn submit_package(raw_txs: Vec<String>) -> OpenBitcoinPackageRequest {
    OpenBitcoinPackageRequest {
        mode: OpenBitcoinPackageMode::Submit,
        raw_txs,
    }
}

fn group_count(status: &serde_json::Value, group: &str, field: &str) -> u64 {
    status["mempool"][group]["value"][field]
        .as_u64()
        .unwrap_or_else(|| panic!("mempool.{group}.{field}"))
}

fn collect_object_keys(value: &serde_json::Value, keys: &mut Vec<String>) {
    let Some(object) = value.as_object() else {
        if let Some(array) = value.as_array() {
            for child in array {
                collect_object_keys(child, keys);
            }
        }
        return;
    };
    for (key, child) in object {
        keys.push(key.clone());
        collect_object_keys(child, keys);
    }
}

fn assert_no_propagation_keys(value: &serde_json::Value, surface: &str) {
    let mut keys = Vec::new();
    collect_object_keys(value, &mut keys);
    for forbidden in FORBIDDEN_PROPAGATION_KEYS {
        assert!(
            !keys.iter().any(|key| key == forbidden),
            "{surface} must not serialize {forbidden}"
        );
    }
}

fn assert_group_has_no_propagation_keys(status: &serde_json::Value, group: &str) {
    let object = status["mempool"][group]["value"]
        .as_object()
        .unwrap_or_else(|| panic!("mempool.{group}.value"));
    for forbidden in FORBIDDEN_PROPAGATION_KEYS {
        assert!(
            !object.contains_key(forbidden),
            "mempool.{group} must not claim {forbidden}"
        );
    }
}

#[test]
fn openbitcoinpackage_relay_disabled_accept_is_still_present() {
    // Arrange
    let (mut context, transaction_hex) = spendable_package_hex();

    // Act
    let accepted = dispatch(
        &mut context,
        MethodCall::OpenBitcoinPackage(submit_package(vec![transaction_hex.clone()])),
    )
    .expect("relay-disabled submit");
    let still_present = dispatch(
        &mut context,
        MethodCall::OpenBitcoinPackage(submit_package(vec![transaction_hex])),
    )
    .expect("already-present submit");
    let mempool = dispatch(
        &mut context,
        MethodCall::GetMempoolInfo(GetMempoolInfoRequest::default()),
    )
    .expect("mempool after accept");

    // Assert
    let first_admission = accepted["members"][0]["admission"]
        .as_str()
        .expect("first admission");
    assert!(
        first_admission == "accepted" || first_admission == "still-present",
        "relay-disabled submit must stay on the local admission axis, got {first_admission}"
    );
    assert_eq!(accepted["members"][0]["relay"], json!("relay_disabled"));
    assert_eq!(
        still_present["members"][0]["admission"],
        json!("still-present")
    );
    assert_eq!(
        still_present["members"][0]["relay"],
        json!("relay_disabled")
    );
    assert!(
        mempool["size"].as_u64().expect("size") >= 1,
        "accepted member must remain in the local mempool"
    );
}

#[test]
fn openbitcoinnetworkstatus_has_no_package_member_table() {
    // Arrange
    let (mut context, transaction_hex) = spendable_package_hex();
    let before = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("status before");

    // Act
    dispatch(
        &mut context,
        MethodCall::OpenBitcoinPackage(submit_package(vec![transaction_hex])),
    )
    .expect("relay-disabled submit");
    let after = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("status after");

    // Assert
    let admission_changed = group_count(&before, "admission", "accepted")
        != group_count(&after, "admission", "accepted")
        || group_count(&before, "admission", "still_present")
            != group_count(&after, "admission", "still_present");
    let retry_changed = group_count(&before, "retry", "relay_disabled")
        != group_count(&after, "retry", "relay_disabled")
        || group_count(&before, "retry", "eligible") != group_count(&after, "retry", "eligible")
        || group_count(&before, "retry", "cleared") != group_count(&after, "retry", "cleared");
    assert!(
        admission_changed || retry_changed,
        "shared aggregates must move after a local package submit"
    );
    assert_eq!(group_count(&after, "retry", "relay_disabled"), 1);
    let mempool = after["mempool"].as_object().expect("mempool object");
    for forbidden in ["members", "fingerprint", "txid", "wtxid"] {
        assert!(
            !mempool.contains_key(forbidden),
            "openbitcoinnetworkstatus.mempool must not echo {forbidden}"
        );
    }
}

#[test]
fn dual_state_json_forbids_propagation_keys() {
    // Arrange
    let (mut context, transaction_hex) = spendable_package_hex();

    // Act
    let package = dispatch(
        &mut context,
        MethodCall::OpenBitcoinPackage(submit_package(vec![transaction_hex])),
    )
    .expect("typed package");
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");

    // Assert
    assert_no_propagation_keys(&package, "openbitcoinpackage");
    let mempool = status["mempool"].as_object().expect("mempool object");
    for forbidden in FORBIDDEN_PROPAGATION_KEYS {
        assert!(
            !mempool.contains_key(forbidden),
            "openbitcoinnetworkstatus.mempool must not claim {forbidden}"
        );
    }
    assert_group_has_no_propagation_keys(&status, "admission");
    assert_group_has_no_propagation_keys(&status, "retry");
}

#[test]
fn served_is_not_cleared() {
    // Arrange
    let member = PackageMemberResult::FinallyPresent(NewlyPresent {
        requested: MempoolMemberIdentity {
            txid: Txid::from_byte_array([0x11; 32]),
            wtxid: Wtxid::from_byte_array([0x22; 32]),
        },
        effective_fee_group_id: EffectiveFeeGroupId::from_u64(1),
    });
    let facts = PackageSubmitRelayFacts {
        served: true,
        ..PackageSubmitRelayFacts::default()
    };

    // Act
    let admission = admission_state_for_member(&member);
    let relay = submit_relay_state_for_member(&member, false, facts);

    // Assert
    assert_eq!(admission, PackageAdmissionState::Accepted);
    assert_eq!(relay, PackageRelayState::Served);
    assert_ne!(admission, PackageAdmissionState::Cleared);
}
