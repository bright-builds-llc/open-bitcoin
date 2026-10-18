// Parity breadcrumbs:
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Version-message services composed from prune mode and explicit BASIC serving.

use open_bitcoin_core::chainstate::{CoinsView, PruneMode};
use open_bitcoin_network::{
    BasicPeerServingCapability, PermissionEffectLabel, ServiceFlags, advertised_service_flags,
};

use super::ManagedPeerNetwork;
use crate::ChainstateStore;

impl<S: ChainstateStore, V: CoinsView> ManagedPeerNetwork<S, V> {
    /// Configured prune mode, independent of whether any body has been deleted.
    pub fn prune_mode(&self) -> PruneMode {
        self.prune_mode
    }

    /// Updates full/limited service classification, preserving configured BASIC serving.
    pub fn set_prune_mode(&mut self, mode: PruneMode) {
        self.prune_mode = mode;
        self.apply_configured_services(self.configured_basic_peer_serving);
    }

    /// Applies explicit general BASIC peer serving, independently of catch-up.
    /// Startup must validate the BASIC prerequisite before calling this setter.
    pub fn set_basic_filter_serving(&mut self, general_configured: bool) {
        self.configured_basic_peer_serving = general_configured;
        self.apply_configured_services(general_configured);
    }

    /// Successfully resolved startup or host transition intent, independent of health.
    pub fn maybe_configured_basic_indexing(&self) -> Option<bool> {
        self.maybe_configured_basic_indexing
    }

    /// General serving intent, independent of permissioned session offers.
    pub fn configured_basic_peer_serving(&self) -> bool {
        self.configured_basic_peer_serving
    }

    pub(crate) fn record_basic_indexing_configuration(&mut self, maybe_enabled: Option<bool>) {
        self.maybe_configured_basic_indexing = maybe_enabled;
    }

    fn apply_configured_services(&mut self, general_configured: bool) {
        let capability = BasicPeerServingCapability {
            general_configured,
            ..Default::default()
        };
        let services = capability.own_services(advertised_service_flags(self.prune_mode));
        self.local_config.services = services;
        self.peer_manager.set_local_services(services);
    }

    /// Stores whether a committed prune delete has removed block data.
    ///
    /// The flag does not choose version-message services.
    pub fn set_serving_have_pruned(&mut self, have_pruned: bool) {
        self.serving_have_pruned = have_pruned;
    }

    /// Services written onto the local version message.
    pub fn local_services(&self) -> ServiceFlags {
        self.local_config.services
    }
}

/// Ordinary peers that request a historical block body are removed.
///
/// The download permission is the only exemption. `noban` does not keep
/// the peer.
pub(super) fn disconnect_for_limited_window_request(
    limited_window_refused: bool,
    active_permission_effects: &[PermissionEffectLabel],
) -> bool {
    limited_window_refused
        && (!active_permission_effects.contains(&PermissionEffectLabel::DownloadServingPolicyInput)
            || active_permission_effects
                .contains(&PermissionEffectLabel::MisbehaviorPolicyProtected))
}

#[cfg(test)]
mod phase161_peer_config_tests {
    use super::*;
    use crate::{ManagedPeerNetwork, MemoryChainstateStore};
    use open_bitcoin_network::LocalPeerConfig;

    fn store_with_validated_genesis() -> (std::path::PathBuf, crate::FjallNodeStore) {
        use open_bitcoin_core::{chainstate::Chainstate, consensus::*, primitives::*};
        static NEXT_DIRECTORY: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let transactions = vec![Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: OutPoint::null(),
                script_sig: ScriptBuf::from_bytes(vec![0, 0x51]).expect("coinbase script"),
                sequence: u32::MAX,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(5_000_000_000).expect("reward"),
                script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("output"),
            }],
            lock_time: 0,
        }];
        let mut block = Block {
            header: BlockHeader {
                version: 1,
                previous_block_hash: BlockHash::default(),
                merkle_root: block_merkle_root(&transactions).expect("merkle").0,
                time: 1_000,
                bits: 0x207fffff,
                nonce: 0,
            },
            transactions,
        };
        block.header.nonce = (0..=u32::MAX)
            .find(|nonce| {
                block.header.nonce = *nonce;
                check_block_header(&block.header).is_ok()
            })
            .expect("easy proof of work");
        let mut chainstate = Chainstate::default();
        let staged = chainstate
            .stage_connect_block_with_current_time(
                &block,
                open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(1),
                1_001,
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default(),
            )
            .expect("validated genesis");
        chainstate
            .commit_staged_connect(staged)
            .expect("accepted genesis");
        let directory = std::env::temp_dir().join(format!(
            "open-bitcoin-phase161-config-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos(),
            NEXT_DIRECTORY.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        std::fs::create_dir(&directory).expect("reserve directory");
        let store = crate::FjallNodeStore::open(&directory).expect("Fjall");
        store
            .save_block(&block, crate::PersistMode::Sync)
            .expect("genesis body");
        store
            .seed_coins_from_snapshot(&chainstate.snapshot())
            .expect("durable validated snapshot");
        (directory, store)
    }

    #[test]
    fn phase161_peer_config_actual_startup_modes_and_preserved_reopen() {
        use crate::{DurableSyncRuntime, SyncRuntimeConfig, chainstate::BasicFilterStartupMode};
        for (mode, expected) in [
            (BasicFilterStartupMode::Enabled, true),
            (BasicFilterStartupMode::Disabled, false),
        ] {
            // Arrange
            let (directory, store) = store_with_validated_genesis();
            if !expected {
                let seeded = DurableSyncRuntime::open_configured(
                    store.clone(),
                    SyncRuntimeConfig::default(),
                    BasicFilterStartupMode::Enabled,
                )
                .expect("persist lifecycle before disabling");
                drop(seeded);
            }
            // Act
            let runtime =
                DurableSyncRuntime::open_configured(store, SyncRuntimeConfig::default(), mode)
                    .expect("explicit startup");
            let explicit = runtime
                .network_handle()
                .basic_configuration_for_test()
                .expect("configuration");
            drop(runtime);
            let reopened = DurableSyncRuntime::open_configured(
                crate::FjallNodeStore::open(&directory).expect("all-handle reopen"),
                SyncRuntimeConfig::default(),
                BasicFilterStartupMode::PreserveSaved,
            )
            .expect("validated saved lifecycle");
            let saved = reopened
                .network_handle()
                .basic_configuration_for_test()
                .expect("saved configuration");
            // Assert
            assert_eq!(explicit, (Some(expected), false));
            assert_eq!(saved, explicit);
            drop(reopened);
            std::fs::remove_dir_all(directory).expect("cleanup");
        }
    }

    #[test]
    fn phase161_peer_config_actual_backend_failure_reports_unknown_transition() {
        use crate::storage::fjall_store::filters::FilterPublicationFault;
        use crate::{DurableSyncRuntime, SyncRuntimeConfig, chainstate::BasicFilterStartupMode};
        // Arrange
        let (directory, store) = store_with_validated_genesis();
        let runtime = DurableSyncRuntime::open_configured(
            store,
            SyncRuntimeConfig::default(),
            BasicFilterStartupMode::Enabled,
        )
        .expect("enabled startup");
        runtime
            .store()
            .set_basic_filter_fault(FilterPublicationFault::BeforeCommit);
        // Act
        let failure = runtime.network_handle().disable_basic_filter_index();
        let configured = runtime
            .network_handle()
            .basic_configuration_for_test()
            .expect("configuration after fault");
        // Assert
        assert!(failure.is_err());
        assert_eq!(configured, (None, false));
        drop(runtime);
        std::fs::remove_dir_all(directory).expect("cleanup");
    }

    #[test]
    fn phase161_peer_config_preserve_saved_absent_is_unknown() {
        use crate::{DurableSyncRuntime, SyncRuntimeConfig, chainstate::BasicFilterStartupMode};
        // Arrange
        let (directory, store) = store_with_validated_genesis();
        // Act
        let runtime = DurableSyncRuntime::open_configured(
            store,
            SyncRuntimeConfig::default(),
            BasicFilterStartupMode::PreserveSaved,
        )
        .expect("no saved BASIC lifecycle");
        let configured = runtime
            .network_handle()
            .basic_configuration_for_test()
            .expect("unknown configuration");
        // Assert
        assert_eq!(configured, (None, false));
        drop(runtime);
        std::fs::remove_dir_all(directory).expect("cleanup");
    }

    #[test]
    fn phase161_peer_config_compatibility_constructor_retains_known_serving_only() {
        // Arrange
        let config = LocalPeerConfig {
            services: ServiceFlags::COMPACT_FILTERS,
            ..Default::default()
        };
        // Act
        let network =
            ManagedPeerNetwork::new(MemoryChainstateStore::default(), config, Default::default());
        // Assert
        assert_eq!(network.maybe_configured_basic_indexing(), None);
        assert!(network.configured_basic_peer_serving());
    }

    #[test]
    fn phase161_peer_config_permissioned_offer_does_not_enable_general_serving() {
        use open_bitcoin_network::{
            InboundHandshakeState, InboundPeerRecord, ParsedPeerPermissionClass,
            PeerPermissionClassRegistry, WireNetworkMessage,
        };
        // Arrange
        let mut network = ManagedPeerNetwork::new(
            MemoryChainstateStore::default(),
            LocalPeerConfig {
                nonce: 5,
                ..Default::default()
            },
            Default::default(),
        );
        let class =
            ParsedPeerPermissionClass::parse("scope", ["127.0.0.1"], ["in", "all"]).expect("class");
        let decision = PeerPermissionClassRegistry::new([class])
            .resolve_inbound("127.0.0.1".parse().expect("IP"));
        let record = InboundPeerRecord {
            peer_id: 1,
            remote_endpoint: "test".into(),
            slot_class: decision.slot_class(),
            connection_class: decision.connection_class(),
            permission_decision: decision,
            handshake_state: InboundHandshakeState::Accepted,
            maybe_remote_nonce: None,
            observed_inbound_peers: 0,
            observed_outbound_peers: 0,
        };
        network
            .peer_manager_mut()
            .add_inbound_peer_record(record)
            .expect("admitted");
        // Act
        network
            .peer_manager_mut()
            .handle_message(
                1,
                WireNetworkMessage::Version(open_bitcoin_network::VersionMessage {
                    nonce: 6,
                    ..Default::default()
                }),
                0,
            )
            .expect("version");
        let scope = network
            .peer_manager()
            .maybe_basic_peer_scope()
            .expect("scope");
        network.set_prune_mode(PruneMode::ManualOnly);
        // Assert
        assert_eq!(scope.all_sessions, 1);
        assert_eq!(scope.offered_sessions, 1);
        assert!(!network.configured_basic_peer_serving());
        assert!(
            !network
                .local_services()
                .contains(ServiceFlags::COMPACT_FILTERS)
        );
        assert_eq!(network.maybe_configured_basic_indexing(), None);
    }

    #[test]
    fn phase161_peer_config_setters_and_clone_preserve_independent_facts() {
        // Arrange
        let mut network = ManagedPeerNetwork::new(
            MemoryChainstateStore::default(),
            LocalPeerConfig::default(),
            Default::default(),
        );
        network.record_basic_indexing_configuration(Some(true));
        network.set_basic_filter_serving(true);
        // Act
        network.set_prune_mode(PruneMode::ManualOnly);
        let cloned = network.clone();
        network.set_basic_filter_serving(false);
        // Assert
        assert_eq!(network.maybe_configured_basic_indexing(), Some(true));
        assert!(!network.configured_basic_peer_serving());
        assert!(
            !network
                .local_services()
                .contains(ServiceFlags::COMPACT_FILTERS)
        );
        assert!(cloned.configured_basic_peer_serving());
        assert_eq!(cloned.maybe_configured_basic_indexing(), Some(true));
        assert!(
            cloned
                .local_services()
                .contains(ServiceFlags::COMPACT_FILTERS)
        );
    }
}
