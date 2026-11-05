// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp
// - packages/bitcoin-knots/src/consensus/merkle.cpp

use super::super::phase167_support as source;
use super::contexts::source_case;
use crate::{
    FjallChainstateStore, FjallNodeStore, ManagedChainstate, chainstate::FlushLifecycle,
    storage::validation_history::ValidationProvenance,
};
use open_bitcoin_core::{
    chainstate::{Chainstate, ChainstateSnapshot, FlushPolicyTime},
    consensus::{
        CandidateHistory, CanonicalChain, ScriptVerifyFlags, TransactionInputContext,
        TransactionValidationContext, block_hash, block_merkle_root, check_block_header,
        crypto::{double_sha256, hash160},
        transaction_wtxid, validate_transaction_with_context,
    },
    primitives::{Amount, Block, OutPoint, ScriptBuf, ScriptWitness, TransactionOutput},
};

#[derive(Clone, Copy, Debug)]
enum FlagCase {
    P2sh,
    Dersig,
    Cltv,
    Csv,
    Nulldummy,
    Witness,
    Taproot,
}

impl FlagCase {
    fn flag(self) -> ScriptVerifyFlags {
        match self {
            Self::P2sh => ScriptVerifyFlags::P2SH,
            Self::Dersig => ScriptVerifyFlags::DERSIG,
            Self::Cltv => ScriptVerifyFlags::CHECKLOCKTIMEVERIFY,
            Self::Csv => ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            Self::Nulldummy => ScriptVerifyFlags::NULLDUMMY,
            Self::Witness => ScriptVerifyFlags::WITNESS,
            Self::Taproot => ScriptVerifyFlags::TAPROOT,
        }
    }

    fn source_height(self) -> usize {
        match self {
            Self::Cltv | Self::Csv => 201,
            Self::Witness => 151,
            _ => 101,
        }
    }

    fn refusal(self) -> &'static str {
        match self {
            Self::P2sh => "script evaluated to false",
            Self::Dersig => "invalid DER signature",
            Self::Cltv | Self::Csv => "unsatisfied locktime",
            Self::Nulldummy => "non-null CHECKMULTISIG dummy argument",
            Self::Witness | Self::Taproot => "witness program mismatch",
        }
    }
}

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

fn script(bytes: Vec<u8>) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes).expect("bounded mechanical script")
}

fn source_facts(case: FlagCase, block: &Block, snapshot: &ChainstateSnapshot) -> OutPoint {
    let height = case.source_height();
    let spend = &block.transactions[1];
    assert_eq!(block.transactions.len(), 2);
    assert_eq!(spend.inputs.len(), 1);
    let outpoint = spend.inputs[0].previous_output.clone();
    assert_eq!(
        source::hex(outpoint.txid.as_bytes()),
        "7346c615f3c47579eab9ad546cf1595cb4199e4882f38742bf8a893f3f46e78f"
    );
    let (vout, sats, original_script) = match height {
        101 => (0, 2_000_000_000, "51"),
        151 => (
            2,
            1_000_000_000,
            "00204ae81572f06e1b88fd5ced7a1a000945432e83e1551e6f721ee9c00b8cc33260",
        ),
        201 => (1, 2_000_000_000, "51"),
        _ => unreachable!("three pinned ordinary rows"),
    };
    assert_eq!(outpoint.vout, vout);
    let coin = snapshot
        .utxos
        .get(&outpoint)
        .expect("independently held prevout");
    assert_eq!(coin.output.value.to_sats(), sats);
    assert_eq!(
        source::hex(coin.output.script_pubkey.as_bytes()),
        original_script
    );
    assert_eq!(coin.created_height, 1);
    assert_eq!(coin.created_median_time_past, 1_296_688_602);
    assert!(coin.is_coinbase);
    assert_eq!(
        snapshot.tip().expect("source parent").height as usize,
        height - 1
    );
    outpoint
}

fn p2sh_script(redeem: u8) -> Vec<u8> {
    let mut output = vec![0xa9, 0x14];
    output.extend_from_slice(&hash160(&[redeem]));
    output.push(0x87);
    assert_eq!(
        source::hex(&output),
        match redeem {
            0x51 => "a914da1745e9b549bd0bfa1a569971c77eba30cd5a4b87",
            0x00 => "a9149f7fd096d37ed2c0e3f7f0cfc924beef4ffceb6887",
            _ => unreachable!("only pinned mechanical redeem bytes"),
        }
    );
    output
}

fn taproot_commitment(block: &mut Block) {
    let mut root_preimage = vec![0; 32];
    root_preimage.extend_from_slice(
        transaction_wtxid(&block.transactions[1])
            .expect("derived witness serialization")
            .as_bytes(),
    );
    let mut commitment_preimage = double_sha256(&root_preimage).to_vec();
    commitment_preimage.extend_from_slice(&[0; 32]);
    let mut commitment = vec![0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed];
    commitment.extend_from_slice(&double_sha256(&commitment_preimage));
    let coinbase = &mut block.transactions[0];
    coinbase.inputs[0].witness = ScriptWitness::new(vec![vec![0; 32]]);
    coinbase.outputs.push(TransactionOutput {
        value: Amount::ZERO,
        script_pubkey: script(commitment),
    });
}

fn reseal(block: &mut Block, source_history: &CandidateHistory) -> CandidateHistory {
    let (root, mutated) = block_merkle_root(&block.transactions).expect("derived merkle root");
    assert!(!mutated);
    block.header.merkle_root = root;
    while check_block_header(&block.header).is_err() {
        block.header.nonce = block.header.nonce.wrapping_add(1);
    }
    let branch = source_history.branch();
    CandidateHistory::check_seeded(
        CanonicalChain::Regtest.spec().rules(),
        block.header.clone(),
        branch[0].clone(),
        branch[1..].to_vec(),
        None,
    )
    .expect("source branch with resealed mechanical candidate")
}

fn derived_case(
    case: FlagCase,
    negative: bool,
) -> (Block, CandidateHistory, ChainstateSnapshot, OutPoint) {
    let (mut block, source_history, mut snapshot) = source_case(case.source_height(), 0, 0);
    let outpoint = source_facts(case, &block, &snapshot);
    let coin = snapshot.utxos.get_mut(&outpoint).expect("source coin");
    let spend = &mut block.transactions[1];
    let mut changed_body = false;
    match case {
        FlagCase::P2sh => {
            let redeem = if negative { 0x00 } else { 0x51 };
            coin.output.script_pubkey = script(p2sh_script(redeem));
            spend.inputs[0].script_sig = script(vec![1, redeem]);
            changed_body = true;
        }
        FlagCase::Dersig => {
            coin.output.script_pubkey = script(vec![0xac, 0x91]);
            let mut unlocking = if negative { vec![1, 1] } else { vec![0] };
            unlocking.push(33);
            unlocking.push(2);
            unlocking.extend_from_slice(&[1; 32]);
            spend.inputs[0].script_sig = script(unlocking);
            changed_body = true;
        }
        FlagCase::Cltv => {
            coin.output.script_pubkey =
                script(vec![if negative { 0x51 } else { 0x00 }, 0xb1, 0x75, 0x51]);
            assert_eq!(spend.version, 2);
            assert_eq!(spend.lock_time, 0);
            assert_eq!(spend.inputs[0].sequence, 0x0040_00e4);
        }
        FlagCase::Csv => {
            coin.output.script_pubkey = script(vec![
                3,
                if negative { 0xe5 } else { 0xe4 },
                0,
                0x40,
                0xb2,
                0x75,
                0x51,
            ]);
            assert_eq!(spend.version, 2);
            assert_eq!(spend.inputs[0].sequence, 0x0040_00e4);
        }
        FlagCase::Nulldummy => {
            coin.output.script_pubkey = script(vec![0, 0, 0xae]);
            spend.inputs[0].script_sig = script(vec![if negative { 0x51 } else { 0x00 }]);
            changed_body = true;
        }
        FlagCase::Witness => {
            assert_eq!(spend.inputs[0].witness.stack(), &[vec![0x51]]);
            if negative {
                let mut program = coin.output.script_pubkey.as_bytes().to_vec();
                program[2] ^= 1;
                coin.output.script_pubkey = script(program);
            }
        }
        FlagCase::Taproot => {
            coin.output.script_pubkey = script(source::bytes(
                "51209b6ce0db0707e29f92bf8893ed1911d397e3d2d76bbc68110c49da2ceec8be23",
            ));
            let mut control =
                source::bytes("c079be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798");
            if negative {
                control[1] ^= 1;
            }
            spend.inputs[0].witness = ScriptWitness::new(vec![vec![0x51], control]);
            taproot_commitment(&mut block);
            changed_body = true;
        }
    }
    let history = if changed_body {
        reseal(&mut block, &source_history)
    } else {
        source_history
    };
    assert_eq!(history.height() as usize, case.source_height());
    assert_eq!(
        history.rules().script_flags(),
        all_flags_except(ScriptVerifyFlags::NONE)
    );
    (block, history, snapshot, outpoint)
}

fn assert_managed_case(case: FlagCase) {
    // Arrange: source-bound parent and prevout are checked before each mutation.
    for negative in [false, true] {
        let (block, history, snapshot, outpoint) = derived_case(case, negative);
        let before_best = snapshot.tip().expect("complete source parent").block_hash;
        let spend = &block.transactions[1];
        let coin = snapshot
            .utxos
            .get(&outpoint)
            .expect("modified mechanical prevout");
        let inputs = vec![TransactionInputContext {
            spent_output: coin.as_spent_output(),
            created_height: coin.created_height,
            created_median_time_past: coin.created_median_time_past,
        }];
        let mut context =
            TransactionValidationContext::from_candidate_history(spend, inputs, &history)
                .expect("same selected scalar facts");
        assert_eq!(context.verify_flags, history.rules().script_flags());
        if negative {
            // Pure same-coin control removes exactly the named flag.
            context.verify_flags = all_flags_except(case.flag());
            let bypass = validate_transaction_with_context(spend, &context);
            assert!(bypass.is_ok(), "{case:?} pure bypass: {bypass:?}");
        }
        let maybe_expected_accepted = if negative {
            None
        } else {
            let mut expected = Chainstate::from_snapshot_for_mechanical_fixture(snapshot.clone());
            let staged = expected
                .stage_seeded_historical_block(&block, &history, i64::from(block.header.time))
                .expect("selected mechanical expected state");
            assert!(!staged.has_earned_work());
            expected
                .absorb_staged_connect(staged)
                .expect("expected mechanical acceptance");
            Some(expected.snapshot())
        };
        let mut nonce = [0; 16];
        getrandom::fill(&mut nonce).expect("test nonce");
        let path = std::env::temp_dir()
            .canonicalize()
            .expect("physical temp parent")
            .join(format!(
                "phase167-ordinary-flag-{:x}",
                u128::from_le_bytes(nonce)
            ));
        {
            let store = FjallNodeStore::open(&path).expect("separate managed fixture store");
            let chain = Chainstate::from_snapshot_for_mechanical_fixture(snapshot.clone());
            assert!(!chain.has_earned_work());
            assert_eq!(
                chain.coins_best_block().expect("initial best"),
                Some(before_best)
            );
            let mut manager = ManagedChainstate::from_chainstate(
                FjallChainstateStore::from_store(store),
                chain,
                FlushLifecycle::ready(FlushPolicyTime::new(0), FlushPolicyTime::new(0), 0, false),
            );
            // Act: internally selected rules reach the managed seeded interpreter.
            let result = manager.connect_seeded_historical_fixture(
                &block,
                &history,
                i64::from(block.header.time),
            );
            // Assert: fixture acceptance never promotes canonical work or BASIC.
            assert!(!manager.chainstate().has_earned_work());
            assert!(manager.chainstate().maybe_canonical_chain().is_none());
            assert!(manager.maybe_basic_index_progress().is_none());
            if negative {
                let error = result.expect_err("selected managed spend must reject");
                let rendered = format!("{error:?}");
                assert!(
                    rendered.contains("TransactionValidation"),
                    "{case:?}: {rendered}"
                );
                assert!(
                    rendered.contains("mandatory-script-verify-flag-failed"),
                    "{case:?}: {rendered}"
                );
                assert!(rendered.contains(case.refusal()), "{case:?}: {rendered}");
                assert_eq!(manager.chainstate().snapshot(), snapshot);
                assert_eq!(
                    manager
                        .chainstate()
                        .coins_best_block()
                        .expect("unchanged best"),
                    Some(before_best)
                );
                assert_eq!(
                    manager
                        .validation_provenance(block_hash(&block.header))
                        .expect("unpromoted provenance"),
                    ValidationProvenance::NeverConnected
                );
            } else {
                let accepted = result.unwrap_or_else(|error| panic!("{case:?}: {error:?}"));
                assert_eq!(accepted.height as usize, case.source_height());
                assert_eq!(accepted.block_hash, block_hash(&block.header));
                assert_eq!(manager.chainstate().tip(), Some(&accepted));
                assert_eq!(
                    manager
                        .chainstate()
                        .coins_best_block()
                        .expect("accepted best"),
                    Some(accepted.block_hash)
                );
                assert_eq!(
                    manager.chainstate().snapshot(),
                    maybe_expected_accepted.expect("positive expected full snapshot")
                );
                assert!(
                    !manager
                        .chainstate()
                        .snapshot()
                        .utxos
                        .contains_key(&outpoint)
                );
                assert_ne!(
                    manager
                        .validation_provenance(block_hash(&block.header))
                        .expect("mechanical provenance"),
                    ValidationProvenance::ScriptsValid
                );
            }
        }
        std::fs::remove_dir_all(path).expect("all fixture handles dropped");
    }
}

#[test]
fn phase167_historical_scripts_ordinary_p2sh_managed_spend() {
    assert_managed_case(FlagCase::P2sh);
}
#[test]
fn phase167_historical_scripts_ordinary_dersig_managed_spend() {
    assert_managed_case(FlagCase::Dersig);
}
#[test]
fn phase167_historical_scripts_ordinary_cltv_managed_spend() {
    assert_managed_case(FlagCase::Cltv);
}
#[test]
fn phase167_historical_scripts_ordinary_csv_managed_spend() {
    assert_managed_case(FlagCase::Csv);
}
#[test]
fn phase167_historical_scripts_ordinary_nulldummy_managed_spend() {
    assert_managed_case(FlagCase::Nulldummy);
}
#[test]
fn phase167_historical_scripts_ordinary_witness_managed_spend() {
    assert_managed_case(FlagCase::Witness);
}
#[test]
fn phase167_historical_scripts_ordinary_taproot_managed_spend() {
    assert_managed_case(FlagCase::Taproot);
}
