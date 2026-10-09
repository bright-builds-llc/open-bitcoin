// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Production reorg preparation earns source preflight before preview effects.

use open_bitcoin_core::{
    chainstate::{AnchoredBlock, CoinsView},
    consensus::{ConsensusParams, ScriptVerifyFlags},
    primitives::Block,
};

pub(in crate::network) fn prepare_reorg<S: crate::ChainstateStore, V: CoinsView>(
    manager: &crate::ManagedChainstate<S, V>,
    disconnect: &[Block],
    replacements: &[AnchoredBlock],
    flags: ScriptVerifyFlags,
    params: ConsensusParams,
) -> Result<
    crate::chainstate::PreparedChainstateReorg,
    open_bitcoin_core::chainstate::ChainstateError,
> {
    manager.prepare_reorg(disconnect, replacements, flags, params)
}
