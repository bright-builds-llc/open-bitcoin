// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/script/script.h
// - packages/bitcoin-knots/src/blockfilter.cpp
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp
// - packages/bitcoin-knots/src/validation.cpp

use std::collections::HashMap;

use open_bitcoin_consensus::chain_proof::ProofWork;
use open_bitcoin_consensus::chain_spec::CanonicalGenesisProof;
use open_bitcoin_consensus::crypto::Sha256;
use open_bitcoin_consensus::{
    CanonicalChain, ConsensusParams, FilterHeaderPredecessor, ScriptVerifyFlags, block_hash,
    block_merkle_root, check_block_header, count_legacy_sigops, transaction_txid,
};
use open_bitcoin_primitives::{
    Amount, Block, BlockHeader, FilterHeader, OutPoint, ScriptBuf, Transaction, TransactionInput,
    TransactionOutput,
};

use crate::coins::{CoinsCache, CoinsOverlay, MemoryCoinsView};
use crate::types::{BlockOverwriteUndo, OverwrittenCoin};
use crate::{
    BlockUndo, ChainPosition, Chainstate, ChainstateSnapshot, Coin, StagedChainstateConnect,
};

fn script(bytes: Vec<u8>) -> ScriptBuf {
    ScriptBuf::from_serialized_bytes(bytes)
}

fn output(bytes: Vec<u8>) -> TransactionOutput {
    TransactionOutput {
        value: Amount::from_sats(1).expect("valid amount"),
        script_pubkey: script(bytes),
    }
}

fn canonical() -> Chainstate {
    let spec = CanonicalChain::Regtest.spec();
    let proof = CanonicalGenesisProof::prove(spec, &spec.genesis_block()).expect("proved genesis");
    Chainstate::from_canonical_genesis(
        &proof,
        CoinsCache::from_parent(MemoryCoinsView::from_coins(
            HashMap::new(),
            Some(proof.hash()),
        )),
    )
    .expect("canonical owner")
}

fn coinbase(height: u32) -> Transaction {
    let height_script = if height <= 16 {
        vec![0x50 + height as u8, 0x51]
    } else {
        vec![1, height as u8]
    };
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: script(height_script),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: Default::default(),
        }],
        outputs: vec![output(vec![0x51])],
        lock_time: 0,
    }
}

fn child(chain: &Chainstate, transactions: Vec<Transaction>) -> Block {
    let tip = chain.tip().expect("tip");
    let mut block = Block {
        header: BlockHeader {
            version: 4,
            previous_block_hash: tip.block_hash,
            time: tip.header.time + 600,
            bits: 0x207fffff,
            ..Default::default()
        },
        transactions,
    };
    block.header.merkle_root = block_merkle_root(&block.transactions).expect("merkle").0;
    while check_block_header(&block.header).is_err() {
        block.header.nonce += 1;
    }
    block
}

fn stage(chain: &mut Chainstate, block: &Block) -> StagedChainstateConnect {
    chain
        .admit_canonical_header(&block.header, i64::from(block.header.time))
        .expect("actual header admission");
    let mut bytes = [0; 32];
    bytes[0] = 2; // Pinned regtest block proof for 0x207fffff.
    let work = chain
        .tip()
        .expect("tip")
        .chain_work
        .wrapping_add(ProofWork::from_le_bytes(bytes));
    chain
        .stage_connect_block_with_current_time(
            block,
            work,
            -1,
            ScriptVerifyFlags::NONE,
            ConsensusParams::default(),
        )
        .expect("actual selected stage")
}

#[test]
fn duplicate_coinbase_outpoint_refuses_before_overlay_publication() {
    // Arrange
    let transaction = coinbase(1);
    let outpoint = OutPoint {
        txid: transaction_txid(&transaction).expect("real txid"),
        vout: 0,
    };
    let prior = Coin {
        output: transaction.outputs[0].clone(),
        is_coinbase: true,
        created_height: 0,
        created_median_time_past: 0,
    };
    let parent =
        MemoryCoinsView::from_coins(HashMap::from([(outpoint.clone(), prior.clone())]), None);
    let mut overlay = CoinsOverlay::new();

    // Act
    let error = super::add_transaction_outputs(&mut overlay, &parent, &transaction, 1, 0)
        .expect_err("existing OutPoint cannot be overwritten");

    // Assert
    assert!(
        matches!(error, crate::ChainstateError::OutputOverwrite { outpoint: refused }
        if refused == outpoint)
    );
    assert_eq!(
        overlay.peek_coin(&parent, &outpoint).expect("overlay read"),
        Some(prior)
    );
}

fn matrix() -> Transaction {
    let mut tx = coinbase(1);
    let mut boundary = vec![0x61; 9999];
    boundary.push(0xac);
    let mut wide = vec![0x61; 10_000];
    wide.push(0xac);
    tx.outputs = vec![
        output(boundary),
        output(wide),
        output(vec![]),
        output(vec![0x6a, 0xac]),
        output(vec![0x51]),
    ];
    tx
}

fn empty_overwrite(block: &Block, height: u32, mtp: i64) -> BlockOverwriteUndo {
    let hash = block_hash(&block.header);
    let mut preimage = b"open-bitcoin/complete-overwrite-undo/v1".to_vec();
    preimage.extend_from_slice(hash.as_bytes());
    preimage.extend_from_slice(&height.to_le_bytes());
    preimage.extend_from_slice(&mtp.to_le_bytes());
    preimage.extend_from_slice(&0_u64.to_le_bytes());
    BlockOverwriteUndo {
        block_hash: hash,
        height,
        creating_median_time_past: mtp,
        overwritten: vec![],
        integrity: Sha256::digest(&preimage),
    }
}

fn expected_overwrite(
    block: &Block,
    height: u32,
    mtp: i64,
    old: &(OutPoint, Coin),
) -> BlockOverwriteUndo {
    let mut batch = empty_overwrite(block, height, mtp);
    let new_coin = Coin {
        output: old.1.output.clone(),
        is_coinbase: true,
        created_height: height,
        created_median_time_past: mtp,
    };
    batch.overwritten.push(OverwrittenCoin {
        transaction_index: 0,
        outpoint: old.0.clone(),
        previous_coin: old.1.clone(),
        new_coin,
    });
    // Independent encoding of the documented complete-overwrite-undo/v1 fields.
    let mut bytes = b"open-bitcoin/complete-overwrite-undo/v1".to_vec();
    bytes.extend_from_slice(batch.block_hash.as_bytes());
    bytes.extend_from_slice(&height.to_le_bytes());
    bytes.extend_from_slice(&mtp.to_le_bytes());
    bytes.extend_from_slice(&1_u64.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(old.0.txid.as_bytes());
    bytes.extend_from_slice(&old.0.vout.to_le_bytes());
    for coin in [&old.1, &batch.overwritten[0].new_coin] {
        bytes.extend_from_slice(&coin.output.value.to_sats().to_le_bytes());
        let script = coin.output.script_pubkey.as_bytes();
        bytes.extend_from_slice(&(script.len() as u64).to_le_bytes());
        bytes.extend_from_slice(script);
        bytes.push(u8::from(coin.is_coinbase));
        bytes.extend_from_slice(&coin.created_height.to_le_bytes());
        bytes.extend_from_slice(&coin.created_median_time_past.to_le_bytes());
    }
    batch.integrity = Sha256::digest(&bytes);
    batch
}

#[test]
fn phase167_serialized_script_coins_canonical_stage_consume_disconnect_complete_state() {
    // Arrange
    let mut chain = canonical();
    let before = chain.snapshot();
    let tx = matrix();
    let txid = transaction_txid(&tx).expect("exact txid");
    // Independently serialized literal fixture (raw SHA256d protocol byte order).
    assert_eq!(
        txid.as_bytes(),
        &[
            0xaa, 0x4d, 0xab, 0x16, 0x42, 0xc4, 0x1c, 0x5d, 0x4d, 0x0a, 0xb9, 0x69, 0xca, 0x95,
            0x29, 0x30, 0x8c, 0xeb, 0x4b, 0x2c, 0xfc, 0xe4, 0x45, 0x77, 0x2a, 0x6a, 0xb2, 0xae,
            0xa8, 0xca, 0x41, 0xc2
        ]
    );
    let block = child(&chain, vec![tx.clone()]);
    let original = block.clone();
    let prior_tip = before.tip().expect("genesis");
    let mtp = i64::from(prior_tip.header.time);
    let expected_undo = BlockUndo {
        transactions: vec![],
        overwrites: vec![empty_overwrite(&block, 1, mtp)],
    };
    let mut expected = before.clone();
    let mut work_bytes = [0; 32];
    work_bytes[0] = 4; // Genesis plus one default-regtest block.
    expected.active_chain.push(ChainPosition::new(
        block.header.clone(),
        1,
        ProofWork::from_le_bytes(work_bytes),
        i64::from(block.header.time),
    ));
    for vout in [0, 2, 4] {
        expected.utxos.insert(
            OutPoint { txid, vout },
            Coin {
                output: tx.outputs[vout as usize].clone(),
                is_coinbase: true,
                created_height: 1,
                created_median_time_past: mtp,
            },
        );
    }
    expected
        .undo_by_block
        .insert(block_hash(&block.header), expected_undo.clone());
    expected.maybe_confirmed_txid_counts = Some(HashMap::from([(txid, 1)]));
    // Act
    let staged = stage(&mut chain, &block);
    // Assert: staging is private and the complete undo has no wide output Coin.
    assert!(staged.has_earned_work());
    assert_eq!(staged.undo(), &expected_undo);
    assert_eq!(chain.snapshot(), before);
    chain.absorb_staged_connect(staged).expect("consume stage");
    assert_eq!(chain.snapshot(), expected);
    assert_eq!(
        chain.coins_best_block().expect("best"),
        Some(block_hash(&block.header))
    );
    assert_eq!(block, original);
    chain.disconnect_tip(&block).expect("actual disconnect");
    assert_eq!(chain.snapshot(), before);
    assert_eq!(
        chain.coins_best_block().expect("best"),
        Some(prior_tip.block_hash)
    );
}

#[test]
fn phase167_serialized_script_coins_basic_and_sigops_observe_exact_wide_body() {
    // Arrange
    let mut chain = canonical();
    let before = chain.snapshot();
    let tx = matrix();
    let original_scripts: Vec<_> = tx
        .outputs
        .iter()
        .map(|output| output.script_pubkey.as_bytes().to_vec())
        .collect();
    let block = child(&chain, vec![tx]);
    let staged = stage(&mut chain, &block);
    let predecessor = FilterHeaderPredecessor::Previous {
        block_hash: before.tip().expect("genesis").block_hash,
        height: 0,
        header: FilterHeader::default(),
    };
    // Act
    let (filter, _) = staged
        .basic_filter_inputs(&block)
        .expect("actual staged inputs")
        .generate(predecessor)
        .expect("BASIC");
    let sigops: Vec<_> = block.transactions[0]
        .outputs
        .iter()
        .map(|output| count_legacy_sigops(&output.script_pubkey).expect("raw syntax count"))
        .collect();
    // Assert
    assert_eq!(filter.encoded_bytes()[0], 3);
    // Pinned Python SipHash oracle and source-derived BASIC selection/Rice encoding.
    assert_eq!(
        filter.encoded_bytes(),
        &[0x03, 0x73, 0xe6, 0x20, 0x8a, 0xe2, 0xa4, 0x54, 0x88]
    );
    assert_eq!(sigops, vec![1, 1, 0, 1, 0]);
    assert_eq!(
        block.transactions[0].outputs[1].script_pubkey.as_bytes(),
        original_scripts[1]
    );
    assert_eq!(chain.snapshot(), before);
}

#[test]
fn phase167_serialized_script_coins_cannot_spend_excluded_wide_output() {
    // Arrange
    let mut chain = canonical();
    let tx = matrix();
    let wide_outpoint = OutPoint {
        txid: transaction_txid(&tx).expect("txid"),
        vout: 1,
    };
    let first = child(&chain, vec![tx]);
    let first_stage = stage(&mut chain, &first);
    chain.absorb_staged_connect(first_stage).expect("consume");
    let before = chain.snapshot();
    let spend = Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: wide_outpoint.clone(),
            script_sig: ScriptBuf::default(),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: Default::default(),
        }],
        outputs: vec![output(vec![0x51])],
        lock_time: 0,
    };
    let block = child(&chain, vec![coinbase(2), spend]);
    chain
        .admit_canonical_header(&block.header, i64::from(block.header.time))
        .expect("header");
    // Act
    let mut work_bytes = [0; 32];
    work_bytes[0] = 6;
    let result = chain.stage_connect_block_with_current_time(
        &block,
        ProofWork::from_le_bytes(work_bytes),
        -1,
        ScriptVerifyFlags::NONE,
        ConsensusParams::default(),
    );
    // Assert
    assert!(
        matches!(result, Err(crate::ChainstateError::MissingCoin { outpoint }) if outpoint == wide_outpoint)
    );
    assert_eq!(chain.snapshot(), before);
}

#[test]
fn phase167_serialized_script_coins_supported_historical_overwrite_restores_all_facts() {
    // Arrange: independently pinned real BIP30 bodies and explicitly seeded contexts.
    let (cases, _) =
        super::super::tests::phase167_bip30::load(super::super::tests::phase167_bip30::DATA)
            .expect("pinned corpus");
    for case in cases {
        let mut chain = super::super::tests::phase167_bip30::fixture(&case);
        let before: ChainstateSnapshot = chain.snapshot();
        let old = case.old.clone();
        // Act
        let staged = chain
            .stage_seeded_historical_block(&case.block, &case.history, case.now)
            .expect("actual supported overwrite stage");
        // Assert
        assert_eq!(chain.snapshot(), before);
        assert_eq!(staged.undo().overwrites.len(), 1);
        let entries = &staged.undo().overwrites[0].overwritten;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].outpoint, old.0);
        assert_eq!(entries[0].previous_coin, old.1);
        assert!(entries[0].new_coin.output.script_pubkey.as_bytes().len() <= 10_000);
        let undo = BlockUndo {
            transactions: case
                .block
                .transactions
                .iter()
                .skip(1)
                .map(|tx| crate::TxUndo {
                    restored_inputs: tx
                        .inputs
                        .iter()
                        .map(|input| before.utxos[&input.previous_output].clone())
                        .collect(),
                })
                .collect(),
            overwrites: vec![expected_overwrite(
                &case.block,
                case.history.height(),
                case.history.parent_median_time_past(),
                &old,
            )],
        };
        assert_eq!(staged.undo(), &undo);
        chain
            .absorb_staged_connect(staged)
            .expect("consume actual overwrite");
        let mut expected_coins = before.utxos.clone();
        expected_coins.insert(
            old.0.clone(),
            Coin {
                output: old.1.output.clone(),
                is_coinbase: true,
                created_height: case.history.height(),
                created_median_time_past: case.history.parent_median_time_past(),
            },
        );
        for tx in case.block.transactions.iter().skip(1) {
            for input in &tx.inputs {
                expected_coins.remove(&input.previous_output);
            }
            let txid = transaction_txid(tx).expect("txid");
            for (vout, output) in tx.outputs.iter().enumerate() {
                expected_coins.insert(
                    OutPoint {
                        txid,
                        vout: vout as u32,
                    },
                    Coin {
                        output: output.clone(),
                        is_coinbase: false,
                        created_height: case.history.height(),
                        created_median_time_past: case.history.parent_median_time_past(),
                    },
                );
            }
        }
        let mut expected = before.clone();
        expected.utxos = expected_coins;
        let mut times: Vec<_> = before
            .active_chain
            .iter()
            .rev()
            .take(10)
            .map(|position| position.header.time)
            .collect();
        times.push(case.block.header.time);
        times.sort_unstable();
        let work = before.tip().expect("tip").chain_work.wrapping_add(
            open_bitcoin_consensus::chain_proof::block_proof(case.block.header.bits)
                .expect("pin target"),
        );
        expected.active_chain.push(ChainPosition::new(
            case.block.header.clone(),
            case.history.height(),
            work,
            i64::from(times[times.len() / 2]),
        ));
        expected
            .undo_by_block
            .insert(block_hash(&case.block.header), undo);
        let counts = expected
            .maybe_confirmed_txid_counts
            .as_mut()
            .expect("complete counts");
        for tx in &case.block.transactions {
            *counts
                .entry(transaction_txid(tx).expect("txid"))
                .or_insert(0) += 1;
        }
        assert_eq!(chain.snapshot(), expected);
        chain
            .disconnect_tip(&case.block)
            .expect("actual supported disconnect");
        assert_eq!(chain.snapshot(), before);
        assert_eq!(chain.get_coin(&old.0).expect("read"), Some(old.1));
    }
}

#[test]
fn phase167_serialized_script_coins_mature_spend_wide_output_restores_complete_undo() {
    // Arrange: genuine canonical maturity100, with no parameter override.
    let mut chain = canonical();
    let funding = coinbase(1);
    let outpoint = OutPoint {
        txid: transaction_txid(&funding).expect("funding"),
        vout: 0,
    };
    for height in 1..=100 {
        let block = child(&chain, vec![coinbase(height)]);
        let staged = stage(&mut chain, &block);
        chain
            .absorb_staged_connect(staged)
            .expect("canonical accepted prefix");
    }
    let before = chain.snapshot();
    let restored = before.utxos[&outpoint].clone();
    let spend = Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: outpoint.clone(),
            script_sig: ScriptBuf::default(),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: Default::default(),
        }],
        outputs: vec![output(vec![0x61; 10_001])],
        lock_time: 0,
    };
    let block = child(&chain, vec![coinbase(101), spend]);
    let hash = block_hash(&block.header);
    let mtp = 1_296_688_602 + 95 * 600;
    let undo = BlockUndo {
        transactions: vec![crate::TxUndo {
            restored_inputs: vec![restored],
        }],
        overwrites: vec![empty_overwrite(&block, 101, mtp)],
    };
    let mut expected = before.clone();
    expected.utxos.remove(&outpoint);
    let coinbase_txid = transaction_txid(&block.transactions[0]).expect("coinbase id");
    expected.utxos.insert(
        OutPoint {
            txid: coinbase_txid,
            vout: 0,
        },
        Coin {
            output: block.transactions[0].outputs[0].clone(),
            is_coinbase: true,
            created_height: 101,
            created_median_time_past: mtp,
        },
    );
    let mut work_bytes = [0; 32];
    work_bytes[0] = 204; // 102 canonical regtest headers, two proof units each.
    expected.active_chain.push(ChainPosition::new(
        block.header.clone(),
        101,
        ProofWork::from_le_bytes(work_bytes),
        1_296_688_602 + 96 * 600,
    ));
    expected.undo_by_block.insert(hash, undo.clone());
    for tx in &block.transactions {
        *expected
            .maybe_confirmed_txid_counts
            .as_mut()
            .expect("counts")
            .entry(transaction_txid(tx).expect("id"))
            .or_insert(0) += 1;
    }
    // Act
    let staged = stage(&mut chain, &block);
    // Assert
    assert_eq!(staged.undo(), &undo);
    assert_eq!(chain.snapshot(), before);
    chain
        .absorb_staged_connect(staged)
        .expect("consume mature spend");
    assert_eq!(chain.snapshot(), expected);
    chain.disconnect_tip(&block).expect("undo mature spend");
    assert_eq!(chain.snapshot(), before);
}
