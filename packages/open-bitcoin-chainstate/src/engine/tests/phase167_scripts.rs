// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp
// - packages/bitcoin-knots/src/script/interpreter.cpp

use super::phase167_context::coinbase_transaction;
use super::phase167_context::{canonical, child, connect, stage, work};
use super::*;
use open_bitcoin_consensus::BlockValidationResult;
use open_bitcoin_consensus::crypto::{Sha256, double_sha256, hash160};
use open_bitcoin_consensus::{transaction_txid, transaction_wtxid};

fn all_flags_except(except: ScriptVerifyFlags) -> ScriptVerifyFlags {
    [
        ScriptVerifyFlags::P2SH,
        ScriptVerifyFlags::DERSIG,
        ScriptVerifyFlags::CHECKLOCKTIMEVERIFY,
        ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
        ScriptVerifyFlags::NULLDUMMY,
        ScriptVerifyFlags::WITNESS,
        ScriptVerifyFlags::TAPROOT,
    ]
    .into_iter()
    .filter(|flag| *flag != except)
    .fold(ScriptVerifyFlags::NONE, |flags, flag| flags | flag)
}

pub(super) fn funded(script_bytes: &[u8], outputs: usize) -> (Chainstate, Transaction) {
    let mut chain = canonical();
    let mut funding = coinbase_transaction(1, 100_000);
    funding.outputs = vec![
        TransactionOutput {
            value: Amount::from_sats(100_000).expect("amount"),
            script_pubkey: script(script_bytes)
        };
        outputs
    ];
    let first = child(&chain, vec![funding.clone()]);
    connect(&mut chain, &first);
    for height in 2..=100 {
        let next = child(&chain, vec![coinbase_transaction(height, 1)]);
        connect(&mut chain, &next);
    }
    (chain, funding)
}

pub(super) fn spending(funding: &Transaction) -> Transaction {
    spend_transaction(
        transaction_txid(funding).expect("serialized funding txid"),
        0,
        90_000,
        TransactionInput::SEQUENCE_FINAL,
    )
}

pub(super) fn body(chain: &Chainstate, spend: Transaction) -> Block {
    let height = chain.tip().expect("tip").height + 1;
    let mut coinbase = coinbase_transaction(height, 1);
    if spend.has_witness() {
        coinbase.inputs[0].witness = ScriptWitness::new(vec![vec![0; 32]]);
        let mut root_preimage = vec![0; 32];
        root_preimage.extend_from_slice(
            transaction_wtxid(&spend)
                .expect("serialized witness")
                .as_bytes(),
        );
        let mut commitment_preimage = double_sha256(&root_preimage).to_vec();
        commitment_preimage.extend_from_slice(&[0; 32]);
        let mut commitment_script = vec![0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed];
        commitment_script.extend_from_slice(&double_sha256(&commitment_preimage));
        coinbase.outputs.push(TransactionOutput {
            value: Amount::ZERO,
            script_pubkey: script(&commitment_script),
        });
    }
    child(chain, vec![coinbase, spend])
}

fn flag_refusal(
    spent_script: &[u8],
    unlocking_script: &[u8],
    version: i32,
    flag: ScriptVerifyFlags,
) {
    // Arrange: real accepted funding coin, followed by99 accepted descendants.
    let (mut chain, funding) = funded(spent_script, 1);
    let mut spend = spending(&funding);
    spend.version = version;
    spend.inputs[0].script_sig = script(unlocking_script);
    let block = body(&chain, spend);
    let before = chain.snapshot();
    let best = chain.coins_best_block().expect("best");
    let fixture = Chainstate::from_snapshot_for_mechanical_fixture(before.clone());
    let params = ConsensusParams {
        subsidy_halving_interval: 150,
        ..ConsensusParams::default()
    };
    // Act: removing exactly the named rule permits the same real-coins interpreter spend.
    let bypass = fixture.stage_connect_block_with_current_time(
        &block,
        work(&chain, &block),
        i64::from(block.header.time),
        all_flags_except(flag),
        params,
    );
    chain
        .admit_canonical_header(&block.header, i64::from(block.header.time))
        .expect("header admitted");
    let rejected = stage(&chain, &block);
    // Assert
    assert!(bypass.is_ok(), "single-rule bypass control: {bypass:?}");
    let error = rejected.expect_err("caller NONE must not bypass selected script rules");
    assert!(
        matches!(error, ChainstateError::TransactionValidation { .. }),
        "actual interpreter refusal: {error:?}"
    );
    if flag == ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
        || flag == ScriptVerifyFlags::CHECKSEQUENCEVERIFY
    {
        assert!(
            matches!(error, ChainstateError::TransactionValidation { source } if source.reject_reason == "mandatory-script-verify-flag-failed" && source.debug_message.as_deref() == Some("unsatisfied locktime"))
        );
    }
    assert_eq!(chain.snapshot(), before);
    assert_eq!(chain.coins_best_block().expect("best"), best);
}

#[test]
fn phase167_scripts_p2sh_executes_redeem_script_on_actual_coin() {
    let redeem = [0x00];
    let mut output = vec![0xa9, 20];
    output.extend_from_slice(&hash160(&redeem));
    output.push(0x87);
    flag_refusal(&output, &[1, 0], 2, ScriptVerifyFlags::P2SH);
}

#[test]
fn phase167_scripts_dersig_rejects_malformed_signature_before_checksig_not() {
    let mut unlocking = vec![1, 1, 33, 2];
    unlocking.extend_from_slice(&[1; 32]);
    flag_refusal(&[0xac, 0x91], &unlocking, 2, ScriptVerifyFlags::DERSIG);
}

#[test]
fn phase167_scripts_cltv_enforces_actual_transaction_locktime() {
    flag_refusal(
        &[0x51, 0xb1, 0x75, 0x51],
        &[],
        2,
        ScriptVerifyFlags::CHECKLOCKTIMEVERIFY,
    );
}

#[test]
fn phase167_scripts_csv_enforces_actual_transaction_version() {
    flag_refusal(
        &[0x51, 0xb2, 0x75, 0x51],
        &[],
        1,
        ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
    );
}

fn active_locktime_positive(opcode: u8) {
    // Arrange: both height100 locks are satisfied at spend height101.
    let (mut chain, funding) = funded(&[1, 100, opcode, 0x75, 0x51], 1);
    let mut spend = spending(&funding);
    spend.version = 2;
    spend.lock_time = 100;
    spend.inputs[0].sequence = 100;
    let block = body(&chain, spend);
    // Act
    chain
        .admit_canonical_header(&block.header, i64::from(block.header.time))
        .expect("admit");
    let result = stage(&chain, &block);
    // Assert: OP_DROP also proves that the opcode left its operand intact.
    chain
        .absorb_staged_connect(result.expect("active lock equality succeeds"))
        .expect("accept");
    assert_eq!(chain.tip().expect("tip").height, 101);
}

#[test]
fn phase167_scripts_active_cltv_positive_equality_preserves_operand() {
    active_locktime_positive(0xb1);
}

#[test]
fn phase167_scripts_active_csv_positive_equality_preserves_operand() {
    active_locktime_positive(0xb2);
}

#[test]
fn phase167_scripts_nulldummy_enforces_actual_multisig_dummy() {
    flag_refusal(&[0, 0, 0xae], &[0x51], 2, ScriptVerifyFlags::NULLDUMMY);
}

#[test]
fn phase167_scripts_witness_requires_actual_witness_program_solution() {
    let mut output = vec![0, 32];
    output.extend_from_slice(&Sha256::digest(&[0]));
    flag_refusal(&output, &[], 2, ScriptVerifyFlags::WITNESS);
}

#[test]
fn phase167_scripts_taproot_requires_actual_keypath_solution() {
    let mut output = vec![0x51, 32];
    output.extend_from_slice(&[2; 32]);
    flag_refusal(&output, &[], 2, ScriptVerifyFlags::TAPROOT);
}

#[test]
fn phase167_scripts_aggregate_80000_80001_counts_actual_witness_coins_before_spending() {
    let witness_script = [0, 0x63, 0xac, 0x68, 0x51];
    let mut program = vec![0, 32];
    program.extend_from_slice(&Sha256::digest(&witness_script));
    let (base, funding) = funded(&program, 5);
    for inputs in [4, 5] {
        // Arrange
        let mut chain = base.clone();
        let mut spend = spending(&funding);
        spend.inputs = (0..inputs)
            .map(|vout| TransactionInput {
                previous_output: OutPoint {
                    txid: transaction_txid(&funding).expect("serialized txid"),
                    vout,
                },
                script_sig: ScriptBuf::default(),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::new(vec![witness_script.to_vec()]),
            })
            .collect();
        let mut block = body(&chain, spend);
        for count in [10_000, 9999] {
            block.transactions[0].outputs.push(TransactionOutput {
                value: Amount::ZERO,
                script_pubkey: script(&vec![0xac; count]),
            });
        }
        block = child(&chain, block.transactions);
        let before = chain.snapshot();
        let best = chain.coins_best_block().expect("best");
        chain
            .admit_canonical_header(&block.header, i64::from(block.header.time))
            .expect("admit");
        // Act
        let result = chain.stage_connect_block_with_current_time(
            &block,
            work(&chain, &block),
            -1,
            all_flags_except(ScriptVerifyFlags::NONE),
            ConsensusParams::default(),
        );
        // Assert
        if inputs == 4 {
            chain
                .absorb_staged_connect(result.expect("79996 legacy +4 witness"))
                .expect("accept");
        } else {
            assert!(
                matches!(result, Err(ChainstateError::BlockValidation { source }) if source.reject_reason == "bad-blk-sigops")
            );
            assert_eq!(chain.snapshot(), before);
            assert_eq!(chain.coins_best_block().expect("best"), best);
        }
    }
}

#[test]
fn phase167_scripts_selected_rewards_include_actual_fees_at_halving_boundaries() {
    // Arrange: four independent matured outputs pay exactly10000sats each.
    let (mut chain, funding) = funded(&[0x51], 4);
    let mut vout = 0;
    for height in 101..=300 {
        let reward = match height {
            149 => 5_000_000_000,
            150 | 299 => 2_500_000_000,
            300 => 1_250_000_000,
            _ => {
                let next = child(&chain, vec![coinbase_transaction(height, 1)]);
                connect(&mut chain, &next);
                continue;
            }
        };
        let mut spend = spending(&funding);
        spend.inputs[0].previous_output.vout = vout;
        vout += 1;
        let mut transactions = vec![coinbase_transaction(height, reward + 10_001), spend];
        let excessive = child(&chain, transactions.clone());
        let before = chain.snapshot();
        let best = chain.coins_best_block().expect("best");
        chain
            .admit_canonical_header(&excessive.header, i64::from(excessive.header.time))
            .expect("admit");
        // Act / Assert: subsidy+actualfee+1 refuses without absorbing any state.
        let error = stage(&chain, &excessive).expect_err("one sat over actual reward");
        assert!(
            matches!(error, ChainstateError::BlockValidation { source } if source.reject_reason == "bad-cb-amount")
        );
        assert_eq!(chain.snapshot(), before);
        assert_eq!(chain.coins_best_block().expect("best"), best);
        transactions[0].outputs[0].value = Amount::from_sats(reward + 10_000).expect("reward");
        let valid = child(&chain, transactions);
        connect(&mut chain, &valid);
        assert_eq!(chain.tip().expect("tip").height, height);
    }
}

fn refused(chain: &mut Chainstate, block: &Block, reason: &str) {
    let before = chain.snapshot();
    let best = chain.coins_best_block().expect("best");
    chain
        .admit_canonical_header(&block.header, i64::from(block.header.time))
        .expect("admit");
    let error = stage(chain, block).expect_err("invalid body cannot produce a stage to absorb");
    match error {
        ChainstateError::BlockValidation { source } if reason == "bad-txns-nonfinal" => {
            assert_eq!(source.result, BlockValidationResult::Consensus);
            assert_eq!(source.reject_reason, reason);
        }
        ChainstateError::TransactionValidation { source } => {
            assert_eq!(source.reject_reason, reason)
        }
        ChainstateError::Serialization {
            context,
            reason: actual,
        } => {
            assert_eq!(context, "candidate history");
            assert!(actual.contains(reason), "{actual} versus {reason}");
        }
        other => panic!("unexpected refusal: {other:?}"),
    }
    assert_eq!(
        chain.snapshot(),
        before,
        "tip/full work/all coins/undo unchanged"
    );
    assert_eq!(chain.coins_best_block().expect("best"), best);
}

#[test]
fn phase167_scripts_csv_ignored_bits_and_operand_disable_version1_canonical() {
    for (operand, version, sequence) in [
        (0x0001_0001_i64, 2, 0x0002_0001),
        (0x8000_0000, 1, u32::MAX),
    ] {
        // Arrange: ignored bits differ; operand-disable precedes version/input checks.
        let mut output = serialized_script_num(operand);
        output.extend_from_slice(&[0xb2, 0x75, 0x51]);
        let (mut chain, funding) = funded(&output, 1);
        let mut spend = spending(&funding);
        spend.version = version;
        spend.inputs[0].sequence = sequence;
        // Act
        let block = body(&chain, spend);
        connect(&mut chain, &block);
        // Assert: a real canonical accepted coins transition, including OP_DROP.
        assert_eq!(chain.tip().expect("tip").height, 101);
    }
}

#[test]
fn phase167_scripts_genesis_empty_maturity_99_refuses_100_accepts() {
    // Arrange
    let mut chain = canonical();
    assert!(
        chain.snapshot().utxos.is_empty(),
        "proved genesis adds no coins"
    );
    let funding = coinbase_transaction(1, 100_000);
    let first = child(&chain, vec![funding.clone()]);
    connect(&mut chain, &first);
    for height in 2..100 {
        let next = child(&chain, vec![coinbase_transaction(height, 1)]);
        connect(&mut chain, &next);
    }
    // Act / Assert
    let premature = body(&chain, spending(&funding));
    refused(
        &mut chain,
        &premature,
        "bad-txns-premature-spend-of-coinbase",
    );
    let hundred = child(&chain, vec![coinbase_transaction(100, 1)]);
    connect(&mut chain, &hundred);
    let mature = body(&chain, spending(&funding));
    connect(&mut chain, &mature);
    assert_eq!(chain.tip().expect("tip").height, 101);
}

#[test]
fn phase167_scripts_consumed_creating_mtp_distant_missing_mismatch_and_bound() {
    // Arrange: the height1 coin is farther back than the ordinary155 header suffix.
    let (mut chain, funding) = funded(&[0x51], 1);
    for height in 101..=200 {
        let next = child(&chain, vec![coinbase_transaction(height, 1)]);
        connect(&mut chain, &next);
    }
    let mut timed = spending(&funding);
    timed.inputs[0].sequence = TransactionInput::SEQUENCE_LOCKTIME_TYPE_FLAG | 1;
    let mut valid = chain.clone();
    let block = body(&valid, timed.clone());
    connect(&mut valid, &block);
    assert!(
        !valid
            .have_coin(&timed.inputs[0].previous_output)
            .expect("spent")
    );
    // Act / Assert: missing distant history is needed only by a consumed time input.
    let block = body(&chain, timed.clone());
    let history = super::super::candidate_rules::history_from_chain(
        open_bitcoin_consensus::CanonicalChain::Regtest
            .spec()
            .rules(),
        &block.header,
        &chain.active_chain,
        155,
    )
    .expect("ordinary header suffix");
    let before = chain.snapshot();
    let apply = |block: &Block| {
        super::super::apply::apply_connect_transactions(
            &mut CoinsOverlay::new(),
            &chain.coins,
            block,
            ScriptVerifyFlags::NONE,
            &history.block_validation_context(i64::from(block.header.time)),
            Some(&history),
            &chain.active_chain[30..],
        )
    };
    assert!(
        matches!(
            apply(&block),
            Err(ChainstateError::Serialization {
                context: "candidate history",
                reason,
            }) if reason.contains("MissingAncestor { height: 0 }")
        ),
        "missing consumed history must refuse in the actual coins kernel"
    );
    for (version, sequence) in [
        (1, TransactionInput::SEQUENCE_LOCKTIME_TYPE_FLAG | 1),
        (2, u32::MAX),
        (2, 1),
    ] {
        let mut spend = timed.clone();
        spend.version = version;
        spend.inputs[0].sequence = sequence;
        let block = body(&chain, spend);
        assert!(
            apply(&block).is_ok(),
            "unused distant history is not demanded"
        );
    }
    assert_eq!(
        chain.snapshot(),
        before,
        "private overlays cannot absorb accepted state"
    );
    let mut mismatch = chain.clone();
    let outpoint = timed.inputs[0].previous_output.clone();
    let mut coin = mismatch.coins.spend_coin(&outpoint).expect("actual coin");
    coin.created_median_time_past += 1;
    mismatch
        .coins
        .add_coin(outpoint, coin, false)
        .expect("inconsistent saved metadata fixture");
    let block = body(&mismatch, timed.clone());
    refused(&mut mismatch, &block, "creating-ancestor-mtp-mismatch");
    for height in 201..=4095 {
        let next = child(&chain, vec![coinbase_transaction(height, 1)]);
        connect(&mut chain, &next);
    }
    let mut equality = chain.clone();
    let block = body(&equality, timed.clone());
    connect(&mut equality, &block);
    let next = child(&chain, vec![coinbase_transaction(4096, 1)]);
    connect(&mut chain, &next);
    let block = body(&chain, timed);
    refused(&mut chain, &block, "creating-ancestor-window-bound");
    // No distant lookup for a disabled input even beyond the finite bound.
    let block = body(&chain, spending(&funding));
    connect(&mut chain, &block);
}

#[test]
fn phase167_scripts_same_block_created_parent_mtp_and_relative_locks() {
    // Arrange: output created by the first actual spend is consumed by the second.
    let (base, funding) = funded(&[0x51], 1);
    let first = spending(&funding);
    for relative in [0, 1] {
        let mut chain = base.clone();
        let mut second = spend_transaction(
            transaction_txid(&first).expect("txid"),
            0,
            80_000,
            TransactionInput::SEQUENCE_LOCKTIME_TYPE_FLAG | relative,
        );
        second.inputs[0].script_sig = ScriptBuf::default();
        let block = child(
            &chain,
            vec![coinbase_transaction(101, 1), first.clone(), second.clone()],
        );
        // Act / Assert
        if relative == 1 {
            refused(&mut chain, &block, "bad-txns-nonfinal");
            continue;
        }
        let parent_mtp = chain.tip().expect("tip").median_time_past;
        connect(&mut chain, &block);
        let coin = chain
            .get_coin(&OutPoint {
                txid: transaction_txid(&second).expect("txid"),
                vout: 0,
            })
            .expect("read")
            .expect("new coin");
        assert_eq!(coin.created_median_time_past, parent_mtp);
        assert_eq!(coin.created_height, 101);
    }
}

fn replacement_path(
    base: &Chainstate,
    first: &Transaction,
    short_times: bool,
) -> (Chainstate, Vec<Block>) {
    let mut chain = base.clone();
    let mut blocks = Vec::new();
    for height in 101..=106 {
        let mut transactions = vec![coinbase_transaction(height, 1)];
        if height == 101 {
            transactions.push(first.clone());
        }
        let mut block = child(&chain, transactions);
        if short_times {
            block.header.time = chain.tip().expect("tip").header.time + 1;
            block.header.nonce = 0;
            while check_block_header(&block.header).is_err() {
                block.header.nonce += 1;
            }
        }
        connect(&mut chain, &block);
        blocks.push(block);
    }
    (chain, blocks)
}

#[test]
fn phase167_scripts_replacement_branch_mtp_disagrees_with_active_tip() {
    // Arrange: same creating parent and coins, but only the long-time branch earns7*512s.
    let (base, funding) = funded(&[0x51], 1);
    let first = spending(&funding);
    let (long, long_blocks) = replacement_path(&base, &first, false);
    let (short, short_blocks) = replacement_path(&base, &first, true);
    let spend = spend_transaction(
        transaction_txid(&first).expect("txid"),
        0,
        80_000,
        TransactionInput::SEQUENCE_LOCKTIME_TYPE_FLAG | 7,
    );
    for (mut owner, disconnect, candidate_owner, replacement, accepts) in [
        (
            long.clone(),
            long_blocks.clone(),
            short.clone(),
            short_blocks.clone(),
            false,
        ),
        (short, short_blocks, long, long_blocks, true),
    ] {
        let last = body(&candidate_owner, spend.clone());
        let mut replacements: Vec<_> = replacement
            .into_iter()
            .map(|block| AnchoredBlock {
                chain_work: candidate_owner
                    .active_chain
                    .iter()
                    .find(|row| row.block_hash == open_bitcoin_consensus::block_hash(&block.header))
                    .expect("accepted replacement")
                    .chain_work,
                block,
            })
            .collect();
        replacements.push(AnchoredBlock {
            chain_work: work(&candidate_owner, &last),
            block: last,
        });
        let disconnect: Vec<_> = disconnect.into_iter().rev().collect();
        let headers: Vec<_> = disconnect
            .iter()
            .map(|block| block.header.clone())
            .collect();
        let replacement_headers: Vec<_> = replacements
            .iter()
            .map(|row| (row.block.header.clone(), i64::from(row.block.header.time)))
            .collect();
        let before = owner.snapshot();
        let best = owner.coins_best_block().expect("best");
        owner
            .admit_canonical_reorg_headers(&headers, &replacement_headers)
            .expect("branch headers");
        // Act
        let staged = owner.stage_reorg(
            &disconnect,
            &replacements,
            ScriptVerifyFlags::NONE,
            ConsensusParams::default(),
        );
        // Assert
        if accepts {
            owner
                .absorb_staged_reorg(staged.expect("replacement parent MTP satisfies lock"))
                .expect("absorb");
            assert_eq!(owner.tip().expect("tip").height, 107);
        } else {
            assert!(
                matches!(staged, Err(ChainstateError::ReplacementBlock { source, .. }) if matches!(source.as_ref(), ChainstateError::BlockValidation { source } if source.reject_reason == "bad-txns-nonfinal" && source.result == BlockValidationResult::Consensus))
            );
            assert_eq!(owner.snapshot(), before);
            assert_eq!(owner.coins_best_block().expect("best"), best);
        }
    }
}
