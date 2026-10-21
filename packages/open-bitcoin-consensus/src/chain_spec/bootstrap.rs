// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::{CanonicalChain, ChainSpec, RootIdentityError};
use crate::ProofWork;
use open_bitcoin_primitives::{Block, BlockHash, BlockHeader};

/// Exact complete genesis and internally derived work, without ordinary-history authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalGenesisProof {
    spec: ChainSpec,
    block: Block,
    work: ProofWork,
}

impl CanonicalGenesisProof {
    /// Prove the selected complete genesis; caller-provided work or acceptance flags are absent.
    pub fn prove(spec: ChainSpec, body: &Block) -> Result<Self, RootIdentityError> {
        spec.check_genesis_block(body)?;
        let work = crate::block_proof(body.header.bits)
            .map_err(|_| RootIdentityError::NonCanonicalHeader)?;
        Ok(Self {
            spec,
            block: body.clone(),
            work,
        })
    }

    /// Selected canonical identity; this alone grants no runtime authority.
    pub const fn spec(&self) -> ChainSpec {
        self.spec
    }

    /// Canonical profile bound to the proved body.
    pub const fn chain(&self) -> CanonicalChain {
        self.spec.chain()
    }

    /// Complete immutable genesis body, including its coinbase output script.
    pub const fn block(&self) -> &Block {
        &self.block
    }

    /// Immutable header of the proved genesis body.
    pub const fn header(&self) -> &BlockHeader {
        &self.block.header
    }

    /// Hash recomputed from the proved header in typed raw-byte order.
    pub fn hash(&self) -> BlockHash {
        crate::block_hash(self.header())
    }

    /// Exact genesis block proof, also the height-zero cumulative work.
    pub const fn work(&self) -> ProofWork {
        self.work
    }
}

#[cfg(test)]
mod tests;
