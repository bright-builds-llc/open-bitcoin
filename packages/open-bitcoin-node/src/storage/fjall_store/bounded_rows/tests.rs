// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txdb.cpp

use super::super::*;
use super::{RowFamily, codec, decode_detached_rows};
use open_bitcoin_core::consensus::{CanonicalChain, chain_proof::block_proof};
use std::collections::BTreeMap;

fn entries() -> Vec<HeaderEntry> {
    let header = CanonicalChain::Regtest.spec().genesis_block().header;
    vec![
        HeaderEntry {
            block_hash: block_hash(&header),
            chain_work: block_proof(header.bits).expect("work"),
            header,
            height: 0,
        };
        400
    ]
}

fn with_store(check: impl FnOnce(FjallNodeStore)) {
    let mut nonce = [0; 16];
    getrandom::fill(&mut nonce).expect("directory nonce");
    let dir = std::fs::canonicalize(std::env::temp_dir())
        .expect("physical temp")
        .join(format!("phase167-rows-{:x}", u128::from_le_bytes(nonce)));
    check(FjallNodeStore::open(&dir).expect("store"));
    std::fs::remove_dir_all(dir).expect("handles closed");
}

fn rows(store: &FjallNodeStore, namespace: StorageNamespace) -> BTreeMap<Vec<u8>, Vec<u8>> {
    store
        .keyspace(namespace)
        .iter()
        .map(|guard| {
            let (key, value) = guard.into_inner().expect("actual rows");
            (key.to_vec(), value.to_vec())
        })
        .collect()
}

#[test]
fn phase167_durable_rows_public_large_small_large_and_exact_legacy_bytes() {
    with_store(|store| {
        // Arrange
        let large = entries();
        let small = &large[..1];
        // Act / Assert
        for entries in [large.as_slice(), small, large.as_slice()] {
            store
                .save_header_entries(entries, PersistMode::Sync)
                .expect("atomic paired write");
            for (namespace, expected) in [
                (
                    StorageNamespace::Headers,
                    encode_header_entries(entries).expect("old codec"),
                ),
                (
                    StorageNamespace::BlockIndex,
                    encode_block_index_entries(entries).expect("old codec"),
                ),
            ] {
                let stored = rows(&store, namespace);
                let family = if namespace == StorageNamespace::Headers {
                    RowFamily::Headers
                } else {
                    RowFamily::BlockIndex
                };
                if expected.len() <= 65536 {
                    assert_eq!(stored.get(b"snapshot".as_slice()), Some(&expected));
                    assert!(!stored.keys().any(|key| key.starts_with(b"snapshot:")));
                } else {
                    assert!(!stored.contains_key(b"snapshot".as_slice()));
                    assert_eq!(
                        decode_detached_rows(family, &stored, true).expect("complete"),
                        Some(expected)
                    );
                }
            }
        }
    });
}

#[test]
fn phase167_durable_rows_legacy_oversized_generic_readable_canonical_refused() {
    with_store(|store| {
        // Arrange
        let entries = entries();
        let bytes = encode_header_entries(&entries).expect("legacy aggregate");
        assert!(bytes.len() > 65536);
        store
            .write_raw_for_test(StorageNamespace::Headers, "snapshot", bytes)
            .expect("old rows");
        let before = rows(&store, StorageNamespace::Headers);
        // Act / Assert
        assert_eq!(
            store
                .load_header_entries()
                .expect("generic compatibility")
                .expect("present")
                .entries,
            entries
        );
        assert!(decode_detached_rows(RowFamily::Headers, &before, true).is_err());
        assert_eq!(
            rows(&store, StorageNamespace::Headers),
            before,
            "read never repairs"
        );
    });
}

#[test]
fn phase167_durable_rows_exact_proved_genesis_rows_unchanged() {
    // Arrange
    let mut nonce = [0; 16];
    getrandom::fill(&mut nonce).expect("nonce");
    let dir = std::fs::canonicalize(std::env::temp_dir())
        .expect("physical temp")
        .join(format!(
            "phase167-rows-genesis-{:x}",
            u128::from_le_bytes(nonce)
        ));
    {
        let (store, recovered) =
            FjallNodeStore::open_canonical_genesis(&dir, CanonicalChain::Regtest)
                .expect("proved genesis")
                .into_parts()
                .expect("owners");
        // Act
        let headers = store
            .load_header_entries()
            .expect("headers")
            .expect("present");
        let (positions, counts) = store.load_chain_meta_for_open().expect("metadata");
        // Assert
        assert_eq!(
            rows(&store, StorageNamespace::Headers).get(b"snapshot".as_slice()),
            Some(&encode_header_entries(&headers.entries).expect("exact old bytes"))
        );
        assert_eq!(
            rows(&store, StorageNamespace::BlockIndex).get(b"snapshot".as_slice()),
            Some(&encode_block_index_entries(&headers.entries).expect("exact old bytes"))
        );
        assert_eq!(
            rows(&store, StorageNamespace::Chainstate).get(b"chain_meta".as_slice()),
            Some(
                &crate::storage::snapshot_codec::encode_chain_meta(&positions, counts.as_ref())
                    .expect("exact old bytes")
            )
        );
        drop(recovered);
        drop(store);
    }
    std::fs::remove_dir_all(dir).expect("all owners closed");
}

#[derive(Clone, Copy)]
pub(super) enum Tamper {
    Mix,
    Missing,
    Extra,
    Alias,
    Order,
    Count,
    Length,
    Version,
    Kind,
    Namespace,
    Generation,
    Digest,
    OldGeneration,
    OversizedManifest,
    OversizedSegment,
    UnknownKey,
    UnknownField,
    Truncated,
}

pub(super) const TAMPERS: [Tamper; 18] = [
    Tamper::Mix,
    Tamper::Missing,
    Tamper::Extra,
    Tamper::Alias,
    Tamper::Order,
    Tamper::Count,
    Tamper::Length,
    Tamper::Version,
    Tamper::Kind,
    Tamper::Namespace,
    Tamper::Generation,
    Tamper::Digest,
    Tamper::OldGeneration,
    Tamper::OversizedManifest,
    Tamper::OversizedSegment,
    Tamper::UnknownKey,
    Tamper::UnknownField,
    Tamper::Truncated,
];

pub(super) fn corrupt(rows: &mut BTreeMap<Vec<u8>, Vec<u8>>, mutation: Tamper) {
    let key = rows
        .keys()
        .find(|key| key.starts_with(b"snapshot:segment:v1:"))
        .expect("segment")
        .clone();
    let manifest = b"snapshot:segments:v1".to_vec();
    match mutation {
        Tamper::Mix => {
            rows.insert(b"snapshot".to_vec(), vec![1]);
        }
        Tamper::Missing => {
            rows.remove(&key);
        }
        Tamper::Extra => {
            let mut extra = key.clone();
            extra.push(b'0');
            rows.insert(extra, vec![1]);
        }
        Tamper::Alias => {
            let mut alias = key.clone();
            alias.push(b'0');
            let bytes = rows.remove(&key).expect("segment");
            rows.insert(alias, bytes);
        }
        Tamper::Order => {
            let keys: Vec<_> = rows
                .keys()
                .filter(|key| key.starts_with(b"snapshot:segment:v1:"))
                .cloned()
                .collect();
            let first = rows[&keys[0]].clone();
            let second = rows[&keys[1]].clone();
            rows.insert(keys[0].clone(), second);
            rows.insert(keys[1].clone(), first);
        }
        Tamper::Count => rows.get_mut(&manifest).expect("manifest")[13] ^= 1,
        Tamper::Length => rows.get_mut(&manifest).expect("manifest")[21] ^= 1,
        Tamper::Version => rows.get_mut(&manifest).expect("manifest")[7] ^= 1,
        Tamper::Kind => rows.get_mut(&manifest).expect("manifest")[9] ^= 1,
        Tamper::Namespace => rows.get_mut(&manifest).expect("manifest")[8] ^= 1,
        Tamper::Generation => rows.get_mut(&manifest).expect("manifest")[22] ^= 1,
        Tamper::Digest => rows.get_mut(&manifest).expect("manifest")[69] ^= 1,
        Tamper::OldGeneration => {
            let mut old = key.clone();
            old[20] = if old[20] == b'a' { b'b' } else { b'a' };
            let bytes = rows.remove(&key).expect("segment");
            rows.insert(old, bytes);
        }
        Tamper::OversizedManifest => rows.get_mut(&manifest).expect("manifest").resize(65537, 0),
        Tamper::OversizedSegment => rows.get_mut(&key).expect("segment").resize(65537, 0),
        Tamper::UnknownKey => {
            rows.insert(b"snapshot:unknown".to_vec(), vec![1]);
        }
        Tamper::UnknownField => rows.get_mut(&manifest).expect("manifest").push(0),
        Tamper::Truncated => {
            rows.get_mut(&key).expect("segment").pop();
        }
    }
}

fn tamper_case(mutation: Tamper) {
    with_store(|store| {
        // Arrange
        store
            .save_header_entries(&entries(), PersistMode::Sync)
            .expect("real writer");
        let before_index = rows(&store, StorageNamespace::BlockIndex);
        let mut changed = rows(&store, StorageNamespace::Headers);
        corrupt(&mut changed, mutation);
        let mut batch = store.db.batch().durability(Some(FjallPersistMode::SyncAll));
        for key in rows(&store, StorageNamespace::Headers).keys() {
            batch.remove(&store.headers, key.clone());
        }
        for (key, value) in &changed {
            batch.insert(&store.headers, key.clone(), value.clone());
        }
        batch.commit().expect("hostile stored bytes");
        // Act / Assert
        assert!(store.load_header_entries().is_err());
        assert!(decode_detached_rows(RowFamily::Headers, &changed, true).is_err());
        assert_eq!(
            rows(&store, StorageNamespace::Headers),
            changed,
            "no repair"
        );
        assert_eq!(
            rows(&store, StorageNamespace::BlockIndex),
            before_index,
            "other accepted projection unchanged"
        );
    });
}

macro_rules! tamper_test {
    ($name:ident, $variant:ident) => {
        #[test]
        fn $name() {
            tamper_case(Tamper::$variant);
        }
    };
}
tamper_test!(phase167_durable_rows_refuses_mixed_root, Mix);
tamper_test!(phase167_durable_rows_refuses_missing, Missing);
tamper_test!(phase167_durable_rows_refuses_extra, Extra);
tamper_test!(phase167_durable_rows_refuses_alias, Alias);
tamper_test!(phase167_durable_rows_refuses_order, Order);
tamper_test!(phase167_durable_rows_refuses_count, Count);
tamper_test!(phase167_durable_rows_refuses_length, Length);
tamper_test!(phase167_durable_rows_refuses_version, Version);
tamper_test!(phase167_durable_rows_refuses_kind, Kind);
tamper_test!(phase167_durable_rows_refuses_namespace, Namespace);
tamper_test!(phase167_durable_rows_refuses_generation, Generation);
tamper_test!(phase167_durable_rows_refuses_digest, Digest);
tamper_test!(phase167_durable_rows_refuses_old_generation, OldGeneration);
tamper_test!(
    phase167_durable_rows_refuses_oversized_manifest,
    OversizedManifest
);
tamper_test!(
    phase167_durable_rows_refuses_oversized_segment,
    OversizedSegment
);
tamper_test!(phase167_durable_rows_refuses_unknown_key, UnknownKey);
tamper_test!(phase167_durable_rows_refuses_unknown_field, UnknownField);
tamper_test!(phase167_durable_rows_refuses_truncated, Truncated);

#[test]
fn phase167_durable_rows_format_bounds_reject_before_reconstruction() {
    // Arrange
    let mut rows = codec::prepare(RowFamily::Headers, vec![7; 65537])
        .expect("format")
        .rows;
    rows.get_mut(b"snapshot:segments:v1".as_slice())
        .expect("manifest")[14..22]
        .copy_from_slice(&u64::MAX.to_be_bytes());
    // Act / Assert
    assert!(decode_detached_rows(RowFamily::Headers, &rows, true).is_err());
}

#[test]
fn phase167_durable_rows_metadata_large_small_large_retains_exact_map_and_loaders() {
    with_store(|store| {
        // Arrange: untrusted positions exercise the data writer, never proof.
        let header = CanonicalChain::Regtest.spec().genesis_block().header;
        let position = open_bitcoin_core::chainstate::ChainPosition {
            block_hash: block_hash(&header),
            height: 0,
            chain_work: block_proof(header.bits).expect("work"),
            header,
            median_time_past: 1296688602,
        };
        let large = vec![position.clone(); 401];
        let small = vec![position];
        let counts = std::collections::HashMap::from([(
            open_bitcoin_core::primitives::Txid::from_byte_array([9; 32]),
            3,
        )]);
        // Act / Assert
        for positions in [&large, &small, &large] {
            store
                .save_unverified_chain_meta(positions, Some(&counts), PersistMode::Sync)
                .expect("shared metadata writer");
            let expected = (positions.clone(), Some(counts.clone()));
            assert_eq!(
                store.load_chain_meta_for_open().expect("metadata"),
                expected
            );
            assert_eq!(
                store.load_hydrate_chain_meta().expect("hydration metadata"),
                expected
            );
            let physical = rows(&store, StorageNamespace::Chainstate);
            let encoded =
                crate::storage::snapshot_codec::encode_chain_meta(positions, Some(&counts))
                    .expect("old codec");
            assert_eq!(
                decode_detached_rows(RowFamily::ChainMeta, &physical, true).expect("strict bytes"),
                Some(encoded.clone())
            );
            assert_eq!(
                physical.contains_key(b"chain_meta".as_slice()),
                encoded.len() <= 65536
            );
        }
    });
}

#[test]
fn phase167_durable_rows_public_writer_bounds_actual_persisted_values() {
    // Arrange
    let mut nonce = [0; 16];
    getrandom::fill(&mut nonce).expect("directory nonce");
    let dir = std::fs::canonicalize(std::env::temp_dir())
        .expect("physical temp")
        .join(format!("phase167-rows-{:x}", u128::from_le_bytes(nonce)));
    {
        let store = FjallNodeStore::open(&dir).expect("store");
        let entries = entries();
        // Act
        store
            .save_header_entries(&entries, PersistMode::Sync)
            .expect("save");
        // Assert
        for namespace in [StorageNamespace::Headers, StorageNamespace::BlockIndex] {
            for row in store.keyspace(namespace).iter() {
                let (_, value) = row.into_inner().expect("actual row");
                assert!(
                    value.len() <= 65536,
                    "actual persisted authority value {} exceeds 65536",
                    value.len()
                );
            }
        }
        assert_eq!(
            store
                .load_header_entries()
                .expect("load")
                .expect("present")
                .entries,
            entries
        );
    }
    std::fs::remove_dir_all(dir).expect("handles closed");
}
