// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/policy/policy.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use super::*;
use open_bitcoin_core::{
    consensus::{SpentOutput, TransactionInputContext},
    primitives::{Amount, ScriptBuf, ScriptWitness, TransactionInput, TransactionOutput},
};
use open_bitcoin_mempool::{
    PolicyConfig, transaction_weight_and_virtual_size, validate_standard_transaction,
};

fn transaction(op_return: bool, witness: bool) -> Transaction {
    let mut script = vec![0; 10_001];
    script[0] = if op_return { 0x6a } else { 0x51 };
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([9; 32]),
                vout: 0,
            },
            script_sig: ScriptBuf::from_bytes(vec![]).expect("empty scriptSig"),
            sequence: u32::MAX,
            witness: if witness {
                ScriptWitness::new(vec![vec![1, 2, 3], vec![4; 32]])
            } else {
                ScriptWitness::default()
            },
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(1).expect("valid amount"),
            script_pubkey: ScriptBuf::from_serialized_bytes(script),
        }],
        lock_time: 0,
    }
}

fn current(tx: Transaction) -> MempoolSnapshot {
    let record = MempoolSnapshotRecord::try_from_canonical(
        tx,
        MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(90)),
    )
    .expect("data record");
    let member = record.member_identity().expect("exact member");
    MempoolSnapshot::try_new_current(
        CapturedMempoolGeneration::new(42),
        PolicyTime::from_unix_seconds(120),
        vec![record],
        BTreeSet::from([member]),
    )
    .expect("bounded snapshot data")
}

fn current_wire(bytes: &[u8]) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({
        "schema_version":crate::SchemaVersion::CURRENT.get(),
        "payload":{"format_version":2,"captured_generation":42,"captured_at_unix_seconds":120,
        "records":[{"transaction":bytes,"accepted_at_unix_seconds":90}],"unbroadcast_members":[]}
    }))
    .expect("raw snapshot JSON")
}

#[test]
fn phase167_serialized_mempool_snapshot_current_exact_scripts_witness_and_members() {
    // Arrange / Act / Assert: representation does not claim policy acceptance.
    for op_return in [false, true] {
        for witness in [false, true] {
            let snapshot = current(transaction(op_return, witness));
            let encoded = encode_mempool_snapshot(&snapshot).expect("current data encoding");
            let decoded = decode_mempool_snapshot(&encoded).expect("raw script snapshot decode");
            assert_eq!(decoded, snapshot);
            assert_eq!(
                encode_mempool_snapshot(&decoded).expect("canonical encoding"),
                encoded
            );
            assert_eq!(
                decoded.records[0].transaction.outputs[0]
                    .script_pubkey
                    .as_bytes()
                    .len(),
                10_001
            );
            let before = snapshot.records[0]
                .member_identity()
                .expect("original identity");
            let after = decoded.records[0]
                .member_identity()
                .expect("loaded identity");
            assert_eq!(after, before);
            assert_eq!(after.txid.as_bytes() == after.wtxid.as_bytes(), !witness);
        }
    }
}

#[test]
fn phase167_serialized_mempool_snapshot_legacy_exact_identity_and_unknown_authority() {
    // Arrange / Act / Assert
    for op_return in [false, true] {
        let tx = transaction(op_return, true);
        let member = canonical_member_identity(&tx).expect("member");
        let bytes = encode_canonical_transaction(&tx).expect("wire");
        let legacy = serde_json::json!({"schema_version":crate::SchemaVersion::CURRENT.get(),"payload":{"records":[{
            "txid":member.txid.to_byte_array(),"wtxid":member.wtxid.to_byte_array(),"transaction":bytes,
            "fee_sats":1000,"virtual_size":transaction_weight_and_virtual_size(&tx).expect("size").1
        }]}});
        let decoded = decode_mempool_snapshot(&serde_json::to_vec(&legacy).expect("legacy JSON"))
            .expect("legacy raw scripts");
        assert_eq!(decoded.records[0].transaction, tx);
        assert_eq!(
            decoded.records[0].acceptance_time,
            MempoolAcceptanceTime::LegacyUnknown
        );
        assert!(decoded.captured_generation().is_none());
        assert!(decoded.unbroadcast_members().is_empty());
        let mut mismatch = legacy.clone();
        mismatch["payload"]["records"][0]["wtxid"][0] =
            serde_json::json!(member.wtxid.as_bytes()[0] ^ 1);
        assert!(
            decode_mempool_snapshot(&serde_json::to_vec(&mismatch).expect("changed legacy"))
                .expect_err("identity mismatch")
                .to_string()
                .contains("identity mismatch")
        );
    }
}

#[test]
fn phase167_serialized_mempool_snapshot_configured_datacarrier_suffix_policy_is_distinct() {
    // Arrange
    let tx = transaction(true, false);
    let context = TransactionInputContext {
        spent_output: SpentOutput {
            value: Amount::from_sats(1000).expect("funding"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("spent script"),
            is_coinbase: false,
        },
        created_height: 1,
        created_median_time_past: 0,
    };
    let weight = transaction_weight_and_virtual_size(&tx).expect("weight").0;
    let configured = PolicyConfig {
        max_datacarrier_bytes: 10_001,
        permit_bare_datacarrier: true,
        ..Default::default()
    };
    // Act / Assert: this exercises existing standardness, not live admission.
    assert!(
        validate_standard_transaction(
            &tx,
            std::slice::from_ref(&context),
            &PolicyConfig::default(),
            weight,
            0
        )
        .is_err()
    );
    assert!(
        validate_standard_transaction(&tx, std::slice::from_ref(&context), &configured, weight, 0)
            .is_ok()
    );
    let mut too_long = tx.clone();
    too_long.outputs[0].script_pubkey =
        ScriptBuf::from_serialized_bytes([vec![0x6a], vec![0; 10_001]].concat());
    assert!(
        validate_standard_transaction(
            &too_long,
            std::slice::from_ref(&context),
            &PolicyConfig {
                max_datacarrier_bytes: 10_002,
                ..configured.clone()
            },
            weight + 4,
            0
        )
        .is_err()
    );
    let nonstandard = transaction(false, false);
    assert!(
        validate_standard_transaction(&nonstandard, &[context], &configured, weight, 0).is_err()
    );
    assert!(
        decode_mempool_snapshot(
            &encode_mempool_snapshot(&current(nonstandard))
                .expect("public data remains representable")
        )
        .is_ok()
    );
}

#[test]
fn phase167_serialized_mempool_snapshot_framing_amount_and_malformed_refuse() {
    // Arrange: offsets are the ordinary no-witness input plus one output layout.
    let bytes = encode_canonical_transaction(&transaction(false, false)).expect("wire");
    let mut cases = Vec::new();
    for length in [0, 4, 5, 41, 47, 58, bytes.len() - 1] {
        cases.push(bytes[..length].to_vec());
    }
    let mut suffix = bytes.clone();
    suffix.push(0);
    cases.push(suffix);
    for amount in [-1i64, 2_100_000_000_000_001, i64::MAX] {
        let mut invalid = bytes.clone();
        invalid[47..55].copy_from_slice(&amount.to_le_bytes());
        cases.push(invalid);
    }
    let mut noncanonical = bytes.clone();
    noncanonical.splice(4..5, [0xfd, 1, 0]);
    cases.push(noncanonical);
    let mut oversized = bytes.clone();
    oversized.splice(55..58, [0xfe, 1, 0, 0, 2]);
    cases.push(oversized);
    let mut witness =
        encode_canonical_transaction(&transaction(false, true)).expect("witness wire");
    witness[5] = 2;
    cases.push(witness);
    // Act / Assert
    for (index, invalid) in cases.iter().enumerate() {
        assert!(
            decode_mempool_snapshot(&current_wire(invalid)).is_err(),
            "corrupt case {index}"
        );
    }
    for invalid in [
        b"{not-json".as_slice(),
        b"{\"schema_version\":999,\"payload\":{\"records\":[]}}",
    ] {
        assert!(decode_mempool_snapshot(invalid).is_err());
    }
}

#[test]
fn phase167_serialized_mempool_snapshot_every_decode_budget_retains_exact_boundary() {
    // Arrange
    let snapshot = current(transaction(false, true));
    let encoded = encode_mempool_snapshot(&snapshot).expect("encoding");
    let tx_bytes = encode_canonical_transaction(&snapshot.records[0].transaction)
        .expect("transaction")
        .len();
    let exact = MempoolSnapshotDecodeLimits::new(encoded.len(), 1, 1, tx_bytes, tx_bytes);
    // Act / Assert
    assert_eq!(
        decode_mempool_snapshot_with_limits(&encoded, exact).expect("exact limits"),
        snapshot
    );
    for dimension in 0..5 {
        let mut below = exact;
        match dimension {
            0 => below.max_encoded_bytes -= 1,
            1 => below.max_records -= 1,
            2 => below.max_unbroadcast_members -= 1,
            3 => below.max_transaction_bytes -= 1,
            _ => below.max_total_transaction_bytes -= 1,
        }
        assert!(
            decode_mempool_snapshot_with_limits(&encoded, below)
                .expect_err("one over budget")
                .to_string()
                .contains("resource bound"),
            "dimension {dimension}"
        );
    }
}

#[test]
fn phase167_serialized_mempool_snapshot_generation_format_and_duplicate_members_refuse() {
    // Arrange
    let encoded = encode_mempool_snapshot(&current(transaction(true, true))).expect("current");
    let value: serde_json::Value = serde_json::from_slice(&encoded).expect("JSON");
    // Act / Assert
    for case in [
        "generation",
        "format",
        "duplicate",
        "unknown-member",
        "future-time",
    ] {
        let mut invalid = value.clone();
        match case {
            "generation" => invalid["payload"]["captured_generation"] = serde_json::json!(u64::MAX),
            "format" => invalid["payload"]["format_version"] = serde_json::json!(3),
            "duplicate" => {
                let member = invalid["payload"]["unbroadcast_members"][0].clone();
                invalid["payload"]["unbroadcast_members"]
                    .as_array_mut()
                    .expect("members")
                    .push(member);
            }
            "unknown-member" => {
                invalid["payload"]["unbroadcast_members"][0]["txid"][0] = serde_json::json!(7)
            }
            _ => {
                invalid["payload"]["records"][0]["accepted_at_unix_seconds"] =
                    serde_json::json!(121)
            }
        }
        assert!(
            decode_mempool_snapshot(&serde_json::to_vec(&invalid).expect("invalid JSON")).is_err(),
            "{case}"
        );
    }
}
