// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp
// - packages/bitcoin-knots/src/pow.cpp

use std::collections::HashMap;

use open_bitcoin_core::{
    chainstate::{ChainPosition, Chainstate, ChainstateSnapshot, Coin},
    consensus::{
        CandidateHistory, CanonicalChain, HistoryHeader, HistoryOrigin, block_proof,
        chain_proof::ProofWork, check_block_header,
    },
    primitives::{Amount, Block, OutPoint, ScriptBuf, Txid},
};
use serde_json::Value;

use super::super::phase167_support as ordinary;

fn selected_row(expected: &Value, height: usize) -> &Value {
    let row = &expected["rows"][height];
    assert_eq!(row["height"].as_u64(), Some(height as u64));
    row
}

pub(super) fn source_case(
    height: usize,
    oldest: u32,
    creating_mtp_delta: i64,
) -> (Block, CandidateHistory, ChainstateSnapshot) {
    // Arrange: source-derived complete parent UTXOs feed each selected candidate.
    assert!(height > 0 && height <= 301 && oldest < height as u32);
    let corpus = ordinary::corpus();
    let expected = ordinary::expectations();
    let block = ordinary::block(ordinary::row(&corpus, height));
    let maybe_reserved = if height == 201 {
        ordinary::assert_creating_ancestor_201(&block, &expected["creating_live_201"]);
        Some(
            expected["creating_live_201"]["reserved_outpoint"]
                .as_str()
                .expect("reserved outpoint"),
        )
    } else {
        assert_eq!(creating_mtp_delta, 0);
        None
    };
    let mut all = Vec::with_capacity(height);
    let mut work = ProofWork::ZERO;
    for ancestor in 0..height {
        let source = ordinary::block(ordinary::row(&corpus, ancestor));
        check_block_header(&source.header).expect("source-bound PoW");
        work = work.wrapping_add(block_proof(source.header.bits).expect("source work"));
        let facts = selected_row(&expected, ancestor);
        assert_eq!(
            work.to_hex(),
            facts["chain_work_hex"].as_str().expect("independent work")
        );
        let mtp = facts["median_time_past"].as_i64().expect("independent MTP");
        all.push(ChainPosition::new(
            source.header,
            ancestor as u32,
            work,
            mtp,
        ));
    }
    let branch: Vec<_> = all[oldest as usize..]
        .iter()
        .rev()
        .map(|position| {
            HistoryHeader::new(
                CanonicalChain::Regtest,
                position.height,
                position.header.clone(),
            )
        })
        .collect();
    let history = CandidateHistory::check_seeded(
        CanonicalChain::Regtest.spec().rules(),
        block.header.clone(),
        branch[0].clone(),
        branch[1..].to_vec(),
        None,
    )
    .expect("source-bound seeded branch");
    assert_eq!(history.origin(), HistoryOrigin::IndependentlySeeded);
    assert_eq!(
        history.parent_median_time_past(),
        selected_row(&expected, height - 1)["median_time_past"]
            .as_i64()
            .expect("parent MTP")
    );
    let mut coins = HashMap::new();
    let utxos = selected_row(&expected, height - 1)["utxos"]
        .as_array()
        .expect("complete UTXO set");
    for source in utxos {
        let txid = source["txid"].as_str().expect("raw txid");
        let vout = source["vout"].as_u64().expect("vout") as u32;
        let outpoint = OutPoint {
            txid: Txid::from_byte_array(ordinary::bytes(txid).try_into().expect("txid width")),
            vout,
        };
        let value = source["sats"]
            .as_str()
            .expect("amount")
            .parse::<i64>()
            .expect("sats");
        let output = open_bitcoin_core::primitives::TransactionOutput {
            value: Amount::from_sats(value).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(ordinary::bytes(
                source["script"].as_str().expect("script"),
            ))
            .expect("bounded script"),
        };
        let mut creating_mtp = source["creating_parent_mtp"]
            .as_i64()
            .expect("creating MTP");
        if maybe_reserved.is_some_and(|reserved| format!("{txid}:{vout}") == reserved) {
            assert_eq!(source["height"].as_u64(), Some(1));
            creating_mtp += creating_mtp_delta;
        }
        let coin = Coin {
            output,
            is_coinbase: source["coinbase"].as_bool().expect("coinbase fact"),
            created_height: source["height"].as_u64().expect("creating height") as u32,
            created_median_time_past: creating_mtp,
        };
        assert!(coins.insert(outpoint, coin).is_none());
    }
    assert_eq!(coins.len(), utxos.len());
    let snapshot = ChainstateSnapshot::new(all[oldest as usize..].to_vec(), coins, HashMap::new());
    assert_eq!(
        snapshot.tip().expect("source parent").block_hash,
        block.header.previous_block_hash
    );
    (block, history, snapshot)
}

#[test]
fn phase167_historical_scripts_creating_missing_complete_actual_kernel() {
    // Arrange: suffix46..200 supplies ordinary parent, expiry and recovery coverage,
    // but cannot contain height0, the creating parent of the reserved height1 coin.
    let (block, missing_history, missing_snapshot) = source_case(201, 46, 0);
    assert_eq!(missing_history.branch().len(), 155);
    let (same_block, complete_history, complete_snapshot) = source_case(201, 0, 0);
    assert_eq!(block, same_block);
    assert_eq!(missing_snapshot.utxos, complete_snapshot.utxos);
    // Act: the same actual apply/Coins kernel sees absent and then complete context.
    let missing = Chainstate::from_snapshot_for_mechanical_fixture(missing_snapshot.clone());
    let refused = missing.stage_seeded_historical_block(
        &block,
        &missing_history,
        i64::from(block.header.time),
    );
    let mut complete = Chainstate::from_snapshot_for_mechanical_fixture(complete_snapshot);
    let accepted = complete.stage_seeded_historical_block(
        &block,
        &complete_history,
        i64::from(block.header.time),
    );
    // Assert: no caller context flag substitutes for the missing ancestor.
    let error = refused.expect_err("actual missing creating MTP must refuse");
    assert!(
        format!("{error:?}").contains("MissingAncestor"),
        "{error:?}"
    );
    assert_eq!(missing.snapshot(), missing_snapshot);
    let staged = accepted.expect("same candidate with complete history reaches kernel");
    assert!(!staged.has_earned_work());
    complete
        .absorb_staged_connect(staged)
        .expect("mechanical acceptance");
    assert!(!complete.has_earned_work());
    assert!(complete.maybe_canonical_chain().is_none());
    assert_eq!(
        complete.tip().expect("accepted conditional tip").height,
        201
    );
}

#[test]
fn phase167_historical_scripts_creating_mtp_actual_kernel() {
    // Arrange: the only changed fact is the reserved coin's creating-MTP metadata.
    let (block, history, damaged_snapshot) = source_case(201, 0, 1);
    let (same_block, same_history, matching_snapshot) = source_case(201, 0, 0);
    assert_eq!(block, same_block);
    assert_eq!(history, same_history);
    assert_eq!(
        damaged_snapshot.active_chain,
        matching_snapshot.active_chain
    );
    // Act
    let damaged = Chainstate::from_snapshot_for_mechanical_fixture(damaged_snapshot.clone());
    let refused =
        damaged.stage_seeded_historical_block(&block, &history, i64::from(block.header.time));
    let matching = Chainstate::from_snapshot_for_mechanical_fixture(matching_snapshot);
    let accepted =
        matching.stage_seeded_historical_block(&block, &history, i64::from(block.header.time));
    // Assert
    let error = refused.expect_err("actual creating MTP mismatch must refuse");
    assert!(
        format!("{error:?}").contains("creating-ancestor-mtp-mismatch"),
        "{error:?}"
    );
    assert_eq!(damaged.snapshot(), damaged_snapshot);
    assert!(
        !accepted
            .expect("matching creating MTP reaches kernel")
            .has_earned_work()
    );
    assert!(!matching.has_earned_work());
}

#[test]
fn phase167_historical_scripts_regtest_retarget_boundary_header_and_body_agree() {
    // Arrange: six exact serialized bodies straddle both retarget boundaries.
    for height in [143_usize, 144, 145, 287, 288, 289] {
        let (block, history, snapshot) = source_case(height, 0, 0);
        assert_eq!(history.height() as usize, height);
        assert_eq!(block.header.bits, history.parent().header.bits);
        let fixture = Chainstate::from_snapshot_for_mechanical_fixture(snapshot.clone());
        assert!(!fixture.has_earned_work());
        // Act: the checked header facts and actual Coins/body kernel share
        // selected no-retarget, PoW, MTP and script deployment context.
        let header = history.check_header_rules(i64::from(block.header.time));
        let body =
            fixture.stage_seeded_historical_block(&block, &history, i64::from(block.header.time));
        // Assert: no source body or scalar context can mint canonical work.
        assert!(header.is_ok(), "header at {height}: {header:?}");
        let staged = body.unwrap_or_else(|error| panic!("body at {height}: {error:?}"));
        assert_eq!(staged.position().height as usize, height);
        assert!(!staged.has_earned_work());
        assert_eq!(fixture.snapshot(), snapshot);
    }
}

#[test]
fn phase167_historical_scripts_header_and_body_refuse_changed_mtp_and_bits() {
    // Arrange: height144 is a source-bound no-retarget boundary.
    let (source, original, snapshot) = source_case(144, 0, 0);
    for mutate in [0_u8, 1] {
        let mut block = source.clone();
        if mutate == 0 {
            block.header.time = original.parent_median_time_past() as u32;
        } else {
            block.header.bits ^= 1;
        }
        while check_block_header(&block.header).is_err() {
            block.header.nonce = block.header.nonce.wrapping_add(1);
        }
        let branch = original.branch();
        let changed = CandidateHistory::check_seeded(
            CanonicalChain::Regtest.spec().rules(),
            block.header.clone(),
            branch[0].clone(),
            branch[1..].to_vec(),
            None,
        )
        .expect("same source-bound branch");
        let fixture = Chainstate::from_snapshot_for_mechanical_fixture(snapshot.clone());
        // Act
        let header = changed.check_contextual_header_rules(i64::from(source.header.time));
        let body =
            fixture.stage_seeded_historical_block(&block, &changed, i64::from(source.header.time));
        // Assert: refusal is through contextual header and the actual body stage.
        let header_error = header.expect_err("changed header context");
        let body_error = body.expect_err("changed body context");
        assert!(
            format!("{body_error:?}").contains(header_error.reject_reason),
            "header={header_error:?}, body={body_error:?}"
        );
        assert_eq!(fixture.snapshot(), snapshot);
        assert!(!fixture.has_earned_work());
    }
}
