// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn named_wallet_registry_and_selection_survive_reopen() {
    // Arrange
    let path = temp_store_path("named-wallet-registry");
    remove_dir_if_exists(&path);
    let alpha = wallet_snapshot();
    let mut beta = wallet_snapshot();
    beta.next_descriptor_id = 99;
    let registry = WalletRegistrySnapshot::new(["alpha".to_string(), "beta".to_string()]);
    let selected = SelectedWalletRecord {
        wallet_name: "beta".to_string(),
    };

    // Act
    {
        let store = FjallNodeStore::open(&path).expect("open store");
        store
            .save_named_wallet_snapshot("alpha", &alpha, PersistMode::Sync)
            .expect("save alpha");
        store
            .save_named_wallet_snapshot("beta", &beta, PersistMode::Sync)
            .expect("save beta");
        store
            .save_wallet_registry(&registry, PersistMode::Sync)
            .expect("save registry");
        store
            .save_selected_wallet(&selected, PersistMode::Sync)
            .expect("save selected wallet");
    }
    let reopened = FjallNodeStore::open(&path).expect("reopen store");

    // Assert
    assert_eq!(
        reopened
            .load_wallet_registry()
            .expect("load registry")
            .expect("registry"),
        registry
    );
    assert_eq!(
        reopened
            .load_selected_wallet()
            .expect("load selected")
            .expect("selected"),
        selected
    );
    assert_eq!(
        reopened
            .load_named_wallet_snapshot("alpha")
            .expect("load alpha"),
        Some(alpha)
    );
    assert_eq!(
        reopened
            .load_named_wallet_snapshot("beta")
            .expect("load beta"),
        Some(beta)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn wallet_rescan_jobs_survive_reopen_with_checkpoint_state() {
    // Arrange
    let path = temp_store_path("wallet-rescan-job");
    remove_dir_if_exists(&path);
    let job = WalletRescanJob {
        wallet_name: "alpha".to_string(),
        target_tip_hash: BlockHash::from_byte_array([8_u8; 32]),
        target_tip_height: 144,
        next_height: 121,
        maybe_scanned_through_height: Some(120),
        maybe_tip_median_time_past: Some(1_700_000_120),
        freshness: WalletRescanFreshness::Partial,
        state: WalletRescanJobState::Scanning,
        maybe_error: None,
    };

    // Act
    {
        let store = FjallNodeStore::open(&path).expect("open store");
        store
            .save_wallet_rescan_job(&job, PersistMode::Sync)
            .expect("save rescan job");
    }
    let reopened = FjallNodeStore::open(&path).expect("reopen store");

    // Assert
    assert_eq!(
        reopened
            .load_wallet_rescan_job("alpha")
            .expect("load job")
            .expect("job"),
        job
    );
    assert_eq!(
        reopened.load_wallet_rescan_jobs().expect("load all jobs"),
        vec![job]
    );

    remove_dir_if_exists(&path);
}
