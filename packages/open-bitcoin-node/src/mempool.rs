use open_bitcoin_core::{
    consensus::{ConsensusParams, ScriptVerifyFlags},
    primitives::Transaction,
};
use open_bitcoin_mempool::{AdmissionResult, Mempool, MempoolError, PolicyConfig};

use crate::{ChainstateStore, ManagedChainstate};

#[derive(Debug, Clone)]
pub struct ManagedMempool {
    mempool: Mempool,
}

impl Default for ManagedMempool {
    fn default() -> Self {
        Self::new(PolicyConfig::default())
    }
}

impl ManagedMempool {
    pub fn new(config: PolicyConfig) -> Self {
        Self {
            mempool: Mempool::new(config),
        }
    }

    pub fn mempool(&self) -> &Mempool {
        &self.mempool
    }

    pub fn submit_transaction<S: ChainstateStore>(
        &mut self,
        chainstate: &ManagedChainstate<S>,
        transaction: Transaction,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<AdmissionResult, MempoolError> {
        self.mempool.accept_transaction(
            transaction,
            &chainstate.chainstate().snapshot(),
            verify_flags,
            consensus_params,
        )
    }
}

#[cfg(test)]
mod tests {
    use open_bitcoin_core::consensus::crypto::hash160;
    use open_bitcoin_core::{
        consensus::{
            ConsensusParams, ScriptVerifyFlags, block_merkle_root, check_block_header,
            transaction_txid,
        },
        primitives::{
            Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
            TransactionInput, TransactionOutput,
        },
    };

    use crate::{ManagedChainstate, ManagedMempool, MemoryChainstateStore};

    const EASY_BITS: u32 = 0x207f_ffff;

    fn script(bytes: &[u8]) -> ScriptBuf {
        ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
    }

    fn redeem_script() -> ScriptBuf {
        script(&[0x51])
    }

    fn p2sh_script() -> ScriptBuf {
        let redeem_hash = hash160(redeem_script().as_bytes());
        let mut bytes = vec![0xa9, 20];
        bytes.extend_from_slice(&redeem_hash);
        bytes.push(0x87);
        script(&bytes)
    }

    fn serialized_script_num(value: i64) -> Vec<u8> {
        if value == 0 {
            return vec![0x00];
        }

        let mut magnitude = value as u64;
        let mut encoded = Vec::new();
        while magnitude > 0 {
            encoded.push((magnitude & 0xff) as u8);
            magnitude >>= 8;
        }

        let mut script = Vec::with_capacity(encoded.len() + 2);
        script.push(encoded.len() as u8);
        script.extend(encoded);
        script.push(0x51);
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
                script_pubkey: p2sh_script(),
            }],
            lock_time: 0,
        }
    }

    fn spend_transaction(
        previous_txid: open_bitcoin_core::primitives::Txid,
        value: i64,
    ) -> Transaction {
        Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: previous_txid,
                    vout: 0,
                },
                script_sig: script(&[0x01, 0x51]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(value).expect("valid amount"),
                script_pubkey: p2sh_script(),
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
            .expect("expected nonce at easy target");
    }

    fn build_block(previous_block_hash: BlockHash, height: u32, value: i64) -> Block {
        let transactions = vec![coinbase_transaction(height, value)];
        let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
        assert!(!maybe_mutated);

        let mut block = Block {
            header: BlockHeader {
                version: 1,
                previous_block_hash,
                merkle_root,
                time: 1_231_006_500 + height,
                bits: EASY_BITS,
                nonce: 0,
            },
            transactions,
        };
        mine_header(&mut block);
        block
    }

    #[test]
    fn managed_mempool_submits_against_managed_chainstate() {
        let mut chainstate = ManagedChainstate::from_store(MemoryChainstateStore::default());
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
        let spendable = build_block(
            open_bitcoin_core::consensus::block_hash(&genesis.header),
            1,
            500_000_000,
        );
        chainstate
            .connect_block(
                &genesis,
                1,
                ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                    | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..ConsensusParams::default()
                },
            )
            .expect("genesis should connect");
        chainstate
            .connect_block(
                &spendable,
                2,
                ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                    | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..ConsensusParams::default()
                },
            )
            .expect("follow-up block should connect");

        let mut mempool = ManagedMempool::default();
        let result = mempool
            .submit_transaction(
                &chainstate,
                spend_transaction(
                    transaction_txid(&genesis.transactions[0]).expect("txid"),
                    499_999_000,
                ),
                ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                    | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..ConsensusParams::default()
                },
            )
            .expect("transaction should be admitted");

        assert!(mempool.mempool().entry(&result.accepted).is_some());
    }
}
