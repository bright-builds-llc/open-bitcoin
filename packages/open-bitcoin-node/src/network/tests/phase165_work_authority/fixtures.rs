// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.h

use super::*;

pub(super) fn fork(
    active_len: u32,
    side_len: u32,
) -> (
    ManagedPeerNetwork<MemoryChainstateStore>,
    Vec<Block>,
    Vec<Block>,
) {
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(42),
        PolicyConfig::default(),
    );
    let mut active = Vec::new();
    let mut parent = BlockHash::default();
    for height in 0..active_len {
        let block = build_block(parent, height, 5_000_000_000);
        network
            .connect_local_block(&block, verify_flags(), consensus_params())
            .expect("genuine active prefix");
        parent = block_hash(&block.header);
        active.push(block);
    }
    let mut side = Vec::new();
    parent = block_hash(&active[0].header);
    for height in 1..=side_len {
        let block = build_block(parent, height, 4_000_000_000);
        network
            .connect_stored_block(
                &block,
                ProofWork::from_le_bytes([255; 32]),
                2_000_000_000,
                verify_flags(),
                consensus_params(),
            )
            .expect("checked retained side body");
        parent = block_hash(&block.header);
        side.push(block);
    }
    (network, active, side)
}

pub(super) fn anchored(blocks: &[Block]) -> Vec<AnchoredBlock> {
    blocks
        .iter()
        .map(|block| AnchoredBlock {
            block: block.clone(),
            chain_work: ProofWork::from_le_bytes([255; 32]),
        })
        .collect()
}

pub(super) fn context() -> ReorgLifecycleContext {
    ReorgLifecycleContext::new(PolicyTime::from_unix_seconds(2_000_000_000))
}

pub(super) fn state(network: &ManagedPeerNetwork<MemoryChainstateStore>) -> String {
    format!(
        "{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}",
        network.chainstate_snapshot().expect("complete state"),
        network.peer_manager,
        network.mempool.mempool(),
        network.blocks_by_hash,
        network.chainstate,
        network.chainstate.work_preparation_observations(),
        (
            network.lifecycle_generation,
            network.dirty_generation,
            &network.peer_effect_ledger,
            &network.snapshot_effect_ledger,
            &network.peer_session_generations
        ),
        (
            &network.transactions_by_txid,
            &network.transactions_by_wtxid,
            &network.relay_serving,
            &network.block_relay_evidence,
            &network.lifecycle_evidence,
            &network.checkpoint_evidence
        )
    )
}
