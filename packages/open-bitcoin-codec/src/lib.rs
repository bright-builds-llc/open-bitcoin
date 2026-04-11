#![forbid(unsafe_code)]

//! Byte-level Bitcoin codecs for the Open Bitcoin pure core.

pub mod block;
pub mod compact_size;
pub mod error;
pub mod network;
pub mod primitives;
pub mod transaction;

pub use block::{encode_block, encode_block_header, parse_block, parse_block_header};
pub use compact_size::{read_compact_size, write_compact_size, MAX_SIZE};
pub use error::CodecError;
pub use network::{
    encode_block_locator,
    encode_inventory_vector,
    encode_message_header,
    encode_network_address,
    parse_block_locator,
    parse_inventory_vector,
    parse_message_header,
    parse_network_address,
};
pub use transaction::{
    encode_transaction,
    parse_transaction,
    parse_transaction_without_witness,
    TransactionEncoding,
};

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
