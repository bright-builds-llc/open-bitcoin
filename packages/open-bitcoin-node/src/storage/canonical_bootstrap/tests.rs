// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/txdb.cpp

use super::{
    CanonicalBootstrapRefusal,
    records::{CanonicalGenesisRecords, Rows},
};
use crate::storage::{snapshot_codec, validation_history as history};
use open_bitcoin_core::{
    chainstate::ChainPosition,
    codec::encode_block,
    consensus::chain_spec::{CanonicalChain, CanonicalGenesisProof},
};
use open_bitcoin_network::HeaderEntry;
use std::collections::{BTreeMap, HashMap};

#[path = "qualification_tests.rs"]
mod qualification_tests;
#[path = "test_support.rs"]
mod test_support;

pub(super) fn fixture(chain: CanonicalChain) -> BTreeMap<String, Rows> {
    let spec = chain.spec();
    let block = spec.genesis_block();
    let proof = CanonicalGenesisProof::prove(spec, &block).expect("canonical genesis");
    let position = ChainPosition::genesis(
        block.header.clone(),
        proof.work(),
        i64::from(block.header.time),
    )
    .expect("position");
    let entries = vec![HeaderEntry {
        block_hash: proof.hash(),
        header: block.header.clone(),
        height: 0,
        chain_work: proof.work(),
    }];
    let id =
        history::BlockValidationIdentity::new(proof.hash(), block.header.previous_block_hash, 0)
            .expect("identity");
    let mut rows = BTreeMap::<String, Rows>::new();
    put(&mut rows, "schema", "schema_version", b"2".to_vec());
    put(
        &mut rows,
        "schema",
        super::records::IDENTITY_KEY,
        super::records::encode_identity(chain),
    );
    put(
        &mut rows,
        "headers",
        "snapshot",
        snapshot_codec::encode_header_entries(&entries).expect("headers"),
    );
    put(
        &mut rows,
        "block_index",
        "snapshot",
        snapshot_codec::encode_block_index_entries(&entries).expect("index"),
    );
    let key = crate::storage::fjall_store::block_key(proof.hash());
    put(
        &mut rows,
        "block_index",
        &key,
        encode_block(&block).expect("body"),
    );
    put(
        &mut rows,
        "chainstate",
        "chain_meta",
        snapshot_codec::encode_chain_meta(&[position], Some(&HashMap::new())).expect("meta"),
    );
    put(&mut rows, "coins", "B", proof.hash().as_bytes().to_vec());
    put(
        &mut rows,
        "block_index",
        history::COVERAGE_KEY,
        history::encode_coverage(history::ValidationCoverage::Complete).to_vec(),
    );
    put(
        &mut rows,
        "block_index",
        &history::record_key(proof.hash()),
        history::encode_record(history::ValidationHistoryRecord::new(
            id,
            history::StoredValidationStatus::ScriptsValid,
        )),
    );
    rows
}

fn put(rows: &mut BTreeMap<String, Rows>, namespace: &str, key: &str, value: Vec<u8>) {
    rows.entry(namespace.to_owned())
        .or_default()
        .insert(key.as_bytes().to_vec(), value);
}

#[test]
fn phase166_records_reject_legacy_or_malformed_embedded_work() {
    // Arrange
    let rows = fixture(CanonicalChain::Regtest);
    // Act / Assert
    for (namespace, key, pointer) in [
        ("headers", "snapshot", "/payload/entries/0/chain_work"),
        ("block_index", "snapshot", "/payload/entries/0/chain_work"),
        (
            "chainstate",
            "chain_meta",
            "/payload/active_chain/0/chain_work",
        ),
    ] {
        for bad in [
            serde_json::json!(42),
            serde_json::json!({"version":1,"kind":"legacy_synthetic","score":42}),
            serde_json::json!({"version":1,"kind":"exact_number","bytes_le":[1]}),
        ] {
            let mut changed = rows.clone();
            let bytes = changed
                .get_mut(namespace)
                .expect("namespace")
                .get_mut(key.as_bytes())
                .expect("record");
            let mut value: serde_json::Value = serde_json::from_slice(bytes).expect("json");
            *value.pointer_mut(pointer).expect("work") = bad;
            *bytes = serde_json::to_vec(&value).expect("json");
            assert!(matches!(
                parse(&changed),
                Err(CanonicalBootstrapRefusal::MalformedRecord)
            ));
        }
    }
}

#[test]
fn phase166_records_accept_only_inventoried_metadata_keys() {
    // Arrange
    let mut rows = fixture(CanonicalChain::Regtest);
    for (namespace, key) in [
        ("runtime", "snapshot"),
        ("wallet", "wallet_registry"),
        ("wallet", "selected_wallet"),
        ("wallet", "wallet_snapshot:alice"),
        ("wallet", "wallet_rescan_job:alice"),
        ("metrics", "snapshot"),
        ("mempool", "snapshot"),
    ] {
        put(&mut rows, namespace, key, vec![]);
    }
    // Act / Assert
    assert!(parse(&rows).is_ok());
    for namespace in [
        "runtime",
        "wallet",
        "metrics",
        "mempool",
        "headers",
        "block_index",
        "chainstate",
        "coins",
        "schema",
        "unknown",
    ] {
        let mut changed = rows.clone();
        put(&mut changed, namespace, "unproved_authority", vec![1]);
        assert!(matches!(
            parse(&changed),
            Err(CanonicalBootstrapRefusal::UnsupportedRecord)
        ));
    }
}

#[test]
fn phase166_records_reject_legacy_unknown_and_interrupted_authority() {
    // Arrange
    let rows = fixture(CanonicalChain::Regtest);
    // Act / Assert
    for (namespace, key) in [
        ("chainstate", "snapshot"),
        ("chainstate", "undo:genesis"),
        ("coins", "Cextra"),
        ("coins", "H"),
        ("runtime", "recovery_marker"),
        ("block_index", "have_pruned"),
        ("block_index", "prune_intent"),
        ("block_index", "prune_summary"),
        ("block_index", "basic_filter:v2:state"),
    ] {
        let mut changed = rows.clone();
        put(&mut changed, namespace, key, vec![1]);
        assert!(matches!(
            parse(&changed),
            Err(CanonicalBootstrapRefusal::UnsupportedRecord)
        ));
    }
    for (namespace, key) in [
        ("headers", "snapshot"),
        ("block_index", "snapshot"),
        ("chainstate", "chain_meta"),
    ] {
        let mut changed = rows.clone();
        let bytes = changed
            .get_mut(namespace)
            .expect("namespace")
            .get_mut(key.as_bytes())
            .expect("row");
        let mut value: serde_json::Value = serde_json::from_slice(bytes).expect("json");
        value["unexpected_authority"] = serde_json::json!(true);
        *bytes = serde_json::to_vec(&value).expect("json");
        assert!(matches!(
            parse(&changed),
            Err(CanonicalBootstrapRefusal::MalformedRecord)
        ));
    }
}

pub(super) fn parse(
    rows: &BTreeMap<String, Rows>,
) -> Result<CanonicalGenesisRecords, CanonicalBootstrapRefusal> {
    CanonicalGenesisRecords::parse_rows(
        rows.iter()
            .map(|(name, rows)| (name.as_str(), rows))
            .collect(),
    )
}

#[test]
fn phase166_records_parse_complete_current_formats_without_authority() {
    // Arrange
    let rows = fixture(CanonicalChain::Regtest);
    // Act
    let parsed = parse(&rows);
    // Assert
    assert!(parsed.is_ok());
}

#[test]
fn phase166_records_missing_rows_differ_from_empty_values() {
    // Arrange
    let rows = fixture(CanonicalChain::Regtest);
    // Act / Assert
    for (namespace, entries) in &rows {
        for key in entries.keys() {
            let mut missing = rows.clone();
            missing.get_mut(namespace).expect("namespace").remove(key);
            assert!(
                matches!(
                    parse(&missing),
                    Err(CanonicalBootstrapRefusal::MissingRecord)
                ),
                "missing {namespace}"
            );
            let mut empty = rows.clone();
            empty
                .get_mut(namespace)
                .expect("namespace")
                .insert(key.clone(), Vec::new());
            assert!(
                matches!(
                    parse(&empty),
                    Err(CanonicalBootstrapRefusal::MalformedRecord)
                ),
                "empty {namespace}"
            );
        }
    }
}

#[test]
fn phase166_records_prove_all_four_canonical_genesis_bindings() {
    // Arrange / Act / Assert
    for chain in [
        CanonicalChain::Mainnet,
        CanonicalChain::Testnet3,
        CanonicalChain::Signet,
        CanonicalChain::Regtest,
    ] {
        let parsed = parse(&fixture(chain)).expect("complete records");
        let proved =
            super::validate_genesis_records(chain, &parsed).expect("complete genesis proof");
        assert_eq!(proved.genesis().chain(), chain);
        assert_eq!(proved.genesis().hash(), chain.spec().genesis_hash());
        assert_eq!(proved.position(), &parsed.active[0]);
        assert!(chain.spec().require_ordinary_history().is_err());
    }
}

fn with_basic(rows: &mut BTreeMap<String, Rows>, committed: bool, active: bool) {
    use crate::storage::{filter_index as filters, fjall_store};
    use open_bitcoin_core::chainstate::{
        BasicFilterInputs, IndexInputProtection,
        filter_index::lifecycle::{IndexGeneration, IndexLifecycle},
    };
    let parsed = parse(rows).expect("genesis records");
    let position = &parsed.active[0];
    let inputs = BasicFilterInputs::from_historical(&parsed.body, position, None).expect("inputs");
    let record = filters::StoredFilterRecord::generate(&inputs, position, None).expect("record");
    let protection = IndexInputProtection::FromHeight(u32::from(committed));
    let generation = IndexGeneration::new(3);
    let owner = if active {
        IndexLifecycle::Active { generation }
    } else {
        IndexLifecycle::Disabled { generation }
    };
    put(
        rows,
        "block_index",
        filters::ownership::OWNER_KEY,
        filters::ownership::encode_owner(owner).to_vec(),
    );
    put(
        rows,
        "block_index",
        filters::STATE_KEY,
        filters::encode_state(filters::StoredFilterState {
            maybe_endpoint: committed.then_some((0, position.block_hash)),
            fence_height: 0,
            fence_hash: position.block_hash,
            protection,
        }),
    );
    let locks: Vec<_> = if active {
        protection.maybe_prune_lock().into_iter().collect()
    } else {
        Vec::new()
    };
    put(
        rows,
        "block_index",
        fjall_store::PRUNE_LOCKS_KEY,
        fjall_store::encode_prune_locks(&locks).expect("locks"),
    );
    if committed {
        put(
            rows,
            "block_index",
            &filters::record_key(position.block_hash),
            filters::encode_record(&record),
        );
        put(
            rows,
            "block_index",
            &filters::active_key(0),
            filters::encode_projection(0, position.block_hash),
        );
    }
}

#[test]
fn phase166_records_saved_basic_complete_empty_and_disabled_prefixes() {
    // Arrange / Act / Assert
    for chain in [
        CanonicalChain::Mainnet,
        CanonicalChain::Testnet3,
        CanonicalChain::Signet,
        CanonicalChain::Regtest,
    ] {
        for committed in [false, true] {
            for active in [false, true] {
                let mut rows = fixture(chain);
                with_basic(&mut rows, committed, active);
                let parsed = parse(&rows).expect("saved BASIC records");
                assert!(super::validate_genesis_records(chain, &parsed).is_ok());
            }
        }
    }
}

#[test]
fn phase166_records_saved_basic_missing_or_corrupt_bindings_refuse() {
    // Arrange
    let chain = CanonicalChain::Regtest;
    let mut rows = fixture(chain);
    with_basic(&mut rows, true, true);
    // Act / Assert
    let keys: Vec<_> = rows["block_index"]
        .keys()
        .filter(|key| key.starts_with(b"basic_filter:") || key.as_slice() == b"prune_locks")
        .cloned()
        .collect();
    for key in keys {
        let mut changed = rows.clone();
        changed.get_mut("block_index").expect("index").remove(&key);
        let parsed = parse(&changed).expect("missing optional record stays absent");
        assert!(super::validate_genesis_records(chain, &parsed).is_err());
        let mut changed = rows.clone();
        let value = changed
            .get_mut("block_index")
            .expect("index")
            .get_mut(&key)
            .expect("row");
        value[0] ^= 255;
        let parsed = parse(&changed).expect("unproved optional bytes");
        assert!(super::validate_genesis_records(chain, &parsed).is_err());
    }
}

#[test]
fn phase166_records_saved_basic_rejects_weak_lock_or_forged_filter_content() {
    use crate::storage::{filter_index as filters, fjall_store};
    use open_bitcoin_core::chainstate::IndexInputProtection;
    // Arrange
    let chain = CanonicalChain::Regtest;
    let mut rows = fixture(chain);
    with_basic(&mut rows, true, true);
    // Act / Assert
    for first in [2, u32::MAX] {
        let mut changed = rows.clone();
        let mut lock = IndexInputProtection::FromHeight(first)
            .maybe_prune_lock()
            .expect("lock");
        lock.height_first = first;
        put(
            &mut changed,
            "block_index",
            fjall_store::PRUNE_LOCKS_KEY,
            fjall_store::encode_prune_locks(&[lock]).expect("locks"),
        );
        assert!(
            super::validate_genesis_records(chain, &parse(&changed).expect("records")).is_err()
        );
    }
    let mut parsed = parse(&rows).expect("records");
    let foreign = parse(&fixture(CanonicalChain::Mainnet)).expect("foreign records");
    let inputs = open_bitcoin_core::chainstate::BasicFilterInputs::from_historical(
        &foreign.body,
        &foreign.active[0],
        None,
    )
    .expect("foreign canonical body");
    let forged = filters::StoredFilterRecord::generate(&inputs, &foreign.active[0], None)
        .expect("self-consistent forged filter");
    let mut forged = filters::encode_record(&forged);
    forged[6..38].copy_from_slice(chain.spec().genesis_hash().as_bytes());
    assert!(
        filters::decode_record(
            &filters::record_key(chain.spec().genesis_hash()),
            &forged,
            None
        )
        .is_ok()
    );
    parsed.basic.insert(
        filters::record_key(chain.spec().genesis_hash()).into_bytes(),
        forged,
    );
    assert!(super::validate_genesis_records(chain, &parsed).is_err());
}

#[test]
fn phase166_records_foreign_selectors_never_promote_complete_records() {
    // Arrange / Act / Assert
    let chains = [
        CanonicalChain::Mainnet,
        CanonicalChain::Testnet3,
        CanonicalChain::Signet,
        CanonicalChain::Regtest,
    ];
    for stored in chains {
        let parsed = parse(&fixture(stored)).expect("records");
        for selected in chains.into_iter().filter(|selected| *selected != stored) {
            assert!(super::validate_genesis_records(selected, &parsed).is_err());
        }
    }
}

#[test]
fn phase166_records_wide_numbers_roundtrip_without_chain_authority() {
    use crate::storage::{
        StorageNamespace,
        snapshot_codec::{ClassifiedWorkRecord, decode_work_record, encode_work_record},
    };
    use open_bitcoin_core::consensus::chain_proof::ProofWork;
    // Arrange: independent numerical-only vector, with significant bits above u128.
    let mut bytes = [0; 32];
    bytes[0] = 0x42;
    bytes[16] = 1;
    bytes[31] = 0x80;
    let number =
        ProofWork::from_hex("8000000000000000000000000000000100000000000000000000000000000042")
            .expect("wide hex");
    // Act
    let encoded = encode_work_record(
        StorageNamespace::Headers,
        &ClassifiedWorkRecord::ExactNumber(number),
    )
    .expect("encode");
    let decoded = decode_work_record(StorageNamespace::Headers, &encoded).expect("decode");
    // Assert
    assert_eq!(number.to_le_bytes(), bytes);
    assert_eq!(decoded, ClassifiedWorkRecord::ExactNumber(number));
    assert_eq!(
        number.to_hex(),
        "8000000000000000000000000000000100000000000000000000000000000042"
    );
    for offset in 0..32 {
        let mut parsed = parse(&fixture(CanonicalChain::Regtest)).expect("records");
        let mut changed = parsed.headers[0].chain_work.to_le_bytes();
        changed[offset] ^= 1;
        let changed = ProofWork::from_le_bytes(changed);
        parsed.headers[0].chain_work = changed;
        parsed.index[0].chain_work = changed;
        parsed.active[0].chain_work = changed;
        assert!(super::validate_genesis_records(CanonicalChain::Regtest, &parsed).is_err());
    }
}

#[test]
fn phase166_records_decode_actual_detached_engine_rows() {
    use crate::storage::fjall_store::inspection::InspectedRecords;
    // Arrange
    let mut nonce = [0; 16];
    getrandom::fill(&mut nonce).expect("nonce");
    let name: String = nonce.iter().map(|byte| format!("{byte:02x}")).collect();
    let root = std::fs::canonicalize(std::env::temp_dir())
        .expect("canonical temp parent")
        .join(format!("phase166-records-{name}"));
    std::fs::create_dir(&root).expect("owned fixture");
    let db = fjall::Database::builder(&root)
        .open()
        .expect("fixture database");
    for (namespace, rows) in fixture(CanonicalChain::Regtest) {
        let keyspace = db
            .keyspace(&namespace, fjall::KeyspaceCreateOptions::default)
            .expect("fixture keyspace");
        for (key, value) in rows {
            keyspace.insert(key, value).expect("fixture row");
        }
    }
    let detached = InspectedRecords::read_existing(&db).expect("bounded existing rows");
    drop(db);
    // Act
    let parsed = CanonicalGenesisRecords::decode(&detached).expect("production detached decoder");
    let result = super::validate_genesis_records(CanonicalChain::Regtest, &parsed);
    std::fs::remove_dir_all(&root).expect("exact owned fixture cleanup");
    // Assert
    assert!(result.is_ok());
    assert!(!root.exists());
}

#[test]
fn phase166_records_proof_rejects_each_binding_mutation() {
    // Arrange
    let chain = CanonicalChain::Regtest;
    // Act / Assert
    for case in 0..16 {
        let mut parsed = parse(&fixture(chain)).expect("complete records");
        let zero = open_bitcoin_core::primitives::BlockHash::from_byte_array([0; 32]);
        let mut one = [0; 32];
        one[0] = 1;
        let wrong_work = parsed.headers[0]
            .chain_work
            .wrapping_add(open_bitcoin_core::consensus::chain_proof::ProofWork::from_le_bytes(one));
        match case {
            0 => parsed.chain = CanonicalChain::Mainnet,
            1 => parsed.identity_hash = zero,
            2 => parsed.body.transactions[0].lock_time += 1,
            3 => parsed.body_key.push('0'),
            4 => parsed.headers[0].height = 1,
            5 => parsed.headers[0].header.time += 1,
            6 => parsed.headers[0].chain_work = wrong_work,
            7 => parsed.index[0].chain_work = wrong_work,
            8 => parsed.active[0].chain_work = wrong_work,
            9 => parsed.active[0].median_time_past += 1,
            10 => parsed.best_block = zero,
            11 => parsed.coverage = history::ValidationCoverage::UnknownLegacy,
            12 => {
                parsed.validation = history::ValidationHistoryRecord::new(
                    parsed.validation.identity(),
                    history::StoredValidationStatus::KnownOnly,
                )
            }
            13 => parsed.headers.push(parsed.headers[0].clone()),
            14 => parsed.index.clear(),
            15 => parsed.active[0].header.previous_block_hash = chain.spec().genesis_hash(),
            _ => unreachable!(),
        }
        assert!(
            super::validate_genesis_records(chain, &parsed).is_err(),
            "case {case}"
        );
    }
}

#[test]
fn phase166_records_self_consistent_descendant_ancestry_remains_unsupported() {
    // Arrange: numerical ancestry cannot supply contextual history qualification.
    let chain = CanonicalChain::Regtest;
    let mut records = parse(&fixture(chain)).expect("records");
    let mut header = records.body.header.clone();
    header.previous_block_hash = chain.spec().genesis_hash();
    header.time += 600;
    let work = records.active[0].chain_work.wrapping_add(
        open_bitcoin_core::consensus::chain_proof::block_proof(header.bits).expect("proof number"),
    );
    let position = ChainPosition::new(header.clone(), 1, work, i64::from(header.time));
    let entry = HeaderEntry {
        block_hash: position.block_hash,
        header,
        height: 1,
        chain_work: work,
    };
    records.headers.push(entry.clone());
    records.index.push(entry);
    records.active.push(position.clone());
    records.best_block = position.block_hash;
    // Act
    let result = super::validate_genesis_records(chain, &records);
    // Assert
    assert!(matches!(
        result,
        Err(CanonicalBootstrapRefusal::UnsupportedHistory)
    ));
}
