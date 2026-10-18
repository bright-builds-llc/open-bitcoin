// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;

impl ForkFixture {
    pub fn extended_original(&self, displaced: &ForkBranch, extra: u32) -> ForkBranch {
        let mut state = Chainstate::new();
        for block in &self.history.blocks {
            state
                .connect_block_with_current_time(
                    block,
                    open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
                    2_000_000_000,
                    self.runtime.verify_flags,
                    self.runtime.consensus_params,
                )
                .expect("independent actual original prefix validation");
        }
        let ancestor = self.history.blocks.len() - displaced.disconnect.len() - 1;
        let mut branch = ForkBranch {
            disconnect: displaced
                .connected
                .iter()
                .rev()
                .map(|anchored| anchored.block.clone())
                .collect(),
            connected: self.history.blocks[ancestor + 1..]
                .iter()
                .map(|block| AnchoredBlock {
                    block: block.clone(),
                    chain_work: open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
                })
                .collect(),
            records: self.history.records[ancestor + 1..].to_vec(),
        };
        let mut parent = self.history.blocks.last().expect("original tip").clone();
        for offset in 1..=extra {
            let height = self.history.blocks.len() as u32 - 1 + offset;
            let block = validated_child(Some(&parent), height, 0x6b, Some(&parent.transactions[0]));
            let position = state
                .connect_block_with_current_time(
                    &block,
                    open_bitcoin_core::consensus::chain_proof::ProofWork::ZERO,
                    2_000_000_000,
                    self.runtime.verify_flags,
                    self.runtime.consensus_params,
                )
                .expect("genuine stronger original extension");
            branch
                .records
                .push(record_from_history(&state, &block, branch.records.last()));
            branch.connected.push(AnchoredBlock {
                block: block.clone(),
                chain_work: position.chain_work,
            });
            parent = block;
        }
        branch
    }
}
