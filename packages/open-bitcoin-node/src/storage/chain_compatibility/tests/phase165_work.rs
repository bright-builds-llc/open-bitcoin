// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

// Reuse the original closed-manifest assertions and controls without editing their source.
// The original tests are also discovered here; these are duplicate controls, not new proofs.
include!("preservation.rs");

fn assert_work_refusal(source: &Path) {
    let before = physical_manifest(source);
    assert_physical_preservation(source);
    for chain in CHAINS {
        let calls = BACKEND_BUILDER_CALLS.get();
        let refusal = ordinary_refusal(source, chain);
        assert_eq!(
            refusal.work_decision(),
            WorkCompatibilityDecision::ExistingUnproved
        );
        assert_eq!(refusal.work_profile(), WorkProfileObservation::Unavailable);
        assert_eq!(BACKEND_BUILDER_CALLS.get(), calls);
        let diagnostic = refusal.to_string();
        assert!(diagnostic.len() < 1024);
        assert!(diagnostic.ends_with("work_decision=existing_unproved"));
        assert!(!diagnostic.contains("private-wallet-secret"));
        assert_eq!(physical_manifest(source), before);
    }
}

fn clone_closed(source: &Path, clone: &Path) {
    for (relative, entry) in physical_manifest(source) {
        let destination = clone.join(relative);
        match entry.kind {
            "directory" => fs::create_dir(&destination).expect("owned clone directory"),
            "file" => fs::write(&destination, entry.bytes).expect("owned clone bytes"),
            #[cfg(unix)]
            "symlink" => {
                std::os::unix::fs::symlink(entry.maybe_link.expect("link target"), &destination)
                    .expect("owned clone link")
            }
            _ => panic!("unsupported clone entry"),
        }
    }
}

#[test]
fn phase165_work_real_builder_probe_stops_before_effect_and_counts_actual_raw_open() {
    // Arrange
    let fixture = Fixture::new();
    let denied = fixture.path().join("denied");
    let before = BACKEND_BUILDER_CALLS.get();
    ORDINARY_BACKEND_OPEN_SPY.set(true);
    // Act
    let blocked = std::panic::catch_unwind(|| FjallNodeStore::open(&denied));
    ORDINARY_BACKEND_OPEN_SPY.set(false);
    // Assert
    assert!(blocked.is_err());
    assert_eq!(BACKEND_BUILDER_CALLS.get(), before + 1);
    assert!(!denied.exists());
    let allowed = fixture.path().join("allowed");
    let raw = FjallNodeStore::open(&allowed).expect("independent owned raw positive");
    assert_eq!(BACKEND_BUILDER_CALLS.get(), before + 2);
    assert!(allowed.is_dir());
    drop(raw);
}

#[test]
fn phase165_work_forged_profiles_and_legacy_exact_mixed_records_preserve_closed_sources() {
    for kind in [
        "legacy-u128",
        "tagged-legacy",
        "raw-exact",
        "mixed",
        "malformed",
    ] {
        // Arrange
        let fixture = Fixture::new();
        let source = fixture.path().join(kind);
        arrange_schema_one(&source);
        {
            let store = FjallNodeStore::open_without_ensure_schema_for_test(&source)
                .expect("owned unmodified schema-one reader");
            let snapshot = store
                .load_chainstate_snapshot()
                .expect("exact fixture snapshot")
                .expect("snapshot present");
            let bytes = crate::storage::snapshot_codec::encode_chainstate_snapshot(&snapshot)
                .expect("fixture encoding");
            let mut value: serde_json::Value =
                serde_json::from_slice(&bytes).expect("fixture JSON");
            if kind == "legacy-u128" || kind == "tagged-legacy" {
                value["schema_version"] = serde_json::json!(1);
            }
            let legacy = serde_json::Value::String("LEGACY_U128_MAX".to_owned());
            match kind {
                "legacy-u128" => value["payload"]["active_chain"][0]["chain_work"] = legacy,
                "tagged-legacy" => {
                    value["payload"]["active_chain"][0]["chain_work"] = serde_json::json!({
                        "version": 1, "kind": "legacy_synthetic", "score": legacy,
                    })
                }
                "mixed" => {
                    let mut legacy_position = value["payload"]["active_chain"][0].clone();
                    legacy_position["chain_work"] = legacy;
                    value["payload"]["active_chain"]
                        .as_array_mut()
                        .expect("positions")
                        .push(legacy_position);
                }
                "malformed" => {
                    value["payload"]["active_chain"][0]["chain_work"] = serde_json::json!({
                        "version": 1, "kind": "exact_number", "bytes_le": [255], "score": 17,
                    })
                }
                "raw-exact" => {}
                _ => panic!("unknown fixture kind"),
            }
            let record = serde_json::to_string(&value)
                .expect("fixture JSON")
                .replace("\"LEGACY_U128_MAX\"", &u128::MAX.to_string())
                .into_bytes();
            store
                .write_raw_for_test(StorageNamespace::Chainstate, "snapshot", record)
                .expect("untrusted source record");
            if kind != "legacy-u128" && kind != "tagged-legacy" {
                store
                    .write_schema_version_for_test(2)
                    .expect("schema-two source");
            }
            fs::write(
                source.join("exact-work-profile.json"),
                b"{\"trusted\":true,\"kind\":\"exact_number\",\"score\":999999999}",
            )
            .expect("forged exact marker");
            fs::write(
                source.join("legacy-work-profile.json"),
                b"{\"kind\":\"legacy_synthetic\",\"accepted\":true}",
            )
            .expect("forged legacy marker");
            fs::write(
                source.join("bitcoin.conf"),
                b"rpcpassword=private-wallet-secret\n",
            )
            .expect("private configuration");
            fs::write(source.join("wallet.dat"), b"private-wallet-secret\0\xff")
                .expect("unrelated wallet");
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(source.join("wallet.dat"), fs::Permissions::from_mode(0o640))
                    .expect("private wallet mode");
                fs::set_permissions(
                    source.join("bitcoin.conf"),
                    fs::Permissions::from_mode(0o600),
                )
                .expect("private configuration mode");
            }
        }
        // Act / Assert: no raw opener or migration is used on this closed source.
        assert_work_refusal(&source);
    }
}

#[test]
fn phase165_work_pending_coins_recovery_prune_and_basic_rows_preserve_together() {
    // Arrange
    let fixture = Fixture::new();
    let source = fixture.path().join("pending");
    arrange_pending_prune(&source);
    {
        let store = FjallNodeStore::open_without_ensure_schema_for_test(&source)
            .expect("owned raw pending fixture");
        let heads = coins_codec::encode_head_blocks_value(&[
            BlockHash::from_byte_array([0x11; 32]),
            BlockHash::from_byte_array([0x22; 32]),
        ])
        .expect("pending paired heads");
        store
            .coins_view()
            .write_raw_bytes(&coins_codec::encode_head_blocks_key(), heads)
            .expect("pending H beside B");
        store
            .write_raw_for_test(StorageNamespace::Runtime, "recovery_marker", vec![0xff, 0])
            .expect("recovery marker");
        store
            .write_raw_for_test(
                StorageNamespace::BlockIndex,
                filter_index::STATE_KEY,
                vec![0xff],
            )
            .expect("malformed BASIC state");
        store
            .write_raw_for_test(
                StorageNamespace::BlockIndex,
                &filter_index::record_key(BlockHash::from_byte_array([0x33; 32])),
                vec![0xff, 1],
            )
            .expect("malformed BASIC row");
    }
    // Act / Assert
    assert_work_refusal(&source);
}

#[test]
fn phase165_work_owned_clone_really_migrates_schema_one_coins_without_work_conversion() {
    // Arrange: schema one is the coins layout; this control already has exact untrusted
    // fixture numbers. Legacy-u128 sources above are never converted for a positive.
    let fixture = Fixture::new();
    let source = fixture.path().join("source");
    let clone = fixture.path().join("clone");
    let (tip, outpoint) = arrange_schema_one(&source);
    assert_work_refusal(&source);
    let before = physical_manifest(&source);
    clone_closed(&source, &clone);
    let clone_before = physical_manifest(&clone);
    let calls = BACKEND_BUILDER_CALLS.get();
    // Act
    let migrated = FjallNodeStore::open(&clone).expect("actual clone schema migration");
    let view = migrated.coins_view();
    // Assert
    assert_eq!(BACKEND_BUILDER_CALLS.get(), calls + 1);
    assert_eq!(view.best_block().expect("migrated B"), Some(tip));
    assert!(
        view.contains_raw_key(&coins_codec::encode_coin_key(&outpoint))
            .expect("migrated C")
    );
    let snapshot = migrated
        .load_chainstate_snapshot()
        .expect("unchanged numeric record")
        .expect("original leftover");
    assert_eq!(
        snapshot.active_chain[0].chain_work,
        ChainPosition::untrusted_fixture_work(1)
    );
    drop(view);
    drop(migrated);
    assert_ne!(physical_manifest(&clone), clone_before);
    assert_eq!(physical_manifest(&source), before);
}

#[test]
fn phase165_work_owned_clone_really_deletes_pending_prune_block_and_undo() {
    // Arrange
    let fixture = Fixture::new();
    let source = fixture.path().join("source");
    let clone = fixture.path().join("clone");
    let hash = arrange_pending_prune(&source);
    assert_work_refusal(&source);
    let before = physical_manifest(&source);
    clone_closed(&source, &clone);
    let clone_before = physical_manifest(&clone);
    let calls = BACKEND_BUILDER_CALLS.get();
    let store = FjallNodeStore::open(&clone).expect("owned clone raw reopen");
    assert_eq!(BACKEND_BUILDER_CALLS.get(), calls + 1);
    assert!(store.has_block(hash).expect("body before"));
    assert!(store.has_undo(hash).expect("undo before"));
    // Act
    crate::storage::fjall_store::resume_prune_intent(&store, &[]).expect("actual prune resume");
    // Assert
    assert!(!store.has_block(hash).expect("body deleted"));
    assert!(!store.has_undo(hash).expect("undo deleted"));
    assert_eq!(store.maybe_prune_intent().expect("intent cleared"), None);
    assert!(store.load_have_pruned().expect("prune recorded"));
    drop(store);
    assert_ne!(physical_manifest(&clone), clone_before);
    assert_eq!(physical_manifest(&source), before);
}

#[test]
fn phase165_work_unsafe_paths_and_injected_io_keep_policy_and_closed_manifest() {
    // Arrange
    let fixture = Fixture::new();
    let source = fixture.path().join("source");
    arrange_schema_one(&source);
    #[cfg(unix)]
    std::os::unix::fs::symlink("source", fixture.path().join("alias")).expect("source alias");
    let before = physical_manifest(fixture.path());
    for chain in CHAINS {
        let mut candidates = vec![source.join("unrelated-private-file/child")];
        #[cfg(unix)]
        candidates.extend(
            ["alias/.", "alias/child", "alias/../source", "alias/"]
                .map(|suffix| fixture.path().join(suffix)),
        );
        for candidate in candidates {
            // Act
            let refusal = ordinary_refusal(&candidate, chain);
            // Assert
            assert_eq!(
                refusal.work_decision(),
                WorkCompatibilityDecision::UnsafePath
            );
            assert_eq!(physical_manifest(fixture.path()), before);
        }
        for kind in [io::ErrorKind::PermissionDenied, io::ErrorKind::Other] {
            let calls = BACKEND_BUILDER_CALLS.get();
            // Act
            let observation = inspect_with(
                &source,
                |path| fs::symlink_metadata(path),
                |_| Err(io::Error::new(kind, "private-wallet-secret\n\u{1b}")),
            );
            let refusal = CompatibilityRefusal::from_observation(chain, observation);
            // Assert
            assert_eq!(
                refusal.work_decision(),
                WorkCompatibilityDecision::InspectionIoFailure
            );
            assert!(!refusal.to_string().contains("private-wallet-secret"));
            assert!(refusal.to_string().len() < 1024);
            assert_eq!(BACKEND_BUILDER_CALLS.get(), calls);
            assert_eq!(physical_manifest(fixture.path()), before);
        }
    }
}
