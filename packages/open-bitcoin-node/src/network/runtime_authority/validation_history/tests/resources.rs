// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txdb.cpp

use super::*;

#[test]
fn phase167_local_submission_resource_normalized_body_exceeds_supported_raw_boundary() {
    use open_bitcoin_core::primitives::{Amount, ScriptBuf, TransactionOutput};
    for mode in [
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        // Arrange: the valid base body fits; the earned reserve increases actual stored bytes.
        let (path, runtime, command) = runtime(mode);
        let mut block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        block.transactions[0].outputs[0].script_pubkey =
            ScriptBuf::from_serialized_bytes(vec![0x51; 65344]);
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
        remine(&mut block);
        let raw_bytes = open_bitcoin_core::codec::encode_block(&block)
            .expect("actual raw body")
            .len();
        assert_eq!(raw_bytes, 65536);
        let before = stable_state(&command);
        let maybe_expected_anchor = (mode == BasicFilterStartupMode::Enabled)
            .then(|| block_hash(&CanonicalChain::Regtest.spec().genesis_header()));
        assert_eq!(maybe_basic_anchor(&command), maybe_expected_anchor);
        let mempool = command.network.mempool_info().expect("mempool");
        let incarnation = command.incarnation;
        // Act
        let error = command
            .submit(wire(&block))
            .expect_err("normalized row before absorption");
        // Assert
        assert_eq!(error.stage, "prepared native resource projection");
        assert_eq!(stable_state(&command), before);
        assert_eq!(
            command.network.mempool_info().expect("unchanged mempool"),
            mempool
        );
        let hash = block_hash(&block.header);
        let normalized = command
            .network
            .read(|network| network.blocks_by_hash.get(&hash).cloned())
            .expect("actual bound body")
            .expect("retained normalized body");
        assert!(
            command
                .network
                .maybe_body_eligibility(hash)
                .expect("actual volatile registry")
                .is_some_and(|facts| facts.available && !facts.failed)
        );
        assert_eq!(
            normalized.transactions[0].inputs[0].witness.stack(),
            &[vec![0; 32]]
        );
        let normalized_bytes = open_bitcoin_core::codec::encode_block(&normalized)
            .expect("actual normalized encoding")
            .len();
        assert!(normalized_bytes > 65536);
        eprintln!(
            "NORMALIZED_RESOURCE mode={mode:?} actual_raw={raw_bytes} actual_normalized={normalized_bytes}"
        );
        assert_eq!(
            command.store.load_block(hash).expect("no saved candidate"),
            None
        );
        assert!(
            command
                .network
                .read(|network| network
                    .chainstate()
                    .maybe_pending_validation_identity(hash)
                    .is_none())
                .expect("no accepted receipt")
        );
        assert_eq!(
            command
                .store
                .maybe_validation_history_record(hash)
                .expect("record")
                .expect("real KnownOnly")
                .status(),
            crate::storage::validation_history::StoredValidationStatus::KnownOnly
        );
        let rows = native_rows(&command.store);
        let maybe_genesis_record = command
            .store
            .load_basic_filter_record(block_hash(&CanonicalChain::Regtest.spec().genesis_header()))
            .expect("full genesis BASIC");
        drop(error);
        let (runtime, command) = reopen(&path, runtime, command, mode);
        assert_ne!(command.incarnation, incarnation);
        assert_eq!(stable_state(&command), before);
        assert_eq!(maybe_basic_anchor(&command), maybe_expected_anchor);
        assert_eq!(native_rows(&command.store), rows);
        assert_eq!(
            command
                .store
                .load_basic_filter_record(block_hash(
                    &CanonicalChain::Regtest.spec().genesis_header()
                ))
                .expect("full reopened BASIC"),
            maybe_genesis_record
        );
        accepted(
            command
                .submit(wire(&child(
                    &CanonicalChain::Regtest.spec().genesis_header(),
                    1,
                )))
                .expect("supported valid next command"),
        );
        close(path, runtime, command);
    }
}

#[test]
fn phase167_local_submission_resource_header_and_knownonly_near_full_inventory_preserve_safe_source()
 {
    for full in [false, true] {
        // Arrange
        let (path, runtime, command) = runtime(BasicFilterStartupMode::Disabled);
        let root = CanonicalChain::Regtest.spec().genesis_header();
        let accepted_block = child(&root, 1);
        accepted(
            command
                .submit(wire(&accepted_block))
                .expect("actual accepted parent"),
        );
        fill_inventory(&command.store, if full { 4096 } else { 4095 });
        let before = accepted_state(&command);
        let rows = native_rows(&command.store);
        let physical = manifest(&path);
        let mut block = if full {
            child(&root, 1)
        } else {
            child(&accepted_block.header, 2)
        };
        if full {
            block.header.time += 100;
            remine(&mut block);
        }
        // Act
        let error = command
            .submit(wire(&block))
            .expect_err("actual native resource refusal");
        // Assert
        assert_eq!(accepted_state(&command), before);
        assert_eq!(
            error.stage,
            if full {
                "early native resource projection"
            } else {
                "prepared native resource projection"
            }
        );
        if full {
            assert_eq!(native_rows(&command.store), rows);
            assert_eq!(manifest(&path), physical);
        } else {
            let after = native_rows(&command.store);
            assert_eq!(units(&after), 4096);
            for name in [
                "wallet",
                "metrics",
                "mempool",
                "runtime",
                "coins",
                "chainstate",
            ] {
                assert_eq!(after[name], rows[name]);
            }
            assert!(
                command
                    .network
                    .maybe_body_eligibility(block_hash(&block.header))
                    .expect("volatile facts")
                    .is_some_and(|facts| !facts.failed)
            );
        }
        drop(error);
        drop(command);
        drop(runtime);
        let (runtime, maybe_command) = DurableSyncRuntime::open_canonical_submission_configured(
            &path,
            config(),
            BasicFilterStartupMode::Disabled,
        )
        .expect("old accepted source safely reopens");
        assert_eq!(
            accepted_state(maybe_command.as_ref().expect("command")),
            before
        );
        close(path, runtime, maybe_command);
    }
}

#[test]
fn phase167_local_submission_wide_body_normalized_commitment_full_reopen_and_next() {
    use open_bitcoin_core::primitives::{Amount, ScriptBuf, TransactionOutput};
    for mode in [
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        // Arrange
        let (path, runtime, command) = runtime(mode);
        let mut block = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
        block.transactions[0].outputs = [
            vec![0x51; 10001],
            vec![0x51; 10000],
            vec![],
            vec![0x6a, 0x51],
        ]
        .into_iter()
        .enumerate()
        .map(|(index, bytes)| TransactionOutput {
            value: Amount::from_sats(if index == 0 { 5_000_000_000 } else { 0 }).expect("subsidy"),
            script_pubkey: ScriptBuf::from_serialized_bytes(bytes),
        })
        .collect();
        remine(&mut block);
        let hash = block_hash(&block.header);
        // Act
        accepted(
            command
                .submit(wire(&block))
                .expect("wide command acceptance"),
        );
        // Assert: independent33 wire/oracle identities, exact body, distinct Coins/BASIC.
        assert_eq!(
            hash.to_byte_array(),
            [
                0xbf, 0x1b, 0xe0, 0x87, 0xd4, 0x82, 0x8e, 0xd8, 0x9f, 0xee, 0xdb, 0x87, 0xe7, 0xe8,
                0x19, 0x8a, 0x13, 0x4f, 0x95, 0xac, 0xc8, 0xb8, 0xd9, 0xaa, 0xc2, 0x65, 0x2b, 0x9c,
                0xbd, 0xd7, 0x89, 0x58
            ]
        );
        assert_eq!(
            command.store.load_block(hash).expect("body"),
            Some(block.clone())
        );
        let outpoint = open_bitcoin_core::primitives::OutPoint {
            txid: open_bitcoin_core::consensus::transaction_txid(&block.transactions[0])
                .expect("txid"),
            vout: 0,
        };
        assert!(
            !command
                .store
                .coins_view()
                .have_coin(&outpoint)
                .expect("Coins")
        );
        assert!(
            command
                .store
                .load_undo(hash)
                .expect("undo")
                .expect("present")
                .transactions
                .is_empty()
        );
        if mode == BasicFilterStartupMode::Enabled {
            match command
                .network
                .basic_filter_query(hash)
                .expect("actual BASIC")
            {
                crate::network::runtime_authority::filter_index::BasicFilterQuery::Found(
                    record,
                ) => assert_eq!(record.encoded_bytes(), &[2, 5, 0x8f, 0x7d, 0x16, 0, 0x40]),
                other => panic!("not actual BASIC: {other:?}"),
            }
        }
        let before_chain = command
            .network
            .chainstate_snapshot()
            .expect("complete state");
        let maybe_before_basic = command
            .store
            .maybe_basic_filter_state()
            .expect("BASIC state");
        let before_headers = command.network.header_entries().expect("all headers");
        drop(command);
        drop(runtime);
        let (runtime, maybe_command) =
            DurableSyncRuntime::open_canonical_submission_configured(&path, config(), mode)
                .expect("complete copied/original safe proof");
        let command = maybe_command.expect("same actual owner");
        assert_eq!(
            command
                .network
                .chainstate_snapshot()
                .expect("complete reopened state"),
            before_chain
        );
        assert_eq!(
            command
                .store
                .maybe_basic_filter_state()
                .expect("reproved BASIC"),
            maybe_before_basic
        );
        assert_eq!(
            command.network.header_entries().expect("earned headers"),
            before_headers
        );
        assert_eq!(
            command.store.load_block(hash).expect("exact reopened body"),
            Some(block.clone())
        );
        let mut next = child(&block.header, 2);
        let commitment = open_bitcoin_core::consensus::crypto::double_sha256(&[0; 64]);
        next.transactions[0].outputs.push(TransactionOutput {
            value: Amount::ZERO,
            script_pubkey: ScriptBuf::from_serialized_bytes(
                [
                    vec![0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed],
                    commitment.to_vec(),
                ]
                .concat(),
            ),
        });
        remine(&mut next);
        let submitted = next.clone();
        accepted(
            command
                .submit(wire(&next))
                .expect("next command with absent reserve"),
        );
        let normalized = command
            .store
            .load_block(block_hash(&next.header))
            .expect("normalized")
            .expect("present");
        assert_eq!(normalized.header, submitted.header);
        assert_eq!(
            normalized.transactions[0].inputs[0].witness.stack(),
            &[vec![0; 32]]
        );
        assert_eq!(
            open_bitcoin_core::consensus::transaction_txid(&normalized.transactions[0]),
            open_bitcoin_core::consensus::transaction_txid(&submitted.transactions[0])
        );
        close(path, runtime, command);
    }
}

#[test]
fn phase167_local_submission_resource_actual_mature_spend_undo_exceeds_body_row_bound() {
    use open_bitcoin_core::primitives::{
        Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
        TransactionOutput,
    };
    for mode in [
        BasicFilterStartupMode::Disabled,
        BasicFilterStartupMode::Enabled,
    ] {
        // Arrange: every funding Coin and maturity step is earned by the real command.
        let (path, runtime, command) = runtime(mode);
        let mut parent = CanonicalChain::Regtest.spec().genesis_header();
        let mut maybe_funding = None;
        for height in 1..=100 {
            let mut block = child(&parent, height);
            if height == 1 {
                block.transactions[0].outputs = (0..768)
                    .map(|index| TransactionOutput {
                        value: if index == 0 {
                            Amount::from_sats(5_000_000_000).expect("subsidy")
                        } else {
                            Amount::ZERO
                        },
                        script_pubkey: ScriptBuf::from_serialized_bytes(vec![0x51]),
                    })
                    .collect();
                remine(&mut block);
                maybe_funding = Some(
                    open_bitcoin_core::consensus::transaction_txid(&block.transactions[0])
                        .expect("actual funding txid"),
                );
            }
            accepted(
                command
                    .submit(wire(&block))
                    .expect("actual mature funding history"),
            );
            if height == 1 {
                let hash = block_hash(&block.header);
                let (_, encoded) = FjallNodeStore::submission_undo_row(
                    hash,
                    &command
                        .store
                        .load_undo(hash)
                        .expect("persisted funding undo")
                        .expect("accepted funding"),
                )
                .expect("actual funding undo encoder");
                assert!(encoded.len() <= 65536);
                eprintln!("FUNDING_UNDO mode={mode:?} actual_undo={}", encoded.len());
            }
            parent = block.header;
        }
        let funding = maybe_funding.expect("real mature funding");
        let spending = Transaction {
            version: 2,
            lock_time: 0,
            inputs: (0..768)
                .map(|vout| TransactionInput {
                    previous_output: OutPoint {
                        txid: funding,
                        vout,
                    },
                    script_sig: ScriptBuf::default(),
                    sequence: u32::MAX,
                    witness: ScriptWitness::default(),
                })
                .collect(),
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(5_000_000_000).expect("conserved input value"),
                script_pubkey: ScriptBuf::from_serialized_bytes(vec![0x51]),
            }],
        };
        let mut block = child(&parent, 101);
        block.transactions.push(spending);
        remine(&mut block);
        let mut invalid = block.clone();
        invalid.transactions[1].inputs[0].script_sig = ScriptBuf::from_serialized_bytes(vec![0x6a]);
        remine(&mut invalid);
        reject(
            &command,
            wire(&invalid),
            "mandatory-script-verify-flag-failed",
        );
        assert!(
            command
                .network
                .maybe_body_eligibility(block_hash(&invalid.header))
                .expect("actual failed script")
                .expect("checked body")
                .failed
        );
        let before = accepted_state(&command);
        let snapshot = command
            .network
            .chainstate_snapshot()
            .expect("complete accepted state");
        let mempool = command.network.mempool_info().expect("mempool");
        command
            .store
            .inspect_submission_inventory()
            .expect("entire actual baseline supported");
        let baseline_units = units(&native_rows(&command.store));
        assert!(baseline_units < 4096);
        let body_bytes = open_bitcoin_core::codec::encode_block(&block)
            .expect("exact complete body")
            .len();
        assert!(body_bytes < 65536);
        // Act
        let error = command
            .submit(wire(&block))
            .expect_err("undo-aware pre-absorption policy");
        let (undo_bytes, projected_coins, complete_counts, basic_bytes) = command
            .network
            .mutate(|network| {
                let hash = block_hash(&block.header);
                let headers = network.peer_manager.header_store();
                let history = headers
                    .canonical_header_history(hash)
                    .expect("actual earned history");
                let context = headers
                    .canonical_header_validation_context(&block.header)
                    .expect("held clock");
                let authorization = network
                    .authorize_connect(&block, command.params)
                    .expect("same actual retained body preference");
                let manager = network.chainstate();
                let prepared = manager
                    .prepare_authorized_connect(
                        authorization,
                        &block,
                        context.current_time,
                        history.rules().script_flags(),
                        command.params,
                    )
                    .expect("same actual genuine stage");
                let projected = manager
                    .project_prepared_submission(&prepared)
                    .expect("read-only staged facts");
                let (_, undo) = FjallNodeStore::submission_undo_row(
                    hash,
                    projected
                        .undo_by_block
                        .get(&hash)
                        .expect("actual complete undo"),
                )
                .expect("actual undo encoder");
                let basic = match manager
                    .maybe_prepared_submission_basic_inputs(&prepared)
                    .expect("actual captured inputs")
                {
                    Some(inputs) => {
                        let previous = manager
                            .maybe_basic_index_progress()
                            .expect("actual BASIC")
                            .maybe_safe_durable_endpoint()
                            .expect("real predecessor");
                        let record = crate::storage::filter_index::StoredFilterRecord::generate(
                            &inputs,
                            prepared.position(),
                            Some(&previous),
                        )
                        .expect("independent actual BASIC data");
                        crate::storage::filter_index::encode_record(&record).len()
                    }
                    None => 0,
                };
                (
                    undo.len(),
                    projected.utxos.len(),
                    projected
                        .maybe_confirmed_txid_counts
                        .expect("complete counts")
                        .len(),
                    basic,
                )
            })
            .expect("actual authority data observation");
        // Assert: the real native reader budget passed; the authority-row gate refused undo.
        assert_eq!(error.stage, "prepared native resource projection");
        assert!(matches!(
            std::error::Error::source(&error)
                .and_then(|source| source.downcast_ref::<crate::StorageError>()),
            Some(crate::StorageError::Corruption {
                namespace: crate::StorageNamespace::BlockIndex,
                ..
            })
        ));
        assert!(undo_bytes > 65536);
        assert_eq!(projected_coins, 101);
        assert_eq!(complete_counts, 102);
        if mode == BasicFilterStartupMode::Enabled {
            assert!(basic_bytes > 0 && basic_bytes < 65536);
        }
        eprintln!(
            "UNDO_RESOURCE mode={mode:?} actual_body={body_bytes} actual_undo={undo_bytes} baseline_units={baseline_units} projected_coins={projected_coins} actual_basic_row={basic_bytes}"
        );
        assert_eq!(accepted_state(&command), before);
        assert_eq!(
            command.network.mempool_info().expect("unchanged mempool"),
            mempool
        );
        let hash = block_hash(&block.header);
        assert_eq!(
            command
                .store
                .load_block(hash)
                .expect("no persisted candidate"),
            None
        );
        assert_eq!(
            command
                .store
                .maybe_validation_history_record(hash)
                .expect("record")
                .expect("actual KnownOnly")
                .status(),
            crate::storage::validation_history::StoredValidationStatus::KnownOnly
        );
        assert!(
            command
                .network
                .maybe_body_eligibility(hash)
                .expect("volatile retained body")
                .is_some_and(|facts| facts.available && !facts.failed)
        );
        assert!(
            !command
                .network
                .read(|network| network
                    .chainstate()
                    .maybe_pending_validation_identity(hash)
                    .is_some())
                .expect("no acceptance receipt")
        );
        drop(error);
        drop(command);
        drop(runtime);
        let (runtime, maybe_command) =
            DurableSyncRuntime::open_canonical_submission_configured(&path, config(), mode)
                .expect("all-drop old accepted history reopens");
        let command = maybe_command.expect("command");
        assert_eq!(
            command
                .network
                .chainstate_snapshot()
                .expect("complete reopened state"),
            snapshot
        );
        accepted(
            command
                .submit(wire(&child(&parent, 101)))
                .expect("supported valid next block"),
        );
        close(path, runtime, command);
    }
}
