// Parity breadcrumbs:
// - packages/bitcoin-knots/src/pubkey.cpp
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::phase167_context::{connect, stage, work};
use super::phase167_scripts::{body, funded, spending};
use super::*;
use open_bitcoin_consensus::script::{ScriptInputVerificationContext, verify_input_script};
use open_bitcoin_consensus::sighash::{SigHashType, legacy_sighash};
use open_bitcoin_consensus::{
    ScriptError, ScriptExecutionData, TransactionInputContext, TransactionValidationContext,
    transaction_txid,
};

// Generated and independently libsecp-verified by consensus signature/tests/high_s.rs,
// using fixed test key [9;32], height1 coinbase [OP_1,OP_1], 100000sat output0,
// version2 spend, final sequence, 90000sat OP_1 output, lock_time0, SIGHASH_ALL.
// Complete generator bytes and the original genuine RED are in plan23/high-s-red.log.
const PUBLIC_KEY: &str = "0256b328b30c8bf5839e24058747879408bdb36241dc9c2e7c619faa12b2920967";
const CHECKSIG: (&str, &str, &str, &str) = (
    "f39c9c32ea34297127617e00a802a38e16061134ac83d6decca40361b470bf07",
    "6b4720e53089682b9460e73a80635937a0580d4b998fb26c8e2ed3081e9db3f0",
    "3045022100efaa733cc67301db9827637340e3c28a7bc14e39b1decd8fb627530a398dbd4602200b90ccbe96c1f9a1ca03a5aae78fbce7c99ff6a8e071109d5d68783619c78f4e01",
    "3046022100efaa733cc67301db9827637340e3c28a7bc14e39b1decd8fb627530a398dbd46022100f46f3341693e065e35fc5a5518704316f10ee63dced78f9e6269e656b66eb1f301",
);
const MULTISIG: (&str, &str, &str, &str) = (
    "c05aa9de83d840005bfb88627186a8a742c4c655983f08828209d016389425be",
    "3f38a3feef293a712f1894c9b206c1a2121a3b4857288f33d47c813e2e3e1032",
    "3044022074a3eadb54b03cf859b30e3621a3144bb70905cc568cab96810fada9028d1936022071e939d65186901193283b6ffa2178ca0e4ef1f5c545744ed52611da573d689901",
    "3045022074a3eadb54b03cf859b30e3621a3144bb70905cc568cab96810fada9028d19360221008e16c629ae796fee6cd7c49005de8734ac5feaf0ea032beceaac4cb278f8d8a801",
);

fn hex(input: &str) -> Vec<u8> {
    input
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).expect("ASCII hex"), 16).expect("hex")
        })
        .collect()
}

fn explicit_policy(
    chain: &Chainstate,
    transaction: &Transaction,
    flags: ScriptVerifyFlags,
) -> Result<(), ScriptError> {
    let before = chain.snapshot();
    let block = body(chain, transaction.clone());
    let history = super::super::candidate_rules::history_from_chain(
        open_bitcoin_consensus::CanonicalChain::Regtest
            .spec()
            .rules(),
        &block.header,
        &chain.active_chain,
        4096,
    )
    .expect("actual branch");
    let coin = chain
        .get_coin(&transaction.inputs[0].previous_output)
        .expect("read")
        .expect("actual funded coin");
    let context = TransactionValidationContext::from_candidate_history(
        transaction,
        vec![TransactionInputContext {
            spent_output: coin.as_spent_output(),
            created_height: coin.created_height,
            created_median_time_past: coin.created_median_time_past,
        }],
        &history,
    )
    .expect("derived context");
    let precomputed = context.precompute(transaction).expect("precompute");
    let result = verify_input_script(ScriptInputVerificationContext {
        script_sig: &transaction.inputs[0].script_sig,
        script_pubkey: &coin.output.script_pubkey,
        witness: &transaction.inputs[0].witness,
        transaction,
        input_index: 0,
        spent_input: &context.inputs[0],
        validation_context: &context,
        spent_amount: coin.output.value,
        verify_flags: context.verify_flags | flags,
        precomputed: &precomputed,
        execution_data: &mut ScriptExecutionData::default(),
    });
    assert_eq!(chain.snapshot(), before);
    assert_eq!(
        chain.coins_best_block().expect("best"),
        Some(before.active_chain.last().expect("tip").block_hash)
    );
    result
}

fn signed_case(multisig: bool) {
    let vector = if multisig { MULTISIG } else { CHECKSIG };
    let mut output = if multisig { vec![0x51, 33] } else { vec![33] };
    output.extend(hex(PUBLIC_KEY));
    output.extend_from_slice(if multisig { &[0x51, 0xae] } else { &[0xac] });
    let (base, funding) = funded(&output, 1);
    assert_eq!(
        transaction_txid(&funding).expect("txid").as_bytes(),
        hex(vector.0).as_slice()
    );
    for signature in [vector.2, vector.3] {
        // Arrange: recompute the actual sighash before inserting the original DER bytes.
        let mut chain = base.clone();
        let mut spend = spending(&funding);
        assert_eq!(
            legacy_sighash(
                &funding.outputs[0].script_pubkey,
                &spend,
                0,
                SigHashType::ALL
            )
            .as_bytes(),
            hex(vector.1).as_slice()
        );
        let bytes = hex(signature);
        let mut unlocking = if multisig { vec![0] } else { vec![] };
        unlocking.push(bytes.len() as u8);
        unlocking.extend_from_slice(&bytes);
        spend.inputs[0].script_sig = script(&unlocking);
        let before = chain.snapshot();
        // Act / Assert: same actual input under policy before canonical acceptance.
        assert_eq!(
            explicit_policy(&chain, &spend, ScriptVerifyFlags::LOW_S),
            if signature == vector.3 {
                Err(ScriptError::SigHighS)
            } else {
                Ok(())
            }
        );
        assert_eq!(chain.snapshot(), before);
        let mut invalid = spend.clone();
        invalid.outputs[0].value = Amount::from_sats(89_999).expect("amount");
        let invalid_block = body(&chain, invalid);
        chain
            .admit_canonical_header(&invalid_block.header, i64::from(invalid_block.header.time))
            .expect("admit");
        assert!(
            matches!(stage(&chain, &invalid_block), Err(ChainstateError::TransactionValidation { source }) if source.reject_reason == "mandatory-script-verify-flag-failed")
        );
        assert_eq!(chain.snapshot(), before);
        assert_eq!(
            chain.coins_best_block().expect("best"),
            Some(before.active_chain.last().expect("tip").block_hash)
        );
        let valid = body(&chain, spend.clone());
        connect(&mut chain, &valid);
        assert_eq!(spend.inputs[0].script_sig.as_bytes(), unlocking);
        assert!(
            !chain
                .have_coin(&spend.inputs[0].previous_output)
                .expect("spent")
        );
        assert_eq!(chain.tip().expect("tip").height, 101);
    }
}

#[test]
fn phase167_policy_checksig_canonical_high_s_and_explicit_low_s() {
    signed_case(false);
}

#[test]
fn phase167_policy_checkmultisig_canonical_high_s_and_explicit_low_s() {
    signed_case(true);
}

#[test]
fn phase167_scripts_policy_minimaldata_cleanstack_remain_absent_on_canonical_path() {
    // Arrange
    let (mut chain, funding) = funded(&[0x51], 1);
    let mut spend = spending(&funding);
    spend.inputs[0].script_sig = script(&[0x4c, 1, 1]);
    let block = body(&chain, spend);
    chain
        .admit_canonical_header(&block.header, i64::from(block.header.time))
        .expect("admit");
    // Act: caller policy flags cannot tighten canonical consensus.
    let staged = chain.stage_connect_block_with_current_time(
        &block,
        work(&chain, &block),
        -1,
        ScriptVerifyFlags::MINIMALDATA | ScriptVerifyFlags::CLEANSTACK,
        ConsensusParams::default(),
    );
    // Assert
    chain
        .absorb_staged_connect(staged.expect("consensus accepts nonminimal push and extra stack"))
        .expect("accept");
    assert!(
        !chain
            .have_coin(&OutPoint {
                txid: transaction_txid(&funding).expect("txid"),
                vout: 0
            })
            .expect("coin")
    );
}

#[test]
fn phase167_policy_explicit_minimaldata_and_cleanstack_refuse_same_funded_inputs() {
    // Arrange
    let (chain, funding) = funded(&[0xb1, 0x75, 0x51], 1);
    let mut spend = spending(&funding);
    spend.lock_time = 1;
    spend.inputs[0].sequence = 100;
    spend.inputs[0].script_sig = script(&[2, 1, 0]);
    // Act / Assert
    assert_eq!(
        explicit_policy(&chain, &spend, ScriptVerifyFlags::NONE),
        Ok(())
    );
    assert_eq!(
        explicit_policy(&chain, &spend, ScriptVerifyFlags::MINIMALDATA),
        Err(ScriptError::VerifyFailed)
    );
    spend.inputs[0].script_sig = script(&[0x51, 0x51]);
    assert_eq!(
        explicit_policy(&chain, &spend, ScriptVerifyFlags::CLEANSTACK),
        Err(ScriptError::WitnessCleanStack)
    );
}
