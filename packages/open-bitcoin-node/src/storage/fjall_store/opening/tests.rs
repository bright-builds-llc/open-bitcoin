// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;
use crate::storage::CanonicalBootstrapRefusal;
use open_bitcoin_core::consensus::CanonicalChain;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::{
    collections::BTreeMap,
    fs, io,
    path::{Path, PathBuf},
    thread,
    time::{Duration, Instant},
};

type Rows = BTreeMap<String, BTreeMap<Vec<u8>, Vec<u8>>>;
const WRITES: usize = 4096;
const VALUE_BYTES: usize = 64 * 1024;
const OBSERVATION: Duration = Duration::from_secs(30);
const KEY: &[u8] = b"phase167-journal-pressure";

fn source() -> PathBuf {
    let mut nonce = [0_u8; 16];
    getrandom::fill(&mut nonce).expect("unique fixture nonce");
    fs::canonicalize(std::env::temp_dir())
        .expect("physical temporary parent")
        .join(format!(
            "phase167-journal-pressure-{:x}",
            u128::from_le_bytes(nonce)
        ))
}

fn rows(store: &FjallNodeStore) -> Rows {
    let snapshot = store.database().snapshot();
    let result = [
        ("headers", &store.headers),
        ("block_index", &store.block_index),
        ("chainstate", &store.chainstate),
        ("coins", &store.coins),
        ("wallet", &store.wallet),
        ("metrics", &store.metrics),
        ("mempool", &store.mempool),
        ("runtime", &store.runtime),
        ("schema", &store.schema),
    ]
    .into_iter()
    .map(|(name, keyspace)| {
        let values = snapshot
            .iter(keyspace)
            .map(|row| {
                let (key, value) = row.into_inner().expect("complete native snapshot row");
                (key.to_vec(), value.to_vec())
            })
            .collect();
        (name.to_owned(), values)
    })
    .collect::<Rows>();
    let mut catalog = store.database().list_keyspace_names();
    catalog.sort();
    assert_eq!(catalog, result.keys().cloned().collect::<Vec<_>>());
    assert_eq!(result.len(), 9, "all required maps and catalog");
    result
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct JournalIdentity {
    relative: PathBuf,
    #[cfg(unix)]
    device: u64,
    #[cfg(unix)]
    inode: u64,
}

mod observation;
use observation::{Observation, ObservationPoint, ObservationSlots, ObservationWork, Physical};

#[test]
fn phase167_journal_pressure_retires_cold_genesis_journal_preserving_authority_observer_known_file_notfound_changed()
 {
    // Arrange
    let root = source();
    fs::create_dir(&root).expect("owned observer root");
    let payload = root.join("payload");
    fs::write(&payload, [1, 2, 3, 4]).expect("four-byte real file");
    let mut work = ObservationWork::default();
    let mut calls = 0;
    // Act
    let result = Physical::observe_with(&root, &mut work, &mut |point, target| {
        if point == ObservationPoint::BeforeMetadata && target == payload {
            fs::remove_file(target).expect("real enumerated descendant unlink");
            calls += 1;
        }
    });
    fs::remove_dir_all(&root).expect("cleanup before intended RED assertion");
    // Assert
    assert_eq!(calls, 1);
    assert_eq!((work.visits, work.bytes), (2, 0));
    assert!(
        matches!(&result, Ok(Observation::Changed(_, _))),
        "actual observer result: {result:?}"
    );
    let Ok(Observation::Changed(error, spent)) = result else {
        panic!("no complete partial observation")
    };
    assert_eq!(error.kind(), io::ErrorKind::NotFound);
    assert_eq!(spent.visits, 2);
}

#[test]
fn phase167_journal_pressure_retires_cold_genesis_journal_preserving_authority() {
    // Arrange: genuine canonical proof, cold schema rows, detached observations only.
    let source = source();
    assert!(!source.exists(), "actual absent disposable source");
    let opened = FjallNodeStore::open_canonical_genesis(&source, CanonicalChain::Regtest)
        .expect("genuine production canonical genesis factory");
    let expected_genesis = opened.genesis().clone();
    let expected_position = opened.position().clone();
    let (store, recovered) = opened.into_parts().expect("actual canonical owners");
    assert_eq!(
        recovered.check(&store).expect("original seal"),
        &expected_genesis
    );
    let before = rows(&store);
    assert!(
        !before["schema"].is_empty(),
        "real genesis schema remains cold"
    );
    assert!(before["metrics"].is_empty(), "existing empty hot namespace");
    let stamp = store.database().stamp().expect("original idle stamp");
    store
        .database()
        .check_stamp(&stamp)
        .expect("original stamp valid");
    let held = store.metrics.clone();
    let started = Instant::now();
    let initial = Physical::read(&source).expect("initial complete physical observation");
    assert_eq!(initial.journals.len(), 1, "one actual initial journal");
    let original = initial.journals[0].clone();
    initial.report("initial", 0, started);
    let value = vec![0x5a; VALUE_BYTES];

    // Act: one key and reused bounded payload, through the real tracked clone.
    for completed in 1..=WRITES {
        held.insert(KEY, value.as_slice())
            .expect("actual bounded native insert");
        if completed % 256 == 0 {
            Physical::observe(&source)
                .expect("bounded producer physical observation")
                .report("producer", completed, started);
            thread::yield_now();
        }
    }
    let observation_started = Instant::now();
    let mut observations = ObservationSlots::default();
    let retired = loop {
        let observation = observations
            .observe(&source, &mut |_, _| {})
            .expect("bounded retirement observation");
        if let Some(retired) =
            observation.maybe_finished(&original, || observation_started.elapsed())
        {
            observation.report(
                if retired {
                    "retired"
                } else {
                    "retained-at-bound"
                },
                WRITES,
                started,
            );
            break retired;
        }
        thread::sleep(Duration::from_millis(5));
    };
    held.remove(KEY)
        .expect("restore exact temporary key absence");

    // Assert: same complete logical source cannot revive the original observations.
    assert_eq!(rows(&store), before, "all nine maps and catalog restored");
    assert!(
        store.database().check_stamp(&stamp).is_err(),
        "original stamp stale"
    );
    assert!(matches!(
        recovered.check(&store),
        Err(CanonicalBootstrapRefusal::ChangedStore)
    ));
    eprintln!(
        "JOURNAL_PRESSURE restored_all9=true cold_schema_equal=true old_stamp_stale=true old_seal_changed_store=true completed_writes={WRITES} retired={retired}"
    );
    drop(held);
    drop(stamp);
    drop(recovered);
    drop(store);

    if retired {
        let reopened =
            FjallNodeStore::reopen_canonical_genesis_existing(&source, CanonicalChain::Regtest)
                .expect("fresh public safe genesis reopen after every original owner drops");
        assert_eq!(reopened.genesis(), &expected_genesis);
        assert_eq!(reopened.position(), &expected_position);
        let (store, recovered) = reopened.into_parts().expect("fresh actual proof consumed");
        store
            .check_submission_physical_inventory()
            .expect("unchanged physical capture and recheck");
        assert_eq!(
            recovered.check(&store).expect("fresh valid seal"),
            &expected_genesis
        );
        assert_eq!(
            rows(&store),
            before,
            "fresh proof retains all nine original maps"
        );
        drop(recovered);
        drop(store);
        eprintln!(
            "JOURNAL_PRESSURE full_drop_safe_reopen=true fresh_seal_valid=true exact_genesis_position_work=true all9_equal=true"
        );
    }
    fs::remove_dir_all(&source).expect("all owners released before fixture cleanup");
    assert!(
        retired,
        "original cold genesis journal must retire automatically"
    );
}

#[test]
fn phase167_journal_pressure_retires_cold_genesis_journal_preserving_authority_observer_known_directory_notfound_changed()
 {
    observation::known_directory_notfound_changed();
}

#[cfg(unix)]
#[test]
fn phase167_journal_pressure_retires_cold_genesis_journal_preserving_authority_observer_root_unsafe_and_permanent_refused()
 {
    observation::root_unsafe_and_permanent_refused();
}

#[test]
fn phase167_journal_pressure_retires_cold_genesis_journal_preserving_authority_observer_limits_precede_walk_io()
 {
    observation::limits_precede_walk_io();
}

#[test]
fn phase167_journal_pressure_retires_cold_genesis_journal_preserving_authority_observer_funded_default_churn_and_denied_edge()
 {
    observation::funded_default_churn_and_denied_edge();
}
