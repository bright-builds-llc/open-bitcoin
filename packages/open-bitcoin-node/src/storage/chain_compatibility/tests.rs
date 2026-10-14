// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;
use crate::storage::FjallNodeStore;
use open_bitcoin_core::consensus::CanonicalChain;
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

pub(super) const CHAINS: [CanonicalChain; 4] = [
    CanonicalChain::Mainnet,
    CanonicalChain::Testnet3,
    CanonicalChain::Signet,
    CanonicalChain::Regtest,
];

pub(super) struct Fixture(PathBuf);

impl Fixture {
    pub(super) fn new() -> Self {
        static SEQUENCE: AtomicU64 = AtomicU64::new(0);
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let root = std::env::temp_dir()
            .canonicalize()
            .expect("physical temporary directory")
            .join(format!(
                "oc-{}-{}-{}",
                std::process::id(),
                time % 1_000_000,
                SEQUENCE.fetch_add(1, Ordering::Relaxed)
            ));
        fs::create_dir(&root).expect("owned fixture directory");
        Self(root)
    }

    pub(super) fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).expect("remove owned fixture");
    }
}

pub(super) fn ordinary_refusal(path: &Path, chain: CanonicalChain) -> CompatibilityRefusal {
    struct Spy;
    impl Drop for Spy {
        fn drop(&mut self) {
            ORDINARY_BACKEND_OPEN_SPY.set(false);
        }
    }
    ORDINARY_BACKEND_OPEN_SPY.set(true);
    let _spy = Spy;
    FjallNodeStore::open_ordinary(path, chain)
        .err()
        .expect("ordinary admission refused before backend")
}

#[test]
fn chain_compatibility_missing_refuses_without_creating() {
    // Arrange
    let directory = Fixture::new();
    let path = directory.path().join("missing");

    // Act
    let refusal = ordinary_refusal(&path, CanonicalChain::Mainnet);

    // Assert
    assert_eq!(refusal.observation(), DatadirObservation::Missing);
    assert_eq!(
        refusal.reason(),
        CompatibilityReason::UnsupportedValidationScope
    );
    assert!(std::fs::symlink_metadata(path).is_err());
}

#[test]
fn chain_compatibility_empty_directory_stays_empty_for_all_selectors() {
    // Arrange
    let fixture = Fixture::new();
    for chain in CHAINS {
        // Act
        let refusal = ordinary_refusal(fixture.path(), chain);
        // Assert
        assert_eq!(refusal.observation(), DatadirObservation::EmptyDirectory);
        assert_eq!(
            refusal.reason(),
            CompatibilityReason::UnsupportedValidationScope
        );
        assert_eq!(fs::read_dir(fixture.path()).expect("directory").count(), 0);
    }
}

#[test]
fn chain_compatibility_nonempty_and_fake_identity_marker_are_unproved() {
    // Arrange
    let fixture = Fixture::new();
    let marker = fixture.path().join("canonical-mainnet-identity.json");
    fs::write(&marker, b"{\"network\":\"mainnet\",\"trusted\":true}").expect("fake marker");
    for chain in CHAINS {
        // Act
        let refusal = ordinary_refusal(fixture.path(), chain);
        // Assert
        assert_eq!(refusal.reason(), CompatibilityReason::ExistingStateUnproved);
        assert_eq!(
            refusal.observed_identity(),
            ObservedChainIdentity::Unavailable
        );
        assert_eq!(refusal.work_profile(), WorkProfileObservation::Unavailable);
        assert_eq!(
            fs::read(&marker).expect("marker"),
            b"{\"network\":\"mainnet\",\"trusted\":true}"
        );
    }
}

#[test]
fn chain_compatibility_regular_file_and_file_ancestor_are_unsafe() {
    // Arrange
    let fixture = Fixture::new();
    let file = fixture.path().join("file");
    fs::write(&file, b"sentinel").expect("file");
    for path in [file.clone(), file.join("child")] {
        // Act
        let observation = inspect_datadir_without_backend(&path);
        // Assert
        assert_eq!(observation, DatadirObservation::UnsafePath);
        assert_eq!(fs::read(&file).expect("sentinel"), b"sentinel");
    }
}

#[cfg(unix)]
#[test]
fn chain_compatibility_special_file_is_unsafe() {
    // Arrange
    let fixture = Fixture::new();
    let path = fixture.path().join("socket");
    let _socket = std::os::unix::net::UnixListener::bind(&path).expect("socket fixture");
    // Act
    let observation = inspect_datadir_without_backend(&path);
    // Assert
    assert_eq!(observation, DatadirObservation::UnsafePath);
}

#[cfg(unix)]
#[test]
fn chain_compatibility_target_and_ancestor_symlinks_refuse_without_descending() {
    // Arrange
    let fixture = Fixture::new();
    let target = fixture.path().join("real");
    fs::create_dir(&target).expect("target");
    fs::create_dir(target.join("child")).expect("child");
    for (name, destination) in [
        ("absolute", target.as_path()),
        ("relative", Path::new("real")),
        ("broken", Path::new("absent")),
    ] {
        let link = fixture.path().join(name);
        std::os::unix::fs::symlink(destination, &link).expect("link");
        let cwd_depth = std::env::current_dir()
            .expect("cwd")
            .components()
            .filter(|component| matches!(component, std::path::Component::Normal(_)))
            .count();
        let relative_link = PathBuf::from("../".repeat(cwd_depth))
            .join(link.strip_prefix("/").expect("absolute fixture"));
        assert_eq!(
            ordinary_refusal(&relative_link, CanonicalChain::Mainnet).observation(),
            DatadirObservation::UnsafePath
        );
        for path in [
            link.clone(),
            link.join("child"),
            link.join("../real"),
            link.join("."),
            PathBuf::from(format!("{}/", link.display())),
        ] {
            // Act
            let refusal = ordinary_refusal(&path, CanonicalChain::Regtest);
            // Assert
            assert_eq!(refusal.observation(), DatadirObservation::UnsafePath);
            assert_eq!(fs::read_link(&link).expect("unchanged link"), destination);
        }
        let inspected = std::cell::RefCell::new(Vec::new());
        let observation = inspect_with(
            &link.join("child"),
            |path| {
                inspected.borrow_mut().push(path.to_path_buf());
                fs::symlink_metadata(path)
            },
            |_| panic!("must not read through symlink"),
        );
        assert_eq!(observation, DatadirObservation::UnsafePath);
        assert!(!inspected.borrow().contains(&link.join("child")));
    }
}

#[test]
fn chain_compatibility_relative_paths_and_missing_ancestors_are_safe_observations() {
    // Arrange
    let fixture = Fixture::new();
    let missing = fixture.path().join("absent/descendant");
    // Act
    let relative = inspect_datadir_without_backend(Path::new("."));
    let nested_missing = inspect_datadir_without_backend(&missing);
    // Assert
    assert_eq!(relative, DatadirObservation::ExistingUnproved);
    assert_eq!(nested_missing, DatadirObservation::Missing);
    assert_eq!(
        inspect_datadir_without_backend(Path::new("")),
        DatadirObservation::UnsafePath
    );
}

#[test]
fn chain_compatibility_metadata_and_directory_io_errors_remain_distinct_from_absence() {
    // Arrange
    let fixture = Fixture::new();
    for (kind, expected) in [
        (
            io::ErrorKind::PermissionDenied,
            InspectionIoKind::PermissionDenied,
        ),
        (io::ErrorKind::Other, InspectionIoKind::Other),
    ] {
        // Act
        let metadata_failure = inspect_with(
            fixture.path(),
            |_| Err(io::Error::new(kind, "secret metadata")),
            |_| panic!("metadata must stop inspection"),
        );
        let directory_failure = inspect_with(
            fixture.path(),
            |path| fs::symlink_metadata(path),
            |_| Err(io::Error::new(kind, "secret directory")),
        );
        let entry_failure = classify_first_entry(&mut std::iter::once(Err::<(), _>(
            io::Error::new(kind, "secret entry"),
        )));
        // Assert
        for observation in [metadata_failure, directory_failure, entry_failure] {
            assert_eq!(
                observation,
                DatadirObservation::InspectionIoFailure(expected)
            );
            assert_eq!(
                CompatibilityRefusal::from_observation(CanonicalChain::Mainnet, observation)
                    .reason(),
                CompatibilityReason::InspectionIoFailure
            );
        }
    }
}

#[test]
fn chain_compatibility_large_directory_reads_only_first_entry() {
    // Arrange
    let fixture = Fixture::new();
    for index in 0..2048 {
        fs::write(fixture.path().join(index.to_string()), b"private").expect("entry");
    }
    let visits = std::cell::Cell::new(0);
    let mut large_iterator = (0..1_000_000).map(|index| {
        visits.set(visits.get() + 1);
        Ok::<_, io::Error>(index)
    });
    // Act
    let physical = inspect_datadir_without_backend(fixture.path());
    let counted = classify_first_entry(&mut large_iterator);
    // Assert
    assert_eq!(physical, DatadirObservation::ExistingUnproved);
    assert_eq!(counted, DatadirObservation::ExistingUnproved);
    assert_eq!(visits.get(), 1);
    assert_eq!(
        classify_first_entry(&mut std::iter::empty::<io::Result<()>>()),
        DatadirObservation::EmptyDirectory
    );
}

#[test]
fn chain_compatibility_diagnostics_are_bounded_redacted_and_truthful() {
    // Arrange
    let observations = [
        DatadirObservation::Missing,
        DatadirObservation::EmptyDirectory,
        DatadirObservation::ExistingUnproved,
        DatadirObservation::UnsafePath,
        DatadirObservation::InspectionIoFailure(InspectionIoKind::PermissionDenied),
        DatadirObservation::InspectionIoFailure(InspectionIoKind::Other),
    ];
    for chain in CHAINS {
        for observation in observations {
            let refusal = CompatibilityRefusal::from_observation(chain, observation);
            // Act
            let diagnostic = refusal.to_string();
            let serialized = serde_json::to_string(&refusal).expect("serialize bounded refusal");
            // Assert
            assert!(diagnostic.len() <= 1024);
            assert!(serialized.len() <= 1024);
            assert_eq!(refusal.selected_chain(), chain);
            assert_eq!(refusal.selected_genesis_hash(), chain.spec().genesis_hash());
            assert!(!refusal.selected_scope().ordinary_history());
            assert!(diagnostic.contains(&format!("selected_network={chain}")));
            for required in [
                "observed_network=unavailable",
                "observed_root=unavailable",
                "observed_work=unavailable",
                "scope=identity_only",
                "ordinary_history=false",
                "Review existing data",
                "separate datadir",
            ] {
                assert!(diagnostic.contains(required));
            }
            for prerequisite in refusal.missing_prerequisites() {
                assert!(diagnostic.contains(prerequisite.code()));
            }
            if chain == CanonicalChain::Mainnet {
                assert!(
                    diagnostic.contains(
                        "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
                    )
                );
            }
        }
    }
    let hostile = format!("private-wallet-secret-{}\n\u{1b}", "x".repeat(8000));
    let refusal = ordinary_refusal(Path::new(&hostile), CanonicalChain::Mainnet);
    assert!(refusal.to_string().len() <= 1024);
    assert!(!refusal.to_string().contains("private-wallet-secret"));
}

#[test]
fn chain_compatibility_backend_spy_is_live_before_any_backend_effect() {
    // Arrange
    let fixture = Fixture::new();
    let path = fixture.path().join("absent");
    ORDINARY_BACKEND_OPEN_SPY.set(true);
    // Act
    let attempted = std::panic::catch_unwind(|| {
        let _ = FjallNodeStore::open(&path);
    });
    ORDINARY_BACKEND_OPEN_SPY.set(false);
    // Assert
    assert!(attempted.is_err());
    assert_eq!(
        fs::symlink_metadata(&path)
            .expect_err("no raw backend effect")
            .kind(),
        io::ErrorKind::NotFound
    );
}
