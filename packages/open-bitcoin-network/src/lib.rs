#![forbid(unsafe_code)]

//! Pure-core peer lifecycle, wire-message, and sync state for Open Bitcoin.

mod error;
mod header_store;
mod message;
mod peer;

pub use error::PeerId;
pub use error::{DisconnectReason, NetworkError};
pub use header_store::{HeaderEntry, HeaderStore, InsertedHeader};
pub use message::{
    HeadersMessage, InventoryList, LocalPeerConfig, MAX_HEADERS_RESULTS, MAX_INV_SIZE,
    PROTOCOL_VERSION, ParsedNetworkMessage, ServiceFlags, USER_AGENT, VersionMessage,
    WireNetworkMessage,
};
pub use peer::{ConnectionRole, PeerAction, PeerManager, PeerState};

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
