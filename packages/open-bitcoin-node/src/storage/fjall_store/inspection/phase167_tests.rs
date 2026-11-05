// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txdb.cpp

use super::*;
use crate::chainstate::BasicFilterStartupMode;
use open_bitcoin_core::consensus::CanonicalChain;

pub(super) fn with_source(check: impl FnOnce(&Path)) {
    crate::storage::canonical_history::with_history(
        BasicFilterStartupMode::Disabled,
        1,
        false,
        |rows, _| with_rows_source(rows, check),
    );
}

type Rows = std::collections::BTreeMap<String, std::collections::BTreeMap<Vec<u8>, Vec<u8>>>;

pub(super) fn with_rows_source(rows: Rows, check: impl FnOnce(&Path)) {
    let mut nonce = [0; 16];
    getrandom::fill(&mut nonce).expect("nonce");
    let source = fs::canonicalize(std::env::temp_dir())
        .expect("physical parent")
        .join(format!("phase167-inspect-{:x}", u128::from_le_bytes(nonce)));
    {
        let db = fjall::Database::builder(&source).open().expect("fixture");
        for (name, rows) in rows {
            let space = db
                .keyspace(&name, fjall::KeyspaceCreateOptions::default)
                .expect("fixture namespace");
            for (key, value) in rows {
                space.insert(key, value).expect("fixture row");
            }
        }
        db.persist(fjall::PersistMode::SyncAll)
            .expect("complete fixture");
    }
    check(&source);
    fs::remove_dir_all(source).expect("all handles released");
}

fn corrupt_spend_script(source: &Path) {
    use super::super::{FjallNodeStore, bounded_rows};
    use crate::storage::{snapshot_codec as codec, validation_history as history};
    use bounded_rows::RowFamily;
    use open_bitcoin_core::consensus::{block_hash, transaction_txid};
    // Arrange a well-formed hostile claimed checkpoint. No live authority is
    // minted here: these are intentionally untrusted raw fixture mutations.
    let store = FjallNodeStore::open(source).expect("owned mutation fixture");
    let mut entries = store
        .load_header_entries()
        .expect("headers")
        .expect("present")
        .entries;
    let (mut active, maybe_counts) = store.load_chain_meta_for_open().expect("metadata");
    let old = active.last().expect("tip").block_hash;
    let mut block = store.load_block(old).expect("body").expect("present");
    let original_block = block.clone();
    let undo = store.load_undo(old).expect("undo").expect("present");
    let original_txid = transaction_txid(&block.transactions[1]).expect("valid spend txid");
    assert!(
        open_bitcoin_core::consensus::script::verify_script(
            &block.transactions[1].inputs[0].script_sig,
            &undo.transactions[0].restored_inputs[0].output.script_pubkey,
        )
        .is_ok()
    );
    block.transactions[1].inputs[0].script_sig =
        open_bitcoin_core::primitives::ScriptBuf::from_bytes(vec![0x6a]).expect("OP_RETURN");
    assert!(
        format!(
            "{:?}",
            open_bitcoin_core::consensus::script::verify_script(
                &block.transactions[1].inputs[0].script_sig,
                &undo.transactions[0].restored_inputs[0].output.script_pubkey,
            )
        )
        .contains("OpReturn")
    );
    crate::chainstate::validation_history::tests::phase167::remine(&mut block);
    let hash = block_hash(&block.header);
    let tip = active.last_mut().expect("tip");
    tip.block_hash = hash;
    tip.header = block.header.clone();
    let height = tip.height;
    let entry = entries
        .iter_mut()
        .find(|entry| entry.block_hash == old)
        .expect("exact header");
    entry.block_hash = hash;
    entry.header = block.header.clone();
    let work = entry.chain_work;
    let mut counts = maybe_counts.expect("complete counts");
    let count = counts.remove(&original_txid).expect("original occurrence");
    counts.insert(
        transaction_txid(&block.transactions[1]).expect("invalid spend txid"),
        count,
    );
    let genesis = open_bitcoin_core::consensus::chain_spec::CanonicalGenesisProof::prove(
        CanonicalChain::Regtest.spec(),
        &CanonicalChain::Regtest.spec().genesis_block(),
    )
    .expect("selected root");
    let mut ordered = entries.clone();
    ordered.sort_by_key(|entry| (entry.height, entry.block_hash));
    let headers = open_bitcoin_network::HeaderStore::readmit_canonical_entries(
        &genesis,
        ordered,
        i64::from(block.header.time) + 1,
    )
    .expect("all actual header gates pass");
    let context = headers
        .canonical_header_validation_context(&block.header)
        .expect("actual parent context");
    open_bitcoin_core::consensus::block::check_block_with_candidate_history(
        &block,
        &headers
            .canonical_header_history(hash)
            .expect("complete context"),
        context.current_time,
    )
    .expect("body/header/reward context precedes real input script");
    let detached = InspectedRecords::read_existing(&store.db).expect("complete valid source");
    let records =
        crate::storage::canonical_history::records::CanonicalHistoryRecords::decode(&detached)
            .expect("valid data");
    let proof = crate::storage::canonical_bootstrap::validate_history_records(
        CanonicalChain::Regtest,
        &records,
    )
    .expect("full actual valid replay");
    let (earned_genesis, mut earned_headers, mut replay) = proof
        .into_parts(&records)
        .expect("actual source-bound owners");
    let observed = earned_headers
        .canonical_header_validation_context(&original_block.header)
        .expect("original held shell time")
        .current_time;
    replay
        .disconnect_tip(&original_block)
        .expect("actual parent via real undo");
    earned_headers
        .insert_canonical_header(block.header.clone(), observed)
        .expect("mutated header independently passes");
    let context = earned_headers
        .canonical_header_validation_context(&block.header)
        .expect("actual admitted context");
    replay
        .admit_canonical_header(&block.header, context.current_time)
        .expect("real parent core admission");
    let parent = replay.admission_snapshot().expect("complete earned parent");
    let error = replay
        .connect_block_with_current_time(
            &block,
            work,
            context.current_time,
            open_bitcoin_core::consensus::ScriptVerifyFlags::NONE,
            open_bitcoin_core::chainstate::engine::canonical_genesis::consensus_params(
                &earned_genesis,
            ),
        )
        .expect_err("actual input script must reject");
    assert!(
        matches!(&error,
            open_bitcoin_core::chainstate::ChainstateError::TransactionValidation { source }
            if source.reject_reason == "mandatory-script-verify-flag-failed"
                && source.debug_message.as_deref() == Some("OP_RETURN encountered")
        ),
        "concrete script failure: {error:?}"
    );
    assert_eq!(
        replay.admission_snapshot().expect("no accepted mutation"),
        parent
    );
    assert_eq!(
        replay.coins_best_block().expect("unchanged best"),
        parent.tip().map(|tip| tip.block_hash)
    );
    eprintln!("HISTORY_SCRIPT_CAUSE concrete={error:?} parent_unchanged=true");
    let mut batch = store
        .db
        .batch()
        .durability(Some(fjall::PersistMode::SyncAll));
    for (family, bytes) in [
        (
            RowFamily::Headers,
            codec::encode_header_entries(&entries).expect("headers"),
        ),
        (
            RowFamily::BlockIndex,
            codec::encode_block_index_entries(&entries).expect("index"),
        ),
        (
            RowFamily::ChainMeta,
            codec::encode_chain_meta(&active, Some(&counts)).expect("active metadata"),
        ),
    ] {
        bounded_rows::append_replacement(
            &store,
            &mut batch,
            bounded_rows::prepare(family, bytes).expect("bounded family"),
        )
        .expect("replace family");
    }
    let identity =
        history::BlockValidationIdentity::new(hash, block.header.previous_block_hash, height)
            .expect("claimed identity");
    batch.remove(&store.block_index, super::super::block_key(old));
    batch.insert(
        &store.block_index,
        super::super::block_key(hash),
        open_bitcoin_core::codec::encode_block(&block).expect("body"),
    );
    batch.remove(&store.block_index, history::record_key(old));
    batch.insert(
        &store.block_index,
        history::record_key(hash),
        history::encode_record(history::ValidationHistoryRecord::new(
            identity,
            history::StoredValidationStatus::ScriptsValid,
        )),
    );
    batch.remove(&store.chainstate, super::super::coins::undo_key(old));
    batch.insert(
        &store.chainstate,
        super::super::coins::undo_key(hash),
        codec::encode_block_undo(&undo).expect("untrusted undo"),
    );
    batch.insert(&store.coins, b"B", hash.as_bytes());
    batch.commit().expect("hostile source fixture");
}

#[test]
fn phase167_preeffect_history_source_symlink_refuses_without_backend_or_mutation() {
    with_source(|source| {
        // Arrange
        let before = copy::Inventory::capture(source).expect("full original manifest");
        let alias = source.with_extension("symlink");
        std::os::unix::fs::symlink(source, &alias).expect("owned symlink fixture");
        let calls = crate::storage::chain_compatibility::BACKEND_BUILDER_CALLS.get();
        // Act / Assert
        assert!(
            super::super::FjallNodeStore::reopen_canonical_history_existing(
                &alias,
                CanonicalChain::Regtest
            )
            .is_err()
        );
        assert_eq!(
            crate::storage::chain_compatibility::BACKEND_BUILDER_CALLS.get(),
            calls
        );
        assert_eq!(
            before,
            copy::Inventory::capture(source).expect("whole original unchanged")
        );
        fs::remove_file(alias).expect("owned alias cleanup");
    });
}

#[test]
fn phase167_preeffect_history_malformed_authority_never_opens_original() {
    use super::super::FjallNodeStore;
    use crate::storage::chain_compatibility::BACKEND_BUILDER_CALLS;
    for case in [
        "schema",
        "missing-body",
        "corrupt-undo",
        "synthetic-work",
        "noncanonical",
        "stale-family",
    ] {
        with_source(|source| {
            // Arrange an isolated hostile source, then freeze its complete manifest.
            {
                let store = FjallNodeStore::open(source).expect("owned hostile fixture");
                let entries = store
                    .load_header_entries()
                    .expect("headers")
                    .expect("present")
                    .entries;
                let tip = entries
                    .iter()
                    .max_by_key(|entry| entry.height)
                    .expect("tip");
                match case {
                    "schema" => {
                        store
                            .schema
                            .insert(b"schema_version", b"3")
                            .expect("schema fault");
                    }
                    "missing-body" => {
                        store
                            .block_index
                            .remove(super::super::block_key(tip.block_hash))
                            .expect("missing body");
                    }
                    "corrupt-undo" => {
                        store
                            .chainstate
                            .insert(super::super::coins::undo_key(tip.block_hash), b"bad")
                            .expect("undo fault");
                    }
                    "synthetic-work" => {
                        let mut entries = entries.clone();
                        entries[1].chain_work =
                            open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO;
                        store
                            .save_header_entries(&entries, crate::PersistMode::Sync)
                            .expect("raw untrusted work");
                        store.block_index.insert(crate::storage::validation_history::COVERAGE_KEY, crate::storage::validation_history::encode_coverage(crate::storage::validation_history::ValidationCoverage::Complete)).expect("retain claimed complete coverage");
                    }
                    "noncanonical" => {
                        store
                            .block_index
                            .insert(
                                super::super::block_key(
                                    CanonicalChain::Regtest.spec().genesis_hash(),
                                ),
                                b"bad root",
                            )
                            .expect("noncanonical body");
                    }
                    "stale-family" => {
                        store
                            .headers
                            .insert(b"snapshot:orphan", b"stale")
                            .expect("stale family");
                    }
                    _ => unreachable!("fixed test case"),
                }
                store
                    .db
                    .persist(fjall::PersistMode::SyncAll)
                    .expect("freeze fixture");
            }
            let before = copy::Inventory::capture(source).expect("full physical before");
            let calls = BACKEND_BUILDER_CALLS.get();
            // Act / Assert
            assert!(
                FjallNodeStore::reopen_canonical_history_existing(source, CanonicalChain::Regtest)
                    .is_err(),
                "{case}"
            );
            assert_eq!(
                BACKEND_BUILDER_CALLS.get(),
                calls,
                "no original backend: {case}"
            );
            assert_eq!(
                before,
                copy::Inventory::capture(source).expect("complete unchanged source"),
                "{case}"
            );
        });
    }
}

#[test]
fn phase167_preeffect_history_worker_valid_and_script_invalid_recoverable_history() {
    crate::storage::canonical_history::with_history(
        BasicFilterStartupMode::Disabled,
        301,
        true,
        |rows, _| {
            with_rows_source(rows, |source| {
                // Arrange / Act: paired real valid history reaches completed child proof.
                let lease = OperationLease::acquire(source).expect("lease");
                canonical_history::inspect_history(&lease, CanonicalChain::Regtest)
                    .expect("valid paired proof");
                assert_eq!(worker::HISTORY_OBSERVATION.get().0, Some(2));
                drop(lease);
                corrupt_spend_script(source);
                let before = copy::Inventory::capture(source).expect("complete invalid source");
                let calls = crate::storage::chain_compatibility::BACKEND_BUILDER_CALLS.get();
                let lease = OperationLease::acquire(source).expect("lease");
                // Unchanged generic worker recovers the complete, well-formed data.
                with_inspection(&lease, |records| {
                    crate::storage::canonical_history::records::CanonicalHistoryRecords::decode(
                        records,
                    )
                    .expect("data-only parser accepts");
                    Ok(())
                })
                .expect("generic recovery succeeds");
                // Assert: history child enters the actual replay and refuses before output.
                assert!(
                    canonical_history::inspect_history(&lease, CanonicalChain::Regtest).is_err()
                );
                assert_eq!(worker::HISTORY_OBSERVATION.get().0, Some(1));
                assert_eq!(
                    before,
                    copy::Inventory::capture(source).expect("invalid source unchanged")
                );
                assert!(!OBSERVED_SCRATCH.take().expect("scratch").exists());
                drop(lease);
                assert!(
                    super::super::FjallNodeStore::reopen_canonical_history_existing(
                        source,
                        CanonicalChain::Regtest
                    )
                    .is_err()
                );
                assert_eq!(
                    before,
                    copy::Inventory::capture(source).expect("no original backend effects")
                );
                assert_eq!(
                    crate::storage::chain_compatibility::BACKEND_BUILDER_CALLS.get(),
                    calls
                );
            });
        },
    );
}

#[test]
fn phase167_preeffect_history_descendant_requires_separate_safe_opener() {
    with_source(|source| {
        // Arrange
        let before = copy::Inventory::capture(source).expect("original manifest");
        // Act
        let lease = OperationLease::acquire(source).expect("lease");
        canonical_history::inspect_history(&lease, CanonicalChain::Regtest)
            .expect("copied full proof");
        drop(lease);
        assert_eq!(
            before,
            copy::Inventory::capture(source).expect("unchanged copied source")
        );
        let opened = super::super::FjallNodeStore::reopen_canonical_history_existing(
            source,
            CanonicalChain::Regtest,
        );
        // Assert
        assert!(opened.is_ok(), "separate safe history reopen is required");
        drop(opened);
    });
}

#[test]
fn phase167_preeffect_history_worker_actual_proof_lifecycle_and_original_preservation() {
    with_source(|source| {
        // Arrange
        let before = copy::Inventory::capture(source).expect("physical manifest");
        let lease = OperationLease::acquire(source).expect("exclusive lease");
        for (fault, expected_stage) in [
            (0, Some(2)),
            (20, Some(1)),
            (21, Some(2)),
            (22, Some(2)),
            (23, Some(2)),
            (11, None),
            (12, None),
        ] {
            worker::history_fault(fault);
            let started = std::time::Instant::now();
            let mut callback = false;
            // Act
            let result = inspect_owned_then(
                &lease,
                |scratch| worker::recover_history(scratch, CanonicalChain::Regtest),
                |_, source, _| {
                    assert!(OperationLease::acquire(source).is_err());
                    let engine = File::open(source.join("lock")).expect("engine lock");
                    assert!(engine.try_lock().is_err());
                    Ok(())
                },
                |records| {
                    callback = true;
                    Ok(records)
                },
            );
            worker::history_fault(0);
            let (stage, child_elapsed) = worker::HISTORY_OBSERVATION.get();
            // Assert
            assert_eq!(
                stage, expected_stage,
                "actual child proof stage, fault {fault}"
            );
            assert_eq!(
                result.is_ok(),
                fault == 0,
                "fault {fault}: {:?}",
                result.as_ref().err()
            );
            assert_eq!(callback, fault == 0);
            assert_eq!(
                before,
                copy::Inventory::capture(source).expect("source preserved")
            );
            let scratch = OBSERVED_SCRATCH.take().expect("actual scratch");
            if matches!(fault, 11 | 12) {
                assert!(scratch.exists(), "uncertain completion cannot clean");
                fs::remove_dir_all(scratch).expect("test supervisor actually reaped");
            } else {
                assert!(!scratch.exists(), "confirmed reap cleaned");
            }
            if (20..=22).contains(&fault) {
                assert!(child_elapsed >= std::time::Duration::from_secs(30));
                assert!(format!("{:?}", result.err()).contains("deadline exceeded"));
            }
            eprintln!(
                "HISTORY_CHILD fault={fault} proof_stage={stage:?} child={child_elapsed:?} total={:?}",
                started.elapsed()
            );
        }
        assert!(OperationLease::acquire(source).is_err());
        drop(lease);
        drop(OperationLease::acquire(source).expect("application released"));
    });
}

#[test]
fn phase167_preeffect_history_worker_cleanup_source_swap_and_selector_gates() {
    with_source(|source| {
        // Arrange
        let before = copy::Inventory::capture(source).expect("complete source");
        let lease = OperationLease::acquire(source).expect("lease");
        for selected in [
            CanonicalChain::Mainnet,
            CanonicalChain::Testnet3,
            CanonicalChain::Signet,
        ] {
            // Act / Assert
            assert!(canonical_history::inspect_history(&lease, selected).is_err());
            assert_eq!(worker::HISTORY_OBSERVATION.get().0, Some(1));
            assert_eq!(
                before,
                copy::Inventory::capture(source).expect("wrong chain preserved")
            );
        }
        FAIL_CLEANUP.set(true);
        assert!(canonical_history::inspect_history(&lease, CanonicalChain::Regtest).is_err());
        assert_eq!(worker::HISTORY_OBSERVATION.get().0, Some(2));
        fs::remove_dir_all(OBSERVED_SCRATCH.take().expect("cleanup failure root"))
            .expect("injected leftover");
        assert_eq!(
            before,
            copy::Inventory::capture(source).expect("cleanup source preserved")
        );
        let saved = source.with_extension("saved");
        let mut callback = false;
        let result = inspect_owned_then(
            &lease,
            |scratch| worker::recover_history(scratch, CanonicalChain::Regtest),
            |stage, source, _| {
                if stage == Stage::Copied {
                    fs::rename(source, &saved).expect("move exact original");
                    std::os::unix::fs::symlink(&saved, source).expect("source swap");
                }
                Ok(())
            },
            |_| {
                callback = true;
                Ok(())
            },
        );
        assert!(result.is_err());
        assert!(!callback);
        fs::remove_file(source).expect("unlink owned source alias");
        fs::rename(saved, source).expect("restore original source");
        assert_eq!(
            before,
            copy::Inventory::capture(source).expect("original whole manifest")
        );
        drop(lease);
    });
}
