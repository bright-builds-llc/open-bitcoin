// Parity breadcrumbs:
// - packages/bitcoin-knots/src/rpc/mempool.cpp

use serde_json::{Value, json};

use open_bitcoin_mempool::{
    EffectiveFeeGroup, EffectiveFeeGroupId, ExistingMember, FeeRate, HardMemberFailure,
    MempoolRejectionCategory, NewlyPresent, PackageMemberResult, PackageReport, PackageStatus,
    PostTrimAbsence, PriorMemberSuccess, TransactionVirtualSize, WellFormedPackage, WitnessAlias,
};
use open_bitcoin_node::core::consensus::{transaction_txid, transaction_wtxid};
use open_bitcoin_node::core::primitives::{
    Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
    Txid, Wtxid,
};

use super::{
    PackageAdmissionState, PackageMemberDualState, PackageMemberProjectionFacts, PackageRelayState,
    project_open_bitcoin_package, project_submitpackage, project_testmempoolaccept,
};

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn transaction_with_input(seed: u8) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([seed; 32]),
                vout: u32::from(seed),
            },
            script_sig: ScriptBuf::default(),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(i64::from(seed) + 1).expect("fixture amount"),
            script_pubkey: ScriptBuf::default(),
        }],
        lock_time: u32::from(seed),
    }
}

fn child_of(parent: &Transaction, seed: u8) -> Transaction {
    let parent_txid = transaction_txid(parent).expect("fixture parent txid");
    Transaction {
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: parent_txid,
                vout: 0,
            },
            ..transaction_with_input(seed).inputs[0].clone()
        }],
        ..transaction_with_input(seed)
    }
}

fn singleton_package(seed: u8) -> WellFormedPackage {
    WellFormedPackage::try_from(vec![transaction_with_input(seed)]).expect("singleton package")
}

fn parent_child_package() -> WellFormedPackage {
    let parent = transaction_with_input(70);
    let child = child_of(&parent, 71);
    WellFormedPackage::try_from(vec![parent, child]).expect("parent-child package")
}

fn identity_at(
    package: &WellFormedPackage,
    index: usize,
) -> open_bitcoin_mempool::MempoolMemberIdentity {
    package
        .maybe_identity_at(index)
        .expect("fixture identity at index")
}

fn fee_group(id: EffectiveFeeGroupId, ordered_wtxids: Vec<Wtxid>) -> EffectiveFeeGroup {
    let virtual_size = TransactionVirtualSize::new(100);
    EffectiveFeeGroup::try_new(
        id,
        ordered_wtxids,
        Amount::from_sats(200).expect("valid base fee"),
        Amount::from_sats(300).expect("valid modified fee"),
        virtual_size,
        FeeRate::from_fee_sats_and_vbytes(300, virtual_size),
    )
    .expect("checked fee group")
}

fn facts_for(
    identity: open_bitcoin_mempool::MempoolMemberIdentity,
    virtual_size: u64,
    base_fee_sats: i64,
) -> PackageMemberProjectionFacts {
    PackageMemberProjectionFacts {
        txid: identity.txid,
        wtxid: identity.wtxid,
        virtual_size,
        base_fee_sats,
    }
}

fn finally_present_report() -> (PackageReport, Vec<PackageMemberProjectionFacts>, Wtxid) {
    let package = singleton_package(11);
    let requested = identity_at(&package, 0);
    let group_id = EffectiveFeeGroupId::from_u64(1);
    let report = PackageReport::try_new(
        &package,
        PackageStatus::Complete,
        vec![PackageMemberResult::FinallyPresent(NewlyPresent {
            requested,
            effective_fee_group_id: group_id,
        })],
        vec![fee_group(group_id, vec![requested.wtxid])],
    )
    .expect("finally-present report");
    let facts = vec![facts_for(requested, 141, 200)];
    (report, facts, requested.wtxid)
}

fn collect_object_keys(value: &Value, keys: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                keys.push(key.clone());
                collect_object_keys(child, keys);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_object_keys(item, keys);
            }
        }
        _ => {}
    }
}

fn first_element(value: &Value) -> &Value {
    value
        .as_array()
        .expect("testmempoolaccept array")
        .first()
        .expect("at least one member")
}

#[test]
fn testmempoolaccept_finally_present_uses_knots_allowed_keys() {
    // Arrange
    let (report, facts, wtxid) = finally_present_report();
    let expected_includes = vec![json!(encode_hex(wtxid.as_bytes()))];

    // Act
    let projected = project_testmempoolaccept(&report, &facts).expect("project");

    // Assert
    let element = first_element(&projected);
    assert_eq!(element["allowed"], json!(true));
    assert_eq!(element["vsize"], json!(141));
    assert!(element["fees"]["base"].is_number());
    assert!(element["fees"]["effective-feerate"].is_number());
    assert_eq!(
        element["fees"]["effective-includes"],
        json!(expected_includes)
    );
    assert!(element.get("package-error").is_none());
    assert!(element.get("reject-reason").is_none());
}

#[test]
fn testmempoolaccept_already_present_is_reject_reason_not_mempool_entry() {
    // Arrange
    let package = singleton_package(12);
    let requested = identity_at(&package, 0);
    let report = PackageReport::try_new(
        &package,
        PackageStatus::Complete,
        vec![PackageMemberResult::AlreadyPresent(ExistingMember {
            requested,
        })],
        vec![],
    )
    .expect("already-present report");

    // Act
    let projected = project_testmempoolaccept(&report, &[]).expect("project");

    // Assert
    let element = first_element(&projected);
    assert_eq!(element["allowed"], json!(false));
    assert_eq!(element["reject-reason"], json!("txn-already-in-mempool"));
    assert!(element.get("vsize").is_none());
    assert!(element.get("fees").is_none());
}

#[test]
fn testmempoolaccept_same_txid_different_witness_is_allowed_false() {
    // Arrange
    let package = singleton_package(13);
    let requested = identity_at(&package, 0);
    let existing_wtxid = transaction_wtxid(&transaction_with_input(99)).expect("other wtxid");
    let report = PackageReport::try_new(
        &package,
        PackageStatus::Complete,
        vec![PackageMemberResult::SameTxidDifferentWitness(
            WitnessAlias {
                requested,
                existing_wtxid,
            },
        )],
        vec![],
    )
    .expect("witness-alias report");

    // Act
    let projected = project_testmempoolaccept(&report, &[]).expect("project");

    // Assert
    let element = first_element(&projected);
    assert_eq!(element["allowed"], json!(false));
    assert_eq!(
        element["reject-reason"],
        json!("txn-same-nonwitness-data-already-in-mempool")
    );
}

#[test]
fn testmempoolaccept_hard_rejected_stays_in_result_body() {
    // Arrange
    let package = singleton_package(14);
    let requested = identity_at(&package, 0);
    let report = PackageReport::try_new(
        &package,
        PackageStatus::Failed,
        vec![PackageMemberResult::HardRejected(
            HardMemberFailure::Policy {
                requested,
                category: MempoolRejectionCategory::RelayFeeTooLow,
                reason: "min relay fee not met".to_string(),
            },
        )],
        vec![],
    )
    .expect("hard-rejected report");

    // Act
    let projected = project_testmempoolaccept(&report, &[]).expect("project");

    // Assert
    let element = first_element(&projected);
    assert_eq!(element["allowed"], json!(false));
    assert_eq!(element["reject-reason"], json!("min relay fee not met"));
    assert!(!element["reject-reason"].is_object());
}

#[test]
fn testmempoolaccept_post_trim_absent_uses_mempool_full() {
    // Arrange
    let package = singleton_package(15);
    let requested = identity_at(&package, 0);
    let report = PackageReport::try_new(
        &package,
        PackageStatus::Failed,
        vec![PackageMemberResult::PostTrimAbsent(PostTrimAbsence {
            requested,
            prior: PriorMemberSuccess::AlreadyPresent,
        })],
        vec![],
    )
    .expect("post-trim report");

    // Act
    let projected = project_testmempoolaccept(&report, &[]).expect("project");

    // Assert
    let element = first_element(&projected);
    assert_eq!(element["allowed"], json!(false));
    assert_eq!(element["reject-reason"], json!("mempool full"));
}

#[test]
fn testmempoolaccept_omits_open_bitcoin_keys() {
    // Arrange
    let (report, facts, _) = finally_present_report();

    // Act
    let projected = project_testmempoolaccept(&report, &facts).expect("project");

    // Assert
    let mut keys = Vec::new();
    collect_object_keys(&projected, &mut keys);
    for forbidden in [
        "fingerprint",
        "admission",
        "relay",
        "effective_fee_groups",
        "txid_hex",
        "replaced_txids",
    ] {
        assert!(
            !keys.iter().any(|key| key == forbidden),
            "unexpected Open Bitcoin key {forbidden}"
        );
    }
}

#[test]
fn testmempoolaccept_missing_finally_present_facts_is_error() {
    // Arrange
    let (report, _facts, _) = finally_present_report();

    // Act
    let result = project_testmempoolaccept(&report, &[]);

    // Assert
    assert_eq!(
        result,
        Err(super::PackageProjectionError::MissingMemberFacts { index: 0 })
    );
}

#[test]
fn testmempoolaccept_preserves_input_member_order() {
    // Arrange
    let package = parent_child_package();
    let parent = identity_at(&package, 0);
    let child = identity_at(&package, 1);
    let group_id = EffectiveFeeGroupId::from_u64(2);
    let report = PackageReport::try_new(
        &package,
        PackageStatus::Partial,
        vec![
            PackageMemberResult::FinallyPresent(NewlyPresent {
                requested: parent,
                effective_fee_group_id: group_id,
            }),
            PackageMemberResult::HardRejected(HardMemberFailure::Policy {
                requested: child,
                category: MempoolRejectionCategory::Validation,
                reason: "bad-txns-inputs-missingorspent".to_string(),
            }),
        ],
        vec![fee_group(group_id, vec![parent.wtxid])],
    )
    .expect("partial report");
    let facts = vec![facts_for(parent, 141, 200)];

    // Act
    let projected = project_testmempoolaccept(&report, &facts).expect("project");

    // Assert
    let elements = projected.as_array().expect("array");
    assert_eq!(elements.len(), 2);
    assert_eq!(elements[0]["allowed"], json!(true));
    assert_eq!(elements[1]["allowed"], json!(false));
}

fn partial_parent_accepted_report() -> (PackageReport, Vec<PackageMemberProjectionFacts>, Wtxid) {
    let package = parent_child_package();
    let parent = identity_at(&package, 0);
    let child = identity_at(&package, 1);
    let group_id = EffectiveFeeGroupId::from_u64(2);
    let report = PackageReport::try_new(
        &package,
        PackageStatus::Partial,
        vec![
            PackageMemberResult::FinallyPresent(NewlyPresent {
                requested: parent,
                effective_fee_group_id: group_id,
            }),
            PackageMemberResult::HardRejected(HardMemberFailure::Policy {
                requested: child,
                category: MempoolRejectionCategory::Validation,
                reason: "bad-txns-inputs-missingorspent".to_string(),
            }),
        ],
        vec![fee_group(group_id, vec![parent.wtxid])],
    )
    .expect("partial report");
    (report, vec![facts_for(parent, 141, 200)], parent.wtxid)
}

#[test]
fn one_report_projects_both_knots_trees() {
    // Arrange
    let (report, facts, parent_wtxid) = partial_parent_accepted_report();
    let parent_wtxid_hex = encode_hex(parent_wtxid.as_bytes());
    let replaced = [Txid::from_byte_array([0x51; 32])];

    // Act
    let accept = project_testmempoolaccept(&report, &facts).expect("accept");
    let submit = project_submitpackage(&report, &facts, &replaced).expect("submit");

    // Assert
    let accept_elements = accept.as_array().expect("accept array");
    assert_eq!(accept_elements.len(), 2);
    let submit_object = submit.as_object().expect("submit object");
    assert!(submit_object.contains_key("package_msg"));
    assert!(submit_object.contains_key("tx-results"));
    assert!(submit_object.contains_key("replaced-transactions"));
    let tx_results = submit["tx-results"].as_object().expect("tx-results");
    assert_eq!(accept_elements[0]["wtxid"], json!(parent_wtxid_hex.clone()));
    assert!(tx_results.contains_key(&parent_wtxid_hex));
    let mut keys = Vec::new();
    collect_object_keys(&accept, &mut keys);
    collect_object_keys(&submit, &mut keys);
    for forbidden in ["fingerprint", "admission", "relay"] {
        assert!(
            !keys.iter().any(|key| key == forbidden),
            "unexpected key {forbidden}"
        );
    }
}

#[test]
fn submitpackage_already_present_is_mempool_entry_without_effective_feerate() {
    // Arrange
    let package = singleton_package(12);
    let requested = identity_at(&package, 0);
    let report = PackageReport::try_new(
        &package,
        PackageStatus::Complete,
        vec![PackageMemberResult::AlreadyPresent(ExistingMember {
            requested,
        })],
        vec![],
    )
    .expect("already-present report");
    let facts = vec![facts_for(requested, 141, 200)];

    // Act
    let projected = project_submitpackage(&report, &facts, &[]).expect("project");

    // Assert
    let wtxid_hex = encode_hex(requested.wtxid.as_bytes());
    let tx_result = &projected["tx-results"][wtxid_hex];
    assert_eq!(tx_result["vsize"], json!(141));
    assert!(tx_result["fees"]["base"].is_number());
    assert!(tx_result["fees"].get("effective-feerate").is_none());
    assert!(tx_result.get("error").is_none());
}

#[test]
fn submitpackage_omits_broadcast_and_propagation_keys() {
    // Arrange
    let (report, facts, _) = partial_parent_accepted_report();

    // Act
    let projected = project_submitpackage(&report, &facts, &[]).expect("project");

    // Assert
    let mut keys = Vec::new();
    collect_object_keys(&projected, &mut keys);
    for forbidden in ["broadcast", "propagated", "public_relay"] {
        assert!(
            !keys.iter().any(|key| key == forbidden),
            "unexpected key {forbidden}"
        );
    }
    let rendered = projected.to_string();
    for forbidden in ["broadcast", "propagated", "public_relay"] {
        assert!(
            !rendered.contains(forbidden),
            "unexpected string {forbidden}"
        );
    }
}

#[test]
fn project_open_bitcoin_package_includes_fingerprint_and_dual_state() {
    // Arrange
    let (report, facts, _) = finally_present_report();
    let dual_state = vec![PackageMemberDualState {
        admission: PackageAdmissionState::Accepted,
        relay: PackageRelayState::Eligible,
    }];

    // Act
    let projected = project_open_bitcoin_package(&report, &facts, &dual_state).expect("project");

    // Assert
    let object = projected.as_object().expect("typed package object");
    assert_eq!(
        object["fingerprint"],
        json!(encode_hex(report.fingerprint().as_bytes()))
    );
    assert_eq!(object["status"], json!("complete"));
    assert!(object.contains_key("effective_fee_groups"));
    let members = object["members"].as_array().expect("input-ordered members");
    assert_eq!(members.len(), 1);
    assert_eq!(members[0]["result"], json!("FinallyPresent"));
    assert_eq!(members[0]["admission"], json!("accepted"));
    assert_eq!(members[0]["relay"], json!("eligible"));
    assert!(members[0]["txid"].is_string());
    assert!(members[0]["wtxid"].is_string());
}

#[test]
fn project_open_bitcoin_package_omits_propagation_keys() {
    // Arrange
    let package = singleton_package(12);
    let requested = identity_at(&package, 0);
    let report = PackageReport::try_new(
        &package,
        PackageStatus::Complete,
        vec![PackageMemberResult::AlreadyPresent(ExistingMember {
            requested,
        })],
        vec![],
    )
    .expect("already-present report");
    let facts = vec![facts_for(requested, 141, 200)];
    let dual_state = vec![PackageMemberDualState {
        admission: PackageAdmissionState::StillPresent,
        relay: PackageRelayState::RelayDisabled,
    }];

    // Act
    let projected = project_open_bitcoin_package(&report, &facts, &dual_state).expect("project");

    // Assert
    let mut keys = Vec::new();
    collect_object_keys(&projected, &mut keys);
    for forbidden in ["propagated", "broadcast", "public_relay"] {
        assert!(
            !keys.iter().any(|key| key == forbidden),
            "unexpected key {forbidden}"
        );
    }
    assert_eq!(projected["members"][0]["admission"], json!("still-present"));
    assert_eq!(projected["members"][0]["relay"], json!("relay_disabled"));
}
