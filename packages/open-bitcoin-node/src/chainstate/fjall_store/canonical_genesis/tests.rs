// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/txdb.cpp

use super::super::*;
use crate::chainstate::BasicFilterStartupMode;
use open_bitcoin_core::{
    chainstate::{CoinsCache, CoinsView, FlushPolicyTime},
    consensus::CanonicalChain,
};

const CHAINS: [CanonicalChain; 4] = [
    CanonicalChain::Mainnet,
    CanonicalChain::Testnet3,
    CanonicalChain::Signet,
    CanonicalChain::Regtest,
];

fn source() -> std::path::PathBuf {
    let mut nonce = [0_u8; 16];
    getrandom::fill(&mut nonce).expect("fixture nonce");
    std::fs::canonicalize(std::env::temp_dir())
        .expect("canonical temp parent")
        .join(format!("phase166-managed-{:x}", u128::from_le_bytes(nonce)))
}

#[test]
fn phase166_managed_recovers_exact_earned_genesis_for_every_selector() {
    for chain in CHAINS {
        // Arrange
        let source = source();
        let opened = FjallNodeStore::open_canonical_genesis(&source, chain).expect("proved store");
        let expected = opened.genesis().clone();
        let (store, recovered) = opened.into_parts().expect("sealed handoff");
        // Act
        let (managed, proof) = ManagedChainstate::from_canonical_genesis(
            store,
            recovered,
            BasicFilterStartupMode::Disabled,
        )
        .expect("canonical manager");
        // Assert
        let state = managed.chainstate();
        let tip = state.tip().expect("genesis tip");
        assert_eq!(proof, expected);
        assert_eq!(tip.height, 0);
        assert_eq!(tip.block_hash, expected.hash());
        assert_eq!(tip.chain_work, expected.work());
        assert_eq!(state.coins_best_block().expect("B"), Some(expected.hash()));
        assert!(
            state
                .coins()
                .complete_snapshot()
                .expect("complete coins")
                .coins
                .is_empty()
        );
        state.validate_live_origin().expect("earned live work");
        state
            .bind_live_work_source(
                open_bitcoin_core::chainstate::engine::canonical_genesis::consensus_params(&proof),
            )
            .expect("same selected parameters");
        assert!(
            managed.maybe_validated_lineage.is_none(),
            "disabled BASIC grants no append lineage"
        );
        assert!(
            managed
                .store
                .inner()
                .maybe_basic_filter_checkpoint()
                .expect("index absent")
                .is_none()
        );
        drop(managed);
        std::fs::remove_dir_all(source).expect("cleanup");
    }
}

#[test]
fn phase166_managed_foreign_store_token_refuses_before_index_initialization() {
    // Arrange
    let first = source();
    let second = source();
    let (original, recovered) =
        FjallNodeStore::open_canonical_genesis(&first, CanonicalChain::Regtest)
            .expect("first")
            .into_parts()
            .expect("first token");
    let (foreign, other_token) =
        FjallNodeStore::open_canonical_genesis(&second, CanonicalChain::Regtest)
            .expect("second")
            .into_parts()
            .expect("second token");
    drop(other_token);
    let observer = foreign.clone();
    // Act
    let result = ManagedChainstate::from_canonical_genesis(
        foreign,
        recovered,
        BasicFilterStartupMode::Enabled,
    );
    // Assert
    assert!(result.is_err());
    assert!(
        observer
            .maybe_basic_filter_checkpoint()
            .expect("index absent")
            .is_none()
    );
    drop(result);
    drop(observer);
    drop(original);
    std::fs::remove_dir_all(first).expect("first cleanup");
    std::fs::remove_dir_all(second).expect("second cleanup");
}

#[test]
fn phase166_managed_intervening_raw_write_invalidates_even_identical_tip() {
    for change_tip in [false, true] {
        // Arrange
        let source = source();
        let opened = FjallNodeStore::open_canonical_genesis(&source, CanonicalChain::Regtest)
            .expect("proved");
        let position = opened.position().clone();
        let (store, recovered) = opened.into_parts().expect("token");
        let positions = if change_tip { vec![] } else { vec![position] };
        store
            .save_chain_meta(&positions, PersistMode::Flush)
            .expect("unvalidated tip write");
        let observer = store.clone();
        // Act
        let result = ManagedChainstate::from_canonical_genesis(
            store,
            recovered,
            BasicFilterStartupMode::Enabled,
        );
        // Assert
        assert!(result.is_err());
        assert!(
            observer
                .maybe_basic_filter_checkpoint()
                .expect("index absent")
                .is_none()
        );
        drop(result);
        drop(observer);
        std::fs::remove_dir_all(source).expect("cleanup");
    }
}

#[test]
fn phase166_managed_raw_reconstruction_cannot_earn_genesis_or_append_lineage() {
    // Arrange
    let source = source();
    let opened =
        FjallNodeStore::open_canonical_genesis(&source, CanonicalChain::Regtest).expect("proved");
    let position = opened.position().clone();
    let (store, recovered) = opened.into_parts().expect("token");
    drop(recovered);
    let state = Chainstate::from_coins_cache(
        CoinsCache::from_parent(store.coins_view()),
        vec![position],
        Default::default(),
        None,
    );
    let now = FlushPolicyTime::from_unix_seconds(0);
    // Act
    let managed = ManagedChainstate::from_chainstate(
        FjallChainstateStore::from_store(store),
        state,
        FlushLifecycle::ready(now, now, 0, false),
    );
    // Assert
    assert!(managed.chainstate().validate_live_origin().is_err());
    assert!(managed.maybe_validated_lineage.is_none());
    drop(managed);
    std::fs::remove_dir_all(source).expect("cleanup");
}

#[test]
fn phase166_managed_active_append_lineage_binds_original_generation_and_genesis() {
    for chain in CHAINS {
        // Arrange
        let source = source();
        let (store, recovered) = FjallNodeStore::open_canonical_genesis(&source, chain)
            .expect("proved")
            .into_parts()
            .expect("token");
        // Act
        let (managed, genesis) = ManagedChainstate::from_canonical_genesis(
            store,
            recovered,
            BasicFilterStartupMode::Enabled,
        )
        .expect("enabled manager");
        // Assert
        let lineage = managed
            .maybe_validated_lineage
            .as_ref()
            .expect("genuine active lineage");
        let proof = managed
            .store
            .inner()
            .maybe_basic_filter_append_proof_with_budget(
                crate::chainstate::basic_filter_turn_budget()
                    .expect("budget")
                    .normal(),
            )
            .expect("proof read")
            .expect("active proof");
        assert!(
            lineage
                .store
                .shares_basic_filter_store(managed.store.inner())
        );
        assert_eq!(lineage.generation, proof.generation());
        assert_eq!(lineage.branch, proof.branch_identity());
        assert_eq!(lineage.maybe_accepted, Some((0, genesis.hash())));
        assert_eq!(proof.durable_tip(), (0, genesis.hash()));
        managed
            .authorize_basic_filter_append_positions(&proof, &[])
            .expect("same live owner");
        drop(proof);
        drop(managed);
        std::fs::remove_dir_all(source).expect("cleanup");
    }
}

#[test]
fn phase166_managed_unvalidated_write_fences_existing_append_proof() {
    // Arrange
    let source = source();
    let (store, recovered) =
        FjallNodeStore::open_canonical_genesis(&source, CanonicalChain::Regtest)
            .expect("proved")
            .into_parts()
            .expect("token");
    let (managed, _) = ManagedChainstate::from_canonical_genesis(
        store,
        recovered,
        BasicFilterStartupMode::Enabled,
    )
    .expect("manager");
    let proof = managed
        .store
        .inner()
        .maybe_basic_filter_append_proof_with_budget(
            crate::chainstate::basic_filter_turn_budget()
                .expect("budget")
                .normal(),
        )
        .expect("read")
        .expect("active");
    managed
        .authorize_basic_filter_append_positions(&proof, &[])
        .expect("original proof");
    // Act
    managed
        .store
        .inner()
        .save_chain_meta(managed.chainstate().active_chain(), PersistMode::Flush)
        .expect("raw write");
    // Assert
    assert!(
        managed
            .authorize_basic_filter_append_positions(&proof, &[])
            .is_err()
    );
    assert!(
        managed
            .store
            .inner()
            .check_basic_filter_append_proof(&proof)
            .is_err()
    );
    drop(proof);
    drop(managed);
    std::fs::remove_dir_all(source).expect("cleanup");
}
