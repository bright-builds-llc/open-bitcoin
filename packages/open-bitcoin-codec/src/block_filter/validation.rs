// Parity breadcrumbs:
// - packages/bitcoin-knots/src/blockfilter.cpp
// - packages/bitcoin-knots/src/util/golombrice.h
// - packages/bitcoin-knots/src/streams.h
// - packages/bitcoin-knots/src/serialize.h

//! Streaming integrity validation for stored BASIC encodings.

use core::fmt;

use super::{BASIC_FILTER_M, BASIC_FILTER_P};
use crate::{CodecError, MAX_SIZE, primitives::Reader, read_compact_size};

/// Corruption or a resource refusal in persisted BASIC bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BasicFilterValidationError {
    Codec(CodecError),
    BytesTooLarge,
    InsufficientBits,
    TruncatedBits,
    ArithmeticOverflow,
    ValueOutOfRange,
    NonzeroPadding,
    TrailingBytes,
}

impl fmt::Display for BasicFilterValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Codec(error) => error.fmt(f),
            Self::BytesTooLarge => f.write_str("BASIC bytes exceed MAX_SIZE"),
            Self::InsufficientBits => f.write_str("BASIC count exceeds minimum bit capacity"),
            Self::TruncatedBits => f.write_str("BASIC Golomb-Rice stream is truncated"),
            Self::ArithmeticOverflow => f.write_str("BASIC arithmetic overflow"),
            Self::ValueOutOfRange => f.write_str("BASIC accumulated value is outside N*M"),
            Self::NonzeroPadding => f.write_str("BASIC final padding is nonzero"),
            Self::TrailingBytes => f.write_str("BASIC encoding contains trailing bytes"),
        }
    }
}

impl std::error::Error for BasicFilterValidationError {}

/// Validate canonical bounded BASIC bytes without allocating mapped values.
/// Work is bounded by input bits, including unary decoding of hostile data.
pub fn validate_basic_filter_encoding(bytes: &[u8]) -> Result<(), BasicFilterValidationError> {
    if bytes.len() as u64 > MAX_SIZE {
        return Err(BasicFilterValidationError::BytesTooLarge);
    }
    let mut reader = Reader::new(bytes);
    let count = read_compact_size(&mut reader).map_err(BasicFilterValidationError::Codec)?;
    // CompactSize's MAX_SIZE bound makes both products representable in u64.
    let range = count * BASIC_FILTER_M;
    let payload = &bytes[bytes.len() - reader.remaining()..];
    if count * u64::from(BASIC_FILTER_P + 1) > payload.len() as u64 * 8 {
        return Err(BasicFilterValidationError::InsufficientBits);
    }
    let mut bits = BitReader {
        bytes: payload,
        position: 0,
    };
    let mut value = 0;
    for _ in 0..count {
        let mut quotient = 0;
        while bits.read()? {
            quotient += 1;
        }
        let mut remainder = 0;
        for _ in 0..BASIC_FILTER_P {
            remainder = (remainder << 1) | u64::from(bits.read()?);
        }
        value = accumulate_delta(value, quotient, remainder)?;
        if value >= range {
            return Err(BasicFilterValidationError::ValueOutOfRange);
        }
    }
    bits.finish()
}

fn accumulate_delta(
    last: u64,
    quotient: u64,
    remainder: u64,
) -> Result<u64, BasicFilterValidationError> {
    quotient
        .checked_mul(1 << BASIC_FILTER_P)
        .and_then(|high| high.checked_add(remainder))
        .and_then(|delta| last.checked_add(delta))
        .ok_or(BasicFilterValidationError::ArithmeticOverflow)
}

struct BitReader<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl BitReader<'_> {
    fn read(&mut self) -> Result<bool, BasicFilterValidationError> {
        let Some(byte) = self.bytes.get(self.position / 8) else {
            return Err(BasicFilterValidationError::TruncatedBits);
        };
        let bit = byte & (1 << (7 - self.position % 8)) != 0;
        self.position += 1;
        Ok(bit)
    }

    fn finish(&mut self) -> Result<(), BasicFilterValidationError> {
        if self.position.div_ceil(8) != self.bytes.len() {
            return Err(BasicFilterValidationError::TrailingBytes);
        }
        while !self.position.is_multiple_of(8) {
            if self.read()? {
                return Err(BasicFilterValidationError::NonzeroPadding);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
