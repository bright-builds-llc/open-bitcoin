// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Bounded detached application records, never a parser for engine files.

use super::{LogicalInventory, StorageError, copy, inspection_error, reader::InspectedRecords};
#[cfg(unix)]
use std::os::unix::{
    ffi::{OsStrExt, OsStringExt},
    fs::MetadataExt,
};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

pub(super) const MAX_JOB: u64 = 16 * 1024;
#[cfg(test)]
pub(super) const MAX_RESULT: u64 = copy::MAX_BYTES + copy::MAX_ENTRIES as u64 * 24 + 64;
const MAGIC: &[u8; 8] = b"OBIR\x01\0\0\0";
const PRUNE_PROOF: &[u8; 8] = b"OBIP\x01\0\0\0";

mod history;

pub(super) enum WorkerJob {
    Generic(OwnedScratchJob),
    History(history::HistoryJob),
    PruneProof(OwnedScratchJob),
}

impl WorkerJob {
    pub(super) fn history(
        owned: OwnedScratchJob,
        selected: open_bitcoin_core::consensus::CanonicalChain,
    ) -> Self {
        Self::History(history::HistoryJob::new(owned, selected))
    }

    pub(super) fn owned(&self) -> &OwnedScratchJob {
        match self {
            Self::Generic(job) | Self::PruneProof(job) => job,
            Self::History(job) => job.owned(),
        }
    }

    pub(super) fn maybe_history_selection(
        &self,
    ) -> Option<open_bitcoin_core::consensus::CanonicalChain> {
        match self {
            Self::Generic(_) | Self::PruneProof(_) => None,
            Self::History(job) => Some(job.selected()),
        }
    }

    pub(super) fn encode(&self) -> Result<Vec<u8>, StorageError> {
        match self {
            Self::Generic(job) => job.encode(),
            Self::History(job) => job.encode(),
            Self::PruneProof(job) => {
                let mut bytes = PRUNE_PROOF.to_vec();
                bytes.extend(job.encode()?);
                if bytes.len() as u64 > MAX_JOB {
                    return Err(refused());
                }
                Ok(bytes)
            }
        }
    }

    pub(super) fn decode(input: impl Read) -> Result<Self, StorageError> {
        let mut bytes = Vec::new();
        io(input.take(MAX_JOB + 1).read_to_end(&mut bytes))?;
        if bytes.len() as u64 > MAX_JOB {
            return Err(refused());
        }
        if bytes.starts_with(MAGIC) {
            return OwnedScratchJob::decode(bytes.as_slice()).map(Self::Generic);
        }
        if let Some(body) = bytes.strip_prefix(PRUNE_PROOF.as_slice()) {
            return OwnedScratchJob::decode(body).map(Self::PruneProof);
        }
        history::HistoryJob::decode(&bytes).map(Self::History)
    }

    pub(super) fn write_result(
        &self,
        output: &mut impl Write,
        records: &InspectedRecords,
    ) -> Result<(), StorageError> {
        match self {
            Self::Generic(job) => write_result(output, job, records),
            Self::History(job) => history::write_result(output, job, records),
            Self::PruneProof(job) => write_inventory(output, job, records, self.inventory()),
        }
    }

    pub(super) fn read_result(
        &self,
        input: impl Read,
        length: u64,
    ) -> Result<InspectedRecords, StorageError> {
        match self {
            Self::Generic(job) => read_result(input, length, job),
            Self::History(job) => history::read_result(input, length, job),
            Self::PruneProof(job) => read_inventory(input, length, job, self.inventory()),
        }
    }

    pub(super) fn inventory(&self) -> LogicalInventory {
        match self {
            Self::PruneProof(_) => LogicalInventory::RegtestPruneProof,
            Self::History(job) => LogicalInventory::for_history(job.selected()),
            Self::Generic(_) => LogicalInventory::Ordinary,
        }
    }

    pub(super) fn result_file(&self, write: bool) -> Result<File, StorageError> {
        self.owned()
            .result_file_with_limit(write, result_limit(self.inventory()))
    }
}

fn result_limit(inventory: LogicalInventory) -> u64 {
    copy::MAX_BYTES + inventory.entry_limit() as u64 * 24 + 64
}

pub(super) struct OwnedScratchJob {
    pub(super) root: PathBuf,
    pub(super) device: u64,
    pub(super) inode: u64,
    pub(super) result_device: u64,
    pub(super) result_inode: u64,
    pub(super) nonce: [u8; 16],
    #[cfg(test)]
    pub(super) fault: u8,
}

fn refused() -> StorageError {
    inspection_error("inspection worker protocol refused")
}
fn io<T>(value: std::io::Result<T>) -> Result<T, StorageError> {
    value.map_err(|_| refused())
}
fn number(output: &mut impl Write, value: u64) -> Result<(), StorageError> {
    io(output.write_all(&value.to_le_bytes()))
}

impl OwnedScratchJob {
    pub(super) fn encode(&self) -> Result<Vec<u8>, StorageError> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&self.nonce);
        for value in [
            self.device,
            self.inode,
            self.result_device,
            self.result_inode,
        ] {
            number(&mut bytes, value)?;
        }
        #[cfg(unix)]
        let root = self.root.as_os_str().as_bytes();
        #[cfg(not(unix))]
        let root = self.root.as_os_str().as_encoded_bytes();
        number(&mut bytes, root.len() as u64)?;
        bytes.extend_from_slice(root);
        #[cfg(test)]
        bytes.push(self.fault);
        if bytes.len() as u64 > MAX_JOB {
            return Err(refused());
        }
        Ok(bytes)
    }

    pub(super) fn decode(input: impl Read) -> Result<Self, StorageError> {
        let mut bytes = Vec::new();
        io(input.take(MAX_JOB + 1).read_to_end(&mut bytes))?;
        if bytes.len() as u64 > MAX_JOB {
            return Err(refused());
        }
        let mut cursor = Decoder::new(bytes.as_slice(), bytes.len() as u64);
        cursor.magic()?;
        let nonce = cursor.nonce()?;
        let device = cursor.number()?;
        let inode = cursor.number()?;
        let result_device = cursor.number()?;
        let result_inode = cursor.number()?;
        let length = cursor.number()?;
        let raw = cursor.bytes(length)?;
        #[cfg(unix)]
        let root = PathBuf::from(std::ffi::OsString::from_vec(raw));
        #[cfg(not(unix))]
        let root = PathBuf::from(String::from_utf8(raw).map_err(|_| refused())?);
        #[cfg(test)]
        let fault = cursor.bytes(1)?[0];
        cursor.finish()?;
        Ok(Self {
            root,
            device,
            inode,
            result_device,
            result_inode,
            nonce,
            #[cfg(test)]
            fault,
        })
    }

    pub(super) fn check_root(&self) -> Result<(), StorageError> {
        let parent = io(fs::canonicalize("/tmp"))?;
        let name = self
            .root
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(refused)?;
        let suffix = name
            .strip_prefix("open-bitcoin-inspection-")
            .ok_or_else(refused)?;
        if self.root.parent() != Some(parent.as_path())
            || suffix.len() != 32
            || !suffix.bytes().all(|byte| byte.is_ascii_hexdigit())
            || io(fs::canonicalize(&self.root))? != self.root
        {
            return Err(refused());
        }
        check_metadata(
            &io(fs::symlink_metadata(&self.root))?,
            true,
            self.device,
            self.inode,
        )
    }

    #[cfg(test)]
    pub(super) fn result_file(&self, write: bool) -> Result<File, StorageError> {
        self.result_file_with_limit(write, MAX_RESULT)
    }

    fn result_file_with_limit(&self, write: bool, limit: u64) -> Result<File, StorageError> {
        self.check_root()?;
        let path = self.root.join("result");
        let selected = io(fs::symlink_metadata(&path))?;
        check_metadata(&selected, false, self.result_device, self.result_inode)?;
        if selected.len() > limit {
            return Err(refused());
        }
        let file = io(OpenOptions::new().read(true).write(write).open(&path))?;
        check_metadata(
            &io(file.metadata())?,
            false,
            self.result_device,
            self.result_inode,
        )?;
        check_metadata(
            &io(fs::symlink_metadata(path))?,
            false,
            self.result_device,
            self.result_inode,
        )?;
        Ok(file)
    }
}

pub(super) fn check_metadata(
    metadata: &fs::Metadata,
    directory: bool,
    device: u64,
    inode: u64,
) -> Result<(), StorageError> {
    if metadata.file_type().is_symlink()
        || (if directory {
            !metadata.is_dir()
        } else {
            !metadata.is_file()
        })
    {
        return Err(refused());
    }
    #[cfg(unix)]
    if metadata.dev() != device
        || metadata.ino() != inode
        || metadata.mode() & 0o7777 != (if directory { 0o700 } else { 0o600 })
        || (!directory && metadata.nlink() != 1)
    {
        return Err(refused());
    }
    #[cfg(not(unix))]
    {
        let _ = (device, inode);
        return Err(refused());
    }
    Ok(())
}

pub(super) fn write_result(
    output: &mut impl Write,
    job: &OwnedScratchJob,
    records: &InspectedRecords,
) -> Result<(), StorageError> {
    write_inventory(output, job, records, LogicalInventory::Ordinary)
}

fn write_inventory(
    output: &mut impl Write,
    job: &OwnedScratchJob,
    records: &InspectedRecords,
    inventory: LogicalInventory,
) -> Result<(), StorageError> {
    let mut budget = copy::Budget::default();
    let limit = inventory.entry_limit();
    io(output.write_all(MAGIC))?;
    io(output.write_all(&job.nonce))?;
    number(output, job.device)?;
    number(output, job.inode)?;
    number(output, records.namespaces.len() as u64)?;
    for (name, rows) in &records.namespaces {
        budget.add_with_entry_limit(name.len() as u64, 0, limit)?;
        number(output, name.len() as u64)?;
        io(output.write_all(name.as_bytes()))?;
        number(output, rows.len() as u64)?;
        for (key, value) in rows {
            let size = (key.len() as u64)
                .checked_add(value.len() as u64)
                .ok_or_else(refused)?;
            budget.add_with_entry_limit(size, 0, limit)?;
            number(output, key.len() as u64)?;
            number(output, value.len() as u64)?;
            io(output.write_all(key))?;
            io(output.write_all(value))?;
        }
    }
    io(output.flush())
}

pub(super) fn read_result(
    input: impl Read,
    length: u64,
    job: &OwnedScratchJob,
) -> Result<InspectedRecords, StorageError> {
    read_inventory(input, length, job, LogicalInventory::Ordinary)
}

fn read_inventory(
    input: impl Read,
    length: u64,
    job: &OwnedScratchJob,
    inventory: LogicalInventory,
) -> Result<InspectedRecords, StorageError> {
    if length > result_limit(inventory) {
        return Err(refused());
    }
    let mut decoder = Decoder::new(input, length);
    decoder.magic()?;
    if decoder.nonce()? != job.nonce
        || decoder.number()? != job.device
        || decoder.number()? != job.inode
    {
        return Err(refused());
    }
    let limit = inventory.entry_limit();
    let count = decoder.count(limit)?;
    let mut namespaces = BTreeMap::new();
    let mut budget = copy::Budget::default();
    for _ in 0..count {
        let length = decoder.number()?;
        budget.add_with_entry_limit(length, 0, limit)?;
        let name = String::from_utf8(decoder.bytes(length)?).map_err(|_| refused())?;
        let count = decoder.count(limit)?;
        let mut rows = BTreeMap::new();
        for _ in 0..count {
            let key_length = decoder.number()?;
            let value_length = decoder.number()?;
            budget.add_with_entry_limit(
                key_length.checked_add(value_length).ok_or_else(refused)?,
                0,
                limit,
            )?;
            let key = decoder.bytes(key_length)?;
            let value = decoder.bytes(value_length)?;
            if rows.insert(key, value).is_some() {
                return Err(refused());
            }
        }
        if namespaces.insert(name, rows).is_some() {
            return Err(refused());
        }
    }
    decoder.finish()?;
    Ok(InspectedRecords { namespaces })
}

struct Decoder<R> {
    input: R,
    remaining: u64,
}
impl<R: Read> Decoder<R> {
    fn new(input: R, remaining: u64) -> Self {
        Self { input, remaining }
    }
    fn fixed<const N: usize>(&mut self) -> Result<[u8; N], StorageError> {
        self.remaining = self.remaining.checked_sub(N as u64).ok_or_else(refused)?;
        let mut bytes = [0; N];
        io(self.input.read_exact(&mut bytes))?;
        Ok(bytes)
    }
    fn magic(&mut self) -> Result<(), StorageError> {
        if &self.fixed::<8>()? != MAGIC {
            return Err(refused());
        }
        Ok(())
    }
    fn nonce(&mut self) -> Result<[u8; 16], StorageError> {
        self.fixed()
    }
    fn number(&mut self) -> Result<u64, StorageError> {
        Ok(u64::from_le_bytes(self.fixed()?))
    }
    fn count(&mut self, limit: usize) -> Result<usize, StorageError> {
        let count = self.number()?;
        if count > limit as u64 {
            return Err(refused());
        }
        usize::try_from(count).map_err(|_| refused())
    }
    fn bytes(&mut self, length: u64) -> Result<Vec<u8>, StorageError> {
        if length > copy::MAX_BYTES || length > self.remaining {
            return Err(refused());
        }
        self.remaining -= length;
        let size = usize::try_from(length).map_err(|_| refused())?;
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(size).map_err(|_| refused())?;
        bytes.resize(size, 0);
        io(self.input.read_exact(&mut bytes))?;
        Ok(bytes)
    }
    fn finish(mut self) -> Result<(), StorageError> {
        let mut tail = [0];
        if self.remaining != 0 || io(self.input.read(&mut tail))? != 0 {
            return Err(refused());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
