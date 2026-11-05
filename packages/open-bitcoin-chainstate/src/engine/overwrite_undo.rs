// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/undo.h

//! Pinned pre-block BIP30 scan and intentionally stronger CVLD06 lossless undo.
//! Knots normal undo does not restore overwritten coinbases. These records do.

use super::{apply, candidate_rules::context_error};
use crate::coins::{CoinsOverlay, CoinsView};
use crate::types::{BlockOverwriteUndo, OverwrittenCoin};
use crate::{BlockUndo, ChainPosition, ChainstateError, Coin};
use open_bitcoin_consensus::crypto::Sha256;
use open_bitcoin_consensus::{CandidateHistory, block_hash, transaction_txid};
use open_bitcoin_primitives::{Block, OutPoint, Transaction};
use std::collections::BTreeSet;

pub(super) fn capture<V: CoinsView>(
    overlay: &CoinsOverlay,
    parent: &V,
    block: &Block,
    height: u32,
    creating_mtp: i64,
    maybe_history: Option<&CandidateHistory>,
) -> Result<BlockOverwriteUndo, ChainstateError> {
    let permits_replacement = maybe_history.is_some_and(|history| {
        history.candidate() == &block.header
            && history.height() == height
            && history.parent_median_time_past() == creating_mtp
            && !history.rules().requires_bip30_scan()
    });
    let mut batch = BlockOverwriteUndo {
        block_hash: block_hash(&block.header),
        height,
        creating_median_time_past: creating_mtp,
        overwritten: Vec::new(),
        integrity: [0; 32],
    };
    // Scan the entire pre-block overlay before ANY transaction may spend a duplicate.
    for (index, tx) in block.transactions.iter().enumerate() {
        let txid = transaction_txid(tx).map_err(apply::txid_serialization_error)?;
        for (vout, output) in tx.outputs.iter().enumerate() {
            let outpoint = OutPoint {
                txid,
                vout: vout as u32,
            };
            let Some(previous_coin) = overlay.peek_coin(parent, &outpoint)? else {
                continue;
            };
            if !permits_replacement
                || index != 0
                || !tx.is_coinbase()
                || !previous_coin.is_coinbase
                || previous_coin.output != *output
                || previous_coin.created_height >= height
                || apply::is_unspendable_script(&output.script_pubkey)
            {
                return Err(ChainstateError::OutputOverwrite { outpoint });
            }
            batch.overwritten.push(OverwrittenCoin {
                transaction_index: index as u32,
                outpoint,
                previous_coin,
                new_coin: Coin {
                    output: output.clone(),
                    is_coinbase: true,
                    created_height: height,
                    created_median_time_past: creating_mtp,
                },
            });
        }
    }
    batch.integrity = integrity(&batch);
    Ok(batch)
}

fn coin_bytes(bytes: &mut Vec<u8>, coin: &Coin) {
    bytes.extend(coin.output.value.to_sats().to_le_bytes());
    let script = coin.output.script_pubkey.as_bytes();
    bytes.extend((script.len() as u64).to_le_bytes());
    bytes.extend(script);
    bytes.push(u8::from(coin.is_coinbase));
    bytes.extend(coin.created_height.to_le_bytes());
    bytes.extend(coin.created_median_time_past.to_le_bytes());
}
fn integrity(batch: &BlockOverwriteUndo) -> [u8; 32] {
    let mut bytes = b"open-bitcoin/complete-overwrite-undo/v1".to_vec();
    bytes.extend(batch.block_hash.to_byte_array());
    bytes.extend(batch.height.to_le_bytes());
    bytes.extend(batch.creating_median_time_past.to_le_bytes());
    bytes.extend((batch.overwritten.len() as u64).to_le_bytes());
    for entry in &batch.overwritten {
        bytes.extend(entry.transaction_index.to_le_bytes());
        bytes.extend(entry.outpoint.txid.to_byte_array());
        bytes.extend(entry.outpoint.vout.to_le_bytes());
        coin_bytes(&mut bytes, &entry.previous_coin);
        coin_bytes(&mut bytes, &entry.new_coin);
    }
    Sha256::digest(&bytes)
}

impl BlockOverwriteUndo {
    /// Validate serialized-field integrity only. A self-consistent imported batch
    /// remains untrusted and cannot establish canonical ancestry or acceptance.
    pub fn validate_integrity(&self) -> Result<(), ChainstateError> {
        if self.integrity != integrity(self) {
            return Err(context_error("overwrite-provenance-integrity"));
        }
        Ok(())
    }
}

impl BlockUndo {
    /// Validate complete batch shape and exact block/occurrence binding only.
    /// This neither proves the prior coins nor authorizes imported state; durable
    /// recovery must still replay and compare through the existing validation owner.
    pub fn validate_overwrite_binding(
        &self,
        block: &Block,
        tip: &ChainPosition,
    ) -> Result<(), ChainstateError> {
        check(self, block, tip)
    }
}

pub(super) fn check(
    undo: &BlockUndo,
    block: &Block,
    tip: &ChainPosition,
) -> Result<(), ChainstateError> {
    let [batch] = undo.overwrites.as_slice() else {
        return Err(context_error("missing-or-duplicate-overwrite-provenance"));
    };
    if batch.block_hash != block_hash(&block.header)
        || batch.block_hash != tip.block_hash
        || batch.height != tip.height
    {
        return Err(context_error("overwrite-provenance-binding"));
    }
    batch.validate_integrity()?;
    let mut seen = BTreeSet::new();
    for entry in &batch.overwritten {
        let tx = block
            .transactions
            .get(entry.transaction_index as usize)
            .ok_or_else(|| context_error("overwrite-transaction-index"))?;
        let output = tx
            .outputs
            .get(entry.outpoint.vout as usize)
            .ok_or_else(|| context_error("overwrite-output-index"))?;
        let expected_txid = transaction_txid(tx).map_err(apply::txid_serialization_error)?;
        if !seen.insert(entry.outpoint.clone())
            || entry.transaction_index != 0
            || !tx.is_coinbase()
            || entry.outpoint.txid != expected_txid
            || entry.new_coin.output != *output
            || !entry.new_coin.is_coinbase
            || entry.new_coin.created_height != tip.height
            || entry.new_coin.created_median_time_past != batch.creating_median_time_past
            || entry.previous_coin.output != *output
            || !entry.previous_coin.is_coinbase
            || entry.previous_coin.created_height >= tip.height
        {
            return Err(context_error("overwrite-occurrence-mismatch"));
        }
    }
    Ok(())
}

/// Replace only an exact captured pre-block coin; no caller overwrite switch exists.
pub(super) fn replace_outputs<V: CoinsView>(
    overlay: &mut CoinsOverlay,
    parent: &V,
    tx: &Transaction,
    batch: &BlockOverwriteUndo,
) -> Result<(), ChainstateError> {
    let txid = transaction_txid(tx).map_err(apply::txid_serialization_error)?;
    for entry in batch.overwritten.iter().filter(|e| e.outpoint.txid == txid) {
        if overlay.peek_coin(parent, &entry.outpoint)?.as_ref() != Some(&entry.previous_coin) {
            return Err(ChainstateError::OutputOverwrite {
                outpoint: entry.outpoint.clone(),
            });
        }
        overlay.spend_coin(parent, &entry.outpoint)?;
    }
    Ok(())
}

/// Restore captured complete Coins after check_new_coins and transaction removal.
/// Callers operate on private overlays and publish only if all restoration succeeds.
pub(super) fn restore_overwritten_coins<V: CoinsView>(
    overlay: &mut CoinsOverlay,
    parent: &V,
    undo: &BlockUndo,
    block: &Block,
    tip: &ChainPosition,
) -> Result<(), ChainstateError> {
    check(undo, block, tip)?;
    for entry in &undo.overwrites[0].overwritten {
        if overlay.peek_coin(parent, &entry.outpoint)?.is_some() {
            return Err(ChainstateError::RestoredCoinOverwrite {
                outpoint: entry.outpoint.clone(),
            });
        }
    }
    for entry in &undo.overwrites[0].overwritten {
        overlay.add_coin(
            parent,
            entry.outpoint.clone(),
            entry.previous_coin.clone(),
            false,
        )?;
    }
    Ok(())
}

pub(super) fn check_new_coins<V: CoinsView>(
    overlay: &CoinsOverlay,
    parent: &V,
    undo: &BlockUndo,
    block: &Block,
    tip: &ChainPosition,
) -> Result<(), ChainstateError> {
    check(undo, block, tip)?;
    for entry in &undo.overwrites[0].overwritten {
        if overlay.peek_coin(parent, &entry.outpoint)?.as_ref() != Some(&entry.new_coin) {
            return Err(ChainstateError::DisconnectSpentOutputMismatch {
                outpoint: entry.outpoint.clone(),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::tests::phase167_bip30::{DATA, fixture, load};
    use super::*;
    use crate::{Chainstate, ChainstateSnapshot};
    use open_bitcoin_consensus::{Bip30Disposition, CanonicalChain, HistoryError, HistoryHeader};
    use open_bitcoin_primitives::{Amount, BlockHash, BlockHeader, Txid};
    use std::collections::HashMap;

    #[test]
    fn phase167_bip30_complete_scan_is_not_missing_or_default_provenance() {
        // Arrange
        let (cases, _) = load(DATA).expect("fixture");
        for case in cases {
            let mut chain = fixture(&case);
            chain
                .coins
                .spend_coin(&case.old.0)
                .expect("fully spent original");
            let before = chain.snapshot();
            // Act
            let stage = chain
                .stage_seeded_historical_block(&case.block, &case.history, case.now)
                .expect("fully spent txid allowed");
            // Assert
            assert_eq!(stage.undo().overwrites.len(), 1);
            assert!(stage.undo().overwrites[0].overwritten.is_empty());
            chain.absorb_staged_connect(stage).expect("absorb");
            let connected = chain.snapshot();
            chain
                .disconnect_tip(&case.block)
                .expect("captured empty scan");
            assert_eq!(chain.snapshot(), before);
            let mut missing = connected;
            missing
                .undo_by_block
                .get_mut(&case.history.candidate_hash())
                .expect("undo")
                .overwrites
                .clear();
            let mut missing = Chainstate::from_snapshot_for_mechanical_fixture(missing);
            let before = missing.snapshot();
            assert!(missing.disconnect_tip(&case.block).is_err());
            assert_eq!(missing.snapshot(), before);
        }
        assert!(BlockUndo::default().overwrites.is_empty());
    }

    #[test]
    fn phase167_bip30_corrupt_restoration_never_changes_accepted_state() {
        // Arrange
        let (cases, _) = load(DATA).expect("fixture");
        for case in cases {
            let mut chain = fixture(&case);
            let stage = chain
                .stage_seeded_historical_block(&case.block, &case.history, case.now)
                .expect("repeat");
            chain.absorb_staged_connect(stage).expect("absorb");
            for mutation in 0..19 {
                let mut snapshot = chain.snapshot();
                let hash = case.history.candidate_hash();
                let undo = snapshot.undo_by_block.get_mut(&hash).expect("undo");
                match mutation {
                    0 => {
                        snapshot.undo_by_block.remove(&hash);
                    }
                    1 => undo.overwrites.clear(),
                    2 => undo.overwrites.push(undo.overwrites[0].clone()),
                    3 => undo.overwrites[0].overwritten.clear(),
                    4 => {
                        let entry = undo.overwrites[0].overwritten[0].clone();
                        undo.overwrites[0].overwritten.push(entry);
                    }
                    5 => {
                        undo.overwrites[0].block_hash =
                            open_bitcoin_consensus::block_hash(&case.original.header)
                    }
                    6 => undo.overwrites[0].height -= 1,
                    7 => undo.overwrites[0].overwritten[0].outpoint.vout = 1,
                    8 => undo.overwrites[0].overwritten[0].outpoint.txid = Txid::default(),
                    9 => {
                        undo.overwrites[0].overwritten[0]
                            .previous_coin
                            .created_median_time_past += 1
                    }
                    10 => undo.overwrites[0].overwritten[0].previous_coin.is_coinbase = false,
                    11 => {
                        undo.overwrites[0].overwritten[0]
                            .new_coin
                            .created_median_time_past += 1
                    }
                    12 => {
                        snapshot
                            .utxos
                            .get_mut(&case.old.0)
                            .expect("new coin")
                            .created_median_time_past += 1;
                    }
                    13 => undo.overwrites[0].integrity[0] ^= 1,
                    14 => undo.overwrites[0].overwritten[0].transaction_index = 1,
                    15 => {
                        let entry = undo.overwrites[0].overwritten[0].clone();
                        undo.overwrites[0].overwritten.push(entry);
                    }
                    16 => {
                        undo.overwrites[0].overwritten[0]
                            .previous_coin
                            .created_height = case.history.height()
                    }
                    17 => undo.overwrites[0].overwritten[0].outpoint.vout = 1,
                    _ => {
                        assert_eq!(mutation, 18, "finite mutation range");
                        undo.overwrites[0].overwritten[0].new_coin.output.value =
                            Amount::from_sats(1).expect("amount")
                    }
                }
                if mutation >= 15 {
                    let batch = &mut snapshot
                        .undo_by_block
                        .get_mut(&hash)
                        .expect("undo")
                        .overwrites[0];
                    batch.integrity = integrity(batch);
                }
                let mut damaged = Chainstate::from_snapshot_for_mechanical_fixture(snapshot);
                let before = damaged.snapshot();
                // Act / Assert
                assert!(
                    damaged.disconnect_tip(&case.block).is_err(),
                    "mutation {mutation}"
                );
                assert_eq!(damaged.snapshot(), before, "mutation {mutation}");
            }
            let mut overlay = CoinsOverlay::new();
            let undo = chain
                .undo_by_block
                .get(&case.history.candidate_hash())
                .expect("undo");
            // A restored-input collision must not be silently overwritten by the helper.
            let before = chain.snapshot();
            let error = restore_overwritten_coins(
                &mut overlay,
                &chain.coins,
                undo,
                &case.block,
                chain.tip().expect("tip"),
            )
            .expect_err("occupied new occurrence");
            assert!(matches!(
                error,
                ChainstateError::RestoredCoinOverwrite { .. }
            ));
            assert_eq!(chain.snapshot(), before);
        }
    }

    #[test]
    fn phase167_bip30_wrong_envelopes_and_missing_inputs_refuse_atomically() {
        // Arrange
        let (cases, _) = load(DATA).expect("fixture");
        for case in cases {
            let mut chain = fixture(&case);
            let before = chain.snapshot();
            // Act / Assert
            let mut wrong_body = case.block.clone();
            wrong_body.transactions[0].outputs[0].value = Amount::from_sats(1).expect("amount");
            assert!(
                chain
                    .stage_seeded_historical_block(&wrong_body, &case.history, case.now)
                    .is_err()
            );
            let mut wrong_header = case.block.clone();
            wrong_header.header.nonce ^= 1;
            assert!(
                chain
                    .stage_seeded_historical_block(&wrong_header, &case.history, case.now)
                    .is_err()
            );
            assert_eq!(chain.snapshot(), before);
            chain.coins.get_coin(&case.old.0).expect("read");
            if case.block.transactions.len() == 2 {
                chain
                    .coins
                    .spend_coin(&case.block.transactions[1].inputs[0].previous_output)
                    .expect("external");
                let before = chain.snapshot();
                assert!(
                    chain
                        .stage_seeded_historical_block(&case.block, &case.history, case.now)
                        .is_err()
                );
                assert_eq!(chain.snapshot(), before);
            }
        }
    }

    // These linked synthetic suffixes exercise only structural rule/kernel formulas.
    // They are not PoW stages, canonical ancestry, or fabricated historical blocks.
    fn probe(
        height: u32,
        maybe_seed: Option<HistoryHeader>,
    ) -> Result<CandidateHistory, HistoryError> {
        let mut ascending = Vec::new();
        let mut previous = BlockHash::default();
        for h in height - 155..height {
            let header = BlockHeader {
                previous_block_hash: previous,
                time: h + 1,
                bits: 0x1d00ffff,
                nonce: h,
                ..BlockHeader::default()
            };
            let row = HistoryHeader::new(CanonicalChain::Mainnet, h, header);
            previous = row.block_hash;
            ascending.push(row);
        }
        let candidate = BlockHeader {
            previous_block_hash: previous,
            time: height + 1,
            ..BlockHeader::default()
        };
        ascending.reverse();
        CandidateHistory::check_seeded(
            CanonicalChain::Mainnet.spec().rules(),
            candidate,
            ascending[0].clone(),
            ascending[1..].to_vec(),
            maybe_seed,
        )
    }

    #[test]
    fn phase167_bip30_bip34_match_wrong_absent_and_recheck_kernel_controls() {
        // Arrange
        let (cases, anchor) = load(DATA).expect("detached real anchor");
        let mut wrong = anchor.clone();
        wrong.header.nonce ^= 1;
        wrong.block_hash = block_hash(&wrong.header);
        let matching = probe(1_983_701, Some(anchor)).expect("seeded formulas");
        let nonmatching = probe(1_983_701, Some(wrong)).expect("conditional wrong branch");
        let recheck = probe(1_983_702, None).expect("anchor unneeded");
        let absent = probe(227_930, None).expect("genuinely before activation");
        // Act / Assert
        assert!(matches!(
            probe(1_983_701, None),
            Err(HistoryError::MissingAncestor { height: 227931 })
        ));
        for (history, expected) in [
            (matching, Bip30Disposition::Bip34Commitment),
            (nonmatching, Bip30Disposition::ScanRequired),
            (recheck, Bip30Disposition::ScanRequired),
            (absent, Bip30Disposition::ScanRequired),
        ] {
            assert_eq!(history.rules().bip30_disposition(), expected);
            let tx = cases[0].block.transactions[0].clone();
            let point = OutPoint {
                txid: transaction_txid(&tx).expect("txid"),
                vout: 0,
            };
            let old = Coin {
                output: tx.outputs[0].clone(),
                is_coinbase: true,
                created_height: 1,
                created_median_time_past: 1,
            };
            let chain = Chainstate::from_snapshot_for_mechanical_fixture(ChainstateSnapshot::new(
                Vec::new(),
                HashMap::from([(point.clone(), old)]),
                HashMap::new(),
            ));
            let block = Block {
                header: history.candidate().clone(),
                transactions: vec![tx],
            };
            let before = chain.snapshot();
            let result = capture(
                &CoinsOverlay::new(),
                &chain.coins,
                &block,
                history.height(),
                history.parent_median_time_past(),
                Some(&history),
            );
            assert_eq!(
                result.is_ok(),
                expected == Bip30Disposition::Bip34Commitment
            );
            assert_eq!(chain.snapshot(), before);
            let mut overlay = CoinsOverlay::new();
            overlay
                .spend_coin(&chain.coins, &point)
                .expect("fully spent");
            assert!(
                capture(
                    &overlay,
                    &chain.coins,
                    &block,
                    history.height(),
                    history.parent_median_time_past(),
                    Some(&history)
                )
                .expect("fully spent distinct")
                .overwritten
                .is_empty()
            );
        }
    }

    #[test]
    fn phase167_bip30_preblock_scan_cannot_be_erased_by_earlier_same_block_spend() {
        // Arrange: a live later transaction's output is spent by an earlier transaction.
        let (cases, _) = load(DATA).expect("fixture");
        let case = &cases[1];
        let mut block = case.block.clone();
        let duplicate = block.transactions[1].clone();
        let point = OutPoint {
            txid: transaction_txid(&duplicate).expect("txid"),
            vout: 0,
        };
        let mut early = duplicate.clone();
        early.inputs[0].previous_output = point.clone();
        block.transactions.insert(1, early);
        let mut chain = fixture(case);
        chain
            .coins
            .add_coin(
                point.clone(),
                Coin {
                    output: duplicate.outputs[0].clone(),
                    is_coinbase: false,
                    created_height: 1,
                    created_median_time_past: 1,
                },
                false,
            )
            .expect("prior duplicate");
        let before = chain.snapshot();
        // Act / Assert: kernel scan precedes body application, even in the repeat context.
        let error = capture(
            &CoinsOverlay::new(),
            &chain.coins,
            &block,
            case.history.height(),
            case.history.parent_median_time_past(),
            Some(&case.history),
        )
        .expect_err("live noncoinbase duplicate");
        assert_eq!(error, ChainstateError::OutputOverwrite { outpoint: point });
        assert_eq!(chain.snapshot(), before);
    }

    #[test]
    fn phase167_bip30_typed_projection_rejects_corrupt_records() {
        // Arrange
        let lines: Vec<_> = DATA.lines().filter(|l| !l.starts_with('#')).collect();
        // Act / Assert: each required record is independently missing, duplicate or tagged wrong.
        for (index, _) in lines.iter().enumerate() {
            let mut missing = lines.clone();
            missing.remove(index);
            assert!(
                load(&(missing.join("\n") + "\n")).is_err(),
                "missing {index}"
            );
            let mut duplicate = lines.clone();
            duplicate.insert(index, lines[index]);
            assert!(
                load(&(duplicate.join("\n") + "\n")).is_err(),
                "duplicate {index}"
            );
        }
        for (from, to) in [
            (
                "OPEN_BITCOIN_PHASE167_BIP30\t1",
                "OPEN_BITCOIN_PHASE167_BIP30\t2",
            ),
            ("\tmainnet\t", "\ttestnet3\t"),
            ("\t5000000000\t", "\t05000000000\t"),
            ("\t4294967295\t", "\t4294967296\t"),
            ("\t1289721830", "\t1289721831"),
            ("\t0456720e1b00\t", "\tGG\t"),
            ("SOURCE\t", "UNKNOWN\t"),
        ] {
            assert!(load(&DATA.replacen(from, to, 1)).is_err(), "{from}");
        }
        assert!(load(&(DATA.to_owned() + "END\t2\n")).is_err());
        assert!(load(&DATA.replace('\n', "\r\n")).is_err());
    }
}
