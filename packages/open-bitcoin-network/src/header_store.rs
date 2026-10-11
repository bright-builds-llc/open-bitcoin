// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use std::collections::BTreeMap;

use open_bitcoin_chainstate::ChainPosition;
use open_bitcoin_consensus::block_hash;
use open_bitcoin_primitives::{BlockHash, BlockHeader, BlockLocator, Hash32};

use crate::error::NetworkError;

mod proof;
pub use proof::HeaderProofFacts;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderEntry {
    pub block_hash: BlockHash,
    pub header: BlockHeader,
    pub height: u32,
    pub chain_work: u128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InsertedHeader {
    pub block_hash: BlockHash,
    pub height: u32,
    pub chain_work: u128,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HeaderStore {
    entries: BTreeMap<BlockHash, HeaderEntry>,
    parents: BTreeMap<BlockHash, Option<BlockHash>>,
    best_tip: Option<BlockHash>,
    proof_facts: BTreeMap<BlockHash, HeaderProofFacts>,
    maybe_best_proof: Option<BlockHash>,
}

impl HeaderStore {
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
            store.update_best_tip(block_hash);
        }
        store.rebuild_proof_facts();

        Ok(store)
    }

    pub fn entries(&self) -> impl Iterator<Item = &HeaderEntry> {
        self.entries.values()
    }

    pub fn seed_from_chain(&mut self, active_chain: &[ChainPosition]) {
        self.entries.clear();
        self.parents.clear();
        self.best_tip = None;
        self.proof_facts.clear();
        self.maybe_best_proof = None;

        for position in active_chain {
            self.record_position(position);
        }
    }

    pub fn record_position(&mut self, position: &ChainPosition) {
        let block_hash = position.block_hash;
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
        self.update_best_tip(block_hash);
        self.record_proof_facts(block_hash);
    }

    pub fn insert_header(&mut self, header: BlockHeader) -> Result<InsertedHeader, NetworkError> {
        let block_hash = block_hash(&header);
        if let Some(existing) = self.entries.get(&block_hash) {
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
            (
                parent.height.saturating_add(1),
                parent.chain_work.saturating_add(header_work_score(&header)),
            )
        } else {
            (0, header_work_score(&header))
        };

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
        self.update_best_tip(block_hash);
        self.record_proof_facts(block_hash);

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

        loop {
            if current.height == target_height {
                return Some(current);
            }
            let parent_hash = self.parents.get(&current.block_hash).copied().flatten()?;
            current = self.entries.get(&parent_hash)?;
        }
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
        loop {
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
            .is_none_or(|current| {
                if candidate.chain_work != current.chain_work {
                    return candidate.chain_work > current.chain_work;
                }
                if candidate.height != current.height {
                    return candidate.height > current.height;
                }
                candidate.block_hash > current.block_hash
            });
        if replace {
            self.best_tip = Some(candidate_hash);
        }
    }
}

fn header_work_score(_header: &BlockHeader) -> u128 {
    1
}

#[cfg(test)]
mod tests;
