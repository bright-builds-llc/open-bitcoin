// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Durable open attaches initialize's cache and lifecycle; leftover UTXOs stay unread.

use std::sync::Arc;

use open_bitcoin_core::{
    chainstate::{Chainstate, FlushPolicyTime},
    consensus::ScriptVerifyFlags,
};
use open_bitcoin_mempool::PolicyConfig;
use open_bitcoin_network::{BlockRelayActivationPolicy, RelayActivationConfig};

use crate::{
    FjallChainstateStore, FjallNodeStore, ManagedChainstate, ManagedNetworkHandle,
    ManagedPeerNetwork,
    chainstate::{BasicFilterStartupMode, initialize_configured},
};

use super::{
    DurableSyncRuntime, DurableTipAnnouncementSink, SyncRuntimeConfig, SyncRuntimeError, progress,
};

impl DurableSyncRuntime {
    /// Opens a durable runtime with all resolved network activation policies.
    pub fn open_with_runtime_activation(
        store: FjallNodeStore,
        config: SyncRuntimeConfig,
        relay_activation: RelayActivationConfig,
        block_relay_activation: BlockRelayActivationPolicy,
        inbound_enabled: bool,
    ) -> Result<Self, SyncRuntimeError> {
        Self::open_with_configured_runtime_activation(
            store,
            config,
            relay_activation,
            block_relay_activation,
            inbound_enabled,
            BasicFilterStartupMode::PreserveSaved,
        )
    }

    /// Opens configured durable indexing without implicitly activating networking.
    pub fn open_configured(
        store: FjallNodeStore,
        config: SyncRuntimeConfig,
        basic_filter_mode: BasicFilterStartupMode,
    ) -> Result<Self, SyncRuntimeError> {
        Self::open_with_configured_runtime_activation(
            store,
            config,
            RelayActivationConfig::default(),
            BlockRelayActivationPolicy::default(),
            false,
            basic_filter_mode,
        )
    }

    /// Apply explicit index startup policy before constructing the managed owner.
    pub fn open_with_configured_runtime_activation(
        store: FjallNodeStore,
        config: SyncRuntimeConfig,
        relay_activation: RelayActivationConfig,
        block_relay_activation: BlockRelayActivationPolicy,
        inbound_enabled: bool,
        basic_filter_mode: BasicFilterStartupMode,
    ) -> Result<Self, SyncRuntimeError> {
        let now = FlushPolicyTime::from_unix_seconds(0);
        let (lifecycle, _view, cache) =
            initialize_configured(&store, now, now, 0, false, u64::MAX, basic_filter_mode)?;
        let (active_chain, maybe_counts) = store.load_chain_meta_for_open()?;
        let undo_by_block = store.load_all_undo_records()?;
        let chainstate =
            Chainstate::from_coins_cache(cache, active_chain, undo_by_block, maybe_counts);
        let managed = ManagedChainstate::from_recovered_chainstate(
            FjallChainstateStore::from_store(store.clone()),
            chainstate,
            lifecycle,
        )?;
        let local_config = progress::local_peer_config(&config);
        let mut network = ManagedPeerNetwork::from_initialized_chainstate(
            managed,
            local_config,
            PolicyConfig::default(),
            config.max_blocks_in_flight_per_peer,
            relay_activation,
            block_relay_activation,
            inbound_enabled,
        );
        if let Some(header_store) = store.load_header_store()? {
            network.seed_header_store(header_store);
        }
        let network = ManagedNetworkHandle::new(network);
        network.initialize_basic_filter_index_owner()?;
        network.drive_basic_filter_index_turn()?;
        let announcement_outboxes = super::AnnouncementOutboxRegistry::default();
        let announcement_network = network.clone();
        let announcement_outboxes_for_sink = announcement_outboxes.clone();
        let durable_tip_announcement_sink: DurableTipAnnouncementSink = Arc::new(move |event| {
            let outboxes = announcement_outboxes_for_sink.snapshots()?;
            let outcomes =
                announcement_network.prepare_block_announcements(event.block(), &outboxes)?;
            announcement_outboxes_for_sink.enqueue_prepared(&announcement_network, outcomes)
        });

        let consensus_params = config.network.consensus_params();
        Ok(Self {
            store,
            network,
            config,
            verify_flags: ScriptVerifyFlags::P2SH,
            consensus_params,
            peer_identity_authority: super::PeerIdentityAuthority::default(),
            peer_backoff: std::collections::BTreeMap::new(),
            inflight_blocks: std::collections::BTreeSet::new(),
            maybe_reconcile_progress: None,
            maybe_pending_durable_tip: None,
            maybe_durable_tip_announcement_sink: Some(durable_tip_announcement_sink),
            announcement_outboxes,
            maybe_inbound_metric_status_provider: None,
        })
    }
}
