// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;
use crate::storage::snapshot_codec::{ClassifiedWorkRecord, decode_work_record};
use crate::storage::snapshot_codec::{decode_chain_meta, encode_chain_meta};

#[test]
fn encode_block_undo_round_trips_and_is_not_full_snapshot() {
    // Arrange
    let undo = BlockUndo {
        transactions: vec![TxUndo {
            restored_inputs: vec![Coin {
                output: output(5_000),
                is_coinbase: false,
                created_height: 6,
                created_median_time_past: 11,
            }],
        }],
    };

    // Act
    let encoded = encode_block_undo(&undo).expect("encode block undo");
    let decoded = decode_block_undo(&encoded).expect("decode block undo");
    let encoded_json = String::from_utf8(encoded).expect("undo encode is JSON");

    // Assert
    assert_eq!(decoded, undo);
    assert!(
        encoded_json.contains("transactions"),
        "undo JSON must contain transactions: {encoded_json}"
    );
    assert!(
        !encoded_json.contains("\"utxos\""),
        "undo JSON must not embed leftover snapshot utxos: {encoded_json}"
    );
}

#[test]
fn chainstate_confirmation_counts_encode_deterministically_and_round_trip() {
    // Arrange
    let first_txid = Txid::from_byte_array([1_u8; 32]);
    let second_txid = Txid::from_byte_array([2_u8; 32]);
    let mut first = chainstate_snapshot();
    first.maybe_confirmed_txid_counts = Some(HashMap::from([(second_txid, 2), (first_txid, 1)]));
    let mut second = chainstate_snapshot();
    second.maybe_confirmed_txid_counts = Some(HashMap::from([(first_txid, 1), (second_txid, 2)]));

    // Act
    let first_encoded = encode_chainstate_snapshot(&first).expect("encode first chainstate");
    let second_encoded = encode_chainstate_snapshot(&second).expect("encode second chainstate");
    let decoded = decode_chainstate_snapshot(&first_encoded).expect("decode chainstate counts");

    // Assert
    assert_eq!(first_encoded, second_encoded);
    assert_eq!(
        decoded.maybe_confirmed_txid_counts,
        first.maybe_confirmed_txid_counts
    );
}

#[test]
fn chainstate_snapshot_without_confirmation_evidence_decodes_as_unknown() {
    // Arrange
    let encoded = encode_chainstate_snapshot(&chainstate_snapshot()).expect("encode chainstate");
    let mut value: serde_json::Value = serde_json::from_slice(&encoded).expect("chainstate JSON");
    value["payload"]
        .as_object_mut()
        .expect("chainstate payload")
        .remove("maybe_confirmed_txid_counts");
    let legacy = serde_json::to_vec(&value).expect("encode legacy chainstate");

    // Act
    let decoded = decode_chainstate_snapshot(&legacy).expect("decode legacy chainstate");

    // Assert
    assert!(decoded.maybe_confirmed_txid_counts.is_none());
}

#[test]
fn wallet_registry_and_selected_wallet_round_trip() {
    // Arrange
    let registry = WalletRegistrySnapshot::new(["alpha".to_string(), "beta".to_string()]);
    let selected = SelectedWalletRecord {
        wallet_name: "beta".to_string(),
    };

    // Act
    let encoded_registry = encode_wallet_registry_snapshot(&registry).expect("encode registry");
    let decoded_registry =
        decode_wallet_registry_snapshot(&encoded_registry).expect("decode registry");
    let encoded_selected = encode_selected_wallet(&selected).expect("encode selected");
    let decoded_selected = decode_selected_wallet(&encoded_selected).expect("decode selected");

    // Assert
    assert_eq!(decoded_registry, registry);
    assert_eq!(decoded_selected, selected);
}

#[test]
fn wallet_rescan_job_round_trips_full_checkpoint_state() {
    // Arrange
    let job = WalletRescanJob {
        wallet_name: "alpha".to_string(),
        target_tip_hash: BlockHash::from_byte_array([7_u8; 32]),
        target_tip_height: 144,
        next_height: 121,
        maybe_scanned_through_height: Some(120),
        maybe_tip_median_time_past: Some(1_700_000_120),
        freshness: WalletRescanFreshness::Partial,
        state: WalletRescanJobState::Scanning,
        maybe_error: None,
    };

    // Act
    let encoded = encode_wallet_rescan_job(&job).expect("encode job");
    let decoded = decode_wallet_rescan_job(&encoded).expect("decode job");

    // Assert
    assert_eq!(decoded, job);
}

#[test]
fn wallet_snapshot_round_trips_through_original_descriptors() {
    // Arrange
    let snapshot = wallet_snapshot();

    // Act
    let encoded = encode_wallet_snapshot(&snapshot).expect("encode wallet");
    let decoded = decode_wallet_snapshot(&encoded).expect("decode wallet");

    // Assert
    assert_eq!(decoded, snapshot);
    assert_eq!(decoded.descriptors[0].descriptor.range_start(), Some(0));
    assert_eq!(decoded.descriptors[0].descriptor.range_end(), Some(1000));
    assert_eq!(decoded.descriptors[0].descriptor.next_index(), Some(2));
}

#[test]
fn phase162_descriptor_legacy_codec_preserves_metadata_and_raw_text() {
    use open_bitcoin_core::wallet::{SingleKeyDescriptor, WalletError};

    // Arrange: explicit historical bytes, independent of the current formatter.
    for (canonical_suffix, legacy_suffix, range) in [
        ("ob:0:0:0", "ob::0:0:0", (0, 0, 0)),
        ("ob:2:5:3", "ob::2:5:3", (2, 5, 3)),
        ("ob:0:1000:2", "ob::0:1000:2", (0, 1000, 2)),
    ] {
        let mut expected = wallet_snapshot();
        let body = expected.descriptors[0]
            .original_text
            .split_once('#')
            .expect("ranged storage body")
            .0
            .to_owned();
        let canonical_text = format!("{body}#{canonical_suffix}");
        let legacy_text = format!("{body}#{legacy_suffix}");
        let canonical = SingleKeyDescriptor::parse(&canonical_text, expected.network)
            .expect("canonical expected descriptor");
        expected.descriptors[0].descriptor = canonical.clone();
        expected.descriptors[0].original_text = canonical_text.clone();
        let encoded = encode_wallet_snapshot(&expected).expect("canonical wallet encode");
        let mut value: serde_json::Value = serde_json::from_slice(&encoded).expect("wallet JSON");
        value["payload"]["descriptors"][0]["original_text"] = legacy_text.clone().into();
        let legacy_bytes = serde_json::to_vec(&value).expect("explicit legacy wallet encode");
        assert!(
            String::from_utf8(legacy_bytes.clone())
                .expect("JSON bytes")
                .contains(&legacy_text)
        );

        // Act
        let public_parse = SingleKeyDescriptor::parse(&legacy_text, expected.network);
        let decoded = decode_wallet_snapshot(&legacy_bytes).expect("existing legacy compatibility");

        // Assert: normalization is only for parsing; retained raw text is not rewritten.
        assert!(matches!(
            public_parse,
            Err(WalletError::DescriptorSyntax(_))
        ));
        let record = &decoded.descriptors[0];
        assert_eq!(record.original_text, legacy_text);
        assert_eq!(record.descriptor, canonical);
        assert_eq!(record.descriptor.range_start(), Some(range.0));
        assert_eq!(record.descriptor.range_end(), Some(range.1));
        assert_eq!(record.descriptor.next_index(), Some(range.2));
        assert_eq!(record.descriptor.storage_text(), canonical_text);
        assert_eq!(
            SingleKeyDescriptor::parse(&record.descriptor.storage_text(), expected.network)
                .expect("future descriptor storage parses directly"),
            canonical
        );
        assert_eq!(
            record
                .descriptor
                .script_pubkey()
                .expect("decoded current script"),
            canonical.script_pubkey().expect("canonical current script")
        );
        assert_eq!(
            record
                .descriptor
                .address(expected.network)
                .expect("decoded address"),
            canonical
                .address(expected.network)
                .expect("canonical address")
        );
        for index in [range.0, range.1, range.2] {
            assert_eq!(
                record
                    .descriptor
                    .script_pubkey_at(index)
                    .expect("decoded index script"),
                canonical
                    .script_pubkey_at(index)
                    .expect("canonical index script")
            );
        }
        expected.descriptors[0].original_text = legacy_text;
        assert_eq!(
            decoded, expected,
            "all other snapshot fields survive unchanged"
        );
    }
}

#[test]
fn header_entries_round_trip_and_validate_header_hashes() {
    // Arrange
    let header = header(2);
    let entry = HeaderEntry {
        block_hash: open_bitcoin_core::consensus::block_hash(&header),
        header,
        height: 1,
        chain_work: open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(2),
    };

    // Act
    let encoded = encode_header_entries(std::slice::from_ref(&entry)).expect("encode headers");
    let decoded = decode_header_entries(&encoded).expect("decode headers");

    // Assert
    assert_eq!(decoded.entries, vec![entry]);
}

#[test]
fn phase165_work_codec_existing_header_snapshot_retains_legacy_score() {
    // Arrange
    let header = header(2);
    let entry = HeaderEntry {
        block_hash: open_bitcoin_core::consensus::block_hash(&header),
        header,
        height: 1,
        chain_work: open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(2),
    };
    let encoded = encode_header_entries(std::slice::from_ref(&entry)).expect("historic headers");
    let mut json: serde_json::Value = serde_json::from_slice(&encoded).expect("historic JSON");
    json["payload"]["entries"][0]["chain_work"] = serde_json::json!(2);
    let encoded = serde_json::to_vec(&json).expect("historic header fixture");
    let score = serde_json::to_vec(&json["payload"]["entries"][0]["chain_work"])
        .expect("historic low score bytes");

    // Act
    let classification = decode_work_record(StorageNamespace::Headers, &score)
        .expect("historic work classification");
    let decoded = decode_header_entries(&encoded);

    // Assert
    assert_eq!(score, b"2");
    assert_eq!(classification, ClassifiedWorkRecord::LegacySynthetic(2));
    assert!(matches!(decoded, Err(StorageError::Corruption { .. })));
}

#[test]
fn phase165_exact_snapshot_and_chain_meta_preserve_high_work_limbs() {
    // Arrange
    let mut snapshot = chainstate_snapshot();
    let mut bytes = [0; 32];
    bytes[31] = 0x80;
    bytes[16] = 0x7f;
    bytes[0] = 0x55;
    let number = open_bitcoin_core::consensus::chain_proof::ProofWork::from_le_bytes(bytes);
    snapshot.active_chain[0].chain_work = number;

    // Act
    let encoded = encode_chainstate_snapshot(&snapshot).expect("exact snapshot");
    let decoded = decode_chainstate_snapshot(&encoded).expect("exact snapshot read");
    let meta = encode_chain_meta(&snapshot.active_chain, None).expect("exact chain meta");
    let (chain, _) = decode_chain_meta(&meta).expect("exact chain meta read");

    // Assert
    assert_eq!(decoded.active_chain[0].chain_work.to_le_bytes(), bytes);
    assert_eq!(chain[0].chain_work, number);
    assert!(
        String::from_utf8(encoded)
            .expect("JSON")
            .contains("exact_number")
    );
}

#[test]
fn phase165_legacy_chain_work_refuses_default_hydration() {
    // Arrange
    let encoded = encode_chainstate_snapshot(&chainstate_snapshot()).expect("snapshot");
    let mut json: serde_json::Value = serde_json::from_slice(&encoded).expect("JSON");
    json["payload"]["active_chain"][0]["chain_work"] = serde_json::json!(1);
    let historic = serde_json::to_vec(&json).expect("historic fixture");

    // Act
    let decoded = decode_chainstate_snapshot(&historic);

    // Assert
    assert!(matches!(decoded, Err(StorageError::Corruption { .. })));
}

#[test]
fn metrics_snapshot_round_trips_samples() {
    // Arrange
    let snapshot = MetricsStorageSnapshot {
        samples: vec![MetricSample::new(MetricKind::HeaderHeight, 1.0, 2)],
    };

    // Act
    let encoded = encode_metrics_snapshot(&snapshot).expect("encode metrics");
    let decoded = decode_metrics_snapshot(&encoded).expect("decode metrics");

    // Assert
    assert_eq!(decoded, snapshot);
}
