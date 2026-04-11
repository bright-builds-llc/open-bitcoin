#![forbid(unsafe_code)]

//! Invariant-bearing Bitcoin domain primitives for Open Bitcoin.

pub mod amount;
pub mod block;
pub mod hash;
pub mod network;
pub mod script;
pub mod transaction;

pub use amount::{Amount, AmountError, COIN, MAX_MONEY};
pub use block::{Block, BlockHeader};
pub use hash::{BlockHash, Hash32, HashLengthError, MerkleRoot, Txid, Wtxid};
pub use network::{
    BLOCK_LOCATOR_DUMMY_VERSION, BlockLocator, InventoryType, InventoryVector, MESSAGE_TYPE_SIZE,
    MessageCommand, MessageCommandError, MessageHeader, NetworkAddress, NetworkMagic,
};
pub use script::{
    MAX_OPS_PER_SCRIPT, MAX_PUBKEYS_PER_MULTISIG, MAX_SCRIPT_ELEMENT_SIZE, MAX_SCRIPT_SIZE,
    ScriptBuf, ScriptError, ScriptWitness,
};
pub use transaction::{OutPoint, Transaction, TransactionInput, TransactionOutput};

pub const fn crate_ready() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::crate_ready;

    #[test]
    fn crate_ready_reports_true() {
        assert!(crate_ready());
    }
}
