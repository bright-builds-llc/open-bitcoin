// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::{CanonicalChain, MissingValidationPrerequisite, ValidationEvidenceClass};
use open_bitcoin_codec::{
    TransactionEncoding, encode_block, encode_block_header, encode_transaction, parse_block,
};
use open_bitcoin_primitives::{Amount, Block, BlockHash, MerkleRoot, ScriptBuf, ScriptWitness};

use super::RootIdentityError;
use crate::crypto::Sha256;
use crate::{block_hash, check_proof_of_work, transaction_txid};

// Independently frozen from pinned chainparams.cpp, not from the constructors under test.
const TRANSACTION_HEX: &str = concat!(
    "01000000010000000000000000000000000000000000000000000000000000000000000000",
    "ffffffff4d04ffff001d0104455468652054696d65732030332f4a616e2f3230303920436861",
    "6e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f",
    "722062616e6b73ffffffff0100f2052a01000000434104678afdb0fe5548271967f1a67130b",
    "7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384",
    "df7ba0b8d578a4c702b6bf11d5fac00000000",
);

const VECTORS: [(CanonicalChain, &str, &str, &str); 4] = [
    (
        CanonicalChain::Mainnet,
        "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d1dac2b7c",
        "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
        "5299fac924b5a2fc19a88876a0042c19ac4d11fe69c3f66e47516e26185f9e99",
    ),
    (
        CanonicalChain::Testnet3,
        "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4adae5494dffff001d1aa4ae18",
        "000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943",
        "8e83a1ce1b5985bd639984e474cb5f01273f6884c6aab920d67c109eb37a276c",
    ),
    (
        CanonicalChain::Signet,
        "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a008f4d5fae77031e8ad22203",
        "00000008819873e925422c1ff0f99f7cc9bbb232af63a077a480a3633bee1ef6",
        "c8abc5554d8a4e1134b27050f0a6a8fc616196bfdd381fb49f49fdea6a811675",
    ),
    (
        CanonicalChain::Regtest,
        "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4adae5494dffff7f2002000000",
        "0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206",
        "a4edaa2bf93d233994bf2d2560809ad89a2e9552c08d6e511116f186a985100f",
    ),
];

fn decode_hex(input: &str) -> Vec<u8> {
    input
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = char::from(pair[0]).to_digit(16).expect("frozen hex digit");
            let low = char::from(pair[1]).to_digit(16).expect("frozen hex digit");
            ((high << 4) | low) as u8
        })
        .collect()
}

fn display_hash(mut raw: [u8; 32]) -> Vec<u8> {
    raw.reverse();
    raw.to_vec()
}

#[test]
fn canonical_chain_genesis_headers_match_frozen_wire_vectors() {
    for (chain, header_hex, _, _) in VECTORS {
        // Arrange
        let expected = decode_hex(header_hex);
        // Act
        let encoded = encode_block_header(&chain.spec().genesis_header());
        // Assert
        assert_eq!(expected.len(), 80);
        assert_eq!(encoded, expected, "{chain}");
    }
}

#[test]
fn canonical_chain_genesis_bodies_match_frozen_wire_and_round_trip() {
    for (chain, header_hex, _, _) in VECTORS {
        // Arrange
        let mut expected = decode_hex(header_hex);
        expected.push(1);
        expected.extend(decode_hex(TRANSACTION_HEX));
        // Act
        let block = chain.spec().genesis_block();
        let encoded = encode_block(&block).expect("canonical block encodes");
        let parsed = parse_block(&expected).expect("independent vector parses");
        // Assert
        assert_eq!(expected.len(), 285);
        assert_eq!(encoded, expected, "{chain}");
        assert_eq!(parsed, block, "{chain}");
        assert_eq!(chain.spec().check_genesis_block(&parsed), Ok(()));
    }
}

#[test]
fn canonical_chain_genesis_hashes_match_pinned_assertions() {
    for (chain, _, hash_hex, _) in VECTORS {
        // Arrange
        let spec = chain.spec();
        // Act
        let hash = spec.genesis_hash();
        // Assert
        assert_eq!(hash, block_hash(&spec.genesis_header()));
        assert_eq!(
            display_hash(hash.to_byte_array()),
            decode_hex(hash_hex),
            "{chain}"
        );
    }
}

#[test]
fn canonical_chain_genesis_whole_body_sha256_is_auxiliary_evidence() {
    for (chain, _, _, digest_hex) in VECTORS {
        // Arrange
        let block = chain.spec().genesis_block();
        let encoded = encode_block(&block).expect("canonical block encodes");
        // Act
        let digest = Sha256::digest(&encoded);
        // Assert
        assert_eq!(digest.to_vec(), decode_hex(digest_hex), "{chain}");
        assert_ne!(digest, chain.spec().genesis_hash().to_byte_array());
    }
}

#[test]
fn canonical_chain_genesis_transaction_matches_independent_bytes_and_txid() {
    for chain in CHAINS {
        // Arrange
        let block = chain.spec().genesis_block();
        let transaction = &block.transactions[0];
        // Act
        let encoded = encode_transaction(transaction, TransactionEncoding::WithWitness)
            .expect("coinbase encodes");
        let txid = transaction_txid(transaction).expect("coinbase txid");
        // Assert
        assert_eq!(encoded.len(), 204);
        assert_eq!(encoded, decode_hex(TRANSACTION_HEX));
        assert_eq!(
            display_hash(txid.to_byte_array()),
            decode_hex("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b")
        );
        assert_eq!(
            block.header.merkle_root.to_byte_array(),
            txid.to_byte_array()
        );
    }
}

#[test]
fn canonical_chain_genesis_rejects_all_twelve_foreign_network_pairs() {
    let mut checked_pairs = 0;
    for chain in CHAINS {
        for foreign in CHAINS.into_iter().filter(|foreign| *foreign != chain) {
            // Arrange
            let spec = chain.spec();
            let candidate = foreign.spec().genesis_block();
            // Act
            let header_result = spec.check_genesis_header(&candidate.header);
            let body_result = spec.check_genesis_block(&candidate);
            // Assert
            assert_eq!(header_result, Err(RootIdentityError::NonCanonicalHeader));
            assert_eq!(body_result, Err(RootIdentityError::NonCanonicalHeader));
            checked_pairs += 1;
        }
    }
    assert_eq!(checked_pairs, 12);
}

fn rejects_header_mutation(mutate: impl Fn(&mut open_bitcoin_primitives::BlockHeader)) {
    for chain in CHAINS {
        // Arrange
        let spec = chain.spec();
        let mut block = spec.genesis_block();
        mutate(&mut block.header);
        // Act
        let header_result = spec.check_genesis_header(&block.header);
        let body_result = spec.check_genesis_block(&block);
        // Assert
        assert_eq!(header_result, Err(RootIdentityError::NonCanonicalHeader));
        assert_eq!(body_result, Err(RootIdentityError::NonCanonicalHeader));
    }
}

#[test]
fn canonical_chain_genesis_rejects_altered_version() {
    rejects_header_mutation(|header| header.version += 1);
}

#[test]
fn canonical_chain_genesis_rejects_altered_parent() {
    rejects_header_mutation(|header| {
        header.previous_block_hash = BlockHash::from_byte_array([1; 32])
    });
}

#[test]
fn canonical_chain_genesis_rejects_altered_merkle_root() {
    rejects_header_mutation(|header| header.merkle_root = MerkleRoot::from_byte_array([1; 32]));
}

#[test]
fn canonical_chain_genesis_rejects_altered_time() {
    rejects_header_mutation(|header| header.time += 1);
}

#[test]
fn canonical_chain_genesis_rejects_altered_bits() {
    rejects_header_mutation(|header| header.bits ^= 1);
}

#[test]
fn canonical_chain_genesis_rejects_altered_nonce() {
    rejects_header_mutation(|header| header.nonce += 1);
}

fn rejects_body_mutation(mutate: impl Fn(&mut Block)) {
    for chain in CHAINS {
        // Arrange
        let spec = chain.spec();
        let mut block = spec.genesis_block();
        mutate(&mut block);
        // Act
        let result = spec.check_genesis_block(&block);
        // Assert
        assert_eq!(spec.check_genesis_header(&block.header), Ok(()));
        assert_eq!(result, Err(RootIdentityError::NonCanonicalBody));
    }
}

#[test]
fn canonical_chain_genesis_rejects_altered_reward() {
    rejects_body_mutation(|block| {
        block.transactions[0].outputs[0].value =
            Amount::from_sats(4_999_999_999).expect("valid changed reward")
    });
}

#[test]
fn canonical_chain_genesis_rejects_altered_coinbase_script() {
    rejects_body_mutation(|block| {
        block.transactions[0].inputs[0].script_sig =
            ScriptBuf::from_bytes(vec![0x51]).expect("valid changed script")
    });
}

#[test]
fn canonical_chain_genesis_rejects_altered_output_script() {
    rejects_body_mutation(|block| {
        block.transactions[0].outputs[0].script_pubkey =
            ScriptBuf::from_bytes(vec![0x51]).expect("valid changed script")
    });
}

#[test]
fn canonical_chain_genesis_rejects_missing_transaction() {
    rejects_body_mutation(|block| block.transactions.clear());
}

#[test]
fn canonical_chain_genesis_rejects_extra_transaction() {
    rejects_body_mutation(|block| block.transactions.push(block.transactions[0].clone()));
}

#[test]
fn canonical_chain_genesis_rejects_altered_sequence() {
    rejects_body_mutation(|block| block.transactions[0].inputs[0].sequence -= 1);
}

#[test]
fn canonical_chain_genesis_rejects_added_witness() {
    rejects_body_mutation(|block| {
        block.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![1]])
    });
}

#[test]
fn canonical_chain_genesis_rejects_altered_locktime() {
    rejects_body_mutation(|block| block.transactions[0].lock_time = 1);
}

#[test]
fn canonical_chain_genesis_rejects_altered_transaction_version() {
    rejects_body_mutation(|block| block.transactions[0].version = 2);
}

#[test]
fn canonical_chain_genesis_rejects_altered_coinbase_outpoint() {
    rejects_body_mutation(|block| block.transactions[0].inputs[0].previous_output.vout = 0);
}

#[test]
fn canonical_chain_genesis_rejects_arbitrary_valid_pow_null_parent_regtest_root() {
    // Arrange
    let spec = CanonicalChain::Regtest.spec();
    let mut header = spec.genesis_header();
    header.time += 1;
    header.nonce = (0..100)
        .find(|nonce| {
            header.nonce = *nonce;
            check_proof_of_work(block_hash(&header).to_byte_array(), header.bits)
                .expect("regtest target")
        })
        .expect("fixed candidate has an easily reproduced valid nonce");
    // Act
    let result = spec.check_genesis_header(&header);
    // Assert
    assert_eq!(
        header.previous_block_hash,
        BlockHash::from_byte_array([0; 32])
    );
    assert!(
        check_proof_of_work(block_hash(&header).to_byte_array(), header.bits)
            .expect("valid target")
    );
    assert_eq!(result, Err(RootIdentityError::NonCanonicalHeader));
}

#[test]
fn canonical_chain_genesis_identity_success_never_authorizes_ordinary_history() {
    for chain in CHAINS {
        // Arrange
        let spec = chain.spec();
        // Act
        let header_result = spec.check_genesis_header(&spec.genesis_header());
        let body_result = spec.check_genesis_block(&spec.genesis_block());
        // Assert
        assert_eq!(header_result, Ok(()));
        assert_eq!(body_result, Ok(()));
        assert!(spec.require_ordinary_history().is_err());
        assert!(!spec.validation_scope().ordinary_history());
    }
}

#[test]
fn canonical_chain_support_default_signet_challenge_is_a_profile_fact_only() {
    // Arrange
    let spec = CanonicalChain::Signet.spec();
    let expected = decode_hex(
        "512103ad5e0edad18cb1f0fc0d28a3d4f1f3e445640337489abb10404f2d1e086be430210359ef5021964fe22d6f8e05b2463c9540ce96883fe3b278760f048f5189f2e6c452ae",
    );
    // Act
    let maybe_challenge = spec.maybe_default_signet_challenge();
    // Assert
    assert_eq!(maybe_challenge, Some(expected.as_slice()));
    assert!(
        spec.require_ordinary_history()
            .expect_err("challenge not validated")
            .missing_prerequisites()
            .contains(&MissingValidationPrerequisite::SignetChallengeValidation)
    );
    for chain in [
        CanonicalChain::Mainnet,
        CanonicalChain::Testnet3,
        CanonicalChain::Regtest,
    ] {
        assert_eq!(chain.spec().maybe_default_signet_challenge(), None);
    }
}

#[test]
fn canonical_chain_genesis_identity_error_messages_are_fixed() {
    assert_eq!(
        RootIdentityError::NonCanonicalHeader.to_string(),
        "noncanonical genesis header"
    );
    assert_eq!(
        RootIdentityError::NonCanonicalBody.to_string(),
        "noncanonical genesis body"
    );
}

const CHAINS: [CanonicalChain; 4] = [
    CanonicalChain::Mainnet,
    CanonicalChain::Testnet3,
    CanonicalChain::Signet,
    CanonicalChain::Regtest,
];

#[test]
fn canonical_chain_support_is_identity_only_for_every_chain() {
    for chain in CHAINS {
        // Arrange
        let spec = chain.spec();
        // Act
        let scope = spec.validation_scope();
        // Assert
        assert_eq!(spec.chain(), chain);
        assert_eq!(scope.chain(), chain);
        assert_eq!(
            scope.evidence_class(),
            ValidationEvidenceClass::IdentityOnly
        );
        assert!(!scope.ordinary_history());
    }
}

#[test]
fn canonical_chain_support_lists_all_missing_prerequisites() {
    for chain in CHAINS {
        // Arrange
        let mut expected = vec![
            MissingValidationPrerequisite::CanonicalBootstrap,
            MissingValidationPrerequisite::ExactWork,
            MissingValidationPrerequisite::CandidateHistory,
        ];
        if chain == CanonicalChain::Signet {
            expected.push(MissingValidationPrerequisite::SignetChallengeValidation);
        }
        // Act
        let scope = chain.spec().validation_scope();
        // Assert
        assert_eq!(scope.missing_prerequisites(), expected);
    }
}

#[test]
fn canonical_chain_support_refuses_every_ordinary_history() {
    for chain in CHAINS {
        // Arrange
        let spec = chain.spec();
        // Act
        let error = spec
            .require_ordinary_history()
            .expect_err("identity cannot authorize history");
        // Assert
        assert_eq!(error.scope(), spec.validation_scope());
        assert_eq!(error.chain(), chain);
        assert_eq!(
            error.missing_prerequisites(),
            spec.validation_scope().missing_prerequisites()
        );
    }
}

#[test]
fn canonical_chain_support_refusal_messages_are_fixed_and_bounded() {
    // Arrange
    let expected = [
        "ordinary history validation is unsupported for mainnet: canonical_bootstrap, exact_work, candidate_history",
        "ordinary history validation is unsupported for testnet3: canonical_bootstrap, exact_work, candidate_history",
        "ordinary history validation is unsupported for signet: canonical_bootstrap, exact_work, candidate_history, signet_challenge_validation",
        "ordinary history validation is unsupported for regtest: canonical_bootstrap, exact_work, candidate_history",
    ];
    for (chain, message) in CHAINS.into_iter().zip(expected) {
        // Act
        let error = chain
            .spec()
            .require_ordinary_history()
            .expect_err("unsupported history");
        // Assert
        assert_eq!(error.to_string(), message);
        assert!(error.to_string().len() < 256);
    }
}

#[test]
fn canonical_chain_support_refusal_propagates_formatter_failure() {
    use std::fmt::{self, Write};

    // Arrange
    struct RejectedOutput;
    impl Write for RejectedOutput {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }
    let error = CanonicalChain::Mainnet
        .spec()
        .require_ordinary_history()
        .expect_err("unsupported history");
    let mut output = RejectedOutput;

    // Act
    let result = write!(&mut output, "{error}");

    // Assert
    assert_eq!(result, Err(fmt::Error));
}

#[test]
fn canonical_chain_support_names_testnet_identity_as_testnet3() {
    // Arrange
    let chain = CanonicalChain::Testnet3;
    // Act
    let name = chain.to_string();
    // Assert
    assert_eq!(name, "testnet3");
}
