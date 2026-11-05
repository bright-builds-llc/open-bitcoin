// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txdb.cpp

use super::*;

type Facts = (
    ObservedCoins,
    Option<BasicIndexProgress>,
    Option<ObservedStoredBasic>,
    ObservedNativeRows,
    ObservedProvenance,
);

fn unpack(observation: CanonicalAcceptedStateObservation) -> Facts {
    observation.fold(|coins, progress, stored, rows, provenance| {
        (coins, progress, stored, rows, provenance)
    })
}

fn observe(command: &CanonicalSubmissionCommand, queries: &[BlockHash]) -> Facts {
    unpack(
        command
            .observe_accepted_state(queries)
            .expect("actual owner observation"),
    )
}

fn independent(command: &CanonicalSubmissionCommand, queries: &[BlockHash]) -> Facts {
    command
        .network
        .read(|network| {
            let manager = network.chainstate();
            let mut hashes: BTreeSet<_> = network
                .peer_manager
                .header_store()
                .entries()
                .map(|entry| entry.block_hash)
                .collect();
            hashes.extend(queries.iter().copied());
            (
                (
                    manager
                        .export_chainstate_snapshot()
                        .expect("independent effective state"),
                    manager
                        .chainstate()
                        .coins_best_block()
                        .expect("independent live Coins"),
                    command
                        .store
                        .coins_view()
                        .complete_snapshot()
                        .expect("independent durable Coins"),
                ),
                manager.maybe_basic_index_progress(),
                command
                    .store
                    .maybe_basic_filter_state()
                    .expect("independent stored BASIC")
                    .map(|state| {
                        (
                            state.maybe_endpoint,
                            state.fence_height,
                            state.fence_hash,
                            state.protection,
                        )
                    }),
                native_rows(&command.store),
                hashes
                    .into_iter()
                    .map(|hash| {
                        (
                            hash,
                            (
                                manager
                                    .validation_provenance(hash)
                                    .expect("independent live provenance")
                                    .into(),
                                command
                                    .store
                                    .validation_provenance(hash)
                                    .expect("independent durable provenance")
                                    .into(),
                            ),
                        )
                    })
                    .collect(),
            )
        })
        .expect("actual independent authority")
}

fn stable(facts: &Facts) -> impl fmt::Debug + PartialEq + use<> {
    (
        facts.0.clone(),
        facts.1.map(|progress| {
            (
                progress.generation(),
                progress.accepted_target(),
                progress.maybe_processed_endpoint(),
                progress.maybe_safe_durable_endpoint(),
                progress.protection(),
                progress.initially_synchronized(),
                progress.state(),
            )
        }),
        facts.2,
        facts.3.clone(),
        facts.4.clone(),
    )
}

fn absent() -> BlockHash {
    BlockHash::from_byte_array([0x47; 32])
}

#[test]
fn phase167_submission_observation_fresh_complete_enabled_disabled_and_queries() {
    for mode in [
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        // Arrange
        let (path, runtime, command) = runtime(mode);
        let before = native_rows(&command.store);
        let genesis = block_hash(&CanonicalChain::Regtest.spec().genesis_header());
        // Act
        let facts = observe(&command, &[absent(), absent()]);
        let expected = independent(&command, &[absent()]);
        let after = native_rows(&command.store);
        close(path, runtime, command);
        // Assert
        assert_eq!(facts, expected);
        assert_eq!(after, before);
        assert_eq!(facts.3.len(), 9);
        assert_eq!(facts.4.len(), 2);
        assert!(facts.4.contains_key(&genesis));
        assert_eq!(
            facts.4[&absent()],
            (
                BasicBlockValidationProvenance::NeverConnected,
                BasicBlockValidationProvenance::NeverConnected,
            )
        );
        assert_eq!(facts.1.is_some(), mode == BasicFilterStartupMode::Enabled);
        assert_eq!(facts.2.is_some(), mode == BasicFilterStartupMode::Enabled);
        assert_eq!(facts.0.1, Some(genesis));
        assert_eq!(facts.0.2.maybe_best_block, Some(genesis));
        assert!(facts.0.2.coins.is_empty());
        assert!(facts.0.2.head_blocks.is_empty());
        assert_eq!(
            facts.3.keys().map(String::as_str).collect::<Vec<_>>(),
            [
                "block_index",
                "chainstate",
                "coins",
                "headers",
                "mempool",
                "metrics",
                "runtime",
                "schema",
                "wallet",
            ]
        );
        assert!(facts.3.values().any(BTreeMap::is_empty));
        assert!(facts.3.values().any(|rows| !rows.is_empty()));
    }
}

#[test]
fn phase167_submission_observation_accepted_duplicate_and_intrinsic_refusal() {
    for mode in [
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        // Arrange
        let (path, runtime, command) = runtime(mode);
        let genesis = CanonicalChain::Regtest.spec().genesis_block();
        let block = child(&genesis.header, 1);
        let hash = block_hash(&block.header);
        accepted(
            command
                .submit(wire(&block))
                .expect("genuine accepted child"),
        );
        let before = observe(&command, &[absent()]);
        // Act
        let duplicate = command.submit(wire(&block)).expect("actual duplicate");
        let after_duplicate = observe(&command, &[absent()]);
        let mut bad = wire(&child(&block.header, 2));
        bad.transactions[0].outputs[0].value_sats -= 1;
        reject(&command, bad, "bad-txnmrklroot");
        let after_invalid = observe(&command, &[absent()]);
        let expected = independent(&command, &[absent()]);
        close(path, runtime, command);
        // Assert
        assert!(
            matches!(duplicate, CanonicalSubmissionOutcome::Duplicate(actual) if actual == hash)
        );
        assert_eq!(after_duplicate, before);
        assert_eq!(after_invalid, before);
        assert_eq!(before, expected);
        assert_eq!(
            before.4[&hash],
            (
                BasicBlockValidationProvenance::ScriptsValid,
                BasicBlockValidationProvenance::ScriptsValid,
            )
        );
    }
}

#[test]
fn phase167_submission_observation_contextual_refusal_keeps_real_known_only_delta() {
    for mode in [
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        // Arrange
        let (path, runtime, command) = runtime(mode);
        let mut block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        block.transactions[0].outputs[0].value =
            open_bitcoin_core::primitives::Amount::from_sats(5_000_000_001).expect("valid amount");
        remine(&mut block);
        let hash = block_hash(&block.header);
        let before = observe(&command, &[hash]);
        // Act
        reject(&command, wire(&block), "bad-cb-amount");
        let after = observe(&command, &[hash]);
        let expected = independent(&command, &[hash]);
        let status = command
            .store
            .maybe_validation_history_record(hash)
            .expect("actual record read")
            .expect("real KnownOnly row")
            .status();
        close(path, runtime, command);
        // Assert
        assert_eq!(after, expected);
        assert_eq!(after.0, before.0);
        assert_eq!(after.1, before.1);
        assert_eq!(after.2, before.2);
        assert_ne!(after.3, before.3);
        assert_eq!(after.3["coins"], before.3["coins"]);
        assert_eq!(
            status,
            crate::storage::validation_history::StoredValidationStatus::KnownOnly
        );
        assert_eq!(
            after.4[&hash],
            (
                BasicBlockValidationProvenance::NeverConnected,
                BasicBlockValidationProvenance::NeverConnected,
            )
        );
    }
}

#[test]
fn phase167_submission_observation_projected_rows_refuse_while_original_is_fresh() {
    // Arrange
    let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
    let block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    let (headers, identity) = command
        .network
        .read(|network| {
            let mut headers = network.peer_manager.header_store().clone();
            let inserted = headers
                .insert_canonical_header_detailed(block.header.clone(), 2_000_000_000)
                .expect("real canonical header insertion");
            let identity = crate::storage::validation_history::BlockValidationIdentity::new(
                inserted.block_hash,
                block.header.previous_block_hash,
                inserted.height,
            )
            .expect("real typed identity");
            (headers.entries().cloned().collect::<Vec<_>>(), identity)
        })
        .expect("actual bound header owner");
    let before = native_rows(&command.store);
    let mut inventory = command
        .store
        .inspect_submission_inventory()
        .expect("actual capture");
    // Act
    inventory
        .project_admitted_headers(&headers, identity)
        .expect("genuine projected KnownOnly");
    inventory
        .check_current_source()
        .expect("original source remains fresh");
    let result = inventory.into_observed_rows();
    let after = native_rows(&command.store);
    close(path, runtime, command);
    // Assert
    assert!(result.is_err());
    assert_eq!(after, before);
}

#[test]
fn phase167_submission_observation_tracked_changed_same_value_and_aba_are_stale() {
    for changes in [vec![vec![2]], vec![vec![1]], vec![vec![2], vec![1]]] {
        // Arrange
        let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
        command
            .store
            .write_raw_for_test(
                crate::StorageNamespace::Wallet,
                "wallet_snapshot:observation",
                vec![1],
            )
            .expect("supported baseline write");
        let before = native_rows(&command.store);
        let inventory = command
            .store
            .inspect_submission_inventory()
            .expect("actual capture");
        // Act
        for value in &changes {
            command
                .store
                .write_raw_for_test(
                    crate::StorageNamespace::Wallet,
                    "wallet_snapshot:observation",
                    value.clone(),
                )
                .expect("actual tracked write");
        }
        let after = native_rows(&command.store);
        let result = inventory.into_observed_rows();
        close(path, runtime, command);
        // Assert: same bytes cannot hide a changed logical source.
        assert!(result.is_err());
        assert_eq!(after == before, changes.last() == Some(&vec![1]));
    }
}

#[test]
fn phase167_submission_observation_foreign_store_params_and_incarnation_refuse() {
    for case in 0..3 {
        // Arrange
        let (path, runtime, mut command) = runtime(BasicFilterStartupMode::Disabled);
        let other_path = directory();
        let (other_runtime, maybe_other) =
            DurableSyncRuntime::open_canonical_submission_configured(
                &other_path,
                config(),
                BasicFilterStartupMode::Disabled,
            )
            .expect("second real factory");
        let other = maybe_other.expect("second earned command");
        let original = native_rows(runtime.store());
        let foreign = native_rows(other_runtime.store());
        if case == 0 {
            command.store = other.store.clone();
        }
        if case == 1 {
            command.params.coinbase_maturity += 1;
        }
        if case == 2 {
            command.incarnation = other.incarnation;
        }
        // Act
        let result = command.observe_accepted_state(&[]);
        let original_after = native_rows(runtime.store());
        let foreign_after = native_rows(other_runtime.store());
        close(path, runtime, command);
        close(other_path, other_runtime, other);
        // Assert
        assert!(result.is_err());
        assert_eq!(original_after, original);
        assert_eq!(foreign_after, foreign);
    }
}

#[test]
fn phase167_submission_observation_post_absorption_error_reports_state_or_real_refusal() {
    use crate::storage::fjall_store::validation_history::HistoryPublicationFault;
    for mode in [
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        for fault in [
            HistoryPublicationFault::BeforeCommit,
            HistoryPublicationFault::AfterCommit,
        ] {
            // Arrange
            let (path, runtime, command) = runtime(mode);
            let block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
            let hash = block_hash(&block.header);
            ABSORB_FAULT.set(Some(fault));
            // Act
            let primary = command
                .submit(wire(&block))
                .expect_err("real publication failure");
            let before = native_rows(&command.store);
            let observed = command.observe_accepted_state(&[hash]);
            let after = native_rows(&command.store);
            let tip = command
                .network
                .maybe_chain_tip()
                .expect("real advanced manager")
                .expect("tip");
            // Assert: an operational failure did not roll back actual absorption.
            assert_eq!(primary.stage, "acceptance/publication");
            assert_eq!(tip.height, 1);
            assert_eq!(tip.block_hash, hash);
            assert_eq!(after, before);
            match observed {
                Ok(capsule) => {
                    assert_eq!(fault, HistoryPublicationFault::BeforeCommit);
                    let facts = unpack(capsule);
                    assert_eq!(facts, independent(&command, &[hash]));
                    assert_eq!(facts.0.0.tip().expect("observed live tip").height, 1);
                    assert_eq!(
                        facts.4[&hash].0,
                        BasicBlockValidationProvenance::ScriptsValid
                    );
                    if fault == HistoryPublicationFault::BeforeCommit {
                        assert_eq!(
                            facts.4[&hash].1,
                            BasicBlockValidationProvenance::NeverConnected
                        );
                    }
                }
                Err(error) => {
                    assert_eq!(fault, HistoryPublicationFault::AfterCommit);
                    assert!(matches!(
                        error.stage,
                        "observation live provenance" | "observation durable provenance"
                    ));
                    assert!(
                        std::error::Error::source(&error).is_some(),
                        "real operational source"
                    );
                    eprintln!("actual post-absorption observation refusal: {error:?}");
                }
            }
            drop(primary);
            close(path, runtime, command);
        }
    }
}

#[test]
fn phase167_submission_observation_full_retained_306_allows_absent_queries_612_union() {
    // Arrange
    let (path, runtime, command) = runtime(BasicFilterStartupMode::Enabled);
    let mut parent = CanonicalChain::Regtest.spec().genesis_header();
    let blocks: Vec<_> = (1..306)
        .map(|height| {
            let block = child(&parent, height);
            parent = block.header.clone();
            block
        })
        .collect();
    admit_only(&command, &blocks);
    let known: BTreeSet<_> = command
        .network
        .header_entries()
        .expect("actual retained headers")
        .into_iter()
        .map(|entry| entry.block_hash)
        .collect();
    assert_eq!(known.len(), 306);
    let mut queries = Vec::new();
    for index in 0_u32.. {
        let mut bytes = [0x47; 32];
        bytes[..4].copy_from_slice(&index.to_le_bytes());
        let hash = BlockHash::from_byte_array(bytes);
        if !known.contains(&hash) {
            queries.push(hash);
        }
        if queries.len() == 306 {
            break;
        }
    }
    let before = native_rows(&command.store);
    // Act
    let facts = observe(&command, &queries);
    let expected = independent(&command, &queries);
    let over = command.observe_accepted_state(&vec![absent(); 307]);
    let after = native_rows(&command.store);
    close(path, runtime, command);
    // Assert
    assert_eq!(facts, expected);
    assert_eq!(facts.4.len(), 612);
    assert!(known.iter().all(|hash| facts.4.contains_key(hash)));
    assert!(queries.iter().all(|hash| facts.4[hash]
        == (
            BasicBlockValidationProvenance::NeverConnected,
            BasicBlockValidationProvenance::NeverConnected,
        )));
    assert_eq!(
        over.expect_err("actual one-over query refusal").stage,
        "observation query bound exceeded"
    );
    assert_eq!(before, after);
}

#[test]
fn phase167_submission_observation_fold_is_unlocked_detached_and_reopens_exact_state() {
    for mode in [
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        // Arrange
        let (path, runtime, command) = runtime(mode);
        let block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        let hash = block_hash(&block.header);
        let genesis = block_hash(&CanonicalChain::Regtest.spec().genesis_header());
        accepted(command.submit(wire(&block)).expect("actual accepted child"));
        let incarnation = command.incarnation;
        // Act: actual try-locks and reentry happen inside the consuming fold.
        let before = command
            .observe_accepted_state(&[absent()])
            .expect("owned observation")
            .fold(|coins, progress, stored, rows, provenance| {
                drop(
                    command
                        .operation
                        .try_lock()
                        .expect("operation guard released"),
                );
                drop(
                    command
                        .network
                        .authority
                        .try_lock()
                        .expect("authority guard released"),
                );
                let facts = (coins, progress, stored, rows, provenance);
                assert_eq!(facts, observe(&command, &[absent()]));
                facts
            });
        let (runtime, command) = reopen(&path, runtime, command, mode);
        let after = observe(&command, &[absent()]);
        let expected = independent(&command, &[absent()]);
        let next_incarnation = command.incarnation;
        close(path, runtime, command);
        // Assert: data retained across ALL drops owns no database/guard/lease.
        assert_ne!(incarnation, next_incarnation);
        assert_eq!(after, expected);
        assert_eq!(stable(&after), stable(&before));
        if let Some(progress) = before.1 {
            assert_eq!(progress.branch_identity(), genesis);
        }
        if let Some(progress) = after.1 {
            assert_eq!(progress.branch_identity(), hash);
        }
    }
}
