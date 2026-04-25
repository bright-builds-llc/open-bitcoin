#![forbid(unsafe_code)]
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::unreachable,
        clippy::todo,
        clippy::unimplemented,
        clippy::panic_in_result_fn,
    )
)]
// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Byte-level Bitcoin codecs for the Open Bitcoin pure core.

pub mod block;
pub mod compact_size;
pub mod error;
pub mod network;
pub mod primitives;
pub mod transaction;

pub use block::{encode_block, encode_block_header, parse_block, parse_block_header};
pub use compact_size::{MAX_SIZE, read_compact_size, write_compact_size};
pub use error::CodecError;
pub use network::{
    encode_block_locator, encode_inventory_vector, encode_message_header, encode_network_address,
    parse_block_locator, parse_inventory_vector, parse_message_header, parse_network_address,
};
pub use transaction::{
    TransactionEncoding, encode_transaction, parse_transaction, parse_transaction_without_witness,
};

pub const fn crate_ready() -> bool {
    true
}

#[cfg(test)]
pub(crate) mod test_support {
    pub fn decode_hex(input: &str) -> Vec<u8> {
        let mut bytes = Vec::new();
        let trimmed = input.trim();
        assert_eq!(trimmed.len() % 2, 0, "hex fixtures must use full bytes");

        let chars: Vec<char> = trimmed.chars().collect();
        for pair in chars.chunks(2) {
            let high = pair[0].to_digit(16).expect("fixture should be hex");
            let low = pair[1].to_digit(16).expect("fixture should be hex");
            bytes.push(((high << 4) | low) as u8);
        }

        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::crate_ready;

    #[test]
    fn crate_ready_reports_true() {
        assert!(crate_ready());
    }
}
