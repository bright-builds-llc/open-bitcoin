// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

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
fn pruned_proof_result_is_bound_to_its_larger_logical_inventory() {
    // Arrange
    let operation = WorkerJob::PruneProof(job());
    let request = operation.encode().expect("prune-proof request");
    let records = InspectedRecords {
        namespaces: BTreeMap::from([(
            "rows".into(),
            (0..copy::MAX_ENTRIES)
                .map(|index| (index.to_le_bytes().to_vec(), vec![1]))
                .collect(),
        )]),
    };
    // Act
    let worker = WorkerJob::decode(request.as_slice()).expect("worker mode");
    let mut result = Vec::new();
    worker
        .write_result(&mut result, &records)
        .expect("extended logical result");
    let length = result.len() as u64;
    // Assert
    assert!(
        operation
            .read_result(result.as_slice(), length)
            .expect("parent uses same mode")
            == records
    );
    assert!(
        WorkerJob::Generic(job())
            .read_result(result.as_slice(), length)
            .is_err()
    );
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
fn phase166_inspection_protocol_rejects_truncation_version_status_trailing_and_receipt_mismatch() {
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
