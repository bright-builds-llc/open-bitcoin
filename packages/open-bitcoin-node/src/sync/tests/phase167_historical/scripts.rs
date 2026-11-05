// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp

use super::source::load_cases;
use crate::{
    FjallChainstateStore, FjallNodeStore, ManagedChainstate, chainstate::FlushLifecycle,
    storage::validation_history::ValidationProvenance,
};
use open_bitcoin_core::{
    chainstate::{Chainstate, FlushPolicyTime},
    consensus::{
        CandidateHistory, CanonicalChain, HistoryError, ScriptVerifyFlags, TransactionInputContext,
        TransactionValidationContext, block_hash, validate_transaction_with_context,
    },
};

#[test]
fn phase167_historical_scripts_exact_exception_bodies_reach_managed_kernel() {
    // Arrange: all inputs originate in the bounded pinned serialized corpus.
    let cases = load_cases(&["script-p2sh-exception", "script-taproot-exception"]);
    assert_eq!(cases.len(), 3);
    for case in cases {
        assert!(case.overwrite_original.is_empty());
        let expected_flags = if case.history.height() == 692_261 {
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::WITNESS
                | ScriptVerifyFlags::DERSIG
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY
                | ScriptVerifyFlags::NULLDUMMY
        } else {
            ScriptVerifyFlags::NONE
        };
        assert_eq!(case.history.rules().script_flags(), expected_flags);
        case.history
            .check_header_rules(case.now)
            .expect("same source-bound contextual header and body rules");
        let mut nonce = [0; 16];
        getrandom::fill(&mut nonce).expect("test nonce");
        let path = std::env::temp_dir()
            .canonicalize()
            .expect("physical temp parent")
            .join(format!("phase167-script-{:x}", u128::from_le_bytes(nonce)));
        {
            let store = FjallNodeStore::open(&path).expect("separate managed fixture store");
            let chain = Chainstate::from_snapshot_for_mechanical_fixture(case.snapshot);
            assert!(!chain.has_earned_work());
            let mut manager = ManagedChainstate::from_chainstate(
                FjallChainstateStore::from_store(store),
                chain,
                FlushLifecycle::ready(FlushPolicyTime::new(0), FlushPolicyTime::new(0), 0, false),
            );
            // Act: the production apply kernel derives and consumes the flags.
            let accepted =
                manager.connect_seeded_historical_fixture(&case.block, &case.history, case.now);
            // Assert: source history remains classified, even after actual managed absorption.
            assert!(accepted.is_ok(), "{}: {accepted:?}", case.purpose);
            assert!(!manager.chainstate().has_earned_work());
            assert!(manager.chainstate().maybe_canonical_chain().is_none());
            assert!(manager.maybe_basic_index_progress().is_none());
            assert_ne!(
                manager
                    .validation_provenance(block_hash(&case.block.header))
                    .expect("mechanical provenance"),
                ValidationProvenance::ScriptsValid
            );
        }
        if case.history.height() == 170_060 {
            assert!(
                FjallNodeStore::reopen_canonical_genesis_existing(
                    &path,
                    case.history.network_rules().chain()
                )
                .is_err(),
                "seeded checkpoint cannot become canonical product state"
            );
        }
        std::fs::remove_dir_all(path).expect("all fixture handles dropped");
    }
}

#[test]
fn phase167_historical_scripts_policy_only_high_s_negative() {
    // Arrange: this exact pre-P2SH exception contains a high-S legacy signature.
    let mut cases = load_cases(&["script-p2sh-exception"]);
    let case = cases.remove(0);
    assert_eq!(case.history.height(), 170_060);
    let spend = &case.block.transactions[3];
    let inputs = spend
        .inputs
        .iter()
        .map(|input| {
            let coin = case
                .snapshot
                .utxos
                .get(&input.previous_output)
                .expect("independently verified spent output");
            TransactionInputContext {
                spent_output: coin.as_spent_output(),
                created_height: coin.created_height,
                created_median_time_past: coin.created_median_time_past,
            }
        })
        .collect();
    let mut context =
        TransactionValidationContext::from_candidate_history(spend, inputs, &case.history)
            .expect("exact selected transaction context");
    assert_eq!(context.verify_flags, ScriptVerifyFlags::NONE);
    // Act: only a policy flag is added to the same body and same coin context.
    let consensus = validate_transaction_with_context(spend, &context);
    context.verify_flags |= ScriptVerifyFlags::LOW_S;
    let policy = validate_transaction_with_context(spend, &context);
    // Assert: policy cannot be forwarded as a consensus deployment flag.
    assert!(
        consensus.is_ok(),
        "source-valid consensus spend: {consensus:?}"
    );
    let error = policy.expect_err("high-S policy-only negative");
    assert_eq!(error.reject_reason, "mandatory-script-verify-flag-failed");
    assert_eq!(error.debug_message.as_deref(), Some("non-low-S signature"));
}

#[test]
fn phase167_historical_scripts_consumed_witness_flag_rejects_corrupt_prevout() {
    // Arrange: exact height692261 tx16 spends one independently verified P2WPKH coin.
    let mut cases = load_cases(&["script-taproot-exception"]);
    let case = cases.remove(0);
    assert_eq!(case.history.height(), 692_261);
    let spend = &case.block.transactions[16];
    assert_eq!(spend.inputs.len(), 1);
    assert!(!spend.inputs[0].witness.is_empty());
    let outpoint = &spend.inputs[0].previous_output;
    let original = case
        .snapshot
        .utxos
        .get(outpoint)
        .expect("source-bound coin");
    let script = original.output.script_pubkey.as_bytes();
    assert_eq!(&script[..2], &[0x00, 0x14]);
    let mut corrupted = case.snapshot.clone();
    let damaged = corrupted.utxos.get_mut(outpoint).expect("same coin");
    let mut wrong_program = script.to_vec();
    wrong_program[2] ^= 1;
    damaged.output.script_pubkey =
        open_bitcoin_core::primitives::ScriptBuf::from_bytes(wrong_program)
            .expect("bounded hostile script");
    let inputs = vec![TransactionInputContext {
        spent_output: damaged.as_spent_output(),
        created_height: damaged.created_height,
        created_median_time_past: damaged.created_median_time_past,
    }];
    let selected_flags = case.history.rules().script_flags();
    assert!(selected_flags.contains(ScriptVerifyFlags::WITNESS));
    let mut context =
        TransactionValidationContext::from_candidate_history(spend, inputs, &case.history)
            .expect("same scalar selected context");
    assert_eq!(context.verify_flags, selected_flags);
    // Act: deliberately wrong flags pass a corrupted witness program in the pure
    // transaction control; the actual seeded apply kernel must consume selected flags.
    let selected = validate_transaction_with_context(spend, &context);
    context.verify_flags = ScriptVerifyFlags::NONE;
    let bypass = validate_transaction_with_context(spend, &context);
    let fixture = Chainstate::from_snapshot_for_mechanical_fixture(corrupted.clone());
    let staged = fixture.stage_seeded_historical_block(&case.block, &case.history, case.now);
    // Assert: same selected scalar facts, real interpreter refusal, no accepted mutation.
    assert!(selected.is_err(), "changed witness program must refuse");
    assert!(
        bypass.is_ok(),
        "wrong consumed flags would bypass: {bypass:?}"
    );
    assert!(
        staged.is_err(),
        "selected kernel must reject corrupt source coin"
    );
    assert_eq!(fixture.snapshot(), corrupted);
    assert!(!fixture.has_earned_work());
    assert_eq!(case.history.rules().script_flags(), selected_flags);
}

#[test]
fn phase167_historical_scripts_missing_anchor_and_recovery_refuse_structural_context() {
    // Arrange: exact mainnet Taproot exception needs a seeded BIP34 anchor;
    // the early testnet exception needs the full source-bound recovery branch.
    let cases = load_cases(&["script-taproot-exception", "script-p2sh-exception"]);
    let main = cases
        .iter()
        .find(|case| case.history.height() == 692_261)
        .expect("mainnet case");
    let testnet = cases
        .iter()
        .find(|case| case.history.height() == 394)
        .expect("testnet case");
    assert_eq!(testnet.history.branch().len(), 394);
    let main_branch = main.history.branch();
    let test_branch = testnet.history.branch();
    // Act: omit only the independently held anchor or early recovery rows.
    let missing_anchor = CandidateHistory::check_seeded(
        CanonicalChain::Mainnet.spec().rules(),
        main.block.header.clone(),
        main_branch[0].clone(),
        main_branch[1..].to_vec(),
        None,
    );
    let missing_recovery = CandidateHistory::check_seeded(
        CanonicalChain::Testnet3.spec().rules(),
        testnet.block.header.clone(),
        test_branch[0].clone(),
        test_branch[1..155].to_vec(),
        None,
    );
    // Assert: constructor refusal is explicitly structural, not durable acceptance.
    assert!(matches!(
        missing_anchor,
        Err(HistoryError::MissingAncestor { height: 227_931 })
    ));
    assert!(matches!(
        missing_recovery,
        Err(HistoryError::MissingAncestor { .. })
    ));
}

#[test]
fn phase167_historical_scripts_wrong_envelope_refuses_before_state_mutation() {
    // Arrange: source proves the exact testnet body, parent and linked ancestry.
    let mut cases = load_cases(&["script-p2sh-exception"]);
    let index = cases
        .iter()
        .position(|case| case.history.height() == 394)
        .expect("testnet case");
    let case = cases.remove(index);
    assert_eq!(case.history.height(), 394);
    let branch = case.history.branch();
    let mut wrong_network = branch[0].clone();
    wrong_network.chain = CanonicalChain::Mainnet;
    let mut wrong_hash = branch[0].clone();
    wrong_hash.block_hash = open_bitcoin_core::primitives::BlockHash::from_byte_array([0; 32]);
    let chain = Chainstate::from_snapshot_for_mechanical_fixture(case.snapshot.clone());
    let mut wrong_body = case.block.clone();
    wrong_body.header.nonce ^= 1;
    // Act: malformed external envelope facts fail before accepted mutation.
    let network = CandidateHistory::check_seeded(
        CanonicalChain::Testnet3.spec().rules(),
        case.block.header.clone(),
        wrong_network,
        branch[1..].to_vec(),
        None,
    );
    let hash = CandidateHistory::check_seeded(
        CanonicalChain::Testnet3.spec().rules(),
        case.block.header.clone(),
        wrong_hash,
        branch[1..].to_vec(),
        None,
    );
    let body = chain.stage_seeded_historical_block(&wrong_body, &case.history, case.now);
    // Assert: these are distinct structural and actual kernel refusal classes.
    assert!(matches!(network, Err(HistoryError::WrongNetwork { .. })));
    assert!(matches!(hash, Err(HistoryError::HashMismatch { .. })));
    assert!(body.is_err(), "wrong candidate header must not stage");
    assert_eq!(chain.snapshot(), case.snapshot);
    assert!(!chain.has_earned_work());
}
