// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/kernel/chainparams.cpp

use super::*;
use crate::coins::{CoinsBatch, CompleteCoinsSnapshot};
use open_bitcoin_consensus::CanonicalChain;
use open_bitcoin_primitives::Txid;

const CHAINS: [CanonicalChain; 4] = [
    CanonicalChain::Mainnet,
    CanonicalChain::Testnet3,
    CanonicalChain::Signet,
    CanonicalChain::Regtest,
];

fn proof(chain: CanonicalChain) -> CanonicalGenesisProof {
    let spec = chain.spec();
    CanonicalGenesisProof::prove(spec, &spec.genesis_block()).expect("canonical body")
}

fn cache(proof: &CanonicalGenesisProof) -> CoinsCache<MemoryCoinsView> {
    CoinsCache::from_parent(MemoryCoinsView::from_coins(
        HashMap::new(),
        Some(proof.hash()),
    ))
}

fn outpoint(proof: &CanonicalGenesisProof) -> OutPoint {
    OutPoint {
        txid: transaction_txid(&proof.block().transactions[0]).expect("genesis txid"),
        vout: 0,
    }
}

#[test]
fn phase166_genesis_coins_earns_one_exact_root_for_every_selector() {
    for chain in CHAINS {
        // Arrange
        let proof = proof(chain);
        // Act
        let state = Chainstate::from_canonical_genesis(&proof, cache(&proof)).expect("earned root");
        // Assert
        assert!(state.has_earned_work());
        assert_eq!(state.active_chain().len(), 1);
        let tip = state.tip().expect("genesis position");
        assert_eq!(tip.block_hash, proof.hash());
        assert_eq!(tip.header, *proof.header());
        assert_eq!(tip.header.previous_block_hash, BlockHash::default());
        assert_eq!(tip.height, 0);
        assert_eq!(tip.chain_work, proof.work());
        assert_eq!(tip.median_time_past, i64::from(proof.header().time));
        assert_eq!(state.coins_best_block(), Ok(Some(proof.hash())));
        assert!(state.undo_by_block().is_empty());
    }
}

#[test]
fn phase166_genesis_output_is_absent_without_transaction_application() {
    for chain in CHAINS {
        // Arrange
        let proof = proof(chain);
        let genesis_outpoint = outpoint(&proof);
        // Act
        let mut state =
            Chainstate::from_canonical_genesis(&proof, cache(&proof)).expect("earned root");
        let lookup = state.get_coin(&genesis_outpoint);
        let spend = state.coins_mut().spend_coin(&genesis_outpoint);
        // Assert
        assert_eq!(lookup, Ok(None));
        assert_eq!(
            spend,
            Err(ChainstateError::MissingCoin {
                outpoint: genesis_outpoint
            })
        );
        assert!(
            state
                .coins()
                .complete_snapshot()
                .expect("complete")
                .coins
                .is_empty()
        );
    }
}

#[test]
fn phase166_genesis_coins_refuses_any_coin_in_parent_or_overlay() {
    for chain in CHAINS {
        for genesis_key in [true, false] {
            for overlay in [true, false] {
                // Arrange
                let proof = proof(chain);
                let key = if genesis_key {
                    outpoint(&proof)
                } else {
                    OutPoint {
                        txid: Txid::from_byte_array([7; 32]),
                        vout: 9,
                    }
                };
                let coin = Coin {
                    output: proof.block().transactions[0].outputs[0].clone(),
                    is_coinbase: true,
                    created_height: 0,
                    created_median_time_past: 0,
                };
                let mut coins = if overlay {
                    cache(&proof)
                } else {
                    CoinsCache::from_parent(MemoryCoinsView::from_coins(
                        HashMap::from([(key.clone(), coin.clone())]),
                        Some(proof.hash()),
                    ))
                };
                if overlay {
                    coins.add_coin(key, coin, false).expect("overlay coin");
                }
                // Act
                let result = Chainstate::from_canonical_genesis(&proof, coins);
                // Assert
                assert!(matches!(result, Err(ChainstateError::WorkRebindMismatch)));
            }
        }
    }
}

#[test]
fn phase166_genesis_coins_refuses_missing_wrong_or_foreign_best_block() {
    for chain in CHAINS {
        for maybe_best_block in [
            None,
            Some(BlockHash::default()),
            Some(proof(CanonicalChain::Regtest).hash()),
        ] {
            // Arrange
            let proof = proof(chain);
            if maybe_best_block == Some(proof.hash()) {
                continue;
            }
            let coins = CoinsCache::from_parent(MemoryCoinsView::from_coins(
                HashMap::new(),
                maybe_best_block,
            ));
            // Act
            let result = Chainstate::from_canonical_genesis(&proof, coins);
            // Assert
            assert!(matches!(result, Err(ChainstateError::WorkRebindMismatch)));
        }
    }
}

struct SnapshotView {
    snapshot: CompleteCoinsSnapshot,
    complete: bool,
}

impl CoinsView for SnapshotView {
    fn complete_snapshot(&self) -> Result<CompleteCoinsSnapshot, ChainstateError> {
        if !self.complete {
            return Err(ChainstateError::CompleteCoinsSnapshotUnavailable);
        }
        Ok(self.snapshot.clone())
    }
    fn get_coin(&self, key: &OutPoint) -> Result<Option<Coin>, ChainstateError> {
        Ok(self.snapshot.coins.get(key).cloned())
    }
    fn have_coin(&self, key: &OutPoint) -> Result<bool, ChainstateError> {
        Ok(self.snapshot.coins.contains_key(key))
    }
    fn best_block(&self) -> Result<Option<BlockHash>, ChainstateError> {
        Ok(self.snapshot.maybe_best_block)
    }
    fn head_blocks(&self) -> Result<Vec<BlockHash>, ChainstateError> {
        Ok(self.snapshot.head_blocks.clone())
    }
    fn batch_write(&mut self, _: CoinsBatch, _: Option<BlockHash>) -> Result<(), ChainstateError> {
        Err(ChainstateError::CoinsStorage {
            detail: "read-only test view".to_owned(),
        })
    }
}

#[test]
fn phase166_genesis_coins_refuses_incomplete_snapshot_even_when_hints_look_empty() {
    // Arrange
    let proof = proof(CanonicalChain::Mainnet);
    let view = SnapshotView {
        snapshot: cache(&proof).complete_snapshot().expect("complete source"),
        complete: false,
    };
    // Act
    let result = Chainstate::from_canonical_genesis(&proof, CoinsCache::from_parent(view));
    // Assert
    assert!(matches!(
        result,
        Err(ChainstateError::CompleteCoinsSnapshotUnavailable)
    ));
}

#[test]
fn phase166_genesis_coins_refuses_interrupted_head_markers() {
    for heads in [
        vec![BlockHash::default()],
        vec![BlockHash::default(), BlockHash::from_byte_array([1; 32])],
    ] {
        // Arrange
        let proof = proof(CanonicalChain::Mainnet);
        let mut snapshot = cache(&proof).complete_snapshot().expect("complete");
        snapshot.head_blocks = heads;
        let view = SnapshotView {
            snapshot,
            complete: true,
        };
        // Act
        let result = Chainstate::from_canonical_genesis(&proof, CoinsCache::from_parent(view));
        // Assert
        assert!(matches!(result, Err(ChainstateError::WorkRebindMismatch)));
    }
}

#[test]
fn phase166_genesis_raw_snapshot_and_cache_imports_remain_untrusted() {
    // Arrange
    let proof = proof(CanonicalChain::Regtest);
    let earned = Chainstate::from_canonical_genesis(&proof, cache(&proof)).expect("earned");
    // Act
    let imported = Chainstate::from_snapshot(earned.snapshot());
    let cached = Chainstate::from_coins_cache(
        cache(&proof),
        earned.active_chain().to_vec(),
        HashMap::new(),
        Some(HashMap::new()),
    );
    // Assert
    for raw in [imported, cached] {
        assert!(!raw.has_earned_work());
        assert_eq!(
            raw.validate_live_origin(),
            Err(ChainstateError::UntrustedWorkLineage)
        );
    }
}

#[test]
fn phase166_genesis_source_binds_selected_parameters_and_independent_owner() {
    for chain in CHAINS {
        // Arrange
        let proof = proof(chain);
        let state = Chainstate::from_canonical_genesis(&proof, cache(&proof)).expect("earned");
        let independent =
            Chainstate::from_canonical_genesis(&proof, cache(&proof)).expect("independent");
        let params = consensus_params(&proof);
        let mut foreign = params;
        foreign.pow_limit_bits ^= 1;
        // Act
        let source = state
            .bind_live_work_source(params)
            .expect("selected source");
        let wrong_params = state.bind_live_work_source(foreign);
        let wrong_owner = independent.check_live_work_source(&source);
        // Assert
        assert!(state.check_live_work_source(&source).is_ok());
        assert!(matches!(
            wrong_params,
            Err(ChainstateError::WorkParametersMismatch)
        ));
        assert_eq!(wrong_owner, Err(ChainstateError::StageOwnerMismatch));
        assert_eq!(params.coinbase_maturity, 100);
        assert_eq!(params.subsidy_halving_interval, 210_000);
        assert_eq!(params.locktime_threshold, 500_000_000);
        assert_eq!(params.sequence_locktime_granularity, 9);
        assert_eq!(params.pow_target_spacing_seconds, 600);
        assert_eq!(params.pow_target_timespan_seconds, 1_209_600);
        assert!(
            params.enforce_bip34_height_in_coinbase
                && params.enforce_bip113_median_time_past
                && params.enforce_segwit
        );
        let expected = match chain {
            CanonicalChain::Mainnet => (0x1d00_ffff, false, false),
            CanonicalChain::Testnet3 => (0x1d00_ffff, true, false),
            CanonicalChain::Signet => (0x1e03_77ae, false, false),
            CanonicalChain::Regtest => (0x207f_ffff, true, true),
        };
        assert_eq!(
            (
                params.pow_limit_bits,
                params.allow_min_difficulty_blocks,
                params.no_pow_retargeting
            ),
            expected
        );
    }
}

#[test]
fn phase166_genesis_public_coins_mutation_invalidates_earned_lineage() {
    // Arrange
    let proof = proof(CanonicalChain::Regtest);
    let mut state = Chainstate::from_canonical_genesis(&proof, cache(&proof)).expect("earned");
    let source = state
        .bind_live_work_source(consensus_params(&proof))
        .expect("source");
    // Act
    state.coins_mut().set_best_block(proof.hash());
    // Assert
    assert!(!state.has_earned_work());
    assert_eq!(
        state.check_live_work_source(&source),
        Err(ChainstateError::UntrustedWorkLineage)
    );
}
