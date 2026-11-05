// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/rpc/mining.cpp

use super::*;

#[test]
fn phase167_local_submission_reopen_bound_physical_maintenance_preserves_logical_source() {
    // Arrange: this is the real existing store, with no second engine or new namespace.
    let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
    let database = command.store.database();
    let catalog = || {
        database
            .list_keyspace_names()
            .into_iter()
            .map(|name| name.to_string())
            .collect::<std::collections::BTreeSet<_>>()
    };
    let before_catalog = catalog();
    assert_eq!(before_catalog.len(), 9);
    assert!(before_catalog.contains("block_index"));
    let keyspace = database
        .keyspace("block_index", fjall::KeyspaceCreateOptions::default)
        .expect("already existing keyspace");
    let before_rows = native_rows(&command.store);
    let observed = command
        .store
        .inspect_submission_inventory()
        .expect("genuine actual observation");
    let before_sequence = database.visible_seqno();
    // Act: force actual physical maintenance without changing a logical row.
    keyspace
        .rotate_memtable_and_wait()
        .expect("actual physical flush");
    let after_sequence = database.visible_seqno();
    // Assert: physical versions cannot invalidate unchanged logical facts.
    assert!(after_sequence > before_sequence);
    assert_eq!(catalog(), before_catalog);
    assert_eq!(native_rows(&command.store), before_rows);
    eprintln!(
        "PHYSICAL_INVENTORY before_seq={before_sequence} after_seq={after_sequence} catalog_equal=true all9_maps_equal=true"
    );
    let result = observed.check_current_source();
    assert!(
        result.is_ok(),
        "unchanged actual logical source after physical maintenance: {result:?}"
    );
    drop(observed);
    drop(keyspace);
    close(path, runtime, command);
}

#[test]
fn phase167_local_submission_reopen_bound_enabled_known_only_count_and_novel_last_slot() {
    // Arrange: retained height and accepted height are deliberately different.
    let mode = BasicFilterStartupMode::Enabled;
    let (path, runtime, command) = runtime(mode);
    let genesis = CanonicalChain::Regtest.spec().genesis_block();
    let mut parent = genesis.header.clone();
    let fork: Vec<_> = (1..=304)
        .map(|height| {
            let mut block = child(&parent, height);
            if height == 1 {
                block.header.time += 100;
                remine(&mut block);
            }
            parent = block.header.clone();
            block
        })
        .collect();
    admit_only(&command, &fork);
    assert_eq!(
        command
            .network
            .header_entries()
            .expect("actual retained")
            .len(),
        305
    );
    assert_eq!(
        command
            .network
            .maybe_chain_tip()
            .expect("actual tip")
            .expect("root")
            .height,
        0
    );
    let block = child(&genesis.header, 1);
    // Act: actual available body preference, not header height, earns acceptance.
    accepted(
        command
            .submit(wire(&block))
            .expect("novel305 to306 actual acceptance"),
    );
    let hash = block_hash(&block.header);
    let root = block_hash(&genesis.header);
    assert_eq!(
        command
            .network
            .header_entries()
            .expect("actual total")
            .len(),
        306
    );
    assert_eq!(maybe_basic_anchor(&command), Some(root));
    let before = stable_state(&command);
    let headers = command
        .network
        .header_entries()
        .expect("complete held headers");
    let incarnation = command.incarnation;
    let rows = native_rows(&command.store);
    let records = [root, hash].map(|hash| {
        command
            .store
            .load_basic_filter_record(hash)
            .expect("full actual BASIC")
    });
    let physical = manifest(&path);
    let mut side = child(&genesis.header, 1);
    side.header.time += 200;
    remine(&mut side);
    // Assert: neither a novel main child nor a novel side identity gets effects.
    for candidate in [child(&block.header, 2), side] {
        let error = command
            .submit(wire(&candidate))
            .expect_err("full actual identity graph");
        assert_eq!(error.stage, "canonical history reopen capacity unavailable");
        assert_eq!(stable_state(&command), before);
        assert_eq!(
            command
                .network
                .header_entries()
                .expect("unchanged volatile headers"),
            headers
        );
        assert_eq!(native_rows(&command.store), rows);
        assert_eq!(manifest(&path), physical);
        assert!(
            command
                .network
                .maybe_body_eligibility(block_hash(&candidate.header))
                .expect("no body")
                .is_none()
        );
    }
    let (runtime, command) = reopen(&path, runtime, command, mode);
    assert_ne!(command.incarnation, incarnation);
    assert_eq!(stable_state(&command), before);
    assert_eq!(
        command
            .network
            .header_entries()
            .expect("fully recovered headers"),
        headers
    );
    assert_eq!(maybe_basic_anchor(&command), Some(hash));
    assert_eq!(native_rows(&command.store), rows);
    assert_eq!(
        [root, hash].map(|hash| command
            .store
            .load_basic_filter_record(hash)
            .expect("full reopened BASIC")),
        records
    );
    for duplicate in [&genesis, &block] {
        assert!(
            matches!(command.submit(wire(duplicate)).expect("actual duplicate continuation"), CanonicalSubmissionOutcome::Duplicate(hash) if hash == block_hash(&duplicate.header))
        );
    }
    assert!(command.submit(wire(&child(&block.header, 2))).is_err());
    close(path, runtime, command);
}

#[test]
fn phase167_local_submission_reopen_bound_post_basic_store_change_reopens_advanced_state() {
    for mode in [
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        // Arrange
        let (path, runtime, command) = runtime(mode);
        let genesis = block_hash(&CanonicalChain::Regtest.spec().genesis_header());
        let block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        CHANGE_AFTER_BASIC.set(true);
        // Act
        let error = command
            .submit(wire(&block))
            .expect_err("actual achieved inventory freshness");
        // Assert: absorption and flush really happened; an operational error is no rollback.
        assert_eq!(error.stage, "accepted achieved inventory");
        let hash = block_hash(&block.header);
        assert_eq!(
            command
                .network
                .maybe_chain_tip()
                .expect("tip")
                .expect("accepted")
                .block_hash,
            hash
        );
        assert_eq!(
            command
                .store
                .validation_provenance(hash)
                .expect("actual receipt"),
            crate::storage::validation_history::ValidationProvenance::ScriptsValid
        );
        assert_eq!(
            command.store.load_block(hash).expect("actual body"),
            Some(block.clone())
        );
        assert!(
            command
                .store
                .load_undo(hash)
                .expect("actual undo")
                .is_some()
        );
        let before = stable_state(&command);
        let incarnation = command.incarnation;
        assert_eq!(
            maybe_basic_anchor(&command),
            (mode == BasicFilterStartupMode::Enabled).then_some(genesis)
        );
        let rows = native_rows(&command.store);
        assert_eq!(rows["runtime"].get(b"snapshot".as_slice()), Some(&vec![1]));
        let records = [genesis, hash].map(|hash| {
            command
                .store
                .load_basic_filter_record(hash)
                .expect("full actual BASIC")
        });
        drop(error);
        let (runtime, command) = reopen(&path, runtime, command, mode);
        assert_ne!(command.incarnation, incarnation);
        assert_eq!(stable_state(&command), before);
        assert_eq!(
            maybe_basic_anchor(&command),
            (mode == BasicFilterStartupMode::Enabled).then_some(hash)
        );
        assert_eq!(native_rows(&command.store), rows);
        assert_eq!(
            [genesis, hash].map(|hash| command
                .store
                .load_basic_filter_record(hash)
                .expect("full reopened BASIC")),
            records
        );
        accepted(
            command
                .submit(wire(&child(&block.header, 2)))
                .expect("valid next actual acceptance"),
        );
        close(path, runtime, command);
    }
}

#[test]
fn phase167_local_submission_reopen_bound_novel_main_before_every_effect() {
    // Arrange: actual factory and actual command produce every accepted identity.
    let path = directory();
    let (runtime, command) = DurableSyncRuntime::open_canonical_submission_configured(
        &path,
        config(),
        BasicFilterStartupMode::Disabled,
    )
    .expect("safe factory");
    let command = command.expect("command");
    let mut parent = CanonicalChain::Regtest.spec().genesis_header();
    for height in 1..=305 {
        let block = child(&parent, height);
        accepted(
            command
                .submit(wire(&block))
                .expect("actual complete prefix"),
        );
        parent = block.header;
    }
    assert_eq!(FjallNodeStore::canonical_history_reopen_limit(), 306);
    let block = child(&parent, 306);
    let before = accepted_state(&command);
    let known = command.network.header_entries().expect("all retained");
    assert_eq!(known.len(), 306);
    let bytes = manifest(&path);
    // Act
    let result = command.submit(wire(&block));
    // Assert: retain concrete unguarded RED and its actual safe-reopen refusal.
    if let Ok(CanonicalSubmissionOutcome::Accepted(position)) = &result {
        eprintln!(
            "UNGUARDED_BOUND accepted_height={} retained={}",
            position.height,
            command
                .network
                .header_entries()
                .expect("actual known")
                .len()
        );
        drop(command);
        drop(runtime);
        assert!(
            DurableSyncRuntime::open_canonical_submission_configured(
                &path,
                config(),
                BasicFilterStartupMode::Disabled
            )
            .is_err()
        );
        std::fs::remove_dir_all(path).expect("all owners released after refused reopen");
        panic!("unguarded command actually accepted307 retained identities");
    }
    assert!(result.is_err());
    assert_eq!(accepted_state(&command), before);
    assert_eq!(
        command.network.header_entries().expect("unchanged known"),
        known
    );
    assert_eq!(manifest(&path), bytes);
    drop(command);
    drop(runtime);
    let (runtime, command) = DurableSyncRuntime::open_canonical_submission_configured(
        &path,
        config(),
        BasicFilterStartupMode::Disabled,
    )
    .expect("still safely reopens306");
    assert_eq!(accepted_state(command.as_ref().expect("command")), before);
    close(path, runtime, command);
}

#[test]
fn phase167_local_submission_reopen_bound_existing_members_and_stale_receipts() {
    // Arrange: every accepted fork receipt comes from this actual authority.
    let mode = BasicFilterStartupMode::Disabled;
    let (path, runtime, command) = runtime(mode);
    let genesis = CanonicalChain::Regtest.spec().genesis_block();
    let active = child(&genesis.header, 1);
    accepted(
        command
            .submit(wire(&active))
            .expect("original active acceptance"),
    );
    let mut first = child(&genesis.header, 1);
    first.header.time += 100;
    remine(&mut first);
    let second = child(&first.header, 2);
    for block in [&first, &second] {
        assert!(
            command.submit(wire(block)).is_err(),
            "known side bodies are not accepted"
        );
    }
    let entries = command
        .network
        .header_entries()
        .expect("actual admitted work");
    let replacements =
        [&first, &second].map(|block| open_bitcoin_core::chainstate::AnchoredBlock {
            block: block.clone(),
            chain_work: entries
                .iter()
                .find(|entry| entry.block_hash == block_hash(&block.header))
                .expect("earned work")
                .chain_work,
        });
    let flags = command
        .network
        .read(|network| {
            network
                .peer_manager
                .header_store()
                .canonical_header_history(block_hash(&second.header))
                .expect("actual branch history")
                .rules()
                .script_flags()
        })
        .expect("held authority");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("shell clock")
        .as_secs();
    // The direct reorg adapter consumes acceptance; its caller retains the real bodies.
    for body in [&first, &second] {
        command
            .store
            .save_block(body, crate::PersistMode::Sync)
            .expect("durable actual replacement body");
    }
    command
        .store
        .inspect_submission_inventory()
        .expect("supported actual replacement source");
    command
        .network
        .reorg_to_branch(
            std::slice::from_ref(&active),
            &replacements,
            open_bitcoin_mempool::ReorgLifecycleContext::new(
                open_bitcoin_mempool::PolicyTime::from_unix_seconds(
                    i64::try_from(now).expect("clock range"),
                ),
            ),
            flags,
            command.params,
        )
        .expect("genuine preferred branch acceptance");
    command
        .network
        .flush_coins(
            FlushMode::Always,
            FlushPolicyTime::from_unix_seconds(now),
            crate::chainstate::probe_disk_free_bytes(&path),
        )
        .expect("actual full flush with disk probe");
    for replacement in &replacements {
        let hash = block_hash(&replacement.block.header);
        assert_eq!(
            command
                .store
                .load_block(hash)
                .expect("actual reorg body source"),
            Some(replacement.block.clone()),
            "actual accepted reorg requires durable body retention"
        );
    }
    let mut failed = child(&second.header, 3);
    failed.transactions[0].outputs[0].value =
        open_bitcoin_core::primitives::Amount::from_sats(5_000_000_001).expect("invalid reward");
    remine(&mut failed);
    reject(&command, wire(&failed), "bad-cb-amount");
    let mut parent = second.header.clone();
    let retained: Vec<_> = (3..=303)
        .map(|height| {
            let block = child(&parent, height);
            parent = block.header.clone();
            block
        })
        .collect();
    admit_only(&command, &retained);
    assert_eq!(
        command
            .network
            .header_entries()
            .expect("all categories")
            .len(),
        306
    );
    let member = &retained[0];
    assert_eq!(
        command
            .store
            .maybe_validation_history_record(block_hash(&member.header))
            .expect("record")
            .expect("genuine known")
            .status(),
        crate::storage::validation_history::StoredValidationStatus::KnownOnly
    );
    // Act: a known identity can still earn acceptance at the complete bound.
    accepted(
        command
            .submit(wire(member))
            .expect("existing KnownOnly acceptance at306"),
    );
    reject(&command, wire(&failed), "duplicate-invalid");
    let mut invalid_member = retained[1].clone();
    invalid_member.transactions[0].inputs[0].witness =
        open_bitcoin_core::primitives::ScriptWitness::new(vec![vec![1; 31]]);
    reject(&command, wire(&invalid_member), "unexpected-witness");
    for block in [&genesis, &active, &first, &second, member] {
        assert!(
            matches!(command.submit(wire(block)).expect("actual accepted duplicate"), CanonicalSubmissionOutcome::Duplicate(hash) if hash == block_hash(&block.header))
        );
    }
    let mut witness_only = member.clone();
    witness_only.transactions[0].inputs[0].witness =
        open_bitcoin_core::primitives::ScriptWitness::new(vec![vec![7; 32]]);
    assert!(matches!(
        command
            .submit(wire(&witness_only))
            .expect("accepted witness-only duplicate"),
        CanonicalSubmissionOutcome::Duplicate(_)
    ));
    let mut main = child(&member.header, 4);
    main.header.time += 200;
    remine(&mut main);
    let mut side = child(&genesis.header, 1);
    side.header.time += 300;
    remine(&mut side);
    let before = accepted_state(&command);
    let known = command
        .network
        .header_entries()
        .expect("complete held identities");
    let rows = native_rows(&command.store);
    let physical = manifest(&path);
    // Assert: novel main and side candidates have no durable or volatile effects.
    for block in [main, side] {
        let mut invalid = wire(&block);
        invalid.transactions[0].outputs[0].value_sats = -1;
        reject(&command, invalid, "bad-txnmrklroot");
        let error = command
            .submit(wire(&block))
            .expect_err("novel identity capacity");
        assert_eq!(error.stage, "canonical history reopen capacity unavailable");
        assert_eq!(accepted_state(&command), before);
        assert_eq!(
            command.network.header_entries().expect("all identities"),
            known
        );
        assert_eq!(native_rows(&command.store), rows);
        assert_eq!(manifest(&path), physical);
        assert!(
            command
                .network
                .maybe_body_eligibility(block_hash(&block.header))
                .expect("no body effects")
                .is_none()
        );
    }
    let stable = stable_state(&command);
    let (runtime, command) = reopen(&path, runtime, command, mode);
    assert_eq!(stable_state(&command), stable);
    assert_eq!(native_rows(&command.store), rows);
    accepted(
        command
            .submit(wire(&retained[1]))
            .expect("valid next known identity after full reproof"),
    );
    close(path, runtime, command);
}

#[test]
fn phase167_local_submission_reopen_bound_post_basic_external_advance_is_operational() {
    for mode in [
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        // Arrange
        let (path, runtime, command) = runtime(mode);
        let block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        ADVANCE_AFTER_BASIC.set(true);
        // Act
        let error = command
            .submit(wire(&block))
            .expect_err("actual post-BASIC bound recheck");
        // Assert: external direct authority is not frozen by the command mutex.
        assert_eq!(error.stage, "canonical history reopen capacity unavailable");
        assert_eq!(
            command
                .network
                .header_entries()
                .expect("actual total")
                .len(),
            307
        );
        assert_eq!(
            command
                .network
                .maybe_chain_tip()
                .expect("accepted tip")
                .expect("tip")
                .block_hash,
            block_hash(&block.header)
        );
        assert_eq!(
            command
                .store
                .validation_provenance(block_hash(&block.header))
                .expect("actual advanced receipt"),
            crate::storage::validation_history::ValidationProvenance::ScriptsValid
        );
        assert!(
            command
                .store
                .load_undo(block_hash(&block.header))
                .expect("actual flushed undo")
                .is_some()
        );
        assert_eq!(
            command
                .store
                .load_block(block_hash(&block.header))
                .expect("actual retained body"),
            Some(block.clone())
        );
        drop(error);
        close(path, runtime, command);
    }
}
