// Parity breadcrumbs:
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use open_bitcoin_chainstate::Chainstate;
use open_bitcoin_chainstate::coins::{CoinsCache, MemoryCoinsView};
use open_bitcoin_consensus::{CanonicalChain, chain_proof::ProofWork};
use std::collections::HashMap;

const CHAINS: [CanonicalChain; 4] = [
    CanonicalChain::Mainnet,
    CanonicalChain::Testnet3,
    CanonicalChain::Signet,
    CanonicalChain::Regtest,
];
const VECTORS: &str =
    include_str!("../../../../../docs/parity/fixtures/phase166-genesis-vectors.json");

fn proof(chain: CanonicalChain) -> CanonicalGenesisProof {
    let spec = chain.spec();
    CanonicalGenesisProof::prove(spec, &spec.genesis_block()).expect("canonical body")
}

fn frozen_work(chain: CanonicalChain) -> ProofWork {
    let marker = format!("\"chain\": \"{}\"", chain.name());
    let row = VECTORS.split_once(&marker).expect("frozen selector row").1;
    let hex = row
        .split_once("\"proof_le\": \"")
        .expect("independent proof")
        .1
        .split('"')
        .next()
        .expect("proof value");
    assert_eq!(hex.len(), 64);
    let mut bytes = [0; 32];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16).expect("frozen hex");
    }
    ProofWork::from_le_bytes(bytes)
}

#[test]
fn phase166_canonical_header_root_matches_independent_work_and_connected_owner() {
    for chain in CHAINS {
        // Arrange
        let proof = proof(chain);
        let coins = CoinsCache::from_parent(MemoryCoinsView::from_coins(
            HashMap::new(),
            Some(proof.hash()),
        ));
        let connected =
            Chainstate::from_canonical_genesis(&proof, coins).expect("connected genesis");
        // Act
        let store = HeaderStore::from_canonical_genesis(&proof).expect("header root");
        let facts = store.maybe_best_proof_header().expect("earned facts");
        // Assert
        assert_eq!(store.entries().count(), 1);
        assert_eq!(store.best_height(), 0);
        assert_eq!(store.best_tip().expect("tip").header, *proof.header());
        assert_eq!(facts.block_hash, proof.hash());
        assert_eq!(facts.height, 0);
        assert_eq!(facts.timestamp, proof.header().time);
        assert_eq!(facts.block_proof, frozen_work(chain));
        assert_eq!(facts.cumulative, frozen_work(chain));
        assert_eq!(
            facts.cumulative,
            connected.tip().expect("connected tip").chain_work
        );
        assert_eq!(store.best_chain_entries().len(), 1);
        assert!(store.maybe_unproved_fixture_tip().is_none());
        assert!(store.maybe_body_eligibility_binding(proof.hash()).is_none());
    }
}

#[test]
fn phase166_canonical_header_imported_exact_numbers_never_earn_proof() {
    for chain in CHAINS {
        // Arrange
        let proof = proof(chain);
        let entry = HeaderEntry {
            block_hash: proof.hash(),
            header: proof.header().clone(),
            height: 0,
            chain_work: frozen_work(chain),
        };
        // Act
        let mut imported = HeaderStore::from_entries([entry]).expect("raw numbers");
        let before = imported.clone();
        let duplicate = imported.insert_header(proof.header().clone());
        // Assert
        assert!(duplicate.is_err());
        assert_eq!(imported, before);
        assert!(imported.maybe_best_proof_header().is_none());
        assert!(imported.maybe_proof_facts(proof.hash()).is_none());
        assert!(imported.best_tip().is_none());
        assert!(imported.maybe_unproved_fixture_tip().is_some());
    }
}

#[test]
fn phase166_canonical_header_rejects_competing_root_without_owner_changes() {
    for chain in CHAINS {
        // Arrange
        let proof = proof(chain);
        let mut store = HeaderStore::from_canonical_genesis(&proof).expect("canonical root");
        let before = store.clone();
        let mut changed = proof.header().clone();
        changed.nonce ^= 1;
        // Act
        let result = store.insert_header_with_params(
            changed,
            &store.maybe_consensus_params.expect("bound params"),
        );
        // Assert
        assert!(result.is_err());
        assert_eq!(store, before);
    }
}

#[test]
fn phase166_canonical_header_rejects_foreign_or_unbound_parameters() {
    for chain in CHAINS {
        // Arrange
        let proof = proof(chain);
        let mut store = HeaderStore::from_canonical_genesis(&proof).expect("canonical root");
        let before = store.clone();
        let mut params = store.maybe_consensus_params.expect("bound params");
        params.pow_limit_bits ^= 1;
        // Act
        let foreign = store.insert_header_with_params(proof.header().clone(), &params);
        let unbound = store.insert_header(proof.header().clone());
        // Assert
        assert!(foreign.is_err());
        assert!(unbound.is_err());
        assert_eq!(store, before);
    }
}

#[test]
fn phase166_canonical_header_does_not_qualify_descendant_history_or_signet() {
    for chain in CHAINS {
        // Arrange
        let proof = proof(chain);
        let store = HeaderStore::from_canonical_genesis(&proof).expect("canonical root");
        // Act
        let refusal = proof
            .spec()
            .require_ordinary_history()
            .expect_err("history remains gated");
        // Assert
        assert!(store.maybe_best_proof_header().is_some());
        assert!(
            refusal
                .missing_prerequisites()
                .contains(&open_bitcoin_consensus::MissingValidationPrerequisite::CandidateHistory)
        );
        if chain == CanonicalChain::Signet {
            assert!(refusal.missing_prerequisites().contains(
                &open_bitcoin_consensus::MissingValidationPrerequisite::SignetChallengeValidation
            ));
        }
    }
}

#[test]
fn phase166_canonical_header_foreign_body_cannot_produce_selected_root() {
    for chain in CHAINS {
        for foreign in CHAINS.into_iter().filter(|other| *other != chain) {
            // Arrange
            let body = foreign.spec().genesis_block();
            // Act
            let result = CanonicalGenesisProof::prove(chain.spec(), &body)
                .map(|proof| HeaderStore::from_canonical_genesis(&proof));
            // Assert
            assert!(result.is_err());
        }
    }
}
