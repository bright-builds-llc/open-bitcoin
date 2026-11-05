// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use crate::chainstate::BasicFilterStartupMode;
use crate::chainstate::validation_history::tests::phase167::{accepted, assert_unaccepted, remine};
use crate::chainstate::validation_history::tests::phase167::{admit, child, network, with_mode};
use open_bitcoin_core::primitives::{
    Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
};
use open_bitcoin_core::{
    chainstate::IndexPrefix,
    consensus::{ScriptVerifyFlags, block_hash},
};

fn with_handle(
    mode: BasicFilterStartupMode,
    check: impl FnOnce(
        &ManagedNetworkHandle<FjallChainstateStore, FjallCoinsView>,
        &crate::FjallNodeStore,
        open_bitcoin_core::consensus::ConsensusParams,
    ),
) {
    with_mode(mode, |manager, store, params| {
        let handle = ManagedNetworkHandle::new(network(manager));
        handle
            .initialize_basic_filter_index_owner()
            .expect("same owner");
        handle
            .drive_basic_filter_index_turn()
            .expect("initial turn");
        check(&handle, &store, params);
    });
}

#[test]
fn phase167_basic_continuation_descendant_requires_absorption_then_real_ordered_turn() {
    with_mode(BasicFilterStartupMode::Enabled, |manager, store, params| {
        // Arrange
        let handle = ManagedNetworkHandle::new(network(manager));
        handle
            .initialize_basic_filter_index_owner()
            .expect("same durable owner");
        handle
            .drive_basic_filter_index_turn()
            .expect("genesis seed");
        let root = handle.maybe_chain_tip().expect("tip").expect("genesis");
        let genesis = store
            .load_basic_filter_record(root.block_hash)
            .expect("record")
            .expect("genesis filter");
        assert_eq!(genesis.identity().previous_header(), Default::default());
        let block = child(&root.header, 1);
        let before = handle.maybe_basic_index_progress().expect("progress");
        let before_accepted = handle
            .read(|n| accepted(n.chainstate(), &store))
            .expect("complete accepted snapshot");
        handle
            .try_mutate(|network| {
                admit(network, std::slice::from_ref(&block), params);
                Ok(())
            })
            .expect("actual header");
        assert_eq!(
            handle
                .maybe_basic_index_progress()
                .expect("header cannot advance"),
            before
        );
        handle
            .try_mutate(|n| {
                n.guard_body(&block)?;
                let stage = n.chainstate().prepare_connect_block_with_current_time(
                    &block,
                    open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
                    i64::MIN,
                    ScriptVerifyFlags::NONE,
                    params,
                )?;
                drop(stage);
                Ok(())
            })
            .expect("genuine immutable preview only");
        handle
            .read(|n| assert_unaccepted(n.chainstate(), &store, &block, &before_accepted))
            .expect("headers and preview cannot accept");
        // Act
        handle
            .connect_local_block(&block, ScriptVerifyFlags::NONE, params)
            .expect("actual managed absorption");
        handle
            .drive_basic_filter_index_turn()
            .expect("accepted descendant BASIC");
        // Assert
        let record = store
            .load_basic_filter_record(block_hash(&block.header))
            .expect("record")
            .expect("descendant indexed");
        assert_eq!(
            record.identity().previous_header(),
            genesis.identity().filter_header()
        );
        let progress = handle
            .maybe_basic_index_progress()
            .expect("progress")
            .expect("owner");
        assert_eq!(progress.maybe_processed_endpoint(), Some(record.identity()));
        assert_eq!(
            progress.maybe_safe_durable_endpoint(),
            Some(genesis.identity()),
            "unflushed coins cannot publish a safe descendant"
        );
        store
            .save_block(&block, crate::PersistMode::Sync)
            .expect("retained accepted body");
        handle
            .flush_coins(
                open_bitcoin_core::chainstate::FlushMode::Always,
                open_bitcoin_core::chainstate::FlushPolicyTime::new(10),
                u64::MAX,
            )
            .expect("same-owner complete coins checkpoint");
        handle
            .drive_basic_filter_index_turn()
            .expect("durable fence promotion");
        let progress = handle
            .maybe_basic_index_progress()
            .expect("progress")
            .expect("owner");
        assert_eq!(
            progress.maybe_safe_durable_endpoint(),
            Some(record.identity())
        );
        let snapshot = handle.chainstate_snapshot().expect("accepted full undo");
        assert_eq!(
            store
                .load_undo(block_hash(&block.header))
                .expect("saved complete undo")
                .as_ref(),
            snapshot.undo_by_block.get(&block_hash(&block.header))
        );
        assert_eq!(
            store
                .maybe_basic_filter_checkpoint()
                .expect("checkpoint")
                .expect("durable")
                .prefix(),
            IndexPrefix::Committed(record.identity())
        );
    });
}

#[test]
fn phase167_basic_continuation_invalid_reward_sigops_locks_witness_and_height_leave_accepted_state()
{
    for mode in [
        BasicFilterStartupMode::Enabled,
        BasicFilterStartupMode::Disabled,
    ] {
        with_handle(mode, |handle, store, params| {
            for fault in 0..5 {
                // Arrange
                let mut block = child(
                    &handle.maybe_chain_tip().expect("tip").expect("root").header,
                    1,
                );
                match fault {
                    0 => {
                        block.transactions[0].outputs[0].value =
                            Amount::from_sats(5_000_000_001).expect("amount")
                    }
                    1 => {
                        block.transactions[0].outputs = vec![
                            TransactionOutput {
                                value: Amount::ZERO,
                                script_pubkey: ScriptBuf::from_bytes(vec![0xac; 7_000])
                                    .expect("bounded script")
                            };
                            3
                        ]
                    }
                    2 => {
                        block.transactions[0].lock_time = block.header.time;
                        block.transactions[0].inputs[0].sequence = 0;
                    }
                    3 => {
                        block.transactions[0].inputs[0].witness =
                            ScriptWitness::new(vec![vec![0; 32]])
                    }
                    _ => {
                        block.transactions[0].inputs[0].script_sig =
                            ScriptBuf::from_bytes(vec![0x52, 0]).expect("wrong height")
                    }
                }
                remine(&mut block);
                handle
                    .try_mutate(|n| {
                        admit(n, std::slice::from_ref(&block), params);
                        Ok(())
                    })
                    .expect("admitted exact header");
                let before = handle
                    .read(|n| accepted(n.chainstate(), store))
                    .expect("snapshot");
                // Act
                let result = handle.connect_local_block(&block, ScriptVerifyFlags::NONE, params);
                // Assert
                assert!(result.is_err(), "actual invalid body {fault}");
                handle
                    .read(|n| assert_unaccepted(n.chainstate(), store, &block, &before))
                    .expect("unchanged all coins/work/undo/provenance/BASIC");
            }
        });
    }
}

#[test]
fn phase167_basic_continuation_mature_p2sh_script_cannot_use_caller_none() {
    for mode in [
        BasicFilterStartupMode::Enabled,
        BasicFilterStartupMode::Disabled,
    ] {
        with_handle(mode, |handle, store, params| {
            // Arrange
            let mut maybe_first = None;
            for height in 1..=100 {
                let mut block = child(
                    &handle
                        .maybe_chain_tip()
                        .expect("tip")
                        .expect("parent")
                        .header,
                    height,
                );
                if height == 1 {
                    let mut script = vec![0xa9, 20];
                    script.extend(open_bitcoin_core::consensus::crypto::hash160(&[0]));
                    script.push(0x87);
                    block.transactions[0].outputs[0].script_pubkey =
                        ScriptBuf::from_bytes(script).expect("P2SH false redeem");
                    remine(&mut block);
                    maybe_first = Some(OutPoint {
                        txid: open_bitcoin_core::consensus::transaction_txid(
                            &block.transactions[0],
                        )
                        .expect("txid"),
                        vout: 0,
                    });
                }
                handle
                    .try_mutate(|n| {
                        admit(n, std::slice::from_ref(&block), params);
                        Ok(())
                    })
                    .expect("header");
                handle
                    .connect_local_block(&block, ScriptVerifyFlags::NONE, params)
                    .expect("real accepted maturity history");
                store
                    .save_block(&block, crate::PersistMode::Sync)
                    .expect("retained history");
                handle
                    .drive_basic_filter_index_turn()
                    .expect("only accepted history progresses");
            }
            let mut invalid = child(
                &handle
                    .maybe_chain_tip()
                    .expect("tip")
                    .expect("parent")
                    .header,
                101,
            );
            invalid.transactions.push(Transaction {
                version: 2,
                lock_time: 0,
                inputs: vec![TransactionInput {
                    previous_output: maybe_first.expect("accepted funding"),
                    script_sig: ScriptBuf::from_bytes(vec![1, 0]).expect("false redeem push"),
                    sequence: u32::MAX,
                    witness: ScriptWitness::default(),
                }],
                outputs: vec![TransactionOutput {
                    value: Amount::from_sats(4_999_999_999).expect("amount"),
                    script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("true"),
                }],
            });
            remine(&mut invalid);
            handle
                .try_mutate(|n| {
                    admit(n, std::slice::from_ref(&invalid), params);
                    Ok(())
                })
                .expect("header");
            let before = handle
                .read(|n| accepted(n.chainstate(), store))
                .expect("complete snapshot");
            // Act
            let error = handle
                .connect_local_block(&invalid, ScriptVerifyFlags::NONE, params)
                .expect_err("selected P2SH interpreter must run");
            // Assert
            assert!(
                error
                    .to_string()
                    .contains("mandatory-script-verify-flag-failed"),
                "{error}"
            );
            handle
                .read(|n| assert_unaccepted(n.chainstate(), store, &invalid, &before))
                .expect("unchanged complete accepted state");
        });
    }
}

#[test]
fn phase167_basic_continuation_storage_error_cannot_publish_false_processed_or_safe_endpoint() {
    use crate::storage::fjall_store::filters::FilterPublicationFault;
    with_handle(BasicFilterStartupMode::Enabled, |handle, store, params| {
        // Arrange
        let block = child(
            &handle.maybe_chain_tip().expect("tip").expect("root").header,
            1,
        );
        handle
            .try_mutate(|n| {
                admit(n, std::slice::from_ref(&block), params);
                Ok(())
            })
            .expect("header");
        handle
            .connect_local_block(&block, ScriptVerifyFlags::NONE, params)
            .expect("actual absorption");
        let before = handle
            .chainstate_snapshot()
            .expect("complete accepted coins/undo/work");
        let before_progress = handle
            .maybe_basic_index_progress()
            .expect("progress")
            .expect("owner");
        let before_stored = store.maybe_basic_filter_state().expect("stored endpoints");
        store.set_basic_filter_fault(FilterPublicationFault::BeforeCheckpoint);
        // Act
        let result = handle.drive_basic_filter_index_turn();
        // Assert
        assert!(result.is_err());
        let after = handle
            .maybe_basic_index_progress()
            .expect("paused")
            .expect("owner");
        assert_eq!(
            after.maybe_processed_endpoint(),
            before_progress.maybe_processed_endpoint()
        );
        assert_eq!(
            after.maybe_safe_durable_endpoint(),
            before_progress.maybe_safe_durable_endpoint()
        );
        assert_eq!(
            handle.chainstate_snapshot().expect("coins remain accepted"),
            before
        );
        assert_eq!(
            store
                .maybe_basic_filter_state()
                .expect("no false stored endpoint"),
            before_stored
        );
        assert_eq!(
            store
                .validation_provenance(block_hash(&block.header))
                .expect("genuine acceptance retained"),
            crate::storage::validation_history::ValidationProvenance::ScriptsValid
        );
        assert!(
            handle.drive_basic_filter_index_turn().is_err(),
            "publication fault requires proved reopen"
        );
        assert_eq!(
            store
                .maybe_basic_filter_state()
                .expect("retry cannot bypass fault"),
            before_stored
        );
    });
}

#[test]
fn phase167_basic_continuation_foreign_and_stale_append_owners_refuse_without_endpoint_progress() {
    with_handle(BasicFilterStartupMode::Enabled, |first, first_store, _| {
        with_handle(
            BasicFilterStartupMode::Enabled,
            |second, second_store, _| {
                // Arrange
                let foreign = first_store
                    .maybe_basic_filter_append_proof()
                    .expect("proof")
                    .expect("active");
                let before = second_store
                    .maybe_basic_filter_state()
                    .expect("stored endpoints");
                // Act / Assert
                assert!(
                    second_store
                        .check_basic_filter_append_proof(&foreign)
                        .is_err()
                );
                assert_eq!(
                    second_store
                        .maybe_basic_filter_state()
                        .expect("foreign unchanged"),
                    before
                );
                let stale = first_store
                    .maybe_basic_filter_append_proof()
                    .expect("proof")
                    .expect("active");
                first.disable_basic_filter_index().expect("disable");
                first.enable_basic_filter_index().expect("new generation");
                let before = first_store
                    .maybe_basic_filter_state()
                    .expect("same endpoint");
                let before_chain = first.chainstate_snapshot().expect("accepted coins");
                assert!(first_store.check_basic_filter_append_proof(&stale).is_err());
                assert!(first.drive_basic_filter_index_turn().is_err());
                assert_eq!(
                    first_store
                        .maybe_basic_filter_state()
                        .expect("stale unchanged"),
                    before
                );
                assert_eq!(
                    first.chainstate_snapshot().expect("coins unchanged"),
                    before_chain
                );
                assert_eq!(
                    second
                        .maybe_chain_tip()
                        .expect("foreign tip")
                        .expect("root")
                        .height,
                    0
                );
            },
        );
    });
}

#[test]
fn phase167_managed_connect_batch_exactness_original_clock_and_generation_controls() {
    use open_bitcoin_core::chainstate::AnchoredBlock;
    with_handle(BasicFilterStartupMode::Disabled, |handle, store, params| {
        handle
            .try_mutate(|n| {
                // Arrange
                let root = n.maybe_chain_tip().expect("genesis").header;
                let active = child(&root, 1);
                admit(n, std::slice::from_ref(&active), params);
                n.connect_local_block(&active, ScriptVerifyFlags::NONE, params)?;
                let mut first = child(&root, 1);
                first.header.time += 100;
                remine(&mut first);
                let second = child(&first.header, 2);
                let blocks = vec![first.clone(), second.clone()];
                admit(n, &blocks, params);
                let rows: Vec<_> = blocks
                    .iter()
                    .map(|b| {
                        (
                            b.header.clone(),
                            n.peer_manager
                                .header_store()
                                .canonical_header_validation_context(&b.header)
                                .expect("actual held clock")
                                .current_time,
                        )
                    })
                    .collect();
                let disconnect = vec![active.header.clone()];
                let branch: Vec<_> = blocks
                    .iter()
                    .map(|b| AnchoredBlock {
                        block: b.clone(),
                        chain_work: n
                            .header_entries()
                            .iter()
                            .find(|e| e.block_hash == block_hash(&b.header))
                            .expect("earned work")
                            .chain_work,
                    })
                    .collect();
                let before = accepted(n.chainstate(), store);
                // Act / Assert
                assert!(
                    n.chainstate_mut()
                        .admit_canonical_body_reorg(&[], &rows)
                        .is_err()
                );
                assert!(
                    n.chainstate_mut()
                        .admit_canonical_body_reorg(&[root], &rows)
                        .is_err()
                );
                let mut reversed = rows.clone();
                reversed.reverse();
                assert!(
                    n.chainstate_mut()
                        .admit_canonical_body_reorg(&disconnect, &reversed)
                        .is_err()
                );
                n.chainstate_mut()
                    .admit_canonical_body_reorg(&disconnect, &rows)?;
                let source = n.chainstate().chainstate().bind_live_work_source(params)?;
                let reversed_clocks: Vec<_> = rows
                    .iter()
                    .map(|(header, _)| (header.clone(), i64::MIN))
                    .collect();
                n.chainstate_mut()
                    .admit_canonical_body_reorg(&disconnect, &reversed_clocks)?;
                n.chainstate()
                    .chainstate()
                    .check_live_work_source(&source)?;
                assert!(
                    n.chainstate()
                        .chainstate()
                        .stage_reorg(
                            std::slice::from_ref(&active),
                            &branch[..1],
                            ScriptVerifyFlags::NONE,
                            params
                        )
                        .is_err()
                );
                let mut wrong_work = branch.clone();
                wrong_work[1].chain_work =
                    open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO;
                assert!(
                    n.chainstate()
                        .chainstate()
                        .stage_reorg(
                            std::slice::from_ref(&active),
                            &wrong_work,
                            ScriptVerifyFlags::NONE,
                            params
                        )
                        .is_err()
                );
                let stage = n.chainstate().chainstate().stage_reorg(
                    std::slice::from_ref(&active),
                    &branch,
                    ScriptVerifyFlags::NONE,
                    params,
                )?;
                assert_unaccepted(n.chainstate(), store, &second, &before);
                let mut newer = second;
                newer.header.time += 1;
                remine(&mut newer);
                admit(n, std::slice::from_ref(&newer), params);
                let held = n
                    .peer_manager
                    .header_store()
                    .canonical_header_validation_context(&newer.header)?
                    .current_time;
                n.chainstate_mut().admit_canonical_body_reorg(
                    &disconnect,
                    &[(first.header, rows[0].1), (newer.header.clone(), held)],
                )?;
                assert!(
                    n.chainstate()
                        .chainstate()
                        .check_live_work_source(&source)
                        .is_err()
                );
                assert!(
                    n.chainstate()
                        .chainstate()
                        .check_staged_reorg(&stage)
                        .is_err()
                );
                assert_unaccepted(n.chainstate(), store, &newer, &before);
                Ok(())
            })
            .expect("actual same-owner exact batch controls");
    });
}
