// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

//! Allocation-bounded streaming decoder for persisted mempool snapshots.

use std::fmt;

use serde::de::{DeserializeSeed, Error as _, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};

use super::{
    MempoolMemberIdentityDto, MempoolOriginV1Dto, MempoolSnapshotDecodeLimits,
    MempoolSnapshotPayloadDto, MempoolSnapshotV1Dto, MempoolSnapshotV1RecordDto,
    MempoolSnapshotV2Dto, MempoolSnapshotV2RecordDto, snapshot_failure,
};
use crate::storage::blob_schema_is_readable;
use crate::storage::mempool_snapshot::MempoolSnapshotError;
use crate::{SchemaVersion, StorageError};

mod key_preflight;
mod transaction;

use key_preflight::validate_raw_object_keys;
use transaction::TransactionSeed;

const RESOURCE_BOUND_MARKER: &str = "mempool snapshot resource bound exceeded";

pub(super) fn decode_bounded_versioned(
    bytes: &[u8],
    limits: MempoolSnapshotDecodeLimits,
) -> Result<MempoolSnapshotPayloadDto, StorageError> {
    validate_raw_object_keys(bytes)?;
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let decoded = VersionedSeed { limits }
        .deserialize(&mut deserializer)
        .map_err(map_stream_error)?;
    deserializer.end().map_err(map_stream_error)?;

    let actual = SchemaVersion::new(decoded.schema_version)?;
    if !blob_schema_is_readable(actual) {
        return Err(StorageError::schema_mismatch(
            SchemaVersion::CURRENT,
            actual,
        ));
    }
    Ok(decoded.payload)
}

struct DecodedVersioned {
    schema_version: u32,
    payload: MempoolSnapshotPayloadDto,
}

struct VersionedSeed {
    limits: MempoolSnapshotDecodeLimits,
}

impl<'de> DeserializeSeed<'de> for VersionedSeed {
    type Value = DecodedVersioned;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(VersionedVisitor {
            limits: self.limits,
        })
    }
}

struct VersionedVisitor {
    limits: MempoolSnapshotDecodeLimits,
}

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum VersionedField {
    SchemaVersion,
    Payload,
    #[serde(other)]
    Unknown,
}

impl<'de> Visitor<'de> for VersionedVisitor {
    type Value = DecodedVersioned;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a versioned mempool snapshot")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut maybe_schema_version = None;
        let mut maybe_payload = None;
        while let Some(field) = map.next_key::<VersionedField>()? {
            match field {
                VersionedField::SchemaVersion => {
                    set_once(
                        &mut maybe_schema_version,
                        map.next_value()?,
                        "schema_version",
                    )?;
                }
                VersionedField::Payload => {
                    let payload = map.next_value_seed(PayloadSeed {
                        limits: self.limits,
                    })?;
                    set_once(&mut maybe_payload, payload, "payload")?;
                }
                VersionedField::Unknown => {
                    return Err(A::Error::unknown_field(
                        "unknown",
                        &["schema_version", "payload"],
                    ));
                }
            }
        }

        Ok(DecodedVersioned {
            schema_version: required(maybe_schema_version, "schema_version")?,
            payload: required(maybe_payload, "payload")?,
        })
    }
}

struct PayloadSeed {
    limits: MempoolSnapshotDecodeLimits,
}

impl<'de> DeserializeSeed<'de> for PayloadSeed {
    type Value = MempoolSnapshotPayloadDto;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(PayloadVisitor {
            limits: self.limits,
        })
    }
}

struct PayloadVisitor {
    limits: MempoolSnapshotDecodeLimits,
}

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum PayloadField {
    FormatVersion,
    CapturedGeneration,
    CapturedAtUnixSeconds,
    Records,
    UnbroadcastMembers,
    #[serde(other)]
    Unknown,
}

impl<'de> Visitor<'de> for PayloadVisitor {
    type Value = MempoolSnapshotPayloadDto;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a current or legacy mempool snapshot payload")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut maybe_format_version = None;
        let mut maybe_captured_generation = None;
        let mut maybe_captured_at = None;
        let mut maybe_records = None;
        let mut maybe_unbroadcast = None;
        let mut total_transaction_bytes = 0_usize;

        while let Some(field) = map.next_key::<PayloadField>()? {
            match field {
                PayloadField::FormatVersion => {
                    set_once(
                        &mut maybe_format_version,
                        map.next_value()?,
                        "format_version",
                    )?;
                }
                PayloadField::CapturedGeneration => {
                    set_once(
                        &mut maybe_captured_generation,
                        map.next_value()?,
                        "captured_generation",
                    )?;
                }
                PayloadField::CapturedAtUnixSeconds => {
                    set_once(
                        &mut maybe_captured_at,
                        map.next_value()?,
                        "captured_at_unix_seconds",
                    )?;
                }
                PayloadField::Records => {
                    let records = map.next_value_seed(RecordsSeed {
                        limits: self.limits,
                        total_transaction_bytes: &mut total_transaction_bytes,
                    })?;
                    set_once(&mut maybe_records, records, "records")?;
                }
                PayloadField::UnbroadcastMembers => {
                    let members =
                        map.next_value_seed(BoundedSequenceSeed::<MempoolMemberIdentityDto>::new(
                            self.limits.max_unbroadcast_members,
                        ))?;
                    set_once(&mut maybe_unbroadcast, members, "unbroadcast_members")?;
                }
                PayloadField::Unknown => {
                    return Err(A::Error::unknown_field(
                        "unknown",
                        &[
                            "format_version",
                            "captured_generation",
                            "captured_at_unix_seconds",
                            "records",
                            "unbroadcast_members",
                        ],
                    ));
                }
            }
        }

        let records = required(maybe_records, "records")?;
        if let Some(format_version) = maybe_format_version {
            let records = records
                .into_iter()
                .map(BoundedRecord::into_current)
                .collect::<Result<Vec<_>, _>>()?;
            return Ok(MempoolSnapshotPayloadDto::CurrentV2(MempoolSnapshotV2Dto {
                format_version,
                captured_generation: required(maybe_captured_generation, "captured_generation")?,
                captured_at_unix_seconds: required(maybe_captured_at, "captured_at_unix_seconds")?,
                records,
                unbroadcast_members: required(maybe_unbroadcast, "unbroadcast_members")?,
            }));
        }

        if maybe_captured_generation.is_some()
            || maybe_captured_at.is_some()
            || maybe_unbroadcast.is_some()
        {
            return Err(A::Error::custom(
                "legacy snapshot contains current-only fields",
            ));
        }
        let records = records
            .into_iter()
            .map(BoundedRecord::into_legacy)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(MempoolSnapshotPayloadDto::LegacyV1(MempoolSnapshotV1Dto {
            records,
        }))
    }
}

struct RecordsSeed<'a> {
    limits: MempoolSnapshotDecodeLimits,
    total_transaction_bytes: &'a mut usize,
}

impl<'de> DeserializeSeed<'de> for RecordsSeed<'_> {
    type Value = Vec<BoundedRecord>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(RecordsVisitor {
            limits: self.limits,
            total_transaction_bytes: self.total_transaction_bytes,
        })
    }
}

struct RecordsVisitor<'a> {
    limits: MempoolSnapshotDecodeLimits,
    total_transaction_bytes: &'a mut usize,
}

impl<'de> Visitor<'de> for RecordsVisitor<'_> {
    type Value = Vec<BoundedRecord>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a bounded mempool record sequence")
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        reject_size_hint(sequence.size_hint(), self.limits.max_records)?;
        let mut records = Vec::with_capacity(
            sequence
                .size_hint()
                .unwrap_or_default()
                .min(self.limits.max_records),
        );
        loop {
            if records.len() == self.limits.max_records {
                reject_extra_element(&mut sequence)?;
                break;
            }
            let Some(record) = sequence.next_element_seed(RecordSeed {
                limits: self.limits,
                total_transaction_bytes: self.total_transaction_bytes,
            })?
            else {
                break;
            };
            records.push(record);
        }
        Ok(records)
    }
}

#[derive(Default)]
struct BoundedRecord {
    maybe_txid: Option<[u8; 32]>,
    maybe_wtxid: Option<[u8; 32]>,
    maybe_transaction: Option<Vec<u8>>,
    maybe_fee_sats: Option<i64>,
    maybe_virtual_size: Option<usize>,
    maybe_accepted_at: Option<Option<i64>>,
    maybe_origin: Option<Option<MempoolOriginV1Dto>>,
    maybe_relay_requested: Option<Option<bool>>,
}

impl BoundedRecord {
    fn into_current<E: serde::de::Error>(self) -> Result<MempoolSnapshotV2RecordDto, E> {
        if self.maybe_txid.is_some()
            || self.maybe_wtxid.is_some()
            || self.maybe_fee_sats.is_some()
            || self.maybe_virtual_size.is_some()
            || self.maybe_origin.is_some()
            || self.maybe_relay_requested.is_some()
        {
            return Err(E::custom(
                "current record contains a derived or legacy field",
            ));
        }
        Ok(MempoolSnapshotV2RecordDto {
            transaction: required(self.maybe_transaction, "transaction")?,
            accepted_at_unix_seconds: required(self.maybe_accepted_at, "accepted_at_unix_seconds")?,
        })
    }

    fn into_legacy<E: serde::de::Error>(self) -> Result<MempoolSnapshotV1RecordDto, E> {
        Ok(MempoolSnapshotV1RecordDto {
            txid: required(self.maybe_txid, "txid")?,
            wtxid: required(self.maybe_wtxid, "wtxid")?,
            transaction: required(self.maybe_transaction, "transaction")?,
            fee_sats: required(self.maybe_fee_sats, "fee_sats")?,
            virtual_size: required(self.maybe_virtual_size, "virtual_size")?,
            maybe_accepted_at_unix_seconds: self.maybe_accepted_at.flatten(),
            maybe_origin: self.maybe_origin.flatten(),
            maybe_relay_requested: self.maybe_relay_requested.flatten(),
        })
    }
}

struct RecordSeed<'a> {
    limits: MempoolSnapshotDecodeLimits,
    total_transaction_bytes: &'a mut usize,
}

impl<'de> DeserializeSeed<'de> for RecordSeed<'_> {
    type Value = BoundedRecord;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(RecordVisitor {
            limits: self.limits,
            total_transaction_bytes: self.total_transaction_bytes,
        })
    }
}

struct RecordVisitor<'a> {
    limits: MempoolSnapshotDecodeLimits,
    total_transaction_bytes: &'a mut usize,
}

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum RecordField {
    Txid,
    Wtxid,
    Transaction,
    FeeSats,
    VirtualSize,
    AcceptedAtUnixSeconds,
    Origin,
    RelayRequested,
    #[serde(other)]
    Unknown,
}

impl<'de> Visitor<'de> for RecordVisitor<'_> {
    type Value = BoundedRecord;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a bounded current or legacy mempool record")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut record = BoundedRecord::default();
        while let Some(field) = map.next_key::<RecordField>()? {
            match field {
                RecordField::Txid => {
                    set_once(&mut record.maybe_txid, map.next_value()?, "txid")?;
                }
                RecordField::Wtxid => {
                    set_once(&mut record.maybe_wtxid, map.next_value()?, "wtxid")?;
                }
                RecordField::Transaction => {
                    let transaction = map.next_value_seed(TransactionSeed::new(
                        self.limits.max_transaction_bytes,
                        self.limits.max_total_transaction_bytes,
                        self.total_transaction_bytes,
                    ))?;
                    set_once(&mut record.maybe_transaction, transaction, "transaction")?;
                }
                RecordField::FeeSats => {
                    set_once(&mut record.maybe_fee_sats, map.next_value()?, "fee_sats")?;
                }
                RecordField::VirtualSize => {
                    set_once(
                        &mut record.maybe_virtual_size,
                        map.next_value()?,
                        "virtual_size",
                    )?;
                }
                RecordField::AcceptedAtUnixSeconds => {
                    set_once(
                        &mut record.maybe_accepted_at,
                        map.next_value()?,
                        "accepted_at_unix_seconds",
                    )?;
                }
                RecordField::Origin => {
                    set_once(&mut record.maybe_origin, map.next_value()?, "origin")?;
                }
                RecordField::RelayRequested => {
                    set_once(
                        &mut record.maybe_relay_requested,
                        map.next_value()?,
                        "relay_requested",
                    )?;
                }
                RecordField::Unknown => {
                    return Err(A::Error::unknown_field(
                        "unknown",
                        &[
                            "transaction",
                            "accepted_at_unix_seconds",
                            "txid",
                            "wtxid",
                            "fee_sats",
                            "virtual_size",
                            "origin",
                            "relay_requested",
                        ],
                    ));
                }
            }
        }
        Ok(record)
    }
}

struct BoundedSequenceSeed<T> {
    max_items: usize,
    marker: std::marker::PhantomData<T>,
}

impl<T> BoundedSequenceSeed<T> {
    const fn new(max_items: usize) -> Self {
        Self {
            max_items,
            marker: std::marker::PhantomData,
        }
    }
}

impl<'de, T> DeserializeSeed<'de> for BoundedSequenceSeed<T>
where
    T: Deserialize<'de>,
{
    type Value = Vec<T>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(BoundedSequenceVisitor::<T> {
            max_items: self.max_items,
            marker: std::marker::PhantomData,
        })
    }
}

struct BoundedSequenceVisitor<T> {
    max_items: usize,
    marker: std::marker::PhantomData<T>,
}

impl<'de, T> Visitor<'de> for BoundedSequenceVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a bounded sequence")
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        reject_size_hint(sequence.size_hint(), self.max_items)?;
        let mut values =
            Vec::with_capacity(sequence.size_hint().unwrap_or_default().min(self.max_items));
        loop {
            if values.len() == self.max_items {
                reject_extra_element(&mut sequence)?;
                break;
            }
            let Some(value) = sequence.next_element()? else {
                break;
            };
            values.push(value);
        }
        Ok(values)
    }
}

struct RejectExtra;

impl<'de> DeserializeSeed<'de> for RejectExtra {
    type Value = ();

    fn deserialize<D>(self, _deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Err(D::Error::custom(RESOURCE_BOUND_MARKER))
    }
}

pub(super) fn reject_extra_element<'de, A>(sequence: &mut A) -> Result<(), A::Error>
where
    A: SeqAccess<'de>,
{
    if sequence.next_element_seed(RejectExtra)?.is_some() {
        return Err(A::Error::custom(RESOURCE_BOUND_MARKER));
    }
    Ok(())
}

pub(super) fn reject_size_hint<E: serde::de::Error>(
    maybe_size: Option<usize>,
    max_items: usize,
) -> Result<(), E> {
    if maybe_size.is_some_and(|size| size > max_items) {
        return Err(E::custom(RESOURCE_BOUND_MARKER));
    }
    Ok(())
}

fn required<T, E: serde::de::Error>(maybe_value: Option<T>, field: &'static str) -> Result<T, E> {
    maybe_value.ok_or_else(|| E::missing_field(field))
}

fn set_once<T, E: serde::de::Error>(
    target: &mut Option<T>,
    value: T,
    field: &str,
) -> Result<(), E> {
    if target.replace(value).is_some() {
        return Err(E::custom(format_args!("duplicate field `{field}`")));
    }
    Ok(())
}

fn map_stream_error(error: serde_json::Error) -> StorageError {
    if error.to_string().contains(RESOURCE_BOUND_MARKER) {
        return snapshot_failure(MempoolSnapshotError::ResourceBoundExceeded);
    }
    snapshot_failure(MempoolSnapshotError::DecodeFailure)
}
