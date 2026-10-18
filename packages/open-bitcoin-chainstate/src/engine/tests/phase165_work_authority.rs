// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp

use super::phase165_tests::{child, live_parent, stage_replacement};
use super::*;
use open_bitcoin_consensus::chain_proof::ProofWork;

#[test]
fn phase165_source_binding_refuses_raw_mechanical_and_different_parameters() {
    // Arrange
    let (chain, _) = live_parent();
    let raw = Chainstate::from_snapshot(chain.snapshot());
    let mechanical = Chainstate::new_for_mechanical_fixture();
    let changed = ConsensusParams {
        coinbase_maturity: 99,
        ..Default::default()
    };
    // Act / Assert
    assert!(matches!(
        raw.bind_live_work_source(ConsensusParams::default()),
        Err(ChainstateError::UntrustedWorkLineage)
    ));
    assert!(matches!(
        mechanical.bind_live_work_source(ConsensusParams::default()),
        Err(ChainstateError::UntrustedWorkLineage)
    ));
    assert!(matches!(
        chain.bind_live_work_source(changed),
        Err(ChainstateError::WorkParametersMismatch)
    ));
}

#[test]
fn phase165_source_binding_refuses_foreign_and_stale_sources() {
    // Arrange
    let (mut chain, _) = live_parent();
    let source = chain
        .bind_live_work_source(ConsensusParams::default())
        .expect("live source");
    let foreign = chain.clone();
    // Act / Assert
    assert_eq!(chain.check_live_work_source(&source), Ok(()));
    assert_eq!(
        foreign.check_live_work_source(&source),
        Err(ChainstateError::StageOwnerMismatch)
    );
    chain.forget_undo(chain.tip().expect("tip").block_hash);
    assert_eq!(
        chain.check_live_work_source(&source),
        Err(ChainstateError::StaleStage)
    );
}

#[test]
fn phase165_source_binding_follows_only_its_exact_preview_stage() {
    // Arrange
    let (mut chain, _) = live_parent();
    let source = chain
        .bind_live_work_source(ConsensusParams::default())
        .expect("source");
    let staged = stage_replacement(&chain);
    let foreign = stage_replacement(&chain.clone());
    // Act / Assert
    assert_eq!(
        chain.check_reorg_work_source(&source, &foreign),
        Err(ChainstateError::StaleStage)
    );
    assert_eq!(chain.check_reorg_work_source(&source, &staged), Ok(()));
    chain
        .install_staged_reorg_preview(&staged)
        .expect("preview");
    assert_eq!(
        chain.check_live_work_source(&source),
        Err(ChainstateError::UntrustedWorkLineage)
    );
    assert_eq!(chain.check_reorg_work_source(&source, &staged), Ok(()));
    chain
        .validate_staged_reorg(staged)
        .expect("own continuation")
        .absorb();
    assert_eq!(
        chain.check_live_work_source(&source),
        Err(ChainstateError::StaleStage)
    );
}

#[test]
fn phase165_replacement_error_names_actual_failing_body() {
    // Arrange
    let (chain, _) = live_parent();
    let mut invalid = child(&chain);
    invalid.transactions.clear();
    let hash = open_bitcoin_consensus::block_hash(&invalid.header);
    let before = chain.snapshot();
    // Act
    let error = chain
        .stage_reorg(
            &[],
            &[AnchoredBlock {
                block: invalid,
                chain_work: ProofWork::ZERO,
            }],
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect_err("actual invalid body");
    // Assert
    assert!(
        matches!(&error, ChainstateError::ReplacementBlock { block_hash, source } if *block_hash == hash && matches!(source.as_ref(), ChainstateError::BlockValidation { .. }))
    );
    assert!(error.to_string().contains("replacement block"));
    assert!(
        error
            .to_string()
            .contains("bad-txnmrklroot (hashMerkleRoot mismatch)"),
        "{error}"
    );
    assert_eq!(chain.snapshot(), before);
    assert!(
        ChainstateError::PreferenceAuthorizationRequired
            .to_string()
            .contains("authorization required")
    );
    assert!(
        ChainstateError::BranchNotPreferred
            .to_string()
            .contains("not preferred")
    );
}
