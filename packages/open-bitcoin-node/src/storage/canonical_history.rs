// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/txdb.cpp

//! Bounded complete canonical history inspection.

use super::canonical_bootstrap::CanonicalBootstrapRefusal as Refusal;
use open_bitcoin_core::consensus::CanonicalChain;

mod proof;
pub(in crate::storage) mod records;
pub(in crate::storage) use proof::ProvedHistoryRecords;

/// Replay with the fresh shell observation captured by canonical_bootstrap.
pub(in crate::storage) fn validate_history_records(
    selected: CanonicalChain,
    records: &records::CanonicalHistoryRecords,
    now: i64,
) -> Result<ProvedHistoryRecords, Refusal> {
    proof::validate_history_records(selected, records, now)
}

#[cfg(test)]
mod tests;

#[cfg(test)]
use crate::chainstate::{BasicFilterStartupMode, validation_history::tests::phase167 as genuine};
#[cfg(test)]
use crate::storage::PersistMode;
#[cfg(test)]
use open_bitcoin_core::{
    consensus::ScriptVerifyFlags,
    primitives::{
        Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
        TransactionOutput,
    },
};
#[cfg(test)]
use records::{CanonicalHistoryRecords, Namespaces};
#[cfg(test)]
use tests::child;

#[cfg(test)]
pub(super) fn with_rows(
    mode: BasicFilterStartupMode,
    check: impl FnOnce(Namespaces, &open_bitcoin_core::chainstate::ChainstateSnapshot),
) {
    with_history(mode, 301, false, check);
}

#[cfg(test)]
pub(super) fn with_history(
    mode: BasicFilterStartupMode,
    descendants: u32,
    spend: bool,
    check: impl FnOnce(Namespaces, &open_bitcoin_core::chainstate::ChainstateSnapshot),
) {
    with_checkpoint(mode, descendants, spend, false, check);
}

#[cfg(test)]
pub(super) fn with_replacement_rows(
    check: impl FnOnce(Namespaces, &open_bitcoin_core::chainstate::ChainstateSnapshot),
) {
    with_checkpoint(BasicFilterStartupMode::Enabled, 301, true, true, check);
}

#[cfg(test)]
fn with_checkpoint(
    mode: BasicFilterStartupMode,
    descendants: u32,
    spend: bool,
    replacement: bool,
    check: impl FnOnce(Namespaces, &open_bitcoin_core::chainstate::ChainstateSnapshot),
) {
    genuine::with_mode(mode, |manager, store, params| {
        let root = store.datadir().to_path_buf();
        let mut network = genuine::network(manager);
        let mut parent = network.maybe_chain_tip().expect("genesis").header;
        let mut maybe_first_txid = None;
        for height in 1..=descendants {
            let mut block = child(&parent, height);
            if height == 1 {
                maybe_first_txid = Some(
                    open_bitcoin_core::consensus::transaction_txid(&block.transactions[0])
                        .expect("first txid"),
                );
            }
            if spend && height == 301 {
                block.transactions.push(Transaction {
                    version: 2,
                    lock_time: 0,
                    inputs: vec![TransactionInput {
                        previous_output: OutPoint {
                            txid: maybe_first_txid.expect("mature coinbase"),
                            vout: 0,
                        },
                        script_sig: ScriptBuf::from_bytes(vec![]).expect("empty input script"),
                        sequence: u32::MAX,
                        witness: ScriptWitness::default(),
                    }],
                    outputs: vec![TransactionOutput {
                        value: Amount::from_sats(4_999_999_999).expect("one sat fee"),
                        script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("true output"),
                    }],
                });
                block.transactions[0].outputs[0].value =
                    Amount::from_sats(1_250_000_001).expect("selected subsidy plus fee");
                genuine::remine(&mut block);
            }
            genuine::admit(&mut network, std::slice::from_ref(&block), params);
            store
                .save_block(&block, PersistMode::Sync)
                .expect("actual retained body");
            network
                .connect_local_block(&block, ScriptVerifyFlags::NONE, params)
                .expect("actual managed acceptance");
            parent = block.header;
        }
        network
            .persist_validation_header_snapshot()
            .expect("paired trusted checkpoint");
        network
            .chainstate_mut()
            .flush_with_mode(
                open_bitcoin_core::chainstate::FlushMode::Always,
                open_bitcoin_core::chainstate::FlushPolicyTime::from_unix_seconds(60_001),
                u64::MAX,
            )
            .expect("complete managed flush");
        let network = crate::ManagedNetworkHandle::new(network);
        if mode == BasicFilterStartupMode::Enabled {
            network
                .initialize_basic_filter_index_owner()
                .expect("actual BASIC owner");
            drain_basic(&network, descendants);
        }
        if replacement {
            replace_tip(&network, &store, params);
            drain_basic(&network, 302);
        }
        let mut expected = network
            .chainstate_snapshot()
            .expect("complete live snapshot");
        let stored = store
            .wallet_scan_chainstate_snapshot()
            .expect("stored")
            .expect("checkpoint");
        assert_eq!(stored.active_chain, expected.active_chain);
        assert_eq!(stored.utxos, expected.utxos);
        assert_eq!(
            stored.maybe_confirmed_txid_counts,
            expected.maybe_confirmed_txid_counts
        );
        for (hash, undo) in &expected.undo_by_block {
            assert_eq!(stored.undo_by_block.get(hash), Some(undo));
        }
        assert_eq!(
            stored.undo_by_block.len(),
            expected.undo_by_block.len() + if replacement { 2 } else { 0 }
        );
        expected.undo_by_block = stored.undo_by_block;
        drop(network);
        drop(store);
        let measured = crate::storage::fjall_store::measure_detached_rows_for_test(&root)
            .expect("actual complete copied inventory");
        eprintln!(
            "HISTORY_CHECKPOINT positions={} replacement={replacement} mode={mode:?} {measured:?}",
            expected.active_chain.len()
        );
        assert!(
            measured.authority_max <= 65536
                && measured.logical_entries <= 4096
                && measured.logical_bytes <= 536870912
        );
        assert!(measured.physical_entries <= 4096 && measured.physical_bytes <= 536870912);
        assert!(measured.manifest_equal && measured.elapsed.as_secs_f64() < 30.0);
        assert_eq!(measured.copy_bytes, measured.physical_bytes);
        let db = fjall::Database::builder(&root)
            .open()
            .expect("owned fixture all-handle reopen");
        let detached =
            crate::storage::fjall_store::inspection::InspectedRecords::read_existing(&db)
                .expect("ALL namespaces under global budget");
        let rows = detached
            .namespaces()
            .map(|name| {
                (
                    name.to_owned(),
                    detached.maybe_records(name).expect("namespace").clone(),
                )
            })
            .collect();
        drop(db);
        if expected.active_chain.len() <= records::MAX_HISTORY_BLOCKS {
            let decoded =
                CanonicalHistoryRecords::decode(&detached).expect("actual detached decode");
            assert_eq!(decoded.snapshot, expected);
        }
        assert!(
            crate::storage::canonical_bootstrap::records::CanonicalGenesisRecords::decode(
                &detached
            )
            .is_err(),
            "genesis-only parser stays strict"
        );
        check(rows, &expected);
    });
}

#[cfg(test)]
type GenuineHandle = crate::ManagedNetworkHandle<
    crate::chainstate::FjallChainstateStore,
    crate::storage::FjallCoinsView,
>;

#[cfg(test)]
fn drain_basic(network: &GenuineHandle, tip: u32) {
    let mut maybe_previous = None;
    for _ in 0..=records::MAX_HISTORY_BLOCKS {
        let turn = network
            .drive_basic_filter_index_turn()
            .expect("ordinary bounded turn");
        let height = turn
            .maybe_progress
            .expect("enabled")
            .maybe_processed_endpoint()
            .expect("endpoint")
            .height();
        assert!(maybe_previous.is_none_or(|previous| height > previous));
        maybe_previous = Some(height);
        if turn.maybe_accepted_lag == Some(0) {
            break;
        }
    }
    assert_eq!(
        network
            .maybe_basic_index_progress()
            .expect("progress")
            .expect("enabled")
            .maybe_safe_durable_endpoint()
            .expect("safe")
            .height(),
        tip
    );
}

#[cfg(test)]
fn replace_tip(
    network: &GenuineHandle,
    store: &crate::storage::FjallNodeStore,
    params: open_bitcoin_core::consensus::ConsensusParams,
) {
    let original = network.chainstate_snapshot().expect("original");
    let disconnected: Vec<_> = original.active_chain[300..]
        .iter()
        .rev()
        .map(|position| {
            store
                .load_block(position.block_hash)
                .expect("body")
                .expect("retained")
        })
        .collect();
    let mut parent = original.active_chain[299].header.clone();
    let mut blocks = Vec::new();
    for height in 300..=302 {
        let mut block = child(&parent, height);
        block.header.time += 100;
        block.transactions[0].lock_time += 10_000;
        if height == 302 {
            let mut spend = disconnected[0].transactions[1].clone();
            spend.lock_time = 302;
            block.transactions.push(spend);
            block.transactions[0].outputs[0].value =
                Amount::from_sats(1_250_000_001).expect("subsidy plus fee");
        }
        genuine::remine(&mut block);
        store
            .save_block(&block, PersistMode::Sync)
            .expect("retained fork body");
        parent = block.header.clone();
        blocks.push(block);
    }
    network
        .receive_message(
            1,
            open_bitcoin_network::WireNetworkMessage::Headers(
                open_bitcoin_network::HeadersMessage {
                    headers: blocks.iter().map(|block| block.header.clone()).collect(),
                },
            ),
            i64::MIN,
            ScriptVerifyFlags::NONE,
            params,
        )
        .expect("actual contextual fork ingress");
    let entries = network.header_entries().expect("earned headers");
    let branch: Vec<_> = blocks
        .iter()
        .map(|block| open_bitcoin_core::chainstate::AnchoredBlock {
            block: block.clone(),
            chain_work: entries
                .iter()
                .find(|entry| {
                    entry.block_hash == open_bitcoin_core::consensus::block_hash(&block.header)
                })
                .expect("earned work")
                .chain_work,
        })
        .collect();
    let transition = network
        .reorg_to_branch(
            &disconnected,
            &branch,
            open_bitcoin_mempool::ReorgLifecycleContext::new(
                open_bitcoin_mempool::PolicyTime::from_unix_seconds(i64::MIN),
            ),
            ScriptVerifyFlags::NONE,
            params,
        )
        .expect("actual preferred replacement");
    assert_eq!(transition.disconnected.len(), 2);
    assert_eq!(transition.connected.len(), 3);
    let current = network.chainstate_snapshot().expect("replacement");
    assert!(current.tip().expect("tip").chain_work > original.tip().expect("old").chain_work);
    assert_ne!(
        current.tip().expect("tip").median_time_past,
        original.tip().expect("old").median_time_past
    );
    for block in &disconnected {
        assert!(
            !current
                .maybe_confirmed_txid_counts
                .as_ref()
                .expect("counts")
                .contains_key(
                    &open_bitcoin_core::consensus::transaction_txid(&block.transactions[0])
                        .expect("disconnected txid")
                )
        );
    }
    network
        .persist_validation_header_snapshot()
        .expect("trusted replacement headers");
    network
        .flush_coins(
            open_bitcoin_core::chainstate::FlushMode::Always,
            open_bitcoin_core::chainstate::FlushPolicyTime::from_unix_seconds(60_002),
            u64::MAX,
        )
        .expect("replacement checkpoint");
}

#[cfg(test)]
mod adversarial {
    use super::records::CanonicalHistoryRecords;
    use super::with_rows;
    use crate::{
        chainstate::BasicFilterStartupMode,
        storage::{
            canonical_bootstrap::validate_history_records, filter_index as filters,
            validation_history as history,
        },
    };
    use open_bitcoin_core::consensus::CanonicalChain;

    #[test]
    fn phase167_history_proof_every_basic_projection_is_regenerated() {
        with_rows(BasicFilterStartupMode::Enabled, |rows, _| {
            for case in [
                "filter",
                "filter-header",
                "projection",
                "missing-filter",
                "extra-filter",
                "fence",
                "endpoint",
                "protection",
                "owner",
                "lock",
            ] {
                // Arrange
                let mut parsed =
                    CanonicalHistoryRecords::parse_rows(&rows).expect("complete genuine history");
                let hash = parsed.snapshot.active_chain[150].block_hash;
                let key = filters::record_key(hash).into_bytes();
                match case {
                    "filter" => {
                        *parsed
                            .basic
                            .get_mut(&key)
                            .expect("filter")
                            .last_mut()
                            .expect("filter bytes") ^= 1;
                    }
                    "filter-header" => parsed.basic.get_mut(&key).expect("filter")[138] ^= 1,
                    "projection" => {
                        *parsed
                            .basic
                            .get_mut(filters::active_key(150).as_bytes())
                            .expect("projection")
                            .last_mut()
                            .expect("hash") ^= 1;
                    }
                    "missing-filter" => {
                        parsed.basic.remove(&key);
                    }
                    "extra-filter" => {
                        parsed
                            .basic
                            .insert(b"basic_filter:v1:record:unknown".to_vec(), vec![1]);
                    }
                    "fence" | "endpoint" | "protection" => {
                        let mut state =
                            filters::decode_state(&parsed.basic[filters::STATE_KEY.as_bytes()])
                                .expect("state");
                        match case {
                            "fence" => state.fence_hash = Default::default(),
                            "endpoint" => state.maybe_endpoint = Some((150, Default::default())),
                            _ => {
                                state.protection =
                                    open_bitcoin_core::chainstate::IndexInputProtection::FromHeight(
                                        303,
                                    )
                            }
                        }
                        parsed.basic.insert(
                            filters::STATE_KEY.as_bytes().to_vec(),
                            filters::encode_state(state),
                        );
                    }
                    "owner" => {
                        parsed
                            .basic
                            .get_mut(filters::ownership::OWNER_KEY.as_bytes())
                            .expect("owner")[2] = 9;
                    }
                    "lock" => {
                        parsed.basic.remove(b"prune_locks".as_slice());
                    }
                    _ => panic!("unknown BASIC mutation"),
                }
                // Act / Assert
                assert!(
                    validate_history_records(CanonicalChain::Regtest, &parsed).is_err(),
                    "{case}"
                );
            }
        });
    }

    #[test]
    fn phase167_history_proof_consumption_binds_complete_source_and_generation() {
        with_rows(BasicFilterStartupMode::Enabled, |rows, _| {
            for case in ["runtime", "basic-generation"] {
                // Arrange
                let original = CanonicalHistoryRecords::parse_rows(&rows).expect("original");
                let proof = validate_history_records(CanonicalChain::Regtest, &original)
                    .expect("full replay");
                let mut changed = rows.clone();
                if case == "runtime" {
                    changed
                        .entry("runtime".into())
                        .or_default()
                        .insert(b"snapshot".to_vec(), vec![1, 2, 3]);
                } else {
                    changed
                        .get_mut("block_index")
                        .expect("index")
                        .get_mut(filters::ownership::OWNER_KEY.as_bytes())
                        .expect("owner")[3] ^= 1;
                }
                let changed = CanonicalHistoryRecords::parse_rows(&changed)
                    .expect("well-formed changed source");
                // Act / Assert: even changed nonauthorizing metadata invalidates consumption.
                assert_ne!(original.source_digest(), changed.source_digest());
                assert!(matches!(
                    proof.into_parts(&changed),
                    Err(super::Refusal::ChangedStore)
                ));
            }
        });
    }

    #[test]
    fn phase167_history_proof_known_only_metadata_never_grants_acceptance() {
        with_rows(BasicFilterStartupMode::Disabled, |mut rows, expected| {
            // Arrange: bounded retained invalid/candidate metadata is not ScriptsValid.
            let hash = open_bitcoin_core::primitives::BlockHash::from_byte_array([9; 32]);
            let parent = expected.active_chain[150].block_hash;
            let identity = history::BlockValidationIdentity::new(hash, parent, 151)
                .expect("candidate identity");
            let record = history::ValidationHistoryRecord::new(
                identity,
                history::StoredValidationStatus::KnownOnly,
            );
            rows.get_mut("block_index").expect("index").insert(
                history::record_key(hash).into_bytes(),
                history::encode_record(record),
            );
            let parsed =
                CanonicalHistoryRecords::parse_rows(&rows).expect("complete namespace parser");
            // Act
            let proof = validate_history_records(CanonicalChain::Regtest, &parsed)
                .expect("harmless metadata is separate");
            let (_, _, replay) = proof.into_parts(&parsed).expect("source bound");
            // Assert
            assert_eq!(
                replay
                    .admission_snapshot()
                    .expect("complete accepted state"),
                *expected
            );
        });
    }

    #[test]
    fn phase167_history_proof_wrong_selector_seeded_suffix_and_cap_refuse() {
        with_rows(BasicFilterStartupMode::Disabled, |rows, _| {
            // Arrange
            let mut parsed = CanonicalHistoryRecords::parse_rows(&rows).expect("original");
            // Act / Assert
            for selected in [
                CanonicalChain::Mainnet,
                CanonicalChain::Testnet3,
                CanonicalChain::Signet,
            ] {
                assert!(validate_history_records(selected, &parsed).is_err());
            }
            parsed.snapshot.active_chain.drain(..100);
            assert!(
                validate_history_records(CanonicalChain::Regtest, &parsed).is_err(),
                "unknown seeded suffix cannot be a root"
            );
            let mut parsed = CanonicalHistoryRecords::parse_rows(&rows).expect("original");
            parsed.snapshot.active_chain.truncate(151);
            assert!(
                validate_history_records(CanonicalChain::Regtest, &parsed).is_err(),
                "remaining accepted history cannot be silently omitted"
            );
        });
    }

    #[test]
    fn phase167_history_records_finite_complete_307_checkpoint_refuses() {
        super::with_history(
            BasicFilterStartupMode::Disabled,
            306,
            false,
            |rows, expected| {
                // Arrange / Act / Assert: real complete rows exceed the finite306 envelope.
                assert_eq!(expected.active_chain.len(), 307);
                assert!(matches!(
                    CanonicalHistoryRecords::parse_rows(&rows),
                    Err(super::Refusal::UnsupportedHistory)
                ));
            },
        );
    }

    #[test]
    fn phase167_history_proof_mature_spend_preserves_spent_identity_and_complete_undo() {
        super::with_history(
            BasicFilterStartupMode::Enabled,
            301,
            true,
            |rows, expected| {
                // Arrange
                let parsed = CanonicalHistoryRecords::parse_rows(&rows)
                    .expect("actual spend-bearing checkpoint");
                let first = expected.active_chain[1].block_hash;
                let txid = open_bitcoin_core::consensus::transaction_txid(
                    &parsed.bodies[&first].transactions[0],
                )
                .expect("spent coinbase");
                assert!(!expected.utxos.keys().any(|outpoint| outpoint.txid == txid));
                assert_eq!(
                    expected
                        .maybe_confirmed_txid_counts
                        .as_ref()
                        .expect("complete occurrences")
                        .get(&txid),
                    Some(&1)
                );
                assert_eq!(
                    expected
                        .maybe_confirmed_txid_counts
                        .as_ref()
                        .expect("complete occurrences")
                        .len(),
                    302
                );
                let tip = expected.tip().expect("tip").block_hash;
                assert_eq!(
                    expected.undo_by_block[&tip].transactions[0]
                        .restored_inputs
                        .len(),
                    1
                );
                // Act
                let proof = validate_history_records(CanonicalChain::Regtest, &parsed)
                    .expect("actual selected script/reward/fee/maturity replay");
                let (_, _, replay) = proof.into_parts(&parsed).expect("same source");
                // Assert
                assert_eq!(
                    replay.admission_snapshot().expect("complete earned state"),
                    *expected
                );
            },
        );
    }
}
