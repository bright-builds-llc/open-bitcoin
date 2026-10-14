// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use crate::context::{
    mempool_recovery::recover_mempool_snapshot_from_store_handle, network::default_verify_flags,
    wallet_state::build_wallet_state_with_store,
};
use open_bitcoin_node::{
    ChainstateStore, SyncNetwork,
    core::chainstate::{CoinsView, PruneMode},
};

mod fixtures;
use fixtures::{OwnedStore, memory_handle, owner_snapshot};

const PROFILES: [(AddressNetwork, SyncNetwork, CanonicalChain); 4] = [
    (
        AddressNetwork::Mainnet,
        SyncNetwork::Mainnet,
        CanonicalChain::Mainnet,
    ),
    (
        AddressNetwork::Testnet,
        SyncNetwork::Testnet,
        CanonicalChain::Testnet3,
    ),
    (
        AddressNetwork::Signet,
        SyncNetwork::Signet,
        CanonicalChain::Signet,
    ),
    (
        AddressNetwork::Regtest,
        SyncNetwork::Regtest,
        CanonicalChain::Regtest,
    ),
];

fn config(address: AddressNetwork, sync: SyncNetwork) -> RuntimeConfig {
    let mut config = RuntimeConfig {
        chain: address,
        prune_mode: PruneMode::ManualOnly,
        ..Default::default()
    };
    config.sync.runtime.network = sync;
    config
}

fn assert_bounded(error: &CanonicalContextError) {
    let message = error.to_string();
    assert!(message.len() < 1024, "{message}");
    assert!(!message.contains("PRIVATE"), "{message}");
}

fn assert_owner_refusal<S: ChainstateStore, V: CoinsView>(
    runtime: &RuntimeConfig,
    handle: &ManagedNetworkHandle<S, V>,
    maybe_store: Option<FjallNodeStore>,
    expected: &str,
) {
    // Arrange
    let before = owner_snapshot(handle);
    // Act: exercise the actual generic ordinary supplied-owner entrypoint.
    let error = ManagedRpcContext::from_runtime_config_with_network_handle(
        runtime,
        handle.clone(),
        maybe_store,
    )
    .map(|_| ())
    .expect_err("ordinary refusal");
    // Assert
    match expected {
        "unproved" => assert!(matches!(error, CanonicalContextError::UnprovedOwner)),
        "mismatch" => assert!(matches!(error, CanonicalContextError::ChainMismatch)),
        "unsupported" => assert!(matches!(
            error,
            CanonicalContextError::UnsupportedValidationScope(_)
        )),
        _ => panic!("invalid test expectation"),
    }
    assert_bounded(&error);
    assert_eq!(owner_snapshot(handle), before);
    assert_eq!(
        handle.prune_mode().expect("prune mode"),
        PruneMode::Disabled
    );
}

#[test]
fn canonical_chain_memory_fixture_owners_refuse_before_startup_mutations() {
    for (address, sync, _) in PROFILES {
        let handle = memory_handle(None);
        assert_owner_refusal(&config(address, sync), &handle, None, "unproved");
    }
}

#[test]
fn canonical_chain_memory_bound_owners_require_matching_supported_scope() {
    for (address, sync, selected) in PROFILES {
        for (_, _, supplied) in PROFILES {
            let handle = memory_handle(Some(supplied));
            assert_owner_refusal(
                &config(address, sync),
                &handle,
                None,
                if selected == supplied {
                    "unsupported"
                } else {
                    "mismatch"
                },
            );
            assert_eq!(
                handle.maybe_canonical_chain().expect("binding"),
                Some(supplied)
            );
        }
    }
}

#[test]
fn canonical_chain_supplied_owner_refusal_makes_zero_store_calls() {
    for (address, sync, selected) in PROFILES {
        for maybe_chain in [
            None,
            Some(selected),
            Some(if selected == CanonicalChain::Mainnet {
                CanonicalChain::Regtest
            } else {
                CanonicalChain::Mainnet
            }),
        ] {
            // Arrange
            let store = fixtures::ObservedStore::default();
            let calls = std::sync::Arc::clone(&store.calls);
            let network =
                ManagedPeerNetwork::new(store, Default::default(), PolicyConfig::default());
            assert!(
                calls.load(std::sync::atomic::Ordering::Relaxed) > 0,
                "spy observes initialization"
            );
            let handle = match maybe_chain {
                Some(chain) => ManagedNetworkHandle::from_canonical_network(network, chain),
                None => ManagedNetworkHandle::from_network_fixture(network),
            };
            calls.store(0, std::sync::atomic::Ordering::Relaxed);
            // Act
            let result = ManagedRpcContext::from_runtime_config_with_network_handle(
                &config(address, sync),
                handle,
                None,
            );
            // Assert
            assert!(result.is_err());
            assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), 0);
        }
    }
}

#[test]
fn canonical_chain_supplied_owner_cannot_override_conflicting_selectors() {
    for (address, _, selected) in PROFILES {
        for (_, sync, other) in PROFILES {
            if selected == other {
                continue;
            }
            let handle = memory_handle(Some(selected));
            assert_owner_refusal(&config(address, sync), &handle, None, "mismatch");
        }
    }
}

#[test]
fn canonical_chain_ordinary_factory_aliases_never_construct_memory_authority() {
    for (address, sync, _) in PROFILES {
        // Arrange
        let runtime = config(address, sync);
        // Act
        let results = [
            ManagedRpcContext::try_from_runtime_config(&runtime),
            ManagedRpcContext::from_runtime_config(&runtime),
            ManagedRpcContext::try_from_runtime_config_with_store(&runtime, None),
            ManagedRpcContext::from_runtime_config_with_store(&runtime, None),
        ];
        // Assert
        for result in results {
            let error = result
                .map(|_| ())
                .expect_err("unsupported ordinary construction");
            assert!(matches!(
                error,
                CanonicalContextError::UnsupportedValidationScope(_)
            ));
            assert_bounded(&error);
        }
        assert!(ManagedRpcContext::for_local_operator(address).is_err());
    }
}

#[test]
fn canonical_chain_durable_supplied_owners_preserve_records_and_wallet_selection() {
    for (address, sync, selected) in PROFILES {
        // Arrange
        let fixture = OwnedStore::new();
        let mut runtime = config(address, sync);
        runtime.maybe_data_dir = Some(fixture.dir.clone());
        let before = fixture.records();
        for maybe_chain in [
            None,
            Some(selected),
            Some(if selected == CanonicalChain::Mainnet {
                CanonicalChain::Regtest
            } else {
                CanonicalChain::Mainnet
            }),
        ] {
            let handle = fixture.handle(maybe_chain);
            let before_bytes = fixtures::manifest(&fixture.dir);
            // Act / Assert
            assert_owner_refusal(
                &runtime,
                &handle,
                Some(fixture.store.clone()),
                match maybe_chain {
                    None => "unproved",
                    Some(chain) if chain == selected => "unsupported",
                    Some(_) => "mismatch",
                },
            );
            assert_eq!(fixture.records(), before);
            assert_eq!(fixtures::manifest(&fixture.dir), before_bytes);
        }
        let error = ManagedRpcContext::from_runtime_config_with_store(
            &runtime,
            Some(fixture.store.clone()),
        )
        .map(|_| ())
        .expect_err("supplied store refuses");
        assert!(matches!(
            error,
            CanonicalContextError::UnsupportedValidationScope(_)
        ));
        assert_eq!(fixture.records(), before);
        fixture.cleanup();
    }
}

#[test]
fn canonical_chain_actual_durable_runtime_handle_refuses_before_context_effects() {
    for (address, sync, _) in PROFILES {
        // Arrange: this is the daemon's exact FjallChainstateStore/FjallCoinsView handle type.
        let fixture = OwnedStore::new();
        let runtime = open_bitcoin_node::DurableSyncRuntime::open(
            fixture.store.clone(),
            open_bitcoin_node::SyncRuntimeConfig {
                network: sync,
                ..Default::default()
            },
        )
        .expect("explicit lower-level durable fixture");
        let handle = runtime.network_handle();
        let before = fixture.records();
        // Act / Assert
        assert_owner_refusal(
            &config(address, sync),
            &handle,
            Some(fixture.store.clone()),
            "unproved",
        );
        assert_eq!(fixture.records(), before);
        drop(handle);
        drop(runtime);
        fixture.cleanup();
    }
}

#[test]
fn canonical_chain_no_handle_fallbacks_preserve_closed_sources() {
    for (address, sync, selected) in PROFILES {
        for shape in ["synthetic", "foreign", "hostile", "empty", "missing"] {
            // Arrange
            let path = fixtures::closed_source(shape);
            let maybe_before = path.exists().then(|| fixtures::manifest(&path));
            let mut runtime = config(address, sync);
            runtime.maybe_data_dir = Some(path.clone());
            runtime.wallet.scope = crate::config::WalletRuntimeScope::DurableNamedRegistry;
            let network = memory_handle(None);
            let before_owner = owner_snapshot(&network);
            // Act: both real no-handle helpers must reach the ordinary opener.
            let wallet_error = build_wallet_state_with_store(&runtime, None)
                .err()
                .expect("wallet refusal");
            let mempool_error = recover_mempool_snapshot_from_store_handle(
                &runtime,
                None,
                &network,
                &PolicyConfig::default(),
                default_verify_flags(),
                ConsensusParams::default(),
            )
            .expect_err("mempool refusal");
            // Assert
            for error in [wallet_error, mempool_error] {
                let CanonicalContextError::CompatibilityRefusal(refusal) = error else {
                    panic!("typed compatibility refusal");
                };
                assert_eq!(refusal.selected_chain(), selected);
                assert_bounded(&CanonicalContextError::CompatibilityRefusal(refusal));
            }
            assert_eq!(owner_snapshot(&network), before_owner);
            assert_eq!(path.exists(), maybe_before.is_some());
            if let Some(before) = maybe_before {
                assert_eq!(fixtures::manifest(&path), before);
            }
            if path.exists() {
                std::fs::remove_dir_all(path).expect("owned cleanup");
            }
        }
    }
}

#[test]
fn canonical_chain_named_fixtures_keep_their_narrow_regression_contract() {
    for (address, sync, _) in PROFILES {
        // Arrange
        let runtime = config(address, sync);
        // Act
        let context = ManagedRpcContext::from_fixture_runtime_config(&runtime);
        let alias = ManagedRpcContext::for_fixture_local_operator(address);
        // Assert
        assert_eq!(context.chain(), address);
        assert_eq!(alias.chain(), address);
        assert_eq!(
            context
                .network
                .maybe_canonical_chain()
                .expect("fixture binding"),
            None
        );
    }
}

#[test]
fn canonical_chain_typed_storage_errors_do_not_render_backend_secrets() {
    // Arrange
    let source = StorageError::BackendFailure {
        namespace: open_bitcoin_node::storage::StorageNamespace::Wallet,
        message: "PRIVATE backend path and bytes".to_string(),
        action: open_bitcoin_node::storage::StorageRecoveryAction::RestoreFromBackup,
    };
    // Act
    let error = CanonicalContextError::from(source);
    // Assert
    assert_bounded(&error);
    assert!(matches!(
        error,
        CanonicalContextError::Storage(StorageError::BackendFailure { .. })
    ));
}
