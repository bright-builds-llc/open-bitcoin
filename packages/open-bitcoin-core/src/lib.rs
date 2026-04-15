#![forbid(unsafe_code)]

//! Pure-core domain crate for Open Bitcoin.

pub use open_bitcoin_chainstate as chainstate;
pub use open_bitcoin_codec as codec;
pub use open_bitcoin_consensus as consensus;
pub use open_bitcoin_mempool as mempool;
pub use open_bitcoin_network as network;
pub use open_bitcoin_primitives as primitives;

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
