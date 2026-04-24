use std::sync::OnceLock;

use open_bitcoin_chainstate::{Chainstate, ChainstateSnapshot};
use open_bitcoin_consensus::crypto::hash160;
use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, TransactionInputContext, block_hash, block_merkle_root,
    check_block_header, transaction_txid,
};
use open_bitcoin_primitives::{
    Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
    TransactionInput, TransactionOutput, Txid,
};

use crate::error::BenchError;

const EASY_BITS: u32 = 0x207f_ffff;
const GENESIS_TIME: u32 = 1_231_006_500;

const BLOCK_HEADER_HEX: &str = include_str!("../../open-bitcoin-codec/testdata/block_header.hex");
const TRANSACTION_VALID_HEX: &str =
    include_str!("../../open-bitcoin-codec/testdata/transaction_valid.hex");
const MESSAGE_HEADER_HEX: &str =
    include_str!("../../open-bitcoin-codec/testdata/message_header.hex");

#[derive(Debug, Clone)]
pub struct BenchFixtures {
    pub consensus: ConsensusFixtures,
    pub codec: CodecFixtures,
    pub chainstate: ChainstateFixtures,
    pub mempool: MempoolFixtures,
    pub network: NetworkFixtures,
}

#[derive(Debug, Clone)]
pub struct ConsensusFixtures {
    pub script_sig: ScriptBuf,
    pub script_pubkey: ScriptBuf,
    pub sigops_script: ScriptBuf,
}

#[derive(Debug, Clone)]
pub struct CodecFixtures {
    pub block_header_bytes: Vec<u8>,
    pub transaction_bytes: Vec<u8>,
    pub message_header_bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ChainstateFixtures {
    pub genesis_snapshot: ChainstateSnapshot,
    pub branch_a_snapshot: ChainstateSnapshot,
    pub branch_a: Block,
    pub branch_b: Block,
}

#[derive(Debug, Clone)]
pub struct MempoolFixtures {
    pub snapshot: ChainstateSnapshot,
    pub standard_spend: Transaction,
    pub input_contexts: Vec<TransactionInputContext>,
}

#[derive(Debug, Clone)]
pub struct NetworkFixtures {
    pub active_chain: Vec<open_bitcoin_chainstate::ChainPosition>,
}

impl BenchFixtures {
    pub fn shared() -> Result<&'static Self, BenchError> {
        static FIXTURES: OnceLock<Result<BenchFixtures, String>> = OnceLock::new();

        match FIXTURES.get_or_init(|| Self::build().map_err(|error| error.to_string())) {
            Ok(fixtures) => Ok(fixtures),
            Err(reason) => Err(BenchError::case_failed(
                "benchmark-fixtures",
                reason.clone(),
            )),
        }
    }

    fn build() -> Result<Self, BenchError> {
        let consensus = ConsensusFixtures {
            script_sig: script(&[0x51])?,
            script_pubkey: script(&[0x51])?,
            sigops_script: script(&[0xac, 0x51, 0xae])?,
        };
        let codec = CodecFixtures {
            block_header_bytes: decode_hex_fixture("block_header.hex", BLOCK_HEADER_HEX)?,
            transaction_bytes: decode_hex_fixture("transaction_valid.hex", TRANSACTION_VALID_HEX)?,
            message_header_bytes: decode_hex_fixture("message_header.hex", MESSAGE_HEADER_HEX)?,
        };
        let chainstate = chainstate_fixtures()?;
        let mempool = mempool_fixtures()?;
        let network = NetworkFixtures {
            active_chain: chainstate.branch_a_snapshot.active_chain.clone(),
        };

        Ok(Self {
            consensus,
            codec,
            chainstate,
            mempool,
            network,
        })
    }
}

pub(crate) fn script(bytes: &[u8]) -> Result<ScriptBuf, BenchError> {
    ScriptBuf::from_bytes(bytes.to_vec())
        .map_err(|error| BenchError::case_failed("fixture-script", error.to_string()))
}

pub(crate) fn decode_hex_fixture(name: &'static str, input: &str) -> Result<Vec<u8>, BenchError> {
    let hex = input.trim();
    if !hex.len().is_multiple_of(2) {
        return Err(BenchError::case_failed(
            name,
            "hex fixture has an odd number of digits",
        ));
    }

    let mut bytes = Vec::with_capacity(hex.len() / 2);
    let mut index = 0;
    while index < hex.len() {
        let end = index + 2;
        let byte = u8::from_str_radix(&hex[index..end], 16)
            .map_err(|error| BenchError::case_failed(name, error.to_string()))?;
        bytes.push(byte);
        index = end;
    }
    Ok(bytes)
}

pub(crate) fn consensus_params() -> ConsensusParams {
    ConsensusParams {
        coinbase_maturity: 1,
        ..ConsensusParams::default()
    }
}

pub(crate) fn verify_flags() -> ScriptVerifyFlags {
    ScriptVerifyFlags::P2SH
        | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
        | ScriptVerifyFlags::CHECKSEQUENCEVERIFY
}

fn chainstate_fixtures() -> Result<ChainstateFixtures, BenchError> {
    let true_script = script(&[0x51])?;
    let genesis_coinbase = coinbase_transaction(0, 500_000_000, true_script.clone())?;
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        GENESIS_TIME,
        vec![genesis_coinbase.clone()],
    )?;
    let mut chainstate = Chainstate::new();
    connect_block(&mut chainstate, &genesis_block, 1)?;
    let genesis_snapshot = chainstate.snapshot();

    let branch_a = build_block(
        block_hash(&genesis_block.header),
        GENESIS_TIME + 100,
        vec![coinbase_transaction(1, 500_000_000, true_script.clone())?],
    )?;
    connect_block(&mut chainstate, &branch_a, 2)?;
    let branch_a_snapshot = chainstate.snapshot();

    let branch_b = build_block(
        block_hash(&genesis_block.header),
        GENESIS_TIME + 150,
        vec![
            coinbase_transaction(1, 500_000_000, true_script.clone())?,
            spend_transaction(
                transaction_txid(&genesis_coinbase).map_err(|error| {
                    BenchError::case_failed("chainstate-fixture", error.to_string())
                })?,
                0,
                499_999_000,
                true_script.clone(),
                script(&[0x51])?,
                TransactionInput::SEQUENCE_FINAL,
            )?,
        ],
    )?;

    Ok(ChainstateFixtures {
        genesis_snapshot,
        branch_a_snapshot,
        branch_a,
        branch_b,
    })
}

fn mempool_fixtures() -> Result<MempoolFixtures, BenchError> {
    let p2sh_script = p2sh_script()?;
    let (snapshot, txids) = sample_chainstate_snapshot(2, p2sh_script.clone())?;
    let Some(previous_txid) = txids.first().copied() else {
        return Err(BenchError::case_failed(
            "mempool-fixture",
            "sample chain did not produce coinbase txids",
        ));
    };
    let standard_spend = spend_transaction(
        previous_txid,
        0,
        499_999_000,
        p2sh_script,
        script(&[0x01, 0x51])?,
        TransactionInput::SEQUENCE_FINAL,
    )?;
    let input_contexts = input_contexts(&snapshot, &standard_spend)?;

    Ok(MempoolFixtures {
        snapshot,
        standard_spend,
        input_contexts,
    })
}

fn sample_chainstate_snapshot(
    block_count: u32,
    output_script: ScriptBuf,
) -> Result<(ChainstateSnapshot, Vec<Txid>), BenchError> {
    let mut chainstate = Chainstate::new();
    let mut previous_hash = BlockHash::from_byte_array([0_u8; 32]);
    let mut txids = Vec::new();

    for height in 0..block_count {
        let block = build_block(
            previous_hash,
            GENESIS_TIME + height,
            vec![coinbase_transaction(
                height,
                500_000_000,
                output_script.clone(),
            )?],
        )?;
        let txid = transaction_txid(&block.transactions[0])
            .map_err(|error| BenchError::case_failed("chainstate-fixture", error.to_string()))?;
        txids.push(txid);
        connect_block(&mut chainstate, &block, u128::from(height + 1))?;
        previous_hash = block_hash(&block.header);
    }

    Ok((chainstate.snapshot(), txids))
}

fn input_contexts(
    snapshot: &ChainstateSnapshot,
    transaction: &Transaction,
) -> Result<Vec<TransactionInputContext>, BenchError> {
    let mut contexts = Vec::with_capacity(transaction.inputs.len());
    for input in &transaction.inputs {
        let Some(coin) = snapshot.utxos.get(&input.previous_output) else {
            return Err(BenchError::case_failed(
                "mempool-fixture",
                "transaction input missing from fixture snapshot",
            ));
        };
        contexts.push(TransactionInputContext {
            spent_output: coin.as_spent_output(),
            created_height: coin.created_height,
            created_median_time_past: coin.created_median_time_past,
        });
    }
    Ok(contexts)
}

fn p2sh_script() -> Result<ScriptBuf, BenchError> {
    let redeem_script = script(&[0x51])?;
    let redeem_hash = hash160(redeem_script.as_bytes());
    let mut bytes = vec![0xa9, 20];
    bytes.extend_from_slice(&redeem_hash);
    bytes.push(0x87);
    script(&bytes)
}

fn coinbase_transaction(
    height: u32,
    value: i64,
    script_pubkey: ScriptBuf,
) -> Result<Transaction, BenchError> {
    let mut script_sig = serialized_script_num(i64::from(height));
    script_sig.push(0x51);

    Ok(Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: script(&script_sig)?,
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: amount(value)?,
            script_pubkey,
        }],
        lock_time: 0,
    })
}

fn spend_transaction(
    previous_txid: Txid,
    vout: u32,
    value: i64,
    script_pubkey: ScriptBuf,
    script_sig: ScriptBuf,
    sequence: u32,
) -> Result<Transaction, BenchError> {
    Ok(Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout,
            },
            script_sig,
            sequence,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: amount(value)?,
            script_pubkey,
        }],
        lock_time: 0,
    })
}

fn build_block(
    previous_block_hash: BlockHash,
    time: u32,
    transactions: Vec<Transaction>,
) -> Result<Block, BenchError> {
    let (merkle_root, maybe_mutated) = block_merkle_root(&transactions)
        .map_err(|error| BenchError::case_failed("block-fixture", error.to_string()))?;
    if maybe_mutated {
        return Err(BenchError::case_failed(
            "block-fixture",
            "fixture block has a mutated merkle tree",
        ));
    }

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
    mine_header(&mut block)?;
    Ok(block)
}

fn connect_block(
    chainstate: &mut Chainstate,
    block: &Block,
    chain_work: u128,
) -> Result<(), BenchError> {
    chainstate
        .connect_block(block, chain_work, verify_flags(), consensus_params())
        .map(|_| ())
        .map_err(|error| BenchError::case_failed("chainstate-fixture", error.to_string()))
}

fn mine_header(block: &mut Block) -> Result<(), BenchError> {
    for nonce in 0..=u32::MAX {
        block.header.nonce = nonce;
        if check_block_header(&block.header).is_ok() {
            return Ok(());
        }
    }

    Err(BenchError::case_failed(
        "block-fixture",
        "could not mine fixture block at easy target",
    ))
}

fn amount(sats: i64) -> Result<Amount, BenchError> {
    Amount::from_sats(sats)
        .map_err(|error| BenchError::case_failed("fixture-amount", error.to_string()))
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
    script
}

#[cfg(test)]
mod tests {
    use super::decode_hex_fixture;

    #[test]
    fn decode_hex_fixture_reports_invalid_hex() {
        // Arrange / Act
        let result = decode_hex_fixture("bad.hex", "0");

        // Assert
        assert!(result.is_err());
    }
}
