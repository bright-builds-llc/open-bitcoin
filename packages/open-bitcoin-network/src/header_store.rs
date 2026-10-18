// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/pow.cpp

use std::collections::{BTreeMap, BTreeSet};

use open_bitcoin_chainstate::ChainPosition;
use open_bitcoin_consensus::{
    ConsensusParams, block_hash,
    chain_proof::block_proof,
    crypto::{check_proof_of_work, check_proof_of_work_with_params},
};
use open_bitcoin_primitives::{BlockHash, BlockHeader, BlockLocator, Hash32};

use crate::error::NetworkError;

mod proof;
pub use proof::HeaderProofFacts;
mod eligibility;
pub use eligibility::{BodyEligibilityError, BodyEligibilityFacts};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderEntry {
    pub block_hash: BlockHash,
    pub header: BlockHeader,
    pub height: u32,
    pub chain_work: open_bitcoin_consensus::chain_proof::ProofWork,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InsertedHeader {
    pub block_hash: BlockHash,
    pub height: u32,
    pub chain_work: open_bitcoin_consensus::chain_proof::ProofWork,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HeaderStore {
    entries: BTreeMap<BlockHash, HeaderEntry>,
    parents: BTreeMap<BlockHash, Option<BlockHash>>,
    best_tip: Option<BlockHash>,
    accepted_headers: BTreeSet<BlockHash>,
    maybe_consensus_params: Option<ConsensusParams>,
    maybe_unproved_fixture_tip: Option<BlockHash>,
    body_eligibility: eligibility::BodyEligibilityRegistry,
}

impl HeaderStore {
    /// Project selected admitted proofs in existing best-chain order, then retained sides.
    /// Generic/unbound and imported numerical facts never authorize body requests.
    pub fn requestable_header_entries(&self, params: &ConsensusParams) -> Vec<HeaderEntry> {
        if self.maybe_consensus_params.as_ref() != Some(params) {
            return Vec::new();
        }
        let mut entries = self.best_chain_entries();
        entries.retain(|entry| self.maybe_proof_facts(entry.block_hash).is_some());
        let best: BTreeSet<_> = entries.iter().map(|entry| entry.block_hash).collect();
        entries.extend(
            self.entries
                .values()
                .filter(|entry| {
                    !best.contains(&entry.block_hash)
                        && self.maybe_proof_facts(entry.block_hash).is_some()
                })
                .cloned(),
        );
        entries
    }

    /// Import numerical/legacy entries without proving ancestry or reception order.
    pub fn from_entries(
        entries: impl IntoIterator<Item = HeaderEntry>,
    ) -> Result<Self, NetworkError> {
        let mut store = Self::default();
        for entry in entries {
            let maybe_parent = if entry.height == 0 {
                None
            } else {
                Some(entry.header.previous_block_hash)
            };
            store.parents.insert(entry.block_hash, maybe_parent);
            store.entries.insert(entry.block_hash, entry);
        }

        for maybe_parent in store.parents.values() {
            let Some(parent) = maybe_parent else {
                continue;
            };
            if !store.entries.contains_key(parent) {
                return Err(NetworkError::MissingHeaderAncestor(*parent));
            }
        }

        for block_hash in store.entries.keys().copied().collect::<Vec<_>>() {
            store.update_unproved_fixture_tip(block_hash);
        }

        Ok(store)
    }

    pub fn entries(&self) -> impl Iterator<Item = &HeaderEntry> {
        self.entries.values()
    }

    pub fn seed_from_chain(&mut self, active_chain: &[ChainPosition]) {
        self.entries.clear();
        self.parents.clear();
        self.best_tip = None;
        self.accepted_headers.clear();
        self.maybe_consensus_params = None;
        self.maybe_unproved_fixture_tip = None;
        self.body_eligibility = eligibility::BodyEligibilityRegistry::default();

        for position in active_chain {
            self.record_position(position);
        }
    }

    /// Record an unproved position; never overwrite an accepted header or select it.
    pub fn record_position(&mut self, position: &ChainPosition) {
        let block_hash = position.block_hash;
        if self.accepted_headers.contains(&block_hash) {
            return;
        }
        self.parents.insert(
            block_hash,
            if position.height == 0 {
                None
            } else {
                Some(position.previous_block_hash())
            },
        );
        self.entries.insert(
            block_hash,
            HeaderEntry {
                block_hash,
                header: position.header.clone(),
                height: position.height,
                chain_work: position.chain_work,
            },
        );
        if self.accepted_headers.is_empty() {
            self.update_unproved_fixture_tip(block_hash);
        }
    }

    /// Admit hash/target-checked ancestry without asserting a selected network limit.
    pub fn insert_header(&mut self, header: BlockHeader) -> Result<InsertedHeader, NetworkError> {
        self.admit_header(header, None)
    }

    /// Admit hash/target-checked ancestry for the selected numerical consensus parameters.
    /// Contextual difficulty and canonical history gates remain adapter responsibilities.
    pub fn insert_header_with_params(
        &mut self,
        header: BlockHeader,
        params: &ConsensusParams,
    ) -> Result<InsertedHeader, NetworkError> {
        self.admit_header(header, Some(params))
    }

    fn admit_header(
        &mut self,
        header: BlockHeader,
        maybe_params: Option<&ConsensusParams>,
    ) -> Result<InsertedHeader, NetworkError> {
        let block_hash = block_hash(&header);
        if self
            .maybe_consensus_params
            .is_some_and(|selected| maybe_params != Some(&selected))
        {
            return Err(invalid_header("header-consensus-params-mismatch"));
        }
        if maybe_params.is_some()
            && !self.accepted_headers.is_empty()
            && self.maybe_consensus_params.is_none()
        {
            return Err(invalid_header("unbound-header-ancestry"));
        }
        if let Some(existing) = self.entries.get(&block_hash) {
            if !self.accepted_headers.contains(&block_hash) {
                return Err(invalid_header("unproved-header"));
            }
            return Ok(InsertedHeader {
                block_hash: existing.block_hash,
                height: existing.height,
                chain_work: existing.chain_work,
            });
        }

        let maybe_parent_hash = if header.previous_block_hash.to_byte_array() == [0_u8; 32] {
            None
        } else {
            Some(header.previous_block_hash)
        };

        let (height, chain_work) = if let Some(parent_hash) = maybe_parent_hash {
            let Some(parent) = self.entries.get(&parent_hash) else {
                return Err(NetworkError::MissingHeaderAncestor(parent_hash));
            };
            if !self.accepted_headers.contains(&parent_hash)
                || parent.block_hash != open_bitcoin_consensus::block_hash(&parent.header)
            {
                return Err(invalid_header("unproved-header-parent"));
            }
            (
                parent
                    .height
                    .checked_add(1)
                    .ok_or_else(|| invalid_header("header-height-overflow"))?,
                parent.chain_work.wrapping_add(
                    block_proof(header.bits).map_err(|_| invalid_header("bad-diffbits"))?,
                ),
            )
        } else {
            if !self.entries.is_empty() {
                return Err(invalid_header("bad-prevblk"));
            }
            (
                0,
                block_proof(header.bits).map_err(|_| invalid_header("bad-diffbits"))?,
            )
        };

        let valid_hash = match maybe_params {
            Some(params) => {
                check_proof_of_work_with_params(block_hash.to_byte_array(), header.bits, params)
            }
            None => check_proof_of_work(block_hash.to_byte_array(), header.bits),
        }
        .map_err(|_| invalid_header("bad-diffbits"))?;
        if !valid_hash {
            return Err(invalid_header("high-hash"));
        }

        self.parents.insert(block_hash, maybe_parent_hash);
        self.entries.insert(
            block_hash,
            HeaderEntry {
                block_hash,
                header,
                height,
                chain_work,
            },
        );
        self.accepted_headers.insert(block_hash);
        self.maybe_consensus_params = maybe_params.copied();
        self.update_best_tip(block_hash);

        Ok(InsertedHeader {
            block_hash,
            height,
            chain_work,
        })
    }

    pub fn contains(&self, hash: &BlockHash) -> bool {
        self.entries.contains_key(hash)
    }

    pub fn entry(&self, hash: &BlockHash) -> Option<&HeaderEntry> {
        self.entries.get(hash)
    }

    pub fn best_height(&self) -> i32 {
        self.best_tip
            .and_then(|hash| self.entries.get(&hash))
            .map_or(-1, |entry| entry.height as i32)
    }

    pub fn best_tip(&self) -> Option<&HeaderEntry> {
        self.best_tip.and_then(|hash| self.entries.get(&hash))
    }

    /// Numerical fixture projection only. Import/restart reception order is unproved.
    pub fn maybe_unproved_fixture_tip(&self) -> Option<&HeaderEntry> {
        self.maybe_unproved_fixture_tip
            .and_then(|hash| self.entries.get(&hash))
    }

    fn update_unproved_fixture_tip(&mut self, candidate_hash: BlockHash) {
        let Some(candidate) = self.entries.get(&candidate_hash) else {
            return;
        };
        if self
            .maybe_unproved_fixture_tip()
            .is_none_or(|current| candidate.chain_work > current.chain_work)
        {
            self.maybe_unproved_fixture_tip = Some(candidate_hash);
        }
    }

    pub fn best_chain_entries(&self) -> Vec<HeaderEntry> {
        self.best_chain_hashes()
            .into_iter()
            .filter_map(|hash| self.entries.get(&hash).cloned())
            .collect()
    }

    pub fn ancestor_at_height(
        &self,
        start_hash: BlockHash,
        target_height: u32,
    ) -> Option<&HeaderEntry> {
        let mut current = self.entries.get(&start_hash)?;
        if target_height > current.height {
            return None;
        }

        for _ in 0..self.entries.len() {
            if current.height == target_height {
                return Some(current);
            }
            let parent_hash = self.parents.get(&current.block_hash).copied().flatten()?;
            current = self.entries.get(&parent_hash)?;
        }
        None
    }

    pub fn median_time_past(&self, tip_hash: BlockHash) -> Option<i64> {
        let mut times = Vec::new();
        let mut maybe_current = Some(tip_hash);
        while let Some(current_hash) = maybe_current {
            let entry = self.entries.get(&current_hash)?;
            times.push(i64::from(entry.header.time));
            if times.len() == 11 {
                break;
            }
            maybe_current = self.parents.get(&current_hash).copied().flatten();
        }
        times.sort_unstable();
        Some(times[times.len() / 2])
    }

    pub fn locator(&self) -> BlockLocator {
        let best_chain = self.best_chain_hashes();
        if best_chain.is_empty() {
            return BlockLocator::default();
        }

        let mut hashes = Vec::new();
        let mut index = best_chain.len() - 1;
        let mut step = 1_usize;
        loop {
            hashes.push(Hash32::from(best_chain[index]));
            if index == 0 {
                break;
            }
            let next_index = index.saturating_sub(step);
            if hashes.len() > 10 {
                step = step.saturating_mul(2);
            }
            index = next_index;
        }

        BlockLocator {
            block_hashes: hashes,
        }
    }

    pub fn headers_after_locator(
        &self,
        locator: &BlockLocator,
        stop_hash: BlockHash,
        limit: usize,
    ) -> Vec<BlockHeader> {
        if limit == 0 {
            return Vec::new();
        }

        let best_chain = self.best_chain_hashes();
        let index_by_hash: BTreeMap<_, _> = best_chain
            .iter()
            .copied()
            .enumerate()
            .map(|(index, hash)| (hash, index))
            .collect();

        let start_index = locator
            .block_hashes
            .iter()
            .find_map(|hash| index_by_hash.get(&BlockHash::from(*hash)).copied())
            .map_or(0, |index| index.saturating_add(1));

        let mut headers = Vec::new();
        for block_hash in best_chain.into_iter().skip(start_index).take(limit) {
            if let Some(entry) = self.entries.get(&block_hash) {
                headers.push(entry.header.clone());
                if block_hash == stop_hash {
                    break;
                }
            }
        }
        headers
    }

    fn best_chain_hashes(&self) -> Vec<BlockHash> {
        let Some(mut current) = self.best_tip else {
            return Vec::new();
        };

        let mut chain = Vec::new();
        for _ in 0..self.entries.len() {
            chain.push(current);
            let Some(parent) = self.parents.get(&current).copied().flatten() else {
                break;
            };
            current = parent;
        }
        chain.reverse();
        chain
    }

    fn update_best_tip(&mut self, candidate_hash: BlockHash) {
        let Some(candidate) = self.entries.get(&candidate_hash) else {
            return;
        };
        let replace = self
            .best_tip
            .and_then(|hash| self.entries.get(&hash))
            .is_none_or(|current| candidate.chain_work > current.chain_work);
        if replace {
            self.best_tip = Some(candidate_hash);
        }
    }
}

fn invalid_header(reason: &str) -> NetworkError {
    NetworkError::InvalidHeader {
        reject_reason: reason.to_owned(),
        maybe_debug_message: None,
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
#[path = "header_store/tests/phase165_tests.rs"]
mod phase165_tests;
