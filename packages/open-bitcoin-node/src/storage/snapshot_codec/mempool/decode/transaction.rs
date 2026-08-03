// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

//! Allocation-bounded transaction byte decoding for mempool snapshots.

use std::fmt;

use serde::Deserializer;
use serde::de::{DeserializeSeed, SeqAccess, Visitor};

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
        deserializer.deserialize_any(TransactionVisitor {
            max_transaction_bytes: self.max_transaction_bytes,
            max_total_transaction_bytes: self.max_total_transaction_bytes,
            total_transaction_bytes: self.total_transaction_bytes,
        })
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

    fn decode_hex<E: serde::de::Error>(&mut self, value: &str) -> Result<Vec<u8>, E> {
        if !value.len().is_multiple_of(2) {
            return Err(E::custom("transaction hex has odd length"));
        }
        let decoded_len = value.len() / 2;
        self.reserve(decoded_len)?;
        let mut bytes = Vec::with_capacity(decoded_len);
        for pair in value.as_bytes().chunks_exact(2) {
            let high = hex_nibble(pair[0]).ok_or_else(|| E::custom("invalid transaction hex"))?;
            let low = hex_nibble(pair[1]).ok_or_else(|| E::custom("invalid transaction hex"))?;
            bytes.push((high << 4) | low);
        }
        Ok(bytes)
    }
}

impl<'de> Visitor<'de> for TransactionVisitor<'_> {
    type Value = Vec<u8>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("bounded hex or legacy byte-array transaction data")
    }

    fn visit_borrowed_str<E>(mut self, value: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.decode_hex(value)
    }

    fn visit_str<E>(mut self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.decode_hex(value)
    }

    fn visit_string<E>(mut self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.decode_hex(&value)
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
