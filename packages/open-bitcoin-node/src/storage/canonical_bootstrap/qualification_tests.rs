// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/txdb.cpp

use super::test_support::*;
use crate::{
    FjallNodeStore, StorageNamespace,
    storage::{
        canonical_bootstrap::CanonicalBootstrapRefusal as Refusal,
        snapshot_codec::{ClassifiedWorkRecord, decode_work_record, encode_work_record},
        validation_history as history,
    },
};
use StorageNamespace::{BlockIndex, Chainstate, Coins, Headers, Schema};
use open_bitcoin_core::{
    chainstate::{FlushMode, FlushPolicyTime},
    consensus::CanonicalChain,
};
use std::fs;

const PRIVATE_SENTINEL: &[u8] = b"private-wallet-qualification-sentinel";

#[test]
fn phase166_qualification_allowed_private_metadata_is_not_authority_or_a_refusal_cause() {
    let _scenario = SCENARIO.lock().expect("serialized durable scenario");
    // Arrange
    let source = source();
    drop(runtime(
        &source,
        CanonicalChain::Regtest,
        crate::SyncNetwork::Regtest,
        false,
    ));
    let raw = FjallNodeStore::open_without_ensure_schema_for_test(&source)
        .expect("unrelated metadata writer");
    keyspace(&raw, StorageNamespace::Wallet)
        .insert("snapshot", PRIVATE_SENTINEL)
        .expect("allowed metadata");
    raw.database()
        .persist(fjall::PersistMode::SyncAll)
        .expect("metadata persistence");
    drop(raw);
    // Act
    let reopened = runtime(
        &source,
        CanonicalChain::Regtest,
        crate::SyncNetwork::Regtest,
        true,
    );
    // Assert
    assert_runtime(&reopened, &vector("regtest"));
    drop(reopened);
    fs::remove_dir_all(source).expect("owned fixture cleanup");
}

#[test]
fn phase166_qualification_wrong_selected_network_preserves_valid_source() {
    let _scenario = SCENARIO.lock().expect("serialized durable scenario");
    assert_corrupt_refusal(
        "selected-network",
        Refusal::InconsistentGenesis,
        CanonicalChain::Mainnet,
        |_| {},
    );
}

#[cfg(unix)]
#[test]
fn phase166_qualification_unsafe_alias_preserves_source_and_link() {
    let _scenario = SCENARIO.lock().expect("serialized durable scenario");
    // Arrange
    let source = source();
    drop(runtime(
        &source,
        CanonicalChain::Regtest,
        crate::SyncNetwork::Regtest,
        false,
    ));
    let alias = source.with_extension("alias");
    std::os::unix::fs::symlink(&source, &alias).expect("owned unsafe alias");
    let before = physical_manifest(&source);
    let alias_before = physical_manifest(&alias);
    let builders = crate::storage::chain_compatibility::BACKEND_BUILDER_CALLS.get();
    // Act
    let result = FjallNodeStore::reopen_canonical_genesis_existing(&alias, CanonicalChain::Regtest);
    // Assert
    assert!(matches!(result, Err(Refusal::StorageUnavailable)));
    assert_eq!(
        builders,
        crate::storage::chain_compatibility::BACKEND_BUILDER_CALLS.get()
    );
    assert!(
        before == physical_manifest(&source),
        "unsafe alias cannot mutate source payload"
    );
    assert!(
        alias_before == physical_manifest(&alias),
        "owned link is unchanged"
    );
    fs::remove_file(alias).expect("remove only owned link");
    fs::remove_dir_all(source).expect("owned fixture cleanup");
}

fn assert_corrupt_refusal(
    name: &str,
    expected: Refusal,
    selected: CanonicalChain,
    mutate: impl FnOnce(&FjallNodeStore),
) {
    // Arrange: authority comes only from the product initializer/runtime.
    let source = source();
    drop(runtime(
        &source,
        CanonicalChain::Regtest,
        crate::SyncNetwork::Regtest,
        false,
    ));
    let raw = FjallNodeStore::open_without_ensure_schema_for_test(&source)
        .expect("explicit corruption-only writer after product drop");
    keyspace(&raw, StorageNamespace::Wallet)
        .insert("snapshot", PRIVATE_SENTINEL)
        .expect("allowed unrelated private metadata");
    mutate(&raw);
    raw.database()
        .persist(fjall::PersistMode::SyncAll)
        .expect("durable test corruption");
    drop(raw);
    let before = physical_manifest(&source);
    let builders = crate::storage::chain_compatibility::BACKEND_BUILDER_CALLS.get();
    // Act
    let result = FjallNodeStore::reopen_canonical_genesis_existing(&source, selected);
    // Assert
    let actual = result
        .err()
        .expect("corrupt input must yield no accepted owner");
    assert_eq!(actual, expected, "fixed category for {name}");
    let diagnostic = format!("{actual:?}: {actual}");
    assert!(diagnostic.len() < 128, "bounded category for {name}");
    assert!(!diagnostic.contains(source.to_str().expect("fixture path")));
    assert!(!diagnostic.contains(std::str::from_utf8(PRIVATE_SENTINEL).expect("sentinel")));
    assert_eq!(
        builders,
        crate::storage::chain_compatibility::BACKEND_BUILDER_CALLS.get(),
        "original backend never reached for {name}"
    );
    assert!(
        before == physical_manifest(&source),
        "source names/types/lengths/bytes/modes/mtime preserved for {name}"
    );
    fs::remove_dir_all(source).expect("owned fixture cleanup");
    eprintln!("qualification case={name} result={actual:?} source_preserved=true");
}

const AUTHORITY_CASES: &[(&str, Refusal, Mutation)] = &[
    (
        "stored-selector",
        Refusal::InconsistentGenesis,
        Mutation::IdentitySelector,
    ),
    (
        "stored-root",
        Refusal::InconsistentGenesis,
        Mutation::IdentityRoot,
    ),
    (
        "full-body-locktime",
        Refusal::InconsistentGenesis,
        Mutation::Body,
    ),
    (
        "header-hash",
        Refusal::MalformedRecord,
        Mutation::JsonByte(Headers, "snapshot", "/payload/entries/0/block_hash/0"),
    ),
    (
        "header-parent",
        Refusal::MalformedRecord,
        Mutation::JsonByte(
            Headers,
            "snapshot",
            "/payload/entries/0/header/previous_block_hash/0",
        ),
    ),
    (
        "header-height",
        Refusal::InconsistentGenesis,
        Mutation::JsonNumber(Headers, "snapshot", "/payload/entries/0/height"),
    ),
    (
        "header-nonce",
        Refusal::MalformedRecord,
        Mutation::JsonNumber(Headers, "snapshot", "/payload/entries/0/header/nonce"),
    ),
    (
        "header-work",
        Refusal::InconsistentGenesis,
        Mutation::JsonByte(
            Headers,
            "snapshot",
            "/payload/entries/0/chain_work/bytes_le/0",
        ),
    ),
    (
        "index-hash",
        Refusal::MalformedRecord,
        Mutation::JsonByte(BlockIndex, "snapshot", "/payload/entries/0/block_hash/0"),
    ),
    (
        "index-work",
        Refusal::InconsistentGenesis,
        Mutation::JsonByte(
            BlockIndex,
            "snapshot",
            "/payload/entries/0/chain_work/bytes_le/0",
        ),
    ),
    (
        "active-hash",
        Refusal::MalformedRecord,
        Mutation::JsonByte(
            Chainstate,
            "chain_meta",
            "/payload/active_chain/0/block_hash/0",
        ),
    ),
    (
        "active-parent",
        Refusal::MalformedRecord,
        Mutation::JsonByte(
            Chainstate,
            "chain_meta",
            "/payload/active_chain/0/header/previous_block_hash/0",
        ),
    ),
    (
        "active-height",
        Refusal::InconsistentGenesis,
        Mutation::JsonNumber(Chainstate, "chain_meta", "/payload/active_chain/0/height"),
    ),
    (
        "active-work",
        Refusal::InconsistentGenesis,
        Mutation::JsonByte(
            Chainstate,
            "chain_meta",
            "/payload/active_chain/0/chain_work/bytes_le/0",
        ),
    ),
    (
        "active-median-time",
        Refusal::InconsistentGenesis,
        Mutation::JsonNumber(
            Chainstate,
            "chain_meta",
            "/payload/active_chain/0/median_time_past",
        ),
    ),
    (
        "legacy-work",
        Refusal::MalformedRecord,
        Mutation::LegacyWork,
    ),
    ("coverage", Refusal::InconsistentGenesis, Mutation::Coverage),
    (
        "validation-status",
        Refusal::InconsistentGenesis,
        Mutation::ValidationStatus,
    ),
    (
        "missing-identity",
        Refusal::MissingRecord,
        Mutation::Missing(Schema, super::super::records::IDENTITY_KEY),
    ),
    (
        "missing-schema",
        Refusal::MissingRecord,
        Mutation::Missing(Schema, "schema_version"),
    ),
    (
        "missing-index",
        Refusal::MissingRecord,
        Mutation::Missing(BlockIndex, "snapshot"),
    ),
    (
        "missing-body",
        Refusal::MissingRecord,
        Mutation::MissingBody,
    ),
    (
        "missing-coverage",
        Refusal::MissingRecord,
        Mutation::Missing(BlockIndex, history::COVERAGE_KEY),
    ),
    (
        "missing-validation",
        Refusal::MissingRecord,
        Mutation::MissingValidation,
    ),
    ("wrong-B", Refusal::InconsistentGenesis, Mutation::BestBlock),
    (
        "unknown-authority-row",
        Refusal::UnsupportedRecord,
        Mutation::Extra(BlockIndex, "unproved-authority"),
    ),
    (
        "missing-header",
        Refusal::MissingRecord,
        Mutation::Missing(Headers, "snapshot"),
    ),
    (
        "missing-active",
        Refusal::MissingRecord,
        Mutation::Missing(Chainstate, "chain_meta"),
    ),
    (
        "missing-B",
        Refusal::MissingRecord,
        Mutation::Missing(Coins, "B"),
    ),
    (
        "interrupted-H",
        Refusal::UnsupportedRecord,
        Mutation::Extra(Coins, "H"),
    ),
    (
        "genesis-C",
        Refusal::UnsupportedRecord,
        Mutation::Extra(Coins, "C"),
    ),
    (
        "oversized-authority",
        Refusal::MalformedRecord,
        Mutation::Oversized,
    ),
    (
        "numerically-consistent-descendant",
        Refusal::UnsupportedHistory,
        Mutation::Descendant,
    ),
];

#[test]
fn phase166_qualification_independent_durable_authority_binding_matrix() {
    let _scenario = SCENARIO.lock().expect("serialized durable scenario");
    for &(name, expected, mutation) in AUTHORITY_CASES {
        assert_corrupt_refusal(name, expected, CanonicalChain::Regtest, |raw| {
            apply_mutation(raw, mutation)
        });
    }
}

fn replace_work_everywhere(
    raw: &FjallNodeStore,
    work: open_bitcoin_core::consensus::chain_proof::ProofWork,
) {
    let value = serde_json::to_value(ClassifiedWorkRecord::ExactNumber(work))
        .expect("exact numerical record");
    for namespace in [StorageNamespace::Headers, StorageNamespace::BlockIndex] {
        rewrite_json(raw, namespace, "snapshot", |json| {
            json["payload"]["entries"][0]["chain_work"] = value.clone()
        });
    }
    rewrite_json(raw, StorageNamespace::Chainstate, "chain_meta", |json| {
        json["payload"]["active_chain"][0]["chain_work"] = value
    });
}

#[test]
fn phase166_qualification_all_32_coordinated_work_bytes_recompute_from_actual_headers() {
    let _scenario = SCENARIO.lock().expect("serialized durable scenario");
    let fixed: [u8; 32] = bytes(
        vector("regtest")["proof_le"]
            .as_str()
            .expect("independent genesis work"),
    )
    .try_into()
    .expect("32 work bytes");
    for offset in 0..32 {
        let mut changed = fixed;
        changed[offset] ^= 1;
        let name = format!("coordinated-work-byte-{offset}");
        assert_corrupt_refusal(
            &name,
            Refusal::InconsistentGenesis,
            CanonicalChain::Regtest,
            |raw| {
                replace_work_everywhere(
                    raw,
                    open_bitcoin_core::consensus::chain_proof::ProofWork::from_le_bytes(changed),
                )
            },
        );
    }
}

#[test]
fn phase166_qualification_wide_exact_numbers_never_become_stored_authority() {
    let _scenario = SCENARIO.lock().expect("serialized durable scenario");
    for case in vectors()["arithmetic_only"]["encoding"]
        .as_array()
        .expect("independent wide vectors")
    {
        let number = numeric_work(&case["hex"]);
        assert_corrupt_refusal(
            "exact-looking-wide-work",
            Refusal::InconsistentGenesis,
            CanonicalChain::Regtest,
            |raw| replace_work_everywhere(raw, number),
        );
    }
}

#[test]
fn phase166_qualification_four_selector_managed_checkpoint_and_independent_reopen() {
    let _scenario = SCENARIO.lock().expect("serialized durable scenario");
    for mode in [
        crate::chainstate::BasicFilterStartupMode::Enabled,
        crate::chainstate::BasicFilterStartupMode::Disabled,
    ] {
        for (chain, network, name) in SELECTORS {
            // Arrange
            let source = source();
            let first = runtime_mode(&source, chain, network, false, mode);
            let expected = vector(name);
            assert_runtime_mode(&first, &expected, mode);
            // Act
            let checkpoint = first
                .network_handle()
                .flush_coins(
                    FlushMode::Always,
                    FlushPolicyTime::from_unix_seconds(100),
                    u64::MAX,
                )
                .expect("actual managed checkpoint");
            assert!(
                checkpoint.wrote_coins,
                "Always performs a real coins checkpoint"
            );
            assert_eq!(
                first
                    .store()
                    .load_chain_meta_for_open()
                    .expect("checkpoint metadata")
                    .1,
                Some(std::collections::HashMap::new()),
                "current explicit empty counts survive the actual checkpoint"
            );
            let store = first.store().clone();
            let view = store.coins_view();
            let handle = first.network_handle();
            drop(first);
            assert!(
                matches!(
                    FjallNodeStore::reopen_canonical_genesis_existing(&source, chain),
                    Err(Refusal::StorageUnavailable)
                ),
                "live handle/adapter/view must prevent false reopen"
            );
            drop(handle);
            drop(store);
            assert!(
                matches!(
                    FjallNodeStore::reopen_canonical_genesis_existing(&source, chain),
                    Err(Refusal::StorageUnavailable)
                ),
                "extracted coins view retains the final store owner"
            );
            drop(view);
            let reopened = runtime_mode(&source, chain, network, true, mode);
            // Assert
            assert_runtime_mode(&reopened, &expected, mode);
            drop(reopened);
            reopen_in_independent_child(&source, name, mode);
            fs::remove_dir_all(source).expect("owned fixture cleanup");
            eprintln!(
                "qualification checkpoint selector={name} basic={mode:?} work_bytes=32 independent_child=true"
            );
        }
    }
}

#[test]
fn phase166_qualification_wide_vectors_are_numeric_only() {
    // Arrange
    let fixture = vectors();
    assert_eq!(
        fixture["arithmetic_only"]["evidence_class"],
        "arithmetic-only-no-chain-authority"
    );
    for case in fixture["arithmetic_only"]["encoding"]
        .as_array()
        .expect("wide encoding vectors")
    {
        let number = numeric_work(&case["hex"]);
        // Act
        let encoded = encode_work_record(
            StorageNamespace::Chainstate,
            &ClassifiedWorkRecord::ExactNumber(number),
        )
        .expect("numeric encoding");
        let decoded =
            decode_work_record(StorageNamespace::Chainstate, &encoded).expect("numeric decoding");
        // Assert
        assert_eq!(decoded, ClassifiedWorkRecord::ExactNumber(number));
        assert_eq!(
            number.to_le_bytes().as_slice(),
            bytes(
                case["little_endian"]
                    .as_str()
                    .expect("independent LE bytes")
            )
        );
        assert_eq!(number.to_hex(), case["hex"]);
    }
    for case in fixture["arithmetic_only"]["addition"]
        .as_array()
        .expect("wrapping arithmetic vectors")
    {
        // Act
        let sum = numeric_work(&case["left_hex"]).wrapping_add(numeric_work(&case["right_hex"]));
        // Assert
        assert_eq!(sum.to_hex(), case["sum_hex"]);
        assert_eq!(
            sum.to_le_bytes().as_slice(),
            bytes(case["sum_le"].as_str().expect("independent sum LE"))
        );
    }
}
