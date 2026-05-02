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

        Ok(store)
    }

    pub fn entries(&self) -> impl Iterator<Item = &HeaderEntry> {
        self.entries.values()
    }

    pub fn seed_from_chain(&mut self, active_chain: &[ChainPosition]) {
        self.entries.clear();
        self.parents.clear();
        self.best_tip = None;

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
mod tests {
    use open_bitcoin_chainstate::ChainPosition;
    use open_bitcoin_consensus::block_hash;
    use open_bitcoin_primitives::{BlockHash, BlockHeader, BlockLocator, Hash32, MerkleRoot};

    use crate::{HeaderStore, NetworkError};

    fn header(previous_block_hash: BlockHash, nonce: u32) -> BlockHeader {
        BlockHeader {
            version: 1,
            previous_block_hash,
            merkle_root: MerkleRoot::from_byte_array([nonce as u8; 32]),
            time: nonce,
            bits: 0x207f_ffff,
            nonce,
        }
    }

    #[test]
    fn header_store_builds_exponential_locators() {
        let mut store = HeaderStore::default();
        let mut previous = BlockHash::from_byte_array([0_u8; 32]);
        for height in 0..15_u32 {
            let current_header = header(previous, height + 1);
            let position =
                ChainPosition::new(current_header.clone(), height, u128::from(height + 1), 0);
            previous = position.block_hash;
            store.record_position(&position);
        }

        let locator = store.locator();
        assert_eq!(locator.block_hashes.len(), 14);
        assert_eq!(
            locator.block_hashes.first().copied(),
            Some(Hash32::from(previous)),
        );
        assert_eq!(
            locator.block_hashes.last().copied(),
            Some(locator.block_hashes[locator.block_hashes.len() - 1]),
        );
    }

    #[test]
    fn headers_after_locator_returns_following_headers() {
        let mut store = HeaderStore::default();
        let genesis_header = header(BlockHash::from_byte_array([0_u8; 32]), 1);
        let genesis = store
            .insert_header(genesis_header.clone())
            .expect("genesis");
        let child_header = header(genesis.block_hash, 2);
        let child = store.insert_header(child_header.clone()).expect("child");
        let descendant_header = header(child.block_hash, 3);
        store
            .insert_header(descendant_header.clone())
            .expect("descendant");

        let locator = open_bitcoin_primitives::BlockLocator {
            block_hashes: vec![genesis.block_hash.into()],
        };
        let headers =
            store.headers_after_locator(&locator, BlockHash::from_byte_array([0_u8; 32]), 2);

        assert_eq!(headers, vec![child_header, descendant_header]);
    }

    #[test]
    fn headers_after_locator_respects_stop_hash() {
        let mut store = HeaderStore::default();
        let genesis = store
            .insert_header(header(BlockHash::from_byte_array([0_u8; 32]), 1))
            .expect("genesis");
        let child_header = header(genesis.block_hash, 2);
        let child = store.insert_header(child_header.clone()).expect("child");
        let descendant_header = header(child.block_hash, 3);
        let descendant = store
            .insert_header(descendant_header.clone())
            .expect("descendant");

        let locator = BlockLocator {
            block_hashes: vec![genesis.block_hash.into()],
        };
        let headers = store.headers_after_locator(&locator, descendant.block_hash, 10);
        assert_eq!(headers, vec![child_header, descendant_header]);
    }

    #[test]
    fn empty_duplicate_and_missing_ancestor_paths_are_covered() {
        let mut store = HeaderStore::default();
        assert_eq!(store.best_height(), -1);
        assert!(store.best_tip().is_none());
        store.update_best_tip(BlockHash::from_byte_array([42_u8; 32]));
        assert!(store.best_tip().is_none());
        store.best_tip = Some(BlockHash::from_byte_array([43_u8; 32]));
        store
            .parents
            .insert(BlockHash::from_byte_array([43_u8; 32]), None);
        assert!(
            store
                .headers_after_locator(&BlockLocator::default(), BlockHash::default(), 1)
                .is_empty()
        );
        store.best_tip = None;
        store.parents.clear();
        assert!(store.locator().block_hashes.is_empty());
        assert!(
            store
                .headers_after_locator(&BlockLocator::default(), BlockHash::default(), 0)
                .is_empty()
        );
        assert!(!store.contains(&BlockHash::from_byte_array([1_u8; 32])));

        let missing_parent = BlockHash::from_byte_array([9_u8; 32]);
        assert_eq!(
            store.insert_header(header(missing_parent, 1)),
            Err(NetworkError::MissingHeaderAncestor(missing_parent)),
        );

        let genesis_header = header(BlockHash::from_byte_array([0_u8; 32]), 7);
        let inserted = store.insert_header(genesis_header.clone()).expect("insert");
        let duplicate = store.insert_header(genesis_header).expect("duplicate");
        assert_eq!(duplicate, inserted);
        assert!(store.contains(&inserted.block_hash));
        assert_eq!(
            store.best_tip().expect("tip").block_hash,
            inserted.block_hash
        );
    }

    #[test]
    fn seed_and_tie_break_paths_prefer_the_expected_tip() {
        let genesis_header = header(BlockHash::from_byte_array([0_u8; 32]), 1);
        let genesis_position = ChainPosition::new(genesis_header.clone(), 0, 1, 0);
        let child_a_header = header(genesis_position.block_hash, 2);
        let child_b_header = header(genesis_position.block_hash, 3);
        let child_a_position = ChainPosition::new(child_a_header.clone(), 1, 2, 0);
        let child_b_position = ChainPosition::new(child_b_header.clone(), 1, 2, 0);

        let mut store = HeaderStore::default();
        store.seed_from_chain(&[genesis_position.clone(), child_a_position.clone()]);
        store.record_position(&child_b_position);

        let expected_tip = child_a_position.block_hash.max(child_b_position.block_hash);
        assert_eq!(store.best_tip().expect("best tip").block_hash, expected_tip);
    }

    #[test]
    fn higher_height_wins_when_work_is_equal() {
        let genesis_header = header(BlockHash::from_byte_array([0_u8; 32]), 1);
        let genesis_position = ChainPosition::new(genesis_header, 0, 1, 0);
        let child_header = header(genesis_position.block_hash, 2);
        let descendant_header = header(block_hash(&child_header), 3);

        let mut store = HeaderStore::default();
        store.record_position(&genesis_position);
        store.record_position(&ChainPosition::new(child_header.clone(), 1, 2, 0));
        store.record_position(&ChainPosition::new(descendant_header.clone(), 2, 2, 0));

        assert_eq!(
            store.best_tip().expect("best tip").block_hash,
            block_hash(&descendant_header),
        );
    }

    #[test]
    fn header_store_rebuilds_from_persisted_entries() {
        // Arrange
        let genesis_header = header(BlockHash::from_byte_array([0_u8; 32]), 1);
        let genesis = ChainPosition::new(genesis_header.clone(), 0, 1, 0);
        let child_header = header(genesis.block_hash, 2);
        let child = ChainPosition::new(child_header, 1, 2, 0);
        let original_entries = [
            crate::HeaderEntry {
                block_hash: child.block_hash,
                header: child.header.clone(),
                height: child.height,
                chain_work: child.chain_work,
            },
            crate::HeaderEntry {
                block_hash: genesis.block_hash,
                header: genesis.header.clone(),
                height: genesis.height,
                chain_work: genesis.chain_work,
            },
        ];

        // Act
        let store = HeaderStore::from_entries(original_entries).expect("rebuild header store");
        let rebuilt_entries = store.entries().cloned().collect::<Vec<_>>();

        // Assert
        assert_eq!(store.best_height(), 1);
        assert_eq!(
            store.best_tip().expect("best tip").block_hash,
            child.block_hash
        );
        assert_eq!(rebuilt_entries.len(), 2);
        assert!(rebuilt_entries.iter().any(|entry| entry.height == 0));
        assert!(rebuilt_entries.iter().any(|entry| entry.height == 1));
    }

    #[test]
    fn header_store_rebuild_rejects_missing_parent() {
        // Arrange
        let missing_parent = BlockHash::from_byte_array([9_u8; 32]);
        let orphan_header = header(missing_parent, 2);
        let orphan_entry = crate::HeaderEntry {
            block_hash: block_hash(&orphan_header),
            header: orphan_header,
            height: 1,
            chain_work: 2,
        };

        // Act
        let error = HeaderStore::from_entries([orphan_entry]).expect_err("missing parent");

        // Assert
        assert_eq!(error, NetworkError::MissingHeaderAncestor(missing_parent));
    }

    #[test]
    fn entry_ancestor_and_median_time_helpers_cover_present_and_missing_paths() {
        // Arrange
        let mut store = HeaderStore::default();
        let mut previous = BlockHash::from_byte_array([0_u8; 32]);
        let mut hashes = Vec::new();
        for height in 0..12_u32 {
            let current_header = header(previous, height + 1);
            previous = block_hash(&current_header);
            hashes.push(previous);
            store
                .insert_header(current_header)
                .expect("header inserts into store");
        }

        // Act / Assert
        let tip_hash = *hashes.last().expect("tip hash");
        assert_eq!(store.entry(&tip_hash).expect("tip entry").height, 11);
        assert!(
            store
                .entry(&BlockHash::from_byte_array([99_u8; 32]))
                .is_none()
        );
        assert_eq!(
            store
                .ancestor_at_height(tip_hash, 5)
                .expect("ancestor")
                .height,
            5,
        );
        assert!(store.ancestor_at_height(tip_hash, 20).is_none());
        assert_eq!(store.median_time_past(tip_hash), Some(7));
        assert!(
            store
                .median_time_past(BlockHash::from_byte_array([100_u8; 32]))
                .is_none()
        );

        // Corrupt one parent edge to cover the helper's missing-parent fallback.
        let child_hash = hashes[2];
        store.parents.insert(child_hash, None);
        assert!(store.ancestor_at_height(tip_hash, 1).is_none());
    }
}
