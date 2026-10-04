// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockfilter.cpp
// - packages/bitcoin-knots/src/blockfilter.h
// - packages/bitcoin-knots/src/undo.h
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

mod independent_vectors {
    include!("../../../../open-bitcoin-consensus/testdata/basic_filter_vectors.rs");
}

fn independent(name: &str) -> &'static independent_vectors::BasicVector {
    independent_vectors::BASIC_VECTORS
        .iter()
        .find(|row| row.name == name)
        .expect("independent fixture name")
}

fn raw_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn assert_independent(
    name: &str,
    block: &open_bitcoin_primitives::Block,
    filter: &open_bitcoin_consensus::block_filter::BasicFilter,
    header: open_bitcoin_primitives::FilterHeader,
) {
    let expected = independent(name);
    assert_eq!(
        open_bitcoin_consensus::block_hash(&block.header),
        filter.block_hash()
    );
    assert_eq!(
        raw_hex(filter.block_hash().as_bytes()),
        expected.block_hash_raw
    );
    assert_eq!(raw_hex(filter.encoded_bytes()), expected.encoded);
    assert_eq!(raw_hex(filter.filter_hash().as_bytes()), expected.hash_raw);
    assert_eq!(raw_hex(header.as_bytes()), expected.header_raw);
}

#[test]
fn staged_genesis_and_explicit_empty_undo_agree_with_no_undo_path() {
    // Arrange
    let fixture = fixtures::spending_chain();
    let empty = crate::Chainstate::default();
    let staged = fixtures::stage(&empty, &fixture.genesis, 1);
    // Act
    let staged_inputs = staged
        .basic_filter_inputs(&fixture.genesis)
        .expect("staged genesis");
    let historical = BasicFilterInputs::from_historical(&fixture.genesis, &staged.position, None)
        .expect("genesis no-undo path");
    // Assert
    assert_eq!(staged_inputs.spent_scripts().count(), 0);
    assert_eq!(
        staged_inputs.generate(FilterHeaderPredecessor::Genesis),
        historical.generate(FilterHeaderPredecessor::Genesis)
    );
    assert!(empty.tip().is_none());
}

#[test]
fn staged_accessor_refuses_a_different_block_without_state_change() {
    // Arrange
    let fixture = fixtures::spending_chain();
    let staged = fixtures::stage(&fixture.chainstate, &fixture.spending, 3);
    let before = fixture.chainstate.snapshot();
    // Act
    let result = staged.basic_filter_inputs(&fixture.funding);
    // Assert
    assert_eq!(result.err(), Some(BasicFilterInputError::BlockIdentity));
    assert_eq!(fixture.chainstate.snapshot(), before);
}

#[test]
fn staged_validation_projects_historical_and_same_block_spends() {
    // Arrange
    let fixture = fixtures::spending_chain();
    let before = fixture.chainstate.snapshot();
    // Act
    let staged = fixtures::stage(&fixture.chainstate, &fixture.spending, 3);
    let inputs = staged
        .basic_filter_inputs(&fixture.spending)
        .expect("actual stage inputs");
    // Assert
    assert_eq!(staged.undo.transactions.len(), 2);
    assert!(staged.undo.transactions[0].restored_inputs[0].is_coinbase);
    assert_eq!(
        staged.undo.transactions[0].restored_inputs[0].created_height,
        1
    );
    assert!(!staged.undo.transactions[1].restored_inputs[0].is_coinbase);
    assert_eq!(
        staged.undo.transactions[1].restored_inputs[0].created_height,
        2
    );
    assert_eq!(
        staged.undo.transactions[0].restored_inputs[0]
            .output
            .script_pubkey
            .as_bytes(),
        fixtures::FUNDING_SCRIPT
    );
    assert_eq!(
        staged.undo.transactions[1].restored_inputs[0]
            .output
            .script_pubkey
            .as_bytes(),
        fixtures::SAME_BLOCK_SCRIPT
    );
    assert_eq!(
        inputs.spent_scripts().collect::<Vec<_>>(),
        vec![fixtures::FUNDING_SCRIPT, fixtures::SAME_BLOCK_SCRIPT]
    );
    assert_eq!(fixture.chainstate.snapshot(), before);
    assert_eq!(
        raw_hex(staged.position.block_hash.as_bytes()),
        independent("actual-spending").block_hash_raw
    );
}

fn funding_header(fixture: &fixtures::SpendingChain) -> open_bitcoin_primitives::FilterHeader {
    let snapshot = fixture.chainstate.snapshot();
    let (genesis_filter, genesis_header) =
        BasicFilterInputs::from_historical(&fixture.genesis, &snapshot.active_chain[0], None)
            .expect("genesis facts")
            .generate(FilterHeaderPredecessor::Genesis)
            .expect("genesis header");
    assert_independent(
        "actual-genesis",
        &fixture.genesis,
        &genesis_filter,
        genesis_header,
    );
    let position = &snapshot.active_chain[1];
    let (filter, header) = BasicFilterInputs::from_historical(
        &fixture.funding,
        position,
        Some(HistoricalBlockUndo {
            block_hash: position.block_hash,
            undo: &snapshot.undo_by_block[&position.block_hash],
        }),
    )
    .expect("funding facts")
    .generate(FilterHeaderPredecessor::Previous {
        block_hash: snapshot.active_chain[0].block_hash,
        height: 0,
        header: genesis_header,
    })
    .expect("funding header");
    assert_independent("actual-funding", &fixture.funding, &filter, header);
    header
}

#[test]
fn retained_history_generates_identical_commitments_after_spent_coins_disappear() {
    // Arrange
    let mut fixture = fixtures::spending_chain();
    let previous_header = funding_header(&fixture);
    let staged = fixtures::stage(&fixture.chainstate, &fixture.spending, 3);
    let position = staged.position.clone();
    let funding_outpoint = fixtures::outpoint(&fixture.funding.transactions[0]);
    let same_block_outpoint = fixtures::outpoint(&fixture.spending.transactions[1]);
    let predecessor = FilterHeaderPredecessor::Previous {
        block_hash: fixture.chainstate.tip().expect("funding tip").block_hash,
        height: 1,
        header: previous_header,
    };
    let staged_inputs = staged
        .basic_filter_inputs(&fixture.spending)
        .expect("stage facts");
    let expected = staged_inputs
        .generate(predecessor)
        .expect("stage commitment");
    assert_independent(
        "actual-spending",
        &fixture.spending,
        &expected.0,
        expected.1,
    );
    assert!(fixture.chainstate.utxos().contains_key(&funding_outpoint));
    assert!(
        !fixture
            .chainstate
            .utxos()
            .contains_key(&same_block_outpoint)
    );
    // Act
    fixture
        .chainstate
        .commit_staged_connect(staged)
        .expect("spending commit");
    let retained = &fixture.chainstate.undo_by_block()[&position.block_hash];
    let inputs = BasicFilterInputs::from_historical(
        &fixture.spending,
        &position,
        Some(HistoricalBlockUndo {
            block_hash: position.block_hash,
            undo: retained,
        }),
    )
    .expect("retained authoritative facts");
    // Assert
    assert!(!fixture.chainstate.utxos().contains_key(&funding_outpoint));
    assert!(
        !fixture
            .chainstate
            .utxos()
            .contains_key(&same_block_outpoint)
    );
    assert_eq!(
        inputs.spent_scripts().collect::<Vec<_>>(),
        vec![fixtures::FUNDING_SCRIPT, fixtures::SAME_BLOCK_SCRIPT]
    );
    assert_eq!(
        inputs.generate(predecessor).expect("retained commitment"),
        expected
    );
    assert_eq!(
        BasicFilterInputs::from_historical(&fixture.spending, &position, None).err(),
        Some(BasicFilterInputError::MissingUndo)
    );
}

#[test]
fn projection_borrows_scripts_in_transaction_and_input_order() {
    // Arrange
    let fixture = fixtures::spending_chain();
    let staged = fixtures::stage(&fixture.chainstate, &fixture.spending, 3);
    let mut block = fixture.spending.clone();
    let mut undo = staged.undo.clone();
    let second_input = block.transactions[2].inputs[0].clone();
    block.transactions[1].inputs.push(second_input);
    let second_coin = undo.transactions[1].restored_inputs[0].clone();
    undo.transactions[0].restored_inputs.push(second_coin);
    let (merkle_root, mutated) = open_bitcoin_consensus::block_merkle_root(&block.transactions)
        .expect("edited fixture merkle");
    assert!(!mutated);
    block.header.merkle_root = merkle_root;
    let mut position = staged.position.clone();
    position.block_hash = open_bitcoin_consensus::block_hash(&block.header);
    position.header = block.header.clone();
    // Act
    let inputs = BasicFilterInputs::from_historical(
        &block,
        &position,
        Some(HistoricalBlockUndo {
            block_hash: position.block_hash,
            undo: &undo,
        }),
    )
    .expect("shape-compatible retained projection");
    let scripts = inputs.spent_scripts().collect::<Vec<_>>();
    // Assert
    assert_eq!(
        scripts,
        vec![
            fixtures::FUNDING_SCRIPT,
            fixtures::SAME_BLOCK_SCRIPT,
            fixtures::SAME_BLOCK_SCRIPT
        ]
    );
    assert_eq!(
        scripts[0].as_ptr(),
        undo.transactions[0].restored_inputs[0]
            .output
            .script_pubkey
            .as_bytes()
            .as_ptr()
    );
    assert_eq!(
        scripts[1].as_ptr(),
        undo.transactions[0].restored_inputs[1]
            .output
            .script_pubkey
            .as_bytes()
            .as_ptr()
    );
}

#[test]
fn replacement_sequences_reuse_common_ancestor_then_their_own_predecessors() {
    // Arrange
    let mut original = fixtures::spending_chain();
    let mut replacement = fixtures::spending_chain();
    let common_header = funding_header(&original);
    assert_eq!(common_header, funding_header(&replacement));
    replacement.spending = fixtures::block(
        replacement.chainstate.tip().expect("funding").block_hash,
        1_300,
        vec![
            fixtures::coinbase(2, fixtures::COINBASE_SCRIPT),
            replacement.spending.transactions[1].clone(),
            fixtures::spend(
                &replacement.spending.transactions[1],
                fixtures::REWARD - 2_000,
                &[0x56],
            ),
        ],
    );
    let before = replacement.chainstate.snapshot();
    let mut tips = Vec::new();
    // Act
    for (fixture, branch_name, next_name) in [
        (&mut original, "actual-spending", "actual-successor"),
        (
            &mut replacement,
            "actual-replacement",
            "actual-replacement-successor",
        ),
    ] {
        let staged = fixtures::stage(&fixture.chainstate, &fixture.spending, 3);
        let predecessor = FilterHeaderPredecessor::Previous {
            block_hash: fixture.chainstate.tip().expect("ancestor").block_hash,
            height: 1,
            header: common_header,
        };
        let (filter, header) = staged
            .basic_filter_inputs(&fixture.spending)
            .expect("replacement facts")
            .generate(predecessor)
            .expect("ancestor commitment");
        assert_independent(branch_name, &fixture.spending, &filter, header);
        let position = fixture
            .chainstate
            .commit_staged_connect(staged)
            .expect("branch commit");
        let next = fixtures::block(
            position.block_hash,
            fixture.spending.header.time + 100,
            vec![fixtures::coinbase(3, &[0x57])],
        );
        let next_stage = fixtures::stage(&fixture.chainstate, &next, 4);
        let (next_filter, next_header) = next_stage
            .basic_filter_inputs(&next)
            .expect("successor facts")
            .generate(FilterHeaderPredecessor::Previous {
                block_hash: position.block_hash,
                height: 2,
                header,
            })
            .expect("branch successor commitment");
        assert_independent(next_name, &next, &next_filter, next_header);
        let tip = fixture
            .chainstate
            .commit_staged_connect(next_stage)
            .expect("successor commit");
        tips.push((tip, next_header));
    }
    // Assert
    assert_ne!(tips[0].1, tips[1].1);
    let position = &replacement.chainstate.snapshot().active_chain[2];
    let undo = &replacement.chainstate.undo_by_block()[&position.block_hash];
    let inputs = BasicFilterInputs::from_historical(
        &replacement.spending,
        position,
        Some(HistoricalBlockUndo {
            block_hash: position.block_hash,
            undo,
        }),
    )
    .expect("replacement history");
    assert_eq!(
        inputs.generate(FilterHeaderPredecessor::Previous {
            block_hash: tips[0].0.block_hash,
            height: tips[0].0.height,
            header: tips[0].1
        }),
        Err(BasicFilterGenerationError::Header(
            FilterHeaderError::PreviousHash
        ))
    );
    assert_eq!(before.active_chain.len(), 2);
    assert_eq!(replacement.chainstate.snapshot().active_chain.len(), 4);
}

#[test]
fn historical_refusals_do_not_mutate_validated_state() {
    // Arrange
    let fixture = fixtures::spending_chain();
    let staged = fixtures::stage(&fixture.chainstate, &fixture.spending, 3);
    let before = fixture.chainstate.snapshot();
    let mut altered = staged.undo.clone();
    altered.transactions[1].restored_inputs.clear();
    // Act
    let missing = BasicFilterInputs::from_historical(&fixture.spending, &staged.position, None);
    let mismatch = historical_error(&fixture.spending, &staged.position, &altered);
    // Assert
    assert_eq!(missing.err(), Some(BasicFilterInputError::MissingUndo));
    assert_eq!(
        mismatch,
        BasicFilterInputError::InputCount {
            index: 2,
            expected: 1,
            actual: 0
        }
    );
    assert_eq!(fixture.chainstate.snapshot(), before);
}
