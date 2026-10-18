// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::{
    Amount, BasicFilterInputs, HistoricalBlockUndo, ScriptBuf, StoredFilterRecord,
    TransactionOutput, TurnHistory, block_merkle_root, check_block_header, fixture_block,
};
use open_bitcoin_core::{
    chainstate::Chainstate,
    consensus::{ConsensusParams, ScriptVerifyFlags, chain_proof::ProofWork},
    primitives::Block,
};

/// Validate actual recipe bodies independently before deriving snapshots or large-block facts.
pub(super) fn genuine_history(
    blocks: &[Block],
    params: ConsensusParams,
    flags: ScriptVerifyFlags,
) -> Chainstate {
    let mut source = Chainstate::new();
    for block in blocks {
        source
            .connect_block_with_current_time(
                block,
                ProofWork::ZERO,
                i64::from(block.header.time) + 1,
                flags,
                params,
            )
            .expect("actual complete peer fixture validation");
    }
    source
}

pub(super) fn large_history() -> TurnHistory {
    use open_bitcoin_core::consensus::{ConsensusParams, ScriptVerifyFlags};
    let mut history = TurnHistory::new(16, 0, 1);
    let params = ConsensusParams {
        coinbase_maturity: 1,
        ..Default::default()
    };
    let mut state = genuine_history(&history.history.blocks, params, ScriptVerifyFlags::P2SH);
    let mut block = fixture_block(
        history
            .history
            .records
            .last()
            .expect("parent")
            .identity()
            .block_hash(),
        16,
    );
    block.transactions[0].inputs[0].script_sig =
        ScriptBuf::from_bytes(vec![1, 16, 0x51]).expect("height");
    for number in 0..40_000_u32 {
        let mut script = vec![4];
        script.extend_from_slice(&number.to_le_bytes());
        script.extend([0x75, 0x51]);
        block.transactions[0].outputs.push(TransactionOutput {
            value: Amount::ZERO,
            script_pubkey: ScriptBuf::from_bytes(script).expect("distinct legal script"),
        });
    }
    block.header.time = 2_600;
    block.header.merkle_root = block_merkle_root(&block.transactions).expect("merkle").0;
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("genuine easy POW");
    let staged = state
        .stage_connect_block_with_current_time(
            &block,
            open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(17),
            2_601,
            ScriptVerifyFlags::P2SH,
            params,
        )
        .expect("genuine consensus-valid large block");
    state
        .commit_staged_connect(staged)
        .expect("commit large block");
    let position = state.tip().expect("tip");
    let inputs = BasicFilterInputs::from_historical(
        &block,
        position,
        Some(HistoricalBlockUndo {
            block_hash: position.block_hash,
            undo: &state.undo_by_block()[&position.block_hash],
        }),
    )
    .expect("complete genuine history");
    history.history.records.push(
        StoredFilterRecord::generate(
            &inputs,
            position,
            history
                .history
                .records
                .last()
                .map(StoredFilterRecord::identity)
                .as_ref(),
        )
        .expect("genuine generated filter"),
    );
    history.history.blocks.push(block);
    history.history.full = state.snapshot();
    history
}
