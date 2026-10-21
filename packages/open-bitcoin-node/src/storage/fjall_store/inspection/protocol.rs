// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Bounded detached application records, never a parser for engine files.

use super::{StorageError, copy, inspection_error, reader::InspectedRecords};
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
pub(super) const MAX_RESULT: u64 = copy::MAX_BYTES + copy::MAX_ENTRIES as u64 * 24 + 64;
const MAGIC: &[u8; 8] = b"OBIR\x01\0\0\0";

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

    pub(super) fn result_file(&self, write: bool) -> Result<File, StorageError> {
        self.check_root()?;
        let path = self.root.join("result");
        let selected = io(fs::symlink_metadata(&path))?;
        check_metadata(&selected, false, self.result_device, self.result_inode)?;
        if selected.len() > MAX_RESULT {
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
    let mut budget = copy::Budget::default();
    io(output.write_all(MAGIC))?;
    io(output.write_all(&job.nonce))?;
    number(output, job.device)?;
    number(output, job.inode)?;
    number(output, records.namespaces.len() as u64)?;
    for (name, rows) in &records.namespaces {
        budget.add(name.len() as u64, 0)?;
        number(output, name.len() as u64)?;
        io(output.write_all(name.as_bytes()))?;
        number(output, rows.len() as u64)?;
        for (key, value) in rows {
            let size = (key.len() as u64)
                .checked_add(value.len() as u64)
                .ok_or_else(refused)?;
            budget.add(size, 0)?;
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
    if length > MAX_RESULT {
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
    let count = decoder.count()?;
    let mut namespaces = BTreeMap::new();
    let mut budget = copy::Budget::default();
    for _ in 0..count {
        let length = decoder.number()?;
        budget.add(length, 0)?;
        let name = String::from_utf8(decoder.bytes(length)?).map_err(|_| refused())?;
        let count = decoder.count()?;
        let mut rows = BTreeMap::new();
        for _ in 0..count {
            let key_length = decoder.number()?;
            let value_length = decoder.number()?;
            budget.add(key_length.checked_add(value_length).ok_or_else(refused)?, 0)?;
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
    fn count(&mut self) -> Result<usize, StorageError> {
        let count = self.number()?;
        if count > copy::MAX_ENTRIES as u64 {
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
mod tests {
    use super::*;
    fn job() -> OwnedScratchJob {
        OwnedScratchJob {
            root: PathBuf::from("/tmp/test"),
            device: 1,
            inode: 2,
            result_device: 3,
            result_inode: 4,
            nonce: [5; 16],
            fault: 0,
        }
    }
    fn records() -> InspectedRecords {
        InspectedRecords {
            namespaces: BTreeMap::from([
                ("empty".into(), BTreeMap::new()),
                (
                    "rows".into(),
                    BTreeMap::from([(vec![0, 255], vec![]), (vec![1], vec![0, 254])]),
                ),
            ]),
        }
    }
    fn encoded() -> Vec<u8> {
        let mut bytes = Vec::new();
        write_result(&mut bytes, &job(), &records()).expect("encode");
        bytes
    }
    #[test]
    fn phase166_inspection_protocol_exact_absence_and_binary_roundtrip() {
        // Arrange
        let bytes = encoded();
        // Act
        let decoded =
            read_result(bytes.as_slice(), bytes.len() as u64, &job()).expect("complete records");
        // Assert
        assert_eq!(decoded.namespaces, records().namespaces);
        assert!(decoded.maybe_records("absent").is_none());
        assert!(
            decoded
                .maybe_records("rows")
                .expect("rows")
                .get(&vec![2])
                .is_none()
        );
        let job_bytes = job().encode().expect("job");
        assert_eq!(
            OwnedScratchJob::decode(job_bytes.as_slice())
                .expect("job")
                .nonce,
            job().nonce
        );
    }
    #[test]
    fn phase166_inspection_protocol_rejects_truncation_version_status_trailing_and_receipt_mismatch()
     {
        // Arrange
        let bytes = encoded();
        // Act / Assert
        for end in 0..bytes.len() {
            assert!(read_result(&bytes[..end], end as u64, &job()).is_err());
        }
        for offset in [4, 5, 8, 24, 32] {
            let mut bad = bytes.clone();
            bad[offset] ^= 1;
            assert!(read_result(bad.as_slice(), bad.len() as u64, &job()).is_err());
        }
        let mut trailing = bytes;
        trailing.push(0);
        assert!(read_result(trailing.as_slice(), trailing.len() as u64, &job()).is_err());
    }
    #[test]
    fn phase166_inspection_protocol_finite_boundaries_and_overflow_before_allocation() {
        // Arrange
        assert_eq!(MAX_JOB, 16 * 1024);
        assert_eq!(copy::MAX_ENTRIES, 4096);
        assert_eq!(copy::MAX_BYTES, 512 * 1024 * 1024);
        assert_eq!(
            MAX_RESULT,
            copy::MAX_BYTES + copy::MAX_ENTRIES as u64 * 24 + 64
        );
        let mut budget = copy::Budget::default();
        // Act / Assert
        budget.add(copy::MAX_BYTES, 0).expect("payload boundary");
        assert!(budget.add(1, 0).is_err());
        let mut units = copy::Budget::default();
        for _ in 0..copy::MAX_ENTRIES {
            units.add(0, 0).expect("unit boundary");
        }
        assert!(units.add(0, 0).is_err());
        for length in [copy::MAX_BYTES + 1, u64::MAX] {
            assert!(Decoder::new([].as_slice(), u64::MAX).bytes(length).is_err());
        }
        assert!(read_result([].as_slice(), MAX_RESULT + 1, &job()).is_err());
        assert!(OwnedScratchJob::decode(vec![0; MAX_JOB as usize + 1].as_slice()).is_err());
        let base = job().encode().expect("job").len();
        let mut boundary = job();
        boundary.root = PathBuf::from("x".repeat(MAX_JOB as usize - base + 9));
        assert_eq!(
            boundary.encode().expect("exact job cap").len() as u64,
            MAX_JOB
        );
        boundary.root.push("y");
        assert!(boundary.encode().is_err());
    }
    #[test]
    fn phase166_inspection_protocol_duplicate_names_keys_and_oversized_counts_refuse() {
        // Arrange: namespace framing starts at byte 48.
        let mut bytes = Vec::new();
        write_result(
            &mut bytes,
            &job(),
            &InspectedRecords {
                namespaces: BTreeMap::from([("n".into(), BTreeMap::new())]),
            },
        )
        .expect("encode");
        let row = bytes[48..].to_vec();
        bytes[40..48].copy_from_slice(&2_u64.to_le_bytes());
        bytes.extend(row);
        // Act / Assert
        assert!(read_result(bytes.as_slice(), bytes.len() as u64, &job()).is_err());
        let mut bytes = encoded();
        bytes[40..48].copy_from_slice(&u64::MAX.to_le_bytes());
        assert!(read_result(bytes.as_slice(), bytes.len() as u64, &job()).is_err());
        let records = InspectedRecords {
            namespaces: BTreeMap::from([("n".into(), BTreeMap::from([(vec![1], vec![2])]))]),
        };
        let mut bytes = Vec::new();
        write_result(&mut bytes, &job(), &records).expect("encode");
        bytes[57..65].copy_from_slice(&2_u64.to_le_bytes());
        let row = bytes[65..].to_vec();
        bytes.extend(row);
        assert!(read_result(bytes.as_slice(), bytes.len() as u64, &job()).is_err());
    }

    #[test]
    fn phase166_inspection_protocol_job_rejects_every_truncation_version_and_trailing_byte() {
        // Arrange
        let bytes = job().encode().expect("bounded job");
        // Act / Assert
        for end in 0..bytes.len() {
            assert!(OwnedScratchJob::decode(&bytes[..end]).is_err());
        }
        let mut bad = bytes.clone();
        bad[4] = 2;
        assert!(OwnedScratchJob::decode(bad.as_slice()).is_err());
        let mut bad = bytes;
        bad.push(0);
        assert!(OwnedScratchJob::decode(bad.as_slice()).is_err());
    }

    #[test]
    fn phase166_inspection_protocol_counts_namespaces_and_rows_together_at_boundary() {
        // Arrange
        let rows = (0_u64..4095)
            .map(|key| (key.to_le_bytes().to_vec(), Vec::new()))
            .collect();
        let mut records = InspectedRecords {
            namespaces: BTreeMap::from([("n".into(), rows)]),
        };
        let mut bytes = Vec::new();
        // Act
        write_result(&mut bytes, &job(), &records).expect("4096 total units");
        let decoded =
            read_result(bytes.as_slice(), bytes.len() as u64, &job()).expect("exact unit boundary");
        // Assert
        assert_eq!(decoded.namespaces, records.namespaces);
        records
            .namespaces
            .get_mut("n")
            .expect("namespace")
            .insert(4095_u64.to_le_bytes().to_vec(), Vec::new());
        assert!(write_result(&mut Vec::new(), &job(), &records).is_err());
        bytes[57..65].copy_from_slice(&4096_u64.to_le_bytes());
        number(&mut bytes, 8).expect("key framing");
        number(&mut bytes, 0).expect("empty value framing");
        bytes.extend_from_slice(&4095_u64.to_le_bytes());
        assert!(read_result(bytes.as_slice(), bytes.len() as u64, &job()).is_err());
    }
}
