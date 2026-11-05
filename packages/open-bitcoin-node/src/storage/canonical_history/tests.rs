// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/pow.cpp
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp

use super::records::{CanonicalHistoryRecords, Namespaces};
use super::with_rows;
use crate::chainstate::BasicFilterStartupMode;
use crate::storage::canonical_bootstrap::validate_history_records;
use open_bitcoin_core::{
    consensus::{CanonicalChain, block_hash, block_merkle_root, check_block_header},
    primitives::{
        Amount, Block, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
        TransactionInput, TransactionOutput,
    },
};

pub(super) fn child(parent: &BlockHeader, height: u32) -> Block {
    let mut number = Vec::new();
    let mut value = height;
    while value > 0 {
        number.push(value as u8);
        value >>= 8;
    }
    if number.last().is_some_and(|byte| byte & 0x80 != 0) {
        number.push(0);
    }
    let mut height_script = if height <= 16 {
        vec![0x50 + height as u8]
    } else {
        let mut script = vec![number.len() as u8];
        script.extend(number);
        script
    };
    height_script.push(0);
    let mut block = Block {
        header: BlockHeader {
            version: 4,
            previous_block_hash: block_hash(parent),
            merkle_root: Default::default(),
            time: parent.time + 1,
            bits: parent.bits,
            nonce: 0,
        },
        transactions: vec![Transaction {
            version: 1,
            lock_time: height,
            inputs: vec![TransactionInput {
                previous_output: OutPoint::null(),
                script_sig: ScriptBuf::from_bytes(height_script).expect("height script"),
                sequence: u32::MAX,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(5_000_000_000 >> (height / 150))
                    .expect("selected subsidy"),
                script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("true script"),
            }],
        }],
    };
    block.header.merkle_root = block_merkle_root(&block.transactions).expect("merkle").0;
    block.header.nonce = (0..100_000)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("bounded regtest mining");
    block
}

#[test]
fn phase167_history_records_genuine_301_rows_fit_existing_authority_bound() {
    with_rows(BasicFilterStartupMode::Disabled, |rows, expected| {
        // Act
        let parsed = CanonicalHistoryRecords::parse_rows(&rows).expect("complete bounded data");
        // Assert: retain complete-state purpose of the original aggregate RED.
        assert_eq!(parsed.snapshot, *expected);
        assert_eq!(parsed.snapshot.active_chain.len(), 302);
        assert_eq!(parsed.snapshot.utxos.len(), 301);
        assert_eq!(parsed.snapshot.undo_by_block.len(), 301);
        let proof = validate_history_records(CanonicalChain::Regtest, &parsed)
            .expect("independently replayed Disabled history");
        let (genesis, headers, replay) = proof.into_parts(&parsed).expect("same complete source");
        assert_eq!(
            genesis.hash(),
            CanonicalChain::Regtest.spec().genesis_hash()
        );
        assert_eq!(headers.entries().count(), 302);
        assert_eq!(
            replay.admission_snapshot().expect("complete replay"),
            *expected
        );
        for name in ["headers", "block_index", "chainstate"] {
            assert!(rows[name].values().all(|bytes| bytes.len() <= 65536));
        }
    });
}

#[test]
fn phase167_history_proof_genuine_301_requires_independent_replay() {
    with_rows(BasicFilterStartupMode::Enabled, |rows, expected| {
        // Arrange
        let records = CanonicalHistoryRecords::parse_rows(&rows).expect("bounded records");
        // Act / Assert
        let started = std::time::Instant::now();
        let proof = validate_history_records(CanonicalChain::Regtest, &records)
            .expect("independently replayed Enabled history");
        let (_, headers, replay) = proof.into_parts(&records).expect("exact source/generation");
        assert_eq!(headers.entries().count(), 302);
        assert_eq!(
            replay
                .admission_snapshot()
                .expect("complete effective replay"),
            *expected
        );
        eprintln!("HISTORY_REPLAY elapsed={:?}", started.elapsed());
        assert!(started.elapsed().as_secs_f64() < 30.0);
    });
}

fn refused(rows: &Namespaces) {
    if let Ok(parsed) = CanonicalHistoryRecords::parse_rows(rows) {
        assert!(validate_history_records(CanonicalChain::Regtest, &parsed).is_err());
    }
}

#[test]
fn phase167_history_proof_each_same_tip_authority_change_refuses() {
    with_rows(BasicFilterStartupMode::Disabled, |rows, _| {
        for case in [
            "header-work",
            "header-height",
            "header-parent",
            "index",
            "active-work",
            "active-mtp",
            "active-link",
            "coin-value",
            "coin-metadata",
            "missing-coin",
            "extra-coin",
            "missing-undo",
            "overwrite-data",
            "undo-mtp",
            "count-value",
            "count-absent",
            "count-missing",
            "count-extra",
            "best",
            "body",
            "missing-body",
            "status",
            "coverage",
            "extra-accepted",
        ] {
            // Arrange: parse the complete genuine checkpoint, retaining its tip.
            let mut parsed = CanonicalHistoryRecords::parse_rows(&rows).expect("complete original");
            let original_tip = parsed.snapshot.tip().expect("tip").clone();
            let hash = parsed.snapshot.active_chain[150].block_hash;
            match case {
                "header-work" => {
                    parsed.headers[1].chain_work =
                        open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO;
                    parsed.index = parsed.headers.clone();
                }
                "header-height" => {
                    parsed.headers[1].height += 1;
                    parsed.index = parsed.headers.clone();
                }
                "header-parent" => {
                    parsed.headers[1].header.previous_block_hash = Default::default();
                    parsed.index = parsed.headers.clone();
                }
                "index" => {
                    parsed.index.pop();
                }
                "active-work" => {
                    parsed.snapshot.active_chain[150].chain_work =
                        open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO
                }
                "active-mtp" => parsed.snapshot.active_chain[150].median_time_past += 1,
                "active-link" => {
                    parsed.snapshot.active_chain[150].header.previous_block_hash =
                        Default::default()
                }
                "coin-value" => {
                    parsed
                        .snapshot
                        .utxos
                        .values_mut()
                        .next()
                        .expect("coin")
                        .output
                        .value = Amount::ZERO
                }
                "coin-metadata" => {
                    parsed
                        .snapshot
                        .utxos
                        .values_mut()
                        .next()
                        .expect("coin")
                        .created_median_time_past += 1
                }
                "missing-coin" => {
                    let key = parsed.snapshot.utxos.keys().next().expect("coin").clone();
                    parsed.snapshot.utxos.remove(&key);
                }
                "extra-coin" => {
                    let coin = parsed.snapshot.utxos.values().next().expect("coin").clone();
                    parsed.snapshot.utxos.insert(OutPoint::null(), coin);
                }
                "missing-undo" => {
                    parsed.snapshot.undo_by_block.remove(&hash);
                }
                "overwrite-data" => parsed
                    .snapshot
                    .undo_by_block
                    .get_mut(&hash)
                    .expect("undo")
                    .overwrites
                    .clear(),
                "undo-mtp" => {
                    parsed
                        .snapshot
                        .undo_by_block
                        .get_mut(&hash)
                        .expect("undo")
                        .overwrites[0]
                        .creating_median_time_past += 1
                }
                "count-value" => {
                    *parsed
                        .snapshot
                        .maybe_confirmed_txid_counts
                        .as_mut()
                        .expect("map")
                        .values_mut()
                        .next()
                        .expect("count") += 1
                }
                "count-absent" => parsed.snapshot.maybe_confirmed_txid_counts = None,
                "count-missing" => {
                    let counts = parsed
                        .snapshot
                        .maybe_confirmed_txid_counts
                        .as_mut()
                        .expect("map");
                    let key = *counts.keys().next().expect("count");
                    counts.remove(&key);
                }
                "count-extra" => {
                    parsed
                        .snapshot
                        .maybe_confirmed_txid_counts
                        .as_mut()
                        .expect("map")
                        .insert(Default::default(), 1);
                }
                "best" => parsed.best_block = Default::default(),
                "body" => {
                    parsed.bodies.get_mut(&hash).expect("body").transactions[0].outputs[0].value =
                        Amount::ZERO
                }
                "missing-body" => {
                    parsed.bodies.remove(&hash);
                }
                "status" => {
                    use crate::storage::validation_history::*;
                    let id = parsed.validation[&hash].identity();
                    parsed.validation.insert(
                        hash,
                        ValidationHistoryRecord::new(id, StoredValidationStatus::KnownOnly),
                    );
                }
                "coverage" => {
                    parsed.coverage =
                        crate::storage::validation_history::ValidationCoverage::UnknownLegacy
                }
                "extra-accepted" => {
                    use crate::storage::validation_history::*;
                    let other = open_bitcoin_core::primitives::BlockHash::from_byte_array([7; 32]);
                    let id = BlockValidationIdentity::new(other, hash, 151).expect("identity");
                    parsed.validation.insert(
                        other,
                        ValidationHistoryRecord::new(id, StoredValidationStatus::ScriptsValid),
                    );
                }
                _ => panic!("unknown case"),
            }
            // Act / Assert: endpoint identity alone cannot authorize changed rows.
            assert_eq!(parsed.snapshot.tip(), Some(&original_tip), "{case}");
            assert!(
                validate_history_records(CanonicalChain::Regtest, &parsed).is_err(),
                "{case}"
            );
        }
    });
}

#[test]
fn phase167_history_records_raw_membership_bounds_and_generations_refuse() {
    with_rows(BasicFilterStartupMode::Disabled, |rows, _| {
        for case in [
            "namespace",
            "unknown-marker",
            "seeded-marker",
            "oversized",
            "path",
            "global-count",
            "stale-generation",
            "missing-fragment",
            "mixed-root",
            "missing-overwrite",
            "missing-counts",
            "head-marker",
            "missing-body",
            "body-merkle",
        ] {
            // Arrange
            let mut changed = rows.clone();
            match case {
                "namespace" => {
                    changed.insert("unknown".into(), Default::default());
                }
                "unknown-marker" | "seeded-marker" => {
                    changed
                        .get_mut("schema")
                        .expect("schema")
                        .insert(format!("{case}:v1").into_bytes(), vec![1]);
                }
                "oversized" => {
                    changed
                        .get_mut("schema")
                        .expect("schema")
                        .insert(b"schema_version".to_vec(), vec![0; 65537]);
                }
                "path" => {
                    changed
                        .get_mut("wallet")
                        .expect("wallet")
                        .insert(vec![b'a'; 4097], vec![1]);
                }
                "global-count" => {
                    let wallet = changed.get_mut("wallet").expect("wallet");
                    for i in 0..4096 {
                        wallet.insert(format!("wallet_snapshot:{i}").into_bytes(), vec![1]);
                    }
                }
                "stale-generation" => {
                    changed
                        .get_mut("headers")
                        .expect("headers")
                        .get_mut(b"snapshot:segments:v1".as_slice())
                        .expect("manifest")[22] ^= 1;
                }
                "missing-fragment" => {
                    let headers = changed.get_mut("headers").expect("headers");
                    let key = headers
                        .keys()
                        .find(|key| key.starts_with(b"snapshot:segment:"))
                        .expect("fragment")
                        .clone();
                    headers.remove(&key);
                }
                "mixed-root" => {
                    changed
                        .get_mut("headers")
                        .expect("headers")
                        .insert(b"snapshot".to_vec(), b"{}".to_vec());
                }
                "missing-counts" => {
                    let state = changed.get_mut("chainstate").expect("state");
                    state.retain(|key, _| !key.starts_with(b"chain_meta"));
                    let parsed = CanonicalHistoryRecords::parse_rows(&rows).expect("original");
                    let bytes = crate::storage::snapshot_codec::encode_chain_meta(
                        &parsed.snapshot.active_chain,
                        None,
                    )
                    .expect("unknown counts");
                    state.insert(b"chain_meta".to_vec(), bytes);
                }
                "missing-overwrite" => {
                    let state = changed.get_mut("chainstate").expect("state");
                    let key = state
                        .keys()
                        .find(|key| key.starts_with(b"undo:"))
                        .expect("undo")
                        .clone();
                    let mut value: serde_json::Value =
                        serde_json::from_slice(&state[&key]).expect("json");
                    value["payload"]
                        .as_object_mut()
                        .expect("versioned data")
                        .remove("overwrites");
                    state.insert(key, serde_json::to_vec(&value).expect("json"));
                }
                "head-marker" => {
                    changed
                        .get_mut("coins")
                        .expect("coins")
                        .insert(b"H".to_vec(), vec![0]);
                }
                "missing-body" => {
                    let index = changed.get_mut("block_index").expect("index");
                    let key = index
                        .keys()
                        .find(|key| key.starts_with(b"block:"))
                        .expect("body")
                        .clone();
                    index.remove(&key);
                }
                "body-merkle" => {
                    let parsed = CanonicalHistoryRecords::parse_rows(&rows).expect("original");
                    let hash = parsed.snapshot.active_chain[150].block_hash;
                    let mut block = parsed.bodies[&hash].clone();
                    block.transactions[0].outputs[0].value = Amount::ZERO;
                    changed.get_mut("block_index").expect("index").insert(
                        crate::storage::fjall_store::block_key(hash).into_bytes(),
                        open_bitcoin_core::codec::encode_block(&block).expect("body"),
                    );
                }
                _ => panic!("unknown case"),
            }
            // Act / Assert
            refused(&changed);
        }
    });
}

#[test]
fn phase167_history_proof_replacement_replays_every_retained_accepted_branch() {
    super::with_replacement_rows(|rows, expected| {
        // Arrange
        let parsed =
            CanonicalHistoryRecords::parse_rows(&rows).expect("complete physical replacement");
        assert_eq!(parsed.headers.len(), 305);
        assert_eq!(expected.active_chain.len(), 303);
        assert_eq!(expected.undo_by_block.len(), 304);
        assert_eq!(
            expected
                .maybe_confirmed_txid_counts
                .as_ref()
                .expect("exact map")
                .len(),
            303
        );
        // Act
        let started = std::time::Instant::now();
        let proof = validate_history_records(CanonicalChain::Regtest, &parsed)
            .expect("all accepted branches replay independently");
        let (_, headers, replay) = proof
            .into_parts(&parsed)
            .expect("all physical undo and source bound");
        let mut actual = replay
            .admission_snapshot()
            .expect("complete active earned core");
        // Assert: retained physical undo is separately proved, not inserted into core.
        assert_eq!(headers.entries().count(), 305);
        assert!(
            headers
                .entries()
                .all(|entry| headers.is_canonically_admitted(entry.block_hash))
        );
        assert!(replay.has_earned_work());
        assert_eq!(
            replay.maybe_canonical_chain(),
            Some(CanonicalChain::Regtest)
        );
        assert_eq!(actual.undo_by_block.len(), 302);
        for (hash, undo) in &actual.undo_by_block {
            assert_eq!(expected.undo_by_block.get(hash), Some(undo));
        }
        actual.undo_by_block = expected.undo_by_block.clone();
        assert_eq!(actual, *expected);
        eprintln!(
            "RETAINED_GRAPH known=305 active=303 retained_undo=304 replay={:?}",
            started.elapsed()
        );
        assert!(started.elapsed().as_secs_f64() < 30.0);
    });
}

#[test]
fn phase167_history_proof_orphan_claims_undo_body_and_basic_cannot_hide_at_same_tip() {
    super::with_replacement_rows(|rows, expected| {
        let base = CanonicalHistoryRecords::parse_rows(&rows).expect("complete replacement");
        let active: std::collections::HashSet<_> = expected
            .active_chain
            .iter()
            .map(|position| position.block_hash)
            .collect();
        let orphan = *base
            .validation
            .keys()
            .find(|hash| !active.contains(hash) && base.validation[hash].identity().height() == 301)
            .expect("formerly accepted orphan");
        assert!(
            base.basic
                .contains_key(crate::storage::filter_index::record_key(orphan).as_bytes())
        );
        for case in [
            "missing-body",
            "body",
            "missing-undo",
            "overwrite-data",
            "previous-coin",
            "status",
            "work",
            "basic",
        ] {
            // Arrange
            let mut parsed =
                CanonicalHistoryRecords::parse_rows(&rows).expect("same complete source");
            match case {
                "missing-body" => {
                    parsed.bodies.remove(&orphan);
                }
                "body" => {
                    parsed.bodies.get_mut(&orphan).expect("body").transactions[0].outputs[0].value =
                        Amount::ZERO
                }
                "missing-undo" => {
                    parsed.snapshot.undo_by_block.remove(&orphan);
                }
                "overwrite-data" => parsed
                    .snapshot
                    .undo_by_block
                    .get_mut(&orphan)
                    .expect("undo")
                    .overwrites
                    .clear(),
                "previous-coin" => {
                    parsed
                        .snapshot
                        .undo_by_block
                        .get_mut(&orphan)
                        .expect("undo")
                        .transactions[0]
                        .restored_inputs[0]
                        .created_median_time_past += 1
                }
                "status" => {
                    use crate::storage::validation_history::*;
                    let identity = parsed.validation[&orphan].identity();
                    parsed.validation.insert(
                        orphan,
                        ValidationHistoryRecord::new(identity, StoredValidationStatus::KnownOnly),
                    );
                }
                "work" => {
                    parsed
                        .headers
                        .iter_mut()
                        .find(|entry| entry.block_hash == orphan)
                        .expect("header")
                        .chain_work = open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO;
                    parsed.index = parsed.headers.clone();
                }
                "basic" => {
                    *parsed
                        .basic
                        .get_mut(crate::storage::filter_index::record_key(orphan).as_bytes())
                        .expect("retained BASIC")
                        .last_mut()
                        .expect("filter byte") ^= 1;
                }
                _ => panic!("unknown orphan mutation"),
            }
            // Act / Assert
            assert_eq!(
                parsed.snapshot.tip(),
                expected.tip(),
                "accepted tip remains exact: {case}"
            );
            assert!(
                validate_history_records(CanonicalChain::Regtest, &parsed).is_err(),
                "{case}"
            );
        }
    });
}
