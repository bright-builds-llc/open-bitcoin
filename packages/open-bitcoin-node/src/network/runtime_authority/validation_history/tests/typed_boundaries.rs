// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/consensus/validation.h

use super::*;

fn resource_probe(wide: bool) {
    use open_bitcoin_core::primitives::{Amount, ScriptBuf, TransactionOutput};
    // Arrange: genuine configured command; only one actual accepted descendant.
    let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
    let mut block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    let count = if wide { 1 } else { 4200 };
    block.transactions[0].outputs = (0..count)
        .map(|index| TransactionOutput {
            value: Amount::from_sats(if index == 0 { 5_000_000_000 } else { 0 })
                .expect("valid total"),
            script_pubkey: ScriptBuf::from_serialized_bytes(vec![
                0x51;
                if wide { 66000 } else { 1 }
            ]),
        })
        .collect();
    remine(&mut block);
    let body_bytes = open_bitcoin_core::codec::encode_block(&block)
        .expect("exact bytes")
        .len();
    let before = accepted_state(&command);
    let before_mempool = command.network.mempool_info().expect("actual mempool");
    let mut malformed = wire(&block);
    malformed.transactions[0].outputs[0].value_sats -= 1;
    reject(&command, malformed, "bad-txnmrklroot");
    // Act
    let outcome = command.submit(wire(&block));
    let was_accepted = matches!(&outcome, Ok(CanonicalSubmissionOutcome::Accepted(_)));
    let live = command
        .network
        .chainstate_snapshot()
        .expect("actual complete state");
    let persisted = command
        .store
        .coins_view()
        .complete_snapshot()
        .expect("actual durable Coins");
    eprintln!(
        "RESOURCE_PROBE wide={wide} body_bytes={body_bytes} actual_outputs={count} command={outcome:?} live_height={} live_coins={} durable_coins={} durable_best={:?}",
        live.tip().expect("tip").height,
        live.utxos.len(),
        persisted.coins.len(),
        persisted.maybe_best_block
    );
    // Assert: resource policy is operational and does not fabricate accepted facts.
    assert!(
        outcome.is_err(),
        "deterministic native limit must refuse before absorption"
    );
    assert_eq!(accepted_state(&command), before);
    assert_eq!(
        command.network.mempool_info().expect("unchanged mempool"),
        before_mempool
    );
    let hash = block_hash(&block.header);
    assert_eq!(
        command
            .store
            .maybe_validation_history_record(hash)
            .expect("retained identity")
            .expect("actual KnownOnly")
            .status(),
        crate::storage::validation_history::StoredValidationStatus::KnownOnly
    );
    let facts = command
        .network
        .maybe_body_eligibility(hash)
        .expect("volatile eligibility")
        .expect("actual checked candidate");
    assert!(
        !facts.failed,
        "resource refusal is not permanent invalidity"
    );
    command
        .network
        .read(|network| assert_eq!(network.blocks_by_hash.get(&hash), Some(&block)))
        .expect("actual volatile normalized body retained");
    assert_eq!(
        command.store.load_block(hash).expect("body not persisted"),
        None
    );
    assert_eq!(
        command
            .network
            .header_entries()
            .expect("earned headers")
            .len(),
        2
    );
    drop(outcome);
    drop(command);
    drop(runtime);
    let reopened = DurableSyncRuntime::open_canonical_submission_configured(
        &path,
        config(),
        BasicFilterStartupMode::Disabled,
    );
    let safe = reopened.is_ok();
    if let Err(error) = &reopened {
        eprintln!("RESOURCE_PROBE wide={wide} configured_reopen_error={error:?}");
    }
    eprintln!("RESOURCE_PROBE wide={wide} accepted={was_accepted} configured_reopen={safe}");
    let (reopened_runtime, reopened_command) =
        reopened.expect("safe old-state reopen after all handles dropped");
    let reopened_command = reopened_command.expect("actual Regtest owner");
    assert_eq!(accepted_state(&reopened_command), before);
    drop(reopened_command);
    drop(reopened_runtime);
    std::fs::remove_dir_all(path).expect("all handles dropped before safe reopen and cleanup");
    assert!(
        !was_accepted && safe,
        "operational refusal preserves the safe old accepted source"
    );
}

fn headers() -> HeaderStore {
    let chain = CanonicalChain::Regtest;
    let proof = open_bitcoin_core::consensus::chain_spec::CanonicalGenesisProof::prove(
        chain.spec(),
        &chain.spec().genesis_block(),
    )
    .expect("proved root");
    let mut headers = HeaderStore::from_canonical_genesis(&proof).expect("earned header owner");
    let params = open_bitcoin_core::chainstate::engine::canonical_genesis::consensus_params(&proof);
    headers
        .check_body_eligibility(proof.block(), params, 2_000_000_000)
        .expect("root linkage");
    headers
}

#[test]
fn phase167_local_submission_resource_probe_complete_body_authority_row() {
    resource_probe(true);
}

#[test]
fn phase167_local_submission_resource_probe_total_logical_coin_rows() {
    resource_probe(false);
}

#[test]
fn phase167_local_submission_typed_header_consensus_and_missing_parent_survive_inference() {
    // Arrange
    let mut headers = headers();
    let mut block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    block.header.version = 3;
    remine(&mut block);
    let mut legacy = headers.clone();
    let legacy_error = legacy
        .insert_canonical_header(block.header.clone(), 2_000_000_000)
        .expect_err("version");
    // Act
    let error = headers
        .insert_canonical_header_detailed(block.header.clone(), 2_000_000_000)
        .expect_err("typed version");
    // Assert
    let reason = error.fold(
        |source, reason| {
            assert_eq!(source.reject_reason, "bad-version");
            reason
        },
        |_, _| panic!("wrong parent"),
        |_| panic!("erased consensus"),
    );
    assert_eq!(reason, "bad-version(0x00000003)");
    let error = headers
        .insert_canonical_header_detailed(block.header.clone(), 2_000_000_000)
        .expect_err("same rejection");
    assert_eq!(error.into_network_error(), legacy_error);
    block.header.previous_block_hash = BlockHash::from_byte_array([0x30; 32]);
    remine(&mut block);
    let mut legacy = headers.clone();
    let prior = legacy
        .insert_canonical_header(block.header.clone(), 2_000_000_000)
        .expect_err("legacy missing");
    let missing = headers
        .insert_canonical_header_detailed(block.header.clone(), 2_000_000_000)
        .expect_err("typed missing");
    missing.fold(
        |_, _| panic!("not consensus"),
        |hash, error| {
            assert_eq!(hash, block.header.previous_block_hash);
            assert_eq!(error, prior);
        },
        |_| panic!("erased direct missing"),
    );
}

#[test]
fn phase167_local_submission_typed_live_validation_and_operational_errors_are_distinct() {
    // Arrange
    let mut headers = headers();
    let mut block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    block.transactions[0].inputs[0].script_sig =
        open_bitcoin_core::primitives::ScriptBuf::from_bytes(vec![0x52, 0]).expect("wrong height");
    remine(&mut block);
    headers
        .insert_canonical_header(block.header.clone(), 2_000_000_000)
        .expect("real header");
    let mut legacy = headers.clone();
    let prior = legacy
        .check_body_eligibility(&block, params(), 2_000_000_000)
        .expect_err("legacy body");
    // Act
    let error = headers
        .check_body_eligibility_detailed(&block, params(), 2_000_000_000)
        .expect_err("actual typed body");
    // Assert
    assert!(
        matches!(&error,BodyEligibilityError::Validation(source) if source.reject_reason=="bad-cb-height")
    );
    assert_eq!(error.into_network_error(), prior);
    let mut wrong = params();
    wrong.coinbase_maturity = 1;
    assert!(matches!(
        headers.check_body_eligibility_detailed(&block, wrong, 0),
        Err(BodyEligibilityError::Network(_))
    ));
    HeaderStore::default()
        .insert_canonical_header_detailed(block.header, 0)
        .expect_err("unowned")
        .fold(
            |_, _| panic!("owner is not consensus"),
            |_, _| panic!("owner is not absent parent"),
            |_| (),
        );
}

#[test]
fn phase167_local_submission_typed_intrinsic_precedes_duplicate_and_real_missing_parent() {
    // Arrange
    let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
    let genesis = CanonicalChain::Regtest.spec().genesis_block();
    let mut corrupt = wire(&genesis);
    corrupt.transactions[0].outputs[0].value_sats -= 1;
    let before = manifest(&path);
    // Act / Assert
    reject(&command, corrupt, "bad-txnmrklroot");
    assert_eq!(manifest(&path), before);
    let mut missing = child(&genesis.header, 1);
    missing.header.previous_block_hash = BlockHash::from_byte_array([0x30; 32]);
    remine(&mut missing);
    let mut corrupt = wire(&missing);
    corrupt.transactions[0].outputs[0].value_sats -= 1;
    reject(&command, corrupt, "bad-txnmrklroot");
    reject(&command, wire(&missing), "prev-blk-not-found");
    assert_eq!(manifest(&path), before);
    close(path, runtime, command);
}

#[test]
fn phase167_local_submission_typed_witness_only_root_and_descendant_duplicates_are_actual() {
    // Arrange
    let (path, runtime, command) = runtime(BasicFilterStartupMode::Enabled);
    let genesis = CanonicalChain::Regtest.spec().genesis_block();
    let block = child(&genesis.header, 1);
    accepted(
        command
            .submit(wire(&block))
            .expect("actual accepted descendant"),
    );
    let before = accepted_state(&command);
    // Act / Assert
    for body in [&genesis, &block] {
        let mut raw = wire(body);
        raw.transactions[0].inputs[0].witness =
            open_bitcoin_core::primitives::ScriptWitness::new(vec![vec![7]]);
        command
            .submit(raw)
            .expect("intrinsic independent accepted identity")
            .fold(
                |_| panic!("not new"),
                |hash| assert_eq!(hash, block_hash(&body.header)),
                |reason| panic!("false rejection {reason}"),
                || panic!("false inconclusive"),
            );
        assert_eq!(accepted_state(&command), before);
    }
    close(path, runtime, command);
}

fn contextual_rejection(witness: bool) {
    use open_bitcoin_core::primitives::{Amount, ScriptBuf, ScriptWitness, TransactionOutput};
    // Arrange
    let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
    let mut block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    let reason = if witness {
        let commitment = open_bitcoin_core::consensus::crypto::double_sha256(&[0; 64]);
        block.transactions[0].outputs.push(TransactionOutput {
            value: Amount::ZERO,
            script_pubkey: ScriptBuf::from_serialized_bytes(
                [
                    vec![0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed],
                    commitment.to_vec(),
                ]
                .concat(),
            ),
        });
        block.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![7]]);
        "bad-witness-nonce-size"
    } else {
        block.transactions[0].inputs[0].script_sig =
            ScriptBuf::from_serialized_bytes(vec![0x52, 0]);
        "bad-cb-height"
    };
    remine(&mut block);
    let mut carrier = headers();
    carrier
        .insert_canonical_header(block.header.clone(), 2_000_000_000)
        .expect("actual header");
    match carrier
        .check_body_eligibility_detailed(&block, params(), 2_000_000_000)
        .expect_err("actual typed context")
    {
        BodyEligibilityError::Validation(source) => assert_eq!(
            source.result,
            if witness {
                open_bitcoin_core::consensus::BlockValidationResult::Mutated
            } else {
                open_bitcoin_core::consensus::BlockValidationResult::Consensus
            }
        ),
        BodyEligibilityError::Network(error) => panic!("false operational category {error:?}"),
    }
    // Act / Assert
    reject(&command, wire(&block), reason);
    reject(
        &command,
        wire(&block),
        if witness { reason } else { "duplicate-invalid" },
    );
    assert_eq!(
        command
            .network
            .maybe_body_eligibility(block_hash(&block.header))
            .expect("actual registry")
            .is_some_and(|facts| facts.failed),
        !witness
    );
    assert_eq!(
        command
            .store
            .load_block(block_hash(&block.header))
            .expect("no accepted body"),
        None
    );
    close(path, runtime, command);
}

#[test]
fn phase167_local_submission_typed_contextual_consensus_rejection_is_permanent() {
    contextual_rejection(false);
}

#[test]
fn phase167_local_submission_typed_witness_mutation_rejection_is_nonpermanent() {
    contextual_rejection(true);
}

#[test]
fn phase167_local_submission_typed_actual_reward_failure_earns_duplicate_invalid() {
    // Arrange
    let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
    let mut block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    block.transactions[0].outputs[0].value =
        open_bitcoin_core::primitives::Amount::from_sats(5_000_000_001)
            .expect("valid monetary range");
    remine(&mut block);
    // Act / Assert
    reject(&command, wire(&block), "bad-cb-amount");
    assert!(
        command
            .network
            .maybe_body_eligibility(block_hash(&block.header))
            .expect("real registry")
            .expect("actual checked candidate")
            .failed
    );
    reject(&command, wire(&block), "duplicate-invalid");
    close(path, runtime, command);
}

#[test]
fn phase167_local_submission_typed_live_order_and_cloned_epoch_match_legacy_wrappers() {
    // Arrange
    let mut live = headers();
    let mut cached = live.clone();
    let block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    live.insert_canonical_header(block.header.clone(), 2_000_000_000)
        .expect("earned header");
    cached
        .insert_canonical_header(block.header.clone(), 2_000_000_000)
        .expect("same earned header");
    let epoch = live.body_eligibility_epoch();
    assert!(epoch.is_current());
    // Act
    live.check_body_eligibility_detailed(&block, params(), 2_000_000_000)
        .expect("actual live false mode");
    cached
        .check_cached_body_eligibility_detailed(&block, params(), 2_000_000_000)
        .expect("actual cached true mode");
    // Assert
    let hash = block_hash(&block.header);
    assert!(
        live.maybe_body_eligibility(hash)
            .expect("live receipt")
            .maybe_sequence
            .is_some()
    );
    assert!(
        cached
            .maybe_body_eligibility(hash)
            .expect("cached receipt")
            .maybe_sequence
            .is_none()
    );
    assert!(!epoch.is_current());
    let live_epoch = live.body_eligibility_epoch();
    let cloned_epoch = {
        let clone = live.clone();
        let epoch = clone.body_eligibility_epoch();
        assert!(epoch.is_current());
        epoch
    };
    assert!(!cloned_epoch.is_current());
    assert!(live_epoch.is_current());
}

#[test]
fn phase167_local_submission_typed_retained_descendants_of_contextually_failed_parent_stay_unsupported()
 {
    // Arrange
    let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
    let mut parent = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    parent.transactions[0].inputs[0].script_sig =
        open_bitcoin_core::primitives::ScriptBuf::from_serialized_bytes(vec![0x52, 0]);
    remine(&mut parent);
    let child = child(&parent.header, 2);
    admit_only(&command, &[parent.clone(), child.clone()]);
    // Act
    reject(&command, wire(&parent), "bad-cb-height");
    // Assert: known nonfailed headers short-circuit the pin's direct-parent check.
    let before = accepted_state(&command);
    assert!(
        command.submit(wire(&child)).is_err(),
        "local branch remains unsupported"
    );
    let descendant = genuine::child(&child.header, 3);
    assert!(
        command.submit(wire(&descendant)).is_err(),
        "no invented inconclusive processing"
    );
    assert_eq!(accepted_state(&command), before);
    assert!(
        command
            .network
            .eligible_body_candidates()
            .expect("actual activation candidates")
            .iter()
            .all(|facts| facts.block_hash != block_hash(&child.header)
                && facts.block_hash != block_hash(&descendant.header))
    );
    close(path, runtime, command);
}

#[test]
fn phase167_local_submission_typed_real_body_flush_and_basic_failures_keep_actual_stage() {
    use crate::storage::fjall_store::filters::FilterPublicationFault as Fault;
    for fault in [
        Fault::BeforeBody,
        Fault::BeforeUndo,
        Fault::BeforeCoins,
        Fault::BeforeChainMeta,
        Fault::BeforeRecords,
        Fault::BeforeCheckpoint,
        Fault::BeforeProtection,
        Fault::BeforeCommit,
        Fault::AfterCommit,
    ] {
        // Arrange
        let (path, runtime, command) = runtime(BasicFilterStartupMode::Enabled);
        let block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        command.store.set_basic_filter_fault(fault);
        // Act
        let result = command.submit(wire(&block));
        // Assert
        assert!(
            result.is_err(),
            "actual fault must prevent durable success: {fault:?}"
        );
        let expected = if fault == Fault::BeforeBody { 0 } else { 1 };
        assert_eq!(
            command
                .network
                .maybe_chain_tip()
                .expect("actual tip")
                .expect("tip")
                .height,
            expected,
            "fault {fault:?}"
        );
        if expected == 1 {
            assert_eq!(
                command
                    .store
                    .load_block(block_hash(&block.header))
                    .expect("real retained body"),
                Some(block)
            );
            assert_eq!(
                command
                    .network
                    .read(
                        |network| network.chainstate().validation_provenance(block_hash(
                            &network.maybe_chain_tip().expect("tip").header
                        ))
                    )
                    .expect("real provenance read")
                    .expect("real accepted provenance"),
                crate::storage::validation_history::ValidationProvenance::ScriptsValid
            );
        }
        drop(result);
        close(path, runtime, command);
    }
}

#[test]
fn phase167_local_submission_typed_novel_direct_failed_parent_rejects_before_effects() {
    // Arrange
    let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
    let mut parent = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    parent.transactions[0].inputs[0].script_sig =
        open_bitcoin_core::primitives::ScriptBuf::from_serialized_bytes(vec![0x52, 0]);
    remine(&mut parent);
    reject(&command, wire(&parent), "bad-cb-height");
    let child = child(&parent.header, 2);
    let hash = block_hash(&child.header);
    assert!(
        command
            .network
            .header_entries()
            .expect("actual entries")
            .iter()
            .all(|entry| entry.block_hash != hash)
    );
    let rows = native_rows(&command.store);
    let physical = manifest(&path);
    // Act / Assert
    let mut invalid = wire(&child);
    invalid.transactions[0].outputs[0].value_sats = -1;
    reject(&command, invalid, "bad-txnmrklroot");
    reject(&command, wire(&child), "bad-prevblk");
    assert_eq!(native_rows(&command.store), rows);
    assert_eq!(manifest(&path), physical);
    assert!(
        command
            .network
            .maybe_body_eligibility(hash)
            .expect("no false linkage")
            .is_none()
    );
    close(path, runtime, command);
}

#[test]
fn phase167_local_submission_typed_publication_failure_after_absorption_keeps_real_receipt() {
    use crate::storage::fjall_store::validation_history::HistoryPublicationFault;
    for fault in [
        HistoryPublicationFault::BeforeCommit,
        HistoryPublicationFault::AfterCommit,
    ] {
        // Arrange
        let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
        let block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        ABSORB_FAULT.set(Some(fault));
        // Act
        let error = command
            .submit(wire(&block))
            .expect_err("actual publication failure");
        // Assert: the error is operational and the actual Core is already advanced.
        assert_eq!(error.stage, "acceptance/publication");
        assert_eq!(
            command
                .network
                .maybe_chain_tip()
                .expect("actual advanced tip")
                .expect("tip")
                .height,
            1
        );
        assert!(
            command
                .network
                .read(|network| network
                    .chainstate()
                    .maybe_pending_validation_identity(block_hash(&block.header))
                    .is_some())
                .expect("real pending receipt")
        );
        drop(error);
        close(path, runtime, command);
    }
}
