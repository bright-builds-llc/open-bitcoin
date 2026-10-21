// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/blockfilter.cpp

use super::*;
use crate::storage::filter_index;
use crate::{
    DurableSyncRuntime, SyncNetwork, SyncRuntimeConfig, chainstate::BasicFilterStartupMode,
};
use open_bitcoin_core::chainstate::CoinsView;
use std::os::unix::fs::MetadataExt;

fn enabled(source: &Path) -> DurableSyncRuntime {
    DurableSyncRuntime::open_canonical_genesis_configured(
        open(source, CanonicalChain::Regtest),
        SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            ..Default::default()
        },
        BasicFilterStartupMode::Enabled,
    )
    .expect("actual canonical BASIC runtime")
}

#[test]
fn phase166_basic_all_live_owners_release_before_independent_reopen() {
    // Arrange
    let source = source();
    let first = enabled(&source);
    let first_identity = Arc::downgrade(&first.store().filter_publication);
    let hash = CanonicalChain::Regtest.spec().genesis_hash();
    let record = first
        .store()
        .load_basic_filter_record(hash)
        .expect("record")
        .expect("genesis");
    let handle = first.network_handle();
    drop(first);
    assert!(
        first_identity.upgrade().is_some(),
        "network handle retains original store"
    );
    assert!(
        FjallNodeStore::reopen_canonical_genesis_existing(&source, CanonicalChain::Regtest)
            .is_err(),
        "a cloned live handle is not reopen evidence"
    );
    drop(handle);
    assert!(
        first_identity.upgrade().is_none(),
        "every original store owner released"
    );
    // Act
    let reopened = enabled(&source);
    // Assert
    assert_eq!(
        reopened
            .store()
            .load_basic_filter_record(hash)
            .expect("reopened bytes"),
        Some(record)
    );
    assert_eq!(
        reopened.store().coins_view().best_block().expect("B"),
        Some(hash)
    );
    drop(reopened);
    fs::remove_dir_all(source).expect("cleanup");
}

#[test]
fn phase166_basic_cleanup_failure_refuses_saved_index_before_original_effects() {
    // Arrange
    let source = source();
    drop(enabled(&source));
    let before = snapshot(&source);
    let builders = crate::storage::chain_compatibility::BACKEND_BUILDER_CALLS.get();
    inspection::inject_cleanup_failure_for_test();
    // Act
    let result =
        FjallNodeStore::reopen_canonical_genesis_existing(&source, CanonicalChain::Regtest);
    // Assert
    assert!(matches!(result, Err(Refusal::StorageUnavailable)));
    assert_eq!(
        builders,
        crate::storage::chain_compatibility::BACKEND_BUILDER_CALLS.get()
    );
    assert_eq!(before, snapshot(&source));
    let scratch = inspection::take_observed_scratch_for_test().expect("exact parent-owned root");
    assert_eq!(
        scratch.parent().expect("parent"),
        fs::canonicalize("/tmp").expect("private temp parent")
    );
    assert!(
        scratch
            .file_name()
            .expect("name")
            .to_string_lossy()
            .starts_with("open-bitcoin-inspection-")
    );
    let identity = fs::symlink_metadata(&scratch).expect("retained root identity");
    assert!(identity.is_dir() && !identity.file_type().is_symlink());
    assert_eq!(identity.mode() & 0o777, 0o700);
    assert!(
        fs::metadata(scratch.join("result"))
            .expect("child result")
            .len()
            > 0
    );
    let child_lock = fs::File::open(scratch.join("database/lock")).expect("scratch engine lock");
    child_lock
        .try_lock()
        .expect("completed child released every engine handle");
    drop(child_lock);
    let checked = fs::symlink_metadata(&scratch).expect("same exact root before cleanup");
    assert_eq!(
        (checked.dev(), checked.ino()),
        (identity.dev(), identity.ino())
    );
    fs::remove_dir_all(&scratch).expect("remove only identity-checked owned root");
    assert!(!scratch.exists());
    drop(enabled(&source));
    fs::remove_dir_all(source).expect("fixture cleanup");
}

fn assert_saved_corruption_preserved(mutate: impl FnOnce(&FjallNodeStore)) {
    let source = source();
    drop(enabled(&source));
    let raw = FjallNodeStore::open_unrecovered(&source).expect("corruption fixture");
    mutate(&raw);
    raw.db
        .persist(FjallPersistMode::SyncAll)
        .expect("durable corruption");
    drop(raw);
    let before = snapshot(&source);
    let builders = crate::storage::chain_compatibility::BACKEND_BUILDER_CALLS.get();
    let result =
        FjallNodeStore::reopen_canonical_genesis_existing(&source, CanonicalChain::Regtest);
    assert!(
        matches!(
            result,
            Err(Refusal::InconsistentIndex) | Err(Refusal::UnsupportedHistory)
        ),
        "saved corruption never grants core/index owner"
    );
    assert_eq!(
        builders,
        crate::storage::chain_compatibility::BACKEND_BUILDER_CALLS.get()
    );
    assert_eq!(
        before,
        snapshot(&source),
        "every original byte/mode/mtime preserved"
    );
    fs::remove_dir_all(source).expect("cleanup");
}

#[test]
fn phase166_basic_wrong_filter_or_predecessor_refuses_before_original_effects() {
    for wrong_content in [false, true] {
        // Arrange / Act / Assert
        assert_saved_corruption_preserved(|raw| {
            let key = filter_index::record_key(CanonicalChain::Regtest.spec().genesis_hash());
            let mut bytes = raw
                .block_index
                .get(&key)
                .expect("read")
                .expect("record")
                .to_vec();
            if wrong_content {
                // Internally consistent empty BASIC still disagrees with the full genesis body.
                let hash = open_bitcoin_core::consensus::crypto::double_sha256(&[0]);
                let mut commitment = hash.to_vec();
                commitment.extend_from_slice(&[0; 32]);
                bytes[102..134].copy_from_slice(&hash);
                bytes[134..166].copy_from_slice(
                    &open_bitcoin_core::consensus::crypto::double_sha256(&commitment),
                );
                bytes[166..170].copy_from_slice(&1_u32.to_le_bytes());
                bytes.truncate(170);
                bytes.push(0);
            } else {
                bytes[70] = 1;
            }
            raw.block_index
                .insert(key, bytes)
                .expect("mutated saved record");
        });
    }
}

#[test]
fn phase166_basic_wrong_tip_owner_or_missing_pair_refuses_before_original_effects() {
    for corruption in 0..5 {
        // Arrange / Act / Assert
        assert_saved_corruption_preserved(|raw| {
            let key = match corruption {
                0 | 1 => filter_index::STATE_KEY.to_owned(),
                2 => filter_index::ownership::OWNER_KEY.to_owned(),
                3 => filter_index::active_key(0),
                _ => filter_index::record_key(CanonicalChain::Regtest.spec().genesis_hash()),
            };
            let mut bytes = raw
                .block_index
                .get(&key)
                .expect("read")
                .expect("saved row")
                .to_vec();
            match corruption {
                0 => bytes[7] ^= 1,  // checkpoint endpoint hash
                1 => bytes[43] ^= 1, // durable fence hash
                2 => bytes[2] = 2,   // unknown saved lifecycle mode
                _ => {
                    raw.block_index.remove(key).expect("missing half pair");
                    return;
                }
            }
            raw.block_index.insert(key, bytes).expect("corruption");
        });
    }
}
