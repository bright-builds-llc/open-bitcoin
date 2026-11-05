// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/blockfilter.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use crate::chainstate::{BasicFilterStartupMode, validation_history::tests::phase167 as genuine};
use crate::storage::{coins_codec, snapshot_codec};
use open_bitcoin_core::{
    chainstate::{Coin, FlushMode, FlushPolicyTime, filter_index::catch_up::TurnWork},
    consensus::{CanonicalChain, ScriptVerifyFlags, transaction_txid, transaction_wtxid},
    primitives::{Amount, OutPoint, ScriptBuf, TransactionOutput},
};

fn wide_body() -> Block {
    let genesis = CanonicalChain::Regtest.spec().genesis_block();
    let mut block = genuine::child(&genesis.header, 1);
    block.transactions[0].outputs = [
        vec![0x51; 10_001],
        vec![0x51; 10_000],
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
    genuine::remine(&mut block);
    block
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn write_body(store: &FjallNodeStore, bytes: Vec<u8>) -> BlockHash {
    let hash = BlockHash::from_byte_array([0x33; 32]);
    store
        .write_raw_for_test(StorageNamespace::BlockIndex, &block_key(hash), bytes)
        .expect("raw body");
    hash
}

#[test]
fn phase167_serialized_body_durable_actual_scanner_exact_output_work() {
    genuine::with_mode(BasicFilterStartupMode::Enabled, |manager, store, _| {
        // Arrange
        let block = wide_body();
        let bytes = encode_block(&block).expect("wire body");
        let hash = store
            .save_block(&block, PersistMode::Sync)
            .expect("retained wide body");
        let mut maybe_reserved = None;
        // Act
        let (decoded, work) = store
            .maybe_basic_filter_turn_body(hash, bytes.len() as u64, true, |reserved| {
                maybe_reserved = Some(reserved);
                Ok(true)
            })
            .expect("actual scanner accepts wide output")
            .expect("admitted");
        // Assert: input scriptSig stays under reservation, outside output ledger.
        assert_eq!(decoded, block);
        assert_eq!(bytes.len(), 20_177);
        assert_eq!(
            work,
            TurnWork {
                body_bytes: 20_177,
                cloned_bytes: 20_177 * 32 + std::mem::size_of::<Block>() as u64,
                script_items: 4,
                script_bytes: 20_003,
                record_operations: 1,
                ..Default::default()
            }
        );
        assert_eq!(
            maybe_reserved,
            Some(TurnWork {
                body_bytes: work.body_bytes,
                cloned_bytes: work.cloned_bytes,
                script_items: 20_177 / 9,
                script_bytes: 20_177,
                ..Default::default()
            })
        );
        assert_eq!(
            hex(hash.as_bytes()),
            "bf1be087d4828ed89feedb87e7e8198a134f95acc8b8d9aac2652b9cbdd78958"
        );
        drop(manager);
    });
}

#[test]
fn phase167_serialized_body_durable_saved_body_exact_identity() {
    genuine::with_manager(|manager, store, _| {
        // Arrange
        let block = wide_body();
        let hash = store
            .save_block(&block, PersistMode::Sync)
            .expect("save complete body");
        // Act
        let loaded = store
            .load_block(hash)
            .expect("serialized load")
            .expect("body");
        // Assert
        assert_eq!(loaded, block);
        assert_eq!(
            encode_block(&loaded).expect("loaded wire"),
            encode_block(&block).expect("original wire")
        );
        let tx = &loaded.transactions[0];
        assert_eq!(
            hex(transaction_txid(tx).expect("txid").as_bytes()),
            "9e1fd39d9630e73d62bfb28e6d6ab4696183021c7893bb23c54ba23bed8f00b6"
        );
        assert_eq!(
            transaction_txid(tx).expect("txid").as_bytes(),
            transaction_wtxid(tx).expect("wtxid").as_bytes()
        );
        assert_eq!(
            loaded.header.merkle_root.as_bytes(),
            transaction_txid(tx).expect("coinbase root").as_bytes()
        );
        drop(manager);
    });
}

#[test]
fn phase167_serialized_body_durable_full_proof_basic_and_all_handle_reopen() {
    genuine::with_mode(BasicFilterStartupMode::Enabled, |manager, store, params| {
        // Arrange: actual selected acceptance, not a claimed snapshot.
        let root = store.datadir().to_path_buf();
        let block = wide_body();
        let hash = block_hash(&block.header);
        let txid = transaction_txid(&block.transactions[0]).expect("txid");
        let mut network = genuine::network(manager);
        genuine::admit(&mut network, std::slice::from_ref(&block), params);
        store.save_block(&block, PersistMode::Sync).expect("body");
        network
            .connect_local_block(&block, ScriptVerifyFlags::NONE, params)
            .expect("genuinely valid wide creation");
        network
            .persist_validation_header_snapshot()
            .expect("trusted headers");
        network
            .chainstate_mut()
            .flush_with_mode(
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(60_001),
                u64::MAX,
            )
            .expect("full checkpoint");
        let expected = network.chainstate_snapshot().expect("accepted full state");
        assert!(!expected.utxos.contains_key(&OutPoint { txid, vout: 0 }));
        assert!(expected.utxos.contains_key(&OutPoint { txid, vout: 1 }));
        assert!(expected.utxos.contains_key(&OutPoint { txid, vout: 2 }));
        assert!(!expected.utxos.contains_key(&OutPoint { txid, vout: 3 }));
        assert!(expected.undo_by_block[&hash].transactions.is_empty());
        let network = crate::ManagedNetworkHandle::new(network);
        network
            .initialize_basic_filter_index_owner()
            .expect("real BASIC owner");
        for _ in 0..3 {
            let turn = network
                .drive_basic_filter_index_turn()
                .expect("real budgeted turn accepts wide output");
            if turn.maybe_accepted_lag == Some(0) {
                break;
            }
        }
        let filter = store
            .load_basic_filter_record(hash)
            .expect("filter")
            .expect("published");
        // Independently serialized/mined with Bun and pinned Python SipHash oracle.
        assert_eq!(
            filter.encoded_bytes(),
            &[0x02, 0x05, 0x8f, 0x7d, 0x16, 0, 0x40]
        );
        let progress = network
            .maybe_basic_index_progress()
            .expect("progress")
            .expect("enabled");
        assert_eq!(
            progress
                .maybe_safe_durable_endpoint()
                .expect("durable")
                .height(),
            1
        );
        let headers = network.header_entries().expect("all headers");
        let validation = store
            .maybe_validation_history_record(hash)
            .expect("validation");
        let basic = store.maybe_basic_filter_state().expect("BASIC state");
        drop(network);
        drop(store);
        // Act: no handles survive the safe copied-child / immediate-original proof.
        let measured = measure_detached_rows_for_test(&root).expect("complete measured source");
        assert!(measured.manifest_equal && measured.authority_max <= 65_536);
        assert!(measured.physical_entries <= 4096 && measured.physical_bytes <= 536_870_912);
        let runtime = crate::sync::DurableSyncRuntime::reopen_canonical_history_configured(
            &root,
            crate::sync::SyncRuntimeConfig {
                network: crate::sync::SyncNetwork::Regtest,
                ..Default::default()
            },
            BasicFilterStartupMode::PreserveSaved,
        )
        .expect("complete copied and original reproof");
        // Assert
        let (stage, child) = FjallNodeStore::history_worker_observation_for_test();
        assert_eq!(stage, Some(2));
        assert!(child < std::time::Duration::from_secs(30));
        assert_eq!(
            runtime
                .network_handle()
                .chainstate_snapshot()
                .expect("reopened full core"),
            expected
        );
        assert_eq!(
            runtime
                .network_handle()
                .header_entries()
                .expect("reopened headers"),
            headers
        );
        assert_eq!(
            runtime.store().load_block(hash).expect("reopened body"),
            Some(block)
        );
        assert_eq!(
            runtime
                .store()
                .load_basic_filter_record(hash)
                .expect("reopened filter"),
            Some(filter)
        );
        assert_eq!(
            runtime
                .store()
                .maybe_basic_filter_state()
                .expect("same state"),
            basic
        );
        assert_eq!(
            runtime
                .store()
                .maybe_validation_history_record(hash)
                .expect("same provenance"),
            validation
        );
        eprintln!("WIDE_BODY_PROOF child={child:?} {measured:?} complete_original_reproof=true");
        drop(runtime);
    });
}

#[test]
fn phase167_serialized_body_durable_pre_scan_native_remaining_and_all_work_reservations() {
    genuine::with_manager(|manager, store, _| {
        // Arrange
        let bytes = encode_block(&wide_body()).expect("body");
        let hash = write_body(&store, bytes.clone());
        // Act / Assert: every reservation counter remains enforceable before scanning.
        for field in 0..4 {
            let result = store
                .maybe_basic_filter_turn_body(hash, bytes.len() as u64, true, |work| {
                    let mut limit = work;
                    match field {
                        0 => limit.body_bytes -= 1,
                        1 => limit.cloned_bytes -= 1,
                        2 => limit.script_items -= 1,
                        _ => limit.script_bytes -= 1,
                    }
                    Ok(work.fits(limit))
                })
                .expect("pre-scan refusal is yield");
            assert!(result.is_none(), "counter {field}");
        }
        assert!(
            store
                .maybe_basic_filter_turn_body(hash, 1, false, |_| panic!(
                    "remaining bound precedes admission"
                ))
                .expect("remaining yield")
                .is_none()
        );
        assert!(
            store
                .maybe_basic_filter_turn_body(hash, 1, true, |_| panic!(
                    "singleton bound precedes admission"
                ))
                .expect_err("singleton bound")
                .to_string()
                .contains("singleton absolute")
        );
        let oversized = write_body(&store, vec![0; 4_000_001]);
        assert!(
            store
                .maybe_basic_filter_turn_body(oversized, u64::MAX, true, |_| panic!(
                    "native length before admission"
                ))
                .expect_err("absolute wire")
                .to_string()
                .contains("absolute wire")
        );
        let malformed = write_body(&store, vec![0]);
        assert!(
            store
                .maybe_basic_filter_turn_body(malformed, 1, true, |_| Ok(false))
                .expect("no scan before admission")
                .is_none()
        );
        drop(manager);
    });
}

fn simple_wire() -> Vec<u8> {
    let genesis = CanonicalChain::Regtest.spec().genesis_block();
    encode_block(&genuine::child(&genesis.header, 1)).expect("small canonical body")
}

#[test]
fn phase167_serialized_body_durable_malformed_framing_counts_amounts_and_suffix_refuse() {
    genuine::with_manager(|manager, store, _| {
        // Arrange: fixed small fixture offsets follow its independent wire layout.
        let valid = simple_wire();
        let mut cases: Vec<(&str, Vec<u8>)> = Vec::new();
        for length in [0, 79, 80, 85, 124, 133, valid.len() - 1] {
            cases.push(("truncated", valid[..length].to_vec()));
        }
        let mut suffix = valid.clone();
        suffix.push(0);
        cases.push(("suffix", suffix));
        let mut noncanonical = valid.clone();
        noncanonical.splice(80..81, [0xfd, 1, 0]);
        cases.push(("noncanonical", noncanonical));
        for count in [0x0200_0001u64, u64::MAX] {
            let mut oversized = valid[..80].to_vec();
            if count <= u64::from(u32::MAX) {
                oversized.push(0xfe);
                oversized.extend((count as u32).to_le_bytes());
            } else {
                oversized.push(0xff);
                oversized.extend(count.to_le_bytes());
            }
            cases.push(("size/overflow", oversized));
        }
        let mut envelope = valid.clone();
        envelope[80] = 252;
        cases.push(("count envelope", envelope));
        let mut script_frame = valid.clone();
        script_frame[122] = 252;
        cases.push(("script frame", script_frame));
        for amount in [-1i64, 2_100_000_000_000_001, i64::MAX] {
            let mut invalid = valid.clone();
            invalid[130..138].copy_from_slice(&amount.to_le_bytes());
            cases.push(("amount", invalid));
        }
        let mut witness_flag = valid.clone();
        witness_flag.splice(85..86, [0, 2, 1]);
        cases.push(("witness flag", witness_flag));
        // Act / Assert: strict load and real turn must both refuse each record.
        for (name, bytes) in cases {
            let hash = write_body(&store, bytes);
            assert!(store.load_block(hash).is_err(), "load {name}");
            let error = store
                .maybe_basic_filter_turn_body(hash, u64::MAX, true, |_| Ok(true))
                .expect_err("turn must refuse malformed body");
            if name == "size/overflow" {
                assert!(error.to_string().contains("count bound"), "{error}");
            }
        }
        drop(manager);
    });
}

#[test]
fn phase167_serialized_body_durable_coin_and_undo_decoders_stay_bounded() {
    // Arrange: deliberately corrupt DTOs never become canonical Coins.
    let output = wide_body().transactions.remove(0).outputs.remove(0);
    let coin = Coin {
        output,
        is_coinbase: true,
        created_height: 1,
        created_median_time_past: 0,
    };
    let bytes = coins_codec::encode_coin_value(&coin).expect("encode untrusted raw record");
    let mut truncated = bytes.clone();
    truncated.pop();
    // Act / Assert
    assert!(coins_codec::decode_coin_value(&bytes).is_err());
    assert!(coins_codec::decode_coin_value(&truncated).is_err());
    let undo = serde_json::json!({"schema_version":2,"payload":{"transactions":[{"restored_inputs":[{"output":{"value_sats":1,"script_pubkey":vec![0x51;10_001]},"is_coinbase":true,"created_height":1,"created_median_time_past":0}]}],"overwrites":[]}});
    assert!(
        snapshot_codec::decode_block_undo(&serde_json::to_vec(&undo).expect("raw undo")).is_err()
    );
    let mut bounded = coin.clone();
    bounded.output.value = Amount::from_sats(1).expect("bounded undo amount");
    bounded.output.script_pubkey = ScriptBuf::from_bytes(vec![0x51; 10_000]).expect("bounded Coin");
    assert_eq!(
        coins_codec::decode_coin_value(
            &coins_codec::encode_coin_value(&bounded).expect("encode bounded Coin")
        )
        .expect("decode bounded Coin"),
        bounded
    );
    let mut bounded_undo = undo;
    bounded_undo["payload"]["transactions"][0]["restored_inputs"][0]["output"]["script_pubkey"] =
        serde_json::json!(vec![0x51; 10_000]);
    let loaded = snapshot_codec::decode_block_undo(
        &serde_json::to_vec(&bounded_undo).expect("bounded undo"),
    )
    .expect("10000 byte undo remains valid");
    assert_eq!(loaded.transactions[0].restored_inputs[0], bounded);
}
