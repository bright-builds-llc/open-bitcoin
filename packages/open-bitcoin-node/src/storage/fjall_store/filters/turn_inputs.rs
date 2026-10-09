// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Borrow the native row and bound all wire counts before the allocating decoder.

use super::*;
use open_bitcoin_core::{chainstate::filter_index::catch_up::TurnWork, primitives::Block};
mod undo;

pub(crate) enum BasicFilterReusableRead {
    Missing,
    Deferred,
    Ready(codec::StoredFilterRecord),
}

impl BasicFilterReusableRead {
    pub(crate) fn is_deferred(&self) -> bool {
        matches!(self, Self::Deferred)
    }
    pub(crate) fn maybe_record(self) -> Option<codec::StoredFilterRecord> {
        match self {
            Self::Ready(record) => Some(record),
            Self::Missing | Self::Deferred => None,
        }
    }
}

impl FjallNodeStore {
    /// Only a bounded exact immutable row can replace filter-generation inputs.
    pub(crate) fn maybe_basic_filter_reusable_record(
        &self,
        position: &open_bitcoin_core::chainstate::ChainPosition,
        work: &mut TurnWork,
        maximum: TurnWork,
        admit: impl FnOnce(&mut TurnWork, TurnWork) -> Result<Option<TurnWork>, StorageError>,
    ) -> Result<BasicFilterReusableRead, StorageError> {
        super::append::charge_append_work(
            work,
            TurnWork {
                record_operations: 1,
                ..Default::default()
            },
            Some(maximum),
        )?;
        let key = codec::record_key(position.block_hash);
        #[cfg(test)]
        self.count_filter_integrity_read();
        let Some(bytes) = self
            .block_index
            .get(&key)
            .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))?
        else {
            return Ok(BasicFilterReusableRead::Missing);
        };
        let Some(maximum) = admit(
            work,
            TurnWork {
                encoded_bytes: bytes.len() as u64,
                cloned_bytes: (bytes.len() as u64)
                    .checked_mul(2)
                    .ok_or_else(|| index_corruption("BASIC reusable row allocation overflow"))?,
                record_operations: 3,
                ..Default::default()
            },
        )?
        else {
            return Ok(BasicFilterReusableRead::Deferred);
        };
        let parsed = codec::parse_record_fields(&key, bytes.as_ref())?;
        if parsed.height != position.height || parsed.parent != position.previous_block_hash() {
            return Err(index_corruption(
                "BASIC immutable row differs from accepted position",
            ));
        }
        let maybe_parent = if position.height == 0 {
            None
        } else {
            Some(self.read_basic_filter_local_identity(
                position.previous_block_hash(),
                work,
                Some(maximum),
            )?)
        };
        codec::decode_record(&key, bytes.as_ref(), maybe_parent.as_ref())
            .map(BasicFilterReusableRead::Ready)
    }

    #[cfg(test)]
    pub(crate) fn delete_basic_input_for_test(
        &self,
        hash: BlockHash,
        undo: bool,
    ) -> Result<(), StorageError> {
        let (namespace, key) = if undo {
            (
                StorageNamespace::Chainstate,
                super::super::coins::undo_key(hash),
            )
        } else {
            (StorageNamespace::BlockIndex, super::super::block_key(hash))
        };
        self.remove_bytes(namespace, &key, crate::storage::PersistMode::Sync)
    }
}

impl FjallNodeStore {
    pub(crate) fn maybe_basic_filter_turn_body(
        &self,
        hash: BlockHash,
        remaining_body_bytes: u64,
        first_candidate: bool,
        admit: impl FnOnce(TurnWork) -> Result<bool, StorageError>,
    ) -> Result<Option<(Block, TurnWork)>, StorageError> {
        let bytes = self
            .block_index
            .get(super::super::block_key(hash))
            .map_err(|error| backend_failure(StorageNamespace::BlockIndex, error))?
            .ok_or_else(|| index_corruption("missing BASIC required body"))?;
        if bytes.len() > 4_000_000 {
            return Err(index_corruption("BASIC body absolute wire bound"));
        }
        // Native length probe precedes scanning or any decoded allocation.
        if bytes.len() as u64 > remaining_body_bytes {
            if first_candidate {
                return Err(index_corruption("BASIC singleton absolute body bound"));
            }
            return Ok(None);
        }
        let work = TurnWork {
            body_bytes: bytes.len() as u64,
            cloned_bytes: bytes.len() as u64 * 32 + std::mem::size_of::<Block>() as u64,
            script_items: bytes.len() as u64 / 9,
            script_bytes: bytes.len() as u64,
            record_operations: 0,
            ..TurnWork::default()
        };
        // This conservative wire-length reservation bounds the scanner itself,
        // every decoded container and every generation before any count loop.
        if !admit(work)? {
            return Ok(None);
        }
        let observed = body_work(bytes.as_ref())?;
        open_bitcoin_core::codec::parse_block(bytes.as_ref())
            .map(|block| Some((block, observed)))
            .map_err(index_corruption)
    }
}

fn body_work(bytes: &[u8]) -> Result<TurnWork, StorageError> {
    let mut cursor = Cursor { rest: bytes };
    cursor.take(80)?;
    let transactions = cursor.count(10)?;
    let mut work = TurnWork {
        body_bytes: bytes.len() as u64,
        // Each container element consumes at least one wire byte; 32 covers
        // witness Vec slots (24), transaction/input/output structs and scripts.
        cloned_bytes: bytes.len() as u64 * 32 + std::mem::size_of::<Block>() as u64,
        record_operations: 1,
        ..TurnWork::default()
    };
    for _ in 0..transactions {
        cursor.take(4)?;
        let mut inputs = cursor.count(41)?;
        let mut flags = 0;
        if inputs == 0 {
            flags = cursor.take(1)?[0];
            if flags != 0 {
                inputs = cursor.count(41)?;
            }
        }
        for _ in 0..inputs {
            cursor.take(36)?;
            cursor.script()?;
            cursor.take(4)?;
        }
        let outputs = if flags == 0 && inputs == 0 {
            0
        } else {
            cursor.count(9)?
        };
        for _ in 0..outputs {
            cursor.take(8)?;
            let script = cursor.script()?;
            work.script_items += 1;
            work.script_bytes += script.len() as u64;
        }
        if flags & 1 != 0 {
            for _ in 0..inputs {
                let items = cursor.count(1)?;
                for _ in 0..items {
                    let len = cursor.size()?;
                    cursor.take(len)?;
                }
            }
        }
        if flags & !1 != 0 {
            return Err(index_corruption("BASIC body witness flag"));
        }
        cursor.take(4)?;
    }
    if !cursor.rest.is_empty() {
        return Err(index_corruption("BASIC body trailing bytes"));
    }
    Ok(work)
}

struct Cursor<'a> {
    rest: &'a [u8],
}
impl<'a> Cursor<'a> {
    fn take(&mut self, len: usize) -> Result<&'a [u8], StorageError> {
        if len > self.rest.len() {
            return Err(index_corruption("BASIC body truncated"));
        }
        let (head, tail) = self.rest.split_at(len);
        self.rest = tail;
        Ok(head)
    }
    fn size(&mut self) -> Result<usize, StorageError> {
        let tag = self.take(1)?[0];
        let value = match tag {
            0xfd => {
                let n =
                    u16::from_le_bytes(self.take(2)?.try_into().map_err(index_corruption)?) as u64;
                if n < 253 {
                    return Err(index_corruption("BASIC body noncanonical count"));
                }
                n
            }
            0xfe => {
                let n =
                    u32::from_le_bytes(self.take(4)?.try_into().map_err(index_corruption)?) as u64;
                if n <= u16::MAX as u64 {
                    return Err(index_corruption("BASIC body noncanonical count"));
                }
                n
            }
            0xff => {
                let n = u64::from_le_bytes(self.take(8)?.try_into().map_err(index_corruption)?);
                if n <= u32::MAX as u64 {
                    return Err(index_corruption("BASIC body noncanonical count"));
                }
                n
            }
            n => u64::from(n),
        };
        if value > open_bitcoin_core::codec::MAX_SIZE {
            return Err(index_corruption("BASIC body count bound"));
        }
        usize::try_from(value).map_err(index_corruption)
    }
    fn count(&mut self, minimum: usize) -> Result<usize, StorageError> {
        let count = self.size()?;
        if count > self.rest.len() / minimum {
            return Err(index_corruption("BASIC body count exceeds envelope"));
        }
        Ok(count)
    }
    fn script(&mut self) -> Result<&'a [u8], StorageError> {
        let len = self.size()?;
        if len > 10_000 {
            return Err(index_corruption("BASIC body script bound"));
        }
        self.take(len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn phase157_turn_malformed_transaction_count_refuses_before_decoder() {
        // Arrange
        let mut bytes = vec![0; 80];
        bytes.extend([0xfe, 0xff, 0xff, 0xff, 0x01]);
        // Act
        let result = body_work(&bytes);
        // Assert
        assert!(
            result
                .expect_err("predecode refusal")
                .to_string()
                .contains("count exceeds envelope")
        );
    }
}
