// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.h
// - packages/bitcoin-knots/src/pow.cpp

use super::*;
use open_bitcoin_consensus::chain_spec::{CanonicalChain, CanonicalGenesisProof};
use open_bitcoin_consensus::{
    BlockValidationContext, CandidateHistory, HistoryHeader, chain_proof::ProofWork,
};

/// Contextual consensus, direct absent parent and local ownership remain distinct.
#[derive(Debug)]
pub enum CanonicalHeaderAdmissionError {
    Validation {
        source: open_bitcoin_consensus::BlockValidationError,
        pinned_reason: String,
    },
    MissingParent {
        hash: BlockHash,
        legacy: NetworkError,
    },
    Network(NetworkError),
}

impl From<NetworkError> for CanonicalHeaderAdmissionError {
    fn from(error: NetworkError) -> Self {
        Self::Network(error)
    }
}

impl CanonicalHeaderAdmissionError {
    /// Consume typed cases without requiring a parent-module reexport.
    pub fn fold<T>(
        self,
        validation: impl FnOnce(open_bitcoin_consensus::BlockValidationError, String) -> T,
        missing_parent: impl FnOnce(BlockHash, NetworkError) -> T,
        network: impl FnOnce(NetworkError) -> T,
    ) -> T {
        match self {
            Self::Validation {
                source,
                pinned_reason,
            } => validation(source, pinned_reason),
            Self::MissingParent { hash, legacy } => missing_parent(hash, legacy),
            Self::Network(error) => network(error),
        }
    }
    /// Exact established error projection, including dynamic version spelling.
    pub fn into_network_error(self) -> NetworkError {
        self.fold(
            |source, reject_reason| NetworkError::InvalidHeader {
                reject_reason,
                maybe_debug_message: source.debug_message,
            },
            |_, legacy| legacy,
            |error| error,
        )
    }
}

/// Private ownership registry; numerical rows and structural context cannot construct it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CanonicalHistory {
    chain: CanonicalChain,
    admitted: BTreeMap<BlockHash, HeaderAdmission>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HeaderAdmission {
    Genesis,
    Contextual { shell_observed_now: i64 },
}

impl CanonicalHistory {
    pub(super) fn from_genesis(proof: &CanonicalGenesisProof) -> Self {
        Self {
            chain: proof.chain(),
            admitted: BTreeMap::from([(proof.hash(), HeaderAdmission::Genesis)]),
        }
    }
}

impl HeaderStore {
    /// Admit through this owner's contextual algorithm with typed failures.
    pub fn insert_canonical_header_detailed(
        &mut self,
        header: BlockHeader,
        shell_observed_now: i64,
    ) -> Result<InsertedHeader, CanonicalHeaderAdmissionError> {
        self.admit_canonical_header_inner(header, shell_observed_now)
    }
    /// The selected chain is absent for arithmetic stores, imported records and fixtures.
    pub fn maybe_canonical_chain(&self) -> Option<CanonicalChain> {
        self.maybe_canonical_history
            .as_ref()
            .map(|history| history.chain)
    }

    /// Read-only earned membership; copied work or a persisted status never creates it.
    /// ```compile_fail
    /// use open_bitcoin_network::HeaderStore;
    /// fn forge(store: &mut HeaderStore) { store.maybe_canonical_history = None; }
    /// ```
    pub fn is_canonically_admitted(&self, hash: BlockHash) -> bool {
        self.maybe_canonical_history
            .as_ref()
            .is_some_and(|history| history.admitted.contains_key(&hash))
    }

    /// Re-admit parent-first stored header bytes from a proved root. Imported height,
    /// work, ordering and status confer no authority; all descendant gates run afresh.
    pub fn readmit_canonical_entries(
        proof: &CanonicalGenesisProof,
        entries: impl IntoIterator<Item = HeaderEntry>,
        shell_observed_now: i64,
    ) -> Result<Self, NetworkError> {
        let mut store = Self::from_canonical_genesis(proof)?;
        for entry in entries {
            if entry.header == *proof.header() {
                continue;
            }
            store.insert_canonical_header(entry.header, shell_observed_now)?;
        }
        Ok(store)
    }

    /// Recover the exact admitted candidate's structural facts from earned parents.
    /// Genesis has no parent context and is handled by its distinct proved-root path.
    pub fn canonical_header_history(
        &self,
        hash: BlockHash,
    ) -> Result<CandidateHistory, NetworkError> {
        if !self.is_canonically_admitted(hash) {
            return Err(invalid_header("unproved-canonical-header"));
        }
        let entry = self
            .entries
            .get(&hash)
            .ok_or(NetworkError::MissingHeaderAncestor(hash))?;
        self.earned_parent_history(&entry.header)
    }

    /// Body replay uses the original shell clock evidence held by this owner.
    /// Callers cannot replace it with candidate time or imported persisted labels.
    pub fn canonical_header_validation_context(
        &self,
        header: &BlockHeader,
    ) -> Result<BlockValidationContext, NetworkError> {
        let hash = block_hash(header);
        let maybe_admission = self
            .maybe_canonical_history
            .as_ref()
            .and_then(|history| history.admitted.get(&hash))
            .copied();
        let Some(HeaderAdmission::Contextual {
            shell_observed_now: now,
        }) = maybe_admission
        else {
            return Err(invalid_header("missing-canonical-header-admission"));
        };
        let history = self.canonical_header_history(hash)?;
        check_context(&history, now)?;
        Ok(history.block_validation_context(now))
    }

    pub(super) fn admit_canonical_header(
        &mut self,
        header: BlockHeader,
        shell_observed_now: i64,
    ) -> Result<InsertedHeader, NetworkError> {
        self.admit_canonical_header_inner(header, shell_observed_now)
            .map_err(CanonicalHeaderAdmissionError::into_network_error)
    }

    fn admit_canonical_header_inner(
        &mut self,
        header: BlockHeader,
        shell_observed_now: i64,
    ) -> Result<InsertedHeader, CanonicalHeaderAdmissionError> {
        let candidate_hash = block_hash(&header);
        if self.is_canonically_admitted(candidate_hash) {
            let existing = self
                .entries
                .get(&candidate_hash)
                .ok_or(NetworkError::MissingHeaderAncestor(candidate_hash))?;
            return Ok(InsertedHeader {
                block_hash: candidate_hash,
                height: existing.height,
                chain_work: existing.chain_work,
            });
        }
        let (history, parent_work) = self.earned_parent_history_detailed(&header)?;
        check_context_detailed(&history, shell_observed_now)?;
        let hash = history.candidate_hash();
        let inserted = InsertedHeader {
            block_hash: hash,
            height: history.height(),
            chain_work: parent_work.wrapping_add(
                block_proof(header.bits).map_err(|_| invalid_header("bad-diffbits"))?,
            ),
        };
        let lineage = self
            .maybe_canonical_history
            .as_mut()
            .ok_or_else(|| invalid_header("missing-canonical-root"))?;
        lineage
            .admitted
            .insert(hash, HeaderAdmission::Contextual { shell_observed_now });
        self.parents.insert(hash, Some(header.previous_block_hash));
        self.entries.insert(
            hash,
            HeaderEntry {
                block_hash: hash,
                header,
                height: inserted.height,
                chain_work: inserted.chain_work,
            },
        );
        self.accepted_headers.insert(hash);
        self.update_best_tip(hash);
        Ok(inserted)
    }

    fn earned_parent_history(
        &self,
        header: &BlockHeader,
    ) -> Result<CandidateHistory, NetworkError> {
        self.earned_parent_history_detailed(header)
            .map(|(history, _)| history)
            .map_err(CanonicalHeaderAdmissionError::into_network_error)
    }

    fn earned_parent_history_detailed(
        &self,
        header: &BlockHeader,
    ) -> Result<(CandidateHistory, ProofWork), CanonicalHeaderAdmissionError> {
        let lineage = self
            .maybe_canonical_history
            .as_ref()
            .ok_or_else(|| invalid_header("missing-canonical-root"))?;
        // Additional checkpoint prerequisites and Signet body challenge remain unimplemented.
        match lineage.chain {
            CanonicalChain::Regtest => {}
            CanonicalChain::Signet => {
                return Err(invalid_header("signet-challenge-validation-unavailable").into());
            }
            _ => {
                return Err(
                    invalid_header("canonical-checkpoint-prerequisites-unavailable").into(),
                );
            }
        }
        let mut hash = header.previous_block_hash;
        if !self.entries.contains_key(&hash) && !lineage.admitted.contains_key(&hash) {
            return Err(CanonicalHeaderAdmissionError::MissingParent {
                hash,
                legacy: invalid_header("unproved-canonical-header-parent"),
            });
        }
        let parent_work = self
            .entries
            .get(&hash)
            .ok_or(NetworkError::MissingHeaderAncestor(hash))?
            .chain_work;
        let mut rows = Vec::new();
        // Parent MTP and the parent's 144-lookback MTP need at most155 rows.
        // Regtest's interval144 and timely recovery are contained in that suffix.
        for _ in 0..155 {
            if !lineage.admitted.contains_key(&hash) {
                return Err(invalid_header("unproved-canonical-header-parent").into());
            }
            let entry = self
                .entries
                .get(&hash)
                .ok_or(NetworkError::MissingHeaderAncestor(hash))?;
            rows.push(HistoryHeader {
                chain: lineage.chain,
                height: entry.height,
                block_hash: hash,
                header: entry.header.clone(),
            });
            if entry.height == 0 {
                break;
            }
            hash = entry.header.previous_block_hash;
        }
        let mut rows = rows.into_iter();
        let parent = rows
            .next()
            .ok_or_else(|| invalid_header("missing-canonical-header-parent"))?;
        let history = CandidateHistory::check(
            lineage.chain.spec().rules(),
            header.clone(),
            parent,
            rows.collect(),
        )
        .map_err(|error| NetworkError::InvalidHeader {
            reject_reason: "invalid-canonical-header-context".to_owned(),
            maybe_debug_message: Some(error.to_string()),
        })?;
        Ok((history, parent_work))
    }
}

fn check_context(history: &CandidateHistory, now: i64) -> Result<(), NetworkError> {
    check_context_detailed(history, now).map_err(CanonicalHeaderAdmissionError::into_network_error)
}

fn check_context_detailed(
    history: &CandidateHistory,
    now: i64,
) -> Result<(), CanonicalHeaderAdmissionError> {
    history
        .check_header_rules(now)
        .map_err(|source| CanonicalHeaderAdmissionError::Validation {
            pinned_reason: if source.reject_reason == "bad-version" {
                format!("bad-version(0x{:08x})", history.candidate().version)
            } else {
                source.reject_reason.to_owned()
            },
            source,
        })
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod submission_failure_tests {
    use super::*;
    use crate::BodyEligibilityError;
    use open_bitcoin_primitives::{Amount, Block, ScriptBuf, ScriptWitness, TransactionOutput};

    fn mine(block: &mut Block, params: ConsensusParams) {
        block.header.merkle_root =
            open_bitcoin_consensus::crypto::block_merkle_root(&block.transactions)
                .expect("actual merkle")
                .0;
        block.header.nonce = 0;
        while open_bitcoin_consensus::block::check_block_header_with_params(&block.header, &params)
            .is_err()
        {
            block.header.nonce += 1;
        }
    }

    fn fixture() -> (HeaderStore, Block, ConsensusParams) {
        let chain = open_bitcoin_consensus::CanonicalChain::Regtest;
        let root = chain.spec().genesis_block();
        let proof =
            open_bitcoin_consensus::chain_spec::CanonicalGenesisProof::prove(chain.spec(), &root)
                .expect("genuine root");
        let mut store = HeaderStore::from_canonical_genesis(&proof).expect("actual header owner");
        let params = store.maybe_consensus_params.expect("held params");
        store
            .check_body_eligibility(&root, params, 2_000_000_000)
            .expect("genuine root linkage");
        let mut block = root;
        block.header.previous_block_hash = proof.hash();
        block.header.version = 4;
        block.header.time += 1;
        block.transactions[0].inputs[0].script_sig =
            ScriptBuf::from_serialized_bytes(vec![0x51, 0]);
        mine(&mut block, params);
        (store, block, params)
    }

    #[test]
    fn phase167_submission_context_failure_only_has_no_linkage_or_sequence_release() {
        // Arrange
        let (mut store, mut block, params) = fixture();
        let root_sequence = store
            .maybe_body_eligibility(block.header.previous_block_hash)
            .expect("root")
            .maybe_sequence
            .expect("actual root arrival");
        block.transactions[0].inputs[0].script_sig =
            ScriptBuf::from_serialized_bytes(vec![0x52, 0]);
        mine(&mut block, params);
        store
            .insert_canonical_header(block.header.clone(), 2_000_000_000)
            .expect("admitted header");
        let hash = block_hash(&block.header);
        let mut descendant = block.clone();
        descendant.header.previous_block_hash = hash;
        descendant.header.time += 1;
        descendant.transactions[0].inputs[0].script_sig =
            ScriptBuf::from_serialized_bytes(vec![0x52, 0]);
        mine(&mut descendant, params);
        store
            .insert_canonical_header(descendant.header.clone(), 2_000_000_000)
            .expect("retained descendant header");
        store
            .check_submission_body_eligibility_detailed(&descendant, params, 0)
            .expect("linkage is not acceptance");
        let child = block_hash(&descendant.header);
        assert!(
            store
                .maybe_body_eligibility(child)
                .expect("actual pending body")
                .maybe_sequence
                .is_none()
        );
        let pending_before = store
            .maybe_body_eligibility(child)
            .expect("preexisting pending child");
        let mut legacy = store.clone();
        let mut cached = store.clone();
        let legacy_error = legacy
            .check_body_eligibility(&block, params, 0)
            .expect_err("legacy error");
        assert_eq!(
            cached
                .check_cached_body_eligibility_detailed(&block, params, 0)
                .expect_err("cached error")
                .into_network_error(),
            legacy_error
        );
        assert!(
            legacy.maybe_body_eligibility(hash).is_none()
                && cached.maybe_body_eligibility(hash).is_none()
        );
        let epoch = store.body_eligibility_epoch();
        assert!(epoch.is_current());
        // Act
        let error = store
            .check_submission_body_eligibility_detailed(&block, params, i64::MIN)
            .expect_err("actual contextual consensus");
        // Assert
        assert!(
            matches!(error,BodyEligibilityError::Validation(source) if source.result==open_bitcoin_consensus::BlockValidationResult::Consensus&&source.reject_reason=="bad-cb-height")
        );
        let facts = store
            .maybe_body_eligibility(hash)
            .expect("genuine failed-only fact");
        assert!(facts.failed && !facts.available && facts.maybe_sequence.is_none());
        assert!(store.maybe_body_eligibility_binding(hash).is_none());
        assert!(!epoch.is_current());
        assert_eq!(
            store.maybe_body_eligibility(child),
            Some(pending_before),
            "failure does not release existing pending child"
        );
        let epoch = store.body_eligibility_epoch();
        store
            .check_submission_body_eligibility_detailed(&block, params, 0)
            .expect_err("idempotent failure");
        assert!(epoch.is_current());
        assert_eq!(store.maybe_body_eligibility(hash), Some(facts));
        let mut valid = block.clone();
        valid.transactions[0].inputs[0].script_sig =
            ScriptBuf::from_serialized_bytes(vec![0x51, 0]);
        mine(&mut valid, params);
        store
            .insert_canonical_header(valid.header.clone(), 2_000_000_000)
            .expect("different valid header");
        store
            .check_submission_body_eligibility_detailed(&valid, params, 0)
            .expect("actual valid body");
        assert_eq!(
            store
                .maybe_body_eligibility(block_hash(&valid.header))
                .expect("receipt")
                .maybe_sequence,
            root_sequence.checked_add(1)
        );
        assert!(
            store
                .eligible_body_candidates(&[block.header.previous_block_hash])
                .iter()
                .all(|facts| facts.block_hash != child)
        );
    }

    #[test]
    fn phase167_submission_noncontextual_and_mutated_errors_never_cache_failure() {
        for case in 0..7 {
            // Arrange
            let (mut store, mut block, mut params) = fixture();
            if case == 2 {
                block.transactions[0].inputs[0].script_sig = ScriptBuf::default();
                mine(&mut block, params);
            }
            if case == 3 {
                let commitment = open_bitcoin_consensus::crypto::double_sha256(&[0; 64]);
                block.transactions[0].outputs.push(TransactionOutput {
                    value: Amount::ZERO,
                    script_pubkey: ScriptBuf::from_serialized_bytes(
                        [
                            vec![0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed],
                            commitment.to_vec(),
                        ]
                        .concat(),
                    ),
                });
                block.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![7]]);
                mine(&mut block, params);
            }
            if case != 4 {
                store
                    .insert_canonical_header(block.header.clone(), 2_000_000_000)
                    .expect("header admission");
            }
            if case == 5 {
                let entry = store
                    .entries
                    .get_mut(&block_hash(&block.header))
                    .expect("actual retained entry");
                entry.header.nonce += 1;
                while open_bitcoin_consensus::block::check_block_header_with_params(
                    &entry.header,
                    &params,
                )
                .is_err()
                {
                    entry.header.nonce += 1;
                }
            }
            if case == 6 {
                store
                    .maybe_canonical_history
                    .as_mut()
                    .expect("actual owner")
                    .admitted
                    .insert(
                        block_hash(&block.header),
                        HeaderAdmission::Contextual {
                            shell_observed_now: 0,
                        },
                    );
            }
            if case == 0 {
                params.coinbase_maturity += 1;
            }
            if case == 1 {
                store = HeaderStore::default();
            }
            let epoch = store.body_eligibility_epoch();
            // Act
            let error = store
                .check_submission_body_eligibility_detailed(&block, params, 0)
                .expect_err("actual earlier gate");
            // Assert
            if case == 2 {
                assert!(matches!(error, BodyEligibilityError::Validation(source)
                    if source.result == open_bitcoin_consensus::BlockValidationResult::Consensus));
            } else if case == 3 {
                assert!(matches!(error, BodyEligibilityError::Validation(source)
                    if source.result == open_bitcoin_consensus::BlockValidationResult::Mutated));
            } else {
                assert!(matches!(case, 0 | 1 | 4 | 5 | 6));
                assert!(matches!(error, BodyEligibilityError::Network(_)));
            }
            assert!(epoch.is_current());
            assert!(
                store
                    .maybe_body_eligibility(block_hash(&block.header))
                    .is_none()
            );
        }
    }

    #[test]
    fn phase167_submission_failure_cache_capacity_is_operational_without_effects() {
        // Arrange
        let (mut store, mut block, params) = fixture();
        block.transactions[0].inputs[0].script_sig =
            ScriptBuf::from_serialized_bytes(vec![0x52, 0]);
        mine(&mut block, params);
        store
            .insert_canonical_header(block.header.clone(), 2_000_000_000)
            .expect("admitted header");
        let capacity = store.replace_submission_capacity_for_test(1);
        let epoch = store.body_eligibility_epoch();
        // Act / Assert
        assert!(matches!(
            store.check_submission_body_eligibility_detailed(&block, params, 0),
            Err(BodyEligibilityError::Network(_))
        ));
        assert!(epoch.is_current());
        assert!(
            store
                .maybe_body_eligibility(block_hash(&block.header))
                .is_none()
        );
        store.replace_submission_capacity_for_test(capacity);
        assert!(matches!(
            store.check_submission_body_eligibility_detailed(&block, params, 0),
            Err(BodyEligibilityError::Validation(_))
        ));
    }
    #[test]
    fn phase167_submission_failure_cache_preserves_existing_genuine_linkage() {
        // Arrange: obtain genuine historical facts through the checked body owner.
        let (mut store, block, params) = fixture();
        store
            .insert_canonical_header(block.header.clone(), 2_000_000_000)
            .expect("admitted header");
        store
            .check_submission_body_eligibility_detailed(&block, params, 0)
            .expect("real checked linkage");
        let hash = block_hash(&block.header);
        let before = store.maybe_body_eligibility(hash).expect("genuine facts");
        assert!(
            before.available
                && before.maybe_sequence.is_some()
                && store.maybe_body_eligibility_binding(hash).is_some()
        );
        let epoch = store.body_eligibility_epoch();
        // Act: mechanical cache conservation; conformance causality is tested separately.
        store
            .retain_submission_failure_for_test(hash)
            .expect("private cache operation");
        // Assert
        let after = store.maybe_body_eligibility(hash).expect("retained facts");
        assert!(
            after.failed && after.available && store.maybe_body_eligibility_binding(hash).is_some()
        );
        assert_eq!(after.maybe_sequence, before.maybe_sequence);
        assert!(!epoch.is_current());
    }
}
