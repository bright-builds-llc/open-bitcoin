// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::CanonicalGenesisProof;
use crate::chain_spec::{CanonicalChain, MissingValidationPrerequisite, RootIdentityError};
use open_bitcoin_primitives::{Amount, Block, ScriptBuf, ScriptWitness};

const CHAINS: [CanonicalChain; 4] = [
    CanonicalChain::Mainnet,
    CanonicalChain::Testnet3,
    CanonicalChain::Signet,
    CanonicalChain::Regtest,
];

#[test]
fn phase166_complete_genesis_proves_every_selector_and_exact_work() {
    // Arrange: independently evaluated GetBlockProof integers, not imported authority.
    let expected = [0x1_0001_0001_u64, 0x1_0001_0001, 4_838_420, 2];
    for (chain, work) in CHAINS.into_iter().zip(expected) {
        let spec = chain.spec();
        let block = spec.genesis_block();
        // Act
        let proof = CanonicalGenesisProof::prove(spec, &block).expect("exact genesis");
        // Assert
        assert_eq!(proof.spec(), spec);
        assert_eq!(proof.chain(), chain);
        assert_eq!(proof.block(), &block);
        assert_eq!(proof.header(), &block.header);
        assert_eq!(proof.hash(), spec.genesis_hash());
        let mut bytes = [0; 32];
        bytes[..8].copy_from_slice(&work.to_le_bytes());
        assert_eq!(proof.work().to_le_bytes(), bytes);
    }
}

fn rejects_mutation(mutate: impl Fn(&mut Block), expected: RootIdentityError) {
    for chain in CHAINS {
        // Arrange
        let spec = chain.spec();
        let mut body = spec.genesis_block();
        mutate(&mut body);
        // Act
        let result = CanonicalGenesisProof::prove(spec, &body);
        // Assert
        assert_eq!(result, Err(expected));
    }
}

#[test]
fn phase166_changed_header_refuses_proof() {
    rejects_mutation(
        |body| body.header.nonce ^= 1,
        RootIdentityError::NonCanonicalHeader,
    );
}

#[test]
fn phase166_changed_witness_refuses_proof() {
    rejects_mutation(
        |body| body.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![1]]),
        RootIdentityError::NonCanonicalBody,
    );
}

#[test]
fn phase166_changed_output_script_refuses_proof() {
    rejects_mutation(
        |body| {
            body.transactions[0].outputs[0].script_pubkey =
                ScriptBuf::from_bytes(vec![0x51]).expect("bounded script")
        },
        RootIdentityError::NonCanonicalBody,
    );
}

#[test]
fn phase166_changed_transaction_fields_refuse_proof() {
    let mutations: [fn(&mut Block); 8] = [
        |body| body.transactions[0].version += 1,
        |body| body.transactions[0].lock_time += 1,
        |body| body.transactions[0].inputs[0].sequence -= 1,
        |body| body.transactions[0].inputs[0].previous_output.vout = 0,
        |body| body.transactions[0].inputs[0].script_sig = ScriptBuf::default(),
        |body| body.transactions[0].outputs[0].value = Amount::ZERO,
        |body| {
            body.transactions[0].outputs.clear();
        },
        |body| {
            body.transactions.clear();
        },
    ];
    for mutation in mutations {
        rejects_mutation(mutation, RootIdentityError::NonCanonicalBody);
    }
}

#[test]
fn phase166_foreign_network_genesis_refuses_proof() {
    for chain in CHAINS {
        for foreign in CHAINS.into_iter().filter(|other| *other != chain) {
            // Arrange
            let body = foreign.spec().genesis_block();
            // Act
            let result = CanonicalGenesisProof::prove(chain.spec(), &body);
            // Assert
            assert_eq!(result, Err(RootIdentityError::NonCanonicalHeader));
        }
    }
}

#[test]
fn phase166_non_genesis_null_parent_refuses_proof() {
    rejects_mutation(
        |body| body.header.time += 1,
        RootIdentityError::NonCanonicalHeader,
    );
}

#[test]
fn phase166_genesis_proof_never_qualifies_history_or_signet_challenge() {
    for chain in CHAINS {
        // Arrange
        let spec = chain.spec();
        let body = spec.genesis_block();
        // Act
        let proof = CanonicalGenesisProof::prove(spec, &body).expect("exact genesis");
        let refusal = proof
            .spec()
            .require_ordinary_history()
            .expect_err("genesis only");
        // Assert
        assert!(
            refusal
                .missing_prerequisites()
                .contains(&MissingValidationPrerequisite::CandidateHistory)
        );
        assert!(
            spec.validation_scope()
                .missing_prerequisites()
                .contains(&MissingValidationPrerequisite::CanonicalBootstrap)
        );
        assert!(
            spec.validation_scope()
                .missing_prerequisites()
                .contains(&MissingValidationPrerequisite::ExactWork)
        );
        if chain == CanonicalChain::Signet {
            assert!(
                refusal
                    .missing_prerequisites()
                    .contains(&MissingValidationPrerequisite::SignetChallengeValidation)
            );
        }
    }
}
