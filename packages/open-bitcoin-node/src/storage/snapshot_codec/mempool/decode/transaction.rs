// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

//! Allocation-bounded transaction byte decoding for mempool snapshots.

use std::fmt;

use serde::de::{DeserializeSeed, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;

use super::{RESOURCE_BOUND_MARKER, reject_extra_element, reject_size_hint};

pub(super) struct TransactionSeed<'a> {
    max_transaction_bytes: usize,
    max_total_transaction_bytes: usize,
    total_transaction_bytes: &'a mut usize,
}

impl<'a> TransactionSeed<'a> {
    pub(super) fn new(
        max_transaction_bytes: usize,
        max_total_transaction_bytes: usize,
        total_transaction_bytes: &'a mut usize,
    ) -> Self {
        Self {
            max_transaction_bytes,
            max_total_transaction_bytes,
            total_transaction_bytes,
        }
    }
}

impl<'de> DeserializeSeed<'de> for TransactionSeed<'_> {
    type Value = Vec<u8>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = <&'de RawValue>::deserialize(deserializer)?;
        let token = raw.get().as_bytes();
        if token.first() == Some(&b'"') {
            return TransactionVisitor {
                max_transaction_bytes: self.max_transaction_bytes,
                max_total_transaction_bytes: self.max_total_transaction_bytes,
                total_transaction_bytes: self.total_transaction_bytes,
            }
            .decode_raw_hex(token);
        }

        let mut nested = serde_json::Deserializer::from_str(raw.get());
        let bytes = LegacyTransactionSeed {
            max_transaction_bytes: self.max_transaction_bytes,
            max_total_transaction_bytes: self.max_total_transaction_bytes,
            total_transaction_bytes: self.total_transaction_bytes,
        }
        .deserialize(&mut nested)
        .map_err(serde::de::Error::custom)?;
        nested.end().map_err(serde::de::Error::custom)?;
        Ok(bytes)
    }
}

struct TransactionVisitor<'a> {
    max_transaction_bytes: usize,
    max_total_transaction_bytes: usize,
    total_transaction_bytes: &'a mut usize,
}

impl TransactionVisitor<'_> {
    fn reserve<E: serde::de::Error>(&mut self, bytes: usize) -> Result<(), E> {
        if bytes > self.max_transaction_bytes {
            return Err(E::custom(RESOURCE_BOUND_MARKER));
        }
        let next_total = self
            .total_transaction_bytes
            .checked_add(bytes)
            .ok_or_else(|| E::custom(RESOURCE_BOUND_MARKER))?;
        if next_total > self.max_total_transaction_bytes {
            return Err(E::custom(RESOURCE_BOUND_MARKER));
        }
        *self.total_transaction_bytes = next_total;
        Ok(())
    }

    fn decode_raw_hex<E: serde::de::Error>(&mut self, token: &[u8]) -> Result<Vec<u8>, E> {
        let Some(value) = token.get(1..token.len().saturating_sub(1)) else {
            return Err(E::custom("transaction hex must be a JSON string"));
        };
        let max_hex_bytes = self
            .max_transaction_bytes
            .checked_mul(2)
            .ok_or_else(|| E::custom(RESOURCE_BOUND_MARKER))?;
        let remaining_total = self
            .max_total_transaction_bytes
            .saturating_sub(*self.total_transaction_bytes);
        let max_remaining_hex_bytes = remaining_total
            .checked_mul(2)
            .ok_or_else(|| E::custom(RESOURCE_BOUND_MARKER))?;
        if value.len() > max_hex_bytes
            || value.len() > max_remaining_hex_bytes
            || value.contains(&b'\\')
        {
            return Err(E::custom(RESOURCE_BOUND_MARKER));
        }
        if !value.len().is_multiple_of(2) {
            return Err(E::custom("transaction hex has odd length"));
        }
        let decoded_len = value.len() / 2;
        self.reserve(decoded_len)?;
        let mut bytes = Vec::with_capacity(decoded_len);
        for pair in value.chunks_exact(2) {
            let high = hex_nibble(pair[0]).ok_or_else(|| E::custom("invalid transaction hex"))?;
            let low = hex_nibble(pair[1]).ok_or_else(|| E::custom("invalid transaction hex"))?;
            bytes.push((high << 4) | low);
        }
        Ok(bytes)
    }
}

struct LegacyTransactionSeed<'a> {
    max_transaction_bytes: usize,
    max_total_transaction_bytes: usize,
    total_transaction_bytes: &'a mut usize,
}

impl<'de> DeserializeSeed<'de> for LegacyTransactionSeed<'_> {
    type Value = Vec<u8>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(LegacyTransactionVisitor {
            max_transaction_bytes: self.max_transaction_bytes,
            max_total_transaction_bytes: self.max_total_transaction_bytes,
            total_transaction_bytes: self.total_transaction_bytes,
        })
    }
}

struct LegacyTransactionVisitor<'a> {
    max_transaction_bytes: usize,
    max_total_transaction_bytes: usize,
    total_transaction_bytes: &'a mut usize,
}

impl<'de> Visitor<'de> for LegacyTransactionVisitor<'_> {
    type Value = Vec<u8>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("bounded hex or legacy byte-array transaction data")
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        reject_size_hint(sequence.size_hint(), self.max_transaction_bytes)?;
        let mut bytes = Vec::with_capacity(
            sequence
                .size_hint()
                .unwrap_or_default()
                .min(self.max_transaction_bytes),
        );
        loop {
            if bytes.len() == self.max_transaction_bytes
                || *self.total_transaction_bytes == self.max_total_transaction_bytes
            {
                reject_extra_element(&mut sequence)?;
                break;
            }
            let Some(byte) = sequence.next_element::<u8>()? else {
                break;
            };
            bytes.push(byte);
            *self.total_transaction_bytes += 1;
        }
        Ok(bytes)
    }
}

const fn hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
