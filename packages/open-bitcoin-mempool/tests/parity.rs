use open_bitcoin_chainstate::{Chainstate, ChainstateSnapshot};
use open_bitcoin_consensus::crypto::hash160;
use open_bitcoin_consensus::{
    ConsensusParams, ScriptVerifyFlags, block_merkle_root, check_block_header, transaction_txid,
};
use open_bitcoin_mempool::{
    LimitDirection, LimitKind, Mempool, MempoolError, PolicyConfig, RbfPolicy,
};
use open_bitcoin_primitives::{
    Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
    TransactionInput, TransactionOutput, Txid,
};

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

fn spend_transaction(previous_txid: Txid, output_value: i64, sequence: u32) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout: 0,
            },
            script_sig: script(&[0x01, 0x51]),
            sequence,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(output_value).expect("valid amount"),
            script_pubkey: p2sh_script(),
        }],
        lock_time: 0,
    }
}

fn non_standard_spend(previous_txid: Txid) -> Transaction {
    let mut transaction =
        spend_transaction(previous_txid, 499_000_000, TransactionInput::SEQUENCE_FINAL);
    transaction.outputs[0].script_pubkey = script(&[0x51]);
    transaction
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

fn mine_header(block: &mut Block) {
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("expected nonce at easy target");
}

fn sample_chainstate_snapshot(block_count: u32) -> (ChainstateSnapshot, Vec<Txid>) {
    let mut chainstate = Chainstate::new();
    let mut previous_hash = BlockHash::from_byte_array([0_u8; 32]);
    let mut txids = Vec::new();

    for height in 0..block_count {
        let block = build_block(previous_hash, height, 500_000_000);
        let txid = transaction_txid(&block.transactions[0]).expect("txid");
        txids.push(txid);
        chainstate
            .connect_block(
                &block,
                u128::from(height + 1),
                ScriptVerifyFlags::P2SH
                    | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                    | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..ConsensusParams::default()
                },
            )
            .expect("block should connect");
        previous_hash = open_bitcoin_consensus::block_hash(&block.header);
    }

    (chainstate.snapshot(), txids)
}

fn submit(
    mempool: &mut Mempool,
    snapshot: &ChainstateSnapshot,
    transaction: Transaction,
) -> Result<open_bitcoin_mempool::AdmissionResult, MempoolError> {
    mempool.accept_transaction(
        transaction,
        snapshot,
        ScriptVerifyFlags::P2SH
            | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
            | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
        ConsensusParams {
            coinbase_maturity: 1,
            ..ConsensusParams::default()
        },
    )
}

#[test]
fn standard_admission_tracks_public_entry_metrics() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let mut mempool = Mempool::default();

    let result = submit(
        &mut mempool,
        &snapshot,
        spend_transaction(
            coinbase_txids[0],
            499_999_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect("standard tx should be admitted");
    let entry = mempool.entry(&result.accepted).expect("entry");

    assert!(result.replaced.is_empty());
    assert!(result.evicted.is_empty());
    assert_eq!(entry.ancestor_stats.count, 1);
    assert_eq!(entry.descendant_stats.count, 1);
}

#[test]
fn non_standard_outputs_fail_public_api_admission() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(2);
    let error = submit(
        &mut Mempool::default(),
        &snapshot,
        non_standard_spend(coinbase_txids[0]),
    )
    .expect_err("non-standard outputs should fail");

    assert!(matches!(error, MempoolError::NonStandard { .. }));
}

#[test]
fn replacement_requires_fee_bump_and_reports_replaced_txids() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let original = spend_transaction(
        coinbase_txids[0],
        499_999_000,
        TransactionInput::MAX_SEQUENCE_NONFINAL - 1,
    );
    let lower_fee_replacement = spend_transaction(
        coinbase_txids[0],
        499_999_100,
        TransactionInput::SEQUENCE_FINAL,
    );
    let higher_fee_replacement = spend_transaction(
        coinbase_txids[0],
        499_998_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut mempool = Mempool::new(PolicyConfig {
        rbf_policy: RbfPolicy::OptIn,
        ..PolicyConfig::default()
    });

    let original_result = submit(&mut mempool, &snapshot, original).expect("original");
    let lower_error = submit(&mut mempool, &snapshot, lower_fee_replacement)
        .expect_err("replacement should fail");
    let higher_result =
        submit(&mut mempool, &snapshot, higher_fee_replacement).expect("replacement");

    assert!(matches!(
        lower_error,
        MempoolError::ReplacementRejected { .. }
    ));
    assert_eq!(higher_result.replaced, vec![original_result.accepted]);
}

#[test]
fn ancestor_limit_and_eviction_truths_hold_through_public_api() {
    let (snapshot, coinbase_txids) = sample_chainstate_snapshot(3);
    let parent = spend_transaction(
        coinbase_txids[0],
        499_999_000,
        TransactionInput::SEQUENCE_FINAL,
    );
    let parent_txid = transaction_txid(&parent).expect("txid");
    let child = spend_transaction(parent_txid, 499_998_000, TransactionInput::SEQUENCE_FINAL);
    let mut strict_mempool = Mempool::new(PolicyConfig {
        max_ancestor_count: 1,
        ..PolicyConfig::default()
    });

    submit(&mut strict_mempool, &snapshot, parent.clone()).expect("parent");
    let ancestor_error =
        submit(&mut strict_mempool, &snapshot, child).expect_err("ancestor limit should fail");
    assert!(matches!(
        ancestor_error,
        MempoolError::LimitExceeded {
            direction: LimitDirection::Ancestor,
            kind: LimitKind::Count,
            ..
        }
    ));

    let mut trim_mempool = Mempool::new(PolicyConfig {
        max_mempool_virtual_size: 140,
        ..PolicyConfig::default()
    });
    let low_fee_result = submit(
        &mut trim_mempool,
        &snapshot,
        spend_transaction(
            coinbase_txids[0],
            499_999_200,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect("low fee");
    let high_fee_result = submit(
        &mut trim_mempool,
        &snapshot,
        spend_transaction(
            coinbase_txids[1],
            499_998_000,
            TransactionInput::SEQUENCE_FINAL,
        ),
    )
    .expect("high fee");

    assert_eq!(high_fee_result.evicted, vec![low_fee_result.accepted]);
}
