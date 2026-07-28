// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use core::net::IpAddr;
use std::collections::BTreeSet;

use open_bitcoin_chainstate::ChainPosition;
use open_bitcoin_codec::{
    BlockTransactions, BlockTransactionsRequest, CompactBlockPayload, PrefilledTransaction,
    SendCompactMessage, ShortId,
};
use open_bitcoin_consensus::{block_hash, check_block_header, transaction_txid, transaction_wtxid};
use open_bitcoin_primitives::{
    Amount, Block, BlockHash, BlockHeader, Hash32, MerkleRoot, MessageCommand, NetworkAddress,
    NetworkMagic, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
    TransactionOutput, Txid, Wtxid,
};

use crate::{
    BanDecision, BanReason, BanScope, BlockInFlightCleanupCause, BlockInFlightCleanupInput,
    BlockRelayActivationPolicy, BlockServingActivationConfig, BlockServingOutcomeLabel,
    BlockServingResourceGateDecision, BlockServingStatusDecision, BlockServingStatusLabel,
    CompactAnnouncementEligibility, CompactAnnouncementEligibilityReason, CompactBlockReceiveFacts,
    CompactDownloadCleanupCause, CompactRelayActivationConfig, CompactRelayCapability,
    CompactRelayPreference, ConnectionRole, DisconnectReason, HeaderStore, HeaderSyncPolicy,
    HeadersMessage, InboundAdmissionRejectionReason, InboundAdmissionSlotClass,
    InboundHandshakeState, InboundPeerRecord, InboundPermissionDecision, InventoryList,
    LocalPeerConfig, NetworkError, OrphanPolicy, OrphanReconsiderationStatus, OrphanStageInput,
    PHASE94_MAX_HEADER_LOCATOR_HASHES, PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER,
    PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS, PHASE94_MAX_INBOUND_TX_REQUESTS_PER_PEER,
    PHASE101_GETDATA_TX_INTERVAL_SECONDS, PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER,
    ParsedPeerPermissionClass, PeerAction, PeerBanEntry, PeerConnectionClass, PeerId, PeerManager,
    PeerPermissionClassRegistry, PermissionEffectLabel, ReceivedTransactionProvenance,
    RejectEvidenceTweak, RelayActivationConfig, RelayDownloadPolicy, RequestPressureInput,
    ResourceGovernanceDecision, ResourceGovernancePolicy, ServiceFlags, TxDownloadAction,
    TxDownloadSuppressionReason, TxRelayId, WireNetworkMessage, classify_block_inflight_cleanup,
};
use open_bitcoin_primitives::{InventoryType, InventoryVector};

use crate::address::{
    AddressAnnouncement, AddressDecisionLabel, AddressDecisionReason, AddressList,
    AddressNetworkKind, AddressSourceKind, GetAddrResponseDecision, LocalAdvertisementDecision,
    PHASE92_GETADDR_RESPONSE_LIMIT, PHASE92_LEARNED_ADDR_BATCH_LIMIT, RoutabilityClass,
};

use super::DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER;
use super::compact_relay::{
    BIP152_MIN_PROTOCOL_VERSION, CompactAnnouncementAction, LocalCompactRelayOfferState,
    PeerCompactAnnouncementInput, maybe_schedule_local_compact_offer,
};

mod peer_fixtures;
use peer_fixtures::*;
mod transaction_fixtures;
use transaction_fixtures::*;
mod compact_fixtures;
use compact_fixtures::*;
mod block_fixtures;
use block_fixtures::*;
mod address_fixtures;
use address_fixtures::*;
mod address_cases;
mod block_announcement_fallback_cases;
mod block_sync_lifecycle_cases;
mod block_sync_request_cap_cases;
mod block_sync_request_policy_cases;
mod compact_download_fallback_cases;
mod compact_download_lifecycle_cases;
mod compact_protocol_cases;
mod handshake_policy_cases;
mod peer_action_cases;
mod phase113_announcement_fallback_cases;
mod phase113_announcement_gate_cases;
mod phase113_negotiation_cases;
mod phase128_offer_cases;
mod transaction_lifecycle_cases;
mod transaction_relay_download_cases;
mod transaction_relay_orphan_candidate_cases;
mod transaction_relay_orphan_lifecycle_cases;
