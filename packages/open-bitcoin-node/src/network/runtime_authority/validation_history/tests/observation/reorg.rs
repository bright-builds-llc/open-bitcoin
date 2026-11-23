// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

fn equal_work_fixture(
    mode: BasicFilterStartupMode,
) -> (
    std::path::PathBuf,
    DurableSyncRuntime,
    CanonicalSubmissionCommand,
    Block,
) {
    let (path, runtime, command) = runtime(mode);
    let common = child(&CanonicalChain::Regtest.spec().genesis_header(), 1);
    accepted(command.submit(wire(&common)).expect("common parent"));
    let active = child(&common.header, 2);
    accepted(command.submit(wire(&active)).expect("first child"));
    let mut side = active;
    side.transactions[0].outputs[0].script_pubkey =
        open_bitcoin_core::primitives::ScriptBuf::from_serialized_bytes(vec![0x52]);
    remine(&mut side);
    assert!(matches!(
        command.submit(wire(&side)).expect("equal-work side body"),
        CanonicalSubmissionOutcome::Inconclusive
    ));
    (path, runtime, command, side)
}

#[test]
fn crec01_stronger_retained_branch_activates_after_reopen() {
    // Arrange
    let mode = BasicFilterStartupMode::Enabled;
    let (path, runtime, command, side) = equal_work_fixture(mode);
    let (runtime, command) = reopen(&path, runtime, command, mode);
    let stronger = child(&side.header, 3);

    // Act
    let position = accepted(command.submit(wire(&stronger)).expect("winning branch"));
    let (runtime, command) = reopen(&path, runtime, command, mode);

    // Assert
    let reopened = command
        .observe_accepted_state(&[])
        .expect("proved winning history");
    reopened.fold(|(snapshot, _, coins), progress, _, _, _| {
        assert_eq!(snapshot.tip(), Some(&position));
        assert_eq!(coins.maybe_best_block, Some(position.block_hash));
        assert_eq!(position.block_hash, block_hash(&stronger.header));
        assert_eq!(
            progress
                .expect("enabled BASIC")
                .maybe_safe_durable_endpoint()
                .expect("indexed replacement")
                .block_hash(),
            position.block_hash
        );
    });
    close(path, runtime, command);
}

#[test]
fn crec01_invalid_winning_candidate_leaves_active_chain_unchanged() {
    // Arrange
    let (path, runtime, command, side) = equal_work_fixture(BasicFilterStartupMode::Enabled);
    let mut invalid = child(&side.header, 3);
    let reward = invalid.transactions[0].outputs[0].value.to_sats();
    invalid.transactions[0].outputs[0].value =
        open_bitcoin_core::primitives::Amount::from_sats(reward + 1).expect("bounded amount");
    remine(&mut invalid);
    let hash = block_hash(&invalid.header);

    // Act / Assert
    reject(&command, wire(&invalid), "bad-cb-amount");
    assert!(
        command
            .store
            .load_block(hash)
            .expect("candidate payload lookup")
            .is_none()
    );
    close(path, runtime, command);
}
