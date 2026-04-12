use std::collections::HashMap;

use open_bitcoin_consensus::{
    BlockValidationContext, ConsensusParams, ScriptVerifyFlags, TransactionInputContext,
    TransactionValidationContext, block_hash, check_block_contextual, transaction_txid,
    validate_transaction_with_context,
};
use open_bitcoin_primitives::{Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, Transaction};

use crate::{
    AnchoredBlock, BlockUndo, ChainPosition, ChainTransition, ChainstateError, ChainstateSnapshot,
    Coin, TxUndo,
};

const MEDIAN_TIME_PAST_WINDOW: usize = 11;
const OP_RETURN: u8 = 0x6a;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Chainstate {
    active_chain: Vec<ChainPosition>,
    utxos: HashMap<OutPoint, Coin>,
    undo_by_block: HashMap<BlockHash, BlockUndo>,
}

impl Chainstate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_snapshot(snapshot: ChainstateSnapshot) -> Self {
        Self {
            active_chain: snapshot.active_chain,
            utxos: snapshot.utxos,
            undo_by_block: snapshot.undo_by_block,
        }
    }

    pub fn snapshot(&self) -> ChainstateSnapshot {
        ChainstateSnapshot::new(
            self.active_chain.clone(),
            self.utxos.clone(),
            self.undo_by_block.clone(),
        )
    }

    pub fn tip(&self) -> Option<&ChainPosition> {
        self.active_chain.last()
    }

    pub fn utxos(&self) -> &HashMap<OutPoint, Coin> {
        &self.utxos
    }

    pub fn connect_block(
        &mut self,
        block: &Block,
        chain_work: u128,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ChainPosition, ChainstateError> {
        let expected_previous = self
            .tip()
            .map_or(BlockHash::from_byte_array([0_u8; 32]), |tip| tip.block_hash);
        let actual_previous = block.header.previous_block_hash;
        if actual_previous != expected_previous {
            return Err(ChainstateError::InvalidTipExtension {
                expected_previous,
                actual_previous,
            });
        }

        let height = self.tip().map_or(0, |tip| tip.height.saturating_add(1));
        let previous_header = self
            .tip()
            .map_or_else(BlockHeader::default, |tip| tip.header.clone());
        let previous_median_time_past = self.tip().map_or(0, |tip| tip.median_time_past);
        let block_context = BlockValidationContext {
            height,
            previous_header,
            previous_median_time_past,
            consensus_params,
        };
        check_block_contextual(block, &block_context)
            .map_err(|source| ChainstateError::BlockValidation { source })?;

        let mut next_utxos = self.utxos.clone();
        let mut block_undo = BlockUndo::default();
        let block_time = i64::from(block.header.time);
        for (transaction_index, transaction) in block.transactions.iter().enumerate() {
            if transaction_index > 0 {
                let transaction_context = build_transaction_context(
                    transaction,
                    &next_utxos,
                    height,
                    block_time,
                    previous_median_time_past,
                    verify_flags,
                    consensus_params,
                )?;
                validate_transaction_with_context(transaction, &transaction_context)
                    .map_err(|source| ChainstateError::TransactionValidation { source })?;

                let mut undo = TxUndo::default();
                for input in &transaction.inputs {
                    let coin = next_utxos
                        .remove(&input.previous_output)
                        .expect("validated transaction inputs must still exist during apply phase");
                    undo.restored_inputs.push(coin);
                }
                block_undo.transactions.push(undo);
            }

            add_transaction_outputs(
                &mut next_utxos,
                transaction,
                height,
                previous_median_time_past,
            )?;
        }

        let median_time_past =
            compute_median_time_past(&self.active_chain, Some(block.header.time));
        let position =
            ChainPosition::new(block.header.clone(), height, chain_work, median_time_past);
        self.utxos = next_utxos;
        self.undo_by_block.insert(position.block_hash, block_undo);
        self.active_chain.push(position.clone());

        Ok(position)
    }

    pub fn disconnect_tip(&mut self, block: &Block) -> Result<ChainPosition, ChainstateError> {
        let Some(tip) = self.active_chain.last().cloned() else {
            return Err(ChainstateError::MissingTip);
        };
        let block_hash = block_hash(&block.header);
        if block_hash != tip.block_hash {
            return Err(ChainstateError::DisconnectBlockMismatch {
                expected_tip: tip.block_hash,
                actual_block: block_hash,
            });
        }

        let Some(block_undo) = self.undo_by_block.remove(&tip.block_hash) else {
            return Err(ChainstateError::MissingUndo {
                block_hash: tip.block_hash,
            });
        };
        if block.transactions.len().saturating_sub(1) != block_undo.transactions.len() {
            return Err(ChainstateError::UndoMismatch {
                expected_transactions: block.transactions.len().saturating_sub(1),
                actual_transactions: block_undo.transactions.len(),
            });
        }

        for transaction_index in (0..block.transactions.len()).rev() {
            let transaction = &block.transactions[transaction_index];
            remove_transaction_outputs(&mut self.utxos, transaction, tip.height)?;

            if transaction_index > 0 {
                let tx_undo = &block_undo.transactions[transaction_index - 1];
                if tx_undo.restored_inputs.len() != transaction.inputs.len() {
                    return Err(ChainstateError::UndoMismatch {
                        expected_transactions: transaction.inputs.len(),
                        actual_transactions: tx_undo.restored_inputs.len(),
                    });
                }

                for input_index in (0..transaction.inputs.len()).rev() {
                    let outpoint = transaction.inputs[input_index].previous_output.clone();
                    if self.utxos.contains_key(&outpoint) {
                        return Err(ChainstateError::RestoredCoinOverwrite { outpoint });
                    }
                    self.utxos
                        .insert(outpoint, tx_undo.restored_inputs[input_index].clone());
                }
            }
        }

        self.active_chain.pop();
        Ok(tip)
    }

    pub fn reorg(
        &mut self,
        disconnect_blocks: &[Block],
        replacement_branch: &[AnchoredBlock],
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ChainTransition, ChainstateError> {
        if disconnect_blocks.len() > self.active_chain.len() {
            return Err(ChainstateError::DisconnectPastGenesis {
                requested: disconnect_blocks.len(),
                available: self.active_chain.len(),
            });
        }

        let mut transition = ChainTransition::default();
        for block in disconnect_blocks {
            transition.disconnected.push(self.disconnect_tip(block)?);
        }
        for anchored_block in replacement_branch {
            let next_block = &anchored_block.block;
            let next_chain_work = anchored_block.chain_work;
            let position =
                self.connect_block(next_block, next_chain_work, verify_flags, consensus_params)?;
            transition.connected.push(position);
        }

        Ok(transition)
    }
}

pub fn prefer_candidate_tip(current: &ChainPosition, candidate: &ChainPosition) -> bool {
    if candidate.chain_work != current.chain_work {
        return candidate.chain_work > current.chain_work;
    }
    if candidate.height != current.height {
        return candidate.height > current.height;
    }

    candidate.block_hash > current.block_hash
}

fn build_transaction_context(
    transaction: &Transaction,
    utxos: &HashMap<OutPoint, Coin>,
    spend_height: u32,
    block_time: i64,
    median_time_past: i64,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
) -> Result<TransactionValidationContext, ChainstateError> {
    let mut inputs = Vec::with_capacity(transaction.inputs.len());
    for input in &transaction.inputs {
        let Some(coin) = utxos.get(&input.previous_output) else {
            return Err(ChainstateError::MissingCoin {
                outpoint: input.previous_output.clone(),
            });
        };
        inputs.push(TransactionInputContext {
            spent_output: coin.as_spent_output(),
            created_height: coin.created_height,
            created_median_time_past: coin.created_median_time_past,
        });
    }

    Ok(TransactionValidationContext {
        inputs,
        spend_height,
        block_time,
        median_time_past,
        verify_flags,
        consensus_params,
    })
}

fn add_transaction_outputs(
    utxos: &mut HashMap<OutPoint, Coin>,
    transaction: &Transaction,
    height: u32,
    created_median_time_past: i64,
) -> Result<(), ChainstateError> {
    let txid = transaction_txid(transaction)
        .expect("typed transactions should serialize for txid derivation");
    for (vout, output) in transaction.outputs.iter().enumerate() {
        if is_unspendable_script(&output.script_pubkey) {
            continue;
        }

        let outpoint = OutPoint {
            txid,
            vout: vout as u32,
        };
        if utxos.contains_key(&outpoint) {
            return Err(ChainstateError::OutputOverwrite { outpoint });
        }

        utxos.insert(
            outpoint,
            Coin {
                output: output.clone(),
                is_coinbase: transaction.is_coinbase(),
                created_height: height,
                created_median_time_past,
            },
        );
    }

    Ok(())
}

fn remove_transaction_outputs(
    utxos: &mut HashMap<OutPoint, Coin>,
    transaction: &Transaction,
    expected_height: u32,
) -> Result<(), ChainstateError> {
    let txid = transaction_txid(transaction)
        .expect("typed transactions should serialize for txid derivation");
    for (vout, output) in transaction.outputs.iter().enumerate() {
        if is_unspendable_script(&output.script_pubkey) {
            continue;
        }

        let outpoint = OutPoint {
            txid,
            vout: vout as u32,
        };
        let Some(existing_coin) = utxos.remove(&outpoint) else {
            return Err(ChainstateError::DisconnectSpentOutputMismatch { outpoint });
        };
        if existing_coin.output != *output
            || existing_coin.created_height != expected_height
            || existing_coin.is_coinbase != transaction.is_coinbase()
        {
            return Err(ChainstateError::DisconnectSpentOutputMismatch { outpoint });
        }
    }

    Ok(())
}

fn compute_median_time_past(active_chain: &[ChainPosition], maybe_new_time: Option<u32>) -> i64 {
    let mut times: Vec<u32> = active_chain
        .iter()
        .rev()
        .take(MEDIAN_TIME_PAST_WINDOW)
        .map(|position| position.header.time)
        .collect();
    if let Some(new_time) = maybe_new_time {
        times.push(new_time);
    }
    if times.is_empty() {
        return 0;
    }

    times.sort_unstable();
    i64::from(times[times.len() / 2])
}

fn is_unspendable_script(script_pubkey: &ScriptBuf) -> bool {
    script_pubkey
        .as_bytes()
        .first()
        .is_some_and(|opcode| *opcode == OP_RETURN)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use open_bitcoin_consensus::{
        ConsensusParams, ScriptVerifyFlags, block_merkle_root, check_block_header,
    };
    use open_bitcoin_primitives::{
        Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
        TransactionInput, TransactionOutput, Txid,
    };

    use super::{Chainstate, compute_median_time_past, prefer_candidate_tip};
    use crate::{AnchoredBlock, BlockUndo, ChainPosition, Coin, TxUndo};

    const EASY_BITS: u32 = 0x207f_ffff;

    fn script(bytes: &[u8]) -> ScriptBuf {
        ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
    }

    fn serialized_script_num(value: i64) -> Vec<u8> {
        if value == 0 {
            return vec![0x00];
        }

        let negative = value < 0;
        let mut magnitude = value.unsigned_abs();
        let mut encoded = Vec::new();
        while magnitude > 0 {
            encoded.push((magnitude & 0xff) as u8);
            magnitude >>= 8;
        }

        if encoded.last().is_some_and(|byte| (byte & 0x80) != 0) {
            encoded.push(if negative { 0x80 } else { 0x00 });
        } else if negative {
            let last = encoded.last_mut().expect("value is non-zero");
            *last |= 0x80;
        }

        let mut script = Vec::with_capacity(encoded.len() + 1);
        script.push(encoded.len() as u8);
        script.extend(encoded);
        script
    }

    fn coinbase_transaction(height: u32, value: i64) -> Transaction {
        let mut script_sig = serialized_script_num(i64::from(height));
        script_sig.push(0x51);
        Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: OutPoint::null(),
                script_sig: script(&script_sig),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(value).expect("valid amount"),
                script_pubkey: script(&[0x51]),
            }],
            lock_time: 0,
        }
    }

    fn spend_transaction(
        previous_txid: Txid,
        previous_vout: u32,
        value: i64,
        sequence: u32,
    ) -> Transaction {
        Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: previous_txid,
                    vout: previous_vout,
                },
                script_sig: script(&[0x51]),
                sequence,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(value).expect("valid amount"),
                script_pubkey: script(&[0x51]),
            }],
            lock_time: 0,
        }
    }

    fn op_return_transaction(previous_txid: Txid) -> Transaction {
        Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: previous_txid,
                    vout: 0,
                },
                script_sig: script(&[0x51]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(10).expect("valid amount"),
                script_pubkey: script(&[0x6a, 0x01, 0x01]),
            }],
            lock_time: 0,
        }
    }

    fn mine_header(block: &mut Block) {
        block.header.nonce = (0..=u32::MAX)
            .find(|nonce| {
                block.header.nonce = *nonce;
                check_block_header(&block.header).is_ok()
            })
            .expect("expected to find a nonce for easy regtest target");
    }

    fn build_block(
        previous_block_hash: BlockHash,
        time: u32,
        transactions: Vec<Transaction>,
    ) -> Block {
        let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
        assert!(!maybe_mutated);

        let mut block = Block {
            header: BlockHeader {
                version: 1,
                previous_block_hash,
                merkle_root,
                time,
                bits: EASY_BITS,
                nonce: 0,
            },
            transactions,
        };
        mine_header(&mut block);
        block
    }

    fn connect_block(
        chainstate: &mut Chainstate,
        block: &Block,
        chain_work: u128,
    ) -> ChainPosition {
        chainstate
            .connect_block(
                block,
                chain_work,
                ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                    | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..ConsensusParams::default()
                },
            )
            .expect("block should connect")
    }

    fn assert_active_tip(chainstate: &Chainstate, expected: &ChainPosition) {
        assert_eq!(chainstate.tip(), Some(expected));
    }

    #[test]
    fn derives_contexts_from_chainstate_metadata() {
        // Arrange
        let mut chainstate = Chainstate::new();
        let genesis_coinbase = coinbase_transaction(0, 50);
        let genesis_block = build_block(
            BlockHash::from_byte_array([0_u8; 32]),
            1_231_006_500,
            vec![genesis_coinbase.clone()],
        );
        let genesis_position = connect_block(&mut chainstate, &genesis_block, 1);
        let spend = spend_transaction(
            Txid::from_byte_array(
                open_bitcoin_consensus::transaction_txid(&genesis_coinbase)
                    .expect("txid")
                    .to_byte_array(),
            ),
            0,
            40,
            1,
        );
        let block = build_block(
            genesis_position.block_hash,
            1_231_006_600,
            vec![coinbase_transaction(1, 50), spend],
        );

        // Act
        let next_position = connect_block(&mut chainstate, &block, 2);

        // Assert
        assert_eq!(next_position.height, 1);
        let spendable = chainstate
            .utxos()
            .values()
            .find(|coin| !coin.is_coinbase)
            .expect("expected transaction output to be added");
        assert_eq!(spendable.created_height, 1);
        assert_eq!(
            spendable.created_median_time_past,
            genesis_position.median_time_past
        );
    }

    #[test]
    fn connect_and_disconnect_round_trip_utxos_and_tip() {
        // Arrange
        let mut chainstate = Chainstate::new();
        let genesis_coinbase = coinbase_transaction(0, 50);
        let genesis_block = build_block(
            BlockHash::from_byte_array([0_u8; 32]),
            1_231_006_500,
            vec![genesis_coinbase.clone()],
        );
        let genesis_position = connect_block(&mut chainstate, &genesis_block, 1);
        let spend = spend_transaction(
            open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid"),
            0,
            40,
            TransactionInput::SEQUENCE_FINAL,
        );
        let block = build_block(
            genesis_position.block_hash,
            1_231_006_600,
            vec![coinbase_transaction(1, 50), spend],
        );
        let connected_position = connect_block(&mut chainstate, &block, 2);

        // Act
        let disconnected = chainstate
            .disconnect_tip(&block)
            .expect("block should disconnect cleanly");

        // Assert
        assert_eq!(disconnected, connected_position);
        assert_active_tip(&chainstate, &genesis_position);
        assert_eq!(chainstate.utxos().len(), 1);
    }

    #[test]
    fn reorg_prefers_heavier_branch_and_preserves_expected_utxos() {
        // Arrange
        let mut chainstate = Chainstate::new();
        let genesis_coinbase = coinbase_transaction(0, 50);
        let genesis_block = build_block(
            BlockHash::from_byte_array([0_u8; 32]),
            1_231_006_500,
            vec![genesis_coinbase.clone()],
        );
        let genesis_position = connect_block(&mut chainstate, &genesis_block, 1);

        let branch_a_coinbase = coinbase_transaction(1, 50);
        let branch_a = build_block(
            genesis_position.block_hash,
            1_231_006_600,
            vec![branch_a_coinbase.clone()],
        );
        let branch_a_position = connect_block(&mut chainstate, &branch_a, 2);

        let branch_b_spend = spend_transaction(
            open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid"),
            0,
            30,
            TransactionInput::SEQUENCE_FINAL,
        );
        let branch_b = build_block(
            genesis_position.block_hash,
            1_231_006_650,
            vec![coinbase_transaction(1, 50), branch_b_spend],
        );
        let branch_b_tip = ChainPosition::new(branch_b.header.clone(), 1, 3, 1_231_006_650);
        assert!(prefer_candidate_tip(&branch_a_position, &branch_b_tip));

        // Act
        let transition = chainstate
            .reorg(
                std::slice::from_ref(&branch_a),
                &[AnchoredBlock {
                    block: branch_b.clone(),
                    chain_work: 3,
                }],
                ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                    | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..ConsensusParams::default()
                },
            )
            .expect("reorg should succeed");

        // Assert
        assert_eq!(transition.disconnected, vec![branch_a_position]);
        assert_eq!(transition.connected.len(), 1);
        assert_eq!(chainstate.tip(), Some(&transition.connected[0]));
        assert_eq!(chainstate.utxos().len(), 2);
    }

    #[test]
    fn connect_block_rejects_premature_coinbase_spend() {
        // Arrange
        let mut chainstate = Chainstate::new();
        let genesis_coinbase = coinbase_transaction(0, 50);
        let genesis_block = build_block(
            BlockHash::from_byte_array([0_u8; 32]),
            1_231_006_500,
            vec![genesis_coinbase.clone()],
        );
        let genesis_position = connect_block(&mut chainstate, &genesis_block, 1);
        let premature_spend = spend_transaction(
            open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid"),
            0,
            40,
            TransactionInput::SEQUENCE_FINAL,
        );
        let block = build_block(
            genesis_position.block_hash,
            1_231_006_600,
            vec![coinbase_transaction(1, 50), premature_spend],
        );

        // Act
        let error = chainstate
            .connect_block(
                &block,
                2,
                ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                    | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
                ConsensusParams::default(),
            )
            .expect_err("premature coinbase spend must fail");

        // Assert
        assert!(matches!(
            error,
            crate::ChainstateError::TransactionValidation { .. }
        ));
    }

    #[test]
    fn connect_block_rejects_missing_prevouts_from_chainstate() {
        let mut chainstate = Chainstate::new();
        let genesis_block = build_block(
            BlockHash::from_byte_array([0_u8; 32]),
            1_231_006_500,
            vec![coinbase_transaction(0, 50)],
        );
        connect_block(&mut chainstate, &genesis_block, 1);

        let missing_prevout = spend_transaction(
            Txid::from_byte_array([4_u8; 32]),
            0,
            40,
            TransactionInput::SEQUENCE_FINAL,
        );
        let block = build_block(
            open_bitcoin_consensus::block_hash(&genesis_block.header),
            1_231_006_600,
            vec![coinbase_transaction(1, 50), missing_prevout],
        );

        let error = chainstate
            .connect_block(
                &block,
                2,
                ScriptVerifyFlags::P2SH,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..ConsensusParams::default()
                },
            )
            .expect_err("missing prevout must fail before mutation");

        assert!(matches!(error, crate::ChainstateError::MissingCoin { .. }));
    }

    #[test]
    fn connect_block_skips_unspendable_outputs() {
        // Arrange
        let mut chainstate = Chainstate::new();
        let genesis_coinbase = coinbase_transaction(0, 50);
        let genesis_block = build_block(
            BlockHash::from_byte_array([0_u8; 32]),
            1_231_006_500,
            vec![genesis_coinbase.clone()],
        );
        let genesis_position = connect_block(&mut chainstate, &genesis_block, 1);
        let op_return = op_return_transaction(
            open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid"),
        );
        let block = build_block(
            genesis_position.block_hash,
            1_231_006_700,
            vec![coinbase_transaction(1, 50), op_return],
        );

        // Act
        connect_block(&mut chainstate, &block, 2);

        // Assert
        assert_eq!(chainstate.utxos().len(), 1);
    }

    #[test]
    fn disconnect_tip_skips_unspendable_outputs_and_reports_missing_created_outputs() {
        let mut chainstate = Chainstate::new();
        let genesis_coinbase = coinbase_transaction(0, 50);
        let genesis_block = build_block(
            BlockHash::from_byte_array([0_u8; 32]),
            1_231_006_500,
            vec![genesis_coinbase.clone()],
        );
        let genesis_position = connect_block(&mut chainstate, &genesis_block, 1);
        let op_return = op_return_transaction(
            open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid"),
        );
        let op_return_block = build_block(
            genesis_position.block_hash,
            1_231_006_700,
            vec![coinbase_transaction(1, 50), op_return],
        );
        let op_return_position = connect_block(&mut chainstate, &op_return_block, 2);

        let disconnected = chainstate
            .disconnect_tip(&op_return_block)
            .expect("disconnect should ignore unspendable outputs");
        assert_eq!(disconnected, op_return_position);

        let spend_block = build_block(
            genesis_position.block_hash,
            1_231_006_600,
            vec![
                coinbase_transaction(1, 50),
                spend_transaction(
                    open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid"),
                    0,
                    40,
                    TransactionInput::SEQUENCE_FINAL,
                ),
            ],
        );
        let missing_created_output = Chainstate {
            active_chain: vec![ChainPosition::new(spend_block.header.clone(), 1, 2, 1)],
            utxos: HashMap::new(),
            undo_by_block: HashMap::from([(
                open_bitcoin_consensus::block_hash(&spend_block.header),
                BlockUndo {
                    transactions: vec![TxUndo {
                        restored_inputs: vec![Coin {
                            output: genesis_block.transactions[0].outputs[0].clone(),
                            is_coinbase: true,
                            created_height: 0,
                            created_median_time_past: 0,
                        }],
                    }],
                },
            )]),
        }
        .disconnect_tip(&spend_block)
        .expect_err("missing created spendable outputs should fail");
        assert!(matches!(
            missing_created_output,
            crate::ChainstateError::DisconnectSpentOutputMismatch { .. }
        ));
    }

    #[test]
    fn median_time_past_uses_the_last_window_of_times() {
        // Arrange
        let positions = (0..12_u32)
            .map(|index| {
                ChainPosition::new(
                    BlockHeader {
                        version: 1,
                        previous_block_hash: BlockHash::from_byte_array([index as u8; 32]),
                        merkle_root: Default::default(),
                        time: index + 10,
                        bits: EASY_BITS,
                        nonce: 0,
                    },
                    index,
                    u128::from(index),
                    i64::from(index + 10),
                )
            })
            .collect::<Vec<_>>();

        // Act
        let median = compute_median_time_past(&positions, None);

        // Assert
        assert_eq!(median, 16);
    }

    #[test]
    fn median_time_past_returns_zero_for_an_empty_chain() {
        assert_eq!(compute_median_time_past(&[], None), 0);
    }

    #[test]
    fn snapshot_round_trip_preserves_accessors() {
        let mut chainstate = Chainstate::new();
        let genesis_block = build_block(
            BlockHash::from_byte_array([0_u8; 32]),
            1_231_006_500,
            vec![coinbase_transaction(0, 50)],
        );
        let genesis_position = connect_block(&mut chainstate, &genesis_block, 1);

        let snapshot = chainstate.snapshot();
        let restored = Chainstate::from_snapshot(snapshot.clone());

        assert_eq!(snapshot.tip(), Some(&genesis_position));
        assert_eq!(restored.tip(), Some(&genesis_position));
        assert_eq!(restored.utxos(), chainstate.utxos());
    }

    #[test]
    fn connect_block_rejects_invalid_tip_extensions() {
        let mut chainstate = Chainstate::new();
        let block = build_block(
            BlockHash::from_byte_array([1_u8; 32]),
            1_231_006_500,
            vec![coinbase_transaction(0, 50)],
        );

        let error = chainstate
            .connect_block(
                &block,
                1,
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default(),
            )
            .expect_err("wrong parent hash must fail");

        assert!(matches!(
            error,
            crate::ChainstateError::InvalidTipExtension { .. }
        ));
    }

    #[test]
    fn disconnect_tip_rejects_missing_tip_and_missing_undo() {
        let mut empty = Chainstate::new();
        let genesis_block = build_block(
            BlockHash::from_byte_array([0_u8; 32]),
            1_231_006_500,
            vec![coinbase_transaction(0, 50)],
        );
        let missing_tip = empty
            .disconnect_tip(&genesis_block)
            .expect_err("empty chain should reject disconnect");
        assert!(matches!(missing_tip, crate::ChainstateError::MissingTip));

        let tip = ChainPosition::new(genesis_block.header.clone(), 0, 1, 1);
        let mut chainstate = Chainstate {
            active_chain: vec![tip.clone()],
            utxos: HashMap::new(),
            undo_by_block: HashMap::new(),
        };
        let missing_undo = chainstate
            .disconnect_tip(&genesis_block)
            .expect_err("missing undo should fail");

        assert!(matches!(
            missing_undo,
            crate::ChainstateError::MissingUndo { block_hash } if block_hash == tip.block_hash
        ));
    }

    #[test]
    fn disconnect_tip_detects_mismatches_and_corrupt_undo_shapes() {
        let genesis_coinbase = coinbase_transaction(0, 50);
        let genesis_block = build_block(
            BlockHash::from_byte_array([0_u8; 32]),
            1_231_006_500,
            vec![genesis_coinbase.clone()],
        );
        let spend = spend_transaction(
            open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid"),
            0,
            40,
            TransactionInput::SEQUENCE_FINAL,
        );
        let block = build_block(
            open_bitcoin_consensus::block_hash(&genesis_block.header),
            1_231_006_600,
            vec![coinbase_transaction(1, 50), spend],
        );
        let tip = ChainPosition::new(block.header.clone(), 1, 2, 1);

        let mismatch = Chainstate {
            active_chain: vec![tip.clone()],
            utxos: HashMap::new(),
            undo_by_block: HashMap::new(),
        }
        .disconnect_tip(&genesis_block)
        .expect_err("wrong block should fail");
        assert!(matches!(
            mismatch,
            crate::ChainstateError::DisconnectBlockMismatch { .. }
        ));

        let undo_shape = Chainstate {
            active_chain: vec![tip.clone()],
            utxos: HashMap::new(),
            undo_by_block: HashMap::from([(tip.block_hash, BlockUndo::default())]),
        }
        .disconnect_tip(&block)
        .expect_err("corrupt top-level undo shape should fail");
        assert!(matches!(
            undo_shape,
            crate::ChainstateError::UndoMismatch { .. }
        ));

        let inner_undo_shape = Chainstate {
            active_chain: vec![tip.clone()],
            utxos: HashMap::from([
                (
                    OutPoint {
                        txid: open_bitcoin_consensus::transaction_txid(&block.transactions[0])
                            .expect("txid"),
                        vout: 0,
                    },
                    Coin {
                        output: block.transactions[0].outputs[0].clone(),
                        is_coinbase: true,
                        created_height: 1,
                        created_median_time_past: 1,
                    },
                ),
                (
                    OutPoint {
                        txid: open_bitcoin_consensus::transaction_txid(&block.transactions[1])
                            .expect("txid"),
                        vout: 0,
                    },
                    Coin {
                        output: block.transactions[1].outputs[0].clone(),
                        is_coinbase: false,
                        created_height: 1,
                        created_median_time_past: 1,
                    },
                ),
            ]),
            undo_by_block: HashMap::from([(
                tip.block_hash,
                BlockUndo {
                    transactions: vec![TxUndo::default()],
                },
            )]),
        }
        .disconnect_tip(&block)
        .expect_err("corrupt inner undo shape should fail");
        assert!(matches!(
            inner_undo_shape,
            crate::ChainstateError::UndoMismatch { .. }
        ));
    }

    #[test]
    fn disconnect_tip_detects_restore_and_output_integrity_failures() {
        let genesis_coinbase = coinbase_transaction(0, 50);
        let genesis_block = build_block(
            BlockHash::from_byte_array([0_u8; 32]),
            1_231_006_500,
            vec![genesis_coinbase.clone()],
        );
        let spend = spend_transaction(
            open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid"),
            0,
            40,
            TransactionInput::SEQUENCE_FINAL,
        );
        let block = build_block(
            open_bitcoin_consensus::block_hash(&genesis_block.header),
            1_231_006_600,
            vec![coinbase_transaction(1, 50), spend.clone()],
        );
        let tip = ChainPosition::new(block.header.clone(), 1, 2, 1);
        let spend_outpoint = spend.inputs[0].previous_output.clone();
        let created_coinbase_outpoint = OutPoint {
            txid: open_bitcoin_consensus::transaction_txid(&block.transactions[0]).expect("txid"),
            vout: 0,
        };
        let created_spend_outpoint = OutPoint {
            txid: open_bitcoin_consensus::transaction_txid(&block.transactions[1]).expect("txid"),
            vout: 0,
        };

        let restore_overwrite = Chainstate {
            active_chain: vec![tip.clone()],
            utxos: HashMap::from([
                (
                    created_coinbase_outpoint.clone(),
                    Coin {
                        output: block.transactions[0].outputs[0].clone(),
                        is_coinbase: true,
                        created_height: 1,
                        created_median_time_past: 1,
                    },
                ),
                (
                    created_spend_outpoint.clone(),
                    Coin {
                        output: block.transactions[1].outputs[0].clone(),
                        is_coinbase: false,
                        created_height: 1,
                        created_median_time_past: 1,
                    },
                ),
                (
                    spend_outpoint.clone(),
                    Coin {
                        output: block.transactions[1].outputs[0].clone(),
                        is_coinbase: false,
                        created_height: 0,
                        created_median_time_past: 0,
                    },
                ),
            ]),
            undo_by_block: HashMap::from([(
                tip.block_hash,
                BlockUndo {
                    transactions: vec![TxUndo {
                        restored_inputs: vec![Coin {
                            output: genesis_block.transactions[0].outputs[0].clone(),
                            is_coinbase: true,
                            created_height: 0,
                            created_median_time_past: 0,
                        }],
                    }],
                },
            )]),
        }
        .disconnect_tip(&block)
        .expect_err("restoring into an occupied outpoint should fail");
        assert!(matches!(
            restore_overwrite,
            crate::ChainstateError::RestoredCoinOverwrite { .. }
        ));

        let mismatch_block = build_block(
            open_bitcoin_consensus::block_hash(&genesis_block.header),
            1_231_006_600,
            vec![coinbase_transaction(1, 50)],
        );
        let mismatch_tip = ChainPosition::new(mismatch_block.header.clone(), 1, 2, 1);
        let mismatch_coinbase_outpoint = OutPoint {
            txid: open_bitcoin_consensus::transaction_txid(&mismatch_block.transactions[0])
                .expect("txid"),
            vout: 0,
        };
        let output_mismatch = Chainstate {
            active_chain: vec![mismatch_tip],
            utxos: HashMap::from([(
                mismatch_coinbase_outpoint,
                Coin {
                    output: mismatch_block.transactions[0].outputs[0].clone(),
                    is_coinbase: true,
                    created_height: 999,
                    created_median_time_past: 1,
                },
            )]),
            undo_by_block: HashMap::from([(
                open_bitcoin_consensus::block_hash(&mismatch_block.header),
                BlockUndo::default(),
            )]),
        }
        .disconnect_tip(&mismatch_block)
        .expect_err("mismatched created output metadata should fail");
        assert!(matches!(
            output_mismatch,
            crate::ChainstateError::DisconnectSpentOutputMismatch { .. }
        ));
    }

    #[test]
    fn reorg_and_tip_preference_cover_remaining_decision_branches() {
        let candidate_same_work_higher_height = ChainPosition::new(
            BlockHeader {
                version: 1,
                previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
                merkle_root: Default::default(),
                time: 2,
                bits: EASY_BITS,
                nonce: 0,
            },
            2,
            5,
            2,
        );
        let current_same_work_lower_height = ChainPosition::new(
            BlockHeader {
                version: 1,
                previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
                merkle_root: Default::default(),
                time: 1,
                bits: EASY_BITS,
                nonce: 0,
            },
            1,
            5,
            1,
        );
        assert!(prefer_candidate_tip(
            &current_same_work_lower_height,
            &candidate_same_work_higher_height,
        ));

        let current_same_height = ChainPosition::new(
            BlockHeader {
                version: 1,
                previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
                merkle_root: Default::default(),
                time: 3,
                bits: EASY_BITS,
                nonce: 0,
            },
            2,
            5,
            3,
        );
        let candidate_same_height = ChainPosition::new(
            BlockHeader {
                version: 1,
                previous_block_hash: BlockHash::from_byte_array([1_u8; 32]),
                merkle_root: Default::default(),
                time: 4,
                bits: EASY_BITS,
                nonce: 0,
            },
            2,
            5,
            4,
        );
        assert_eq!(
            prefer_candidate_tip(&current_same_height, &candidate_same_height),
            candidate_same_height.block_hash > current_same_height.block_hash
        );

        let mut empty = Chainstate::new();
        let error = empty
            .reorg(
                &[build_block(
                    BlockHash::from_byte_array([0_u8; 32]),
                    1_231_006_500,
                    vec![coinbase_transaction(0, 50)],
                )],
                &[],
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default(),
            )
            .expect_err("cannot disconnect past genesis");
        assert!(matches!(
            error,
            crate::ChainstateError::DisconnectPastGenesis { .. }
        ));
    }

    #[test]
    fn script_num_helper_covers_negative_and_high_bit_cases() {
        assert_eq!(serialized_script_num(-1), vec![1, 0x81]);
        assert_eq!(serialized_script_num(128), vec![2, 0x80, 0x00]);
    }
}
