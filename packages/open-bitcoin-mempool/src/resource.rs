// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/test/functional/mempool_limit.py

//! Deterministic mempool resource accounting.
//!
//! Version 1 counts only logical state owned by the Rust mempool:
//! - each entry-map [`Txid`] key and fixed [`MempoolEntry`] value;
//! - fixed transaction input and output elements;
//! - scriptSig, scriptPubKey, and witness payload bytes;
//! - one Rust `Vec<u8>` header for every witness item;
//! - direct parent and child [`Txid`] identities; and
//! - each spent-outpoint [`OutPoint`] key and [`Txid`] value.
//!
//! Allocator capacity and slack, hash-table buckets, C++ pointer estimates, and
//! network or node caches are intentionally excluded.
//!
//! [`accounted_memory_for_entry`] accounts one entry,
//! [`build_resource_ledger`] builds the cache through checked ledger mutations,
//! and [`recompute_resource_ledger`] is the independent full-state oracle.

use std::collections::HashMap;
use std::mem::size_of;

use open_bitcoin_primitives::{OutPoint, TransactionInput, TransactionOutput, Txid};

use crate::MempoolEntry;

/// Version of the deterministic Rust-owned mempool accounting formula.
pub const MEMPOOL_RESOURCE_ACCOUNTING_VERSION: u32 = 1;

/// Failure to represent a resource-accounting component or aggregate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceAccountingError {
    /// Adding or multiplying the named component exceeded `usize`.
    Overflow { component: &'static str },
}

impl core::fmt::Display for ResourceAccountingError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Overflow { component } => {
                write!(
                    formatter,
                    "mempool resource accounting overflow: {component}"
                )
            }
        }
    }
}

impl std::error::Error for ResourceAccountingError {}

/// A transaction's BIP141 virtual size in virtual bytes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionVirtualSize(usize);

impl TransactionVirtualSize {
    /// Zero virtual bytes.
    pub const ZERO: Self = Self(0);

    /// Creates a virtual-size value from virtual bytes.
    pub const fn new(virtual_bytes: usize) -> Self {
        Self(virtual_bytes)
    }

    /// Returns this value as virtual bytes.
    pub const fn as_usize(self) -> usize {
        self.0
    }

    /// Adds two virtual sizes without saturating or wrapping.
    pub fn checked_add(
        self,
        other: Self,
        component: &'static str,
    ) -> Result<Self, ResourceAccountingError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(ResourceAccountingError::Overflow { component })
    }
}

/// Memory bytes attributed to logical state owned by the Rust mempool.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountedMempoolMemory(usize);

impl AccountedMempoolMemory {
    /// Zero accounted bytes.
    pub const ZERO: Self = Self(0);

    /// Creates an accounted-memory value from bytes.
    pub const fn new(bytes: usize) -> Self {
        Self(bytes)
    }

    /// Returns the accounted byte count.
    pub const fn as_usize(self) -> usize {
        self.0
    }

    /// Adds two accounted-memory values without saturating or wrapping.
    pub fn checked_add(
        self,
        other: Self,
        component: &'static str,
    ) -> Result<Self, ResourceAccountingError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(ResourceAccountingError::Overflow { component })
    }
}

/// Configured capacity for accounted mempool memory in bytes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct MempoolCapacity(usize);

impl MempoolCapacity {
    /// Zero configured capacity.
    pub const ZERO: Self = Self(0);

    /// Creates a capacity value from bytes.
    pub const fn new(bytes: usize) -> Self {
        Self(bytes)
    }

    /// Returns the configured byte count.
    pub const fn as_usize(self) -> usize {
        self.0
    }

    /// Adds two capacity values without saturating or wrapping.
    pub fn checked_add(
        self,
        other: Self,
        component: &'static str,
    ) -> Result<Self, ResourceAccountingError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(ResourceAccountingError::Overflow { component })
    }
}

/// Cached totals with checked entry and spent-outpoint mutation methods.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MempoolResourceLedger {
    total_virtual_size: TransactionVirtualSize,
    accounted_memory: AccountedMempoolMemory,
}

impl MempoolResourceLedger {
    /// Empty resource totals.
    pub const ZERO: Self = Self {
        total_virtual_size: TransactionVirtualSize::ZERO,
        accounted_memory: AccountedMempoolMemory::ZERO,
    };

    /// Creates a ledger from independently computed totals.
    pub const fn new(
        total_virtual_size: TransactionVirtualSize,
        accounted_memory: AccountedMempoolMemory,
    ) -> Self {
        Self {
            total_virtual_size,
            accounted_memory,
        }
    }

    /// Returns total transaction virtual size.
    pub const fn total_virtual_size(self) -> TransactionVirtualSize {
        self.total_virtual_size
    }

    /// Returns total accounted mempool memory.
    pub const fn accounted_memory(self) -> AccountedMempoolMemory {
        self.accounted_memory
    }

    /// Adds one canonical entry to a prospective ledger.
    pub fn checked_add_entry(
        &mut self,
        entry: &MempoolEntry,
    ) -> Result<(), ResourceAccountingError> {
        let vsize_component = "total transaction virtual size";
        let memory_component = "total entry accounted memory";
        let virtual_size = TransactionVirtualSize::new(entry.virtual_size);
        self.total_virtual_size = self
            .total_virtual_size
            .checked_add(virtual_size, vsize_component)?;
        let entry_memory = accounted_memory_for_entry(entry)?;
        let accounted_memory = self
            .accounted_memory
            .checked_add(entry_memory, memory_component)?;
        self.accounted_memory = accounted_memory;
        Ok(())
    }
    pub fn checked_add_spent_outpoints(
        &mut self,
        count: usize,
    ) -> Result<(), ResourceAccountingError> {
        let component = "total spent-outpoint accounted memory";
        let element_bytes = spent_outpoint_accounted_bytes()?;
        let spent_bytes = checked_product(count, element_bytes, component)?;
        let spent_memory = AccountedMempoolMemory::new(spent_bytes);
        let accounted_memory = self.accounted_memory.checked_add(spent_memory, component)?;
        self.accounted_memory = accounted_memory;
        Ok(())
    }
}
pub fn accounted_memory_for_entry(
    entry: &MempoolEntry,
) -> Result<AccountedMempoolMemory, ResourceAccountingError> {
    let mut bytes = 0_usize;
    let input_component = "transaction input elements";
    let output_component = "transaction output elements";
    let witness_component = "witness item headers";
    let parent_component = "parent identities";
    let child_component = "child identities";
    bytes = checked_sum(bytes, size_of::<Txid>(), "entry-map key")?;
    bytes = checked_sum(bytes, size_of::<MempoolEntry>(), "fixed mempool entry")?;
    let input_count = entry.transaction.inputs.len();
    let input_bytes = checked_product(input_count, size_of::<TransactionInput>(), input_component)?;
    bytes = checked_sum(bytes, input_bytes, input_component)?;
    let output_count = entry.transaction.outputs.len();
    let output_element_bytes = size_of::<TransactionOutput>();
    let output_bytes = checked_product(output_count, output_element_bytes, output_component)?;
    bytes = checked_sum(bytes, output_bytes, output_component)?;
    let script_sig_lengths = entry
        .transaction
        .inputs
        .iter()
        .map(|input| input.script_sig.as_bytes().len());
    let script_sig_bytes = checked_values(script_sig_lengths, "scriptSig payload")?;
    bytes = checked_sum(bytes, script_sig_bytes, "scriptSig payload")?;
    let witness_item_counts = entry
        .transaction
        .inputs
        .iter()
        .map(|input| input.witness.stack().len());
    let witness_count = checked_values(witness_item_counts, witness_component)?;
    let witness_headers = checked_product(witness_count, size_of::<Vec<u8>>(), witness_component)?;
    bytes = checked_sum(bytes, witness_headers, witness_component)?;
    let witness_lengths = entry
        .transaction
        .inputs
        .iter()
        .flat_map(|input| input.witness.stack())
        .map(Vec::len);
    let witness_bytes = checked_values(witness_lengths, "witness payload")?;
    bytes = checked_sum(bytes, witness_bytes, "witness payload")?;
    let script_pubkey_lengths = entry
        .transaction
        .outputs
        .iter()
        .map(|output| output.script_pubkey.as_bytes().len());
    let script_pubkey_bytes = checked_values(script_pubkey_lengths, "scriptPubKey payload")?;
    bytes = checked_sum(bytes, script_pubkey_bytes, "scriptPubKey payload")?;
    let parent_bytes = checked_product(entry.parents.len(), size_of::<Txid>(), parent_component)?;
    bytes = checked_sum(bytes, parent_bytes, parent_component)?;
    let child_bytes = checked_product(entry.children.len(), size_of::<Txid>(), child_component)?;
    checked_sum(bytes, child_bytes, child_component).map(AccountedMempoolMemory::new)
}
pub fn build_resource_ledger(
    entries: &HashMap<Txid, MempoolEntry>,
    spent_outpoints: &HashMap<OutPoint, Txid>,
) -> Result<MempoolResourceLedger, ResourceAccountingError> {
    let mut ledger = entries
        .values()
        .try_fold(MempoolResourceLedger::ZERO, checked_ledger_entry)?;
    ledger
        .checked_add_spent_outpoints(spent_outpoints.len())
        .map(|()| ledger)
}
pub fn recompute_resource_ledger(
    entries: &HashMap<Txid, MempoolEntry>,
    spent_outpoints: &HashMap<OutPoint, Txid>,
) -> Result<MempoolResourceLedger, ResourceAccountingError> {
    let initial = (TransactionVirtualSize::ZERO, AccountedMempoolMemory::ZERO);
    let (total_virtual_size, accounted_memory) =
        entries.values().try_fold(initial, checked_oracle_entry)?;
    let spent_component = "oracle spent-outpoint index";
    let spent_element_bytes = spent_outpoint_accounted_bytes()?;
    let spent_bytes = checked_product(spent_outpoints.len(), spent_element_bytes, spent_component)?;
    let spent_memory = AccountedMempoolMemory::new(spent_bytes);
    let memory_component = "oracle total spent-outpoint accounted memory";
    accounted_memory
        .checked_add(spent_memory, memory_component)
        .map(|memory| MempoolResourceLedger::new(total_virtual_size, memory))
}
fn spent_outpoint_accounted_bytes() -> Result<usize, ResourceAccountingError> {
    let outpoint_bytes = size_of::<OutPoint>();
    let txid_bytes = size_of::<Txid>();
    checked_sum(outpoint_bytes, txid_bytes, "spent-outpoint key/value")
}
fn checked_ledger_entry(
    mut ledger: MempoolResourceLedger,
    entry: &MempoolEntry,
) -> Result<MempoolResourceLedger, ResourceAccountingError> {
    ledger.checked_add_entry(entry).map(|()| ledger)
}
fn checked_oracle_entry(
    totals: (TransactionVirtualSize, AccountedMempoolMemory),
    entry: &MempoolEntry,
) -> Result<(TransactionVirtualSize, AccountedMempoolMemory), ResourceAccountingError> {
    let virtual_size = TransactionVirtualSize::new(entry.virtual_size);
    let total_virtual_size = totals
        .0
        .checked_add(virtual_size, "oracle total transaction virtual size")?;
    let entry_memory = accounted_memory_for_entry(entry)?;
    totals
        .1
        .checked_add(entry_memory, "oracle total entry accounted memory")
        .map(|memory| (total_virtual_size, memory))
}
pub(crate) fn checked_sum(
    total: usize,
    value: usize,
    component: &'static str,
) -> Result<usize, ResourceAccountingError> {
    total
        .checked_add(value)
        .ok_or(ResourceAccountingError::Overflow { component })
}
pub(crate) fn checked_product(
    count: usize,
    element_size: usize,
    component: &'static str,
) -> Result<usize, ResourceAccountingError> {
    count
        .checked_mul(element_size)
        .ok_or(ResourceAccountingError::Overflow { component })
}
fn checked_values(
    mut values: impl Iterator<Item = usize>,
    component: &'static str,
) -> Result<usize, ResourceAccountingError> {
    values.try_fold(0, |total, value| checked_sum(total, value, component))
}
