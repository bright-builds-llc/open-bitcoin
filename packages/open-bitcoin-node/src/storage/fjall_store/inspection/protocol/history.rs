// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Opt-in full-history operation around the unchanged detached v1 frames.

use super::*;
use open_bitcoin_core::consensus::CanonicalChain;

const REQUEST: &[u8; 8] = b"OBIJ\x02\0\0\0";
const RESPONSE: &[u8; 8] = b"OBIR\x02\0\0\0";
const ENVELOPE: u64 = 10;
const MODE: u8 = 1;

pub(in crate::storage::fjall_store::inspection) struct HistoryJob {
    owned: OwnedScratchJob,
    selected: CanonicalChain,
}

impl HistoryJob {
    pub(super) fn new(owned: OwnedScratchJob, selected: CanonicalChain) -> Self {
        Self { owned, selected }
    }
    pub(super) fn owned(&self) -> &OwnedScratchJob {
        &self.owned
    }
    pub(super) fn selected(&self) -> CanonicalChain {
        self.selected
    }
    pub(super) fn encode(&self) -> Result<Vec<u8>, StorageError> {
        let payload = self.owned.encode()?;
        if (payload.len() as u64)
            .checked_add(ENVELOPE)
            .ok_or_else(refused)?
            > MAX_JOB
        {
            return Err(refused());
        }
        let mut bytes = Vec::new();
        bytes.extend_from_slice(REQUEST);
        bytes.extend([MODE, selector(self.selected)]);
        bytes.extend(payload);
        Ok(bytes)
    }
    pub(super) fn decode(bytes: &[u8]) -> Result<Self, StorageError> {
        if bytes.len() as u64 > MAX_JOB || bytes.len() < ENVELOPE as usize {
            return Err(refused());
        }
        let selected = read_envelope(&mut &bytes[..10], REQUEST)?;
        Ok(Self::new(OwnedScratchJob::decode(&bytes[10..])?, selected))
    }
}

fn selector(selected: CanonicalChain) -> u8 {
    match selected {
        CanonicalChain::Mainnet => 0,
        CanonicalChain::Testnet3 => 1,
        CanonicalChain::Signet => 2,
        CanonicalChain::Regtest => 3,
    }
}

fn read_envelope(input: &mut impl Read, magic: &[u8; 8]) -> Result<CanonicalChain, StorageError> {
    let mut header = [0; 10];
    io(input.read_exact(&mut header))?;
    if &header[..8] != magic || header[8] != MODE {
        return Err(refused());
    }
    match header[9] {
        0 => Ok(CanonicalChain::Mainnet),
        1 => Ok(CanonicalChain::Testnet3),
        2 => Ok(CanonicalChain::Signet),
        3 => Ok(CanonicalChain::Regtest),
        _ => Err(refused()),
    }
}

struct BoundedWriter<'a, W> {
    output: &'a mut W,
    remaining: u64,
}
impl<W: Write> Write for BoundedWriter<'_, W> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let remaining = self
            .remaining
            .checked_sub(bytes.len() as u64)
            .ok_or_else(|| std::io::Error::other("history result limit"))?;
        let written = self.output.write(bytes)?;
        self.remaining = remaining + (bytes.len() - written) as u64;
        Ok(written)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.output.flush()
    }
}

pub(super) fn write_result(
    output: &mut impl Write,
    job: &HistoryJob,
    records: &InspectedRecords,
) -> Result<(), StorageError> {
    let mut output = BoundedWriter {
        output,
        remaining: MAX_RESULT,
    };
    io(output.write_all(RESPONSE))?;
    io(output.write_all(&[MODE, selector(job.selected)]))?;
    super::write_result(&mut output, &job.owned, records)
}

pub(super) fn read_result(
    mut input: impl Read,
    length: u64,
    job: &HistoryJob,
) -> Result<InspectedRecords, StorageError> {
    if !(ENVELOPE..=MAX_RESULT).contains(&length) {
        return Err(refused());
    }
    if read_envelope(&mut input, RESPONSE)? != job.selected {
        return Err(refused());
    }
    super::read_result(input, length - ENVELOPE, &job.owned)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn job(selected: CanonicalChain) -> HistoryJob {
        HistoryJob::new(
            OwnedScratchJob {
                root: PathBuf::from("/tmp/test"),
                device: 1,
                inode: 2,
                result_device: 3,
                result_inode: 4,
                nonce: [5; 16],
                fault: 0,
            },
            selected,
        )
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

    #[test]
    fn phase167_preeffect_history_worker_codec_exact_nested_v1_and_all_selectors() {
        for selected in [
            CanonicalChain::Mainnet,
            CanonicalChain::Testnet3,
            CanonicalChain::Signet,
            CanonicalChain::Regtest,
        ] {
            // Arrange
            let job = job(selected);
            let records = records();
            let request = job.encode().expect("request");
            // Act
            let decoded = WorkerJob::decode(request.as_slice()).expect("typed request");
            let mut response = Vec::new();
            decoded
                .write_result(&mut response, &records)
                .expect("result");
            let recovered = decoded
                .read_result(response.as_slice(), response.len() as u64)
                .expect("bound response");
            // Assert
            assert_eq!(decoded.maybe_history_selection(), Some(selected));
            assert_eq!(&request[10..], job.owned.encode().expect("original frame"));
            let mut v1 = Vec::new();
            super::super::write_result(&mut v1, &job.owned, &records).expect("v1");
            assert_eq!(&response[10..], v1);
            assert_eq!(recovered.namespaces, records.namespaces);
            assert!(
                super::super::read_result(response.as_slice(), response.len() as u64, &job.owned)
                    .is_err()
            );
            assert!(read_result(v1.as_slice(), v1.len() as u64, &job).is_err());
        }
    }

    #[test]
    fn phase167_preeffect_history_worker_codec_rejects_every_truncation_binding_and_tail() {
        // Arrange
        let job = job(CanonicalChain::Regtest);
        let request = job.encode().expect("request");
        let mut response = Vec::new();
        write_result(&mut response, &job, &records()).expect("result");
        // Act / Assert
        for end in 0..request.len() {
            assert!(WorkerJob::decode(&request[..end]).is_err(), "request {end}");
        }
        for end in 0..response.len() {
            assert!(
                read_result(&response[..end], end as u64, &job).is_err(),
                "response {end}"
            );
        }
        for offset in [0, 4, 5, 7, 8, 9, 10, 14, 18, 34, 42] {
            let mut bad = response.clone();
            bad[offset] ^= 128;
            assert!(
                read_result(bad.as_slice(), bad.len() as u64, &job).is_err(),
                "binding {offset}"
            );
        }
        for offset in [0, 4, 5, 7, 8, 9, 10, 14] {
            let mut bad = request.clone();
            bad[offset] ^= 128;
            assert!(
                WorkerJob::decode(bad.as_slice()).is_err(),
                "request binding {offset}"
            );
        }
        let mut trailing = request;
        trailing.push(0);
        assert!(WorkerJob::decode(trailing.as_slice()).is_err());
        let mut wrong_selected = response.clone();
        wrong_selected[9] = selector(CanonicalChain::Mainnet);
        assert!(read_result(wrong_selected.as_slice(), wrong_selected.len() as u64, &job).is_err());
        response.push(0);
        assert!(read_result(response.as_slice(), response.len() as u64, &job).is_err());
    }

    #[test]
    fn phase167_preeffect_history_worker_codec_outer_budgets_include_envelope() {
        // Arrange
        let mut job = job(CanonicalChain::Regtest);
        let base = job.encode().expect("base").len();
        job.owned.root = PathBuf::from("x".repeat(MAX_JOB as usize - base + 9));
        // Act / Assert
        assert_eq!(job.encode().expect("exact outer cap").len() as u64, MAX_JOB);
        job.owned.root.push("y");
        assert!(job.encode().is_err());
        assert!(WorkerJob::decode(vec![0; MAX_JOB as usize + 1].as_slice()).is_err());
        assert!(read_result([].as_slice(), MAX_RESULT + 1, &job).is_err());
        let mut bytes = Vec::new();
        let mut writer = BoundedWriter {
            output: &mut bytes,
            remaining: 10,
        };
        writer.write_all(&[0; 10]).expect("exact bound");
        assert!(writer.write_all(&[1]).is_err());
        assert_eq!(bytes, vec![0; 10]);
        let mut writer = BoundedWriter {
            output: &mut bytes,
            remaining: u64::MAX,
        };
        writer.remaining = 0;
        assert!(writer.write_all(&[1]).is_err());
    }
}
